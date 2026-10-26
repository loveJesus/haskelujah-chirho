<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native integer power primitive Chirho

Row 538 starts from `ae6ee0f8fecddf7983e7f47f90d60fb9276b9ac6`. It owns the integer `^#` path on LLVM and Cranelift so P06 prints GHC's `1024` instead of being refused. The shared runtime helper must preserve the interpreter's existing primitive convention: wrapping `i64` exponentiation, exponent zero returns one, and a negative exponent returns zero.

## Placement

- The arithmetic implementation lives once in the RTS FFI surface.
- LLVM declares and calls that runtime entry for `^#/2`.
- Cranelift imports the same entry through its existing required-runtime-call table and calls it for `^#/2`.
- The driver control replaces the old Cranelift-refusal assertion with one exact cross-backend result.
- The native failure workflow records that `^#` has left the refused set.

## Boundaries

- This unit does not change floating exponentiation, Wasm lowering, type inference, or any other refused primitive.
- P06 is the GHC-parity claim. Negative exponents retain the compiler's already-established primitive convention; this unit does not claim that convention as GHC surface-language parity.
- Verification uses the worktree's warm incremental target. There is no machine-wide Cargo lock, cold target rebuild, serial mutation campaign, or whole-receipt tree hash walk.

## Checklist

- [x] Record the row, owned mechanism, boundaries, and 2–5 minute gate before source edits.
- [x] Add the shared RTS helper and ordinary unit tests for its arithmetic contract.
- [x] Lower `^#/2` through the helper on LLVM and Cranelift.
- [x] Turn P06 into an exact interpreter/LLVM/Cranelift regression test.
- [x] Update the native workflow at the changed seam.
- [x] Run the warm focused tests, changed-package lint, and diff check. Gitleaks follows on the committed range.
- [x] Record measured timings and evidence, fast-forward `main_chirho` to source commit `aa94a17b`, and close progress row 538 GREEN.

## Measured evidence

- `cargo fmt --all -- --check`: GREEN in 1.25 seconds.
- The first focused P06 run compiled the worktree and passed on the interpreter, LLVM, and Cranelift in 13.34 seconds. The warm rerun passed in 3.43 seconds and printed `1024\n` on both native backends.
- The complete RTS, LLVM-backend, and Cranelift-backend library suites passed together in 5.90 seconds: 48 + 38 + 42 tests.
- Warm Clippy over the RTS, both native backends, and driver tests exited 0 in 10.30 seconds. The repository's existing warnings remain outside this range; none points at a changed or inserted line.
- `git diff --check` is clean. The one new Rust file is 15 lines; no file newly crosses 1,500 lines. The already-oversized Cranelift and LLVM lowering files grow only at their primitive dispatch seams.
- Gitleaks scanned the one-commit source range and found no leaks. `main_chirho` fast-forwarded from `ae6ee0f8` to `aa94a17b`; progress row 538 then closed GREEN with the same measured evidence.
