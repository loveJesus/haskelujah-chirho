<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# GHC typecheck corpus — deep lanes (2026-09-03)

Directive: L.J. — "please try and get our GHC tests to all pass, see if there are deeper better thought out ways to pass the tests".
Baseline (fresh binary at dd6d694a, pure-shell detector, both passes byte-identical to the committed artifacts):
should_compile 858 of 938 accepted; should_fail 180 of 767 rejected.

Root-cause map: `spec-chirho/ultracode-handoff-chirho/` (five cluster JSONs + README). Gates and traps: that README, non-negotiable.

## Structural finding to surface to L.J.
- `crates/haskelujah-typing-chirho/src/infer_chirho.rs` is 27,545 lines and `crates/haskelujah-parser-chirho/src/lower_chirho.rs` is 17,909 lines. Both are far past the 1.5k-line rule. New mechanisms in this push go into focused sibling modules (`skolem_chirho.rs`, …) and hook into `infer_chirho.rs` at a handful of call sites; splitting the two big files is its own lane and needs L.J.'s go-ahead.

## Lane 1 — rigid type variables (skolems) + env-scoped given equalities
The checker has no rigid-variable concept: signature variables are instantiated as flexible metavariables per equation, so `f :: Int -> a; f x y = x + y` is accepted, and GADT refinement works only because that freshening lets each equation solve `a` privately. Design: skolemize the signature once per binding; re-do GADT refinement as *local given equalities* recorded in the innermost env scope and applied during type normalization. One mechanism serves both axes.
- [x] `skolem_chirho.rs`: skolem naming (`name%N`, Display strips the suffix), `skolemize_scheme_parts_chirho`, refinement unification helper
- [x] `TyEnvChirho`: per-scope refinement list, `apply_subst` covers refinement values
- [x] `normalize_ty_chirho` applies live refinements
- [x] `bind_pat_chirho` constructor arms: GADT-style constructors (at any depth, `Just Refl`) refine skolems instead of failing silently
- [x] Signed-binding sites (Phase 3b top-level, local binds, pattern bindings) instantiate the signature with skolems; Phase 3c checks against the same rigid signature
- [x] Diagnostics: rigid-variable note on skolem mismatches; "could not deduce" wording for skolem preds (only when no instance could ever apply)
- [x] Unit tests beside the code (typing crate: 300 green); `tests/rigid_tyvars_chirho.rs` in the driver (14 green)
- [x] Probe set (`scratchpad/probes/`) before/after: 20/20
- [x] Regressions fixed at the root, not guarded: ScopedTypeVariables now links to the enclosing skolem (annotations, type applications, inner signatures), class-method schemes quantify class vars first, dependency edges to signed bindings are cut (RelaxedPolyRec; off under Haskell98), pattern bindings generalize before the signature check, IO defaulting only for argument-less bindings, type-changing record update, functional-dependency improvement on rigid and given positions, substitution reaches every predicate argument, constructor result unified after its argument patterns, associated-family defaults apply per instance, wanteds discharged through instances whose contexts the givens entail
- [x] Rebuild, accept-subset gate (938, find-based), reject count (767): six iterations, each regression traced to its root (see the "Known remaining" list for the three that are not this lane's)
- [x] New mechanisms live in child modules of `infer_chirho.rs` (`infer_chirho/rigid_chirho.rs`, `equalities_chirho.rs`, `records_chirho.rs`) rather than adding to the 28k-line file
- [x] `cargo test -p haskelujah-typing-chirho` (303), driver tests (rigid 14, given-equalities 10, integration 8); zero compiler warnings; clippy clean on the new modules. Pre-existing and untouched: two parser tests that read preprocessed `containers` sources fail at HEAD too, and the parser's nested-parens proptest overflows the default test stack at HEAD too
- [x] Workspace suite partitioned against the untouched `dd6d694a` worktree (2026-09-06): 21 of the 26 red driver tests were this lane's, all from one cause — ground wanteds discharged at every given scope's end never reached a binding's scheme, so the dictionary pass had no evidence for `fromInteger`/`+` and the STG interpreter died on `take 5 "hello"`. Fixed in `givens_discharge_wanted_chirho` (instances alone never discharge at scope end); 18 tests green again, the other 3 were malformed literals (trap 18) now repaired. Pre-existing at baseline, untouched: 5 driver tests, `cranelift_round_trip_recursive_io_return_unit`, the 2 `haskelujah-package` cabal tests, `golden_parse_all`, the 6 LLVM codegen tests, the 2 parser containers tests, the parser proptest stack
- [x] Two-pass measurement after the fix (2026-09-06, fixed binary, idle passes at -P 4, zero timeouts): should_compile 869 of 938 and should_fail 206 of 767, both passes byte-identical and identical to the pre-fix lane measurement — the discharge fix moved neither axis. Artifacts superseded from these passes
- [ ] Commit by named path; DB row; broker SLOT-FREE

## Known remaining accept-axis items (root causes outside this lane)
- `T10856`: a record constructor with a context (`Show a => Mk {..}`) is lowered to a placeholder scheme — parser lane
- `T24845a`: `a + b ~ a + c` in a context is lowered as a class constraint headed by `+` — parser lane
- `T15370`: only passes GHC with `-fdefer-type-errors` supplied by `all.T`; our diagnosis is correct for the source alone (documented exception class)

## Lane 2 — promoted / named kinds (accept axis, ~14 est)
- [ ] `KindChirho` gains a named/promoted constructor; `type_to_kind_chirho` stops collapsing to `*`; PolyKinds-aware defaulting

## Lane 3 — stuck type-family equalities deferred + given-equality rewriting (accept axis, ~12 est)
- [x] Given `~` constraints become scoped rewrite rules (rigid or family side rewritten), including the `+` interaction; insoluble closed givens (`Int ~ Bool`) type their unreachable body
- [x] Stuck family applications on unsolved metavars defer the equality; retried at equation end, given-scope end and module end
- [x] Associated type family defaults apply per instance, not as general equations
- [ ] Remaining cluster members: `T26030` (expected type must reach a `do` block's last `case`), `SplitWD`, `T5490`, `tc251`

## Lane 4 — pattern-binding SCC ordering (accept axis, ~4 est)
## Lane 5 — filed parser bugs (record-field forall lowering; data-binder kind mis-assignment)
## Lane 6 — reject-axis follow-ons that build on lane 1: could-not-deduce on skolems, existential skolems, class default methods, pattern-mismatch reporting

## Closing
- [x] Workspace suites: driver lib 1755 green + the 5 pre-existing reds + the documented hang (skip it); typing 304; driver integration 32; cranelift, package, golden-parse, bulk and curated suites re-run on the fixed lane and compared with the untouched worktree (only pre-existing reds remain; see the lane-1 note). Warning scan: `cargo build` clean; clippy on the typing crate 63 warnings, all pre-existing in the 28k-line file, none in the lane's modules
- [x] Two-pass re-measurement of BOTH axes; both artifacts superseded per their headers (round down, QUOTE-AS both together)
- [ ] Push `main_chirho` → `gh_chirho`; broker SLOT-FREE; site redeploy request if a rounding boundary moves
