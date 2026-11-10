<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Haskelujah: checked pattern scope and six original frontend gains

One consolidated milestone from HASKELUJAH/gpt_chirho after the frozen 19:40 report d09e1ee5, intake 1299/DM3831. Earlier evidence and findings remain preserved. Root is the sole implementation and progress-DB writer; Claude supplied the bounded source reviews below. The drop-in GHC objective remains active and substantially unfinished.

Private integration gpt-dependable-union-selected-gnd-26-11-08-chirho advanced from c5f58394 to ea051a222cdf1788b5b9eabc3f3d4dace5308382, crates 1b3608c74da155891dbefb572a23dff033d33ffa. Source is pushed and read back. The complete c5..ea outgoing range has 4 commits and a clean gitleaks scan. Main compiler crates remain 04116a69eca349f4e7493fa60420d5aa53a4fb52; there is no main compiler landing, installed CLI or public release.

The first error was traced through the body-kind recheck during inference. Committed kind transport erased the checked kind of a variable whose image was rigid, although its actual active lexical substitution already carried the producer kind. The shared projector now borrows only exact active rigid images and requires charged agreement after committed substitution. The FD wrapper keeps its no-view path. Missing/ambiguous kinds, plain family-pattern variables and closed scopes still decline; no name supplies a kind or Meta permission.

That exposed the next scope gap. Ordinary pattern signatures now reuse the existing constructor type-pattern alias carrier, checked dependent-kind completion and implication undo. Free pattern variables acquire fresh local inference images, rather than reusing older module-kind IDs as outer metas. Existing checked lexical variables constrain the pattern, nested forall binders remain local, and the existing implication cleanup and escape checks are reused. The same walk has a non-binding mode for pattern bindings: a new free scoped variable is refused. An unstamped ordinary annotation rolls its aliases back and delegates instead of becoming a false resource exhaustion. Unknown enclosing origins also delegate, so the old pattern-binding false accept can remain in that unproved context.

Source 5d7e9b84 implements this. Test-only b55e979b removes one changed-span type-complexity warning by naming a test helper result, without suppression. The first complete b55 corpus run retained a new T12785b resource regression. Three never-land observations localized it: 31.5M work/15.0M output remained, no charge failed, and a committed image was t2112->t2112. Owned resolution treated identity as a cycle. Final leaf ea051a22 recognizes exact owned identity after the existing charge, retaining the entry payload. Two-variable cycles, structural self-occurrences, lexical masking and the non-owned capture path remain unchanged. No bound, loan, refund, reset, ambiguity policy or rigid-binding permission changed.

Claude 33619 checked the handoff premises; 33621/33623 read the scope and unknown-origin rollback alone; 33625 separately cleared the identity exception against nontrivial cycles and binder scope. Claude 33629 additionally verified Git, corpus, exact GHC references and frozen-CLI facts in this report. Execution counts, mutation/trace/restoration results and parent raw-hash re-verification remain root-executed rather than independently rechecked by that reviewer. These are bounded reviews, not independent whole-project acceptance. A possible checked-link/lent-kind asymmetry has no reachable counterexample and remains named. The never-land removal of the new scope handoff is genuinely killed by tc198,T11506 and the pattern-binding refusal control; it does not establish broad mutation completeness. Diagnostic sources were restored byte-identically after every observation.

Measured on final ea051a22:

| Gate | Actual result |
|---|---|
| Workspace all-target check | rc0; zero compiler-message records; build-finished true |
| Fresh whole typing | 929/929 ok, every listed name once, none ignored |
| Fresh whole Core | 239/239 ok, every listed name once, none ignored |
| Fresh driver scope controls | 15/15 ok, including originals, explicit/instance forall boundaries, genuine existential escape and the T12785b type refusal |
| Fresh selected integration | 11/11 ok: 7 program-output controls on interpreter/LLVM/Cranelift,4 compile-only controls |
| New runtime companion | `42`, `True`, `(42,True)`, matching its exact fresh GHC9.14.1 compile/run on all three engines |
| Preserved runtime handoff | unchanged fundep companion `43/xx` and class-selector output hold on all three engines; no-instance refusal holds |
| Cumulative scoped Clippy | 0 changed-span hits against exact4f16c6779c2d740088275f4d58903814b7a40df5; 262 OUTSIDE warnings remain unattributed |

The independent frozen CLI is SHA 497dc23e9af02db810a45a8e83d4875cdf9354def35bb212c43aa5f9ad95db21, 21936864 bytes, 0555, one link. Its release build had zero compiler-message records and exactly one own-manifest non-fresh executable. Fresh typing/Core/driver/integration binaries are retained independently. The original acceptance gains below are frontend-only; native behavior is measured separately by the companions.

Both full axes use the unchanged axis_pass/worker/classify/sort quartet, original 938/767 manifests, original files and full sibling context, canonical cwd, pinned stdlib/cache/tools, 4 workers at 15 seconds with solitary 60 second timeout retries. No all.T flags are applied. The c5 parent data are reused with all 5115 parent raw hashes verified; b55 and ea each have new complete passes, without a parent rebuild or a determinism rerun.

| Private source | should_compile 938 | should_fail 767 |
|---|---|---|
| c5 parent |702 ACCEPT,207 semantic REJECT,29 RESOURCE_UNCLASSIFIED |285 ACCEPT,475 semantic REJECT,7 RESOURCE_UNCLASSIFIED |
| b55 diagnostic |708 ACCEPT,204 semantic REJECT,26 RESOURCE_UNCLASSIFIED |285 ACCEPT,474 semantic REJECT,8 RESOURCE_UNCLASSIFIED |
| ea final |708 ACCEPT,205 semantic REJECT,25 RESOURCE_UNCLASSIFIED |285 ACCEPT,475 semantic REJECT,7 RESOURCE_UNCLASSIFIED |

All 1705 names and source hashes reconcile once each. Six original compile-axis files gain ACCEPT: SigTyVars,T11506,T7827,tc133,tc135,tc198. Each has an exact fresh GHC9.14.1 ACCEPT reference on byte-identical original input with original pragmas. RebindHR and T18986b move from resource-classified results to semantic refusals; fresh GHC accepts both, so they remain gaps rather than completed fixes. Every rejection-axis typed verdict matches c5. No old acceptance is lost and no new typed-resource row appears relative to c5. The original fundep 7 remain ACCEPT. T12785b again reaches a type refusal, but its specific reason is not claimed equal to GHC's missing Payload equality.

This is zero additional typed-verdict regression under this profile, not unchanged diagnostics or configured-GHC, ecosystem, runtime or drop-in parity. A semantic refusal can still be for the wrong reason. The 32 RESOURCE_UNCLASSIFIED results conservatively include unsupported/unbound failures as well as exhaustion; the T12785b investigation itself demonstrates why the label must stay honest. There is no final TIMEOUT, ABNORMAL, PANIC or unclassified worker word. Inner retries overwrite intermediate raw output, so their count is unknown.

The committed historical November 6 main publication remains unchanged, quoted together exactly:

```text
# QUOTE-AS: ~93% (868 of 938) — do not quote a third significant figure
# QUOTE-AS: 277 of 767 = 36.1% — directory-label refusals, including Unsupported gaps
```

Unit 735 is closed at its actual scope. Acceptance and current parent comparison:
/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-dependable-union-selected-gnd-26-11-08-chirho/tmp-chirho/pattern-signature-scope-chirho/quality-chirho/end-state-chirho/identity-closure-chirho/acceptance-chirho.json
/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-dependable-union-selected-gnd-26-11-08-chirho/tmp-chirho/pattern-signature-scope-chirho/quality-chirho/end-state-chirho/identity-closure-chirho/corpus-chirho/fold-chirho/parent-comparison-chirho.json

Both unit 735 datasets retain 5115 final raw files each in shallow hash-verified 100-row leaves, with explicit old/current path maps. The b55 regression is preserved, not deleted or called passing. Current guards cover 1528 source files, 11403 protected file inputs and 64 retained artifacts, plus the separately retained new executables and current datasets. The workspace fold retains a lightweight post-stage guard snapshot (its full-rescan flag is false); the linked acceptance aggregate explains that field and includes a fresh final full guard. Full release/corpus/integration checks remain separately retained. The historical 10320 temporary files and 6 older artifacts are still missing; these measurements do not recover them or repair the old controller's lost references.

There is no warning-free-tree verdict: 262 OUTSIDE warnings and 15 unwaived full-main suffix findings remain. Whole-main and independent engineering acceptance remain open. Next is the shared expected-type handoff for qualified/rank-n arguments, starting from original tc092; ordinary metas must remain monotypes, and contexts must not be stripped to manufacture acceptance. Root continues under the existing goal, with no development pause, new resource/credential authority, public release or portfolio queue requested.
