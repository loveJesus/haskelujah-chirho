<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Rank-N visible type application workflow

```mermaid
flowchart TD
    source_chirho[Required forall, term args, at-type args, and type lambdas]
    source_chirho --> cst_chirho[CST separates required forall and type-lambda binders]
    cst_chirho --> spine_chirho[Close term AppExpr before wrapping TypeAppExpr]
    spine_chirho --> ast_chirho[AST preserves RequiredForall and TypeLam]
    ast_chirho --> kind_chirho[Kind inference scopes both forall visibilities]
    kind_chirho --> scheme_chirho[Type inference instantiates ordinary scheme variables]
    scheme_chirho --> required_chirho{Outermost binder visibility}
    required_chirho -->|forall a ->| ordinary_arg_chirho[Convert next ordinary argument to a type]
    required_chirho -->|forall a.| visible_arg_chirho[Consume next at-type argument]
    ordinary_arg_chirho --> substitution_chirho[Substitute binder in remaining body]
    visible_arg_chirho --> substitution_chirho
    ast_chirho --> type_lambda_chirho[Check TypeLam against outer invisible forall]
    type_lambda_chirho --> core_chirho[Desugar to Core TyLam]
    substitution_chirho --> term_chirho[Infer following term arguments]
    scheme_chirho --> equation_chirho[Consume equation patterns and retain residual function type]
    equation_chirho --> expected_chirho[Check RHS against residual expected type]
    expected_chirho --> application_chirho[Constrain application result before checking argument]
    application_chirho --> family_chirho[Reduce family-dependent argument type]
    expected_chirho --> branch_chirho{Polymorphic expected result}
    branch_chirho -->|if| if_chirho[Check both branches against expected type]
    branch_chirho -->|case| case_chirho[Check each scoped alternative against expected type]
    if_chirho --> lambda_chirho[Bind rank-N lambda parameter as fresh-per-use scheme]
    case_chirho --> lambda_chirho
    lambda_chirho --> gates_chirho[T17594f, Vta1, Vta2, and bounded suite gates]
    core_chirho --> gates_chirho
    term_chirho --> gates_chirho
```

## Invariants

- Explicit type arguments consume quantified variables in source order.
- Required arguments consume only `forall a ->`; ordinary term applications cannot erase an
  invisible `forall a.` binder.
- Quantifiers nested inside a rank-N parameter are instantiated only when they are outermost at
  the application site; type application never skips a function arrow.
- Mixed application spines such as `f typeArg @Visible value` remain inside one equation and do
  not absorb the following declaration.
- Type-lambda binder order is preserved when term and type binders are mixed.
- An equation may consume fewer term arguments than its signature; its RHS is checked against
  the residual function type rather than falling back to unconstrained inference.
- When an application result is known, shared type variables are specialized before checking
  family-dependent argument types, so `F Char ~ Bool` can guide a `Bool` argument.
- Polymorphic expected types flow through `if` branches and scoped `case` alternatives before
  branch lambdas are inferred; each use of a rank-N lambda parameter is freshly instantiated.
- Monomorphic and GADT branch inference keeps its established lenient merge path.
- Existing top-level scheme-variable instantiation remains the first path.
- Unsupported or excessive type arguments remain diagnostics/fallbacks rather than silently
  changing an unrelated monomorphic type.

## Current boundary

- Required declaration arguments are matched positionally; named required binders are not yet
  added to the scoped type-variable environment of the equation body.
- Template Haskell reification currently maps required and invisible foralls to the existing
  `ForallTChirho` representation because the TH type AST does not yet encode binder visibility.
