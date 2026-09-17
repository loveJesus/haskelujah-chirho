<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Annotated associated defaults Chirho

The first full corpus diagnostic exposed T18585: the new default-validity check
required bare variables and rejected legal kind-annotated variables. Seven bounded
GHC9.14.1 observations, with predictions recorded first, establish two valid forms
and five rejecting controls. The original source bytes, commands, diagnostics,
version and hashes remain in `before-chirho.jsonl`; `reference-chirho.jsonl`
adapts those same observations to the shared CLI replay format without rerunning
or inventing reference verdicts.

The pre-repair CLI rejects both valid forms. Five of the seven new Rust controls
fail before repair: two false rejections and three invalid cases rejected for an
unrelated structural reason. Two pre-existing promoted-default controls also run
under that filter, giving4/9 before and9/9 after. `focused-tests-chirho.json`
retains both runs, including the filter counts. This is not a full-suite claim.

Structural validity peels parentheses and kind annotations only to establish
variable identity and distinctness. Naming still checks every original LHS
annotation. Kind inference checks the applied family head with those original
arguments and unifies its result kind with the RHS in an equation-local scope.
Only after those checks does per-instance default substitution use the peeled
variable names. The wrong-result consumer proves that a legal annotated default
is actually instantiated, not silently discarded.

`after-chirho.jsonl` agrees7/7 with GHC, with no timeout or abnormal exit. T18585
recovers in the final corpus diagnostic and no other verdict moves relative to
the pre-repair890/264 sets. Implicit-kind default-argument validity and extension
licensing remain separate known gaps; these controls do not claim to fix them.

## Independent scope follow-up

`scope-chirho/reference-claude-chirho.jsonl` preserves six later GHC observations
unchanged; `replay-input-chirho.jsonl` only adds the explicit source-root name
needed by the common runner. The frozen candidate agrees2/6: the two positive
controls pass, but four invalid inputs also pass. KV1/KV2 need rigid equation-local
kind variables; KV3 is the separately known implicit-kind default-argument rule;
KV4 leaks a class type parameter absent from the equation LHS into its RHS.
The `candidate-chirho.jsonl` records these as disagreements, not passing tests.
No claim is made here about when those gaps were introduced, and no source was
edited while the final gate was running. They require a subsequent scoped repair.
