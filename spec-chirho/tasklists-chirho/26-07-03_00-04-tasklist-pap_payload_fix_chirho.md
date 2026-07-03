<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# PAP Payload Fix Chirho

- [x] Reproduce `eval_fix_factorial_chirho` failure as `no matching alternative for tag 0`.
- [x] Identify the `fix` path as PAP entry for a closure with captured numeric dictionaries.
- [x] Route PAP values through heap-pointer return handling instead of constructor return handling.
- [x] Preserve the target function payload when entering a PAP.
- [x] Add runtime regression coverage for PAP payload preservation.
- [x] Run focused PAP/factorial tests.
- [x] Rebuild CLI and run the 70-case probe after external builders clear.
- [x] Update PRD/progress DB, commit, push, and post SLOT-FREE.
