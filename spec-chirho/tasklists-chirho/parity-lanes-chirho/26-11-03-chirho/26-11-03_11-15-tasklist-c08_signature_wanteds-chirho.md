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
- [x] **Root replay (#31114).**
  - `8aa021bc`, `672e2151` and `2655b23f` replayed onto root `43163cf0` as `b819a64c`, `451da7e4` and `0ee7d42b`.
  - `cargo check -p haskelujah-driver --all-targets -j2` passes with no warning. No semantic claim yet.
- [x] **C08NestedPatternContext:** fresh GHC 9.14.1 accepts it, and its sha matches the probe (root `tmp-chirho/final-extra-oracle-chirho/receipts-chirho.json`).
- [ ] **Root's run:** the full typing unit pass, then the CLI, check and interpreter controls, and the unchanged corpus axes. Receipts for any failure will be routed here.

- [x] **The insoluble-given review (#31117), refuted by measurement (#31121).**
  - The case: `fChirho :: forall a. (Int ~ Bool) => ProxyChirho a -> OpaqueChirho a; fChirho _ = 3`.
  - Fresh GHC 9.14.1 REJECTS it with GHC-39999, "Could not deduce Num (OpaqueChirho a) ... from the context Int ~ Bool".
  - The candidate rejects it with E0204; published main wrongly accepts it.
  - It is a measured NEGATIVE: no insoluble-given skip, and the proposed bypass (#31119) is withdrawn.
  - Root's `tmp-chirho/c08-exfalso-oracle-chirho/C08ContradictoryGivenChirho.hs` (sha256 8fd098fc…) is the measured source. Its non-comment lines equal this unit's `pending-measurement-chirho/C08InsolubleGivenChirho` probe once the module name is normalized.

- [x] **Focused lease (#31124), on root `45bea1ff`** (the lane was replayed; the earlier tip is kept on `c08-signature-wanteds-r1-chirho`).
  - **Measured cause of the accepted C08 cases:** `decide_family_with_limits_chirho` on `TwoChirho (OpaqueChirho a) (OpaqueChirho b)` returns `UnknownChirho`, over 2 branches. The apartness kernel (`closed_family_apartness_chirho.rs`) stays uncertain for two different stuck applications against `TwoChirho Int Bool`, so nothing certifies.
  - **It is not the import.** `OpaqueChirho a` decides NoMatch over 0 branches and is reported with `import Data.Proxy` (`visible=true`); the seeded classes work. That came from a temporary env-gated trace, removed afterwards.
  - **Measured cause of C08PatternContext's four false reports:** the data and newtype GADT registration builds the constructor's scheme from `ast_type_to_ty_chirho`, which drops `Num (OpaqueChirho a) =>`, so the pattern hook saw no context.
  - **Fix:** `context_constructors_chirho`, a private InferCtx set, marked by the declaration producer at both GADT registration sites (`note_written_constructor_context_chirho`) and read by the pattern hook.
  - **Results:**
    - the irreducible negative and the 8 accepted programs pass;
    - the 2 blocked negatives fail, as measured (`C08LocalProxyChirho.hs: GHC rejects this program; we accepted it`);
    - the ordered-family table is 8/10: C08 for the same reducer cause, and B01 for the known missing `natValChirho` STG binding;
    - the typing library passes 502/502;
    - the driver all-targets check has no warning, and Clippy (typing and driver tests) finds nothing in the changed files.
  - **Logs:** `tmp-chirho/c08-lease-chirho/r1-*`.
  - **Residual:** an imported GADT constructor's context is not transported, so the mark covers only this module's constructors.

## Caveats kept in the record

- **The pattern-evidence boundary is module-wide and conservative.**
  - Once any pattern matches a constructor whose context or refinement the checker does not install, or whose scheme is unknown, and once any GADT refinement is added, every later signature closure in that module stays off.
  - An unrelated later signature can therefore stay unchecked.
  - Real pattern-local givens are the later repair.
- **Class-origin trust is inherited from the seed producer.** A seeded standard class is trusted on base's guarantee that it declares no bare-variable instance head.
  - A whole-environment replacement clears that trust, and a source class declaration of the same name removes it.
  - An orphan universal instance in an uncompiled package stays outside the view.
- **C08VarInstance stays accepted.** GHC rejects it as "Overlapping instances", but criterion 7 leaves any class with a bare-variable head unjudged.
