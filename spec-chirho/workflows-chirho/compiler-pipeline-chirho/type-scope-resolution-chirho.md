<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Type-scope resolution workflow

This workflow is owned by `resolve_module_with_imports_chirho` and `check_module_type_scope_chirho`. It runs after imports and all module-local type definitions have populated `NameEnvChirho`, but before kind and type inference.

```mermaid
flowchart TD
    module_ast_chirho[Lowered module AST] --> imports_chirho[Resolve imported interfaces]
    imports_chirho --> import_result_chirho{Every interface available?}
    import_result_chirho -->|No| import_error_chirho[Retain causal import diagnostic and skip dependent lookups]
    import_result_chirho -->|Yes| local_defs_chirho[Bind all local type constructors and classes]
    local_defs_chirho --> associated_defs_chirho[Bind associated type-family names]
    associated_defs_chirho --> walk_uses_chirho[Walk source-level type uses]

    walk_uses_chirho --> tycon_use_chirho{Type constructor or class use?}
    tycon_use_chirho -->|Yes| wired_type_chirho{Wired-in equality, active StarIsType, list, function, unit or tuple constructor?}
    wired_type_chirho -->|Yes| continue_chirho
    wired_type_chirho -->|No| qualified_chirho{Qualified?}
    qualified_chirho -->|No| unqualified_lookup_chirho[Lookup type namespace then DataKinds constructor namespace when licensed]
    qualified_chirho -->|Yes| qualified_lookup_chirho[Lookup current-module local or imported qualified namespace then licensed constructor namespace]
    unqualified_lookup_chirho --> lookup_result_chirho{Found?}
    qualified_lookup_chirho --> lookup_result_chirho
    lookup_result_chirho -->|No| undefined_type_chirho[Emit E0101 once per source use]
    lookup_result_chirho -->|Yes| continue_chirho[Continue bounded AST walk]

    walk_uses_chirho --> promoted_use_chirho{Explicit promoted constructor?}
    promoted_use_chirho --> wired_promoted_chirho{Wired-in list, unit or tuple constructor?}
    wired_promoted_chirho -->|Yes| continue_chirho
    wired_promoted_chirho -->|No| promoted_lookup_chirho[Lookup value/data-constructor namespace]
    promoted_lookup_chirho --> lookup_result_chirho

    walk_uses_chirho --> tyvar_use_chirho{Type-variable use?}
    tyvar_use_chirho --> explicit_scope_chirho[Track declaration and explicit forall binders]
    explicit_scope_chirho --> policy_chirho{Declaration policy}
    policy_chirho -->|Signature without outer forall, existential or family scope| implicit_quantification_chirho[Preserve permitted implicit quantification]
    policy_chirho -->|Type synonym or ordinary field| lexical_lookup_chirho{Lexically bound?}
    lexical_lookup_chirho -->|No| undefined_tyvar_chirho[Emit E0101 type-variable diagnostic]
    lexical_lookup_chirho -->|Yes| continue_chirho

    explicit_scope_chirho --> binder_order_chirho[Check explicit forall binder annotations left-to-right]
    binder_order_chirho --> prior_binder_chirho{Kind variable introduced earlier?}
    prior_binder_chirho -->|No| undefined_tyvar_chirho
    prior_binder_chirho -->|Yes| continue_chirho

    walk_uses_chirho --> signature_root_chirho{Outermost invisible forall?}
    walk_uses_chirho --> gadt_header_chirho{GADT constructor signature?}
    gadt_header_chirho --> independent_gadt_scope_chirho[Do not inherit declaration-head binders]
    independent_gadt_scope_chirho --> signature_root_chirho
    signature_root_chirho -->|Yes| forall_all_chirho[Require every free variable to be explicitly or lexically bound]
    signature_root_chirho -->|No| implicit_signature_chirho[Implicitly quantify otherwise-free signature variables]
    forall_all_chirho --> lexical_lookup_chirho
    implicit_signature_chirho --> continue_chirho

    walk_uses_chirho --> record_field_chirho{Record field contains unreliable invisible-forall or quantified syntax?}
    record_field_chirho -->|Yes| defer_record_chirho[Defer until flat record-field lowering preserves source scope]
    record_field_chirho -->|No| continue_chirho

    undefined_type_chirho --> driver_gate_chirho[Driver stops before kind inference unless errors are deferred]
    undefined_tyvar_chirho --> driver_gate_chirho
```

The walker performs no filesystem lookup or module scanning. Its successful-resolution path is linear in the lowered AST with hash-backed namespace lookups; the error-only suggestion path compares against the already-populated in-memory environment, and duplicate diagnostics are suppressed by issue/name/span. The record-field deferral is an AST trust boundary for the open `bug-record-field-forall-lowering-chirho.md`; ordinary and required-forall record fields remain covered, and the branch can disappear when that parser fix lands.
