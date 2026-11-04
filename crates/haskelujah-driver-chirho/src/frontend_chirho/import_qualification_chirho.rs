// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Import qualification for the frontend, moved from the driver root (row 691, L4): qualifying
//! imported types and schemes by module alias, the safe unqualified and preferred qualified type
//! names and synonym keys an import brings, and when an imported scheme overrides a placeholder.

use super::*;

fn is_placeholder_import_scheme_chirho(
    scheme_chirho: &haskelujah_typing_chirho::SchemeChirho,
) -> bool {
    if !scheme_chirho.preds_chirho.is_empty() || scheme_chirho.vars_chirho.len() != 1 {
        return false;
    }
    match &scheme_chirho.ty_chirho {
        haskelujah_typing_chirho::TyChirho::VarChirho(var_chirho) => {
            *var_chirho == scheme_chirho.vars_chirho[0] && var_chirho.0 >= 9000
        }
        _ => false,
    }
}

/// The scheme a qualified import gives `name_chirho` when the interface it selected has no
/// `module.name` key (row 608). Only an interface proven builtin reads the generated GHC agreement
/// table. A source module's interface, whatever its name, and any interface of unknown origin
/// keep today's placeholder.
/// workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
pub fn qualified_import_alias_scheme_chirho(
    iface_chirho: &ModuleIfaceChirho,
    name_chirho: &str,
    builtins_chirho: &std::collections::HashMap<String, haskelujah_typing_chirho::SchemeChirho>,
) -> Option<haskelujah_typing_chirho::SchemeChirho> {
    if iface_chirho.exports_chirho.origin_chirho
        != haskelujah_naming_chirho::IfaceOriginChirho::BuiltinChirho
    {
        return None;
    }
    haskelujah_typing_chirho::infer_chirho::qualified_builtin_alias_scheme_chirho(
        &iface_chirho.name_chirho,
        name_chirho,
        builtins_chirho,
    )
}

pub(crate) fn should_override_imported_scheme_chirho(
    existing_scheme_chirho: Option<&haskelujah_typing_chirho::SchemeChirho>,
    new_scheme_chirho: &haskelujah_typing_chirho::SchemeChirho,
) -> bool {
    match existing_scheme_chirho {
        None => true,
        Some(existing_scheme_chirho) => {
            is_placeholder_import_scheme_chirho(existing_scheme_chirho)
                && !is_placeholder_import_scheme_chirho(new_scheme_chirho)
        }
    }
}

fn collect_local_type_names_chirho(
    module_chirho: &ModuleChirho,
) -> std::collections::HashSet<String> {
    module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho { name_chirho, .. }
            | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
            | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
            | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. }
            | DeclChirho::ClassDeclChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect()
}

fn default_safe_unqualified_imported_type_names_chirho() -> std::collections::HashSet<String> {
    // These names are already modeled as shared runtime-facing types across
    // multiple re-export modules, so keeping them bare avoids spurious
    // mismatches like B.ByteString vs ByteString, I.IORef vs IORef, and
    // E.SomeException vs SomeException.
    ["ByteString", "Ordering", "IORef", "SomeException"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

pub(crate) fn collect_safe_unqualified_imported_type_names_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> std::collections::HashSet<String> {
    let local_type_names_chirho = collect_local_type_names_chirho(module_chirho);
    let mut safe_type_names_chirho = default_safe_unqualified_imported_type_names_chirho();
    safe_type_names_chirho.retain(|name_chirho| !local_type_names_chirho.contains(name_chirho));
    safe_type_names_chirho.extend(
        module_chirho
            .imports_chirho
            .iter()
            .filter(|import_chirho| !import_chirho.qualified_chirho)
            .flat_map(|import_chirho| {
                let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
                ifaces_chirho
                    .iter()
                    .rev()
                    .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
                    .into_iter()
                    .flat_map(|iface_chirho| {
                        haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho(
                            &iface_chirho.exports_chirho,
                            &import_chirho.spec_chirho,
                        )
                    })
            })
            .filter_map(|(name_chirho, namespace_chirho, _span_chirho)| {
                (namespace_chirho
                    == haskelujah_naming_chirho::env_chirho::NamespaceChirho::TypeChirho
                    && !local_type_names_chirho.contains(&name_chirho))
                .then_some(name_chirho)
            }),
    );
    safe_type_names_chirho
}

/// Give each type name a module may spell bare the bare synonym key that spelling needs (row 656).
///
/// A name with exactly one exporter among the module's imports is canonicalized to its bare
/// spelling, by the import rewrite and by inference alike, whether or not an import lists it. A
/// synonym so spelled therefore needs a bare key, or a re-exported alias stops expanding at a
/// parent the module never imported by name. The key is cloned from that exporter's qualified
/// key, only when no bare key exists and the qualified one is already in the merged input. It
/// serves expansion only: naming still decides what the source may write.
/// workflow: compiler-pipeline-chirho/type-scope-resolution-chirho
pub(crate) fn align_preferred_synonym_keys_chirho(
    merged_imported_type_synonyms_chirho: &mut ImportedTypeSynonymsChirho,
    preferred_qualified_type_names_chirho: &std::collections::HashMap<String, String>,
) {
    for (type_name_chirho, exporter_chirho) in preferred_qualified_type_names_chirho {
        if merged_imported_type_synonyms_chirho.contains_key(type_name_chirho) {
            continue;
        }
        if let Some(synonym_chirho) = merged_imported_type_synonyms_chirho
            .get(&format!("{exporter_chirho}.{type_name_chirho}"))
            .cloned()
        {
            merged_imported_type_synonyms_chirho.insert(type_name_chirho.clone(), synonym_chirho);
        }
    }
}

pub(crate) fn collect_preferred_qualified_type_names_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> std::collections::HashMap<String, String> {
    let local_type_names_chirho = collect_local_type_names_chirho(module_chirho);
    let mut exporters_by_type_name_chirho: std::collections::HashMap<
        String,
        std::collections::HashSet<String>,
    > = std::collections::HashMap::new();

    for import_chirho in &module_chirho.imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        let Some(iface_chirho) = ifaces_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
        else {
            continue;
        };
        for exported_type_name_chirho in iface_chirho.exports_chirho.types_chirho.keys() {
            if local_type_names_chirho.contains(exported_type_name_chirho) {
                continue;
            }
            exporters_by_type_name_chirho
                .entry(exported_type_name_chirho.clone())
                .or_default()
                .insert(module_name_chirho.clone());
        }
    }

    exporters_by_type_name_chirho
        .into_iter()
        .filter_map(|(type_name_chirho, exporters_chirho)| {
            (exporters_chirho.len() == 1).then(|| {
                (
                    type_name_chirho,
                    exporters_chirho
                        .into_iter()
                        .next()
                        .expect("single exporter should exist"),
                )
            })
        })
        .collect()
}

pub(crate) fn qualify_imported_ast_type_chirho(
    ty_chirho: &TypeChirho,
    qualifiable_type_names_chirho: &std::collections::HashSet<String>,
    qualifier_chirho: &str,
    unqualified_type_names_chirho: &std::collections::HashSet<String>,
    preferred_qualified_type_names_chirho: &std::collections::HashMap<String, String>,
) -> TypeChirho {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => {
            if name_chirho.family_key_chirho().is_some() || name_chirho.promotion_chirho().is_some()
            {
                return ty_chirho.clone();
            }
            let bare_name_chirho = name_chirho.text_chirho();
            let should_canonicalize_unqualified_chirho = unqualified_type_names_chirho
                .contains(bare_name_chirho)
                || preferred_qualified_type_names_chirho.contains_key(bare_name_chirho);
            if should_canonicalize_unqualified_chirho {
                TypeChirho::ConChirho(haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(
                    haskelujah_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
                        bare_name_chirho.to_string(),
                        name_chirho.span_chirho(),
                    ),
                ))
            } else if qualifiable_type_names_chirho.contains(bare_name_chirho) {
                let span_chirho = name_chirho.span_chirho();
                TypeChirho::ConChirho(haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(
                    haskelujah_ast_chirho::name_chirho::RawNameChirho::qualified_chirho(
                        qualifier_chirho.to_string(),
                        bare_name_chirho.to_string(),
                        span_chirho,
                    ),
                ))
            } else {
                ty_chirho.clone()
            }
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            span_chirho,
        } => TypeChirho::AppChirho {
            fun_chirho: Box::new(qualify_imported_ast_type_chirho(
                fun_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            arg_chirho: Box::new(qualify_imported_ast_type_chirho(
                arg_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        TypeChirho::FunChirho {
            arg_chirho,
            mult_chirho,
            result_chirho,
            span_chirho,
        } => TypeChirho::FunChirho {
            arg_chirho: Box::new(qualify_imported_ast_type_chirho(
                arg_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            mult_chirho: mult_chirho.clone(),
            result_chirho: Box::new(qualify_imported_ast_type_chirho(
                result_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        TypeChirho::TupleChirho {
            elements_chirho,
            span_chirho,
        } => TypeChirho::TupleChirho {
            elements_chirho: elements_chirho
                .iter()
                .map(|element_chirho| {
                    qualify_imported_ast_type_chirho(
                        element_chirho,
                        qualifiable_type_names_chirho,
                        qualifier_chirho,
                        unqualified_type_names_chirho,
                        preferred_qualified_type_names_chirho,
                    )
                })
                .collect(),
            span_chirho: *span_chirho,
        },
        TypeChirho::ListChirho {
            element_chirho,
            span_chirho,
        } => TypeChirho::ListChirho {
            element_chirho: Box::new(qualify_imported_ast_type_chirho(
                element_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        TypeChirho::ParenChirho {
            inner_chirho,
            span_chirho,
        } => TypeChirho::ParenChirho {
            inner_chirho: Box::new(qualify_imported_ast_type_chirho(
                inner_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            span_chirho,
        } => TypeChirho::QualChirho {
            context_chirho: context_chirho.clone(),
            body_chirho: Box::new(qualify_imported_ast_type_chirho(
                body_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            span_chirho,
        } => TypeChirho::ForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(qualify_imported_ast_type_chirho(
                body_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        _ => ty_chirho.clone(),
    }
}

fn qualify_imported_ty_chirho(
    ty_chirho: &haskelujah_typing_chirho::TyChirho,
    qualifiable_type_names_chirho: &std::collections::HashSet<String>,
    qualifier_chirho: &str,
    unqualified_type_names_chirho: &std::collections::HashSet<String>,
    preferred_qualified_type_names_chirho: &std::collections::HashMap<String, String>,
) -> haskelujah_typing_chirho::TyChirho {
    match ty_chirho {
        haskelujah_typing_chirho::TyChirho::ConChirho(name_chirho) => {
            if family_exports_chirho::is_opaque_family_or_promotion_key_chirho(name_chirho) {
                return ty_chirho.clone();
            }
            let bare_name_chirho = name_chirho
                .rsplit('.')
                .next()
                .unwrap_or(name_chirho.as_str());
            if unqualified_type_names_chirho.contains(bare_name_chirho)
                || preferred_qualified_type_names_chirho.contains_key(bare_name_chirho)
            {
                haskelujah_typing_chirho::TyChirho::ConChirho(bare_name_chirho.to_string())
            } else if qualifiable_type_names_chirho.contains(bare_name_chirho) {
                haskelujah_typing_chirho::TyChirho::ConChirho(format!(
                    "{qualifier_chirho}.{bare_name_chirho}"
                ))
            } else {
                haskelujah_typing_chirho::TyChirho::ConChirho(name_chirho.clone())
            }
        }
        haskelujah_typing_chirho::TyChirho::AppChirho(fun_chirho, arg_chirho) => {
            haskelujah_typing_chirho::TyChirho::AppChirho(
                Box::new(qualify_imported_ty_chirho(
                    fun_chirho,
                    qualifiable_type_names_chirho,
                    qualifier_chirho,
                    unqualified_type_names_chirho,
                    preferred_qualified_type_names_chirho,
                )),
                Box::new(qualify_imported_ty_chirho(
                    arg_chirho,
                    qualifiable_type_names_chirho,
                    qualifier_chirho,
                    unqualified_type_names_chirho,
                    preferred_qualified_type_names_chirho,
                )),
            )
        }
        haskelujah_typing_chirho::TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => {
            haskelujah_typing_chirho::TyChirho::FunChirho(
                Box::new(qualify_imported_ty_chirho(
                    arg_chirho,
                    qualifiable_type_names_chirho,
                    qualifier_chirho,
                    unqualified_type_names_chirho,
                    preferred_qualified_type_names_chirho,
                )),
                Box::new(qualify_imported_ty_chirho(
                    result_chirho,
                    qualifiable_type_names_chirho,
                    qualifier_chirho,
                    unqualified_type_names_chirho,
                    preferred_qualified_type_names_chirho,
                )),
                *mult_chirho,
            )
        }
        haskelujah_typing_chirho::TyChirho::TupleChirho(elements_chirho) => {
            haskelujah_typing_chirho::TyChirho::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|element_chirho| {
                        qualify_imported_ty_chirho(
                            element_chirho,
                            qualifiable_type_names_chirho,
                            qualifier_chirho,
                            unqualified_type_names_chirho,
                            preferred_qualified_type_names_chirho,
                        )
                    })
                    .collect(),
            )
        }
        haskelujah_typing_chirho::TyChirho::ListChirho(element_chirho) => {
            haskelujah_typing_chirho::TyChirho::ListChirho(Box::new(qualify_imported_ty_chirho(
                element_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )))
        }
        haskelujah_typing_chirho::TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } => haskelujah_typing_chirho::TyChirho::ForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(qualify_imported_ty_chirho(
                body_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
        },
        other_chirho => other_chirho.clone(),
    }
}

pub(crate) fn qualify_imported_scheme_for_iface_chirho(
    scheme_chirho: &haskelujah_typing_chirho::SchemeChirho,
    qualifiable_type_names_chirho: &std::collections::HashSet<String>,
    qualifier_chirho: &str,
    unqualified_type_names_chirho: &std::collections::HashSet<String>,
    preferred_qualified_type_names_chirho: &std::collections::HashMap<String, String>,
) -> haskelujah_typing_chirho::SchemeChirho {
    if qualifiable_type_names_chirho.is_empty() {
        return scheme_chirho.clone();
    }
    haskelujah_typing_chirho::SchemeChirho {
        vars_chirho: scheme_chirho.vars_chirho.clone(),
        preds_chirho: scheme_chirho
            .preds_chirho
            .iter()
            .map(|pred_chirho| {
                haskelujah_typing_chirho::ty_chirho::SchemePredChirho::from_proof_chirho(
                    pred_chirho.class_identity_chirho().clone(),
                    haskelujah_typing_chirho::ty_chirho::SchemePredPartsChirho {
                        kind_tys_chirho: pred_chirho
                            .kind_tys_chirho
                            .iter()
                            .map(|ty_chirho| {
                                qualify_imported_ty_chirho(
                                    ty_chirho,
                                    qualifiable_type_names_chirho,
                                    qualifier_chirho,
                                    unqualified_type_names_chirho,
                                    preferred_qualified_type_names_chirho,
                                )
                            })
                            .collect(),
                        class_name_chirho: pred_chirho.class_name_chirho.clone(),
                        ty_chirho: qualify_imported_ty_chirho(
                            &pred_chirho.ty_chirho,
                            qualifiable_type_names_chirho,
                            qualifier_chirho,
                            unqualified_type_names_chirho,
                            preferred_qualified_type_names_chirho,
                        ),
                        extra_tys_chirho: pred_chirho
                            .extra_tys_chirho
                            .iter()
                            .map(|ty_chirho| {
                                qualify_imported_ty_chirho(
                                    ty_chirho,
                                    qualifiable_type_names_chirho,
                                    qualifier_chirho,
                                    unqualified_type_names_chirho,
                                    preferred_qualified_type_names_chirho,
                                )
                            })
                            .collect(),
                    },
                )
            })
            .collect(),
        ty_chirho: qualify_imported_ty_chirho(
            &scheme_chirho.ty_chirho,
            qualifiable_type_names_chirho,
            qualifier_chirho,
            unqualified_type_names_chirho,
            preferred_qualified_type_names_chirho,
        ),
    }
}
