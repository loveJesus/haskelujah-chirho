<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# BUG — compiled binaries print a heap ADDRESS instead of a computed list

**Found** 2026-07-27 by `claude_chirho` (HASKELUJAH) while checking whether the sieve
miscompile ([[bug-comprehension-letrec-capture-chirho]]) also affects the native backends.
It does not — but this does, and it is more user-visible.

**Severity: HIGH, and maximally visible.** Anyone who compiles a program that prints a
computed list gets a large number instead. No error, no warning. The number **changes every
run** (ASLR), so it is not even stably wrong.

Affects **both** native backends — LLVM and Cranelift — so it is in the shared native
`show`/print path, not in a single code generator. The STG interpreter (`haskelujah run`,
the default execution path) is correct throughout.

## Repro

```haskell
main :: IO ()
main = print (filter odd [1,2,3,4,5])
```

```
$ haskelujah run    ND.hs        =>  [1,3,5]          correct
$ haskelujah compile ND.hs --output nd && ./nd
                                 =>  4309262449       WRONG
$ ./nd                           =>  4371849281       different every run
$ ./nd                           =>  44841306657
$ haskelujah compile ND.hs --cranelift --output nd2 && ./nd2
                                 =>  4308279329       same fault, other backend
```

## ROOT CAUSE — VERIFIED. `print`'s fallback renders EVERY value as an Int.

This supersedes both earlier framings in this file. The mechanism is now read directly out
of the emitted IR, and the simplest repro is not a list at all:

```haskell
main = print (id True)      -- native prints  1        interpreter prints  True
main = print (id (1.5::Double))
                            -- native prints  460943421861   (raw bit pattern)
```

`print` in the LLVM backend has a syntactic fast path and a fallback
(`codegen_chirho.rs:2012-2018`). The fallback calls the generic `show`, and that function is
emitted as:

```llvm
define i64 @haskelujah_show(i64 %v) {
  %t1 = call i64 @haskelujah_enter_thunk_chirho(i64 %v)
  %t2 = call i64 @haskelujah_show_int_chirho(i64 %t1)   ; <-- unconditional
  ret i64 %t2
}
```

It forces the thunk and renders **whatever comes back as an Int**. Identical in every
program dumped. That single line explains all the variety:

| value reaching the fallback | forced representation | printed as |
|---|---|---|
| `Bool` | constructor tag | `1` |
| derived-`Show` constructor | constructor tag | `82` |
| `Double` | raw bits | `460943421861` |
| list | heap pointer | `50885298529` |
| `Int` | the integer | **correct — by coincidence** |

`Int` being right is an accident of the value already being an integer, which is exactly why
this survived: the one type anyone tests first is the one type that works.

**A separate, smaller fault also confirmed:** an *explicit* `show` call resolves its
dictionary correctly for scalars (`show (id True)` → `True`, `show (id 1.5)` → `1.5`) but
NOT for lists (`putStrLn (show (id [1,2,3]))` → pointer). So list rendering is broken on
both the explicit and the fallback path, while scalars are broken only on the fallback path.

### How I got here, including the wrong turn

I first wrote that the fault was "syntax-directed classification at the call site". That was
a description of the *fast path*, not the bug. I then claimed the generic `show` was
unconditionally `show_int` and made a falsifiable prediction — `show (id True)` should print
a number. **It printed `True`, so the prediction failed and the claim was wrong as stated.**
The resolution: explicit `show` and `print`'s fallback are *different paths*, and only the
latter goes through the Int-rendering generic. Testing the prediction is what separated
them; asserting it would have shipped a third wrong mechanism.

### Fix direction — evidence says the fix is upstream of codegen

Core for the failing program carries **no `Show` dictionary at all**:

```
main = (v1 (v2 True))          -- print (id True); v1 = print, v2 = id
main = (v1 (v2 (1.5 @Double))) -- print (id 1.5)
```

So the dictionary pass never resolves `Show` for `print`'s argument — `print` is treated as
an IO primop (it is listed in `is_io_primop_chirho`) rather than as a function that needs
class evidence. The backend then has no type information left to dispatch on, which is why
its fallback falls back to Int.

Explicit `show` **does** get resolved. Same value, two spellings, two outcomes:

```
print (id True)              -> 1        WRONG
putStrLn (show (id True))    -> True     correct
```

That makes the minimal fix the Haskell definition itself — rewrite `print x` as
`putStrLn (show x)` before dictionary resolution, so `print` inherits the `show` machinery
that already works, instead of the backend guessing a type it was never given.

Caveats to respect before attempting it:

- `print` is a builtin on the interpreter path too, where it is currently **correct**. Any
  rewrite changes both paths, so the full driver suite (1,735 tests) is the gate, not a
  spot check.
- This fixes scalars and derived-`Show` constructors. It does **not** fix lists: explicit
  `show` on a list is independently broken (`putStrLn (show (id [1,2,3]))` → pointer), so
  `Show [a]` from `Show a` remains a separate, larger job.

### FIX PLAN — reuse the evidence mechanism that already exists (read-only prep, 2026-07-27)

Tracing the dictionary path gives a concrete plan that adds no new machinery.

**How `show` actually resolves.** There is no `Show [a]` from `Show a`. Dispatch is a
hardcoded enumeration of *concrete* triples in
`crates/haskelujah-core-chirho/src/dict_chirho/instance_chirho.rs`:

```
("Show", "show", "Int",  2), ("Show", "show", "Bool", 0), ("Show", "show", "Double", 2),
("Show", "show", "[Char]", 2), ("Show", "show", "[Int]", 2),
("Show", "show", "Maybe Int", 2), ("Show", "show", "Maybe String", 2), …
```

Each generates a binding named `$prim_Show_show_<TypeKey>`. So `show` is correct exactly
when the resolved type is *in that table*, and `print` is wrong because **it never consults
the table at all** — it is an IO primop (`is_io_primop_chirho`) and no `Show` evidence is
ever attached to its argument. Confirmed in Core: `main = (v1 (v2 True))`, no dictionary.

**Why the dict pass cannot fix this alone.** `rewrite_chirho.rs` reasons *structurally*, not
from a type environment — e.g. `print_arg_needs_int_default_chirho` inspects the expression
shape for numeric markers. It has no expression→type map, so it cannot name the argument's
type key by itself.

**But the type checker already records exactly that.** `MethodOccurrenceRecordChirho`
(`infer_chirho.rs:114`) carries:

```rust
pub struct MethodOccurrenceRecordChirho {
    pub name_chirho: String,        // e.g. "show"
    pub ordinal_chirho: u32,
    pub class_name_chirho: String,  // e.g. "Show"
    pub ty_key_chirho: String,      // e.g. "Bool" — THE RESOLVED CONCRETE TYPE
}
```

and the driver already consumes these to build the evidence map
(`driver lib.rs:3160-3200`, which sorts its id vectors and is deterministic).

**Refinement — `print` ALREADY carries the constraint.** `infer_chirho.rs:9494` binds

```haskell
print :: forall a. Show a => a -> IO ()
```

with a real `SchemePredChirho { class_name: "Show", ty: a }`. So the checker *does* generate
and solve a `Show` constraint at every `print` site — the type is known. The gap is
narrower than "print isn't typed as a method use":

> Occurrence records are threaded for class **methods** (`show`, `==`, `compare`). `print`
> is a **constrained function**, not a method, so its solved evidence is simply never
> recorded and never reaches the dict pass.

That is a meaningfully smaller fix than adding type information — the information already
exists and is already correct. It only has to be *written down* at the call site in the form
the dict pass already consumes.

It also suggests the general shape of the real repair: evidence threading currently covers
class methods but not constrained functions. `print` is the most visible casualty, not a
special case. Fixing the general case would repair every user-written
`f :: Show a => a -> …` too, and is probably the better investment if the narrow version
proves awkward.

**Therefore the fix, in three bricks:**

1. **Typing** — when inferring `print e`, record a method occurrence for the *implicit*
   `show`: `{ name: "show", class_name: "Show", ty_key: <resolved type key of e> }`. This is
   the same call the checker already makes for an explicit `show`; `print` is simply not
   currently treated as a method use.
2. **Dict pass** — at a `print` application, look up that occurrence and rewrite to the
   concrete instance, i.e. `print e` → `putStrLn ($prim_Show_show_<TyKey> e)`. The existing
   `print_arg_needs_int_default_chirho` special case stays: numeric defaulting to `Int` is
   correct Haskell and must not be disturbed.
3. **Backend** — no change needed. Once a concrete `show` is applied, `print`'s
   Int-rendering fallback is never reached for these programs.

### VALUE CASE, MEASURED — 7 of 8 enumerated Show types are broken through `print`

Every type below already has an entry in the instance enumeration, so all of them are
repaired by threading `print`'s evidence. Measured with `print (id (…))` so the value is
hidden from the syntactic fast path, native binary vs interpreter:

| type | native | interpreter | |
|---|---|---|---|
| `Int` | `42` | `42` | ok — the coincidence |
| `Bool` | **`1`** | `True` | BROKEN |
| `Char` | **`120`** | `'x'` | BROKEN — that is the ASCII code |
| `Double` | **`460943421861370265`** | `1.5` | BROKEN — raw bits |
| `String` / `[Char]` | **`4299977852`** | `"hi"` | BROKEN — pointer |
| `[Int]` | **`4347617521`** | `[1,2,3]` | BROKEN — pointer |
| `Maybe Int` | **`4337604625`** | `Just 3` | BROKEN — pointer |
| `Maybe String` | **`4312209457`** | `Just "hi"` | BROKEN — pointer |

`Char` printing as `120` is the clearest single illustration of the mechanism: `show_int`
applied to a character.

This doubles as the **test matrix**. A landed fix should turn every row above green, and the
`Int` row must stay green — it is the one case the current fallback gets right, so it is
also the one a careless fix could break.

**Scope honesty.** This fixes every type *present in the enumeration* — `Bool`, `Double`,
`Char`, `[Int]`, `[Char]`, the `Maybe` entries, and derived-`Show` constructors once their
key resolves. It does **not** give us `Show [a]` from `Show a` in general; a list of a type
absent from the table stays broken. Real polymorphic dictionary construction is a separate,
larger job and should not be smuggled into this fix.

**Gates this needs:** `eval_` suite (1002 tests, 199s) is the right fast gate — it is where a
changed `print` would show as altered program output — plus a native round-trip check that
`print (id True)` emits `True`, and a regression test that `print 5` still defaults to `Int`.

### Backend-only tag dispatch is RULED OUT — do not attempt it

An earlier suggestion in this file was to dispatch in the backend fallback on the forced
value's runtime tag. **That cannot work, and the evidence is in the bug itself.**

`crates/haskelujah-rts-chirho/src/native_layout_chirho.rs` does define an object kind tag
(`ThunkChirho`/`FunChirho`/`ConChirho`/`PapChirho`, 2 bits) and a constructor tag
(`CON_TAG_SHIFT_CHIRHO`). But those only describe **heap objects**. The failing case proves
scalars are not heap objects:

> `print (id True)` prints `1`. If `True` were a heap `Con`, rendering it as an integer
> would print a *pointer* — a large number, as the list case does. It printed `1`, so
> `True` is an unboxed `1`.

And the integer `1` is also an unboxed `1`. **At runtime, `True` and `1` are the same
bits.** No amount of tag inspection can separate them, because there is no tag to inspect.
The same argument applies to `Double`, whose raw bits printed as `460943421861`.

So the type information genuinely must come from the compiler; it cannot be recovered in the
runtime. The fix belongs at dictionary resolution, and the tag-dispatch route is a dead end
rather than merely an inferior one.

This also explains why `Int` is the one type that works: the fallback assumes Int, and for
Int that assumption happens to be right.

## Earlier framing — kept for the record, superseded above

The filename says "list pointer" because that is how I first hit it. Further probing shows
lists are only the most obvious instance. The real fault is:

> Native `show`/`print` is **syntax-directed at the call site**. `classify_expr_show_kind_chirho`
> (`crates/haskelujah-backend-llvm-chirho/src/codegen_chirho.rs:1071`) pattern-matches the
> Core *expression* — literals, comparison primops, a hardcoded list of names like
> `not`/`==`/`compare`. Anything it cannot classify syntactically falls off that fast path,
> and the generic path emits an internal representation instead of a rendered value.

Hiding a value behind *any* function call is enough to fall off the path:

| program | native | interpreter | verdict |
|---|---|---|---|
| `print (Just 3)` | `Just 3` | `Just 3` | ok — syntactic constructor |
| `print (id R)`, `data C = R \| G deriving Show` | **`82`** | `R` | **WRONG** — raw tag/number |
| `print ((1,2) :: (Int,Int))` | **`$tuple2 1 2`** | `(1,2)` | **WRONG** — internal name leaks |
| `putStrLn (show [1,2,3])` | `[1,2,3]` | `[1,2,3]` | ok — literal |
| `putStrLn (show (filter odd [1,2,3,4,5]))` | **`4301791409`** | `[1,3,5]` | **WRONG** |
| `print (id [1,2,3])` | **`52932119073`** | `[1,2,3]` | **WRONG** — even `id` defeats it |
| `show (1 + 1)` | `2` | `2` | ok — computed Int is fine |

Three distinct wrong renderings — a heap pointer, a raw constructor tag, and the internal
name `$tuple2` — all from the same cause: no working generic `show` on the native path.
`print (id R)` is the cleanest proof, since `id` changes nothing about the value and only
hides it from a syntactic matcher.

This is very likely the same root as the known, already-documented gap that "native
class-method dispatch is still catching up to the STG interpreter" — `show` is a class
method, and rendering a list needs `Show [a]` from `Show a`. What is newly documented here
is not that the gap exists but that **it fails silently and plausibly** rather than erroring.

## Scope — first narrowing (lists), kept for the record

| program | native | interpreter |
|---|---|---|
| `print (1 + 1)` | `2` | `2` — **ok** |
| `print [1,2,3]` (literal list) | `[1,2,3]` | `[1,2,3]` — **ok** |
| `print (take 3 [1..])` | `53234108865` | `[1,2,3]` — **WRONG** |
| `print (filter odd [1,2,3,4,5])` | `4309262449` | `[1,3,5]` — **WRONG** |
| `print (map (*2) [1,2,3])` | `4350563985` | `[2,4,6]` — **WRONG** |
| `print (length (filter odd [1,2,3,4,5]))` | `3` | `3` — **ok** |

The last row is the important one. `length (filter odd …)` is **correct** natively, so the
list is being **computed correctly** — the cons cells are there and countable. Only
rendering it via `print`/`show` is broken, and what gets printed is the pointer to the list
rather than the list.

Literal lists work, so there is a path that handles a statically-known list and a different
path that does not handle a runtime-constructed one.

## Why this matters more than its size suggests

- It is the **first thing a new user hits**. "Compile a Haskell program that prints a list"
  is roughly the second program anyone writes.
- It is **silent**. A crash would be safer; this returns a plausible-looking integer.
- It is **not stably wrong** — the value differs per run, so a naive golden-output test
  would flag it as flaky rather than wrong.
- It means "compiles to native code" and "compiled native code produces the right output"
  are currently different claims for anything list-shaped. Any public statement about the
  backends should not conflate them.

## Not to be confused with the other two open findings

- [[bug-comprehension-letrec-capture-chirho]] — a wrong *branch* taken in a comprehension
  guard. That one is on the **STG interpreter** path and is NOT what this is.
- [[bug-nondeterministic-typecheck-chirho]] — unstable accept/reject at **compile time**.
  Different phase entirely.

Three distinct defects, three different phases. They should be reported separately.

## Next step

Find where `show`/`print` dispatches for a list in the native runtime and compare it with
the literal-list path that works. `length` working proves the data is intact, so this is a
rendering/dispatch bug, not a codegen or GC bug — which should make it considerably easier
to fix than the other two.
