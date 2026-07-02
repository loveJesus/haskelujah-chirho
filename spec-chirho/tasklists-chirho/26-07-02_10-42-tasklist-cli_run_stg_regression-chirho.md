<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# CLI Run STG Regression Chirho

- [x] Confirm Claude's full-probe regression report: native-default CLI `run` printed raw pointers/thunks for ordinary values.
- [x] Restore `haskelujah run` to the STG interpreter path.
- [x] Keep explicit native execution on `haskelujah compile`.
- [x] Rebuild the CLI with `CARGO_BUILD_JOBS=1`.
- [x] Target-check the pointer-regression repros: show-string, maybe-do, either-fmap, and mapMaybe.
- [x] Target-check the native LLVM higher-order operator round-trip.
- [x] Update PRD v2.2.26 with the rollback and pending full-probe acceptance gate.
