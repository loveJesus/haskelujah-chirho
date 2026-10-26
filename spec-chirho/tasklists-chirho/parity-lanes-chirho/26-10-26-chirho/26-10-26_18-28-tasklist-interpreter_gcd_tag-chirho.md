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

## Gate

- Source commit `5707e0b4` passed the warm focused gate in 49 seconds.
- `cargo build -p haskelujah` completed without a compiler warning.
- The typing library passed 362/362 tests; `evaluation_values_chirho` passed 9/9; and
  `native_error_helpers_chirho` passed 35/35. The former R5 pair is exact on the interpreter,
  LLVM and Cranelift.
- Clippy completed for the typing and driver libraries/tests. It reported the repository's existing
  diagnostics, but none on a changed line. Rustfmt and diff-check were clean; gitleaks found no
  leak in the one-commit source range.
- Reach check: removing only the two schemes makes the exact typing assertion fail and returns the
  interpreter matrix to `no matching alternative`; restoring the source makes both pass.

## Checklist

- [x] Record the GHC/all-engine matrix and the occurrence-evidence shape.
- [x] Identify the first wrong permission: missing builtin schemes.
- [x] Implement the narrow repair with focused controls.
- [x] Run the focused gate under three minutes.
- [x] Verify rustfmt, diff-check, changed-line Clippy, and gitleaks.
- [ ] Land and close progress row 564.
