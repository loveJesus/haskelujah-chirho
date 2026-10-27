<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Ord's method set and compare-backed defaults Chirho

## Origin and scope

Row 612 core.
- **Reserved:** gpt_chirho #29977, trace-only behind rows 609 and 606.
- **Approved by #30088, on checkpoint #30087:**
  - The split: row 612 is the compare-backed core; row 626 is the `<=`-only minimal definition; row 627 is end-to-end overridable `max` and `min`.
  - Ord's layout is `[Eq | <, <=, >, >=, compare, max, min]`, with `compare` in slot 5, and every builder and selector is layout-driven.
  - A stub row never donates.
  - Row 565's extrema helper may stay through rows 612 and 626, but it is not the final route; row 627 makes `max` and `min` method-driven, `maximum` and `minimum` included.
- **Branch:** `ord-method-set-chirho`, from main `a7d3336a`.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/ord-method-set-chirho/probes-chirho`.
  - The trace set in `sources-chirho`, and the core set in `core-chirho/sources-chirho`.
  - The pins in `pins-chirho/sources-chirho`, and the instances without `compare` in `neither-chirho/sources-chirho`.
  - Each set has GHC's outputs beside it.
- **Main** (the row 606 tip `9105f1c0`, whose compiler source main carries):
  - **Silently wrong:** operators and `compare` inside `Ord a =>` code at computed Strings, Doubles with NaN and signed zero, `[Int]` and user and derived types, with the two natives disagreeing; polymorphic `max` and `minimum`; `max` and `min` at Double NaN; Ord reached through `Integral`, `RealFrac` and `(Show a, Ord a)`.
  - **Loud:** an instance's own `>=` (`missing native binding >=`); the interpreter at every non-Int key (`LtIntChirho`, `LeIntChirho`).

## Placement

- **Typing:** `class_chirho.rs` seeds Ord with GHC's seven methods.
- **Dictionary pass:**
  - The new `instance_chirho/ordering_defaults_chirho.rs` holds the default rows; `instance_chirho/` has 14 entries.
  - `method_default_row_chirho` in `instance_chirho.rs` asks Eq's and Ord's defaults before a stub, for the dictionary resolver and the conditional list path.
  - `list_orders_chirho.rs` fills every `Ord [T]` slot in layout order.
  - Row 610's written-operator pass leaves a proof at the binding's own dictionary to the dictionary route.
  - `evidenced_aggregates_chirho.rs` folds own-key `maximum` and `minimum` over the dictionary's `compare`.
- **Controls:**
  - `tests/dictionary_evidence_chirho/ordering_chirho/ord_methods_chirho.rs`; `ordering_chirho/` has 11 entries.
  - The all-engine assertion the five sibling control files each copied now lives once in `dictionary_evidence_chirho.rs`.
  - The layout and selector units in `dict_pass_tests_chirho.rs`.
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`, with every passage that said Ord holds only `compare` updated.

## Checklist

- [x] Trace and checkpoint #30087; split and layout approved #30088.
- [x] Source: the seven methods, the defaults from real bodies only, the layout-driven list builder, and the own-key routes.
  - A generic-head (G5) coverage rule was written and then removed: row 600's G6 never makes an Ord instance generic, because Ord has a superclass, so the rule could not run.
- [x] Controls: GHC-measured, byte for byte; the layout and selector units updated.
  - **First suite run** (1701 tests): 1698 passed. The three failures were the layout unit, rewritten for the approved layout, and two intended movers: row 565's polymorphic `maximum` residual now answers `"c"`, and row 610's stub-key pin names `missing method Ord.< for VChirho`.
- [x] **Rerun row 608's generator and drift checks** (`generator608-chirho.log`): the outputs are byte-identical, 726 agree and 679 disagree, so the delta is none. The 18 qualified-alias, builtin-alias and reference tests pass.
- [x] A `<=`-only instance's `max` and `min` move too: GHC defines them through `<=`, so its own `<=` donates (`OLeOnlyExtremaChirho`, now a control). Its `compare` and other operators stay with row 626.
- [x] Commit; replay onto the newest main; broad moved-shape scan.
  - The source sits on main `37ddaa5a`. Main moved from `a7d3336a` with row 620's test fixtures, the progress DB and tasklists only, so its compiler source is the row 606 tip's.
  - **Scan** (`scan612-chirho.log`, on `d1177999`): 3585/3585 passed in 732 s, with no mover beyond the intended ones.
    - It covered every lib, the parser's included, and the 20 driver suites, leaving out `ghc_bulk`.
    - It skipped the same 18 tests as before. The count is row 606's 3582, plus this row's 3 controls.
- [x] **Reach** (`tools-chirho/reach_612_chirho.zsh`, log `reach612-r1-chirho.log`, on `d1177999`):
  - **M1, the six methods out of the seeded Ord:** the movers fail with `missing STG binding <`, and both named-failure controls lose their method names.
  - **M2, the defaults off:** the movers fail with `missing method Ord.< for Int`. Row 565's Int-shaped extrema and row 610's String operators fail too, because their keys now read the default rows.
  - **M3, the own-key operator route off:** `LtIntChirho`, and polymorphic `<` without `compare` stops naming its stub.
  - **M4, own-key `maximum` and `minimum` off:** `LeIntChirho`.
  - **M5, the list builder's defaults off:** `missing method Ord.< for [Int]`.
  - **M6, `max` picking as `min`:** wrong values in row 565's control and in the `max`/`min` defaults control.
  - Every restore was byte-identical, and the restored tree passes 16/16.
- [x] Judge main against the contract copy; warm gate; record.
  - **Main `37ddaa5a`** is GREEN against the contract copy first (73 movers, 9 holds). Its probes give main's measured answers (`baseline-37ddaa5a-chirho.log`).
  - **r1, RED at the changed-lines Clippy step:** the residual table moved out of row 565's pin kept one entry, and `single_element_loop` fired. The loop became a direct call, amended into the source `00cdba1c`.
  - **r2, GREEN on `00cdba1c` (base `37ddaa5a`) in 147.76 s:**
    - 1353/1353 focused tests. The tests step runs the families an Ord method change can move, and the scan runs the rest.
    - No Clippy diagnostic on the 456 changed lines in 16 files.
    - The contract judge GREEN.
    - rustfmt, diff-check and gitleaks clean (1 commit), and a build with no warning.
  - **The tip's probe reports** (`tip-00cdba1c-chirho` in each set): 8 programs are GHC on all three engines (24 cells), and all 18 Cranelift compiles carry the marker.
- [ ] Land on main and close progress row 612 (gpt).
