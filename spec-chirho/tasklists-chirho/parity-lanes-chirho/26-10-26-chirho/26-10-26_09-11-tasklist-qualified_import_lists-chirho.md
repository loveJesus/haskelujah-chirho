<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Qualified import lists bind their qualifier (row 536) Chirho

L.J.'s direction is GHC test passing. Row 531's r3 freeze holds one independent exposure, `I2_import_list_only_head_chirho`, owned IMPORT-SCOPE (#28456, #28473):

```haskell
import qualified Data.List.NonEmpty as NE (head, NonEmpty(..))
main = print (NE.toList (1 NE.:| [2 :: Int]))
```

GHC 9.14.1 rejects it with GHC-76037: `NE.toList` is outside the explicit import list. Main and the replay both accept the check. On main the run fails on the unbound `:|`; on the replay it newly prints `:| 1 [2]`.

The trace (#28482) found the cause:
- Naming builds the correct qualified table, without `NE.toList`, but never checks value occurrences against it.
- Typing's fallback then drops the qualifier and reads the bare `toList` (`infer_chirho.rs` 2713-2715 on the replay).

Owner: claude_chirho writes source only, on `qualified-import-lists-chirho` from the exact frozen replay tip b709644c (gpt_chirho #28483, with the addenda #28488 and #28493).
- gpt_chirho opened row 536 (DB-only commit a674260f) and owns the builder, main and the DB.
- No Cargo or CLI runs before the reviewed source checkpoint.

**Row 536 is not independently landable.** It must keep the frozen 531 pairing intact and later enter the combined 531+536+517 gate.

## Contract

- **The rule (INV-I2).** It applies to a qualified value occurrence `Q.x` in an expression: a variable, an operator, or a constructor in an expression or record construction.
  - **Closed qualifier.** Q is closed only when every import whose effective qualifier is Q is an explicit, non-hiding list, and every item of that list is represented. Multiple explicit lists contribute their union.
  - **Open qualifier.** An open import, a `hiding` import, or a list holding an unrepresented item leaves Q open, under today's behavior.
  - **What may supply x under a closed Q:**
    - a directly listed name, even when the interface omits it;
    - a member written in a listed `T(…, x, …)`;
    - a member of a listed `T(..)` whose member list is proven complete.
  - Only two kinds of member list are proven complete: those from source-derived interfaces, and the builtin `NonEmpty`. Any other `T(..)` keeps Q open.
  - **Current module.** If Q is the current module's own name, its own top-level values stay in scope.
  - **Otherwise.** The occurrence is a scope error (E0100, GHC-76037's class). Typing's fallback and desugar are not changed.
- **The syntax classifier.** It sits beside `ImportDeclChirho` in `haskelujah-ast-chirho`. It answers only from syntax and never says whether a value is exposed.
  - **The effective qualifier:** the alias, otherwise the module name.
  - **UNRESTRICTED:** no spec, or an explicit list with any unrepresented item. It makes no exclusion claim.
  - **HIDING(items):** a `hiding` spec, with every written item kept.
  - **EXPLICIT(items):** a non-hiding list, only when every item is represented.
  - Row 536 closes only EXPLICIT. Interface lookup and `T(..)` completeness stay in naming.
- **The marker.** The lowering produces `ImportItemChirho::UnrepresentedChirho` for items it cannot represent:
  - nameless namespace wildcards (`data ..`, `type ..`);
  - `pattern P`, which was misread as a variable named `pattern`.
  - It imports nothing, hides nothing and re-exports nothing, as when it was dropped.
  - Every exhaustive consumer handles it: the name extraction, the export merge, the export removal, and the classifier's own "represented" test. None has a wildcard arm, so a new item form fails to compile until it is classified.

## Boundaries, named and not claimed

- Constructor patterns and record field labels.
- The operations a do statement selects (`M.do`'s `M.>>=`, and a reserved `M.fail`).
- Unimported qualifiers (I6, and row 532's Q01/Q02).
- Bare-seed over-exposure (I8).
- Shared-alias ambiguity (I3, I3b, M1).
- Builtin interfaces that are not proven complete (`Semigroup(..)`, `RWST(..)`, and others). Their `T(..)` keeps the qualifier open.
- Two source-derived exceptions, which are left open:
  - a data type with a GADT constructor (its record fields are not lowered);
  - every type of a module that declares or enables pattern synonyms (a bundled `T(.., P)` loses `P`).
- Diagnostic bytes are this checker's, not GHC's.

## Controls (predictions until real logs)

GHC 9.14.1 receipts are measured from the committed sources.

- **Negatives, GHC-76037:**
  - I2;
  - `import Data.Monoid as M (getAll)` with `M.getSum` (the GHC `rename/should_fail/T19843i` shape);
  - `import qualified Prelude (map)` with `Prelude.Left` (the GHC `module/mod88` shape: a constructor expression);
  - an empty list `M ()` with `M.size`;
  - an unlisted operator, `M.!`;
  - two lists under one alias, with `L.head`;
  - `import Data.List (sort)` with `Data.List.nub`.
- **Positives:**
  - a listed variable;
  - `NonEmpty(..)` supplying `NE.:|`;
  - an open alias;
  - `hiding`;
  - an open full import;
  - a listed name under the full-module qualifier;
  - an explicit list plus an open import under one alias;
  - two lists under one alias;
  - `NE (reverse)`, missing from the interface;
  - `Semigroup.sconcat` through the partial builtin `Semigroup(..)`;
  - `Lazy.runRWST` through the partial builtin `RWST(..)`;
  - a listed operator;
  - the implicit `Prelude.map`;
  - `Main.foo`;
  - `D.Proxy` through an unrepresented `(data ..)`;
  - a source-derived record `X(..)` supplying `X.name`.
- **Structural:**
  - the classifier's three modes (an explicit list with an unrepresented item is UNRESTRICTED; `hiding` keeps its items);
  - the producer's marker;
  - the consumers importing, hiding and re-exporting nothing for it;
  - completeness marks: local data, newtype and class are marked; GADT, pattern synonyms and re-exports are not; builtin NonEmpty is marked; merging unions the marks.

## Mutants (predicted kill sets, credited only by real logs)

| Mutant | Change | Predicted kill set |
|---|---|---|
| M1 | the check never fires | every negative, and I2 back to OVERACCEPT |
| M2 | constructor occurrences skipped | the `Prelude.Left` negative |
| M3 | `as` ignored | the alias negatives |
| M4 | any list import closes the qualifier | the list-plus-open positive |
| M5 | `hiding` treated as a list | the `hiding` positive |
| M6 | a data `T(..)` supplies nothing | the `NonEmpty(..)` and `NE (reverse)` positives |
| M7 | a builtin class `C(..)` closes on its method list | `Semigroup.sconcat`, plus the driver parsec tests |
| M8 | the interface-filtered table replaces the written list | `NE (reverse)` |
| M9 | the marker ignored | the `(data ..)` positive |
| M10 | every builtin assumed complete | `Lazy.runRWST` |

## Candidate gate contract (#28483)

- The pairing is the frozen CLI 260780d0 against the candidate.
- Among r3's A, S, B and C, exactly I2 may move, from OVERACCEPT to a deterministic GHC-class rejection.
- I6 and row 532's Q01/Q02 stay held.
- The corpus and execution stay identical.
- Any other measured movement is RED.
- No signal, timeout, panic, overflow or non-start counts as failing closed.

## Checklist

- [x] Write the tasklist on a fresh branch from exact b709644c.
- [ ] AST: the marker, the item's "represented" test and the syntax classifier, with structural tests. Naming: the exhaustive consumers.
- [ ] Parser: produce the marker for nameless items and `pattern P`, with producer tests.
- [ ] Naming: completeness marks (source-derived local types, the builtin NonEmpty, union on merge), with tests.
- [ ] Naming: the value-scope module and its own test file, hooked after type scope.
- [ ] Driver: the GHC-measured behavioral controls, with GHC 9.14.1 receipts from the committed sources.
- [ ] Workflow: `compiler-pipeline-chirho/value-scope-resolution-chirho.md`.
- [ ] Hygiene with no Cargo: rustfmt as a stdin no-op, `git diff --check` and gitleaks. Record the source-derived reach scan as a prediction.
- [ ] Post the tracked source checkpoint before any builder request.
