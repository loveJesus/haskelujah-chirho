<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Class member walk Chirho

Row 685 (#30832-#30837). HASKELUJAH/claude2_chirho owns this source unit in its worktree, branch
`class-member-walk-685-chirho` on main 03ce43a3 (crate tree equal to row 683's tested tip).
GPT owns main and the ledger. No remote push or release.

- [x] Leak trace (#30835): the CST where-block stops at a leaked `instance`/`class`/`import`/`module`;
      the lowering walk stops at the first non-method member after a method. Both date from the
      2026-03-29 dependency fixes. Method-like leaked members before the stop would already be taken.
- [x] Class-only design approved (#30836): the class caller passes the names its body declares
      (method signatures, associated type and data declarations, read before walking); a fixity
      naming only those is a legal member. The instance caller passes none: GHC 9.14.1 rejects an
      instance-body fixity (root's oracle). Every other break stays.
- [x] Name collection lowers only signature and associated type/data nodes, so no expression is
      lowered twice and no occurrence origin is minted (#30837).
- [x] Rule 4 re-enabled from row 683's held delta unchanged (files byte-identical to 0cb9abce).
- [x] Fresh GHC 9.14.1: FixityBetween, the sibling layout, T10816, the associated-member positive
      and seven further declared-name shapes accepted; the default-body program prints 4, 42, 5, 6;
      DefaultChirho (GHC-59794) and an undeclared-operator class fixity (GHC-54721) rejected.
- [x] Lowering controls: FixityBetween keeps its default; T10816 keeps its members; a following
      top-level fixity, function and instance stay siblings with and without Haddock comments; an
      undeclared-operator fixity and an instance fixity still end their walks. 36 tests pass with
      the existing leak tests.
- [x] Driver controls: 19 pass, including the default body after a class fixity executed on the
      interpreter, LLVM and Cranelift (inherited vs written).
- [x] Focused gate on 5ed91359 (4 min 57 s): changed-line Clippy 0 hits and no diagnostics in
      changed files, warning-free check, parser 406/406, typing validity 30/30, typing integration
      119/119 (2 ignored), instance obligations 28/28, rustfmt, diff-check, gitleaks.
- [x] Frozen candidate CLI sha256 21642e026283d0d5ed9534a3bfaaec6134813fc1e45a402f290c9ec8f8326567:
      the seven declared-name shapes (backtick fixity, two-operator fixity, associated-operator
      fixity, INLINE and MINIMAL pragmas, Haddock comments, kinded associated type between) are
      accepted like GHC and row 683, with no E0209. The default-body program row 683 rejected
      now checks. One complete pass on both axes (216 s): should_compile is member-for-member
      identical to row 683 (886/938, verdict file 4f3c3fb7); should_fail 259/767, whose only
      mover is MissingDefaultMethodBinding for GHC's reason. E0209 appears only in the six
      should_fail files rows 683 and 685 reject (T17955 counted as coincidental).
- [ ] Tested tip and SLOT-FREE.

## Gate record

Receipts: `tmp-chirho/r685-class-member-walk-chirho/` in the claude2 worktree: `gate-chirho/`,
`oracle-chirho/` (GHC, row 683 and candidate verdicts for every control and shape), and
`corpus-chirho/comparison-chirho.json` (row 683 retained pass 3bb311b3 and the original e60d256f
as baselines, neither re-run).

Outside this unit: wrong or unknown class/instance members (an undeclared-operator class fixity
and an instance fixity stay residual wrong-accepts), and every row 683 boundary.
