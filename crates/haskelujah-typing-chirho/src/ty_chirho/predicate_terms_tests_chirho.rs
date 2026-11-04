// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Synthetic predicate-substrate controls. These do not claim a GHC verdict.
use super::*;
use crate::ty_chirho::{SchemePredChirho, SchemePredPartsChirho, TyVarChirho};
use haskelujah_ast_chirho::name_chirho::{
    ClassIdentityUnknownChirho, CompilerClassChirho, PredicateClassIdentityChirho,
    PredicateClassProofChirho,
};

fn predicate_chirho(
    proof_chirho: PredicateClassProofChirho,
    kind_chirho: &str,
) -> SchemePredChirho {
    SchemePredChirho::from_producer_chirho(
        SchemePredPartsChirho {
            kind_tys_chirho: vec![TyChirho::ConChirho(kind_chirho.to_owned())],
            class_name_chirho: "CChirho".to_owned(),
            ty_chirho: TyChirho::VarChirho(TyVarChirho(0)),
            extra_tys_chirho: vec![TyChirho::VarChirho(TyVarChirho(1))],
        },
        proof_chirho,
    )
}
fn proof_chirho(identity_chirho: CompilerClassChirho) -> PredicateClassProofChirho {
    PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::CompilerChirho(
        identity_chirho,
    ))
}

#[test]
fn mapping_and_borrowed_iteration_preserve_all_groups_and_identity_chirho() {
    let predicate_chirho = predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool");
    let mut visited_chirho = Vec::new();
    let mapped_chirho = predicate_chirho.mapped_chirho(|term_chirho| {
        visited_chirho.push(term_chirho.clone());
        if matches!(term_chirho, TyChirho::VarChirho(_)) {
            TyChirho::int_chirho()
        } else {
            term_chirho.clone()
        }
    });
    assert_eq!(
        visited_chirho,
        predicate_chirho
            .semantic_tys_chirho()
            .cloned()
            .collect::<Vec<_>>()
    );
    assert_eq!(mapped_chirho.kind_tys_chirho, vec![TyChirho::bool_chirho()]);
    assert_eq!(mapped_chirho.ty_chirho, TyChirho::int_chirho());
    assert_eq!(mapped_chirho.extra_tys_chirho, vec![TyChirho::int_chirho()]);
    assert_eq!(
        mapped_chirho.class_identity_chirho(),
        predicate_chirho.class_identity_chirho()
    );
}

#[test]
fn duplicate_meet_keeps_first_order_and_unknown_is_absorbing_chirho() {
    let mut duplicate_chirho =
        predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool");
    let different_kind_chirho =
        predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Char");
    let unknown_chirho = predicate_chirho(
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho),
        "Bool",
    );
    duplicate_chirho.meet_class_identity_chirho(unknown_chirho.class_identity_chirho());
    let values_chirho = OrderedPredicateSetChirho::try_new_chirho(vec![
        predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool"),
        different_kind_chirho.clone(),
        unknown_chirho,
    ])
    .unwrap_or_else(|_| panic!("small set fits bounds"))
    .into_vec_chirho();
    assert_eq!(values_chirho.len(), 2);
    assert_eq!(
        values_chirho[0].class_identity_chirho(),
        duplicate_chirho.class_identity_chirho()
    );
    assert_eq!(values_chirho[1], different_kind_chirho);
}

#[test]
fn conflicting_positive_proofs_do_not_survive_a_legacy_equal_key_chirho() {
    let values_chirho = OrderedPredicateSetChirho::try_new_chirho(vec![
        predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool"),
        predicate_chirho(proof_chirho(CompilerClassChirho::ShowChirho), "Bool"),
    ])
    .unwrap_or_else(|_| panic!("small set fits bounds"))
    .into_vec_chirho();
    assert_eq!(values_chirho.len(), 1);
    assert_eq!(
        values_chirho[0].class_identity_chirho(),
        &PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ConflictChirho)
    );
}

#[test]
fn refusing_a_large_term_returns_every_original_predicate_chirho() {
    let mut large_chirho = predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool");
    large_chirho
        .extra_tys_chirho
        .push(TyChirho::ConChirho("x".repeat(5000)));
    let original_chirho = vec![
        predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool"),
        large_chirho,
    ];
    let failure_chirho = match OrderedPredicateSetChirho::try_new_chirho(original_chirho.clone()) {
        Ok(_) => panic!("oversized predicate must be refused"),
        Err(failure_chirho) => failure_chirho,
    };
    assert_eq!(
        failure_chirho.error_chirho,
        PredicateSetErrorChirho::TermBoundChirho
    );
    assert_eq!(failure_chirho.predicates_chirho, original_chirho);
    assert_eq!(
        failure_chirho.predicates_chirho[0].class_identity_chirho(),
        original_chirho[0].class_identity_chirho()
    );
}
