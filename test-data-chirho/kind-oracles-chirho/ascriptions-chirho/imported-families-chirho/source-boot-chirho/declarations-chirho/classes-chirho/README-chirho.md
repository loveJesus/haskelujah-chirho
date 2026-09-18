<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Source-local boot-class agreement Chirho

Reference observations were independently collected by Claude with GHC9.14.1.
Each JSONL record retains the exact source files, command, exit, diagnostics,
prediction and source hashes. Refuted predictions stay in the evidence. They
are not definitions of the language; the measured outcomes are the reference.

The seventy-eight class observations are under `references-chirho`: original18,
kind-annotated8, edges15, MINIMAL4, nonempty-body9, associated-family9 and
boundary7, plus injectivity-annotation8. Twelve separate associated-result-binder
observations exercise declaration/default syntax with instances and consumers.
Ten additional independently executed injectivity probes are retained separately;
they are corroborating observations, not ten more entries in the class matrix.

The Rust integration cases run the same class sources through both filesystem
entry points. Rejections require the applicable agreement reason, not merely an
error somewhere in the dependency graph. The fixture modules are grouped by
head, method, associated-family, boundary and injectivity contracts; no source bytes were
changed by that split. The CLI replay adds real process/provenance evidence.

## Contracts exercised

- Class kind and functional dependencies agree even for abstract boot classes.
- Abstract means no written context, no methods and no associated families.
  An empty `where {}` alone does not make a promise concrete; `() =>` does.
- Concrete superclass and method lists are positional. Method parameters retain
  the class's binder slots. Alpha-renaming is permitted; exchanging slots is not.
- Method default presence and generic-default types agree. Default body text is
  not an agreement contract.
- Associated families compare kinds, type/data form, injectivity and defaults
  positionally. Their names are not an extra positional equality requirement.
  Explicit and omitted `family` spellings denote the same declaration.
- An injectivity annotation names its declared result and input binders. An
  unknown name is a declaration error, not absent metadata that can agree with
  another absence. Explicit input positions compare as a sorted, deduplicated
  set: order and multiplicity do not change the promise.
- The boot MINIMAL formula must imply the implementation formula. It is not
  string equality or reverse implication. The proof is bounded, with resource
  exhaustion reported as unproved rather than a successful comparison.
- Missing checked kind metadata cannot establish agreement. Public top-level
  availability is checked before local contracts; member availability follows
  those contracts so a body disagreement is not disguised as an export error.

## Scope and limitations

The checkpoint results below describe97218fde. The subsequent default-scope
repair and its separate reference/recovery controls are recorded in
`default-annotations-chirho/scope-chirho/README-chirho.md`; historical outputs
here are not silently overwritten by that continuation.

`before-chirho.jsonl` records the original forty-five probes against the
pre-brick CLI. Later additions have independent GHC records, not invented
before-results. `after-chirho.jsonl` is the pre-annotation-repair CLI replay,
with its exact binary hash in every row. Post-repair replays are in
`gates-chirho/*-after-chirho.jsonl`. Gate records describe source hashes and
the commands actually run.
`injectivity-before-chirho.jsonl` agrees on only four of Claude's eight new cases:
two invalid annotations were accepted and two equivalent promises were rejected.
The declaration checks and set comparison repair those cases; an unresolved
parameter slot still cannot establish agreement. Hidden-kind annotation slots
are not newly represented by this repair.

The twelve syntax probes initially agree on ten. S5 (implicit kind arguments in
an associated default) and S9 (TypeFamilyDependencies licensing) remain recorded
false accepts. There is no ignored test standing in for either missing feature.

The checkpoint CLI agrees on all78 class observations and all10 separately retained
injectivity observations. The syntax replay still agrees on10/12, with exactly
the two gaps above. All100 observations completed without timeout, using one
stable CLI SHA256:
`4207c3bb492078aaf21959efd74dd1b2d2c5657bb5d9b1e73e8a0faef80e7ef0`.

Final gates pass parser371, naming137, typing401, integration412, canaries7 and
driver1776, with zero ignored or filtered tests. Formatting, workspace all-target
check and explicit CLI build pass. The four-package all-target clippy invocation
exits0 but retains464 warning-message lines including duplicates and summaries;
it is not lint-clean. Three new collapsible-if findings were repaired before
the final freeze without suppressions. The full driver suite was rerun afterward.

The first full corpus diagnostic exposed T18585's valid annotated default.
The repair keeps original annotations in naming/kind checking and only peels
wrappers for structural validity and per-instance substitution. Seven GHC-backed
controls, including a wrong-result consumer, now agree. The final diagnostic is
891/938 and264/767: eight accept recoveries and no losses relative26f77cad,
but eighteen gains and nine losses relative main still forbid a main merge.
The first890/264 diagnostic and its regression are retained beside the final run.
Claude's seventeen-file reject-reason audit retains its original text in JSON
and a trailing-whitespace-normalized Markdown display; the two formerly
adjacent arity diagnostics now explicitly report the parameter-count mismatch.

The later six-probe scope review agrees only2/6. Fresh equation kind variables
can still be specialized, and a class type parameter absent from the default LHS
can leak into its RHS; implicit-kind default validity is also still incomplete.
These disagreeing observations live under `default-annotations-chirho/scope-chirho`
and are not counted as implemented contracts or attributed to an older commit
without a measured bracket.

New source files stay below1k lines, with fixture groups and validity split into
focused modules. Legacy infer/lower/iface roots still violate the file-size rule;
their decomposition and the retained lint debt remain unresolved quality work.

This brick does not establish imported/open-family injective improvement,
hidden-kind improvement, defining-origin identity for re-exports, or a fully
passing upstream corpus. Those remain separate work. Public measurement files,
their denominators and label conventions are unchanged.

The first broad driver run found a real regression in the unchanged
`assoc_type_family_parsed_chirho` test: the newly retained `:: *` kind became
multiplication with two fabricated empty operands. GHC accepts the source. A
leading-star atom repair makes that test pass without changing its expectation;
the focused parser control was demonstrated red before the repair and green
after it. Qualified multiplication and NoStarIsType controls remain separate.

## Replaying the observations

From the worktree root, explicitly build `cargo build -p haskelujah`, then use
the sibling `families-chirho/observe_chirho.ts` with the absolute CLI path, an
output JSONL path, and the selected reference JSONL paths. The replay verifies
that the CLI hash is stable; timeout or crash is not a semantic rejection.
`gate_chirho.ts` records focused compilation, test and lint commands with source
hashes. Neither runner edits corpus inputs or chooses an oracle from our output.

`reference_chirho.py` and `edges_chirho.py` preserve Claude's original runners
unchanged. They cover their original subsets; the complete JSONL sources are
the replay input for later additions.
