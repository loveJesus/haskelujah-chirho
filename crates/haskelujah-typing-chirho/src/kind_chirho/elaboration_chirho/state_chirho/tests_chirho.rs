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
            refinement_mark_chirho: None,
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
    for pending_field_chirho in [false, true] {
        let scope_chirho = scopes_chirho();
        let mut module_chirho = module_chirho();
        let mut checked_chirho = checked_chirho(&mut module_chirho, &scope_chirho);
        if pending_field_chirho {
            checked_chirho
                .state_chirho
                .queue_constructor_field_kinds_chirho(
                    vec![(
                        ElaborationStateChirho::type_universe_chirho(),
                        SpanChirho::DUMMY_CHIRHO,
                    )],
                    SpanChirho::DUMMY_CHIRHO,
                )
                .unwrap();
        }
        let failure_chirho = checked_chirho
            .state_chirho
            .charge_chirho(usize::MAX, 0, SpanChirho::DUMMY_CHIRHO)
            .unwrap_err();
        let result_chirho = crate::infer_chirho::infer_elaborated_module_chirho(
            &module_chirho,
            crate::infer_chirho::InferenceImportContextChirho {
                unary_variable_instance_heads_covered_chirho: false,
                local_class_prefix_instances_complete_chirho: false,
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
                refinement_mark_chirho: None,
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

fn runtime_type_chirho(representation_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho(
            haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho::RuntimeTypeChirho
                .type_key_chirho()
                .to_owned(),
        )),
        Box::new(representation_chirho),
    )
}

#[test]
fn lifted_equation_uses_the_existing_state_link_and_normalizer_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let variable_chirho = state_chirho
        .fresh_chirho(None, SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    state_chirho
        .begin_inference_links_chirho(SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    let application_chirho = runtime_type_chirho(TyChirho::VarChirho(variable_chirho));
    state_chirho
        .unify_chirho(
            &application_chirho,
            &ElaborationStateChirho::type_universe_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
    let links_chirho = state_chirho
        .finish_inference_links_chirho(SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    assert_eq!(
        links_chirho.lookup_chirho(&variable_chirho),
        Some(crate::ty_chirho::representation_chirho::boxed_representation_chirho(true))
    );
    assert_eq!(
        state_chirho
            .resolve_chirho(&application_chirho, SpanChirho::DUMMY_CHIRHO)
            .unwrap(),
        ElaborationStateChirho::type_universe_chirho()
    );
}

#[test]
fn representation_equation_preserves_rigidity_and_restores_its_scope_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let variable_chirho = state_chirho
        .fresh_chirho(None, SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    let application_chirho = runtime_type_chirho(TyChirho::VarChirho(variable_chirho));
    let error_chirho = state_chirho
        .check_rigid_chirho(
            &application_chirho,
            &ElaborationStateChirho::type_universe_chirho(),
            &[variable_chirho],
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap_err();
    assert!(error_chirho.mismatch_chirho.is_some());
    assert!(
        state_chirho
            .substitution_chirho
            .lookup_chirho(&variable_chirho)
            .is_none()
    );
    assert!(state_chirho.rigid_variables_chirho.is_empty());
    state_chirho
        .unify_chirho(
            &application_chirho,
            &ElaborationStateChirho::type_universe_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
}

#[test]
fn representation_normalization_preserves_unlifted_and_opaque_terms_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let unlifted_chirho = runtime_type_chirho(
        crate::ty_chirho::representation_chirho::boxed_representation_chirho(false).clone(),
    );
    let opaque_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("TYPE".into())),
        Box::new(
            crate::ty_chirho::representation_chirho::boxed_representation_chirho(true).clone(),
        ),
    );
    for term_chirho in [unlifted_chirho, opaque_chirho] {
        assert_eq!(
            state_chirho
                .resolve_chirho(&term_chirho, SpanChirho::DUMMY_CHIRHO)
                .unwrap(),
            term_chirho
        );
        assert!(
            state_chirho
                .unify_chirho(
                    &term_chirho,
                    &ElaborationStateChirho::type_universe_chirho(),
                    SpanChirho::DUMMY_CHIRHO
                )
                .is_err()
        );
    }
}

#[test]
fn representation_normalization_exhaustion_stays_terminal_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let term_chirho = runtime_type_chirho(
        crate::ty_chirho::representation_chirho::boxed_representation_chirho(true).clone(),
    );
    state_chirho.output_left_chirho = 0;
    let error_chirho = state_chirho
        .resolve_chirho(&term_chirho, SpanChirho::DUMMY_CHIRHO)
        .unwrap_err();
    assert_eq!(state_chirho.failure_chirho(), Some(&error_chirho));
    assert_eq!(
        state_chirho
            .resolve_chirho(
                &ElaborationStateChirho::type_universe_chirho(),
                SpanChirho::DUMMY_CHIRHO
            )
            .unwrap_err(),
        error_chirho
    );
}

#[test]
fn binding_role_requires_a_live_producer_and_survives_variable_links_chirho() {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let unknown_chirho = state_chirho.fresh_chirho(None, span_chirho).unwrap();
    state_chirho
        .note_source_variable_chirho(
            unknown_chirho,
            haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho,
            true,
            span_chirho,
        )
        .unwrap();
    state_chirho.set_variable_kind_chirho(
        unknown_chirho,
        ElaborationStateChirho::type_universe_chirho(),
    );
    assert_eq!(
        state_chirho.variable_binding_chirho(unknown_chirho),
        ElaborationVariableBindingChirho::UnknownChirho
    );
    let meta_chirho = state_chirho
        .fresh_chirho(
            Some(ElaborationStateChirho::type_universe_chirho()),
            span_chirho,
        )
        .unwrap();
    assert_eq!(
        state_chirho.variable_binding_chirho(meta_chirho),
        ElaborationVariableBindingChirho::MetaChirho
    );
    let universal_chirho = state_chirho
        .fresh_chirho(
            Some(ElaborationStateChirho::type_universe_chirho()),
            span_chirho,
        )
        .unwrap();
    state_chirho
        .mark_universal_chirho(universal_chirho, span_chirho)
        .unwrap();
    // No source specificity on either variable: the barrier still follows the
    // actual representative selected by the ordinary unifier.
    state_chirho
        .unify_chirho(
            &TyChirho::VarChirho(universal_chirho),
            &TyChirho::VarChirho(meta_chirho),
            span_chirho,
        )
        .unwrap();
    let TyChirho::VarChirho(root_chirho) = state_chirho
        .resolve_chirho(&TyChirho::VarChirho(universal_chirho), span_chirho)
        .unwrap()
    else {
        panic!("variable link must retain a variable representative")
    };
    assert_eq!(
        state_chirho.variable_binding_chirho(root_chirho),
        ElaborationVariableBindingChirho::UniversalChirho
    );
    let fresh_chirho = state_chirho
        .fresh_chirho(
            Some(ElaborationStateChirho::type_universe_chirho()),
            span_chirho,
        )
        .unwrap();
    state_chirho
        .unify_chirho(
            &TyChirho::VarChirho(unknown_chirho),
            &TyChirho::VarChirho(fresh_chirho),
            span_chirho,
        )
        .unwrap();
    let TyChirho::VarChirho(root_chirho) = state_chirho
        .resolve_chirho(&TyChirho::VarChirho(unknown_chirho), span_chirho)
        .unwrap()
    else {
        panic!("unknown link must retain a variable representative")
    };
    assert_eq!(
        state_chirho.variable_binding_chirho(root_chirho),
        ElaborationVariableBindingChirho::UnknownChirho
    );
}

#[test]
fn universal_binding_status_does_not_replace_scoped_rigidity_chirho() {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let universal_chirho = state_chirho
        .fresh_chirho(
            Some(ElaborationStateChirho::type_universe_chirho()),
            span_chirho,
        )
        .unwrap();
    state_chirho
        .mark_universal_chirho(universal_chirho, span_chirho)
        .unwrap();
    let mark_chirho = state_chirho
        .open_rigid_scope_chirho(&[universal_chirho], span_chirho)
        .unwrap();
    assert!(
        state_chirho
            .unify_chirho(
                &TyChirho::VarChirho(universal_chirho),
                &ElaborationStateChirho::constraint_universe_chirho(),
                span_chirho
            )
            .is_err()
    );
    state_chirho.close_rigid_scope_chirho(mark_chirho);
    state_chirho
        .unify_chirho(
            &TyChirho::VarChirho(universal_chirho),
            &ElaborationStateChirho::constraint_universe_chirho(),
            span_chirho,
        )
        .expect("temporary scope restoration leaves evidence-based solving available");
}

#[test]
fn representation_defaults_only_live_unprotected_meta_roots_chirho() {
    use haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho;
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let runtime_rep_chirho = TyChirho::ConChirho(
        CompilerRepresentationChirho::RuntimeRepChirho
            .type_key_chirho()
            .into(),
    );
    let meta_chirho = state_chirho
        .fresh_chirho(Some(runtime_rep_chirho.clone()), span_chirho)
        .unwrap();
    let universal_chirho = state_chirho
        .fresh_chirho(Some(runtime_rep_chirho.clone()), span_chirho)
        .unwrap();
    state_chirho
        .mark_universal_chirho(universal_chirho, span_chirho)
        .unwrap();
    let unknown_chirho = state_chirho.fresh_chirho(None, span_chirho).unwrap();
    state_chirho.set_variable_kind_chirho(unknown_chirho, runtime_rep_chirho.clone());
    let protected_chirho = state_chirho
        .fresh_chirho(Some(runtime_rep_chirho), span_chirho)
        .unwrap();
    let candidates_chirho = HashSet::from([
        meta_chirho,
        universal_chirho,
        unknown_chirho,
        protected_chirho,
    ]);
    let defaults_chirho = state_chirho
        .default_representation_variables_chirho(
            &candidates_chirho,
            &HashSet::from([protected_chirho]),
            span_chirho,
        )
        .unwrap();
    assert_eq!(defaults_chirho.len_chirho(), 1);
    assert_eq!(
        defaults_chirho.lookup_chirho(&meta_chirho),
        Some(crate::ty_chirho::representation_chirho::boxed_representation_chirho(true))
    );
    for variable_chirho in [universal_chirho, unknown_chirho, protected_chirho] {
        assert_eq!(
            state_chirho
                .resolve_chirho(&TyChirho::VarChirho(variable_chirho), span_chirho)
                .unwrap(),
            TyChirho::VarChirho(variable_chirho)
        );
    }
}

#[test]
fn protected_kind_closure_includes_an_outer_types_representation_chirho() {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let kind_chirho = state_chirho.fresh_value_kind_chirho(span_chirho).unwrap();
    let roots_chirho = kind_chirho.free_vars_chirho();
    let outer_chirho = state_chirho
        .fresh_chirho(Some(kind_chirho.clone()), span_chirho)
        .unwrap();
    let protected_chirho = state_chirho
        .kind_dependency_closure_chirho(&[outer_chirho], span_chirho)
        .unwrap();
    assert!(
        roots_chirho
            .iter()
            .all(|variable_chirho| protected_chirho.contains(variable_chirho))
    );
    let candidates_chirho = roots_chirho.into_iter().collect();
    assert!(
        state_chirho
            .default_representation_variables_chirho(
                &candidates_chirho,
                &protected_chirho,
                span_chirho
            )
            .unwrap()
            .is_empty_chirho()
    );
    let defaults_chirho = state_chirho
        .default_representation_variables_chirho(&candidates_chirho, &HashSet::new(), span_chirho)
        .unwrap();
    assert_eq!(defaults_chirho.len_chirho(), 1);
    assert_eq!(
        state_chirho
            .resolve_chirho(&kind_chirho, span_chirho)
            .unwrap(),
        ElaborationStateChirho::type_universe_chirho()
    );
}

#[test]
fn representation_defaulting_cannot_reset_an_exhausted_state_chirho() {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let kind_chirho = state_chirho.fresh_value_kind_chirho(span_chirho).unwrap();
    let candidates_chirho = kind_chirho.free_vars_chirho().into_iter().collect();
    let failure_chirho = state_chirho
        .charge_chirho(usize::MAX, 0, span_chirho)
        .unwrap_err();
    assert_eq!(
        state_chirho
            .default_representation_variables_chirho(
                &candidates_chirho,
                &HashSet::new(),
                span_chirho
            )
            .unwrap_err(),
        failure_chirho
    );
    assert_eq!(state_chirho.failure_chirho(), Some(&failure_chirho));
}
