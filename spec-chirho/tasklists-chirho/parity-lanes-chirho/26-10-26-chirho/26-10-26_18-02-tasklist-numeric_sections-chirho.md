<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Numeric sections at Double Chirho

Row 554 was split from the #28942 residual in #28950 and stacks on row 553. Its scope was ruled in #28994: measure division first, then keep `/#` out. Its source was accepted in #29012. It starts from row 553's tip.

`build_section_body_chirho` turns `(+ e)`, `(e -)` and `(* e)` into `\x -> +# x e`, `\x -> -# e x` and `\x -> *# x e` before any type is known. At Double:

- **the interpreter refused them:** `AddIntChirho`, `SubIntChirho` and `MulIntChirho`;
- **the native backends were silently wrong:** they added IEEE bits as integers, giving `NaN`, `[0.0,0.0]` and `1.348e308`.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

| program | GHC | before: interpreter / natives | now, all three engines |
|---|---|---|---|
| `map (+ 0.5) ([1.0, 2.0] :: [Double])` | `[1.5,2.5]` | `AddIntChirho` / garbage | exact |
| `map (* 2) ([1.5, -2.5] :: [Double])` | `[3.0,-5.0]` | `MulIntChirho` / `[0.0,0.0]` | exact |
| `map (10 -) ([1.5, 2.5] :: [Double])` | `[8.5,7.5]` | `SubIntChirho` / garbage | exact |
| `(+ 1) (2.5 :: Double)` | 3.5 | `AddIntChirho` / `NaN` | exact |
| `map (+ (1 :: Double)) [1.5, 2.5]` | `[2.5,3.5]` | refused / `1.348e308` | exact |
| `fmap (+ 1) (Just (1.5 :: Double))` | `Just 2.5` | `AddIntChirho` / `Just 1.348e308` | exact |
| `map (+ 0) ([-1.5, 1] :: [Double])` | `[-1.5,1.0]` | `AddIntChirho` / exact by accident | exact |
| N1 `map (+ 0) ([-3 / 2, 1] :: [Double])` | `[-1.5,1.0]` | `AddIntChirho` / exact (row 553) | exact |
| `/` sections: `(/ 2)` and `(2 /)`, mapped and direct | GHC | exact (held) | exact (held) |
| Int sections `(* 3)`, `(10 -)` | `[3,6]`, `[9,8]` | exact (held) | exact (held) |
| `take 4 (iterate (* 2) (1.5 :: Double))` | `[1.5,3.0,6.0,12.0]` | `MulIntChirho` / `[1.5,0.0,0.0,0.0]` | unchanged: the named residual |

- **`/` sections are holds.** `build_section_body_chirho` has no `/` entry, so they take the Fractional evidence path and no `/#` is emitted.
- **`-` has only a left section.** `(- e)` is negation in Haskell, not a section.

## Placement

- **The table.** Row 547/553's one table, `double_counterpart_chirho` in `rewrite_chirho/double_primops_chirho.rs`, gains `+#`, `-#` and `*#`.
- **Both existing rebuild sites read it,** each on its own proof:
  - the method-reference walker, when an operand keys Double (row 547);
  - `rewrite_expr_with_type_key_chirho`, when its own key is exactly Double (row 553).
- **No new inference.** Infix `+`, `-` and `*` are untouched: they reach the Num dictionary, or `+.#` beside a float literal.
- **Controls,** in `tests/dictionary_evidence_chirho/ordering_chirho/double_primops_chirho.rs`, each measured on GHC as written:
  - a 13-line program on all three engines;
  - row 553's N1 control, now an all-engine claim, because the `AddIntChirho` stop it held was this row's;
  - the `iterate` residual, pinned unchanged at `MulIntChirho` in the interpreter.

## Checklist

- [x] Measure division sections in both directions, direct and higher-order. All four are exact everywhere, so `/#` stays out (#29009).
- [x] Extend the one table with `+#`, `-#` and `*#`.
- [x] Add and measure the controls. The dictionary-evidence suite passes.
- [x] Record the extension in the dictionary-evidence workflow.
- [x] Reach mutation, with only the table reverted to row 553's version:
  - the 13-line sections control and the N1 all-engine control fail;
  - the `iterate` residual, the negated-division holds and row 547's ordering control pass;
  - the table was restored byte-identical, with a verified core recompile.
- [ ] Commit, run the focused gate, and record it.
