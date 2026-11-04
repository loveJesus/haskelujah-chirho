// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Visible and required type applications, type lambdas and rank-n expected types.
//! Moved from `infer_chirho.rs` in row 661.

use super::test_support_chirho::{
    dummy_name_chirho, rank_n_identity_consumer_lambda_chirho, rank_n_identity_consumer_ty_chirho,
};
use super::*;
use haskelujah_ast_chirho::expr_chirho::{AltChirho, RhsChirho};

#[test]
fn infer_visible_type_application_specializes_named_scheme_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let a_chirho = TyVarChirho(9000);
    ctx_chirho.env_chirho.bind_chirho(
        "idTy".to_string(),
        SchemeChirho {
            vars_chirho: vec![a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(a_chirho),
                TyChirho::VarChirho(a_chirho),
            ),
        },
    );

    let expr_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::TypeAppChirho {
            expr_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("idTy"))),
            ty_chirho: TypeChirho::ConChirho(dummy_name_chirho("Int")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            1,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
    assert_eq!(final_ty_chirho, TyChirho::int_chirho());
}

#[test]
fn infer_visible_type_application_specializes_rank_n_parameter_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let rank_n_var_chirho = TyVarChirho(9003);
    ctx_chirho.env_chirho.bind_chirho(
        "rankNParameterChirho".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::ForallChirho {
                vars_chirho: vec![rank_n_var_chirho],
                body_chirho: Box::new(TyChirho::fun_chirho(
                    TyChirho::VarChirho(rank_n_var_chirho),
                    TyChirho::VarChirho(rank_n_var_chirho),
                )),
            },
        },
    );

    let expr_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::TypeAppChirho {
            expr_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                "rankNParameterChirho",
            ))),
            ty_chirho: TypeChirho::ConChirho(dummy_name_chirho("Int")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            1,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
    assert_eq!(final_ty_chirho, TyChirho::int_chirho());
}

#[test]
fn infer_required_type_argument_consumes_promoted_list_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let required_var_chirho = TyVarChirho(9004);
    ctx_chirho.env_chirho.bind_chirho(
        "requiredChirho".to_string(),
        SchemeChirho::mono_chirho(TyChirho::RequiredForallChirho {
            vars_chirho: vec![required_var_chirho],
            body_chirho: Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Proxy".to_string())),
                Box::new(TyChirho::VarChirho(required_var_chirho)),
            )),
        }),
    );
    let expr_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("requiredChirho"))),
        arg_chirho: Box::new(ExprChirho::ListChirho {
            elements_chirho: vec![
                ExprChirho::ConChirho(dummy_name_chirho("Num")),
                ExprChirho::ConChirho(dummy_name_chirho("Show")),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
    let expected_list_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(":".to_string())),
            Box::new(TyChirho::ConChirho("Num".to_string())),
        )),
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho(":".to_string())),
                Box::new(TyChirho::ConChirho("Show".to_string())),
            )),
            Box::new(TyChirho::ConChirho("'[]".to_string())),
        )),
    );
    assert_eq!(
        final_ty_chirho,
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Proxy".to_string())),
            Box::new(expected_list_chirho),
        )
    );
}

#[test]
fn infer_type_lambda_checks_against_outer_forall_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let expected_var_chirho = TyVarChirho(9005);
    let expected_ty_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![expected_var_chirho],
        body_chirho: Box::new(TyChirho::fun_chirho(
            TyChirho::VarChirho(expected_var_chirho),
            TyChirho::VarChirho(expected_var_chirho),
        )),
    };
    let value_name_chirho = dummy_name_chirho("valueChirho");
    let expr_chirho = ExprChirho::TypeLamChirho {
        binder_chirho: dummy_name_chirho("typeChirho"),
        body_chirho: Box::new(ExprChirho::LamChirho {
            pats_chirho: vec![PatChirho::VarChirho(value_name_chirho.clone())],
            body_chirho: Box::new(ExprChirho::VarChirho(value_name_chirho)),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (_, inferred_ty_chirho) = ctx_chirho.infer_expr_against_expected_chirho(
        &expr_chirho,
        &expected_ty_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert_eq!(inferred_ty_chirho, expected_ty_chirho);
    assert!(ctx_chirho.diagnostics_chirho.is_empty_chirho());
}

#[test]
fn infer_rank_n_expected_type_through_if_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let expected_ty_chirho = rank_n_identity_consumer_ty_chirho();
    let expr_chirho = ExprChirho::IfChirho {
        cond_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
        then_chirho: Box::new(rank_n_identity_consumer_lambda_chirho()),
        else_chirho: Box::new(rank_n_identity_consumer_lambda_chirho()),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_against_expected_chirho(
        &expr_chirho,
        &expected_ty_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );

    assert_eq!(
        subst_chirho.apply_ty_chirho(&inferred_ty_chirho),
        expected_ty_chirho
    );
    assert!(ctx_chirho.diagnostics_chirho.is_empty_chirho());
}

#[test]
fn infer_rank_n_expected_type_through_case_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let expected_ty_chirho = rank_n_identity_consumer_ty_chirho();
    let bool_alt_chirho = |constructor_chirho: &str| AltChirho {
        pat_chirho: PatChirho::ConChirho {
            con_chirho: dummy_name_chirho(constructor_chirho),
            args_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: RhsChirho::UnguardedChirho(rank_n_identity_consumer_lambda_chirho()),
        where_binds_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let expr_chirho = ExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
        alts_chirho: vec![bool_alt_chirho("True"), bool_alt_chirho("False")],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_against_expected_chirho(
        &expr_chirho,
        &expected_ty_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );

    assert_eq!(
        subst_chirho.apply_ty_chirho(&inferred_ty_chirho),
        expected_ty_chirho
    );
    assert!(ctx_chirho.diagnostics_chirho.is_empty_chirho());
}

#[test]
fn infer_visible_type_application_reuses_scoped_type_variable_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let scoped_m_chirho = TyVarChirho(9001);
    let n_chirho = TyVarChirho(9002);
    ctx_chirho
        .scoped_tyvars_chirho
        .insert("m".to_string(), scoped_m_chirho);
    ctx_chirho.env_chirho.bind_chirho(
        "proof".to_string(),
        SchemeChirho {
            vars_chirho: vec![n_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Dict".to_string())),
                Box::new(TyChirho::VarChirho(n_chirho)),
            ),
        },
    );

    let expr_chirho = ExprChirho::TypeAppChirho {
        expr_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("proof"))),
        ty_chirho: TypeChirho::VarChirho(dummy_name_chirho("m")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (_subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    assert_eq!(
        ty_chirho,
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Dict".to_string())),
            Box::new(TyChirho::VarChirho(scoped_m_chirho))
        )
    );
}

#[test]
fn infer_visible_type_application_wildcard_uses_signature_equality_chirho() {
    use haskelujah_ast_chirho::ty_chirho::ConstraintChirho as AstConstraintChirho;
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let var_ast_chirho = |name_chirho: &str| TypeChirho::VarChirho(dummy_name_chirho(name_chirho));
    let con_ast_chirho = |name_chirho: &str| TypeChirho::ConChirho(dummy_name_chirho(name_chirho));
    let app_ast_chirho = |fun_chirho: TypeChirho, arg_chirho: TypeChirho| TypeChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let promoted_ty_list_ast_chirho = || TypeChirho::PromotedListChirho {
        elements_chirho: vec![con_ast_chirho("Ty")],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let expr_ast_chirho = |t_chirho: TypeChirho, lrs_chirho: TypeChirho| {
        app_ast_chirho(app_ast_chirho(con_ast_chirho("Expr"), t_chirho), lrs_chirho)
    };
    let kt_ast_chirho = |lrs_chirho: TypeChirho| {
        app_ast_chirho(
            app_ast_chirho(
                app_ast_chirho(con_ast_chirho("KT"), con_ast_chirho("A")),
                lrs_chirho,
            ),
            con_ast_chirho("IO"),
        )
    };

    let sig_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![
            dummy_name_chirho("m").into(),
            dummy_name_chirho("t").into(),
            dummy_name_chirho("lrs").into(),
            dummy_name_chirho("bind").into(),
        ],
        body_chirho: Box::new(TypeChirho::QualChirho {
            context_chirho: vec![AstConstraintChirho::ClassChirho {
                kind_args_chirho: Vec::new(),
                class_chirho: dummy_name_chirho("~"),
                args_chirho: vec![
                    var_ast_chirho("bind"),
                    expr_ast_chirho(var_ast_chirho("t"), var_ast_chirho("lrs")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            body_chirho: Box::new(app_ast_chirho(var_ast_chirho("m"), var_ast_chirho("bind"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let scheme_chirho = ctx_chirho.ast_type_to_scheme_chirho(&sig_chirho);
    assert_eq!(scheme_chirho.vars_chirho.len(), 4);
    let m_var_chirho = scheme_chirho.vars_chirho[0];
    let t_var_chirho = scheme_chirho.vars_chirho[1];
    let lrs_var_chirho = scheme_chirho.vars_chirho[2];
    let bind_var_chirho = scheme_chirho.vars_chirho[3];
    let expected_scheme_body_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::VarChirho(m_var_chirho)),
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Expr".to_string())),
                Box::new(TyChirho::VarChirho(t_var_chirho)),
            )),
            Box::new(TyChirho::VarChirho(lrs_var_chirho)),
        )),
    );
    assert_eq!(scheme_chirho.ty_chirho, expected_scheme_body_chirho);
    assert!(
        !scheme_chirho
            .ty_chirho
            .free_vars_chirho()
            .contains(&bind_var_chirho),
        "equality-constrained result should rewrite bind out of the scheme body"
    );

    ctx_chirho
        .env_chirho
        .bind_chirho("test_gr".to_string(), scheme_chirho);
    let type_app_chirho =
        |expr_chirho: ExprChirho, ty_chirho: TypeChirho| ExprChirho::TypeAppChirho {
            expr_chirho: Box::new(expr_chirho),
            ty_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
    let expr_net_ty_list_ast_chirho =
        expr_ast_chirho(con_ast_chirho("Net"), promoted_ty_list_ast_chirho());
    let test_gr_expr_chirho = type_app_chirho(
        type_app_chirho(
            type_app_chirho(
                type_app_chirho(
                    ExprChirho::VarChirho(dummy_name_chirho("test_gr")),
                    kt_ast_chirho(promoted_ty_list_ast_chirho()),
                ),
                TypeChirho::WildcardChirho {
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ),
            promoted_ty_list_ast_chirho(),
        ),
        expr_net_ty_list_ast_chirho,
    );
    let ty_app_chirho = |fun_chirho: TyChirho, arg_chirho: TyChirho| {
        TyChirho::AppChirho(Box::new(fun_chirho), Box::new(arg_chirho))
    };
    let promoted_ty_list_ty_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(":".to_string())),
            Box::new(TyChirho::ConChirho("Ty".to_string())),
        )),
        Box::new(TyChirho::ConChirho("'[]".to_string())),
    );
    let expr_net_ty_list_ty_chirho = ty_app_chirho(
        ty_app_chirho(
            TyChirho::ConChirho("Expr".to_string()),
            TyChirho::ConChirho("Net".to_string()),
        ),
        promoted_ty_list_ty_chirho.clone(),
    );
    let expected_ty_chirho = ty_app_chirho(
        ty_app_chirho(
            ty_app_chirho(
                ty_app_chirho(
                    TyChirho::ConChirho("KT".to_string()),
                    TyChirho::ConChirho("A".to_string()),
                ),
                promoted_ty_list_ty_chirho,
            ),
            TyChirho::ConChirho("IO".to_string()),
        ),
        expr_net_ty_list_ty_chirho,
    );

    let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_against_expected_chirho(
        &test_gr_expr_chirho,
        &expected_ty_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert_eq!(
        subst_chirho.apply_ty_chirho(&inferred_ty_chirho),
        expected_ty_chirho
    );
}
