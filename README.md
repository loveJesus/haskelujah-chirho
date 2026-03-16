<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 -->

# Haskeluya

> *"For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life."* — John 3:16

Haskeluya is a Haskell compiler written in Rust. It targets practical compatibility with real-world Haskell (GHC semantics, Cabal packages, Hackage libraries) through a typed, modular pipeline with first-class WebAssembly support and multiple native code generation backends.

## Status

| Metric | Value |
|---|---|
| Tests | **1,816 passing**, 0 failures |
| Workspace | 21 crates |
| Codebase | ~100,000 lines of Rust |
| Rust edition | 2024 (rustc 1.93.0+) |
| License | MIT OR Apache-2.0 |

## Features

**Language support:**
- Haskell 2010 lexing, layout insertion, and parsing
- Hindley-Milner type inference with typeclasses, multi-parameter type classes, functional dependencies, and deriving (Eq, Ord, Show, Functor, Foldable, Traversable, Generic)
- Kind inference with explicit kind signatures
- GADTs, existential quantification, type applications, type families, DataKinds, ConstraintKinds
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

**Optimization passes:**
- Inlining (INLINE/NOINLINE/INLINABLE pragmas)
- Specialization (SPECIALIZE pragma)
- Common subexpression elimination
- Strictness analysis and worker/wrapper
- Constructor specialization (SpecConstr)
- Demand analysis and dead argument elimination
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
haskeluya check MyModule.hs

# Evaluate via the STG interpreter
haskeluya run MyModule.hs

# Compile to a native executable (via LLVM)
haskeluya compile MyModule.hs -o main

# Compile to WebAssembly
haskeluya compile MyModule.hs --wasm -o out.wasm

# Compile via Cranelift
haskeluya compile MyModule.hs --cranelift -o main

# Build a multi-module project
haskeluya build my-project/

# Start the REPL
haskeluya repl
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
