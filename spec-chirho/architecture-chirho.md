<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Architecture Chirho

## Compiler Pipeline

The preferred pipeline is:

1. source loading and package graph construction
2. lexing and parsing into a lossless syntax representation
3. name resolution and module graph validation
4. kind checking and type inference/checking
5. desugaring into a smaller typed core
6. optimization and evidence elaboration
7. lowering into an STG-like or similarly runtime-oriented representation
8. backend lowering to LLVM IR, WebAssembly, or an interpreter-friendly form
9. linking, packaging, or interactive loading depending on execution mode

The phase boundaries should be explicit in the type system so each pass consumes one representation and produces another.

## Preferred Rust Workspace Layout

The compiler should be split into focused crates with clear ownership:

- `rhasky-span-chirho`: source offsets, file ids, and spans
- `rhasky-diagnostics-chirho`: structured diagnostics, notes, labels, and rendering
- `rhasky-syntax-chirho`: tokens, trivia, and syntax node kinds
- `rhasky-parser-chirho`: parser and error recovery
- `rhasky-ast-chirho`: typed AST wrappers over syntax structures
- `rhasky-hir-chirho`: resolved module-level representation
- `rhasky-namer-chirho`: name resolution, imports, exports, and package visibility
- `rhasky-types-chirho`: kinds, types, substitutions, predicates, and typeclass data
- `rhasky-typecheck-chirho`: inference, checking, instance resolution, and evidence
- `rhasky-core-chirho`: small typed core representation
- `rhasky-simplify-chirho`: optimizer passes and normalization
- `rhasky-stg-chirho`: runtime-oriented intermediate form for laziness and closure conversion
- `rhasky-runtime-chirho`: runtime services shared by script, REPL, and compiled programs
- `rhasky-backend-llvm-chirho`: LLVM lowering and native-oriented code generation
- `rhasky-backend-wasm-chirho`: WebAssembly lowering and runtime integration
- `rhasky-package-db-chirho`: Cabal parsing, package database logic, and Hackage index access
- `rhasky-driver-chirho`: orchestration, incremental cache policy, and build planning
- `rhasky-cli-chirho`: command-line interface
- `rhasky-repl-chirho`: interactive loop, session state, and incremental evaluation
- `rhasky-test-harness-chirho`: golden tests, compatibility suites, and property-test utilities

## Crate Boundary Rules

- Syntax and parsing crates must not depend on typechecking crates.
- Backend crates must consume shared lowered forms instead of reaching back into earlier phases.
- Package management logic must remain reusable by compiler, script mode, and REPL mode.
- Runtime services must be defined once and consumed by both native and WebAssembly execution paths.
- Diagnostics must be format-agnostic so CLI, editor tooling, and REPL can share them.

## Data Structure Preferences

- use stable ids or arenas where graph-shaped compiler data benefits from identity stability
- keep immutable phase outputs where practical
- store provenance information so diagnostics can survive transformations
- model invalid states with types whenever that reduces downstream checks

## Constraint Solving Note

`propagators-chirho` is worth evaluating for narrow parts of the design such as constraint propagation, incremental analysis, or solver experimentation. It should remain optional unless it proves better than a purpose-built solver for type inference, dependency solving, or REPL invalidation.

## Incrementality

Incremental behavior should exist at more than one layer:

- file and module parsing caches
- interface and typechecking caches
- package build plan caches
- runtime session caches for scripting and REPL

The build driver should be able to reuse artifacts aggressively without corrupting correctness.

