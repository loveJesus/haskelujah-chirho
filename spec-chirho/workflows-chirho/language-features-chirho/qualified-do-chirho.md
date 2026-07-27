<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Qualified do workflow

```mermaid
flowchart TD
    source_chirho[Source token M.do] --> layout_chirho[Layout recognizes qualified do opener]
    layout_chirho --> parser_chirho[CST DoExpr preserves qualified keyword token]
    parser_chirho --> lower_chirho[AST stores optional module qualifier]
    lower_chirho --> typing_chirho[Type inference checks statements in lexical order]
    typing_chirho --> desugar_chirho[Core desugaring selects M.>>=, M.>>, and M.fail]
    desugar_chirho --> runtime_chirho[STG/native execution uses the qualified methods]
    runtime_chirho --> gates_chirho[QualifiedDo plus ordinary do and RecursiveDo gates]
```

## Invariants

- A qualified do opener is a layout keyword only when the final qualified segment is exactly
  `do`; identifiers such as `Module.done` remain ordinary qualified identifiers.
- The qualifier is semantic AST data, not reconstructed later from source text.
- Missing qualified methods fail through normal name resolution; they do not fall back silently
  to Prelude methods.
- Imported Prelude aliases use the existing bare-export linker bridge. Other imported qualifiers
  remain qualified and loud until Core linking distinguishes same-named exports by module.
- Unqualified `do` and `mdo` retain their existing lowering and dispatch paths.
