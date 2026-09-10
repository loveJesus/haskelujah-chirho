// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn type_chirho() -> TypeChirho {
    TypeChirho::ConChirho(name_chirho("Type"))
}

fn arrow_chirho(left_chirho: TypeChirho, right_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::FunChirho {
        arg_chirho: Box::new(left_chirho),
        result_chirho: Box::new(right_chirho),
        mult_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn context_chirho() -> KindInferCtxChirho {
    KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho())
}

#[test]
fn invisible_head_binder_is_lexical_but_not_a_visible_kind_arrow_chirho() {
    let mut ctx_chirho = context_chirho();
    let mut invisible_chirho =
        TyVarChirho::annotated_chirho(name_chirho("kChirho"), AstKindChirho::StarChirho);
    invisible_chirho.visibility_chirho =
        haskelujah_ast_chirho::decl_chirho::TyVarVisibilityChirho::InvisibleChirho;
    let visible_chirho =
        TyVarChirho::annotated_chirho(name_chirho("aChirho"), AstKindChirho::StarChirho);
    ctx_chirho.infer_data_decl_kind_chirho(
        "BoxChirho",
        &[invisible_chirho, visible_chirho],
        None,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(ctx_chirho.diagnostics_chirho.is_empty_chirho());
    assert_eq!(
        ctx_chirho.env_chirho.lookup_chirho("kChirho"),
        Some(&KindChirho::StarChirho)
    );
    assert_eq!(
        ctx_chirho.env_chirho.lookup_chirho("BoxChirho"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::StarChirho
        ))
    );
}

#[test]
fn required_forall_kind_consumes_arguments_but_a_forall_type_does_not_chirho() {
    let binder_chirho =
        TyVarChirho::annotated_chirho(name_chirho("aChirho"), AstKindChirho::StarChirho);
    let invisible_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![binder_chirho.clone()],
        body_chirho: Box::new(type_chirho()),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let visible_chirho = TypeChirho::RequiredForallChirho {
        vars_chirho: vec![binder_chirho],
        body_chirho: Box::new(type_chirho()),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut ctx_chirho = context_chirho();
    assert_eq!(
        ctx_chirho.type_to_kind_chirho(&invisible_chirho),
        KindChirho::StarChirho
    );
    assert_eq!(
        ctx_chirho.type_to_kind_chirho(&visible_chirho),
        KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho)
    );
    let term_type_chirho = TypeChirho::RequiredForallChirho {
        vars_chirho: vec![TyVarChirho::annotated_chirho(
            name_chirho("aChirho"),
            AstKindChirho::StarChirho,
        )],
        body_chirho: Box::new(arrow_chirho(
            TypeChirho::VarChirho(name_chirho("aChirho")),
            TypeChirho::VarChirho(name_chirho("aChirho")),
        )),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert_eq!(
        ctx_chirho.infer_type_kind_chirho(&term_type_chirho),
        KindChirho::StarChirho
    );
    assert!(ctx_chirho.env_chirho.lookup_chirho("aChirho").is_none());
}

#[test]
fn result_kind_prefix_retains_shared_head_kind_identity_chirho() {
    let mut ctx_chirho = context_chirho();
    let params_chirho = ["aChirho", "bChirho"].map(|text_chirho| {
        TyVarChirho::annotated_chirho(
            name_chirho(text_chirho),
            AstKindChirho::VarChirho("kChirho".into()),
        )
    });
    let sig_chirho = DataKindSigChirho::ResultChirho(arrow_chirho(
        TypeChirho::VarChirho(name_chirho("kChirho")),
        type_chirho(),
    ));
    ctx_chirho.infer_data_decl_kind_chirho(
        "TestChirho",
        &params_chirho,
        Some(&sig_chirho),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(ctx_chirho.diagnostics_chirho.is_empty_chirho());
    let shared_chirho = KindChirho::VarChirho(ctx_chirho.kind_var_cache_chirho["kChirho"]);
    assert_eq!(
        ctx_chirho.env_chirho.lookup_chirho("TestChirho"),
        Some(&KindChirho::arrow_n_chirho(
            [shared_chirho.clone(), shared_chirho.clone(), shared_chirho],
            KindChirho::StarChirho
        ))
    );
}

#[test]
fn complete_signature_reconciles_head_kinds_before_constructors_chirho() {
    let mut ctx_chirho = context_chirho();
    let params_chirho = [TyVarChirho::annotated_chirho(
        name_chirho("aChirho"),
        AstKindChirho::StarChirho,
    )];
    let sig_chirho = DataKindSigChirho::StandaloneChirho {
        signature_chirho: arrow_chirho(arrow_chirho(type_chirho(), type_chirho()), type_chirho()),
        result_chirho: None,
    };
    ctx_chirho.infer_data_decl_kind_chirho(
        "BoxChirho",
        &params_chirho,
        Some(&sig_chirho),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert_eq!(ctx_chirho.diagnostics_chirho.error_count_chirho(), 1);
    assert!(
        ctx_chirho
            .diagnostics_chirho
            .to_string()
            .contains("data declaration signature")
    );
}

#[test]
fn complete_signature_does_not_double_count_head_parameters_chirho() {
    for result_chirho in [None, Some(type_chirho())] {
        let mut ctx_chirho = context_chirho();
        let sig_chirho = DataKindSigChirho::StandaloneChirho {
            signature_chirho: arrow_chirho(type_chirho(), type_chirho()),
            result_chirho,
        };
        let params_chirho = [TyVarChirho::plain_chirho(name_chirho("aChirho"))];
        ctx_chirho.infer_data_decl_kind_chirho(
            "BoxChirho",
            &params_chirho,
            Some(&sig_chirho),
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(ctx_chirho.diagnostics_chirho.is_empty_chirho());
        assert_eq!(
            ctx_chirho
                .subst_chirho
                .apply_chirho(ctx_chirho.env_chirho.lookup_chirho("aChirho").unwrap()),
            KindChirho::StarChirho
        );
        assert_eq!(
            ctx_chirho.env_chirho.lookup_chirho("BoxChirho"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::StarChirho
            ))
        );
    }
}

#[test]
fn missing_or_contradictory_result_tail_is_rejected_at_declaration_chirho() {
    for result_chirho in [None, Some(arrow_chirho(type_chirho(), type_chirho()))] {
        let mut ctx_chirho = context_chirho();
        // No head binders. A missing tail is Type; the written arrow is incompatible
        // with a complete Type kind. Neither error depends on constructor use.
        let signature_chirho = if result_chirho.is_none() {
            arrow_chirho(type_chirho(), type_chirho())
        } else {
            type_chirho()
        };
        let sig_chirho = DataKindSigChirho::StandaloneChirho {
            signature_chirho,
            result_chirho,
        };
        ctx_chirho.infer_data_decl_kind_chirho(
            "BoxChirho",
            &[],
            Some(&sig_chirho),
            SpanChirho::DUMMY_CHIRHO,
        );
        assert_eq!(ctx_chirho.diagnostics_chirho.error_count_chirho(), 1);
        assert!(
            ctx_chirho
                .diagnostics_chirho
                .to_string()
                .contains("data declaration signature")
        );
    }
}

#[test]
fn standalone_implicit_names_do_not_capture_enclosing_kind_identity_chirho() {
    let mut ctx_chirho = context_chirho();
    let enclosing_chirho = ctx_chirho.fresh_var_chirho();
    ctx_chirho
        .kind_var_cache_chirho
        .insert("kChirho".into(), enclosing_chirho);
    let sig_chirho = DataKindSigChirho::StandaloneChirho {
        signature_chirho: arrow_chirho(
            TypeChirho::VarChirho(name_chirho("kChirho")),
            type_chirho(),
        ),
        result_chirho: None,
    };
    // Leave the head polymorphic: constraining it to Type would require the
    // separate, still-unimplemented rigid-kind-signature contract.
    let params_chirho = [TyVarChirho::plain_chirho(name_chirho("aChirho"))];
    ctx_chirho.infer_data_decl_kind_chirho(
        "BoxChirho",
        &params_chirho,
        Some(&sig_chirho),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(ctx_chirho.diagnostics_chirho.is_empty_chirho());
    assert_eq!(
        ctx_chirho.kind_var_cache_chirho["kChirho"],
        enclosing_chirho
    );
    assert_eq!(
        ctx_chirho
            .subst_chirho
            .apply_chirho(&KindChirho::VarChirho(enclosing_chirho)),
        KindChirho::VarChirho(enclosing_chirho)
    );
}

#[test]
fn both_constraint_contracts_report_once_and_keep_local_shadow_boundary_chirho() {
    let sig_chirho = DataKindSigChirho::StandaloneChirho {
        signature_chirho: TypeChirho::ConChirho(name_chirho("Constraint")),
        result_chirho: Some(TypeChirho::ConChirho(name_chirho("Constraint"))),
    };
    let mut ctx_chirho = context_chirho();
    ctx_chirho.infer_data_decl_kind_chirho(
        "BoxChirho",
        &[],
        Some(&sig_chirho),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert_eq!(ctx_chirho.diagnostics_chirho.error_count_chirho(), 1);
    let mut shadow_ctx_chirho = context_chirho();
    shadow_ctx_chirho
        .local_kind_decl_names_chirho
        .insert("Constraint".into());
    shadow_ctx_chirho.infer_data_decl_kind_chirho(
        "BoxChirho",
        &[],
        Some(&sig_chirho),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(shadow_ctx_chirho.diagnostics_chirho.is_empty_chirho());
}
