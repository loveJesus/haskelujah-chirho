<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Method-driven extrema Chirho

## Origin and scope

Row 627.
- **Split from row 612:** gpt_chirho #30088 named it as end-to-end overridable `max` and `min`.
  - `maximum` and `minimum` are included.
  - Do not branch permanently on whether an instance wrote an override.
- **Released by #30160:**
  - The full measured ordinary-`compare`/reversed-`<=` program is a required control: its concrete `max` must move from `MixChirho 2` to GHC's `MixChirho 1`.
  - At the end, row 565's compare helpers no longer decide any served concrete `max`, `min`, `maximum` or `minimum` key.
- **Branch:** `ord-extrema-methods-chirho`, from main `4f6e938c` (row 626 landed as `92f0d064` and `b89de152`).

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **GHC.List:** `maximum = foldl1' max` and `minimum = foldl1' min`, so an instance's own `max` or `min` decides both the methods and the folds.
- **Probes:** `tmp-chirho/ord-extrema-methods-chirho/probes-chirho`, with GHC's outputs and the `tip626-294fd482-chirho` and `wip1-chirho` reports.
  - The main reports come from row 626's tip, whose compiler source main `4f6e938c` carries.
- **Main (silently wrong on every engine):**
  - **The mixed program's concrete `max`:** `MixChirho 2`, where GHC gives `MixChirho 1`.
  - **An instance's own `max`:** `(WChirho 2,WChirho 1,WChirho 5)`, where GHC gives `(WChirho 0,WChirho 1,WChirho 0)`.
  - **An instance's own `min`:** concrete, `minimum` in and outside `Ord a =>` code, and `zipWith min`. Main gave `(UChirho 1,UChirho 2,UChirho 3,UChirho 5,[UChirho 1])`, where GHC gives `(UChirho 7,UChirho 2,UChirho 7,UChirho 7,[UChirho 7])`.
  - **An own `max` through polymorphic `maximum` and `foldr1 max`:** main gave `WChirho 5` and `WChirho 9`, where GHC gives `WChirho 0`.
- **The candidate:** all four programs move to GHC on all three engines.
- **Holds:** builtin, String, derived, list, `Maybe`, tuple and Double-NaN extrema, first-class and partial.

## Placement

- **`prelude_chirho/ordering_extrema_chirho.rs`:** row 565's four compare-backed helpers are replaced by one fold, `$ord_fold1_chirho = \pick xs -> case xs of (:) h t -> foldl pick h t`.
- **`rewrite_chirho/evidenced_aggregates_chirho.rs`:**
  - `maximum` and `minimum` fold the key's `max` or `min` row (stubs included, so a slot with no body fails by name), and at the binding's own key, its dictionary slot.
  - `max` and `min` leave the evidenced route and take the method route.
- **`instance_chirho/ordering_defaults_chirho.rs`:** `ordering_method_row_chirho` finds a key's row under its spellings, and `generate_evidenced_extremum_rows_chirho` gives keys without a dictionary their `max` and `min` rows from the `<=` answer.
- **Controls:** `tests/dictionary_evidence_chirho/ordering_chirho/extrema_methods_chirho.rs` (`ordering_chirho/` has 13 entries). Row 565's control comments say what they now exercise; its test names stay as receipts.
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`, with the rows 565, 612 and 626 passages that row 627 changes.
  - The file is 1062 lines, over the 1k aim. Its Ord sections could move to their own document in a docs row.

## Checklist

- [x] Trace on the row 626 tip; release #30160.
- [x] Source: one method-driven fold, rows for keys without a dictionary, and row 565's helpers deleted.
- [x] Controls: GHC-measured, byte for byte, the required mixed program included.
  - **First suite run** (1407 tests): 1407 passed, with no moved pin; row 565's programs hold through the method rows.
- [x] **Reach** (`tools-chirho/reach_627_chirho.zsh`, log `reach627-r1-chirho.log`, on `9a9d2781`):
  - **M1, the concrete fold off:** `maximum` falls back to the Int-shaped binding. An own `max` gives `(WChirho 0,WChirho 1,WChirho 5)`, and the holds program and row 565's control stop at `LeIntChirho`.
  - **M2, the own-key fold off:** polymorphic `minimum` gives `UChirho 5`, and row 612's polymorphic-extrema control stops at `LeIntChirho`.
  - **M3, the generated rows off:** `max` at `Maybe Char` stops at `LeIntChirho`.
  - Every restore was byte-identical, and the restored tree passes 8/8. Row 565's helpers are deleted, so no path to them is left to mutate.
- [x] Commit; replay onto the newest main; broad moved-shape scan.
  - The source `9a9d2781` was first scanned on main `4f6e938c`: 3589/3589 in 800 s.
  - Main then moved to `a2567c61` with row 628's CoreId-named native bindings across both backends. The rebased source `ac1890eb` was re-probed (all five programs keep their answers on all three engines) and re-scanned: 3589/3589 in 903 s, with no mover (`scan627-rebased-chirho.log`).
    - That is row 626's 3587, plus this row's 2 controls. It covered every lib and the 20 driver suites, leaving out `ghc_bulk`, and skipped the same 18.
- [x] Judge main against the contract copy; warm gate; record.
  - **Main `4f6e938c`** and then **main `a2567c61`** are GREEN against the contract copy first (73 movers, 9 holds). Their probes give main's measured answers (`baseline-4f6e938c-chirho.log`, `baseline-a2567c61-chirho.log`).
  - **r1, GREEN on `ac1890eb` (base `a2567c61`) in 226.49 s:**
    - 1357/1357 focused tests, row 626's filter plus the extrema controls;
    - no Clippy diagnostic on the 229 changed lines in 7 files;
    - the contract judge GREEN;
    - rustfmt, diff-check and gitleaks clean (1 commit), and a build with no warning.
  - **The tip's probe reports** (`tip-ac1890eb-chirho`): 5 programs are GHC on all three engines (15 cells), and all 5 Cranelift compiles carry the marker.
  - Main has since moved to `e0dafd3d` with the progress DB only, so the commits replay patch-identically.
- [x] Land on main and close progress row 627 (gpt): source `447c975c`, gate record `cda7df5b`; independent replay checks passed core 197/197 and the two all-engine extrema-method controls.
