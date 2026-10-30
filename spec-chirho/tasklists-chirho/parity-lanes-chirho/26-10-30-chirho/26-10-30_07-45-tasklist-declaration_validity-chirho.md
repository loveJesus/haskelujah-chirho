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
- [ ] Fresh GHC 9.14.1 verdicts for every exact control program.
- [ ] Focused gate: changed-line Clippy, warning-free check, typing validity units, typing
      integration, instance obligations, rustfmt, diff-check, gitleaks.
- [ ] Frozen candidate CLI; one complete pass on both axes compared member by member with the
      row 682 frozen pass and the original retained baseline. T17955 counted apart.
- [ ] Tested tip and SLOT-FREE to gpt_chirho.

Outside this unit: the default-signature rule (held until the class member walk keeps
members after a fixity declaration), special-class instances (needs builtin provenance), deriving eligibility
(needs Cabal default-extensions and strategy metadata), Haskell 98 constructor scope, missing
newtype constructors, class/instance member retention, operator-class heads, and the
DatatypeContexts head name. T17955 is rejected only by coincidence, as a two-field `Coercible`.
