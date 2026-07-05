<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# Strict Numeric Argument Defaults Chirho

- [x] Reproduce IORef/STM/Data.Map strict argument failures.
- [x] Identify where `fromInteger` survives into STG in strict value positions.
- [x] Patch the narrow dict-pass/defaulting path without broad numeric rewrites.
- [x] Run targeted watch-surface tests and probe.
- [x] Commit, push, and hand off for watch-surface acceptance.

## Follow-Up Churn Chirho

- [x] Narrow generated Map/IORef/STM strict defaults so source-level functions named `mapInsert` are not rewritten.
- [x] Re-run original six fixed targets plus four Map/BST churn cases.
- [x] Re-run fresh CLI warm-up and 70-case probe.
