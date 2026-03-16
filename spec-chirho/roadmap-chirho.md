<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Roadmap Chirho

## Milestone Chirho 0 — Workspace Bootstrap

Bootstrap the Rust workspace, span infrastructure, diagnostics foundation, and testing harness.

Acceptance criteria:

- workspace crate graph matches the architecture spec closely enough to evolve without a rewrite
- `haskelujah-span-chirho` provides file IDs, byte offsets, and span types
- `haskelujah-diagnostics-chirho` uses real spans, supports error codes and labeled secondary spans
- golden-test and property-test infrastructure exists in `haskelujah-test-harness-chirho`
- Git repository initialized on `main_chirho` with `gh_chirho` remote
- `progress-chirho.sqlite` tracking active

## Milestone Chirho 1 — Lexer, Parser, and Module Loading

Full Haskell 2010 lexer with layout rule, error-recovering parser producing a lossless red-green CST, basic Cabal file parsing, and module loading from the filesystem.

Acceptance criteria:

- lex all Haskell 2010 token types including string gaps, numeric literals, and qualified operators
- layout rule implemented via virtual brace/semicolon insertion at the lexer level
- parse non-trivial modules into a red-green lossless CST with source-preserving diagnostics
- typed AST wrappers over CST for convenient downstream consumption
- parse basic Cabal package descriptions (name, version, dependencies, exposed-modules)
- resolve module imports across a package-local file graph
- test corpus of valid and invalid Haskell inputs with golden tests

## Milestone Chirho 2 — Name Resolution, Kinds, and Typechecking

Name resolution across module graphs, kind inference/checking, Hindley-Milner type inference with Tier 1 extensions, type class resolution, and local package databases.

Acceptance criteria:

- resolve names across multi-module packages with proper import/export semantics
- build and validate module dependency graphs
- infer and check kinds for all type-level constructs
- type inference with Tier 1 extension support (OverloadedStrings, ScopedTypeVariables, FlexibleInstances, etc.)
- type class instance resolution and evidence elaboration (dictionary passing)
- local package databases for multi-package builds
- typecheck a curated library compatibility corpus
- unsupported features recorded precisely with structured diagnostics

## Milestone Chirho 3 — Core IR, Desugaring, and Script Execution

Desugaring pass, typed core IR, core-to-core optimization, STG-like runtime IR, tree-walking interpreter for script mode, and interface file serialization.

Acceptance criteria:

- desugar do-notation, list comprehensions, guards, pattern match compilation, and where clauses into core
- lower typed programs into a runtime-oriented STG-like IR
- execute simple programs in script mode via a tree-walking interpreter without full native codegen
- serialize and load interface files for separate compilation
- reuse package and interface caches across repeated script runs
- support rewrite rules in the simplifier for list fusion and similar optimizations
- Tier 2 extension support in the frontend pipeline

## Milestone Chirho 4 — LLVM Backend

STG to LLVM IR lowering, native closure and thunk representation, GC integration, and compiled executable output.

Acceptance criteria:

- emit correct LLVM IR for a meaningful subset of programs
- implement native closure and thunk representation with header words and GC info
- integrate copying GC with LLVM-generated code
- produce executable binaries through the shared runtime model
- compare outputs against the compatibility test suite
- C FFI support via ccall calling convention

## Milestone Chirho 5 — Package Ecosystem Integration

Hackage index retrieval, dependency solving, build planning, and multi-package compilation of real-world libraries.

Acceptance criteria:

- retrieve and index Hackage packages
- solve dependency graphs with version constraints and package flags
- build plans handling conditional sections and platform-specific code
- compile selected real-world libraries with tracked compatibility gaps
- consume interface files across package boundaries
- `haskelujah-base-chirho` covers enough of Prelude and common modules for real packages

## Milestone Chirho 6 — WebAssembly Backend

STG to Wasm lowering, Wasm-compatible runtime, WASI FFI bridge, and browser-hostable output.

Acceptance criteria:

- emit Wasm-targeted artifacts from the shared STG lowered form
- implement Wasm-compatible closure/thunk representation
- integrate linear-memory GC or Wasm GC for memory management
- bridge FFI through WASI imports
- run scripted workloads in a Wasm-oriented environment
- validate package loading assumptions needed for browser-hosted or embedded tooling

## Milestone Chirho 7 — Interactive REPL

Interactive evaluation loop with session state, incremental module loading, type inspection, and dual native/Wasm hosting.

Acceptance criteria:

- session state persists across evaluations
- modules can be loaded and reloaded incrementally
- type and value inspection works through shared diagnostics and runtime services
- multiline declarations supported
- explicit package imports and environment introspection
- REPL can run on both native runtime and Wasm-hosted environment

## Milestone Chirho 8 — Ecosystem Hardening

Broader Hackage compatibility, Tier 3 extensions, concurrency runtime, profiling, and performance optimization.

Acceptance criteria:

- compatibility corpus includes a larger slice of important Hackage libraries
- unsupported cases categorized by extension gap, runtime gap, or package-system gap
- green thread scheduler with forkIO, MVar, and basic STM support
- profiling infrastructure for time and allocation measurement
- Tier 2 extensions fully stable, selected Tier 3 extensions supported
- build performance and diagnostics quality improved without architectural churn
