<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Annotated pattern bindings Chirho (row 675)

Base: b76fcd03. GPT parent owns main, builder leases and the DB; this lane owns parser recognition and binding-only annotation checks. Source e6179559 has passed the focused gate and is ready for the parent's landing review. No main or DB write, remote push or release.

- [x] Trace tc135's missing `t3` to signature-first declaration dispatch; existing SigPat lowering needs no change.
- [x] Add bounded same-declaration annotation/binding recognition and syntax holds.
- [x] Trace the unsafe typing boundary: TypeAnnot discarded its type; add a scoped annotation check only for local and top-level PatBind.
- [x] Add all-engine positive/hold controls and variable, tuple, list, nested, local and scoped-variable negatives.
- [x] Refresh onto main ff4b5274 while preserving this lane's owned changes.
- [x] Measure GHC and baseline/candidate outcomes under the parent's builder lease: all eleven candidate check verdicts agree with GHC; the four positive programs execute to their GHC outputs on all three engines.
- [x] Run parser controls, all-engine binding controls, typing library and changed-line lint/hygiene gates.
- [x] Remove recognition and annotation checking separately to establish behavioral reach; both mutants compile and their controls fail, then source is restored with fresh mtimes and the restored libraries/controls pass.
- [x] Commit explicit owned files and report the tested tip, exact movers and remaining boundaries to the parent.

The separate rank-n argument/lambda annotation defect (tc194) is not changed here. No acceptance percentage or fresh corpus count is claimed before paired candidate measurement.

The source controls retain the exact measured programs. On GHC 9.14.1, four positive programs print their expected outputs, exact tc135 typechecks, and all six negatives reject. The frozen baseline rejects the annotated-binding table, scoped-binding program and tc135; the candidate accepts them. The baseline accepts the variable, tuple, list and nested invalid annotations; the candidate rejects each. The other four check verdicts hold. The two baseline local/scoped-negative refusals were unbound-name errors; the candidate checks the actual annotation.

The parser library passes 401/401 and typing 370/370. Six integration groups pass, including interpreter, LLVM and real Cranelift output comparisons. Removing parser recognition fails tc135's retained-local-binding assertion. Disabling binding annotation checking fails the negative table because `valueChirho :: Bool = 3` is accepted. Restored library and integration runs pass again. Raw oracle and check receipts, generated probes and restored-run logs are under `tmp-chirho/row675-chirho/`.

## Final focused gate

The workspace/all-target check and CLI build have no compiler warning. Changed-line Clippy reads 293 existing diagnostic locations and finds zero on the row's 430 changed Rust lines in eleven files. This is a scoped result, not a claim that workspace Clippy has no existing warnings. Rustfmt, diff-check and gitleaks are clean; gitleaks scans the one source commit. The source was made on ff4b5274 and replayed over the docs-only b76fcd03, without a crate-byte change.

GHC probes and candidate/baseline checks use eleven generated source files, taken directly from the integration controls plus exact tc135. The check verdicts move exactly seven cells toward GHC: three rejections become valid accepts, four invalid accepts become rejections. Four verdicts hold. Four positive programs are exact on all three engines. The paired published corpus is for the parent's integrated measurement; this row has not rerun it.

The two reach mutants compiled and exited 101 at their decisive assertions. Recognition off gives tc135 two local declarations instead of one binding. Annotation checking off accepts `BadVariableAnnotationChirho.hs`. Mutations were restored through patch writes, which refresh mtimes, then both libraries and the six integration groups were rebuilt and passed. The parser file's SHA-256 after restore is baa3cc428c06a26694a39ae51d61ab748f82ba433e51b5336990f28b8d4c24b7, equal to its pre-mutation hash. The typing file then received only the parent-requested comment cleanup distinguishing the retained argument/lambda route.

The measured frozen CLI is `tmp-chirho/row675-chirho/haskelujah-candidate-chirho` (SHA-256 1fbfa557fc95a637219304a6b32afede1935577e4139ee119a755e3bfb76daa0). A final CLI was rebuilt after source restore and comment cleanup and is retained separately as `haskelujah-final-chirho`. The builder slot was released to the parent immediately after the gate. No further Cargo/GHC/test run followed that release.
