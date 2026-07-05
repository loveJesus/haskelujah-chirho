<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Higher Order Payload Compare

- [x] Reproduce the `sortBy compare [Int]` failure and confirm it dispatches `compare` at `[Int]` instead of `Int`.
- [x] Patch higher-order typed argument rewriting to prefer payload keys for callable method/dict arguments.
- [x] Recheck the `sortBy`/`insertBy`/`maximumBy`/`minimumBy`/`groupBy` targeted tests.
- [x] Run focused core/driver gates plus warm-up and full probe.
- [x] Log progress before committing only touched files.
