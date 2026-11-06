// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The strictness and worker/wrapper tests, moved from the simplifier's inline tests (row 700).

use super::*;
use crate::expr_chirho::CoreLitChirho;
use crate::simplify_chirho::test_support_chirho::dummy_binder_chirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

// -- Strictness analysis tests --

#[test]
fn strictness_case_scrutinee_is_strict_chirho() {
    // \x -> case x of { _ -> 42 }
    // x is strict (used as case scrutinee)
    let x_chirho = dummy_binder_chirho("x", 0);
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: x_chirho.clone(),
        body_chirho: Box::new(CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            bind_chirho: dummy_binder_chirho("_", 99),
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DefaultChirho,
                binders_chirho: vec![],
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
            }],
        }),
    };
    let demands_chirho = analyze_demand_chirho(&expr_chirho);
    assert_eq!(demands_chirho.len(), 1);
    assert_eq!(demands_chirho[0].1, DemandChirho::StrictChirho);
}

#[test]
fn strictness_primop_arg_is_strict_chirho() {
    // \x -> \y -> x +# y
    // Both x and y are strict (primop arguments)
    let x_chirho = dummy_binder_chirho("x", 0);
    let y_chirho = dummy_binder_chirho("y", 1);
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: x_chirho.clone(),
        body_chirho: Box::new(CoreExprChirho::LamChirho {
            binder_chirho: y_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(CoreIdChirho(0)),
                    CoreExprChirho::VarChirho(CoreIdChirho(1)),
                ],
            }),
        }),
    };
    let demands_chirho = analyze_demand_chirho(&expr_chirho);
    assert_eq!(demands_chirho.len(), 2);
    assert_eq!(demands_chirho[0].1, DemandChirho::StrictChirho);
    assert_eq!(demands_chirho[1].1, DemandChirho::StrictChirho);
}

#[test]
fn strictness_unused_var_is_lazy_chirho() {
    // \x -> 42
    // x is lazy (never used)
    let x_chirho = dummy_binder_chirho("x", 0);
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: x_chirho.clone(),
        body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
    };
    let demands_chirho = analyze_demand_chirho(&expr_chirho);
    assert_eq!(demands_chirho.len(), 1);
    assert_eq!(demands_chirho[0].1, DemandChirho::LazyChirho);
}

#[test]
fn strictness_mixed_demands_chirho() {
    // \x -> \y -> case x of { _ -> y }
    // x is strict (case scrutinee), y is lazy (just returned)
    let x_chirho = dummy_binder_chirho("x", 0);
    let y_chirho = dummy_binder_chirho("y", 1);
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: x_chirho.clone(),
        body_chirho: Box::new(CoreExprChirho::LamChirho {
            binder_chirho: y_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                bind_chirho: dummy_binder_chirho("_", 99),
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(1)),
                }],
            }),
        }),
    };
    let demands_chirho = analyze_demand_chirho(&expr_chirho);
    assert_eq!(demands_chirho.len(), 2);
    assert_eq!(
        demands_chirho[0].1,
        DemandChirho::StrictChirho,
        "x should be strict"
    );
    assert_eq!(
        demands_chirho[1].1,
        DemandChirho::LazyChirho,
        "y should be lazy"
    );
}

#[test]
fn worker_wrapper_creates_worker_chirho() {
    // f = \x -> case x of { _ -> 42 }
    // Should generate wrapper f and worker $wf
    let x_chirho = dummy_binder_chirho("x", 0);
    let binding_chirho = CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(100),
            name_chirho: "f".to_string(),
            ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LamChirho {
            binder_chirho: x_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                bind_chirho: dummy_binder_chirho("_", 99),
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                }],
            }),
        },
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };

    let result_chirho = worker_wrapper_chirho(vec![binding_chirho]);
    assert_eq!(result_chirho.len(), 2, "should have wrapper + worker");
    assert_eq!(
        result_chirho[0].binder_chirho.name_chirho, "f",
        "first should be wrapper"
    );
    assert_eq!(
        result_chirho[1].binder_chirho.name_chirho, "$wf",
        "second should be worker"
    );
    assert_eq!(
        result_chirho[1].inline_chirho,
        InlineAnnotationChirho::AlwaysChirho,
        "worker should be marked INLINE"
    );
}

#[test]
fn worker_wrapper_skips_lazy_only_chirho() {
    // f = \x -> 42  (x is lazy — no worker/wrapper needed)
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
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };

    let result_chirho = worker_wrapper_chirho(vec![binding_chirho]);
    assert_eq!(
        result_chirho.len(),
        1,
        "lazy-only function should not be split"
    );
}

#[test]
fn worker_wrapper_skips_noinline_chirho() {
    // {-# NOINLINE f #-}
    // f = \x -> case x of { _ -> 42 }
    // Should NOT transform despite strict arg
    let binding_chirho = CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(100),
            name_chirho: "f".to_string(),
            ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                bind_chirho: dummy_binder_chirho("_", 99),
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                }],
            }),
        },
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NeverChirho,
    };

    let result_chirho = worker_wrapper_chirho(vec![binding_chirho]);
    assert_eq!(
        result_chirho.len(),
        1,
        "NOINLINE function should not be split"
    );
}
