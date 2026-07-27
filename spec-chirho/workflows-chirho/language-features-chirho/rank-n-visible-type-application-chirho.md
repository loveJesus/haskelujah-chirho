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
    core_chirho --> gates_chirho[T17594f plus focused and suite gates]
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
- Existing top-level scheme-variable instantiation remains the first path.
- Unsupported or excessive type arguments remain diagnostics/fallbacks rather than silently
  changing an unrelated monomorphic type.

## Current boundary

- Required declaration arguments are matched positionally; named required binders are not yet
  added to the scoped type-variable environment of the equation body.
- Template Haskell reification currently maps required and invisible foralls to the existing
  `ForallTChirho` representation because the TH type AST does not yet encode binder visibility.
