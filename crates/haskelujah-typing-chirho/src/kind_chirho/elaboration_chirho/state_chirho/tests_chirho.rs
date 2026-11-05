// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The real module handoff retains both its allowance and terminal failure.

use super::super::*;
use super::*;
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::name_chirho::{
    CompilerTypeConstructorChirho, NameChirho, RawNameChirho, TypeConstructorDeclarationChirho,
};

fn module_chirho() -> ModuleChirho {
    ModuleChirho {
        name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "ContinuationBudgetChirho",
            SpanChirho::DUMMY_CHIRHO,
        )),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![],
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

fn scopes_chirho() -> TypeConstructorScopeChirho {
    let mut scope_chirho = TypeConstructorScopeChirho::default();
    for constructor_chirho in [
        CompilerTypeConstructorChirho::TypeUniverseChirho,
        CompilerTypeConstructorChirho::MaybeChirho,
        CompilerTypeConstructorChirho::IntChirho,
    ] {
        scope_chirho.bind_chirho(
            constructor_chirho.name_chirho().to_owned(),
            Some(TypeConstructorDeclarationChirho::compiler_data_chirho(
                constructor_chirho,
            )),
        );
    }
    scope_chirho
}

fn checked_chirho(
    module_chirho: &mut ModuleChirho,
    scope_chirho: &TypeConstructorScopeChirho,
) -> ElaborationResultChirho {
    elaborate_module_chirho(
        module_chirho,
        ElaborationImportsChirho {
            registry_chirho: &TypeFamilyEnvChirho::default(),
            type_scope_chirho: scope_chirho,
            family_scope_chirho: &TypeFamilyScopeChirho::default(),
            promotion_scope_chirho: &PromotionScopeChirho::default(),
            builtin_classes_chirho: &BuiltinClassScopeChirho::default(),
        },
    )
    .unwrap()
}

fn argument_chirho(
    checked_chirho: &mut ElaborationResultChirho,
    scope_chirho: &TypeConstructorScopeChirho,
) -> Result<(TyChirho, SubstChirho), ElaborationErrorChirho> {
    elaborate_required_argument_chirho(
        &ExprChirho::ConChirho(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "Maybe",
            SpanChirho::DUMMY_CHIRHO,
        ))),
        &TyChirho::fun_chirho(
            ElaborationStateChirho::type_universe_chirho(),
            ElaborationStateChirho::type_universe_chirho(),
        ),
        ArgumentElaborationContextChirho {
            equality_chirho: crate::infer_chirho::EqualityNormalizationViewChirho {
                substitutions_chirho: &SubstChirho::empty_chirho(),
                legacy_synonyms_chirho: &HashMap::new(),
                type_scope_chirho: Some(scope_chirho),
                family_scope_chirho: Some(&TypeFamilyScopeChirho::default()),
                family_module_chirho: "MainChirho",
                promotion_present_chirho: false,
                evidence_chirho: Default::default(),
            },
            state_chirho: &mut checked_chirho.state_chirho,
            registry_chirho: &mut checked_chirho.registry_chirho,
            type_scope_chirho: scope_chirho,
            family_scope_chirho: &TypeFamilyScopeChirho::default(),
            promotion_scope_chirho: &PromotionScopeChirho::default(),
            scoped_variables_chirho: &HashMap::new(),
            images_chirho: &SubstChirho::empty_chirho(),
            refinements_chirho: &crate::env_chirho::TyEnvChirho::new_chirho(),
        },
    )
}

#[test]
fn late_argument_keeps_the_original_remaining_allowance_chirho() {
    let scope_chirho = scopes_chirho();
    let mut fresh_chirho = checked_chirho(&mut module_chirho(), &scope_chirho);
    assert_eq!(
        argument_chirho(&mut fresh_chirho, &scope_chirho).unwrap().0,
        TyChirho::ConChirho("Maybe".into())
    );
    let mut exhausted_chirho = checked_chirho(&mut module_chirho(), &scope_chirho);
    let owner_chirho = exhausted_chirho.state_chirho.owner_chirho;
    let remaining_chirho = exhausted_chirho.state_chirho.work_left_chirho;
    exhausted_chirho
        .state_chirho
        .charge_chirho(remaining_chirho, 0, SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    assert!(exhausted_chirho.state_chirho.failure_chirho().is_none());
    let failure_chirho = argument_chirho(&mut exhausted_chirho, &scope_chirho).unwrap_err();
    assert!(failure_chirho.message_chirho.contains("bound exhausted"));
    assert_eq!(exhausted_chirho.state_chirho.owner_chirho, owner_chirho);
    assert_eq!(
        exhausted_chirho.state_chirho.failure_chirho(),
        Some(&failure_chirho)
    );
    assert_eq!(
        argument_chirho(&mut exhausted_chirho, &scope_chirho).unwrap_err(),
        failure_chirho
    );
}

#[test]
fn adopted_latched_failure_is_fatal_without_another_argument_chirho() {
    let scope_chirho = scopes_chirho();
    let mut module_chirho = module_chirho();
    let mut checked_chirho = checked_chirho(&mut module_chirho, &scope_chirho);
    let failure_chirho = checked_chirho
        .state_chirho
        .charge_chirho(usize::MAX, 0, SpanChirho::DUMMY_CHIRHO)
        .unwrap_err();
    let result_chirho = crate::infer_chirho::infer_elaborated_module_chirho(
        &module_chirho,
        crate::infer_chirho::InferenceImportContextChirho {
            unary_variable_instance_heads_covered_chirho: false,
            imported_types_chirho: &HashMap::new(),
            imported_type_synonyms_chirho: &HashMap::new(),
            imported_type_families_chirho: &checked_chirho.registry_chirho,
            imported_class_env_chirho: &crate::class_chirho::ClassEnvChirho::default(),
            unavailable_selectors_chirho: &HashSet::new(),
            imported_record_field_names_chirho: &HashMap::new(),
            safe_unqualified_imported_type_names_chirho: &HashSet::new(),
            preferred_qualified_type_names_chirho: &HashMap::new(),
            promotion_scope_chirho: None,
            family_scope_chirho: None,
            type_constructor_scope_chirho: Some(Arc::new(scope_chirho)),
        },
        checked_chirho.state_chirho,
    );
    assert_eq!(
        result_chirho
            .non_deferrable_diagnostics_chirho
            .diagnostics_chirho(),
        &[failure_chirho.into_diagnostic_chirho()]
    );
}

#[test]
fn rejected_nested_argument_keeps_the_original_solver_trail_chirho() {
    let scope_chirho = scopes_chirho();
    let mut checked_chirho = checked_chirho(&mut module_chirho(), &scope_chirho);
    checked_chirho
        .state_chirho
        .begin_inference_links_chirho(SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    checked_chirho
        .state_chirho
        .record_link_chirho(
            TyVarChirho(17),
            TyChirho::int_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
    let failure_chirho = argument_chirho(&mut checked_chirho, &scope_chirho).unwrap_err();
    assert!(failure_chirho.message_chirho.contains("nested argument"));
    assert_eq!(
        checked_chirho.state_chirho.inference_links_chirho,
        Some(vec![TyVarChirho(17)])
    );
    assert_eq!(
        checked_chirho
            .state_chirho
            .finish_inference_links_chirho(SpanChirho::DUMMY_CHIRHO)
            .unwrap()
            .lookup_chirho(&TyVarChirho(17)),
        Some(&TyChirho::int_chirho())
    );
}

#[test]
fn structural_resource_refusal_remains_terminal_chirho() {
    let scope_chirho = scopes_chirho();
    let mut checked_chirho = checked_chirho(&mut module_chirho(), &scope_chirho);
    let mut deep_chirho = TyChirho::int_chirho();
    for _ in 0..256 {
        deep_chirho = TyChirho::ListChirho(Box::new(deep_chirho));
    }
    let failure_chirho = checked_chirho
        .state_chirho
        .resolve_chirho(&deep_chirho, SpanChirho::DUMMY_CHIRHO)
        .unwrap_err();
    assert_eq!(
        checked_chirho.state_chirho.failure_chirho(),
        Some(&failure_chirho)
    );
    assert_eq!(
        argument_chirho(&mut checked_chirho, &scope_chirho).unwrap_err(),
        failure_chirho
    );
}

#[test]
fn qualified_term_homonym_cannot_acquire_a_lexical_type_proof_chirho() {
    let scope_chirho = scopes_chirho();
    let mut checked_chirho = checked_chirho(&mut module_chirho(), &scope_chirho);
    let variable_chirho = checked_chirho
        .state_chirho
        .fresh_chirho(
            Some(ElaborationStateChirho::type_universe_chirho()),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
    let variables_chirho = HashMap::from([("aChirho".to_owned(), variable_chirho)]);
    for (name_chirho, accepts_chirho) in [
        (
            RawNameChirho::unqualified_chirho("aChirho", SpanChirho::DUMMY_CHIRHO),
            true,
        ),
        (
            RawNameChirho::qualified_chirho("QChirho", "aChirho", SpanChirho::DUMMY_CHIRHO),
            false,
        ),
    ] {
        let result_chirho = elaborate_required_argument_chirho(
            &ExprChirho::VarChirho(NameChirho::RawChirho(name_chirho)),
            &ElaborationStateChirho::type_universe_chirho(),
            ArgumentElaborationContextChirho {
                equality_chirho: crate::infer_chirho::EqualityNormalizationViewChirho {
                    substitutions_chirho: &SubstChirho::empty_chirho(),
                    legacy_synonyms_chirho: &HashMap::new(),
                    type_scope_chirho: Some(&scope_chirho),
                    family_scope_chirho: Some(&TypeFamilyScopeChirho::default()),
                    family_module_chirho: "MainChirho",
                    promotion_present_chirho: false,
                    evidence_chirho: Default::default(),
                },
                state_chirho: &mut checked_chirho.state_chirho,
                registry_chirho: &mut checked_chirho.registry_chirho,
                type_scope_chirho: &scope_chirho,
                family_scope_chirho: &TypeFamilyScopeChirho::default(),
                promotion_scope_chirho: &PromotionScopeChirho::default(),
                scoped_variables_chirho: &variables_chirho,
                images_chirho: &SubstChirho::empty_chirho(),
                refinements_chirho: &crate::env_chirho::TyEnvChirho::new_chirho(),
            },
        );
        if accepts_chirho {
            assert_eq!(
                result_chirho.unwrap().0,
                TyChirho::VarChirho(variable_chirho)
            );
        } else {
            assert!(
                result_chirho
                    .unwrap_err()
                    .message_chirho
                    .contains("qualified term variable")
            );
        }
    }
}
