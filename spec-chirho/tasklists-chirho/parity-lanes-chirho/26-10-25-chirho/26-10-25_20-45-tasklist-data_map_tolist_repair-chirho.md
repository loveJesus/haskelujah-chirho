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
- [x] C5 (`3c5b77e4`): workflow and tasklist updates per `#28378`.
- [x] C6: the Gate ticks and the gate runs record, after the r4 GREEN gate (`#28407`).

## Gate

The gate contract is `#28376` and `#28378`, amended by `#28386`/`#28388` (mutator invocation and source-derived focused argument shapes), `#28395` (`--runtime-pair` Clippy bijection against row 533 v6's driver log), and `#28400`/`#28402` (raw-string `M1` expected-text matcher against Rust 1.93's un-backticked assertion output). It runs only under a builder handed by name, on source tip `3c5b77e49bc2f984b39655529220a4c8d24436e3` over `main_chirho` `e7b6c69d6069b7d513db9cd0553cd5d0b17d542e`, comparing against the frozen row 533 v6 GREEN candidate CLI `b19f30b844e6d9249975b15e02e9054e6ab7fd089917233ca23a8a37a31dec98` (identical `crates/` tree `7a3f6a66` to `main_chirho` `e7b6c69d`). Every item below is ticked from the GREEN r4 run, `dm534-data-map-3c5b77e4-r4-chirho`:

- [x] Preflight pins exact base `e7b6c69d`, tip `3c5b77e4`, the 5 commits and 7 unit paths (only `haskelujah-core-chirho`, `haskelujah-runtime-chirho`, and `haskelujah-driver-chirho` changed under `crates/`), crate tree hashes, committed unit test sha256 `9046dc27d1743ffb29868a1d4462b5999dbf973ff36cf3c850128a3317d65a52`, GHC 9.14.1 `12/12` re-derivation, judge selftest `97 PASS`, frozen base CLI `b19f30b8`, 14 tool pins, 13 C-locale immutable sets, and 3 held refs (`patterns-fundep-rc5-replay-chirho` `de92912a`, `import-scope-join-amp-main-chirho` `000d5c45`, `derived-show-precedence-replay-chirho` `e0307f5c`).
- [x] Warning-free candidate build (`rc 0`, 0 warnings) and frozen candidate CLI (`15e1b77b7422e9ea5cea8f00754579be765a114c7b129969b57b28484f1f9f95`).
- [x] All 9 focused suites pass as their contracts require: `core-lib` `161` (base `160` + 1 new Core test), `runtime-lib` `54` (all 54 source-discovered `#[test]` functions), `dictionary_evidence` `68` (base `64` + 4 new tests), `driver-filtered` `609` passed / `1244` filtered out (exact row 533 v6 multiset plus all 142 `eval_data_chirho` tests including `eval_import_data_map_chirho`), `show_precedence` `2`, `instance_obligations` `28`, `io_actions` `9`, `typing-lib` `358`, and `typing_integration` `40`.
- [x] Lint and `gitleaks`: all 5 Clippy log pairs match one-to-one across forms (`core` `130` same-form + `4` cross-form; `driver` `375` + `87`; `cli` `353` + `78`; `typing` `56` + `0`; `runtime` `--runtime-pair` `12` same-form + `1` cross-form, `13 vs 13` within `crates/haskelujah-runtime-chirho/` and `crates/haskelujah-rts-chirho/`), with 0 errors, nothing unparsed, and no diagnostic on an inserted/changed line or in a new file; `gitleaks` scans all 5 unit commits with 0 leaks.
- [x] Unit oracle (`oracle-data_map_tolist_chirho`): exactly the 7 movers (`M05`, `M02`, `D01`, `M01`, `M03`, `M04`, `M06`) move from non-GHC (`6 WRONG`, `1 LOUD`) to `EXACT` with GHC stdout and empty stderr; the 2 holds (`M07`, `M08`) stay `EXACT`; the 3 unasserted boundaries (`D02` `WRONG`, `T07` `LOUD`, `Q01` `LOUD`) retain their base verdict/detail and byte-identical streams; all 6 guard unit oracles (`derived_show` `19`, `list_orders` `15`, `string_lists` `9`, `higher_order_occurrences` `34`, `generic_heads` `19`, `ordering_literals` `28`) are identical between base and candidate at their pinned counts.
- [x] Guards: all 37 matrix rows are identical with 24 requirements `MET`, and all 12 RC5 probes are identical between base and candidate.
- [x] Root-specific mutants (`caught 2 of 2`, baseline `VALID` at 5, restored `BASELINE` at 5, all 3 mutation-owned files restored byte-exact to their `3c5b77e4` tip blobs):
  - `M1-hop-a-alias-guard-dropped`: `CAUGHT` with `passed=2` and exact 3-failure partition (`qualified_data_map_from_list_and_to_list_answer_as_ghc_chirho`, `qualified_data_map_from_list_size_answers_as_ghc_chirho`, `dict_chirho::rewrite_chirho::occurrence_rows_chirho::tests_chirho::a_builtin_alias_without_dict_params_canonicalizes_despite_reference_evidence_chirho`).
  - `M2-hop-b-tuple2-reverted`: `CAUGHT` with `passed=3` and exact 2-failure partition (`qualified_data_map_from_list_and_to_list_answer_as_ghc_chirho`, `qualified_data_map_to_list_insert_answers_as_ghc_chirho`).
- [x] Corpus and postflight: candidate CLI completes with byte-identical passes; membership (`accept-pass1-chirho.txt` `939` lines, `reject-pass1-chirho.txt` `768` lines) and replay (`replay-verdicts-chirho.txt` `296` lines) are byte-identical to row 533 v6; execution stays at `30/35 PASS` (`fixed none, lost none`); postflight confirms both frozen CLIs unchanged, tracked tree clean at `3c5b77e4`, all 3 mutated files at their tip blobs, gate script, 14 tools, 13 immutable sets (C locale) and 3 held refs at their pins, and no `cargo`, `rustc`, `clippy`, or CLI process alive.

## Gate runs

All receipts live under `tmp-chirho/data-map-534-chirho/gate-chirho/runs-chirho/` in the `haskelujah-agy-chirho` worktree. Source tip `3c5b77e49bc2f984b39655529220a4c8d24436e3` never changed across `r1`–`r4`. Tooling round `r1` was refined before any builder handoff (`#28386`, `#28388`) and never launched a gate run.

### RED: `dm534-data-map-3c5b77e4-r2-chirho` (tooling r2), tip `3c5b77e4`

- Launched under builder handoff `#28391`. `preflight`, `build`, and all 9 `focused` suites passed `GREEN`; stopped `RED` at `lint` (`STAGE-EXIT lint 1`) before `oracle`, `guards`, `mutants`, or `corpus`.
- Cause (`#28393`, `#28395`): a gate-baseline defect, not a product warning. `gate_dm_chirho.sh` required `0` warnings in `clippy-haskelujah-runtime.log`, whereas `cargo clippy -p haskelujah-runtime --all-targets` emits 13 pre-existing warnings (`12` in `crates/haskelujah-runtime-chirho/`, `1` in `crates/haskelujah-rts-chirho/`), none on `eval_chirho.rs` (the only runtime file touched by row 534). Those same 13 warnings were already present in row 533 v6's `clippy-haskelujah-driver.log` (`12` in short form, `1` in long form).
- Hashes (pinned in `IMMUTABLE_CHIRHO` as `r2-red-run` and `r2-red-launcher` from `r3` onward):
  - frozen candidate CLI: `c19281e02ca47b3df25b0ffbfc07afc6adb07ebd365ba3a91d274d2d1ad7781a`;
  - frozen base CLI: `b19f30b844e6d9249975b15e02e9054e6ab7fd089917233ca23a8a37a31dec98`;
  - gate script: `4e41778579cdd6f228de7863f3324eb1e4c9b734308028b1d235d720be01ce82`;
  - ledger (`stages-chirho.tsv`): `8cd62e14954108aeaa7d30f8172459ddd79ead4a6ad601bc7241d4146aa68a0b` (`11` `GREEN` rows, `lint` `RED`, `gitleaks` `GREEN`);
  - exit codes (`exit-codes-chirho.tsv`): `51872fed1a9a880bac2f434773f06fc5f27724a2c32e628dea82df37f260ddc3` (`build 0`, `focused 0`, `lint 1`);
  - C-locale run digest: `ffa2d02a0680d49bf41a9253ee01a22a223c7490db8ebfe79fd2374ce3e9b94b` (`1,480` files);
  - C-locale launcher digest: `fc7a8ef101b97fa850502c29cf8c250a6a5fd1d4eba4bee6df141ea592ab7948` (`4` files).

### RED: `dm534-data-map-3c5b77e4-r3-chirho` (tooling r3), tip `3c5b77e4`

- Tooling `r3` (`#28395`, `#28396`) added `--runtime-pair` to `clippy-bijection` in `dm_gate_checks_chirho.py`, comparing `clippy-haskelujah-runtime.log` against row 533 v6's `clippy-haskelujah-driver.log` restricted to `crates/haskelujah-runtime-chirho/` and `crates/haskelujah-rts-chirho/` (`13 vs 13` primary diagnostics: `12` same-form + `1` cross-form).
- Launched under builder handoff `#28398`. `preflight`, `build`, all 9 `focused` suites, 5-pair `lint` + `gitleaks`, all 7 `oracle` units (`oracle-data_map_tolist_chirho` + 6 guard units), and `guards` (`37` matrix rows + `24` requirements `MET` + `12` RC5 probes) passed `GREEN`; stopped `RED` at `mutants` (`STAGE-EXIT mutants 1`) before `corpus`.
- Cause (`#28399`, `#28400`, `#28402`): a harness expected-text defect in `mutate_data_map_chirho.py`, not a product or isolation failure. Both mutants produced their exact reviewed failure partitions and restored all 3 owned files byte-exact to their `3c5b77e4` tip blobs (`baseline VALID passed=5 failed=[]`, `M1` `passed=2` with its exact 3 failures, `M2` `CAUGHT` with `passed=3` and its exact 2 failures, `restored BASELINE passed=5 failed=[]`), but `mutate_data_map_chirho.py` specified `M1`'s expected failure substring with backticks (`('left: `"0\\n"`', 'right: `"2\\n"`')`), whereas `rustc 1.93.0`'s `assert_eq!` in `M1-hop-a-alias-guard-dropped.log` outputs `left: "0\n"` and `right: "2\n"` without backticks (literal backslash + `n`), causing `harness_chirho.py` to classify `M1` as `CAUGHT-ELSEWHERE` (`caught 1 of 2`).
- Hashes (pinned in `IMMUTABLE_CHIRHO` as `r3-red-run` and `r3-red-launcher` in `r4`):
  - frozen candidate CLI: `c19281e02ca47b3df25b0ffbfc07afc6adb07ebd365ba3a91d274d2d1ad7781a`;
  - frozen base CLI: `b19f30b844e6d9249975b15e02e9054e6ab7fd089917233ca23a8a37a31dec98`;
  - gate script: `933c198917fa5fa6484f0d0641ddc533bf788adc1ff0f4498e1ea2a13ff4a0dc`;
  - ledger (`stages-chirho.tsv`): `122642e5c5c1c77517689cfbe36f42dff2da4efcb5207c54e095174c4ac5a0ab` (`21` `GREEN` rows, `mutants` `RED`);
  - exit codes (`exit-codes-chirho.tsv`): `42384520b37b09edb9a0e1a363d71398556d02ebdb432e18aca449688a223b0f` (`build 0`, `focused 0`, `lint 0`, `oracle 0`, `guards 0`, `mutants 1`);
  - `logs-chirho/mutations.log`: `1709cdedcde1c021ef67df6f461e6c0b7c4c0fa5ff9a70768910b19c000583a5`;
  - `logs-chirho/mutations.compare`: `3488a33cbdd002173b0f022b772f4a3c6a47d859e43a40075a0f9256e2580076`;
  - C-locale run digest: `9b0f9a4934efd5d10017648cde048d195066acee91ebed8ba51ddb72e5d78edc` (`3,365` files);
  - C-locale launcher digest: `1c73e80523b6c448256628c88ff6594317aae5dd869eb1a902e2d9d7627533e7` (`7` files).

### GREEN: `dm534-data-map-3c5b77e4-r4-chirho` (tooling r4), tip `3c5b77e4`

- Tooling `r4` (`#28400`, `#28402`, `#28403`) updated `mutate_data_map_chirho.py` line 76 to the raw-string tuple `(r'left: "0\n"', r'right: "2\n"')`, wired the preserved `r3` real mutation logs into `selftest_dm_chirho.py` (`97/97 PASS`) and `tooling_proof_dm_chirho.zsh` (`100/100 PASS`) with separate negative controls for both the old backticked reasons and the non-raw single-backslash variant, and pinned `r3-red-run` and `r3-red-launcher` (`13` immutable sets total).
- Launched under builder handoff `#28404` at `2026-10-26 00:26:26 EDT`. All 7 requested stages (`build`, `focused`, `lint`, `oracle`, `guards`, `mutants`, `corpus`) and `postflight` exited `0` (`00:26:51`, `00:33:40`, `00:33:45`, `00:38:00`, `00:39:35`, `00:39:56`, `00:55:07 EDT`), with `24/24` `GREEN` rows in `stages-chirho.tsv`.
- Hashes:
  - ledger (`stages-chirho.tsv`): `bee8ec5cecf8d8e0602f936e91c56225e8eaebc29cacf2cf6385dd92714ae121` (`24` rows, all `GREEN`);
  - exit codes (`exit-codes-chirho.tsv`): `4f9bf1b6660034f06d91c1bc90451ca2f41497bbd06a096935c028fbe080c3c2` (`7` rows, all `0`);
  - `inputs-chirho.txt`: `0341e6f1a6283d2df3b125ae703b03141920620644700d01cdc7a17cc59271c5`;
  - frozen candidate CLI (`haskelujah-dm534-data-map-3c5b77e4-r4-chirho-chirho`): `15e1b77b7422e9ea5cea8f00754579be765a114c7b129969b57b28484f1f9f95`;
  - frozen base CLI (`haskelujah-base-dm534-data-map-3c5b77e4-r4-chirho-chirho`): `b19f30b844e6d9249975b15e02e9054e6ab7fd089917233ca23a8a37a31dec98`;
  - gate script (`gate_dm_chirho.sh`): `4c051086043cc0a4e87d118e715a9804441db1732e5eccab402464c85bf03a59`;
  - `logs-chirho/lint-multiset.log`: `0326598098c45d2a549d3356df6c6ad3bca1a0ad3df502c332ff68d2169465d9`;
  - `logs-chirho/oracle-data_map_tolist_chirho.compare`: `b21991a49cdf3f6bc1ba043d3eb4b7310aac03d1b6ca3e85b0eee133149c9507`;
  - `logs-chirho/mutations.log`: `5adbf35165ed8fed3ea500b3d6db8b8cdde7173aec20202ae789257bace35a71`;
  - `logs-chirho/mutations.compare`: `20bfab611a8d7ea8e2599302b02faa1bd3c56d70de53ae1da467284db4f8a05c`;
  - `mutations/M1-hop-a-alias-guard-dropped.diff`: `7b9dc58dcb11cb76ff7ea1eb4fac405a15e43d95b2faf4f1007197f484a66709`;
  - `mutations/M2-hop-b-tuple2-reverted.diff`: `0f1733b9abbaaca4dacc6e0cb26f4f206f2c248f8a0dd1e6bd220d229b95cf3d`;
  - `logs-chirho/corpus.compare`: `96bfd8b130b646efc8160c2768a1fbfc5dbd3fc3d163d07fa9aaef09e3f57e55`;
  - C-locale run digest: `3234aa61632f5880da8f77e01c248dc143660b31beed37fec0c79afbb142a50e` (`14,711` files);
  - C-locale launcher digest: `0a74992cfa9b88ab6cd692c01ce5481c3aa4e596b20f7d9cebd1c4de57c9e5f4` (`8` files).
- Stage summary:
  - **Preflight & Build:** clean tip `3c5b77e4` (`5` commits and `7` unit paths from `e7b6c69d`); GHC 9.14.1 `12/12`; judge selftest `97 PASS`; `14` tools, `13` immutable C-locale sets, and `3` held refs at their pins; `cargo build` exited `0` with `0` warnings.
  - **Focused (`9` suites):** `core-lib` `161`, `runtime-lib` `54`, `dictionary_evidence` `68`, `driver-filtered` `609` passed / `1244` filtered out (including all `142` `eval_data_chirho` tests and `eval_import_data_map_chirho`), `show_precedence` `2`, `instance_obligations` `28`, `io_actions` `9`, `typing-lib` `358`, `typing_integration` `40`.
  - **Lint (`5` Clippy pairs + `gitleaks`):** `core` `130` same-form + `4` cross-form; `driver` `375` + `87`; `cli` `353` + `78`; `typing` `56` + `0`; `runtime` (`--runtime-pair`) `12` same-form + `1` cross-form (`13 vs 13`); `0` errors, nothing unparsed, nothing on an inserted/changed line or in a new file; `gitleaks` scanned `5` commits with `0` leaks.
  - **Oracle & Guards:** `oracle-data_map_tolist_chirho` moved all `7` movers (`M05`, `M02`, `D01`, `M01`, `M03`, `M04`, `M06`) to `EXACT`, held `M07` and `M08` at `EXACT`, and kept `D02` (`WRONG`), `T07` (`LOUD`), and `Q01` (`LOUD`) byte-identical on their base verdicts/streams; all `6` guard unit oracles (`derived_show` `19`, `list_orders` `15`, `string_lists` `9`, `higher_order_occurrences` `34`, `generic_heads` `19`, `ordering_literals` `28`), all `37` matrix rows (`24` `MET`), and all `12` RC5 probes were identical between base and candidate.
  - **Mutants (`caught 2 of 2`):** `baseline VALID` (`passed=5 failed=[]`), `M1-hop-a-alias-guard-dropped CAUGHT` (`passed=2`, `3` failures), `M2-hop-b-tuple2-reverted CAUGHT` (`passed=3`, `2` failures), `restored BASELINE` (`passed=5 failed=[]`), and all `3` mutation-owned files restored byte-exact to their `3c5b77e4` tip blobs (`constrained_references_chirho.rs` `d9499bd7`, `eval_chirho.rs` `dd1f1b3f`, `data_map_tolist_chirho.rs` `9046dc27`).
  - **Corpus & Postflight:** `accept-pass1-chirho.txt` (`939` lines), `reject-pass1-chirho.txt` (`768` lines), and `replay-verdicts-chirho.txt` (`296` lines) were byte-identical to row 533 v6; execution stayed at `30/35 PASS` (`fixed none, lost none`); postflight verified both frozen CLIs, clean tree, tip blobs, `14` tools, `13` immutable sets, `3` held refs, and `0` live compiler/gate/CLI processes.
- **Not claimed:** reproducible dev-profile binary bytes (`r2` and `r3` froze `c19281e0`; after `r3`'s mutation restore rebuilt the workspace from the restored tip sources, `r4` froze `15e1b77b` at the identical `3c5b77e4` tree and toolchain, and behavior was gated on `15e1b77b`), native `Data.Map` primop coverage (`LLVM` and `Cranelift`), bare unqualified `Data.Map` `fromList`/`toList` (`D02`), or the `resolve_or_fresh_id_chirho` local-binder `CoreId` collision (`T07`, `Q01`). Nothing was written to `main_chirho` or the progress DB.
