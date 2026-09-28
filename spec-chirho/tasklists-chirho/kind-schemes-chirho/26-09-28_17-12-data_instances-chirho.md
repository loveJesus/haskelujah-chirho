<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Data-family instance constructors

Owner: HASKELUJAH/gpt_chirho, continuing open row484 under L.J.'s direct `continue dev`. Isolated worktree, pre-beta compiler, local reversible changes. Checkpoint `row484-data-instance-before-chirho` at a15b9e28. No main, DB, published inventory, site or peer worktree writes.

Placement: a distinct `DataFamilyInstanceDeclChirho` preserves the full instance head (including its written kind and quantification), data/newtype form, constructors and deriving applications. Parsing and lowering use focused declaration modules. Type registration shares the existing constructor machinery; a family instance never rebinds the family as an ordinary datatype and never registers a type-family reduction equation. Consumers check the real family application. No filename-specific exemptions.

- [x] Preserve the instance head and constructor bodies through CST and AST, with two passing parser controls.
- [x] Connect naming, kind checking, constructor schemes and execution consumers; reproduce and repair T16188.
- [x] Verify ordinary/GADT instances, distinct instances of one family, wrong-result/wrong-kind controls and exact execution.
- [x] Run focused regression gates; retain the actual remaining failures and update the workflow.
- [ ] Finish the post-lint-cleanup family-test/lint rerun: two test launches stalled before Rust harness entry, not scored as test passes.
- [ ] Checkpoint only owned paths.
- [ ] Later integration: reconcile the new declaration with main's exhaustive occurrence walker; do-selection stays separate. No landing claim for this old kind-lane checkpoint.

Acceptance is behavioral: unchanged T16188 must typecheck, T17067 must stay accepted, illegal type-family patterns must stay rejected, and constructor execution must produce the specified value. Focused gates are not a corpus no-regression or landing claim. A broader AST feature may expose additional gaps; retain and diagnose those rather than disabling nominal family classification.

## Focused oracle predictions, before the GHC run

Fixtures live under test-data-chirho/kind-oracles-chirho/classifier-contracts-chirho/data-instances-chirho/. Predictions recorded before running: PatternGiven and Constructors should accept and run to `1\n2\n` and `7\nTrue\n`. PatternSibling should reject because the False branch cannot construct the True witness. ExpressionWanted should reject because no match supplies the equality. WrongResult should reject the constructor result not matching its data-instance head; WrongKind should reject Maybe in a Type slot.

The first driver run was 1 pass, 1 failure: constructor execution and the wrong-use control passed; unchanged T16188 still rejected twice. Local-only tracing observed `True ~ ReNotEmpty @t t80` and `True ~ ReNotEmpty @t t81` queued as wanted equalities at the tuple case pattern, with no call to the error-only refinement path. The trace has been removed. The proposed repair moves only equalities introduced by a refining pattern match into the pattern's lexical scope; expression wanteds stay wanteds. The shared rewrite traversal keeps signature givens and scope-owned pattern givens from drifting.

## Measured implementation and boundary

- GHC 9.14.1 agrees with all six predictions; unchanged T16188 also accepts. Reference JSON retains source SHA-256, exit code and combined raw output. Both positive programs also run to the predicted bytes under GHC.
- The first whole-pattern promotion prototype was unsound. A new adversarial probe `(Refl, True)` supplied an unrelated `Result c ~ Bool` from the mere presence of Refl. GHC rejects; the new driver test failed against that prototype. The repaired binder checks the constructor result BEFORE its fields, passing the actual scrutinee type outside-in. Three isolated negatives (tuple, ordinary pair, unconstrained field of a GADT) now reject. Their GHC-05617 diagnostics are retained.
- Five driver tests now pass: unchanged T16188, constructor execution and wrong use, wrong result/kind, branch-local family givens and expression wanteds, and the three unrelated-proof shapes. The two positive programs produce exact GHC output through STG, LLVM and Cranelift.
- Pattern-family scope controls cover flexible and rigid arguments, preserve preceding wanteds, and prove that equality of family results never equates the arguments. A given expires with its pattern scope.
- Shared constructor registration, pattern binding and one-pass type rewriting live in focused modules. The large inference root shrank by over 500 lines; no parallel constructor-pattern implementation was added.
- The supported slice is top-level DATA instances. Newtype representation and deriving are retained but explicitly diagnosed as unimplemented; no rejection credit is claimed for those implementation gaps. Explicit quantified instance heads, full imported constructor metadata, associated-instance representation and coercions are not claimed.
- On the outside-in implementation: AST 7, Core 128, naming 138, parser 377, typing 412: 1,062 unit tests passed, zero failed/ignored/filtered and no compiler warnings. Full typing integration: 478 passed, zero failed/ignored/filtered, 333.71s, no compiler warnings. Formatting and diff-whitespace checks pass. Earlier 58-family-test and 1,062-unit runs before the outside-in correction are superseded.
- Claude2 independently reproduced all seven GHC fixtures, source hashes, error codes/positions and both exact executions in room receipt #25109. This is independent GHC evidence, not independent verification of this branch's compiler.
- All-target Clippy on the six directly changed packages completed with 313 distinct location/message warnings, so this is NOT a clean-lint claim. One warning had its primary location on a new/changed line: the extracted constructor helper carried `repeat().take()`. Replaced with `repeat_n()` without suppressions; post-cleanup family tests and lint recheck are pending. All other changes after the 478-test run are comments. The initial Clippy launch stalled before Rust entry at `_dyld_start`; it was terminated, its version command succeeded, and the rerun completed. No cause is assigned to that launch stall.
- The final-source family-test command rebuilt successfully with no compiler warnings, then its executable stalled before harness entry twice. PID-specific samples of 88590 and 5025 showed only `_dyld_start`, 112K footprint and zero CPU; no test began. Both were terminated with SIGTERM and Cargo returned 101, so these are interrupted attempts, not passing gates. The final Clippy retry likewise never emitted a check and was terminated. Broad `ps` queries also stalled, while PID-specific `ps`, `pgrep`, Git and broker HTTP worked. Two owned `ps` probes remained kernel-pending after SIGTERM/SIGKILL; no peer process, machine restart or security setting was touched. Final runtime/lint verification remains explicitly open rather than relabelling the pre-cleanup gates.
- Retained local gate logs live under repository-root-relative `tmp-chirho/data-instance-chirho/`, not OS tmp. No corpus totals or published compatibility claims were changed.
