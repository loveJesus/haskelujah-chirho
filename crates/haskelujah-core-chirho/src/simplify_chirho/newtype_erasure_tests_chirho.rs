// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Newtype constructor erasure (row 694): a listed constructor disappears from
//! construction, unsaturated use and matching; every other constructor stays.

use std::collections::{HashMap, HashSet};

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};

use super::erase_newtype_constructors_chirho;

fn binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
    BinderChirho {
        id_chirho: CoreIdChirho(id_chirho),
        name_chirho: name_chirho.to_string(),
        ty_chirho: TyChirho::int_chirho(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn int_chirho(value_chirho: i64) -> CoreExprChirho {
    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(value_chirho))
}

fn con_chirho(name_chirho: &str, args_chirho: Vec<CoreExprChirho>) -> CoreExprChirho {
    CoreExprChirho::ConAppChirho {
        con_name_chirho: name_chirho.to_string(),
        args_chirho,
    }
}

/// One module binding `mainChirho` (id 1) to `rhs_chirho`, erased for `AgeChirho`.
fn erased_rhs_chirho(
    rhs_chirho: CoreExprChirho,
) -> (CoreExprChirho, HashMap<CoreIdChirho, String>) {
    let mut module_chirho = CoreModuleChirho {
        name_chirho: "MainChirho".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: binder_chirho("mainChirho", 1),
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::from([(CoreIdChirho(1), "mainChirho".to_string())]),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };
    erase_newtype_constructors_chirho(
        &mut module_chirho,
        &HashSet::from(["AgeChirho".to_string()]),
    );
    let names_chirho = module_chirho.names_chirho.clone();
    (
        module_chirho
            .bindings_chirho
            .pop()
            .expect("the binding survives")
            .rhs_chirho,
        names_chirho,
    )
}

#[test]
fn a_saturated_listed_constructor_becomes_its_field_chirho() {
    let (erased_chirho, _) = erased_rhs_chirho(con_chirho("AgeChirho", vec![int_chirho(7)]));
    assert_eq!(erased_chirho, int_chirho(7));
}

#[test]
fn an_unlisted_constructor_keeps_its_box_chirho() {
    let boxed_chirho = con_chirho(
        "JustChirho",
        vec![con_chirho("AgeChirho", vec![int_chirho(7)])],
    );
    let (erased_chirho, _) = erased_rhs_chirho(boxed_chirho);
    // Only the inner, listed constructor goes.
    assert_eq!(erased_chirho, con_chirho("JustChirho", vec![int_chirho(7)]));
}

#[test]
fn an_unsaturated_listed_constructor_becomes_the_identity_chirho() {
    let (erased_chirho, names_chirho) = erased_rhs_chirho(con_chirho("AgeChirho", vec![]));
    let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = erased_chirho
    else {
        panic!("an unsaturated newtype constructor must become a lambda");
    };
    assert_eq!(
        *body_chirho,
        CoreExprChirho::VarChirho(binder_chirho.id_chirho)
    );
    // The fresh binder is past every id the module used, and is named.
    assert!(binder_chirho.id_chirho.0 > 1);
    assert!(names_chirho.contains_key(&binder_chirho.id_chirho));
}

#[test]
fn matching_a_listed_constructor_binds_without_forcing_chirho() {
    // case scrutinee of caseBinder { AgeChirho field -> field; _ -> 0 }
    let case_chirho = CoreExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(con_chirho("AgeChirho", vec![int_chirho(7)])),
        bind_chirho: binder_chirho("caseBinderChirho", 2),
        result_ty_chirho: TyChirho::int_chirho(),
        alts_chirho: vec![
            CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho("AgeChirho".to_string()),
                binders_chirho: vec![binder_chirho("fieldChirho", 3)],
                rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(3)),
            },
            CoreAltChirho {
                con_chirho: AltConChirho::DefaultChirho,
                binders_chirho: vec![],
                rhs_chirho: int_chirho(0),
            },
        ],
    };
    let (erased_chirho, _) = erased_rhs_chirho(case_chirho);
    let expected_chirho = CoreExprChirho::LetChirho {
        rec_chirho: false,
        binds_chirho: vec![(binder_chirho("caseBinderChirho", 2), int_chirho(7))],
        body_chirho: Box::new(CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(
                binder_chirho("fieldChirho", 3),
                CoreExprChirho::VarChirho(CoreIdChirho(2)),
            )],
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(3))),
        }),
    };
    assert_eq!(erased_chirho, expected_chirho);
}

#[test]
fn matching_an_unlisted_constructor_still_dispatches_chirho() {
    let case_chirho = CoreExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
        bind_chirho: binder_chirho("caseBinderChirho", 2),
        result_ty_chirho: TyChirho::int_chirho(),
        alts_chirho: vec![CoreAltChirho {
            con_chirho: AltConChirho::DataConChirho("JustChirho".to_string()),
            binders_chirho: vec![binder_chirho("fieldChirho", 3)],
            rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(3)),
        }],
    };
    let (erased_chirho, _) = erased_rhs_chirho(case_chirho.clone());
    assert_eq!(erased_chirho, case_chirho);
}
