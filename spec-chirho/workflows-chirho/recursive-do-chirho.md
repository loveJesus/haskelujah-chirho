*For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)*

# RecursiveDo workflow

This workflow keeps `mdo` and `rec` contextual, lowers recursive statement groups before type
inference, and represents the recursive values as one lazy `mfix` knot.

```mermaid
flowchart TD
    source_chirho[Haskell source] --> raw_lexer_chirho[Raw lexer: mdo and rec are VarId]
    raw_lexer_chirho --> pragma_tokens_chirho[Read actual pragma tokens]
    pragma_tokens_chirho --> extension_state_chirho{RecursiveDo enabled?}
    extension_state_chirho -->|No| ordinary_tokens_chirho[Keep ordinary identifiers]
    extension_state_chirho -->|Yes| contextual_tokens_chirho[Reclassify mdo and rec]
    contextual_tokens_chirho --> layout_chirho[Layout: do, mdo, and rec open blocks]
    layout_chirho --> cst_chirho[CST with RecStmt groups]
    cst_chirho --> ast_transform_chirho[CST-to-AST RecursiveDo transform]
    ast_transform_chirho --> dependency_check_chirho{Forward or bind-self dependency?}
    dependency_check_chirho -->|No| sequential_do_chirho[Keep ordinary sequential do AST]
    dependency_check_chirho -->|Yes| mfix_knot_chirho[Ordinary do AST with one mfix knot]
    sequential_do_chirho --> monadfix_dict_chirho
    mfix_knot_chirho --> lazy_projection_chirho[Core lazy tuple projections]
    lazy_projection_chirho --> monadfix_dict_chirho[MonadFix dictionary selection]
    monadfix_dict_chirho --> stg_knot_chirho[STG letrec result thunk and update]
    stg_knot_chirho --> lazy_return_chirho[ReturnIO preserves the result thunk]
    lazy_return_chirho --> observable_result_chirho[Driver forces only the final observable result]
```

## Invariants

- Strings and ordinary comments never enable an extension.
- Later `NoRecursiveDo` flags override earlier `RecursiveDo` flags.
- RecursiveDo syntax does not add a permanent AST variant after lowering.
- `mdo` prefixes without a forward or bind-self dependency remain sequential.
- The knot tuple is projected lazily; desugaring must not force it before `mfix` produces it.
- ReturnIO preserves its payload thunk; forcing occurs at strict uses or the observable driver
  boundary.
- Existing IO `do` lowering remains on its proven primitive path.

## Landing state

- Frontend contextual keyword, layout, and CST support: implemented.
- CST-to-AST dependency segmentation and knot transform: implemented.
- Lazy Core tuple projections and IO/Maybe/list MonadFix dictionaries: implemented.
- STG letrec knot behavior, lazy ReturnIO payloads, and final-result forcing boundary:
  implemented.
