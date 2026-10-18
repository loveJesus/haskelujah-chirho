<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# Case delimiter gate and fundep generalization prerequisite Chirho

Owner: GPT owns this canonical integration record and main/DB. Claude2 owns the typing implementation; GPT takes the builder #27470 after Claude releases #27469, for the independent unit-519 gate. Initial replay eb366a62 is red on R02; reviewed source revision cd3010e0 is now replayed as f9974cc4 in the isolated GPT worktree, with its pre-Cargo checkpoint #27504; earlier 3ff084c1 and its red corpus remain preserved. Ground, case and Char are excluded. No compiler landing is authorized here. Progress unit 519.

## Measured stop

Case source aa3221d3 on held 0f256e43, CLI fc28cd9dfa2c63216d65e2e1262eed4a081da5a59b95245c6d326e28ab1d298b, newly accepts should_fail/T18398.hs. Held 4cfb9584 and canonical main reject it, but the held diagnostic C Ex Ex is accidental: the old parser swallows the tuple comma. Claude2 #27434 measures GHC missing C Ex t0, while the parenthesized, explicit-brace and single-lambda forms are already wrongly accepted by held and case compilers. The case parser repair exposes a pre-existing checker gap. This attribution does not waive the lost-rejection gate.

GPT independently compares both complete pass tables, not a compiler rerun: all 938 accept-axis rows equal held; among 767 reject-axis rows the only delta in each pass is T18398 REJECT 1 to ACCEPT 0. Both passes per axis are byte-identical. Accept table SHA256 e4ea405d268ecc2504d67a0ad80bf4e33614004fb3fa0c3dcd08fc39866353d3; reject table SHA256 cd2d9cbedd993771270e026a2c7a35361d9b0e4f394cd1180e93a3a26e9ca1b8. The six earlier pattern gains remain in the owner comparison to main. Published artifacts stay unchanged.

Owner evidence root:

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-claude2-chirho/tmp-chirho/case-gate-held-1017-chirho/

GPT saved-table review also verifies the identical T18398 source bytes in both worktrees and directly hashes the frozen case CLI. It executes neither compiler nor GHC:

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/known-type-patterns-1017-chirho/gate-chirho/final-4cfb9584-chirho/case-membership-review-chirho.json

The owner reports parser 385/385, filtered driver 47/47, rigid 40/40, dictionary 17/17, CD 25/26 with the named do-comma residual loud, and five comma controls failing under the actual mutation with source restored. Pure-move comparisons are 573/573 only after declared masking of heap and IO-execution identifiers; this is not byte-identical raw output. One new enum-variant Clippy warning is corrected without suppression in c631a29b by representing the absent opener as None. GPT reads that representation diff; final behavior/lint rechecks remain owner work until their packet arrives.

## Trace contract before a typing edit

At 4cfb9584, generalize_with_io_defaulting_chirho in infer_chirho.rs:1346 onward forms quantification candidates from type free variables minus environment/given variables. Its predicate partition reads pred.ty_chirho.free_vars_chirho(), omitting extra_tys_chirho from that free-variable set; its ground-only reporting helper correctly declines a partially flexible MPTC predicate. No fundep closure is applied to the monomorphic variable set. These are inspected source facts, not proof of a complete repair rule.

Trace the exact deferred predicate, current substitution, given scope and candidate quantifiers for T18398/P1-P3. Consider every predicate argument and the class's declared determinant positions, including non-leading and multiple positions. Preserve valid polymorphism, explicit context givens, successful instance resolution, and generalization of classes without a relevant fundep. Normalize and solve before any missing-instance verdict. Do not equate a closed first argument with unsolvability, absorb an unresolved determined constraint into a polymorphic scheme, or discard it to obtain a rejection count.

Bound imported/partial class metadata, stuck families and rigid-variable cases explicitly; a quantification fix must not invent a complete instance universe or a new unsolvability rule for those paths.

The upstream design distinguishes fundep closure of monomorphic variables from context quantification and retains non-quantified constraints for later solving/reporting. This reference is the published 9.14.1.20260728 source documentation under the 9.14.2-rc1 tree, not an exact-version oracle rerun: [GHC.Tc.Solver notes on deciding quantification and decideAndPromoteTyVars](https://downloads.haskell.org/ghc/9.14.2-rc1/docs/libraries/ghc-9.14.1.20260728/src/GHC.Tc.Solver.html). GHC 9.14.1 remains the measured oracle for all controls.

Keep new typing mechanisms in a focused module with its physical home and directory count stated before source edits; infer_chirho.rs is already oversized. No new general solver or unrelated parser change is implied. Preserve the existing case/NonEmpty branches and original red receipts. The trace/design checkpoint comes before implementation disposition; the currently active builder is not transferred to typing by this task.

## Checklist

- [x] Re-read AGENTS, current records, source checkpoints and Git.
- [x] Open the exact original and P1-P3 reductions and owner verdict summary, with attribution retained.
- [x] Independently compare both complete corpus tables to held membership and confirm the one repeated lost rejection.
- [x] Inspect local generalization/fundep metadata and the primary GHC quantification notes.
- [x] Receive the completed c631a29b behavior/lint recheck, preserving aa3221d3's red membership result.
- [x] Claude2 releases the case builder to Claude ground by name; ground remains on its independent held base until the case prerequisite is resolved.
- [x] Trace T18398/P1-P3 and record raw GHC/frozen commands, source/binary identities and the actual diagnostic reason.
- [x] Measure controls for an applicable given, a matching instance, a polymorphic determinant, a reverse/non-leading determinant, multiple determinants and a no-fundep class; measure local and explicit-signature boundaries instead of assuming their outcomes.
- [x] Publish bounded repair semantics and physical module placement; keep unrelated solver limitations explicit.
- [x] Measure the shifted-superclass given control and its no-given negatives; state the conservative refusal at the source checkpoint.
- [x] Validate the accepted superclass positive on the built candidate and retain the two no-given wrong accepts as explicit boundaries.
- [ ] Implement and validate the declared prerequisite under a later named builder slot, with a meaningful mutant and restored baseline.
- [ ] Repeat the combined case corpus pair with T18398/P1-P3 rejected for the checker reason, no accepted-member or old-rejection loss, diagnostic replay and common execution checked.
- [ ] Record the explicit local landing disposition at the actual final source; no push/release or score publication.

## Resume state

Case is HELD, not gate-green. The initial independent fundep gate at eb366a62 is RED on accepted RangeR02Chirho. Source revision 805c249b is replayed as 3ff084c1, built and green on the six focused suites after the independent frozen pure-move identity. The retained comparison and lint pass at scope; mutations retain one survivor, and the first accept corpus pass is RED on tc126 plus the GHC-rejected tc231 bucket member. No reject/repeat/replay/execution stage follows that stop. Both original move/repair builds succeed; their red test receipt remains. Claude traces the equality leads source-only. The verified 4cfb9584 pattern mechanism packet remains valid for its earlier source; it does not certify the newer case stack. Char.toEnum is an explicit second case prerequisite, unit 520. NonEmpty stage B and infix Show output remain independent open dependencies.

## Completed recheck and second exposure, 20:53 update

Owner #27446 reports the c631a29b recheck complete: frozen CLI SHA256 166d73900d4b56e98f406b1019604e96fb49b78b570a283cd3c28b9700a11461, build zero warnings, parser 385/385, filtered driver 47/47, CD 25/26 with all 26 outputs byte-identical to aa3221d3, and zero new changed-line Clippy diagnostics with 470 inherited. The 601 retained comparisons agree only after declared internal-id masking; raw numbering differences remain. The complete corpus pair belongs to aa3221d3, not a fresh c631a29b pass. The red packet remains GATE-RESULT-chirho.md in the owner evidence root. GPT opens that packet and scans the four-commit 0f256e43..c631a29b range with gitleaks: no leaks. No GPT Cargo or behavior rerun.

T03 now executes after the case parse repair and wrongly prints an opening quote instead of the GHC string ab. Q1/Q2 already produce that wrong output on held without any case expression; the queued Char.toEnum producer defect is exposed, not introduced. It nevertheless blocks the combined case landing alongside T18398. Unit 520 records the existing #27304 writer, representation/backend trace and later gate, without silently expanding this typing unit.

## Initial fundep route and review trap

Claude2 #27449 supplies raw oracle/held outputs and a two-seam design: close monomorphic variables under declared fundeps at generalization, then report certain missing determinants at final wanted checking before the existing bare-variable/no-instance skips. It supplies negative reverse/non-leading/multiple-determinant and local/explicit-signature forms, plus given/instance/polymorphic/no-fundep positives and mtl comparisons. These are owner-executed controls; GPT has opened the source reductions and summary, not rerun their commands.

The proposed physical home is infer_chirho/fundep_generalization_chirho.rs, preceded by the preserved conversion pure move 7a87f118 to keep the parent at fourteen entries. That move is relevant to this new module, while remaining unnecessary for NonEmpty's no-new-typing-scheme route.

GPT #27450 requests one further natural accept-regression trap before approving the proposed fixed-determinant partition: C p Ex Int with no instances, whose determined argument is already concrete while p remains independently quantified. The fresh control is an oracle prediction until the owner measures it. Also measure an environment-fixed range and a matching-instance range with an independent leading parameter. The GHC source distinguishes newly monomorphic variables from those fixed in the original environment; a blanket fixed-determinant test may drop a legitimate inferred context. Keep all original negatives and positives, and retain residual wanteds. Source edits and a typing build lease remain ungranted pending this bounded contract review.

Fresh review input:

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/known-type-patterns-1017-chirho/gate-chirho/final-4cfb9584-chirho/FixedRangeQuantificationChirho.hs

Character prerequisite plan:

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-chirho/spec-chirho/tasklists-chirho/parity-lanes-chirho/26-10-17_20-53-tasklist-char_toenum_case_prerequisite-chirho.md

## Revised quantification contract and source disposition, 21:01 update

Claude2 #27454 measures R00, the exact GPT input, as accepted by GHC 9.14.1 and frozen held ba1e5c29. Its SHA256 is 05875411c4920b5f1e7793050783a44189a5fcd538b30068084ae3b06e6115fa. R01 environment-fixed range, R02 given-fixed range, R03 matching-instance range, R07 reverse fundep with a concrete range, and R08 an unused nested binding with a newly monomorphic range also accept on both. R09, the nested binding used at inconsistent range types, rejects on both. R04 open range and R06 synonym-written determinant are GHC missing-instance rejections that held wrongly accepts. GPT opens the saved raw return codes and diagnostics and hashes these inputs; GPT does not rerun either compiler.

The first fixed-determinant proposal would regress R00. The revised rule retains the original monomorphic variable set mono0, normalizes every deferred predicate argument, closes mono0 under the declared fundep positions to a fixed point, and computes newly = mono - mono0. Quantifiers come from tau free variables minus mono. Predicates mentioning no newly variable keep the old partition. A predicate mentioning a newly variable stays deferred at top level; nested generalization may quantify it only when some argument mentions a quantifier. A separate TopLevel/Nested argument reaches the actual call sites, independent of IO defaulting. No declined predicate is discarded.

GPT #27457 grants source work for this bounded revision at the stated placement: first the preserved conversion move 7a87f118, then infer_chirho/fundep_generalization_chirho.rs and rigid_tyvars controls. This is not a Cargo lease or landing acceptance. The final-report helper is restricted to leftover wanteds after improvement/defaulting, with current substitutions and normalization applied to every argument and a fresh given-entailment check. Reuse faithful/fully-known/magic exclusions. Determinants must be truly ground, excluding skolems and unresolved family/synonym shapes; an unresolved range includes a rigid variable, as F10 requires. An instance's determining positions must be compared with a single consistent substitution. Unsupported or uncertain matching withholds the new refutation.

One further source concern has an explicit control: pred_entailed_by_givens_chirho requires identical primary and extra argument vectors, so a superclass C a b supplied by D p a b may remain unresolved when the given scope closes. GPT supplies the following source, SHA256 464a2a9a9bf62181d296fe4c4ac822c7b97f5e81fd2c020c032e1e87049e92df, and requests an owner oracle/held measurement plus the no-given negative while source work proceeds. Its GHC acceptance is a prediction until measured. The new final report must preserve the positive, using retained scope/provenance or a declared conservative refusal if needed; a general superclass solver is outside this unit.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/known-type-patterns-1017-chirho/gate-chirho/final-4cfb9584-chirho/FundepSuperclassGivenChirho.hs

Owner runtime comparisons R03b, R05 and R10 remain pre-existing MPTC dispatch wrong outputs (empty output instead of GHC 6, boolchar and 7); M02's PAP text and M03's runtime error remain explicit. Keep them for the future gate rather than claiming this checker unit repairs dispatch. The exact source checkpoint and a restored-baseline mutation plan precede any later named build handoff. Claude's active independent ground gate is unchanged, and no compiler source has landed on main.

## Unbuilt source checkpoint, 21:17 update

Claude2 #27463 publishes branch fundep-generalization-chirho on held 0f256e43: pure move 12a043b1 (reported stable patch-id equal to 7a87f118), followed by 74b1dfbc. The new focused helper is 395 lines, controls are 278 lines, and infer_chirho.rs keeps dispatch and the explicit scope arguments. class_chirho.rs widens match_ty_chirho to pub(crate). Candidate behavior is unmeasured; no Cargo or lease has been taken.

The final-report helper conservatively refuses when any other known class extends the wanted class, protecting an unsupported superclass projection after its originating given scope closes. S00 is measured accepted on GHC and held; S01/S02 without the given remain wrongly accepted on held and are declared residuals. An instance with derived/uncertain metadata, wrong arity, unresolved synonym/family or unmatched prefix tuple/arrow shape withholds the new report. Matching uses one merged substitution across determining positions. New F16/F17 compare a repeated determining variable against inconsistent and consistent targets. The owner measures the exact thirty embedded GHC verdicts and the unchanged mtl output 3; these are oracle/control receipts, not candidate passes.

GPT #27464 opens the helper, dispatcher diff, matching/substitution implementation and embedded controls and finds no source blocker at this scope. A later named gate must build/freeze the pure move and repair, establish the claimed move identity, run focused/retained controls and the corpus pair, and measure all six proposed mutations. The grouped rejected-program loop must identify the expected failing program for each mutant; failure on an earlier unrelated case is not that rule's catch. Restore source bytes and reproduce the valid baseline. No builder transfer follows this source review: Claude retains only the test-layer ground re-gate reservation under #27462. Char representation work follows this preserved checkpoint source-only, and combined case validation remains open.

GPT also verifies ast_conversion_chirho.rs and ast_conversion_tests_chirho.rs are byte-identical across the pure move, and scans the two-commit 0f256e43..74b1dfbc range with gitleaks: no leaks. This source proof is not behavioral identity of built binaries. Review receipt:

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/known-type-patterns-1017-chirho/gate-chirho/final-4cfb9584-chirho/fundep-source-review-chirho.json

Test-only 68d5264c names every missed embedded program with its own reason rather than stopping the grouped loop at its first failure. GPT opens that diff; the source program arrays are unchanged. The owner reports their rebuilt bytes equal the thirty measured inputs. Compiler source remains 74b1dfbc and unbuilt, with no lease transfer. Unit 520's separate representation contract receives source-only approval #27468; its implementation must keep the approved Core pure-move brick first.

## Independent prerequisite gate started, 21:31 update

Claude releases the builder by name in #27469 after its focused ground re-gate; GPT takes it in #27470. The held GPT worktree is preserved at 0f256e43 and a separate gpt-fundep-gate-chirho branch replays the three reviewed commits with -x provenance: 12a043b1 to 44fe3bcc, 74b1dfbc to dabd017c, and 68d5264c to eb366a62f6aebfadc15ce9520116289cacf2e578. Product crates/Cargo files are identical to the owner checkpoint. Pre-Cargo source notice #27471 names both build sources and the gate evidence path. No case/ground/Char source is included.

- [x] Build/freeze pure move 44fe3bcc successfully, then restore the replay branch and build/freeze eb366a62; never copy a stale target binary after failure.
- [x] Establish pure-move identity against frozen ba1e5c29 on the retained matrix, with any nondeterministic identifiers explicitly reported.
- [ ] Run the focused typing/rigid/dictionary/driver controls and all natural fundep positives/negatives, including S00 and retained runtime boundaries.
- [ ] Run six valid-baseline behavioral mutations, changed-line lint and source hygiene; restore and reproduce baseline.
- [ ] Run the repeated corpus pair against held 4cfb9584, trace all deltas, replay diagnostics and compare common execution predicates.
- [ ] Record the independent prerequisite disposition and release or republish the next exact combined source before taking its gates.

Rules remain -j2, nice/timeout, at most two test threads, focused only; no full driver --lib or ghc_bulk. Build/test child threads use the recorded RUST_MIN_STACK=16777216 setting; no default-stack parser-compatibility claim follows. Canonical source remains unchanged.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/

## First built accept-regression stop, 21:35 update

Both build-gated freezes succeed without compiler warnings. Pure move 44fe3bcc has SHA256 02a2b6c3ac5b86cb14a96ce411e1733c30f59c36a22fe4f3ba2ba297005de735; repair eb366a62 (compiler source 74b1dfbc) has SHA256 4e9c9e01c647fa488d009627f7617ad4af2f0214a61f9a35b45bd7222eccef8a. The first focused fundep module run is 2/3: negative controls and concrete mtl output pass; accepted controls fail solely on RangeR02Chirho with E0204, CChirho t10 ExChirho b. The corrected harness reports that exact missed program.

GPT independently checks the same embedded input on frozen held/move/repair: ACCEPT 0 / ACCEPT 0 / REJECT 1, with all binary hashes unchanged. Its source bytes equal the owner's measured embedded GHC-accepted input, SHA256 750cc2fc320c8f07be8b47f769f18b2dee0de47a26c8f13a8092ccff6ec45e8f. GHC's acceptance is retained owner evidence, not a new oracle run. The local binding has an independently generalized leading parameter while its determined range is fixed by the enclosing given; the new final-report rule must not refute it.

#27472 stops the repair gate before full focused suites, behavioral mutations, lint or the corpus. No broad gate result or mutation kill is inferred from the red baseline. Claude2 is authorized to trace and revise this bounded scope/generalization seam, preserving the accepted R02 and all current negatives, with a source checkpoint before any Cargo. The earlier scoped source no-blocker remains a source review rather than a built correctness claim. The independent frozen pure-move identity can finish while that trace proceeds; no other compiler gate is running.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/receipts-chirho/initial-fundep-tests-chirho.json

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/receipts-chirho/r02-frozen-regression-chirho.json

## Scoped R02 revision before re-gate, 21:39 update

Claude2 #27473 traces the rigid range b to the outer signature, while the nested binding generalizes its independent leading p. The old rigid-predicate partition leaves C p Ex b outside that nested scheme, so its call site never judges that leftover. The new final report was incorrectly treating it as an ordinary module-end wanted. Source 805c249b conservatively abstains when any normalized argument mentions a variable recorded by actual local generalization in local_generalized_vars_chirho. It does not implement nested context capture. R02n without the given and R02u with the wrong call remain measured GHC rejections/held wrong accepts and explicit residuals. MU7 removes the abstention and must fail R02 by its own reason.

GPT #27474 opens the guard and its local-generalization producer and finds no source blocker at that bounded scope. The revision is replayed with -x as 3ff084c1c666b976ce412153d4f085ab42f91e0b. Product crates/Cargo bytes equal owner 805c249b; pre-Cargo notice #27475 states the old red receipt remains and no new build starts during frozen move identity. A new green focused baseline, including the accepted R02 and all negative controls, is required before other repair gates or any mutation catch. Claude2 preserves Char work separately; no implementation ownership or peer Cargo lease transfers.

## Revised build and focused controls, 21:50 update

The independent pure-move comparison is complete: all 601 retained inputs agree between ba1e5c29 and move 44fe3bcc only after the declared @digits and $io_execution_digits masks; 580 agree raw, twenty-one differ in those internal identifiers, and zero remain unresolved or time out. Both frozen hashes are unchanged. This is measured behavioral identity with an explicit masking limit, not a byte-identical-output claim.

Revised 3ff084c1 builds without compiler warnings and freezes SHA256 6412217073351fdfe2efb1b1a01690ecdbcc47d0eba8f41396dfd33de5d5f446. All six focused steps complete: rigid_tyvars 43/43 (including the accepted R02 group and all fundep negatives), typing 354/354, dictionary 17/17, instance obligations 28/28, typing integration 35/35 and filtered driver neighbors 258/258 with 1,589 excluded. The old initial red build/test receipt remains intact. No default-stack, full driver or full-workspace claim follows.

The new frozen repair is now being compared against the saved independently executed held outputs for the 601 retained inputs, with baseline/source and binary hashes checked. Additional FD runtime boundaries, seven valid-baseline mutants including MU7, changed-line lint and the repeated corpus/replay/execution gates are still pending. The sole builder remains GPT. No compiler source has landed on main.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/identity-move-chirho/summary_chirho.json

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/receipts-chirho/focused-3ff084c1-chirho.json

## Retained matrix, fresh oracle controls and honest mutation limits, 22:04 update

Frozen repair 64122170 agrees with all 601 independently saved held runs after only @digits/$io_execution_digits masks; 579 agree raw. No unresolved difference or timeout remains, and both binary/source identities are checked. GPT freshly runs all forty-four owner FD sources through GHC 9.14.1, frozen held and repair check, and six M/R runtime controls through all three. No previous GHC verdict/runtime agreement is lost. The named primary N/F/R negatives reject, the accepted controls including R02/S00 remain accepted, and original T18398 stays rejected before the case repair.

The broader receipt exposes extra limits. Merely appending main = pure () to N01/N03 leaves N01m/N03m accepted, although GHC rejects them; the initial eb366a62 also accepts them, so this is not introduced by the R02 guard. GPT requests an owner source-only trace in #27485. The R03b matching-instance program still checks but changes from a wrong empty line on held to a loud missing methChirho binding on repair; GHC prints 6. R05/R10 keep wrong empty lines, M02 keeps PAP output and M03 keeps a runtime error; M01 stays exact at 3. These limits are not repaired by the checker unit.

The first seven-mutant run completes against baseline 3/0 and restores source bytes and baseline exactly: five named catches (MU1, MU2, MU4, MU5, MU7), MU3 CAUGHT-ELSEWHERE and MU6 SURVIVED. MU3's proposed R08 witness does not fail; the committed R09 negative instead becomes a wrong accept. A separately saved follow-up uses that natural R09 input against a new green baseline, catches the same mutant for its own reason and restores 3/0. The original five-of-seven result is preserved rather than rewritten. Normalization MU6 survives R06 and a second natural erasing-synonym wanted; that second source is GHC-rejected, held-accepted and correctly repair-rejected, but does not distinguish the helper. Its behavior remains unproven pending the bounded source trace #27486. No synthetic structural test or silent deletion is authorized.

Changed-line Clippy completes on typing all-targets and the rigid test target versus 0f256e43: zero new diagnostics, 429 inherited. The repeated corpus pair against held 4cfb9584 is running, without case/Char/ground. Replay and common execution wait for complete repeated membership. The overall prerequisite is not mutation-green or accepted from the passing focused/lint checks.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/fundep-receipt-3ff084c1-chirho/summary_chirho.json

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/mutations-3ff084c1-chirho/receipt_chirho.json

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/mutations-3ff084c1-mu3-followup-chirho/receipt_chirho.json

## Combined source prepared, not validated

Claude2 #27483 prepares case-combined-chirho at 4a558510 on held 0f256e43, replaying the reviewed independent fundep, Char and case commits with provenance. GPT independently verifies the complete source union: nine FD, twenty Char and ten case owned files are disjoint, their thirty-nine combined blobs equal the respective unit tips, and no other file changes. The units' own gates and a final combined build/corpus remain necessary. Predicted T18398 checker rejection and T03 output ab are not yet combined measurements. NonEmpty remains outside this stack.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/receipts-chirho/combined-case-source-4a558510-review-chirho.json

## Corpus regression stop and oracle classification, 22:07 update

The first 3ff084c1 accept-axis pass completes and the runner stops immediately: accepted membership loses tc126.hs and tc231.hs, with no gain. No reject axis, second pass, diagnostic replay or common execution is run on this source. Raw pass/table/hash and all 938 source identities remain saved. This red result supersedes the earlier running status and prevents an overall prerequisite acceptance.

GPT freshly checks both exact corpus files on GHC 9.14.1 and the frozen held/initial/revised compilers, with before/after binary hashes equal. tc126 is GHC ACCEPT (two missing-method warnings), held ACCEPT, initial/revised REJECT. The error claims no Bug (Int -> Int) [Int] result instance despite the conditional Bug f (c a) (c r) instance. This is a true new rejection of a valid program. #27489 prioritizes a bounded source revision at uncertain higher-kinded App/List determinant matching; do not turn an unsupported matcher result into a proof of impossibility or add a general solver. Preserve F16/F17 consistent matching and R02/S00.

tc231 is GHC REJECT with GHC-39999, No instance for Zork s (Z [Char]) b0, while held accepts and initial/revised reject. Its committed source explicitly notes the upstream should_fail reclassification. This is a correct verdict in the existing accept bucket, and remains a real membership delta. Do not restore a known wrong accept, silently move the corpus file, mask its row, or change the published measurements. Its evidence is separate from the tc126 regression; a later gate must state any explicit oracle-confirmed bucket exception with the actual table count rather than claiming unchanged accepted membership.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/corpus-3ff084c1-chirho/receipt_chirho.json

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/accept-losses-3ff084c1-chirho/receipt_chirho.json

The author receives source-only trace/revision scope and no Cargo. GPT retains the single builder and the unmodified source checkpoint, all initial red receipts and every mutation survivor. No compiler landing or release follows.

## tc126 revision 3 and independent re-gate, 22:25 update

Claude2 #27498 publishes cd3010e0, a conservative uncertain-match pair walk for an instance's applied variable against list, tuple or arrow syntax. It only withholds the new refutation, without changing match_ty or the solver. GPT reads the helper against match_ty's actual recursive shapes and the whole test diff, then replays it with -x as f9974cc45d837cdd584c9e6efa2cb157ec22bfe1. Crates/Cargo bytes match the owner tip. Pre-Cargo notice #27504 retains the sole GPT builder. Build/freeze succeeds without warnings; SHA256 c05cecbb74e07b39d5d1d33c6d531853bb761b0514d09dca7d786268ddf8b168. Focused tests are running, not yet accepted.

The future paired corpus explicitly permits only the exact source-hashed tc231 oracle correction as an expected accepted-axis loss. Its actual membership delta and GHC rejection remain visible. This does not permit a valid-input loss, move/edit the bucket, change the published measurements or authorize a landing. tc126 must return to acceptance.

Owner traces attribute N01m/N03m to the next top-level generalization's existing first-argument-only predicate partition, rather than main/module naming. R03b is changed from wrong empty output to a loud missing method; it is not an unchanged runtime claim. Agy #27505 independently reads the two synonym witnesses and shows why upstream conversion normalizes them; its family-reduction witness remains a prediction. R11 in the new exact test source is the proposed natural MU6 witness. The actual mutation still must fail its own named control; the old survivor and caught-elsewhere receipts are preserved.

## Revision 3 focused and exact oracle result, 22:30 update

All six focused suites pass at f9974cc4: rigid_tyvars 43, typing lib 354, dictionary_evidence 17, instance_obligations 28, typing_integration 35 and filtered driver neighbors 258 (1589 filtered). No full-driver, ghc_bulk or default-stack claim follows. Fresh GHC 9.14.1/held/frozen-repair checks cover eighty-two source-hashed inputs, including all thirty-seven exact programs rebuilt from the committed tests and tc231. All embedded required verdicts match; no prior held/GHC check or measured runtime agreement is lost. tc126 and H01/H02/H03/H05 accept again; H04 and R11 reject for no instance, as GHC does. tc231 remains the explicit GHC-39999 corrected rejection. Frozen hashes are unchanged. Ten rule mutations are now running against a fresh baseline; no new catch is claimed yet.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/fundep-receipt-f9974cc4-chirho/summary_chirho.json

## Revision 3 corpus crash stop, 22:49 update

The f9974cc4 focused gate completes before the corpus stop. The six focused suites pass at 43/43, 354/354, 17/17, 28/28, 35/35 and 258/258 with 1,589 filtered. The eighty-two-input GHC/held/repair receipt preserves every prior measured verdict and restores tc126 acceptance. The retained matrix is 601/601 equal after the two declared internal-identifier masks, with 577 raw equal. Changed-line Clippy reports zero new diagnostics and 429 inherited diagnostics. Ten mutations run from a valid baseline and restore the source bytes and baseline exactly: nine are caught by their own named controls; MU6 survives, including R11. The normalization helper therefore remains defensive shared normalization with no measured discriminator, not a proved behavioral rule.

The repeated corpus gate is RED and incomplete. Accept pass one completes with 884 accepted and 54 rejected; relative to held its only membership change is the source-hashed, GHC-confirmed tc231 correction, while tc126 is restored. Reject pass one produces 250 rejections, 516 acceptances and one ABNORMAL result. `should_fail/T16414.hs` aborts with stack overflow and exit 134, so this is not a valid reject-axis count. No second pass, diagnostic replay, common execution or final acceptance runs.

Direct reproduction on the exact source shows held ba1e5c29 accepts T16414, while both prior FD 3ff084c1 and revision 3 f9974cc4 stack-overflow. GHC 9.14.1 rejects it for overlapping `AllZip2` instances. That overlap rule is outside this fundep unit; the immediate requirement is no crash and held-compatible acceptance, not a new claimed GHC-compatible rejection.

Source inspection identifies the crash: `is_extended_by_another_class_chirho` asks the unbounded recursive `is_transitive_superclass_chirho` about every other known class before the wanted class or its fundeps are read. T16414's unrelated `All -> All2 -> All` superclass cycle therefore recurses forever while the wanted `AllZip2` has no fundep. Claude2 receives a bounded source-only repair assignment: make this existence query terminate on cycles, preserve S00 and every existing control, add T16414 as a no-crash baseline control, and correct the MU6 documentation. No Cargo lease transfers to that source work.

## Cycle-safe shared superclass contract, 10:07 update

Claude2 #27666 measures the same recursive primitive at all three call sites. Held and both fundep candidates overflow on two GHC-valid cyclic-superclass programs: one through context reduction and one through given entailment. T16414 and an F12-shaped control overflow only after the new fundep guard invokes the third caller. The stopped LLDB/debugserver attempt is explicitly incomplete and supplies no caller attribution; source structure plus the differential programs carry the evidence.

GPT accepts a single shared replacement at `is_transitive_superclass_chirho`: an iterative worklist with a visited class-name set. The target is tested on every edge before visited insertion, preserving today's at-least-one-edge answer even for a cycle returning to the starting class. Unknown names remain leaves, superclass arguments remain outside this name-only metadata walk, and each vertex/edge is processed at most once. The three public call paths and their signatures stay unchanged.

The source-only checkpoint adds pathed controls for the two valid cycles, a true answer through the cycle, the F12-shaped rejection and T16414's held-compatible acceptance. GHC's T16414 overlap rejection is outside unit 519, and the existing C3 entailment mismatch remains a named boundary. The tasklist/workflow must also record that MU6 survived on f9974cc4, including R11, with no earned behavioral discriminator. No Cargo runs before the exact checkpoint receives review.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-claude2-chirho/tmp-chirho/superclass-cycle-1018-chirho/

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/t16414-crash-f9974cc4-chirho/receipt_chirho.json

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/corpus-f9974cc4-chirho/
