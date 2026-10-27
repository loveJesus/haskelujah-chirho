<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Module alias exports Chirho

Row 621 (traced at the #30126 checkpoint; released by gpt_chirho #30288 once row 629 closed the
native link) makes `module M` in an export list follow Haskell 2010, section 5.2: it exports
every entity in scope both unqualified (`e`) and qualified by M (`M.e`), whichever imports bring
them.

## Defects on main

- D1: a self export (`module A (module A)`) exported only the module's own definitions, never
  what an unqualified `import B as A` brings (GHC's mod173 `partition`; p1).
- D2: `module L` merged every import aliased `L`, qualified-only ones included (p2).
- D3: one import's `hiding` was applied to everything already exported, so it deleted an
  explicitly exported local of the same name (p10).

## Checklist

- [x] `filter_exports_chirho`'s `module M` arm builds the unqualified scope and the M scope
  import by import and exports their intersection: values, types, associated types and
  completeness marks.
- [x] Naming negatives, one per defect, over the builtin `Data.List` interface. Each fails on
  main's rule at its own assertion.
- [x] GHC-measured controls, `native_symbol_identity_chirho/module_exports_chirho.rs`, one test
  per program. p1, p3, p4, p5, p7, p9 and p10 are exact on the interpreter, LLVM and true
  Cranelift. p2, p6 and p8 are refused on both native backends. p11 is row 629's n1, byte for
  byte.
- [x] Probe receipts: 11 programs, each through `check`, Cranelift and LLVM, on main's CLI and
  the candidate's. 9 cells move (p1, p2, p10), all to GHC. All 33 candidate cells agree with
  GHC, and every built cell carries its backend's marker.
- [x] Workflow: the export-side section of `value-scope-resolution-chirho.md`.
- [x] Gate on 0d0eac63: naming 156; typing_integration 70 (2 ignored); the combined native
  target 29; the driver library's 345 project, re-export and Hackage tests; the `module X`
  corpus sweep, 4,811 files with 0 moved under `check`; a warning-free check; changed-line
  Clippy 0 over 550 lines; rustfmt, diff-check and gitleaks clean.
- [x] Replayed onto b62328fb (row 617, no shared file) with the same crates patch-id. There the
  check is warning-free, naming passes 156, the combined target 29, and typing_integration 74
  (row 617's four new tests included).

## Residual (outside row 621)

- The interpreter's multi-module path (`compile_modules_chirho`) still runs p2, p6 and p8, as
  it does on main. It gives each module every upstream export's type by bare name, so a name
  that `module M` does not export still type-checks there, then links by name. The project path
  filters by import (`filter_seeded_imported_types_for_source_chirho`), so `check` and every
  native build refuse them.
