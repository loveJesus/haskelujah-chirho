<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 582: RealFloat Double properties Chirho

- [x] Reserve progress row 582 and reset the isolated RealFloat worktree to exact main `db0cd136`.
- [x] Measure `floatRadix`, `floatDigits`, and `floatRange` on GHC 9.14.1 and all three Haskelujah engines.
- [x] Add the three body-backed properties to Row 581's generated-Core module without touching active Row 579 or Row 580's owned implementation files.
- [x] Add GHC-measured all-engine controls, first-class use, and user-defined-name ownership.
- [x] Kill the shared generator route with a narrow mutation.
- [ ] Run the warm focused gate, changed-line Clippy, rustfmt, diff-check, and gitleaks.
- [ ] Commit only Row 582 paths, fast-forward main, and close progress row 582.

## Boundaries Chirho

- This row owns only `floatRadix`, `floatDigits`, and `floatRange` at Double.
- `decodeFloat`, `encodeFloat`, `exponent`, `significand`, and `scaleFloat` remain separate RealFloat work.
- No backend or RTS primitive is warranted for properties that are constant for a concrete Double representation.

## Measurement Chirho

- Exact main after Row 580 (`cb8d38b3`) type-checks the control, then every engine stops at `floatRadix`: `missing STG binding` in the interpreter and `missing native binding` in LLVM and Cranelift.
- GHC 9.14.1 and the candidate print the same six lines: `2`, `53`, `(-1021,1024)`, then the corresponding first-class lists over ordinary, exceptional, signed-zero, normal, and subnormal values.
- A program hiding and redefining all three names prints its own `9`, `8`, and `(7,6)` on GHC and all three candidate engines.
- Removing only `generate_real_float_properties_chirho()` makes the property control fail at the original `missing STG binding floatRadix`; restoring it returns all five focused RealFloat controls to GREEN.
