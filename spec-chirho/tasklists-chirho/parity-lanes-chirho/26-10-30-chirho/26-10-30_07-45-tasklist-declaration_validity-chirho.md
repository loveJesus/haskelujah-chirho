<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Declaration validity Chirho

Row 683, from the row 681 declaration-validity audit (#30819-#30829). HASKELUJAH/claude2_chirho
owns this source unit in its worktree, branch `decl-validity-683-chirho` on main 6ffe8072.
GPT owns main and the ledger. No remote push or release.

- [x] Audit the declaration-validity family read-only and post the source checkpoint (#30824).
- [x] Move `typing/src/validity_chirho.rs` byte-identically to `validity_chirho/mod.rs`
      (blob b78d9c7a; typing/src stays at 18 entries).
- [x] Group `annotated_pattern_bindings_chirho.rs` under `typing_integration_chirho/declarations_chirho/`;
      its tc135 include gains exactly one `../` (old blob 2238d192 reproduced; same tc135 bytes).
- [x] Four rules drafted on retained shapes (E0209), computed before deriving, reported after
      naming, never deferred. First candidate gate green; GHC 9.14.1 agrees on all 15 exact
      control programs.
- [x] Boundary found (#30830): with a fixity declaration between a default signature and its
      binding, the class member walk drops the binding and the default-signature rule rejects
      a program GHC and row 682 accept. Decision A (#30831): hold that rule; its source,
      controls and the FixityBetween counterexample are receipts under
      `tmp-chirho/r683-declaration-validity-chirho/held-rule4-chirho/` and `oracle-chirho/`.
- [x] Rules 1-3 only: newtype field count, GADT newtype context, instance-body repeats.
      Controls: typing unit tests; driver negatives with naming reported first; 10
      should_compile files; direct validator tests on lowered AST, including the
      DatatypeContexts one-field precondition.
- [x] Fresh GHC 9.14.1 verdicts for every exact control program: 8 negatives rejected with
      the expected codes, 7 positives accepted. The three-rule candidate agrees except where
      a gap is named: DefaultChirho (rule 4 held) and OperatorChirho (pre-existing E0101
      for `(:|)`, identical on the row 682 CLI).
- [x] Focused gate on e189dbaa (4 min 15 s): changed-line Clippy 0 hits, warning-free check,
      typing validity 29/29, typing integration 112/112 (2 ignored), instance obligations
      28/28, rustfmt, diff-check, gitleaks.
- [x] Frozen candidate CLI sha256 3bb311b31909b6c7e762a1d1c18fcc50e8e1351317cc65369b6797729159c42e;
      one complete pass on both axes (201 s). should_compile is member-for-member identical
      to the row 682 pass (886/938, verdict file 4f3c3fb7). should_fail 258/767: T15523,
      T15796, T23308 and tcfail038 move for GHC's reason; T17955 moves only by coincidence.
      E0209 appears in no other corpus file's diagnostics.
- [ ] Tested tip and SLOT-FREE to gpt_chirho.

## Gate record

Receipts: `tmp-chirho/r683-declaration-validity-chirho/` in the claude2 worktree. `gate-chirho/`
holds the three-rule gate, `corpus-chirho/comparison-chirho.json` names both retained
baselines (row 682 frozen pass 83016e3c and the original e60d256f) and every mover with its
first diagnostic, and `oracle-chirho/` holds the GHC and candidate verdicts. The four-rule
first gate, its CLI and the held rule-4 patches are under `gate-four-rule-chirho/` and
`held-rule4-chirho/`.

Outside this unit: the default-signature rule (held until the class member walk keeps
members after a fixity declaration), special-class instances (needs builtin provenance), deriving eligibility
(needs Cabal default-extensions and strategy metadata), Haskell 98 constructor scope, missing
newtype constructors, class/instance member retention, operator-class heads, and the
DatatypeContexts head name. T17955 is rejected only by coincidence, as a two-field `Coercible`.
