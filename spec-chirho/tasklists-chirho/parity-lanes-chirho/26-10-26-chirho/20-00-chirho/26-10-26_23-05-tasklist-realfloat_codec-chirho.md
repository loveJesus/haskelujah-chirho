<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 590: Double `decodeFloat` and `encodeFloat` Chirho

- [x] Reserve progress row 590 from exact main `ae7ecfc1` and publish the non-overlap boundary.
- [x] Measure GHC 9.14.1 and exact main on ordinary values, signed zero, subnormals, maximum
  finite values, non-finite values, rounding, overflow, underflow, first-class use, and
  program-owned names.
- [x] Prove the generated-Core composition over the landed Double representation rows.
- [x] Add body-backed Double rows and guarded Prelude aliases without touching native `recip#`
  or lazy-value lowering.
- [x] Add all-engine, structural, ownership, and reach controls.
- [x] Run a warm focused gate, changed-line Clippy, rustfmt, diff-check, and gitleaks.
- [x] Replay onto current main, fast-forward, and close progress row 590.

## Boundary Chirho

- The standard polymorphic surface schemes already exist.
- This row owns the executable Double rows for `decodeFloat` and `encodeFloat` only.
- The initially proposed implementation lives in the existing
  `prelude_chirho/real_number_methods_chirho.rs` and composes already-landed operations.
- Arbitrary-precision `Integer` remains a compiler-wide limitation. The row matches GHC throughout
  the currently reachable signed machine-word domain and states the minimum-word residual.
- Row 583 owns native `recip#`; Row 588 owns native lazy-value lowering. Row 590 touches neither.

## Measurement Chirho

- Exact main fails both native runs with `missing native binding decodeFloat`; the interpreter
  stops at `missing STG binding decodeFloat`. Program-owned versions remain exact.
- GHC's finite examples include `(6755399441055744,-52)` for `1.5`,
  `(4503599627370496,-1126)` for the minimum subnormal, and
  `(9007199254740991,971)` for the largest finite Double.
- GHC also exposes the IEEE payload representation through this method: positive Infinity,
  negative Infinity, and NaN decode to signed mantissas with exponent `972`, not to a special
  failure or zero.
- `encodeFloat` controls pin signed values, the minimum subnormal, smallest normal, largest finite
  value, rounding at `2^53 + 1`, both reachable machine-word extremes, overflow, underflow,
  first-class use, and program-owned names. The exact minimum word is a named residual: spelling
  `-9223372036854775808` directly, reading the current `minBound`, or converting a computed
  `minBound` to `Integer` produces zero before `encodeFloat` is called.

## Implementation Chirho

- `decodeFloat x` special-cases either signed zero to `(0,0)`. Otherwise it returns
  `truncate (scaleFloat 53 (significand x))` and `exponent x - 53`, directly through the existing
  representation primops.
- `encodeFloat m n` is `scaleFloat n (fromIntegral m)` over the compiler's current signed
  machine-word `Integer` representation.
- The shared generated-Core case builder now carries its actual result type. Its earlier Boolean
  annotation on Row 589's tuple-valued `properFraction` branch did not affect execution, but was
  incorrect Core metadata; Boolean predicate rows retain their Boolean wrapper.
- Removing only the codec generator call restores the exact missing-binding failure. Restoring the
  call makes the two all-engine controls pass again.

## Gate Chirho

- The warm focused libraries passed 395/395 across Core, runtime, RTS, LLVM, and Cranelift.
- The full `evaluation_values_chirho` target passed 22/22. After adding the reachable signed-word
  boundary cases, both Row 590 all-engine controls passed again, 2/2.
- The generator-call mutation failed at the named missing `decodeFloat` binding. Its byte-exact
  restoration rebuilt Core and returned both controls to green.
- Clippy checked every Core and driver target. Its 562 inherited warnings include none on a Row
  590 changed or inserted line. Rustfmt and diff-check are clean.
- Gitleaks scanned the one-commit source range and found no leaks.

## Landing Chirho

- Source tip `b1c4a262` fast-forwarded from the exact reservation base `281e79dc`; no replay was
  needed and the gated tree is the landed tree.
- Progress row 590 closed in the following DB-only documentation commit after SQLite reported one
  changed row and `integrity_check=ok`.
