<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 -->

# RHasky Chirho

> *"For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life."* — John 3:16

RHasky Chirho is a Haskell compiler written in Rust. It targets practical compatibility with real-world Haskell (GHC semantics, Cabal packages, Hackage libraries) through a typed, modular pipeline with first-class WebAssembly support and multiple native code generation backends.

## Status

| Metric | Value |
|---|---|
| Tests | **1,536 passing**, 0 failures |
| Workspace | 19 crates |
| Codebase | ~100,000 lines of Rust |
| Rust edition | 2024 (rustc 1.93.0+) |
| License | MIT OR Apache-2.0 |

## Features

**Language support:**
- Haskell 2010 lexing, layout insertion, and parsing
- Hindley-Milner type inference with typeclasses, multi-parameter type classes, functional dependencies, and deriving (Eq, Ord, Show, Functor, Foldable, Traversable)
- Kind inference with explicit kind signatures
- Pattern match exhaustiveness and redundancy checking
- True lazy evaluation with infinite lists and bang patterns
- IO, closures, recursion, list operations, arithmetic sequences
- Monad transformers (StateT, ReaderT, WriterT, ExceptT, MaybeT)
- Overloaded strings and overloaded lists
- Existential quantification, type applications, default signatures
- WHNF semantics (seq, deepseq, force, evaluate, NFData)
- Exception handling (catch/throw/try/bracket/finally with stack unwinding)
- FFI (foreign function interface)
- Automatic Prelude import, qualified imports, module re-exports
- Hierarchical multi-module compilation with dependency ordering
- Orphan instance detection
- Incremental compilation with fingerprinting and artifact caching

**Optimization passes:**
- Inlining (INLINE/NOINLINE/INLINABLE pragmas)
- Specialization (SPECIALIZE pragma)
- Common subexpression elimination
- Core simplification

**Backends:**
- LLVM IR (native executables via clang)
- WebAssembly (binary .wasm output)
- Cranelift (native object files: x86_64, aarch64, s390x, riscv64)
- JVM bytecode (.class files) -- experimental
- BEAM bytecode (.beam files) -- experimental

## Compiler Pipeline

The compiler runs a 12-phase pipeline, wired end-to-end in `haskeluya-driver-chirho`:

```
Haskell source
  1. Lex              — haskeluya-syntax-chirho tokenizer
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

The driver exposes a shared frontend runner (`run_frontend_chirho`), so check, compile, and multi-module flows all execute the same analysis phases.

## Workspace Crates

### Foundation

| Crate | Purpose |
|---|---|
| `haskeluya-span-chirho` | Source locations, file IDs, source maps, span arithmetic |
| `haskeluya-diagnostics-chirho` | Structured diagnostics with codes, labels, suggestions, ANSI color rendering |
| `haskeluya-syntax-chirho` | Token kinds, syntax kinds, green tree data structures, lexer, layout |
| `haskeluya-test-harness-chirho` | Golden test helpers and snapshot utilities |

### Frontend

| Crate | Purpose |
|---|---|
| `haskeluya-parser-chirho` | CST parser (green tree builder), golden tests, property tests |
| `haskeluya-ast-chirho` | AST data types and CST-to-AST lowering |
| `haskeluya-naming-chirho` | Scopes, imports, module interfaces, qualified names, orphan-instance warnings |
| `haskeluya-typing-chirho` | Kind inference, HM type inference, typeclasses, deriving, exhaustiveness checking |

### Middle End and Runtime

| Crate | Purpose |
|---|---|
| `haskeluya-core-chirho` | Core IR (System FC-style), desugaring, dictionary passing, simplification, pretty-printing |
| `haskeluya-runtime-chirho` | STG machine: heap, stack, values, primops, GC, evaluator, FFI, exception handling |
| `haskeluya-driver-chirho` | Pipeline orchestration, shared frontend, compilation coordination |

### Backends

| Crate | Purpose |
|---|---|
| `haskeluya-backend-llvm-chirho` | Core to textual LLVM IR, native executable path |
| `haskeluya-backend-wasm-chirho` | Core to binary WebAssembly |
| `haskeluya-backend-cranelift-chirho` | Native backend via Cranelift (x86_64, aarch64, s390x, riscv64) |
| `haskeluya-backend-jvm-chirho` | JVM .class bytecode (experimental) |
| `haskeluya-backend-beam-chirho` | BEAM .beam bytecode (experimental) |

### Packaging and Tooling

| Crate | Purpose |
|---|---|
| `haskeluya-package-chirho` | Cabal file parsing, version constraints, Hackage URL construction |
| `haskeluya-incremental-chirho` | Fingerprinting, dependency graph, artifact caching, recompilation avoidance |
| `haskeluya-cli-chirho` | Command-line interface and REPL |

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
cargo test -p haskeluya-driver-chirho

# Run parser golden tests
cargo test -p haskeluya-parser-chirho --test golden_parse_chirho
```

### CLI Usage

```bash
# Type-check a Haskell source file
cargo run -p haskeluya-cli-chirho -- check examples-chirho/MainChirho.hs

# Evaluate via the STG interpreter
cargo run -p haskeluya-cli-chirho -- run examples-chirho/MainChirho.hs

# Compile to a native executable (via LLVM)
cargo run -p haskeluya-cli-chirho -- compile examples-chirho/MainChirho.hs -o main-chirho

# Compile to WebAssembly
cargo run -p haskeluya-cli-chirho -- compile examples-chirho/MainChirho.hs --wasm -o out-chirho.wasm

# Compile via Cranelift
cargo run -p haskeluya-cli-chirho -- compile examples-chirho/MainChirho.hs --cranelift -o main-chirho

# Build a multi-module project
cargo run -p haskeluya-cli-chirho -- build examples-chirho/project-chirho

# Start the REPL
cargo run -p haskeluya-cli-chirho -- repl
```

**REPL commands:** `:type <expr>`, `:info <name>`, `:load <file>`, `:reload`, `:let <decl>`, `:clear`, `:{`/`:}` (multi-line), `:quit`

**Diagnostic flags:** `--dump-core`, `--dump-stg`, `--dump-llvm`

## Naming Convention

All identifiers created in this project use the **Chirho suffix** (e.g., `function_name_chirho` in Rust, `functionNameChirho` in Haskell/JS, `ClassNameChirho` for types, `CONSTANT_NAME_CHIRHO` for constants). This applies to variables, functions, types, modules, file names, directory names, database columns, API routes, and all other identifiers without exception.

See [AGENTS.md](AGENTS.md) for the full convention with language-specific examples.

## Project Structure

- `crates/` -- all 19 workspace crates
- `spec-chirho/` -- specifications, progress database, phase archive
- `examples-chirho/` -- example Haskell source files
- `AGENTS.md` -- authoritative project spec, naming convention, and phase priorities
- `codex-analysis-chirho.md` -- engineering review of the repository

## Current Development

The project completed Phase 1 (128 priorities) and is in **Phase 2**, which focuses on making the compiler practical:

- **Done:** true lazy evaluation, lazy I/O, bang patterns, WHNF semantics (deepseq/force/evaluate/NFData), automatic Prelude import, qualified imports, module re-exports, orphan instance detection, hierarchical multi-module compilation, LLVM/WebAssembly/Cranelift backend revival, CLI (compile/run/repl/check/build), structured error messages, inlining/specialization/CSE optimization, existential quantification, type applications, overloaded strings/lists, kind signatures, default signatures, monad transformers, property-based testing, backend round-trip smoke tests
- **In progress:** STM, circular module imports, shared RTS library, type families, DataKinds, ConstraintKinds, DeriveGeneric, Template Haskell, foreign exports, full Cabal parsing, Hackage integration, dependency resolution, strictness analysis, demand analysis, benchmark suite, GHC test suite integration, Haskell Report conformance

See [AGENTS.md](AGENTS.md) for the full Phase 2 priority list with detailed status.

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
