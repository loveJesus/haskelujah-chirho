<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Eq's method set and defaults Chirho

## Origin and scope

Row 606.
- **Reserved:** gpt_chirho #29763, from row 603's `/=` trace, behind rows 608, 609 and 614.
- **Approved by #30062, on checkpoint #30061:**
  - `/=` joins the seeded Eq, and every dictionary and selector follows the sorted layout `[/=, ==]`.
  - The defaults go both ways: `/= = not . ==` and `== = not . /=`.
  - A default comes only from a real body-backed counterpart; a stub is never a donor.
  - A key with neither body keeps today's named failure, pinned.
  - Ord defaults and other class layouts are out of scope (row 612).
- **Branch:** `eq-not-equal-chirho`, from main `28cd9cf4`.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/eq-not-equal-chirho/probes-chirho`, in `sources-chirho`, `sections-chirho`, `division-chirho` and `defaults-chirho`. Each has GHC outputs and the main and candidate reports.
- **Main `28cd9cf4`:**
  - **Silently wrong:**
    - a user instance's own `(/=)`: `(False,False)`, where GHC gives `(True,False)`;
    - `/=` sections natively at computed Strings, NaN and `-0.0`;
    - an instance defining only `==`, or only `/=`.
  - **Loud:** first-class `(/=)`; `/=` at pairs and `Maybe`; a derived enum's `/=` in the interpreter.
- **The candidate:**
  - **8 programs move to GHC on all three engines:** both methods defined; only `==`; only `/=` (also through `elem` and `lookup`); first-class `(/=)`; structured infix; sections; a derived enum; and generic heads writing one method.
  - **Holds:** infix `/=` at Int, Char, Bool, computed Strings and NaN; Eq through polymorphic code and Ord's superclass dictionary; the 609 section sets.
  - **Neither body:** each method keeps its named failure. `==` names `Eq.==` as before; `/=` names `Eq./=`, where main named `Eq.==`. GHC loops.
  - **Residual:** `subtract 1` stays `no matching alternative`.

## Placement

- **Typing:** `class_chirho.rs` seeds Eq with both methods.
- **Dictionary pass:**
  - The new `instance_chirho/equality_defaults_chirho.rs` holds the default rows and the structured-key step; `instance_chirho/` has 13 entries.
  - The dictionary resolver and the conditional list path ask it before a stub, and the list Eq builder for its `/=`.
  - A generic head's G5 coverage counts an Eq method whose counterpart has a source row. The donor is read under the written spelling.
- **Desugar:** the infix `/=` rewrite is removed, and `/=` joins `SECTION_METHOD_OPERATORS_CHIRHO`.
- **Controls:**
  - `tests/dictionary_evidence_chirho/ordering_chirho/equality_methods_chirho.rs`; `ordering_chirho/` has 10 entries.
  - The layout and selector units in `dict_pass_tests_chirho.rs`.
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`, with its stale `/=` lines updated.

## Checklist

- [x] Trace and checkpoint #30061; approved #30062.
- [x] Source: the method, the defaults both ways from real bodies only, and the desugar route.
- [x] Controls: GHC-measured, byte for byte; the layout and selector units updated.
  - **First suite run:** two generic-head tests failed. G5 required a source row for every slot, so an instance writing only `==` stopped being generic. G5 now counts the Eq counterpart.
  - Row 603's structured `/=` pin moved, as its label anticipated.
- [x] **Rerun row 608's generator and drift checks** (`generator608-chirho.log`): the outputs are byte-identical, 726 agree and 679 disagree, so the delta is none. The 18 qualified-alias, builtin-alias and reference tests pass.
- [ ] Commit; replay onto the newest main; broad moved-shape scan.
- [ ] Reach: the `/=` method entry; the default generator; the desugar route.
- [ ] Judge main against the contract copy; warm gate; record.
- [ ] Land on main and close progress row 606 (gpt).
