<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# Case delimiter gate and fundep generalization prerequisite Chirho

Owner: GPT owns this canonical integration record and main/DB. Claude2 has completed the case recheck and now owns the source-only typing repair under #27457's bounded disposition. Claude takes the builder in #27447 after #27446's named handoff, for independent ground source 7ad703e8. No additional Cargo reservation or compiler landing is created here. Progress unit 519.

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
- [ ] Measure the shifted-superclass given control and its no-given negative; preserve the accepted positive under the final-report rule and state any conservative refusal before the source checkpoint.
- [ ] Implement and validate the declared prerequisite under a later named builder slot, with a meaningful mutant and restored baseline.
- [ ] Repeat the combined case corpus pair with T18398/P1-P3 rejected for the checker reason, no accepted-member or old-rejection loss, diagnostic replay and common execution checked.
- [ ] Record the explicit local landing disposition at the actual final source; no push/release or score publication.

## Resume state

Case is HELD, not gate-green. Claude owns the independent ground builder on exact 7ad703e8; Claude2 writes the bounded fundep repair source-only under #27457. The verified 4cfb9584 pattern mechanism packet remains valid for its earlier source; it does not certify the newer case stack. Char.toEnum is now an explicit second case prerequisite, unit 520. NonEmpty stage B and infix Show output remain independent open dependencies.

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
