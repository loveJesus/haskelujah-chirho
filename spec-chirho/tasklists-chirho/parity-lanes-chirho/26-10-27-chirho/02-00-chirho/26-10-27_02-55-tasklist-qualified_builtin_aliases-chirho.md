<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# A qualified builtin alias takes its bare scheme where GHC agrees Chirho

Row 608 (gpt_chirho #29787 and #29839, placement #29853, accepted #29854): row 598's R2. A
qualified import of a builtin module typed `Q.x` as the placeholder `forall a. a` whenever no
`Module.x` key existed, so `print (L.reverse xs)` defaulted Show to Int.

## Trace and design (#29837)

- The defect is driver Phase 3.9's `base_scheme_chirho` chain. A qualified import never fell back
  to the bare scheme, as an unqualified import does.
- The data comes from GHC 9.14.1 rather than a hand list. For every unkeyed builtin export of a
  module GHC exposes, ghci's `:type M.x` is compared with the bare builtin scheme of `x` under
  one normalization contract.
- The rule takes exact agreement only (ruling 2): the weaker instance rule would reject Foldable
  uses that GHC accepts.

## Checklist

- [x] Production code:
  - the generated table `builtins_chirho/qualified_alias_agreement_chirho.rs`, sorted, with
    `#[rustfmt::skip]` on the table alone;
  - the binary-search lookup `qualified_builtin_alias_scheme_chirho` in `builtins_chirho/mod.rs`,
    with a one-line re-export in `infer_chirho.rs`;
  - Phase 3.9's qualified branch, the only call.
- [x] Tooling and evidence in `test-data-chirho/qualified-alias-agreement-chirho/`: the README
  (GHC 9.14.1, the packages, the contract and the command), `generate_chirho.py`,
  `counts-chirho.txt` and `disagreements-chirho.tsv`.
- [x] Counts: 12,721 unkeyed exports; 721 agree, 684 disagree, 2,870 have no bare scheme, 8,446
  are not typed by GHC.
  - Against the prototype's 708/697: 13 gained and 0 lost.
  - 6 `runST` entries came from the printer's corrected rank-2 parentheses.
  - 7 Eq-constrained list functions came from row 603's added constraints.
- [x] Determinism: regenerated on c2a92a17, 3f162042 and 7787ef02, byte-identical each time.
- [x] Shared test helper: the all-engine assertion moved to the typing-integration crate root,
  serving rows 598 and 608.
- [x] Pre-commit ruling (#29878): a project-local module named like a builtin must not read the
  table, so a guard comes before the commit. The provenance design (#29882) was approved (#29884):
  - `IfaceOriginChirho` on `IfaceExportsChirho`, defaulting to source;
  - one marking loop at the end of `builtin_module_ifaces_chirho`;
  - a merge that stays builtin only when every copy is, in either order;
  - the driver's `qualified_import_alias_scheme_chirho`, which Phase 3.9 calls with the selected
    interface.
- [x] Direct tests: every builtin interface is builtin and the default is source; merges in both
  orders; a source `Data.List`, merged with the seed either way, and a source re-export of
  builtin names, miss `reverse` and `lookup`.
- [x] End-to-end sibling control: not expressible. The loader does not load a project-local
  `Data/List.hs` in place of the builtin `Data.List`, in `check` or `compile`: its own names are
  unbound. Base compiled a shadowing program and crashed (native exit 139); the candidate
  rejects it at compile time, because the selected interface is the builtin one.
- [x] Reach mutation on the pre-guard candidate: with Phase 3.9's table lookup removed, 4 of the
  6 controls fail by named assertion. The two holds pass. The file was restored byte-identical.
- [x] Row 535 contract judge on the pre-guard candidate: GREEN, 41 programs.
- [x] After the guard: regeneration byte-identical (721/684/2,870/8,446).
  - The 59-program transition, base against the guarded candidate: 34 GAIN, 5 CHANGED, holds
    69 exact, 33 fail, 27 rejected and 9 accepted. No signal or timeout.
  - The pre-guard candidate against the guarded one: no cell moved.
  - Corpus selector, 408 and 285 files: 0 moved.
- [x] Gate r1 on 76b448cb (main 7787ef02), 342 s:
  - build with 0 warnings; naming 153, core 193, typing 366;
  - 14 driver suites 357, and the driver library's frontend, qualified, import and scope tests 279;
  - Clippy: none of 574 warnings on a changed line; diff-check and gitleaks clean.
  - rustfmt wanted the `infer_chirho.rs` re-export wrapped. That was fixed by a formatting-only
    amend (d89d087b), and all 11 changed Rust files were then rustfmt-clean.
- [x] On the guarded build:
  - the row 535 contract judge is GREEN: 126 cells, 73 movers and 9 holds;
  - the reach mutation without Phase 3.9's lookup fails 4 of 6 controls by named assertion and
    passes the 2 holds, and the file was restored byte-identical.

## Residuals

- The 684 disagreements keep the placeholder:
  - Foldable generalization;
  - Data.Maybe's over-general schemes;
  - `intercalate`;
  - the missing constraints on `forever`, `mapM` and `sequence`.
- `comparing` and Data.Set's `member` have no runtime body. LLVM lacks `isDigit#`.
- Sibling user modules' types do not reach the checker at all, and the loader does not load a
  project-local module in place of a builtin module's name. Neither reaches the table: the origin
  guard keeps every source interface, merged copies included, on today's placeholder.
