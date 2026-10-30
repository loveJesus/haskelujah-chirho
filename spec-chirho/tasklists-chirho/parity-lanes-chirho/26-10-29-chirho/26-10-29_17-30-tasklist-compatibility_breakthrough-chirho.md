<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Compatibility breakthrough Chirho

L.J. asked to improve or close GHC compatibility and enlist claude2_chirho. Base: be548a9c. GPT owns row 661 and then 665. Claude2 owns independent row 670. One machine builder, two jobs and at most four test threads; GPT owns the DB. Overall compatibility remains open until evidence supports closure.

- [x] Refresh committed measurements, source state and lane ownership; enlist Claude2 via broker #30749.
- [x] Replay row 661's lint preparation bdc4b637; inference source is unchanged from the prepared base.
- [x] Apply the prepared row 661 stage and independently verify item equality, mutation controls, test paths and corpus inference dumps.
- [x] Run a focused row 661 gate, update moved workflow paths and land the verified split (closed in ff4b5274; docs placement b76fcd03).
- [x] Land row 675 after repairing explicit disabled-extension false accepts; independent paired corpus verification gains tc135 alone, with all other verdicts held.
- [x] Land rows 663/664 (e196be3 and record 4a6419a): inferred/owned numeric dictionaries and partial Cranelift Show-selector correction, with actual regression controls. Fractional and imported result-only ABI remain explicit boundaries.
- [x] Land range typing, standard Enum identity and occurrence evidence (row 665), with all-engine GHC controls and holds: source 9228f374, integrated with 671 through 1b824e5e.
- [x] Review the Enum Double stack and composition controls. Hold row 670: after 663 repairs Num-only composition, the inferred Fractional+Enum composition still newly prints Infinity natively. Row 677 owns the remaining indexed superclass route; no Double instance is enabled in this landing.
- [x] Land Enum's eight-slot dispatch (row 671) with row 665 for existing instances: source 14ea7341, required lint repairs through 79c74740 and record 1b824e5e. Double rows and feature controls stay on the held branch.
- [x] Run paired compatibility measurements, record exact movers and residuals, and independently verify final-main execution behavior. The reviewed consolidated fleet report was delivered once to AICEO in DM #2557; delivery is not engineering acceptance.

The paired measurements now describe the final ee64da7b source, crate tree 2d144db46f2e8477d7fbe4cf6bece3dd5ce1522d. The independently built frozen CLI is e60d256fc0e66b7621910e186756cc67829d6c0c174a456ebd2150e47925b077. Old CLI receipts remain supporting evidence at their original source only.

## Enum integration boundary

Broker #30797 reports fresh GHC 9.14.1, interpreter, LLVM and true Cranelift measurements on the 663+665 base and the proposed 670/671 stack. Num-only inferred range composition now reaches GHC on all three engines. Inferred `enumFromTo (x / 2) (x / 2 + 1)` at Double newly prints `[Infinity,Infinity,Infinity]` on both native backends; the base rejects it for the absent Enum Double instance. Its signed twin is exact. Standalone inferred division already prints Infinity natively on the base, so the arithmetic root is pre-existing but the new instance exposes a new silent wrong answer.

Decision #30802: keep 670 held and finish 665+671 for existing instances. No runtime approximation or silent-wrong residual is accepted to enable Double. Claude2 keeps the sole builder for the bounded split and focused gate; GPT keeps main and the single-writer ledger. Indexed Fractional superclass evidence (677), imported result-only dictionary inventory (678), Float representation (676), context-instance reachability and broader native Show remain open.

Claude2's tested tip eeea2c21 was replayed in nine commits onto main 5408513e, producing 1b824e5e. Both crate trees are exactly 2d144db46f2e8477d7fbe4cf6bece3dd5ce1522d; the replay changed no crate byte. The first Enum gate is retained as RED: its semantic suites passed, but six changed-line Clippy findings blocked landing. The behavior-neutral eager fallback/let-chain repair passed its warning-free workspace check, zero-new-diagnostic Clippy check, Core 196, focused Enum/range 27 and method-occurrence 7 controls. The earlier full focused suites passed typing 377, naming 156, driver evidence 37, dictionary evidence 169 and typing integration 97 (2 ignored). Unrelated integration tests were not rerun after the lint-only repair. No full driver or ghc_bulk suite ran.

Broker #30810 returned the builder. GPT accepted it in #30811 and is building a fresh final-main CLI for independent probes and paired corpus measurement. Context-instance call-site dictionary reachability, Char enumFrom on the interpreter and bounded-key maxBound overruns remain measured residuals. Row 673 receives partial credit for the qualified same-name control; its full requested matrix is not claimed closed.

## Final measurement and open losses

One fresh baseline pass and two complete candidate passes per axis completed, with identical candidate TSVs, stable executable hashes and unchanged full source manifests. Each pass retains raw stdout/stderr/exit/source hashes. The fresh baseline equals the previous published file sets exactly. No timeout, panic, abnormal exit or unclassified row is credited as an ordinary verdict.

Both current quote lines, verbatim:

```text
# QUOTE-AS: ~94% (884 of 938) — do not quote a third significant figure
# QUOTE-AS: 253 of 767 = 33.0% — ~32% truncated, ~33% nearest; NO single label
```

Fresh GHC 9.14.1 confirms tc135's gained acceptance and tcfail009's gained rejection. It also accepts tc087, which now fails the explicit Enum Double obligation on its fractional range. That is a valid-source checker loss from the held feature, not a matching GHC verdict. The old range escaped endpoint and Enum inference. The accept total is unchanged because one gain and one loss offset; do not present that unchanged total as no movement. Keep the sound obligation and explicit unsupported boundary rather than a corpus-specific bypass or unsafe Double instance. Full GHC compatibility is not closed.

The fresh final-main executable independently matches GHC for five programs on interpreter, LLVM (retained IR) and marker-proven Cranelift, 15/15 execution cells: inferred Double increment, owned mixed Int/Double addition, two Show dictionaries, range ownership and polymorphic Bool/Ordering enumeration. Additional package-import checks pass for transformers and Parsec. The MTL Reader alias smoke is RED: GHC accepts it, while both frozen baseline and final CLI reject it with byte-identical Reader/ReaderT mismatch diagnostics. This pre-existing boundary is attached to row 623; it is not hidden by the successful runtime controls.

Receipts: canonical `tmp-chirho/final-breakthrough-chirho/`, with build/source identity, frozen CLI, `paired-measurement-chirho/`, fresh GHC mover oracles and `smoke-chirho/`. The initial extra smoke script stops at MTL and stays RED; the completion record preserves all 15 passing runtime cells and records all three package checks without changing their expectations.

## Row 661 verification

P1 is bdc4b637, replayed from Claude's 4e5a0809. S is ff133d3e. The shell is 633 lines, implementations occupy category homes, and every new file remains below 1,500 lines (the largest is expressions_chirho.rs at 1,092). Existing children are pure renames. The verifier accounts for 288 items, 65 listed visibility additions and the 111 moved test paths; all six verifier mutations fail for their named reason.

The freshly built canonical inference harness processes all 3,864 tracked Haskell files without panic or timeout. Its complete 1,699,662,965-byte stream is cmp-equal to the retained d6ec2a12 stream. The inference subtree has no intervening changes between that base and be548a9c. This is behavioral neutrality, not a compatibility improvement.

Focused nextest: 1,583 passed, 0 failed, 1 slow, 1,870 outside the filter. The workspace all-target check is warning-free. Formatting, diff-check and the two-commit gitleaks scan pass. The native contract judge is GREEN after its completed 41-program run. No full driver or ghc_bulk suite was run.

The first gate remains RED at its changed-line Clippy step: default Git diff reports the unchanged eight-argument public entry as added. The approved plan explicitly preserves this API and its existing diagnostic. The same checker with Git's histogram diff identifies the unchanged block and reports zero new diagnostics over 14,026 changed lines. The byte verifier independently checks that function. No diagnostic is suppressed in source, and workspace Clippy still has pre-existing findings.

Two contract invocations are invalid evidence: the first used relative paths although the runner changes directories; a premature judge read the second run before completion. The completed absolute-path second run judges GREEN. Both mistakes are retained rather than counted as compiler failures or passing runs.

Receipts: `tmp-chirho/infer-proof-chirho/` in the GPT breakthrough worktree; the prepared item/move/test manifests and old full dumps remain in Claude's `tmp-chirho/infer-split-chirho/`. The frozen pre-semantic CLI is `tmp-chirho/compat-baseline-chirho/haskelujah-baseline-chirho`, SHA-256 546cce8c0a1ea1d1532431bcb4b064cd5cf4ab08d93af03c12a5626523c3f924.
