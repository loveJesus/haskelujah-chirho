// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Case and function-pattern binders: what each alternative's right-hand side binds.

use super::*;

// -------------------------------------------------------------------
// Constructor field binding tests
// -------------------------------------------------------------------

#[test]
fn desugar_case_con_binders_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // case x of { Just a -> a; Nothing -> 0 }
    let expr_chirho = ExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
        alts_chirho: vec![
            haskelujah_ast_chirho::expr_chirho::AltChirho {
                pat_chirho: PatChirho::ConChirho {
                    con_chirho: dummy_name_chirho("Just"),
                    args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("a"))],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho(
                    "a",
                ))),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            haskelujah_ast_chirho::expr_chirho::AltChirho {
                pat_chirho: PatChirho::ConChirho {
                    con_chirho: dummy_name_chirho("Nothing"),
                    args_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                    LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO, None),
                )),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

    if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
        let alts_chirho = explicit_alts_chirho(alts_chirho);
        assert_eq!(alts_chirho.len(), 2);
        // Just a → binders should have 1 binder named "a"
        assert_eq!(alts_chirho[0].binders_chirho.len(), 1);
        assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "a");
        assert!(matches!(
            alts_chirho[0].con_chirho,
            AltConChirho::DataConChirho(ref s_chirho) if s_chirho == "Just"
        ));
        // Nothing → no binders
        assert_eq!(alts_chirho[1].binders_chirho.len(), 0);
        // The RHS of the Just branch should reference "a"'s id
        let a_id_chirho = alts_chirho[0].binders_chirho[0].id_chirho;
        assert!(matches!(
            alts_chirho[0].rhs_chirho,
            CoreExprChirho::VarChirho(id_chirho) if id_chirho == a_id_chirho
        ));
    } else {
        panic!("expected CaseChirho, got {:?}", core_chirho);
    }
}

#[test]
fn desugar_case_multi_field_binders_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // case p of { MkPoint x y -> x }
    let expr_chirho = ExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("p"))),
        alts_chirho: vec![haskelujah_ast_chirho::expr_chirho::AltChirho {
            pat_chirho: PatChirho::ConChirho {
                con_chirho: dummy_name_chirho("MkPoint"),
                args_chirho: vec![
                    PatChirho::VarChirho(dummy_name_chirho("x")),
                    PatChirho::VarChirho(dummy_name_chirho("y")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

    if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
        let alts_chirho = explicit_alts_chirho(alts_chirho);
        assert_eq!(alts_chirho.len(), 1);
        assert_eq!(alts_chirho[0].binders_chirho.len(), 2);
        assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "x");
        assert_eq!(alts_chirho[0].binders_chirho[1].name_chirho, "y");
        // RHS should reference x's binder
        let x_id_chirho = alts_chirho[0].binders_chirho[0].id_chirho;
        assert!(matches!(
            alts_chirho[0].rhs_chirho,
            CoreExprChirho::VarChirho(id_chirho) if id_chirho == x_id_chirho
        ));
    } else {
        panic!("expected CaseChirho");
    }
}

#[test]
fn desugar_case_infix_con_binders_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // case xs of { x : rest -> x }
    let expr_chirho = ExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("xs"))),
        alts_chirho: vec![haskelujah_ast_chirho::expr_chirho::AltChirho {
            pat_chirho: PatChirho::InfixConChirho {
                left_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("x"))),
                op_chirho: dummy_name_chirho(":"),
                right_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("rest"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

    if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
        let alts_chirho = explicit_alts_chirho(alts_chirho);
        assert_eq!(alts_chirho.len(), 1);
        assert!(matches!(
            alts_chirho[0].con_chirho,
            AltConChirho::DataConChirho(ref s_chirho) if s_chirho == ":"
        ));
        assert_eq!(alts_chirho[0].binders_chirho.len(), 2);
        assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "x");
        assert_eq!(alts_chirho[0].binders_chirho[1].name_chirho, "rest");
    } else {
        panic!("expected CaseChirho");
    }
}

#[test]
fn desugar_case_tuple_binders_chirho() {
    use haskelujah_ast_chirho::expr_chirho::AltChirho as AstAltChirho;
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // case t of { (a, b) -> a }
    let expr_chirho = ExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("t"))),
        alts_chirho: vec![AstAltChirho {
            pat_chirho: PatChirho::TupleChirho {
                elements_chirho: vec![
                    PatChirho::VarChirho(dummy_name_chirho("a")),
                    PatChirho::VarChirho(dummy_name_chirho("b")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho("a"))),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

    if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
        let alts_chirho = explicit_alts_chirho(alts_chirho);
        assert_eq!(alts_chirho.len(), 1);
        assert!(matches!(
            alts_chirho[0].con_chirho,
            AltConChirho::DataConChirho(ref s_chirho) if s_chirho == "$tuple2"
        ));
        assert_eq!(alts_chirho[0].binders_chirho.len(), 2);
        assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "a");
        assert_eq!(alts_chirho[0].binders_chirho[1].name_chirho, "b");
    } else {
        panic!("expected CaseChirho");
    }
}

#[test]
fn desugar_case_record_binders_chirho() {
    use haskelujah_ast_chirho::pat_chirho::PatFieldChirho;
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // case p of { MkPoint { xCoord = a, yCoord = b } -> a }
    let expr_chirho = ExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("p"))),
        alts_chirho: vec![haskelujah_ast_chirho::expr_chirho::AltChirho {
            pat_chirho: PatChirho::RecordChirho {
                has_wildcard_chirho: false,
                con_chirho: dummy_name_chirho("MkPoint"),
                fields_chirho: vec![
                    PatFieldChirho {
                        name_chirho: dummy_name_chirho("xCoord"),
                        pattern_chirho: PatChirho::VarChirho(dummy_name_chirho("a")),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    PatFieldChirho {
                        name_chirho: dummy_name_chirho("yCoord"),
                        pattern_chirho: PatChirho::VarChirho(dummy_name_chirho("b")),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho("a"))),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

    if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
        let alts_chirho = explicit_alts_chirho(alts_chirho);
        assert_eq!(alts_chirho.len(), 1);
        assert!(matches!(
            alts_chirho[0].con_chirho,
            AltConChirho::DataConChirho(ref s_chirho) if s_chirho == "MkPoint"
        ));
        assert_eq!(alts_chirho[0].binders_chirho.len(), 2);
        assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "a");
        assert_eq!(alts_chirho[0].binders_chirho[1].name_chirho, "b");
    } else {
        panic!("expected CaseChirho");
    }
}

#[test]
fn desugar_case_wildcard_binders_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // case x of { Just _ -> 1; Nothing -> 0 }
    let expr_chirho = ExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
        alts_chirho: vec![
            haskelujah_ast_chirho::expr_chirho::AltChirho {
                pat_chirho: PatChirho::ConChirho {
                    con_chirho: dummy_name_chirho("Just"),
                    args_chirho: vec![PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                    LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None),
                )),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            haskelujah_ast_chirho::expr_chirho::AltChirho {
                pat_chirho: PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO),
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                    LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO, None),
                )),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

    if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
        let alts_chirho = explicit_alts_chirho(alts_chirho);
        assert_eq!(alts_chirho.len(), 2);
        // Just _ → the one field still has a binder, matched against nothing
        assert_eq!(alts_chirho[0].binders_chirho.len(), 1);
        // _ → default, no binders
        assert_eq!(alts_chirho[1].binders_chirho.len(), 0);
    } else {
        panic!("expected CaseChirho");
    }
}

#[test]
fn desugar_view_pattern_exact_binder_rhs_uses_view_app_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    let scrut_id_chirho = ctx_chirho.fresh_id_chirho("scrut");
    let pat_chirho = PatChirho::ViewChirho {
        expr_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("double"))),
        pat_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("n"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let failure_chirho = CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0));
    // `f (double -> n) = n`: the row's result is exactly the view's binder.
    let core_chirho = ctx_chirho.match_row_chirho(
        &pat_chirho,
        scrut_id_chirho,
        &failure_chirho,
        &mut |ctx_chirho| {
            CoreExprChirho::VarChirho(ctx_chirho.lookup_scope_chirho("n").expect("n is bound"))
        },
    );

    let CoreExprChirho::AppChirho {
        fun_chirho: _,
        arg_chirho,
    } = core_chirho
    else {
        panic!("expected direct view application for exact binder RHS");
    };
    assert!(matches!(
        arg_chirho.as_ref(),
        CoreExprChirho::VarChirho(id_chirho) if *id_chirho == scrut_id_chirho
    ));
}

#[test]
fn desugar_case_nil_first_does_not_record_dead_method_fallback_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    ctx_chirho
        .set_method_occurrence_names_chirho(std::collections::HashSet::from(["==".to_string()]));
    let eq_expr_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("=="))),
            arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("k"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("a"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let expr_chirho = ExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("xs"))),
        alts_chirho: vec![
            haskelujah_ast_chirho::expr_chirho::AltChirho {
                pat_chirho: PatChirho::ListChirho {
                    elements_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                    LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO, None),
                )),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            haskelujah_ast_chirho::expr_chirho::AltChirho {
                pat_chirho: PatChirho::InfixConChirho {
                    left_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("a"))),
                    op_chirho: dummy_name_chirho(":"),
                    right_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("rest"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(eq_expr_chirho),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let _core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

    let eq_occurrences_chirho = ctx_chirho
        .method_occurrences_chirho
        .values()
        .filter(|(name_chirho, _)| name_chirho == "==")
        .count();
    assert_eq!(eq_occurrences_chirho, 1);
}

#[test]
fn desugar_function_con_pattern_binders_chirho() {
    // Test that function definition with constructor pattern also extracts binders
    // f (Just x) = x
    // f Nothing = 0
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("f"),
            matches_chirho: vec![
                MatchArmChirho {
                    pats_chirho: vec![PatChirho::ConChirho {
                        con_chirho: dummy_name_chirho("Just"),
                        args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                        dummy_name_chirho("x"),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                MatchArmChirho {
                    pats_chirho: vec![PatChirho::ConChirho {
                        con_chirho: dummy_name_chirho("Nothing"),
                        args_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                        LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO, None),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
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
    let f_rhs_chirho = &output_chirho.module_chirho.bindings_chirho[0].rhs_chirho;
    // f = \arg0 -> case arg0 of { Just x -> x; Nothing -> 0 }
    if let CoreExprChirho::LamChirho { body_chirho, .. } = f_rhs_chirho {
        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = body_chirho.as_ref() {
            let alts_chirho = explicit_alts_chirho(alts_chirho);
            assert_eq!(alts_chirho.len(), 2);
            // Just x → 1 binder
            assert_eq!(alts_chirho[0].binders_chirho.len(), 1);
            assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "x");
            // Nothing → 0 binders
            assert_eq!(alts_chirho[1].binders_chirho.len(), 0);
        } else {
            panic!("expected CaseChirho body");
        }
    } else {
        panic!("expected LamChirho");
    }
}
