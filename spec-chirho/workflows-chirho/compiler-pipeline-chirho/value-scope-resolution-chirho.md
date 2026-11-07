<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Value-scope resolution workflow

This workflow is owned by `resolve_module_with_imports_chirho` and `check_module_value_scope_chirho` (rows 536 and 598).
- It runs after the type-scope check (`type-scope-resolution-chirho.md`), once imports and every top-level definition are in `NameEnvChirho`.
- It runs before Phase 3.9 seeds the type environment, and before inference.

It closes one gap. Naming built the right qualified table for an explicit import list, but never checked a value occurrence against it. So typing's fallback dropped the qualifier and read a bare binding the list leaves out (I2: `NE.toList` under `import qualified Data.List.NonEmpty as NE (head, NonEmpty(..))`, GHC-76037).

Row 598 closes two more:
- Nothing checked that a qualifier names an import. `X.reverse`, and `Data.List.reverse` with no import of `Data.List`, checked and ran.
- After naming, a qualified reference resolved by its bare text. A local `reverse` captured `L.reverse`, and a local `foo` captured `Main.foo`, on every engine. The second section below covers that.

```mermaid
flowchart TD
    import_decl_chirho[Import declaration] --> lowered_chirho{Every list item lowered faithfully?}
    lowered_chirho -->|"No: a nameless wildcard, pattern P, or a data item with a member list"| unrepresented_chirho["UnrepresentedChirho: imports, hides and re-exports nothing"]
    lowered_chirho -->|Yes| represented_chirho["Value and type items; a named data x is a value item"]
    unrepresented_chirho --> classifier_chirho["Syntax classifier: effective qualifier and list mode"]
    represented_chirho --> classifier_chirho
    classifier_chirho --> mode_chirho{List mode}
    mode_chirho -->|"UNRESTRICTED: no spec, or an unrepresented item"| open_chirho["Qualifier open: today's behavior"]
    mode_chirho -->|"HIDING, items kept"| open_chirho
    mode_chirho -->|"EXPLICIT, every item represented"| supply_chirho["Supply: listed names, written members, members of a complete T(..); a bare T supplies no value"]
    supply_chirho --> dotdot_chirho{"Every listed T(..) marked complete?"}
    dotdot_chirho -->|No| open_chirho
    dotdot_chirho -->|Yes| union_chirho[Union with the other lists of the same qualifier]
    union_chirho --> other_open_chirho{Any other import of that qualifier open?}
    other_open_chirho -->|Yes| open_chirho
    other_open_chirho -->|No| closed_chirho[Qualifier closed]

    source_iface_chirho[Source module interface] --> local_chirho{"Declared here, with no GADT constructor, in a module that neither declares nor enables pattern synonyms?"}
    local_chirho -->|Yes| mark_chirho[Mark the member list complete]
    local_chirho -->|"No, or re-exported"| unmarked_chirho[Leave unmarked]
    builtin_iface_chirho[Builtin interface] --> verified_chirho{"Entry equals GHC's declaration? NonEmpty"}
    verified_chirho -->|Yes| mark_chirho
    verified_chirho -->|"No: Semigroup, RWST and the rest"| unmarked_chirho
    mark_chirho --> merge_chirho{"Copies of one module merged: does every copy exporting the type mark it?"}
    merge_chirho -->|"Yes, disjoint marks included"| dotdot_chirho
    merge_chirho -->|"No: an unmarked copy clears the mark"| unmarked_chirho
    unmarked_chirho --> dotdot_chirho

    every_module_chirho[Every module] --> walk_chirho["Copy the declarations, clear do-selected operations, walk written occurrences"]
    walk_chirho --> qualified_chirho{"Qualified Q.x: variable, operator, constructor or record construction?"}
    qualified_chirho -->|No| pass_chirho[Not this check]
    qualified_chirho -->|Yes| named_chirho{"Q an import's effective qualifier, or this module's name?"}
    named_chirho -->|"No (row 598)"| unimported_chirho["E0100 not in scope, GHC-76037's class: no module named Q is imported"]
    named_chirho -->|Yes| q_closed_chirho{Q closed?}
    closed_chirho -.-> q_closed_chirho
    q_closed_chirho -->|No| pass_chirho
    q_closed_chirho -->|Yes| supplied_chirho{"x supplied, or Q is this module and x one of its own top-level values?"}
    supplied_chirho -->|Yes| pass_chirho
    supplied_chirho -->|No| e0100_chirho["E0100 not in scope, GHC-76037's class; inference never runs"]
```

## Rules

- **The qualifier.** It is the `as` alias, otherwise the module name (`ImportDeclChirho::effective_qualifier_chirho`). An unqualified import with a list closes its module name too: `import Data.List (sort)` closes `Data.List`.
- **The classifier is syntax only.** `ImportDeclChirho::list_mode_chirho` never answers whether a value is exposed, never reads an interface and never decides `T(..)` completeness. Row 517's core route may reuse its modes and add only its own narrow semantics.
- **A listed name is supplied even where the builtin interface omits it.** The written list is the authority, and interfaces here are partial.
- **A bare `T` item supplies no value.** `import M (T)` brings the type or class `T`, never a constructor that shares its spelling: GHC 9.14.1 reports `P.Proxy` under `Data.Proxy as P (Proxy)` as GHC-01928, and GHC's own T25877 expects the same for a source type. Only written members and the members of a complete `T(..)` supply values.
- **A named `data x` item is a value item.** The `data` keyword puts the name in the value namespace whatever its spelling, so lowering reads `data Proxy`, `data f` and `data (:+)` as value items. GHC 9.14.1 imports the constructor for `data Proxy` and never the type of the same name (gpt_chirho #28515, #28517). The list stays EXPLICIT, so every other value under its qualifier is still checked. A `data` item with a member list has no faithful item and stays unrepresented, and a named `type T` stays a bare type item.
- **`T(..)` closes only on a member list marked complete.** Losing a mark only leaves a qualifier open.
- **Marks merge per type.** When copies of one module merge (`merge_module_ifaces_chirho`), a type both copies export stays marked only when both mark it, because the unmarked copy may export a member no entry lists, such as a bundled pattern synonym. A type only one copy exports keeps that copy's mark, so disjoint marked types still union. Over many copies, a type stays marked only where every copy exporting it marks it, in any order.
- **The walk.** It is the AST's exhaustive occurrence walk, over a copy of the declarations. Row 536 took it only when some qualifier was closed; since row 598 it runs for every module. On a 149-line qualified module, `check` took 1.69-1.74 s before and 1.71-1.73 s after. The operations a do statement selects are cleared first, because they are not written.
- **Every qualifier names something (row 598).** It must be an import's effective qualifier or the module's own name. The driver injects the implicit Prelude before naming, so `Prelude` counts unless the module opts out or imports Prelude itself. An alias replaces its module's name: `import Data.List as L` binds `L`, never `Data.List`.

## After naming: a qualified reference never takes a local binder (row 598)

```mermaid
flowchart TD
    reference_chirho["Written reference: a variable, an infix operator, or a section's operator"] --> written_qualified_chirho{Qualified?}
    written_qualified_chirho -->|No| chain_chirho["Scope chain, innermost first (resolve_var_chirho)"]
    written_qualified_chirho -->|Yes| spelled_chirho{"Bound in scope under its full spelling? The seeded Data.Map and QualifiedDo aliases are"}
    spelled_chirho -->|Yes| alias_chirho[That binding]
    spelled_chirho -->|No| own_chirho{"Q is this module, and its top level binds x?"}
    own_chirho -->|Yes| top_chirho[The module's top-level binding]
    own_chirho -->|No| unscoped_chirho["resolve_unscoped_var_chirho: past every scope; the linker binds the bare export name"]
    unscoped_chirho --> apart_chirho["Beside a same-named top-level binding, the dictionary pass spells the import's body $prelude_x (row 597)"]

    checker_chirho["Checker: the scheme of Q.x"] --> keyed_chirho{"Q.x keyed, or exactly one key ends in .Q.x?"}
    keyed_chirho -->|Yes| scheme_chirho[That scheme]
    keyed_chirho -->|No| outermost_chirho["Bare x in the outermost scope only: the module level and the builtins (lookup_outermost_chirho)"]
    outermost_chirho -->|Unbound| unbound_chirho[E0202 unbound variable]
```

- **Why the bare text was wrong.** No `let`, `where`, lambda or `case` binder can be written with a qualifier, so no local binder can answer for `Q.x`.
  - The desugarer's `ExprChirho::VarChirho` arm used the full spelling only when it was bound in scope, then fell back to the bare text through the scope chain.
  - The infix and section paths resolved the operator by bare text alone.
  - The checker's fallback read the bare name through every scope, so `L.nonexistentChirho` beside a `let nonexistentChirho` checked, where GHC 9.14.1 reports GHC-76037.
- **One route.** `resolve_written_reference_chirho` (`desugar_chirho/references_chirho/qualified_references_chirho.rs`) serves the variable arm, the infix fallback and `build_section_body_chirho`.
- **A section's operator keeps its qualifier (row 609).** The parser lowers `(2 P.-)` through its right-section fallback whenever the trailing operator was folded into an infix node.
  - That fallback made one unqualified name, `P.-`, of the operator, and of each operator before it (`(1 P.+ 2 P.-)`).
  - The desugarer then built a constructor application named `P.-`. The interpreter printed `[P.- 2 0.5]`, and both natives printed a denormal.
  - The fallback now splits the name with `name_from_text_chirho`, as the direct-token path always did, so the route above sees `P` and `-`.
- **An operator the program owns keeps its own binding over every shortcut (row 614).**
  - **Owned:** `names_owned_binding_chirho` (`desugar_chirho/references_chirho/qualified_references_chirho.rs`) asks whether a written operator names a binding the module owns:
    - unqualified and bound in scope, at top level or locally;
    - or qualified by the module's own name, naming its top-level binding.
  - **Where it is checked:**
    - at the head of the infix arm, before `$`, `$!`, `$!!`, `.`, `/=`, `>>`, `>>=`, `&&`, `||`, `**`, `^`, the float-literal arithmetic and the primop table;
    - before the section table;
    - before the prefix `div`, `mod`, `quot` and `rem` rewrite, through `is_owned_binding_id_chirho` on the resolved id.
  - **The route:** an owned operator takes the reference route above, through `written_operator_application_chirho`, which the infix and section fallbacks share. Every other operator keeps its shortcut, so the Prelude's answers are unchanged.
  - **The defect:** each shortcut chose by spelling alone. A program's hidden-and-redefined `&&` gave `(True,True,False)` where GHC gives `(False,False,False)`, and a `where`-bound `/` gave the Prelude's `2.0` for `6.0 / 3.0`.
  - **Boundaries:**
    - An infix `$` is rewritten to an application by the parser's fixity resolution, before naming (row 619).
    - An operator imported from another user module is not in scope, and keeps its shortcut; a single-file run links no sibling module (#29717).
    - The definition forms `a - b = …` and `(f . g) x = …` are rows 617 and 618.
- **This module's name** takes its own top-level binding. Otherwise it takes a name no scope binds, such as one of its own class methods.
- **An import's namesake.** A top-level binding the module itself writes no longer captures an import's `L.reverse`. Row 597's `spell_compiler_owned_apart_chirho` gives the import's generated body its own native symbol. Before row 597, the two `reverse` bindings crashed LLVM (exit 139) and Cranelift refused the second declaration.
- **A sibling user module's alias.** `import qualified Lib as L` with a local `twice` used to capture `L.twice`, giving a wrong answer natively. It now reaches `Lib`'s export. A single-file native compile links no sibling module, so it fails "missing native binding `twice`", as plain `import Lib` already did (ruled #29717). The interpreter's `run` loads no sibling module at all.

## Boundaries (row 536 claims none of these)

- Constructor patterns and record field labels, which the walk does not visit.
- `M.do`'s selected `M.>>=`, `M.>>` and reserved `M.fail`.
- Unimported qualifiers in patterns (`X.Just y ->`) and in types (`1 :: X.Int`) still check where GHC reports GHC-76037: the walk does not visit patterns, and type scope is its own check (row 598).
- `Main.reverse`, where the module's own name qualifies an import rather than one of its own values, still checks where GHC reports GHC-76037 (row 598).
- A qualified alias of a builtin module takes its bare scheme only where GHC 9.14.1 types the export exactly so (row 608). The rest keep the placeholder `forall a. a`; see the builtin type environment.
- Bare-seed over-exposure (I8).
- Shared-alias ambiguity (I3, I3b, M1).
- Builtin member lists not proven complete.
- Diagnostic bytes: GHC's wording is not matched.
- Naming errors are skipped in defer mode, as type-scope errors are.

## Code

- **The marker and the classifier:** `crates/haskelujah-ast-chirho/src/module_chirho.rs`, with `ImportItemChirho::UnrepresentedChirho`, `is_represented_chirho`, `ImportListModeChirho`, `effective_qualifier_chirho` and `list_mode_chirho`.
- **The producer:** `lower_import_spec_item_chirho` in `crates/haskelujah-parser-chirho/src/lower_chirho/module_items_chirho/module_items_chirho.rs`.
- **The exhaustive consumers:** `import_item_names_chirho` (`resolve_chirho.rs`), and `merge_import_item_into_exports_chirho` and `remove_import_item_from_exports_chirho` (`iface_chirho.rs`).
- **Completeness:**
  - `IfaceExportsChirho::complete_member_types_chirho`;
  - `locally_complete_type_names_chirho` (`type_exports_chirho.rs`), called by `build_iface_with_imports_chirho`;
  - the builtin `Data.List.NonEmpty` entry;
  - `merge_module_ifaces_chirho`, through `merged_complete_member_types_chirho` (`type_exports_chirho.rs`).
- **The check:** `crates/haskelujah-naming-chirho/src/value_scope_chirho.rs`, with its tests in `value_scope_tests_chirho.rs`.

## Controls

- **GHC-measured programs:** `crates/haskelujah-driver-chirho/tests/typing_integration_chirho/qualified_import_lists_chirho.rs`. It holds 9 programs GHC 9.14.1 rejects and 16 it accepts, run through the parser and name resolution, and I2 through the whole driver. Of the 9, 8 are rejected with GHC-76037, and 1 (a bare type item's same-named constructor) with GHC-01928. A `data Proxy` item is paired: its constructor is supplied, and an unlisted `P.asProxyTypeOf` beside it is rejected, so the list stays closed.
- **The classifier's three modes:** in `module_chirho.rs`.
- **The producer, including the `data` items:** in `lower_chirho/module_items_chirho/import_item_tests_chirho.rs`.
- **Completeness, the per-type merge and the inert marker:** in `type_exports_tests_chirho.rs`.
- **The check itself, including a bare source type that supplies no constructor:** in `value_scope_tests_chirho.rs`.

## Export side: `module M` exports what is in scope both as `e` and as `M.e` (row 621)

Haskell 2010, section 5.2: `module M` in an export list exports every entity in scope both unqualified (`e`) and qualified by M (`M.e`). The two scopes may come from different imports. `filter_exports_chirho`'s `ExportSpecChirho::ModuleChirho` arm builds both and exports their intersection.

```mermaid
flowchart TD
    item_chirho["Export item: module M"] --> locals_chirho["Unqualified scope starts with the module's own entities"]
    item_chirho --> self_chirho{"M is this module's own name?"}
    self_chirho -->|Yes| m_locals_chirho["M scope starts with them too"]
    self_chirho -->|No| m_empty_chirho["M scope starts empty"]
    locals_chirho --> each_chirho["Each import alone: its interface under its own list or hiding"]
    m_locals_chirho --> each_chirho
    m_empty_chirho --> each_chirho
    each_chirho -->|"not qualified"| unqualified_chirho["Joins the unqualified scope"]
    each_chirho -->|"effective qualifier is M"| m_scope_chirho["Joins the M scope"]
    unqualified_chirho --> meet_chirho["Export each value and type in both scopes"]
    m_scope_chirho --> meet_chirho
```

### Rules

- **Each import is weighed alone.** One import's `hiding` removes names only from what that import brings. Before row 621 it was applied to everything already exported, so it deleted an explicitly exported local of the same name (p10).
- **A qualified import never reaches the unqualified scope.** Under `import qualified B as L`, `module L` exports nothing from B (p2).
- **A self export takes unqualified aliases.** `module A (module A)` exports A's own entities and what an unqualified `import B as A` brings (p1, GHC's mod173), but nothing a qualified `import qualified B as A` brings (p8).
- **Types follow values.** A type in both scopes is exported with the M scope's associated types and completeness mark.

### Boundaries

- The interpreter's multi-module path (`compile_modules_chirho`) used to give each module every upstream export's type by bare name, so a name `module M` does not export still type-checked there and linked by name. Since row 647 it gives each module only the value schemes of the modules it imports, through the project path's own `filter_seeded_imported_types_for_source_chirho`, so p2, p6 and p8 are refused there as in `check` and every native build. Since row 668 it filters type synonyms the same way, through `filter_seeded_type_synonyms_for_source_chirho`, once row 656's bare expansion keys had removed the alias-chain failures that filter used to cause here (`type-scope-resolution-chirho.md`).
- Where a re-exported value is defined is row 642's.
- Diagnostic bytes: an unexported name is reported by inference as E0202 `unbound variable`, where GHC 9.14.1 reports GHC-88464.

### Code and controls

- **The rule:** `filter_exports_chirho` in `crates/haskelujah-naming-chirho/src/iface_chirho.rs`, through `merge_imported_exports_chirho`.
- **Each defect alone:** three tests in `value_scope_tests_chirho.rs` over the builtin `Data.List` interface. Each fails on the old rule at its own assertion.
- **GHC-measured programs:** `crates/haskelujah-driver-chirho/tests/native_symbol_identity_chirho/module_exports_chirho.rs`. 7 programs GHC 9.14.1 runs are exact on the interpreter, LLVM and true Cranelift, and the 3 it rejects are refused on all three. Row 621's p11 is row 629's n1 in `import_identity_chirho.rs`.

## A literal's method is never a lexical binding (row 660)

A literal names a class method the program never wrote: `fromInteger` for an integer literal, `fromString` under OverloadedStrings and `fromList` under OverloadedLists. The desugarer used to resolve that name lexically, so a program's own `fromInteger`, top-level or `where`-bound, captured every literal. One with a literal in its own body recursed:
- the interpreter hit its step limit;
- the natives ran to a timeout or died with SIGBUS;
- a captured `fromString` printed nothing on the interpreter and died with SIGSEGV on the natives.

```mermaid
flowchart TD
    literal_chirho["Literal: integer; string under OverloadedStrings; list under OverloadedLists"] --> rebind_chirho{RebindableSyntax?}
    rebind_chirho -->|Yes| lexical_chirho["resolve_var_chirho: the binding in scope, as GHC defines"]
    rebind_chirho -->|No| canonical_chirho["canonical_free_id_chirho: the name's one shared id"]
    canonical_chirho --> mint_chirho["mint_method_occurrence_chirho: the literal's own occurrence"]
    mint_chirho --> record_chirho["record_literal_occurrence_chirho: origin, span and role"]
    record_chirho --> join_chirho["Driver join: the checker's proof for that literal, by provenance"]
    join_chirho --> proved_chirho{Proof joined?}
    proved_chirho -->|Yes| keyed_chirho["Dictionary pass: the proven key's method, a custom IsString instance's included"]
    proved_chirho -->|No| fallback_chirho["finish_chirho: the canonical id, the route a program without the binding always took"]
    written_chirho["A fromInteger, fromString or fromList the program writes"] --> normal_chirho["resolve_var_chirho: the program's binding where one is in scope"]
```

### Rules

- **Only the generated reference changes.** A call the program writes still resolves lexically to the program's binding.
- **The helper mints the literal's occurrence itself.** `resolve_syntax_method_chirho` does this whether or not the method is in the general evidence set, so `fromString` and `fromList` get one too, and a custom IsString literal reaches its own instance by its proof. A program with no binding of the name gets the same ids as before.
- **RebindableSyntax keeps the lexical route.** NoImplicitPrelude alone does not: GHC's literals still use the standard methods.

### Boundaries

- **A custom IsList instance is not claimed.** The checker types an OverloadedLists literal as a list and publishes no IsList proof. That goes to row 667, with GHC's `fromListN`.
- **Ranges (`enumFrom*`, row 665) and `do` operations (row 666)** still resolve lexically.

### Code and controls

- **The rule:** `crates/haskelujah-core-chirho/src/desugar_chirho/references_chirho/syntax_methods_chirho.rs`, called from the three literal sites in `desugar_chirho.rs`.
- **The id choice:** two unit tests in `desugar_chirho/references_chirho/provenance_tests_chirho.rs`.
- **GHC-measured programs:** the row 660 tables in `crates/haskelujah-driver-chirho/tests/dictionary_evidence_chirho/fractional_literals_chirho.rs`. 14 movers, the 3 RebindableSyntax holds and 6 ordinary holds are exact on the interpreter, LLVM and true Cranelift.


## Selected constructor fields Chirho

```mermaid
flowchart TD
    declaration_chirho[Actual constructor AST and positive parent owner] --> fields_chirho[Complete physical labels on exported constructor]
    declaration_chirho --> label_owners_chirho[Actual finite owner set on exported labels]
    fields_chirho --> selection_chirho[Existing export, re-export and import selection]
    label_owners_chirho --> selection_chirho
    selection_chirho --> scope_chirho[Same field map value: physical sequence plus eligible selected labels]
    scope_chirho --> import_context_chirho[Existing InferenceImportContext carries selected accounts]
    import_context_chirho --> type_chirho[Typing: physical field/type zip; eligible omitted binders]
    type_chirho --> finish_chirho[InferResult finish to existing DesugarInputs]
    finish_chirho --> core_chirho[Core: full physical layout; eligible omitted binders]
    core_chirho --> demanded_chirho{Named field demands lost positioning proof?}
    demanded_chirho -->|Yes| refuse_chirho[Typed pre-Core field-account input failure]
    demanded_chirho -->|No| output_chirho[Core output]
    conflict_chirho[Weak or conflicting fragment] --> blocked_chirho[Absorbing Blocked in same payload]
    blocked_chirho --> scope_chirho
    legacy_chirho[Ownerless builtin or raw TH] --> compatibility_chirho[Explicit LegacyUnproven compatibility]
    compatibility_chirho --> type_chirho
    label_owners_chirho --> future_chirho[Future lexical admission: only positive actual-owner evidence]
```

Row712 publishes complete ordered labels only from the real ordinary/newtype/GADT constructor declaration and its existing positive owner. The same exported value payload also carries the actual finite owner set for field labels, including shared/DuplicateRecordFields declarations and standalone forwarding. Label selection must include that actual owner; same spelling, availability, defining-module text or a type-wide member list does not grant it. Fragment conflicts enter absorbing Blocked state rather than unioning labels or reopening legacy absence. Callable selector availability stays separate from field labels.

The existing typing/Core map value separates complete physical order from eligible omitted wildcard binders. Full physical labels keep their indices through field/type correspondence, construction, updates and matching. Only omitted wildcard pattern binders consume the eligible subset. The final map flows from inference through the existing driver/Core handoff; there is no parallel field registry. Exact qualified requests never borrow an unrelated bare namesake. Actual local declarations, including positional/fieldless empty labels, replace imported accounts consistently under bare and their real module-qualified keys. Empty labels do not imply zero runtime constructor arity.

Compatibility callers use the existing two-argument inference/context entry instead of a second eight-argument positional forwarding API. Its convenience route keeps an unseeded ClassEnv::new_chirho(), false imported-completeness flags, empty selected/unavailable sets and absent naming scopes; callers with a supplied class environment borrow that exact environment. This changes API assembly, without granting naming or class proof from absence. The10d6 one-hit Clippy result stays retained; the follow-up source and fresh gates are separate evidence.

The existing borrowed pre-Core cursor refuses a named-field demand when selected positioning evidence was lost, using a truthful typed field-account input cause. This retains existing constructor/projection/resource causes, codes and messages. Unused wildcard and empty positional syntax do not demand a named position. Physical metadata is positioning evidence, not general lexical label admission.

Actual compile-only GHC9.14.1 references32870/32887 establish selected versus hidden wildcard policy, including qualified-only imports. A constructor-only wildcard remains valid when unused or preserving an outer binder; a hidden/sibling selector does not become an omitted binder. The new Haskelujah source/evaluation controls are prepared, unrun until their own scheduler grants; earlier76d2 is compilation-only evidence. Selected-positive controls avoid assuming an unmeasured duplicate-pattern-binder policy.

Full field authority and lexical admission remain OPEN. Builtin record tables such as Identity/SelectT/generic wrappers still lack actual declaration owners, and TH data conversion lacks the written-role stamp required by the owner producer. Their existing ownerless/raw LegacyUnproven compatibility remains explicit; future lexical admission must never consume it as positive proof. General source lexical admission, Do typing710, TH producer repair and native behavior are separate work.
