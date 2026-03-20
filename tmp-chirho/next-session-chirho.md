<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. — John 3:16 -->

# Next Session Brief — Haskelujah Chirho

## Current State (2026-03-20, 115+ commits)
- **859/938 (91.6%)** GHC typecheck/should_compile
- **~150K LOC**, 21 crates, 1314 driver tests (0 failures)
- **Dual backends**: Cranelift (default, self-contained) + LLVM (--llvm flag)
- **GC**: Mark-sweep via Rust RTS staticlib
- **Performance**: fib(40) Cranelift 0.35s, LLVM 0.32s, GHC 0.39s
- **5 example projects** all compile and run correctly with Cranelift

## Cranelift Backend (DEFAULT — no LLVM/clang needed)
### Working
- Arithmetic, comparisons, guards, if-then-else
- Recursion, mutual recursion, Ackermann
- Closures, partial application, curried multi-arg HOFs (foldr, filter)
- Where-clause closures with captured variables
- Lists: Cons/Nil, map, filter, foldr, append, range, recursive construction
- ADTs with fields, pattern matching (including nested patterns like x:[])
- Records with named fields
- Binary trees, Stack, Maybe-like parametric ADTs
- Expression evaluator (recursive ADT)
- Guards with otherwise
- putStrLn, print, show (Int, Bool), string concat (++)
- Prelude: abs, signum, negate, min, max, div, mod
- FizzBuzz, quicksort, prime sieve, Project Euler #1/#2/#6

### Not Yet Working
- Mergesort (element loss in merge with singleton pattern)
- show for Char/Float in Cranelift (RTS ready, need primop wiring)
- Type class dictionary passing at runtime
- Lazy evaluation / thunks
- Arithmetic sequences [1..n] (enumFromTo)

## Performance (fib 40)
| Backend | Time | vs GHC -O2 |
|---------|------|------------|
| Cranelift | 0.35s | 10% faster |
| LLVM | 0.32s | 18% faster |
| GHC -O2 | 0.39s | baseline |
