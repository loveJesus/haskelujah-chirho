<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 -->

# Haskelujah Chirho

> *"For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life."* — John 3:16

Haskelujah Chirho is everything you need to develop, build, and ship Haskell — compiler, package manager, script runner, REPL, test runner, LSP, and AI integration, all in one binary. Written in Rust, targeting GHC compatibility with Cranelift (default), LLVM, and WebAssembly backends.

**All-in-one toolchain:**

| Tool | Command | What it does |
|---|---|---|
| Compiler | `haskelujah build` | Parse `.cabal`, compile modules, produce native executables |
| Package Manager | `haskelujah install aeson` | Fetch from Hackage, resolve deps, extract `.cabal` metadata |
| Script Runner | `#!/usr/bin/env haskelujah` | Run `.hs` files directly with shebang |
| REPL | `haskelujah repl` | Interactive `:type`, `:info`, `:load`, expression eval |
| Test Runner | `haskelujah test` | Compile and run test suites |
| LSP Server | `haskelujah lsp` | Real-time diagnostics, hover types for VS Code, Neovim, Zed |
| AI / MCP | `haskelujah mcp` | Expose project to AI assistants via Model Context Protocol |
| Checker | `haskelujah check` | Type-check without codegen |
| Formatter | `haskelujah fmt` | Trim whitespace, normalize indentation, clean blank lines |
| Editor | `haskelujah edit` | Auto-detects GUI/TUI. `--gui` for cross-platform GUI, `--cli` for terminal |

**Batteries-included standard library** — no `haskelujah install` needed:

| Module | What it does |
|---|---|
| `Haskelujah.JSON` | Encode/decode JSON values |
| `Haskelujah.Test` | Test framework (assertEqual, runTests) |
| `Haskelujah.Args` | CLI argument parsing (getFlag, getOption) |
| `Haskelujah.HTTP` | HTTP client |
| `Haskelujah.Prelude` | Extended prelude (trim, chunksOf, nub') |
| `Haskelujah.File` | File utilities |
| `Haskelujah.Text` | toLower, toUpper, splitOn, contains, padLeft |
| `Haskelujah.Map` | Association-list map (insert, lookup, union) |
| `Haskelujah.Set` | Sorted-list set (insert, member, intersection) |
| `Haskelujah.Pretty` | Pretty printer (nest, hsep, vsep, render) |
| `Haskelujah.Random` | Pseudo-random numbers (nextInt, shuffle, choice) |
| `Haskelujah.Process` | Shell/process execution |
| `Haskelujah.Time` | Time utilities (now, sleep, measure) |

## Status

| Metric | Value |
|---|---|
| GHC Compat | **833/938 (88.8%)** typecheck/should_compile |
| Curated Tests | **537/537 (100%)** compile-and-run correctness tests |
| Total Tests | **2,500+ passing**, 0 failures |
| Hackage Packages | **27 compile** including **transformers** + **mtl** + **parsec** + **deepseq** + **vector** + **HUnit** + **contravariant** |
| Workspace | 25 crates (compiler + MCP + LSP + GUI + editor + runtime) |
| Codebase | ~174,000 lines of Rust |
| Module interfaces | 375 synthetic Haskell modules |
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

### Install

```bash
# From crates.io
cargo install haskelujah

# From git (latest)
cargo install --git https://github.com/loveJesus/haskelujah-chirho haskelujah

# From source
cargo install --path crates/haskelujah-cli-chirho
```

### Build from source

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
| Type checking | Reference | 88.8% compatible (833/938) |
| Compilation speed | ~1-5s for small files | ~0.2-0.4s |
| Runtime (fib 42) | 1.26s (-O2) | 0.91s CL/LLVM (**38% faster**) |
| Ackermann(3,11) | 0.26s | 0.62s CL, 1.03s LLVM |
| Quicksort 1K | 0.22s | 0.20s CL, **0.15s LLVM** |
| Prime sieve 10K | 0.28s | 0.59s CL, **0.18s LLVM** |
| Tail call optimization | Full | Both backends (100M+ iterations) |
| Hackage packages | cabal-install | `haskelujah install` — **16+ packages compile** (transformers, mtl, parsec, comonad, semigroupoids) |
| Native code | Via NCG or LLVM | Cranelift (default) or LLVM + clang |
| WebAssembly | Via Asterius/GHCJS | Built-in (beta) |
| Package manager | cabal-install / Stack | Built-in `build` command |
| Project scaffold | `cabal init` | `haskelujah init` |
| REPL | GHCi | `haskelujah repl` |
| Lazy evaluation | Full | **Lazy** (Cranelift + LLVM + STG) — infinite lists work! |
| Garbage collection | Generational GC | **Mark-sweep GC active** (Rust RTS staticlib) |
| Type classes | Full dictionary passing | Type checking OK, runtime partial |
| GADTs | Full | 91.5% type checking, compilation for simple cases |
| Template Haskell | Full | Partial (makeLenses works) |
| FFI | Full C interop | Basic libc (puts, printf, malloc) |

### Working Program Examples

The `examples-chirho/` directory contains verified working programs:

- **showcase**: Multi-module demo — fibonacci, collatz, GCD, primes, quicksort, foldl, filter
- **stats**: Statistics calculator — mean, variance, min, max, filter with IO report
- **todo**: Todo list manager — Priority ADT, list counting, string formatting
- **number-converter**: Interactive number converter — binary, octal, hex, digit sum
- **hello-world**: Fibonacci, Collatz, Euler #1, closures, list operations
- **multi-module**: Cross-module imports with library
- **expr-eval**: Recursive algebraic expression evaluator
- **bst**: Binary search tree with insertion and traversal
- **euler**: Project Euler #1, #2, #6 with correct answers

All 9 examples produce correct output on both Cranelift and LLVM backends.

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

- **Lazy evaluation**: Both Cranelift and LLVM backends have lazy constructor fields — infinite lists like `take 5 (repeat 42)` work! STG interpreter has full laziness.
- **Garbage collection**: Mark-sweep GC is active (threshold 1000 allocations). Root tracking via `gc_root_push` at allocation sites.
- **Type class dicts at runtime**: Type checking supports full typeclasses; compiled code uses simplified dictionary elision. Complex polymorphic dispatch is partial.
- **String as [Char]**: String literals are C strings internally. `unpack`/`pack` works but isn't transparent.
- **Template Haskell**: Basic splices and `makeLenses` work; full TH is incomplete.
- **FFI**: Basic libc interop. Full C header parsing not yet implemented.

## Roadmap

- **Lazy let-bindings** — thunkify non-recursive let-bound expressions for full GHC semantics
- **Lazy let-bindings** — thunkify non-recursive let-bound expressions for full GHC semantics
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
