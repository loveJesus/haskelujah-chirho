# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

# WI-004 Identity Functor Applicative Monad Body-Backed Row

- [x] Confirm no heavy cargo/rustc/haskelujah/ghc_bulk jobs are running before work.
- [x] Verify `Identity` is class-registered but bodyless for Functor/Applicative/Monad.
- [x] Add body-backed `Identity` and `runIdentity` imported newtype erasure.
- [x] Add body-backed `Identity` Functor/Applicative/Monad generated methods.
- [x] Add focused eval pin for `fmap`, `pure`, `<*>`, `>>=`, and `>>`.
- [x] Run targeted single-threaded tests only.
- [x] Update PRD/progress and commit if gates pass.
