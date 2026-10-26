<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Integer literals divided at Double Chirho

Row 543 was claimed provisionally in #28738, under AICEO's #28722. It was developed from `1b804afa` and rebased cleanly onto `66832ade8acde25f343a6193c60b1af79a6c8b4e`, where rows 539, 541 and 542 are closed, before it was posted.

It owns one dictionary-pass rule: the type key of a literal's `fromInteger` application. Integer literals desugar to `fromInteger n`. The checker solves each literal's type, and the driver joins that evidence to the literal's occurrence id. Even so, `infer_type_key_chirho` answered `Int` for every such application, by name.

A method that takes its key from its first argument, as `/` does, then looked for a `Fractional Int` row whenever that argument was a literal. It found none and left `/` unbound.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

| program | GHC | interpreter before | native before (LLVM, Cranelift) |
|---|---|---|---|
| D01 `print (0 / 0 :: Double)` | NaN | `missing STG binding /` | `missing native binding /` |
| D02 `print (1 / 2 :: Double)` | 0.5 | the same | the same |
| D04 `print (compare (0 / 0 :: Double) 1)` | GT | the same | the same |
| D05 `x :: Double; x = 1 / 2` | 0.5 | the same | the same |
| D06 `print (7 / 2)` | 3.5 | the same | the same |
| D03 `print ((1 :: Double) / 2)` | 0.5 | 0.5 | 0.5 |
| D07 `print (1.0 / 2.0 :: Double)` | 0.5 | 0.5 | 0.5 |
| D09 `print (1 + 2 :: Double)` | 3.0 | 3.0 | `3` (native Show Double, outside) |
| D08 `print ((1 :: Double) < 2)` | True | `primop LtIntChirho: expected Int#, got (1##, 2##)` | True |

The "before" columns are the interpreter at 1b804afa. The native backends were measured through row 539's CLI, whose dictionary pass is 1b804afa's.

## Placement

- **The rule** is in `dict_chirho/rewrite_chirho/type_keys_chirho.rs`.
  - The `fromInteger` branch of `infer_type_key_chirho` returns the occurrence's Num record from `occurrence_evidence_chirho`.
  - It falls back to `Int` only when there is no record.
- **The controls** are in `tests/dictionary_evidence_chirho/fractional_literals_chirho.rs`. Each runs through the interpreter and both native backends, and each program's GHC 9.14.1 output was measured as written.
  - D02, D05 and D06 print GHC's output on all three engines.
  - D04 prints GT on all three too, together with row 539's native `compareFloat#` and row 542's interpreter NaN ordering.
  - D01 prints `NaN` natively.
  - Int literals still dispatch at Int.
- **The workflow**, `language-features-chirho/dictionary-evidence-chirho.md`, records the rule beside the invariant that records are consulted before heuristics.

## Boundaries

- **D08 is out of scope.** `(1 :: Double) < 2` is routed to Int's `<#` by the desugarer's operator route in `desugar_chirho.rs`, before the dictionary pass. The combined 531+536+517 stack edits that file. It needs its own row.
- **D01's interpreter output is out of scope.** It now divides, but the interpreter's `show` prints NaN as `NaN.0`, where GHC prints `NaN`. That is a residual.
- **D04 composes with rows 539 and 542.** At 1b804afa it reached the interpreter's NaN compare (EQ) and the native refusal. On 66832ade, where both are closed, it prints GHC's GT on all three engines.
- **No change** to type checking, the evidence join, the desugarer, or either native backend.
- **The gate** is warm, in this worktree. Negative cases are ordinary tests. The candidate CLI runs once over row 535's saved contract, as rows 538 to 541 moved it.

## Checklist

- [x] Measure D01-D09 on GHC, the interpreter and both native backends, before any edit.
- [x] Trace the root to `infer_type_key_chirho`'s `fromInteger` branch, and check that the occurrence evidence reaches the dictionary pass.
- [x] Key the literal's application by its own Num record, with `Int` only as the fallback.
- [x] Add the controls, and measure each program on GHC as written.
- [x] Show the controls reach the rule. With only the rule reverted, the D02/D05/D06 control and the D01 control fail and the Int control passes. With the rule restored, all three pass, and the dictionary-evidence suite is 71/71.
- [x] Record the rule in the dictionary-evidence workflow.
- [ ] Move onto current main_chirho, commit, and run the warm gate.
- [ ] Record the measured gate evidence.
