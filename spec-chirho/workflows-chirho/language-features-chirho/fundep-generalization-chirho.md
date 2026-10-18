<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Functional dependencies at generalization and at the module end

A fundep `a -> b` makes `b` a function of `a`. In a wanted `C Ex t` of
`class C a b | a -> b`, the closed `Ex` fixes `t`. GHC does not quantify `t`,
and it does not quantify a constraint that mentions such a variable (Note [Do
not quantify over constraints that determine a variable] in GHC.Tc.Solver). The
constraint is left over and becomes "No instance for C Ex t0" (should_fail
T18398). Code: `crates/haskelujah-typing-chirho/src/infer_chirho/fundep_generalization_chirho.rs`.

```mermaid
flowchart TD
    G[generalize_with_io_defaulting_chirho<br/>scope TopLevel or Nested] --> M0[mono0: environment and given variables]
    M0 --> C[fundep_newly_mono_vars_chirho<br/>close mono0 over the deferred wanteds' fundeps<br/>normalized arguments, to a fixed point]
    C --> Q[quantified = type variables - mono0 - newly]
    Q --> P{place_fundep_wanted_chirho:<br/>does the wanted mention a newly variable?}
    P -- no, or it mentions a rigid variable --> E[existing partition, unchanged]
    P -- yes, top level --> D[stays deferred]
    P -- yes, nested, mentions a quantified variable --> S[quantified into the scheme]
    P -- yes, nested, no quantified variable --> D
    D --> F[check_deferred_preds_chirho, at the module end]
    F --> B{report_unsatisfiable_fundep_wanted_chirho:<br/>fully known and faithful, not magic, not entailed by givens,<br/>no other class extends it, no variable a nested binding quantified,<br/>a fundep with ALL determining arguments closed and a determined<br/>argument holding a variable, and no instance matches the<br/>determining positions under ONE substitution<br/>(an uncertain position counts as a match)}
    B -- yes --> R[no instance for ...]
    B -- no --> X[existing module-end checks]
```

## Rules and their controls

- A wanted that mentions no newly monomorphic variable keeps today's
  partition: R00 `meth p Ex (0 :: Int)` (concrete range), R01, R07, F08 and F15
  (no fundep).
- At top level, a wanted over a newly monomorphic variable is never
  quantified: N01-N03 (T18398 reductions), F05 (`C4 p Ex p0`, with `p`
  quantified), F12, R04.
- A nested binding quantifies such a wanted when it mentions a quantified
  variable, and its call sites decide: R08 (never called) is accepted, and R09
  (called at `Int` and `'c'`) is rejected.
- The module-end refutation needs every determining argument closed:
  - no variable, rigid skolem, synonym, stuck family or `#` head after
    normalization (R06 resolves through a synonym);
  - a determined argument that holds a variable, flexible or rigid (F10);
  - no instance whose determining positions match under one substitution
    (F16 is rejected and F17 accepted, against `instance C3 x x Int`).
- A position matching cannot read with certainty counts as a match. That
  includes an applied variable in an instance head against list, tuple or
  arrow syntax (`needs_syntax_split_chirho`): `instance Shape (c a) (c a)`
  binds `c` to `[]`, `(,) Int` or `(->) Bool`, but `match_ty_chirho` never
  splits that syntax. Accepted: GHC's tc126 (read whole from the corpus), and
  H01-H03 (list, tuple and arrow) with H05 (`Maybe (c a)` against
  `Maybe [Int]`). Before this revision gpt_chirho's corpus gate rejected each of
  them (#27489). H04 (`Shape ExChirho r`, a nullary determinant) is still a
  certain mismatch and stays rejected.
- A class that another class extends is never refuted at the module end. A
  given of the subclass could supply it by superclass projection with its
  arguments permuted, which `pred_entailed_by_givens_chirho` does not model:
  S00 `D p Ex b =>` is accepted. As a consequence, S01 and S02 (the same body
  without the given) stay accepted, while GHC rejects them: that boundary is
  kept on purpose.

- A leftover wanted that mentions a variable some nested binding quantified
  is never refuted at the module end. It escaped that binding's scheme,
  because a wanted over a rigid variable is never quantified there, so no
  call site judged it. GHC quantifies it and checks each call: R02
  (`let g p = meth p Ex b in g q` under `C q Ex b =>`) is valid, and it stays
  accepted (gpt_chirho's built gate, #27472).

## Boundaries kept

- R02n (the same body with no given) and R02u (the same given, with `g`
  called at `True`) are rejected by GHC and stay accepted here, as on held.
  Quantifying such a wanted into the nested scheme is the repair that would
  judge them.

- No instance improvement at generalization: GHC improves R03 to `b := Int` at
  the definition, while haskelujah keeps its module-end improvement.
- MPTC runtime (gpt_chirho's frozen gate on 3ff084c1, #27485):
  - R05 and R10 keep held's empty line where GHC prints;
  - M02 keeps its PAP, M03 its runtime error, and M01 prints 3 exactly;
  - R03b CHANGED. Held printed an empty line (rc 0). The rule now fails it
    loudly (rc 1), `missing STG binding methChirho`, where GHC prints 6. The
    wanted `C p Ex b` is no longer quantified into `fChirho`'s scheme, so
    `fChirho` takes no dictionary. The module end improves `b := Int` and
    checks the instance, but gives no evidence to the method occurrence
    inside `fChirho`. GHC improves before generalizing and solves it from the
    instance. R11b (`KeyChirho [e]` with `instance C Ex Int`) fails the same
    way.
- A later top-level binding absorbs a leftover wanted (N01m and N03m: GHC
  rejects, and they are accepted here as on held). The existing partition reads
  only the FIRST argument's variables (`pred_chirho.ty_chirho`), so `C Ex t`,
  whose first argument is closed, counts as "all quantified" at the NEXT
  top-level generalization. It joins that binding's inferred scheme, and when
  the binding has a signature (`main :: IO ()`), the signature's scheme
  replaces the inferred one and the wanted is dropped. Measured on the frozen
  CLIs: N03g (`g :: ()` after `f`) and N03h (`g = ()` after `f`) are accepted,
  and N03p (`main` BEFORE `f`) is rejected. Neither the module name nor `main`
  matters. Reading every argument there is a broader repair of the existing
  partition, so it is not part of this unit.
- Normalization (MU6 survived on 3ff084c1): R06's synonym is already expanded
  by `ast_type_to_ty_chirho`, which expands synonyms and reduces families when
  it converts the annotation, and `unify_normalized_chirho` normalizes both
  sides of every unification. At the module end, phase 1 of
  `check_deferred_preds_chirho` normalizes every argument after the final
  substitution, so the report's own normalization only re-reads what fundep
  improvement and defaulting add later. Deferred wanteds are substituted
  without normalizing (`apply_subst_all_chirho`). The helper's normalization
  therefore matters at closure and placement when a family that was stuck at
  unification reduces after a later solve and drops a variable. R11
  (`KeyChirho [e] = ExChirho`, the list fixed by `xs ++ []`) is that case:
  rejected by GHC and by the rule, accepted by held, and by source reading
  accepted under MU6. The MU6 run is still owed.

Controls live in `crates/haskelujah-driver-chirho/tests/rigid_tyvars_chirho/fundep_generalization_chirho.rs`.
Every embedded program was measured against GHC 9.14.1 exactly as the test
builds it. tc126 is the corpus file itself. tc231 (GHC-39999, `No instance for
Zork s (Z [Char]) b0`) is a correct new rejection that sits in the corpus's
accept bucket. It stays there as an actual membership delta.
