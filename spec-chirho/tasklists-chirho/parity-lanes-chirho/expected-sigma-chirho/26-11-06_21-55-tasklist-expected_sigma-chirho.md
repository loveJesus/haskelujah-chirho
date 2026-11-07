<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Shared expected-sigma checking Chirho

Scope: one ordinary expected-expression entry for an outer invisible `forall`, using the existing rigid-signature, given-predicate and escape machinery. This is an isolated compiler checkpoint, not a main landing or whole-kind preservation claim.

- [x] Pin the historical compile-loss witnesses without calling them current global losses.
- [x] Execute the exact five GHC 9.14.1 compile-only references: tc092, T11339b and T11339c accept; T11339 rejects with GHC-16675; CapturedAnnotationChirho rejects with GHC-25897. Actual method: unchanged `run_owned_chirho` (`2a80fda8`), not execution of `dab3`.
- [x] Create isolated branch `gpt-expected-sigma-688-chirho` from combined source `28360838bdedec2148eaa276ede3909e751264d8`; preserve the original source and receipts.
- [x] Adapt the already-landed 708 nominality prerequisite: remove only the normal-mode Int/Double empty-substitution arm and port the three exact direct controls from `0afac144`. No strict-kind, representation or resource-policy change.
- [x] Pin five direct expected-expression controls before the sigma repair: qualified empty list, qualified section, captured-meta refusal, monomorphic-body refusal and explicit type-lambda binder preservation. Existing whole-typing QuickLook/RequiredForall controls remain required after the repair.
- [x] Measure the before state with the unchanged controls and retain every result: 841 fresh library names, exactly five selected outcomes, three passes and two positive-control failures (qualified empty list and section). The original relative-path mapping refusal is separately retained; the clean list was reused without replay.
- [x] Add general ordinary invisible-forall checking while preserving named/application QuickLook and explicit type-lambda/RequiredForall contracts.
- [x] Run the unchanged focused controls, whole typing library and changed-span lint under an actual scheduler grant: five focused passes, all 841 fresh-listed library tests pass, and zero changed-span Clippy hits against `28360838` with 38 outside-span diagnostics.
- [x] Commit and push the coherent source checkpoint `1d3a694f7922a36748e0edea3e38ddffed93b2a5` after formatting/diff checks and outgoing secret scan; retain the original before failures and pending preservation explicitly.
- [x] Measure all five original source witnesses on the product compiler before and after, retaining the executor refusal and every actual outcome. tc092 changes from a valid-source refusal to acceptance; T11339b, T11339c and T11339 remain `UNKNOWN_RESOURCE_BOUND`; the capture refusal stays byte-identical.
- [ ] Trace the first normalization handoff and term growth behind the three resource-held outcomes at the existing bound. No cap increase, input change, per-witness guard or wrong-reason waiver is authorized.
- [ ] Complete ordinary preservation and integration before any main or broad-kind acceptance claim.

## Placement and boundaries

The production entry belongs in `infer_chirho/terms_chirho/expected_chirho.rs`. It must open the existing quantified records through `with_rigid_signature_chirho`, carry written qualified predicates through the existing scope, publish only the checked public substitution and return the original sigma shape. Named and application spines retain their current QuickLook route. Explicit type lambdas continue consuming their own binders. The existing annotations test directory has six entries; one focused child keeps it at seven. No new solver, registry, inference context field, defaulting rule, do-typing rule or per-witness guard is part of this unit.

The 708 prerequisite is a contextual port because this engine routes ordinary unification through `unify_with_kind_policy_chirho(..., false, ...)`; the original A9 patch therefore does not apply textually. Its normal-mode bypass alone is removed. The three exact test bodies now live in the existing `unify_chirho/tests_chirho.rs` child, which this branch already uses.

The implementation consumes the already-normalized `ForallChirho` record directly into the existing `SchemeChirho`, leaving its complete qualified body for `with_rigid_signature_chirho` to expose. This avoids rebuilding predicates or adding another prefix projection. Existing application and infix spines retain their current application checker; explicit type lambdas retain their binder-specific arm. The checker publishes the existing escape-checked public substitution and reapplies it to the original expected sigma.

Before-state summary: `tmp-chirho/expected-sigma-chirho/before-7778c7b7-continuation-r1-chirho/summary-chirho.json`, SHA-256 `20a82556787e48a8a6743d7617200ab2aca2f7e466be56531c780845f0940a9a`. All 1,389 source pins, tools and the fresh-listed test executable remain unchanged in the captured postflight. No product-source witnesses, full typing suite, nominality filters or native behavior were executed in that gate.

After-state summary: `tmp-chirho/expected-sigma-chirho/after-1d3a694f-r1-chirho/summary-chirho.json`, SHA-256 `3cbe583e2f2e82e087e1facdabdcc9e8f3bef7a3e8209c04488f4930e27d4db7`. The same five controls pass; the two qualified-expression positives are the measured flips, while both refusal guards and the explicit binder contract stay passing. The complete fresh inventory is byte-equal to the before list, with 841 unique whole-library outcomes and no failures or ignored tests. The five focused controls and three nominality controls are included in those 841; they are not additional unique outcomes. The full library also retains its existing QuickLook and RequiredForall controls.

Four after-stage receipts and eight raw streams remain pinned. All 1,389 source files, tool bytes, the current fresh-listed test executable and all four original process groups pass postflight (`05632f7fc29259193d80459bb589bf0272665b3e4e2ff44ec39dffe3d2a7714a`). Evidence manifest SHA-256: `4866fd9249809ad7bfe793aeb45dd62973c221afa3fb5f24f09686a7dc552a04`. Clippy emitted 65 raw warning records, reduced by the existing recursive judge to 38 diagnostics outside the changed spans and zero hits over 354 changed lines in seven files. Outside-span diagnostics are not independently attributed to the parent; this is not a whole-feature zero-warning claim.

The original five GHC references establish oracle authority, not product witness success. The direct/library gates above do not themselves establish the product-source outcomes recorded below, broad corpus comparison, current-main preservation, native behavior or main landing.

## Original-source product comparison

Actual grants 32798, 32807 and 32809 use the separate clean product gate, the unchanged `run_owned_chirho` (`2a80fda8`) and literal `check <absolute-original-source>` commands. The exact source bytes and pragmas stay unchanged. The GHC-only `-fno-warn-incomplete-patterns` option has no product-CLI equivalent and is explicitly unmapped. Each product check has a full 60-second process cap; no process timeout occurred. Source, tools, supplier roots, explicit standard library, observational cache and frozen binaries pass the recorded postflights.

The before source is `7778c7b7311388426896df87152f8d6dbe2077b1`, with frozen mode-0555 CLI SHA-256 `6e56bffb704ecf325fe950d88ea4e61f3e849453bca99e9afc5981cb617c71f4`. Its own-manifest CLI build succeeded with zero compiler diagnostics. The first executor then raised `ValueError` before any of the five product checks entered `Popen`; all five were unstarted in that attempt. That refusal is retained independently and is not a compiler verdict. The continuation reused the same frozen binary without rebuilding. It records inherited stack limits (soft 8,372,224 bytes, hard 67,092,480 bytes), uses `RUST_MIN_STACK` as the existing environment setting, and makes no claim that a 16 MiB main stack was installed.

Before continuation summary SHA-256: `f8949958926b72c15a52955b022137d2bb674f7dd7e9749f73f9f0c7fa5eea1e`. All five checks actually returned E0200: tc092 and both valid T11339 variants were false refusals; T11339 refused through generic sigma mismatches rather than its GHC-16675 MR reason; CapturedAnnotationChirho retained the intended rigid escape refusal. This is not a two-negative compatibility pass.

The after source is `1d3a694f7922a36748e0edea3e38ddffed93b2a5`, with frozen mode-0555 CLI SHA-256 `139444a5894616f74b315ddbd144cba0b7f4676a42e1e9677d9d14ea6ddbad92`. Its own-manifest build succeeded with zero compiler diagnostics. tc092 now accepts, measuring one original GHC-valid repair. T11339b, T11339c and T11339 each return E0300, `type synonym normalization resource bound exhausted`. They are `UNKNOWN_RESOURCE_BOUND`, not semantic rejections or guarded negative passes. CapturedAnnotationChirho retains byte-identical stderr before and after.

After summary SHA-256: `6ec9867ef1dbe5946fbdf3ddcfba7ba91675890e50179aaf163c859e693dc371`. Paired comparison SHA-256: `778aa5340ee9a91a3c640f45eb6d7047effd5e10100e1ab0e8d031df1d1bc336`. The resource-held outcomes keep this unit open; the 841 library passes and five original-source checks are separate scopes.

Build scope correction from retained Cargo JSON: there was no separate RTS prebuild or emitted/native-program execution. Cargo compiled the RTS dependency in the before CLI build (`fresh=false`, `libhaskelujah_rts-1c555b3e31df8a28.{rlib,a}`) and reused that artifact in the after build (`fresh=true`). This was ordinary CLI dependency compilation, not an additional runtime behavioral gate.

The immutable product packet is under `haskelujah-gpt-expected-sigma-probe-gate-chirho/tmp-chirho/expected-sigma-product-chirho/`. The executor refusal, before continuation, after run and original-five comparison remain separate artifacts; no summary is rewritten to hide the refusal or reclassify E0300 as a semantic verdict.

GHC evidence:

`/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-chirho/tmp-chirho/frontier-closure-chirho/expected-sigma-reference-chirho/ghc-r1-chirho/summary-chirho.json`

Summary SHA-256: `752bf5cc90b053da4756dfdd64ea6c5bc7c9bc0ae5cb1d6bcb3dfeacb45a8472`. Six receipts, twelve raw streams and all twelve input hashes remain pinned; all six owned process groups are empty. The target mission remains GHC 9.6/Haskell2010 plus common extensions; this reference records the explicitly pinned 9.14.1 oracle.
