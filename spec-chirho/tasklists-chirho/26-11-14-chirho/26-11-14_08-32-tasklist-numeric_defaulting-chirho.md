<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Real numeric defaulting after generalization

Owner HASKELUJAH/gpt_chirho, unit 754. L.J. directed continued development. Source branch next-pipeline-numeric-defaulting-chirho starts at 92430836 (source 1d6e51da, unit 753); checkpoint tag unit-754-numeric-defaulting-before-chirho. Main compiler source remains frozen. Claude2 provides bounded architectural assistance; GPT implements and executes the gates.

- [x] Recover current state, one-writer ownership, ledger and capacity; reuse the existing worktree and warm target (61.5 GiB free).
- [x] Settle the smallest faithful defaultability rule against GHC source and execute sharp oracle programs.
- [x] Implement module-end defaulting through the existing type arena, equality solver and scoped dictionary evidence. Preserve generalized/skolem variables and user-class ambiguity.
- [ ] Execute defaulted arithmetic, polymorphic reuse, nominal identity, explicit-policy declines, and budget/failure controls; run meaningful scoped gates and the unchanged 353-original profile.
- [ ] Update the implementation map/workflow, scan outgoing secrets, commit and push source, and close/sync the canonical ledger.

Placement at brick 1: defaulting belongs in the existing TC class solver/checker module close, after dependency-group generalization and before unresolved-dictionary publication/final meta zonking. It must use the original Num identity and an actual checked Num Integer instance, never spelling or unique-instance heuristics. Ordinary solver evidence remains at each Wanted's scope. Candidate proof precedes any type assignment. Any narrowing from GHC's complete rule is an explicit Unsupported boundary. Explicit defaults and additional extension profiles currently decline before TC and must never be ignored.

Finish line: ordinary resultChirho = 1 + 2 can evaluate to exact Integer 3 without an annotation; numeric functions that generalize keep their dictionary abstraction, and a user class cannot trigger numeric defaulting. Broader Num Int/Double, general default declarations, defaulting plugins, native Integer and IO-main are separate work.


Decision #34227 adopted: built-in defaulting belongs only to home modules, not ghc-internal. Only the real Num class is standard in this subset. A candidate must be a flexible bare meta mentioned only by unresolved Num constraints, with a present checked context-free Num Integer instance. All assignments and evidence use existing solvers/scopes. Explicit default declarations and additional profiles remain Unsupported. A home class on the same unary meta proves ambiguity even when the first reported wanted is Num. This reporting proof must be re-derived before variable-headed instances or named defaults are admitted.

Measured at the reviewed source: 149 next-driver tests plus 26 whole TC and 205 whole Core tests pass, none failed/ignored; all-target workspace check and build have zero compiler-message records. An initial helper omitted its ClassInfo fixture records, and early driver expectations crossed the existing Prelude-import join, used a qualified-import form, or expected rendered Bool rather than its runtime representation. Those raw failures are retained and corrected. The broad run then found two historical Unsupported assertions for ordinary literals; only those superseded cases were removed, with their remaining refusal coverage preserved. The combined filtered driver/TC attempts do not count zero selected TC tests as coverage; the separate whole TC run supplies it.

GHC r2 references cover 14 byte-exact source files: 9 compile, 5 reject (all GHC-39999; Num-first produces two diagnostics). Eleven queried values include two intentionally Unsupported policy forms; nine runtime results in the supported seven programs match the next interpreter. All 14 frozen-CLI check/Core outcomes and two PATH-restricted checks match their scoped expectations. Explicit default (), explicit default Integer, ExtendedDefaultRules and a missing rigid Num instance remain honest Unsupported cases.

The unchanged fixed 353-file profile has only tc002 and tc018 moving Unsupported to ACCEPT. Both original files separately compile under GHC 9.14.1. No other disposition changes occur. This profile omits all.T flags/ways and is not a configured GHC ratio. Frozen CLI 09bcbbfb0c149be795128cd041272d3647a387797f497d41c4dd428c448f17b0, 60,398,072 bytes, 0555, nlink 1, fresh=false.

Claude2 #34233 reviewed all 21 r1 source/fixture hashes, found no semantic or uncharged-growth defect in scope, and explained why each new control discriminates. Its read did not trace the equality solver's internal level/touchability rules and executed no gate. Two subsequent old-test expectation removals are outside that source basis; production bytes are identical. A deliberate spelling-only Num mutant is killed by the home Num control, with zero compiler messages, then classes_chirho.rs is restored byte-exact; the seven focused driver controls pass again after restoration. The final source basis has 23 files, maximum 505 lines. Existing quadratic group scans and runtime instruction/root retention remain open.

Evidence: tmp-chirho/next-defaulting-754-chirho/checkpoint-chirho/. Final Clippy attribution, source push and canonical ledger closure follow. Free space is 61.1 GiB; one existing worktree/target is reused, with no new source archive or full workspace copy.
