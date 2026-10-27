<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# The CST parser, split by syntactic category Chirho

## Origin and scope

Row 640, behavior-neutral structure.
- **Ordered by gpt_chirho #30215:** row 617's whitespace-rule repair (in `starts_infix_fun_bind_chirho`) is blocked behind this row. `cst_parser_chirho.rs` was 6,432 lines.
- **Placement and landing proof approved #30218, on checkpoint #30217.**
- **Branch:** `cst-parser-split-r2-chirho`, from main `7b7eaf9c`, in two commits:
  - **1/2, lint only:** `573a1fc1` drops the needless `return;` that ended the backtick-infix branch of `parse_type_chirho`.
  - **2/2, the move,** proved against `573a1fc1`.
- **Why two commits.** The first version (`cst-parser-split-chirho`, `97388445`) went RED at gate r1's changed-line Clippy step: the move carried that pre-existing `needless_return` into `types_chirho.rs`. A standalone `-p haskelujah-parser-chirho` Clippy run had missed it. #30218's rule applies: repair in a lint-only preparation commit before the move, then compare against that lint-clean parent.

## Before

- 203 lines of header and raw-token mapping, `map_token_kind_chirho` and four helpers.
- The `ParserChirho` struct, and one `impl` of 5,555 lines holding 136 methods.
- `parse_to_cst_chirho`.
- A 653-line inline `tests_chirho` module.

## Placement (`crates/haskelujah-parser-chirho/src/`)

- `cst_parser_chirho.rs` (121 lines): the public shell. The module docs, the child modules, `ParserChirho`, `new_chirho`, `parse_chirho` and `parse_to_cst_chirho`.
- `cst_parser_chirho/` (12 entries):

| File | Lines | Methods | Contents |
| --- | ---: | ---: | --- |
| `cursor_chirho.rs` | 523 | 26 | raw-token mapping and the token cursor, boundaries, operator predicates |
| `module_chirho.rs` | 312 | 6 | the module header, exports, imports and the top-level dispatch |
| `data_decls_chirho.rs` | 692 | 9 | data, newtype, GADT, records, deriving |
| `declarations_chirho.rs` | 465 | 12 | type families and aliases, class, instance, fixity, deriving, default, pattern synonyms, foreign |
| `bindings_chirho.rs` | 805 | 18 | value declarations, the infix-definition lookahead, guards, where |
| `types_chirho.rs` | 642 | 16 | types |
| `expressions_chirho.rs` | 779 | 18 | infix, lambda, let, if, case, do |
| `atoms_chirho.rs` | 736 | 11 | application and atoms, layout blocks |
| `patterns_chirho.rs` | 611 | 10 | patterns |
| `template_haskell_chirho.rs` | 223 | 8 | splices and quotes |
| `let_qualifier_chirho.rs` | 135 | — | unchanged |

- `cst_parser_chirho/tests_chirho/` (3 entries): `parser_tests_chirho.rs`, the former inline module, beside the two existing test files moved in by `git mv`. `#[path]` keeps every module path.

## Mechanics

- **Mechanical.** `tmp-chirho/cst-parser-split-chirho/tools-chirho/split_cst_parser_chirho.py` cuts each method at its rustfmt boundary, with its docs, from HEAD's text.
- **Visibility.** Each file has its own `impl ParserChirho`. A method or helper gets `pub(super)` only when another file calls it: 78 do. Struct fields stay private to the parent.
- **The parent** lost its now-unused `TokenKindChirho` import, and its organization line names the category files.

## Docs

The paths now point at the category files:
- the Mermaid node in `workflows-chirho/language-features-chirho/pattern-matching-chirho.md`;
- the parser paths in `bug-record-field-forall-lowering-chirho.md` and `bug-gadt-record-fields-dropped-chirho.md`.

## Checklist

- [x] Placement checkpoint #30217; approved #30218.
- [x] Gate r1 on `97388445`: RED at changed-line Clippy (the moved `needless_return`).
- [x] **The lint-only commit** `573a1fc1`: the parser crate's Clippy findings drop from 107 to 106; the full CST dumps below are byte-identical before and after it; 400/400 parser tests.
- [x] **The move proof** against `573a1fc1` (`tools-chirho/verify_split_chirho.py`, `verify-base-573a1fc1-chirho.txt`):
  - all 136 methods are byte-identical apart from 78 listed `pub(super)` additions;
  - all 5 helpers are identical;
  - the inline tests' 41 functions and 110 string literals are identical, and their tokens match; rustfmt re-joined 3 statements that the dedent let fit on one line;
  - the two moved test files are byte-identical.
  - **The verifier was itself mutated:** one space added inside a moved method made it fail, naming the method; the restore was byte-identical.
  - **It reads the right base:** run against pre-lint main `7b7eaf9c`, it fails on exactly one method, `parse_type_chirho`.
- [x] **The same trees.** A throwaway harness (`tmp-chirho/cst-parser-split-chirho/cst-dump-chirho`, never committed) parses all 3,864 tracked `.hs` files with `parse_to_cst_chirho` and writes every CST debug dump in full.
  - `parse_to_cst_chirho` returns only the tree, and the parser reports errors as `ErrorNodeChirho` nodes in it, so the dumps carry its diagnostics.
  - Three builds, each from its own `git archive` or the worktree: main `7b7eaf9c`, the lint commit `573a1fc1` and the split. Their 76 MB of full dumps are byte-identical (`cmp`, one SHA-256), with no panic.
  - Each binary's dependency file names its own parser layout: the single file for the two bases, the category files for the split.
- [x] **The parser crate:** 400/400 tests at the split and at `573a1fc1`, with no compiler warning; the 400 test names, 47 of them in the CST parser's modules, are identical.
- [ ] No Clippy finding on a changed line (the gate's step).
- [ ] Commit; broad moved-shape scan.
- [ ] Judge main against the contract copy; warm gate; record.
- [ ] Land on main and close progress row 640 (gpt). Then row 617, the whitespace rule, in `bindings_chirho.rs`.
