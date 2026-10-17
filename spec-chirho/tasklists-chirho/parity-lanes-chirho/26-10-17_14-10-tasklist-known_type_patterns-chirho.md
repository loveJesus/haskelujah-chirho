<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Known-type pattern checking Chirho

## Authority and ownership

GPT's lane was reserved in #27110 and activated in #27154 under L.J.'s parity direction. Work starts from `9986576b` in the existing clean, fully merged `haskelujah-gpt-chirho` worktree, on new branch `gpt-known-type-patterns-chirho`. The former `gpt-kind-scope-chirho` ref remains at its old commit, and the unmerged kind/data-family worktree is preserved. No source edits in canonical main, no Cargo until a named builder handoff, and no remote push. Canonical progress row 504 belongs to GPT; the worktree's DB is not written.

## Brick 1: mechanism and placement

At `9986576b`, `bind_pat_against_chirho` in `infer_chirho/guards_chirho.rs:227` binds a pattern against a fresh variable, then unifies the complete pattern type with the scrutinee. If any nested constructor refines, `refine_scrutinee_by_pattern_chirho` opens all skolems and promotes the complete pattern unifier into local givens. A sibling `True` therefore contributes `a ~ Bool` even where only `Refl` may provide evidence. Separately, `bind_pat_chirho` checks constructor children before the constructor result; `unify_constructor_result_chirho` ignores a failed ordinary constructor match unless the entire scrutinee is a bare skolem. Thus `f :: Int -> Int; f (Just x) = x` is accepted.

Move the existing pattern-binding function into focused `infer_chirho/pattern_bindings_chirho.rs`, leaving a module declaration and dispatch calls in the oversized inference file. Keep public behavior unchanged in that first source brick. Next, check constructor results against the known scrutinee before children; only that constructor's own result may introduce GADT givens. Check children outside-in under those givens. Do not mark an ordinary enclosing constructor as refining because a descendant is a GADT. Case/guard binding must use the same known-type route instead of whole-pattern fallback. Report genuine constructor/tuple/list shape mismatches; do not treat numeric or overloaded string literals as fixed constructors.

The first supported repair concerns structural/constructor patterns. Literal-pattern class predicates and unsupported imported/family metadata remain explicit boundaries unless the shared mechanism requires and measures their repair. No classifier-sized flip promises and no corpus-name exceptions.

## Checklist

- [x] Read the current source routes, prior GHC/main/candidate counterexamples, worktree workflow and ownership state.
- [x] Reuse the clean merged GPT worktree; preserve old branch refs and unrelated worktrees.
- [x] Choose the focused module and outside-in evidence boundary before writing the mechanism.
- [x] Pure extraction of the existing 377-line binding function into the focused module; visibility and workflow annotation only, with source-identity evidence retained. No build or behavior claim yet.
- [ ] Build a GHC-measured control matrix across equations, cases and guards: generic rigid sibling negatives, specialized equality positives, concrete constructor/tuple/list mismatch negatives, nested constructors/records, wrappers and lexical escape controls.
- [ ] Implement constructor-result-first binding and remove whole-pattern refinement promotion on known-type paths. Preserve constructor contexts, higher-rank fields, unknown metadata boundaries and source positions.
- [ ] Publish source-only checkpoint for peer review before Cargo.
- [ ] Merge current main into the isolated candidate before validation if another unit lands; verify source scope and resolve only owned changes.
- [ ] Take a named builder lease; focused typing/guard tests, negative and positive executions, corpus pair with unchanged accept membership, replay/execution boundaries and one mutation per repaired evidence rule.
- [ ] Independently review and land the measured candidate, close progress row, audit owned commit scope and gitleaks, then release the builder/main/DB leases explicitly.

## Existing evidence

Canonical `tmp-chirho/gpt-parity-review-chirho/known-type-patterns-chirho/` retains the measured baseline. The generic `Equal a b` sibling controls are invalid; the earlier specialized `Equal a Bool` sources with draft `negative` filenames are valid positives and stay classified as such. Do not repeat that earlier fixture-label mistake. The old kind-schemes branch is reference material, not a stack to integrate wholesale.
