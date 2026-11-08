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
        ground_pred_chirho: None,
        applied_head_chirho: None,
        class_name_chirho: "Show".to_string(),
        class_identity_chirho:
            haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho::UnknownChirho(
                haskelujah_ast_chirho::name_chirho::ClassIdentityUnknownChirho::LegacyChirho,
            ),
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
        ground_pred_chirho: None,
        applied_head_chirho: Some(vec![int_chirho(), second_chirho]),
        class_name_chirho: "AskChirho".to_string(),
        class_identity_chirho:
            haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho::UnknownChirho(
                haskelujah_ast_chirho::name_chirho::ClassIdentityUnknownChirho::LegacyChirho,
            ),
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

#[test]
fn agreeing_predicate_shapes_meet_class_proof_in_both_orders_chirho() {
    use crate::class_chirho::{PredChirho, PredPartsChirho};
    use haskelujah_ast_chirho::name_chirho::CompilerClassChirho;
    let proven_chirho = PredChirho::from_compiler_chirho(
        CompilerClassChirho::EqChirho,
        PredPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "Eq".to_string(),
            ty_chirho: int_chirho(),
            extra_tys_chirho: Vec::new(),
        },
    );
    let unknown_chirho = PredChirho::new_chirho("Eq", int_chirho());
    // The producer carries each predicate's own class proof beside its ground goal.
    let record_chirho = |predicate_chirho: PredChirho| ReferenceEvidenceChirho {
        class_name_chirho: "Eq".to_string(),
        class_identity_chirho: predicate_chirho.class_identity_chirho().clone(),
        ty_key_chirho: Some("Int".to_string()),
        applied_ty_chirho: None,
        applied_head_chirho: None,
        ground_pred_chirho: Some(predicate_chirho),
    };
    for predicates_chirho in [
        [proven_chirho.clone(), unknown_chirho.clone()],
        [unknown_chirho.clone(), proven_chirho.clone()],
    ] {
        let agreed_chirho = agreeing_references_chirho(HashMap::from([(
            1_u32,
            predicates_chirho
                .into_iter()
                .map(|predicate_chirho| vec![record_chirho(predicate_chirho)])
                .collect(),
        )]));
        let record_chirho = &agreed_chirho[&1][0];
        assert_eq!(record_chirho.ty_key_chirho.as_deref(), Some("Int"));
        // The position's own class proof meets the same way (row 692, gpt #32681).
        assert_eq!(
            &record_chirho.class_identity_chirho,
            unknown_chirho.class_identity_chirho()
        );
        assert_eq!(
            record_chirho
                .ground_pred_chirho
                .as_ref()
                .unwrap()
                .class_identity_chirho(),
            unknown_chirho.class_identity_chirho()
        );
    }
    let agreed_chirho = agreeing_references_chirho(HashMap::from([(
        1_u32,
        vec![
            vec![record_chirho(proven_chirho.clone())],
            vec![record_chirho(proven_chirho.clone())],
        ],
    )]));
    assert_eq!(
        agreed_chirho[&1][0]
            .ground_pred_chirho
            .as_ref()
            .unwrap()
            .class_identity_chirho(),
        proven_chirho.class_identity_chirho()
    );
    assert_eq!(
        &agreed_chirho[&1][0].class_identity_chirho,
        proven_chirho.class_identity_chirho()
    );
}
