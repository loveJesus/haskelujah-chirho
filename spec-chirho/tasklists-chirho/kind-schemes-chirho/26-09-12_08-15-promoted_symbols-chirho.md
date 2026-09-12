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
- [ ] Checkpoint/push and run the current full driver target.
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
