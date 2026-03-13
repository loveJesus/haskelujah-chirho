<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Vision Chirho

## Product Goal

Build a production-grade Haskell compiler in Rust that is as close to a 1:1 replacement as practical for everyday Haskell development, while fixing architectural pain points that make existing implementations difficult to evolve.

## User-Facing Modes

### Batch Compiler Mode

The compiler must build libraries and executables with deterministic outputs, stable diagnostics, good cache behavior, and backends for native code and WebAssembly.

### Script Mode

The runtime must execute Haskell files like a scripting language with low startup latency, package resolution, and a mode that privileges quick edit-run cycles over peak optimization.

### REPL Mode

The architecture must leave room for a future REPL that can:

- load packages and modules incrementally
- preserve session state
- evaluate expressions quickly
- surface rich type information and diagnostics
- reuse compiled artifacts when possible

## Compatibility Goal

The long-term target is maximal practical compatibility with the real Haskell ecosystem:

- Haskell language compatibility
- Cabal package compatibility
- Hackage package retrieval and local package database support
- enough GHC behavior compatibility that common libraries and build workflows can work without source forks

This does not require bug-for-bug compatibility by default. The default posture is source compatibility and package compatibility, with cleaner internal implementation and explicit compatibility shims where ecosystem reality requires them.

## Quality Goal

The project should improve on known pain points:

- clearer module boundaries
- stronger internal invariants
- easier testing
- better diagnostics
- better portability
- better compile-time ergonomics for the compiler itself through a crate-split Rust workspace

## Non-Goals For The Earliest Milestones

- immediate support for every GHC extension
- immediate bug-for-bug parity in obscure corner cases
- peak native-code performance before correctness and compatibility
- locking the project into an LLVM-only future

## Success Criteria

The project is on track when it can:

1. parse, resolve, and typecheck non-trivial Haskell modules
2. compile a curated compatibility suite of real libraries
3. execute scripts with acceptable startup latency
4. run an interactive REPL on the same runtime foundations
5. emit both native-oriented IR and WebAssembly-oriented artifacts from the same core pipeline

