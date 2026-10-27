<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 599: Result-only Num propagation Chirho

- [x] Reserve Row 599 in the canonical progress database on `main_chirho`.
- [x] Measure the result-only `Num b` conversion shape on GHC 9.14.1 and the interpreter, LLVM and real Cranelift.
- [x] Trace where `convert :: (Integral a, Num b) => a -> b; convert = fromIntegral` loses or defaults its result dictionary.
- [x] Post a bounded design checkpoint before compiler source changes, including the overlap audit against Rows 594, 597 and 598.
- [x] Implement only the proven result-type evidence repair and add structural plus all-engine controls.
- [ ] Mutation-prove the repair, run the focused warm gate and land through the canonical single-writer chain.

## Scope Chirho

- Directly annotated `fromIntegral` and monomorphic aliases already became exact in Row 596 and must remain holds.
- This row owns only a separately declared result-only polymorphic `Num b` predicate instantiated at a concrete call-site result.
- Rows 594 and 597 own Prelude-name resolution. Row 598 owns qualified-name resolution. Their files and mechanisms stay outside this row.

## Measured baseline Chirho

- GHC 9.14.1 prints exact `Int -> Integer` values and the expected rounded `Int -> Double` values.
- The interpreter prints the first Integer value, then stops at `ShowFloatChirho: expected Double#, got 42#`.
- LLVM and real `--cranelift` print `2.08e-322` for 42 at Double and `[-0.0,NaN,0.0,NaN]` for the mapped Double conversion.
- The generated `fromIntegral` already takes Integral and Num dictionaries. The enclosing `convertChirho` keeps only Integral because the dictionary pass defaults its result-only Num predicate to Int.
- A local generalized literal with no checker `$own` evidence remains a measured hold: GHC prints `(42.0,42)`, while main stops in the interpreter and prints `2.08e-322` natively for the Double result.

## Bounded repair Chirho

- Keep a result-only `Num` predicate only when a free constrained reference in that binding body carries the exact existing `$own:<predicate-index>` evidence for it.
- Reuse one body-aware inventory for both caller pre-seeding and definition transformation.
- With no exact body proof, preserve today's defaulting. This avoids the prior blanket change that made ordinary entry points into unapplied dictionary functions.
- The mechanism is disjoint from Rows 594 and 597's name ownership and Row 598's qualifier resolution.
- Reservation base: `09c49012f69f46b0a169ac3e7097b6c0b9f5460e`.
- Result-only `Enum` remains outside the row: the interpreter prints `97`, while both natives already print GHC's `'a'`. Row 599 keeps its `Num` bound rather than widening a partial route.

## Evidence Chirho

- The candidate Core takes `$dIntegral` and `$dNum` dictionary lambdas; each call supplies its independently proven Integral and Num dictionaries.
- The four-line GHC control is byte-identical on the interpreter, LLVM and marker-verified real Cranelift.
- The hook-only mutation drops the retained indices from `[0,1]` to `[0]`, fails the structural control, and restores the interpreter's `ShowFloatChirho: expected Double#, got 42#` failure. The source was restored byte-identical.
- Core library: 185/185. Dictionary-evidence integration target: 117/117. Row 596's direct and monomorphic control, result-only Applicative/Monad, and the result-only Enum boundary all pass.
- Clippy reports no diagnostic on a changed line; rustfmt and diff-check are clean.
- Claude's B8 local-literal hold is unchanged on the final source: the interpreter stops at `ShowFloatChirho`, and both natives print `(2.08e-322,42)`.
