// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Name resolution pass
//!
//! Walks the AST and resolves all name references. Reports diagnostics for
//! undefined names, ambiguous names, and shadowing warnings.
//!
//! Supports multi-module compilation: given a set of pre-compiled module
//! interfaces, import declarations are resolved to bring the correct names
//! into scope (both unqualified and qualified).

use haskelujah_ast_chirho::module_chirho::{ImportDeclChirho, ImportItemChirho, ModuleChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

use crate::env_chirho::{NameEnvChirho, NamespaceChirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};
use crate::type_exports_chirho::{canonical_type_name_chirho, canonical_value_name_chirho};
use crate::type_scope_chirho::check_module_type_scope_chirho;
use crate::value_scope_chirho::check_module_value_scope_chirho;

/// Error codes for name resolution diagnostics.
pub(crate) const UNDEFINED_VALUE_CODE_CHIRHO: u16 = 100;
pub(crate) const UNDEFINED_TYPE_CODE_CHIRHO: u16 = 101;
const UNKNOWN_MODULE_CODE_CHIRHO: u16 = 102;

/// Result of name resolution.
pub struct ResolveResultChirho {
    /// The name environment after resolution (contains all definitions).
    pub env_chirho: NameEnvChirho,
    /// Diagnostics (errors and warnings) from resolution.
    pub diagnostics_chirho: DiagnosticBundleChirho,
}

/// Resolve names in a module without any imported module interfaces.
///
/// Equivalent to `resolve_module_with_imports_chirho(module, &[])`.
pub fn resolve_module_chirho(module_chirho: &ModuleChirho) -> ResolveResultChirho {
    resolve_module_with_imports_chirho(module_chirho, &[])
}

/// Resolve names in a module with access to pre-compiled module interfaces.
///
/// Import declarations are resolved against the provided interfaces to bring
/// the correct names into scope. Qualified imports create qualified name
/// bindings that can be looked up via `lookup_qualified_chirho`.
pub fn resolve_module_with_imports_chirho(
    module_chirho: &ModuleChirho,
    available_modules_chirho: &[ModuleIfaceChirho],
) -> ResolveResultChirho {
    let mut env_chirho = NameEnvChirho::new_chirho();
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();

    // Phase 1: Process imports — bring imported names into scope.
    resolve_imports_chirho(
        &module_chirho.imports_chirho,
        available_modules_chirho,
        &mut env_chirho,
        &mut diagnostics_chirho,
    );

    // Phase 2: Collect all top-level type names (data, newtype, type, class).
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            haskelujah_ast_chirho::decl_chirho::DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                ..
            } => {
                bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::TypeChirho);
                for con_chirho in constructors_chirho {
                    if let haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                        signature_chirho,
                        ..
                    } = con_chirho
                    {
                        for field_chirho in
                            signature_chirho.record_fields_chirho().unwrap_or_default()
                        {
                            for field_name_chirho in &field_chirho.names_chirho {
                                bind_name_chirho(
                                    &mut env_chirho,
                                    field_name_chirho,
                                    NamespaceChirho::ValueChirho,
                                );
                            }
                        }
                    }
                    match con_chirho {
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            name_chirho,
                            ..
                        }
                        | haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                            name_chirho,
                            ..
                        } => {
                            bind_name_chirho(
                                &mut env_chirho,
                                name_chirho,
                                NamespaceChirho::ValueChirho,
                            );
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                            name_chirho,
                            fields_chirho,
                            ..
                        } => {
                            bind_name_chirho(
                                &mut env_chirho,
                                name_chirho,
                                NamespaceChirho::ValueChirho,
                            );
                            // Record field accessor functions are value bindings
                            for field_chirho in fields_chirho {
                                for field_name_chirho in &field_chirho.names_chirho {
                                    bind_name_chirho(
                                        &mut env_chirho,
                                        field_name_chirho,
                                        NamespaceChirho::ValueChirho,
                                    );
                                }
                            }
                        }
                    }
                }
            }
            haskelujah_ast_chirho::decl_chirho::DeclChirho::NewtypeDeclChirho {
                name_chirho,
                constructor_chirho,
                ..
            } => {
                bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::TypeChirho);
                if let haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                    signature_chirho,
                    ..
                } = constructor_chirho
                {
                    for field_chirho in signature_chirho.record_fields_chirho().unwrap_or_default()
                    {
                        for field_name_chirho in &field_chirho.names_chirho {
                            bind_name_chirho(
                                &mut env_chirho,
                                field_name_chirho,
                                NamespaceChirho::ValueChirho,
                            );
                        }
                    }
                }
                match constructor_chirho {
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                        name_chirho: con_name_chirho,
                        ..
                    }
                    | haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                        name_chirho: con_name_chirho,
                        ..
                    } => {
                        bind_name_chirho(
                            &mut env_chirho,
                            con_name_chirho,
                            NamespaceChirho::ValueChirho,
                        );
                    }
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                        name_chirho: con_name_chirho,
                        fields_chirho,
                        ..
                    } => {
                        bind_name_chirho(
                            &mut env_chirho,
                            con_name_chirho,
                            NamespaceChirho::ValueChirho,
                        );
                        for field_chirho in fields_chirho {
                            for field_name_chirho in &field_chirho.names_chirho {
                                bind_name_chirho(
                                    &mut env_chirho,
                                    field_name_chirho,
                                    NamespaceChirho::ValueChirho,
                                );
                            }
                        }
                    }
                }
            }
            haskelujah_ast_chirho::decl_chirho::DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                ..
            } => {
                bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::TypeChirho);
            }
            haskelujah_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho {
                name_chirho,
                methods_chirho,
                associated_tfs_chirho,
                ..
            } => {
                bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::TypeChirho);
                for associated_tf_chirho in associated_tfs_chirho {
                    bind_name_chirho(
                        &mut env_chirho,
                        &associated_tf_chirho.name_chirho,
                        NamespaceChirho::TypeChirho,
                    );
                }
                for method_chirho in methods_chirho {
                    bind_name_chirho(
                        &mut env_chirho,
                        &method_chirho.name_chirho,
                        NamespaceChirho::ValueChirho,
                    );
                }
            }
            haskelujah_ast_chirho::decl_chirho::DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                ..
            } => {
                bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::TypeChirho);
            }
            _ => {}
        }
    }

    // Phase 3: Collect all top-level value bindings.
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            haskelujah_ast_chirho::decl_chirho::DeclChirho::FunBindChirho {
                name_chirho, ..
            }
            | haskelujah_ast_chirho::decl_chirho::DeclChirho::TypeSigChirho {
                name_chirho, ..
            } => {
                // Only skip binding if there is already a *local* (non-imported)
                // binding for this name. Imported names should be shadowed by
                // local definitions.
                let already_local_chirho = env_chirho
                    .lookup_value_chirho(name_chirho.text_chirho())
                    .map_or(false, |info_chirho| !info_chirho.imported_chirho);
                if !already_local_chirho {
                    bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::ValueChirho);
                }
            }
            haskelujah_ast_chirho::decl_chirho::DeclChirho::PatBindChirho {
                pat_chirho, ..
            } => {
                // Top-level pattern binding: extract bound variable names
                bind_pat_names_chirho(&mut env_chirho, pat_chirho);
            }
            haskelujah_ast_chirho::decl_chirho::DeclChirho::InstanceDeclChirho { .. } => {
                // Instance methods are not top-level value bindings. Leaving
                // them out of the module env avoids shadowing imported class
                // methods such as `mzipWith` / `contramap` inside instance
                // bodies like `ExceptT`.
            }
            haskelujah_ast_chirho::decl_chirho::DeclChirho::PatSynDeclChirho {
                name_chirho,
                ..
            } => {
                // Pattern synonyms are constructor-like in the value namespace
                bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::ValueChirho);
            }
            _ => {}
        }
    }

    // Workflow: spec-chirho/workflows-chirho/compiler-pipeline-chirho/
    // type-scope-resolution-chirho.md. Type uses are checked only after imports
    // and every top-level type definition have populated the namespace.
    // A missing interface makes every dependent lookup unknowable, so retain
    // the causal import diagnostic instead of cascading guessed scope errors.
    if !diagnostics_chirho.has_errors_chirho() {
        check_module_type_scope_chirho(
            module_chirho,
            &env_chirho,
            available_modules_chirho,
            &mut diagnostics_chirho,
        );
    }

    // Workflow: spec-chirho/workflows-chirho/compiler-pipeline-chirho/
    // value-scope-resolution-chirho.md. Qualified value uses are then checked
    // against the explicit import lists that close their qualifier, so no
    // later phase can stand a bare binding in for one a list leaves out.
    if !diagnostics_chirho.has_errors_chirho() {
        check_module_value_scope_chirho(
            module_chirho,
            &env_chirho,
            available_modules_chirho,
            &mut diagnostics_chirho,
        );
    }

    ResolveResultChirho {
        env_chirho,
        diagnostics_chirho,
    }
}

// ---------------------------------------------------------------------------
// Import resolution
// ---------------------------------------------------------------------------

/// Process import declarations against available module interfaces.
fn resolve_imports_chirho(
    imports_chirho: &[ImportDeclChirho],
    available_chirho: &[ModuleIfaceChirho],
    env_chirho: &mut NameEnvChirho,
    diagnostics_chirho: &mut DiagnosticBundleChirho,
) {
    for import_chirho in imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();

        // Find the module interface.
        let iface_chirho =
            crate::iface_chirho::select_import_iface_chirho(available_chirho, import_chirho);

        let iface_chirho = match iface_chirho {
            Some(i_chirho) => i_chirho,
            None => {
                diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_chirho(
                        format!("could not find module `{module_name_chirho}`"),
                        import_chirho.span_chirho,
                    )
                    .with_code_chirho(ErrorCodeChirho::error_chirho(UNKNOWN_MODULE_CODE_CHIRHO))
                    .with_note_chirho("the module may not have been compiled yet"),
                );
                continue;
            }
        };

        // Determine which names to import.
        let names_to_import_chirho =
            compute_imported_names_chirho(&iface_chirho.exports_chirho, &import_chirho.spec_chirho);

        // Determine the qualifier for qualified lookup.
        let qualifier_chirho = import_chirho
            .alias_chirho
            .as_ref()
            .map(|a_chirho| a_chirho.text_chirho().to_string())
            .unwrap_or_else(|| module_name_chirho.to_string());

        // Bind names.
        for (name_chirho, ns_chirho, span_chirho) in &names_to_import_chirho {
            // Always bind as qualified.
            env_chirho.bind_qualified_chirho(
                format!("{qualifier_chirho}.{name_chirho}"),
                *ns_chirho,
                *span_chirho,
            );

            // If not a qualified-only import, also bind unqualified.
            if !import_chirho.qualified_chirho {
                env_chirho.bind_import_chirho(name_chirho.clone(), *ns_chirho, *span_chirho);
            }
        }
    }
}

/// Compute which (name, namespace, span) triples should be imported from a
/// module's exports, respecting the import specification.
pub fn compute_imported_names_chirho(
    exports_chirho: &IfaceExportsChirho,
    spec_chirho: &Option<haskelujah_ast_chirho::module_chirho::ImportSpecChirho>,
) -> Vec<(String, NamespaceChirho, SpanChirho)> {
    // Collect all available names from the module's exports.
    let all_names_chirho = collect_export_names_chirho(exports_chirho);

    match spec_chirho {
        None => {
            // import Module — import everything.
            all_names_chirho
        }
        Some(spec_chirho) => {
            if spec_chirho.hiding_chirho {
                // import Module hiding (a, b) — import everything EXCEPT listed.
                let hidden_chirho: Vec<String> = spec_chirho
                    .items_chirho
                    .iter()
                    .flat_map(|item_chirho| import_item_names_chirho(item_chirho, exports_chirho))
                    .collect();
                all_names_chirho
                    .into_iter()
                    .filter(|(name_chirho, _, _)| !hidden_chirho.contains(name_chirho))
                    .collect()
            } else {
                // import Module (a, b) — import only listed.
                let wanted_chirho: Vec<String> = spec_chirho
                    .items_chirho
                    .iter()
                    .flat_map(|item_chirho| import_item_names_chirho(item_chirho, exports_chirho))
                    .collect();
                all_names_chirho
                    .into_iter()
                    .filter(|(name_chirho, _, _)| wanted_chirho.contains(name_chirho))
                    .collect()
            }
        }
    }
}

/// Flatten a module's exports into (name, namespace, span) triples.
fn collect_export_names_chirho(
    exports_chirho: &IfaceExportsChirho,
) -> Vec<(String, NamespaceChirho, SpanChirho)> {
    let mut result_chirho = Vec::new();

    for val_chirho in exports_chirho.values_chirho.values() {
        result_chirho.push((
            canonical_value_name_chirho(&val_chirho.name_chirho),
            NamespaceChirho::ValueChirho,
            val_chirho.span_chirho,
        ));
    }

    for ty_chirho in exports_chirho.types_chirho.values() {
        result_chirho.push((
            canonical_type_name_chirho(&ty_chirho.name_chirho),
            NamespaceChirho::TypeChirho,
            ty_chirho.span_chirho,
        ));
        for con_name_chirho in &ty_chirho.constructors_chirho {
            result_chirho.push((
                canonical_value_name_chirho(con_name_chirho),
                NamespaceChirho::ValueChirho,
                ty_chirho.span_chirho,
            ));
        }
        for method_name_chirho in &ty_chirho.methods_chirho {
            result_chirho.push((
                canonical_value_name_chirho(method_name_chirho),
                NamespaceChirho::ValueChirho,
                ty_chirho.span_chirho,
            ));
        }
    }

    result_chirho
}

/// Extract the names from an import item.
fn import_item_names_chirho(
    item_chirho: &ImportItemChirho,
    exports_chirho: &IfaceExportsChirho,
) -> Vec<String> {
    match item_chirho {
        ImportItemChirho::VarChirho(name_chirho) => {
            vec![canonical_value_name_chirho(name_chirho.text_chirho())]
        }
        ImportItemChirho::TyConChirho {
            name_chirho,
            members_chirho,
        } => {
            let type_name_chirho = canonical_type_name_chirho(name_chirho.text_chirho());
            let mut names_chirho = vec![type_name_chirho.clone()];
            match members_chirho {
                haskelujah_ast_chirho::module_chirho::ExportMembersChirho::AllChirho => {
                    if let Some(ty_chirho) = exports_chirho.types_chirho.get(&type_name_chirho) {
                        for con_chirho in &ty_chirho.constructors_chirho {
                            names_chirho.push(con_chirho.clone());
                            // Also add bare operator form: (:|) -> :|
                            if con_chirho.starts_with('(') && con_chirho.ends_with(')') {
                                let bare_chirho = &con_chirho[1..con_chirho.len() - 1];
                                if !bare_chirho.is_empty() {
                                    names_chirho.push(bare_chirho.to_string());
                                }
                            }
                        }
                        for method_chirho in &ty_chirho.methods_chirho {
                            names_chirho.push(method_chirho.clone());
                            if method_chirho.starts_with('(') && method_chirho.ends_with(')') {
                                let bare_chirho = &method_chirho[1..method_chirho.len() - 1];
                                if !bare_chirho.is_empty() {
                                    names_chirho.push(bare_chirho.to_string());
                                }
                            }
                        }
                    }
                    if let Some(associated_types_chirho) = exports_chirho
                        .associated_types_chirho
                        .get(&type_name_chirho)
                    {
                        names_chirho.extend(associated_types_chirho.iter().cloned());
                    }
                }
                haskelujah_ast_chirho::module_chirho::ExportMembersChirho::SomeChirho(
                    member_names_chirho,
                ) => {
                    for mn_chirho in member_names_chirho {
                        let member_name_chirho =
                            canonical_value_name_chirho(mn_chirho.text_chirho());
                        names_chirho.push(member_name_chirho.clone());
                        if exports_chirho
                            .associated_types_chirho
                            .get(&type_name_chirho)
                            .is_some_and(|associated_types_chirho| {
                                associated_types_chirho.contains(&member_name_chirho)
                            })
                        {
                            names_chirho.push(canonical_type_name_chirho(&member_name_chirho));
                        }
                    }
                }
                haskelujah_ast_chirho::module_chirho::ExportMembersChirho::NoneChirho => {}
            }
            names_chirho
        }
        // An item lowering could not represent names nothing, so it imports,
        // and hides, nothing: exactly what it did when lowering dropped it.
        // workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
        ImportItemChirho::UnrepresentedChirho(_) => Vec::new(),
    }
}

/// Report an "undefined name" diagnostic with optional "did you mean?" suggestions.
pub fn report_undefined_chirho(
    diagnostics_chirho: &mut DiagnosticBundleChirho,
    name_chirho: &str,
    namespace_chirho: NamespaceChirho,
    span_chirho: SpanChirho,
) {
    report_undefined_with_suggestions_chirho(
        diagnostics_chirho,
        name_chirho,
        namespace_chirho,
        span_chirho,
        None,
    );
}

/// Report an "undefined name" diagnostic with "did you mean?" suggestions
/// based on names currently in scope.
pub fn report_undefined_with_suggestions_chirho(
    diagnostics_chirho: &mut DiagnosticBundleChirho,
    name_chirho: &str,
    namespace_chirho: NamespaceChirho,
    span_chirho: SpanChirho,
    env_chirho: Option<&NameEnvChirho>,
) {
    use haskelujah_diagnostics_chirho::suggest_chirho::{
        default_max_distance_chirho, format_did_you_mean_chirho, suggest_similar_names_chirho,
    };

    let code_chirho = match namespace_chirho {
        NamespaceChirho::ValueChirho => UNDEFINED_VALUE_CODE_CHIRHO,
        NamespaceChirho::TypeChirho => UNDEFINED_TYPE_CODE_CHIRHO,
    };
    let msg_chirho = match namespace_chirho {
        NamespaceChirho::ValueChirho => format!("variable not in scope: `{name_chirho}`"),
        NamespaceChirho::TypeChirho => format!("type not in scope: `{name_chirho}`"),
    };
    let mut diag_chirho = DiagnosticChirho::error_with_code_chirho(
        ErrorCodeChirho::error_chirho(code_chirho),
        msg_chirho,
        span_chirho,
    );

    // Add "did you mean?" note if we have scope information.
    if let Some(env_chirho) = env_chirho {
        let candidates_chirho = env_chirho.all_names_in_namespace_chirho(namespace_chirho);
        let max_dist_chirho = default_max_distance_chirho(name_chirho.len());
        let suggestions_chirho = suggest_similar_names_chirho(
            name_chirho,
            candidates_chirho.into_iter(),
            max_dist_chirho,
            3,
        );
        if let Some(note_chirho) = format_did_you_mean_chirho(&suggestions_chirho) {
            diag_chirho = diag_chirho.with_note_chirho(note_chirho);
        }
    }

    diagnostics_chirho.push_chirho(diag_chirho);
}

/// Extract variable names from a pattern and bind them in the value namespace.
fn bind_pat_names_chirho(env_chirho: &mut NameEnvChirho, pat_chirho: &PatChirho) {
    match pat_chirho {
        PatChirho::VarChirho(name_chirho) => {
            env_chirho.bind_chirho(
                name_chirho.text_chirho().to_string(),
                NamespaceChirho::ValueChirho,
                name_chirho.span_chirho(),
            );
        }
        PatChirho::ConChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                bind_pat_names_chirho(env_chirho, arg_chirho);
            }
        }
        PatChirho::TupleChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                bind_pat_names_chirho(env_chirho, elem_chirho);
            }
        }
        PatChirho::ListChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                bind_pat_names_chirho(env_chirho, elem_chirho);
            }
        }
        PatChirho::AsChirho {
            name_chirho,
            pattern_chirho,
            ..
        } => {
            env_chirho.bind_chirho(
                name_chirho.text_chirho().to_string(),
                NamespaceChirho::ValueChirho,
                name_chirho.span_chirho(),
            );
            bind_pat_names_chirho(env_chirho, pattern_chirho);
        }
        PatChirho::ParenChirho { inner_chirho, .. }
        | PatChirho::LazyChirho { inner_chirho, .. }
        | PatChirho::BangChirho { inner_chirho, .. } => {
            bind_pat_names_chirho(env_chirho, inner_chirho);
        }
        PatChirho::InfixConChirho {
            left_chirho,
            right_chirho,
            ..
        } => {
            bind_pat_names_chirho(env_chirho, left_chirho);
            bind_pat_names_chirho(env_chirho, right_chirho);
        }
        PatChirho::RecordChirho { fields_chirho, .. } => {
            for field_chirho in fields_chirho {
                bind_pat_names_chirho(env_chirho, &field_chirho.pattern_chirho);
            }
        }
        PatChirho::ViewChirho { pat_chirho, .. } => {
            bind_pat_names_chirho(env_chirho, pat_chirho);
        }
        PatChirho::TypeAnnotChirho { pat_chirho, .. } => {
            bind_pat_names_chirho(env_chirho, pat_chirho);
        }
        // Literals, wildcards, negated literals — no bindings
        PatChirho::LitChirho(_) | PatChirho::WildcardChirho(_) | PatChirho::NegChirho { .. } => {}
    }
}

fn bind_name_chirho(
    env_chirho: &mut NameEnvChirho,
    name_chirho: &NameChirho,
    namespace_chirho: NamespaceChirho,
) {
    let text_chirho = match namespace_chirho {
        NamespaceChirho::ValueChirho => canonical_value_name_chirho(name_chirho.text_chirho()),
        NamespaceChirho::TypeChirho => canonical_type_name_chirho(name_chirho.text_chirho()),
    };
    env_chirho.bind_chirho(text_chirho, namespace_chirho, name_chirho.span_chirho());
}

// ---------------------------------------------------------------------------
// Orphan instance detection
// ---------------------------------------------------------------------------

/// Warning code for orphan instances (W0402).
const ORPHAN_INSTANCE_CODE_CHIRHO: u16 = 402;

/// Collect all type constructor and class names defined locally in this module.
fn local_defined_names_chirho(module_chirho: &ModuleChirho) -> std::collections::HashSet<String> {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    let mut names_chirho = std::collections::HashSet::new();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho { name_chirho, .. }
            | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
            | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
            | DeclChirho::ClassDeclChirho { name_chirho, .. }
            | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. } => {
                names_chirho.insert(name_chirho.text_chirho().to_string());
            }
            _ => {}
        }
    }
    names_chirho
}

/// Extract all type constructor names mentioned in a type (recursively).
fn type_con_names_chirho(ty_chirho: &haskelujah_ast_chirho::ty_chirho::TypeChirho) -> Vec<String> {
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;
    let mut result_chirho = Vec::new();
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => {
            result_chirho.push(name_chirho.text_chirho().to_string());
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        }
        | TypeChirho::KindApplicationChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            result_chirho.extend(type_con_names_chirho(fun_chirho));
            result_chirho.extend(type_con_names_chirho(arg_chirho));
        }
        TypeChirho::KindAnnotationChirho {
            ty_chirho,
            kind_chirho,
            ..
        } => {
            result_chirho.extend(type_con_names_chirho(ty_chirho));
            result_chirho.extend(type_con_names_chirho(kind_chirho));
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho: res_chirho,
            ..
        } => {
            result_chirho.extend(type_con_names_chirho(arg_chirho));
            result_chirho.extend(type_con_names_chirho(res_chirho));
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                result_chirho.extend(type_con_names_chirho(elem_chirho));
            }
        }
        TypeChirho::ListChirho { element_chirho, .. } => {
            result_chirho.extend(type_con_names_chirho(element_chirho));
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            result_chirho.extend(type_con_names_chirho(inner_chirho));
        }
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        }
        | TypeChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            for binder_chirho in vars_chirho {
                if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                    result_chirho.extend(type_con_names_chirho(kind_chirho));
                }
            }
            result_chirho.extend(type_con_names_chirho(body_chirho));
        }
        TypeChirho::QualChirho { body_chirho, .. } => {
            result_chirho.extend(type_con_names_chirho(body_chirho));
        }
        // Type variables — no type constructors
        TypeChirho::VarChirho(_) => {}
        // DataKinds promoted constructor — the constructor name is a type-level entity
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            result_chirho.push(name_chirho.text_chirho().to_string());
        }
        // DataKinds promoted list — recurse into elements
        TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                result_chirho.extend(type_con_names_chirho(elem_chirho));
            }
        }
        // PartialTypeSignatures: `_` wildcard has no type constructor names.
        TypeChirho::WildcardChirho { .. } => {}
        // Type-level literal — no type constructor names to extract.
        TypeChirho::LitChirho { .. } => {}
    }
    result_chirho
}

/// Check for orphan instances in a module.
///
/// An instance `instance C T` is an *orphan* if neither the class `C` nor
/// any type constructor mentioned in the instance head `T` is defined in
/// the current module. Orphan instances can cause incoherence and are
/// flagged with warning W0402.
pub fn check_orphan_instances_chirho(module_chirho: &ModuleChirho) -> DiagnosticBundleChirho {
    let local_names_chirho = local_defined_names_chirho(module_chirho);
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();

    for decl_chirho in &module_chirho.decls_chirho {
        if let haskelujah_ast_chirho::decl_chirho::DeclChirho::InstanceDeclChirho {
            class_chirho,
            kind_args_chirho,
            types_chirho,
            span_chirho,
            ..
        } = decl_chirho
        {
            let class_name_chirho = class_chirho.text_chirho().to_string();

            // Collect all type constructors from the instance head types.
            let mut head_cons_chirho: Vec<String> = Vec::new();
            for ty_chirho in kind_args_chirho.iter().chain(types_chirho) {
                head_cons_chirho.extend(type_con_names_chirho(ty_chirho));
            }

            // The instance is NOT orphan if:
            // 1. The class is defined locally, OR
            // 2. Any type constructor in the head is defined locally
            let is_local_chirho = local_names_chirho.contains(&class_name_chirho)
                || head_cons_chirho
                    .iter()
                    .any(|con_chirho| local_names_chirho.contains(con_chirho));

            if !is_local_chirho {
                let type_strs_chirho: Vec<_> = types_chirho
                    .iter()
                    .map(|t_chirho| format!("{:?}", t_chirho))
                    .collect();
                let head_chirho = if type_strs_chirho.is_empty() {
                    class_name_chirho.clone()
                } else {
                    format!("{} ...", class_name_chirho)
                };

                diagnostics_chirho.push_chirho(
                    DiagnosticChirho::warning_chirho(
                        format!(
                            "orphan instance: `instance {head_chirho}` — \
                             neither the class `{class_name_chirho}` nor any type \
                             in the instance head is defined in this module"
                        ),
                        *span_chirho,
                    )
                    .with_code_chirho(ErrorCodeChirho::warning_chirho(ORPHAN_INSTANCE_CODE_CHIRHO))
                    .with_note_chirho(
                        "move the instance to the module that defines the class \
                         or the type, or use {-# OPTIONS_GHC -fno-warn-orphans #-}",
                    ),
                );
            }
        }
    }

    diagnostics_chirho
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho;
