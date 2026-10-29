// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Generalized dictionary ownership is produced by top-level schemes and consumed by the
//! method and literal evidence channels with the same predicate index (row 663).

use super::test_support_chirho::dummy_name_chirho;
use super::*;

fn variable_chirho(ty_chirho: &TyChirho) -> TyVarChirho {
    let TyChirho::VarChirho(var_chirho) = ty_chirho else {
        panic!("expected variable, got {ty_chirho:?}");
    };
    *var_chirho
}

fn num_scheme_chirho(var_chirho: TyVarChirho, preceding_chirho: bool) -> SchemeChirho {
    let ty_chirho = TyChirho::VarChirho(var_chirho);
    let mut preds_chirho = Vec::new();
    if preceding_chirho {
        preds_chirho.push(SchemePredChirho {
            class_name_chirho: "Eq".to_string(),
            ty_chirho: ty_chirho.clone(),
            extra_tys_chirho: vec![],
        });
    }
    preds_chirho.push(SchemePredChirho {
        class_name_chirho: "Num".to_string(),
        ty_chirho: ty_chirho.clone(),
        extra_tys_chirho: vec![],
    });
    SchemeChirho {
        vars_chirho: vec![var_chirho],
        preds_chirho,
        ty_chirho: TyChirho::fun_chirho(ty_chirho.clone(), ty_chirho),
    }
}

#[test]
fn top_level_generalization_gives_method_and_literal_the_same_owner_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let ty_chirho = ctx_chirho.fresh_var_chirho();
    let mut origins_chirho =
        haskelujah_ast_chirho::provenance_chirho::OriginSupplyChirho::new_chirho();
    let literal_origin_chirho = origins_chirho.fresh_chirho();
    ctx_chirho.capture_occurrence_predicates_chirho(
        &dummy_name_chirho("+"),
        &[PredChirho::new_chirho("Num", ty_chirho.clone())],
    );
    ctx_chirho.capture_literal_evidence_chirho(
        &LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, Some(literal_origin_chirho)),
        "Num",
        &ty_chirho,
    );
    ctx_chirho.deferred_preds_chirho.push((
        PredChirho::new_chirho("Num", ty_chirho.clone()),
        SpanChirho::DUMMY_CHIRHO,
    ));
    let scheme_chirho = ctx_chirho.generalize_with_io_defaulting_chirho(
        &TyChirho::fun_chirho(ty_chirho.clone(), ty_chirho),
        false,
        GeneralizationScopeChirho::TopLevelChirho,
    );
    assert_eq!(scheme_chirho.preds_chirho.len(), 1);
    let method_chirho = ctx_chirho.finalize_occurrence_records_chirho(&SubstChirho::empty_chirho());
    let (_, literal_chirho) =
        ctx_chirho.finalize_literal_evidence_chirho(&SubstChirho::empty_chirho());
    assert_eq!(method_chirho.len(), 1);
    assert_eq!(method_chirho[0].ty_key_chirho, "$own:0");
    assert_eq!(
        literal_chirho[&literal_origin_chirho].ty_key_chirho,
        "$own:0"
    );
}

#[test]
fn two_num_dictionaries_keep_their_independent_parameter_indices_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let left_chirho = ctx_chirho.fresh_var_chirho();
    let right_chirho = ctx_chirho.fresh_var_chirho();
    for ty_chirho in [&right_chirho, &left_chirho] {
        ctx_chirho.deferred_preds_chirho.push((
            PredChirho::new_chirho("Num", ty_chirho.clone()),
            SpanChirho::DUMMY_CHIRHO,
        ));
    }
    let scheme_chirho = ctx_chirho.generalize_with_io_defaulting_chirho(
        &TyChirho::fun_n_chirho(
            [left_chirho.clone(), right_chirho.clone()],
            TyChirho::TupleChirho(vec![left_chirho.clone(), right_chirho.clone()]),
        ),
        false,
        GeneralizationScopeChirho::TopLevelChirho,
    );
    assert_eq!(scheme_chirho.preds_chirho.len(), 2);
    assert_eq!(
        ctx_chirho.inferred_dictionary_owner_key_chirho("Num", variable_chirho(&right_chirho)),
        Some("$own:0".to_string())
    );
    assert_eq!(
        ctx_chirho.inferred_dictionary_owner_key_chirho("Num", variable_chirho(&left_chirho)),
        Some("$own:1".to_string())
    );
}

#[test]
fn nested_generalization_never_claims_an_enclosing_dictionary_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let ty_chirho = ctx_chirho.fresh_var_chirho();
    ctx_chirho.deferred_preds_chirho.push((
        PredChirho::new_chirho("Num", ty_chirho.clone()),
        SpanChirho::DUMMY_CHIRHO,
    ));
    ctx_chirho.generalize_with_io_defaulting_chirho(
        &TyChirho::fun_chirho(ty_chirho.clone(), ty_chirho.clone()),
        false,
        GeneralizationScopeChirho::NestedChirho,
    );
    assert_eq!(
        ctx_chirho.inferred_dictionary_owner_key_chirho("Num", variable_chirho(&ty_chirho)),
        None
    );
}

#[test]
fn a_missing_or_conflicting_scc_owner_stays_unproved_chirho() {
    let var_chirho = TyVarChirho(12234);
    let mut shifted_chirho = InferCtxChirho::new_chirho();
    shifted_chirho.record_inferred_dictionary_owners_chirho(&num_scheme_chirho(var_chirho, false));
    shifted_chirho.record_inferred_dictionary_owners_chirho(&num_scheme_chirho(var_chirho, true));
    assert_eq!(
        shifted_chirho.inferred_dictionary_owner_key_chirho("Num", var_chirho),
        None
    );
    shifted_chirho.record_inferred_dictionary_owners_chirho(&num_scheme_chirho(var_chirho, false));
    assert_eq!(
        shifted_chirho.inferred_dictionary_owner_key_chirho("Num", var_chirho),
        None
    );

    let mut missing_chirho = InferCtxChirho::new_chirho();
    missing_chirho.record_inferred_dictionary_owners_chirho(&SchemeChirho::mono_chirho(
        TyChirho::VarChirho(var_chirho),
    ));
    missing_chirho.record_inferred_dictionary_owners_chirho(&num_scheme_chirho(var_chirho, false));
    assert_eq!(
        missing_chirho.inferred_dictionary_owner_key_chirho("Num", var_chirho),
        None
    );
}

#[test]
fn substituted_owners_intersect_and_concrete_targets_disappear_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let left_chirho = TyVarChirho(12234);
    let right_chirho = TyVarChirho(12235);
    ctx_chirho.record_inferred_dictionary_owners_chirho(&num_scheme_chirho(left_chirho, false));
    ctx_chirho.record_inferred_dictionary_owners_chirho(&num_scheme_chirho(right_chirho, true));
    let mut subst_chirho = SubstChirho::empty_chirho();
    subst_chirho.insert_chirho(left_chirho, TyChirho::VarChirho(right_chirho));
    ctx_chirho.apply_subst_all_chirho(&subst_chirho);
    assert_eq!(ctx_chirho.inferred_dictionary_owners_chirho.len(), 1);
    assert!(ctx_chirho.inferred_dictionary_owners_chirho[&right_chirho].is_empty());
    assert_eq!(
        ctx_chirho.inferred_dictionary_owner_key_chirho("Num", right_chirho),
        None
    );
    ctx_chirho.record_inferred_dictionary_owners_chirho(&num_scheme_chirho(right_chirho, true));
    assert_eq!(
        ctx_chirho.inferred_dictionary_owner_key_chirho("Num", right_chirho),
        None
    );
    subst_chirho.insert_chirho(right_chirho, TyChirho::ConChirho("Int".to_string()));
    ctx_chirho.apply_subst_all_chirho(&subst_chirho);
    assert!(ctx_chirho.inferred_dictionary_owners_chirho.is_empty());
}

#[test]
fn interleaved_duplicate_predicates_keep_one_stable_parameter_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let source_chirho = ctx_chirho.fresh_var_chirho();
    let result_chirho = ctx_chirho.fresh_var_chirho();
    for pred_chirho in [
        PredChirho::new_chirho("Num", result_chirho.clone()),
        PredChirho::new_chirho("Integral", source_chirho.clone()),
        PredChirho::new_chirho("Num", result_chirho.clone()),
    ] {
        ctx_chirho
            .deferred_preds_chirho
            .push((pred_chirho, SpanChirho::DUMMY_CHIRHO));
    }
    let scheme_chirho = ctx_chirho.generalize_with_io_defaulting_chirho(
        &TyChirho::fun_chirho(source_chirho, result_chirho.clone()),
        false,
        GeneralizationScopeChirho::TopLevelChirho,
    );
    assert_eq!(scheme_chirho.preds_chirho.len(), 2);
    assert_eq!(scheme_chirho.preds_chirho[0].class_name_chirho, "Num");
    assert_eq!(scheme_chirho.preds_chirho[1].class_name_chirho, "Integral");
    assert_eq!(
        ctx_chirho.inferred_dictionary_owner_key_chirho("Num", variable_chirho(&result_chirho)),
        Some("$own:0".to_string())
    );
}
