<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 596: Integral conversion parity Chirho

- [x] Reserve Row 596 in the canonical progress database.
- [x] Measure the direct minimum-word literal and the `toInteger` / `fromIntegral` conversion routes on GHC 9.14.1 and all three Haskelujah engines.
- [x] Trace the wrong route in Core: the polymorphic source reference reaches the generated `Int -> Double` `fromIntegral` binding.
- [x] Replace that monomorphic binding with the standard dictionary composition `fromInteger . toInteger`, consuming the checker’s two predicate proofs in scheme order.
- [x] Add structural and all-engine controls for Int-to-Integer exactness, Int-to-Double preservation, first-class use, and downstream `encodeFloat`.
- [x] Prove reach by removing the conversion route and observing the controls fail.
- [x] Run the focused warm gate, changed-line Clippy, rustfmt, and diff review; run gitleaks over the committed range before handoff.
- [x] Commit the source and gate evidence, then hand the tested tip to the canonical landing chain.

## Placement Chirho

- Keep the conversion builder in `dict_chirho/prelude_chirho/real_number_methods_chirho.rs`; `prelude_chirho.rs` is already 17k lines and must shrink, not grow.
- Reuse `reference_evidence_chirho`, which already carries the `Integral` source and `Num` result predicates separately.
- Do not touch Rows 593-595 or broaden this row to unsupported `Integral` representations.

## Named residual Chirho

- A separately declared result-only polymorphic `Num b` predicate still follows the pass's existing Int-default policy: `convert :: (Integral a, Num b) => a -> b; convert = fromIntegral` therefore remains a separate result-type-propagation row. Concrete result annotations on `fromIntegral` and monomorphic aliases are in this row and are exact.

## Reach and focused gate Chirho

- The reach mutation restored only the former monomorphic `Int -> Double` binding. The all-engine control failed immediately: the interpreter rounded both machine-word endpoints, `encodeFloat` returned `0.0`, and the native cells could no longer meet the GHC output. Restoring the candidate made the control pass again.
- `cargo build -p haskelujah` completed with zero warnings.
- The core library passed 184 of 184 tests. The complete `dictionary_evidence_chirho` integration binary passed 116 of 116 tests on six threads, including the interpreter, LLVM and Cranelift control.
- `cargo fmt --all -- --check` and `git diff --check` are clean. Clippy completed successfully for the core and driver test targets; it reported no diagnostic on a changed line. Three pre-existing diagnostics in untouched lines of `prelude_chirho.rs` remain outside this row.
- Source commit `4e57d152` rebased cleanly onto canonical `main_chirho` at `7f79fa8c`. The structural control and all-engine control passed again on the combined tree, `git diff --check` stayed clean, and gitleaks found no leak in the rebased source commit.
