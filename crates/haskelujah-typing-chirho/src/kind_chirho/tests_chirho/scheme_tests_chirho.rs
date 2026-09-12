// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn classifier_checks_are_bounded_and_retire_before_the_next_scope_chirho() {
    let mut context_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());
    let parameter_chirho = context_chirho.fresh_var_chirho();
    context_chirho
        .kind_binder_classifiers_chirho
        .insert(parameter_chirho, KindChirho::StarChirho);
    let deep_chirho = (0..160).fold(
        KindChirho::ConChirho("Int".into()),
        |argument_chirho, _index_chirho| {
            KindChirho::app_chirho(KindChirho::ConChirho("Maybe".into()), argument_chirho)
        },
    );
    context_chirho.unify_chirho(
        &KindChirho::VarChirho(parameter_chirho),
        &deep_chirho,
        "bounded classifier",
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(
        context_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| diagnostic_chirho
                .message_chirho
                .contains("classifier checking exceeded its resource bound")),
        "exhaustion must report its own outcome, not an unrelated error"
    );

    let mut context_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());
    for (actual_chirho, invalid_chirho) in [
        (KindChirho::StarChirho, false),
        (
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
            true,
        ),
    ] {
        context_chirho
            .env_chirho
            .bind_chirho("LocalChirho".into(), actual_chirho);
        let parameter_chirho = context_chirho.fresh_var_chirho();
        context_chirho
            .kind_binder_classifiers_chirho
            .insert(parameter_chirho, KindChirho::StarChirho);
        context_chirho.unify_chirho(
            &KindChirho::VarChirho(parameter_chirho),
            &KindChirho::ConChirho("LocalChirho".into()),
            "changed local binding",
            SpanChirho::DUMMY_CHIRHO,
        );
        assert_eq!(
            context_chirho.diagnostics_chirho.has_errors_chirho(),
            invalid_chirho
        );
    }
}

#[test]
fn inferred_kind_arguments_solve_classifiers_without_sharing_occurrences_chirho() {
    let mut context_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());
    let kind_chirho = KindVarChirho(200);
    let value_chirho = KindVarChirho(201);
    let scheme_chirho = KindSchemeChirho {
        quantified_chirho: vec![kind_chirho, value_chirho],
        specified_chirho: [kind_chirho, value_chirho].into_iter().collect(),
        classifiers_chirho: vec![KindChirho::StarChirho, KindChirho::VarChirho(kind_chirho)],
        source_names_chirho: vec![None, None],
        body_chirho: KindChirho::arrow_chirho(
            KindChirho::VarChirho(value_chirho),
            KindChirho::StarChirho,
        ),
    };
    let (_, first_chirho) = context_chirho.open_kind_scheme_parts_chirho(&scheme_chirho, false);
    let (_, second_chirho) = context_chirho.open_kind_scheme_parts_chirho(&scheme_chirho, false);
    context_chirho.unify_chirho(
        &first_chirho[1],
        &KindChirho::ConChirho("Either".into()),
        "inferred argument",
        SpanChirho::DUMMY_CHIRHO,
    );
    context_chirho.unify_chirho(
        &second_chirho[1],
        &KindChirho::ConChirho("Int".into()),
        "independent argument",
        SpanChirho::DUMMY_CHIRHO,
    );
    assert_eq!(
        context_chirho.subst_chirho.apply_chirho(&first_chirho[0]),
        KindChirho::arrow_n_chirho(
            [KindChirho::StarChirho, KindChirho::StarChirho],
            KindChirho::StarChirho
        )
    );
    assert_eq!(
        context_chirho.subst_chirho.apply_chirho(&second_chirho[0]),
        KindChirho::StarChirho
    );
    assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
    let (_, invalid_chirho) = context_chirho.open_kind_scheme_parts_chirho(&scheme_chirho, false);
    context_chirho.unify_chirho(
        &invalid_chirho[0],
        &KindChirho::StarChirho,
        "fixed classifier",
        SpanChirho::DUMMY_CHIRHO,
    );
    context_chirho.unify_chirho(
        &invalid_chirho[1],
        &KindChirho::ConChirho("Either".into()),
        "wrong inferred argument",
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(context_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn opaque_promoted_occurrences_neither_capture_each_other_nor_type_names_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
    let name_chirho = haskelujah_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
        "WrapChirho",
        SpanChirho::DUMMY_CHIRHO,
    );
    let constructor_chirho = TypeChirho::PromotedConChirho {
        name_chirho: haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(name_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let first_chirho = ctx_chirho.infer_type_kind_chirho(&constructor_chirho);
    ctx_chirho.unify_chirho(
        &first_chirho,
        &KindChirho::StarChirho,
        "first occurrence",
        SpanChirho::DUMMY_CHIRHO,
    );
    let second_chirho = ctx_chirho.infer_type_kind_chirho(&constructor_chirho);
    ctx_chirho.unify_chirho(
        &second_chirho,
        &KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
        "second occurrence",
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "opaque promoted occurrences must not share a monomorphic hole"
    );
    assert!(
        ctx_chirho.env_chirho.lookup_chirho("WrapChirho").is_none(),
        "a data-constructor occurrence must not create a type-namespace binding"
    );
}

#[test]
fn scheme_instances_share_within_but_not_between_uses_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
    let variable_chirho = ctx_chirho.fresh_kind_chirho();
    let scheme_chirho = KindSchemeChirho::generalize_chirho(KindChirho::arrow_chirho(
        variable_chirho.clone(),
        variable_chirho,
    ));
    let first_chirho = ctx_chirho.open_kind_scheme_chirho(&scheme_chirho, false);
    let second_chirho = ctx_chirho.open_kind_scheme_chirho(&scheme_chirho, false);
    let KindChirho::ArrowChirho(left_chirho, right_chirho) = &first_chirho else {
        panic!("Expected arrow kind");
    };
    assert_eq!(left_chirho, right_chirho);
    assert!(
        first_chirho
            .free_vars_chirho()
            .iter()
            .all(|variable_chirho| !second_chirho.free_vars_chirho().contains(variable_chirho))
    );
}

#[test]
fn monomorphic_binding_keeps_its_inference_identity_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
    let kind_chirho = ctx_chirho.fresh_kind_chirho();
    let binding_chirho = KindBindingChirho::MonoChirho(kind_chirho.clone());
    assert_eq!(
        ctx_chirho.instantiate_binding_chirho(&binding_chirho),
        kind_chirho
    );
    ctx_chirho.unify_chirho(
        &kind_chirho,
        &KindChirho::StarChirho,
        "control",
        SpanChirho::DUMMY_CHIRHO,
    );
    assert_eq!(
        ctx_chirho.instantiate_binding_chirho(&binding_chirho),
        KindChirho::StarChirho
    );
}

#[test]
fn bound_kind_variables_ignore_ambient_substitution_and_defaulting_chirho() {
    let variable_chirho = KindVarChirho(100);
    let mut env_chirho = KindEnvChirho::new_chirho();
    env_chirho.bind_generalized_chirho(
        "BoxChirho".into(),
        KindChirho::arrow_chirho(
            KindChirho::VarChirho(variable_chirho),
            KindChirho::StarChirho,
        ),
    );
    env_chirho.apply_subst_chirho(&KindSubstChirho::singleton_chirho(
        variable_chirho,
        KindChirho::StarChirho,
    ));
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(env_chirho);
    ctx_chirho.finalize_chirho(true);
    assert_eq!(
        ctx_chirho
            .env_chirho
            .lookup_chirho("BoxChirho")
            .unwrap()
            .free_vars_chirho(),
        vec![variable_chirho]
    );
    let binding_chirho = ctx_chirho
        .env_chirho
        .lookup_binding_chirho("BoxChirho")
        .unwrap()
        .clone();
    let instance_chirho = ctx_chirho.instantiate_binding_chirho(&binding_chirho);
    assert!(
        !instance_chirho
            .free_vars_chirho()
            .contains(&variable_chirho)
    );
}

#[test]
fn rigid_kinds_unify_only_with_themselves_or_flexible_variables_chirho() {
    let rigid_chirho = KindChirho::RigidChirho(KindVarChirho(0));
    let fresh_chirho = KindChirho::VarChirho(KindVarChirho(1));
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    for (left_chirho, right_chirho) in [
        (&rigid_chirho, &fresh_chirho),
        (&fresh_chirho, &rigid_chirho),
    ] {
        let subst_chirho =
            unify_kind_chirho(left_chirho, right_chirho, "rigid control", span_chirho).unwrap();
        assert_eq!(subst_chirho.apply_chirho(&fresh_chirho), rigid_chirho);
        assert!(!subst_chirho.map_chirho.contains_key(&KindVarChirho(0)));
    }
    assert!(unify_kind_chirho(&rigid_chirho, &rigid_chirho, "same", span_chirho).is_ok());
    for bad_chirho in [
        KindChirho::StarChirho,
        KindChirho::RigidChirho(KindVarChirho(2)),
    ] {
        assert!(unify_kind_chirho(&rigid_chirho, &bad_chirho, "different", span_chirho).is_err());
    }
    assert!(
        unify_kind_chirho(
            &fresh_chirho,
            &KindChirho::arrow_chirho(fresh_chirho.clone(), rigid_chirho),
            "occurs",
            span_chirho
        )
        .is_err()
    );
}

#[test]
fn nested_constructor_scopes_restore_shadowed_bindings_chirho() {
    let mut env_chirho = KindEnvChirho::new_chirho();
    env_chirho.bind_generalized_chirho("BoxChirho".into(), KindChirho::VarChirho(KindVarChirho(8)));
    env_chirho.begin_scope_chirho();
    env_chirho.hide_chirho("BoxChirho");
    env_chirho.bind_chirho("localChirho".into(), KindChirho::StarChirho);
    env_chirho.begin_scope_chirho();
    env_chirho.bind_chirho("localChirho".into(), KindChirho::ConstraintChirho);
    env_chirho.end_scope_chirho();
    assert_eq!(
        env_chirho.lookup_chirho("localChirho"),
        Some(&KindChirho::StarChirho)
    );
    env_chirho.end_scope_chirho();
    assert!(env_chirho.lookup_chirho("localChirho").is_none());
    assert!(matches!(
        env_chirho.lookup_binding_chirho("BoxChirho"),
        Some(KindBindingChirho::PolyChirho(_))
    ));
}

#[test]
fn kind_extension_override_order_keeps_standalone_separate_chirho() {
    for (extensions_chirho, expected_chirho) in [
        (vec!["NoCUSKs", "CUSKs"], true),
        (vec!["CUSKs", "StandaloneKindSignatures"], false),
        (vec!["StandaloneKindSignatures", "CUSKs"], true),
        (vec!["Haskell2010", "NoCUSKs"], false),
    ] {
        let flags_chirho = extensions_chirho
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        assert_eq!(
            constructors_chirho::cusks_enabled_chirho(&flags_chirho),
            expected_chirho
        );
    }
}
