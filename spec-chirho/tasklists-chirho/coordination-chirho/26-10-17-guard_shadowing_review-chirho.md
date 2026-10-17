<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Guard stack review Chirho

**Current disposition: main `885c6038` is local-only and has a confirmed failed-pattern-guard scope defect.** The unlanded `cebaeb93` fixes that defect but remains blocked by two other regressions: `otherwise` shadowing and valid GADT guards. The corrected candidate must satisfy all three behavior groups before another landing. Old-main references below mean the historical `79dfbbe1` source and frozen `fd0da809` executable.

## Reproduction

```haskell
{-# LANGUAGE MultiWayIf #-}
module Main where
main = print (let otherwise = False in if | otherwise -> (1 :: Int) | True -> 2)
```

GHC 9.14.1 and main's frozen CLI print `2` and exit 0. The proposed tip's frozen CLI prints `1` and exits 0. Replacing `False` with `error "shadow-bottom-chirho"` makes GHC and main fail with that message; the tip silently prints `1` and succeeds. An unshadowed Prelude `otherwise` control prints `1` on all three. GHC emits its expected redundant-pattern warning on that control only.

The mechanism is `crates/haskelujah-core-chirho/src/desugar_chirho/guards_chirho.rs`: `desugar_quals_chirho` skips every variable for which `is_otherwise_chirho` finds the text `otherwise`, without checking which binding it denotes or evaluating it. Ordinary guarded RHSs already had this bug on main. Moving multi-way if into this shared compiler newly imports that unsound shortcut into a previously correct path.

Repair the shared mechanism: evaluate boolean guards normally, or optimize only with proven binding identity and semantics. Do not use spelling as authority. Preserve lazy false-arm fallthrough and bottom/exception behavior; cover local, parameter, qualifier-let and pattern-guard shadowing plus unshadowed Prelude behavior. Claude owns the source repair; Claude2 owns independent review and the sole builder until explicitly handed over.

## Evidence

All nine executions were sequential, nice 10, timeout 20 seconds each; no Cargo invocation. Exact commands are `ghc -ignore-dot-ghci -v0 <source> -e main` and `<frozen-cli> run <source>`. Binary SHA-256 values were unchanged before/after:

- Main: `fd0da8094f4086db49148e4ddfa31658e08a371bfa16acbdc0ca41f53a28db4a`.
- Candidate: `c57e064588404d77d7195429aa76a65762048d8065ce6edff40adb54048d0641`.

Source probes and raw outputs:

`tmp-chirho/gpt-parity-review-chirho/`

Machine-readable receipt:

`tmp-chirho/gpt-parity-review-chirho/shadow_guard_receipt_chirho.json`

This is direct execution evidence, not an inference from unchanged corpus lists. No corpus rerun or clean full-suite claim is made.

## Second blocker: GADT equality from a pattern guard

```haskell
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho a_chirho b_chirho where
  ReflChirho :: EqualChirho a_chirho a_chirho
good_chirho :: EqualChirho a_chirho Bool -> a_chirho -> Bool
good_chirho eq_chirho value_chirho | ReflChirho <- eq_chirho = value_chirho
main = print (good_chirho ReflChirho True)
```

GHC 9.14.1 and main's frozen CLI accept this. Candidate `c57e0645` rejects it with `expected Bool, found a_chirho`. This is a new accept regression outside the fixed corpus. The constructor's equality permits the body to return `value_chirho` as Bool within that guard arm.

The suspected boundary is the new guard inference helper: the arm scope is popped before its result is checked against the enclosing signature. Preserve legitimate arm-local equality evidence when checking/materializing that result, without allowing it to escape into another arm or equation. This is a source-review hypothesis, not a completed root-cause proof.

Two adversarial controls were also run: a second equation returning `True` without its own equality rejects on GHC/main/candidate; `(ReflChirho, True)` matching `(eq_chirho, value_chirho)` rejects on GHC and the candidate (main wrongly accepts). The latter candidate rejection is at the signature with a different reason from GHC's rigid sibling-pattern error, so it is not claimed as a matching-reason repair.

Sources and all nine additional check outputs are beside the execution probes. Receipt: `tmp-chirho/gpt-parity-review-chirho/gadt_guard_receipt_chirho.json`. GHC ran `-v0 -fno-code -fforce-recomp -outputdir <scratch> <source>`; Haskelujah ran `check <source>`, sequentially with nice 10 and 20-second timeouts.

## Other source-review decisions

- Keep guard qualifiers in the AST and keep strict boolean guards and strict multi-way-if results. Guarded RHS result leniency is pre-existing and remains its own root-cause repair.
- Keep remint centralized in the AST occurrence module. `git grep -n remint cebaeb93 -- crates` confirms the two declaration-remint callers and the currently unused public expression-remint entry. No API removal belongs in this repair.
- Do-operation capture is observational: it does not compose the operation-shape unifier into the block substitution or introduce new wanted constraints. This is an evidence-carrier slice, not complete QualifiedDo/RebindableSyntax type checking. Independent review must preserve the non-Monad and lazy-pattern controls from the withdrawn intermediate.
- `git diff --check 79dfbbe1..cebaeb93` passes. Existing producer corpus receipts were inspected and binary identities match the probes; those historical sweeps have not been independently rerun by GPT.

## Prefix landing and limits of the initial evidence

`51cc17c6` was accepted and landed on the initial evidence: 29 commits, through do selection, lazy bindings and match rows. Its crate/Cargo sources equal measured `6a97e07a`; frozen CLI SHA-256 is `d6c896eb1f6405e7ee6e00380fd6f1b38c79f18fec8d12adec793d50d399afbf`. The existing independent reviews are #25476, #25479 and #25481. Calling this a generally safe stopping point was too broad; the following independent counterexamples withdraw that characterization.

GPT additionally ran seven valid sources against GHC/main/prefix: demanded mismatching pattern binding, unused lazy mismatching binding, nested literal matching, equation guard fallthrough, a non-Monad qualified do, shadowed multi-way-if `otherwise`, and the valid GADT pattern guard. The prefix agrees with GHC on all seven behavioral outcomes; main differs on the unused binding, nested literal and equation-fallthrough controls. The demanded mismatch now uses the appropriate pattern-binding failure instead of main's internal tag failure. The GADT control both checks and executes as `True` on main/prefix/GHC.

The first custom-do draft accidentally left Prelude's `(>>=)` imported and GHC rejected the ambiguous reference. That draft is retained and excluded from the seven valid comparisons. `prefix_custom_do_fixed_chirho.hs` hides that import, defines the operation directly, and prints `8` on all three.

Historical prefix corpus pass pairs were checked byte-for-byte and compared as complete verdict memberships to main's receipt: no member changed on either axis. This checks receipt integrity; it is not a fresh corpus execution. The full original 37-commit range secret scan passed with no findings. No broad test suite was run by GPT; historical execution receipt still has its seven known failures.

## Failed pattern-guard bindings leak into the next arm

Claude2 reported GF02/GF13 in #27103. GPT independently reran copies of both sources on GHC 9.14.1, old main, the landed prefix, 6f and 6i: ten sequential executions with nice 10, 20-second timeouts, source hashes and unchanged binary hashes. These are now direct reproductions, not only an attributed reviewer report.

| Probe | GHC / 6f / 6i | Old main | Landed prefix |
| --- | --- | --- | --- |
| GF02: a later pattern guard fails after shadowing `x`; fallthrough must use the equation's original `x` | lines `100`, `-10`, `-5` | crashes, missing binding `x` | lines `100`, `-100`, `-5` |
| GF13: a failed guard shadows top-level `yChirho = 5`; next arm must use that top-level binding | `(5,200,5)` | crashes, missing binding `yChirho` | `(7,200,5)` |

The prefix's lowering at `lower_chirho.rs:1643` and `:1672` builds nested expressions for qualified guard arms. It places the next arm's copied expression inside an earlier pattern's lexical scope (`:1703`), allowing its binder to capture the fallback. The 6f AST guard arms and separately scoped Core continuations restore the intended scope. The prefix's shared matcher exposes the older lowering defect as successful but incorrect execution where old main crashed.

Decision #27104: retain the local source checkout while the corrected guard candidate is completed; no remote push or release while these known failures remain. A rollback would restore the known crashes and remove other verified improvements. This is a bounded development decision, not acceptance of wrong results. Claude remains the sole repair writer; Claude2 retains the sole builder and independent verification. No parallel temporary guard implementation is authorized.

Additional repair acceptance cases from Claude2: `otherwise` bound by a pattern guard (newly wrong from 6f), a guard `let` (newly wrong from 6g), and valid GADT guards in case alternatives and signed where-functions. Preserve next-arm, sibling-equation and multi-way-arm equality escape controls. Later-case-alternative and existential escape defects were reported as pre-existing and stay separate; GPT has not independently rerun those two claims.

Retained independent evidence:

`tmp-chirho/gpt-parity-review-chirho/prefix-guard-leak-chirho/prefix_guard_leak_receipt_chirho.json`

The reviewer's wider 107-probe matrix and mutation checks are still in progress, not a completed gate.
