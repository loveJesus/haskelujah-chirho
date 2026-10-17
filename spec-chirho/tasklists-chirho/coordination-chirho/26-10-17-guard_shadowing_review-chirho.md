<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Guard stack review Chirho

**Current disposition: final source `e9c2ab25` is landed locally on main.** The reviewed repairs resolve the failed-arm scope, otherwise/GADT guard and LLVM helper failures below. The final independent packet is accepted for these units, with known pre-existing parser/probe/execution failures explicitly retained. No remote push or release. Old-main references below mean the historical `79dfbbe1` source and frozen `fd0da809` executable.

## Reproduction

```haskell
{-# LANGUAGE MultiWayIf #-}
module Main where
main = print (let otherwise = False in if | otherwise -> (1 :: Int) | True -> 2)
```

GHC 9.14.1 and main's frozen CLI print `2` and exit 0. The proposed tip's frozen CLI prints `1` and exits 0. Replacing `False` with `error "shadow-bottom-chirho"` makes GHC and main fail with that message; the tip silently prints `1` and succeeds. An unshadowed Prelude `otherwise` control prints `1` on all three. GHC emits its expected redundant-pattern warning on that control only.

The mechanism is `crates/haskelujah-core-chirho/src/desugar_chirho/guards_chirho.rs`: `desugar_quals_chirho` skips every variable for which `is_otherwise_chirho` finds the text `otherwise`, without checking which binding it denotes or evaluating it. Ordinary guarded RHSs already had this bug on main. Moving multi-way if into this shared compiler newly imports that unsound shortcut into a previously correct path.

Repair the shared mechanism: evaluate boolean guards normally, or optimize only with proven binding identity and semantics. Do not use spelling as authority. Preserve lazy false-arm fallthrough and bottom/exception behavior; cover local, parameter, qualifier-let and pattern-guard shadowing plus unshadowed Prelude behavior. Claude owns the source repair; Claude2 owns independent review and the sole builder until explicitly handed over.

## Evidence

All nine executions were sequential, nice 10, timeout 20 seconds each; no Cargo invocation. Exact commands are `ghc -ignore-dot-ghci -v0 <source> -e main` and `<frozen-cli> run <source>`. Binary SHA-256 values were unchanged before/after:

- Main: `fd0da8094f4086db49148e4ddfa31658e08a371bfa16acbdc0ca41f53a28db4a`.
- Candidate: `c57e064588404d77d7195429aa76a65762048d8065ce6edff40adb54048d0641`.

Source probes and raw outputs:

`tmp-chirho/gpt-parity-review-chirho/`

Machine-readable receipt:

`tmp-chirho/gpt-parity-review-chirho/shadow_guard_receipt_chirho.json`

This is direct execution evidence, not an inference from unchanged corpus lists. No corpus rerun or clean full-suite claim is made.

## Second blocker: GADT equality from a pattern guard

```haskell
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho a_chirho b_chirho where
  ReflChirho :: EqualChirho a_chirho a_chirho
good_chirho :: EqualChirho a_chirho Bool -> a_chirho -> Bool
good_chirho eq_chirho value_chirho | ReflChirho <- eq_chirho = value_chirho
main = print (good_chirho ReflChirho True)
```

GHC 9.14.1 and main's frozen CLI accept this. Candidate `c57e0645` rejects it with `expected Bool, found a_chirho`. This is a new accept regression outside the fixed corpus. The constructor's equality permits the body to return `value_chirho` as Bool within that guard arm.

The suspected boundary is the new guard inference helper: the arm scope is popped before its result is checked against the enclosing signature. Preserve legitimate arm-local equality evidence when checking/materializing that result, without allowing it to escape into another arm or equation. This is a source-review hypothesis, not a completed root-cause proof.

Two adversarial controls were also run: a second equation returning `True` without its own equality rejects on GHC/main/candidate; `(ReflChirho, True)` matching `(eq_chirho, value_chirho)` rejects on GHC and the candidate (main wrongly accepts). The latter candidate rejection is at the signature with a different reason from GHC's rigid sibling-pattern error, so it is not claimed as a matching-reason repair.

Sources and all nine additional check outputs are beside the execution probes. Receipt: `tmp-chirho/gpt-parity-review-chirho/gadt_guard_receipt_chirho.json`. GHC ran `-v0 -fno-code -fforce-recomp -outputdir <scratch> <source>`; Haskelujah ran `check <source>`, sequentially with nice 10 and 20-second timeouts.

## Other source-review decisions

- Keep guard qualifiers in the AST and keep strict boolean guards and strict multi-way-if results. Guarded RHS result leniency is pre-existing and remains its own root-cause repair.
- Keep remint centralized in the AST occurrence module. `git grep -n remint cebaeb93 -- crates` confirms the two declaration-remint callers and the currently unused public expression-remint entry. No API removal belongs in this repair.
- Do-operation capture is observational: it does not compose the operation-shape unifier into the block substitution or introduce new wanted constraints. This is an evidence-carrier slice, not complete QualifiedDo/RebindableSyntax type checking. Independent review must preserve the non-Monad and lazy-pattern controls from the withdrawn intermediate.
- `git diff --check 79dfbbe1..cebaeb93` passes. Existing producer corpus receipts were inspected and binary identities match the probes; those historical sweeps have not been independently rerun by GPT.

## Prefix landing and limits of the initial evidence

`51cc17c6` was accepted and landed on the initial evidence: 29 commits, through do selection, lazy bindings and match rows. Its crate/Cargo sources equal measured `6a97e07a`; frozen CLI SHA-256 is `d6c896eb1f6405e7ee6e00380fd6f1b38c79f18fec8d12adec793d50d399afbf`. The existing independent reviews are #25476, #25479 and #25481. Calling this a generally safe stopping point was too broad; the following independent counterexamples withdraw that characterization.

GPT additionally ran seven valid sources against GHC/main/prefix: demanded mismatching pattern binding, unused lazy mismatching binding, nested literal matching, equation guard fallthrough, a non-Monad qualified do, shadowed multi-way-if `otherwise`, and the valid GADT pattern guard. The prefix agrees with GHC on all seven behavioral outcomes; main differs on the unused binding, nested literal and equation-fallthrough controls. The demanded mismatch now uses the appropriate pattern-binding failure instead of main's internal tag failure. The GADT control both checks and executes as `True` on main/prefix/GHC.

The first custom-do draft accidentally left Prelude's `(>>=)` imported and GHC rejected the ambiguous reference. That draft is retained and excluded from the seven valid comparisons. `prefix_custom_do_fixed_chirho.hs` hides that import, defines the operation directly, and prints `8` on all three.

Historical prefix corpus pass pairs were checked byte-for-byte and compared as complete verdict memberships to main's receipt: no member changed on either axis. This checks receipt integrity; it is not a fresh corpus execution. The full original 37-commit range secret scan passed with no findings. No broad test suite was run by GPT; historical execution receipt still has its seven known failures.

## Failed pattern-guard bindings leak into the next arm

Claude2 reported GF02/GF13 in #27103. GPT independently reran copies of both sources on GHC 9.14.1, old main, the landed prefix, 6f and 6i: ten sequential executions with nice 10, 20-second timeouts, source hashes and unchanged binary hashes. These are now direct reproductions, not only an attributed reviewer report.

| Probe | GHC / 6f / 6i | Old main | Landed prefix |
| --- | --- | --- | --- |
| GF02: a later pattern guard fails after shadowing `x`; fallthrough must use the equation's original `x` | lines `100`, `-10`, `-5` | crashes, missing binding `x` | lines `100`, `-100`, `-5` |
| GF13: a failed guard shadows top-level `yChirho = 5`; next arm must use that top-level binding | `(5,200,5)` | crashes, missing binding `yChirho` | `(7,200,5)` |

The prefix's lowering at `lower_chirho.rs:1643` and `:1672` builds nested expressions for qualified guard arms. It places the next arm's copied expression inside an earlier pattern's lexical scope (`:1703`), allowing its binder to capture the fallback. The 6f AST guard arms and separately scoped Core continuations restore the intended scope. The prefix's shared matcher exposes the older lowering defect as successful but incorrect execution where old main crashed.

Decision #27104: retain the local source checkout while the corrected guard candidate is completed; no remote push or release while these known failures remain. A rollback would restore the known crashes and remove other verified improvements. This is a bounded development decision, not acceptance of wrong results. Claude remains the sole repair writer; Claude2 retains the sole builder and independent verification. No parallel temporary guard implementation is authorized.

Additional repair acceptance cases from Claude2: `otherwise` bound by a pattern guard (newly wrong from 6f), a guard `let` (newly wrong from 6g), and valid GADT guards in case alternatives and signed where-functions. Preserve next-arm, sibling-equation and multi-way-arm equality escape controls. Later-case-alternative and existential escape defects were reported as pre-existing and stay separate; GPT has not independently rerun those two claims.

Retained independent evidence:

`tmp-chirho/gpt-parity-review-chirho/prefix-guard-leak-chirho/prefix_guard_leak_receipt_chirho.json`

The reviewer's wider 107-probe matrix and mutation checks are still in progress, not a completed gate.

## Corrected candidate and remaining known-type defect

Claude's repair is `6bc8e755`; `877d2693` merges main's bookkeeping. GPT verified identical crate/Cargo sources between those commits, ancestry from `b890d61b` and a clean diff check. The repair removes the `otherwise` spelling shortcut and compares guard results against their target while the arm's GADT refinements still hold. Claude2's build log records successful compilation with no compiler warnings. This is not a Clippy claim.

GPT ran the frozen repair binary, SHA-256 `15166830215f759064383247bfd76a54514fff7e73ac75dc630aad3dfb55e4ea`, unchanged before/after, against twelve critical controls. All pass: False/bottom/Prelude `otherwise`; a valid GADT result and an escaping-equation rejection; GF02/GF13 outputs; concrete `Int` versus `Just` rejection in case/guard; and legitimate specialized-equality positives. Receipt: `tmp-chirho/gpt-parity-review-chirho/repaired-candidate-chirho/critical_repair_receipt_chirho.json`. Broker decisions #27110/#27111 authorize validation only, not landing.

The generic rigid-sibling counterexample remains a wrong accept on the repair and current main. GPT independently measured both guard and case forms with `EqualChirho a_chirho b_chirho -> a_chirho -> b_chirho`: GHC rejects with GHC-25897; current main and repair accept. Only the unrepaired tip rejects the guard form, through the lost-result-refinement defect; its case form also accepts. `Int -> Int` with an equation pattern `Just value` is wrongly accepted on main, unrepaired tip and repair, while the corresponding case and guard remain correctly rejected. These controls establish the baseline boundary; they do not establish type soundness.

The first specialized probes used `EqualChirho a_chirho Bool`: GHC accepts the sibling `True` legitimately because `ReflChirho` proves `a_chirho ~ Bool`. Their filenames retain the original `negative` draft label, but they are classified and used as positive controls. The later `generic_rigid_sibling_*` probes quantify both sides and are the genuine negatives. All sources, verdicts and hashes are retained under `tmp-chirho/gpt-parity-review-chirho/known-type-patterns-chirho/`.

Disposition: the pre-existing generic rigid-sibling defect does not alone block this bounded guard repair. Any newly accepted invalid program relative to current main requires a new disposition, and the case/guard concrete-type rejection controls must remain effective. GPT reserves the next root-cause unit: checking patterns against a known type, with per-constructor outside-in givens, rigid sibling wanteds and concrete constructor mismatches handled together. The old kind/data-family branch is reference material, not an integration candidate. No parallel edits to Claude's repair are authorized.

## Native backend failure and repair

GPT inspected Claude2's focused-test receipt: 54 passed, one failed, exit 101. `llvm_round_trip_comprehensive_demo_chirho` fails native linking with `invalid redefinition of function 'haskelujah_error'`. The repair build itself compiled successfully; that does not make its native output valid. The wider interpreter matrix, backend comparison, mutation and corpus gates are not declared complete.

Broker #27114 asks Claude2 to compare the same focused case against current-main source and the unrepaired tip and retain the IR. Claude owns a bounded backend repair if the guard change introduces or exposes the failure; restoring spelling-based `otherwise` behavior is not an acceptable workaround.

Read-only source lead, sent to Claude in #27115: LLVM `codegen_chirho.rs:949` creates a fresh inner codegen for a lifted lambda without inheriting `emitted_names_chirho`. Around `:1067`, the child definitions are appended to the parent without merging that registry back. Runtime wrappers are guarded by the registry at `:1355`, and top-level bindings at `:1822`. Parent/child/sibling helper emission can therefore duplicate a symbol. This remains a source hypothesis pending IR/baseline evidence; no source fix or successful native execution is claimed by GPT.

Claude2's measured baseline #27116 confirms the native regression: the exact demo compiles/runs on old main, current-main source and `cebaeb93`, printing `primes: 25` / `sumsq: 385`, but fails to link on `877d2693`. D8 (guarded parent and guarded where-helper without `otherwise`) and D9 (explicit error paths in both) fail on every baseline, so the underlying module-emission bug is pre-existing. The repair exposes it in a program that previously worked. Its IR shows helper definitions in parent and lifted child, matching the source lead. These native runs are Claude2's evidence, not GPT executions.

Decision #27117: this newly exposed demo failure blocks landing. Claude owns a separate bounded backend commit; ordinary `otherwise` evaluation stays. Claude2 must prove the demo, D8/D9 positive outputs, exercised parent/captured-child error paths and sibling/nested helper use by actual compile/link/run. A check-only corpus pair may carry over to a backend-only change only with exact source-diff evidence, both source/binary identities and final focused/native verification. No frontend/Core changes may silently inherit the earlier corpus evidence.

Claude's backend repair `c635d5d0` moves the module's emitted-name registry into a lifted closure's code generator and back after compilation. The registry is transferred with `std::mem::take`, including through nested closures; function-local state stays separate. GPT reviewed the exact diff. Test-only `e9c2ab25` strengthens the error-path assertion to require a signal or nonzero exit and the expected message; writing stderr after exit zero is insufficient. Compiler sources are identical between those last two commits. `877d2693..e9c2ab25` changes only the LLVM backend, driver tests and the tasklist; no frontend, Core or check-path source changes.

Claude2's exact final build, SHA-256 `4dceabb3833a1b95551233a71c3cf6b0b2cdde8b0e1f36de7948dc62a046c0a9`, compiles without compiler warnings. Its filtered controls pass: 55 driver tests including the formerly failing demo, three stricter native-error tests, one Cranelift otherwise test, 14 rigid-typing tests and 11 parser/occurrence tests. D2-D9 compile and run on both native backends with GHC's output. The historical demo D1 also runs with its existing expected output; GHC rejects its ambiguous `enumFromTo`, so D1 is a backend regression contract, not a GHC-conformance positive. These native executions are Claude2's receipts, read and checked by GPT.

GPT independently reran the twelve critical controls on this exact final binary, unchanged before/after; all pass. Receipt: `tmp-chirho/gpt-parity-review-chirho/final-candidate-chirho/critical_final_receipt_chirho.json`. GPT also checked `T20768_fail.hs`: both GHC and the final candidate reject the Bool/Char guard mismatch; diagnostic orientation and anchor differ. No diagnostic-parity claim follows.

## Corpus and reject-analysis evidence

Claude2 ran two complete passes per axis on the frozen `877d2693` binary. GPT checked byte-identical pairs, unique full relative source identities, exit-code consistency and complete verdict memberships. Accept membership is unchanged from main and `cebaeb93`; the sole reject-membership change from main is `T20768_fail.hs`, matching GHC's reason. The source-diff evidence above permits this check-only pair to carry to `e9c2ab25`; Claude2 additionally runs one direct pass per axis on the final binary. A final corpus/replay/execution packet and mutation checks remain pending at this checkpoint. Retained receipt reviews: `tmp-chirho/gpt-parity-review-chirho/repaired-candidate-chirho/`.

Agy's baseline taxonomy is useful intake, but keyword classification by GHC diagnostic is not attribution to our checker's mechanism. Claude's independent audit (#27131/#27133) reproduces 50 standalone GHC acceptances among the historical 532 wrong accepts, and three standalone acceptances among the historical 235 rejections. His retained RC-0 list has 59 unique files, with `T10285a` duplicated in the report's total; eleven listed members actually reject standalone and two standalone acceptances were omitted. GPT checked the retained table identities and member-list hashes, not all GHC executions. These are attributed protocol findings; published denominators and labels stay unchanged pending a separately reviewed protocol decision by L.J.

All 26 RC-3 taxonomy members reject under Claude's standalone GHC oracle, but their shared diagnostic does not prove a shared checker cause or 26 fix gains. `T20768_fail` is the one measured candidate flip already present here. Prospective gains and zero-regression claims remain estimates until a candidate is tested. Claude also reports an independently reduced lost substitution from an `if` condition; this is a separate lead for source tracing. Broker #27135 requests bounded root-cause recommendations rather than classifier-sized implementation promises.

Claude's citation audit (#27136) also finds several named functions absent from every inspected source ref, and a claimed FlexibleInstances check at a line that actually lists unlifted type names. Those sections cannot support implementation leases without corrected traces. The confirmed instance-method path snapshots deferred predicates, checks the body under method givens and then truncates every leftover wanted. The confirmed `IfChirho` path applies its Bool-condition substitution to the environment but omits it from the substitution returned to its caller.

GPT independently read both source paths on canonical main, then ran M01/M02/M06 and K01/K02 on GHC and final binary `4dceabb3`, with stable binary identity. M01/M02 and K01 reproduce wrong accepts; the corresponding top-level method-body control M06 and explicit-Bool control K02 reject on both compilers. Receipt: `tmp-chirho/gpt-parity-review-chirho/agy-report-audit-chirho/next_lane_baseline_receipt_chirho.json`. Broker #27139 queues Claude on instance-method discharge, Claude2 on condition-substitution composition, GPT on known-type pattern checking and Agy on evidence correction/adversarial review. Source activation awaits the guard landing receipt; there is no additional builder lease or projected corpus gain.

Agy's corrected report SHA-256 `721fc4ba46bd4c39885b23ff36e92ac6566c72b64274790f5f5631664f60aad7` supplies per-file command, source-identity and verdict tables (#27149). GPT verified the table row counts, disjoint complete membership, exact RC-0 acceptance set and file-for-file agreement of its GHC exits with Claude's independent table. Claude independently confirms the replacement source sites (#27150), with a remaining requirement to state the correct ref beside each line number. These corrections restore the report's use as baseline evidence; they do not convert taxonomy counts into repair predictions. The original and corrected report snapshots and receipt-index hashes are retained in GPT's audit directory.

Early review of the queued instance-method design (#27143/#27144/#27146) found that instance contexts are not included in the current specialized method givens and that flexible instance/method variables could still bypass final constraint checks. The revised design converts the actual instance context with the same variable map, uses signature skolemization, judges leftovers while givens are live, restores unrelated state and preserves local literal defaulting. Unsupported quantified premises must not bypass existing method inference/type checks. Placement moves the existing method checker into the focused instance-obligations module, reducing the oversized inference dispatcher. This is a design disposition only; no candidate gain is claimed.

## Final integration disposition

Claude2's final packet #27152 completes the declared evidence. GPT checked the clean detached reviewer worktree at `e9c2ab2517415b69affd942741c5b6b8c35f309b` and stable final binary hash `4dceabb3833a1b95551233a71c3cf6b0b2cdde8b0e1f36de7948dc62a046c0a9`. Direct final corpus tables are byte-identical to the earlier two-pass pair; all 289 rejections replay, with zero exit anomalies. The wider matrix remains 93/107 and the retained set 73/87, with the same baseline misses. The execution set remains 28/35 with its seven historical failing predicates. These are bounded measurements, not full correctness certification.

All seven source mutations are caught: last-arm otherwise shortcut (OS09/OS14), dropped GADT signature mismatch (RJ16), lost LLVM registry return (D9), lost Core arm scope (GF02/GF13/MW07), guard-let closure only at comma (LG04), omitted strict-let forcing (LL04), and shared multi-way-if checker scope (RJ03/RJ18). The initial local-let mutation selector refused an ambiguous match; the corrected targeted mutation was built and measured, and the source restored cleanly. GPT independently supplied another strict-do-let canary: GHC and final exit 1 with `strict-let-chirho`, while mutant `2d8d4669` prints `5` and exits 0. Source and raw receipt: `tmp-chirho/gpt-parity-review-chirho/final-candidate-chirho/strict_do_let_mutation_receipt_chirho.json`.

The parser suite is RED in its default environment: `parser_handles_nested_parens_chirho` SIGABRTs on the 2 MiB test-thread stack. Claude2 reproduced it twice on the final source and on each earlier source including `79dfbbe1`; it passes at 8 MiB. GPT inspected the final and old-main failure logs. A larger test stack is evidence about the pre-existing failure, not its repair. The other reported crate unit suites pass (typing 354, core 129, LLVM 38, AST 13); filtered guard/parser controls pass as recorded above. No full driver or Cargo ghc_bulk suite was run by GPT or as this final review gate.

Leader disposition #27153 accepts local integration at this scope. After rereading instructions, PRD, tasklist and Git state, GPT fast-forwarded `b890d61b..e9c2ab25`. The DB and two dirty coordination documents remained byte-identical across the merge. A clean 42-commit gitleaks scan covers `79dfbbe1..e9c2ab25`. Review row 495 and integration rows 496-502 are complete; historical rows through 494 are preserved. Both measurement artifacts now identify the paired and final measurements and preserve the published protocol; they do not silently apply the new standalone-GHC taxonomy. Source code is unchanged from the independently tested candidate. Final report and receipt snapshots live under `tmp-chirho/gpt-parity-review-chirho/final-candidate-chirho/`. No remote push or global green claim accompanies this landing.
