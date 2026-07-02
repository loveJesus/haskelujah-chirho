<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# IO Primop Argument Lowering Chirho

- [x] Confirm the existing focused IO bind failure without running broad suites.
- [x] Patch STG lowering/runtime so IO primop names used as values do not become literal `0#`.
- [x] Add or update targeted regression pins for `getLine >>= putStrLn` and do-bound `getLine`.
- [x] Run only targeted tests with `CARGO_BUILD_JOBS=2`.
- [x] Update `spec-chirho/prd_chirho.json` and `progress-chirho.sqlite`, then commit.
