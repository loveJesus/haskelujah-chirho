// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Resource exhaustion is an unproved equality, never a successful rewrite.

use super::*;

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
