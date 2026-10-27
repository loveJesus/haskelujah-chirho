<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Direct import identity Chirho

Row 629 (gpt_chirho #30235; design #30234) is the direct-import half: a reference to another
module of the program names that module's binding. Row 642 owns re-export origins.

## Bounds (#30235)

- The identity is typed, and survives simplification, offset and remap, both merges, and dedup.
- One shared link decision serves both merges. Compiler bindings are never candidates.
- The unique-source fallback for a re-export is temporary. Zero or several candidates are refused
  by name.
- No first-wins at any layer for imported user references. Builtin and compiler references keep
  row 628's rule.

## Checklist

- [x] Typed `ImportedReferenceChirho` and the provenance role `imported_references_chirho`.
- [x] Driver: the supplier map over the program's own modules. Single-file compiles pass no
  program modules, so they are unchanged.
- [x] Desugaring: written references take the identity, in qualified, aliased and unqualified
  form.
- [x] Dictionary pass: no Prelude body for an imported reference, which also shadows class
  methods (x4's `compare`).
- [x] `ImportLinkerChirho`, shared by the native and interpreter merges.
- [x] Identity-spelled dedup keys in the native merge.
- [x] `eval_modules_with_machine_chirho`, so the interpreter's multi-module output can be
  asserted.
- [x] Controls: `tests/native_symbol_identity_chirho/import_identity_chirho.rs` (9, a submodule of
  row 628's target, per #30276) and `import_links_tests_chirho.rs` (6).
- [x] Reach: four single-edit mutants, 80 s in all, each file restored byte-identical. Each
  mechanism is killed separately:
  - M1, desugaring keyed by name: by the identity and qualified/aliased controls;
  - M2, the dictionary pass's exclusion left empty: by both Prelude controls;
  - M3, the link taking the first definer: by the qualified/aliased control;
  - M4, dedup by plain name: by d1.
- [x] Gate (5c678843, 948 s; the driver library modules alone took 577 s):
  - core 193+4; LLVM 38, Cranelift 42, Wasm 12;
  - driver library 1,725: compile, project, install, eval_*, extensions, and the link tests;
  - `import_identity` 9, `native_symbol_identity` 12, `native_error_helpers` 60,
    `dictionary_evidence` 149, `typing_integration` 70, `cranelift_roundtrip` 43, and 34
    more;
  - receipts: 95 cells, with 14 changed exactly as intended; a1, z1, d1; and p10 under 621's
    prototype at `700`;
  - changed-line Clippy 0.
- [x] Replay onto main and post the tip (#30269).
- [x] Placement correction (#30276). The suite moves byte-identical to
  `tests/native_symbol_identity_chirho/import_identity_chirho.rs`, a submodule of row 628's
  target. That target moves into the same directory as `main.rs`, Cargo's directory form of
  the same target. Otherwise the requested directory would itself have been a 28th root entry.
  The test root stays at main's 27. The combined target runs 21 tests.
