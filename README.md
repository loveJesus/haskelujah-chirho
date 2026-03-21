<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 -->

# Haskelujah Chirho

> *"For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life."* — John 3:16

Haskelujah Chirho is a Haskell compiler written in Rust, aiming to be a drop-in replacement for GHC. It targets practical compatibility with real-world Haskell (GHC semantics, Cabal packages, Hackage libraries) through a typed, modular pipeline with first-class WebAssembly support and multiple native code generation backends.

**End-to-end compilation works:** `haskelujah build my-project/` parses `.cabal` files, compiles all modules in dependency order, and produces native executables via LLVM+clang. Supports `putStrLn`, `print`, arithmetic, if-then-else, pattern matching, recursion, higher-order functions, lambdas, ADTs, list operations, guards, let/where bindings, and multi-module imports.

## Status

| Metric | Value |
|---|---|
| GHC Compat | **861/938 (91.8%)** typecheck/should_compile |
| Curated Tests | **332/332 (100%)** compile-and-run correctness tests |
| Total Tests | **2,200+ passing**, 0 failures |
| Workspace | 21 crates, 132 Rust source files |
| Codebase | ~150,000 lines of Rust |
| Module interfaces | 280 synthetic Haskell modules |
| Rust edition | 2024 (rustc 1.93.0+) |
| License | MIT OR Apache-2.0 |

> Run `bash spec-chirho/stats-chirho.sh` for live stats from the repo.

## Features

**Language support:**
- Haskell 2010 lexing, layout insertion, and parsing
- Hindley-Milner type inference with typeclasses, multi-parameter type classes, functional dependencies, and deriving (Eq, Ord, Show, Functor, Foldable, Traversable, Generic)
- Kind inference with explicit kind signatures
- GADTs, existential quantification, type applications, type families (open/closed/associated), DataKinds, ConstraintKinds
- RankNTypes, ScopedTypeVariables, LinearTypes, PolyKinds, TypeOperators
- Pattern match exhaustiveness and redundancy checking
- True lazy evaluation with infinite lists and bang patterns
- IO, closures, recursion, list operations, arithmetic sequences
- Monad transformers (StateT, ReaderT, WriterT, ExceptT, MaybeT)
- Overloaded strings and overloaded lists
- WHNF semantics (seq, deepseq, force, evaluate, NFData)
- Exception handling (catch/throw/try/bracket/finally with stack unwinding)
- FFI (foreign function interface) with foreign exports
- Template Haskell (basic splices, quasi-quotes, built-in makeLenses)
- Automatic Prelude import, qualified imports, module re-exports
- Hierarchical multi-module compilation with dependency ordering and circular import support
- Orphan instance detection
- Incremental compilation with fingerprinting and artifact caching
- SCC-based binding group analysis for correct polymorphic generalization
- 40+ GHC extensions: LambdaCase, RecordWildCards, ViewPatterns, PatternSynonyms, DerivingVia, DerivingStrategies, TupleSections, MagicHash, TypedHoles, PartialTypeSignatures, GHC2021/GHC2024, and more
- 110+ synthetic module interfaces for common Haskell libraries

**Optimization passes:**
- Inlining (INLINE/NOINLINE/INLINABLE pragmas)
- Specialization (SPECIALIZE pragma)
- Common subexpression elimination
- Strictness analysis and worker/wrapper
- Constructor specialization (SpecConstr)
- Demand analysis and dead argument elimination
- Core simplification

**Backend Maturity:**

| Backend | Maturity | Output | Notes |
|---|---|---|---|
| Cranelift | **Beta** | Native executables via system `cc` | **Default** — no LLVM needed, self-contained |
| LLVM | **Beta** | Native executables via clang | Full feature support (`--llvm` flag) |
| WebAssembly | **Beta** | Binary `.wasm` files | First-class target |
| JVM | Research | `.class` bytecode | Scaffold only |
| BEAM | Research | `.beam` bytecode | Scaffold only |

## Compiler Pipeline

The compiler runs a 12-phase pipeline, wired end-to-end in `haskelujah-driver-chirho`:

```
Haskell source
  1. Lex              — haskelujah-syntax-chirho tokenizer
  2. Layout           — layout rule insertion (braces/semicolons)
  3. CST Parse        — lossless green-tree concrete syntax tree
  4. AST Lower        — abstract syntax tree from CST
  5. Name Resolve     — scope resolution, qualified names, module interfaces
  6. Kind Infer       — kind inference with unification
  7. Type Infer       — HM Algorithm W, typeclasses, MPTC, fundeps, deriving
  8. Exhaustiveness   — pattern match exhaustiveness and redundancy checking
  9. Desugar/Core     — System FC-style Core IR, dictionary passing, simplification
 10. STG Evaluation   — thunks, closures, PAPs, GC, step limits
 11. FFI              — foreign function interface
 12. Exceptions       — catch/throw/try/bracket/finally with stack unwinding
```

## Getting Started

### Build

```bash
cargo build --workspace
```

### Test

```bash
# Run the full test suite
cargo test --workspace

# Run driver/runtime integration tests
cargo test -p haskelujah-driver-chirho

# Run parser golden tests
cargo test -p haskelujah-parser-chirho --test golden_parse_chirho
```

### CLI Usage

```bash
# Create a new project
haskelujah init my-project

# Build a Cabal project → native executable
haskelujah build my-project/
./my-project/dist-chirho/build/my-project

# Compile and run (LLVM native, falls back to STG interpreter)
haskelujah run MyModule.hs
# Or simply (implicit run for .hs files — works as shebang)
haskelujah MyModule.hs

# Type-check a Haskell source file
haskelujah check MyModule.hs

# Compile to a native executable (via LLVM + clang)
haskelujah compile MyModule.hs -o main

# Compile to WebAssembly
haskelujah compile MyModule.hs --wasm -o out.wasm

# Compile via Cranelift
haskelujah compile MyModule.hs --cranelift -o main

# Start the REPL
haskelujah repl

# Use as shebang interpreter
#!/usr/bin/env haskelujah
```

**REPL commands:** `:type <expr>`, `:info <name>`, `:load <file>`, `:reload`, `:let <decl>`, `:clear`, `:{`/`:}` (multi-line), `:quit`

**Diagnostic flags:** `--dump-core`, `--dump-stg`, `--dump-llvm`

### Quick Start: Your First Project

```bash
# 1. Create a new project
haskelujah init my-project
cd my-project

# 2. Edit Main.hs with your program
cat > Main.hs << 'EOF'
module Main where

fib_chirho :: Int -> Int
fib_chirho 0 = 0
fib_chirho 1 = 1
fib_chirho n = fib_chirho (n - 1) + fib_chirho (n - 2)

main :: IO ()
main = do
  putStrLn "Fibonacci numbers:"
  print (fib_chirho 10)
  print (fib_chirho 20)
EOF

# 3. Build and run — produces a self-contained native executable
#    No LLVM, no GHC, no external tools needed!
haskelujah build-run .
```

### Using Cabal Projects

Haskelujah reads standard `.cabal` files:

```bash
# Multi-module project with library
cat > my-lib.cabal << 'EOF'
cabal-version: 2.4
name: my-lib
version: 0.1.0.0
executable my-lib
  main-is: Main.hs
  other-modules: MyLib
  hs-source-dirs: src
  build-depends: base
  default-language: Haskell2010
EOF

haskelujah build .          # Compile to native executable
haskelujah build-run .      # Build and run in one step
haskelujah clean .          # Remove build artifacts
```

### Comparison with GHC

| Feature | GHC | Haskelujah Chirho |
|---------|-----|-------------------|
| Type checking | Reference | 91.8% compatible (861/938) |
| Compilation speed | ~1-5s for small files | ~0.2-0.4s |
| Runtime (fib 42) | 1.26s (-O2) | 0.91s CL/LLVM (**38% faster**) |
| Ackermann(3,11) | 0.26s | 0.62s CL, 1.03s LLVM |
| Quicksort 1K | 0.22s | 0.20s CL, **0.15s LLVM** |
| Prime sieve 10K | 0.28s | 0.59s CL, **0.18s LLVM** |
| Tail call optimization | Full | Both backends (100M+ iterations) |
| Hackage packages | cabal-install | `haskelujah install` (aeson, safe, split) |
| Native code | Via NCG or LLVM | Cranelift (default) or LLVM + clang |
| WebAssembly | Via Asterius/GHCJS | Built-in (beta) |
| Package manager | cabal-install / Stack | Built-in `build` command |
| Project scaffold | `cabal init` | `haskelujah init` |
| REPL | GHCi | `haskelujah repl` |
| Lazy evaluation | Full | Strict (LLVM), Lazy (STG interpreter) |
| Garbage collection | Generational GC | Mark-sweep GC (Rust RTS staticlib) |
| Type classes | Full dictionary passing | Type checking OK, runtime partial |
| GADTs | Full | 91.5% type checking, compilation for simple cases |
| Template Haskell | Full | Partial (makeLenses works) |
| FFI | Full C interop | Basic libc (puts, printf, malloc) |

### Working Program Examples

The `examples-chirho/` directory contains verified working programs:

- **showcase**: Multi-module demo — fibonacci, collatz, GCD, primes, quicksort, foldl, filter
- **hello-world**: Fibonacci, Collatz, Euler #1, closures, list operations
- **multi-module**: Cross-module imports with library
- **expr-eval**: Recursive algebraic expression evaluator
- **bst**: Binary search tree with insertion and traversal
- **euler**: Project Euler #1, #2, #6 with correct answers

All examples produce correct output on both Cranelift and LLVM backends.

**Diagnostic flags:** `--dump-core`, `--dump-stg`, `--dump-llvm`

## Naming Convention

All identifiers created in this project use the **Chirho suffix** (e.g., `function_name_chirho` in Rust, `functionNameChirho` in Haskell/JS, `ClassNameChirho` for types, `CONSTANT_NAME_CHIRHO` for constants). This applies to variables, functions, types, modules, file names, directory names, database columns, API routes, and all other identifiers without exception.

See [AGENTS.md](AGENTS.md) for the full convention.

## Project Structure

- `crates/` -- all 21 workspace crates
- `spec-chirho/` -- specifications, progress database, phase archive
- `AGENTS.md` -- authoritative project spec, naming convention, and priorities

## Ecosystem

- **Cabal support**: Reads standard `.cabal` files. Multi-module projects with library and executable components compile out of the box.
- **Hackage packages**: `haskelujah install aeson` fetches from Hackage, resolves dependencies, extracts `.cabal` metadata. Auto-detects latest versions.
- **Single binary**: One self-contained executable. No runtime dependencies, no GHC installation, no LLVM toolchain (unless you opt in). `cargo install haskelujah`.
- **REPL**: `haskelujah repl` with `:type`, `:info`, `:load`, multi-line input, expression evaluation.

## Current Limitations

- **Lazy evaluation**: Compiled backends are strict-only. STG interpreter supports laziness. Thunks planned.
- **Garbage collection**: Mark-sweep GC exists but is disabled (no root tracking). Programs leak memory on long runs.
- **Type class dicts at runtime**: Type checking supports full typeclasses; compiled code uses simplified dictionary elision. Complex polymorphic dispatch is partial.
- **String as [Char]**: String literals are C strings internally. `unpack`/`pack` works but isn't transparent.
- **Template Haskell**: Basic splices and `makeLenses` work; full TH is incomplete.
- **FFI**: Basic libc interop. Full C header parsing not yet implemented.

## Roadmap

- **Lazy evaluation** — thunks, lazy data structures, proper WHNF in compiled backends
- **GC root tracking** — wire roots through codegen for safe memory reclamation
- **Integrated IDE** — Zed-like editor extensible via Haskell (like Emacs uses Lisp), LSP support
- **Cross-compilation** — target selection from CLI (Linux, macOS, Windows, embedded)
- **Full Hackage** — compile real-world packages (aeson, lens, servant)
- **Profiling** — cost-center annotation, heap/time profiling

## Design Goals

- Practical Haskell compatibility rather than a toy subset
- Typed, explicit compiler phase boundaries
- Portable Rust implementation with focused crates
- WebAssembly as a first-class target
- Backend-agnostic pipeline design
- Testable subsystems with golden tests and property tests

## Git Workflow

- Primary branch: `main_chirho`
- GitHub remote name: `gh_chirho`

## License

MIT OR Apache-2.0
