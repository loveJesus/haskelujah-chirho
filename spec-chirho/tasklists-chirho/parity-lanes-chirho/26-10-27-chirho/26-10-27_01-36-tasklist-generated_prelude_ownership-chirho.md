<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Generated Prelude code keeps to its own dependencies Chirho

## Origin and scope

Row 597.
- **Found during row 594 (#29604);** reserved to me in #29605 and #29606, and begun after 594 landed (#29664).
- **Scope ruling (#29690):** all seven measured generated-dependency movers (`foldl`, `reverse`, `gcd`, `mapM_`, `putStrLn`, `append`, `insert`) go under one compiler-ownership rule, with no per-name allowlist.
- **Bounds:**
  - Only the two plain-name body-backed sites use the new lookup.
  - `$`-internal rows keep their route.
  - The `$prelude_` spelling applies only on collision.
- **Out of scope:** the qualified-name family is row 598 (claude2), whose paths are disjoint (#29692).
- **Branch:** `generated-ownership-chirho`, from main `36431dbe`, replayed onto `71b0dbf9`.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/generated-ownership-chirho/probes-chirho`, with GHC outputs and per-engine reports for main and both prototypes.
- **Main `36431dbe`, deterministic and identical on every engine:**
  - a hidden-and-redefined `foldl` under `sum` and `product` gives `(0,1,7)`; GHC gives `(6,10,7)`;
  - `reverse` under `isSuffixOf` gives `False`; GHC gives `True`;
  - `gcd` under `lcm` gives `(24,1)`; GHC gives `(12,1)`;
  - `mapM_` under `forM_` gives `own|own`; GHC gives `1|2|own`;
  - `putStrLn` under `print` gives `own 7`; GHC gives `7`.
- **Main `36431dbe`, no hiding needed:**
  - A program's own top-level `append` breaks `concat` and `concatMap`. The interpreter fails on `MulIntChirho`, and the natives print nothing.
  - A binary search tree's `insert` beside Data.List `sort` fails with `Non-exhaustive patterns`.
- **Holds on main:** hidden-and-redefined `length`, `reverse` and `even` used directly, and a hidden `map` beside `concatMap` and `mapM_`.
- **The trace:** generated code reaches its dependencies by name through two routes.
  1. **`resolve_or_fresh_id_chirho`.** About 143 plain names reach it. Its fallback took the program's top-level binder, so the generated binding shared the program's id, the program's body won, and every generated caller called it.
  2. **`lookup_body_backed_name_id_chirho`.** It returns the program's binding first. Of its 16 call sites, only `"foldl"` (row 552's fold) and `"putStrLn"` (the evidenced `print`) pass plain names. The others pass `$` names.
- **The natives name top-level functions by spelling:**
  - With compiler-owned ids alone (prototype 1), Cranelift refused with `Duplicate definition of identifier: foldl`.
  - LLVM merged the two: the program's `foldl (+) 7 [1,2]` gave 10, and its `gcd` and `reverse` became the Prelude's.
- **With the change (prototype 2, then the committed source):** all nine programs match GHC on all three engines, 27/27 cells.
- **A confounded probe, set aside:** a `putStrLn` probe that called `IO.putStrLn` through a qualified import mixed in row 598's family, and hung on main. The unqualified probe replaced it; the original is kept in `confounded-chirho`.

## Placement

- **`dict_chirho/mod.rs`:**
  - `module_source_binder_ids_chirho`, seeded with the top-level binders under source names;
  - the resolver's skip;
  - `lookup_compiler_owned_name_id_chirho`;
  - `spell_compiler_owned_apart_chirho`;
  - `is_compiler_internal_name_chirho`, which treats `$` followed by an identifier character as internal, and `$` and `$!` as source operators.
- **`dict_chirho/rewrite_chirho.rs`:** the spelling call at output assembly.
- **`rewrite_chirho/evidenced_aggregates_chirho.rs` (`foldl`) and `rewrite_chirho/method_refs_chirho.rs` (`putStrLn`):** the two body-backed sites.
- **Controls:**
  - `tests/native_error_helpers_chirho.rs`: the seven movers and the two holds, every source byte for byte the GHC-measured probe. The file is now 1043 lines, under the 1.5k limit. `tests/dictionary_evidence_chirho/` already has 14 entries, so it got no new file.
  - `dict_pass_tests_chirho.rs`: two new structural tests, and row 594's namesake test re-pinned to the `$prelude_length` spelling.
  - `print_evidence_tests_chirho.rs`: re-pinned to the compiler's `putStrLn`, and now also pinning that the program's `$prim_Show_show_Bool` row, id 100, still serves.
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`. Row 594's "not covered" note now points here.

## Checklist

- [x] Trace, inventory and probes on GHC and main; design checkpoint (#29689); scope ruling (#29690); path overlap posted (#29692).
- [x] **Broad scan of the prototype** (`proto2-scan-chirho.log`): 3125 run, 3123 passed. The only movers are the two re-pinned core tests, reported before commit (#29707).
- [x] Controls measured on GHC as written; all pass on the candidate with row 594's holds.
- [x] **Reach** (`tools-chirho/reach_597_chirho.zsh`, log `reach597-r1-chirho.log`):
  - **M1, the resolver takes the program's source binder again:** the structural namesake and `print` tests fail, and so do the hidden-redefinition (`isSuffixOf` gives `False`) and helper-namesake controls.
  - **M2, the two body-backed sites back on the old lookup:** the `print` test fails, and the `foldl` control gives `(0,1,7)`.
  - **M3, no spelling:** the structural namesake, row 594 namesake and `print` tests fail, and natively the `foldl` control gives `(6,10,10)`. The helper-namesake control fails too.
  - The holds pass under every mutation. Every restore was byte-identical, and the restored tree passes.
- [x] Commit; replay onto the newest main; judge main against the contract copy; warm gate; record.
  - The source `a6e5b82d` sits on main `71b0dbf9`, the newest at the gate.
  - **Main `71b0dbf9`** was GREEN against the contract copy first (73 movers, 9 holds). All seven movers failed there exactly as measured on every engine, and both holds matched GHC (`baseline-71b0dbf9-chirho.log`).
  - **r1, GREEN on `a6e5b82d` (base `71b0dbf9`) in 230.64 s:**
    - 891/891 focused tests;
    - no Clippy diagnostic on the 245 changed lines in 7 files, and a build with no warning;
    - the contract judge GREEN;
    - rustfmt, diff-check and gitleaks clean (1 commit).
- [x] **Review repair (#29728):** the pre-existing `collect_binders_chirho` doc had ended up above the new `is_compiler_internal_name_chirho`, where rustdoc attached it to the classifier. It is back above `collect_binders_chirho`.
  - The change is comment-only, so per #29728 only rustfmt and diff-check reran, and both are clean.
  - Source `30006f6d` differs from the gated `a6e5b82d` only in those two doc lines.
- [ ] Land on main and close progress row 597 (gpt).
