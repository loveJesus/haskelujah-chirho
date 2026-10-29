<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Annotated pattern bindings Chirho (row 675)

Base: b76fcd03. GPT parent owns main, builder leases and the DB; this lane owns parser recognition and binding-only annotation checks. The initial e6179559/db9a7367 tip passed its focused gate but was held when independent extension probes exposed two regressions. Repaired source 1eb4e8a8 adds the required license check and legacy aliases and passes its new focused gate. The parent owns final corpus verification and landing. No main or DB write, remote push or release.

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

## Initial focused gate (held after additional probes)

The workspace/all-target check and CLI build have no compiler warning. Changed-line Clippy reads 293 existing diagnostic locations and finds zero on the row's 430 changed Rust lines in eleven files. This is a scoped result, not a claim that workspace Clippy has no existing warnings. Rustfmt, diff-check and gitleaks are clean; gitleaks scans the one source commit. The source was made on ff4b5274 and replayed over the docs-only b76fcd03, without a crate-byte change.

GHC probes and candidate/baseline checks use eleven generated source files, taken directly from the integration controls plus exact tc135. The check verdicts move exactly seven cells toward GHC: three rejections become valid accepts, four invalid accepts become rejections. Four verdicts hold. Four positive programs are exact on all three engines. The paired published corpus is for the parent's integrated measurement; this row has not rerun it.

The two reach mutants compiled and exited 101 at their decisive assertions. Recognition off gives tc135 two local declarations instead of one binding. Annotation checking off accepts `BadVariableAnnotationChirho.hs`. Mutations were restored through patch writes, which refresh mtimes, then both libraries and the six integration groups were rebuilt and passed. The parser file's SHA-256 after restore is baa3cc428c06a26694a39ae51d61ab748f82ba433e51b5336990f28b8d4c24b7, equal to its pre-mutation hash. The typing file then received only the parent-requested comment cleanup distinguishing the retained argument/lambda route.

The measured frozen CLI is `tmp-chirho/row675-chirho/haskelujah-candidate-chirho` (SHA-256 1fbfa557fc95a637219304a6b32afede1935577e4139ee119a755e3bfb76daa0). A CLI was rebuilt after source restore and comment cleanup and is retained separately as `haskelujah-final-chirho`. The builder slot was released to the parent immediately after that gate. Further execution waited for the parent's explicit extension-repair builder lease below.

## Extension repair after independent review

The parent compared both full axes on the held candidate and found only tc135 improving on should_compile, with no other verdict change. Its additional six extension probes nevertheless found the explicit Haskell2010 and NoScopedTypeVariables programs moving from baseline refusal to candidate acceptance, where GHC reports GHC-74097. The held tip was therefore not safe to land.

- [x] Add a PatBind-only E0208 diagnostic when ScopedTypeVariables is disabled.
- [x] Recognize PatternSignatures and NoPatternSignatures in the existing module flag helper.
- [x] Measure all seven extension controls against GHC, including the disabled alias counterpart; four accept and three reject, as intended.
- [x] Run the seven integration groups: the original six plus the extension table, all passing.
- [x] Complete fresh parser/typing, workspace check, changed-line Clippy, build and hygiene checks, then freeze and report the repaired CLI.

The same scope still excludes argument/lambda pattern checks. No enum or range source is changed by this repair. The new gate's retained logs are `repair-*-chirho.log` under the existing row675 receipt directory. The parent owns the final candidate corpus sweep and landing decision.

## Repaired focused gate

- Parser 401/401 and typing 370/370 pass.
- Seven binding integration groups pass, including the extension table, six invalid type annotations, exact tc135 and four positive programs on the interpreter, LLVM and real Cranelift.
- All eighteen generated probe verdicts agree with GHC 9.14.1. The frozen CLI refuses explicit Haskell2010, NoScopedTypeVariables and NoPatternSignatures with the missing-extension diagnostic, and accepts the default, modern, enabled-legacy and PatternSignatures-alias cases.
- Workspace/all-target check and the rebuilt CLI have zero compiler warnings.
- Changed-line Clippy reads 293 existing diagnostic locations and finds zero on 498 changed Rust lines in twelve files. Rustfmt and diff-check are clean. Gitleaks scans the three commits through repaired source 1eb4e8a8 with no leaks; the final record extends that range without changing source.

The repaired frozen executable is `tmp-chirho/row675-chirho/haskelujah-repaired-chirho`, SHA-256 d8c7f9c0286a90db6f964b993015b9ea423b27c2045520a5e26ee09d190b071a. Its baseline and candidate check receipts are under `repair-cli-chirho/`, with the exact eighteen rows in `repair-checks-chirho.tsv`. The builder slot was released to the parent after this gate. No further compiler/test execution was performed by this lane after release.
