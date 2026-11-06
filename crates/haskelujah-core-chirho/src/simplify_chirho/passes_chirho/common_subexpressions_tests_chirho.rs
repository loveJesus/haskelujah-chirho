// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The common subexpression elimination (CSE) tests, moved from the simplifier's inline tests (row 700).

use super::*;
use crate::expr_chirho::{BinderChirho, CoreLitChirho};
use crate::simplify_chirho::test_support_chirho::dummy_binder_chirho;

// ── CSE tests ───────────────────────────────────────────────────────

fn mk_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
    dummy_binder_chirho(name_chirho, id_chirho)
}

#[test]
fn cse_top_level_duplicate_chirho() {
    // Two top-level bindings with identical RHS should be deduplicated:
    // x = +# 1 2;  y = +# 1 2  →  x = +# 1 2;  y = x
    let bindings_chirho = vec![
        CoreBindingChirho {
            binder_chirho: mk_binder_chirho("x", 100),
            rhs_chirho: CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                ],
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        },
        CoreBindingChirho {
            binder_chirho: mk_binder_chirho("y", 101),
            rhs_chirho: CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                ],
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        },
    ];

    let result_chirho = cse_top_level_chirho(bindings_chirho);
    // y should be redirected to x (Var(100))
    assert_eq!(
        result_chirho[1].rhs_chirho,
        CoreExprChirho::VarChirho(CoreIdChirho(100))
    );
    // x should keep its original RHS
    assert!(matches!(
        result_chirho[0].rhs_chirho,
        CoreExprChirho::PrimOpChirho { .. }
    ));
}

#[test]
fn cse_top_level_no_duplicate_chirho() {
    // Different RHSes should not be deduplicated
    let bindings_chirho = vec![
        CoreBindingChirho {
            binder_chirho: mk_binder_chirho("x", 100),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        },
        CoreBindingChirho {
            binder_chirho: mk_binder_chirho("y", 101),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        },
    ];

    let result_chirho = cse_top_level_chirho(bindings_chirho);
    // Both should keep their original RHS (trivial expressions skip CSE anyway)
    assert_eq!(
        result_chirho[0].rhs_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))
    );
    assert_eq!(
        result_chirho[1].rhs_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2))
    );
}

#[test]
fn cse_top_level_recursive_skipped_chirho() {
    // Recursive bindings should not be CSE'd
    let rhs_chirho = CoreExprChirho::PrimOpChirho {
        name_chirho: "+#".to_string(),
        args_chirho: vec![
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
        ],
    };
    let bindings_chirho = vec![
        CoreBindingChirho {
            binder_chirho: mk_binder_chirho("x", 100),
            rhs_chirho: rhs_chirho.clone(),
            is_rec_chirho: true, // recursive — skip
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        },
        CoreBindingChirho {
            binder_chirho: mk_binder_chirho("y", 101),
            rhs_chirho: rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        },
    ];

    let result_chirho = cse_top_level_chirho(bindings_chirho);
    // Neither should be redirected (x is recursive)
    assert!(matches!(
        result_chirho[0].rhs_chirho,
        CoreExprChirho::PrimOpChirho { .. }
    ));
    assert!(matches!(
        result_chirho[1].rhs_chirho,
        CoreExprChirho::PrimOpChirho { .. }
    ));
}

#[test]
fn cse_let_binding_duplicate_chirho() {
    // let x = +# 1 2; y = +# 1 2 in +# x y
    // → let x = +# 1 2; y = x in +# x y
    let expr_chirho = CoreExprChirho::LetChirho {
        rec_chirho: false,
        binds_chirho: vec![
            (
                mk_binder_chirho("x", 200),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                    ],
                },
            ),
            (
                mk_binder_chirho("y", 201),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                    ],
                },
            ),
        ],
        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
            name_chirho: "+#".to_string(),
            args_chirho: vec![
                CoreExprChirho::VarChirho(CoreIdChirho(200)),
                CoreExprChirho::VarChirho(CoreIdChirho(201)),
            ],
        }),
    };

    let result_chirho = cse_expr_chirho(&expr_chirho);
    if let CoreExprChirho::LetChirho { binds_chirho, .. } = &result_chirho {
        // y should be redirected to x
        assert_eq!(
            binds_chirho[1].1,
            CoreExprChirho::VarChirho(CoreIdChirho(200))
        );
    } else {
        panic!("expected LetChirho");
    }
}

#[test]
fn cse_let_binding_different_rhs_chirho() {
    // let x = +# 1 2; y = +# 3 4 in ... — different RHSes, no CSE
    let expr_chirho = CoreExprChirho::LetChirho {
        rec_chirho: false,
        binds_chirho: vec![
            (
                mk_binder_chirho("x", 200),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                    ],
                },
            ),
            (
                mk_binder_chirho("y", 201),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(3)),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(4)),
                    ],
                },
            ),
        ],
        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(200))),
    };

    let result_chirho = cse_expr_chirho(&expr_chirho);
    if let CoreExprChirho::LetChirho { binds_chirho, .. } = &result_chirho {
        // Both should keep their original RHS
        assert!(matches!(
            binds_chirho[0].1,
            CoreExprChirho::PrimOpChirho { .. }
        ));
        assert!(matches!(
            binds_chirho[1].1,
            CoreExprChirho::PrimOpChirho { .. }
        ));
    } else {
        panic!("expected LetChirho");
    }
}

#[test]
fn cse_redirect_in_body_chirho() {
    // let x = ConApp("True", []); y = ConApp("True", []) in y
    // → let x = ConApp("True", []); y = x in y  — and body references y→x
    let expr_chirho = CoreExprChirho::LetChirho {
        rec_chirho: false,
        binds_chirho: vec![
            (
                mk_binder_chirho("x", 300),
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: "True".to_string(),
                    args_chirho: vec![],
                },
            ),
            (
                mk_binder_chirho("y", 301),
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: "True".to_string(),
                    args_chirho: vec![],
                },
            ),
        ],
        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(301))),
    };

    let result_chirho = cse_expr_chirho(&expr_chirho);
    if let CoreExprChirho::LetChirho {
        binds_chirho,
        body_chirho,
        ..
    } = &result_chirho
    {
        // y's RHS redirected to x
        assert_eq!(
            binds_chirho[1].1,
            CoreExprChirho::VarChirho(CoreIdChirho(300))
        );
        // body's reference to y redirected to x
        assert_eq!(**body_chirho, CoreExprChirho::VarChirho(CoreIdChirho(300)));
    } else {
        panic!("expected LetChirho");
    }
}
