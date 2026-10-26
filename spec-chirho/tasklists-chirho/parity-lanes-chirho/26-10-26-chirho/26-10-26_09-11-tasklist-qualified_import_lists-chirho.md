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
  - **A bare `T` supplies no value**, not even a constructor spelled `T` (GHC-01928, #28506).
  - Only two kinds of member list are proven complete: those from source-derived interfaces, and the builtin `NonEmpty`. Any other `T(..)` keeps Q open.
  - **Marks merge per type** when copies of one module merge (#28506). A type both copies export stays complete only when both mark it. A type only one copy exports keeps its mark, so disjoint marks still union.
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
  - `pattern P`, which was misread as a variable named `pattern`;
  - a `data` item with a member list or other suffix after its name (#28517), which lowering used to read as a type item.
  - It imports nothing, hides nothing and re-exports nothing. For the wildcards and `pattern P`, that is what they did before.
- **A named `data x` item is a value item** (#28515, #28517). Lowering reads `data Proxy`, `data f` and `data (:+)` as `VarChirho`, whatever the spelling, since GHC 9.14.1 imports the value-level name and never the same-named type. The imported spelling is the replay's, and the list stays EXPLICIT. A named `type T` stays a bare type item.
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
- **Negative, GHC-01928 (#28506):** `import qualified Data.Proxy as P (Proxy)` with `P.Proxy`. A bare type item supplies no constructor of the same name.
- **Negative, GHC-76037 (#28517):** `import qualified Data.Proxy as P (data Proxy)` with `x = (P.Proxy, P.asProxyTypeOf)`. Only `P.asProxyTypeOf` is out of scope: the `data` item supplies `P.Proxy`, and the list stays closed.
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
  - `Seq.Empty` through an unrepresented `pattern Empty` item (this replaced the planned `D.Proxy` through `(data ..)`, see the record below);
  - `P.Proxy` through a `data Proxy` item (#28517);
  - a source-derived record `X(..)` supplying `X.name`.
- **Structural:**
  - the classifier's three modes (an explicit list with an unrepresented item is UNRESTRICTED; `hiding` keeps its items);
  - the producer's marker;
  - the consumers importing, hiding and re-exporting nothing for it;
  - completeness marks: local data, newtype and class are marked; GADT, pattern synonyms and re-exports are not; builtin NonEmpty is marked;
  - copies of one module merge marks per type: disjoint marks survive, a type every copy marks stays marked, and an unmarked copy clears the mark in any order (#28506);
  - a bare `T` of a source `data T = T` supplies no `T`, while `T(..)` does (the shape of GHC's T25877, #28506);
  - the producer's `data` items (#28517): `data Proxy`, `data f` and `data (:+)` are value items and `type Proxy` a bare type item, in an EXPLICIT list; `data T(..)` and `data T(MkT)` are unrepresented.

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
| M9 | the marker ignored | the `pattern Empty` positive |
| M10 | every builtin assumed complete | `Lazy.runRWST` |
| M11 (proposed) | a bare `T` item supplies the value `T` again | the `P.Proxy` negative and the bare source `T` naming test |
| M12 (proposed) | marks merge as a union again | the unmarked-copy cases of the per-type merge test |
| M13 | a constructor-headed `data` item read as a type item again | the `data Proxy` positive, the paired negative's exact messages, and the producer's `data Proxy` test |

M11 and M12 are proposed with the #28506 corrections. They join the gate's mutant set only if gpt_chirho accepts them. M13 was ruled in #28517; its partition is credited only from real logs.

## Candidate gate contract (#28483)

- The pairing is the frozen CLI 260780d0 against the candidate.
- Among r3's A, S, B and C, exactly I2 may move, from OVERACCEPT to a deterministic GHC-class rejection.
- I6 and row 532's Q01/Q02 stay held.
- The corpus and execution stay identical.
- Any other measured movement is RED.
- No signal, timeout, panic, overflow or non-start counts as failing closed.

## Checklist

- [x] Write the tasklist on a fresh branch from exact b709644c.
- [x] AST: the marker, the item's "represented" test and the syntax classifier, with structural tests. Naming: the exhaustive consumers (b31d36c2).
- [x] Parser: produce the marker for nameless items and `pattern P`, with producer tests (966ff5b1).
- [x] Naming: completeness marks (source-derived local types, the builtin NonEmpty, union on merge), with tests (56c23a6b). The union merge was unsound; e6155ce9 replaces it per type.
- [x] Naming: the value-scope module and its own test file, hooked after type scope (2a282abd).
- [x] Driver: the GHC-measured behavioral controls, with GHC 9.14.1 receipts from the committed sources (ce46c757).
- [x] Workflow: `compiler-pipeline-chirho/value-scope-resolution-chirho.md` (7b204b91).
- [x] Hygiene with no Cargo: rustfmt as a stdin no-op, `git diff --check` and gitleaks. Record the source-derived reach scan as a prediction.
- [x] Post the tracked source checkpoint before any builder request (#28504; not accepted in #28506).
- [x] #28506 correction 1: a bare type item supplies no value, with a source-derived naming negative and the GHC-measured `Data.Proxy` driver negative (f95620ac).
- [x] #28506 correction 2: completeness marks merge per type, with merge controls (e6155ce9).
- [x] #28506 docs: the workflow and this tasklist state both rules and the corrected count.
- [x] Hygiene re-run with no Cargo: rustfmt as a stdin no-op, `git diff --check` and gitleaks. GHC 9.14.1 ran only on the new committed control (r3).
- [x] Post the corrected source checkpoint (#28514; both corrections accepted in #28515, with one new blocker).
- [x] #28515/#28517: a named `data x` item is a value item, and a `data` item with a member list is unrepresented, with producer tests (c6ac9d95).
- [x] #28517: the GHC-measured `data Proxy` positive and its paired negative (15bc256a), with GHC 9.14.1 receipt r4 from the committed file.
- [x] #28517 docs: the superseded boundary claim is removed, and the workflow and this tasklist state the rule.
- [x] Hygiene and reach re-run with no Cargo.
- [ ] Post the corrected source checkpoint. The candidate gate tooling stays withheld until it is accepted.

## Source checkpoint record

Branch `qualified-import-lists-chirho`, from exact b709644c. No Cargo or haskelujah CLI ran. GHC 9.14.1 ran only on the control programs.

### Hygiene

- rustfmt 1.8.0, on the pinned toolchain 1.93.0, is a stdin no-op on all 12 changed Rust files.
- `git diff --check b709644c HEAD` is clean.
- gitleaks 8.30.1 scanned 7 commits through the workflow commit and found no leaks (the report is `[]`).

### Shape

- The typing and core trees are byte-identical to the replay's (c96a8a10, a8f98083). Typing's fallback and desugar are not changed.
- One path is shared with the replay, `parser/src/lower_chirho.rs`:
  - this unit's hunks there are the test `mod` line and the import-item lowering;
  - the replay's own hunks in that file are module declarations and moved code.
- Every new file is under 1.5k lines, and every touched directory is under 15 entries (`lower_chirho/` is now 14).
- Three files were already over the size limit and grew minimally:
  - `iface_chirho.rs`, from 18,704 to 18,735 lines;
  - `lower_chirho.rs`, from 16,331 to 16,348;
  - `resolve_chirho.rs`, from 1,809 to 1,830.
  - The first two need their own split units.

### GHC 9.14.1 receipts

The programs are decoded from the committed file, and each prediction was fixed before GHC ran.

- **r1: 21 of 22 agreed.**
  - The refuted prediction is a finding: GHC 9.14.1 parses `import qualified Data.Proxy as D (data ..)` as an error (GHC-58481, `parse error on input ‘..’`).
  - So the namespace wildcard is a form from a later GHC testsuite (T25901, copied into ghc-tests-chirho), not a 9.14 form.
  - The marker still keeps such a list open, since lowering accepts it.
  - The control was replaced by a `pattern Empty` item, which GHC 9.14.1 does accept.
  - Before posting, C3's message was rebuilt (0f40eb85 became 966ff5b1) to say the same. The later commits were replayed unchanged, and the tree differs only by that test's doc note.
- **r2: 22 of 22 agree.**
  - 7 programs are rejected with exactly GHC-76037, naming the same references the controls expect.
  - 15 programs are accepted and print the recorded values.

### Source-derived reach scan

This is a lexical prediction, not a measurement, under the ruled rule.

- **Gate corpus:**
  - should_compile: 148 programs have a closed qualifier. 2 references go through one, and both are listed.
  - should_fail: 79 programs have a closed qualifier. 2 references go through one, both listed.
  - So 0 rows are predicted to move.
- **r3 oracle:**
  - A: exactly I2's `NE.toList` is predicted rejected. Its `NE.:|` is supplied by `NonEmpty(..)`.
  - S, B and C: nothing goes through a closed qualifier.
- **Execution:** nothing.
- **Embedded test programs in crates:** the only predicted rejections are this unit's own negatives.
- **Repository `.hs` files:** nothing.
- **Informational only, outside the gate:**
  - The local package cache has 154 references whose acceptance depends on source-derived member lists.
  - The GHC rename suite's T25901 sub-wildcard rows are scanner artifacts, written in post-9.14 syntax.

### Row 536 is not independently landable

It must keep the frozen 531 pairing intact, and it enters the combined 531+536+517 gate.

## Correction record (#28506)

gpt_chirho did not accept the checkpoint (#28504) and found two load-bearing defects. Both are corrected in commits on top of 3e31096d. The eight reviewed commits are unchanged. No Cargo or haskelujah CLI ran.

### The count

The checkpoint post reported +2,040/−23. `git diff --stat b709644c..3e31096d` is 14 paths, +2,099/−23.

### Correction 1: a bare type item supplies no value (f95620ac)

- The value scope admitted a listed `T`'s own name as a value. So `import qualified Data.Proxy as P (Proxy)` supplied `P.Proxy`, which GHC rejects. That insertion is removed: only written members and the members of a complete `T(..)` supply values.
- Controls:
  - the naming test `a_bare_type_item_supplies_no_constructor_of_the_same_name_chirho`: for a source `data T = T`, `Bare (T)` leaves `Bare.T` out of scope while `Full (T(..))` supplies `Full.T`;
  - the driver negative `BareTypeItemChirho.hs`;
  - `NonEmpty(..)` still supplies `NE.:|`.
- GHC 9.14.1 receipt r3 ran only on that new committed control, decoded from f95620ac, with the prediction fixed first: check rc 1, exactly GHC-01928, naming `P.Proxy`. 1 of 1 agrees: `Data constructor out of scope: ‘P.Proxy’`, with GHC's note that the type constructor cannot appear in that position.
- GHC's own rename test T25877 expects GHC-01928 for the source-derived shape (`X (A)` with `X.A`).

### Correction 2: marks merge per type (e6155ce9)

- `merge_module_ifaces_chirho` unioned the marks. A copy exporting a type unmarked, one that may bundle a pattern synonym, left the other copy's mark standing on a list that might be incomplete.
- Marks now merge per type through `merged_complete_member_types_chirho`, read before a copy's types join the entry.
- Controls in `type_exports_tests_chirho.rs`:
  - disjoint marks survive;
  - a type both copies mark stays marked, beside a type only one copy exports;
  - an unmarked copy clears the mark in either order and in a three-copy merge, while the marked copy's other type keeps its mark.
- The single-source driver path merges only the builtin interfaces, plus `Haskelujah.*` stdlib modules when the source imports the stdlib. So the one builtin `Data.List.NonEmpty` block keeps its mark, and I2 still closes.

### Hygiene, re-run with no Cargo

- rustfmt 1.8.0 is a stdin no-op on all 12 changed Rust files.
- `git diff --check b709644c e6155ce9` is clean.
- gitleaks 8.30.1 scanned 10 commits through e6155ce9 and found no leaks (the report is `[]`).
- Sizes after the corrections:
  - `type_exports_chirho.rs` 818 lines, `type_exports_tests_chirho.rs` 770, `value_scope_chirho.rs` 284, `value_scope_tests_chirho.rs` 521, the driver controls 308;
  - `iface_chirho.rs` is now 18,739 lines; it still needs its own split unit.

### Reach scan r2 (a prediction)

The r1 scanner is kept unchanged. r2 reads items as lowering does, and classifies every reference under both the r1 model and the corrected one.

- **Gate corpus and r3 oracle:** nothing changes. I2's `NE.toList` stays the only predicted rejection.
- **Embedded test programs:** only the new `P.Proxy` negative changes.
- **Execution and repository `.hs` files:** nothing.
- **Informational only:**
  - Local package cache: by line inspection, 410 of its 416 changed occurrences are type positions, which the value walk does not visit. One is an import line the lexer misread, and one is a record pattern. The other four are constructor expressions, each supplied by a source-derived `T(..)`.
  - GHC rename and module suites: the only value-position change is T25877's `X.A`. GHC rejects it with GHC-01928, and the corrected rule now does too.
  - One gate-corpus program and four rename-suite programs hold a closed `data X` item. None has a reference through it.

Row 536 remains not independently landable.

## Correction record (#28515, #28517)

gpt_chirho accepted both #28506 corrections in #28515 and found one new blocker. My #28514 had named the `data P` form as an unmeasured boundary; gpt measured it, and it was a real new false rejection.

### The finding

- GHC 9.14.1 accepts `import qualified Data.Proxy as P (data Proxy)` with `value = P.Proxy`, under `ExplicitNamespaces`. With `value :: P.Proxy ()` added, it rejects only the type occurrence, with GHC-76037. So `data Proxy` imports the constructor, never the type.
- Lowering ignored the keyword and read `data Proxy` as a bare type item. After f95620ac such an item supplies no value, so row 536 newly rejected `P.Proxy`.
- A trace before writing (#28516) showed why the item could not simply be marked unrepresented, as `pattern P` is:
  - the import filter matches exports by string in both namespaces, so the old reading did import `Proxy`;
  - a marker would have imported nothing, a new E0202 path for a constructor from a source module.

### The ruling and the producer (c6ac9d95)

- gpt ruled option B (#28517). The producer recognizes the `data` keyword at the head of an item, before the ordinary classification.
- A single named `data x` lowers as `VarChirho(x)`: a constructor, a variable or an operator, `data (:+)` included. The imported spelling is the replay's; the namespace is now the value one.
- A `data` item with a member list or other suffix lowers as `UnrepresentedChirho`, so a list is never read as part of a name. A `data` keyword inside a member list (`T(data ..)`) is untouched.
- A named `type T` stays `TyConChirho(T, None)`.
- The AST docs say what `VarChirho` and `UnrepresentedChirho` now hold.

### Controls (15bc256a) and GHC 9.14.1 receipt r4

- **Producer:**
  - `data Proxy` alone is one value item in an EXPLICIT list;
  - in `(data Proxy, type Proxy, data f, data (:+))`, the three `data` items are value items and `type Proxy` is a bare type item;
  - `data T(..)` and `data T(MkT)` are unrepresented and leave the list UNRESTRICTED.
- **Driver:** `DataItemChirho.hs` is admitted. `DataItemUnlistedChirho.hs` rejects exactly `P.asProxyTypeOf`, beside a supplied `P.Proxy`. The negative proves the list stays closed, which the positive alone could not.
- **r4** ran only on those two programs, decoded from 15bc256a, with the predictions fixed first. 2 of 2 agree:
  - `DataItemChirho.hs`: check rc 0, then it built, ran and printed `2`;
  - `DataItemUnlistedChirho.hs`: check rc 1, exactly GHC-76037, `Not in scope: ‘P.asProxyTypeOf’`, and nothing about `P.Proxy`.

### Hygiene, re-run with no Cargo

- rustfmt 1.8.0 is a stdin no-op on all 12 changed Rust files.
- `git diff --check b709644c 15bc256a` is clean.
- gitleaks 8.30.1 scanned 13 commits through 15bc256a and found no leaks (the report is `[]`).

### Reach scan r3 (a prediction)

r3 reads named `data` items as c6ac9d95 does, and classifies every reference under both r2's model and its own.

- **Gate corpus, r3 oracle, execution and repository `.hs` files:** no reference changes. I2's `NE.toList` stays the only predicted rejection.
- **Embedded test programs:** only the new controls change. Their three `P.Proxy` references are listed under r3 and would have been rejected under r2.
- **Informational populations** (the package cache, and the GHC rename and module suites): nothing changes.

Row 536 remains not independently landable.
