<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Annotated pattern bindings Chirho (row 675)

Base: ff4b5274. GPT parent owns main, builder leases and the DB; this lane owns parser recognition and binding-only annotation checks. The source is held until GHC/candidate measurements and the focused gate. No remote push.

- [x] Trace tc135's missing `t3` to signature-first declaration dispatch; existing SigPat lowering needs no change.
- [x] Add bounded same-declaration annotation/binding recognition and syntax holds.
- [x] Trace the unsafe typing boundary: TypeAnnot discarded its type; add a scoped annotation check only for local and top-level PatBind.
- [x] Add all-engine positive/hold controls and variable, tuple, list, nested, local and scoped-variable negatives.
- [x] Refresh onto main ff4b5274 while preserving this lane's owned changes.
- [x] Measure GHC and baseline/candidate outcomes under the parent's builder lease: all eleven candidate check verdicts agree with GHC; the four positive programs execute to their GHC outputs on all three engines.
- [ ] Run parser controls, all-engine binding controls, typing library and changed-line lint/hygiene gates.
- [x] Remove recognition and annotation checking separately to establish behavioral reach; both mutants compile and their controls fail, then source is restored with fresh mtimes and the restored libraries/controls pass.
- [ ] Commit explicit owned files and report the tested tip, exact movers and remaining boundaries to the parent.

The separate rank-n argument/lambda annotation defect (tc194) is not changed here. No acceptance percentage or fresh corpus count is claimed before paired candidate measurement.

The source controls retain the exact measured programs. On GHC 9.14.1, four positive programs print their expected outputs, exact tc135 typechecks, and all six negatives reject. The frozen baseline rejects the annotated-binding table, scoped-binding program and tc135; the candidate accepts them. The baseline accepts the variable, tuple, list and nested invalid annotations; the candidate rejects each. The other four check verdicts hold. The two baseline local/scoped-negative refusals were unbound-name errors; the candidate checks the actual annotation.

The parser library passes 401/401 and typing 370/370. Six integration groups pass, including interpreter, LLVM and real Cranelift output comparisons. Removing parser recognition fails tc135's retained-local-binding assertion. Disabling binding annotation checking fails the negative table because `valueChirho :: Bool = 3` is accepted. Restored library and integration runs pass again. Raw oracle and check receipts, generated probes and restored-run logs are under `tmp-chirho/row675-chirho/`.
