<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Ultracode push — GHC compatibility, both axes (2026-07-31)

Directive: L.J. — "Please ultracode further haskelujah development".
Baseline (from committed artifacts at 8e9030b5): should_compile ~91% (858 of 938); should_fail ~20% (157 of 767).
Weak axis: 610 wrongly-accepted should_fail files. Clustered by GHC error code from corpus `.stderr` files:
GHC-39999 no-instance ~109 · GHC-83865 type-mismatch ~82 · GHC-25897 rigid-skolem ~50 · GHC-18872 ~26 · GHC-91028 ~15 · GHC-91510 ~14 · GHC-88464 not-in-scope/holes ~13 · GHC-55233 ~12 · no-stderr 60.

Bonus defect found during scouting: `haskelujah check` stack-overflows on any file under /tmp (symlink cycle) and hangs for minutes under $HOME — unbounded recursive directory walk in module search. In-repo files unaffected. Disqualifying for drop-in GHC use; candidate robustness lane.

## Plan
- [x] Scout: fleet idle (gpt 4d, claude2 parked), builder free, lane claimed on broker (#9668)
- [x] Fresh debug build of `haskelujah` at 8e9030b5; smoke-tested against corpus
- [x] Cluster wrongly-accepted should_fail by GHC error code; extract per-cluster file lists + the 80 should_compile failures
- [x] Workflow Map: 4 reject-cluster mappers + 1 accept-axis mapper (parallel, read-only + prebuilt binary) — all 5 completed (wf_db38b46e-604, ~473k tokens)
- [x] Workflow Synthesize — subagent hit the weekly credit limit (resets 8pm ET); synthesis done INLINE by claude_chirho instead; full map + ranked lanes committed at spec-chirho/ultracode-handoff-chirho/
- [x] **Lane 2 landed by claude_chirho** — certainly-unsolvable ground constraints now rejected (`is_certainly_unsolvable_pred_chirho`), hooked into both the generalization swallow and the phase-3 zero-instance skip. Also fixed a real seed hole (unit had no `Show`/`Eq`/`Ord`/`Read`/`Bounded`/`Enum` instance). 6 accept-axis regressions caught by the gate and each fixed at its root, not guarded around; 9 unit tests. Traps documented in the handoff README.
- [ ] Fix lanes 1, 3, 4, 5, 6 per handoff README (sequential, one builder, subset regression gate)
- [ ] Full workspace cargo test + warning scan after lanes
- [ ] Two-pass re-measurement of BOTH axes, supersede both artifacts, commit (`git add -f`)
- [ ] Adversarial review of full diff (hack/test-gaming scan) — findings addressed
- [x] Log to progress DB, push `main_chirho` → `gh_chirho`, broker slot updated (map recuperated, builder free, no code changed)

Bonus findings recorded in the handoff: ~60 of 610 wrongly-accepted are measurement noise (aux modules / GHC-now-accepts) — L.J. decision needed before reclassifying; out-of-repo stack-overflow triple-confirmed; tcfail158 layout mis-nesting bug.
