// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The absence and dead argument tests, moved from the simplifier's inline tests (row 700).

use super::*;
use crate::expr_chirho::CoreLitChirho;
use crate::simplify_chirho::test_support_chirho::dummy_binder_chirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

// -----------------------------------------------------------------------
// Demand analysis tests
// -----------------------------------------------------------------------

#[test]
fn usage_absent_variable_chirho() {
    // \x -> 42 — x is never used
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("x", 0),
        body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
    };
    let usage_chirho = analyze_usage_chirho(&expr_chirho);
    assert_eq!(usage_chirho.len(), 1);
    assert_eq!(usage_chirho[0].1, UsageChirho::AbsentChirho);
}

#[test]
fn usage_used_once_chirho() {
    // \x -> x — x is used exactly once
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("x", 0),
        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
    };
    let usage_chirho = analyze_usage_chirho(&expr_chirho);
    assert_eq!(usage_chirho.len(), 1);
    assert_eq!(usage_chirho[0].1, UsageChirho::UsedOnceChirho);
}

#[test]
fn usage_used_many_chirho() {
    // \x -> x +# x — x is used twice
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("x", 0),
        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
            name_chirho: "+#".to_string(),
            args_chirho: vec![
                CoreExprChirho::VarChirho(CoreIdChirho(0)),
                CoreExprChirho::VarChirho(CoreIdChirho(0)),
            ],
        }),
    };
    let usage_chirho = analyze_usage_chirho(&expr_chirho);
    assert_eq!(usage_chirho.len(), 1);
    assert_eq!(usage_chirho[0].1, UsageChirho::UsedManyChirho);
}

#[test]
fn usage_multi_arg_mixed_chirho() {
    // \x -> \y -> \z -> x +# z — x used once, y absent, z used once
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("x", 0),
        body_chirho: Box::new(CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("y", 1),
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("z", 2),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(CoreIdChirho(0)),
                        CoreExprChirho::VarChirho(CoreIdChirho(2)),
                    ],
                }),
            }),
        }),
    };
    let usage_chirho = analyze_usage_chirho(&expr_chirho);
    assert_eq!(usage_chirho.len(), 3);
    assert_eq!(
        usage_chirho[0].1,
        UsageChirho::UsedOnceChirho,
        "x used once"
    );
    assert_eq!(usage_chirho[1].1, UsageChirho::AbsentChirho, "y absent");
    assert_eq!(
        usage_chirho[2].1,
        UsageChirho::UsedOnceChirho,
        "z used once"
    );
}

#[test]
fn dead_arg_elim_removes_absent_chirho() {
    // f = \x -> \y -> x — y is dead, should be eliminated
    let binding_chirho = CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(100),
            name_chirho: "f".to_string(),
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::int_chirho(),
                TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("y", 1),
                body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            }),
        },
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };

    let result_chirho = dead_arg_elimination_chirho(vec![binding_chirho]);
    assert_eq!(result_chirho.len(), 1);
    // Should be \x -> x (y removed)
    if let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = &result_chirho[0].rhs_chirho
    {
        assert_eq!(binder_chirho.name_chirho, "x");
        assert!(matches!(
            body_chirho.as_ref(),
            CoreExprChirho::VarChirho(CoreIdChirho(0))
        ));
    } else {
        panic!("expected lambda after dead arg elim");
    }
}

#[test]
fn dead_arg_elim_keeps_used_chirho() {
    // f = \x -> x — x is used, should NOT be eliminated
    let binding_chirho = CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(100),
            name_chirho: "f".to_string(),
            ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        },
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };

    let result_chirho = dead_arg_elimination_chirho(vec![binding_chirho]);
    assert_eq!(result_chirho.len(), 1);
    // Should still be \x -> x (nothing to eliminate)
    if let CoreExprChirho::LamChirho { binder_chirho, .. } = &result_chirho[0].rhs_chirho {
        assert_eq!(binder_chirho.name_chirho, "x");
    } else {
        panic!("expected lambda preserved");
    }
}

#[test]
fn dead_arg_elim_skips_noinline_chirho() {
    // {-# NOINLINE f #-}
    // f = \x -> 42 — x is dead but NOINLINE prevents elimination
    let binding_chirho = CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(100),
            name_chirho: "f".to_string(),
            ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
        },
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NeverChirho,
    };

    let result_chirho = dead_arg_elimination_chirho(vec![binding_chirho]);
    assert_eq!(result_chirho.len(), 1);
    // Should still be \x -> 42 (NOINLINE preserves everything)
    if let CoreExprChirho::LamChirho { binder_chirho, .. } = &result_chirho[0].rhs_chirho {
        assert_eq!(binder_chirho.name_chirho, "x");
    } else {
        panic!("expected lambda preserved for NOINLINE");
    }
}

#[test]
fn dead_arg_elim_middle_arg_chirho() {
    // f = \x -> \y -> \z -> x +# z — y is dead (middle arg)
    let binding_chirho = CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(100),
            name_chirho: "f".to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("y", 1),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("z", 2),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(CoreIdChirho(0)),
                            CoreExprChirho::VarChirho(CoreIdChirho(2)),
                        ],
                    }),
                }),
            }),
        },
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };

    let result_chirho = dead_arg_elimination_chirho(vec![binding_chirho]);
    assert_eq!(result_chirho.len(), 1);
    // Should be \x -> \z -> x +# z (y removed)
    if let CoreExprChirho::LamChirho {
        binder_chirho: b1_chirho,
        body_chirho,
    } = &result_chirho[0].rhs_chirho
    {
        assert_eq!(b1_chirho.name_chirho, "x");
        if let CoreExprChirho::LamChirho {
            binder_chirho: b2_chirho,
            ..
        } = body_chirho.as_ref()
        {
            assert_eq!(
                b2_chirho.name_chirho, "z",
                "y should be eliminated, z remains"
            );
        } else {
            panic!("expected second lambda for z");
        }
    } else {
        panic!("expected lambda after dead arg elim");
    }
}

#[test]
fn count_usage_shadowed_chirho() {
    // \x -> let x = 99 in x — the outer x is absent (shadowed by let binding)
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: dummy_binder_chirho("x", 0),
        body_chirho: Box::new(CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(
                dummy_binder_chirho("x", 0),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99)),
            )],
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        }),
    };
    let usage_chirho = analyze_usage_chirho(&expr_chirho);
    assert_eq!(usage_chirho.len(), 1);
    // The outer x is shadowed by the let binding, so it's absent
    assert_eq!(usage_chirho[0].1, UsageChirho::AbsentChirho);
}
