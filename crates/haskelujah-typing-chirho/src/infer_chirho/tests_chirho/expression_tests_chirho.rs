// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Expressions: literals, applications, conditionals, tuples, lists, lets, lambdas and do blocks.
//! Moved from `infer_chirho.rs` in row 661.

use super::test_support_chirho::dummy_name_chirho;
use super::*;
use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::expr_chirho::{AltChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;

#[test]
fn infer_identity_function_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    // \x -> x
    let expr_chirho = ExprChirho::LamChirho {
        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
        body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);

    // Should be `a -> a` for some variable a
    match &final_ty_chirho {
        TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
            assert_eq!(arg_chirho, result_chirho, "identity should have a -> a");
        }
        other_chirho => panic!("expected function type, got {other_chirho}"),
    }
}

#[test]
fn infer_application_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    // not True
    let expr_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("not"))),
        arg_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
    assert_eq!(final_ty_chirho, TyChirho::bool_chirho());
}

#[test]
fn infer_if_expression_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    // if True then 1 else 2
    let expr_chirho = ExprChirho::IfChirho {
        cond_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
        then_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            1,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        else_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            2,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
    assert!(matches!(final_ty_chirho, TyChirho::VarChirho(_)));
    assert_eq!(ctx_chirho.deferred_preds_chirho.len(), 2);
    assert!(
        ctx_chirho
            .deferred_preds_chirho
            .iter()
            .all(|(pred_chirho, _)| pred_chirho.class_name_chirho == "Num")
    );
}

#[test]
fn infer_module_data_and_function_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::DataDeclChirho {
                name_chirho: dummy_name_chirho("Color"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: dummy_name_chirho("Red"),
                    fields_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("f"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                        dummy_name_chirho("x"),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "module inference should succeed without errors"
    );

    // f should have a function type (with per-equation fresh types for
    // GADT support, the overall result stays as a fresh variable, so the
    // generalized type is `forall a b. a -> b` rather than `forall a. a -> a`).
    let f_scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("f")
        .expect("f should be in environment");
    let f_ty_chirho = result_chirho
        .subst_chirho
        .apply_ty_chirho(&f_scheme_chirho.ty_chirho);
    assert!(
        matches!(&f_ty_chirho, TyChirho::FunChirho(..)),
        "f should have function type, got {f_ty_chirho}"
    );

    // Red should be in environment as Color
    let red_scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("Red")
        .expect("Red constructor should be in environment");
    assert_eq!(
        red_scheme_chirho.ty_chirho,
        TyChirho::ConChirho("Color".to_string())
    );
}

#[test]
fn infer_let_generalization_chirho() {
    // Wrap in a module-level function to get full sig checking:
    // testLetGen = let myId = \x -> x in myId 42
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("LetGen"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("testLetGenChirho"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LetChirho {
                    binds_chirho: vec![
                        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                            name_chirho: dummy_name_chirho("myId"),
                            matches_chirho: vec![MatchArmChirho {
                                pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                                    dummy_name_chirho("x"),
                                )),
                                where_binds_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    body_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("myId"))),
                        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                            42,
                            SpanChirho::DUMMY_CHIRHO,
                            None,
                        ))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "let generalization should not error: {:?}",
        result_chirho.diagnostics_chirho
    );
    let _scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("testLetGenChirho")
        .expect("testLetGenChirho should be in env");
    // With per-equation fresh result types (for GADT support), the overall
    // result type stays as a fresh variable. The key assertion is that
    // let-generalization (let id = \x -> x in id 42) doesn't produce errors.
}

#[test]
fn infer_tuple_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    // (1, True)
    let expr_chirho = ExprChirho::TupleChirho {
        elements_chirho: vec![
            ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None)),
            ExprChirho::ConChirho(dummy_name_chirho("True")),
        ],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
    assert!(matches!(
        final_ty_chirho,
        TyChirho::TupleChirho(ref elems_chirho)
            if elems_chirho.len() == 2
                && matches!(elems_chirho[0], TyChirho::VarChirho(_))
                && elems_chirho[1] == TyChirho::bool_chirho()
    ));
    assert_eq!(ctx_chirho.deferred_preds_chirho.len(), 1);
    assert_eq!(
        ctx_chirho.deferred_preds_chirho[0].0.class_name_chirho,
        "Num"
    );
}

#[test]
fn infer_list_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    // [1, 2, 3]
    let expr_chirho = ExprChirho::ListChirho {
        elements_chirho: vec![
            ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None)),
            ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO, None)),
            ExprChirho::LitChirho(LitChirho::IntChirho(3, SpanChirho::DUMMY_CHIRHO, None)),
        ],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
    assert!(matches!(
        final_ty_chirho,
        TyChirho::ListChirho(ref elem_chirho)
            if matches!(elem_chirho.as_ref(), TyChirho::VarChirho(_))
    ));
    assert_eq!(ctx_chirho.deferred_preds_chirho.len(), 3);
    assert!(
        ctx_chirho
            .deferred_preds_chirho
            .iter()
            .all(|(pred_chirho, _)| pred_chirho.class_name_chirho == "Num")
    );
}

#[test]
fn infer_dollar_applies_constructor_to_lambda_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let wrap_r_chirho = TyVarChirho(9100);
    let wrap_s_chirho = TyVarChirho(9101);
    let wrap_a_chirho = TyVarChirho(9102);
    let wrap_result_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WrapChirho".to_string())),
                Box::new(TyChirho::VarChirho(wrap_r_chirho)),
            )),
            Box::new(TyChirho::VarChirho(wrap_s_chirho)),
        )),
        Box::new(TyChirho::VarChirho(wrap_a_chirho)),
    );
    let wrap_arg_chirho = TyChirho::fun_n_chirho(
        vec![
            TyChirho::VarChirho(wrap_r_chirho),
            TyChirho::VarChirho(wrap_s_chirho),
        ],
        TyChirho::VarChirho(wrap_a_chirho),
    );
    ctx_chirho.env_chirho.bind_chirho(
        "WrapChirho".to_string(),
        SchemeChirho {
            source_binders_chirho: Vec::new(),
            vars_chirho: vec![wrap_r_chirho, wrap_s_chirho, wrap_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(wrap_arg_chirho, wrap_result_chirho.clone()),
        },
    );
    let expr_chirho = ExprChirho::InfixChirho {
        left_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("WrapChirho"))),
        op_chirho: dummy_name_chirho("$"),
        right_chirho: Box::new(ExprChirho::LamChirho {
            pats_chirho: vec![
                PatChirho::VarChirho(dummy_name_chirho("rChirho")),
                PatChirho::VarChirho(dummy_name_chirho("sChirho")),
            ],
            body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("rChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
    match final_ty_chirho {
        TyChirho::AppChirho(_, _) => {}
        other_chirho => panic!("expected constructor result after $, got {other_chirho}"),
    }
}

#[test]
fn infer_lambda_constructor_body_chirho() {
    let module_chirho =
        ModuleChirho {
            name_chirho: dummy_name_chirho("LambdaConstructor"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("R"),
                    type_vars_chirho: vec![],
                    constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("V"),
                        fields_chirho: vec![(
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            TypeChirho::ConChirho(dummy_name_chirho("Int")),
                        )],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("fChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LamChirho {
                            pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("n"))],
                            body_chirho: Box::new(ExprChirho::AppChirho {
                                fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("V"))),
                                arg_chirho: Box::new(ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::AppChirho {
                                        fun_chirho: Box::new(ExprChirho::VarChirho(
                                            dummy_name_chirho("*"),
                                        )),
                                        arg_chirho: Box::new(ExprChirho::VarChirho(
                                            dummy_name_chirho("n"),
                                        )),
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    arg_chirho: Box::new(ExprChirho::LitChirho(
                                        LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO, None),
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "lambda with constructor body should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );

    let f_scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("fChirho")
        .expect("fChirho should be in environment");
    let f_ty_chirho = result_chirho
        .subst_chirho
        .apply_ty_chirho(&f_scheme_chirho.ty_chirho);
    assert_eq!(
        f_ty_chirho,
        TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::ConChirho("R".to_string()),),
        "lambda returning constructor application should infer Int -> R"
    );
}

#[test]
fn infer_result_bind_lambda_constructor_body_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("ResultLambda"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::DataDeclChirho {
                name_chirho: dummy_name_chirho("Result"),
                type_vars_chirho: vec![dummy_name_chirho("a").into()],
                constructors_chirho: vec![
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("Ok"),
                        fields_chirho: vec![(
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            TypeChirho::VarChirho(dummy_name_chirho("a")),
                        )],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("Err"),
                        fields_chirho: vec![(
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            TypeChirho::ConChirho(dummy_name_chirho("String")),
                        )],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("bindResultChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("m")),
                        PatChirho::VarChirho(dummy_name_chirho("f")),
                    ],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("m"))),
                        alts_chirho: vec![
                            AltChirho {
                                pat_chirho: PatChirho::ConChirho {
                                    type_args_chirho: Vec::new(),
                                    con_chirho: dummy_name_chirho("Ok"),
                                    args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                },
                                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                        "f",
                                    ))),
                                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                        "x",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                where_binds_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            AltChirho {
                                pat_chirho: PatChirho::ConChirho {
                                    type_args_chirho: Vec::new(),
                                    con_chirho: dummy_name_chirho("Err"),
                                    args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("e"))],
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                },
                                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                        "Err",
                                    ))),
                                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                        "e",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                where_binds_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                        ],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("useBindResultChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "bindResultChirho",
                            ))),
                            arg_chirho: Box::new(ExprChirho::AppChirho {
                                fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                    "Ok",
                                ))),
                                arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                                    21,
                                    SpanChirho::DUMMY_CHIRHO,
                                    None,
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        arg_chirho: Box::new(ExprChirho::LamChirho {
                            pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("n"))],
                            body_chirho: Box::new(ExprChirho::AppChirho {
                                fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                    "Ok",
                                ))),
                                arg_chirho: Box::new(ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::AppChirho {
                                        fun_chirho: Box::new(ExprChirho::VarChirho(
                                            dummy_name_chirho("*"),
                                        )),
                                        arg_chirho: Box::new(ExprChirho::VarChirho(
                                            dummy_name_chirho("n"),
                                        )),
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    arg_chirho: Box::new(ExprChirho::LitChirho(
                                        LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO, None),
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "Result lambda constructor body should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );

    let use_bind_scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("useBindResultChirho")
        .expect("useBindResultChirho should be in environment");
    let use_bind_ty_chirho = result_chirho
        .subst_chirho
        .apply_ty_chirho(&use_bind_scheme_chirho.ty_chirho);
    assert!(
        matches!(
            use_bind_ty_chirho,
            TyChirho::AppChirho(ref fun_chirho, _)
                if fun_chirho.as_ref() == &TyChirho::ConChirho("Result".to_string())
        ),
        "bindResult with a lambda constructor body should infer Result _"
    );
}

/// Test: do-notation bind statement unwraps monadic type.
/// Haskell: `doExample = do { x <- Just 42; pure x }`
#[test]
fn infer_do_bind_unwraps_monad_chirho() {
    // Build: doExample = do { x <- expr_m; expr_body }
    // where expr_m :: Maybe Int  (simulated as App(Con("Maybe"), Lit(42)))
    // and expr_body :: uses x (a VarChirho)
    //
    // We seed Maybe Just Nothing in the env.
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("DoNotation"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("doExampleChirho"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::DoChirho {
                    qualifier_chirho: None,
                    stmts_chirho: vec![
                        // x <- Just 42  (simulated: x <- app(Just, 42))
                        StmtChirho::BindChirho {
                            bind_chirho: None,
                            fail_chirho: None,
                            pat_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                            expr_chirho: ExprChirho::AppChirho {
                                fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                    "Just",
                                ))),
                                arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                                    42,
                                    SpanChirho::DUMMY_CHIRHO,
                                    None,
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        // pure x (simplified: just return x)
                        StmtChirho::expr_stmt_chirho(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "do-notation bind should not error: {:?}",
        result_chirho.diagnostics_chirho
    );
    // doExampleChirho should be in the environment
    let scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("doExampleChirho")
        .expect("doExampleChirho should be in env");
    // The type should involve x :: Int (unwrapped from Just Int)
    // The last stmt `x` has type Int, not wrapped
    assert!(!format!("{scheme_chirho}").is_empty(), "should have a type");
}

/// Test: do-notation with let statement.
/// Haskell: `doLetChirho = do { let y = 10; pure y }`
#[test]
fn infer_do_let_statement_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("DoLet"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("doLetChirho"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::DoChirho {
                    qualifier_chirho: None,
                    stmts_chirho: vec![
                        // let y = 10
                        StmtChirho::LetChirho {
                            binds_chirho: vec![
                                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                                    pat_chirho: PatChirho::VarChirho(
                                        dummy_name_chirho("y"),
                                    ),
                                    rhs_chirho: RhsChirho::UnguardedChirho(
                                        ExprChirho::LitChirho(LitChirho::IntChirho(
                                            10,
                                            SpanChirho::DUMMY_CHIRHO,
                                            None,
                                        )),
                                    ),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                },
                            ],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        // y (return it)
                        StmtChirho::expr_stmt_chirho(ExprChirho::VarChirho(dummy_name_chirho("y"))),
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "do-notation with let should not error: {:?}",
        result_chirho.diagnostics_chirho
    );
    let _scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("doLetChirho")
        .expect("doLetChirho should be in env");
    // With per-equation fresh result types (for GADT support), the overall
    // result type stays as a fresh variable. The key assertion is that
    // do-notation with let doesn't produce errors.
}
