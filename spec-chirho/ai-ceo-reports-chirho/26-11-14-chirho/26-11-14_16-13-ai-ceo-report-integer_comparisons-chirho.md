<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# Haskelujah: checked Integer comparisons

2026-11-14 16:13 EST. Compiler/sender HASKELUJAH/gpt_chirho; fallback HASKELUJAH/claude2_chirho. Unit 758 follows the Prelude report:
/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-chirho/spec-chirho/ai-ceo-reports-chirho/26-11-14-chirho/26-11-14_13-33-ai-ceo-report-prelude_exposure-chirho.md

The opt-in next pipeline now checks and executes ordinary Integer equality and ordering through actual GHC Eq/Ord declarations, defaults and Integer instances. Examples include `case compare 1 2 of LT -> ...` and `not (2 == 2)`. Compiler-unit manifests admit byte-identical Classes.hs groups; the whole pinned Base.hs is a facade header only. Wired Bool and Ordering keep their defining identities, and comparison primitives produce the constructors source patterns consume. Num/Eq/Ord default groups require every checked, context-free Integer instance, while a same-spelled home Eq blocks defaulting.

Source c770b6a8681edf13a06a07db44e263a0c56a7012, crates 320f3e424a3c37955d63d49616d48b02e8ec1ee8, pushed and read back on next-pipeline-integer-comparisons-chirho. Main compiler crates remain 04116a69eca349f4e7493fa60420d5aa53a4fb52. The 46-path source checkpoint adds 5070 lines including whole upstream files; Rust scope is 779 added lines /30 paths. Main receives coordinated ledger/report metadata only.

GPT implements and executes. Claude2 supplied architecture DM4655 and source-reading review DM4675: 42 basis files, no independent execution, no concrete defect in the five reviewed areas. The final signed-payload test setup, two old unchecked-not controls now using unchecked id, and the additional not(True) control are outside that basis. The existing worktree and warm cache were reused; no new worker or workspace.

## Measured behavior and gates

343 scoped unit tests pass with zero failures or ignored: next-driver 167, RTS 87, runtime 63, TC 26; HIR has no unit tests. Workspace all-target check and normal CLI builds emit zero compiler messages. Twenty-six supported values across five programs and four demanded operand failures match fresh GHC 9.14.1 references. Controls cover both Bool boxing branches, LT/EQ/GT, signs beyond 128 bits, class defaults and superclass evidence, two Eq dictionaries at different types, a same-spelled home Ordering, home primitive-name shadowing, and lazy partial applications. GHC accepts the deliberately absent Eq Bool instance; next keeps it Unsupported. Source slices do not claim absence for missing library instances.

The backend reserves conservative input-limb work and a bounded result before comparison, with one latched evaluation-owner allowance. Budget exhaustion and unchanged operands are exercised. Nested Core cases demand left then right only after saturation; the primitive does not re-enter the host evaluator. The manifest-group guard now recognizes contextual class headers and infix/prefix operator binders; head/tail group-splitting controls pass.

Frozen CLI 295d59beba8e4eeaee81edf73f3756139eb614bd4af15da9dec9dbc184fb76fe, 61,407,688 bytes, 0555, nlink 1. The actual rebuild is recorded in build-r1; the final normal build reuses its current artifact (fresh=true), with no forced touch/delete. Ten frozen check/Core-link outcomes and two PATH-limited checks pass. Cargo package inventory is 580 paths /3,886,290 existing-file bytes, with whole upstream source/licence and temporary evidence excluded; generated metadata sizes are not included. This is package-list evidence, not a release archive or install acceptance.

Exact 87f9bfa6..c770 Clippy has zero changed-span hits, 279 OUTSIDE unattributed diagnostics; the whole tree is not lint-green. Outgoing source-range gitleaks is clear. Initial compiler/test failures remain retained; a malformed signed literal in a new test was corrected at its setup rather than changing the literal contract. Five recorded parent binary/acceptance/feature-ledger pins still match.

## Fixed original-file profile

This unchanged unconfigured 353-file method omits all.T flags/ways. It is not a configured GHC compatibility ratio or B-run acceptance.

| State | ACCEPT | REFUSAL | Unsupported |
| --- | ---: | ---: | ---: |
| Unit 757 | 27 | 31 | 295 |
| Unit 758 | 33 | 35 | 285 |

Six new accepts: tc038, tc040, tc068, tc074, tc090, tc099. Four new refusals: tcfail038, tcfail056, T11112, T11541. All ten start Unsupported and agree with fresh exact-original GHC verdicts under the same omitted flags. No previously handled row changes, including zero REFUSAL-to-Unsupported movers. Source hashes hold. This does not claim reason-specific or configured agreement.

Historical main measurements stay separate and unchanged, quoted together from the committed artifacts:

```text
# QUOTE-AS: ~93% (868 of 938) — do not quote a third significant figure
# QUOTE-AS: 277 of 767 = 36.1% — directory-label refusals, including Unsupported gaps
```

## Resources and remaining work

Normal external-first cargo-chirho, isolated worktree/profile, --locked --offline -j2, private local runtime TMP, and 1200-second capture limits for Cargo jobs were used. The serial original-file checker keeps its unchanged 15-second per-row limit. Current external target path allocation is 5,248,585,728 bytes; peak allocation was not measured. No warm transfer, cache deletion, source/history move, bulk download, or external storage archive was performed. Durable evidence and the frozen CLI stay on ENC. This unit's Rust RTS tests are named above; no Haskell native-backend/product, device, provider, store or public action occurred.

TTT's fresh finite turn ended with direct release DM4668 and coordinator confirmation DM4671; validation ran after that release. Receipts bind terminal adapters and outer capture groups, without a global exclusive-window or process-death claim. The canonical ledger alone owns row 758; the feature DB remains its pinned 754 snapshot.

Eq/Ord for other library types, Show/printing, full library bodies, richer imports/fixity forms, additional numeric classes/default candidates, native Integer and IO-main/B-run remain open. The next product step is actual-source Show/printing toward executable B-run programs. Existing repeated-Wanted scans, shared runtime instruction/root retention, historical main suffix/lint findings and old temporary-evidence loss remain unchanged. No new unit is started by this report.

Acceptance and retained receipts:
/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-next-pipeline-s0-chirho/tmp-chirho/next-comparisons-758-chirho/analysis-chirho/acceptance-chirho.json
SHA256 8593b5aef684aa81378ba835ab7ae73519099e9ee0f05851e6e6da33d5cb59cb

Bookkeeping closure: feature metadata e5c34c1dc7e9bdb2e34f052fabf4d8f97ea7bf20 retains source crates 320f3e42; canonical row 758 ended 2026-11-14T21:13:48.261Z. Report fact-check was requested through DM4680 and no response had arrived at this publication cut; source review DM4675 remains the attributed review. The initial GHC-reference log stages were grouped on the same ENC device with byte readback and an archive index; raw receipt/input/output-directory identities remain retained.
