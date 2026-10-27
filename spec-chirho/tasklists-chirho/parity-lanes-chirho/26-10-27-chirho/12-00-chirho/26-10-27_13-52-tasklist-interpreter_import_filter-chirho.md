<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Interpreter import filter Chirho

Row 647 (released in #30410; design B approved in #30423) gives the interpreter's multi-module path
only the value schemes of the modules each module imports, as the project path does. Row 656 owns
the separate type-synonym filter bug.

## Bounds (#30423)

- Only the value-scheme seam. `compile_modules_chirho` passes
  `filter_seeded_imported_types_for_source_chirho(source, accumulated imported types)` to the
  frontend.
- No synonym filter in this row, and `compile_modules_incremental_chirho` is unchanged.
- Proof:
  - p2, p6 and p8 are refused on the interpreter.
  - Row 621's control gains the interpreter assertion and loses its residual note.
  - These stay green: the 29 callers, install 8/8, native_symbol_identity,
    method_occurrence_evidence, typing_integration, and the project, re-export and Hackage
    selection.
  - One mutation, removing the value-filter call, is killed by the interpreter refusal control.

## Checklist

- [x] Trace checkpoint (#30419) with two prototypes:
  - A, both filters: 2 of the 29 callers fail on GHC-valid Parsec-style alias chains.
  - B, value schemes only: everything stays green.
  - The project path's own synonym filter rejects the GHC-valid alias chain natively, which is
    row 656.
- [x] The seam in `compile_modules_chirho`, with its comment.
- [x] Row 621's refusal control asserts the interpreter. The native-only helper is private again,
  and the workflow section is updated.
- [x] Mutant: removing the filter call is killed by
  `names_out_of_both_scopes_are_not_exported_chirho` on the interpreter
  (`interpreter: Ok("2\n")`). I first restored the file with `git checkout`, which wiped the
  uncommitted seam. The seam was reapplied and proven line-identical to the tested diff before
  committing.
- [x] Gate on fbe6a20d:
  - the 29 callers, install 8/8, native_symbol_identity 45 and method_occurrence_evidence 7;
  - typing_integration 78 (2 ignored);
  - the driver library's 350 project, re-export, Hackage and multi-module tests, the same 350
    names row 642 passed;
  - changed-line Clippy 0 over 18 lines, a warning-free check, rustfmt, diff-check and gitleaks.
- [x] Replayed onto df542bb0 (row 619, no shared file) with the same patch-id. There the check is
  warning-free, native_symbol_identity passes 45, the 29 callers and install pass, and
  typing_integration passes 81 (row 619's three new controls included).
