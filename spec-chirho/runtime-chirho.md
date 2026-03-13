<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Runtime Chirho

## Execution Modes

The runtime architecture must support three execution modes from a shared foundation:

- batch execution for compiled libraries and executables
- script execution with low startup latency
- REPL execution with incremental state and interactive evaluation

The wrong design would optimize only for ahead-of-time native compilation and bolt on scripting later. This project should avoid that trap.

## Recommended Runtime Shape

The compiler should lower high-level Haskell into a runtime-oriented form that can feed multiple backends:

- an optimizing LLVM path for native-oriented builds
- a WebAssembly path for sandboxed and browser-adjacent environments
- an interpreter or lightweight JIT path for scripts and REPL workloads

This suggests a shared runtime-oriented IR, likely STG-like in spirit, before backend-specific lowering.

## Why Scripting And REPL Matter Early

Scripting and REPL support impose requirements that affect early design decisions:

- fast module loading
- resumable session state
- stable symbol identities
- cached interface loading
- low-latency evaluation of small expressions
- dynamic package availability during a session

If these concerns are ignored until late milestones, the runtime and backend layering will likely become too batch-oriented.

## WebAssembly Direction

WebAssembly should be treated as a serious target for:

- sandboxed script execution
- embedded teaching and exploration environments
- browser-hosted REPL experiences
- portability for tooling and package inspection services

The Wasm backend should share as much lowering and runtime logic as practical with the native backend.

## Runtime Services

The shared runtime crate should eventually define:

- closure and thunk representation
- evaluation entrypoints
- memory management strategy
- scheduling hooks for concurrency support
- exception propagation hooks
- package and module initialization support

These services must be designed so both compiled binaries and interactive sessions can use them.

## Script Runner Direction

The script runner should:

- accept a file or expression entrypoint
- resolve packages with the same package database logic as batch builds
- cache interfaces and lowered artifacts
- privilege startup and feedback latency over maximum optimization

## REPL Direction

The REPL should eventually support:

- multiline declarations
- module loading and reloading
- type inspection
- evaluated bindings that persist across commands
- explicit package imports and environment introspection
- a mode that can run either on the native runtime or within a Wasm-hosted environment

## Runtime Risk Areas

- preserving lazy semantics while still enabling fast script startup
- choosing a memory management strategy that works for both native and Wasm targets
- balancing interpreter simplicity against eventual JIT needs
- aligning package loading semantics across batch, script, and REPL modes

## Memory Management Direction

### Native Path

Copying generational garbage collector, following GHC's proven approach for lazy languages with high allocation rates:

- two-space copying collector for the nursery (young generation)
- aging-based promotion to older generations
- start with a simple two-space collector, add generational support incrementally
- heap objects carry header words with GC metadata and tag bits

### WebAssembly Path

Linear-memory GC implemented within Wasm's linear memory space:

- same logical algorithm as the native collector, different memory layout
- more portable than depending on the Wasm GC proposal (still evolving)
- trade implementation effort for deployment portability

### Abstraction

GC operations abstracted behind a trait boundary in `rhasky-runtime-chirho` so that native and Wasm backends provide different implementations without changing the STG lowering or thunk evaluation protocol.

### Early Requirements

Even before GC is implemented, runtime data structures must be designed with GC in mind:

- object headers must reserve space for GC metadata
- pointer identification must be unambiguous (no tagged unions that confuse the collector)
- stack maps or conservative scanning must be planned for

## Concurrency Model

### Target Milestone

Full concurrency support is targeted for Milestone 8, but runtime data structures must be designed thread-safe from the start.

### Native Path

Green threads multiplexed on OS threads with a work-stealing scheduler (following GHC's approach). Primitives: `forkIO`, `MVar`, `STM` (basic), `IORef` with atomic operations.

### WebAssembly Path

Single-threaded with cooperative scheduling initially. `SharedArrayBuffer` and atomics for multi-threaded Wasm when browser/runtime support is sufficient.

### Early Requirements

These must be addressed from Milestone 0, not retrofitted:

- thunk updates must use atomic operations (compare-and-swap) so that concurrent forcing is safe
- heap layout must be safe for concurrent GC (no torn reads of multi-word objects)
- no global mutable state without explicit synchronization
- blackhole detection must work correctly under concurrent evaluation

Retrofitting concurrency into a single-threaded runtime is effectively a rewrite. The cost of atomic thunk updates is negligible on modern hardware.

## Exception Model

### Synchronous Exceptions

Implemented through the IO monad evaluation path. `try`, `catch`, `throw`, and `throwIO` as runtime primitives. Key subtlety: a forced thunk can throw an exception, which the evaluator must handle by propagating the exception to the forcing context.

### Asynchronous Exceptions

Deferred to Milestone 8, arriving with the concurrency model. GHC's `throwTo` semantics (one thread throws to another) requires interruptible operations, exception masking (`mask`, `uninterruptibleMask`), and careful interaction with MVars and STM.

### Script Mode

Top-level exception handler that catches all exceptions, prints them to stderr, and exits with a non-zero exit code. This ensures scripts fail visibly rather than silently.

### Imprecise Exceptions

Pure code that evaluates to bottom (e.g. `error "msg"`) should produce imprecise exceptions following GHC's semantics. The choice of which exception to surface when multiple bottoms exist is implementation-defined.

