<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# A signature's wanted that nothing in scope can serve (row 686, C08) Chirho

## Origin and scope

- **Enlisted** by L.J. through gpt_chirho's #31073. The trace checkpoint was #31077; the source GO, on prototype `7eb930c0`, was #31084.
- **Oracle slot:** #31086. Receipts are in `tmp-chirho/c08-oracle-chirho/`.
- **Branch:** `c08-signature-wanteds-chirho`, in worktree `haskelujah-claude-c08-chirho`, at `7eb930c0`.
- **Root holds the only builder.** This unit is source-only until root replays it or grants a build lease.

## The gap

- **Inside the scope:** at a signature scope's end, the literal's wanted, `Num (TwoChirho (OpaqueChirho a) (OpaqueChirho b))`, is resolved and meta-free. Discharge keeps it, because no given takes part.
- **At the module end:** `with_given_preds_chirho` truncates the givens. The module-end rigid branch then reports only through `rigid_pred_certainly_undeducible_chirho`, which needs a fully-known class and no family arguments, and otherwise drops the wanted.

## Oracle (GHC 9.14.1; root's combined first candidate `d2d3df6e` and second candidate `5b236c6d`)

- **Correction (#31104):** `d2d3df6e` is a prototype comparison binary, not main. Claude2 measured published main `ce999556`: C08, C08LocalProxy, C08OpenOnly, C08Where and C08VarInstance all check rc 0 there too. Main shares the wrong acceptance, so the closure improves main's should-fail semantics.

- **GHC rejects; both CLIs accept:** C08, C08LocalProxy, C08Where and C08OpenOnly, each with GHC-39999.
  - C08VarInstance is rejected by GHC as "Overlapping instances". It is a conservative residual: criterion 7 leaves it accepted.
- **GHC accepts; both CLIs check:** C07, C08Given, C08SuperGiven, C08GivenEq, C08Unsigned, NumFunctions and FamilyContext.
  - Pre-existing on both CLIs: C08Unsigned and FamilyContext fail at run time in the interpreter, with `no instance dictionary for Num/Show %family:...`.

## Checklist

- [x] Read-only trace and checkpoint (#31077).
- [x] Oracle slot, and SLOT-FREE (#31086).
- [x] **Source:**
  - `with_given_scope_chirho` and `with_signature_givens_chirho` (`schemes_chirho.rs`);
  - the two signed-binding switches (`module_chirho.rs`, `bindings_chirho.rs`);
  - `solving_chirho/signature_wanteds_chirho.rs`, the closure and the certificate over `decide_family_with_limits_chirho`.
  - rustfmt is clean.
- [x] **Controls:** `signature_wanteds_chirho.rs`, nine programs byte-identical to the oracle files.
- [x] **Seam 1, criterion 8 (GO #31107):** `seeded_class_names_chirho`.
  - Captured from the seed producer's output in `new_chirho`.
  - Cleared with the whole-environment replacement, and a name is removed by a source class declaration.
  - Not cleared for `Data.Proxy`.
  - Boundary: an orphan universal instance in an uncompiled package.
- [x] **Seam 2, unmodeled pattern evidence (GO #31107, with the inherited-evidence correction):** `unmodeled_pattern_evidence_seen_chirho`, a module-wide flag that never resets.
  - It is set in `bind_constructor_pattern_chirho` and in the record-pattern arm, for a constructor with a context, with a refined result, or with an unknown scheme.
  - It is also set where a GADT refinement is added (`rigid_chirho.rs`).
  - Once set, later closures stay off, nested scopes included.
  - Residual: an unrelated later signature can stay unchecked.
- [x] **Holds:**
  - C08PatternContext: root measured GHC 9.14.1, main ce999556 and R2 all checking rc 0 (`c08-pattern-evidence-oracle-chirho`).
  - C08NestedPatternContext: a `where`-bound signature that inherits the enclosing pattern's evidence. Its GHC measurement is pending.
- [ ] Root replays the patch and runs Cargo, or grants a build lease. Then the focused gate and the corpus measurement.
