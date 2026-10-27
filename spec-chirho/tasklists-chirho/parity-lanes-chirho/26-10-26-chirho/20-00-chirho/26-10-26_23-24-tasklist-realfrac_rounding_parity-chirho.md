<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# RealFrac rounding parity Chirho

## Scope

Row 591 closes the measured interpreter residual for `round`, `floor`, and `ceiling` at `Double`.
GHC 9.14.1 and both native backends already wrap finite out-of-range values to the low machine-word
bits and return zero for non-finite inputs. The interpreter saturated because its three arms used
direct Rust casts. Row 589 already moved `truncate`; row 591 reuses the same row-562 RTS conversion
functions for the other three methods and corrects the stale four-method control.

## Checklist

- [x] Measure direct, mapped, first-class, and `RealFrac`-polymorphic ordinary inputs on GHC and all three engines.
- [x] Measure NaN, infinities, and finite out-of-range values; bound the change to the three interpreter arms.
- [x] Export the three existing RTS conversions and call them from the interpreter.
- [x] Make the exceptional four-method control assert the GHC answer everywhere.
- [x] Run focused RTS, runtime, dictionary-evidence, and ordinary rounding controls.
- [x] Run rustfmt, changed-line Clippy, diff-check, and gitleaks.
- [ ] Commit the source, land on `main_chirho`, close progress row 591, and report the tested tip.

## Gate r1

- Source commit: `3d7909b7` on reservation base `7406c4d0`.
- Focused tests: 65 passed, 0 failed: RTS conversion table 2, runtime library 58,
  dictionary-evidence rounding controls 3, and `properFraction` controls 2.
- Reach: restoring the three direct Rust casts makes the exceptional all-engine control fail on
  exactly the interpreter's `round`, `floor`, and `ceiling` lines; `truncate` stays exact from row
  589. Restoring the row-591 calls makes the control pass again.
- Rustfmt and diff-check are clean. Clippy reports no diagnostic on a changed line across the RTS,
  runtime, and driver test targets. Gitleaks scanned the source commit and found no leaks.
