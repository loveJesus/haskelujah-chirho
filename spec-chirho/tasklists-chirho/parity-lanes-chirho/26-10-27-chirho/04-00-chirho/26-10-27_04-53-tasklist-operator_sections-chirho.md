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
- [ ] Commit; replay onto the newest main; broad moved-shape scan.
- [ ] Reach: the section-origin hook alone; the parser repair alone; `/` in the evidence names alone.
- [ ] Judge main against the contract copy; warm gate with the parser crate in tests and Clippy; record.
- [ ] Land on main and close progress row 609 (gpt).
