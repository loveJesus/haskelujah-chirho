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

## Layout Rule

Haskell's whitespace-sensitive syntax is specified as a lexer transformation. The lexer must track indentation context and insert virtual tokens:

- `{` when a layout-introducing keyword (`where`, `let`, `do`, `of`) is followed by a token at a certain column
- `;` when a new line begins at the same indentation level as the current layout context
- `}` when a new line begins at a lesser indentation level, or when the enclosing context ends

The layout rule interacts with many constructs and its correct implementation is critical: nearly all real Haskell code relies on it. The parser grammar should remain context-free by consuming the virtual tokens the lexer inserts.

Implementation should be tested extensively against GHC's lexer output using a large corpus of real Haskell files.

## Syntax Tree Strategy

The parser should produce a red-green lossless concrete syntax tree (CST), following the approach proven by Roslyn (C#) and rust-analyzer (Rust):

- **Green nodes** are immutable, interned, identity-free tree nodes that store text and child structure. They enable cheap cloning and sharing.
- **Red nodes** are position-aware wrappers that carry parent pointers and absolute offsets, created on demand during traversal.
- **Trivia** (whitespace, comments) is preserved in the tree, attached to tokens as leading or trailing trivia.

This design supports:

- error recovery (the parser can produce a partial tree for malformed input)
- formatting and refactoring tools (all original text is preserved)
- REPL and IDE integration (incremental reparsing of changed regions)
- typed AST wrappers that provide convenient access patterns over the untyped CST

The `rhasky-ast-chirho` crate should provide typed wrappers that project structured views over CST nodes without copying.

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

Some crates (e.g. `rhasky-simplify-chirho`, `rhasky-stg-chirho`) may start as modules within their parent crate and split out when they grow large enough to justify independent compilation.

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

## Error Modeling Strategy

The diagnostics system should support:

- **Byte-offset spans** via `rhasky-span-chirho`, not line numbers alone. Spans reference a file ID and a byte range.
- **Error codes** for machine-readable classification (e.g. `E0001` for parse errors, `E0100` for name resolution, `E0200` for type errors). Codes enable documentation, filtering, and IDE quick-fix lookup.
- **Primary span** identifying the main location of the problem.
- **Secondary labeled spans** pointing to related locations (e.g. "first defined here", "expected because of this annotation").
- **Fix suggestions** with concrete text replacements, enabling IDE auto-fix and CLI `--fix` modes.
- **Severity levels**: error, warning, info, hint.
- **Accumulation**: the compiler should continue after non-fatal errors, collecting diagnostics rather than aborting at the first failure. A diagnostic bundle represents the full set of issues from a phase.

Diagnostics must be format-agnostic: the same `DiagnosticChirho` data can be rendered as terminal output, LSP diagnostics, JSON, or REPL messages.

## Constraint Solving Note

`propagators-chirho` is worth evaluating for narrow parts of the design such as constraint propagation, incremental analysis, or solver experimentation. It should remain optional unless it proves better than a purpose-built solver for type inference, dependency solving, or REPL invalidation.

## Incrementality

Incremental behavior should exist at more than one layer:

- file and module parsing caches
- interface and typechecking caches
- package build plan caches
- runtime session caches for scripting and REPL

The build driver should be able to reuse artifacts aggressively without corrupting correctness.

## Interface File Format

Separate compilation requires serialized module signatures. The interface file format should store:

- exported type signatures, kind information, and type class instances
- inlining candidates and unfoldings for cross-module optimization
- rewrite rules defined in the module
- fixity declarations and warning annotations
- a fingerprint/hash for incremental invalidation

The format should be a custom binary representation designed for Rhasky's type system, not GHC's `.hi` format. GHC's interface files are tightly coupled to GHC internals and unstable across versions.

Interface files should be versioned so that stale artifacts from an older compiler version are detected and rejected cleanly.

Granularity: start with per-module hashing for invalidation. Refine to per-declaration fingerprinting if build times warrant finer-grained recompilation avoidance.
