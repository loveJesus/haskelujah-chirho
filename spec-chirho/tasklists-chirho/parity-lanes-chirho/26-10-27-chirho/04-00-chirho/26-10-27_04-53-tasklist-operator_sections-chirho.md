<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Operator sections take their evidence Chirho

## Origin and scope

Row 609.
- **Reserved:** gpt_chirho #29863, from row 606's trace (#29861).
- **Rulings:**
  - #29874, on checkpoint #29869:
    - `/` joins the occurrence population;
    - comparisons leave for row 610;
    - the NaN and signed-zero transition waits for row 610;
    - the program-owned `(+)` is a mover.
  - #29990, on checkpoint #29989: the qualified operator-section parser repair is absorbed. The float-literal infix shortcut is row 614.
- **Scope:**
  - Sections of `+`, `-`, `*`, `/` and `==` take the written operator's evidence, by generalizing R1's origin route; row 610 already routes the comparisons.
  - Without evidence, today's fallback stays.
  - `/=` is row 606. `subtract`, `++`, `^` and `**` are pinned as they are.
- **Branch:** `operator-sections-chirho`, from main `263a3b9b` (after rows 610 and 611).

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/operator-sections-chirho/probes-chirho`, in `sources-chirho`, `infix-chirho`, `divide-chirho` and `qualified-chirho`. Each has GHC outputs and the main and candidate reports.
- **Main `263a3b9b`:**
  - Double `+`, `-` and `*` sections: `[4.49423283715579e307]` and `[0.0]` natively, and `AddIntChirho` in the interpreter.
  - `/` sections, infix, `zipWith (/)`, `foldr (/) 1` and `(/) 1`: `missing binding /`.
  - A program-owned `(+)` in sections: `([5],[7],20)`, where GHC gives `([6],[10],20)`.
  - Qualified sections: wrong or loud. `(2 P.-)` gives `[P.- 2 0.5]` in the interpreter and a denormal natively.
- **The candidate, GHC on all three engines:**
  - **Movers:** the Double arithmetic sections; the program-owned `(+)`; the NaN and signed-zero comparison line under `0 / 0`; concrete Double `/`; and qualified `+`, `-`, `*` and `/` sections, both directions, at Int and Double, nested ones included.
  - **Holds:** R1's `==`, Int and Char, `++`, `^` and `**`, row 610's comparisons, a dictionary-polymorphic Fractional, and a program-owned `(/)`.
  - **Fails closed:** a forced `/` at Float names `missing method Fractional./ for Float` on every engine.
  - **Residual:** `subtract 1` stays `no matching alternative`.
- **Row 554's iterate residual** answers `[1.5,3.0,6.0,12.0]` everywhere, and is re-pinned.

## Placement

- **Desugar:** `desugar_chirho/ordering_operators_chirho.rs` became `operator_evidence_chirho.rs`.
  - One predicate holds `SECTION_METHOD_OPERATORS_CHIRHO`, for R1's `==` and `+`, `-` and `*`, beside row 610's Ord operators.
  - R1's table guard in `build_section_body_chirho` folds into it.
- **Driver:** `/` joins `EVIDENCE_METHOD_NAMES_CHIRHO`.
- **Parser:** both right-section fallback sites in `lower_chirho.rs` split qualified operator names with `name_from_text_chirho`.
  - The units are in `lower_chirho/expr_syntax_chirho/qualified_sections_tests_chirho.rs`.
  - `lower_chirho.rs` holds 16,234 lines, and `desugar_chirho.rs` 5,465: structural debt, named and not grown here.
- **Controls:** `tests/dictionary_evidence_chirho/ordering_chirho/operator_sections_chirho.rs` (`ordering_chirho/` has 9 entries), beside row 554's section controls. `dictionary_evidence_chirho/` already holds 14 entries.
- **Workflows:** `dictionary-evidence-chirho.md` and `value-scope-resolution-chirho.md`.
- **This shard:** gpt's new `04-00-chirho`. The day directory held 15 files before it.

## Checklist

- [x] Re-measure on main `263a3b9b`. Row 610 made the comparison sections exact.
- [x] Source: the predicate, `/` in the evidence names, and the parser fallback repair (after #29990).
- [x] Controls: GHC-measured, byte for byte. Parser units for the direct and folded paths, nested qualified operators included.
- [x] Commit; replay onto the newest main; broad moved-shape scan.
  - The source `74a7e5ce` sits on main `ebd47b78`, which adds only the row 613 and 614 reservations.
  - **Scan** (`scan609-chirho.log`): 3575/3575 passed in 525 s, with no mover. It covered every lib, the parser's now included, and the 20 driver suites, leaving out `ghc_bulk`.
    - 18 tests were skipped: the 16 `proptest_chirho::eval` properties and row 608's two ignored generator steps.
    - The count is row 610's 3172, plus the parser's 390, this row's 4 parser units and 5 controls, and row 611's 4 controls.
- [x] **Reach** (`tools-chirho/reach_609_chirho.zsh`, log `reach609-r1-chirho.log`):
  - **M1, the section-origin hook off:** the movers, holds, qualified and iterate controls fail.
    - The M1 CLI matches main's reports in 37 cells. Its wrong arithmetic answers (`4.49e307`, `4.45e-308`, `0.0`) include those main never printed because it failed first on `/`.
    - R1's `==` sections revert too, since R1's rule now rides the predicate.
    - The other differing cells are the `/` and parser mechanisms.
  - **M2, the parser's trailing-operator site reverted:** the operand-first and nested parser units read `(None, "P.-")`. The qualified control prints main's `[P.+ 1 1]` and `[P.- 10 3]`.
  - **M3, `/` out of the evidence names:** the fail-closed, `/` and qualified controls fail.
  - **M4, the parser's inner-operator site reverted:** the nested unit reads `(None, "P.+")`, and the qualified control fails.
  - Every restore was byte-identical, and the restored tree passes 4/4 parser units and 6/6 controls.
- [x] Judge main against the contract copy; warm gate with the parser crate in tests and Clippy; record.
  - **Main `ebd47b78`** is GREEN against the contract copy first (73 movers, 9 holds). Its probes give main's measured answers (`baseline-ebd47b78-chirho.log`).
  - **r1, GREEN on `74a7e5ce` (base `ebd47b78`) in 245.86 s:**
    - 1676/1676 focused tests, the parser library included;
    - no Clippy diagnostic on the 360 changed lines in 9 files, with the parser linted;
    - the contract judge GREEN;
    - rustfmt, diff-check and gitleaks clean (1 commit), and a build with no warning.
  - **The tip's probe reports** (`tip-74a7e5ce-chirho` in each set): 16 programs are GHC on all three engines (48/48 cells), and all 20 Cranelift compiles carry the marker.
- [ ] Land on main and close progress row 609 (gpt).
