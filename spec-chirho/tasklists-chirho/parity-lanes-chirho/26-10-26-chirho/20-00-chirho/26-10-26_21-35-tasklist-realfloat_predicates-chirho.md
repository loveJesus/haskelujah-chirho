<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 581: RealFloat predicates Chirho

- [x] Reserve progress row 581 and create an isolated worktree from exact main `2687441f`.
- [x] Measure `isNaN`, `isInfinite`, `isDenormalized`, `isNegativeZero`, and `isIEEE` on GHC 9.14.1 and all three Haskelujah engines.
- [x] Trace the generated Prelude and dictionary-evidence route; keep the fix in Core unless the measurement disproves that boundary.
- [x] Add one bounded RealFloat-predicate module without exceeding the `prelude_chirho/` entry cap.
- [x] Add GHC-measured all-engine controls for finite values, infinities, NaN, normal and denormal boundaries, and signed zero.
- [x] Prove the new route is reached with a narrow mutation and preserve user-defined top-level names.
- [x] Run rustfmt, focused tests, changed-line Clippy, diff-check, gitleaks, and a warm behavior gate.
- [x] Record the measured result, commit only Row 581 paths, and hand the tested tip back for independent landing.

## Boundaries Chirho

- No new backend or RTS primitive unless a pure generated-Core definition cannot reproduce GHC's observable result.
- No edits to Row 579's report-text definitions or Row 580's `pi`/`atan2` implementation.
- Preserve the existing behavior for unresolved or non-Double RealFloat evidence.
- This row does not claim the rest of `RealFloat`, such as `floatRadix`, `decodeFloat`, `encodeFloat`, or `scaleFloat`.

## Measurement Chirho

- Main `2687441f` type-checks the program, then every engine stops at the first demanded name: the interpreter reports `missing STG binding isNaN`, and both natives report `missing native binding isNaN`.
- GHC 9.14.1 and the candidate produce the same nine output lines on the interpreter, LLVM, and Cranelift. The rows cover `0.0`, `-0.0`, `1.0`, both infinities, NaN, `5.0e-324`, `2.2250738585072014e-308`, a top-level negative zero, and first-class use through `map`.
- A `RealFloat a =>` wrapper is exact in the interpreter at Double. Its native compilation remains blocked by the pre-existing `pi#/1` and `recip#/1` dictionary route, which Row 581 does not widen.
- Removing the five names from the occurrence-evidence list changes none of these results, so the final source does not touch that list.
- Removing only `generate_real_float_predicates_chirho()` makes the all-engine control fail immediately at `missing STG binding isNaN`; restoring it returns the three focused controls to GREEN.

## Gate Chirho

- Warm gate on source commit `be036eb1`: GREEN in 52.94 seconds.
- `haskelujah-core --lib`: 175/175.
- `evaluation_values_chirho`: 12/12, including all three Row 581 controls.
- `dictionary_evidence_chirho`: 109/109.
- Rustfmt, diff-check, build, and gitleaks: GREEN; one commit scanned and no leaks found.
- Clippy: zero diagnostics in either new file or on any changed line. Four existing diagnostics remain on unchanged lines in the two touched parent files.
- The source range is six owned paths. `prelude_chirho/` has 14 entries on this branch; Row 579's coordinated `report_chirho/` move keeps the combined layout below 15.
