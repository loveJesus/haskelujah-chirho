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
    let values_chirho = OrderedPredicateSetChirho::new_chirho(vec![
        predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool"),
        different_kind_chirho.clone(),
        unknown_chirho,
    ])
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
    let values_chirho = OrderedPredicateSetChirho::new_chirho(vec![
        predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool"),
        predicate_chirho(proof_chirho(CompilerClassChirho::ShowChirho), "Bool"),
    ])
    .into_vec_chirho();
    assert_eq!(values_chirho.len(), 1);
    assert_eq!(
        values_chirho[0].class_identity_chirho(),
        &PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ConflictChirho)
    );
}

fn legacy_dedup_chirho(mut values_chirho: Vec<SchemePredChirho>) -> Vec<SchemePredChirho> {
    let mut seen_chirho = std::collections::HashSet::new();
    values_chirho.retain(|value_chirho| seen_chirho.insert(value_chirho.clone()));
    values_chirho
}

fn assert_resource_shape_chirho(
    values_chirho: Vec<SchemePredChirho>,
    expected_chirho: PredicateSetErrorChirho,
    work_chirho: usize,
) {
    let legacy_chirho = legacy_dedup_chirho(values_chirho.clone());
    let first_pointer_chirho = values_chirho[0].class_name_chirho.as_ptr();
    let mut remaining_chirho = work_chirho;
    let actual_chirho = OrderedPredicateSetChirho::with_metadata_work_chirho(
        values_chirho,
        work_chirho,
        |weight_chirho| match remaining_chirho.checked_sub(weight_chirho) {
            Some(next_chirho) => {
                remaining_chirho = next_chirho;
                true
            }
            None => false,
        },
    );
    assert_eq!(
        actual_chirho.metadata_refusal_chirho(),
        Some(expected_chirho)
    );
    let actual_chirho = actual_chirho.into_vec_chirho();
    assert_eq!(
        actual_chirho, legacy_chirho,
        "ordinary predicate shape, order and dedup are exactly the legacy result"
    );
    assert_eq!(
        actual_chirho[0].class_name_chirho.as_ptr(),
        first_pointer_chirho,
        "the first owned record survives without reconstruction"
    );
    for predicate_chirho in actual_chirho {
        assert_eq!(
            predicate_chirho.class_identity_chirho(),
            &PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ResourceChirho(
                expected_chirho
            ))
        );
    }
}

#[test]
fn large_predicate_refusal_keeps_legacy_shape_order_and_dedup_chirho() {
    let first_chirho = predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool");
    let mut large_chirho = predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Char");
    large_chirho
        .extra_tys_chirho
        .push(TyChirho::ConChirho("x".repeat(5000)));
    assert_resource_shape_chirho(
        vec![
            first_chirho.clone(),
            large_chirho.clone(),
            first_chirho,
            large_chirho,
        ],
        PredicateSetErrorChirho::WeightChirho { limit_chirho: 4096 },
        1 << 25,
    );
}

#[test]
fn deep_predicate_refusal_keeps_legacy_shape_order_and_dedup_chirho() {
    let mut deep_chirho = predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool");
    for _index_chirho in 0..256 {
        deep_chirho.ty_chirho = TyChirho::ListChirho(Box::new(deep_chirho.ty_chirho));
    }
    let other_chirho = predicate_chirho(
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho),
        "Char",
    );
    assert_resource_shape_chirho(
        vec![
            deep_chirho.clone(),
            other_chirho.clone(),
            deep_chirho,
            other_chirho,
        ],
        PredicateSetErrorChirho::DepthChirho { limit_chirho: 128 },
        1 << 25,
    );
}

#[test]
fn work_exhaustion_keeps_legacy_shape_and_terminal_resource_metadata_chirho() {
    let first_chirho = predicate_chirho(
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho),
        "Bool",
    );
    let other_chirho = predicate_chirho(proof_chirho(CompilerClassChirho::ShowChirho), "Char");
    assert_resource_shape_chirho(
        vec![
            first_chirho.clone(),
            other_chirho.clone(),
            first_chirho,
            other_chirho,
        ],
        PredicateSetErrorChirho::WorkChirho { limit_chirho: 0 },
        0,
    );
}

#[test]
fn iterative_legacy_relation_matches_public_eq_for_every_type_variant_and_group_chirho() {
    use crate::ty_chirho::MultChirho;
    let variable_chirho = TyVarChirho(0);
    let terms_chirho = vec![
        TyChirho::VarChirho(variable_chirho),
        TyChirho::ConChirho("TChirho".to_owned()),
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("FChirho".to_owned())),
            Box::new(TyChirho::int_chirho()),
        ),
        TyChirho::FunChirho(
            Box::new(TyChirho::int_chirho()),
            Box::new(TyChirho::bool_chirho()),
            MultChirho::ManyChirho,
        ),
        TyChirho::FunChirho(
            Box::new(TyChirho::int_chirho()),
            Box::new(TyChirho::bool_chirho()),
            MultChirho::OneChirho,
        ),
        TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::bool_chirho()]),
        TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
        TyChirho::ForallVarChirho("aChirho".to_owned()),
        TyChirho::QualChirho {
            context_chirho: vec![
                TyChirho::ConChirho("FirstChirho".to_owned()),
                TyChirho::ConChirho("SecondChirho".to_owned()),
            ],
            body_chirho: Box::new(TyChirho::int_chirho()),
        },
        TyChirho::QualChirho {
            context_chirho: vec![
                TyChirho::ConChirho("SecondChirho".to_owned()),
                TyChirho::ConChirho("FirstChirho".to_owned()),
            ],
            body_chirho: Box::new(TyChirho::int_chirho()),
        },
        TyChirho::QualChirho {
            context_chirho: vec![TyChirho::ConChirho("FirstChirho".to_owned())],
            body_chirho: Box::new(TyChirho::bool_chirho()),
        },
        TyChirho::ForallChirho {
            vars_chirho: vec![variable_chirho],
            body_chirho: Box::new(TyChirho::VarChirho(variable_chirho)),
        },
        TyChirho::RequiredForallChirho {
            vars_chirho: vec![variable_chirho],
            body_chirho: Box::new(TyChirho::VarChirho(variable_chirho)),
        },
        TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(1)],
            body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(1))),
        },
    ];
    let mut values_chirho = Vec::new();
    for term_chirho in &terms_chirho {
        for group_chirho in 0..3 {
            let mut value_chirho =
                predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool");
            match group_chirho {
                0 => value_chirho.kind_tys_chirho = vec![term_chirho.clone()],
                1 => value_chirho.ty_chirho = term_chirho.clone(),
                _ => value_chirho.extra_tys_chirho = vec![term_chirho.clone()],
            }
            values_chirho.push(value_chirho);
        }
    }
    let mut other_class_chirho = values_chirho[0].clone();
    other_class_chirho.class_name_chirho = "OtherChirho".to_owned();
    values_chirho.push(other_class_chirho);
    let mut no_kind_chirho = values_chirho[0].clone();
    no_kind_chirho.kind_tys_chirho.clear();
    values_chirho.push(no_kind_chirho);
    let mut no_extra_chirho = values_chirho[0].clone();
    no_extra_chirho.extra_tys_chirho.clear();
    values_chirho.push(no_extra_chirho);
    let mut two_extras_chirho = values_chirho[0].clone();
    two_extras_chirho
        .extra_tys_chirho
        .push(TyChirho::bool_chirho());
    values_chirho.push(two_extras_chirho);
    for left_chirho in &values_chirho {
        for right_chirho in &values_chirho {
            assert_eq!(
                super::super::legacy_predicate_key_chirho::equal_chirho(left_chirho, right_chirho),
                left_chirho == right_chirho
            );
        }
    }
    let mut duplicate_chirho = values_chirho.clone();
    duplicate_chirho.extend(values_chirho.iter().rev().cloned());
    assert_eq!(
        OrderedPredicateSetChirho::new_chirho(duplicate_chirho.clone()).into_vec_chirho(),
        legacy_dedup_chirho(duplicate_chirho)
    );
}

#[test]
fn enclosing_predicate_meet_touches_only_the_first_contained_record_chirho() {
    let first_chirho = predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool");
    let second_chirho = predicate_chirho(proof_chirho(CompilerClassChirho::ShowChirho), "Bool");
    let mut values_chirho = vec![first_chirho, second_chirho.clone()];
    let incoming_chirho = predicate_chirho(
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho),
        "Bool",
    );
    assert!(
        append_enclosing_predicate_chirho(&mut values_chirho, incoming_chirho, 1 << 25, |_| true)
            .is_none()
    );
    assert_eq!(values_chirho.len(), 2);
    assert_eq!(
        values_chirho[0].class_identity_chirho(),
        &PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho)
    );
    assert_eq!(
        values_chirho[1].class_identity_chirho(),
        second_chirho.class_identity_chirho()
    );
}

#[test]
fn enclosing_predicate_refusal_preserves_contained_duplicates_and_absent_append_order_chirho() {
    let first_chirho = predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Bool");
    let second_chirho = first_chirho.clone();
    let incoming_chirho = first_chirho.clone();
    let mut values_chirho = vec![first_chirho, second_chirho.clone()];
    let refusal_chirho =
        append_enclosing_predicate_chirho(&mut values_chirho, incoming_chirho, 0, |_| false)
            .expect("metadata refusal");
    assert_eq!(values_chirho.len(), 2);
    assert_eq!(
        values_chirho[0].class_identity_chirho(),
        &PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ResourceChirho(
            refusal_chirho
        ))
    );
    assert_eq!(
        values_chirho[1].class_identity_chirho(),
        second_chirho.class_identity_chirho()
    );
    let incoming_chirho = predicate_chirho(proof_chirho(CompilerClassChirho::EqChirho), "Char");
    let old_values_chirho = values_chirho.clone();
    let _metadata_refusal_chirho =
        append_enclosing_predicate_chirho(&mut values_chirho, incoming_chirho.clone(), 0, |_| {
            false
        });
    assert_eq!(&values_chirho[..2], &old_values_chirho);
    assert_eq!(values_chirho[2], incoming_chirho);
    assert!(matches!(
        values_chirho[2].class_identity_chirho(),
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ResourceChirho(_))
    ));
}
