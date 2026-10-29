<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Compatibility breakthrough Chirho

L.J. asked to improve or close GHC compatibility and enlist claude2_chirho. Base: be548a9c. GPT owns row 661 and then 665. Claude2 owns independent row 670. One machine builder, two jobs and at most four test threads; GPT owns the DB. Overall compatibility remains open until evidence supports closure.

- [x] Refresh committed measurements, source state and lane ownership; enlist Claude2 via broker #30749.
- [x] Replay row 661's lint preparation bdc4b637; inference source is unchanged from the prepared base.
- [x] Apply the prepared row 661 stage and independently verify item equality, mutation controls, test paths and corpus inference dumps.
- [x] Run a focused row 661 gate, update moved workflow paths and land the verified split (closed in ff4b5274; docs placement b76fcd03).
- [x] Land row 675 after repairing explicit disabled-extension false accepts; independent paired corpus verification gains tc135 alone, with all other verdicts held.
- [ ] Implement range typing, standard Enum identity and occurrence evidence (row 665), with all-engine GHC controls and holds.
- [ ] Review and integrate Claude2's tested Enum Double change when ready.
- [ ] Complete Enum's eight-slot dispatch (row 671) before landing newly reachable Double polymorphic calls. Row 670 is held for native wrong answers/SIGBUS until that prerequisite and range evidence form a safe stack.
- [ ] Run paired compatibility measurements if frontend semantics change, record exact movers and residuals, and report the verified improvement.

The published measurements predate this resumed work. The semantic gate must use an executable built from the current candidate; old CLI receipts are supporting evidence only.

## Row 661 verification

P1 is bdc4b637, replayed from Claude's 4e5a0809. S is ff133d3e. The shell is 633 lines, implementations occupy category homes, and every new file remains below 1,500 lines (the largest is expressions_chirho.rs at 1,092). Existing children are pure renames. The verifier accounts for 288 items, 65 listed visibility additions and the 111 moved test paths; all six verifier mutations fail for their named reason.

The freshly built canonical inference harness processes all 3,864 tracked Haskell files without panic or timeout. Its complete 1,699,662,965-byte stream is cmp-equal to the retained d6ec2a12 stream. The inference subtree has no intervening changes between that base and be548a9c. This is behavioral neutrality, not a compatibility improvement.

Focused nextest: 1,583 passed, 0 failed, 1 slow, 1,870 outside the filter. The workspace all-target check is warning-free. Formatting, diff-check and the two-commit gitleaks scan pass. The native contract judge is GREEN after its completed 41-program run. No full driver or ghc_bulk suite was run.

The first gate remains RED at its changed-line Clippy step: default Git diff reports the unchanged eight-argument public entry as added. The approved plan explicitly preserves this API and its existing diagnostic. The same checker with Git's histogram diff identifies the unchanged block and reports zero new diagnostics over 14,026 changed lines. The byte verifier independently checks that function. No diagnostic is suppressed in source, and workspace Clippy still has pre-existing findings.

Two contract invocations are invalid evidence: the first used relative paths although the runner changes directories; a premature judge read the second run before completion. The completed absolute-path second run judges GREEN. Both mistakes are retained rather than counted as compiler failures or passing runs.

Receipts: `tmp-chirho/infer-proof-chirho/` in the GPT breakthrough worktree; the prepared item/move/test manifests and old full dumps remain in Claude's `tmp-chirho/infer-split-chirho/`. The frozen pre-semantic CLI is `tmp-chirho/compat-baseline-chirho/haskelujah-baseline-chirho`, SHA-256 546cce8c0a1ea1d1532431bcb4b064cd5cf4ab08d93af03c12a5626523c3f924.
