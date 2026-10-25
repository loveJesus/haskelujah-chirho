<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Derived Show precedence Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB (row 533).

**The bug.** Derived Show builds `show` by concatenation (`"C" ++ " " ++ show a1 ++ ...`).
- Nested constructors, negative fields and records are wrong on main.
- Contexts at precedence 11 fall to a runtime renderer that parenthesizes any text containing a space and renders operator constructors prefix.
- Trace and disposition: gpt_chirho #28285, #28294, #28295. The receipts are in tmp-chirho/derived-show-chirho/: 55 probes, every one confirmed by GHC 9.14.1.

**The fix.** Derived Show becomes a precedence-aware Core renderer on the existing Show shape machinery, following GHC's derived rules:
- prefix: `showParen (d > 10)`, fields at 11;
- record: `showParen (d > 10)`, fields at 0, labels in declaration order;
- infix: a binary symbolic constructor uses `showParen (d > p)` with both operands at p+1, where p is the module's declared precedence or 9.

The Show class keeps exposing only `show`. `$prim_Show_showsPrec_<key>` is an internal, body-backed helper, and `$prim_Show_show_<key>` is its wrapper at precedence 0.

**Held gain it unblocks.** The patterns plus fundep replay (row 531, de92912a). It carries +10 should_fail rejections and the GHC-agreeing tc231 correction, which cannot gate while G2, K06f and N1 are newly WRONG.

The branch `derived-show-precedence-chirho` starts from exact main 6ea3de750d6e6a657f6635fb14e0ed07a0040c70. Its two report commits above 9944ed4c touch only the report, and the crates tree is ad3f3661.

## Source

- [x] C1 (542d98b2): this plan.
- [x] C2 (e9873607): the source and its tests, 12 paths.
  - **Typing:**
    - deriving_chirho/show_chirho.rs (326 lines) takes derive_show_chirho, unchanged, out of mod.rs, which goes from 3,503 to 3,419 lines.
    - `derived_show_metadata_chirho` returns `DerivedShowChirho` for each `data` type whose deriving clause names Show: visible parameters, plus each constructor's name, form and field types, all in declaration order.
    - The form is prefix, a record with its labels, or infix. Infix applies only to a binary symbolic constructor, at the module's declared precedence or 9.
    - A GADT constructor, or a field type outside the key grammar, leaves the type out.
  - **Driver:** lib.rs builds the metadata beside newtype_info and passes it to `dict_pass_module_full_with_evidence_chirho`, the single dictionary-pass call, which every path reaches.
  - **Core:**
    - The `DerivedChirho` shape (shape_chirho.rs) is admitted at any precedence by `derived_show_key_chirho` when every field of every constructor parses at its precedence.
    - Recursion counts as rendering. Each key is checked once: a `false` is final, and a `true` is kept unless it rests on a key still being checked further out.
    - derived_chirho.rs (473 lines) generates `$prim_Show_showsPrec_<key> d x`, one recursive helper per concrete key.
    - `rewrite_derived_show_rows_chirho` points each renderable monomorphic type's public row `$prim_Show_show_<T>` at the helper at 0. That covers evidenced print, `show` occurrences and the instance dictionary.
    - The Show class layout, selectors and runtime are unchanged.
  - **Tests:**
    - tests/dictionary_evidence_chirho/derived_show_chirho.rs: 3 tests covering the 8 movers, C04 and the 10 derived holds.
    - show_chirho/derived_tests_chirho.rs: 7 Core tests:
      - infix thresholds and operands at p + 1;
      - prefix fields at 11 and record fields at 0;
      - fail-closed;
      - parameter substitution;
      - the NonEmpty seam at infixr 5, with no negative-number parentheses at 6;
      - the public row;
      - a 40-level fan-out graph checked once per key.
    - deriving_chirho/show_tests_chirho.rs: 4 metadata tests.
- [x] Checks without Cargo:
  - every touched Rust file is a pinned-rustfmt no-op, and `diff --check` and gitleaks are clean;
  - the new files are all under 500 lines, and the directories hold 4, 4 and 10 entries.
  - GHC 9.14.1 confirms all 19 extracted programs. Main's frozen 77586815 is WRONG on 9 (the 8 movers and C04) and EXACT on the 10 holds, matching the test partition.
  - Receipts: tmp-chirho/derived-show-chirho/oracle-unit-main-r2-chirho/. The earlier oracle-unit-main-chirho/ is an invalid partial run: extraction failed before any program ran.
- [x] Docs (C3): a Derived Show section in print-show-evidence-chirho.md, and these ticks.

C04 also moves: a monomorphic derived type reached through a dictionary now answers as GHC does. The instance dictionary's `show` is the public row that gpt_chirho #28295 asked to keep. A type with parameters reached through a dictionary stays outside this unit.

## Gate

Proposed with the source checkpoint; it runs only under a builder handed by name.
- [ ] The 8 main movers become GHC-exact: S01, S03, S04, S06, S11, S18, C05, D03. C04 moves with them.
- [ ] The 14 measured exact rows and every other non-boundary trace row hold.
- [ ] The native backends still print derived values: the LLVM and Cranelift derived-enum and field-constructor round trips now run the helper.
- [ ] Mutants on prefix, record and infix precedence and on fixity are caught.
- [ ] The corpus does not move.

## Outside this unit

- The runtime Show precedence methods, meaning showsPrec, shows and showParen as functions plus the class defaults (B rows and S13).
- C01, a hand-written show-only instance at nested precedence.
- C04 for polymorphic derived types.
- S09, newtype erasure.
- Standalone deriving.
- Hand-written instances.
- Declaration forms: GADT, prefix symbolic (I07) and backtick (I08, G1).
- NonEmpty metadata, which unit 517 supplies.
- T10 and T11.
