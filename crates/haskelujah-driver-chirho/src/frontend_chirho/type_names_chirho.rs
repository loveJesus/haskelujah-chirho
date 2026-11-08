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
    exported_chirho: &mut ImportedTypeSynonymsChirho,
    module_chirho: &str,
    alias_chirho: &str,
    parameters_chirho: &[String],
    rhs_chirho: &TypeChirho,
    transport_chirho: &mut ImportQualificationChirho<'_>,
) -> Result<(), DiagnosticBundleChirho> {
    let module_chirho = transport_chirho.key_chirho(module_chirho)?;
    let alias_chirho = transport_chirho.key_chirho(alias_chirho)?;
    let qualified_chirho =
        transport_chirho.key_chirho(&format!("{module_chirho}.{alias_chirho}"))?;
    // Both map entries are real copies and share one latched transport budget.
    for key_chirho in [alias_chirho, qualified_chirho] {
        let parameters_chirho = transport_chirho.parameters_chirho(parameters_chirho)?;
        let rhs_chirho = transport_chirho.ast_chirho(rhs_chirho)?;
        exported_chirho.insert(key_chirho, (parameters_chirho, rhs_chirho));
    }
    Ok(())
}

fn seed_transitive_type_synonyms_chirho(
    exported_chirho: &mut ImportedTypeSynonymsChirho,
    module_chirho: &str,
    rhs_chirho: &TypeChirho,
    imported_chirho: &ImportedTypeSynonymsChirho,
    seen_chirho: &mut std::collections::HashSet<String>,
    transport_chirho: &mut ImportQualificationChirho<'_>,
) -> Result<(), DiagnosticBundleChirho> {
    // This compatibility path is only for unknown metadata. Checked aliases
    // retain their complete private dependency closure in the common catalogue.
    let mut pending_chirho = vec![rhs_chirho];
    while let Some(rhs_chirho) = pending_chirho.pop() {
        haskelujah_ast_chirho::decl_chirho::check_type_projection_chirho(
            rhs_chirho,
            transport_chirho.budget_chirho,
        )
        .map_err(|_| {
            synonym_export_failure_chirho(
                "type synonym dependency transport resource bound exhausted",
                rhs_chirho.span_chirho(),
            )
        })?;
        let mut names_chirho = std::collections::HashSet::new();
        collect_type_constructor_names_chirho(rhs_chirho, &mut names_chirho);
        let mut names_chirho: Vec<_> = names_chirho.into_iter().collect();
        names_chirho.sort();
        for name_chirho in names_chirho {
            let bare_chirho = name_chirho
                .rsplit_once('.')
                .map_or(name_chirho.as_str(), |(_, bare_chirho)| bare_chirho);
            if !seen_chirho.insert(transport_chirho.key_chirho(bare_chirho)?) {
                continue;
            }
            let qualified_chirho =
                transport_chirho.key_chirho(&format!("{module_chirho}.{bare_chirho}"))?;
            let Some((parameters_chirho, body_chirho)) = imported_chirho
                .get(&name_chirho)
                .or_else(|| imported_chirho.get(&qualified_chirho))
                .or_else(|| imported_chirho.get(bare_chirho))
            else {
                continue;
            };
            insert_type_synonym_with_module_alias_chirho(
                exported_chirho,
                module_chirho,
                bare_chirho,
                parameters_chirho,
                body_chirho,
                transport_chirho,
            )?;
            pending_chirho.push(body_chirho);
        }
    }
    Ok(())
}

fn synonym_export_failure_chirho(
    message_chirho: &str,
    span_chirho: haskelujah_span_chirho::SpanChirho,
) -> DiagnosticBundleChirho {
    let mut diagnostics_chirho = DiagnosticBundleChirho::default();
    diagnostics_chirho.push_chirho(
        haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
            haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
            message_chirho,
            span_chirho,
        ),
    );
    diagnostics_chirho
}

pub(crate) fn exported_type_synonyms_from_module_chirho(
    module_chirho: &ModuleChirho,
    iface_chirho: &ModuleIfaceChirho,
    imported_chirho: &ImportedTypeSynonymsChirho,
    infer_chirho: &InferResultChirho,
) -> Result<ImportedTypeSynonymsChirho, DiagnosticBundleChirho> {
    let mut budget_chirho =
        haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho();
    exported_type_synonyms_with_budget_chirho(
        module_chirho,
        iface_chirho,
        imported_chirho,
        infer_chirho,
        &mut budget_chirho,
    )
}

pub(super) fn exported_type_synonyms_with_budget_chirho(
    module_chirho: &ModuleChirho,
    iface_chirho: &ModuleIfaceChirho,
    imported_chirho: &ImportedTypeSynonymsChirho,
    infer_chirho: &InferResultChirho,
    budget_chirho: &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho,
) -> Result<ImportedTypeSynonymsChirho, DiagnosticBundleChirho> {
    let names_chirho = std::collections::HashSet::new();
    let preferred_chirho = std::collections::HashMap::new();
    let mut transport_chirho = ImportQualificationChirho {
        iface_chirho: None,
        type_names_chirho: &names_chirho,
        qualifier_chirho: "",
        safe_chirho: &names_chirho,
        preferred_chirho: &preferred_chirho,
        budget_chirho,
        span_chirho: module_chirho.span_chirho,
    };
    let mut exported_chirho = ImportedTypeSynonymsChirho::new();
    let mut seen_chirho = std::collections::HashSet::new();
    let mut exports_chirho: Vec<_> = iface_chirho.exports_chirho.types_chirho.iter().collect();
    exports_chirho.sort_by_key(|(name_chirho, _)| *name_chirho);
    for (name_chirho, entry_chirho) in exports_chirho {
        transport_chirho.key_chirho(name_chirho)?;
        if let Some(declaration_chirho) = &entry_chirho.declaration_chirho {
            if declaration_chirho.role_chirho
                != haskelujah_ast_chirho::name_chirho::TypeConstructorRoleChirho::TypeSynonymChirho
            {
                continue;
            }
            let key_chirho = declaration_chirho.type_key_chirho();
            let schema_chirho = infer_chirho
                .type_families_chirho
                .declaration_schemes_chirho()
                .get_chirho(&key_chirho)
                .ok_or_else(|| {
                    synonym_export_failure_chirho(
                        "checked exported synonym has no declaration telescope",
                        entry_chirho.span_chirho,
                    )
                })?;
            let syntax_chirho = schema_chirho
                .synonym_syntax_chirho
                .as_ref()
                .filter(|_| schema_chirho.owner_chirho.is_none() && !schema_chirho.held_chirho)
                .ok_or_else(|| {
                    synonym_export_failure_chirho(
                        "checked exported synonym has no canonical producer syntax",
                        entry_chirho.span_chirho,
                    )
                })?;
            let parameters_chirho = syntax_chirho
                .parameters_chirho()
                .iter()
                .map(|parameter_chirho| transport_chirho.key_chirho(parameter_chirho.text_chirho()))
                .collect::<Result<Vec<_>, _>>()?;
            insert_type_synonym_with_module_alias_chirho(
                &mut exported_chirho,
                &iface_chirho.name_chirho,
                name_chirho,
                &parameters_chirho,
                syntax_chirho.rhs_chirho(),
                &mut transport_chirho,
            )?;
            continue;
        }
        let local_chirho = module_chirho
            .decls_chirho
            .iter()
            .find_map(|declaration_chirho| {
                if let DeclChirho::TypeAliasDeclChirho {
                    name_chirho: local_chirho,
                    type_vars_chirho,
                    rhs_chirho,
                    ..
                } = declaration_chirho
                {
                    (local_chirho.text_chirho() == name_chirho)
                        .then_some((type_vars_chirho, rhs_chirho))
                } else {
                    None
                }
            });
        if let Some((parameters_chirho, rhs_chirho)) = local_chirho {
            let parameters_chirho = parameters_chirho
                .iter()
                .map(|parameter_chirho| transport_chirho.key_chirho(parameter_chirho.text_chirho()))
                .collect::<Result<Vec<_>, _>>()?;
            let mut rhs_chirho = transport_chirho.ast_chirho(rhs_chirho)?;
            if let Some(scope_chirho) = infer_chirho.promotion_scope_chirho.as_deref() {
                stamp_promotions_chirho(&mut rhs_chirho, scope_chirho);
            }
            stamp_exported_family_references_chirho(
                &mut rhs_chirho,
                infer_chirho,
                &iface_chirho.name_chirho,
            );
            insert_type_synonym_with_module_alias_chirho(
                &mut exported_chirho,
                &iface_chirho.name_chirho,
                name_chirho,
                &parameters_chirho,
                &rhs_chirho,
                &mut transport_chirho,
            )?;
            seed_transitive_type_synonyms_chirho(
                &mut exported_chirho,
                &iface_chirho.name_chirho,
                &rhs_chirho,
                imported_chirho,
                &mut seen_chirho,
                &mut transport_chirho,
            )?;
        } else if let Some((parameters_chirho, rhs_chirho)) = imported_chirho.get(name_chirho) {
            insert_type_synonym_with_module_alias_chirho(
                &mut exported_chirho,
                &iface_chirho.name_chirho,
                name_chirho,
                parameters_chirho,
                rhs_chirho,
                &mut transport_chirho,
            )?;
            seed_transitive_type_synonyms_chirho(
                &mut exported_chirho,
                &iface_chirho.name_chirho,
                rhs_chirho,
                imported_chirho,
                &mut seen_chirho,
                &mut transport_chirho,
            )?;
        }
    }
    Ok(exported_chirho)
}
