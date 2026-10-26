<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# foldr1, and the report folds' own types Chirho

Row 558 (gpt_chirho #28974) adds the report's `foldr1` beside row 548's `scanl1`, `foldl1` and
`until` in `report_folds_chirho.rs`. Its measurement found one more root of the same family.
`foldl1`, `foldr1` and `until` had no builtin type scheme. Over Double, their step and `show`
therefore defaulted to Int: a silent wrong answer natively, and in the interpreter a failure or
`192` for `until`. Row 548 had shipped that for `foldl1` and `until`. `scanl1`, which has its
scheme, was exact everywhere.

## Contract

- `foldr1 f (x:xs) = case xs of [] -> x; _ -> f x (foldr1 f xs)`. An empty list fails with
  GHC's payload `Prelude.foldr1: empty list`.
- The builtin schemes: `foldl1'`, `foldl1` and `foldr1` share `(a -> a -> a) -> [a] -> a` in one
  loop, and `until` is `(a -> Bool) -> (a -> a) -> a -> a`.
- On all three engines, GHC-exact:
  - nonempty and right-associated: `foldr1 (-) [10,3,2]` is 9, not foldl1's 5;
  - singleton;
  - first-class use through `sum . map`, `map`, and `mapM_ (print . foldr1 max)`;
  - Double `foldr1`, `foldl1` and `until`;
  - the empty-list failure.
- Hold: native laziness. `foldr1 (\x _ -> x) [1, undefined]` is exact in the interpreter, but both
  natives fail with `Prelude.undefined`. `const 1 undefined` fails the same way with no foldr1:
  natively a known call forces its argument.

## Checklist

- [x] Measure GHC: nonempty, singleton, first-class, laziness, Double, and the empty failure.
- [x] Add `foldr1` and share the empty-list `error` alternative with `foldl1`.
- [x] Trace the Double failures to the missing schemes (the `(-)` argument was the generic Int
  binding in Core), and add the three schemes.
- [x] Baseline main 90ee1434 against the candidate on 18 probes on all three engines:
  - 30 gains, among them foldl1 and until at Double (before: an integer natively; in the
    interpreter, `SubIntChirho` or `192`);
  - the empty list moves from a missing binding to GHC's payload on all three engines;
  - 0 RED; the native laziness case keeps its `Prelude.undefined`.
- [x] Corpus: 3 files name these functions (2 should_compile, 1 should_fail), with the same
  verdicts on main and on the candidate.
- [x] All-engine controls generated from runghc's output; native_error_helpers 38/38.
- [x] Rebase onto 90ee1434; the builtin-type-environment doc keeps row 564's section and adds
  this row's.
- [x] Reach mutations: 4 of 4 killed by named assertions, every file restored byte-identical.
  - foldr1's scheme removed: `SubIntChirho`.
  - until's scheme removed: `192`.
  - foldr1's step arguments swapped: `-11`.
  - foldr1's empty case given foldl1's message.
- [x] Gate r1 on 46ff0cab: build 0 warnings; core 170, typing 362, seven driver suites 194, and the
  driver library's fold/until/scan/report filter 36.
  - Clippy put 2 `inconsistent_digit_grouping` warnings on the scheme ids `7507_1`-`7507_4`.
  - r2 spells them `75071`-`75074`, the same values: 0 of 616 warnings on changed lines;
    typing 362 and native_error_helpers 37; diff-check and gitleaks clean.
