<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Interpreter ties-to-even round Chirho

Row 559 owns the interpreter-only wrong answers in `round` at exact half ties. The
GHC-measured seed is `map round [0.5,1.5,2.5,-0.5] == [0,2,2,0]`; the current
interpreter returns `[1,2,3,-1]`, while both native engines explicitly refuse the
unsupported `round#/1` primitive.

## Contract

- `round` on finite `Double` values uses Haskell's nearest-integer, ties-to-even rule.
- Positive and negative half ties, neighbouring non-ties, signed zero, and integral values
  agree with GHC 9.14.1.
- Existing `truncate`, `floor`, and `ceiling` behavior is measured and remains outside this
  row unless the trace proves they share the exact defective helper.
- LLVM and Cranelift keep their named `round#/1` refusal; implementing that primitive is a
  separate native parity row.

## Checklist

- [x] Trace the `round` class-method route, generated Core row, and interpreter primitive.
- [x] Choose the smallest shared placement after the trace; do not special-case the sweep
  program or its list context.
- [x] Commit GHC-measured positive, negative, tie, and non-tie controls.
- [x] Run a warm focused interpreter proof plus the affected dictionary/numeric suites.
- [x] Verify zero changed-line Clippy findings, rustfmt, diff-check, and gitleaks.
- [x] Land source tip `b12a7994` and close progress row 559.

## Measured source checkpoint

- The route is already correct: the generated Core is `round# :: Double -> Int`, and STG
  lowering preserves `PrimOpKindChirho::RoundChirho`.
- The first wrong operation was the interpreter's use of Rust `f64::round`, which resolves
  half ties away from zero. `f64::round_ties_even` gives GHC's sixteen measured answers.
- Restoring only `f64::round` makes the new control fail at every affected half tie, while
  the non-ties and integral values remain equal.
- Runtime is 57/57, the existing direct round test passes, and dictionary evidence is 83/83.
  Both native backends still refuse exactly `round#/1` in the committed boundary control.
- `eval_chirho.rs` is 5,489 lines: something is wrong with that size. This row changes only
  the existing four-arm numeric-conversion block; extracting the evaluator families is a
  separate structural unit rather than an excuse to widen this parity fix.
