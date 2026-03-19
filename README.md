<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 -->

# Haskelujah Chirho

> *"For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life."* — John 3:16

Haskelujah Chirho is a Haskell compiler written in Rust, aiming to be a drop-in replacement for GHC. It targets practical compatibility with real-world Haskell (GHC semantics, Cabal packages, Hackage libraries) through a typed, modular pipeline with first-class WebAssembly support and multiple native code generation backends.

**End-to-end compilation works:** `haskelujah build my-project/` parses `.cabal` files, compiles all modules in dependency order, and produces native executables via LLVM+clang. Supports `putStrLn`, `print`, arithmetic, if-then-else, pattern matching, recursion, higher-order functions, lambdas, ADTs, list operations, guards, let/where bindings, and multi-module imports.

## Status

| Metric | Value |
|---|---|
| GHC Compat | **857/938 (91.4%)** typecheck/should_compile |
| Tests | **2,121 passing**, 0 failures |
| Workspace | 21 crates, 129 Rust source files |
| Codebase | ~137,200 lines of Rust |
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
| LLVM | **Beta** | Native executables via clang | Primary compilation target |
| WebAssembly | **Beta** | Binary `.wasm` files | First-class target |
| Cranelift | Experimental | Native objects (x86_64, aarch64, s390x, riscv64) | Fast compile times |
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
# Type-check a Haskell source file
haskelujah check MyModule.hs

# Evaluate via the STG interpreter
haskelujah run MyModule.hs

# Compile to a native executable (via LLVM + clang)
haskelujah compile MyModule.hs -o main

# Build a Cabal project (parses .cabal, compiles all modules, links executable)
haskelujah build my-project/

# Compile to WebAssembly
haskelujah compile MyModule.hs --wasm -o out.wasm

# Compile via Cranelift
haskelujah compile MyModule.hs --cranelift -o main

# Build a multi-module project
haskelujah build my-project/

# Start the REPL
haskelujah repl
```

**REPL commands:** `:type <expr>`, `:info <name>`, `:load <file>`, `:reload`, `:let <decl>`, `:clear`, `:{`/`:}` (multi-line), `:quit`

**Diagnostic flags:** `--dump-core`, `--dump-stg`, `--dump-llvm`

## Naming Convention

All identifiers created in this project use the **Chirho suffix** (e.g., `function_name_chirho` in Rust, `functionNameChirho` in Haskell/JS, `ClassNameChirho` for types, `CONSTANT_NAME_CHIRHO` for constants). This applies to variables, functions, types, modules, file names, directory names, database columns, API routes, and all other identifiers without exception.

See [AGENTS.md](AGENTS.md) for the full convention.

## Project Structure

- `crates/` -- all 21 workspace crates
- `spec-chirho/` -- specifications, progress database, phase archive
- `AGENTS.md` -- authoritative project spec, naming convention, and priorities

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
