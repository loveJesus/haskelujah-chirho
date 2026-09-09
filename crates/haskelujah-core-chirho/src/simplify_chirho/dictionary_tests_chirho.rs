// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, InlineAnnotationChirho,
};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::{elide_dicts_chirho, free_vars_chirho};

fn binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
    BinderChirho {
        id_chirho: CoreIdChirho(id_chirho),
        name_chirho: name_chirho.to_string(),
        ty_chirho: TyChirho::int_chirho(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn binding_chirho(
    name_chirho: &str,
    id_chirho: u32,
    rhs_chirho: CoreExprChirho,
) -> CoreBindingChirho {
    CoreBindingChirho {
        binder_chirho: binder_chirho(name_chirho, id_chirho),
        rhs_chirho,
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    }
}

fn var_chirho(id_chirho: u32) -> CoreExprChirho {
    CoreExprChirho::VarChirho(CoreIdChirho(id_chirho))
}

fn app_chirho(fun_chirho: CoreExprChirho, arg_chirho: CoreExprChirho) -> CoreExprChirho {
    CoreExprChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
    }
}

fn selector_chirho() -> CoreBindingChirho {
    binding_chirho(
        "selectChirho",
        1,
        CoreExprChirho::LamChirho {
            binder_chirho: binder_chirho("evidenceChirho", 2),
            body_chirho: Box::new(CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(var_chirho(2)),
                bind_chirho: binder_chirho("caseChirho", 3),
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EvidenceChirho".to_string()),
                    binders_chirho: vec![
                        binder_chirho("superChirho", 4),
                        binder_chirho("methodChirho", 5),
                    ],
                    rhs_chirho: var_chirho(5),
                }],
            }),
        },
    )
}

fn dictionary_chirho() -> CoreBindingChirho {
    binding_chirho(
        "instanceChirho",
        6,
        CoreExprChirho::ConAppChirho {
            con_name_chirho: "EvidenceChirho".to_string(),
            args_chirho: vec![var_chirho(7), var_chirho(8)],
        },
    )
}

#[test]
fn known_evidence_selects_the_actual_method_not_a_name_default_chirho() {
    let source_chirho = app_chirho(app_chirho(var_chirho(1), var_chirho(6)), var_chirho(9));
    let rewritten_chirho =
        elide_dicts_chirho(&source_chirho, &[selector_chirho(), dictionary_chirho()]);
    assert_eq!(rewritten_chirho, app_chirho(var_chirho(8), var_chirho(9)));
}

#[test]
fn unknown_evidence_keeps_its_binder_and_call_argument_chirho() {
    let source_chirho = CoreExprChirho::LamChirho {
        binder_chirho: binder_chirho("$dNumInt", 10),
        body_chirho: Box::new(app_chirho(
            app_chirho(var_chirho(1), var_chirho(10)),
            var_chirho(9),
        )),
    };
    let rewritten_chirho = elide_dicts_chirho(&source_chirho, &[selector_chirho()]);
    assert_eq!(
        rewritten_chirho, source_chirho,
        "a suggestive dictionary name is not proof of its runtime value"
    );
    assert_eq!(
        free_vars_chirho(&rewritten_chirho),
        free_vars_chirho(&source_chirho)
    );
}

#[test]
fn missing_evidence_is_not_fabricated_from_a_selector_name_chirho() {
    let source_chirho = app_chirho(
        var_chirho(1),
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
    );
    let mut selector_chirho = selector_chirho();
    selector_chirho.binder_chirho.name_chirho = "$sel_Num_fromInteger".to_string();
    assert_eq!(
        elide_dicts_chirho(&source_chirho, &[selector_chirho]),
        source_chirho
    );
}

#[test]
fn local_evidence_and_type_wrappers_use_the_same_projection_chirho() {
    let dictionary_chirho = dictionary_chirho();
    let source_chirho = CoreExprChirho::LetChirho {
        rec_chirho: false,
        binds_chirho: vec![(
            dictionary_chirho.binder_chirho,
            dictionary_chirho.rhs_chirho,
        )],
        body_chirho: Box::new(app_chirho(
            CoreExprChirho::TyAppChirho {
                expr_chirho: Box::new(var_chirho(1)),
                ty_chirho: TyChirho::int_chirho(),
            },
            var_chirho(6),
        )),
    };
    let rewritten_chirho = elide_dicts_chirho(&source_chirho, &[selector_chirho()]);
    let CoreExprChirho::LetChirho { body_chirho, .. } = rewritten_chirho else {
        panic!("expected let")
    };
    assert_eq!(*body_chirho, var_chirho(8));
}

#[test]
fn recursive_aliases_terminate_without_inventing_evidence_chirho() {
    let source_chirho = app_chirho(var_chirho(1), var_chirho(6));
    let definitions_chirho = [
        selector_chirho(),
        binding_chirho("cycleChirho", 6, var_chirho(7)),
        binding_chirho("backChirho", 7, var_chirho(6)),
    ];
    assert_eq!(
        elide_dicts_chirho(&source_chirho, &definitions_chirho),
        source_chirho
    );
}

#[test]
fn runtime_calls_keep_evidence_even_when_callee_is_not_a_selector_chirho() {
    let source_chirho = app_chirho(var_chirho(11), var_chirho(6));
    let mut dictionary_chirho = dictionary_chirho();
    dictionary_chirho.binder_chirho.name_chirho = "$fCustomChirho".to_string();
    assert_eq!(
        elide_dicts_chirho(&source_chirho, &[dictionary_chirho]),
        source_chirho
    );
}
