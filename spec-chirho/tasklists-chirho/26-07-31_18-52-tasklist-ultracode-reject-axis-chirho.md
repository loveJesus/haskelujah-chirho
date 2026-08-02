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
- [x] **Lane 1 assessed → BLOCKED** (not a compiler problem): `should_compile/all.T` supplies `-fdefer-type-errors` on the command line for ≥22 hole tests, absent from the sources, so the mapper's proposed pragma guard never fires. Would cost 8–22 accept files to gain 9. Evidence recorded in the handoff README; reading `all.T` from compiler code would be test-gaming.
- [x] **Lane 3 landed** (f14f8d60 + 2d9126e6): instance-head kind checking — `infer_module_kinds_chirho` had no `InstanceDeclChirho` arm at all. Reject 169 → 172. Six accept-axis false rejections caught by the gate, all traced to our own lowering dropping head forms; fixed by suppressing UNDER-application only, so over-application stays sound.
- [x] **Lane 4 landed** (8df50557 + ef9d8d8d): validity walker now covers class method sigs, positional+GADT ctor types, and constraints inside `ctx =>`; plus an unlifted-newtype guard. Reject 172 → 180 (**~22% → ~23%**). Designed by an 8-agent ultracode workflow (4 designers + 4 skeptics); **all four skeptics refuted their designer**, which is what kept six accept-axis regressions out of the tree.
- [ ] Lane 6 (bounded module search — crash fix, no axis movement) — narrowed design in hand
- [ ] Lane 5 (signature skolemization, +16, medium) and lanes 7+/A remain unstarted
- [ ] Full workspace cargo test + warning scan after lanes
- [x] Two-pass re-measurement of BOTH axes after each landed lane, artifacts superseded, committed (`git add -f`)
- [x] Adversarial review — done per-lane as the workflow's Refute phase, findings addressed at the root rather than guarded around
- [x] Log to progress DB (rows 462, 463), push `main_chirho` → `gh_chirho`, broker updated (#10186 claim, #10273 completion + stale-plaque alert)

## Findings that outlived the lanes

- **Measurement hazard (now in the accept-axis artifact).** The interactive `grep` is shadowed by a ugrep wrapper whose `-q` exit status is *content-dependent* — it reported "no match" on compiler output containing `error[E0200]` (reproduce: `should_compile/tc124.hs`). A gate run through it under-reports failures and inflates the accept axis. All gates re-run with a pure-shell detector (`gate_run_chirho.sh`); committed baselines reproduce exactly under it, so nothing published was wrong — but it would have been, silently.
- **Two parser defects filed**, both blocking reject-axis checks that are otherwise clean:
  - `spec-chirho/bug-record-field-forall-lowering-chirho.md` — record fields split on `=>` before lifting a leading `forall`, manufacturing a quantified constraint the source never wrote.
  - `spec-chirho/bug-data-binder-kind-misassigned-chirho.md` — a data-head *binder's* kind annotation is recorded as the *declaration's* return kind, making `data Foo (_ :: Constraint)` (accepted by GHC) indistinguishable from `T14048a` (rejected by GHC).
- **The map's estimates run high** because they assume reachability. Lane 4 was estimated +18 and delivered +8: instance heads, class/instance contexts and data families are *unreachable through our own lowering*, not merely unimplemented. Treat every remaining lane estimate as an upper bound until reachability is checked.

Bonus findings recorded in the handoff: ~60 of 610 wrongly-accepted are measurement noise (aux modules / GHC-now-accepts) — L.J. decision needed before reclassifying; out-of-repo stack-overflow triple-confirmed; tcfail158 layout mis-nesting bug.
