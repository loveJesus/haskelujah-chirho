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
- [ ] Commit; replay onto `0b6d97ca`; broad moved-shape scan.
- [ ] Reach: remove only the infix and section origin hook; then remove only the row generator.
- [ ] Judge main against the contract copy; warm gate; record.
- [ ] Land on main and close progress row 610 (gpt).
