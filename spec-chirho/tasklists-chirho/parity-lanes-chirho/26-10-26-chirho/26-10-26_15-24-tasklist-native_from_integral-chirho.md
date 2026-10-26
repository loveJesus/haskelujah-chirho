<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native `fromIntegral#` Chirho

Row 540 starts from `6f027ffd13953318ae2997ba2de841cd8a37582e`. It owns the native Int-to-Double conversion primitive so P03 moves from LLVM refusal and Cranelift `NaN` to GHC's exact `3.5`.

## Placement and boundary

- LLVM emits signed `i64` to `double` conversion and returns the resulting IEEE-754 bits in the uniform `i64` value representation.
- Cranelift emits the equivalent signed conversion and bitcast at the existing primitive dispatch seam.
- The driver regression embeds the saved P03 source and checks its exact output once through the interpreter and both native backends.
- This unit does not change other numeric conversions, `recip#`, surface typing, Wasm, or the float renderer.

## Checklist

- [x] Record the unit and the seconds-scale warm gate before source edits.
- [x] Lower `fromIntegral#/1` on LLVM and Cranelift.
- [x] Add the ordinary P03 cross-backend regression test.
- [x] Update the native workflow at the changed seam.
- [x] Run format, the focused P03 test, both backend libraries, changed-package Clippy, and diff check. Gitleaks follows on the committed range.
- [ ] Land the reviewed tip and close progress row 540 GREEN.

## Measured evidence

- `cargo fmt --all -- --check`: GREEN in 1.11 seconds after formatting the one wrapped test constant.
- The first P03 run compiled the worktree and passed on the interpreter, LLVM, and Cranelift in 14.25 seconds. The warm rerun passed in 3.26 seconds with exact `3.5\n` native output.
- Both native backend library suites passed together in 4.29 seconds: Cranelift 42/42 and LLVM 38/38.
- Warm Clippy over both native backends and driver tests exited 0 in 9.89 seconds. The existing repository warnings do not point at any changed or inserted line.
- `git diff --check` is clean. No file or directory is added outside this tasklist, and the only source growth is at the two existing primitive dispatch seams plus the driver control.
- Gitleaks scanned the committed source range and found no leaks.
- Receipt correction: the source commit's first tasklist version expanded the abbreviated base incorrectly. Its Git parent was always the exact `6f027ffd13953318ae2997ba2de841cd8a37582e`; this record now names it correctly.
