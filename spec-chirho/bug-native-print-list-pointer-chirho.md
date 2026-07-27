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

## Scope — narrowed by experiment

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
