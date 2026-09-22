<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Preserve data-family declaration form

Owner: HASKELUJAH/gpt_chirho, existing row484, isolated gpt-kind-schemes-chirho worktree. Main, DB, published inventories and peer provenance work remain untouched. Before checkpoint: row484-data-family-form-before-chirho at ff735ce157ef1aaa6bc5a0bb01e07450983b6ba2.

T17067 is a genuine regression: GHC9.14.1 accepts both data-family applications in its type-family patterns. Lowering currently discards the top-level data/type keyword, and checked kind exports classify both as reducible families. Associated declarations already retain data_chirho but their kind classification does not consume it.

Placement: retain data_chirho at the existing family AST producer, use the same field for associated heads instead of re-scanning their CST, and export a distinct DataFamilyChirho shape. It is nominal for matching and elaboration, but remains distinct from ordinary data and type families for contract agreement. Do not relax the equation-pattern guard, invent equations, or classify by source spelling. Keep term-family registration out of the data-family path.

- [x] Trace the producer, local classification, imported contract and equation-pattern consumer.
- [x] Record GHC predictions and exact controls before changing compiler behavior.
- [x] Preserve form through lowering, local/imported kind classification, associated heads and term registration.
- [x] Verify T17067, local/imported/associated positives, type-family negatives, shadowing and an exact execution control.
- [x] Run focused parser/typing/driver gates, formatting and changed-line lint review; report existing debt separately.
- [ ] Update workflow and checkpoint only owned paths. This is not a main landing or a corpus no-regression claim.

Reference predictions: local nominal-family pattern and execution accept and print 7; qualified imported nominal-family pattern accepts and prints 7; same-spelled local data-family and qualified provider type-family remain distinct; associated data-family pattern accepts; local/imported/associated type-family patterns reject for the illegal-family-application rule. No filename-specific behavior.

## Focused evidence

Durable fixtures and raw observations: `test-data-chirho/kind-oracles-chirho/classifier-contracts-chirho/data-families-chirho/`. GHC9.14.1 accepts all four positive controls and each prints exactly `7\n`; all three negatives report GHC-73138 for an illegal type synonym family application. GHC also reports GHC-30337 about a variable in a noninjective position, which is not a diagnostic-parity claim for this repair.

The retained parent CLI (SHA-256 `88c17197835dd4ccd6afc1b998f426a9ff5c284f05ebfddda0d84e5e9872063b`, identical before/after) rejects all four valid controls. It rejects the local and imported type-family negatives but wrongly accepts the associated-type-family negative. These are recorded in `before-chirho.json`, not inferred from the patch.

The first focused driver run after the repair passes 8 tests: the three new tests plus five existing associated-data/boot-contract tests. The new execution control prints `7\n` for both the local and associated cases through STG, LLVM and Cranelift. This is a filtered focused result, not a full driver or corpus gate.

## Final focused gate and remaining red observations

The gate ran 2026-09-22 12:44:59Z to 12:51:04Z on the unchanged crate-source manifest in `data-family-gates-chirho/receipt-chirho.json`, with `RUST_MIN_STACK=16777216`, nice10, one build job and two test threads. Formatting passes. Naming138, parser375, TH18, typing410, typing-integration473 and canaries7 all pass, with zero failed/ignored/filtered tests in those suites. The live constraints-package test passes separately, 1 run / 1775 filtered / 66.27s; its package prerequisite hash is retained. Build/test stderr contains zero compiler warning lines. This is not a complete workspace or driver-library gate.

Clippy is NOT clean. The retained audit has 433 distinct warnings and returns exit1 for one changed-line overlap: the whole `KindHeadShapeChirho` enum is flagged by `clippy::enum_variant_names` for the mandated Chirho suffix. The identical warning exists in the retained parent log (uncompressed SHA-256 `dc61cfc51b5ca372873b241ef5f91982d1618f01ae04931fadf1d77cbf1f29fa`, enum lines7-12 before this patch). No warning is suppressed or called a clean-lint pass. The other 432 warnings remain existing debt outside changed primary spans.

The freshly built, frozen CLI has SHA-256 `df001df8f8ecac55215afcba9739ae38f5ecff8d64f09b1fdf009712538c643b`, identical before/after the candidate replay. All seven checking probes agree with GHC: four valid data-family programs accept, three illegal type-family-pattern programs reject with the intended rule. Local and associated execution again print exactly `7\n` through the CLI. Unchanged T17067 also exits0 in the fresh CLI; GHC9.14.1 `-v0 -fforce-recomp -fno-code -fno-write-interface` exits0 with empty output on the same source, SHA-256 `bc907637c8e7c804100d04fc3e52aef9522f4ddf049cb51595ae385fac46785c`.

Two extra CLI execution probes remain RED and are retained, not removed from the receipt: ImportedChirho and ShadowChirho fail to locate ProviderChirho. The CLI run path passes one source and its basename to `eval_source_with_machine_chirho`, unlike its check path's source-graph loader. Replaying the retained parent CLI on both sources produces byte-identical missing-provider stderr, so this is not introduced by data-family classification. The imported compile_modules controls and CLI checks pass, but imported CLI execution is NOT claimed. Candidate evidence is `candidate-chirho.json`; the parent run replay is retained outside OS tmp at `tmp-chirho/gnd-resume-chirho/data-family-gates-chirho/parent-run-replay-chirho.json`. Its two missing-provider outputs are also recorded in that file.

The recipe therefore exits1 because these additional execution expectations fail, and the changed-line lint recipe exits1 as described above. This is a bounded implementation checkpoint with known red observations, not a declaration that every gate is green or that row484 is landable. No published measurement, public number, main checkout, peer worktree or DB row changed.

## Diagnostic corpus delta: NOT landable

After peer HOLD acknowledgement #24561, START #24563 and DONE #24564 bracketed one complete diagnostic pass per axis on the frozen candidate. The 938/767 inventories have no missing/duplicate source identities in the parent comparison, no initial timeouts, no abnormal exits/panic headers and no exit/diagnostic disagreement. Candidate SHA-256 is identical before and after. All output and per-file hashes are retained under `../data-family-corpus-chirho/`; this is not the required two-pass landing measurement.

Against the retained parent88c171, the exact delta is:

- Accept: T17067 recovers; T16188 is newly rejected. No other accept movement.
- Reject: T16204c newly rejects; no losses. Its real Rep/Type mismatch is caught, but the diagnostic is ADJACENT: GHC-83865 anchors sTo at16:8 and expects Sing@Type from actual Sing@Rep, while ours anchors id at16:5, reports expected Rep/found Type, and repeats the diagnostic. No full reason or anchor parity is claimed.

GHC9.14.1 was rerun on unchanged T16188 (exit0, no output) and T16204c (exit1, GHC-83865). T16188 is therefore a real candidate regression, not a corpus-inventory correction. Its actual lowered declaration trace retains the Sing data-family HEAD but drops both data INSTANCE declarations: the only retained ordinary data constructors are RegExp's App; SFalse, STrue and SApp never reach the AST. The trace and source-hash-linked diagnostics are retained in the corpus directory. Temporary tracing code was removed and the gated crate manifest was rechecked.

Next producer decision: a data-family instance needs an explicit AST form preserving its family application, binders/kind and constructors. It must not become a second ordinary declaration of the family name, and must not be fabricated as a type-family equation. The constructor schemes then supply the scoped GADT refinements this valid program requires. That is a separate implementation brick, not an excuse to weaken nominal classification or restore the former noninjective-family classification. No such instance-representation implementation is included in this checkpoint.
