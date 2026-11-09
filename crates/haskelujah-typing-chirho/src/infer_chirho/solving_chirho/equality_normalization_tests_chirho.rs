// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Resource exhaustion is an unproved equality, never a successful rewrite.

use super::*;

#[test]
fn empty_catalogue_and_equality_images_normalize_only_typed_lifted_universes_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let lifted_chirho =
        crate::ty_chirho::representation_chirho::boxed_representation_chirho(true).clone();
    let intrinsic_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho(
            haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho::RuntimeTypeChirho
                .type_key_chirho()
                .into(),
        )),
        Box::new(lifted_chirho.clone()),
    );
    let opaque_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("TYPE".into())),
        Box::new(lifted_chirho),
    );
    let type_chirho = TyChirho::ConChirho("%universe:type".into());
    assert_eq!(
        ctx_chirho.expand_type_synonyms_chirho(&intrinsic_chirho),
        type_chirho
    );
    assert_eq!(
        ctx_chirho
            .normalize_equality_type_chirho(&intrinsic_chirho, 0)
            .unwrap(),
        type_chirho
    );
    assert_eq!(
        ctx_chirho.expand_type_synonyms_chirho(&opaque_chirho),
        opaque_chirho
    );
    assert_eq!(
        ctx_chirho
            .normalize_equality_type_chirho(&opaque_chirho, 0)
            .unwrap(),
        opaque_chirho
    );
    ctx_chirho
        .equality_substitutions_chirho
        .insert_chirho(TyVarChirho(74), intrinsic_chirho);
    assert_eq!(
        ctx_chirho
            .normalize_equality_type_chirho(&TyChirho::VarChirho(TyVarChirho(74)), 0)
            .unwrap(),
        type_chirho
    );
}

#[test]
fn growing_evidence_exhausts_one_shared_output_allowance_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let source_chirho = TyChirho::ConChirho("GrowingChirho".to_owned());
    ctx_chirho.record_equality_rewrites_chirho(vec![(
        source_chirho.clone(),
        TyChirho::TupleChirho(vec![source_chirho.clone(), source_chirho.clone()]),
    )]);
    assert!(
        ctx_chirho
            .normalize_equality_type_chirho(&source_chirho, 0)
            .is_err()
    );
    ctx_chirho.defer_equality_chirho(
        source_chirho,
        TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(
        ctx_chirho
            .retry_deferred_equalities_chirho()
            .is_empty_chirho()
    );
    assert_eq!(ctx_chirho.deferred_equalities_chirho.len(), 1);
    ctx_chirho.finish_equality_binding_group_chirho(&[0], &[]);
    assert!(ctx_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn cyclic_evidence_is_unknown_and_is_not_generalized_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let a_chirho = TyChirho::ConChirho("CycleAChirho".to_owned());
    let b_chirho = TyChirho::ConChirho("CycleBChirho".to_owned());
    ctx_chirho.record_equality_rewrites_chirho(vec![
        (a_chirho.clone(), b_chirho.clone()),
        (b_chirho, a_chirho.clone()),
    ]);
    assert!(
        ctx_chirho
            .normalize_equality_type_chirho(&a_chirho, 0)
            .is_err()
    );
    ctx_chirho.defer_equality_chirho(
        a_chirho,
        TyChirho::VarChirho(TyVarChirho(900)),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(
        ctx_chirho
            .generalizable_equality_preds_chirho(&[TyVarChirho(900)])
            .is_empty()
    );
    assert!(!ctx_chirho.deferred_equalities_chirho[0].generalized_chirho);
}

#[test]
fn a_long_linked_evidence_chain_is_charged_before_copying_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    for index_chirho in 0..2_000 {
        ctx_chirho.record_equality_rewrites_chirho(vec![(
            TyChirho::ConChirho(format!("Source{index_chirho}Chirho")),
            TyChirho::ConChirho(format!("Target{index_chirho}Chirho")),
        )]);
    }
    assert!(
        ctx_chirho
            .normalize_equality_type_chirho(&TyChirho::int_chirho(), 0)
            .is_err()
    );
}

#[test]
fn synonym_formals_never_capture_global_equality_evidence_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let formal_chirho = TyChirho::ForallVarChirho("aChirho".to_owned());
    let family_chirho = |argument_chirho| {
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("OpaqueChirho".to_owned())),
            Box::new(argument_chirho),
        )
    };
    ctx_chirho.type_synonyms_chirho.insert(
        "AliasChirho".to_owned(),
        (
            vec!["aChirho".to_owned()],
            family_chirho(formal_chirho.clone()),
        ),
    );
    ctx_chirho.record_equality_rewrites_chirho(vec![(
        family_chirho(formal_chirho),
        TyChirho::int_chirho(),
    )]);
    let application_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("AliasChirho".to_owned())),
        Box::new(TyChirho::bool_chirho()),
    );
    assert_eq!(
        ctx_chirho
            .normalize_equality_type_chirho(&application_chirho, 0)
            .unwrap(),
        family_chirho(TyChirho::bool_chirho())
    );
}

#[test]
fn sparse_substitutions_are_chased_without_becoming_synonym_arguments_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho
        .equality_substitutions_chirho
        .insert_chirho(TyVarChirho(900), TyChirho::VarChirho(TyVarChirho(901)));
    ctx_chirho
        .equality_substitutions_chirho
        .insert_chirho(TyVarChirho(901), TyChirho::int_chirho());
    assert_eq!(
        ctx_chirho
            .normalize_equality_type_chirho(&TyChirho::VarChirho(TyVarChirho(900)), 0)
            .unwrap(),
        TyChirho::int_chirho()
    );
}

#[test]
fn tentative_images_exhaust_output_before_copying_the_expansion_chirho() {
    let variable_chirho = TyVarChirho(990);
    let source_chirho = TyChirho::TupleChirho(vec![TyChirho::VarChirho(variable_chirho); 1024]);
    let image_chirho = TyChirho::TupleChirho(vec![TyChirho::int_chirho(); 64]);
    let mut budget_chirho = equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
    assert!(
        equality_normalization_chirho::substitute_meta_with_budget_chirho(
            &source_chirho,
            variable_chirho,
            &image_chirho,
            &mut budget_chirho
        )
        .is_none()
    );
}

#[test]
fn ordinary_unification_exhaustion_is_coded_and_non_deferrable_chirho() {
    let context_chirho = InferCtxChirho::new_chirho();
    let oversized_chirho = TyChirho::TupleChirho(vec![TyChirho::int_chirho(); 16_385]);
    let failure_chirho = context_chirho.unify_ordinary_chirho(
        &TyChirho::VarChirho(TyVarChirho(990)),
        &oversized_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(matches!(
        failure_chirho,
        Err(UnifyErrorChirho::ResourceExhaustedChirho { .. })
    ));
    assert!(context_chirho.has_non_deferrable_failure_chirho());
    let result_chirho = context_chirho.finish_chirho();
    assert!(result_chirho.diagnostics_chirho.is_empty_chirho());
    assert!(
        result_chirho
            .non_deferrable_diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| diagnostic_chirho.to_string().contains("error[E0300]"))
    );
}

#[test]
fn stable_owned_public_images_reuse_storage_but_borrowed_results_need_output_chirho() {
    for checked_owner_chirho in [false, true] {
        let mut context_chirho = InferCtxChirho::new_chirho();
        if checked_owner_chirho {
            context_chirho.elaboration_state_chirho =
                crate::kind_chirho::elaboration_chirho::ElaborationStateChirho::new_chirho()
                    .unwrap();
        }
        let input_chirho =
            TyChirho::ListChirho(Box::new(TyChirho::ConChirho("StableValueChirho".repeat(8))));
        let TyChirho::ListChirho(element_chirho) = &input_chirho else {
            unreachable!()
        };
        let TyChirho::ConChirho(name_chirho) = element_chirho.as_ref() else {
            unreachable!()
        };
        let buffer_chirho = name_chirho.as_ptr();
        let mut budget_chirho =
            equality_normalization_chirho::EqualityBudgetChirho::limited_chirho(100_000, 0);
        let change_chirho = context_chirho
            .normalize_signature_public_image_change_chirho(&input_chirho, &mut budget_chirho)
            .unwrap();
        assert!(change_chirho.is_unchanged_chirho());
        assert!(budget_chirho.remaining_chirho().0 < 100_000);
        assert_eq!(budget_chirho.remaining_chirho().1, 0);
        let output_chirho = change_chirho.retain_owned_chirho(input_chirho);
        let TyChirho::ListChirho(element_chirho) = &output_chirho else {
            unreachable!()
        };
        let TyChirho::ConChirho(name_chirho) = element_chirho.as_ref() else {
            unreachable!()
        };
        assert_eq!(name_chirho.as_ptr(), buffer_chirho);
        let mut borrowed_budget_chirho =
            equality_normalization_chirho::EqualityBudgetChirho::limited_chirho(100_000, 0);
        assert!(
            context_chirho
                .normalize_signature_public_image_chirho(
                    &output_chirho,
                    &mut borrowed_budget_chirho
                )
                .is_err()
        );
        assert!(borrowed_budget_chirho.remaining_chirho().0 < 100_000);
        assert_eq!(borrowed_budget_chirho.remaining_chirho().1, 0);
    }
}

#[test]
fn a_shrinking_public_rewrite_materializes_its_result_without_an_eager_input_copy_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let name_chirho = "LargeOriginalChirho".repeat(64);
    let source_chirho = TyChirho::ConChirho(name_chirho.clone());
    context_chirho
        .record_equality_rewrites_chirho(vec![(source_chirho.clone(), TyChirho::bool_chirho())]);
    // One evidence chunk, its two copied premises, and the selected Bool image.
    // There is no allowance for a second complete copy of the large input.
    let output_chirho = 1 + 1 + name_chirho.len() + 5 + 5;
    let mut budget_chirho =
        equality_normalization_chirho::EqualityBudgetChirho::limited_chirho(100_000, output_chirho);
    let change_chirho = context_chirho
        .normalize_signature_public_image_change_chirho(&source_chirho, &mut budget_chirho)
        .unwrap();
    assert!(!change_chirho.is_unchanged_chirho());
    assert_eq!(
        change_chirho.image_chirho(&source_chirho),
        &TyChirho::bool_chirho()
    );
    assert_eq!(budget_chirho.remaining_chirho().1, 0);
    let mut borrowed_budget_chirho =
        equality_normalization_chirho::EqualityBudgetChirho::limited_chirho(100_000, output_chirho);
    assert_eq!(
        context_chirho
            .normalize_signature_public_image_chirho(&source_chirho, &mut borrowed_budget_chirho)
            .unwrap(),
        TyChirho::bool_chirho()
    );
    assert_eq!(borrowed_budget_chirho.remaining_chirho().1, 0);
}

#[test]
fn a_stable_later_pass_preserves_the_earlier_owned_meta_image_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let input_chirho = TyChirho::VarChirho(TyVarChirho(970));
    let image_chirho = TyChirho::ListChirho(Box::new(TyChirho::bool_chirho()));
    context_chirho
        .equality_substitutions_chirho
        .insert_chirho(TyVarChirho(970), image_chirho.clone());
    let mut budget_chirho =
        equality_normalization_chirho::EqualityBudgetChirho::limited_chirho(100_000, 6);
    let change_chirho = context_chirho
        .normalize_signature_public_image_change_chirho(&input_chirho, &mut budget_chirho)
        .unwrap();
    assert!(!change_chirho.is_unchanged_chirho());
    assert_eq!(
        change_chirho.retain_owned_chirho(input_chirho),
        image_chirho
    );
    assert_eq!(budget_chirho.remaining_chirho().1, 0);
}

#[test]
fn the_selected_next_public_image_keeps_metadata_ignored_by_equality_chirho() {
    use haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho;
    let mut context_chirho = InferCtxChirho::new_chirho();
    let make_chirho = |specificity_chirho| TyChirho::ForallChirho {
        vars_chirho: crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(vec![
            crate::ty_chirho::QuantifiedVariableChirho {
                variable_chirho: TyVarChirho(971),
                kind_chirho: Some(std::sync::Arc::new(TyChirho::ConChirho(
                    "%universe:type".into(),
                ))),
                source_chirho: None,
                specificity_chirho: Some(specificity_chirho),
            },
        ]),
        body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(971))),
    };
    let input_chirho = make_chirho(TyVarSpecificityChirho::InferredChirho);
    let selected_chirho = make_chirho(TyVarSpecificityChirho::SpecifiedChirho);
    assert_eq!(input_chirho, selected_chirho);
    context_chirho.record_equality_rewrites_chirho(vec![(input_chirho.clone(), selected_chirho)]);
    let mut budget_chirho = equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
    let change_chirho = context_chirho
        .normalize_signature_public_image_change_chirho(&input_chirho, &mut budget_chirho)
        .unwrap();
    assert!(!change_chirho.is_unchanged_chirho());
    let output_chirho = change_chirho.retain_owned_chirho(input_chirho);
    let TyChirho::ForallChirho { vars_chirho, .. } = output_chirho else {
        panic!("selected universal image remains universal")
    };
    assert_eq!(
        vars_chirho.records_chirho()[0].specificity_chirho,
        Some(TyVarSpecificityChirho::SpecifiedChirho)
    );
}

#[test]
fn copied_sibling_names_and_changed_binder_kinds_keep_their_output_cost_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let variable_chirho = TyVarChirho(972);
    context_chirho
        .equality_substitutions_chirho
        .insert_chirho(variable_chirho, TyChirho::bool_chirho());
    let input_chirho = TyChirho::TupleChirho(vec![
        TyChirho::ConChirho("CopiedSiblingChirho".repeat(64)),
        TyChirho::VarChirho(variable_chirho),
    ]);
    let mut budget_chirho =
        equality_normalization_chirho::EqualityBudgetChirho::limited_chirho(100_000, 32);
    assert!(
        context_chirho
            .normalize_signature_public_image_change_chirho(&input_chirho, &mut budget_chirho)
            .is_err()
    );
    assert!(budget_chirho.remaining_chirho().1 < 32);
    let variable_chirho = TyVarChirho(973);
    context_chirho.equality_substitutions_chirho.insert_chirho(
        variable_chirho,
        TyChirho::ConChirho("CopiedKindChirho".repeat(64)),
    );
    let input_chirho = TyChirho::RequiredForallChirho {
        vars_chirho: crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(vec![
            crate::ty_chirho::QuantifiedVariableChirho {
                variable_chirho: TyVarChirho(974),
                kind_chirho: Some(std::sync::Arc::new(TyChirho::VarChirho(variable_chirho))),
                source_chirho: None,
                specificity_chirho: None,
            },
        ]),
        body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(974))),
    };
    let mut budget_chirho =
        equality_normalization_chirho::EqualityBudgetChirho::limited_chirho(100_000, 32);
    assert!(
        context_chirho
            .normalize_signature_public_image_change_chirho(&input_chirho, &mut budget_chirho)
            .is_err()
    );
    assert!(budget_chirho.remaining_chirho().1 < 32);
}
