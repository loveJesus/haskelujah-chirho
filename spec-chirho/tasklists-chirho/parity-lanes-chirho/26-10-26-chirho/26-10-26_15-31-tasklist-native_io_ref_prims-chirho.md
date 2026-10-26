<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native IORef primitives Chirho

Row 541 starts from `1b804afaa91d49c02c290e0b736839dc5d82b9f7`. It owns native `newIORef#/1`, `readIORef#/1`, and `writeIORef#/2`, so the saved P01 program moves from a named compile refusal on both backends to GHC's exact `2`.

The progress row and landing wait for the combined 531 + 536 + 517 main/DB hold to release. Source work is explicitly allowed in this dedicated worktree by #28703.

## Placement and boundary

- The RTS owns IORef allocation, allocation-kind validation, reading, and mutation. A reference is a tagged registered allocation whose sole word is traced by the existing conservative collector; the cell itself follows the native backends' current process-lifetime root contract for produced heap values.
- Allocation roots the initial value across a possible collection. Reading and writing accept only allocations created as native IORefs; a constructor, thunk, immediate, or stale pointer fails closed.
- LLVM and Cranelift only force operands and call the three RTS entries. They do not duplicate mutable-heap rules.
- The saved P01 program is the end-to-end contract. STRef, TVar, concurrency, weak references, and atomic IORef operations are outside this unit.

## Checklist

- [x] Trace the saved P01 refusal and choose the shared RTS boundary.
- [x] Add the allocation-kind marker, the three RTS entries, and ordinary unit controls.
- [x] Lower the three primitives on LLVM and Cranelift.
- [x] Change the saved P01 driver control from refusal to exact `2` everywhere.
- [x] Update the native workflow at the changed seam.
- [x] Run the warm 2–5 minute gate: format, P01, RTS plus both backend libraries, changed-package Clippy, diff check, and gitleaks.
- [ ] Rebase onto the released main, land, and close progress row 541 GREEN.

## Measured evidence

- The saved P01 program passed with exact `2\n` through the interpreter, LLVM, and Cranelift. A warm pre-correction run took 5.42 seconds; the corrected run also passed while contending on Cargo's shared package-cache lock.
- The RTS, LLVM, and Cranelift library suites passed together: 50 + 38 + 42 = 130 tests. The final source-change run took 6.46 seconds.
- The RTS controls prove integer round-trip mutation, reject ordinary constructor allocations as IORefs, root a heap payload across allocation-triggered collection, and trace it through the live cell.
- `cargo fmt --all -- --check` is GREEN. Warm changed-package Clippy took 3.87 seconds; its diagnostics are pre-existing and none lands on a changed or inserted line.
- The source gate compiles once and runs ordinary unit tests. It has no serial source-edit mutants, cold targets, frozen-binary reruns, or broad receipt-tree hashing.
- Post-checkpoint review found that Cranelift does not install a general root for every direct-call result. The correction roots the IORef cell under the same conservative process-lifetime contract already used for native produced heap values; the two RTS controls and P01 passed again afterward.
