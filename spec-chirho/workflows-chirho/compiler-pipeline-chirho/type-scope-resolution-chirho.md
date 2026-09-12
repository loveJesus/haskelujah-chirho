<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Type-scope resolution workflow

This workflow is owned by `resolve_module_with_imports_chirho` and `check_module_type_scope_chirho`. It runs after imports and all module-local type definitions have populated `NameEnvChirho`, but before kind and type inference.

```mermaid
flowchart TD
    lexed_tokens_chirho[Lexed source tokens] --> layout_normalization_chirho[Normalize explicit and implicit layout contexts]
    layout_normalization_chirho --> explicit_close_chirho{Explicit right brace closes nested implicit contexts?}
    explicit_close_chirho -->|Yes| unwind_layout_chirho[Emit virtual closes before the explicit close]
    explicit_close_chirho -->|No| source_cst_chirho[Source CST]
    unwind_layout_chirho --> source_cst_chirho
    source_cst_chirho --> faithful_lowering_chirho[Lower namespace-bearing declaration shapes]
    faithful_lowering_chirho --> lowered_shape_chirho{Shape represented faithfully?}
    lowered_shape_chirho -->|Yes| module_ast_chirho[Lowered module AST]
    lowered_shape_chirho -->|No, but retained AST proves the boundary| ast_boundary_chirho[Defer only the affected check at a documented trust boundary]
    ast_boundary_chirho --> module_ast_chirho
    lowered_shape_chirho -->|No faithful carrier| preserve_sibling_chirho[Consume only the unsupported construct's own layout region]
    preserve_sibling_chirho --> representation_blocker_chirho[Keep the construct gap explicit while preserving following declarations]

    module_ast_chirho --> local_iface_chirho[Collect canonical local type/value exports]
    local_iface_chirho --> associated_iface_chirho[Retain class-to-associated-family relationships]
    associated_iface_chirho --> import_filter_chirho[Apply all/selected/hiding import and export rules]
    import_filter_chirho --> imports_chirho[Resolve imported interfaces]
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

    associated_iface_chirho --> selected_parent_chirho{Class member selection}
    selected_parent_chirho -->|Class(..)| all_associated_chirho[Expose every associated type plus the parent relation]
    selected_parent_chirho -->|Class(F)| selected_associated_chirho[Expose only selected associated types plus the parent relation]
    selected_parent_chirho -->|Class| parent_only_chirho[Expose only the class]
    all_associated_chirho --> import_filter_chirho
    selected_associated_chirho --> import_filter_chirho
    parent_only_chirho --> import_filter_chirho

    walk_uses_chirho --> associated_instance_use_chirho{Bare associated family inside an instance?}
    associated_instance_use_chirho --> parent_instance_chirho[Resolve the qualified or unqualified parent class]
    parent_instance_chirho --> member_visible_chirho{Interface says member belongs to that visible parent?}
    member_visible_chirho -->|Yes| continue_chirho
    member_visible_chirho -->|No| undefined_type_chirho

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

    walk_uses_chirho --> constructor_prefix_chirho{Constructor span proves lowering dropped an existential prefix?}
    constructor_prefix_chirho -->|Yes| defer_existential_chirho[Defer only that constructor's field-variable scope]
    constructor_prefix_chirho -->|No| continue_chirho

    undefined_type_chirho --> driver_gate_chirho[Driver stops before kind inference unless errors are deferred]
    undefined_tyvar_chirho --> driver_gate_chirho
```

The walker performs no filesystem lookup or module scanning. Its successful-resolution path is linear in the lowered AST with hash-backed namespace lookups; the error-only suggestion path compares against the already-populated in-memory environment, and duplicate diagnostics are suppressed by issue/name/span.

Canonical interface maps are the authority for imported names. Type and value operators are normalized once at that boundary, and associated families remain first-class type exports while carrying their parent-class relation through selected imports, hiding, explicit exports, and module re-exports. An associated member imported through a qualified class may be used bare only inside an instance of that same visible class; it is not inserted into the module's general unqualified type namespace. This is the `GHC.Exts.IsList` / `Item` boundary exercised by the real `containers` projects.

The built-in GHC.Types interface includes its exported TYPE classifier. It enters
scope through ordinary explicit/qualified imports; omitting it from a selected
import or hiding it still produces E0101. A downstream runtime-kind seed is not
authority to bypass that visibility check. Compound forall-binder classifiers
delegate to the same type-name walker, including their retained promotion and
literal syntax.

Layout and lowering preserve scope before naming sees the AST. An explicit right brace first closes implicit contexts nested inside its matching explicit context, so the enclosing module regains its virtual declaration separator. Unsupported data/newtype-family instances are still representation blockers, but their scanner stops at their own layout boundary; it must never consume a following signature or declaration. Data and newtype GADT constructor blocks share one parser, while standalone kind signatures retain their complete flat child sequence for the common type reconstruction path.

The kind/type layer uses the same faithful shape downstream: required foralls contribute the kind of their body; symbolic standalone kind operators preserve application order; imported closed Boolean families reduce only when their leading argument selects an equation and otherwise remain stuck.

Two deliberately narrow AST trust boundaries remain. The record-field branch covers `bug-record-field-forall-lowering-chirho.md`; ordinary and required-forall fields remain checked. The constructor-prefix branch covers `bug-existential-constructor-scope-dropped-chirho.md`; it requires a prefix constructor name whose source span begins after its enclosing constructor span, excludes infix constructors, and defers only that constructor's field-variable scope. Both branches should disappear when the AST can carry the source binders and contexts directly.
