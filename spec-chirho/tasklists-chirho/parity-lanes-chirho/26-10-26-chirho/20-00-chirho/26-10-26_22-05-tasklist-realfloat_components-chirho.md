<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 585: RealFloat Double exponent and significand Chirho

- [x] Reserve progress row 585 and branch from exact main `530858f8`.
- [x] Measure `exponent`, `significand`, and the initially proposed `scaleFloat` on GHC 9.14.1 and exact main across the three engines.
- [x] Implement one IEEE-754 Double representation path shared by the interpreter, LLVM, and Cranelift.
- [x] Add GHC-measured edge controls and structural RTS tests.
- [x] Kill each primitive route with a narrow reach mutation.
- [x] Run the warm focused gate, changed-line Clippy, rustfmt, diff-check, and gitleaks.
- [ ] Commit only Row 585 paths, fast-forward main, and close progress row 585.

## Boundaries Chirho

- This row owns `exponent` and `significand` at Double only.
- `decodeFloat` and `encodeFloat` remain separate RealFloat work.
- `scaleFloat` moved to reserved Row 587 after its direct negative exponent exposed a distinct argument-key defect.
- Row 584 and Row 586 own rigid-variable evidence and dictionary-passing aggregates; this row does not change evidence routing.

## Measurement Chirho

- Exact main stops at a missing STG binding for each of the three names in the interpreter; native executions do not produce the GHC values.
- GHC gives exponent `[0,0,1,2,-996,1025,1025]` and significand `[0.0,0.0,0.75,-0.625,0.6696928794914171,0.5,0.75]` over zero, signed zero, ordinary finite values, a subnormal-scale value, Infinity, and NaN.
- The proposed `scaleFloat (-3)` control exposed a second mechanism before commit: the Double result key reached the Int exponent and retargeted `negate#` to `negateFloat#`. The interpreter failed loudly and the native route would silently produce the wrong answer. Every scaleFloat source change was removed from this row and Row 587 was reserved.

## Placement Chirho

- Extend the existing generated-Core RealFloat module; do not add a fifteenth entry to `prelude_chirho/`.
- Put the pure Double representation routines beside the existing floating routines in `haskelujah-rts-chirho/src/float_math_chirho.rs`; `haskelujah-rts-chirho/src/` is already at its fifteen-entry limit.
- Native backends call those exact RTS routines. The interpreter calls the same pure functions, so representation behavior has one owner.

## Implementation and reach Chirho

- The missing surface schemes were part of the root: without them, the zero result of `significand` retained an Int-shaped route and printed `0`. The explicit RealFloat schemes make the result types match GHC before dictionary generation.
- `$prim_RealFloat_exponent_Double` and `$prim_RealFloat_significand_Double` are body-backed generated rows. Their guarded aliases yield to a program-owned top-level definition.
- The RTS table pins GHC's answers for ordinary finite values, both signed zeros, the minimum subnormal, the smallest normal, the largest finite Double, Infinity, and NaN. The all-engine control pins direct and first-class uses.
- Replacing the normal exponent result with zero makes the RTS table fail with the observed exponent vector collapsed to zeros. Changing the significand exponent field from 1022 to 1023 makes the raw-bits assertion for `1.5` fail. Both mutations were restored byte-identical, and the focused RTS and all-engine controls passed again.

## Gate Chirho

- The warm focused gate is GREEN in 36.91 seconds: build 4.24 seconds; 621 affected-library tests in 18.94 seconds; 16/16 all-engine evaluation tests in 13.73 seconds.
- Clippy completed for the seven affected packages with no diagnostic on a Row 585 changed line. Its remaining warnings predate this row and sit outside the changed lines.
- `cargo fmt --all -- --check` and `git diff --check` are clean. The staged source range passes gitleaks 8.30.1 with no leak.

## Replay Chirho

- Source commit `759db0e3` replayed cleanly as `386e8dda` onto exact main `68d271a1`. The crates tree stayed byte-identical at `d70ef65212dc91bd6cc541a4b70104a172cf6f6d`.
- The post-replay gate is GREEN in 17.63 seconds: build 3.47 seconds, the focused RTS representation table 1/1 in 0.54 seconds, and the all-engine evaluation suite 16/16 in 13.62 seconds.
- The replayed range is rustfmt-clean and diff-check-clean. Gitleaks 8.30.1 scanned its one commit and found no leak.
