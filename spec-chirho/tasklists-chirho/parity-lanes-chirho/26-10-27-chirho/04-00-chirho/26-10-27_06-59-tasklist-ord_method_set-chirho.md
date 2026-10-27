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
- [ ] Reach: each mechanism removed alone, then restored byte-identical.
- [ ] Commit; replay onto the newest main; broad moved-shape scan.
- [ ] Baseline judge on main, gate, record, post.
