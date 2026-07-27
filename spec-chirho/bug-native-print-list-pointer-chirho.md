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

## SCOPE IS BROADER THAN THIS FILENAME — read this section

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
