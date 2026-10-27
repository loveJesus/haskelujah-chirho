<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Generated Prelude bindings never take a local binder's id Chirho

## Origin and scope

Row 594 (A). I measured it during row 593 (#29549), and gpt assigned it in #29550 and released it in #29583.
- **The rule:** generated Prelude binding resolution skips program-owned binders deterministically instead of selecting in hash order.
- **The ruling (#29605):** it stays local-binder-only, exactly as measured.
- **Branch:** `local-alias-resolve-chirho`, from main `7f79fa8c`, replayed onto `cbd7be40` and then `9ecd15e1`.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/local-alias-chirho/probes-chirho` (the `L_*` probes) and `probes594-chirho` (the `O_*`, `T_*` and `hidden-chirho` probes). The control is `controls594-chirho`.
- **The sweep:** `tools-chirho/sweep_594_chirho.zsh` runs every program 8 times interpreted and twice on each native backend. Each pass hashes its name table afresh.
- **Main `cbd7be40`:**
  - The six local Prelude names fail every interpreted and every native run, each with a missing binding for one of the six.
  - The `L_*` probes fail intermittently on every engine. For example, `length` failed 6 of 8 interpreted runs, and `reverse` 7 of 8.
  - A lone local `foldl n = n + 1` beside `sum [1,2,3]` is silently wrong. LLVM prints `(4336781493,6)`, the interpreter fails on `AddIntChirho`, and Cranelift prints nothing. The generated `foldl` took the local binder's id, which captured row 552's fold inside `main`.
- **The trace:** `resolve_or_fresh_id_chirho` returned the first id named like the generated binding in the name table's hash order, skipping only occurrence ids. Beside a local `where length _ = 99`, that was the local binder's id in about half the passes. The Prelude function was then left unbound.
- **With the change:** every row 594 control and every `L_*` and `O_*` probe matches GHC in every run, on all three engines.
- **Not moved, and identical on main and the candidate:**
  - **Row 597 (#29605, #29606):** a hidden-and-redefined top-level `foldl` under `sum` and `product` gives `(0,1,7)`; GHC gives `(6,10,7)`. A hidden-and-redefined `reverse` under `isSuffixOf` gives `False`; GHC gives `True`.
  - **The qualified-name family (report-only):** `L.reverse` beside `reverse`, `M.filter` beside `filter`, and `P.length` beside a redefined `length` all fail with `no matching alternative for tag 47`. `L.reverse` alone fails natively with `show: no native Show instance for a boxed value`. An undeclared qualifier is accepted where GHC rejects it.

## Placement

- **`dict_chirho/mod.rs`:**
  - `module_local_binder_ids_chirho` holds the local binders, seeded with row 593's set.
  - `resolve_or_fresh_id_chirho` prefers the free id, then takes the least id that is neither a local binder nor an occurrence id, then a fresh one.
- **`dict_chirho/prelude_chirho.rs`:** row 580's alias helper goes through `resolve_or_fresh_id_chirho`.
- **Controls:**
  - `dict_pass_tests_chirho.rs`: the three 16-pass structural tests, beside row 580's `pi` test, which now shares their helper.
  - `tests/native_error_helpers_chirho.rs`: the six-name program, and the lone local `foldl` (its source is the probe measured above, byte for byte).
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`.

## Checklist

- [x] Measure on GHC and main; trace; assigned (#29550, #29583); ruling (#29605).
- [x] Resolver change; the control and every A probe match GHC on all three engines.
- [x] Controls measured on GHC as written; the new controls and the row 580/593 holds pass.
- [x] Reach (`tools-chirho/reach_594_chirho.zsh`; r2 is the receipt, `reach594-r2-chirho.log`):
  - **M1, the old resolver (the first id in hash order):** all three structural tests fail, and so do both driver controls.
  - **M2, the local-binder skip removed:** the lone-local `reverse` structural case fails, and so does the lone local `foldl` on the engines.
  - **M3, the free-id preference removed:** the top-level namesake test fails with `[3,3]`.
  - Every restore was byte-identical, and the restored tree passes.
  - r1, before the lone `foldl` control existed, killed M2 only structurally.
  - **A receipt note:** a refused first attempt reused row 593's `reach-r1-chirho` name and overwrote 593's superseded r1 log. Row 593's cited receipt, `reach-r2-chirho.log`, is intact.
- [x] **Broad moved-shape scan** (`moved-scan594-chirho.log`): 3119/3119 passed in 519 s.
  - It covered rts, runtime, core, typing, both backends, the harness and every driver lib.
  - It added 20 driver suites. Row 593's ten are dictionary_evidence, native_error_helpers, numeric_hierarchy, evaluation_values, cranelift_roundtrip, typing_integration, native_laziness, io_actions, show_precedence and codegen_scaling. Row 594's broader set adds canaries, curated_semantics, declaration_contexts, given_equalities, instance_obligations, lazy_patterns, method_occurrence_evidence, record_fields, rigid_tyvars and transformers.
  - Only the 16 `proptest_chirho::eval` properties were left out, and so was `ghc_bulk`, an aggregate tracker.
- [ ] Clippy; replay onto the newest main; judge main against the contract copy; warm gate; record.
- [ ] Land on main and close progress row 594 (gpt). Then row 597.
