<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Checked-kind GND continuation, row484

Isolated work on gpt-kind-schemes-chirho, not a main-line landing or a public
compatibility measurement. The pre-change checkpoint is ff75ea3f, tagged
row484-gnd-kind-seam-before-chirho. Canonical progress row484 remains open.

## Contract

Deriving retains the entire written class application. Naming checks it under
the declaration's lexical binders. Generalized newtype deriving runs inside the
existing kind context after declaration kinds close and before ordinary instance
checking and finalization. Its generated heads therefore participate in the same
checked hidden-argument publication as written instances.

The class's checked residual kind determines how many trailing newtype parameters
to remove; the representation must permit that eta reduction. Supplied class
arguments survive in both the generated head and representation constraint.
Failed checking publishes no instance. The work budgets and phase graph are in
the declaration-kinds workflow, with the implementation in the focused deriving
children and kind_chirho/phases_chirho.

Standard Ix, MonadIO, MonadFix and MonadTrans classifiers are seeded centrally,
not guessed at the deriving site. Local same-spelled declarations still shadow
them. Generic1 remains unsupported stock generation with an explicit warning;
Data/Typeable/Lift preserve existing metadata-only behavior. No runtime coercion,
role proof, complete deriving strategies, Generic1 methods or dictionary-function
claim follows from frontend acceptance.

## Evidence inventory

- The parent gnd-next-brick-chirho.jsonl retains eight GHC 9.14.1 source cases.
  G4's original negative prediction was refuted: its inferred class kind permits
  the argument, so acceptance is the reference verdict.
- reference-probes-chirho.jsonl adds arrow, prefix, two-kind, list, tuple and two
  imported-class cases. The imported provider's accidental extra retained LF was
  corrected against its recorded hash; the correction is recorded on both rows.
- stock-reference-probes-chirho.jsonl adds five strategy/classifier controls.
- monad-reference-probes-chirho.jsonl adds six standard-contract controls,
  including wrong-kind rejections and local shadowing. Predictions precede the
  retained reference commands, exits and full diagnostics.
- checkpoint-chirho holds compressed command logs, source hashes, lint debt,
  immutable-CLI replay receipts and the next-reference/oracle note.
- tools-chirho contains the exact bounded replay/corpus detector and gate runner.
  Corpus START/DONE coordination is external to the runner and remains required.

The host reboot on 2026-09-20 erased unarchived /private/tmp receipts. Earlier
reported GND corpus figures are historical reports, not reopenable evidence.
The retained source graphs survived and were replayed after rebuilding. New logs
are inside this worktree; the large read-only CLI copies live in ignored
tmp-chirho/gnd-resume-chirho, outside operating-system temporary storage.

## Gate status

The first persistent recovery run passed naming138, parser374, TH18, typing409,
canaries7 and typing integration470, all zero failed/ignored/filtered. The live
constraints package ran once with1775 filtered tests and passed; its cabal input
was present and hashed before/after. Clippy reports433 distinct warnings, zero
primary spans on changed lines: lint is NOT clean. The CLI6aab9ed6 replayed all26
reference cases and10 corpus recoveries, T3955 and T12734 included, with stable
hashes and no timeouts/panics/unexpected exits.

That run preceded the final path-only phase grouping. The final-gates-chirho
receipt now repeats all those passing suites on the final layout. Its live
package test passes in83.54s; formatting passes and there are no compiler warning
lines. Final lint accounting remains433 distinct warnings, zero primary spans
on changed lines. Frozen CLI
88c17197835dd4ccd6afc1b998f426a9ff5c284f05ebfddda0d84e5e9872063b
replays all36 cases in final-replay-chirho.jsonl with a stable before/after hash.
The nine negative controls report the intended unary, eta or kind-arity errors,
not missing metadata.

The diagnostic pair after the peer's explicit release measures890/938 on the
accept inventory and268/767 on the reject inventory, with zero initial/unresolved
timeouts, runtime panics, unexpected exits or output limits. Binary and source
hashes stayed identical. This is ONE pass per axis, not a landing measurement.
The complete observations, sets and comparison are in checkpoint-chirho/corpus-chirho.

All1705 source hashes match the retained parent650be538 and first candidate5c145678
observations. Against that parent, accept gains are ClassDefaultInHsBoot,
ClassDefaultInHsBootA2, ClassDefaultInHsBootA3 and T18036b; no accept losses.
The only reject gain is T6001, with no reject losses. T6001 is ADJACENT, not
diagnostic parity, and was already rejected before this GND unit. Against the
first candidate, all nine accept regressions recover plus T18036b; the two
wrong-reason rejections T15712/T26137 disappear. Their GHC rules remain missing.
T12734 is accepted again, so the first GND diagnostic's new loss is repaired.

Against main c74db426, the separate full-branch comparison is16 accept gains and
11 losses,59 reject gains and19 losses. Those include older row484 work and
main's independent duplicate-instance lane, not just this GND unit. No overall
no-regression, main landing or published measurement update is claimed. The
remaining differences are not waived by this isolated checkpoint.

T5481 is a distinct oracle issue: current GHC rejects its out-of-scope default
RHS variables and all.T:360 explicitly uses compile_fail, despite the containing
directory's name. Its directory-based acceptance loss is not a valid program to
make pass. Keep that interpretation beside the unchanged count; neither corpus
membership nor a public denominator is changed by this checkpoint. T17067, in
contrast, passes the measured reference and its data-family-pattern rejection
remains a real defect outside this GND repair.
