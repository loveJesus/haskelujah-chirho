<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Structured Eq and the Eq-polymorphic list consumers Chirho

## Origin and scope

Row 603, assigned in #29744.
- **Found:** I measured it and reported it as RED in #29742 while probing tuple instances.
- **The invariant:** an Eq-evidenced operation at a concrete key K calls a body-backed `$prim_Eq_==_K`, whether the consumer is direct `==`, generated list equality, or `elem`/`notElem`/`lookup`/`nub`/`group`/`isPrefixOf`/`isSuffixOf`.
- **Rulings:**
  - #29747 takes both mechanisms in one row.
  - #29759 adds typing T1-T3 inside the row.
  - #29763 keeps `/=` out, as row 606, the Eq method-set repair.
- **Branch:** `structured-eq-chirho`, from main `02f6c268`, replayed onto `979f0a2a`, then `cbcf7372` (row 605) and `0aba3695` (rows 598 and 607).
- **Row 604** (tuple instances, arities 4-15) follows only after 603 lands.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/tuple-instances-chirho/probes-chirho`, in `eq-chirho`, `elem-chirho`, `edges-chirho`, `holds-chirho` and `ne-chirho`.
- **Main `02f6c268` and `0aba3695`, silently wrong on LLVM and Cranelift:**
  - computed Strings: `elem ("a" ++ "b") ["x","ab"]` gives `False`, and `lookup (show 12)` gives `Nothing`;
  - `nub` keeps duplicates, and `notElem` gives `True`;
  - `["a" ++ "b"] == ["ab"]` gives `False`, `group` splits equal Strings, and `isPrefixOf`/`isSuffixOf` give `False`;
  - Double: `elem (-0.0) [0.0]` gives `False`, NaN gives `True`, and `lookup 0.0` gives `Nothing`;
  - pairs, `Maybe` and lists under `elem`, `lookup`, `nub` and `notElem`.
  - The interpreter failed on `EqIntChirho`, except for Strings.
- **Main, loud:** tuple and `Maybe` `==` fail with `missing binding '=='`.
- **The trace:**
  - The generated `elem`, `lookup`, `nub`, `group` and `isPrefixOf` compared with Int `==#`, and `$eqList_T` did too for every `T` but Double.
  - Eq occurrence and reference keys were head-only for `Maybe` and tuples; only Show and Ord keyed whole.
  - The schemes of `elem`, `notElem`, `lookup`, `isPrefixOf` and `isSuffixOf` lacked `Eq a`, so they recorded no evidence. `nub` and `group` had it.
- **`/=` (#29762):**
  - Infix `/=` lowers to `not (x == y)` with an evidence-free `==`; it is exact at String and Double, and named-failing at structured keys.
  - The `(/= x)` section lowers to `/=#`. That is a silent wrong natively, attributed to row 606.

## Placement

- **Typing:**
  - `infer_chirho.rs`: Eq is added to the full-key classes. Row 598's hunk in the same file is separate.
  - `infer_chirho/evidence_chirho.rs`: its documentation only. The reference-channel full key, T2, was dropped as inert: reach M2 survived, because the route reads occurrence evidence.
  - The builtin schemes (`effects_chirho/part_08_chirho.rs`, `classes_chirho/part_09_chirho.rs`) gain GHC's `Eq a`.
- **Dictionary pass:**
  - The new `instance_chirho/structured_equalities_chirho.rs` holds the rows, the existing-row lookup and the five compiler-owned By-helpers. `instance_chirho/` now has 11 entries.
  - `conditional_chirho.rs`: `$eqList_T` takes a boxed element's row.
  - `rewrite_chirho/evidenced_aggregates_chirho.rs`: the consumer hook, beside the extrema hook. The Int-shaped key list is shared.
  - `rewrite_chirho.rs`: the generation call.
  - `structured_orders_chirho.rs`: its Core builders are widened to `pub(super)`, and `ordering_chirho` is renamed `nullary_con_chirho`.
- **Controls:**
  - `tests/dictionary_evidence_chirho/ordering_chirho/structured_equalities_chirho.rs`, beside its Ord counterpart. `dictionary_evidence_chirho/` stays at 14 entries.
  - `instance_chirho/structured_equalities_tests_chirho.rs`.
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`, after row 572's bullet.

## Checklist

- [x] Measure, trace and report the RED (#29742); assigned (#29744); checkpoints #29746, #29758 and #29762; rulings #29747, #29759 and #29763.
- [x] Typing T1 and T3 (T2 dropped, as above), the rows, the route and `$eqList_T`. The evidence trace probe was env-gated and removed by script.
- [x] Controls measured on GHC as written. All 13 movers match GHC on all three engines. The holds (Int, Char, Bool; infix `/=` at String and Double) and the two kept named failures pass.
  - The interpreter's String cells were already right; their programs are among the movers because their native cells move.
  - The Word and Int8 holds were set aside: main's interpreter fails on their literals (`missing method Num.fromInteger`) before equality.
- [x] **Broad moved-shape scan** (`scan603-chirho.log`): 3143/3143 passed in 564 s. It covered every lib and 20 driver suites, leaving out only the 16 `proptest_chirho::eval` properties and `ghc_bulk`. No mover.
- [x] **Replayed onto main `0aba3695`,** over row 598, whose `infer_chirho.rs` hunk at line 2716 is separate from T1 at line 6735.
  - The controls pass on the union.
  - The 597/598 seam probes (`T_*`) fail identically on main and the candidate. They are the R2 qualified-builtin family.
- [x] **Main `0aba3695`** was GREEN against the contract copy first (73 movers, 9 holds). Every mover failed there as measured (`baseline-0aba3695-chirho.log`).
- [x] **Reach** (`tools-chirho/reach_603_chirho.zsh`, log `reach603-r1-chirho.log`):
  - M1 (T1), M3 (`elem` without `Eq`), M4 (the route), M5 (`$eqList_T` on `==#`), M6 (no composed rows) and M7 (a stub counts as a row) are killed.
  - M2 (T2) survived, so T2 was dropped.
  - Every restore was byte-identical, and the restored tree passes.
- [ ] Commit; warm gate; record.
- [ ] Land on main and close progress row 603 (gpt). Then row 604.
