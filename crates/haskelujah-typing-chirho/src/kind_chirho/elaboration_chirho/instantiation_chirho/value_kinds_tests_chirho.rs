// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The projection keeps lexical ownership, Unknown metadata and the original
//! terminal allowance; it never turns an ambient slot into a binder producer.

use super::*;
use haskelujah_ast_chirho::name_chirho::{CompilerRepresentationChirho, NameChirho, RawNameChirho};

fn with_projection_chirho(action_chirho: impl FnOnce(&mut TypeElaboratorChirho<'_>)) {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let module_chirho = ModuleChirho {
        name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "ProjectionChirho",
            span_chirho,
        )),
        exports_chirho: None,
        imports_chirho: Vec::new(),
        decls_chirho: Vec::new(),
        extensions_chirho: Vec::new(),
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: Vec::new(),
        deriving_via_chirho: Vec::new(),
        span_chirho,
        origin_supply_chirho: Default::default(),
    };
    let registry_chirho = TypeFamilyEnvChirho::default();
    let type_scope_chirho = TypeConstructorScopeChirho::default();
    let family_scope_chirho = TypeFamilyScopeChirho::default();
    let promotion_scope_chirho = PromotionScopeChirho::default();
    let builtin_classes_chirho = BuiltinClassScopeChirho::default();
    let mut elaborator_chirho = TypeElaboratorChirho::new_chirho(
        &module_chirho,
        ElaborationImportsChirho {
            registry_chirho: &registry_chirho,
            type_scope_chirho: &type_scope_chirho,
            family_scope_chirho: &family_scope_chirho,
            promotion_scope_chirho: &promotion_scope_chirho,
            builtin_classes_chirho: &builtin_classes_chirho,
            source_unit_chirho: None,
        },
    )
    .unwrap();
    action_chirho(&mut elaborator_chirho);
}

#[test]
fn unknown_local_binder_cannot_borrow_an_ambient_kind_chirho() {
    with_projection_chirho(|elaborator_chirho| {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        let variable_chirho = elaborator_chirho
            .state_chirho
            .fresh_chirho(
                Some(ElaborationStateChirho::type_universe_chirho()),
                span_chirho,
            )
            .unwrap();
        let term_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![variable_chirho].into(),
            body_chirho: Box::new(TyChirho::VarChirho(variable_chirho)),
        };
        assert_eq!(
            elaborator_chirho
                .checked_term_kind_chirho(&term_chirho, span_chirho)
                .unwrap(),
            None
        );
        assert!(elaborator_chirho.state_chirho.failure_chirho().is_none());
    });
}

#[test]
fn local_kind_dependencies_are_alpha_opened_before_ambient_substitution_chirho() {
    with_projection_chirho(|elaborator_chirho| {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        let representation_chirho = elaborator_chirho
            .state_chirho
            .fresh_chirho(None, span_chirho)
            .unwrap();
        let value_chirho = elaborator_chirho
            .state_chirho
            .fresh_chirho(None, span_chirho)
            .unwrap();
        let runtime_rep_chirho = TyChirho::ConChirho(
            CompilerRepresentationChirho::RuntimeRepChirho
                .type_key_chirho()
                .into(),
        );
        let runtime_type_chirho = TyChirho::ConChirho(
            CompilerRepresentationChirho::RuntimeTypeChirho
                .type_key_chirho()
                .into(),
        );
        let kind_chirho = TyChirho::AppChirho(
            Box::new(runtime_type_chirho),
            Box::new(TyChirho::VarChirho(representation_chirho)),
        );
        let record_chirho = |variable_chirho, kind_chirho| QuantifiedVariableChirho {
            variable_chirho,
            kind_chirho: Some(Arc::new(kind_chirho)),
            source_chirho: None,
            specificity_chirho: Some(TyVarSpecificityChirho::SpecifiedChirho),
        };
        let term_chirho = TyChirho::ForallChirho {
            vars_chirho: QuantifiedVariablesChirho::from_records_chirho(vec![
                record_chirho(representation_chirho, runtime_rep_chirho),
                record_chirho(value_chirho, kind_chirho),
            ]),
            body_chirho: Box::new(TyChirho::VarChirho(value_chirho)),
        };
        elaborator_chirho
            .state_chirho
            .substitution_chirho
            .insert_chirho(
                representation_chirho,
                crate::ty_chirho::representation_chirho::boxed_representation_chirho(true).clone(),
            );
        let projected_chirho = elaborator_chirho
            .checked_term_kind_chirho(&term_chirho, span_chirho)
            .unwrap()
            .unwrap();
        assert_ne!(
            projected_chirho,
            ElaborationStateChirho::type_universe_chirho()
        );
        let TyChirho::AppChirho(_, argument_chirho) = projected_chirho else {
            panic!("the bound representation remains explicit");
        };
        let TyChirho::VarChirho(fresh_chirho) = *argument_chirho else {
            panic!("the binder was not specialized");
        };
        assert_ne!(fresh_chirho, representation_chirho);
        assert_eq!(
            elaborator_chirho
                .state_chirho
                .variable_binding_chirho(fresh_chirho),
            ElaborationVariableBindingChirho::UniversalChirho
        );
    });
}

#[test]
fn value_projection_depth_failure_remains_terminal_for_a_tiny_followup_chirho() {
    with_projection_chirho(|elaborator_chirho| {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        let mut term_chirho = TyChirho::ConChirho("OpaqueChirho".into());
        for _ in 0..130 {
            term_chirho = TyChirho::ListChirho(Box::new(term_chirho));
        }
        let error_chirho = elaborator_chirho
            .checked_term_kind_chirho(&term_chirho, span_chirho)
            .unwrap_err();
        assert_eq!(
            elaborator_chirho.state_chirho.failure_chirho(),
            Some(&error_chirho)
        );
        let tiny_chirho = elaborator_chirho
            .checked_term_kind_chirho(&TyChirho::ConChirho("OpaqueChirho".into()), span_chirho)
            .unwrap_err();
        assert_eq!(tiny_chirho, error_chirho);
        assert_eq!(
            elaborator_chirho.state_chirho.failure_chirho(),
            Some(&error_chirho)
        );
    });
}

#[test]
fn parameter_refinement_prefix_keeps_outer_evidence_and_excludes_own_patterns_chirho() {
    let mut environment_chirho = crate::env_chirho::TyEnvChirho::new_chirho();
    environment_chirho
        .add_refinement_chirho("outerChirho".into(), TyChirho::VarChirho(TyVarChirho(91)));
    let entry_chirho = environment_chirho.refinement_mark_chirho();
    environment_chirho.add_refinement_chirho("ownChirho".into(), TyChirho::bool_chirho());
    environment_chirho.push_scope_chirho();
    environment_chirho.add_refinement_chirho("outerChirho".into(), TyChirho::bool_chirho());
    environment_chirho.apply_subst_chirho(&SubstChirho::singleton_chirho(
        TyVarChirho(91),
        TyChirho::int_chirho(),
    ));
    assert!(environment_chirho.refinement_mark_is_live_chirho(entry_chirho));
    assert_eq!(
        environment_chirho.lookup_refinement_at_mark_chirho("outerChirho", Some(entry_chirho)),
        Some(&TyChirho::int_chirho())
    );
    assert_eq!(
        environment_chirho.lookup_refinement_at_mark_chirho("ownChirho", Some(entry_chirho)),
        None
    );
    environment_chirho.pop_scope_chirho();
    assert!(environment_chirho.refinement_mark_is_live_chirho(entry_chirho));
}

#[test]
fn parameter_refinement_token_cannot_survive_its_captured_scope_chirho() {
    let mut environment_chirho = crate::env_chirho::TyEnvChirho::new_chirho();
    environment_chirho.push_scope_chirho();
    let entry_chirho = environment_chirho.refinement_mark_chirho();
    environment_chirho.pop_scope_chirho();
    assert!(!environment_chirho.refinement_mark_is_live_chirho(entry_chirho));
}

#[test]
fn representation_fixedness_distinguishes_metas_universals_and_missing_proof_chirho() {
    with_projection_chirho(|elaborator_chirho| {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        let kind_chirho = elaborator_chirho
            .state_chirho
            .fresh_value_kind_chirho(span_chirho)
            .unwrap();
        let TyChirho::AppChirho(_, representation_chirho) = &kind_chirho else {
            panic!("actual open value kind");
        };
        let TyChirho::VarChirho(variable_chirho) = **representation_chirho else {
            panic!("actual inference meta");
        };
        assert_eq!(
            elaborator_chirho
                .state_chirho
                .value_representation_chirho(
                    &kind_chirho,
                    &elaborator_chirho.registry_chirho,
                    span_chirho
                )
                .unwrap(),
            ValueRepresentationChirho::FLEXIBLE_CHIRHO
        );
        elaborator_chirho
            .state_chirho
            .mark_universal_chirho(variable_chirho, span_chirho)
            .unwrap();
        assert_eq!(
            elaborator_chirho
                .state_chirho
                .value_representation_chirho(
                    &kind_chirho,
                    &elaborator_chirho.registry_chirho,
                    span_chirho
                )
                .unwrap(),
            ValueRepresentationChirho::NONFIXED_CHIRHO
        );
        let opaque_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(
                CompilerRepresentationChirho::RuntimeTypeChirho
                    .type_key_chirho()
                    .into(),
            )),
            Box::new(TyChirho::ConChirho("IntRep".into())),
        );
        assert_eq!(
            elaborator_chirho
                .state_chirho
                .value_representation_chirho(
                    &opaque_chirho,
                    &elaborator_chirho.registry_chirho,
                    span_chirho
                )
                .unwrap(),
            ValueRepresentationChirho::UNKNOWN_CHIRHO
        );
    });
}

#[test]
fn missing_representation_metadata_neither_certifies_nor_adds_an_ordinary_failure_chirho() {
    with_projection_chirho(|elaborator_chirho| {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        let kind_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(
                CompilerRepresentationChirho::RuntimeTypeChirho
                    .type_key_chirho()
                    .into(),
            )),
            Box::new(TyChirho::ConChirho("UnverifiedRepresentationChirho".into())),
        );
        assert_eq!(
            elaborator_chirho
                .state_chirho
                .value_representation_chirho(
                    &kind_chirho,
                    &elaborator_chirho.registry_chirho,
                    span_chirho
                )
                .unwrap(),
            ValueRepresentationChirho::UNKNOWN_CHIRHO
        );
        elaborator_chirho
            .state_chirho
            .queue_constructor_field_kinds_chirho(vec![(kind_chirho, span_chirho)], span_chirho)
            .unwrap();
        let substitutions_chirho = SubstChirho::empty_chirho();
        let synonyms_chirho = HashMap::new();
        elaborator_chirho.state_chirho.finish_constructor_field_kinds_chirho(&elaborator_chirho.registry_chirho,
            crate::infer_chirho::EqualityNormalizationViewChirho {
                substitutions_chirho: &substitutions_chirho, legacy_synonyms_chirho: &synonyms_chirho,
                type_scope_chirho: None, family_scope_chirho: None, family_module_chirho: "ProjectionChirho",
                promotion_present_chirho: true, evidence_chirho: Default::default(),
            }).expect("Unknown representation metadata stays non-authoritative without a new ordinary error");
        assert!(elaborator_chirho.state_chirho.failure_chirho().is_none());
    });
}
