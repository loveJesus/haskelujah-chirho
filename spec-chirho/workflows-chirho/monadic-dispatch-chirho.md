<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Workflow: typeclass method dispatch (Functor/Applicative/Monad)

How a class-method use (`fmap`, `<*>`, `>>=`, `>>`, `return`/`pure`) travels the pipeline,
and where per-type dispatch happens. Functions on this DAG carry a
`// workflow: monadic-dispatch-chirho` comment in code.

```mermaid
flowchart TD
    SRC["Haskell source\ndo-block / operator use"] --> DESUGAR["desugar_chirho.rs\ndesugar_expr_chirho (ops ~3886-3900)\ndesugar_do_chirho (~4913)"]
    DESUGAR -->|"do stmts → >>= / >> chains (WI-002)\noperators → Var apps (WI-001)"| CORE["Core IR"]
    CORE --> DICTPASS["dict_chirho/rewrite_chirho.rs\nrewrite_method_refs_with_locals_chirho (~483)"]
    DICTPASS -->|"App spine, head is class method"| HK["try_dispatch_hk_method_chirho (~409)\ninfer arg type key →\nnormalize_instance_head_key_chirho (~380)"]
    HK -->|"key resolved, body exists"| PRIM["$prim_Class_method_Type body\ndict_chirho/prelude_chirho.rs\n(e.g. $prim_Monad_>>=_Maybe)"]
    HK -->|"no key / no body"| SEL["try_rewrite_method_var_chirho (~316)\n$sel_Class_method $dClass_Type\n(dict selector path — GPT-owned)"]
    SEL -->|"no dict either: Var survives untouched"| FALLBACK["Var keeps original name"]
    PRIM --> STG["driver stg_lower_chirho.rs"]
    FALLBACK --> STG
    STG -->|"name fallback ~1668-1670:\n'>>='|'bindIO#' → BindIOChirho\n'>>'|'thenIO#' → ThenIOChirho\n'return'|'pure'|'returnIO#' → ReturnIOChirho"| MACHINE["runtime MachineChirho\nIO primops preserve today's behavior\n(invariant INV-001, GPT WI-D)"]
```

Key invariant (INV-001): user-level `>>=`/`>>`/`return`/`pure` may only be *positively*
dispatched to a `$prim_*` body when the type key resolves and the body exists; every
unresolved case must reach STG under its original name so lines 1668-1670 keep the
IO fast path byte-identical. IO primops (`bindIO#`/`thenIO#`/`returnIO#`) stay primops.

See `spec-chirho/prd_chirho.json` work items WI-001 (operators), WI-002 (do-notation),
WI-003 (return/pure position) for the active changes on this DAG.
