// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Applied evidence types (row 694, P1, P2): carried only for ground applications, and never
//! able to make evidence that agreed on its keys disagree.

use std::collections::HashMap;

use super::*;
use crate::ty_chirho::TyVarChirho;

fn box_of_chirho(argument_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("BoxChirho".to_string())),
        Box::new(argument_chirho),
    )
}

fn int_chirho() -> TyChirho {
    TyChirho::ConChirho("Int".to_string())
}

fn show_record_chirho(applied_chirho: Option<TyChirho>) -> ReferenceEvidenceChirho {
    ReferenceEvidenceChirho {
        applied_head_chirho: None,
        class_name_chirho: "Show".to_string(),
        ty_key_chirho: Some("BoxChirho".to_string()),
        applied_ty_chirho: applied_chirho,
    }
}

#[test]
fn only_a_ground_application_is_carried_chirho() {
    assert_eq!(
        applied_evidence_ty_chirho(&box_of_chirho(int_chirho())),
        Some(box_of_chirho(int_chirho()))
    );
    assert_eq!(applied_evidence_ty_chirho(&int_chirho()), None);
    assert_eq!(
        applied_evidence_ty_chirho(&box_of_chirho(TyChirho::VarChirho(TyVarChirho(3)))),
        None
    );
    assert_eq!(
        applied_evidence_ty_chirho(&TyChirho::ListChirho(Box::new(int_chirho()))),
        None
    );
}

#[test]
fn an_applied_type_never_makes_agreeing_keys_disagree_chirho() {
    let bool_box_chirho = box_of_chirho(TyChirho::ConChirho("Bool".to_string()));
    let agreed_chirho = agreeing_references_chirho(HashMap::from([
        (
            1u32,
            vec![
                vec![show_record_chirho(Some(box_of_chirho(int_chirho())))],
                vec![show_record_chirho(Some(box_of_chirho(int_chirho())))],
            ],
        ),
        (
            2u32,
            vec![
                vec![show_record_chirho(Some(box_of_chirho(int_chirho())))],
                vec![show_record_chirho(Some(bool_box_chirho))],
            ],
        ),
    ]));
    assert_eq!(
        agreed_chirho[&1],
        vec![show_record_chirho(Some(box_of_chirho(int_chirho())))]
    );
    assert_eq!(agreed_chirho[&2], vec![show_record_chirho(None)]);
}

#[test]
fn captures_that_disagree_on_a_key_prove_nothing_chirho() {
    let mut other_key_chirho = show_record_chirho(None);
    other_key_chirho.ty_key_chirho = Some("Int".to_string());
    let agreed_chirho = agreeing_references_chirho(HashMap::from([(
        1u32,
        vec![vec![show_record_chirho(None)], vec![other_key_chirho]],
    )]));
    assert!(agreed_chirho.is_empty());
}

#[test]
fn only_a_complete_ground_head_is_carried_chirho() {
    // `AskChirho Int (BoxChirho Int)` keeps its whole head (row 694, #31909). A one-parameter
    // head, or one naming a variable anywhere, carries none, so no instance is chosen for it by
    // matching.
    let head_chirho = vec![int_chirho(), box_of_chirho(int_chirho())];
    assert_eq!(
        applied_evidence_head_chirho(&head_chirho),
        Some(head_chirho.clone())
    );
    assert_eq!(
        applied_evidence_head_chirho(&[box_of_chirho(int_chirho())]),
        None
    );
    assert_eq!(
        applied_evidence_head_chirho(&[
            int_chirho(),
            box_of_chirho(TyChirho::VarChirho(TyVarChirho(3)))
        ]),
        None
    );
}

#[test]
fn only_a_head_every_capture_resolved_alike_survives_chirho() {
    let head_record_chirho = |second_chirho: TyChirho| ReferenceEvidenceChirho {
        applied_head_chirho: Some(vec![int_chirho(), second_chirho]),
        class_name_chirho: "AskChirho".to_string(),
        ty_key_chirho: Some("Int".to_string()),
        applied_ty_chirho: None,
    };
    let bool_chirho = TyChirho::ConChirho("Bool".to_string());
    let agreed_chirho = agreeing_references_chirho(HashMap::from([
        (
            1u32,
            vec![
                vec![head_record_chirho(bool_chirho.clone())],
                vec![head_record_chirho(bool_chirho.clone())],
            ],
        ),
        (
            2u32,
            vec![
                vec![head_record_chirho(bool_chirho.clone())],
                vec![head_record_chirho(int_chirho())],
            ],
        ),
    ]));
    assert_eq!(agreed_chirho[&1], vec![head_record_chirho(bool_chirho)]);
    assert_eq!(agreed_chirho[&2][0].applied_head_chirho, None);
}
