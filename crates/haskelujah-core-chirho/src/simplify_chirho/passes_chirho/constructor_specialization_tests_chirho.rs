// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The constructor specialization (SpecConstr) tests, moved from the simplifier's inline tests (row 700).

use super::*;
use crate::expr_chirho::{CoreAltChirho, CoreLitChirho};
use crate::simplify_chirho::test_support_chirho::dummy_binder_chirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

// -----------------------------------------------------------------------
// SpecConstr tests
// -----------------------------------------------------------------------

#[test]
fn spec_constr_finds_pattern_chirho() {
    // f = \xs -> case xs of
    //   Nil -> 0
    //   Cons x rest -> x +# f rest
    // `f` is recursive, case scrutinee is the param `xs`,
    // recursive call `f rest` passes `rest` (an alt binder).
    let xs_chirho = dummy_binder_chirho("xs", 0);
    let x_chirho = dummy_binder_chirho("x", 1);
    let rest_chirho = dummy_binder_chirho("rest", 2);

    let body_chirho = CoreExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        bind_chirho: dummy_binder_chirho("_", 99),
        result_ty_chirho: TyChirho::int_chirho(),
        alts_chirho: vec![
            CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho("Nil".to_string()),
                binders_chirho: vec![],
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            },
            CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho("Cons".to_string()),
                binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(CoreIdChirho(1)),
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(100))),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(2))),
                        },
                    ],
                },
            },
        ],
    };

    let patterns_chirho =
        find_call_patterns_chirho(&CoreIdChirho(100), &[xs_chirho.clone()], &body_chirho);
    assert_eq!(patterns_chirho.len(), 1);
    assert_eq!(patterns_chirho[0].arg_idx_chirho, 0);
    assert_eq!(patterns_chirho[0].con_name_chirho, "Cons");
    assert_eq!(patterns_chirho[0].field_binders_chirho.len(), 2);
}

#[test]
fn spec_constr_creates_copy_chirho() {
    // Same as above but wrapped in a CoreBindingChirho
    let xs_chirho = dummy_binder_chirho("xs", 0);
    let x_chirho = dummy_binder_chirho("x", 1);
    let rest_chirho = dummy_binder_chirho("rest", 2);

    let binding_chirho = CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(100),
            name_chirho: "f".to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LamChirho {
            binder_chirho: xs_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                bind_chirho: dummy_binder_chirho("_", 99),
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nil".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Cons".to_string()),
                        binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "+#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(CoreIdChirho(1)),
                                CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(
                                        100,
                                    ))),
                                    arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(
                                        2,
                                    ))),
                                },
                            ],
                        },
                    },
                ],
            }),
        },
        is_rec_chirho: true,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };

    let result_chirho = spec_constr_chirho(vec![binding_chirho]);
    // Should have original f + $sc_f_Cons
    assert_eq!(result_chirho.len(), 2, "should have original + specialized");
    assert_eq!(result_chirho[0].binder_chirho.name_chirho, "f");
    assert_eq!(result_chirho[1].binder_chirho.name_chirho, "$sc_f_Cons");
    assert!(
        result_chirho[1].is_rec_chirho,
        "specialized copy should be recursive"
    );
    assert_eq!(
        result_chirho[1].inline_chirho,
        InlineAnnotationChirho::AlwaysChirho
    );
}

#[test]
fn spec_constr_skips_non_recursive_chirho() {
    // Non-recursive function should not be spec-constr'd
    let binding_chirho = CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(100),
            name_chirho: "f".to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        },
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };

    let result_chirho = spec_constr_chirho(vec![binding_chirho]);
    assert_eq!(result_chirho.len(), 1, "non-recursive should pass through");
}

#[test]
fn spec_constr_skips_noinline_chirho() {
    // NOINLINE recursive function should not be spec-constr'd
    let xs_chirho = dummy_binder_chirho("xs", 0);
    let x_chirho = dummy_binder_chirho("x", 1);
    let rest_chirho = dummy_binder_chirho("rest", 2);

    let binding_chirho = CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(100),
            name_chirho: "f".to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LamChirho {
            binder_chirho: xs_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                bind_chirho: dummy_binder_chirho("_", 99),
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nil".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Cons".to_string()),
                        binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                        rhs_chirho: CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(100))),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(2))),
                        },
                    },
                ],
            }),
        },
        is_rec_chirho: true,
        inline_chirho: InlineAnnotationChirho::NeverChirho,
    };

    let result_chirho = spec_constr_chirho(vec![binding_chirho]);
    assert_eq!(result_chirho.len(), 1, "NOINLINE should not be specialized");
}

#[test]
fn spec_constr_no_pattern_found_chirho() {
    // Recursive function that doesn't case-match its arg
    // f = \x -> f (x +# 1) — no constructor pattern
    let binding_chirho = CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(100),
            name_chirho: "f".to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(100))),
                arg_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(CoreIdChirho(0)),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    ],
                }),
            }),
        },
        is_rec_chirho: true,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };

    let result_chirho = spec_constr_chirho(vec![binding_chirho]);
    assert_eq!(result_chirho.len(), 1, "no constructor pattern found");
}
