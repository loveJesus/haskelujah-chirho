<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Ord operators at the proved key Chirho

## Origin and scope

Row 610.
- **Reserved:** gpt_chirho #29874 at main `7787ef02`, out of row 609's checkpoint (#29869). Comparisons left row 609, and its loud-to-silent NaN and signed-zero transition waits for this row.
- **Scope (#29874):**
  - The four Ord operators, infix and section, reach the existing body-backed compare route from their occurrence's evidence, not an Int primop or an operand-guess table.
  - A missing or stub compare row stays loud.
  - No inference from pointer representation. Eq `/=` stays out.
- **Branch:** `ordering-operators-chirho`, from main `7787ef02`, replayed onto `0b6d97ca` (row 608).

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/ordering-operators-chirho/probes-chirho`, in `sources-chirho`, `keys-chirho` and `more-chirho`. Each has its GHC outputs and the main and candidate reports.
- **Main `7787ef02`:**
  - Computed Strings in sections and lambdas: silently wrong natively (`filter (< "b") ["a" ++ "", "c"]` gave `[]`), and refused by the interpreter.
  - Lambda-bound and zipped Doubles at NaN and `-0.0`: wrong, or `missing native binding >=`.
  - A derived enum, a program's own `compare`, derived fields, a newtype, `Maybe`, lists and tuples in sections and lambdas: wrong natively, refused by the interpreter.
  - A program's hidden-and-redefined `(<)`, and a `where`-bound `(<=)`: lost to the primop on every engine.
  - First-class and prefix `(<)`, `(<=)`, `(>)` and `(>=)`: `missing binding` on every engine.
- **The candidate:**
  - 12 programs move to GHC on all three engines.
  - 3 holds stay GHC: Int, Char and Bool with compare, extrema and structured Ord; NaN on each side of each operator, and the signed-zero matrix, at annotated Doubles; Integer.
  - Float stays GHC in the interpreter.
- **Unchanged from main, named:**
  - Polymorphic `Ord a =>` code keeps the blind primop: `smallerChirho ("a" ++ "") "b"` gives `False` natively.
  - A stub compare row keeps main's primop for written operators: builtin `()`, Float and Word, and a program instance that defines only `<=`, whose `<` and `>` still compare representations. That instance's own `<=` is now served.
  - First-class at a stub key stays loud.
  - Word and Int64 literals, and `compare` on a `<=`-only instance, fail as before.

## Placement

- **Desugar:** the new `desugar_chirho/ordering_operators_chirho.rs` decides which written operators take evidence, and records their occurrences. `desugar_chirho.rs` gains a field and four call lines; `desugar_chirho/` has 12 entries.
- **Driver:** `lib.rs` passes `operator_occurrences_chirho` to `dict_pass_module_full_with_evidence_chirho`.
- **Dictionary pass:**
  - The new `instance_chirho/ordering_operators_chirho.rs` holds the answers, the pass over bindings and the generated rows; `instance_chirho/` has 12 entries.
  - `rewrite_chirho.rs` runs the generator after the Eq rows, and resolves written operators before its binding loop.
  - `double_counterpart_chirho` is visible to the dictionary pass.
  - `existing_keyed_row_chirho` generalizes row 603's `existing_eq_row_chirho`.
- **Controls:** `tests/dictionary_evidence_chirho/ordering_chirho/ordering_operators_chirho.rs`; `ordering_chirho/` has 8 entries.
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`, after row 604's bullet. Row 551's and R1's stale lines are updated there.

## Checklist

- [x] Measure on GHC and main; trace the desugar tables, the compare route and the evidence dispatch.
- [x] **Route:** written operators resolve as references. Answers at the proved key go in place for written uses, and in rows for first-class ones. Without an answer, the primop comes back.
  - A spine hook missed sections under `filter`, whose typed walker rewrites node by node. Resolving before the binding loop serves every walker.
- [x] The core and typing libraries pass 559/559, and the dictionary-evidence suite 134/134.
- [x] **Row 551's boundary pin** (`orderings_outside_the_compare_route_still_fail_in_the_interpreter_chirho`) caught prefix `(<)` moving. It is now `prefix_less_answers_through_its_compare_row_chirho`, the same program at GHC's `False`.
- [x] Controls: GHC-measured, byte for byte; 5 tests.
- [x] Commit; replay onto the newest main; broad moved-shape scan.
  - The source sits on main `26942425`, which is row 608 plus the row 611 reservation.
  - **Scan** (`scan610-chirho.log`): 3172/3172 passed in 611 s, with no mover. It covered every lib and the 20 driver suites, leaving out `ghc_bulk`.
    - 18 tests were skipped: the 16 `proptest_chirho::eval` properties and row 608's two ignored generator steps.
    - The count is row 604's 3154, plus 5 new controls and 13 new row 608 tests.
- [x] **Reach** (`tools-chirho/reach_610_chirho.zsh`, log `reach610-r1-chirho.log`):
  - **M1, the infix and section origin hook off:** the written-operator and program-owned controls fail; OProgramOwned prints main's `([1],[5],False,[1])`.
    - The M1 CLI, run on every probe set and engine, matches main's reports in 50 cells.
    - The other 16 are first-class and prefix uses served by M2's rows; written cells main never reached because a first-class use failed first, which reproduce its wrong answers; heap addresses one apart; and main's truncated ODoubleSides report.
  - **M2, the generated rows off:** prefix `(<)`, the first-class controls and OStringInfix's `zipWith (<)` fail with `missing STG binding <`. An instance's own `(<)` row still serves.
  - Both restores were byte-identical, and the restored tree passes 7/7.
- [x] Judge main against the contract copy; warm gate; record.
  - **Main `26942425`** is GREEN against the contract copy first (73 movers, 9 holds). Its probes give main's measured answers (`baseline-26942425-chirho.log`).
  - **r1, RED at the changed-line Clippy step:** `enum_variant_names` on the new answer enum, whose three variants all end in `Chirho`. Its two row-backed variants became one row over both operands with an optional Ordering test, and the source commit was amended.
  - **r2, GREEN on `87952912` (base `26942425`) in 257.35 s:**
    - 1277/1277 focused tests;
    - no Clippy diagnostic on the 574 changed lines in 12 files;
    - the contract judge GREEN;
    - rustfmt, diff-check and gitleaks clean (1 commit), and a build with no warning.
  - The tip's probe reports are under each probe set's `tip-87952912-chirho`.
- [x] Land on main and close progress row 610 (gpt).
  - Fast-forwarded tested tip `6ccdd252` onto main.
  - Independent landing check: `cargo test -p haskelujah-driver --test dictionary_evidence_chirho` passed 134/134; `cargo fmt --all -- --check`, `git diff --check 26942425..6ccdd252` and gitleaks over the two-commit range were clean.
