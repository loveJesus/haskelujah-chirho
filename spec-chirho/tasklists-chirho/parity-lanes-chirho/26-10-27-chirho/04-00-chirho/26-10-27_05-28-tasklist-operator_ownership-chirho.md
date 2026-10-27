<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# An owned operator keeps its binding Chirho

## Origin and scope

Row 614.
- **Reserved:** gpt_chirho #29990, for the float-literal infix shortcut found in row 609's checkpoint (#29989).
- **Widened by #30035:**
  - One ownership predicate runs before all three shortcut families: the infix chain, the section table, and prefix `div`, `mod`, `quot` and `rem`.
  - It covers an unqualified operator bound at top level or locally, and a self-qualified one naming the module's top-level binding.
  - Otherwise the Prelude shortcut stays exactly.
- **Narrowed by #30041:** infix `$`, which the parser rewrites before naming, is row 619.
- **Out of scope:** the definition forms `a - b = …` (row 617) and `(f . g) x = …` (row 618). An operator imported from a user module stays the unlinked boundary.
- **Branch:** `operator-shadowing-chirho`, from main `5dd389d1`.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/operator-shadowing-chirho`.
  - `probes-chirho` holds the families and holds.
  - `residual-chirho` holds row 619's `$`.
  - `gaps-chirho` holds rows 617 and 618.
  - Each has GHC outputs and the main and candidate reports.
- **Main `166bf535`:** silently wrong on every engine whenever the program owns the operator.
  - The float-literal `+ - * /`, top-level and `where`-bound;
  - `**` and `^`; `&&` and `||`; `$!`, `$!!` and `.`;
  - `/=` and `++`;
  - `div` and `mod` infix, and `div`, `mod`, `quot` and `rem` prefix;
  - sections of `++`, `^`, `**` and `/=`;
  - self-qualified `Main.&&` and `Main.div`.
- **The candidate:**
  - **12 programs move to GHC on all three engines:** 36 cells.
  - **Holds:** the Prelude infix and section sets, a program-owned `>>=` and `>>`, a backticked `div` section, and a multi-name operator signature.
- **Unchanged, named:**
  - a program-owned infix `$`: `(+ 1) $ 5 = 6`, where GHC gives `7` (row 619);
  - `a - b = …`, which fails with an infinite type (row 617);
  - `(f . g) x = …`, which leaves `.` unbound (row 618).

## Placement

- **The predicate:** `desugar_chirho/qualified_references_chirho.rs`. `names_owned_binding_chirho` and `is_owned_binding_id_chirho` sit beside row 598's resolver and share its own-top-level lookup.
- **Sites:** `desugar_chirho.rs`, at the infix arm's head, the section table and the prefix division rewrite.
- **Refactor:** the infix and section fallbacks share `written_operator_application_chirho` (`desugar_chirho/operator_evidence_chirho.rs`), so `desugar_chirho.rs` shrinks.
- **Controls:** `tests/typing_integration_chirho/operator_ownership_chirho.rs`, beside row 598's controls (`typing_integration_chirho/` has 7 entries).
- **Workflow:** `compiler-pipeline-chirho/value-scope-resolution-chirho.md`.

## Checklist

- [x] Trace every shortcut (#30034); ruling #30035; checkpoint on `$` (#30040), ruling #30041.
- [x] Source: the predicate at the three sites, and the shared reference route.
- [x] Controls: GHC-measured, byte for byte; 4 tests, with `$` excluded.
- [ ] Commit; replay onto the newest main; broad moved-shape scan.
- [ ] Reach: the predicate removed at each site alone.
- [ ] Judge main against the contract copy; warm gate; record.
- [ ] Land on main and close progress row 614 (gpt).
