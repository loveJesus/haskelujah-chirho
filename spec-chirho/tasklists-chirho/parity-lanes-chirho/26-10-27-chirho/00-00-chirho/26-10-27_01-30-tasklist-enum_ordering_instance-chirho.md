<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 602: Enum Ordering instance Chirho

- [x] Reserve Row 602 in the canonical progress database on `main_chirho`.
- [x] Measure `Ordering` enumeration on GHC 9.14.1 and the interpreter, LLVM and real Cranelift.
- [x] Trace the typing instance seed and compiler-owned body-backed Enum rows.
- [x] Post a bounded design checkpoint and overlap audit before compiler source changes.
- [x] Implement only the missing `Enum Ordering` instance and required method bodies.
- [x] Mutation-prove the repair.
- [x] Run the focused warm gate.
- [x] Land through the canonical single-writer chain and close Row 602.

## Scope Chirho

- Cover `succ`, `pred`, `toEnum`, `fromEnum`, and finite enumeration ranges for `Ordering` where GHC defines them.
- Keep `Ord Ordering`, extrema such as `maximum [LT, GT, EQ]`, generated-Prelude ownership and qualified-name resolution outside this row.
- Reservation base: `71b0dbf9`.

## Evidence Chirho

- GHC 9.14.1 prints `EQ`, `EQ`, `LT`, `EQ`, `GT`, `(0,1,2)`,
  `[LT,EQ,GT]`, and `[GT,EQ,LT]` for the accepted control. Before this row,
  every occurrence was rejected with E0204, `no instance for Enum Ordering`.
- GHC's endpoint messages are `Prelude.Enum.Ordering.succ: bad argument`,
  `Prelude.Enum.Ordering.pred: bad argument`, and
  `Prelude.Enum.Ordering.toEnum: bad argument`.
- The typing seed lacked only the ground `Enum Ordering` instance. The dictionary generator
  already builds `$fEnumOrdering` once its four `$prim_Enum_*_Ordering` rows have bodies.
- Native LT/EQ/GT values already are scalar 0/1/2. The interpreter's boxed constructors need a
  bounded adapter at `<#`, `>#`, `-#`, and `enumStepChirho#` inside finite-range generation.
  Its out-of-range one-step sentinel is compared before it can enter a result list.
- Row 597's dictionary rewrite paths and row 598's desugar/name/type-environment paths are not
  touched.
- With only the interpreter adapter hook removed, the normal control fails at
  `GtIntChirho: expected Int#` while the Char range hold remains exact. Restoring the hook makes
  the source file byte-identical (SHA-256 `cd9a9f9c469e779f267df734470576d3b55156231fba4180afba1244ea18a2ff`),
  rebuilds without warnings, and restores the eight-line GHC answer.
- Gate r1 is GREEN: 1,107/1,107 focused tests in 175.84 seconds, including the interpreter,
  LLVM, real Cranelift, every Row 602 control, and Row 597's generated-Prelude ownership control.
  The 41-program contract judge is GREEN at 126/126, accounting for 73 movers and 9 holds with no
  Row 602 movement. Clippy reports no warning on a changed line; rustfmt, diff-check and gitleaks
  are clean.
