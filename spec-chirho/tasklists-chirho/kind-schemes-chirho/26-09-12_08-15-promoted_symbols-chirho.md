<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Parenthesized promoted symbols, row484

Checkpoint86fb321d preserves the tuple implementation and frozen885/248
diagnostic. Ten main-relative accept regressions remain; main and DB/artifacts
stay untouched. This is a bounded follow-up to the independent parser review,
not a substitute for recovering those regressions.

## Brick 1

Hypothesis: '(:) is dispatched as a unary tuple and lost at both structured and
flat routes. First run unchanged prefix, qualified-prefix, record-field and
wrong-tail controls against GHC9.14.1 and the frozen1c183b98 CLI. If confirmed,
recognize a constructor symbol inside the already-owned parentheses and retain
it in the existing PromotedConChirho representation. Keep tuple operands on the
tuple path, reject malformed/value-operator syntax, and retain following
declarations. Reuse the existing focused promoted parsing/lowering modules;
no new AST shape or type/kind-policy waiver is planned.

The parser repair exposed a second independently referenced failure: qualified
and unqualified spellings of the same local promoted constructor compare as
different type constants. Extend the existing kind-elaboration handoff with an
exact alias map derived only from the module's registered constructors. Type
conversion can then use the local identity for that proven self-qualification;
unknown or foreign qualifiers must remain distinct. This is not blanket
qualifier stripping or complete imported-constructor alias resolution. The map
is built once per module and looked up by full spelling, not rebuilt per use.

- [x] Fresh GHC/reference and baseline controls establish the reachable defect.
- [x] Parser regression controls distinguish symbols from tuple operands.
- [x] Repair both routes; preserve the symbol namespace, qualifier and span.
- [x] Focused and broader parser/typing/integration/CLI gates.
- [x] Checkpoint/push and run the current full driver target.
- [ ] Return to main-relative regressions; final landing gates remain required.

This local/reversible pre-beta choice is within L.J.'s continue authority. A
reference failure changes the probe, not the language rules. No corpus gain is
predicted from this review follow-up.

## Measured state

Frozen1c183b98 accepts both invalid Bool-tail sources as well as the four valid
sources. GHC9.14.1 rejects both invalid tails with GHC-83865 and accepts the
other four. Adding AST controls makes the structured and flat tests fail on the
missing colon; the family-tuple control stays green (1/3,363 filtered). The
first four driver controls run3/4 on the old parser: the invalid-tail assertion
fails. Parser preservation alone then makes the valid qualified/local identity
control fail with E0200. Exact owned aliases repair that mismatch without
stripping foreign qualifiers. All six focused driver cases now pass.

Full parser366/366, typing381/381, integration206/206 and canaries7/7 pass with
zero ignored/filtered. Explicit CLI build, workspace all-target check and format
check exit0. Clippy exits0 with467 warning-message lines including duplicate
and summary notices; this is not lint-clean. No clippy message points at the
new promoted-symbol/alias paths. The fresh CLI agrees with all six unchanged
reference sources, requiring E0300 for each invalid tail. Its SHA256 is
42372d48759a313d6644da2fc0b3adbd736b4f25ee24220e95d0f62df3fe56c7.
Evidence is under
test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho.

The first qualified probe attempted to import (:) from GHC.Types and Prelude;
neither exports it under GHC9.14.1. The retained reference instead defines a
local symbolic constructor and checks its legitimate self-qualified spelling.
No expected verdict was copied from our output. Malformed CST recovery and
complete import-alias resolution remain outside this checkpoint's claim.

The full current driver target, full workspace execution, and final two-pass
corpus gates are still owed. Earlier1774/1774 belongs to a426, not these edits.
The last frozen corpus diagnostic remains885/248 at1c183b98 and is not a
measurement of this newer source. Main and the published artifacts remain put.

## Associated producer follow-up

Checkpoint5b954deb is pushed and remote-exact. Read-only review identified that
associated instance equations and class defaults still use the context-free
converter, unlike top-level equations. This is a hypothesis until unchanged
GHC/reference and frozen5b probes exercise the mismatch. Reuse the existing
converter with a borrowed, exact local identity map if confirmed; imported
syntax must not inherit the consumer module's aliases. Keep associated binder
semantics unchanged, and move its registration block out of the oversized
infer root into the existing family-declarations module rather than extending
that root. Sources/evidence live in the promoted-symbols associated leaf.

- [x] Reference and frozen-before controls for associated patterns, RHS/defaults.
- [x] One promoted-identity conversion for all owned producers; negative controls.
- [x] Gate, checkpoint and push this repair before the current full driver run.

Six unchanged sources are now independently checked by GHC9.14.1: three valid
associated pattern/result/default cases and three invalid coordinate swaps.
Frozen5b correctly checks the pattern pair but rejects the valid result/default
cases on qualified constructor spelling. Its two invalid result/default cases
also reject on that spelling, not Int/Bool. The focused suite proves sensitivity:
8/12 pass, four fail,200 filtered. This is not a broad gate.

The first default draft left its class parameter's kind implicit. GHC rejected
the default with GHC-41522 because it specialized a hidden kind argument. Both
retained default probes explicitly bind the parameter at Type and were rerun;
the invalid draft does not establish a compiler obligation.

Final associated checkpoint gates: parser366, typing381, integration212,
canaries7, all green with zero ignored/filtered. All12 unchanged reference
sources agree on the explicit CLI; each coordinate negative now reports
Int/Bool E0200 instead of qualified-spelling E0200. CLI SHA256:
45609f0461430cd18d723a3eab86ff082d4fd4e473fe6ef885e0310ab23bb7ab.
Format, workspace all-target check and explicit build pass without compiler
warnings. Selected clippy completes with467 warning-message lines including
duplicates/summaries; not lint-clean. Commands and log hashes are in
promoted-symbols-chirho/associated-gates-chirho.json. The first crate gate
could not compile because a pre-existing unit test used the removed root
re-export; it now names its owning module explicitly. That failed attempt is
not counted as a test run. The full driver and corpus were still owed at that
checkpoint; the frozen results below supersede that pending status.

## Frozen current driver and corpus diagnostic

Compiler checkpoint8c30a457 is pushed and remote-exact. Its unchanged source
passes the full driver library target:1774 passed,0 failed,0 ignored,0 filtered,
with the documented16MiB Rust test-thread stack. The test body took368.67s;
the bounded build/test command took374838ms. There were0 compiler-warning
message lines in this run. The retained driver JSON names stdout and stderr
separately, hashes each, and identifies the combined stdout-then-stderr hash.

One frozen diagnostic per axis on the same source/CLI completes at885/938
accept and248/767 reject. Both verdict sets are byte-identical to1c183b98:
no file moved in either direction. There are0 unresolved timeouts and0
unexpected exits. The CLI digest above and clean source were checked before
and after both passes. This is not the final two-pass landing gate.

Main-relative accept gains remain13 and regressions remain10: CoerceToVDQ,
ControlMonadClassesState, T13879, T16204a, T16204b, T17067, T18129, T22560c,
T23543 and T26358. Main121d4f2c, the canonical DB and published artifacts
remain untouched. The next reduced boundary is imported-family equation
checking: GHC accepts an unchanged equation with either a local or imported
family declaration, while this checkpoint only accepts the local form.
Do not weaken RHS closure to cover that missing imported contract.

Evidence: promoted-symbols-chirho/associated-full-driver-chirho.json and
associated-diagnostic-chirho.jsonl. Full workspace execution and the final
two-pass corpus gate remain owed after the regressions are repaired.
