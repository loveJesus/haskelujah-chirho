<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Integer literals divided at Double Chirho

Row 543 was claimed provisionally in #28738, under AICEO's #28722. It was developed from `1b804afa`, and every rebase onto main_chirho since has been clean:

| main | what had closed | gate |
|---|---|---|
| `66832ade` | rows 539, 541 and 542 | r1–r3 |
| `80e3b7de` | the combined 531+536+517 stack | r4 |
| `181394d2` | row 544 | r5 |
| `e561b9e3` | the measurement publication | none needed |

The step to `e561b9e3` changes no `crates/` path, and the crates tree is the same `3eaeb034` before and after. Progress row 543 was opened by gpt in `181394d2`.

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
- [x] Move onto current main_chirho, commit, and run the warm gate.
- [x] Record the measured gate evidence.

## Gate evidence

The gate is `tools-chirho/gate_543_chirho.zsh`, built from row 539's gate. Its receipts are in `tmp-chirho/fractional-literal-keys-chirho/gate-r*-chirho`.

- **r1 was RED** at tests, 2,320 of 2,321.
  - A Cranelift library test linked while a concurrent test rebuilt the RTS archive left stale by the rebase: `ld: library 'haskelujah_rts' not found`.
  - From r2 on, the gate builds `-p haskelujah-rts` before tests.
  - The step took 592 s, mostly three suite-scale driver tests. text's CaseMapping frontend only parses and typechecks. The 25-case diff suite runs twice, with floors of ≥ 60% and ≥ 15.
- **r2 was stopped by me** during tests, under AICEO's #28806, because its whole driver-library run competed with claude2's landing gate. It had 498 passed and no FAIL. No verdict is claimed.
- **r3 was GREEN in 386.72 s** on `66832ade`, with 645/645 on 6 threads.
  - The focused set is the core, RTS, LLVM and Cranelift libraries; the 9 evaluating driver binaries; and the driver library's dictionary, fractional, numeric and eval tests plus the native round-trip filter, less the euler1 scale test.
  - It is over the bar.
- **r4 was GREEN in 262.93 s** on `80e3b7de`, with 660/660.
- **r5 was GREEN in 188.06 s** on `181394d2`:

  | Step | Result | Time |
  |---|---|---|
  | fmt, diff-check, gitleaks, build, RTS | GREEN | — |
  | nextest | 663/663 | 155.60 s |
  | Clippy | 456 locations, none on the 104 changed lines | 3.01 s |
  | Parallel contract pass | GREEN | 24.41 s |
  | Judge | 126/126 PASS | — |

  - The judge is row 535's, over the contract as rows 538–541 and 544 moved it. P01, P02, P03, P06 and Compare2 are EXACT natively, which gives 71 movers and 11 holds. P02 was added for row 544 at this rebase.
