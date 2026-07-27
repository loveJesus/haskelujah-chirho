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
    ast_transform_chirho --> mfix_knot_chirho[Ordinary do AST with one mfix knot]
    mfix_knot_chirho --> lazy_projection_chirho[Core lazy tuple projections]
    lazy_projection_chirho --> monadfix_dict_chirho[MonadFix dictionary selection]
    monadfix_dict_chirho --> stg_knot_chirho[STG result thunk and update]
    stg_knot_chirho --> observable_result_chirho[Recursive values observed on demand]
```

## Invariants

- Strings and ordinary comments never enable an extension.
- Later `NoRecursiveDo` flags override earlier `RecursiveDo` flags.
- RecursiveDo syntax does not add a permanent AST variant after lowering.
- The knot tuple is projected lazily; desugaring must not force it before `mfix` produces it.
- Existing IO `do` lowering remains on its proven primitive path.

## Landing state

- Frontend contextual keyword, layout, and CST support: implemented.
- CST-to-AST knot transform, lazy Core projections, MonadFix dictionaries, and STG behavior:
  pending in the same tasklist.
