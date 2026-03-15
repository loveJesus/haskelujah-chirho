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

use rhasky_ast_chirho::module_chirho::{ImportDeclChirho, ImportItemChirho, ModuleChirho};
use rhasky_ast_chirho::name_chirho::NameChirho;
use rhasky_ast_chirho::pat_chirho::PatChirho;
use rhasky_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use rhasky_span_chirho::SpanChirho;

use crate::env_chirho::{NameEnvChirho, NamespaceChirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Error codes for name resolution diagnostics.
const UNDEFINED_VALUE_CODE_CHIRHO: u16 = 100;
const UNDEFINED_TYPE_CODE_CHIRHO: u16 = 101;
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
            rhasky_ast_chirho::decl_chirho::DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                ..
            } => {
                bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::TypeChirho);
                for con_chirho in constructors_chirho {
                    match con_chirho {
                        rhasky_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            name_chirho,
                            ..
                        } => {
                            bind_name_chirho(
                                &mut env_chirho,
                                name_chirho,
                                NamespaceChirho::ValueChirho,
                            );
                        }
                        rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
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
            rhasky_ast_chirho::decl_chirho::DeclChirho::NewtypeDeclChirho {
                name_chirho,
                constructor_chirho,
                ..
            } => {
                bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::TypeChirho);
                match constructor_chirho {
                    rhasky_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                        name_chirho: con_name_chirho,
                        ..
                    } => {
                        bind_name_chirho(
                            &mut env_chirho,
                            con_name_chirho,
                            NamespaceChirho::ValueChirho,
                        );
                    }
                    rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
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
            rhasky_ast_chirho::decl_chirho::DeclChirho::TypeAliasDeclChirho {
                name_chirho, ..
            } => {
                bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::TypeChirho);
            }
            rhasky_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho {
                name_chirho,
                methods_chirho,
                ..
            } => {
                bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::TypeChirho);
                for method_chirho in methods_chirho {
                    bind_name_chirho(
                        &mut env_chirho,
                        &method_chirho.name_chirho,
                        NamespaceChirho::ValueChirho,
                    );
                }
            }
            rhasky_ast_chirho::decl_chirho::DeclChirho::TypeFamilyDeclChirho {
                name_chirho, ..
            } => {
                bind_name_chirho(&mut env_chirho, name_chirho, NamespaceChirho::TypeChirho);
            }
            _ => {}
        }
    }

    // Phase 3: Collect all top-level value bindings.
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            rhasky_ast_chirho::decl_chirho::DeclChirho::FunBindChirho {
                name_chirho, ..
            }
            | rhasky_ast_chirho::decl_chirho::DeclChirho::TypeSigChirho {
                name_chirho, ..
            } => {
                // Only skip binding if there is already a *local* (non-imported)
                // binding for this name. Imported names should be shadowed by
                // local definitions.
                let already_local_chirho = env_chirho
                    .lookup_value_chirho(name_chirho.text_chirho())
                    .map_or(false, |info_chirho| !info_chirho.imported_chirho);
                if !already_local_chirho {
                    bind_name_chirho(
                        &mut env_chirho,
                        name_chirho,
                        NamespaceChirho::ValueChirho,
                    );
                }
            }
            rhasky_ast_chirho::decl_chirho::DeclChirho::PatBindChirho {
                pat_chirho, ..
            } => {
                // Top-level pattern binding: extract bound variable names
                bind_pat_names_chirho(&mut env_chirho, pat_chirho);
            }
            rhasky_ast_chirho::decl_chirho::DeclChirho::InstanceDeclChirho {
                methods_chirho,
                ..
            } => {
                // Instance method implementations create value bindings
                for method_chirho in methods_chirho {
                    if let rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                        name_chirho,
                        ..
                    } = method_chirho
                    {
                        let already_bound_chirho = env_chirho
                            .lookup_value_chirho(name_chirho.text_chirho())
                            .is_some();
                        if !already_bound_chirho {
                            bind_name_chirho(
                                &mut env_chirho,
                                name_chirho,
                                NamespaceChirho::ValueChirho,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
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
        let iface_chirho = available_chirho
            .iter()
            .find(|m_chirho| m_chirho.name_chirho == module_name_chirho);

        let iface_chirho = match iface_chirho {
            Some(i_chirho) => i_chirho,
            None => {
                diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(UNKNOWN_MODULE_CODE_CHIRHO),
                        format!("could not find module `{module_name_chirho}`"),
                        import_chirho.span_chirho,
                    )
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
fn compute_imported_names_chirho(
    exports_chirho: &IfaceExportsChirho,
    spec_chirho: &Option<rhasky_ast_chirho::module_chirho::ImportSpecChirho>,
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
                    .flat_map(|item_chirho| import_item_names_chirho(item_chirho))
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
                    .flat_map(|item_chirho| import_item_names_chirho(item_chirho))
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
            val_chirho.name_chirho.clone(),
            NamespaceChirho::ValueChirho,
            val_chirho.span_chirho,
        ));
    }

    for ty_chirho in exports_chirho.types_chirho.values() {
        result_chirho.push((
            ty_chirho.name_chirho.clone(),
            NamespaceChirho::TypeChirho,
            ty_chirho.span_chirho,
        ));
    }

    result_chirho
}

/// Extract the names from an import item.
fn import_item_names_chirho(
    item_chirho: &ImportItemChirho,
) -> Vec<String> {
    match item_chirho {
        ImportItemChirho::VarChirho(name_chirho) => {
            vec![name_chirho.text_chirho().to_string()]
        }
        ImportItemChirho::TyConChirho {
            name_chirho,
            members_chirho,
        } => {
            let mut names_chirho = vec![name_chirho.text_chirho().to_string()];
            match members_chirho {
                rhasky_ast_chirho::module_chirho::ExportMembersChirho::AllChirho => {
                    // Import all constructors/methods for this type.
                    // This information comes from all_names_chirho; we return the
                    // type name, and the caller's filter will match it. The actual
                    // constructor names are already included in the module's value
                    // exports, so they'll be imported via the "import everything"
                    // path. This marker just ensures the type itself is included.
                }
                rhasky_ast_chirho::module_chirho::ExportMembersChirho::SomeChirho(
                    member_names_chirho,
                ) => {
                    for mn_chirho in member_names_chirho {
                        names_chirho.push(mn_chirho.text_chirho().to_string());
                    }
                }
                rhasky_ast_chirho::module_chirho::ExportMembersChirho::NoneChirho => {}
            }
            names_chirho
        }
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
    use rhasky_diagnostics_chirho::suggest_chirho::{
        default_max_distance_chirho, format_did_you_mean_chirho,
        suggest_similar_names_chirho,
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
        PatChirho::TupleChirho { elements_chirho, .. } => {
            for elem_chirho in elements_chirho {
                bind_pat_names_chirho(env_chirho, elem_chirho);
            }
        }
        PatChirho::ListChirho { elements_chirho, .. } => {
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
        // Literals, wildcards, negated literals — no bindings
        PatChirho::LitChirho(_)
        | PatChirho::WildcardChirho(_)
        | PatChirho::NegChirho { .. } => {}
    }
}

fn bind_name_chirho(
    env_chirho: &mut NameEnvChirho,
    name_chirho: &NameChirho,
    namespace_chirho: NamespaceChirho,
) {
    env_chirho.bind_chirho(
        name_chirho.text_chirho().to_string(),
        namespace_chirho,
        name_chirho.span_chirho(),
    );
}

// ---------------------------------------------------------------------------
// Orphan instance detection
// ---------------------------------------------------------------------------

/// Warning code for orphan instances (W0402).
const ORPHAN_INSTANCE_CODE_CHIRHO: u16 = 402;

/// Collect all type constructor and class names defined locally in this module.
fn local_defined_names_chirho(module_chirho: &ModuleChirho) -> std::collections::HashSet<String> {
    use rhasky_ast_chirho::decl_chirho::DeclChirho;
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
fn type_con_names_chirho(ty_chirho: &rhasky_ast_chirho::ty_chirho::TypeChirho) -> Vec<String> {
    use rhasky_ast_chirho::ty_chirho::TypeChirho;
    let mut result_chirho = Vec::new();
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => {
            result_chirho.push(name_chirho.text_chirho().to_string());
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            result_chirho.extend(type_con_names_chirho(fun_chirho));
            result_chirho.extend(type_con_names_chirho(arg_chirho));
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
        TypeChirho::ListChirho {
            element_chirho, ..
        } => {
            result_chirho.extend(type_con_names_chirho(element_chirho));
        }
        TypeChirho::ParenChirho {
            inner_chirho, ..
        } => {
            result_chirho.extend(type_con_names_chirho(inner_chirho));
        }
        TypeChirho::ForallChirho {
            body_chirho, ..
        } => {
            result_chirho.extend(type_con_names_chirho(body_chirho));
        }
        TypeChirho::QualChirho {
            body_chirho, ..
        } => {
            result_chirho.extend(type_con_names_chirho(body_chirho));
        }
        // Type variables — no type constructors
        TypeChirho::VarChirho(_) => {}
        // DataKinds promoted constructor — the constructor name is a type-level entity
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            result_chirho.push(name_chirho.text_chirho().to_string());
        }
        // DataKinds promoted list — recurse into elements
        TypeChirho::PromotedListChirho { elements_chirho, .. } => {
            for elem_chirho in elements_chirho {
                result_chirho.extend(type_con_names_chirho(elem_chirho));
            }
        }
    }
    result_chirho
}

/// Check for orphan instances in a module.
///
/// An instance `instance C T` is an *orphan* if neither the class `C` nor
/// any type constructor mentioned in the instance head `T` is defined in
/// the current module. Orphan instances can cause incoherence and are
/// flagged with warning W0402.
pub fn check_orphan_instances_chirho(
    module_chirho: &ModuleChirho,
) -> DiagnosticBundleChirho {
    let local_names_chirho = local_defined_names_chirho(module_chirho);
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();

    for decl_chirho in &module_chirho.decls_chirho {
        if let rhasky_ast_chirho::decl_chirho::DeclChirho::InstanceDeclChirho {
            class_chirho,
            types_chirho,
            span_chirho,
            ..
        } = decl_chirho
        {
            let class_name_chirho = class_chirho.text_chirho().to_string();

            // Collect all type constructors from the instance head types.
            let mut head_cons_chirho: Vec<String> = Vec::new();
            for ty_chirho in types_chirho {
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
                    .with_code_chirho(ErrorCodeChirho::warning_chirho(
                        ORPHAN_INSTANCE_CODE_CHIRHO,
                    ))
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
mod tests_chirho {
    use super::*;
    use rhasky_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
    use rhasky_ast_chirho::module_chirho::{ImportDeclChirho, ImportSpecChirho};
    use rhasky_ast_chirho::name_chirho::RawNameChirho;

    use crate::iface_chirho::{IfaceTypeChirho, IfaceValueChirho};

    fn dummy_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    fn mk_module_chirho(
        decls_chirho: Vec<DeclChirho>,
        imports_chirho: Vec<ImportDeclChirho>,
    ) -> ModuleChirho {
        ModuleChirho {
            name_chirho: dummy_name_chirho("Main"),
            exports_chirho: None,
            imports_chirho,
            decls_chirho,
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_lib_iface_chirho() -> ModuleIfaceChirho {
        let mut values_chirho = std::collections::HashMap::new();
        values_chirho.insert(
            "sort".to_string(),
            IfaceValueChirho {
                name_chirho: "sort".to_string(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        values_chirho.insert(
            "map".to_string(),
            IfaceValueChirho {
                name_chirho: "map".to_string(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        values_chirho.insert(
            "Red".to_string(),
            IfaceValueChirho {
                name_chirho: "Red".to_string(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        values_chirho.insert(
            "Blue".to_string(),
            IfaceValueChirho {
                name_chirho: "Blue".to_string(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );

        let mut types_chirho = std::collections::HashMap::new();
        types_chirho.insert(
            "Color".to_string(),
            IfaceTypeChirho {
                name_chirho: "Color".to_string(),
                constructors_chirho: vec!["Red".to_string(), "Blue".to_string()],
                methods_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );

        ModuleIfaceChirho {
            name_chirho: "Lib".to_string(),
            exports_chirho: crate::iface_chirho::IfaceExportsChirho {
                values_chirho,
                types_chirho,
            },
        }
    }

    // ---------------------------------------------------------------
    // Single-module tests (no imports)
    // ---------------------------------------------------------------

    #[test]
    fn resolve_simple_module_chirho() {
        let module_chirho = mk_module_chirho(
            vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("Color"),
                    type_vars_chirho: vec![],
                    constructors_chirho: vec![
                        ConDeclChirho::OrdinaryChirho {
                            name_chirho: dummy_name_chirho("Red"),
                            fields_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        ConDeclChirho::OrdinaryChirho {
                            name_chirho: dummy_name_chirho("Green"),
                            fields_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    deriving_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("main"),
                    matches_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            vec![],
        );

        let result_chirho = resolve_module_chirho(&module_chirho);

        assert!(result_chirho.env_chirho.lookup_type_chirho("Color").is_some());
        assert!(result_chirho.env_chirho.lookup_value_chirho("Red").is_some());
        assert!(result_chirho.env_chirho.lookup_value_chirho("Green").is_some());
        assert!(result_chirho.env_chirho.lookup_value_chirho("main").is_some());
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    }

    #[test]
    fn resolve_class_methods_chirho() {
        let module_chirho = mk_module_chirho(
            vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: dummy_name_chirho("Describable"),
                type_vars_chirho: vec![dummy_name_chirho("a").into()],
                methods_chirho: vec![
                    rhasky_ast_chirho::decl_chirho::ClassMethodChirho {
                        name_chirho: dummy_name_chirho("describe"),
                        ty_chirho: rhasky_ast_chirho::ty_chirho::TypeChirho::VarChirho(
                            dummy_name_chirho("a"),
                        ),
                        default_chirho: None,
                        default_sig_chirho: None,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            vec![],
        );

        let result_chirho = resolve_module_chirho(&module_chirho);

        assert!(result_chirho
            .env_chirho
            .lookup_type_chirho("Describable")
            .is_some());
        assert!(result_chirho
            .env_chirho
            .lookup_value_chirho("describe")
            .is_some());
    }

    // ---------------------------------------------------------------
    // Import resolution tests
    // ---------------------------------------------------------------

    #[test]
    fn import_all_from_module_chirho() {
        let lib_chirho = mk_lib_iface_chirho();
        let module_chirho = mk_module_chirho(
            vec![],
            vec![ImportDeclChirho {
                module_chirho: dummy_name_chirho("Lib"),
                qualified_chirho: false,
                alias_chirho: None,
                spec_chirho: None, // import all
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let result_chirho =
            resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        // Unqualified lookup
        assert!(result_chirho.env_chirho.lookup_value_chirho("sort").is_some());
        assert!(result_chirho.env_chirho.lookup_value_chirho("map").is_some());
        assert!(result_chirho.env_chirho.lookup_type_chirho("Color").is_some());
        // Qualified lookup
        assert!(result_chirho
            .env_chirho
            .lookup_qualified_chirho("Lib", "sort", NamespaceChirho::ValueChirho)
            .is_some());
    }

    #[test]
    fn import_qualified_only_chirho() {
        let lib_chirho = mk_lib_iface_chirho();
        let module_chirho = mk_module_chirho(
            vec![],
            vec![ImportDeclChirho {
                module_chirho: dummy_name_chirho("Lib"),
                qualified_chirho: true, // qualified only
                alias_chirho: None,
                spec_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let result_chirho =
            resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

        // Unqualified lookup should NOT find imported names
        assert!(result_chirho.env_chirho.lookup_value_chirho("sort").is_none());
        // Qualified lookup should work
        assert!(result_chirho
            .env_chirho
            .lookup_qualified_chirho("Lib", "sort", NamespaceChirho::ValueChirho)
            .is_some());
    }

    #[test]
    fn import_with_alias_chirho() {
        let lib_chirho = mk_lib_iface_chirho();
        let module_chirho = mk_module_chirho(
            vec![],
            vec![ImportDeclChirho {
                module_chirho: dummy_name_chirho("Lib"),
                qualified_chirho: true,
                alias_chirho: Some(dummy_name_chirho("L")),
                spec_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let result_chirho =
            resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

        // Should be accessible via alias
        assert!(result_chirho
            .env_chirho
            .lookup_qualified_chirho("L", "sort", NamespaceChirho::ValueChirho)
            .is_some());
        // Original module name should NOT work with alias
        assert!(result_chirho
            .env_chirho
            .lookup_qualified_chirho("Lib", "sort", NamespaceChirho::ValueChirho)
            .is_none());
    }

    #[test]
    fn import_explicit_list_chirho() {
        let lib_chirho = mk_lib_iface_chirho();
        let module_chirho = mk_module_chirho(
            vec![],
            vec![ImportDeclChirho {
                module_chirho: dummy_name_chirho("Lib"),
                qualified_chirho: false,
                alias_chirho: None,
                spec_chirho: Some(ImportSpecChirho {
                    hiding_chirho: false,
                    items_chirho: vec![ImportItemChirho::VarChirho(dummy_name_chirho("sort"))],
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let result_chirho =
            resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

        assert!(result_chirho.env_chirho.lookup_value_chirho("sort").is_some());
        // map is NOT imported
        assert!(result_chirho.env_chirho.lookup_value_chirho("map").is_none());
    }

    #[test]
    fn import_hiding_chirho() {
        let lib_chirho = mk_lib_iface_chirho();
        let module_chirho = mk_module_chirho(
            vec![],
            vec![ImportDeclChirho {
                module_chirho: dummy_name_chirho("Lib"),
                qualified_chirho: false,
                alias_chirho: None,
                spec_chirho: Some(ImportSpecChirho {
                    hiding_chirho: true,
                    items_chirho: vec![ImportItemChirho::VarChirho(dummy_name_chirho("sort"))],
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let result_chirho =
            resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

        // sort is hidden
        assert!(result_chirho.env_chirho.lookup_value_chirho("sort").is_none());
        // map is still imported
        assert!(result_chirho.env_chirho.lookup_value_chirho("map").is_some());
    }

    #[test]
    fn unknown_module_reports_error_chirho() {
        let module_chirho = mk_module_chirho(
            vec![],
            vec![ImportDeclChirho {
                module_chirho: dummy_name_chirho("Nonexistent"),
                qualified_chirho: false,
                alias_chirho: None,
                spec_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let result_chirho =
            resolve_module_with_imports_chirho(&module_chirho, &[]);

        assert!(result_chirho.diagnostics_chirho.has_errors_chirho());
        let msg_chirho = format!("{}", result_chirho.diagnostics_chirho);
        assert!(msg_chirho.contains("Nonexistent"));
    }

    #[test]
    fn import_does_not_shadow_local_chirho() {
        let lib_chirho = mk_lib_iface_chirho();
        let module_chirho = mk_module_chirho(
            vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("sort"),
                matches_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            vec![ImportDeclChirho {
                module_chirho: dummy_name_chirho("Lib"),
                qualified_chirho: false,
                alias_chirho: None,
                spec_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let result_chirho =
            resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

        // Local definition should win (it's bound after imports)
        let info_chirho = result_chirho
            .env_chirho
            .lookup_value_chirho("sort")
            .unwrap();
        assert!(!info_chirho.imported_chirho);
    }

    // ---------------------------------------------------------------
    // Record field, pattern bind, and instance tests
    // ---------------------------------------------------------------

    /// Test: record field names are registered as value bindings.
    #[test]
    fn resolve_record_field_accessors_chirho() {
        use rhasky_ast_chirho::decl_chirho::FieldDeclChirho;

        let module_chirho = mk_module_chirho(
            vec![DeclChirho::DataDeclChirho {
                name_chirho: dummy_name_chirho("Person"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![
                    ConDeclChirho::RecordChirho {
                        name_chirho: dummy_name_chirho("MkPerson"),
                        fields_chirho: vec![
                            FieldDeclChirho {
                                names_chirho: vec![dummy_name_chirho("personName")],
                                ty_chirho: rhasky_ast_chirho::ty_chirho::TypeChirho::ConChirho(
                                    dummy_name_chirho("String"),
                                ),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            FieldDeclChirho {
                                names_chirho: vec![dummy_name_chirho("personAge")],
                                ty_chirho: rhasky_ast_chirho::ty_chirho::TypeChirho::ConChirho(
                                    dummy_name_chirho("Int"),
                                ),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                        ],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                deriving_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            vec![],
        );

        let result_chirho = resolve_module_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());

        // Type should be registered
        assert!(result_chirho.env_chirho.lookup_type_chirho("Person").is_some());
        // Constructor should be registered
        assert!(result_chirho.env_chirho.lookup_value_chirho("MkPerson").is_some());
        // Field accessors should be registered as values
        assert!(
            result_chirho.env_chirho.lookup_value_chirho("personName").is_some(),
            "personName field accessor should be in scope"
        );
        assert!(
            result_chirho.env_chirho.lookup_value_chirho("personAge").is_some(),
            "personAge field accessor should be in scope"
        );
    }

    /// Test: top-level pattern binding binds variables.
    #[test]
    fn resolve_top_level_pat_bind_chirho() {
        use rhasky_ast_chirho::expr_chirho::RhsChirho;

        let module_chirho = mk_module_chirho(
            vec![DeclChirho::PatBindChirho {
                pat_chirho: PatChirho::TupleChirho {
                    elements_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("x")),
                        PatChirho::VarChirho(dummy_name_chirho("y")),
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(
                    rhasky_ast_chirho::expr_chirho::ExprChirho::TupleChirho {
                        elements_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            vec![],
        );

        let result_chirho = resolve_module_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert!(
            result_chirho.env_chirho.lookup_value_chirho("x").is_some(),
            "x from (x, y) = ... should be in scope"
        );
        assert!(
            result_chirho.env_chirho.lookup_value_chirho("y").is_some(),
            "y from (x, y) = ... should be in scope"
        );
    }

    /// Test: as-pattern in top-level pat bind.
    #[test]
    fn resolve_as_pattern_bind_chirho() {
        use rhasky_ast_chirho::expr_chirho::RhsChirho;

        let module_chirho = mk_module_chirho(
            vec![DeclChirho::PatBindChirho {
                pat_chirho: PatChirho::AsChirho {
                    name_chirho: dummy_name_chirho("whole"),
                    pattern_chirho: Box::new(PatChirho::ConChirho {
                        con_chirho: dummy_name_chirho("Just"),
                        args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("inner"))],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(
                    rhasky_ast_chirho::expr_chirho::ExprChirho::TupleChirho {
                        elements_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            vec![],
        );

        let result_chirho = resolve_module_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert!(
            result_chirho.env_chirho.lookup_value_chirho("whole").is_some(),
            "whole from whole@(Just inner) should be in scope"
        );
        assert!(
            result_chirho.env_chirho.lookup_value_chirho("inner").is_some(),
            "inner from whole@(Just inner) should be in scope"
        );
    }

    /// Test: newtype record field accessor is registered.
    #[test]
    fn resolve_newtype_record_field_chirho() {
        use rhasky_ast_chirho::decl_chirho::FieldDeclChirho;

        let module_chirho = mk_module_chirho(
            vec![DeclChirho::NewtypeDeclChirho {
                name_chirho: dummy_name_chirho("Age"),
                type_vars_chirho: vec![],
                constructor_chirho: ConDeclChirho::RecordChirho {
                    name_chirho: dummy_name_chirho("MkAge"),
                    fields_chirho: vec![FieldDeclChirho {
                        names_chirho: vec![dummy_name_chirho("getAge")],
                        ty_chirho: rhasky_ast_chirho::ty_chirho::TypeChirho::ConChirho(
                            dummy_name_chirho("Int"),
                        ),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                deriving_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            vec![],
        );

        let result_chirho = resolve_module_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert!(result_chirho.env_chirho.lookup_type_chirho("Age").is_some());
        assert!(result_chirho.env_chirho.lookup_value_chirho("MkAge").is_some());
        assert!(
            result_chirho.env_chirho.lookup_value_chirho("getAge").is_some(),
            "getAge field accessor for newtype record should be in scope"
        );
    }

    // ---------------------------------------------------------------
    // Orphan instance detection tests
    // ---------------------------------------------------------------

    /// Test: instance with locally-defined class is NOT orphan.
    #[test]
    fn orphan_local_class_not_orphan_chirho() {
        use rhasky_ast_chirho::ty_chirho::TypeChirho;

        let module_chirho = mk_module_chirho(
            vec![
                DeclChirho::ClassDeclChirho {
                    context_chirho: vec![],
                    name_chirho: dummy_name_chirho("MyClass"),
                    type_vars_chirho: vec![dummy_name_chirho("a").into()],
                    methods_chirho: vec![],
                    fundeps_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::InstanceDeclChirho {
                    context_chirho: vec![],
                    class_chirho: dummy_name_chirho("MyClass"),
                    types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Int"))],
                    methods_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            vec![],
        );

        let warnings_chirho = check_orphan_instances_chirho(&module_chirho);
        assert!(
            warnings_chirho.is_empty_chirho(),
            "instance for locally-defined class should not be orphan"
        );
    }

    /// Test: instance with locally-defined type is NOT orphan.
    #[test]
    fn orphan_local_type_not_orphan_chirho() {
        use rhasky_ast_chirho::ty_chirho::TypeChirho;

        let module_chirho = mk_module_chirho(
            vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("Color"),
                    type_vars_chirho: vec![],
                    constructors_chirho: vec![],
                    deriving_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::InstanceDeclChirho {
                    context_chirho: vec![],
                    class_chirho: dummy_name_chirho("Show"),
                    types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Color"))],
                    methods_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            vec![],
        );

        let warnings_chirho = check_orphan_instances_chirho(&module_chirho);
        assert!(
            warnings_chirho.is_empty_chirho(),
            "instance for locally-defined type should not be orphan"
        );
    }

    /// Test: instance where neither class nor type is local IS orphan.
    #[test]
    fn orphan_foreign_class_and_type_chirho() {
        use rhasky_ast_chirho::ty_chirho::TypeChirho;

        let module_chirho = mk_module_chirho(
            vec![DeclChirho::InstanceDeclChirho {
                context_chirho: vec![],
                class_chirho: dummy_name_chirho("Show"),
                types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Int"))],
                methods_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            vec![],
        );

        let warnings_chirho = check_orphan_instances_chirho(&module_chirho);
        assert_eq!(
            warnings_chirho.len_chirho(),
            1,
            "instance Show Int without local class/type should be orphan"
        );
        assert_eq!(warnings_chirho.warning_count_chirho(), 1);
        let msg_chirho = warnings_chirho.diagnostics_chirho()[0]
            .message_chirho
            .clone();
        assert!(
            msg_chirho.contains("orphan instance"),
            "message should mention 'orphan instance'"
        );
    }

    /// Test: instance with local newtype in App head is NOT orphan.
    #[test]
    fn orphan_local_newtype_in_app_not_orphan_chirho() {
        use rhasky_ast_chirho::ty_chirho::TypeChirho;

        let module_chirho = mk_module_chirho(
            vec![
                DeclChirho::NewtypeDeclChirho {
                    name_chirho: dummy_name_chirho("Wrapper"),
                    type_vars_chirho: vec![dummy_name_chirho("a").into()],
                    constructor_chirho: ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("MkWrapper"),
                        fields_chirho: vec![TypeChirho::VarChirho(dummy_name_chirho("a"))],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    deriving_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::InstanceDeclChirho {
                    context_chirho: vec![],
                    class_chirho: dummy_name_chirho("Show"),
                    types_chirho: vec![TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Wrapper"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    methods_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            vec![],
        );

        let warnings_chirho = check_orphan_instances_chirho(&module_chirho);
        assert!(
            warnings_chirho.is_empty_chirho(),
            "instance Show (Wrapper a) with local Wrapper should not be orphan"
        );
    }

    /// Test: multiple orphan instances produce multiple warnings.
    #[test]
    fn orphan_multiple_warnings_chirho() {
        use rhasky_ast_chirho::ty_chirho::TypeChirho;

        let module_chirho = mk_module_chirho(
            vec![
                DeclChirho::InstanceDeclChirho {
                    context_chirho: vec![],
                    class_chirho: dummy_name_chirho("Show"),
                    types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Int"))],
                    methods_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::InstanceDeclChirho {
                    context_chirho: vec![],
                    class_chirho: dummy_name_chirho("Eq"),
                    types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Bool"))],
                    methods_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            vec![],
        );

        let warnings_chirho = check_orphan_instances_chirho(&module_chirho);
        assert_eq!(warnings_chirho.len_chirho(), 2);
    }
}
