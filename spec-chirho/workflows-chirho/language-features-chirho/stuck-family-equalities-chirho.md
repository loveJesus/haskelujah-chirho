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
    retry_chirho -->|still stuck| wait_chirho[Keep waiting; dropped at module end]
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
- Equalities still stuck on an ambiguous variable at the end of the module are dropped rather
  than reported as ambiguity.


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
