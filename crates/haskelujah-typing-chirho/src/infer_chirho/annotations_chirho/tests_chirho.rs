// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Captured-meta escape, lexical masking and bounded failure controls.

use super::*;
use crate::ty_chirho::TyVarChirho;
use std::collections::HashSet;

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
            vars_chirho: vec![TyVarChirho(2)],
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
        source_binders_chirho: Vec::new(),
        vars_chirho: vec![quantified_chirho],
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
        source_binders_chirho: Vec::new(),
        vars_chirho: vec![quantified_chirho],
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
    context_chirho.next_var_chirho = 9000;
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
    context_chirho.install_type_pattern_alias_chirho(
        "aChirho".into(),
        TyVarChirho(0),
        TyChirho::int_chirho(),
    );
    let mark_chirho = context_chirho.type_pattern_alias_undo_chirho.len();
    context_chirho.install_type_pattern_alias_chirho(
        "aChirho".into(),
        TyVarChirho(1),
        TyChirho::bool_chirho(),
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
        context_chirho.install_type_pattern_alias_chirho(
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
