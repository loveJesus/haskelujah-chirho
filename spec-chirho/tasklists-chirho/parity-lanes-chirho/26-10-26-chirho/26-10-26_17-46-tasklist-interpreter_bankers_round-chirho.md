<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Interpreter ties-to-even round Chirho

Row 559 owns the interpreter-only wrong answers in `round` at exact half ties. The
GHC-measured seed is `map round [0.5,1.5,2.5,-0.5] == [0,2,2,0]`; the current
interpreter returns `[1,2,3,-1]` while the native engines are already exact.

## Contract

- `round` on finite `Double` values uses Haskell's nearest-integer, ties-to-even rule.
- Positive and negative half ties, neighbouring non-ties, signed zero, and integral values
  agree with GHC 9.14.1.
- Existing `truncate`, `floor`, and `ceiling` behavior is measured and remains outside this
  row unless the trace proves they share the exact defective helper.
- LLVM and Cranelift answers remain byte-exact; this row does not widen their conversion
  surface.

## Checklist

- [ ] Trace the `round` class-method route, generated Core row, and interpreter primitive.
- [ ] Choose the smallest shared placement after the trace; do not special-case the sweep
  program or its list context.
- [ ] Commit GHC-measured positive, negative, tie, and non-tie controls.
- [ ] Run a warm focused interpreter proof plus the affected dictionary/numeric suites.
- [ ] Verify zero changed-line Clippy findings, rustfmt, diff-check, and gitleaks.
- [ ] Land and close progress row 559.
