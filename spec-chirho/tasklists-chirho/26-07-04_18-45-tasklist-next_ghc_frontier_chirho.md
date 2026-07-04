<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Next GHC Frontier Chirho

- [x] Inspect current WI-010/failure catalog and pick one tractable representative.
- [x] Reproduce the representative with the current debug binary.
- [x] Implement a focused fix or document the concrete blocker.
- [x] Add focused regression coverage if code changes land.
- [x] Run targeted gates and the full probe if code changes land.
- [x] Update PRD/progress, commit, push, and release the slot.

Notes:

- `T12045a` advanced past `FreeCat :: Cat k -> Cat k` after expanding local type synonyms in standalone kind signatures.
- `T12045a` still fails later at `T2 @Type Maybe`, which needs type-family reduction for `F Type ~ Type -> Type`.
