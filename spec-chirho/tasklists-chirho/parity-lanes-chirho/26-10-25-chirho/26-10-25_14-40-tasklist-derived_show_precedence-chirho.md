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
    - Recursion counts as rendering, and final answers are kept (C5 replaces this memo; see below):
      - a `false` is always final, since an assumption only ever answers `true`;
      - a `true` that rested on a key still being checked further out is provisional and is not kept, so a key inside a recursive cycle can be checked again after the outer key is final;
      - acyclic fan-out is checked once per key. (C2's commit message says each key is checked once; C4 corrects that.)
    - derived_chirho.rs (473 lines) generates `$prim_Show_showsPrec_<key> d x`, one recursive helper per concrete key.
    - `rewrite_derived_show_rows_chirho` points each renderable monomorphic type's public row `$prim_Show_show_<T>` at the helper at 0. That covers evidenced print, `show` occurrences and the instance dictionary.
    - The Show class layout, selectors and runtime are unchanged.
  - **Tests:**
    - tests/dictionary_evidence_chirho/derived_show_chirho.rs: 3 tests covering the 8 movers, C04 and the 10 derived holds.
    - show_chirho/derived_tests_chirho.rs: 7 Core tests in C2, 9 after C4:
      - infix thresholds and operands at p + 1;
      - prefix fields at 11 and record fields at 0;
      - fail-closed;
      - parameter substitution;
      - the NonEmpty seam at infixr 5, with no negative-number parentheses at 6;
      - the public row;
      - a 40-level acyclic fan-out graph, checked once per key;
      - (C4) a mutual recursion with a missing renderer (AChirho needs BChirho, which needs AChirho and an unshowable FooChirho): both stay unrenderable, and the recursion state clears;
      - (C4) a renderable mutual recursion, admitted from either side, with the state cleared. The self-recursive positive is the infix test's EChirho.
    - deriving_chirho/show_tests_chirho.rs: 4 metadata tests.
- [x] Checks without Cargo:
  - every touched Rust file is a pinned-rustfmt no-op, and `diff --check` and gitleaks are clean;
  - the new files are all under 500 lines, and the directories hold 4, 4 and 10 entries.
  - GHC 9.14.1 confirms all 19 extracted programs. Main's frozen 77586815 is WRONG on 9 (the 8 movers and C04) and EXACT on the 10 holds, matching the test partition.
  - Receipts: tmp-chirho/derived-show-chirho/oracle-unit-main-r2-chirho/. The earlier oracle-unit-main-chirho/ is an invalid partial run: extraction failed before any program ran.
- [x] Docs (C3): a Derived Show section in print-show-evidence-chirho.md, and these ticks.

- [x] C4, source only, after gpt_chirho's source review (#28303):
  - The comments and this tasklist state the memo's exact behavior.
  - Two mutual-recursion tests are added, making 16 new tests in all.
  - C04 is the ninth claimed mover. A monomorphic derived type reached through a dictionary answers as GHC does, because the instance dictionary's `show` is the public row #28295 asked to keep.
  - The trace (tmp-chirho/derived-show-chirho/TRACE-chirho.md) is kept as measured. It listed C04 outside the claim; the public-row placement refined that diagnosis.
  - The boundary is now a derived type with parameters reached through a dictionary.
- [x] Replay onto main 9a820c64 after row 532 landed (#28320). C1-C4 became 89a7f150, 214d14f6, 27d4cf39 and 66d76d6f. The union proof is GREEN 14/14 (tmp-chirho/derived-show-chirho/replay-9a820c64-chirho/).
- [x] C5 (286a57b3), source only. A read-only desk-check of the never-compiled unit found two defects in the support check, and gpt_chirho gave GO in #28328:
  - **D1, a compiler crash on valid Haskell.** A non-regular datatype such as `data N a = Nest (N [a]) | Flat a` met a new key at every level. The parser's depth guard restarted on every call, so the check recursed until the stack overflowed. This happened even for a monomorphic type that only holds such a field. Main compiles these programs.
  - **D2, compile time.** A provisional `true` was never reused within its outermost check, so a cycle with fan-out was checked 2^n times.
  - **The fix.**
    - The check fails closed past the shared depth bound (64) and for a key over 512 bytes.
    - A `false` that depends on the depth bound is not kept.
    - A `true` carries the deepest nesting at which it held.
    - Provisional trues are reused within the outermost check and promoted only when it renders.
    - The parenthesized branch reuses parenthesize_chirho.
  - **Three Core tests:**
    - a nested datatype in both shapes;
    - the depth boundary in either call order, the depth-limit mutant's control;
    - a 40-type fan-out cycle, the promotion mutant's control.
  - That makes 19 new tests in all: core 12, typing 4, driver 3.
- [x] C6: these ticks, and the workflow's bounded semantics.

## Gate

The shape is gpt_chirho #28303, amended by #28328. It runs only under a builder handed by name, on the final tip over main 9a820c64, with the base built in the gate at exact 9a820c64.
- [ ] The 9 movers become GHC-exact: S01, S03, S04, S06, S11, S18, C05, D03, and the monomorphic C04.
- [ ] A parameterized derived type reached through a dictionary is measured as a calibration probe. If it moves, the gate stops RED for review.
- [ ] The 14 measured exact rows and every other non-boundary trace row hold.
- [ ] Bounded failure, end to end. Each of these terminates within the gate's timeout and stays on the base route:
  - `N` at a concrete type;
  - the growing `Seq` family;
  - an unprinted monomorphic `M` that holds `N Int`.

  A stack overflow, signal, timeout, panic or missing result is RED.
- [ ] The native backends still print derived values: the LLVM and Cranelift derived-enum and field-constructor round trips now run the helper.
- [ ] typing_integration is identical to the base. Row 532's GREEN receipts and counts are pinned guards.
- [ ] Mutants are caught on prefix, record and infix precedence, on fixity, on the depth bound (M9) and on provisional promotion (M10).
- [ ] The corpus does not move.

## Outside this unit

- The runtime Show precedence methods, meaning showsPrec, shows and showParen as functions plus the class defaults (B rows and S13).
- C01, a hand-written show-only instance at nested precedence.
- A derived type with parameters reached through a dictionary. The monomorphic C04 is inside the unit.
- S09, newtype erasure.
- Standalone deriving.
- Hand-written instances.
- Declaration forms: GADT, prefix symbolic (I07) and backtick (I08, G1).
- NonEmpty metadata, which unit 517 supplies.
- T10 and T11.
