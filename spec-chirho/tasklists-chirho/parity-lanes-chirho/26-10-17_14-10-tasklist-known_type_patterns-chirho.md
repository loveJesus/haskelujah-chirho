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
- [x] Merge current main c4e192cc into the isolated candidate at 74822ffc, preserving the landed comprehension parser repair; no conflicts or worktree DB writes.
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

## Source review and contextual positive controls

Claude2 #27228 found no source blocker within the requested four areas: result-first constructor binding, same-known-outer-type refinement, direct known-type case/guard entry and preserved higher-rank fields. This is a scoped source verdict; candidate behavior remains unmeasured. Agy's separate source review remains pending.

GPT independently measured Chirho-renamed copies of all 13 review sources on GHC 9.14.1 and frozen current-main dda4b202 (source f36a5975, main c4e192cc), retaining original/copy hashes and every command/output. The 50 commands comprise check pairs for all sources and runtime pairs except the malformed-arity source. Eleven positive sources become two focused tests in `tests/rigid_tyvars_chirho/known_type_contexts_chirho.rs`: equality givens for tuple/constructor patterns, refinement reaching later tuple/character/string arguments and case siblings, signed/unsigned pattern synonyms, family reduction and imported constructors. All eleven check on both baselines, and ten have identical successful GHC/main stdout. The full imported-constructor source is check-only.

Imported infix `:|` and `:+` runtime bindings fail on current main, independently reproduced with the full and reduced imported-constructor sources; neither is a candidate regression or a successful runtime control. KR11's over-applied `Just` is rejected by GHC and accepted by current main. Constructor arity recovery remains the existing explicit boundary, with no new test that pins its wrong acceptance. These residual sources and raw receipts stay in `tmp-chirho/known-type-patterns-1017-chirho/review-contexts-chirho/`.

The existing five runtime controls and ten new runtime positives share one output assertion in the integration-test parent. At 93a295db there were 54 distinct embedded sources across the two child modules; they were measured on their stated frozen baselines, not on the unbuilt candidate. The only additions after bf80c304 are these test controls/helpers and main's already-landed parser source. Rustfmt is parsing/style verification only. Cargo remains withheld until the named return from Claude's revised instance-method gate; no source landing or push.

## Completed Agy source-review correction

Agy #27244 replaces the inaccurate fenced code in Sections 3 and 7 with pinned-source excerpts and retains all nine B2 sources, commands, outputs and hashes. GPT independently checked all eleven fenced blocks in those sections against their pinned files after Markdown indentation normalization, validated the nine durable source hashes and command paths and the frozen-main binary identity, then reran all nine exact sources against GHC 9.14.1 and current frozen main dda4b202 (18 commands). All baseline polarities reproduce.

B2_01 is a valid rank-N-field GADT matched with empty record syntax: GHC accepts, main rejects. Its exact source is now an additional positive check in known_type_contexts_chirho.rs, making 55 distinct embedded sources. Candidate acceptance is an expectation of the already-written shared result check, not a measured fix. B2_02/B2_03 (family applications inside GADT indices) and B2_09 (written constructor equality context) are valid programs rejected by main; B2_04-B2_08 (contradicting written pattern signatures) are invalid programs main accepts. These eight remain explicit pre-existing comparison probes, never tests that require the old wrong verdict. No annotation, constructor-context or family-given repair is claimed here.

Owned receipts and exact copies are under `tmp-chirho/known-type-patterns-1017-chirho/b2-review-chirho/`. Main d994f6d3's metadata was merged at d3758f9c before this test-only addition; typing/Core source remains bf80c304. Claude2's and Agy's core source reviews are now dispositioned, with named residuals. The next required boundary is the named builder return and measured final combined gate, including these additional probes. No Cargo or remote push.

## Named build and measured record-pattern producer blocker

Claude #27264 handed the builder to GPT, accepted in #27265. The pure typing extraction ddb700b8 built without compiler warnings (frozen 5b95ea88), and all 65 retained static sources match its exact pre-extraction main 9986576b/e9c2ab25 (4dceabb3) in exit, stdout and stderr. The known-type branch then merged measured method main 3199bf9c at 40e61a8f; the sole conflict was the adjacent module declarations, resolved by keeping both instance_signatures_chirho and pattern_bindings_chirho.

Combined CLI 4722e91e built without compiler warnings. The nine new focused controls are 8/9: P07 still accepts the incompatible Bool/Char record fields. Checking all 43 initial sources independently confirms this is their sole polarity miss. No corpus gate or source landing is claimed for that candidate.

Temporary tracing, restored byte-for-byte afterward, shows the record checker supplies Bool as the second field's type correctly. The parser lowerer has already replaced its written Char literal with a field pun: rightFChirho = 'x' becomes VarChirho(rightFChirho). The RecordPat arm of lower_pat_chirho handles flat identifiers/constructors but ignores flat literal and wildcard tokens, then flushes a pun. This is a producer defect inside the structural-pattern unit, not failure of growing field substitution. Traced executables are not gate binaries. Receipts are under gate-chirho/record_trace_chirho.txt and record_ast_trace_chirho.txt.

Placement before repair: extract the existing RecordPat lowering body unchanged into parser src/lower_patterns_chirho/record_patterns_chirho.rs, retaining it as a logical child of lower_chirho through a path declaration. This keeps src and lower_chirho below fifteen entries and shrinks the oversized lowering file. Pure move a2182700 built without compiler warnings; all 65 retained static sources match 40e61a8f in exit, stdout and stderr. Its source differs only in indentation/rustfmt trailing call commas, the enclosing span calculation and dispatch/import placement.

The repair belongs at parse_record_pat_fields_chirho: after a named field's equals, call the ordinary pattern parser and require a field separator. It emits the same pattern nodes already handled by ordinary lowering, preserving literals, wildcards, constructor arguments and nested delimiters with the existing literal decoder and origins. Remaining expression tokens are syntax errors. Puns and RecordWildCards retain their token route; qualified field names are recognized by the lowerer. Focused producer tests live beside the extracted module, and the existing driver P07 remains mandatory. Five GHC-measured runtime controls cover a character field, a constructor argument, a wildcard that must not bind the selector name, a compound tuple argument and a non-final numeric field. Claude2 #27278 independently reproduces silent wrong answers on landed main; Agy #27276 independently traces the producer, with symbolic details corrected in #27277. Repeat the combined focused gate before corpus work.

At 5f0365e5, the parser's four producer controls and all 24 rigid-type integration tests pass, including P07 and the five new exact runtime outputs. The CLI builds without compiler warnings (97a35fbb). Clippy identified two collapsible-if warnings in the owned repair/extraction; they are corrected without suppression before the final gate. Existing warnings on unchanged lines remain outside that zero-new-warning claim.

The malformed-expression control proves a CST ErrorNode, not a frontend rejection: record_expression_pattern_neg_chirho is rejected by GHC, but both frozen baseline 4722e91e and candidate 97a35fbb exit zero because the frontend accepts the partial AST despite that error node. Keep this diagnostic-propagation gap explicit and retain the raw receipts; no runtime/check test requires its old acceptance. Complete propagation of parser error nodes is a separate unit. The valid-pattern repair and its runtime outputs do not claim to close it.
