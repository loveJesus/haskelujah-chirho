<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Haskelujah Chirho Spec Index

## Mission

Haskelujah Chirho is a Rust implementation of a Haskell compiler and runtime with three first-class execution modes:

- batch compilation for executables and libraries
- script execution with fast startup
- an eventual interactive REPL with incremental loading and evaluation

The design target is maximal practical compatibility with real-world Haskell and the Cabal/Hackage ecosystem, while improving internal modularity, diagnostics, portability, and compiler maintainability.

## What This Spec Package Covers

- [vision-chirho.md](./vision-chirho.md): product goals, non-goals, and success criteria
- [architecture-chirho.md](./architecture-chirho.md): compiler pipeline and Rust crate layout
- [compatibility-chirho.md](./compatibility-chirho.md): compatibility contract for source, packages, and runtime behavior
- [runtime-chirho.md](./runtime-chirho.md): execution model for batch, scripting, REPL, LLVM, and WebAssembly
- [roadmap-chirho.md](./roadmap-chirho.md): staged delivery plan and milestone acceptance criteria
- [review-questions-chirho.md](./review-questions-chirho.md): review prompts with resolutions
- [prior-art-chirho.md](./prior-art-chirho.md): lessons from Eta, GRIN, UHC, PureScript, and other projects
- [prd-chirho.json](./prd-chirho.json): machine-readable product requirements document for iteration

## Hard Requirements

- Implement the compiler in Rust.
- Keep the pipeline backend-agnostic.
- Treat LLVM IR as the first serious native backend.
- Treat WebAssembly as a first-class backend, not an afterthought.
- Support Cabal package descriptions and Hackage package retrieval.
- Aim at compiling real ecosystem libraries, not only toy examples.
- Preserve compatibility by default and improve architecture internally.
- Split the implementation into focused Rust crates for portability and build speed.
- Keep scripting and eventual REPL support in scope from the beginning of the architecture.

## Core Design Principles

- Compatibility first, divergence only when deliberate and documented.
- Typed phase boundaries between compiler stages.
- Small crates with stable interfaces.
- Strong diagnostics and reproducible builds.
- Runtime design decided early enough to support laziness, scripting, and REPL use cases.
- Incremental compilation and caching where it reduces edit-run latency.

