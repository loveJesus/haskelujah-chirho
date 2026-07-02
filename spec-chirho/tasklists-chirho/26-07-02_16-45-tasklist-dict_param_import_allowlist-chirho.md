<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Dict Param Import Allowlist Chirho

- [x] Read Claude's full-probe regression report for `d09a4735`.
- [x] Replace broad module-env dict-param seeding with an explicit imported-export allowlist.
- [x] Thread the allowlist through multi-module compilation paths while leaving normal single-module compilation empty.
- [x] Re-run the three multi-module constrained export representatives.
- [x] Build a fresh CLI and verify representative probe regressions no longer print raw dict selectors.
- [x] Run the 70-case probe locally and restore PASS=70 FAIL=0.
- [x] Update `spec-chirho/prd_chirho.json` with v2.2.58 regression/fix notes.
