<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Prefix minus against the fixity table Chirho

## Origin and scope

Row 555 came out of the #28942 negation residual and was split off in #28950. gpt accepted its scope in #28957: "Own prefix-minus precedence against the fixity table, not numeric evidence or backend primops." It was unblocked in #29025 and starts from main_chirho `241a62e8`.

## The bug

The CST's negation node holds only the operand written after `-`. So fixity resolution grouped `-3 ^ 2` as `(negate 3) ^ 2`.

Haskell 2010 (§10.6) resolves prefix minus as `infixl 6`. A negation takes in each following operator that binds tighter.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Before** is the reach mutation: only the resolver's hook is reverted, and the parser recompile is confirmed.
- **Now** is this branch.
- **Cells** read interpreter / LLVM / Cranelift; a single value applies to all three.
- **Probes** are in `tmp-chirho/negative-precedence-chirho/probes-chirho`, one GHC binary each.

| program | GHC | before | now |
|---|---|---|---|
| `-3 ^ 2 :: Int` | -9 | 9 | exact |
| `- 2 ^ 2 :: Int` | -4 | 4 | exact |
| ``-7 `mod` 3 :: Int`` | -1 | 2 | exact |
| ``-7 `div` 2 :: Int`` | -3 | -4 | exact |
| `-2 ^ 2 + 1 :: Int` | -3 | 5 | exact |
| `(-4 :: Int) == - 2 ^ 2` | True | False | exact |
| `(\x -> -x ^ 2) (3 :: Int)` | -9 | 9 | exact |
| ``-3 `max` 2 :: Int`` (no fixity declaration: `infixl 9`) | -3 | 2 | exact |
| `- [10, 20, 30] !! 1 :: Int` | -20 | `no instance for Num [Int]` | exact |
| `-xChirho * 2`, top-level `xChirho :: Double` | -3.0 | `NegIntChirho` / -6.0 / -6.0 | exact |
| Boundaries, regrouped: `-3 * 2 + 1`, ``-7 `rem` 2``, ``-7 `quot` 2`` at Int; `-3 / 2`, `-2.5 * 2` at Double | -5, -1, -3, -1.5, -5.0 | exact | exact |
| Holds, not regrouped: `(-3) ^ 2`, `negate 3 ^ 2`, `abs (-3) ^ 2`, `-3 + 2`, `-3 < 2`, `map (+ (-3)) [1, 2]` | 9, 9, 9, -1, True, `[-2,-1]` | exact | exact |
| `-2 ** 2 :: Double` | -4.0 | 4.0 / refuses `**#` / refuses | `NegIntChirho` / refuses / refuses |
| `-3 ^ 2 :: Double` | -9.0 | `PowIntChirho` / 0.0 / 0.0 | unchanged |
| `-xChirho ^ 2` | -2.25 | `NegIntChirho` / 0.0 / 0.0 | `PowIntChirho` / 0.0 / 0.0 |

The last three rows are numeric evidence, not precedence:

- **`^` at Double** fails with no negation: `3 ^ 2 :: Double` is `PowIntChirho` / 0.0 / 0.0.
- **`**#`** is refused natively with or without a negation.
- **`-2 ** 2`** now negates the `**` result, and the dictionary pass has no key for it. The interpreter therefore stops at `NegIntChirho`; before, it printed a wrong 4.0.
- **A bare negated top-level Double** fails the same way, before and after: `-xChirho` is `NegIntChirho` / -3.0 / -3.0 (GHC: -1.5). Nothing is absorbed there.

## Reach on the GHC corpus

A throwaway scan lowered all 537 `ghc-tests-chirho` programs and looked for a negation directly over an infix node, which only absorption makes. It found 62 prefix negations, and none is followed by a tighter operator, so no corpus program regroups.

The scan was validated on the probes first. It reached exactly the 18 programs this row regroups.

## Placement

- **Rule.** `absorb_prefix_minus_chirho` lives in `lower_chirho/expr_syntax_chirho/prefix_minus_chirho.rs`. `resolve_infix_precedence_chirho` calls it first, before its early returns, so a one-operator chain such as `-3 ^ 2` is covered.
- **Mechanism.** Precedence comes from `operator_fixity_chirho`: declared fixities, then the builtin table, then `infixl 9`. A negation is a bare `NegChirho` operand; parentheses lower to `ParenChirho`.
- **Spans.** The new negation's span covers what it takes in.
- **Grouping.** `lower_chirho/` was at 14 entries. The type-operator pair moved into `type_syntax_chirho/` (by `#[path]`, as its siblings are), so `lower_chirho/` now holds 13 with the new `expr_syntax_chirho/`.
- **Parser tests** (`prefix_minus_tests_chirho.rs`) cover:
  - the absorbed operators;
  - the operators that stay outside;
  - declared `infixl 7` and `infixr 8` against a declared `infixl 6`;
  - an undeclared backtick operator.
- **Driver controls** are in `tests/curated_semantics_chirho.rs`, beside the backtick-fixity regression.
  - There is one mover program and one boundaries-and-holds program, each measured on GHC as written and run on all three engines.
  - An interpreter test pins the `-2 ** 2` residual.
- **Workflow:** `spec-chirho/workflows-chirho/compiler-pipeline-chirho/operator-fixity-chirho.md`.

## Boundaries

- **No numeric evidence.** The Double `**` and `^` rows above stay as measured.
- **No backend primops.**
- **No new diagnostic.** GHC rejects a prefix minus straight after an operator of precedence 6 or tighter (`2 * -3`). Lowering still accepts it, and the negation takes in only what follows it.

## Checklist

- [x] Trace read-only and measure P1-P7 on GHC and all three engines.
- [x] Absorb tighter operators under a bare negation, against the fixity table.
- [x] Parser tests for the rule, the holds and declared fixities: 3/3, plus the moved type-operator tests.
- [x] Probes P8-P29 on GHC and all three engines.
- [x] Reach mutation, with only the hook reverted, in two rounds:
  - **Round 1** measured the baseline cells above. Its boundaries-and-holds program failed on the `-xChirho * 2` line. That line was a mover, so it now sits in the mover program.
  - **Round 2, on the rebased tip:**
    - the absorbing and declared-fixity parser tests and the mover program fail;
    - the parser holds test, the boundaries-and-holds program and the other three curated tests pass.
  - **Each restore** was byte-identical, the parser recompiled, and P1 is -9 again.
- [x] Reach scans: nothing regroups.
  - the 537 corpus programs hold 62 negations;
  - the 41 contract programs hold 4 negations.
- [x] Pin `-2 ** 2 :: Double` as the named residual: the interpreter stops at `NegIntChirho`.
- [x] Commit, run the warm gate, and record it.
  - **r1 was GREEN on `c1f9719e` (base `af9dcac6`) in 202.21 s:**
    - 1094/1094 focused tests, now including the parser library, the golden suites and the GHC-corpus suites;
    - no Clippy diagnostic on the 311 changed lines;
    - the judge at 126/126, with 73 movers and 9 holds over the 29 landed changes, so no contract cell moved.
- [x] Land final tip `e41e1405` on main and close progress row 555 GREEN.
