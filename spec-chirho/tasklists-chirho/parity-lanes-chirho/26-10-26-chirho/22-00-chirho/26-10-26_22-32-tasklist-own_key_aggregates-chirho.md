<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# sum and product at a binding's own Num key Chirho

## Origin and scope

Row 586 is mechanism B of gpt's evidence repair (#29427). It was split from row 584 at my checkpoint (#29436) and reserved to me in #29438.
- It covers a dictionary-passing `sum` and `product` for occurrences whose evidence is the binding's own key, with N4 and C3 required.
- Row 583, native `recip#`, replays after this row lands.
- The branch `own-aggregates-chirho` is on main `cbc8605f`. It is replayed onto the newest main before the gate.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/own-aggregates-chirho/probes-chirho` (A1-A4, C3, N4). The baseline is main `b786bd39` with row 584.
- **Main, at Double:** the interpreter fails on `AddIntChirho` or `MulIntChirho`, and the natives are silently wrong.
  - A1, `sum` order: `(-1.3671875000000001e-3,6)`; GHC gives `(0.6000000000000001,6)`.
  - A2, a point-free `product`: `(0.0,42,5.0e-324)`; GHC gives `(12.0,42,1.0)`.
  - A3, two predicates: `((NaN,6),(9,0.0))`; GHC gives `((4.0,6),(9,1.5))`.
  - N4: `NaN`; GHC gives `4.0`.
  - Int instantiations were right by coincidence.
- **The trace:** `sum` carries `(Num, $own:0)`, or `$own` under `Fractional a`. Row 552's concrete route finds no `$prim_Num_+_$own` row, so it falls back to the Prelude's Int-shaped `sum`.
- **With the change:**
  - A1, A2, A3 and N4 are GHC-exact on all three engines.
  - A4 and C3 are exact in the interpreter; their natives still refuse `recip#/1`, which is row 583.
- **Local diagnostic, never committed:** with the held `recip#` patch applied, C1-C5 and A4 are GHC-exact on all three engines.
- **Moved-shape scan:** typing, core and all driver lib tests, plus dictionary_evidence, evaluation_values, typing_integration, numeric_hierarchy and native_error_helpers. 2623/2623 passed, so nothing else moved.

## Placement

- **`rewrite_chirho/evidenced_aggregates_chirho.rs`:**
  - `aggregate_shape_chirho` and `aggregate_fold_chirho` are shared with row 552's concrete route.
  - `own_aggregate_chirho` resolves `d` through `own_dict_chirho`.
  - `evidenced_value_in_scope_chirho` and `evidenced_applied_head_in_scope_chirho` add the binding's dictionaries.
- **`rewrite_chirho/method_refs_chirho.rs`:** the three in-scope sites, which are first-class, argument and applied head.
- **Controls:** `tests/native_error_helpers_chirho.rs`, beside row 552's.
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`.

## Checklist

- [x] Measure on GHC and main; trace the route.
- [x] Implement the own-key fold and the in-scope hooks; the shapes match GHC as above.
- [x] Moved-shape scan: nothing else moved.
- [x] Controls measured on GHC as written; they pass, as does row 552's hold.
- [x] Reach (`tools-chirho/reach_586_chirho.zsh`, log `reach-r1-chirho.log`). All three were killed, each restore was byte-identical, and row 552's hold passed under every mutation:
  - **M1, no own route:** both new controls fail.
  - **M2, the own index ignored:** the two-predicate control fails.
  - **M3, a right fold:** `0.6` against GHC's `0.6000000000000001`.
- [x] Clippy finds nothing on the changed lines; its hits in `method_refs_chirho.rs` are older lines my hunks shifted. rustfmt is clean.
- [x] Commit; replay onto the newest main; judge main against the contract copy; warm gate; record it.
  - The source replayed cleanly over row 589's DB-only reservation as `b787261e`.
  - Main `006384e6` was GREEN against the contract copy (73 movers, 9 holds). Its controls failed as measured: the interpreter stopped at `AddIntChirho`, and the natives were silently wrong.
  - **r1 was GREEN on `b787261e` (base `006384e6`) in 228.11 s:**
    - 863/863 focused tests;
    - no Clippy diagnostic on the 188 changed lines in 3 files;
    - the judge at 126/126.
    - It ran 48 s over the three-minute aim. The test set was row 580's, and every step ran slower (the contract took 31.7 s against 21 s), which points to machine load.
- [ ] Land on main and close progress row 586 (gpt). Then row 583 replays the `recip#` patch and remeasures C1-C5.
