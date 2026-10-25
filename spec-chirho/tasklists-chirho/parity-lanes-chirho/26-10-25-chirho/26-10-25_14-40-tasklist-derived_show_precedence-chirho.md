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

- [ ] Typing:
  - deriving_chirho/show_chirho.rs takes derive_show_chirho out of the 3,503-line mod.rs;
  - it builds normalized metadata for the types whose deriving clause names Show: constructor order, form, record labels, field types and operator precedence.
- [ ] Driver: build that metadata from the module and pass it to the dictionary pass, beside newtype_info.
- [ ] Core: dict_chirho/show_chirho/derived_chirho.rs, with:
  - a DerivedChirho shape, admitted at any precedence when every field is supported, and fail-closed otherwise;
  - one recursive helper per concrete key;
  - each supported monomorphic derived type's public row rewritten to the helper at 0;
  - a seam through which unit 517 can later supply NonEmpty `:|` as infixr 5.
- [ ] Tests:
  - the 8 main movers and the measured holds, end to end in the driver;
  - infix structure and fixity in Core unit tests.
- [ ] Docs: the print-show-evidence workflow, and this tasklist.
- [ ] Checks without Cargo: rustfmt, `diff --check`, gitleaks, GHC receipts for every new program, and path and directory limits.

## Gate

Proposed with the source checkpoint; it runs only under a builder handed by name.
- [ ] The 8 main movers become GHC-exact: S01, S03, S04, S06, S11, S18, C05, D03.
- [ ] The 14 measured exact rows and every other non-boundary trace row hold.
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
