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
- Structural-unification success is not evidence of injectivity. In the isolated
  row484 branch, `defer_stuck_family_equality_chirho` also defers `G a ~ G b`
  rather than deriving `a ~ b`; assigning an entire application to a metavariable
  is still allowed when the occurs check passes. A GHC9.14.1/STG record-update
  control has `G T1 = G T2 = Int` while another field changes Char to Bool.
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
