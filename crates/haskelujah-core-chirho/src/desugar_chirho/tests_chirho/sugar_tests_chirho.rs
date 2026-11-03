// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Desugaring sugar: do-notation (plain and qualified), where-bindings, record
//! construction and update, and list comprehensions.

use super::*;

#[test]
fn desugar_do_notation_chirho() {
    use haskelujah_ast_chirho::expr_chirho::StmtChirho;

    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // do { x <- getLine; putStrLn x }
    let expr_chirho = ExprChirho::DoChirho {
        qualifier_chirho: None,
        stmts_chirho: vec![
            StmtChirho::BindChirho {
                bind_chirho: None,
                fail_chirho: None,
                pat_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                expr_chirho: ExprChirho::VarChirho(dummy_name_chirho("getLine")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            StmtChirho::expr_stmt_chirho(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("putStrLn"))),
                arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
        ],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // do-bind lowers to real monadic bind, not a let:
    // (>>=) getLine (\x -> putStrLn x)
    match core_chirho {
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            match fun_chirho.as_ref() {
                CoreExprChirho::AppChirho {
                    fun_chirho: bind_fun_chirho,
                    arg_chirho: get_line_arg_chirho,
                } => {
                    assert!(matches!(
                        bind_fun_chirho.as_ref(),
                        CoreExprChirho::VarChirho(_)
                    ));
                    assert!(matches!(
                        get_line_arg_chirho.as_ref(),
                        CoreExprChirho::VarChirho(_)
                    ));
                }
                other_chirho => panic!("expected bind application, got {other_chirho:?}"),
            }
            match arg_chirho.as_ref() {
                CoreExprChirho::LamChirho {
                    binder_chirho,
                    body_chirho,
                } => match body_chirho.as_ref() {
                    CoreExprChirho::AppChirho {
                        fun_chirho: put_str_ln_fun_chirho,
                        arg_chirho: put_str_ln_arg_chirho,
                    } => {
                        assert!(matches!(
                            put_str_ln_fun_chirho.as_ref(),
                            CoreExprChirho::VarChirho(_)
                        ));
                        assert_eq!(
                            put_str_ln_arg_chirho.as_ref(),
                            &CoreExprChirho::VarChirho(binder_chirho.id_chirho)
                        );
                    }
                    other_chirho => {
                        panic!("expected putStrLn application, got {other_chirho:?}")
                    }
                },
                other_chirho => {
                    panic!("expected bind continuation lambda, got {other_chirho:?}")
                }
            }
        }
        other_chirho => panic!("expected do-bind application, got {other_chirho:?}"),
    }
}

#[test]
fn desugar_qualified_do_uses_qualified_bind_chirho() {
    use haskelujah_ast_chirho::expr_chirho::StmtChirho;

    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    let expr_chirho = ExprChirho::DoChirho {
        qualifier_chirho: Some("FlowChirho".to_string()),
        stmts_chirho: vec![
            StmtChirho::BindChirho {
                bind_chirho: None,
                fail_chirho: None,
                pat_chirho: PatChirho::VarChirho(dummy_name_chirho("valueChirho")),
                expr_chirho: ExprChirho::VarChirho(dummy_name_chirho("actionChirho")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            StmtChirho::expr_stmt_chirho(ExprChirho::VarChirho(dummy_name_chirho("finishChirho"))),
        ],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    let CoreExprChirho::AppChirho { fun_chirho, .. } = core_chirho else {
        panic!("expected qualified bind application");
    };
    let CoreExprChirho::AppChirho {
        fun_chirho: bind_fun_chirho,
        ..
    } = fun_chirho.as_ref()
    else {
        panic!("expected qualified bind function application");
    };
    let CoreExprChirho::VarChirho(bind_id_chirho) = bind_fun_chirho.as_ref() else {
        panic!("expected qualified bind variable");
    };

    assert_eq!(
        ctx_chirho
            .names_chirho
            .get(bind_id_chirho)
            .map(String::as_str),
        Some("FlowChirho.>>=")
    );
}

#[test]
fn desugar_where_binds_chirho() {
    use haskelujah_ast_chirho::expr_chirho::LocalBindChirho;

    // f x = y where y = x
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("f"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho(
                    "y",
                ))),
                where_binds_chirho: vec![LocalBindChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("y"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
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

    let output_chirho = desugar_module_chirho(&module_chirho);
    let binding_chirho = &output_chirho.module_chirho.bindings_chirho[0];
    assert_eq!(binding_chirho.binder_chirho.name_chirho, "f");

    // The body should be Lam { body: Let { ... } }
    if let CoreExprChirho::LamChirho { body_chirho, .. } = &binding_chirho.rhs_chirho {
        assert!(
            matches!(**body_chirho, CoreExprChirho::LetChirho { .. }),
            "expected where clause to produce a Let, got {:?}",
            body_chirho
        );
    } else {
        panic!("expected LamChirho");
    }
}

#[test]
fn desugar_record_con_chirho() {
    use haskelujah_ast_chirho::expr_chirho::FieldAssignChirho;

    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // MkPoint { x = 1, y = 2 }
    let expr_chirho = ExprChirho::RecordConChirho {
        has_wildcard_chirho: false,
        con_chirho: dummy_name_chirho("MkPoint"),
        fields_chirho: vec![
            FieldAssignChirho {
                name_chirho: dummy_name_chirho("x"),
                value_chirho: ExprChirho::LitChirho(LitChirho::IntChirho(
                    1,
                    SpanChirho::DUMMY_CHIRHO,
                    None,
                )),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            FieldAssignChirho {
                name_chirho: dummy_name_chirho("y"),
                value_chirho: ExprChirho::LitChirho(LitChirho::IntChirho(
                    2,
                    SpanChirho::DUMMY_CHIRHO,
                    None,
                )),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // Should be ConApp("MkPoint", [1, 2])
    assert!(matches!(core_chirho, CoreExprChirho::ConAppChirho { .. }));
    if let CoreExprChirho::ConAppChirho {
        con_name_chirho,
        args_chirho,
    } = &core_chirho
    {
        assert_eq!(con_name_chirho, "MkPoint");
        assert_eq!(args_chirho.len(), 2);
        // Args are fromInteger-wrapped: App(fromInteger, Lit(2))
        assert!(matches!(
            &args_chirho[1],
            CoreExprChirho::AppChirho { arg_chirho, .. }
                if matches!(arg_chirho.as_ref(), CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)))
        ));
    }
}

#[test]
fn desugar_record_update_chirho() {
    use haskelujah_ast_chirho::expr_chirho::FieldAssignChirho;

    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // p { x = 10 }
    let expr_chirho = ExprChirho::RecordUpdateChirho {
        expr_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("p"))),
        fields_chirho: vec![FieldAssignChirho {
            name_chirho: dummy_name_chirho("x"),
            value_chirho: ExprChirho::LitChirho(LitChirho::IntChirho(
                10,
                SpanChirho::DUMMY_CHIRHO,
                None,
            )),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // Should produce an application involving $setField_x
    assert!(matches!(core_chirho, CoreExprChirho::AppChirho { .. }));
}

#[test]
fn desugar_list_comp_chirho() {
    use haskelujah_ast_chirho::expr_chirho::StmtChirho;

    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // [x | x <- xs]
    let expr_chirho = ExprChirho::ListCompChirho {
        body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
        quals_chirho: vec![StmtChirho::BindChirho {
            bind_chirho: None,
            fail_chirho: None,
            pat_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
            expr_chirho: ExprChirho::VarChirho(dummy_name_chirho("xs")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        parallel_quals_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // [x | x <- xs] → letrec go = \$xs -> case $xs of ... in go xs
    assert!(matches!(
        core_chirho,
        CoreExprChirho::LetChirho {
            rec_chirho: true,
            ..
        }
    ));
}

#[test]
fn desugar_list_comp_with_guard_chirho() {
    use haskelujah_ast_chirho::expr_chirho::StmtChirho;

    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // [x | x <- xs, even x]
    let expr_chirho = ExprChirho::ListCompChirho {
        body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
        quals_chirho: vec![
            StmtChirho::BindChirho {
                bind_chirho: None,
                fail_chirho: None,
                pat_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                expr_chirho: ExprChirho::VarChirho(dummy_name_chirho("xs")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            StmtChirho::expr_stmt_chirho(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("even"))),
                arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
        ],
        parallel_quals_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // [x | x <- xs, even x] → letrec go = \$xs -> case $xs of { ... } in go xs
    // The generator produces a LetChirho wrapping the recursive go function
    assert!(matches!(
        core_chirho,
        CoreExprChirho::LetChirho {
            rec_chirho: true,
            ..
        }
    ));
}
