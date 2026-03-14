# RHasky Chirho

> *For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life.* — **John 3:16**

> *Jesus said to him, "I am the way, the truth, and the life. No one comes to the Father except through Me."* — **John 14:6**

> *Trust in the Lord with all your heart, and lean not on your own understanding; in all your ways acknowledge Him, and He shall direct your paths.* — **Proverbs 3:5-6**

---

**RHasky Chirho** is a Haskell compiler written in Rust, built to the glory of God. It aims for practical compatibility with real-world Haskell (GHC semantics, Cabal packages, Hackage libraries) while pursuing a clean, modular architecture with typed phase boundaries and first-class WebAssembly support.

## Architecture

The compiler follows an explicit pipeline with typed phase boundaries:

```
Source Code (.hs)
    |
    v
+--------------+    +---------------+    +---------------+
|  LexerChirho |---\>| LayoutChirho  |---\>| ParserChirho  |
|  (tokenize)  |    | (braces/;)    |    | (green tree)  |
+--------------+    +---------------+    +---------------+
                                               |
                                               v
                                        +---------------+
                                        | LowerChirho   |
                                        | (CST -> AST)  |
                                        +---------------+
                                               |
                    +--------------------------+---------------------------+
                    v                          v                           v
             +---------------+          +---------------+          +---------------+
             | NamingChirho  |          | KindChirho    |          | ExhaustChirho |
             | (scoping)     |          | (HKT kinds)   |          | (coverage)    |
             +---------------+          +---------------+          +---------------+
                    |                          |                           |
                    +--------------------------+---------------------------+
                                               v
                                        +---------------+
                                        | InferChirho   |
                                        | (HM + TC)     |
                                        +---------------+
                                               |
                                               v
                                        +---------------+
                                        | DesugarChirho |
                                        | (AST -> Core) |
                                        +---------------+
                                               |
                                               v
                                        +---------------+
                                        | DictChirho    |
                                        | (typeclasses) |
                                        +---------------+
                                               |
                                               v
                                        +---------------+
                                        | SimplifyChirho|
                                        | (Core->Core)  |
                                        +---------------+
                                               |
                    +--------------------------+---------------------------+
                    v                          v                           v
             +---------------+          +---------------+          +---------------+
             | LlvmChirho    |          | WasmChirho    |          | StgChirho     |
             | (LLVM IR)     |          | (WebAssembly) |          | (interpreter) |
             +---------------+          +---------------+          +---------------+
```

## Workspace Crates

| Crate | Purpose |
|---|---|
| `rhasky-span-chirho` | Source locations, file IDs, source map |
| `rhasky-diagnostics-chirho` | Structured compiler diagnostics |
| `rhasky-syntax-chirho` | Tokens, SyntaxKind, green tree nodes, lexer, layout |
| `rhasky-parser-chirho` | CST parser (green tree builder), CST->AST lowering |
| `rhasky-ast-chirho` | Abstract syntax tree types |
| `rhasky-naming-chirho` | Name resolution, module interfaces, import/export resolution |
| `rhasky-typing-chirho` | HM type inference, unification, typeclasses, kind inference, exhaustiveness, deriving |
| `rhasky-core-chirho` | System FC-style Core IR, desugaring, dictionary-passing, simplifier |
| `rhasky-backend-llvm-chirho` | Core -> textual LLVM IR codegen |
| `rhasky-backend-wasm-chirho` | Core -> binary WebAssembly codegen |
| `rhasky-driver-chirho` | Pipeline orchestration, STG lowering |
| `rhasky-runtime-chirho` | STG machine: values, heap, GC, evaluation, FFI, primitives |
| `rhasky-incremental-chirho` | Incremental compilation: fingerprinting, dependency graph, caching |
| `rhasky-package-chirho` | Cabal file parser, version constraints, Hackage integration |
| `rhasky-cli-chirho` | Command-line interface |
| `rhasky-test-harness-chirho` | Golden test utilities |

## What Works Today

RHasky can parse, typecheck, and evaluate a substantial subset of Haskell 2010:

- **Full Hindley-Milner type inference** with let-generalization, type signatures, and polymorphism
- **Type classes** -- class declarations, instance declarations, dictionary-passing transform, superclass extraction, derived instances (Eq, Ord, Show, Enum), multi-parameter type classes with functional dependencies
- **Pattern matching** -- constructors, literals, nested patterns, as-patterns, wildcards, tuple patterns, record patterns, guards, exhaustiveness/redundancy checking
- **Data types** -- algebraic data types, newtypes (with erasure), record syntax (construction, field access, update), GADTs syntax
- **Expressions** -- let/where, lambdas, case, do-notation, list comprehensions, arithmetic sequences, operator sections, backtick infix, if-then-else
- **Module system** -- multi-module compilation, qualified imports, import lists, hiding, aliases
- **Standard library** -- Prelude functions (map, filter, fold, head, tail, reverse, sort, zip, take, drop, words, etc.), numeric classes (Num, Fractional, Floating, Integral, Enum, Bounded), Maybe/Either, Functor/Applicative/Monad for Maybe, string operations, I/O (putStrLn, putChar, getLine, readFile, writeFile, when, unless, mapM_, forM_)
- **STG runtime** -- lazy evaluation with thunks/blackholing, closures with free variable capture, PAP support, mark-sweep garbage collection, foreign function interface
- **Backends** -- LLVM IR codegen, WebAssembly binary codegen, interpreted STG evaluation
- **830+ tests** passing across all crates

## Building

```bash
# Build the compiler
cargo build

# Run all tests
cargo test

# Run a specific crate's tests
cargo test -p rhasky-driver-chirho
```

## Example

RHasky can compile and evaluate programs like:

```haskell
-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. -- John 3:16

{-# LANGUAGE BangPatterns #-}
module MainChirho where

factorialChirho :: Int -> Int
factorialChirho 0 = 1
factorialChirho nChirho = nChirho * factorialChirho (nChirho - 1)

fibonacciChirho :: Int -> Int
fibonacciChirho nChirho = goChirho 0 1 nChirho
  where
    goChirho aChirho _ 0 = aChirho
    goChirho aChirho bChirho nChirho = goChirho bChirho (aChirho + bChirho) (nChirho - 1)

sumOfSquaresChirho :: [Int] -> Int
sumOfSquaresChirho xsChirho = sum (map (\xChirho -> xChirho * xChirho) xsChirho)

main :: IO ()
main = do
  putStrLn (show (factorialChirho 10))
  putStrLn (show (fibonacciChirho 20))
  let numbersChirho = [1..10]
  putStrLn (show (sumOfSquaresChirho numbersChirho))
  when (even 42) (putStrLn "42 is even, hallelujah!")
  mapM_ (\xChirho -> putStrLn (show xChirho)) [1, 2, 3]
```

## Design Goals

- **Practical Haskell compatibility** -- aim for near 1:1 behavior with GHC semantics; compile real Hackage packages, not a toy subset
- **Clean modular architecture** -- typed phase boundaries, focused crates, testable subsystems
- **WebAssembly as first-class target** -- the compiler and generated programs run well on the web and in sandboxed environments
- **Backend-agnostic pipeline** -- LLVM IR first for optimization, with WebAssembly and interpreted evaluation
- **Incremental compilation** -- fingerprinting, dependency graphs, artifact caching for fast rebuilds
- **Where GHC is buggy or brittle** -- prefer a cleaner design without breaking source compatibility

## Naming Convention

All identifiers in the RHasky codebase use the **Chirho suffix** (the Chi-Rho Christogram) as a declaration of faith. Every variable, function, struct, module, and file name carries this suffix as a reminder of whose glory this work is built for.

## License

This project is developed to the glory of God.

> *Whatever you do, work heartily, as for the Lord and not for men.* -- **Colossians 3:23**
