// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! General expected-sigma entry, rigidity and existing spine contracts.

use super::*;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn variable_chirho(text_chirho: &str) -> ExprChirho {
    ExprChirho::VarChirho(name_chirho(text_chirho))
}

fn sigma_chirho(
    context_chirho: &mut InferCtxChirho,
    body_chirho: impl FnOnce(TyChirho) -> TyChirho,
) -> TyChirho {
    let variable_chirho = context_chirho.fresh_var_chirho();
    let TyChirho::VarChirho(identifier_chirho) = variable_chirho.clone() else {
        unreachable!("fresh type variable")
    };
    TyChirho::ForallChirho {
        vars_chirho: vec![identifier_chirho].into(),
        body_chirho: Box::new(TyChirho::QualChirho {
            context_chirho: vec![TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Ord".into())),
                Box::new(variable_chirho.clone()),
            )],
            body_chirho: Box::new(body_chirho(variable_chirho)),
        }),
    }
}

fn assert_closed_chirho(context_chirho: &InferCtxChirho) {
    assert!(context_chirho.skolem_bindings_chirho.is_empty_chirho());
    assert!(!context_chirho.type_normalization_exhausted_chirho.get());
}

#[test]
fn an_empty_list_checks_at_a_qualified_sigma_and_keeps_its_prefix_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let expected_chirho = sigma_chirho(&mut context_chirho, |element_chirho| {
        TyChirho::ListChirho(Box::new(element_chirho))
    });
    let expression_chirho = ExprChirho::ListChirho {
        elements_chirho: Vec::new(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (_, actual_chirho) = context_chirho.infer_expr_against_expected_chirho(
        &expression_chirho,
        &expected_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(
        !context_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
    assert_eq!(actual_chirho, expected_chirho);
    assert_closed_chirho(&context_chirho);
}

#[test]
fn a_section_checks_at_a_qualified_sigma_without_specializing_it_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let operator_variable_chirho = context_chirho.fresh_var_chirho();
    let TyChirho::VarChirho(identifier_chirho) = operator_variable_chirho.clone() else {
        unreachable!("fresh type variable")
    };
    context_chirho.env_chirho.bind_chirho(
        "keepChirho".into(),
        SchemeChirho {
            vars_chirho: vec![identifier_chirho].into(),
            preds_chirho: Vec::new(),
            ty_chirho: TyChirho::fun_chirho(
                operator_variable_chirho.clone(),
                TyChirho::fun_chirho(TyChirho::bool_chirho(), operator_variable_chirho),
            ),
        },
    );
    context_chirho.env_chirho.bind_chirho(
        "conditionChirho".into(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );
    let expression_chirho = ExprChirho::LeftSectionChirho {
        op_chirho: name_chirho("keepChirho"),
        arg_chirho: Box::new(variable_chirho("conditionChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let expected_chirho = sigma_chirho(&mut context_chirho, |variable_chirho| {
        TyChirho::fun_chirho(variable_chirho.clone(), variable_chirho)
    });
    let (_, actual_chirho) = context_chirho.infer_expr_against_expected_chirho(
        &expression_chirho,
        &expected_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(
        !context_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
    assert_eq!(actual_chirho, expected_chirho);
    assert_closed_chirho(&context_chirho);
}

#[test]
fn a_captured_outer_meta_cannot_supply_a_qualified_universal_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let captured_chirho = context_chirho.fresh_var_chirho();
    context_chirho.env_chirho.bind_chirho(
        "capturedChirho".into(),
        SchemeChirho::mono_chirho(captured_chirho),
    );
    let expected_chirho = sigma_chirho(&mut context_chirho, |variable_chirho| variable_chirho);
    context_chirho.infer_expr_against_expected_chirho(
        &ExprChirho::ParenChirho {
            inner_chirho: Box::new(variable_chirho("capturedChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        &expected_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(context_chirho.diagnostics_chirho.has_errors_chirho());
    assert_closed_chirho(&context_chirho);
}

#[test]
fn a_monomorphic_body_cannot_choose_a_qualified_universal_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.env_chirho.bind_chirho(
        "conditionChirho".into(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );
    let expected_chirho = sigma_chirho(&mut context_chirho, |variable_chirho| variable_chirho);
    context_chirho.infer_expr_against_expected_chirho(
        &ExprChirho::ParenChirho {
            inner_chirho: Box::new(variable_chirho("conditionChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        &expected_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(context_chirho.diagnostics_chirho.has_errors_chirho());
    assert_closed_chirho(&context_chirho);
}

#[test]
fn an_explicit_type_lambda_still_consumes_its_own_binder_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let expected_chirho = sigma_chirho(&mut context_chirho, |element_chirho| {
        TyChirho::ListChirho(Box::new(element_chirho))
    });
    let expression_chirho = ExprChirho::TypeLamChirho {
        binder_chirho: name_chirho("elementChirho"),
        body_chirho: Box::new(ExprChirho::ListChirho {
            elements_chirho: Vec::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (_, actual_chirho) = context_chirho.infer_expr_against_expected_chirho(
        &expression_chirho,
        &expected_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(
        !context_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
    assert_eq!(actual_chirho, expected_chirho);
    assert!(
        !context_chirho
            .scoped_tyvars_chirho
            .contains_key("elementChirho")
    );
    assert_closed_chirho(&context_chirho);
}

fn check_section_with_an_enclosing_meta_chirho(right_section_chirho: bool) {
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
    check_section_with_an_enclosing_meta_chirho(false);
}

#[test]
fn checking_a_right_section_keeps_enclosing_meta_escape_analysis_usable_chirho() {
    check_section_with_an_enclosing_meta_chirho(true);
}
