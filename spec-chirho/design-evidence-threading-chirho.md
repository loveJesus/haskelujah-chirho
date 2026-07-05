<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Evidence Threading (dict-pass v2) — design draft for joint review
Author: claude_chirho v1 (2026-07-05, @417800de). Reviewer: gpt_chirho. Status: DRAFT — no implementation until gpt review + L.J. visibility.

## Problem
68-test bucket `unresolved-class-method-binder-contexts` (PRD v2.2.89): prelude class methods (`==`, `+`, …) inside lambdas / where / letrec / do-let reach STG unresolved (`missing STG binding`) because the dict pass proves type keys syntactically and those contexts never see a provable key. Formerly masked by silent Int-0 + ambient defaults. Root architecture gap: dispatch decisions are made by RE-INFERRING types syntactically in the dict pass instead of consuming what the type checker already proved.

## Goal / invariant
Every class-method occurrence resolves to exactly one of: (a) concrete `$prim_Class_method_Type` row via known instance; (b) an in-scope dict parameter (evidence lambda); (c) a LOUD residual error naming the unresolved constraint. Never silent, never ambient-default. INV-001 unchanged (IO primops name-preserved).

## Mechanisms
1. **Occurrence-evidence side table (core upgrade).** During inference, `infer_chirho` records per-occurrence method instantiations: `occurrence (CoreId-to-be / span) -> (class, resolved_ty)` whenever the solver has a concrete type. Dict pass CONSUMES the table first; syntactic strict-key logic remains only as fallback where the table is silent. This is the GHC shape (typechecker emits evidence; desugarer consumes) without full core-elaboration surgery.
2. **Report defaulting at inference time.** Implement Haskell Report 4.3.4 defaulting (default list `Integer`, `Double`; our pragmatic list `Int, Integer, Double` — matches existing behavior) at generalization for ambiguous `Num`-rooted constraint sets. The 68 bucket is dominated by defaultable literal cases (`n == 1`, `x + 1`); proper defaulting gives the solver concrete tys, which fills the side table, which dispatches. Replaces the ad-hoc narrow Int-defaulting patches (show-args, modifyIORef, numeric-under-print) with the real algorithm — those intercepts then retire.
3. **Local evidence abstraction.** Local bindings (where/let/letrec) and lambda parameters whose inferred types carry class constraints get dict parameters like top-level constrained bindings already do (extends `dict_param_bindings` allowlist mechanics to local scopes). Call sites apply evidence where their keys are provable. Recursive locals: evidence bound once at the letrec ring, shared inside (fixes where_mutual_recursion).
4. **Precedence rule.** Table > existing intercepts > preserved-name loud residual. One rule, no double dispatch.

## Phases (each gated: full probe + 68-list rerun + targeted + package smokes transformers/mtl/constraints/deepseq)
- P1: side-table plumbing, behavior-neutral when table empty (typing emits; dict-pass prefers when present).
- P2: Report defaulting at generalization → table covers the literal-driven majority of the 68.
- P3: local evidence abstraction (where/letrec/lambda) → recursion cases.
- P4: retire ad-hoc Int-defaulting intercepts + shrink syntactic key paths where the table covers; keep loud residual.

## Risks
- CoreId/span identity across lower/desugar: anchor table keys at AST-lowering IDs carried into Core binders (existing spans are lossy — verify before P1).
- Intercept conflicts: covered by the precedence rule; P4 removes overlap.
- Kind/VTA interaction: none expected (table is term-level); gpt's #2 sidecar is kind-level and orthogonal.
- Blast radius: dict-pass is the 70->7 lesson zone — every phase lands behind the independent full-probe gate.

## Acceptance
`unresolved-class-method-binder-contexts` 68 -> 0; probe 70/70; no new loud residuals in a full driver sweep (post-crasher-fix); packages still build; ad-hoc defaulting intercepts deleted (P4).
