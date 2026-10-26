<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# The Ord extrema read their key's compare row Chirho

## Origin and scope

Row 565 is the Ord aggregate brick.
- Row 552's commit left `maximum` and `minimum` for it, after row 551 (#28932).
- gpt left it to me in #29065, and I claimed it in #29093.
- gpt assigned and accepted the bounded design in #29096:
  - preserve the Int, Char and Bool routes;
  - route Double and other concrete keys through a body-backed `Ord.compare`;
  - a key without such a row keeps today's path;
  - the empty list stays a named residual.

The branch starts from main_chirho `b4041964`. Main's tip `90ee1434` is crates-identical to it.

## The bug

`max` and `min` are generated Int-only, over `<=#` and `>=#`. `maximum` and `minimum` fold them: `foldl max (head xs) (tail xs)`.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Before** is main `b4041964`; **now** is this branch.
- **Cells** read interpreter / LLVM / Cranelift; a single value applies to all three.
- **Probes** are in `tmp-chirho/ord-extrema-chirho/probes-chirho`, one GHC binary each.

| program | GHC | before | now |
|---|---|---|---|
| `(maximum, minimum) [1.5, -2.5, 0.5 :: Double]` | (1.5,-2.5) | `LeIntChirho` / exact / exact | exact |
| `maximum [0/0, 1]`, `maximum [1, 0/0]`, `minimum [0/0, 1]` | (NaN,1.0,1.0) | `LeIntChirho` / (NaN,NaN,1.0) / same | exact |
| `(maximum, minimum) ["pear","apple","zoo"]` | ("zoo","apple") | `LeIntChirho` / ("zoo","pear") / same | exact |
| `(maximum, minimum) [[1,2],[3],[1,2,3]]` | ([3],[1,2]) | `LeIntChirho` / ([1,2,3],[1,2]) / ([3],[3]) | exact |
| derived: `(maximum [Green, Red, Blue], minimum [Green, Blue])` | (Blue,Green) | `LeIntChirho` / (Green,Blue) / same | exact |
| `(maximum, minimum) [LT, GT, EQ]` | (GT,LT) | `LeIntChirho` / exact / exact | exact |
| `(max "ab" "b", min [2,1] [1,2])` | ("b",[1,2]) | `LeIntChirho` / ("b",[2,1]) / same | exact |
| `(max 1.5 2.5, min 'a' 'b', max Green Blue)` | (2.5,'a',Blue) | `LeIntChirho` / (2.5,'a',Green) / same | exact |
| `zipWith max`, `map minimum`, `foldl max`, `foldr min`, `let p = max "m"` | per program | | exact |
| Int, Char, Bool, `maximum (map negate ...)` | | exact | exact |
| `maximum [Just 3, Nothing, Just 1]` | Just 3 | `LeIntChirho` / exact / Just 1 | unchanged |
| `maximum [(1,'b'), (2,'a'), (1,'c')]` | (2,'a') | `LeIntChirho` / (1,'c') / (1,'b') | unchanged |
| `largestChirho :: Ord a => [a] -> a; largestChirho = maximum`, at String | "c" | | `LeIntChirho` / "a" / "a" |
| `maximum ([] :: [Int])` | `Prelude.maximum: empty list` | `no matching alternative` | unchanged |

## Placement

- **Evidence.** The driver's per-occurrence evidence names gain `maximum` and `minimum` (`evidence_join_chirho.rs`). `max` and `min` were already there, and the checker's schemes carry `Ord a`.
- **Helpers.** Four polymorphic helpers, each taking a compare function, are generated in `prelude_chirho/ordering_extrema_chirho.rs`:
  - `$ord_max_by_chirho cmp x y = case cmp x y of GT -> x; _ -> y`, which is GHC's default `max` through `<=`;
  - `$ord_min_by_chirho`, its mirror;
  - `$ord_maximum_by_chirho` and `$ord_minimum_by_chirho`, which fold the picks as the Int-shaped bindings do.
- **Rewrite.** `evidenced_extremum_chirho`, in `rewrite_chirho/evidenced_aggregates_chirho.rs`, makes an evidenced occurrence at key K into `helper $prim_Ord_compare_K`. It does so only when that row is body-backed, and never for Int, Char or Bool.
- **The applied-head hook.** Row 552's head check in `method_refs_chirho.rs` covered only `sum` and `product`. It now calls `evidenced_applied_head_chirho`, which tries the aggregates and then the extrema. `evidenced_value_chirho` serves first-class occurrences through the same helper, after `print`.
  - Without that hook, the applied `maximum xs` never reached the extrema: the probes did not move. A debug print on the occurrence map showed the evidence did arrive, `(Ord, [Char])`.
- **Controls** are in `tests/dictionary_evidence_chirho/ordering_chirho/extrema_chirho.rs`, each program measured on GHC as written:
  - the mover program, on all three engines;
  - the Int-shaped holds, on all three;
  - four named residuals, in the interpreter.
- **Existing tests updated:**
  - Row 551's residual list drops `max "a" "b"`, which moved.
  - The ordering-literals test asserts T11 `maximum [LT, GT, EQ]` (GT) instead of leaving it an open boundary.
- **Workflows:**
  - `language-features-chirho/dictionary-evidence-chirho.md`, under the Ord lane;
  - `print-show-evidence-chirho.md`, beside row 552.

## Boundaries

- **Maybe and tuple keys** have no compare row; brick 2 covers them.
- **A polymorphic `Ord a =>` wrapper** holds a dictionary, not a key.
- **The empty-list message.**
- **Found while writing the controls:** `print` of a 4-tuple is rejected (`no instance for Show (a, b, c, d)`). The program uses pairs instead.

## Checklist

- [x] Measure E1-E16 on GHC and main's three engines; claim with the design (#29093); gpt assigned row 565 (#29096).
- [x] Generate the helpers, add the evidence names, add the rewrite and its applied-head hook.
- [x] Probes E1-E21 on the candidate: every routed key exact everywhere, holds and residuals unchanged.
- [x] Controls measured on GHC as written; the dictionary-evidence suite is 91/91.
- [x] Reach mutation with only the extremum hook reverted (`.or_else(...evidenced_extremum_chirho...)`):
  - the mover program and T11 fail;
  - the Int-shaped holds, the residuals and row 551's residual list pass;
  - the probes go back to `LeIntChirho`;
  - the restore is byte-identical, core recompiled, and the tree is clean.
- [x] Corpus reach: 3 `ghc-tests-chirho` programs use the extrema (T079, T267, T268), all at Int. They match GHC before and after.
- [x] Commit (`e873a791` on `90ee1434`) and run gate r1.
  - **r1 was RED at the judge, on A01GcdLcmChirho alone.**
  - That is row 564's move. The program is `print (gcd 12 (18 :: Int), lcm 4 (6 :: Int))`, with no extrema. The interpreter and both natives now print GHC's `(6,12)`, after `c4575ebd`, which landed after this contract copy was made.
  - The copy now records A01 as EXACT on all three engines.
  - Main `51fe6e2c` itself, judged against the updated copy, is GREEN at 126/126.
- [x] Rebase onto `6ba58d57` (rows 558 and 520). No path is shared with either.
  - No contract program uses Enum conversions.
  - E22, `(foldr1 max ["b","c","a"], foldl1 min [2.5, 1.5])`, gives ("c",1.5) and is exact everywhere.
- [ ] Run the warm gate on the rebased tip and record it.
- [ ] Land on main and close progress row 565.
