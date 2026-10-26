<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# `gcd`/`lcm` Integral result evidence parity Chirho

Row 564 owns the missing builtin type schemes for Prelude `gcd` and `lcm`. Their generated Core
bodies already compute correctly, but the placeholder typing route leaves the result unconstrained.
An unannotated tuple consumer therefore records `Show (,)` instead of `Show (Int,Int)` and reaches
an impossible runtime case on the interpreter and both native backends. This row does not change
Char Enum, String representation/rendering, prefix-minus fixity, or the Ord aggregate lane.

## Contract

- Measure GHC 9.14.1 and all three engines before source.
- Trace why individual and annotated results pass while the unannotated printed pair fails.
- Seed exactly `gcd, lcm :: Integral a => a -> a -> a`; do not add a print fallback.
- Cover positive, negative, zero, equal, coprime, and first-class calls as the traced scope permits.
- Preserve GHC's nonnegative `gcd` and `lcm` results.

## Trace

- GHC prints `(6,12)`. Each engine instead reached `no matching alternative` for the saved R5
  pair, while standalone calls and an explicitly annotated `(Int,Int)` tuple were exact.
- The failing `print` capture resolved as `TupleChirho([VarChirho(5), Int])` and finalized to the
  head key `(,)`; the annotated form resolved as `TupleChirho([Int, Int])`.
- No builtin typing scheme named `gcd` or `lcm`. Phase 3.9 therefore supplied a placeholder whose
  two arguments and result were not one Integral type. The generated Core arithmetic was sound.
- Adding the public schemes makes the typed second operand resolve the result to Int and produces
  body-backed `Show (Int,Int)` evidence on all three engines.

## Checklist

- [x] Record the GHC/all-engine matrix and the occurrence-evidence shape.
- [x] Identify the first wrong permission: missing builtin schemes.
- [x] Implement the narrow repair with focused controls.
- [ ] Run the focused gate under three minutes.
- [ ] Verify rustfmt, diff-check, changed-line Clippy, and gitleaks.
- [ ] Land and close progress row 564.
