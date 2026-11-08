// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Let and top-level recursion detection, and bang and lazy patterns.

use super::*;

#[test]
fn desugar_let_non_recursive_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // let x = 10 in x
    let expr_chirho = ExprChirho::LetChirho {
        binds_chirho: vec![
            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                pat_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                    LitChirho::IntChirho(10, SpanChirho::DUMMY_CHIRHO, None),
                )),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    if let CoreExprChirho::LetChirho { rec_chirho, .. } = &core_chirho {
        assert!(!rec_chirho, "non-recursive let should have rec=false");
    } else {
        panic!("expected LetChirho");
    }
}

#[test]
fn desugar_let_recursive_detected_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // let f x = f x in f 0
    let expr_chirho = ExprChirho::LetChirho {
        binds_chirho: vec![
            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("f"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("f"))),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        body_chirho: Box::new(ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("f"))),
            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                0,
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    if let CoreExprChirho::LetChirho { rec_chirho, .. } = &core_chirho {
        assert!(rec_chirho, "self-recursive let should have rec=true");
    } else {
        panic!("expected LetChirho");
    }
}

#[test]
fn desugar_top_level_is_rec_chirho() {
    // Top-level bindings in Haskell are always implicitly mutually recursive
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
    assert!(
        output_chirho.module_chirho.bindings_chirho[0].is_rec_chirho,
        "top-level bindings should be marked recursive"
    );
}

#[test]
fn desugar_bang_pattern_case_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // case x of { !y -> y }
    let expr_chirho = ExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
        alts_chirho: vec![haskelujah_ast_chirho::expr_chirho::AltChirho {
            pat_chirho: PatChirho::BangChirho {
                inner_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("y"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho("y"))),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // Should produce a case expression (bang delegates to variable → default alt)
    assert!(matches!(core_chirho, CoreExprChirho::CaseChirho { .. }));
    if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
        assert_eq!(alts_chirho.len(), 1);
        assert!(matches!(
            alts_chirho[0].con_chirho,
            AltConChirho::DefaultChirho
        ));
    }
}

#[test]
fn desugar_lazy_pattern_case_chirho() {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    // case x of { ~(Just a) -> a }
    let expr_chirho = ExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
        alts_chirho: vec![haskelujah_ast_chirho::expr_chirho::AltChirho {
            pat_chirho: PatChirho::LazyChirho {
                inner_chirho: Box::new(PatChirho::ConChirho {
                    type_args_chirho: Vec::new(),
                    con_chirho: dummy_name_chirho("Just"),
                    args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("a"))],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho("a"))),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
    // A lazy pattern scrutinises NOTHING where it is matched: the result is
    // the binding of `a`, not a case on `x`. (The old desugaring made this a
    // default alternative whose binder `a` stood for the whole scrutinee, so
    // `case Just 7 of ~(Just n) -> n` printed `Just 7`.)
    let CoreExprChirho::LetChirho { binds_chirho, .. } = &core_chirho else {
        panic!("a lazy alternative must not scrutinise at the match: {core_chirho:?}");
    };
    // `a` is bound through a selector that matches `Just` when demanded.
    let (_, selector_chirho) = binds_chirho
        .iter()
        .find(|(binder_chirho, _)| binder_chirho.name_chirho == "a")
        .expect("the lazy pattern binds a");
    let CoreExprChirho::CaseChirho { alts_chirho, .. } = selector_chirho else {
        panic!("a is selected by a match: {selector_chirho:?}");
    };
    assert!(matches!(
        &alts_chirho[0].con_chirho,
        AltConChirho::DataConChirho(con_chirho) if con_chirho == "Just"
    ));
}

#[test]
fn desugar_bang_pattern_fun_arg_chirho() {
    // f !x = x  → should produce a Lam with the var bound
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("f"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![PatChirho::BangChirho {
                    inner_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("x"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
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
    let f_rhs_chirho = &output_chirho.module_chirho.bindings_chirho[0].rhs_chirho;
    // Should be \x -> x (bang on simple var pattern is a simple lambda)
    assert!(matches!(f_rhs_chirho, CoreExprChirho::LamChirho { .. }));
}
