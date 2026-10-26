<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# The type-keyed rebuild takes the Double primops Chirho

Row 553 was split out of the #28942 negated-list residual and ruled in #28950. Its first proposed mechanism, a missing literal record, was withdrawn after my trace (#28958); it was re-ruled in #28959. It starts from main_chirho `b2d67eb3`, where rows 551 and 552 are closed.

Row 547 retargets the desugarer's blind Int primops (`<#`, `<=#`, `>#`, `>=#`, `negate#`) to their float counterparts in the method-reference walker. `rewrite_expr_with_type_key_chirho` (`method_args_chirho.rs`) also rebuilds primops whenever it rewrites an argument at a type key. It kept their original names.

List elements under `map (+ 0)` take that path at the exact key Double:

- the literal's `fromInteger` resolved at Num Double;
- the enclosing `negate#` stayed Int around a Double value.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

| program | GHC | at b2d67eb3, interpreter / LLVM / Cranelift | now |
|---|---|---|---|
| N1 `print (map (+ 0) ([-3 / 2, 1] :: [Double]))` | `[-1.5,1.0]` | `NegIntChirho` / `[-0.75,1.0]` / `[-0.75,1.0]`, silently WRONG | `AddIntChirho` (row 554's boundary) / `[-1.5,1.0]` / `[-1.5,1.0]` |
| N2 `print ([-3 / 2, 1] :: [Double])` | `[-1.5,1.0]` | exact everywhere | exact everywhere |
| N3 `print (head ([-3 / 2, 1] :: [Double]))` | -1.5 | exact everywhere | exact everywhere |
| N4 `print (-3 / 2 :: Double)` | -1.5 | exact everywhere | exact everywhere |

The "at b2d67eb3" cells were measured by the reach mutation on this tree, with only the hook reverted.

## Placement

- **The hook.** In `rewrite_expr_with_type_key_chirho`'s PrimOp arm, a rebuilt primop takes row 547's float counterpart only when that traversal's own key is exactly "Double". That is the key it has just resolved the operands at.
- **The table.** It is row 547's one table, `double_counterpart_chirho`, now `pub(super)` in `double_primops_chirho.rs`.
- **The controls** are in `tests/dictionary_evidence_chirho/ordering_chirho/double_primops_chirho.rs`. Each program was measured on GHC as written.
  - N1 is exact on LLVM and Cranelift. In the interpreter it is pinned at `AddIntChirho` and not at `NegIntChirho`.
  - N2-N4 are exact on all three engines.

## Boundaries

- **No inference from `map`, a list, or the surrounding syntax.** The key is the traversal's own.
- **No `+#`, `-#` or `*#`, no sections, no precedence.** Those are rows 554 and 555.
- **N1 is not claimed exact on all engines before row 554.** The interpreter still stops at the section `(+ 0)`.

## Checklist

- [x] Trace the mechanism and confirm it with a throwaway, reverted experiment (#28958).
- [x] Share row 547's table and hook the type-keyed rebuild at the exact key Double.
- [x] Add the controls, measured on GHC as written. The dictionary-evidence suite is 83/83.
- [x] Reach mutation, with only the hook reverted:
  - N1 returns to `NegIntChirho` in the interpreter and to `[-0.75,1.0]` on both natives;
  - the mechanism control fails and the holds control passes;
  - the restored hook is byte-identical, core recompiled, and N1 is `[-1.5,1.0]` on LLVM again.
- [x] Record the hook in the dictionary-evidence workflow.
- [x] Commit, run the warm gate, and record it.
  - **r1 was GREEN on `b2d67eb3` in 173.39 s:** 685/685, no Clippy diagnostic on the 67 changed lines, and the judge at 126/126 with the 26 landed moves.
  - **r2 on `75988583` was RED at the judge on B06HandShowsPrecTopChirho**, while all 686 tests passed.
    - Row 556, landed in between, had moved it. The interpreter gained GHC's `TChirho 5`, and the natives' named line became R2's boxed-Show line.
    - B06 moves the same way with only this row's hook reverted, after a verified core recompile, so the move is row 556's. gpt recorded it on row 556 in #28986 (`cebbbcfb`).
  - **r3 on `5b2ceca6` was GREEN in 219.45 s**, with the contract copy recording row 556's B06 move:
    - 686/686 focused tests;
    - no Clippy diagnostic on the 67 changed lines;
    - the judge at 126/126, with 73 movers and 9 holds over the 29 landed changes.
  - **The final rebase onto `cebbbcfb`** keeps the crates tree `d631cab1`, the one r3 gated.
- **Measured, not claimed:** claude2's sweep case `filter (> 0.5) ([0.25, 1.5] :: [Double])` was `GtInt` in the interpreter on main, and is exact on all three engines here. It is an ordering section rebuilt at the key Double, so it is the same mechanism (#28986).
