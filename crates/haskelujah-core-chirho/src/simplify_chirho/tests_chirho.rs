// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The simplifier's core unit tests, moved from its inline test module (row 700).

use super::test_support_chirho::dummy_binder_chirho;
use super::*;
use crate::expr_chirho::CoreLitChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

#[test]
fn beta_reduction_chirho() {
    // (\x -> x) 42 → 42
    let expr_chirho = CoreExprChirho::AppChirho {
        fun_chirho: Box::new(CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        }),
        arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
    };

    let result_chirho = simplify_expr_chirho(&expr_chirho);
    assert_eq!(
        result_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
    );
}

#[test]
fn dead_binding_elimination_chirho() {
    // let x = 42 in 99 → 99  (x is unused)
    let expr_chirho = CoreExprChirho::LetChirho {
        rec_chirho: false,
        binds_chirho: vec![(
            dummy_binder_chirho("x", 0),
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
        )],
        body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99))),
    };

    let result_chirho = simplify_expr_chirho(&expr_chirho);
    assert_eq!(
        result_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99))
    );
}

#[test]
fn live_binding_preserved_chirho() {
    // let x = 42 in x → let x = 42 in x  (x IS used)
    let expr_chirho = CoreExprChirho::LetChirho {
        rec_chirho: false,
        binds_chirho: vec![(
            dummy_binder_chirho("x", 0),
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
        )],
        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
    };

    let result_chirho = simplify_expr_chirho(&expr_chirho);
    assert!(matches!(result_chirho, CoreExprChirho::LetChirho { .. }));
}

#[test]
fn case_of_known_literal_chirho() {
    // case 1 of { 1 -> 10; _ -> 20 } → 10
    let expr_chirho = CoreExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))),
        bind_chirho: dummy_binder_chirho("wild", 99),
        result_ty_chirho: TyChirho::int_chirho(),
        alts_chirho: vec![
            CoreAltChirho {
                con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(1)),
                binders_chirho: vec![],
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
            },
            CoreAltChirho {
                con_chirho: AltConChirho::DefaultChirho,
                binders_chirho: vec![],
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(20)),
            },
        ],
    };

    let result_chirho = simplify_expr_chirho(&expr_chirho);
    assert_eq!(
        result_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10))
    );
}

#[test]
fn case_of_known_literal_default_chirho() {
    // case 99 of { 1 -> 10; _ -> 20 } → 20
    let expr_chirho = CoreExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99))),
        bind_chirho: dummy_binder_chirho("wild", 99),
        result_ty_chirho: TyChirho::int_chirho(),
        alts_chirho: vec![
            CoreAltChirho {
                con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(1)),
                binders_chirho: vec![],
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
            },
            CoreAltChirho {
                con_chirho: AltConChirho::DefaultChirho,
                binders_chirho: vec![],
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(20)),
            },
        ],
    };

    let result_chirho = simplify_expr_chirho(&expr_chirho);
    assert_eq!(
        result_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(20))
    );
}

#[test]
fn nested_beta_reduction_chirho() {
    // (\x -> \y -> x) 1 2 → 1
    let inner_lam_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("y", 1),
        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
    };
    let outer_lam_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("x", 0),
        body_chirho: Box::new(inner_lam_chirho),
    };
    let app1_chirho = CoreExprChirho::AppChirho {
        fun_chirho: Box::new(outer_lam_chirho),
        arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))),
    };
    let app2_chirho = CoreExprChirho::AppChirho {
        fun_chirho: Box::new(app1_chirho),
        arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2))),
    };

    let result_chirho = simplify_expr_chirho(&app2_chirho);
    assert_eq!(
        result_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))
    );
}

#[test]
fn simplify_module_chirho() {
    // Module with one binding: f = (\x -> x) 42
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("f", 10),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 0),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                }),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    assert_eq!(result_chirho.bindings_chirho.len(), 1);
    assert_eq!(
        result_chirho.bindings_chirho[0].rhs_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
    );
}

#[test]
fn free_vars_basic_chirho() {
    // \x -> y  — y is free, x is bound
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("x", 0),
        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
    };
    let fv_chirho = free_vars_chirho(&expr_chirho);
    assert!(fv_chirho.contains(&CoreIdChirho(1)));
    assert!(!fv_chirho.contains(&CoreIdChirho(0)));
}

#[test]
fn subst_respects_shadowing_chirho() {
    // subst [x := 42] in (\x -> x) should NOT substitute
    let lam_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("x", 0),
        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
    };
    let result_chirho = subst_var_chirho(
        &lam_chirho,
        CoreIdChirho(0),
        &CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
    );
    // Should be unchanged — the lambda shadows x
    assert_eq!(result_chirho, lam_chirho);
}

#[test]
fn constant_fold_primop_add_chirho() {
    // PrimOp("+#", [3, 4]) → 7
    let expr_chirho = CoreExprChirho::PrimOpChirho {
        name_chirho: "+#".to_string(),
        args_chirho: vec![
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(3)),
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(4)),
        ],
    };
    let result_chirho = simplify_expr_chirho(&expr_chirho);
    assert_eq!(
        result_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(7))
    );
}

#[test]
fn constant_fold_primop_mul_chirho() {
    // PrimOp("*#", [6, 7]) → 42
    let expr_chirho = CoreExprChirho::PrimOpChirho {
        name_chirho: "*#".to_string(),
        args_chirho: vec![
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(6)),
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(7)),
        ],
    };
    let result_chirho = simplify_expr_chirho(&expr_chirho);
    assert_eq!(
        result_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
    );
}

#[test]
fn constant_fold_primop_negate_chirho() {
    // PrimOp("negate#", [42]) → -42
    let expr_chirho = CoreExprChirho::PrimOpChirho {
        name_chirho: "negate#".to_string(),
        args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))],
    };
    let result_chirho = simplify_expr_chirho(&expr_chirho);
    assert_eq!(
        result_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(-42))
    );
}

#[test]
fn primop_non_literal_args_preserved_chirho() {
    // PrimOp("+#", [Var(0), Lit(1)]) — not foldable, should be preserved
    let expr_chirho = CoreExprChirho::PrimOpChirho {
        name_chirho: "+#".to_string(),
        args_chirho: vec![
            CoreExprChirho::VarChirho(CoreIdChirho(0)),
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
        ],
    };
    let result_chirho = simplify_expr_chirho(&expr_chirho);
    assert!(matches!(result_chirho, CoreExprChirho::PrimOpChirho { .. }));
}

// ── Inlining tests ──

#[test]
fn expr_size_lit_chirho() {
    assert_eq!(
        expr_size_chirho(&CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
        1
    );
}

#[test]
fn expr_size_app_chirho() {
    // f x → 1 (App) + 1 (Var f) + 1 (Var x) = 3
    let expr_chirho = CoreExprChirho::AppChirho {
        fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
    };
    assert_eq!(expr_size_chirho(&expr_chirho), 3);
}

#[test]
fn expr_size_lam_chirho() {
    // \x -> 42 → 1 (Lam) + 1 (Lit) = 2
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("x", 0),
        body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
    };
    assert_eq!(expr_size_chirho(&expr_chirho), 2);
}

#[test]
fn inline_always_annotation_chirho() {
    // f = 42 (INLINE), g = f
    // After inlining: g = 42
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 10),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::AlwaysChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("g", 11),
                rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    // g should be inlined to 42
    assert_eq!(
        result_chirho.bindings_chirho[1].rhs_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
    );
}

#[test]
fn noinline_annotation_chirho() {
    // f = 42 (NOINLINE), g = f
    // After inlining: g should still be Var(f), not 42
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 10),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NeverChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("g", 11),
                rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    // g should NOT be inlined — f is NOINLINE
    assert_eq!(
        result_chirho.bindings_chirho[1].rhs_chirho,
        CoreExprChirho::VarChirho(CoreIdChirho(10))
    );
}

#[test]
fn auto_inline_trivial_chirho() {
    // f = x (trivial var-to-var), g = f
    // After inlining: g = x
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 10),
                rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(5)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("g", 11),
                rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    // g should be inlined to Var(5) since f = Var(5) is trivial
    assert_eq!(
        result_chirho.bindings_chirho[1].rhs_chirho,
        CoreExprChirho::VarChirho(CoreIdChirho(5))
    );
}

#[test]
fn no_auto_inline_non_trivial_chirho() {
    // f = \x -> x (non-trivial, size=2), g = f (no annotation)
    // Auto-inlining should NOT inline because f is not trivial and has no INLINE
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 10),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 0),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("g", 11),
                rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    // g should NOT be inlined since f is a lambda (not trivial)
    assert_eq!(
        result_chirho.bindings_chirho[1].rhs_chirho,
        CoreExprChirho::VarChirho(CoreIdChirho(10))
    );
}

#[test]
fn inlinable_small_function_chirho() {
    // f = \x -> x (INLINABLE, size=2 <= threshold), g = f
    // Should inline because INLINABLE + small
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 10),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 0),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::InlinableChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("g", 11),
                rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    // g should be the identity lambda after inlining
    assert!(matches!(
        result_chirho.bindings_chirho[1].rhs_chirho,
        CoreExprChirho::LamChirho { .. }
    ));
}

#[test]
fn recursive_not_inlined_chirho() {
    // f = f (recursive), g = f
    // Recursive bindings should never be auto-inlined
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 10),
                rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("g", 11),
                rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    // g should not be inlined — f is recursive
    assert_eq!(
        result_chirho.bindings_chirho[1].rhs_chirho,
        CoreExprChirho::VarChirho(CoreIdChirho(10))
    );
}

#[test]
fn inline_always_large_function_chirho() {
    // f = \x -> \y -> PrimOp("+#", [x, y]) (INLINE — should inline regardless of size)
    let large_rhs_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("x", 0),
        body_chirho: Box::new(CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("y", 1),
            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(CoreIdChirho(0)),
                    CoreExprChirho::VarChirho(CoreIdChirho(1)),
                ],
            }),
        }),
    };
    assert!(expr_size_chirho(&large_rhs_chirho) > 1); // size = 5
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 10),
                rhs_chirho: large_rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::AlwaysChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("g", 11),
                rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    // g should be the lambda from f, not Var(10)
    assert!(matches!(
        result_chirho.bindings_chirho[1].rhs_chirho,
        CoreExprChirho::LamChirho { .. }
    ));
}

#[test]
fn simplify_reorders_acyclic_rec_let_binds_chirho() {
    let result_binder_chirho = dummy_binder_chirho("result", 1);
    let a_binder_chirho = dummy_binder_chirho("a", 2);
    let b_binder_chirho = dummy_binder_chirho("b", 3);
    let expr_chirho = CoreExprChirho::LetChirho {
        rec_chirho: true,
        binds_chirho: vec![
            (
                result_binder_chirho.clone(),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(a_binder_chirho.id_chirho),
                        CoreExprChirho::VarChirho(b_binder_chirho.id_chirho),
                    ],
                },
            ),
            (
                a_binder_chirho.clone(),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
            ),
            (
                b_binder_chirho.clone(),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(15)),
            ),
        ],
        body_chirho: Box::new(CoreExprChirho::VarChirho(result_binder_chirho.id_chirho)),
    };

    let simplified_chirho = simplify_expr_chirho(&expr_chirho);
    let mut bind_order_chirho = Vec::new();
    let mut cursor_chirho = &simplified_chirho;
    loop {
        match cursor_chirho {
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => {
                assert!(
                    !rec_chirho,
                    "acyclic letrec should become nested non-rec lets"
                );
                assert_eq!(
                    binds_chirho.len(),
                    1,
                    "acyclic letrec should lower to one binding per let"
                );
                bind_order_chirho.push(binds_chirho[0].0.id_chirho);
                cursor_chirho = body_chirho;
            }
            CoreExprChirho::VarChirho(id_chirho) => {
                assert_eq!(*id_chirho, result_binder_chirho.id_chirho);
                break;
            }
            _ => panic!("expected nested let chain ending in result var"),
        }
    }
    assert_eq!(
        bind_order_chirho,
        vec![
            a_binder_chirho.id_chirho,
            b_binder_chirho.id_chirho,
            result_binder_chirho.id_chirho
        ]
    );
}

#[test]
fn simplify_preserves_recursive_let_cycle_chirho() {
    let f_binder_chirho = dummy_binder_chirho("f", 10);
    let expr_chirho = CoreExprChirho::LetChirho {
        rec_chirho: true,
        binds_chirho: vec![(
            f_binder_chirho.clone(),
            CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 11),
                body_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_binder_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))),
                }),
            },
        )],
        body_chirho: Box::new(CoreExprChirho::VarChirho(f_binder_chirho.id_chirho)),
    };

    let simplified_chirho = simplify_expr_chirho(&expr_chirho);
    let CoreExprChirho::LetChirho { rec_chirho, .. } = simplified_chirho else {
        panic!("expected LetChirho");
    };
    assert!(rec_chirho, "true recursive lets should stay recursive");
}

// -----------------------------------------------------------------------
// Specialization pass tests
// -----------------------------------------------------------------------

#[test]
fn specialize_creates_copy_chirho() {
    // f = \x -> x, with {-# SPECIALIZE f :: Int -> Int #-}
    let mut spec_map_chirho = std::collections::HashMap::new();
    spec_map_chirho.insert("f".to_string(), vec!["Int -> Int".to_string()]);

    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("f", 1),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 2),
                body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(2))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: spec_map_chirho,
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    // Should have 2 bindings: original f + $spec_f_0
    assert_eq!(result_chirho.bindings_chirho.len(), 2);
    assert_eq!(
        result_chirho.bindings_chirho[1].binder_chirho.name_chirho,
        "$spec_f_0"
    );
    assert_eq!(
        result_chirho.bindings_chirho[1].inline_chirho,
        InlineAnnotationChirho::AlwaysChirho
    );
}

#[test]
fn specialize_multiple_types_chirho() {
    // f = \x -> x, with two specializations
    let mut spec_map_chirho = std::collections::HashMap::new();
    spec_map_chirho.insert(
        "f".to_string(),
        vec!["Int -> Int".to_string(), "Double -> Double".to_string()],
    );

    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("f", 1),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 2),
                body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(2))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: spec_map_chirho,
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    // Should have 3 bindings: original f + $spec_f_0 + $spec_f_1
    assert_eq!(result_chirho.bindings_chirho.len(), 3);
    assert_eq!(
        result_chirho.bindings_chirho[1].binder_chirho.name_chirho,
        "$spec_f_0"
    );
    assert_eq!(
        result_chirho.bindings_chirho[2].binder_chirho.name_chirho,
        "$spec_f_1"
    );
}

#[test]
fn specialize_nonexistent_binding_chirho() {
    // SPECIALIZE for a binding that doesn't exist — should be a no-op
    let mut spec_map_chirho = std::collections::HashMap::new();
    spec_map_chirho.insert("nonexistent".to_string(), vec!["Int -> Int".to_string()]);

    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("f", 1),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: spec_map_chirho,
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    // Should still have just 1 binding — nonexistent target is silently skipped
    assert_eq!(result_chirho.bindings_chirho.len(), 1);
}

#[test]
fn specialize_preserves_rhs_chirho() {
    // Specialized copy should have the same RHS as the original
    let mut spec_map_chirho = std::collections::HashMap::new();
    spec_map_chirho.insert("f".to_string(), vec!["Int -> Int".to_string()]);

    let rhs_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("x", 2),
        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
            name_chirho: "+#".to_string(),
            args_chirho: vec![
                CoreExprChirho::VarChirho(CoreIdChirho(2)),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
            ],
        }),
    };
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("f", 1),
            rhs_chirho: rhs_chirho.clone(),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: spec_map_chirho,
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    let config_chirho = SimplifyConfigChirho::default();
    let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
    assert_eq!(result_chirho.bindings_chirho.len(), 2);
    // The specialized copy's RHS should be structurally identical
    // (after simplification, it may get optimized, but the structure should match)
    assert!(matches!(
        &result_chirho.bindings_chirho[1].rhs_chirho,
        CoreExprChirho::LamChirho { .. }
    ));
}
