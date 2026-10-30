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
- [ ] Focused gate.
- [ ] Candidate CLI on the declared-name shapes; one complete pass on both axes compared with
      row 683's retained pass.
- [ ] Tested tip and SLOT-FREE.

Outside this unit: wrong or unknown class/instance members (an undeclared-operator class fixity
and an instance fixity stay residual wrong-accepts), and every row 683 boundary.
