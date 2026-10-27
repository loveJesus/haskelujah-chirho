<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Double `properFraction` parity Chirho

Row 589 owns the missing `properFraction` binding at `Double` and the interpreter's inherited
`truncate#` mismatch that its out-of-range contract exposed. It does not change numeric evidence,
native lowering, or the other three rounding methods from row 562.

## Measured contract

- [x] Measure GHC 9.14.1 and main `006384e6` on the interpreter, LLVM and Cranelift.
  - GHC prints `(3,0.75)` and `(-3,-0.75)` for `Double -> (Int, Double)`.
  - All three Haskelujah engines fail on the missing `properFraction` binding.
  - `truncate`, `round`, `floor` and `ceiling` already match GHC and remain holds.
- [x] Measure boundaries before source.
  - GHC maps NaN and both infinities to `(0,0.0)` and normalizes `-0.0` to `(0,0.0)`.
  - A finite `Double` beyond the `Int` range keeps its wrapped `Int` but has a zero fraction.
  - The compiler's wider-than-`Int` `Integer` representation remains a separate limitation;
    this row covers an `Integer` result only where its value is representable as `Int`.

## Implementation

- [x] Add the standard `(RealFrac a, Integral b) => a -> (b, a)` checker scheme.
- [x] Generate one guarded internal Double binding and a public alias that yields to a program's
  own top-level `properFraction`.
- [x] Compute the integral result with the existing `truncate#`; compute the fractional Double
  without converting an out-of-range wrapped `Int` back to Double. Normalize non-finite values
  and signed zero to GHC's `(0,0.0)`.
- [x] Remove the inherited interpreter RED by routing `truncate#` through row 562's existing RTS
  wrapping conversion. Rust's direct cast saturates `1.0e20`; GHC and both natives wrap it.
- [x] Keep the Real number generator directory within its entry bound and give the broader module
  a truthful name.

## Proof

- [x] All-engine control: ordinary positive and negative fractions, exact integers, signed zero,
  subnormal, NaN, infinities, an out-of-range finite `Int`, first-class use, and a representable
  `Integer` result.
- [x] A program-owned top-level `properFraction` remains authoritative.
- [x] Structural control pins the internal binding, guarded alias, tuple, and absence of a new
  primitive or backend call.
- [x] Reach mutation removes the generator call and restores the named missing-binding failure.
- [x] Warm focused gate: relevant typing/core/driver suites, changed-line Clippy, rustfmt,
  diff-check and gitleaks.

## Gate Chirho

- Focused libraries passed 677/677: typing, Core, runtime, and RTS.
- `evaluation_values_chirho` passed 20/20, including both all-engine Row 589 controls.
- The generator-removal reach mutation restored the named missing-binding failure; the exact source
  restored afterward passed again.
- Clippy checked every target in the five touched crates. Its 628 inherited warnings include none
  on a Row 589 changed or inserted line. The first screen found the new structural tests above
  non-test items; moving that module to the file's end cleared the finding without suppression.
- After replaying onto Row 586, the structural binding test, RTS wrapping table, and both
  all-engine controls passed again (1 + 1 + 2). Rustfmt and diff-check are clean. Gitleaks scanned
  the final one-commit range and found no leaks.
