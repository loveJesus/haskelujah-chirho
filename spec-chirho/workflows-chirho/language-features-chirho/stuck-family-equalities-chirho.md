<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Stuck type-family equalities and given equalities workflow

A type-family application whose argument is still an unsolved unification variable cannot be
compared with anything yet (`n ~ N t` before `t` is known), and a signature context may state an
equality that unification alone cannot decide (`F a ~ Bool =>`, `Int ~ Bool =>`). Both used to be
reported as mismatches; now the first is *deferred* and the second becomes a *rewrite rule*.

```mermaid
flowchart TD
    sig_chirho[Signature context equalities in ast_type_to_scheme_seeded_chirho]
    sig_chirho --> decide_chirho{Plain unification decides it?}
    decide_chirho -->|yes| subst_chirho[Applied as a substitution to the scheme]
    decide_chirho -->|no: stuck family or closed-type clash| pred_chirho[Kept as a `~` predicate]
    pred_chirho --> check_chirho[Checking the binding: install_given_equalities_chirho]
    check_chirho --> plus_chirho[`+` interaction: x + y ~ x + z gives y ~ z]
    plus_chirho --> rules_chirho[given_rewrites_chirho: rigid or family side ↦ other side]
    rules_chirho --> normalize_chirho[normalize_ty_chirho applies refinements, then rewrites]
    pred_chirho --> use_chirho[Using the binding: instantiate_chirho]
    use_chirho --> wanted_chirho[deferred_equalities_chirho]
    unify_chirho[unify_normalized_chirho meets a family stuck on a variable] --> wanted_chirho
    unify_chirho --> assign_chirho{Assign whole application to a metavariable?}
    assign_chirho -->|yes, occurs check passes| subst_chirho
    assign_chirho -->|no: would invert family arguments| wanted_chirho
    wanted_chirho --> retry_chirho[retry_deferred_equalities_chirho at equation end, given scope end, module end]
    retry_chirho -->|solved side| unify2_chirho[Unify and apply]
    retry_chirho -->|still stuck| owner_chirho[Keep the equality with its implication owner and evidence]
    owner_chirho --> inferred_chirho[Inferred binding: retain a generalizable ~ predicate]
    owner_chirho --> checked_chirho[Checked binding or SCC closure: prove, retain valid generalization, or report]
    owner_chirho --> legacy_chirho[Unmodeled or unowned legacy path: conservative terminal handling]
    retry_chirho -->|closed mismatch| report_chirho[E0200 at the original span]
    assoc_chirho[Associated family default] --> instance_chirho[Instantiated per instance that leaves the family undefined]
    instance_chirho --> normalize_chirho
```

## Invariants

- An associated type family's default is never a general equation of the family: `type Fam a =
  Maybe a` inside a class produces `Fam Int = Maybe Int` for `instance Cls Int`, and nothing for an
  abstract `a`.
- Deferral happens only when a family application is stuck on a *unification variable*
  (`ty_is_stuck_family_on_var_chirho`); a family stuck on a rigid variable is decided by the given
  rewrite rules or reported.
- Structural-unification success is not evidence of injectivity.
  `defer_stuck_family_equality_chirho` also defers `G a ~ G b` rather than deriving
  `a ~ b`; assigning an entire application to a metavariable is still allowed when the
  occurs check passes. The GHC 9.14.1 control is a type-changing record update (T10808)
  where `G T1 = G T2 = Int` while another field changes Char to Bool: checking
  `y = y r1` against the output record's field type meets `G t8 ~ G t7`, both stuck on
  unsolved variables, and deriving `t8 ~ t7` forced the output back to `R T1`
  (observed with an instrumented build, 2026-09-19). Carried from row 484's 327aaef7.
  This is the noninjective default, not implementation of user-written
  [injectivity annotations](https://downloads.haskell.org/ghc/latest/docs/users_guide/exts/type_families.html#injective-type-families).
- Rewrite rules are oriented from the side that cannot otherwise be simplified (a rigid variable
  or a family application) to the other side; two differing closed types (`Int ~ Bool`) also
  become a rule, which is how an insoluble given types its unreachable body.
- Given rewrites and deferred equalities follow the running substitution
  (`apply_subst_all_chirho`), and rules are truncated with the givens they came from.
- `~` never reaches class reasoning: it is skipped by the unsolvable and undeducible checks, and
  instantiating a scheme turns a `~` predicate into a deferred equality, not a class wanted.
- Discharging wanteds at a given scope's end also solves through instances whose contexts the
  givens entail (`pred_entailment_under_givens_chirho`), and improves wanteds by the functional
  dependencies of the givens in scope (`fundep_improve_from_givens_chirho`). A wanted that
  instances settle on their own — the `Num Int` of a literal — is NOT discharged there
  (`givens_discharge_wanted_chirho`): it stays deferred so generalization absorbs it into the
  binding's scheme, which is where the dictionary pass reads the evidence for the body's class
  methods. Discharging it early left `take 5 "hello"` with a bare `fromInteger` at run time.

## Current boundary

- A constraint written with type operators (`a + b ~ a + c`) is mis-lowered by the parser as a
  class constraint headed by `+`; the `+` interaction rule is therefore only reachable through
  the AST shapes the lowering produces today (parser lane).
- Injectivity of user-declared families is not modelled; only the builtin `+` interaction exists.
- The owned equality route below rejects an unsolved checked/SCC obligation even when both
  sides remain open or the signature introduces no skolem. Unowned and unsupported legacy paths
  retain their conservative terminal behavior; this is not a complete ambiguity checker.


## Row 686: identity and ordered reduction

The row 686 combined candidate connects preserved written equations to one immutable family definition. This section describes that candidate's route; the tasklist records its pending combined compiler and corpus gates. A parser carrier or isolated component test does not establish the whole route.

```mermaid
flowchart TD
    written_chirho[Written family equations with heads, binders, spans and closedness]
    written_chirho --> validity_chirho[Naming and written-declaration validity]
    validity_chirho --> scope_chirho[Resolve the family in the module's actual import scope]
    scope_chirho --> registry_chirho[One opaque family identity and fixed visible arity]
    registry_chirho --> registration_chirho[Equation-local variables and distinct wildcard variables]
    registration_chirho --> compatibility_chirho[Freshen both left and right sides for compatibility]
    compatibility_chirho --> blockers_chirho[Keep earlier incompatible or uncertain blocker indices]
    blockers_chirho --> snapshot_chirho[Immutable definition shared by export aliases]
    snapshot_chirho --> imports_chirho[Replace cumulative snapshots and deduplicate independent merges]
    imports_chirho --> target_chirho[Normalize a target under one shared work and output budget]
    target_chirho --> lookup_chirho[Exact selected identity, then one saturation split]
    lookup_chirho --> match_chirho{One-way branch match?}
    match_chirho -->|no| next_chirho[Try next written branch]
    match_chirho -->|yes| apart_chirho{Every blocker surely apart?}
    apart_chirho -->|yes| rhs_chirho[Substitute RHS, then apply extra result arguments]
    apart_chirho -->|no or uncertain| stuck_chirho[Leave application stuck]
    lookup_chirho -->|empty, ambiguous, held or budget exhausted| stuck_chirho
    rhs_chirho --> target_chirho
```

`TypeFamilyRegistryChirho` stores definitions once, with stable branch identities and module-owned export aliases. Open definitions require a cold coherence check. Re-importing the same verified rows keeps that proof; adding unverified rows invalidates it. A closed declaration's first held equation prevents every later fallback from bypassing it. Exact lookup never tries a different qualified, bare or builtin definition when the selected family stays stuck.

The apartness unifier and one-way matcher have different jobs. Apartness freshens each branch's left and right sides together, binds both sides' variables, preserves shared stuck applications and compares the substituted results. Occurs failures, unknown heads and exhausted resources are uncertain. Matching binds only the equation's pattern variables. List, tuple and function sugar shares a canonical view, with arrow multiplicity and promoted-constructor identity preserved.

Promoted names use the separate promoted-constructor identity workflow. Ordinary source type spellings do not acquire nominal authority from a qualifier: an import alias may spell a different transitive module's name. The comparison catalogue therefore leaves raw qualified heads unknown unless a producer supplied an opaque identity. Existing exporter stamps, including unresolved stamps, survive later imports unchanged. Diagnostic rendering writes readable family and promoted names without changing internal equality or keys.

The behavioral controls reuse the existing `typing_integration` and `native_symbol_identity` targets. Their single-module and Cabal project programs are copied byte for byte from fresh GHC 9.14.1 runs, including abstract rejection twins and qualified/re-exported identities. The component suite separately checks finite/uncertain/disjoint comparisons, coherence, snapshot deduplication and resource exhaustion. Final claims require the interpreter, LLVM, true Cranelift and both fixed corpus axes on the same candidate.

## A signature's wanted that nothing in scope can serve (row 686, C08 prototype)

This records the original C08 prototype and its support boundary. The R5 owner-frame route below supersedes its description of equality ownership and locally produced constructor contexts. The conservative C08 class-wanted guard still exists; it never decides whether an owned `~` obligation is valid.

`twoChirho :: Proxy a -> Proxy b -> TwoChirho (OpaqueChirho a) (OpaqueChirho b); twoChirho _ _ = 3` leaves the literal's wanted, `Num (TwoChirho (OpaqueChirho a) (OpaqueChirho b))`, resolved and meta-free at the signature scope's end. Discharge keeps it, because no given takes part. The scope then truncates its givens, and the module-end check's rigid branch drops it. GHC 9.14.1 rejects the program with GHC-39999.

```mermaid
flowchart TD
    scope_chirho["with_signature_givens_chirho: givens and equality rewrites installed"] --> body_chirho["Body checked against the rigid signature; substitutions applied to the wanteds"]
    body_chirho --> discharge_chirho["Discharge with the givens; retry deferred equalities"]
    discharge_chirho --> own_chirho{"Names this signature's skolems, carries no inference variable?"}
    own_chirho -->|No| keep_chirho["Stays deferred, as before"]
    own_chirho -->|Yes| entailed_chirho{"Entailed under the givens, raw or normalized?"}
    entailed_chirho -->|Yes| keep_chirho
    entailed_chirho -->|No| certified_chirho{"An argument is a certified irreducible family application, and no instance head of the visible class is a bare variable at that position?"}
    certified_chirho -->|No| keep_chirho
    certified_chirho -->|Yes| report_chirho["no instance for: reported, and removed from the deferred list"]
    report_chirho --> truncate_chirho["Givens and rewrites leave scope"]
    keep_chirho --> truncate_chirho
```

- **Placement.** The check runs at the signature scope's end, from `with_signature_givens_chirho` in `solving_chirho/schemes_chirho.rs`, while the givens and equality rewrites are still installed. Top-level and local signed bindings call it. Instance-obligation scopes keep `with_given_preds_chirho`, unchanged. The rule is in `solving_chirho/signature_wanteds_chirho.rs`.
- **The certificate.** It consumes the reducer's `decide_family_with_limits_chirho` through exact registry identities.
  - `NoMatch` certifies a definition that has no equations.
  - Otherwise, `NoMatch` or `Blocked` certifies only when every rigid variable sits inside a nested application that is itself certified. A pattern's local refinement of a bare rigid argument could select an equation.
  - Intrinsic, held, unverified, Unknown and exhausted work never certifies.
- **The class.** Only a bare-variable instance head can match an application that never reduces. Its visibility is trusted for a fully-known class, or for a class the standard seed itself produced (`seeded_class_names_chirho`).
  - That set is captured from the seed producer's output.
  - An ordinary import such as `Data.Proxy` leaves it alone; a whole-environment replacement clears it, and a source declaration of a class removes that name.
  - Boundary: an orphan universal instance in an uncompiled package stays outside this view.
- **Original unmodeled pattern-evidence boundary.** The initial prototype did not install constructor contexts or GADT refinements as pattern-local givens; constructor instantiation deferred a context as a wanted.
  - So `unmodeled_pattern_evidence_seen_chirho` is set the first time any pattern matches such a constructor, or one whose scheme is unknown, and the first time a refinement is added.
  - Once set, every later closure in the module stays off, nested signatures included, since a nested signature can inherit the evidence before its own scope begins.
  - The module-wide guard remains conservative for the C08 class refuter: an unrelated later signature can stay unchecked. R5 adds actual local typing givens, but does not remove that guard or establish runtime constructor dictionary transport.
  - The original GADT scheme producer dropped its written context, so the declaration producer added `context_constructors_chirho`. R5 preserves locally produced contexts through signature conversion; the earlier source mark and flag remain.
  - Imported placeholder/stub schemes still supply no complete constructor-context evidence. R5 can use a real supplied scheme, but does not claim complete imported-constructor transport.
- **Initial reducer measurement, retained as history.**
  - `TwoChirho (OpaqueChirho a) (OpaqueChirho b)`, two different stuck applications against the blocker `TwoChirho Int Bool`, decides `Unknown`, not `Blocked`: the apartness kernel stays uncertain.
  - Those initial C08 acceptances were RED. Later reducer/blocker work and the existing C08 controls are separate from the owner-frame measurements below; the initial Unknown result is not a current acceptance claim.
  - A single open family with no equations (C08OpenOnly) decides `NoMatch` and is reported.
- **Unchanged:** the module-end rigid refuter, every family blocker, and every other caller. Nothing else calls a stuck family unsolvable.
- **Controls:** `typing_integration_chirho/declarations_chirho/signature_wanteds_chirho.rs`, copied byte for byte from fresh GHC 9.14.1 oracle files.
  - One irreducible negative.
  - Two negatives that wait on the reducer's verdict on the blocker.
  - Eight accepted programs, the two constructor-pattern holds among them.


## R5: owned equality implications and constructor-pattern evidence

This is the held November 3 integration candidate, not a completed row or a compatibility publication. An equality wanted now stores its source span and implication owner. A scope retains only its new evidence plus a shared parent link; entering a pattern does not clone the growing type environment. The implementation lives in `solving_chirho/equality_scopes_chirho.rs`, `solving_chirho/equality_normalization_chirho.rs` and `terms_chirho/constructor_patterns_chirho.rs`, with short hooks in equations, cases, lambdas and binding groups.

```mermaid
flowchart TD
    binding_chirho[Enter source binding: inferred or checked owner]
    binding_chirho --> pattern_chirho[Enter equation, case alternative or lambda implication]
    pattern_chirho --> constructor_chirho[Open the real constructor scheme]
    constructor_chirho --> universal_chirho[Result binders determined through generative structure become flexible]
    constructor_chirho --> existential_chirho[Field-only or family-hidden binders become local rigid skolems]
    constructor_chirho --> unknown_chirho[Unknown constructor shape supplies no invented evidence]
    universal_chirho --> result_chirho[Meet constructor result with the same datatype head]
    existential_chirho --> result_chirho
    result_chirho --> givens_chirho[Install actual result equalities and written constructor contexts as local givens]
    givens_chirho --> body_chirho[Infer body and nested bindings; wanteds retain their owner]
    body_chirho --> solve_chirho[Discharge and retry before evidence or environment scopes pop]
    solve_chirho --> escape_chirho[Check outgoing type and changed older metas for existential escape]
    escape_chirho --> public_chirho[Materialize safe public images; overlay replacements after normal substitution composition]
    public_chirho --> pop_chirho[Pop local givens and refinements]
    pop_chirho --> group_chirho[Source SCC closure]
    group_chirho --> generalized_chirho[Retain only equalities actually included in every inferred member scheme]
    group_chirho --> rejected_chirho[Report unresolved checked or nongeneralizable obligation]
    unknown_chirho --> boundary_chirho[Keep unsupported implication boundary explicit]
    solve_chirho --> budget_chirho[Shared normalization work or output allowance exhausted]
    budget_chirho --> unproved_chirho[Keep unproved; never certify or generalize; owning check reports bounded failure]
```

- A checked binding cannot silently acquire a missing equality, including a monomorphic signature whose result leaves hidden inference variables in a wanted. An inferred local binding has its own generalization boundary even inside an outer checked function. Generalization retains a complete `~` predicate; free-variable occurrence alone is not proof that another recursive group member retained it. An unresolved equality cannot be certified through a checked sibling whose temporary inferred scheme is later replaced by its written signature.
- Pattern instantiation is distinct from expression instantiation. Universals are recognized through positively generative datatype/list/tuple/function structure and resolved promoted constructors. A family or unknown promoted head supplies no injectivity. Field-only existential variables remain rigid. Expression uses retain flexible ordinary instantiation and wanted constraints; a pattern's givens are not a license to invent evidence at an expression use.
- A GADT result comparison keeps family components whole. A result such as `Equal (Opaque a) Int` can provide the local equality needed by `Refl`; it does not prove equality of the arguments of a noninjective family. Each case alternative must meet the common result under its own evidence. Discarding a mismatch after an earlier alternative fixed that result was the measured R3 `CaseLeakedBranch` wrong acceptance; R5 refuses it.
- Scope exit examines both the outgoing result and the delta of substitutions for metas created before the pattern. Thus an existential cannot escape through an older function argument while the result itself is closed. If local evidence eliminates the existential, the safe public image is installed and carried out explicitly. Ordinary substitution composition preserves older same-domain images, so the pattern solution separately overlays these public replacements during accumulation.
- One equality normalization allowance covers evidence preparation, name comparisons, sparse substitution chasing, synonym expansion, every fixed-point pass and family reduction: at most 10,000,000 work units, 16,384 output units and depth 128, with an additional 100-pass limit. The changed-meta log is capped at 16,384 keys. A growing rewrite, cyclic rewrite or overlong evidence chain yields an explicit unproved/exhausted result. It is never equal to a fabricated sentinel, removed as solved or generalized as a successful constraint. The owning supported check reports the bounded failure.
- Unknown family heads, held branches, incomplete constructor schemes and unsupported legacy paths do not gain a proof from this machinery. The C08 module-wide class-wanted guard remains a separate conservative boundary. There is no unconditional module-final rejection of every pending equality.

### Measured checkpoint and remaining boundaries

Root executed the frozen R5 integration source: the typing library passes 573/573 and Core passes 214/214. The 21 new typing controls cover normalization growth/cycles, ownership and generalization, SCC predicate retention, local givens, generative/promotion identity, rigid existentials, older-meta escape, public-image materialization and the case mismatch. The driver normal run passes 18/19 tests: all six measured negative programs are refused, all eleven positive programs check, and ten positive programs match fresh GHC 9.14.1 on the interpreter, LLVM and true Cranelift. The exact tracked T16512a, T23162b and T23162d sources are refused. These are focused results, not fresh fixed-axis totals or implementation of the missing user-family injectivity annotations.

Receipts are `tmp-chirho/equality-r5-sorts-core-tests-r3-chirho.log` and `tmp-chirho/equality-r5-driver-cases-chirho.log`; the earlier R3 RED is retained in `tmp-chirho/equality-frames-r3-driver-tests-chirho.log`.

The sole driver RED is `RecordContext`: it checks, but its interpreter run reports `no binding named main`. That parser/complete GADT-record carrier gap is pre-existing on the retained baseline, and a separate writer owns it. This run stops that individual program at the interpreter failure; it establishes no native runtime result for that program. Imported constructor contexts are usable only when a real complete scheme reaches inference; placeholder interfaces do not prove their field/context shape. Installing a class predicate as a typing given does not prove that runtime constructor dictionary packing/extraction exists. The separate class-context probes and source SCC/local-owner probes remain required boundaries, not runtime wins attributed to this equality unit.

Row 686 remains held for the combined source, regression and fixed-axis gates. The unchanged fixture manifests and both committed measurement artifacts remain the authority for published compatibility numbers.
