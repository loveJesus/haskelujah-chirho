<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Re-export origins Chirho

Row 642 (design #30401, approved as fix-forward in #30403) is the provenance half of row 629: an
imported value links to the program module that defines it, whatever module forwarded it. It also
repairs row 629's regression: a builtin re-exported through a program module (q4 to q4d, m1) was
exact before 629 and refused natively after it.

## Bounds (#30403)

- `IfaceValueChirho.defining_module_chirho: Option<String>` is `Some` only for a source module's
  own definition. Re-exports clone it, and nothing stamps a forwarder over it. Builtins and stubs
  have `None`.
- `import_suppliers_chirho` records program-owned defining modules. A value with no program
  origin keeps row 628's builtin route.
- Link rules: one origin links to that module's own non-compiler source binding; several are
  refused as ambiguous; a named origin with no binding is refused. The unique-source fallback and
  the "would guess" refusal are deleted.
- The identity is (name, sorted defining modules), so several paths to one origin give one CoreId.
- Proof: q4 to q4d lead, exact on three engines, with each builtin's Core route asserted; a builtin
  through two forwarders; the adversarial namesake; origin preservation through every re-export
  path; four named mutants; reruns of rows 628, 629, 621 and 648.

## Checklist

- [x] Trace and design checkpoint (#30401). The regression was reproduced on d1192c01 and its
  pre-629 exactness measured on 1be3a521.
- [x] Naming: the origin field. `collect_all_definitions_chirho` stamps the definer. The builtin,
  module-search and dependency stubs get `None`.
- [x] Driver: the supplier map keyed by origin. The provenance routes a forwarded builtin as
  builtin, and an id with a user identity is never a builtin reference.
- [x] Core: the identity's field is `defining_modules_chirho`.
- [x] Linker: the three origin rules, with no fallback.
- [x] Controls: `reexport_origins_chirho.rs` (12 GHC programs and 4 interface-origin tests), r1
  made exact and z1's new refusal in `import_identity_chirho.rs`, and the link tests rewritten.
- [x] Mutants M1 to M4, each killed by its named controls and restored byte-identical.
- [x] Workflow: `native-symbol-identity-chirho.md` carries the final origin rule and residuals.
- [x] Probe receipts: of the tip's 39 cells, 36 agree with GHC. The other 3 are the residuals
  below, unchanged since before row 629. Every built cell carries its backend's marker.
- [x] The canonical builtin interface dump is byte-identical to main's (518 modules).
- [x] Gate on 458ac273, replayed onto 27dc9540 first:
  - core 197, naming 156, typing_integration 78 (2 ignored), and native_symbol_identity 45;
  - the driver library's 350 project, re-export, Hackage and multi-module tests;
  - a warning-free check, changed-line Clippy 0 over 982 lines, rustfmt, diff-check and gitleaks.
- [x] Landed the exact two-commit chain on `main_chirho` at `8c3efdcf`. Independent main checks:
  core 197/197, naming 156/156, row 642's re-export-origin controls 16/16, and a warning-free
  workspace check. Progress row 655 records the closure.

## Residuals (not row 642)

- `check` rejects q1 (a two-level chain) and q4d (a project Prelude). Its single-file stub
  interfaces carry no types for re-exported values, and it did so before row 629.
- The frontend accepts q3, which GHC rejects with GHC-87543. Only the link refuses it.
- Class methods and constructors still link through their class and type.
