# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

# WI-004 Ordering Monoid Body-Backed Row

- [x] Confirm no heavy cargo/rustc/haskelujah/ghc_bulk jobs are running before work.
- [x] Verify `Ordering` is class-registered but bodyless for Semigroup/Monoid.
- [x] Add body-backed `Ordering` Semigroup/Monoid generated methods.
- [x] Add focused eval pin for `<>`, `mempty`, and `mconcat`.
- [x] Run targeted single-threaded tests only.
- [x] Update PRD/progress and commit if gates pass.
