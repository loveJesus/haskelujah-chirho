<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Interpreter Float Ordering Chirho

Provisional row 542 starts from held main `1b804afaa91d49c02c290e0b736839dc5d82b9f7`. It owns the interpreter's `compareFloat#` NaN ordering only. GHC's `Ord Double` gives `GT` when either operand is NaN and `EQ` for negative zero against positive zero; the interpreter currently converts an unordered `partial_cmp` result to `EQ`.

The source is disjoint from row 539's LLVM and Cranelift lowering and the combined 531 + 536 + 517 stack. Main and the progress DB remain held by Claude2.

## Placement

- Put the pure ordering rule and its boundary controls in a small `eval_chirho/float_ordering_chirho.rs` module instead of adding more logic and tests to the pre-existing 5,489-line evaluator.
- Keep evaluator dispatch responsible only for extracting two `FloatChirho` operands and building the `Ordering` value.
- Add one end-to-end interpreter control to the existing numeric hierarchy suite. It covers NaN on the left, right and both sides, plus negative zero.
- Native lowering is outside this unit; row 539 owns it.

## Checklist

- [x] Measure the four expected results on GHC 9.14.1 and reproduce the current interpreter mismatch.
- [x] Extract the GHC-compatible float ordering rule into its own bounded module.
- [x] Add the pure boundary controls and one end-to-end interpreter control.
- [x] Run a warm 2-5 minute gate: focused runtime and driver controls, format, changed-package Clippy, diff-check and gitleaks.
- [ ] Rebase onto released main, land, and close progress row 542 GREEN.

## Measured evidence

- GHC 9.14.1 prints `GT`, `GT`, `GT`, `EQ` for NaN-left, NaN-right, NaN-both and negative-zero controls.
- The pre-change focused interpreter control failed with `EQ`, `EQ`, `EQ`, `EQ`, reproducing the `partial_cmp(..).unwrap_or(Equal)` defect exactly.
- After the change, both pure runtime controls passed in 3.46 seconds, and the end-to-end interpreter control passed in 4.97 seconds.
- The complete runtime library passed 56/56 in 0.15 seconds; the complete numeric hierarchy suite passed 6/6 in 4.55 seconds.
- Warm runtime Clippy completed in 0.21 seconds and the focused driver-test Clippy in 12.10 seconds. They report only pre-existing diagnostics; none is on a changed or inserted line. Format and diff-check are clean.
- The gate compiles the ordinary negative and boundary controls once. It uses no cold target, machine-wide process refusal, serial source-edit mutant loop, frozen-binary rerun or receipt-tree hash walk.

## Post-rebase gate

- The source commit rebased cleanly onto `1eaad249`, main after Rows 539 and 541 and both progress closes.
- The runtime library passed 56/56, and the focused numeric-hierarchy control passed with GHC's `GT GT GT EQ` behavior.
- Row 539's Compare2 control stayed exact through the interpreter, LLVM, and Cranelift. Its 13-line float/string edge control is now promoted from native-only to all three engines and passed there as well.
- Format and diff check are clean. Warm Clippy completed successfully with only pre-existing diagnostics outside the changed files and lines; gitleaks scanned both Row 542 commits and found no leaks.
- The branch is ready for a fast-forward landing. The final checklist item remains open until `main_chirho` and progress row 542 are committed together.
