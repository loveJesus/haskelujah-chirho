// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Captured-meta escape, lexical masking and bounded failure controls.

use super::*;
use crate::ty_chirho::TyVarChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use std::collections::HashSet;

#[test]
fn required_substitution_avoids_capture_in_the_remaining_full_telescope_chirho() {
    use crate::ty_chirho::{QuantifiedVariableChirho, QuantifiedVariablesChirho};
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    let mut context_chirho = InferCtxChirho::new_chirho();
    let consumed_chirho = TyVarChirho(200);
    let external_chirho = TyVarChirho(201);
    context_chirho
        .scoped_tyvars_chirho
        .insert("externalChirho".into(), external_chirho);
    let remaining_chirho = QuantifiedVariableChirho {
        variable_chirho: external_chirho,
        kind_chirho: Some(std::sync::Arc::new(TyChirho::VarChirho(consumed_chirho))),
        source_chirho: None,
        specificity_chirho: None,
    };
    let function_chirho = TyChirho::RequiredForallChirho {
        vars_chirho: vec![consumed_chirho].into(),
        body_chirho: Box::new(TyChirho::ForallChirho {
            vars_chirho: QuantifiedVariablesChirho::from_records_chirho(vec![remaining_chirho]),
            body_chirho: Box::new(TyChirho::TupleChirho(vec![
                TyChirho::VarChirho(consumed_chirho),
                TyChirho::VarChirho(external_chirho),
            ])),
        }),
    };
    let (_, result_chirho) = context_chirho
        .infer_required_type_application_chirho(
            &function_chirho,
            &ExprChirho::VarChirho(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                "externalChirho",
                SpanChirho::DUMMY_CHIRHO,
            ))),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
    let TyChirho::ForallChirho {
        vars_chirho,
        body_chirho,
    } = result_chirho
    else {
        panic!("remaining quantifier was erased");
    };
    assert_ne!(vars_chirho[0], external_chirho);
    assert_eq!(
        vars_chirho.record_chirho(0).unwrap().kind_chirho.as_deref(),
        Some(&TyChirho::VarChirho(external_chirho))
    );
    assert_eq!(
        *body_chirho,
        TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(external_chirho),
            TyChirho::VarChirho(vars_chirho[0])
        ])
    );
    assert!(
        !context_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
}

#[test]
fn annotation_escape_follows_committed_chains_and_qualified_contexts_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let skolem_chirho = "ownedRigidChirho".to_owned();
    context_chirho
        .equality_substitutions_chirho
        .insert_chirho(TyVarChirho(0), TyChirho::VarChirho(TyVarChirho(2)));
    context_chirho.equality_substitutions_chirho.insert_chirho(
        TyVarChirho(2),
        TyChirho::QualChirho {
            context_chirho: vec![TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("CheckedClassChirho".to_owned())),
                Box::new(TyChirho::ForallVarChirho(skolem_chirho.clone())),
            )],
            body_chirho: Box::new(TyChirho::bool_chirho()),
        },
    );
    context_chirho.env_chirho.bind_chirho(
        "capturedChirho".to_owned(),
        SchemeChirho::mono_chirho(TyChirho::VarChirho(TyVarChirho(0))),
    );
    let captured_chirho = context_chirho
        .capture_signature_metas_chirho(&SchemeChirho::mono_chirho(TyChirho::bool_chirho()));
    context_chirho.check_annotation_escape_chirho(
        captured_chirho,
        &HashSet::from([skolem_chirho]),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(context_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(!context_chirho.type_normalization_exhausted_chirho.get());
}

#[test]
fn annotation_escape_respects_nested_binders_and_foreign_rigids_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.equality_substitutions_chirho.insert_chirho(
        TyVarChirho(0),
        TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(2)].into(),
            body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(2))),
        },
    );
    context_chirho.equality_substitutions_chirho.insert_chirho(
        TyVarChirho(2),
        TyChirho::ForallVarChirho("ownedRigidChirho".to_owned()),
    );
    context_chirho.equality_substitutions_chirho.insert_chirho(
        TyVarChirho(3),
        TyChirho::ForallVarChirho("enclosingRigidChirho".to_owned()),
    );
    context_chirho.env_chirho.bind_chirho(
        "capturedChirho".to_owned(),
        SchemeChirho::mono_chirho(TyChirho::VarChirho(TyVarChirho(0))),
    );
    context_chirho.env_chirho.bind_chirho(
        "foreignChirho".to_owned(),
        SchemeChirho::mono_chirho(TyChirho::VarChirho(TyVarChirho(3))),
    );
    let captured_chirho = context_chirho
        .capture_signature_metas_chirho(&SchemeChirho::mono_chirho(TyChirho::bool_chirho()));
    context_chirho.check_annotation_escape_chirho(
        captured_chirho,
        &HashSet::from(["ownedRigidChirho".to_owned()]),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(!context_chirho.type_normalization_exhausted_chirho.get());
}

#[test]
fn annotation_escape_cycle_is_fatal_resource_failure_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho
        .equality_substitutions_chirho
        .insert_chirho(TyVarChirho(0), TyChirho::VarChirho(TyVarChirho(2)));
    context_chirho
        .equality_substitutions_chirho
        .insert_chirho(TyVarChirho(2), TyChirho::VarChirho(TyVarChirho(0)));
    context_chirho.env_chirho.bind_chirho(
        "capturedChirho".to_owned(),
        SchemeChirho::mono_chirho(TyChirho::VarChirho(TyVarChirho(0))),
    );
    let captured_chirho = context_chirho
        .capture_signature_metas_chirho(&SchemeChirho::mono_chirho(TyChirho::bool_chirho()));
    context_chirho.check_annotation_escape_chirho(
        captured_chirho,
        &HashSet::from(["ownedRigidChirho".to_owned()]),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(context_chirho.type_normalization_exhausted_chirho.get());
}

#[test]
fn nested_signature_check_reports_capture_before_restoring_skolems_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let outer_chirho = context_chirho.fresh_var_chirho();
    context_chirho.env_chirho.bind_chirho(
        "capturedChirho".to_owned(),
        SchemeChirho::mono_chirho(outer_chirho.clone()),
    );
    let TyChirho::VarChirho(quantified_chirho) = context_chirho.fresh_var_chirho() else {
        unreachable!()
    };
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![quantified_chirho].into(),
        preds_chirho: vec![],
        ty_chirho: TyChirho::VarChirho(quantified_chirho),
    };
    context_chirho.with_rigid_signature_chirho(
        &scheme_chirho,
        SpanChirho::DUMMY_CHIRHO,
        |context_chirho, expected_chirho| {
            let substitution_chirho = context_chirho
                .unify_normalized_chirho(&outer_chirho, expected_chirho, SpanChirho::DUMMY_CHIRHO)
                .unwrap();
            context_chirho.apply_subst_all_chirho(&substitution_chirho);
        },
    );
    assert!(context_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(context_chirho.skolem_bindings_chirho.is_empty_chirho());
}

#[test]
fn signature_equality_exports_safe_public_image_before_evidence_leaves_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let outer_chirho = context_chirho.fresh_var_chirho();
    context_chirho.env_chirho.bind_chirho(
        "capturedChirho".to_owned(),
        SchemeChirho::mono_chirho(outer_chirho.clone()),
    );
    let TyChirho::VarChirho(quantified_chirho) = context_chirho.fresh_var_chirho() else {
        unreachable!()
    };
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![quantified_chirho].into(),
        preds_chirho: vec![crate::ty_chirho::SchemePredChirho::legacy_chirho(
            crate::ty_chirho::SchemePredPartsChirho {
                kind_tys_chirho: vec![],
                class_name_chirho: "~".to_owned(),
                ty_chirho: TyChirho::VarChirho(quantified_chirho),
                extra_tys_chirho: vec![TyChirho::int_chirho()],
            },
        )],
        ty_chirho: TyChirho::VarChirho(quantified_chirho),
    };
    let (checked_chirho, public_chirho) = context_chirho.with_rigid_signature_chirho(
        &scheme_chirho,
        SpanChirho::DUMMY_CHIRHO,
        |context_chirho, expected_chirho| {
            let substitution_chirho = context_chirho
                .unify_ordinary_chirho(&outer_chirho, expected_chirho, SpanChirho::DUMMY_CHIRHO)
                .unwrap();
            context_chirho.apply_subst_all_chirho(&substitution_chirho);
            substitution_chirho
        },
    );
    let final_chirho =
        context_chirho.publish_signature_solution_chirho(checked_chirho, &public_chirho);
    assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(!context_chirho.type_normalization_exhausted_chirho.get());
    assert_eq!(
        final_chirho.apply_ty_chirho(&outer_chirho),
        TyChirho::int_chirho()
    );
    assert_eq!(
        context_chirho
            .env_chirho
            .lookup_chirho("capturedChirho")
            .unwrap()
            .ty_chirho,
        TyChirho::int_chirho()
    );
    assert!(context_chirho.skolem_bindings_chirho.is_empty_chirho());
}

#[test]
fn checked_annotation_above_import_placeholder_range_retains_its_type_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.elaboration_state_chirho.next_variable_chirho = 9000;
    let name_chirho = super::super::test_support_chirho::dummy_name_chirho;
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let expression_chirho = ExprChirho::TypeAppChirho {
        expr_chirho: Box::new(ExprChirho::AnnChirho {
            expr_chirho: Box::new(ExprChirho::VarChirho(name_chirho("undefined"))),
            ty_chirho: TypeChirho::ForallChirho {
                vars_chirho: vec![name_chirho("aChirho").into()],
                body_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
                span_chirho,
            },
            span_chirho,
        }),
        ty_chirho: TypeChirho::ConChirho(name_chirho("Int")),
        span_chirho,
    };
    let (_, inferred_chirho) = context_chirho.infer_expr_chirho(&expression_chirho);
    assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
    assert_eq!(inferred_chirho, TyChirho::int_chirho());
}

#[test]
fn type_pattern_alias_shadow_restores_exact_previous_image_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.install_type_pattern_alias_image_chirho(
        "aChirho".into(),
        TyVarChirho(0),
        crate::subst_chirho::SubstitutionImageChirho { image_chirho: TyChirho::int_chirho(), kind_chirho: Some(std::sync::Arc::new(crate::kind_chirho::elaboration_chirho::ElaborationStateChirho::type_universe_chirho())), source_chirho: None },
    );
    let mark_chirho = context_chirho.type_pattern_alias_undo_chirho.len();
    context_chirho.install_type_pattern_alias_image_chirho(
        "aChirho".into(),
        TyVarChirho(1),
        crate::subst_chirho::SubstitutionImageChirho { image_chirho: TyChirho::bool_chirho(), kind_chirho: Some(std::sync::Arc::new(crate::kind_chirho::elaboration_chirho::ElaborationStateChirho::type_universe_chirho())), source_chirho: None },
    );
    let aliases_chirho = context_chirho.type_pattern_aliases_since_chirho(mark_chirho);
    context_chirho.restore_type_pattern_aliases_chirho(mark_chirho);
    assert_eq!(
        context_chirho.scoped_tyvars_chirho["aChirho"],
        TyVarChirho(0)
    );
    assert!(
        context_chirho
            .skolem_bindings_chirho
            .lookup_chirho(&TyVarChirho(1))
            .is_none()
    );
    for (name_chirho, variable_chirho, image_chirho) in aliases_chirho {
        context_chirho.install_type_pattern_alias_image_chirho(
            name_chirho,
            variable_chirho,
            image_chirho,
        );
    }
    assert_eq!(
        context_chirho.scoped_tyvars_chirho["aChirho"],
        TyVarChirho(1)
    );
    assert_eq!(
        context_chirho
            .skolem_bindings_chirho
            .lookup_chirho(&TyVarChirho(1)),
        Some(&TyChirho::bool_chirho())
    );
    context_chirho.restore_type_pattern_aliases_chirho(mark_chirho);
    assert_eq!(
        context_chirho.scoped_tyvars_chirho["aChirho"],
        TyVarChirho(0)
    );
    assert_eq!(
        context_chirho
            .skolem_bindings_chirho
            .lookup_chirho(&TyVarChirho(0)),
        Some(&TyChirho::int_chirho())
    );
}

#[test]
fn constructor_type_pattern_wrapper_depth_is_non_deferrable_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let mut pattern_chirho = TypeChirho::WildcardChirho { span_chirho };
    for _index_chirho in 0..130 {
        pattern_chirho = TypeChirho::ParenChirho {
            inner_chirho: Box::new(pattern_chirho),
            span_chirho,
        };
    }
    context_chirho.bind_constructor_type_pattern_chirho(
        &pattern_chirho,
        &TyChirho::int_chirho(),
        None,
        span_chirho,
    );
    assert!(context_chirho.type_normalization_exhausted_chirho.get());
    assert!(context_chirho.type_pattern_alias_undo_chirho.is_empty());
    let result_chirho = context_chirho.finish_chirho();
    assert!(
        result_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
}

#[test]
fn constructor_prefix_resource_failure_remains_terminal_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let mut term_chirho = TyChirho::int_chirho();
    for _index_chirho in 0..130 {
        term_chirho = TyChirho::ForallChirho {
            vars_chirho: Vec::new().into(),
            body_chirho: Box::new(term_chirho),
        };
    }
    context_chirho.instantiate_constructor_pattern_with_variables_chirho(
        &SchemeChirho::mono_chirho(term_chirho),
    );
    assert!(
        context_chirho
            .elaboration_state_chirho
            .failure_chirho()
            .is_some()
    );
    context_chirho.instantiate_constructor_pattern_with_variables_chirho(
        &SchemeChirho::mono_chirho(TyChirho::int_chirho()),
    );
    assert!(
        context_chirho
            .elaboration_state_chirho
            .failure_chirho()
            .is_some()
    );
    assert!(
        context_chirho
            .finish_chirho()
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
}

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn variable_chirho(text_chirho: &str) -> ExprChirho {
    ExprChirho::VarChirho(name_chirho(text_chirho))
}

fn assert_closed_chirho(context_chirho: &InferCtxChirho) {
    assert!(context_chirho.skolem_bindings_chirho.is_empty_chirho());
    assert!(!context_chirho.type_normalization_exhausted_chirho.get());
}

fn check_section_with_an_enclosing_meta_chirho(right_section_chirho: bool, list_chirho: bool) {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let enclosing_chirho = context_chirho.fresh_var_chirho();
    let operator_chirho = if right_section_chirho {
        TyChirho::fun_chirho(
            TyChirho::bool_chirho(),
            TyChirho::fun_chirho(enclosing_chirho.clone(), enclosing_chirho.clone()),
        )
    } else {
        TyChirho::fun_chirho(
            enclosing_chirho.clone(),
            TyChirho::fun_chirho(TyChirho::bool_chirho(), enclosing_chirho.clone()),
        )
    };
    context_chirho.env_chirho.bind_chirho(
        "keepChirho".into(),
        SchemeChirho::mono_chirho(operator_chirho),
    );
    context_chirho.env_chirho.bind_chirho(
        "conditionChirho".into(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );
    let expected_chirho = TyChirho::fun_chirho(enclosing_chirho.clone(), enclosing_chirho);
    let expected_chirho = if list_chirho {
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("[]".into())),
            Box::new(expected_chirho),
        )
    } else {
        expected_chirho
    };
    let captured_chirho = context_chirho
        .capture_signature_metas_chirho(&SchemeChirho::mono_chirho(expected_chirho.clone()));
    assert!(captured_chirho.is_some());
    let expression_chirho = if right_section_chirho {
        ExprChirho::RightSectionChirho {
            arg_chirho: Box::new(variable_chirho("conditionChirho")),
            op_chirho: name_chirho("keepChirho"),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    } else {
        ExprChirho::LeftSectionChirho {
            op_chirho: name_chirho("keepChirho"),
            arg_chirho: Box::new(variable_chirho("conditionChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    };
    let expression_chirho = if list_chirho {
        ExprChirho::ListChirho {
            elements_chirho: vec![expression_chirho],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    } else {
        expression_chirho
    };
    let (substitution_chirho, actual_chirho) = context_chirho.infer_expr_against_expected_chirho(
        &expression_chirho,
        &expected_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert_eq!(
        actual_chirho,
        substitution_chirho.apply_ty_chirho(&expected_chirho)
    );
    context_chirho.check_annotation_escape_chirho(
        captured_chirho,
        &std::collections::HashSet::new(),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(
        !context_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
    assert_closed_chirho(&context_chirho);
}

#[test]
fn checking_a_left_section_keeps_enclosing_meta_escape_analysis_usable_chirho() {
    check_section_with_an_enclosing_meta_chirho(false, false);
}

#[test]
fn checking_a_right_section_keeps_enclosing_meta_escape_analysis_usable_chirho() {
    check_section_with_an_enclosing_meta_chirho(true, false);
}

#[test]
fn checking_a_list_section_keeps_enclosing_meta_escape_analysis_usable_chirho() {
    check_section_with_an_enclosing_meta_chirho(false, true);
}
