<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Evidenced sum and product Chirho

Row 552 repairs the silent native wrong answers and interpreter failures caused by the
Int-shaped generic `sum` and `product` bindings when their element type is `Double`.

## Contract

- A `sum` or `product` occurrence with a solved `Num K` proof becomes a `foldl` over the
  body-backed `Num` operator and `fromInteger` rows for exactly `K`.
- An unapplied occurrence uses the same replacement, so `map sum` and `map product` keep
  their element type.
- A missing proof, dangling name-table entry, generated missing-method body, or missing
  `foldl` body keeps the existing canonical route. No key is guessed.
- `maximum` and `minimum` stay in the later Ord aggregate brick. Numeric sections,
  type-keyed nested primops, and negative-literal precedence stay in Rows 553–555.

## Placement

- `evidence_join_chirho.rs` opts `sum` and `product` into per-occurrence evidence.
- `rewrite_chirho/evidenced_aggregates_chirho.rs` owns the replacement and shares the
  existing Row 550 evidenced-value hook.
- `occurrence_rows_tests_chirho.rs` proves dangling names do not count as bodies.
- `native_error_helpers_chirho.rs` carries the GHC-measured all-engine contract.
- `print-show-evidence-chirho.md` records the workflow and fail-closed boundary.

## Checklist

- [x] Measure Double sums, products, empty identities, first-class uses, and Int/fold holds
  against GHC 9.14.1.
- [x] Implement the applied and first-class evidence route in one child module.
- [x] Reject dangling IDs and missing-method bodies with body-backed lookups.
- [x] Preserve Row 551's compare hook while replaying onto `075ce045`.
- [x] Run the warm focused proof: core 169/169; native error helpers 36/36;
  dictionary evidence 81/81; numeric hierarchy 6/6; show precedence 2/2;
  IO actions 9/9; typing integration 43/43.
- [x] Verify zero changed-line Clippy findings, rustfmt, diff-check, and gitleaks.
- [x] Land source tip `d3aa44c9` and record progress row 552.
