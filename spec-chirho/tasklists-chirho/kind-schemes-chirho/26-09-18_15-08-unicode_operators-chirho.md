<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Unicode operator identity, row484

Continue row484 in the isolated gpt-kind-schemes-chirho worktree. Parent 7834a2ce
is pushed; class-method reduction is separately retained in 8dd8f633.
Main, canonical progress DB and public artifacts remain untouched.

## Evidence and placement before implementation

Claude2 measured T11754 against GHC 9.14.1 and immutable 7b11cea6. His source
matrix distinguishes non-ASCII symbols inside infix data-constructor names
from ASCII and prefix controls. This is not proof that the associated-family
checker causes the damage. The lexer currently consumes ASCII operator bytes
and emits a standalone Unicode symbol separately; verify token boundaries
directly before any production change.

The repair belongs in lexer_chirho/symbols_chirho.rs: one shared character
predicate and maximal-munch scanner, also used by the CST's qualified-operator
classification. Keep UTF-8 byte spans and reserved punctuation boundaries.
Do not add constructor-specific kind guards or hard-code a list of corpus
operators. Keep scanning linear, with constant-time character classification.

Review refinement: GHC 9.14.1 rejects the four bracket-operator sources that I
predicted it would accept. Its Lexer.Interface.adjustChar classifies Ps/Pe/Pi/Pf
as graphic rather than symbol characters. Follow that measured category policy,
not the broader Haskell2010 report wording or a per-character blacklist. The
compiler also needs to preserve lexical failure: the old CST mapped an Error
token to VarId and the frontend accepted all four sources even after the lexer
classified them correctly. Add the parser child lexical_errors_chirho.rs and a
compiler-facing parse_lexically_checked_chirho entry, reusing the existing token
stream and rejecting before lowering. Keep recovery parsing for tooling and
do not claim general grammar-error propagation.

Dependency choice: unicode-general-category 1.1.0, exact pin and renamed crate
key with Chirho suffix. Official docs.rs lists release 2025-09-16, no transitive
dependencies, and a two-level constant-time lookup. Use GHC's Unicode symbol
categories plus connector/dash/other punctuation, retaining Haskell's ASCII
special-character treatment; arbitrary non-letters are not symbols. Sources checked 2026-09-18:
https://docs.rs/crate/unicode-general-category/1.1.0
https://docs.rs/unicode-general-category/1.1.0/unicode_general_category/enum.GeneralCategory.html

## Checklist

- [x] Demonstrate mixed-Unicode operator token identity red on the current lexer.
- [x] Retain GHC source controls for constructor use, qualified use, punctuation
 boundaries and the reduced associated-family case; label invalid probes.
- [x] Repair shared symbol scanning and qualified-token classification.
- [x] Run exact output/check controls, parser suite, typing integration,
 canaries and unchanged T11754; preserve genuine negative controls.
- [x] Format and inspect Clippy's primary spans on changed/new lines.
- [ ] Inspect exact owned diff, commit and push, then verify the remote tip;
 no corpus-neutrality or full landing claim without its gate.

## Measured result

The lexer now consumes the complete symbol spelling before classifying reserved
punctuation. Its symbol predicate is shared with the CST; qualified local names
retain embedded dots. This repairs T11754 without an associated-family guard.
The independent GHC controls also found three accepted wrong answers on the
immutable parent: `->⊗`, `→⊗` and `∀→` printed 19 instead of 42. All thirteen
single-module execution controls now print the exact GHC output, 42.

Four bracket controls were invalid GHC programs, refuting my initial prediction.
Their raw predictions and GHC-21231 results are retained. Matching GHC's category
rule was insufficient on its own: the CST recovery path fabricated variables
from lexer errors. The compiler now rejects the first lexical error as E0001,
before lowering, through its shared frontend. Recovery-only header/tooling
parsing stays available; general grammar-error propagation and malformed-header
dependency scheduling are not claimed.

The first full parser gate caught an introduced UTF-8 slicing panic on malformed
`'🌀`. Checked access repaired it; the minimized input is a permanent deterministic
control. The final parser gate includes that repair. Lexer source separation
reduces its root from 2,060 to 1,107 lines; the scanner is 164 lines and the moved
unit tests are 694 lines. The CST root remains oversized inherited debt.

Final gates, RUST_MIN_STACK=16777216, -j3:

- Parser 372 unit + 12 integration, naming 138, typing 407.
- Driver typing integration 442, canaries 7, evaluation-values 4 (including the
  thirteen-source GHC output table). All these targets have zero ignored/filtered.
- Live constraints-package regression: 1 passed, 1775 filtered, 64.20 seconds;
  constraints.cabal was present before and after. This is not a full driver gate.
- CLI build and format passed. Clippy emitted 625 warning messages, with zero
  primary spans on changed/new lines. The tree is NOT lint-clean.

The final immutable CLI was hashed before and after 49 focused observations:

`/private/tmp/haskelujah-unicode-final-chirho.tIfxlU/haskelujah-chirho`

SHA-256: `650be5380c3e7d9bf3efb537add5de6b3135bb7cda1a845281e5560f00f83e32`.

Durable evidence is under
`test-data-chirho/kind-oracles-chirho/ascriptions-chirho/imported-families-chirho/source-boot-chirho/declarations-chirho/classes-chirho/default-annotations-chirho/scope-chirho/corpus-chirho/unicode-operators-chirho/`:
28 GHC reference graphs (15 initial, 7 category/keyword, 5 qualified, unchanged
T11754), exact source fixtures, final CLI replay and focused-gate receipts.
Qualified graphs are tested through compile_modules; the earlier single-module
CLI run attempts that report a missing provider are labelled unusable for that claim.

## Limits and next work

The separate infix-constructor runtime problem remains: the ASCII `:^:` control
already fails on the parent, and `:⊗:` / `:×:` now typecheck but also report a
missing STG binding at execution. This checkpoint does not repair that path or
claim complete Unicode identifier/extension behavior or native-backend parity.

ClassDefaultInHsBoot/A2/A3 and T14441 remain independent open reductions. T13585a
and T10808 still accept in the focused replay. No new complete corpus measurement
was made; selected fixes cannot establish the current whole-branch regression
count. Full driver/workspace and paired corpus gates are still owed before a
main landing. Main, public artifacts and the canonical progress DB stay untouched.
