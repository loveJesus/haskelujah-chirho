<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Value-scope resolution workflow

This workflow is owned by `resolve_module_with_imports_chirho` and `check_module_value_scope_chirho` (row 536).
- It runs after the type-scope check (`type-scope-resolution-chirho.md`), once imports and every top-level definition are in `NameEnvChirho`.
- It runs before Phase 3.9 seeds the type environment, and before inference.

It closes one gap. Naming built the right qualified table for an explicit import list, but never checked a value occurrence against it. So typing's fallback dropped the qualifier and read a bare binding the list leaves out (I2: `NE.toList` under `import qualified Data.List.NonEmpty as NE (head, NonEmpty(..))`, GHC-76037).

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

    closed_chirho --> walk_chirho["Copy the declarations, clear do-selected operations, walk written occurrences"]
    walk_chirho --> qualified_chirho{"Qualified Q.x: variable, operator, constructor or record construction?"}
    qualified_chirho -->|No| pass_chirho[Not this check]
    qualified_chirho -->|Yes| q_closed_chirho{Q closed?}
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
- **The walk.** It is the AST's exhaustive occurrence walk, over a copy of the declarations, taken only when some qualifier is closed. The operations a do statement selects are cleared first, because they are not written.

## Boundaries (row 536 claims none of these)

- Constructor patterns and record field labels, which the walk does not visit.
- `M.do`'s selected `M.>>=`, `M.>>` and reserved `M.fail`.
- Unimported qualifiers (I6, and row 532's Q01/Q02).
- Bare-seed over-exposure (I8).
- Shared-alias ambiguity (I3, I3b, M1).
- Builtin member lists not proven complete.
- Diagnostic bytes: GHC's wording is not matched.
- Naming errors are skipped in defer mode, as type-scope errors are.

## Code

- **The marker and the classifier:** `crates/haskelujah-ast-chirho/src/module_chirho.rs`, with `ImportItemChirho::UnrepresentedChirho`, `is_represented_chirho`, `ImportListModeChirho`, `effective_qualifier_chirho` and `list_mode_chirho`.
- **The producer:** `lower_import_spec_item_chirho` in `crates/haskelujah-parser-chirho/src/lower_chirho.rs`.
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
- **The producer, including the `data` items:** in `lower_chirho/import_item_tests_chirho.rs`.
- **Completeness, the per-type merge and the inert marker:** in `type_exports_tests_chirho.rs`.
- **The check itself, including a bare source type that supplies no constructor:** in `value_scope_tests_chirho.rs`.
