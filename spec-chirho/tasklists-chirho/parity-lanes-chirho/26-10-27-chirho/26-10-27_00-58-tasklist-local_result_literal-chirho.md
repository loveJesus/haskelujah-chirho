<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 600: Local result literal Chirho

- [x] Reserve Row 600 in the canonical progress database on `main_chirho`.
- [x] Measure the locally generalized result-only `Num` literal on GHC 9.14.1 and the interpreter, LLVM and real Cranelift.
- [x] Trace the local binding's inferred scheme, literal evidence and dictionary-pass route.
- [x] Post a bounded design checkpoint and overlap audit before compiler source changes.
- [x] Implement only the proven local-evidence repair with structural and all-engine controls.
- [x] Mutation-prove the repair and run the focused warm gate.
- [ ] Land through the canonical single-writer chain and close Row 600.

## Scope Chirho

- The mover is `totalChirho :: Num a => [a] -> a; totalChirho xs = sum xs where sum _ = 42`, instantiated at Double and Int.
- Row 584's explicitly signed rigid literals and Row 599's body-proven result-only Num reference remain holds.
- Rows 594 and 597 own generated Prelude name resolution. Row 598 owns qualified-name resolution. Their mechanisms and files stay outside this row.
- Reservation base: `686001ca`.

## Baseline and trace Chirho

- GHC 9.14.1 prints `(42.0,42)`. The interpreter stops at `ShowFloatChirho: expected Double#, got 42#`; LLVM and real Cranelift both silently print `(2.08e-322,42)`.
- The dumped Core keeps `totalChirho`'s `$dNum`, but the local `sum` body is `fromInteger $fNumInt 42`. Its locally generalized result variable is recorded in `local_generalized_vars_chirho`, and instantiation records relate it to the enclosing signature skolem.
- `local_specialization_key_chirho` follows those records but accepts only a concrete head at the end. The enclosing `ForallVarChirho` therefore yields no literal evidence, and the dictionary pass falls back to `Num Int`.

## Bounded repair Chirho

- The literal-evidence finalizer may translate a locally generalized variable's one agreeing instantiation at an enclosing signature skolem into that exact predicate's `$own:<index>` key, through `own_parameter_key_chirho`.
- Concrete local specialization keeps its current path. Multiple, open, cyclic, instance-method-skolem or disagreeing instantiations keep no evidence. Reference and method occurrence channels stay unchanged in this row.
- The dictionary pass already consumes literal `$own:<index>` evidence under the enclosing binding's dictionary. No local Core dictionary lambda, name-resolution change or backend change is required.

## Reach Chirho

- Removing only the rigid-skolem endpoint makes the structural test fail with `None` instead of `$own:0` and restores the mover's interpreter failure at `42#`.
- The file was restored byte-identical (`57932aa8bb4f7f145384ac1c79429af90b86c8bb8822936bd670811a3952ef2c`). Row 584's rigid-literal and Row 599's result-only-reference controls remain green.

## Gate r1 Chirho

- GREEN on source commit `66f240d7` in about 184 seconds.
- Build completed with no warning. Typing and core libraries passed 550/550; dictionary evidence passed 118/118; the driver evidence/literal/generalization slice passed 79/79, including its 99-second property-test tail.
- Nextest marked the existing `a_section_under_iterate_stays_the_named_residual_chirho` process as `LEAK` while recording it PASS; no test failed or was skipped in the selected suites.
- Clippy emitted no diagnostic on either changed Rust file. Rustfmt, diff-check and gitleaks over the source commit are clean.
