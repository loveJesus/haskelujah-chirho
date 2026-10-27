<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 592: exact minimum-word literal Chirho

## Bound

Make the directly written machine-word minimum, `-9223372036854775808`, retain its value through parsing, prefix-minus fixity, desugaring and all three execution engines. Keep positive `9223372036854775808` and larger magnitudes at the existing bounded-integer limitation. Do not absorb the separately measured `fromIntegral (minBound :: Int) :: Integer` conversion defect.

## Plan

- [x] Measure GHC 9.14.1 and main `38c5201d`: direct minimum literals become `0` on all three engines, while subtraction and `toInteger minBound` are exact.
- [x] Reserve progress row 592 and rebase the isolated worktree over landed row 583.
- [x] Preserve the one otherwise-unrepresentable magnitude only beneath a direct prefix-minus CST node, without changing positive overflow.
  - Decimal, hexadecimal, octal and binary spellings all carry the same marker. If fixity absorbs a tighter operator, the marker returns to the existing positive-overflow boundary instead of entering the chain as `i64::MIN`.
- [x] Desugar that marked negative literal through `fromInteger` without executing `negate#` on `i64::MIN`.
- [x] Add parser/fixity and all-engine GHC controls; pin the positive-overflow and `fromIntegral minBound` boundaries.
  - The exact control covers Int, Integer, Double, `encodeFloat`, each supported radix, subtraction and `toInteger minBound` on the interpreter, LLVM and real Cranelift.
  - `fromIntegral minBound` remains separately wrong: the interpreter prints `-9223372036854776000` and the natives print the raw-word `-4332462841530417152`; this row does not move it.
- [x] Mutation-check the parser marker and Core bypass, then run the warm focused gate with changed-line Clippy, rustfmt, diff-check and gitleaks.
  - Marker mutation: parser grouping and the all-engine control fail, with direct literals returning to zero.
  - Core-bypass mutation: the all-engine control aborts at an attempted `i64::MIN` negation.
  - Gate r1 was ABNORMAL on the nested-parentheses parser proptest with a default-stack overflow. Exact main `8318aad6` aborts identically. With the test thread stack set to 32 MiB, the exact test and all 390 parser tests pass.
  - Focused remainder: core 183/183, evaluation values 23/23 and dictionary evidence 115/115 in 95.22 seconds. Clippy reports no diagnostic on a changed line; rustfmt and diff-check are clean.
  - Gitleaks scanned the staged source and docs: no leaks.
- [x] Commit the source and gate record; replay onto current main.
  - Source tip `105dd013` is a clean replay over main `8318aad6`. Its crates tree `d73fc0977287cc4670eab4903402a9aca9120412` is byte-identical to gated pre-replay source `4abfbf75`.
  - Post-replay parser and all-engine controls pass; diff-check and the one-commit gitleaks scan are clean.
- [x] Land on main and close progress row 592.
  - Fast-forwarded source `105dd013` and gate record `9289fa67`; the progress closure is the following docs-only commit.

## Placement

- Prefix-minus marker and parser control: `haskelujah-parser-chirho/src/lower_chirho/expr_syntax_chirho/`.
- Core lowering: the existing negation arm in `haskelujah-core-chirho/src/desugar_chirho.rs`.
- Three-engine control: the existing RealFloat codec test module, because `encodeFloat` makes the lost Integer bit pattern observable.
- Workflow update: `compiler-pipeline-chirho/operator-fixity-chirho.md`.
