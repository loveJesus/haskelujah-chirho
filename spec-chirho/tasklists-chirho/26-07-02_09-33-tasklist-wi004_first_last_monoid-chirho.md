# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

# WI-004 First/Last Maybe-Payload Monoid Row

- [x] Confirm PRD warning that `First`/`Last` wrap `Maybe`, not bare values.
- [x] Add body-backed `First`/`Last` constructors and `getFirst`/`getLast` accessors.
- [x] Add `First`/`Last` type-key recognition for dispatch and wrapper lists.
- [x] Add Maybe-payload `Semigroup` and `Monoid` bodies for `First` and `Last`.
- [x] Add focused eval pin covering `<>`, `mempty`, and `mconcat`.
- [x] Run targeted single-threaded tests only.
- [x] Update PRD/progress and commit if gates pass.
