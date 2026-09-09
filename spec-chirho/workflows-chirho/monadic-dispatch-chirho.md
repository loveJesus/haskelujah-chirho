<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Workflow: typeclass method dispatch (Functor/Applicative/Monad/Alternative)

How a class-method use (`fmap`, `<*>`, `<|>`, `>>=`, `>>`, `return`/`pure`) travels the pipeline,
and where per-type dispatch happens. Functions on this DAG carry a
`// workflow: monadic-dispatch-chirho` comment in code.

```mermaid
flowchart TD
    SRC["Haskell source\ndo-block / operator use"] --> DESUGAR["desugar_chirho.rs\ndesugar_expr_chirho (ops ~3886-3900)\ndesugar_do_chirho (~4913)"]
    DESUGAR -->|"do stmts → >>= / >> chains (WI-002)\noperators → Var apps (WI-001)"| CORE["Core IR"]
    CORE --> DICTPASS["dict_chirho/rewrite_chirho.rs\nrewrite_method_refs_with_locals_chirho"]
    DICTPASS --> OWN["Own dictionary evidence before monad-context guesses"]
    OWN -->|"scheme supplies the dictionary"| SEL
    OWN -->|"no own dictionary"| HK["try_dispatch_hk_method_chirho\ninfer arg type key →\nnormalize_instance_head_key_chirho"]
    HK -->|"typed method argument\n(e.g. empty :: [Int])"| TYPEDARG["try_rewrite_typed_method_arg_chirho\nreuse selected instance key"]
    TYPEDARG --> PRIM
    HK -->|"key resolved, body exists"| PRIM["$prim_Class_method_Type body\ndict_chirho/prelude_chirho.rs\n(e.g. $prim_Monad_>>=_Maybe)"]
    HK -->|"no key / no body"| SEL["try_rewrite_method_var_chirho (~316)\n$sel_Class_method $dClass_Type\n(dict selector path — GPT-owned)"]
    SEL -->|"no dict either: Var survives untouched"| FALLBACK["Var keeps original name"]
    PRIM --> ACTIONS["prepare_io_actions_chirho\nIO action functions and lazy result packets"]
    FALLBACK --> ACTIONS
    ACTIONS --> STG["driver stg_lower_chirho.rs"]
    STG --> MACHINE["runtime MachineChirho\nexecute effects only when sequencing actions"]
```

Key invariant: concrete dispatch requires a resolved type key and an actual body.
An enclosing binding's own dictionary is also real evidence and takes precedence
over a syntactic monad-context guess. Both `pure` and `return` use the Applicative
pure selector there; Monad supplies its Applicative superclass. Unknown instances
must not be guessed merely from the result's runtime bits.

The legacy unresolved-name fallback remains for cases with neither evidence nor a
known body; it is not a claim of general polymorphic-monad compatibility. The old
identity-style IO fast-path contract is superseded by reusable action functions:
Core IO operations become dormant action values, and execution yields a distinct
result packet. See testing-chirho/execution-oracles-chirho.md.

See `spec-chirho/prd_chirho.json` work items WI-001 (operators), WI-002 (do-notation),
WI-003 (return/pure position) for the active changes on this DAG.
