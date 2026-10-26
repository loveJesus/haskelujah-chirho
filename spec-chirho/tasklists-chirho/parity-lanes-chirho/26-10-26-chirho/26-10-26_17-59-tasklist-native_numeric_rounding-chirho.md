<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native numeric rounding primitives Chirho

Row 562 owns the LLVM and Cranelift refusals for `round#/1`, `truncate#/1`, `floor#/1`,
and `ceiling#/1`. Row 559 made interpreter `round` GHC-exact and deliberately preserved
these named native boundaries.

## Contract

- Each backend lowers the four `Double -> Int` primitives with GHC-compatible direction:
  nearest ties-to-even, toward zero, toward negative infinity, and toward positive infinity.
- Positive and negative values, exact half ties, neighbouring non-ties, integral values, and
  higher-order/list uses are measured separately against GHC 9.14.1.
- The interpreter remains byte-exact to Row 559.
- GHC 9.14.1 converts NaN and both infinities to zero. Finite out-of-range values wrap to the
  machine `Int`; `1.0e20` is `7766279631452241920` and its negative is the opposite value.
  Both native backends reproduce those bytes through the shared RTS conversion. The interpreter's
  saturating exceptional-input cast remains the named residual.
- No generic unsupported primitive is silently converted to zero, and no ambient host rounding
  mode may change the result.

## Checklist

- [x] Measure all four operations as separate GHC/native programs, including exceptional inputs.
- [x] Trace the LLVM and Cranelift scalar conversion operations and their failure behavior.
- [x] Choose explicit native operation names backed by one shared RTS semantic implementation.
- [x] Add focused native controls and interpreter holds.
- [x] Run the warm backend, runtime, and driver proof under the 2–5 minute bar.
- [x] Verify zero changed-line Clippy findings, rustfmt, diff-check, and gitleaks.
- [x] Land and close progress row 562.

## Focused proof

- RTS library: 58 passed, including the direction and exceptional conversion controls.
- LLVM library: 38 passed.
- Cranelift library: 42 passed.
- Fractional-literal integration: 7 passed, including both native backends and the interpreter hold.
- Clippy reported no warning on a changed line; the workspace's existing warnings remain visible.
- After row 554 landed, the source commit replayed cleanly onto `00dce20b`; the same focused proof
  passed again, and gitleaks found no leak in the final one-commit range.
