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
- NaN, infinities, and values outside `Int` range are measured before source and remain named
  boundaries unless the backend mechanism can reproduce GHC's failure without guessing.
- No generic unsupported primitive is silently converted to zero, and no ambient host rounding
  mode may change the result.

## Checklist

- [ ] Measure all four operations as separate GHC/native programs, including exceptional inputs.
- [ ] Trace the LLVM and Cranelift scalar conversion operations and their failure behavior.
- [ ] Choose one explicit operation table per backend, with shared semantic names where useful.
- [ ] Add focused native controls and interpreter holds.
- [ ] Run the warm backend, runtime, and driver proof under the 2–5 minute bar.
- [ ] Verify zero changed-line Clippy findings, rustfmt, diff-check, and gitleaks.
- [ ] Land and close progress row 562.
