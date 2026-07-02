# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

# WI-004 Const Functor Body-Backed Row

- [x] Confirm PRD Wave A lists `Const(fmap only)` and excludes context-dependent Applicative/Monad rows.
- [x] Add body-backed `Const` and `getConst` imported newtype erasure.
- [x] Add `Const` type-key recognition for dispatch.
- [x] Add body-backed `Functor Const` generated `fmap` method.
- [x] Add focused eval pin for `fmap` preserving the Const payload.
- [x] Run targeted single-threaded tests only.
- [x] Update PRD/progress and commit if gates pass.
