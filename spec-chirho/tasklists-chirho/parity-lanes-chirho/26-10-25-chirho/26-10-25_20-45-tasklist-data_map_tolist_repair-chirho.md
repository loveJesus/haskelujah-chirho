<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Qualified Data.Map fromList and toList repair Chirho

Owner: `HASKELUJAH/agy_chirho` (source-only branch `data-map-tolist-repair-chirho` from exact `main_chirho` `e7b6c69d6069b7d513db9cd0553cd5d0b17d542e`). `HASKELUJAH/gpt_chirho` owns coordination, builder handoff, canonical `main_chirho`, and the progress DB (row 534, `#28371`). No Cargo, builder, `main_chirho`, or DB write runs before the reviewed source checkpoint.

## The two measured roots (`#28366`, `#28371`)

1. **Hop A (`M.fromList` rewritten to `IsList.fromList` list identity):**
   - `seed_builtin_import_aliases_chirho` (`desugar_chirho.rs:290-314`) binds `"M.fromList"` to `CoreId("mapFromList")` in `var_scope_chirho`.
   - Because `M.fromList` has a constrained type scheme (`Ord k => [(k, v)] -> Map k v`), `constrained_names_chirho` contains `"M.fromList"`. `reference_occurrence_chirho` (`desugar_chirho.rs:197-220`) mints a fresh occurrence id whose surface name in `names_chirho` is `"M.fromList"`, mapped in `method_occurrence_canon_chirho` to `("fromList", CoreId("mapFromList"))`, and `reference_evidence_chirho` records `[("Ord", Some("<key>"))]`.
   - In `DictPassCtxChirho::occurrence_head_replacement_chirho` (`crates/haskelujah-core-chirho/src/dict_chirho/rewrite_chirho/constrained_references_chirho.rs:313-318`), the guard `if self.reference_evidence_chirho.contains_key(&head_id_chirho) { return None; }` deferred canonicalization to the dictionary-parameter call-site path (`collect_dict_param_app_chirho` / `apply_reference_dicts_chirho`), which only fires when `self.dict_param_bindings_chirho.contains_key(canonical_id_chirho)`.
   - Because `"mapFromList"` is a runtime builtin with no Core binding in `dict_param_bindings_chirho`, returning `None` left the occurrence id `"M.fromList"` on the application spine whenever the enclosing application did not enter `try_rewrite_typed_higher_order_args_chirho` (`args_chirho.len() >= 2`). `collect_method_app_chirho` stripped `"M."`, saw `"fromList"` in `method_selectors_chirho` (`IsList`), and rewrote `M.fromList` to `$sel_IsList_fromList $fIsList[tN]` (which after `b0ace8e1` evaluates to `\x -> id# x`, returning the raw input list instead of a `"$Map"` heap closure).

2. **Hop B (`M.toList` heap pair constructor mismatch with Core `"$tuple2"`):**
   - Once `M.toList` (`mapToList`) receives a `"$Map"` closure (either via `M.singleton`/`M.insert` as in `M02`, or via `M.fromList` after Hop A is fixed), `MachineChirho::build_pair_list_chirho` (`crates/haskelujah-runtime-chirho/src/eval_chirho.rs:4700-4705`) allocated 2-tuple closures with constructor name `"(,)"` and `lookup_con_tag_chirho("(,)", 0)` (`tag = 0`).
   - Core/STG lowering, `prelude_chirho.rs` (`fst`, `snd`, `zip`, `unzip`), and `ShowShapeChirho::TupleChirho` (`show_chirho.rs:193-196`) represent 2-tuples as `"$tuple2"`, matching its registered tag in `con_tag_map_chirho`.
   - Forcing the generated `$prim_Show_show_[(k, v)]` helper or `fst` on `build_pair_list_chirho`'s elements therefore crashed with `no matching alternative for tag 0 at @N`.

## Bounded placement

1. **`crates/haskelujah-core-chirho/src/dict_chirho/rewrite_chirho/constrained_references_chirho.rs:313-318` (`occurrence_head_replacement_chirho`):**
   Defer occurrence canonicalization to the dictionary-parameter call-site path only when `self.dict_param_bindings_chirho.contains_key(canonical_id_chirho)` holds:
   ```rust
   if self.reference_evidence_chirho.contains_key(&head_id_chirho)
       && self
           .dict_param_bindings_chirho
           .contains_key(canonical_id_chirho)
   {
       return None;
   }
   ```
   When `canonical_id_chirho` (`CoreId("mapFromList")`) has no dictionary parameters, `occurrence_head_replacement_chirho` restores `Some(CoreExprChirho::VarChirho(*canonical_id_chirho))` before `collect_method_app_chirho` runs.

2. **`crates/haskelujah-runtime-chirho/src/eval_chirho.rs:4700-4705` (`build_pair_list_chirho`):**
   Look up the tag and construct each pair closure using `"$tuple2"` instead of `"(,)"`:
   ```rust
   let tuple2_tag_chirho = self.lookup_con_tag_chirho("$tuple2", 0);
   ...
   let tup_closure_chirho =
       ClosureChirho::con_chirho(tuple2_tag_chirho, "$tuple2", vec![k_chirho, v_chirho]);
   ```

## Claimed movers, holds, and boundaries (12 extracted programs, 4 tests in `dictionary_evidence_chirho/data_map_tolist_chirho.rs`)

- **Movers to GHC-exact (7 extracted programs across 3 separately named tests):**
  - `qualified_data_map_from_list_size_answers_as_ghc_chirho` (Hop A only):
    1. `M05_map_size_fromlist_chirho.hs`: `M.size (M.fromList [(2, "b"), (1, "a") :: (Int, String)])` -> `"2\n"` (base: `"0\n"`, `WRONG`; isolates Hop A without `M.toList` or `Show`).
  - `qualified_data_map_to_list_insert_answers_as_ghc_chirho` (Hop B only):
    2. `M02_map_tolist_insert_chirho.hs`: `M.toList (M.insert (2 :: Int) "b" (M.singleton 1 "a"))` -> `"[(1,\"a\"),(2,\"b\")]\n"` (base: `no matching alternative for tag 0 at @2135`, `LOUD`; isolates Hop B without `M.fromList`).
  - `qualified_data_map_from_list_and_to_list_answer_as_ghc_chirho` (combined Hop A + Hop B):
    3. `D01_qualified_data_map_tolist_chirho.hs`: `M.toList (M.fromList [(2, 'b'), (1, 'a') :: (Int, Char)])` -> `"[(1,'a'),(2,'b')]\n"` (base `e7b6c69d`: `"[]\n"`, `WRONG`).
    4. `M01_map_tolist_int_values_chirho.hs`: `M.toList (M.fromList [(2, 20), (1, 10 :: Int)] :: M.Map Int Int)` -> `"[(1,10),(2,20)]\n"` (base: `"[]\n"`, `WRONG`).
    5. `M03_map_tolist_char_annotated_map_chirho.hs`: `M.toList (M.fromList [(2, 'b'), (1, 'a')] :: M.Map Int Char)` -> `"[(1,'a'),(2,'b')]\n"` (base: `"[]\n"`, `WRONG`).
    6. `M04_map_tolist_string_values_chirho.hs`: `M.toList (M.fromList [(2 :: Int, "b"), (1, "a")])` -> `"[(1,\"a\"),(2,\"b\")]\n"` (base: `"[]\n"`, `WRONG`).
    7. `M06_map_tolist_fst_projection_chirho.hs` (sorted-key projection control): `let xsChirho = M.toList (M.fromList [(2, 'b'), (1, 'a') :: (Int, Char)]) in print (map fst xsChirho)` -> `"[1,2]\n"` (base: `"[]\n"`, `WRONG`; verifies sorted key order and `"$tuple2"` field projection via `fst` independent of pair `Show`).
- **Holds (`list_to_list_and_insert_from_list_holds_answer_as_ghc_chirho` + existing `eval_import_data_map_chirho` in `eval_data_chirho.rs:600-614`):**
  8. `M07_list_tolist_non_interference_chirho.hs`: `import GHC.Exts (toList); main = print (toList [1, 2, 3 :: Int])` -> `"[1,2,3]\n"` (`EXACT` on both base and candidate).
  9. `M08_map_size_insert_fromlist_hold_chirho.hs` (the exact expression from `eval_import_data_map_chirho`): `M.size (M.insert 2 "b" (M.fromList [(1, "a")]))` -> `"2\n"` (`EXACT` on both base and candidate; on base `b19f30b8`, 3-argument `M.insert 2 "b" (...)` already entered `try_rewrite_typed_higher_order_args_chirho` and bypassed `collect_method_app_chirho`, whereas 1-argument `M05` hit `collect_method_app_chirho` and returned `0`. Both `M08` and `eval_import_data_map_chirho` are holds, not restored movers).
- **Explicit boundaries outside this unit (3 extracted programs recorded for oracle measurement, unasserted in Rust tests, plus native backend boundary):**
  10. `D02_unqualified_data_map_tolist_chirho.hs`: `import Data.Map (fromList, toList); main = print (toList (fromList [(2, 'b'), (1, 'a') :: (Int, Char)]))` -> GHC prints `"[(1,'a'),(2,'b')]\n"`; base and candidate print `"[(2,'b'),(1,'a')]\n"` (`STAYS-WRONG`). `seed_builtin_import_aliases_chirho` only binds qualified aliases (`M.fromList`, `M.toList`), so bare `fromList` and `toList` dispatch `IsList` list identity on both sides. Kept as a distinguishing bare-import control and classified from measurement, not assumed fixed.
  11. `T07_lexical_shadows_beat_the_alias_chirho.hs`: local parameter/lambda binder `toList` alongside `import Data.List.NonEmpty (NonEmpty(..), toList)` -> GHC prints `"3\n[5]\n[7,8]\n"`; base and candidate fail with `missing STG binding ':|' for CoreId 22` on `main_chirho` and `unbound variable: toList` under held unit 517 (`STAYS-LOUD`). In `DesugarCtxChirho::resolve_or_fresh_id_chirho` (`desugar_chirho.rs:144-166`), the local binder `toList` reuses `CoreId(83)` from `names_chirho`, colliding with the top-level `toList` alias in `substitute_var_chirho`. Explicitly outside this unit.
  12. `Q01_i1_with_typed_outer_tolist_chirho.hs`: `let toList = length in ...` alongside free `toList (1 :| [2, 3 :: Int])` -> GHC prints `"(3,[1,2,3])\n"`; base and candidate fail with `missing STG binding ':|' for CoreId 14` on `main_chirho` and `unbound variable: toList` under held unit 517 (`STAYS-LOUD`, same `resolve_or_fresh_id_chirho` local-binder `CoreId` collision). Explicitly outside this unit.
  - **Native boundary (`#28377`, `#28378`):** neither `rts-chirho` nor either native backend implements `Data.Map` primops (`LLVM` rejects `mapToList#/1` at compile time, and `Cranelift` lowers unknown primops to `0` and prints `[]`). Row 534 changes neither and does not claim native `Data.Map` coverage.

## Root-specific mutants (isolated across the separate Hop A, Hop B, combined, and Core unit tests)

1. **Mutant `M1` (Hop A — `constrained_references_chirho.rs:313-318`):**
   Revert `if self.reference_evidence_chirho.contains_key(&head_id_chirho) && self.dict_param_bindings_chirho.contains_key(canonical_id_chirho)` back to `if self.reference_evidence_chirho.contains_key(&head_id_chirho)`.
   - Fails Core unit test `dict_chirho::rewrite_chirho::occurrence_rows_tests_chirho::a_builtin_alias_without_dict_params_canonicalizes_despite_reference_evidence_chirho` (`core-lib`).
   - Fails Hop A test `data_map_tolist_chirho::qualified_data_map_from_list_size_answers_as_ghc_chirho` (`M05` reverts to `"0\n"`) and combined test `data_map_tolist_chirho::qualified_data_map_from_list_and_to_list_answer_as_ghc_chirho`.
   - Hop B test `data_map_tolist_chirho::qualified_data_map_to_list_insert_answers_as_ghc_chirho` (`M02`) and holds test `data_map_tolist_chirho::list_to_list_and_insert_from_list_holds_answer_as_ghc_chirho` (`M07`, `M08`) pass.

2. **Mutant `M2` (Hop B — `eval_chirho.rs:4700-4705`):**
   Revert `"$tuple2"` back to `"(,)"` in `MachineChirho::build_pair_list_chirho`.
   - Fails Hop B test `data_map_tolist_chirho::qualified_data_map_to_list_insert_answers_as_ghc_chirho` (`M02` fails with `no matching alternative for tag 0`) and combined test `data_map_tolist_chirho::qualified_data_map_from_list_and_to_list_answer_as_ghc_chirho`.
   - Core unit test `dict_chirho::rewrite_chirho::occurrence_rows_tests_chirho::a_builtin_alias_without_dict_params_canonicalizes_despite_reference_evidence_chirho`, Hop A test `data_map_tolist_chirho::qualified_data_map_from_list_size_answers_as_ghc_chirho` (`M05`), and holds test `data_map_tolist_chirho::list_to_list_and_insert_from_list_holds_answer_as_ghc_chirho` (`M07`, `M08`) pass.

## Corpus check-mode identity prediction

- **Membership and replay:** byte-identical to `main_chirho` (`e7b6c69d`): `should_compile` 885/938, `should_fail` 242/767, replay 295 lines. Neither Hop A (Core dictionary pass) nor Hop B (runtime `build_pair_list_chirho`) alters parser, naming, kind, or type-inference (`check`-mode) verdicts.
- **Execution corpus:** 30/35 unchanged (no execution corpus program calls `Data.Map.toList` or `Data.Map.fromList`).

## Source checklist

- [x] C1 (`b47e9c13`): this tasklist and bounded placement plan.
- [x] C2 (`436fa6b2`): Hop A (`constrained_references_chirho.rs`), Hop B (`eval_chirho.rs`), Core unit test (`occurrence_rows_tests_chirho.rs`), and end-to-end integration tests.
- [x] C3 (`1b2485d3`): initial workflow documentation and checkpoint hashes.
- [x] C4 (`c3a91169`): move `data_map_tolist_chirho.rs` into `tests/dictionary_evidence_chirho/` (`10 -> 11` entries) and split the 7 movers into 3 separately observable tests (`qualified_data_map_from_list_size_answers_as_ghc_chirho` for Hop A `M05`, `qualified_data_map_to_list_insert_answers_as_ghc_chirho` for Hop B `M02`, and `qualified_data_map_from_list_and_to_list_answer_as_ghc_chirho` for combined `D01`, `M01`, `M03`, `M04`, `M06`) plus the holds test (`list_to_list_and_insert_from_list_holds_answer_as_ghc_chirho` for `M07`, `M08`).
- [x] Checks without Cargo:
  - every touched Rust file is a pinned-rustfmt no-op (`rustfmt --check --edition 2024`), and `git diff --check` and `gitleaks` are clean;
  - `data_map_tolist_chirho.rs` is 123 lines, `dictionary_evidence_chirho.rs` is 270 lines, `occurrence_rows_tests_chirho.rs` is 346 lines, and `tests/dictionary_evidence_chirho/` holds 11 entries;
  - GHC 9.14.1 confirms all 12 extracted programs (`GREEN: 12/12 as recorded`). Frozen `main_chirho` CLI `b19f30b844e6d9249975b15e02e9054e6ab7fd089917233ca23a8a37a31dec98` (identical `crates/` tree to `e7b6c69d`) measures `WRONG` on 6 movers (`D01`, `M01`, `M03`, `M04`, `M05`, `M06`), `LOUD` on 1 mover (`M02`, `no matching alternative for tag 0 at @2135`), `EXACT` on the 2 holds (`M07`, `M08`), `WRONG` on boundary `D02` (`[(2,'b'),(1,'a')]`), and `LOUD` on boundaries `T07` and `Q01`;
  - Receipts: `tmp-chirho/data-map-534-chirho/ghc-receipts-chirho/` (C-locale digest `ff3544295141533d3a80eec9464d5ee8db7ffefb8b6dc458bfeaea3ff8e3487e`, `receipts-chirho.tsv` sha256 `fb7643df78a841352d7ae99768777e19e60785164b6f24d2058250f2843b4181`), `tmp-chirho/data-map-534-chirho/oracle-unit-main-r2-chirho/` (C-locale digest `5d8b7c32507d4f4829b316753757d7d72e845992d0b8fd51524127be8f8b75fa`), and `tmp-chirho/data-map-534-chirho/oracle-unit-main-chirho/` (`7d0f00647ff4eecc6d39c3f6357851ede747aa108a0617e016c1da0d414a47cc`); unit sha256 `9046dc27d1743ffb29868a1d4462b5999dbf973ff36cf3c850128a3317d65a52`.
- [x] C5: workflow and tasklist updates per `#28378`.
