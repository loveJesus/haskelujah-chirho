<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 587: scaleFloat Double and argument keys Chirho

- [x] Reserve progress row 587 and branch from exact main `52e5f3ed`.
- [x] Measure `scaleFloat` on GHC 9.14.1 and exact main across the interpreter, LLVM, and Cranelift.
- [x] Give the Int exponent and Double value distinct, explicit rewrite keys.
- [x] Implement one measured Double scaling routine shared by all three engines.
- [x] Add edge, ownership, first-class, structural, and argument-key controls.
- [x] Kill the scaling route and the argument-key route with narrow mutations.
- [x] Run the warm focused gate, changed-line Clippy, rustfmt, diff-check, and gitleaks.
- [x] Commit only Row 587 paths, replay onto current main, fast-forward, and close progress row 587.

## Boundaries Chirho

- This row owns `scaleFloat :: Int -> Double -> Double` and only the per-argument key needed by that generated concrete row.
- `decodeFloat` and `encodeFloat` remain separate RealFloat work.
- Row 584 owns rigid-signature literal evidence. Row 586 owns dictionary-passing `sum` and `product`. This row does not change either route.
- The Double value keeps its existing result key. The Int exponent must never inherit that key; a negative exponent therefore retains Int `negate#` rather than becoming `negateFloat#`.

## Initial measurement Chirho

- GHC maps `scaleFloat 3` over zero, signed zero, ordinary values, `1.0e-300`, Infinity, and NaN as `[0.0,-0.0,12.0,-20.0,8.0e-300,Infinity,NaN]`.
- GHC maps `scaleFloat (-3)` over `1.5`, `-2.5`, and `1.0e-300` as `[0.1875,-0.3125,1.25e-301]`.
- The stopped Row 585 draft proved the key defect before commit: applying the result key `Double` to the exponent rebuilt its `negate#` as `negateFloat#`. The interpreter failed loudly, while a native route would silently return the wrong value.
- Exact main has no `scaleFloat` binding on any engine. GHC 9.14.1 and the candidate are byte-identical across ordinary and negative exponents, `1023`, `1024`, `-1074`, `-1075`, `maxBound`, `minBound`, signed zero, the minimum subnormal, Infinity, NaN, and first-class `map (uncurry scaleFloat)`.

## Implementation and reach Chirho

- The body-backed Double row is `Int -> Double -> Double`; the generated-row application and the primitive rebuild each preserve the exponent's Int key independently. A program-owned function merely named `scaleFloat` is not special-cased and may keep a Double first argument.
- The interpreter, LLVM, and Cranelift call one RTS implementation. It uses C `scalbn`, clamping the Haskell `Int` exponent to C's `int` range so huge positive and negative exponents overflow or underflow instead of wrapping.
- Mutation S1 added one to the scaling exponent. The RTS table failed at `12.0` versus `24.0`.
- Mutation K1 removed the generated-row application key. A freshly rebuilt CLI failed the negative exponent at `NegFloatChirho: expected Double#, got 3#`; the restored tree returns GHC's values.

## Gate Chirho

- The warm behavior gate is GREEN in 17.84 seconds: build 2.21 seconds; 754/754 tests across the RTS, runtime, core, typing, LLVM, and Cranelift libraries in 6.21 seconds; evaluation values 18/18 in 9.42 seconds.
- Clippy completed on every affected package and target. It first found the new structural test above non-test items; moving that test module to the file's end cleared the changed-line finding. The final run has no diagnostic on a Row 587 changed line; remaining warnings predate this range.
- `cargo fmt --all -- --check` and `git diff --check` are clean. Gitleaks runs on the committed range before replay.

## Replay Chirho

- Source commit `31806bc4` replayed cleanly as `1568b1d2` onto exact main `0ac0e6df`, after Row 584 landed. All fifteen non-overlap path blobs are byte-identical; the shared dictionary-evidence workflow keeps both rows' text.
- The post-replay gate is GREEN in 23.80 seconds: build 3.58 seconds; RTS scale table 1/1 in 0.54 seconds; structural key control 1/1 in 1.06 seconds; evaluation values 18/18 in 12.46 seconds; and Row 584's signature evidence controls 2/2 in 6.16 seconds.
- The replayed range is rustfmt-clean and diff-check-clean. Gitleaks 8.30.1 scanned its one commit and found no leak.

## Placement Chirho

- Extend the existing `real_number_methods_chirho.rs` generated rows; do not add a module or directory entry.
- Put the pure Double scaling routine beside Row 585's representation routines in `haskelujah-rts-chirho/src/float_math_chirho.rs`.
- Keep the per-argument rule beside the existing typed rewrite traversal in `method_args_chirho.rs`, with a structural control that sees the Int exponent remain Int.
