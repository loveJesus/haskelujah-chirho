// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type names for the frontend seed, moved from the driver root (row 691, L3): the built-in
//! type-family seed and its extension, a module's type constructor names, and the type
//! synonyms a module exports, with their alias-qualified and transitive entries.

use super::*;

/// Seed builtin type family equations that packages commonly depend on.
/// PrimState is defined in the `primitive` package but many packages
/// use it transitively (aeson, resourcet, vector-builder, etc.).
pub(crate) fn seed_builtin_type_families_chirho() -> ImportedTypeFamiliesChirho {
    haskelujah_typing_chirho::infer_chirho::seed_type_family_registry_chirho()
}

pub(crate) fn extend_imported_type_families_chirho(
    target_chirho: &mut ImportedTypeFamiliesChirho,
    source_chirho: ImportedTypeFamiliesChirho,
) -> Result<(), String> {
    target_chirho
        .merge_chirho(&source_chirho)
        .map_err(|error_chirho| {
            format!("conflicting imported type-family snapshots: {error_chirho}")
        })
}

fn collect_type_constructor_names_chirho(
    ty_chirho: &TypeChirho,
    names_chirho: &mut std::collections::HashSet<String>,
) {
    fn constraint_names_chirho(
        constraint_chirho: &haskelujah_ast_chirho::ty_chirho::ConstraintChirho,
        names_chirho: &mut std::collections::HashSet<String>,
    ) {
        use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;
        match constraint_chirho {
            ConstraintChirho::ClassChirho {
                args_chirho,
                kind_args_chirho,
                ..
            } => {
                for arg_chirho in kind_args_chirho.iter().chain(args_chirho) {
                    collect_type_constructor_names_chirho(arg_chirho, names_chirho);
                }
            }
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho,
                kind_chirho,
                ..
            } => {
                constraint_names_chirho(constraint_chirho, names_chirho);
                collect_type_constructor_names_chirho(kind_chirho, names_chirho);
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => {
                for binder_chirho in vars_chirho {
                    if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                        collect_type_constructor_names_chirho(kind_chirho, names_chirho);
                    }
                }
                for constraint_chirho in context_chirho {
                    constraint_names_chirho(constraint_chirho, names_chirho);
                }
                constraint_names_chirho(body_chirho, names_chirho);
            }
        }
    }
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => {
            if name_chirho.family_key_chirho().is_none() && name_chirho.promotion_chirho().is_none()
            {
                names_chirho.insert(name_chirho.full_name_chirho());
            }
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
        }
        | TypeChirho::KindAnnotationChirho {
            ty_chirho: fun_chirho,
            kind_chirho: arg_chirho,
            ..
        } => {
            collect_type_constructor_names_chirho(fun_chirho, names_chirho);
            collect_type_constructor_names_chirho(arg_chirho, names_chirho);
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            collect_type_constructor_names_chirho(arg_chirho, names_chirho);
            collect_type_constructor_names_chirho(result_chirho, names_chirho);
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                collect_type_constructor_names_chirho(elem_chirho, names_chirho);
            }
        }
        TypeChirho::ListChirho { element_chirho, .. } => {
            collect_type_constructor_names_chirho(element_chirho, names_chirho);
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            collect_type_constructor_names_chirho(inner_chirho, names_chirho);
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            for constraint_chirho in context_chirho {
                constraint_names_chirho(constraint_chirho, names_chirho);
            }
            collect_type_constructor_names_chirho(body_chirho, names_chirho);
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
                    collect_type_constructor_names_chirho(kind_chirho, names_chirho);
                }
            }
            collect_type_constructor_names_chirho(body_chirho, names_chirho);
        }
        TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                collect_type_constructor_names_chirho(elem_chirho, names_chirho);
            }
        }
        TypeChirho::PromotedConChirho { .. } => {}
        TypeChirho::VarChirho(_)
        | TypeChirho::WildcardChirho { .. }
        | TypeChirho::LitChirho { .. } => {}
    }
}

fn insert_type_synonym_with_module_alias_chirho(
    exported_type_synonyms_chirho: &mut ImportedTypeSynonymsChirho,
    module_name_chirho: &str,
    alias_name_chirho: &str,
    params_chirho: Vec<String>,
    rhs_chirho: TypeChirho,
) {
    exported_type_synonyms_chirho.insert(
        alias_name_chirho.to_string(),
        (params_chirho.clone(), rhs_chirho.clone()),
    );
    exported_type_synonyms_chirho.insert(
        format!("{module_name_chirho}.{alias_name_chirho}"),
        (params_chirho, rhs_chirho),
    );
}

fn seed_transitive_type_synonyms_chirho(
    exported_type_synonyms_chirho: &mut ImportedTypeSynonymsChirho,
    module_name_chirho: &str,
    rhs_chirho: &TypeChirho,
    resolved_imported_type_synonyms_chirho: &ImportedTypeSynonymsChirho,
    seen_chirho: &mut std::collections::HashSet<String>,
) {
    let mut referenced_names_chirho = std::collections::HashSet::new();
    collect_type_constructor_names_chirho(rhs_chirho, &mut referenced_names_chirho);
    for referenced_name_chirho in referenced_names_chirho {
        let bare_name_chirho = referenced_name_chirho
            .rsplit_once('.')
            .map(|(_prefix_chirho, bare_name_chirho)| bare_name_chirho)
            .unwrap_or(referenced_name_chirho.as_str())
            .to_string();
        if !seen_chirho.insert(bare_name_chirho.clone()) {
            continue;
        }
        let candidate_keys_chirho = [
            referenced_name_chirho.clone(),
            format!("{module_name_chirho}.{bare_name_chirho}"),
            bare_name_chirho.clone(),
        ];
        let Some((params_chirho, transitive_rhs_chirho)) = candidate_keys_chirho
            .iter()
            .find_map(|key_chirho| resolved_imported_type_synonyms_chirho.get(key_chirho))
            .cloned()
        else {
            continue;
        };
        insert_type_synonym_with_module_alias_chirho(
            exported_type_synonyms_chirho,
            module_name_chirho,
            &bare_name_chirho,
            params_chirho,
            transitive_rhs_chirho.clone(),
        );
        seed_transitive_type_synonyms_chirho(
            exported_type_synonyms_chirho,
            module_name_chirho,
            &transitive_rhs_chirho,
            resolved_imported_type_synonyms_chirho,
            seen_chirho,
        );
    }
}

pub(crate) fn exported_type_synonyms_from_module_chirho(
    module_chirho: &ModuleChirho,
    iface_chirho: &ModuleIfaceChirho,
    resolved_imported_type_synonyms_chirho: &ImportedTypeSynonymsChirho,
    infer_result_chirho: &InferResultChirho,
) -> ImportedTypeSynonymsChirho {
    let mut exported_type_synonyms_chirho = ImportedTypeSynonymsChirho::new();
    let mut local_type_alias_names_chirho = std::collections::HashSet::new();
    let mut transitive_seen_chirho = std::collections::HashSet::new();
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::TypeAliasDeclChirho {
            name_chirho,
            type_vars_chirho,
            rhs_chirho,
            ..
        } = decl_chirho
        {
            let alias_name_chirho = name_chirho.text_chirho().to_string();
            local_type_alias_names_chirho.insert(alias_name_chirho.clone());
            if iface_chirho
                .exports_chirho
                .types_chirho
                .contains_key(&alias_name_chirho)
            {
                let exported_rhs_chirho = stamped_synonym_rhs_chirho(
                    rhs_chirho,
                    infer_result_chirho,
                    &iface_chirho.name_chirho,
                );
                insert_type_synonym_with_module_alias_chirho(
                    &mut exported_type_synonyms_chirho,
                    &iface_chirho.name_chirho,
                    &alias_name_chirho,
                    type_vars_chirho
                        .iter()
                        .map(|ty_var_chirho| ty_var_chirho.text_chirho().to_string())
                        .collect(),
                    exported_rhs_chirho.clone(),
                );
                seed_transitive_type_synonyms_chirho(
                    &mut exported_type_synonyms_chirho,
                    &iface_chirho.name_chirho,
                    &exported_rhs_chirho,
                    resolved_imported_type_synonyms_chirho,
                    &mut transitive_seen_chirho,
                );
            }
        }
    }
    for exported_type_name_chirho in iface_chirho.exports_chirho.types_chirho.keys() {
        if local_type_alias_names_chirho.contains(exported_type_name_chirho)
            || exported_type_synonyms_chirho.contains_key(exported_type_name_chirho)
        {
            continue;
        }
        if let Some((params_chirho, rhs_chirho)) =
            resolved_imported_type_synonyms_chirho.get(exported_type_name_chirho)
        {
            insert_type_synonym_with_module_alias_chirho(
                &mut exported_type_synonyms_chirho,
                &iface_chirho.name_chirho,
                exported_type_name_chirho,
                params_chirho.clone(),
                rhs_chirho.clone(),
            );
            seed_transitive_type_synonyms_chirho(
                &mut exported_type_synonyms_chirho,
                &iface_chirho.name_chirho,
                rhs_chirho,
                resolved_imported_type_synonyms_chirho,
                &mut transitive_seen_chirho,
            );
        }
    }
    exported_type_synonyms_chirho
}

fn stamped_synonym_rhs_chirho(
    rhs_chirho: &TypeChirho,
    infer_result_chirho: &InferResultChirho,
    module_name_chirho: &str,
) -> TypeChirho {
    let mut exported_rhs_chirho = rhs_chirho.clone();
    if let Some(scope_chirho) = infer_result_chirho.promotion_scope_chirho.as_deref() {
        stamp_promotions_chirho(&mut exported_rhs_chirho, scope_chirho);
    }
    stamp_exported_family_references_chirho(
        &mut exported_rhs_chirho,
        infer_result_chirho,
        module_name_chirho,
    );
    exported_rhs_chirho
}
