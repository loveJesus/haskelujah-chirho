<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# CST-to-AST lowering, split by category Chirho

## Origin and scope

Row 641, behavior-neutral structure.
- **Released** by gpt_chirho in #30312, after row 617.
- **Placement checkpoint #30313; approved #30314:** ten category homes, a flat module tree, and the arm extraction first.
- **Branch:** `lower-split-chirho`, from main `b62328fb`.
- **Lane:** `tmp-chirho/lower-split-chirho/`. It holds the full AST dumps (on the 4 TB volume), the scripts and the receipts.

## Before

`crates/haskelujah-parser-chirho/src/lower_chirho.rs`, 16,235 lines:
- **The shell:** the docs, 20 child modules, `lower_module_chirho`, `LowerCtxChirho` and two literal helpers.
- **One `impl LowerCtxChirho`** of 91 methods, plus a second of 2 methods.
- **13 free items.**
- **An inline `tests_chirho`** of 155 tests and 14 helpers.
- **23 more top-level `#[test]` functions and 1 helper** after the module.
- **`lower_expr_chirho` alone was 1,420 lines.**

## Three commits, each proved on its own

1. **`ffc2c751`, lint only.**
   - The move would carry 80 pre-existing Clippy findings, so they are cleared first: 74 of them with `cargo clippy --fix`.
   - The two `filter_map`s that only keep or drop are now filters.
   - Two pairs of identical branches in `parse_kind_atom_chirho` are merged.
   - `enum_variant_names` is allowed, with a reason, on two enums whose every variant carries the Chirho suffix.
   - **Proof:** the full AST dumps of all 3,864 tracked `.hs` files are byte-identical to main's; 402/402 parser tests pass; Clippy finds nothing in the file (the workspace drops from 444 to 364).
2. **`57a349be`, the arm extraction.**
   - `lower_paren_expr_chirho` and `lower_list_expr_chirho` take `node_chirho`, `base_chirho` and `span_chirho`. The arms become tail calls, and `lower_expr_chirho` is now 875 lines.
   - **Proof (`verify-extraction-chirho.txt`):**
     - The bodies match token for token, with the same string literals. rustfmt dropped 3 trailing commas from each, in calls that now fit on one line.
     - The rest of the file is unchanged.
     - The verifier was mutated: an added statement failed it.
     - The full AST dumps are byte-identical to the parent's and to main's.
3. **The move** (`tools-chirho/split_lower_chirho.py`). Detailed below.

## Placement (`crates/haskelujah-parser-chirho/src/`)

- **`lower_chirho.rs` (252 lines):** the public shell. It holds the docs, the child modules, `lower_module_chirho`, `LowerCtxChirho` and the literal helpers.
  - It also keeps three free items that the existing modules name through `super::`: `ChildChirho`, `is_type_kind_chirho` and `AssocChirho`. Those files therefore move byte-identical, with no re-export and no field visibility change.
  - Two existing modules glob-import `super::*`, so the parent re-exports `is_pat_kind_chirho` for them.
- **The module tree stays flat.** Every file is a direct child of `lower_chirho` through `#[path]`.

| `lower_chirho/` | Files (lines) |
| --- | --- |
| `context_chirho.rs` | 443: new, spans, children, pragmas, names, placeholders, kind predicates |
| `test_support_chirho.rs` | 696: the 15 shared test helpers |
| `module_items_chirho/` | `module_items_chirho` 626; `import_item_tests` (moved); `module_item_tests` 544 |
| `declarations_chirho/` | `declarations_chirho` 451; `foreign_chirho` 236; the moved `contexts` and `declaration_kinds` files with their tests; `declaration_tests` 545 |
| `data_types_chirho/` | `data_types_chirho` 867; `data_type_tests` 866 |
| `classes_chirho/` | `classes_chirho` 735; `type_families_chirho` 395; `instance_signature_tests` (moved); `class_tests` 349; `instance_tests` 756 |
| `bindings_chirho/` | `bindings_chirho` 956; the moved `do_selection` and its tests; `binding_tests` 773; `guard_do_tests` 634 |
| `expr_syntax_chirho/` | `expressions_chirho` 941; `bracketed_exprs_chirho` 649; `expression_parts_chirho` 534; `fixity_chirho` 164; the prefix-minus and qualified-section files; `expression_tests` 650 |
| `type_syntax_chirho/` | `types_chirho` 866; its 5 files; `type_tests` 853; `kind_tests` 296 |
| `patterns_chirho/` | `patterns_chirho` 633; the moved `record_patterns` and its tests (folding `lower_patterns_chirho/` away); `pattern_tests` 902 |
| `occurrences_chirho/` | the moved `occurrences`, `occurrence_tests` and `walk_hook_tests` |

- **Bounds:** every file is under 1,000 lines, the largest being `bindings_chirho.rs` at 956. `lower_chirho/` holds 11 entries, and its subdirectories hold 2 to 8.
- **Corrections to checkpoint #30313:**
  - There are 178 tests, not 169: the inline module's 155 and 23 more top-level ones. There are 15 helpers.
  - The tests went into 11 files, not about nine: the bindings and classes each needed two to stay under 1,000 lines.
  - The 15 helpers went into one shared support file.

## Checklist

- [x] Placement checkpoint #30313; approved #30314.
- [x] **The lint-only commit `ffc2c751` and the extraction commit `57a349be`,** each with full AST dumps byte-identical to main's.
- [x] **The move proof** (`tools-chirho/verify_lower_split_chirho.py`, `verify-split-chirho.txt`) against `57a349be`:
  - 95/95 methods and 13 free items match, with 71 listed `pub(super)` additions. Each is byte-identical apart from `pub(super)`, except 10 signatures that `pub(super)` pushed past 100 columns. Each of those equals rustfmt's output for its widened base.
  - 178/178 tests and 15/15 helpers are token-identical, with all 1,208 string literals identical. String contents are never dedented.
    - 29 tests were re-joined by rustfmt after the dedent.
    - 6 had a block arm unwrapped or a comma changed by rustfmt. Each equals rustfmt's output for its dedented base.
  - All 21 existing files are byte-identical, the 13 moved ones among them.
  - The receipt lists every test's old and new module path, and the parser's test list holds all 178 at their new paths.
  - **The verifier was mutated:**
    - one space inside a widened method failed it, naming the method;
    - so did one inside a never-widened method;
    - one inside a test's Haskell string failed it on the string literal.
    - Every restore was byte-identical.
- [x] **The same ASTs.** The harness (`ast-dump-chirho/`, never committed) lowers all 3,864 tracked `.hs` files with `lower_module_chirho` and writes every dump in full.
  - It destructures `ModuleChirho` whole and prints its two hash maps in key order.
  - The split's 39.6 MB of dumps are byte-identical to main's, with no panic.
  - Main's dumps were deterministic across two runs.
  - The binary's dependency file names 23 lowering files.
- [x] **The parser crate:** 402/402 tests; no compiler warning; Clippy unchanged at 364 workspace findings. The only lowering finding is the pre-existing one in the byte-identically moved `walk_hook_tests_chirho.rs`.
- [x] **Docs:** seven living docs point at the new homes.
  - Workflows: flat-type syntax, instance obligations, value-scope resolution, operator fixity.
  - Bug records: data binders, GADT records, pattern-synonym contexts.
  - Dated tasklists and stale line numbers stay as history.
- [ ] Commit the move; broad moved-shape scan.
- [ ] Judge main against the contract copy; warm gate; record.
- [ ] Land on main and close progress row 641 (gpt). Then rows 618 and 619, then row 639.
