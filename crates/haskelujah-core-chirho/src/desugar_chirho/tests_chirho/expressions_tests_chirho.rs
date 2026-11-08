// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Desugaring expressions: functions, conditionals, literals, lambdas, lists, infix
//! operators, sections, guards and arithmetic sequences.

use super::*;

#[test]
fn desugar_simple_function_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("f"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho(
                    "x",
                ))),
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

    let output_chirho =
        desugar_module_chirho(&module_chirho).expect("valid constructor signatures in fixture");
    let core_chirho = &output_chirho.module_chirho;
    assert_eq!(core_chirho.name_chirho, "Test");
    assert_eq!(core_chirho.bindings_chirho.len(), 1);
    assert_eq!(
        core_chirho.bindings_chirho[0].binder_chirho.name_chirho,
        "f"
    );
    assert!(matches!(
        core_chirho.bindings_chirho[0].rhs_chirho,
        CoreExprChirho::LamChirho { .. }
    ));
    // Name map should contain the binder's name
    assert!(
        output_chirho
            .names_chirho
            .values()
            .any(|n_chirho| n_chirho == "f")
    );
}

#[test]
fn desugar_if_to_case_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    let expr_chirho = ExprChirho::IfChirho {
        cond_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
        then_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            1,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        else_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            0,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    assert!(matches!(core_chirho, CoreExprChirho::CaseChirho { .. }));
    if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
        assert_eq!(alts_chirho.len(), 2);
        assert!(matches!(
            alts_chirho[0].con_chirho,
            AltConChirho::DataConChirho(ref s_chirho) if s_chirho == "True"
        ));
    }
}

#[test]
fn desugar_literal_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    let expr_chirho =
        ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO, None));
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // Integer literals are wrapped with fromInteger for numeric
    // overloading: `42` → `fromInteger 42`
    match &core_chirho {
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            assert!(matches!(fun_chirho.as_ref(), CoreExprChirho::VarChirho(_)));
            assert_eq!(
                **arg_chirho,
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
            );
        }
        other_chirho => panic!("expected App(fromInteger, 42), got {:?}", other_chirho),
    }
}

#[test]
fn desugar_lambda_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    let expr_chirho = ExprChirho::LamChirho {
        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
        body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    assert!(matches!(core_chirho, CoreExprChirho::LamChirho { .. }));
}

#[test]
fn desugar_list_to_cons_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    let expr_chirho = ExprChirho::ListChirho {
        elements_chirho: vec![
            ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None)),
            ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO, None)),
        ],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // Should be nested ConApp(`:`, [1, ConApp(`:`, [2, ConApp(`[]`, [])])])
    assert!(matches!(core_chirho, CoreExprChirho::ConAppChirho { .. }));
}

#[test]
fn desugar_pretty_prints_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("Pretty"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("main"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                    LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO, None),
                )),
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

    let desugar_out_chirho =
        desugar_module_chirho(&module_chirho).expect("valid constructor signatures in fixture");
    let output_chirho =
        crate::pretty_chirho::pretty_module_chirho(&desugar_out_chirho.module_chirho);
    assert!(output_chirho.contains("-- module Pretty"));
    assert!(output_chirho.contains("main"));
    assert!(output_chirho.contains("0"));
}

#[test]
fn desugar_infix_to_primop_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // a + b (Int operands) → App(App(Var(+), fromInteger(1)), fromInteger(2))
    // +, -, * go through the dict transform for type-aware dispatch
    let expr_chirho = ExprChirho::InfixChirho {
        left_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            1,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        op_chirho: dummy_name_chirho("+"),
        right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            2,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // +, -, * are class methods (Num), so they desugar to App chains
    assert!(matches!(core_chirho, CoreExprChirho::AppChirho { .. }));

    // Float operands should use the float primop directly
    let float_expr_chirho = ExprChirho::InfixChirho {
        left_chirho: Box::new(ExprChirho::LitChirho(LitChirho::FloatChirho(
            1.5,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        op_chirho: dummy_name_chirho("+"),
        right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::FloatChirho(
            2.5,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let float_core_chirho = ctx_chirho.desugar_expr_chirho(&float_expr_chirho);
    assert!(matches!(
        float_core_chirho,
        CoreExprChirho::PrimOpChirho { .. }
    ));
    if let CoreExprChirho::PrimOpChirho { name_chirho, .. } = &float_core_chirho {
        assert_eq!(name_chirho, "+.#");
    }
}

#[test]
fn desugar_infix_user_op_to_app_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // a `foo` b → App(App(Var(foo), a), b)  — non-builtin operator
    let expr_chirho = ExprChirho::InfixChirho {
        left_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            1,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        op_chirho: dummy_name_chirho("foo"),
        right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            2,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // Non-builtin should still produce App
    assert!(matches!(core_chirho, CoreExprChirho::AppChirho { .. }));
}

#[test]
fn desugar_left_section_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // (+ 1) → \y -> (+) y 1  -- Wait, no, LeftSection is (op arg): \y -> op arg y
    // Actually per Haskell: (+ 1) means \y -> (+) y 1 (right section)
    // But in our AST LeftSection is (op arg): the operator comes first.
    // Let's just check it produces a lambda.
    let expr_chirho = ExprChirho::LeftSectionChirho {
        op_chirho: dummy_name_chirho("+"),
        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            1,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    assert!(matches!(core_chirho, CoreExprChirho::LamChirho { .. }));
}

#[test]
fn desugar_right_section_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    let expr_chirho = ExprChirho::RightSectionChirho {
        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            1,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        op_chirho: dummy_name_chirho("+"),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    assert!(matches!(core_chirho, CoreExprChirho::LamChirho { .. }));
}

#[test]
fn desugar_guards_chirho() {
    use haskelujah_ast_chirho::expr_chirho::GuardedExprChirho;

    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    let rhs_chirho = RhsChirho::GuardedChirho(vec![
        GuardedExprChirho::boolean_chirho(
            ExprChirho::ConChirho(dummy_name_chirho("True")),
            ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None)),
            SpanChirho::DUMMY_CHIRHO,
        ),
        GuardedExprChirho::boolean_chirho(
            ExprChirho::ConChirho(dummy_name_chirho("True")),
            ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO, None)),
            SpanChirho::DUMMY_CHIRHO,
        ),
    ]);
    let core_chirho = ctx_chirho.desugar_rhs_chirho(&rhs_chirho);
    // Should be nested case: case True of { True -> 1; _ -> case True of { True -> 2; _ -> error } }
    assert!(matches!(core_chirho, CoreExprChirho::CaseChirho { .. }));
    if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
        assert_eq!(alts_chirho.len(), 2); // True branch + default
        // Default branch should be another case
        assert!(matches!(
            alts_chirho[1].rhs_chirho,
            CoreExprChirho::CaseChirho { .. }
        ));
    }
}

#[test]
fn desugar_arith_seq_from_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // [1..]
    let expr_chirho = ExprChirho::ArithSeqChirho {
        from_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            1,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        then_chirho: None,
        to_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // Should be App(enumFrom, 1) — a function call, not a primop
    assert!(matches!(core_chirho, CoreExprChirho::AppChirho { .. }));
}

#[test]
fn desugar_arith_seq_from_to_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // [1..10]
    let expr_chirho = ExprChirho::ArithSeqChirho {
        from_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            1,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        then_chirho: None,
        to_chirho: Some(Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            10,
            SpanChirho::DUMMY_CHIRHO,
            None,
        )))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // Should be App(App(enumFromTo, 1), 10) — a function call, not a primop
    assert!(matches!(core_chirho, CoreExprChirho::AppChirho { .. }));
}
