<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Interpreter Show Float Finite Chirho

Row 549 owns the finite-number formatting boundary left explicit by row 546.

## Measured rule

- GHC 9.14.1 uses fixed notation when the normalized decimal exponent is from
  `-1` through `6`, inclusive.
- It uses scientific notation at exponent `-2` or below and `7` or above.
- Scientific mantissas always contain a decimal point (`1.0e7`, `1.0e-2`).
- Rust's lower-exponent formatter supplies the shortest mantissa and exponent needed
  by that branch. Rust's `Debug` formatter supplies GHC's required `.0` and signed zero
  in the fixed branch.
- Row 546's `NaN`, `Infinity`, and `-Infinity` spellings remain unchanged.
- Native whole-`Double` formatting is a separate residual and is not claimed here.

## Checklist

- [x] Measure both exponent boundaries, signed zero, ordinary fractions, extrema, and
      Rust's three relevant formatters.
- [x] Centralize finite fixed/scientific selection in the shared RTS
      `show_double_chirho`.
- [x] Add unit and interpreter end-to-end controls for the threshold and mantissa rule.
- [x] Update the Show workflow with the finite rule.
- [x] Run the warm focused gate inside 2–5 minutes.
- [ ] Land on current `main_chirho` and close progress row 549 GREEN.

## Source checkpoint

- The formatter lives in `haskelujah-rts-chirho`, not the interpreter. The runtime
  crate calls it directly, and row 550 will reuse the same function for native output.
- GHC 9.14.1 measurements cover both thresholds, both signs, the smallest subnormal,
  `MIN_POSITIVE`, and `MAX`. Rust's shortest lower-exponent mantissa matches those
  measured digits; the helper adds `.0` only where GHC requires it.
- Warm proof is GREEN in 19.7 seconds: RTS 56/56, runtime 57/57, and fractional-literal
  integration 4/4. Rustfmt and diff-check are clean. Clippy reports no diagnostic in a
  changed file; its warnings are pre-existing lines elsewhere.
