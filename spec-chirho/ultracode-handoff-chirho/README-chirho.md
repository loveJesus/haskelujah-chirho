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
| 1 | **Typed holes are errors** — `infer_chirho.rs:2925-2933` demote-to-warning becomes `error[E0207]` unless `-fdefer-typed-holes`/`-fdefer-type-errors` pragma | `map-reject-scope-holes-kinds-chirho.json` / `typed_holes_warning_only_chirho` | +9 reject | low |
| 2 | **Ground no-instance constraints** — the core solver leak, three stacked hatches: vacuous `all()` in `generalize_with_io_defaulting_chirho` (`infer_chirho.rs:1144-1159`), var-leniency in `entails_depth_chirho` (`class_chirho.rs:365-380`), zero-instance skip (`infer_chirho.rs:6664-6672`). Add `is_certainly_unsolvable_chirho` with the 5-condition guard; hook BOTH generalization and phase-3. Include the Coercible-ground special case (same skip site). | `map-reject-no-instance-chirho.json` / `ground_no_instance_swallowed_chirho` + `map-reject-mismatch-kind-chirho.json` / `coercible_ground_unsolved_chirho` | +20-23 reject | low |
| 3 | **Kind-position checks** in `kind_chirho.rs`: instance-head arity (no `InstanceDeclChirho` arm exists), constraint-used-as-type (class app returns `ConstraintChirho`), unsaturated tycon in `*` position (tuple-element shared-fresh-kind bug ~1295, assoc-type default RHS never checked, local where/let sigs never walked) | `map-reject-mismatch-kind-chirho.json` / `instance_head_kind_arity_chirho`, `constraint_used_as_type_chirho`, `unsaturated_tycon_in_star_position_chirho` | +18 reject | low |
| 4 | **Validity-walker extensions** in `validity_chirho.rs`: illegal-polytype positions (class method sigs, data/GADT ctor fields, instance heads — walker currently only does TypeSig+TypeAlias, `validity_chirho.rs:106-129`); unlifted newtype without `UnliftedNewtypes`; data/newtype return kind `Constraint` | `map-reject-skolem-ambiguity-chirho.json` / `validity_walker_coverage_91510_chirho` + `map-reject-scope-holes-kinds-chirho.json` / `unlifted_newtype_without_extension_chirho`, `data_decl_constraint_return_kind_chirho` | +18 reject | low |
| 5 | **Signature skolemization** — the typing crate has NO rigid/skolem concept (`grep rigid\|skolem` = zero hits). `TyChirho::ForallVarChirho` already has exact skolem unify semantics (`unify_chirho.rs:208-212`); use it to instantiate signature foralls in the Phase-3c subsume check (`infer_chirho.rs:1637-1658`, `:6280`) instead of fresh metavars. Biggest single lane; run gates religiously. | `map-reject-skolem-ambiguity-chirho.json` / `sig_skolem_concretization_25897_chirho` | +16 reject | medium |
| 6 | **Bounded module search (robustness)** — `haskelujah check` on any file outside the repo stack-overflows (symlink cycle `/tmp`→`/private/tmp`) or hangs minutes (`$HOME` scan). Triple-confirmed independently by three mappers. Depth limit + visited-inode cycle guard in the sibling-module search. No axis movement; drop-in credibility. | first-hand evidence, this doc | crash fix | low |
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
