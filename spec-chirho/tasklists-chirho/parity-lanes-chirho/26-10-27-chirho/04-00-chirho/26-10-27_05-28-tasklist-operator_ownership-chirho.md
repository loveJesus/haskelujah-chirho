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
- [x] Commit; replay onto the newest main; broad moved-shape scan.
  - The source `f4056ccc` sits on main `b83c7491`, which adds only the row 619 reservation.
  - **Scan** (`scan614-chirho.log`): 3579/3579 passed in 719 s, with no mover.
    - It covered every lib, the parser's included, and the 20 driver suites, leaving out `ghc_bulk`.
    - It skipped the same 18 tests as before. The count is row 609's 3575, plus this row's 4 controls.
- [x] **Reach** (`tools-chirho/reach_614_chirho.zsh`, log `reach614-r1-chirho.log`). Each site was removed alone and gave main's answers there:
  - **M1, the infix arm's head:** `(3.5,-0.5,3.0,2.0,3.5)` and `(3.5,2.0)`. Infix `div` and `mod` revert while prefix division stays exact.
  - **M2, the section table:** `([[1,9]],[8],[9.0],[2],[9])`.
  - **M3, the prefix rewrite:** `(9,14,3,1,3,1)`, and the local prefix cells revert.
  - Every restore was byte-identical, and the restored tree passes 4/4. The Prelude holds passed under every mutation.
- [x] Judge main against the contract copy; warm gate; record.
  - **Main `b83c7491`** is GREEN against the contract copy first (73 movers, 9 holds). Its probes give main's measured answers (`baseline-b83c7491-chirho.log`).
  - **r1, GREEN on `f4056ccc` (base `b83c7491`) in 331.72 s:**
    - 1746/1746 focused tests, with `typing_integration_chirho` added;
    - no Clippy diagnostic on the 254 changed lines in 5 files;
    - the contract judge GREEN;
    - rustfmt, diff-check and gitleaks clean (1 commit), and a build with no warning.
  - **Timing:** past the 2-5 minute aim, because the whole typing-integration binary joined the tests step.
  - **The tip's probe reports** (`tip-f4056ccc-chirho` in each set): 16 programs are GHC on all three engines. All 18 programs that compile carry the Cranelift marker; rows 617 and 618's two gaps fail type checking.
- [x] Land on main and close progress row 614 (gpt).
  - Fast-forwarded the tested tip `30d580a6` onto main: source `f4056ccc`, gate record `30d580a6`.
  - Independent landing checks: operator-ownership controls 4/4, qualified-reference seam 5/5, rustfmt, diff-check and gitleaks clean.
