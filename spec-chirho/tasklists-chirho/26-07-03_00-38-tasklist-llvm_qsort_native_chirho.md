<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# LLVM Qsort Native Regression Chirho

- [x] Confirm the representative `llvm_round_trip_qsort_random_500_chirho` failure on current HEAD.
- [x] Localize whether the failure comes from list generation, filter, append, qsort recursion, closure capture, or native runtime printing.
- [x] Apply the smallest backend/runtime fix if the root cause is isolated.
- [x] Run targeted LLVM regression tests and the 70-case probe if code changes land.
- [x] Update `spec-chirho/prd_chirho.json` and progress notes with measured results.
