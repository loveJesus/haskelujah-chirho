<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Roadmap Chirho

## Milestone Chirho 0

Bootstrap the Rust workspace and testing harness.

Acceptance criteria:

- workspace crate graph matches the architecture spec closely enough to evolve without a rewrite
- diagnostics and source span crates compile cleanly
- golden-test and property-test infrastructure exists

## Milestone Chirho 1

Frontend foundation: lexing, parsing, syntax trees, and basic module loading.

Acceptance criteria:

- parse non-trivial modules with source-preserving diagnostics
- resolve module imports across a package-local graph
- maintain a test corpus of valid and invalid inputs

## Milestone Chirho 2

Name resolution, kinds, and typechecking for a meaningful compatibility subset.

Acceptance criteria:

- typecheck a curated library compatibility corpus
- record unsupported features precisely rather than failing vaguely
- stabilize typed intermediate representations

## Milestone Chirho 3

Typed core, runtime-oriented IR, and a script-capable execution path.

Acceptance criteria:

- lower typed programs into a runtime-oriented IR
- execute simple programs in script mode without full native codegen
- reuse package and interface caches across repeated runs

## Milestone Chirho 4

LLVM backend for native-oriented builds.

Acceptance criteria:

- emit LLVM IR for a meaningful subset of programs
- run compiled executables through the shared runtime model
- compare outputs against a compatibility suite

## Milestone Chirho 5

Package ecosystem integration.

Acceptance criteria:

- parse Cabal package descriptions reliably
- retrieve and index Hackage packages
- compile selected real-world libraries with tracked compatibility gaps

## Milestone Chirho 6

WebAssembly backend and sandbox-friendly runtime support.

Acceptance criteria:

- emit Wasm-targeted artifacts from the shared lowered form
- run scripted workloads in a Wasm-oriented environment
- prove package loading assumptions needed for browser-hosted or embedded tooling

## Milestone Chirho 7

Interactive REPL.

Acceptance criteria:

- session state persists across evaluations
- modules can be loaded and reloaded incrementally
- type and value inspection works through shared diagnostics and runtime services

## Milestone Chirho 8

Broader ecosystem compatibility and quality hardening.

Acceptance criteria:

- compatibility corpus includes a larger slice of important Hackage libraries
- unsupported cases are categorized by extension, runtime gap, or package-system gap
- profiling, diagnostics, and build performance are improved without architectural churn

