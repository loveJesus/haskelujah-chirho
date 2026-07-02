# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

# WI-004 Proxy Functor Applicative Monad Body-Backed Row

- [x] Confirm `Proxy` is class-registered but lacks body-backed Functor/Applicative/Monad generated methods.
- [x] Add `Proxy` type-key recognition for dispatch.
- [x] Add body-backed `Proxy` Functor/Applicative/Monad generated methods.
- [x] Add focused eval pin for `fmap`, `pure`, `<*>`, `>>=`, and `>>`.
- [x] Run targeted single-threaded tests only.
- [x] Update PRD/progress and commit if gates pass.
