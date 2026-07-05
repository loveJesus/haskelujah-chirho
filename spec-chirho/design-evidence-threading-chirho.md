<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Evidence Threading (dict-pass v2) — design draft for joint review
Author: claude_chirho v1 (2026-07-05, @417800de). Reviewer: gpt_chirho — APPROVED WITH CORRECTIONS (#1930, folded below as v2). Status: ready for P1 implementation (occurrence-id plumbing first).

## Problem
68-test bucket `unresolved-class-method-binder-contexts` (PRD v2.2.89): prelude class methods (`==`, `+`, …) inside lambdas / where / letrec / do-let reach STG unresolved (`missing STG binding`) because the dict pass proves type keys syntactically and those contexts never see a provable key. Formerly masked by silent Int-0 + ambient defaults. Root architecture gap: dispatch decisions are made by RE-INFERRING types syntactically in the dict pass instead of consuming what the type checker already proved.

## Goal / invariant
Every class-method occurrence resolves to exactly one of: (a) concrete `$prim_Class_method_Type` row via known instance; (b) an in-scope dict parameter (evidence lambda); (c) a LOUD residual error naming the unresolved constraint. Never silent, never ambient-default. INV-001 unchanged (IO primops name-preserved).

## Mechanisms
1. **Occurrence-evidence side table (core upgrade; v2 keying per gpt review).** `CoreIdChirho` identifies binders/globals — NOT occurrences (all `+` uses share the method id), and Core `VarChirho`/`AppChirho` carry no span. Therefore: introduce an explicit `MethodOccurrenceIdChirho` assigned during DESUGARING for each class-method occurrence, carried in `DesugarOutputChirho` into the dict pass. Inference-side entries bridge via `(source span, resolved name, ordinal)` and are translated to occurrence ids at desugar; raw spans alone are forbidden (generated/dummy spans are lossy). Table: `MethodOccurrenceIdChirho -> (class, resolved_ty)`, filled only where the solver had a concrete type. Dict pass consumes the table FIRST; strict-key logic is fallback.
2. **Report defaulting at inference time.** Haskell Report 4.3.4 defaulting (pragmatic list `Int, Integer, Double`) implemented IN inference/generalization as a real substitution applied to the type AND its predicates — explicitly NOT another dict-pass fallback. The 68 bucket is literal-driven (`n == 1`, `x + 1`, filter predicates, do-let arith); after landing, RE-RUN the 68 list to measure true coverage rather than assuming.
3. **Local evidence abstraction.** Local bindings (where/let/letrec) and lambda parameters whose inferred types carry class constraints get dict parameters like top-level constrained bindings already do — via the same explicit-ALLOWLIST mechanics as 66537f7a, proven per binding, NEVER seeded from all module-env names (the d09a4735 lesson: broad seeding broke show across 63 probe cases). Recursive locals: evidence bound once at the letrec ring, shared inside (fixes where_mutual_recursion).
4. **Precedence rule.** Table > existing intercepts > preserved-name loud residual. One rule, no double dispatch.

## Phases (each gated: full probe + 68-list rerun + targeted + package smokes transformers/mtl/constraints/deepseq)
- P1: occurrence-id plumbing + side table, behavior-neutral when table empty (desugar assigns `MethodOccurrenceIdChirho`; typing bridges; dict-pass prefers table when present). NOTHING ELSE lands before this keying layer exists (gpt review condition).
- P2: Report defaulting at generalization (substitution over type+preds) → re-measure the 68 list for true coverage.
- P3: local evidence abstraction (allowlisted, per d09a4735 lesson) → recursion cases.
- P4: retire ad-hoc paths ONLY after P2/P3 gates prove coverage — named retirement list: `try_rewrite_show_numeric_default_chirho`, `try_rewrite_modify_ioref_numeric_default_chirho`, `rewrite_numeric_methods_with_type_key_chirho`, lambda-arg numeric marker propagation, ground `dict_vars` seeding in `rewrite_binding_chirho`. Keep loud residual.

## Risks
- CoreId/span identity across lower/desugar: anchor table keys at AST-lowering IDs carried into Core binders (existing spans are lossy — verify before P1).
- Intercept conflicts: covered by the precedence rule; P4 removes overlap.
- Kind/VTA interaction: none expected (table is term-level); gpt's #2 sidecar is kind-level and orthogonal.
- Blast radius: dict-pass is the 70->7 lesson zone — every phase lands behind the independent full-probe gate.

## Acceptance
`unresolved-class-method-binder-contexts` 68 -> 0; probe 70/70; no new loud residuals in a full driver sweep (post-crasher-fix); packages still build; ad-hoc defaulting intercepts deleted (P4).
