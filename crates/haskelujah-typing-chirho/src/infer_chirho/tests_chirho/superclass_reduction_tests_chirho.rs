// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The seed's proven superclass relation and the proof-aware context reduction (row 692).
//! A wanted leaves a generalized scheme only along proven compiler superclass edges at the
//! same complete arguments; an unknown or source participant stays, and no survivor's proof
//! changes. The programs it repairs are the driver's evaluation tests that use a local
//! overloaded function (`classify`, `safeDivide`, `gcd`).

use haskelujah_ast_chirho::name_chirho::{
    ClassIdentityUnknownChirho, CompilerClassChirho, PredicateClassIdentityChirho,
    PredicateClassProofChirho,
};
use haskelujah_span_chirho::SpanChirho;

use super::InferCtxChirho;
use crate::class_chirho::PredChirho;
use crate::ty_chirho::{SchemePredChirho, SchemePredPartsChirho, TyChirho, TyVarChirho};

/// Every superclass relation the seed proves, in the seed's own order. `Data`'s `Typeable`
/// superclass is not here: Typeable has a hidden kind prefix.
const SEEDED_RELATIONS_CHIRHO: [(CompilerClassChirho, &[CompilerClassChirho]); 19] = [
    (
        CompilerClassChirho::OrdChirho,
        &[CompilerClassChirho::EqChirho],
    ),
    (
        CompilerClassChirho::FractionalChirho,
        &[CompilerClassChirho::NumChirho],
    ),
    (
        CompilerClassChirho::FiniteBitsChirho,
        &[CompilerClassChirho::BitsChirho],
    ),
    (
        CompilerClassChirho::ApplicativeChirho,
        &[CompilerClassChirho::FunctorChirho],
    ),
    (
        CompilerClassChirho::MonadChirho,
        &[CompilerClassChirho::ApplicativeChirho],
    ),
    (
        CompilerClassChirho::TraversableChirho,
        &[
            CompilerClassChirho::FunctorChirho,
            CompilerClassChirho::FoldableChirho,
        ],
    ),
    (
        CompilerClassChirho::FloatingChirho,
        &[CompilerClassChirho::FractionalChirho],
    ),
    (
        CompilerClassChirho::RealFracChirho,
        &[
            CompilerClassChirho::RealChirho,
            CompilerClassChirho::FractionalChirho,
        ],
    ),
    (
        CompilerClassChirho::RealFloatChirho,
        &[
            CompilerClassChirho::RealFracChirho,
            CompilerClassChirho::FloatingChirho,
        ],
    ),
    (
        CompilerClassChirho::MonadIoChirho,
        &[CompilerClassChirho::MonadChirho],
    ),
    (
        CompilerClassChirho::MonadFailChirho,
        &[CompilerClassChirho::MonadChirho],
    ),
    (
        CompilerClassChirho::MonadFixChirho,
        &[CompilerClassChirho::MonadChirho],
    ),
    (
        CompilerClassChirho::MonadZipChirho,
        &[CompilerClassChirho::MonadChirho],
    ),
    (
        CompilerClassChirho::SplitGenChirho,
        &[CompilerClassChirho::RandomGenChirho],
    ),
    (
        CompilerClassChirho::RealChirho,
        &[
            CompilerClassChirho::NumChirho,
            CompilerClassChirho::OrdChirho,
        ],
    ),
    (
        CompilerClassChirho::IntegralChirho,
        &[
            CompilerClassChirho::RealChirho,
            CompilerClassChirho::EnumChirho,
        ],
    ),
    (
        CompilerClassChirho::AlternativeChirho,
        &[CompilerClassChirho::ApplicativeChirho],
    ),
    (
        CompilerClassChirho::MonadPlusChirho,
        &[
            CompilerClassChirho::MonadChirho,
            CompilerClassChirho::AlternativeChirho,
        ],
    ),
    (
        CompilerClassChirho::MonoidChirho,
        &[CompilerClassChirho::SemigroupChirho],
    ),
];

fn proven_chirho(identity_chirho: CompilerClassChirho) -> PredicateClassProofChirho {
    PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::CompilerChirho(
        identity_chirho,
    ))
}

fn parts_chirho(class_name_chirho: &str, ty_chirho: &TyChirho) -> SchemePredPartsChirho {
    SchemePredPartsChirho {
        kind_tys_chirho: Vec::new(),
        class_name_chirho: class_name_chirho.to_string(),
        ty_chirho: ty_chirho.clone(),
        extra_tys_chirho: Vec::new(),
    }
}

fn compiler_predicate_chirho(
    identity_chirho: CompilerClassChirho,
    ty_chirho: &TyChirho,
) -> SchemePredChirho {
    SchemePredChirho::from_compiler_chirho(
        identity_chirho,
        parts_chirho(identity_chirho.legacy_name_chirho(), ty_chirho),
    )
}

fn variable_chirho(index_chirho: u32) -> TyChirho {
    TyChirho::VarChirho(TyVarChirho(index_chirho))
}

#[test]
fn every_seeded_superclass_relation_is_a_proven_unary_template_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    for (class_chirho, superclasses_chirho) in SEEDED_RELATIONS_CHIRHO {
        let record_chirho = ctx_chirho
            .class_env_chirho
            .compiler_class_record_chirho(class_chirho)
            .unwrap_or_else(|| panic!("{class_chirho:?} is seeded"));
        assert!(
            record_chirho.kind_vars_chirho.is_empty() && !record_chirho.is_mptc_chirho(),
            "{class_chirho:?} is unary with no kind prefix"
        );
        let legacy_chirho: Vec<&str> = superclasses_chirho
            .iter()
            .map(|superclass_chirho| superclass_chirho.legacy_name_chirho())
            .collect();
        assert_eq!(
            record_chirho.supers_chirho, legacy_chirho,
            "{class_chirho:?}"
        );
        let templates_chirho = record_chirho.superclass_templates_chirho();
        assert_eq!(templates_chirho.len(), superclasses_chirho.len());
        for (template_chirho, superclass_chirho) in templates_chirho.iter().zip(superclasses_chirho)
        {
            assert_eq!(
                template_chirho.class_identity_chirho(),
                &proven_chirho(*superclass_chirho)
            );
            assert_eq!(
                template_chirho.class_name_chirho,
                superclass_chirho.legacy_name_chirho()
            );
            assert_eq!(
                template_chirho.ty_chirho,
                TyChirho::VarChirho(record_chirho.var_chirho)
            );
            assert!(
                template_chirho.kind_tys_chirho.is_empty()
                    && template_chirho.extra_tys_chirho.is_empty()
            );
            let superclass_record_chirho = ctx_chirho
                .class_env_chirho
                .compiler_class_record_chirho(*superclass_chirho)
                .unwrap_or_else(|| panic!("{superclass_chirho:?} is seeded"));
            assert!(
                superclass_record_chirho.kind_vars_chirho.is_empty()
                    && !superclass_record_chirho.is_mptc_chirho(),
                "{superclass_chirho:?} is unary with no kind prefix"
            );
        }
    }
}

#[test]
fn the_data_typeable_relation_stays_a_legacy_spelling_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    let typeable_chirho = ctx_chirho
        .class_env_chirho
        .compiler_class_record_chirho(CompilerClassChirho::TypeableChirho)
        .expect("Typeable is seeded");
    assert!(!typeable_chirho.kind_vars_chirho.is_empty());
    let data_chirho = ctx_chirho
        .class_env_chirho
        .compiler_class_record_chirho(CompilerClassChirho::DataChirho)
        .expect("Data is seeded");
    assert_eq!(data_chirho.supers_chirho, vec!["Typeable".to_string()]);
    assert!(data_chirho.super_templates_chirho.is_empty());
    let a_chirho = variable_chirho(9_100);
    assert!(!ctx_chirho.compiler_superclass_entails_chirho(
        &compiler_predicate_chirho(CompilerClassChirho::DataChirho, &a_chirho),
        &compiler_predicate_chirho(CompilerClassChirho::TypeableChirho, &a_chirho),
    ));
}

#[test]
fn a_proven_class_entails_its_superclasses_at_the_same_arguments_only_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    let a_chirho = variable_chirho(9_101);
    let b_chirho = variable_chirho(9_102);
    for (given_chirho, wanted_chirho) in [
        (
            CompilerClassChirho::OrdChirho,
            CompilerClassChirho::EqChirho,
        ),
        // Through Real.
        (
            CompilerClassChirho::IntegralChirho,
            CompilerClassChirho::NumChirho,
        ),
        // Through Real and Ord.
        (
            CompilerClassChirho::IntegralChirho,
            CompilerClassChirho::EqChirho,
        ),
        (
            CompilerClassChirho::RealFloatChirho,
            CompilerClassChirho::FractionalChirho,
        ),
        // Through Applicative.
        (
            CompilerClassChirho::MonadChirho,
            CompilerClassChirho::FunctorChirho,
        ),
    ] {
        let given_at_a_chirho = compiler_predicate_chirho(given_chirho, &a_chirho);
        let wanted_at_a_chirho = compiler_predicate_chirho(wanted_chirho, &a_chirho);
        assert!(
            ctx_chirho.compiler_superclass_entails_chirho(&given_at_a_chirho, &wanted_at_a_chirho),
            "{given_chirho:?} entails {wanted_chirho:?}"
        );
        assert!(
            !ctx_chirho.compiler_superclass_entails_chirho(&wanted_at_a_chirho, &given_at_a_chirho),
            "{wanted_chirho:?} never entails its subclass {given_chirho:?}"
        );
        assert!(
            !ctx_chirho.compiler_superclass_entails_chirho(
                &given_at_a_chirho,
                &compiler_predicate_chirho(wanted_chirho, &b_chirho)
            ),
            "{given_chirho:?} at one argument says nothing at another"
        );
    }
    // Num has no superclass in the seed.
    assert!(!ctx_chirho.compiler_superclass_entails_chirho(
        &compiler_predicate_chirho(CompilerClassChirho::NumChirho, &a_chirho),
        &compiler_predicate_chirho(CompilerClassChirho::EqChirho, &a_chirho),
    ));
}

#[test]
fn an_unproven_participant_neither_leaves_nor_entails_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    let a_chirho = variable_chirho(9_103);
    let ord_chirho = compiler_predicate_chirho(CompilerClassChirho::OrdChirho, &a_chirho);
    let eq_chirho = compiler_predicate_chirho(CompilerClassChirho::EqChirho, &a_chirho);
    let legacy_chirho =
        |name_chirho: &str| SchemePredChirho::legacy_chirho(parts_chirho(name_chirho, &a_chirho));
    let source_eq_chirho = SchemePredChirho::from_proof_chirho(
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho),
        parts_chirho("Eq", &a_chirho),
    );
    assert!(!ctx_chirho.compiler_superclass_entails_chirho(&ord_chirho, &legacy_chirho("Eq")));
    assert!(!ctx_chirho.compiler_superclass_entails_chirho(&ord_chirho, &source_eq_chirho));
    assert!(!ctx_chirho.compiler_superclass_entails_chirho(&legacy_chirho("Ord"), &eq_chirho));
}

#[test]
fn generalization_removes_only_proven_entailed_wanteds_and_keeps_every_proof_chirho() {
    let a_chirho = variable_chirho(9_104);
    let b_chirho = variable_chirho(9_105);
    let ty_chirho = TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone());
    let generalized_chirho = |wanteds_chirho: Vec<PredChirho>| {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.deferred_preds_chirho = wanteds_chirho
            .into_iter()
            .map(|wanted_chirho| (wanted_chirho, SpanChirho::DUMMY_CHIRHO))
            .collect();
        ctx_chirho
            .generalize_chirho(&ty_chirho)
            .preds_chirho
            .into_iter()
            .map(|kept_chirho| {
                (
                    kept_chirho.class_name_chirho.clone(),
                    kept_chirho.class_identity_chirho().clone(),
                )
            })
            .collect::<Vec<_>>()
    };
    // `Eq a` leaves through `Ord a`; the unrelated `Show b` keeps its own proof.
    assert_eq!(
        generalized_chirho(vec![
            PredChirho::compiler_chirho(CompilerClassChirho::EqChirho, a_chirho.clone()),
            PredChirho::compiler_chirho(CompilerClassChirho::OrdChirho, a_chirho.clone()),
            PredChirho::compiler_chirho(CompilerClassChirho::ShowChirho, b_chirho.clone()),
        ]),
        vec![
            (
                "Ord".to_string(),
                proven_chirho(CompilerClassChirho::OrdChirho)
            ),
            (
                "Show".to_string(),
                proven_chirho(CompilerClassChirho::ShowChirho)
            ),
        ]
    );
    // An unproven `Eq a` is not removed by its spelling, and `Ord a` stays proven.
    assert_eq!(
        generalized_chirho(vec![
            PredChirho::new_chirho("Eq", a_chirho.clone()),
            PredChirho::compiler_chirho(CompilerClassChirho::OrdChirho, a_chirho.clone()),
        ]),
        vec![
            (
                "Eq".to_string(),
                PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho)
            ),
            (
                "Ord".to_string(),
                proven_chirho(CompilerClassChirho::OrdChirho)
            ),
        ]
    );
}
