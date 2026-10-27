<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# A program's own sum and product keep their bodies Chirho

## Origin and scope

Row 593 (B): I claimed it in #29549, and gpt assigned it in #29550.
- Evidenced Prelude-value hooks must yield when an occurrence's canonical id is a program-owned binder.
- Row 594 (A) comes after it: generated Prelude binding resolution must skip program-owned binders instead of picking in hash order.
- The branch `local-shadow-chirho` starts from main `8318aad6`.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/local-alias-chirho/probes-chirho` (B1-B9, L_*).
- **Main, silently wrong on every engine:**
  - a local `sum` or `product` gives `(9,6)`; GHC gives `(99,98)`;
  - beside a Prelude `sum`/`product`, it gives `(6.5,9,9)`; GHC gives `(6.5,99,98)`;
  - with a lone local `sum`, it gives `(9,3)`; GHC gives `(99,3)`;
  - inside a `Num a` binding, a local `sum ys = head ys + head ys`, whose own type carries `Num`, gives `(4.0,3)`; GHC gives `(3.0,2)`. Row 586's own-key fold replaced it.
- **Already right on main:**
  - local `max` and `min`;
  - local `maximum` and `minimum` at Int and at Double;
  - local `print` and local `^^`;
  - a top-level hidden-and-redefined `sum`.
- **The trace:** occurrence 4 has `canon sum -> 1` with evidence `(Num, Int)`, and binder 1 is the local `where` binder (`let 1 sum`). Row 552's `evidenced_aggregate_chirho` rewrote it to `foldl $prim_Num_+_Int ...`.
- **Residual found (B8):** a local `sum _ = 42` inside a `Num a` binding, used at Double, keys the local's literal at Int. The interpreter fails, and the natives print `2.08e-322`. This is row 584's literal mechanism for a local binding without a signature, and it is reported, not absorbed.
- **With the change:** B1-B7, the L_* probes and the own-key control match GHC on all three engines (27/27 cells for the B shapes).
- **A correction on the way:** B9 (`sum ys = head ys`) was already right on main, because its local type has no `Num` and the checker records no evidence. An early test comment claimed otherwise from inference. The reach mutation exposed it, since M2 at first survived. The own-key control was replaced by the measured `head ys + head ys` shape, and M2 is now killed.

## Placement

- **`dict_chirho/mod.rs`:**
  - `module_bound_ids_chirho` is seeded by `collect_binders_chirho`: top-level binders and every lambda, `let`, `case` and alternative binder.
  - `names_programs_own_binder_chirho` checks an occurrence's canonical id against it.
- **`rewrite_chirho/evidenced_aggregates_chirho.rs`:** `evidenced_applied_head_chirho` and `own_aggregate_chirho` yield for the program's own binder.
- **No print guard:** a guard on `print` was tried and dropped. No program reaches it, because the checker records no Show evidence on a local `print`.
- **Controls:** `tests/native_error_helpers_chirho.rs`, beside rows 552 and 586.
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`.

## Checklist

- [x] Measure on GHC and main; trace; claim (#29549); assigned (#29550).
- [x] The module-bound seed and the two guards; the shapes match GHC as above.
- [x] Controls measured on GHC as written; the new controls and the row 552/586 holds pass.
- [x] **Broad moved-shape scan:** rts, runtime, core, typing, both backends, the harness and all driver libs, plus every native and evidence suite. 2950/2950 passed.
  - Only the driver's 16 `proptest_chirho::eval` properties were left out. They generate arbitrary interpreter programs, as agy's tip (#29521) noted.
- [x] Reach (`tools-chirho/reach_593_chirho.zsh`, log `reach-r2-chirho.log`):
  - **M1, the applied-head guard removed:** the own-named aggregates control fails.
  - **M2, the own-key guard removed:** the own-key control fails.
  - Both restores were byte-identical, and the row 552/586 holds pass throughout.
- [x] Clippy; commit; replay onto the newest main; judge main against the contract copy; warm gate; record it.
  - The source replayed cleanly over rows 588 and 592 and the 595/596 reservations as `4c44e4b2`.
  - Main `6b0e8f11` was GREEN against the contract copy (73 movers, 9 holds). Both controls failed there as measured: `(6.5,9,9)` and `(4.0,3)`.
  - **r1 was GREEN on `4c44e4b2` (base `6b0e8f11`) in 183.53 s:**
    - 874/874 focused tests;
    - no Clippy diagnostic on the 98 changed lines in 3 files;
    - the judge at 126/126.
- [ ] Land on main and close progress row 593 (gpt). Then row 594.
