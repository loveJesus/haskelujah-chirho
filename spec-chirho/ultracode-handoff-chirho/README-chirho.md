<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Ultracode handoff — GHC-compat fix lanes, synthesized 2026-07-31

**What this is.** On 2026-07-31 L.J. directed an ultracode push on compiler development. Five parallel mapper agents produced a complete root-cause map of both GHC-compat axes (the five `map-*.json` files beside this README, ~473k tokens of analysis distilled). The synthesis stage hit the weekly credit limit, so the orchestrating agent (claude_chirho, Fable) did the synthesis inline — this document. **No compiler code was changed**; the tree at handoff is clean at `8e9030b5` plus spec-chirho docs. Any capable model can execute the lanes below without re-deriving anything.

**Baseline (committed artifacts, quote verbatim, both axes always together):**
- should_compile: ~91% (858 of 938) — do not quote a third significant figure
- should_fail: ~20% (157 of 767) — do not quote a third significant figure

Artifacts: `spec-chirho/ghc-should-compile-measurement-chirho.txt`, `spec-chirho/ghc-should-fail-measurement-chirho.txt`. Read both before quoting anything; re-measure per their own headers after landing lanes.

## Measurement truth finding (needs L.J. decision, do NOT auto-apply)

Of the 610 wrongly-accepted should_fail files, **~60 have no `.stderr`** and the scope mapper verified most are not genuine misses: **~31 are auxiliary modules** of multi-module tests (their error belongs to the main file, e.g. `T15850_Lib`, `T20289_A`, `T6018Afail`), and **≥8 carry in-file comments that modern GHC now ACCEPTS them** (`AmbigFDs` "now succeeds", `tcfail071`, `tcfail124`, `T7545`, `FDsFromGivens`, `tcfail165`, `tcfail105`, `T15438`). Reclassifying would change the artifact's denominator — that is a methodology change to be made *in the artifact, documented per its own conventions*, only with L.J.'s sign-off. Detail: `map-reject-scope-holes-kinds-chirho.json`, cluster `no_stderr_corpus_noise_chirho`.

## Ranked fix lanes (execute in order; one builder at a time)

Unlock estimates are conservative and OVERLAP across lanes (same file can sit in two clusters) — the sum overstates the total. Every lane's full brief (evidence, exact guard, emission site) lives in the named JSON cluster; read it before coding.

| # | Lane | Brief in | Est. unlock | Risk |
|---|------|----------|-------------|------|
| ~~1~~ | **BLOCKED — do not attempt as briefed.** Typed-holes-as-errors would be MORE GHC-faithful (holes are errors by default in GHC) but is unlandable under the zero-regression rule, and the blocker is the harness, not the compiler. `should_compile/all.T` runs at least 22 tests with `-fdefer-type-errors` **on the command line, absent from the source** — verified 2026-08-01: `holes`, `holes2`, `holes3`, `hole_constraints`, `abstract_refinement_hole_fits`, `free_monad_hole_fits`, `constraint_hole_fits`, `valid_hole_fits` all PASS today and would all break; zero of the 22 carry the flag in-source. The mapper's proposed guard ("keep the warning when OPTIONS_GHC contains the flag") therefore never fires. Reading `all.T` from compiler code would be test-gaming. Revisit only if the harness is taught to pass per-test flags. | `map-reject-scope-holes-kinds-chirho.json` / `typed_holes_warning_only_chirho` | +9 reject, **−8 to −22 accept** | blocked |
| 2 | **Ground no-instance constraints** — the core solver leak, three stacked hatches: vacuous `all()` in `generalize_with_io_defaulting_chirho` (`infer_chirho.rs:1144-1159`), var-leniency in `entails_depth_chirho` (`class_chirho.rs:365-380`), zero-instance skip (`infer_chirho.rs:6664-6672`). Add `is_certainly_unsolvable_chirho` with the 5-condition guard; hook BOTH generalization and phase-3. Include the Coercible-ground special case (same skip site). | `map-reject-no-instance-chirho.json` / `ground_no_instance_swallowed_chirho` + `map-reject-mismatch-kind-chirho.json` / `coercible_ground_unsolved_chirho` | +20-23 reject | low |
| 3 | **Kind-position checks** in `kind_chirho.rs`: instance-head arity (no `InstanceDeclChirho` arm exists), constraint-used-as-type (class app returns `ConstraintChirho`), unsaturated tycon in `*` position (tuple-element shared-fresh-kind bug ~1295, assoc-type default RHS never checked, local where/let sigs never walked) | `map-reject-mismatch-kind-chirho.json` / `instance_head_kind_arity_chirho`, `constraint_used_as_type_chirho`, `unsaturated_tycon_in_star_position_chirho` | +18 reject | low |
| ~~4~~ | **LANDED 2026-08-02 — `8df50557` + artifacts `ef9d8d8d`. Actual: +8 reject (172→180), not the estimated +18.** Walker now covers class method sigs, positional+GADT ctor types, and the constraints inside any `ctx =>`; plus an unlifted-newtype guard. **Why the estimate ran high — reachability, not effort:** instance heads (`lower_instance_head_types_chirho` drops the `forall` token), class/instance contexts (`parse_simple_constraint_segment_chirho` returns `None` for any segment with `forall`/`=>`), and data families (dropped wholesale at `lower_chirho.rs:862`) are all UNREACHABLE through our own lowering. Record ctor fields were excluded and `T7019` dropped on purpose — see the two filed bug docs. Sub-item "return kind `Constraint`" DEFERRED, refuted, filed. | as before | ~~+18~~ **+8 actual** | landed |
| 5 | **Signature skolemization** — the typing crate has NO rigid/skolem concept (`grep rigid\|skolem` = zero hits). `TyChirho::ForallVarChirho` already has exact skolem unify semantics (`unify_chirho.rs:208-212`); use it to instantiate signature foralls in the Phase-3c subsume check (`infer_chirho.rs:1637-1658`, `:6280`) instead of fresh metavars. Biggest single lane; run gates religiously. | `map-reject-skolem-ambiguity-chirho.json` / `sig_skolem_concretization_25897_chirho` | +16 reject | medium |
| ~~6~~ | **LANDED 2026-08-02 — `b6ae5a89`.** Bounded the walk: symlinks never followed (free cycle guard — `read_dir` already knows the entry type), directory budget 2048, depth 64, file budget 16384, one-shot stderr warning on truncation. `>60s` (unbounded, timed out) → **1.13s**. Both axes byte-identical afterwards; all 8 `T16234/` tests pass. **The brief's premise was partly wrong and is corrected here: the STACK OVERFLOW DOES NOT REPRODUCE** — a symlink cycle self-limits via the kernel's `ELOOP` at ~32 levels. The real defect is unbounded **scope**, and it only reproduces when the checked file's *parent* has a large subtree; a file in a leaf directory is fine, which is why casual spot-probes look clean. Reproduce with a file placed directly in `/private/tmp` (80418 dirs, 4274 `.hs`). | first-hand evidence, this doc | crash fix | landed |
| 7+ | Later lanes: instance-decl obligations (+9), pattern-unify swallow sites (+8), signature ambiguity check (+7), skolem could-not-deduce (+8), insoluble fundeps (+4), existential escape (+4), levity whitelist (+5), type-level not-in-scope (+6), unbound constructors (+2), monad-comprehension `then/by` shapes (+3) | respective JSONs | ~+56 reject | low-med |
| A | Accept axis (80 fails, full bucket map in `map-accept-axis-chirho.json`): promoted/named kinds instead of `* `collapse (+14, medium), stuck-family equality deferral + given-equality rewriting (+12, medium), pattern-binding SCC ordering (+4, low), wired-in names heqT/charSing/Prelude.Experimental (+3, low), small parser items (LHS result sigs, TypeAbstractions binders) | `map-accept-axis-chirho.json` | +20-30 accept | varies |

**Explicitly rejected as unfixable-safely:** import-blind builtin seeding (`join` visible without `import Control.Monad`) — the seeded env is load-bearing for the 858; needs real per-module base export surfaces first. See `missing_import_builtins_always_seeded_chirho` (guard: "Do not attempt a heuristic").

**Bonus defect (file separately if not fixed):** layout bug — one-space-indented decls after a `where` block get mis-nested and later top-level signatures silently vanish from checking (reproduced via `tcfail158`, see mismatch-kind JSON notes).

## Traps found while landing lane 2 (read before writing a reject-axis check)

Each of these produced a real accept-axis regression or a wasted cycle. They are not
hypothetical — every one was caught by the gates, not by review.

1. **Every module has an implicit `import Prelude`.** The driver injects it
   (`haskelujah-driver-chirho/src/lib.rs:2167`). A guard conditioned on "module has no
   imports" is therefore *always false* and silently disables the whole check. Condition on
   *non-Prelude* imports instead.
2. **Any non-Prelude import invalidates a seeded class's instance inventory.** A sibling
   module can declare `instance Show I` that our class env never sees, so "no instance
   registered" stops meaning "no instance exists". Only classes declared in the module
   under inference keep a complete universe (an imported module cannot instance a class it
   cannot see — modulo hs-boot cycles, which is why `ClassDefaultInHsBootA3` is in the
   corpus). Regressions caught: `InstanceWarnings`, `ClassDefaultInHsBootA3`.
3. **Our own inference invents constraints under some extensions.** `RebindableSyntax`
   rebinds `negate`/literals while we still emit `Num` wanteds (`RebindNegate`); the rank-n
   family instantiates a `forall`-typed record field once and reuses it at two types,
   inventing `Num Char` (`T18802`). Both were previously *accepted only because the bogus
   wanted was swallowed*. Any check that stops swallowing must exempt these — see
   `UNFAITHFUL_CONSTRAINT_EXTENSIONS_CHIRHO`. Verify the exemption costs nothing: none of
   lane 2's twelve wins enable those extensions.
4. **The seeded instance table has real holes, and a strict check exposes them as false
   positives.** `T17343` failed on `Show ()` — unit had *no* seeded `Show`/`Eq`/`Ord`
   instance at all (only `Semigroup`/`Monoid`). Fix the seed; do not weaken the check.
5. **Some tests pass GHC flags in `all.T`, not in the source.** `DoExpansion1` is run with
   `-fdefer-type-errors` on the command line, so GHC emits a *warning* where we emit an
   error — with byte-identical content and location (`Num String` at 7:19). Reading `all.T`
   from compiler code would be test-gaming; the honest position is that our diagnosis is
   correct given the source alone. Note it, do not chase it.
6. **Confirm a "new" false positive is actually yours.** A probe that looks like your bug
   may predate your change. Build a binary with your commit reverted and re-run the probe
   before you weaken anything — that check saved a genuine win here, since the
   `Num ()`-from-`() =>` pattern-synonym defect turned out to be pre-existing
   (`spec-chirho/bug-patsyn-empty-context-num-chirho.md`).

## Traps found while landing lane 4 (read before writing ANY corpus gate)

7. **`grep` is not `grep` here — and it silently inflates the accept axis.** In these sessions
   the interactive `grep` is shadowed by a ugrep wrapper function whose `-q` exit status is
   **content-dependent**: it returns "no match" on compiler output that demonstrably contains
   `error[E0200]`. Reproduce: run the binary on `should_compile/tc124.hs`, pipe to
   `grep -q "error\[E"` (says no match) versus `command grep -q` (matches). A gate run through
   the wrapper UNDER-REPORTS failures, i.e. reports a *better* compatibility number than the
   compiler earns. **Use pure-shell matching** — `case "$out" in *"error[E"*|*panic*)` — as in
   the scratch runner `gate_run_chirho.sh`. This was caught mid-lane; committed baselines
   reproduce exactly under the correct detector, so nothing published was wrong.
8. **Estimates in the lane table assume reachability. Check it first.** Lane 4 was estimated
   +18 and delivered +8, entirely because three of its named sites cannot be reached through
   our own lowering. Before costing a lane, dump the AST our parser actually produces for one
   target file; do not assume the source shape survives to the checker.
9. **A designer agent's "zero accept-axis risk" is not evidence.** In lane 4 the designer
   scanned all 938 files and reported zero risk; the skeptic then found six currently-passing
   files the guard would reject, because the trigger was manufactured by our lowering *after*
   the scan looked at the source. Always pair a design pass with an adversarial pass that
   re-derives the trigger from the CODE, and probe with the binary.

## Hard gates for every lane (non-negotiable)

1. Rebuild before any corpus run (stale-binary trap): `nice -n 5 cargo build -j2 --bin haskelujah` — zero warnings, fix never suppress.
2. **Accept-regression subset gate**: full 938-file should_compile pass (use `find` — 8 files live in `T16234/`, a glob misses them; `/opt/homebrew/bin/timeout 15` per file). PASS must be ≥ 858 AND the failing set must be a subset of the 80 listed in the should_compile artifact. Any new failure = guard too eager: fix or revert. Never land a regression.
3. **Reject count**: full 767-file should_fail pass; a file counts rejected iff output contains `error[E` or `panic` (the artifact's counting rule). Must be ≥ previous lane's count.
4. Rejections come from real analysis with proper `error[EXXXX]` diagnostics. Panics-as-rejection is forbidden. Consulting corpus filenames/paths/content from compiler code is forbidden.
5. `cargo test -p` for every touched crate, green.
6. Commit per lane, only named files (`git add <path>`, never `-A`), message `compat: <what>` (or `fix:` for lane 6), trailer `Co-Authored-By:` line per session convention. spec-chirho files need `git add -f`. Do not touch `spec-chirho/progress-chirho.sqlite` (single writer: the orchestrating agent).
7. After the last lane: full `cargo test --workspace`, then two-pass re-measurement of BOTH axes and artifact supersede per the artifacts' own headers (round DOWN, QUOTE-AS format, CHANGE SINCE with commit attribution), committed `docs: supersede artifacts — ...`.

## Error-emission conventions (from the mappers, verified)

`DiagnosticChirho::error_with_code_chirho(ErrorCodeChirho::error_chirho(CODE), msg, span)` — E0200 TYPE_MISMATCH / E0202 UNBOUND_VAR / E0204 UNSATISFIED_CONSTRAINT (`infer_chirho.rs:30-35`), E0206 ILLEGAL_TYPE_FORM (`validity_chirho.rs:27`), E0300 KIND_MISMATCH (`kind_chirho.rs:616`). Emission pattern at `infer_chirho.rs:6673-6679`; validity notes support "perhaps you intended to use X" suggestions (`validity_chirho.rs:48-66`). The driver honors kind errors (`haskelujah-driver-chirho/src/lib.rs:1896-1898`), so kind fixes surface immediately.
