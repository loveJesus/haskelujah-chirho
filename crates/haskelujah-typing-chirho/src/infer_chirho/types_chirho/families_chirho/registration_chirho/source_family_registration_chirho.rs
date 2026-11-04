// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declaration-only family registration for authoritative discovered source modules.
//! No expression, signature body or instance method body is inferred here.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.md.

use super::{
    InferCtxChirho, TypeFamilyRegistryChirho, TypeFamilyScopeChirho, ast_type_to_syn_rhs_chirho,
};
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::promotion_chirho::PromotionScopeChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_diagnostics_chirho::DiagnosticBundleChirho;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[path = "source_declarations_chirho.rs"]
mod source_declarations_chirho;

/// Every scope is built by the same naming admission as the checked frontend.
pub struct SourceFamilyRegistrationContextChirho<'source_chirho> {
    pub registry_chirho: &'source_chirho TypeFamilyRegistryChirho,
    pub family_scope_chirho: Arc<TypeFamilyScopeChirho>,
    pub promotion_scope_chirho: Arc<PromotionScopeChirho>,
    pub type_scope_chirho: Arc<haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho>,
    pub builtin_classes_chirho: haskelujah_ast_chirho::name_chirho::BuiltinClassScopeChirho,
    pub imported_synonyms_chirho: &'source_chirho HashMap<String, (Vec<String>, TypeChirho)>,
    pub safe_unqualified_type_names_chirho: &'source_chirho HashSet<String>,
    pub preferred_qualified_type_names_chirho: &'source_chirho HashMap<String, String>,
}

/// A cold registration snapshot, never a claim that the source's values type-check.
pub struct SourceFamilyRegistrationResultChirho {
    pub registry_chirho: TypeFamilyRegistryChirho,
    pub family_scope_chirho: Arc<TypeFamilyScopeChirho>,
}

fn context_chirho(
    module_chirho: &ModuleChirho,
    imports_chirho: SourceFamilyRegistrationContextChirho<'_>,
) -> Result<(ModuleChirho, InferCtxChirho), DiagnosticBundleChirho> {
    let mut owned_chirho = source_declarations_chirho::declaration_module_chirho(module_chirho)?;
    let elaboration_chirho =
        crate::kind_chirho::elaboration_chirho::elaborate_declaration_module_chirho(
            &mut owned_chirho,
            crate::kind_chirho::elaboration_chirho::ElaborationImportsChirho {
                registry_chirho: imports_chirho.registry_chirho,
                type_scope_chirho: &imports_chirho.type_scope_chirho,
                family_scope_chirho: &imports_chirho.family_scope_chirho,
                promotion_scope_chirho: &imports_chirho.promotion_scope_chirho,
                builtin_classes_chirho: &imports_chirho.builtin_classes_chirho,
            },
        )
        .map_err(|error_chirho| {
            DiagnosticBundleChirho::new_chirho(vec![error_chirho.into_diagnostic_chirho()])
        })?;
    let module_chirho = &owned_chirho;
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.adopt_elaboration_state_chirho(elaboration_chirho.state_chirho);
    context_chirho.type_constructor_scope_chirho = Some(imports_chirho.type_scope_chirho);
    context_chirho.set_imported_type_name_preferences_chirho(
        imports_chirho.safe_unqualified_type_names_chirho,
        imports_chirho.preferred_qualified_type_names_chirho,
    );
    if let Err(error_chirho) = context_chirho
        .type_families_chirho
        .merge_chirho(&elaboration_chirho.registry_chirho)
    {
        context_chirho
            .family_registration_error_chirho(error_chirho.to_string(), module_chirho.span_chirho);
        return Err(context_chirho.finish_chirho().diagnostics_chirho);
    }
    context_chirho.promotion_scope_chirho = Some(imports_chirho.promotion_scope_chirho);
    context_chirho
        .prepare_family_scope_chirho(module_chirho, Some(imports_chirho.family_scope_chirho));
    // Exported synonym RHSs already carry their exporter's promotion/family stamps.
    let imported_promotions_chirho = PromotionScopeChirho::default();
    let mut synonyms_chirho = imports_chirho
        .imported_synonyms_chirho
        .iter()
        .collect::<Vec<_>>();
    synonyms_chirho.sort_by(|left_chirho, right_chirho| left_chirho.0.cmp(right_chirho.0));
    for (name_chirho, (parameters_chirho, rhs_chirho)) in synonyms_chirho {
        if context_chirho.uses_checked_declaration_chirho(name_chirho) {
            continue;
        }
        let rhs_chirho = ast_type_to_syn_rhs_chirho(
            rhs_chirho,
            parameters_chirho,
            Some(&imported_promotions_chirho),
        );
        context_chirho.register_type_synonym_chirho(
            name_chirho.clone(),
            parameters_chirho.clone(),
            rhs_chirho,
        );
    }
    for declaration_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::TypeAliasDeclChirho {
            name_chirho,
            type_vars_chirho,
            rhs_chirho,
            ..
        } = declaration_chirho
        {
            if context_chirho.uses_checked_declaration_chirho(&name_chirho.full_name_chirho()) {
                continue;
            }
            let parameters_chirho = type_vars_chirho
                .iter()
                .map(|variable_chirho| variable_chirho.text_chirho().to_owned())
                .collect::<Vec<_>>();
            let mut rhs_chirho = rhs_chirho.clone();
            if let Some(scope_chirho) = context_chirho.family_scope_chirho.as_deref() {
                super::stamp_family_references_chirho(
                    &mut rhs_chirho,
                    scope_chirho,
                    &context_chirho.type_families_chirho,
                    &context_chirho.family_module_name_chirho,
                );
            }
            let rhs_chirho = ast_type_to_syn_rhs_chirho(
                &rhs_chirho,
                &parameters_chirho,
                context_chirho.promotion_scope_chirho.as_deref(),
            );
            context_chirho.register_type_synonym_chirho(
                name_chirho.text_chirho().to_owned(),
                parameters_chirho,
                rhs_chirho,
            );
        }
    }
    Ok((owned_chirho, context_chirho))
}

fn finish_chirho(
    context_chirho: InferCtxChirho,
) -> Result<SourceFamilyRegistrationResultChirho, DiagnosticBundleChirho> {
    let result_chirho = context_chirho.finish_chirho();
    if result_chirho
        .non_deferrable_diagnostics_chirho
        .has_errors_chirho()
    {
        return Err(result_chirho.non_deferrable_diagnostics_chirho);
    }
    if result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(result_chirho.diagnostics_chirho);
    }
    Ok(SourceFamilyRegistrationResultChirho {
        registry_chirho: result_chirho.type_families_chirho,
        family_scope_chirho: result_chirho
            .family_scope_chirho
            .expect("source registration prepared its family scope"),
    })
}

/// Only real OPEN headers may precede branch registration. In particular an empty
/// closed definition would be an immutable, incorrect replacement for its body.
pub fn predeclare_source_open_families_chirho(
    module_chirho: &ModuleChirho,
    imports_chirho: SourceFamilyRegistrationContextChirho<'_>,
) -> Result<SourceFamilyRegistrationResultChirho, DiagnosticBundleChirho> {
    let (owned_chirho, mut context_chirho) = context_chirho(module_chirho, imports_chirho)?;
    let module_chirho = &owned_chirho;
    for declaration_chirho in &module_chirho.decls_chirho {
        match declaration_chirho {
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                closed_chirho: false,
                data_family_chirho: false,
                span_chirho,
                ..
            } => {
                context_chirho.register_open_family_chirho(
                    name_chirho.text_chirho().to_owned(),
                    type_vars_chirho
                        .iter()
                        .filter(|variable_chirho| variable_chirho.is_visible_chirho())
                        .count(),
                    *span_chirho,
                );
            }
            DeclChirho::ClassDeclChirho {
                associated_tfs_chirho,
                ..
            } => {
                // A written associated default can accompany the same header.
                let mut names_chirho = std::collections::HashSet::new();
                for family_chirho in associated_tfs_chirho {
                    if !family_chirho.data_family_chirho
                        && names_chirho.insert(family_chirho.name_chirho.text_chirho())
                    {
                        context_chirho.register_open_family_chirho(
                            family_chirho.name_chirho.text_chirho().to_owned(),
                            family_chirho
                                .type_vars_chirho
                                .iter()
                                .filter(|binder_chirho| binder_chirho.is_visible_chirho())
                                .count(),
                            family_chirho.span_chirho,
                        );
                    }
                }
            }
            _ => {}
        }
    }
    finish_chirho(context_chirho)
}

/// Reuse the inference engine's declaration converters, without checking terms.
/// The caller owns the snapshot and publishes it only after the whole source
/// closure and strict export publication succeed.
pub fn register_source_module_families_chirho(
    module_chirho: &ModuleChirho,
    imports_chirho: SourceFamilyRegistrationContextChirho<'_>,
) -> Result<SourceFamilyRegistrationResultChirho, DiagnosticBundleChirho> {
    let (owned_chirho, mut context_chirho) = context_chirho(module_chirho, imports_chirho)?;
    let module_chirho = &owned_chirho;
    context_chirho.register_module_families_chirho(module_chirho);
    // Associated defaults and explicit instance-body family equations are
    // registered by these declaration-only methods in ordinary inference too.
    for declaration_chirho in &module_chirho.decls_chirho {
        context_chirho.process_class_decl_chirho(declaration_chirho);
    }
    for declaration_chirho in &module_chirho.decls_chirho {
        context_chirho.process_instance_decl_chirho(declaration_chirho);
    }
    context_chirho.verify_imported_open_families_chirho();
    finish_chirho(context_chirho)
}
