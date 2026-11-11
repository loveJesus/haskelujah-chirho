<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Next checker: ordinary data and patterns

Owner: HASKELUJAH/gpt_chirho, unit 743. L.J. asked to continue useful work after the generalization checkpoint. Continue the same first-slice vertical on next-pipeline-data-patterns-chirho; keep the generalization source and CLI frozen.

Placement: HIR owns data/constructor identities and pattern matrices; tc registers checked constructor schemes and ordinary algebraic pattern equalities; next's direct Core elaborator owns constructor workers and ordered match compilation. Reuse the existing parser and interpreter, with no legacy typing/dictionary call or new dependency. Read GHC.Tc.Gen.Pat's ordinary-constructor path and GHC.HsToCore.Match's pattern-order rule, and extend the written implementation map.

- [x] Verify the sole writer, clean source checkpoint, canonical progress ownership and available disk.
- [x] Read the existing HIR/checker/Core boundary and the relevant GHC constructor/match rules.
- [x] Carry ordinary data declarations, applied types and patterns through identity-based HIR; keep unsupported syntax explicit.
- [x] Check constructor field kinds, instantiate constructor schemes per use, and check patterns/branches before publishing Core.
- [x] Emit constructor workers and source-ordered matching with shared fallthrough; preserve lazy variables/wildcards and field evaluation order.
- [x] Execute positive data/match/recursion controls, invalid identity/arity/type controls, and GHC references on the exact fixtures.
- [ ] Run focused gates and the unchanged 353-original comparison against unit 742, with scoped lint attribution and resource outcomes retained.
- [ ] Update the implementation map/workflow, secret-scan, commit/push measured progress, and close the canonical DB row.

Initial scope: ordinary positional H98 data with Type-kinded parameters; constructor, variable, wildcard, as, tuple, list and character patterns in case, lambda and function equations. Contexts, GADTs, records, deriving, strict fields, lazy-pattern syntax and overloaded patterns remain Unsupported. No real standard-library, Main IO, native or full first-slice pass is implied.

## Measured before source checkpoint

The 24 next-driver tests pass (15 retained controls plus 9 data/pattern/recovery tests). Six owned programs evaluate to the same characters as GHC 9.14.1: a/r/c/f/e/i. The seventh valid program compiles and fails on a demanded unmatched pattern in both engines. Five invalid field, arity, nominal, duplicate-binder and kind cases refuse. The initial list/tuple fixture accidentally lost its lambda backslash while being written: that invalid source and its R1 rejection are retained; the corrected source has a separate R2 GHC check/run. No test expectation was changed to hide that authoring error.

Workspace all-target check and a fresh debug CLI build have zero compiler-message records. The fixed 353-original comparison moves seven expected-compile cases (tc004, tc015, tc016, tc032, tc064, tc070, T10489) to ACCEPT and two expected-reject cases (tcfail002, tcfail028) to semantic refusal. Counts are 15 ACCEPT, 4 refusal and 334 Unsupported; no previous correct status regresses. This original-file profile omits all.T flags/ways and is not configured GHC compatibility or the full first-slice gate.

Evidence: tmp-chirho/next-data-patterns-743-chirho/ in this worktree. Source checkpoint, changed-span lint attribution and final closure follow after these recorded checks.
