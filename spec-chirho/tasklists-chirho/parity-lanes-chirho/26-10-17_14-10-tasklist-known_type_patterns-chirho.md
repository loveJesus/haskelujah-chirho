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
- [x] Build a GHC-measured control matrix across equations, cases and guards: generic rigid sibling negatives, specialized equality positives, concrete constructor/tuple/list mismatch negatives, nested constructors/records, wrappers and lexical escape controls.
- [x] Implement constructor-result-first binding and remove whole-pattern refinement promotion on known-type paths. Preserve constructor contexts, higher-rank fields, unknown metadata boundaries and source positions.
- [x] Publish source-only checkpoint for peer review before Cargo.
- [ ] Merge current main into the isolated candidate before validation if another unit lands; verify source scope and resolve only owned changes.
- [ ] Take a named builder lease; focused typing/guard tests, negative and positive executions, corpus pair with unchanged accept membership, replay/execution boundaries and one mutation per repaired evidence rule.
- [ ] Independently review and land the measured candidate, close progress row, audit owned commit scope and gitleaks, then release the builder/main/DB leases explicitly.

## Existing evidence

Canonical `tmp-chirho/gpt-parity-review-chirho/known-type-patterns-chirho/` retains the measured baseline. The generic `Equal a b` sibling controls are invalid; the earlier specialized `Equal a Bool` sources with draft `negative` filenames are valid positives and stay classified as such. Do not repeat that earlier fixture-label mistake. The old kind-schemes branch is reference material, not a stack to integrate wholesale.

## Brick 2 source decisions after adversarial review

Current lane merge 42bf1a61 contains main 8a2bcd7e; metadata merges changed no compiler source. Independent receipts under the canonical `tmp-chirho/gpt-parity-review-chirho/known-type-patterns-chirho/agy-baseline-audit-chirho/` verify all 15 Agy probe source hashes and 30 GHC/current-main verdicts. Five positives, nine wrong accepts and the already-correct P10 equation negative are retained distinctly. Agy's source account incorrectly described ignored residual/error propagation: at 1cd3feff `infer_chirho/rigid_chirho.rs:204-239` already propagates opened-unification failure and returns the residual substitution. Directed correction request #27200 asks for exact source citations; those existing mechanisms are preserved.

Placement stays in pattern_bindings_chirho.rs. A shared constructor-pattern helper serves prefix and infix constructors, opens only outer constructor foralls, peels argument types, meets the result with the known scrutinee first, then checks fields left-to-right under the resulting substitution and local givens. Higher-rank field foralls remain closed. Record fields similarly see the growing substitution; positional `C {}` still checks C's result even without record-field metadata. Unknown imported placeholder schemes keep their existing fallback.

Only this constructor's own declared result can refine indices. A failed GADT match may refine only when normalized result and scrutinee have the same known outer constructor; it cannot turn a bare skolem into a data type. Concrete shape mismatches are reported, including tuple/list patterns. Cases and guards enter this same known-type binder, removing whole-pattern promotion and its unused recursive-refinement helpers.

Fixed character and non-overloaded string patterns must meet their fixed types, which also prevents the P07 mixed Bool/Char record pattern from escaping. Numeric, fractional and OverloadedStrings pattern obligations remain a named boundary; no unconditional Int/String choice or new literal-evidence path is introduced. A separate constructor-only shared-record-field control distinguishes growing-substitution behavior from character-pattern checking.

Controls live in `tests/rigid_tyvars_chirho/known_type_patterns_chirho.rs`, included from the existing integration test file, with GHC-measured Haskell sources and negative reasons. This avoids adding another integration-test root. The pattern-matching workflow will document the new shared route and explicit boundaries. Rustfmt/source-scope checks can run now; all behavior and corpus gain claims wait for a named builder lease. Claude2 has the current lease, followed by Claude's revised instance-method gate under #27205.

## Frozen source scope and measured baseline

43 exact sources are embedded in six tests under the existing rigid_tyvars integration target. The 38 static controls were each checked on GHC 9.14.1 and frozen ff4f0eb2 (76 retained commands), with every intended polarity confirmed. Five additional runtime programs matched exact GHC stdout on that frozen main (10 retained commands): nested GADT, explicit constructor forall, higher-rank field, existential payload/function and record GADT. This is baseline evidence only. The candidate compiler and its tests remain unbuilt/unrun; rustfmt parsed the owned Rust sources successfully.

The 15 original Agy controls were independently checked before Chirho-renaming: 30 commands, source hashes identical to Agy's receipt and every current-main/GHC verdict reproduced. The existing refine_skolems_by_unification_chirho helper was compared byte-for-byte with 1cd3feff and is unchanged. The only enclosing-file source edits are the known-type binder's removed redundant span parameter/call arguments; actual pattern spans now anchor its shape diagnostics.

Retained evidence in this worktree: `tmp-chirho/known-type-patterns-1017-chirho/controls_manifest_chirho.json`, `controls_baseline_receipt_chirho.json`, `runtime_baseline_receipt_chirho.json`, `source_review_receipt_chirho.json`. The source checkpoint is intended for independent review before a builder lease. No corpus flip count is promised. Unknown/placeholder imported constructors, malformed constructor arity recovery, literal class obligations and full existential-escape checking remain explicit support boundaries.

Proposed meaningful mutations for the later gate: restore whole-pattern promotion (generic sibling cases/guards wrongly accept), allow a GADT to refine a bare rigid scrutinee (P04b wrongly accepts), check fields before the known result (partial/nested GADT controls), ignore structural mismatch errors (equation tuple/list/constructor negatives), omit growing record-field substitution (constructor-only record sibling), and open higher-rank field foralls (positive field/runtime controls). Each needs a measured behavioral difference before it is counted as caught; redundant mutations are reported as such.

Source checkpoint bf80c304 was sent in broker #27210 (three deliveries). Compiler/test sources remain exactly bf80c304; this follow-up records publication only. Peer review, a named builder lease and all candidate gates remain pending.
