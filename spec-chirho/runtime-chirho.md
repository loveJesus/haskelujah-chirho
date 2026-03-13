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

