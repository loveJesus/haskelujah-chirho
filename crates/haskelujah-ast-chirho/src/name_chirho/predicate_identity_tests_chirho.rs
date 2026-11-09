// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::provenance_chirho::OriginSupplyChirho;

fn unit_chirho(value_chirho: &str) -> SourceUnitIdentityChirho {
    SourceUnitIdentityChirho::from_producer_chirho(Arc::from(value_chirho))
        .expect("producer supplied a unit key")
}

#[test]
fn the_seed_class_vocabulary_has_45_distinct_display_names_chirho() {
    use CompilerClassChirho::*;
    let seeded_chirho = [
        EqChirho,
        OrdChirho,
        ShowChirho,
        NumChirho,
        FractionalChirho,
        BitsChirho,
        FiniteBitsChirho,
        FunctorChirho,
        ApplicativeChirho,
        MonadChirho,
        FoldableChirho,
        TraversableChirho,
        ReadChirho,
        IsOptionChirho,
        EnumChirho,
        BoundedChirho,
        FloatingChirho,
        RealFracChirho,
        RealFloatChirho,
        MonadIoChirho,
        MonadFailChirho,
        MonadFixChirho,
        FormatTimeChirho,
        ParseTimeChirho,
        TypeableChirho,
        DataChirho,
        KnownNatChirho,
        KnownSymbolChirho,
        KnownCharChirho,
        MonadZipChirho,
        RandomGenChirho,
        SplitGenChirho,
        ContravariantChirho,
        CoercibleChirho,
        RealChirho,
        IntegralChirho,
        NfDataChirho,
        AlternativeChirho,
        MonadPlusChirho,
        SemigroupChirho,
        MonoidChirho,
        MonadTransChirho,
        IsStringChirho,
        IsListChirho,
        GenericChirho,
    ];
    assert_eq!(seeded_chirho.len(), 45);
    assert_eq!(
        seeded_chirho
            .iter()
            .copied()
            .map(CompilerClassChirho::legacy_name_chirho)
            .collect::<std::collections::HashSet<_>>()
            .len(),
        45
    );
    assert!(!seeded_chirho.contains(&HasFieldChirho));
}

#[test]
fn witnesses_and_required_proofs_are_not_dictionary_properties_chirho() {
    for class_chirho in [
        CompilerClassChirho::KnownNatChirho,
        CompilerClassChirho::KnownSymbolChirho,
        CompilerClassChirho::KnownCharChirho,
        CompilerClassChirho::TypeableChirho,
        CompilerClassChirho::HasFieldChirho,
    ] {
        assert_eq!(
            class_chirho.evidence_chirho(),
            CompilerClassEvidenceChirho::WitnessChirho
        );
    }
    assert_eq!(
        CompilerClassChirho::CoercibleChirho.evidence_chirho(),
        CompilerClassEvidenceChirho::ProofRequiredChirho
    );
    assert_eq!(
        CompilerClassChirho::EqChirho.evidence_chirho(),
        CompilerClassEvidenceChirho::DictionaryChirho
    );
    assert_eq!(
        CompilerClassChirho::HasFieldChirho.legacy_name_chirho(),
        "HasField"
    );
}

#[test]
fn a_proof_meet_keeps_equal_identities_and_refuses_conflicts_chirho() {
    let left_chirho = PredicateClassProofChirho::ProvenChirho(
        PredicateClassIdentityChirho::CompilerChirho(CompilerClassChirho::EqChirho),
    );
    assert_eq!(
        left_chirho.clone().meet_chirho(left_chirho.clone()),
        left_chirho
    );
    assert_eq!(
        left_chirho.meet_chirho(PredicateClassProofChirho::ProvenChirho(
            PredicateClassIdentityChirho::CompilerChirho(CompilerClassChirho::OrdChirho),
        )),
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ConflictChirho)
    );
}

#[test]
fn unknown_is_absorbing_and_keeps_the_first_unknown_chirho() {
    let first_chirho =
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ForeignChirho);
    let later_chirho =
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ResourceChirho(
            PredicateMetadataRefusalChirho::WorkChirho { limit_chirho: 1 },
        ));
    let proven_chirho = PredicateClassProofChirho::ProvenChirho(
        PredicateClassIdentityChirho::CompilerChirho(CompilerClassChirho::ShowChirho),
    );
    assert_eq!(first_chirho.clone().meet_chirho(later_chirho), first_chirho);
    assert_eq!(
        first_chirho.clone().meet_chirho(proven_chirho.clone()),
        first_chirho
    );
    assert_eq!(
        proven_chirho.meet_chirho(first_chirho.clone()),
        first_chirho
    );
}

#[test]
fn source_without_a_unit_stays_unknown_chirho() {
    assert_eq!(
        PredicateClassProofChirho::source_chirho(None, Arc::from("Prelude"), Arc::from("Eq")),
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho)
    );
    assert_eq!(
        SourceUnitIdentityChirho::from_producer_chirho(Arc::from("")),
        Err(ClassIdentityUnknownChirho::MissingUnitChirho)
    );
}

#[test]
fn a_source_homonym_does_not_mint_a_compiler_identity_chirho() {
    let proof_chirho = PredicateClassProofChirho::source_chirho(
        Some(unit_chirho("actual-unit-chirho")),
        Arc::from("Prelude"),
        Arc::from("Eq"),
    );
    assert!(matches!(
        proof_chirho.identity_chirho(),
        Some(PredicateClassIdentityChirho::SourceChirho { .. })
    ));
    assert_eq!(proof_chirho.unknown_reason_chirho(), None);
    assert_eq!(
        PredicateClassProofChirho::source_chirho(
            Some(unit_chirho("actual-unit-chirho")),
            Arc::from(""),
            Arc::from("Eq")
        ),
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::WrittenChirho)
    );
}

#[test]
fn defining_units_are_retained_as_opaque_producer_keys_chirho() {
    let left_unit_chirho = unit_chirho("producer-unit-left-chirho");
    let right_unit_chirho = unit_chirho("producer-unit-right-chirho");
    assert_eq!(
        left_unit_chirho.as_str_chirho(),
        "producer-unit-left-chirho"
    );
    let left_chirho = PredicateClassProofChirho::source_chirho(
        Some(left_unit_chirho),
        Arc::from("AChirho"),
        Arc::from("CChirho"),
    );
    let right_chirho = PredicateClassProofChirho::source_chirho(
        Some(right_unit_chirho),
        Arc::from("AChirho"),
        Arc::from("CChirho"),
    );
    assert_ne!(left_chirho, right_chirho);
    assert_eq!(
        left_chirho.meet_chirho(right_chirho),
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ConflictChirho)
    );
}

#[test]
fn instance_declaration_origins_stay_distinct_within_one_module_chirho() {
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let first_origin_chirho =
        InstanceDeclarationOriginChirho::OriginChirho(supply_chirho.fresh_chirho());
    let second_origin_chirho =
        InstanceDeclarationOriginChirho::OriginChirho(supply_chirho.fresh_chirho());
    let first_chirho = InstanceIdentityChirho::source_chirho(
        Some(unit_chirho("actual-unit-chirho")),
        Arc::from("AChirho"),
        Some(first_origin_chirho),
    );
    let second_chirho = InstanceIdentityChirho::source_chirho(
        Some(unit_chirho("actual-unit-chirho")),
        Arc::from("AChirho"),
        Some(second_origin_chirho),
    );
    assert_ne!(first_chirho, second_chirho);
    assert_eq!(first_chirho.clone(), first_chirho);
    assert_eq!(
        InstanceIdentityChirho::source_chirho(
            None,
            Arc::from("AChirho"),
            Some(first_origin_chirho)
        ),
        InstanceIdentityChirho::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho)
    );
    assert_eq!(
        InstanceIdentityChirho::source_chirho(
            Some(unit_chirho("actual-unit-chirho")),
            Arc::from("AChirho"),
            Some(InstanceDeclarationOriginChirho::SpanChirho(
                SpanChirho::DUMMY_CHIRHO
            ))
        ),
        InstanceIdentityChirho::UnknownChirho(ClassIdentityUnknownChirho::WrittenChirho)
    );
}

#[test]
fn a_standalone_authority_carries_exactly_the_unshared_current_class_accounts_chirho() {
    let class_chirho = |module_chirho: &str, name_chirho: &str| {
        TypeConstructorDeclarationChirho::source_chirho(
            module_chirho,
            name_chirho,
            TypeConstructorRoleChirho::ClassChirho,
        )
    };
    let data_chirho = |module_chirho: &str, name_chirho: &str| {
        TypeConstructorDeclarationChirho::source_chirho(
            module_chirho,
            name_chirho,
            TypeConstructorRoleChirho::DataChirho,
        )
    };
    let authority_chirho = SourceUnitAuthorityChirho::from_driver_chirho(
        unit_chirho("main"),
        [
            class_chirho("Main", "ShapeChirho"),
            class_chirho("Main", "SharedChirho"),
            class_chirho("Main", "ColorChirho"),
            data_chirho("Main", "PointChirho"),
            TypeConstructorDeclarationChirho::compiler_data_chirho(
                crate::name_chirho::CompilerTypeConstructorChirho::ClassChirho(
                    CompilerClassChirho::EqChirho,
                ),
            ),
            class_chirho("", "EmptyModuleChirho"),
            class_chirho("Main", ""),
        ],
        [
            class_chirho("Main", "SharedChirho").identity_chirho,
            // An imported account under another role still names the class's identity.
            data_chirho("Main", "ColorChirho").identity_chirho,
        ],
    );
    assert_eq!(authority_chirho.unit_chirho(), &unit_chirho("main"));
    assert!(authority_chirho.admits_declaration_chirho(&class_chirho("Main", "ShapeChirho")));
    // A current account an import also names is dropped, whatever the imported role.
    assert!(!authority_chirho.admits_declaration_chirho(&class_chirho("Main", "SharedChirho")));
    assert!(!authority_chirho.admits_declaration_chirho(&class_chirho("Main", "ColorChirho")));
    // Exact accounts only: another role, module or qualifier, a compiler account or an empty
    // name is never carried.
    assert!(!authority_chirho.admits_declaration_chirho(&data_chirho("Main", "PointChirho")));
    assert!(!authority_chirho.admits_declaration_chirho(&data_chirho("Main", "ShapeChirho")));
    assert!(!authority_chirho.admits_declaration_chirho(&class_chirho("Data.Main", "ShapeChirho")));
    assert!(!authority_chirho.admits_declaration_chirho(&class_chirho("", "EmptyModuleChirho")));
    assert!(!authority_chirho.admits_declaration_chirho(&class_chirho("Main", "")));
    assert_eq!(
        authority_chirho,
        SourceUnitAuthorityChirho::from_driver_chirho(
            unit_chirho("main"),
            [class_chirho("Main", "ShapeChirho")],
            [],
        )
    );
}
