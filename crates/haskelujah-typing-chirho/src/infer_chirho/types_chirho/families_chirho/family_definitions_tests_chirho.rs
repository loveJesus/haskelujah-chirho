// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn source_id_chirho(module_chirho: &str, name_chirho: &str) -> TypeFamilyIdChirho {
    TypeFamilyIdChirho::SourceChirho {
        module_chirho: module_chirho.to_owned(),
        name_chirho: name_chirho.to_owned(),
    }
}

fn branch_chirho(
    family_chirho: &TypeFamilyIdChirho,
    producer_chirho: &str,
    offset_chirho: u32,
    input_chirho: &str,
    output_chirho: &str,
) -> TypeFamilyBranchChirho {
    TypeFamilyBranchChirho {
        id_chirho: TypeFamilyBranchIdChirho {
            family_chirho: family_chirho.clone(),
            producer_module_chirho: producer_chirho.to_owned(),
            declaration_start_chirho: offset_chirho,
            equation_index_chirho: 0,
        },
        binders_chirho: Vec::new(),
        lhs_chirho: vec![TyChirho::ConChirho(input_chirho.to_owned())],
        rhs_chirho: TyChirho::ConChirho(output_chirho.to_owned()),
        blockers_chirho: Vec::new(),
        held_chirho: false,
    }
}

fn definition_chirho(
    id_chirho: &TypeFamilyIdChirho,
    reduction_chirho: TypeFamilyReductionChirho,
    branches_chirho: Vec<TypeFamilyBranchChirho>,
) -> TypeFamilyDefinitionChirho {
    TypeFamilyDefinitionChirho::new_chirho(id_chirho.clone(), 1, reduction_chirho, branches_chirho)
        .unwrap()
}

#[test]
fn source_and_compiler_identities_do_not_collide_chirho() {
    let source_chirho = source_id_chirho("GHC.TypeNats", "+");
    let seed_chirho = TypeFamilyIdChirho::CompilerSeedChirho {
        module_chirho: "GHC.TypeNats".to_owned(),
        name_chirho: "+".to_owned(),
    };
    let intrinsic_chirho =
        TypeFamilyIdChirho::IntrinsicChirho(IntrinsicTypeFamilyChirho::AddNatChirho);
    assert_ne!(source_chirho.key_chirho(), seed_chirho.key_chirho());
    assert_ne!(source_chirho.key_chirho(), intrinsic_chirho.key_chirho());
    assert_ne!(seed_chirho.key_chirho(), intrinsic_chirho.key_chirho());
    assert_ne!(
        source_id_chirho("A", "B.C").key_chirho(),
        source_id_chirho("A.B", "C").key_chirho()
    );
}

#[test]
fn closed_export_paths_share_one_definition_and_order_chirho() {
    let id_chirho = source_id_chirho("Origin", "F");
    let first_chirho = branch_chirho(&id_chirho, "Origin", 10, "Int", "Bool");
    let mut second_chirho = branch_chirho(&id_chirho, "Origin", 10, "Char", "Int");
    second_chirho.id_chirho.equation_index_chirho = 1;
    second_chirho.blockers_chirho.push(0);
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    registry_chirho
        .insert_chirho(definition_chirho(
            &id_chirho,
            TypeFamilyReductionChirho::ClosedChirho,
            vec![first_chirho, second_chirho],
        ))
        .unwrap();
    registry_chirho
        .export_alias_chirho("Origin.F".to_owned(), &id_chirho)
        .unwrap();
    registry_chirho
        .export_alias_chirho("Forwarder.F".to_owned(), &id_chirho)
        .unwrap();
    let TypeFamilyAliasResolutionChirho::UniqueChirho(origin_chirho) =
        registry_chirho.resolve_export_chirho("Origin.F")
    else {
        panic!("origin must resolve")
    };
    let TypeFamilyAliasResolutionChirho::UniqueChirho(forwarder_chirho) =
        registry_chirho.resolve_export_chirho("Forwarder.F")
    else {
        panic!("forwarder must resolve")
    };
    assert!(Arc::ptr_eq(&origin_chirho, &forwarder_chirho));
    assert_eq!(
        forwarder_chirho.branches_chirho()[0].rhs_chirho,
        TyChirho::bool_chirho()
    );
    assert_eq!(
        forwarder_chirho.branches_chirho()[1].blockers_chirho,
        vec![0]
    );
    let snapshot_chirho = registry_chirho.clone();
    for _repeat_chirho in 0..100 {
        registry_chirho.merge_chirho(&snapshot_chirho).unwrap();
    }
    assert_eq!(registry_chirho.definition_count_chirho(), 1);
    assert_eq!(
        registry_chirho
            .definition_chirho(&id_chirho.key_chirho())
            .unwrap()
            .branches_chirho()
            .len(),
        2
    );
}

#[test]
fn aliases_to_distinct_families_remain_ambiguous_chirho() {
    let first_chirho = source_id_chirho("First", "F");
    let second_chirho = source_id_chirho("Second", "F");
    let mut left_chirho = TypeFamilyRegistryChirho::new_chirho();
    let mut right_chirho = TypeFamilyRegistryChirho::new_chirho();
    left_chirho
        .insert_chirho(definition_chirho(
            &first_chirho,
            TypeFamilyReductionChirho::ClosedChirho,
            Vec::new(),
        ))
        .unwrap();
    right_chirho
        .insert_chirho(definition_chirho(
            &second_chirho,
            TypeFamilyReductionChirho::ClosedChirho,
            Vec::new(),
        ))
        .unwrap();
    left_chirho
        .export_alias_chirho("Forwarder.F".to_owned(), &first_chirho)
        .unwrap();
    right_chirho
        .export_alias_chirho("Forwarder.F".to_owned(), &second_chirho)
        .unwrap();
    left_chirho.merge_chirho(&right_chirho).unwrap();
    let TypeFamilyAliasResolutionChirho::AmbiguousChirho(ids_chirho) =
        left_chirho.resolve_export_chirho("Forwarder.F")
    else {
        panic!("two origins cannot become one")
    };
    assert_eq!(
        ids_chirho.into_iter().collect::<BTreeSet<_>>(),
        BTreeSet::from([first_chirho, second_chirho])
    );
}

#[test]
fn open_branch_union_is_bounded_under_cumulative_reimports_chirho() {
    let id_chirho = source_id_chirho("Definition", "F");
    let first_chirho = branch_chirho(&id_chirho, "InstanceA", 10, "Int", "Bool");
    let second_chirho = branch_chirho(&id_chirho, "InstanceB", 10, "Char", "Int");
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    registry_chirho
        .insert_chirho(definition_chirho(
            &id_chirho,
            TypeFamilyReductionChirho::OpenChirho,
            vec![first_chirho.clone()],
        ))
        .unwrap();
    let mut cumulative_chirho = TypeFamilyRegistryChirho::new_chirho();
    cumulative_chirho
        .insert_chirho(definition_chirho(
            &id_chirho,
            TypeFamilyReductionChirho::OpenChirho,
            vec![first_chirho, second_chirho],
        ))
        .unwrap();
    for _repeat_chirho in 0..100 {
        registry_chirho.merge_chirho(&cumulative_chirho).unwrap();
    }
    let branches_chirho = registry_chirho
        .definition_chirho(&id_chirho.key_chirho())
        .unwrap()
        .branches_chirho();
    assert_eq!(branches_chirho.len(), 2);
    assert_eq!(
        branches_chirho[0].id_chirho.producer_module_chirho,
        "InstanceA"
    );
    assert_eq!(
        branches_chirho[1].id_chirho.producer_module_chirho,
        "InstanceB"
    );
}

#[test]
fn conflicting_snapshot_merge_preserves_the_existing_registry_chirho() {
    let id_chirho = source_id_chirho("Definition", "F");
    let first_chirho = branch_chirho(&id_chirho, "Instance", 10, "Int", "Bool");
    let mut changed_chirho = first_chirho.clone();
    changed_chirho.rhs_chirho = TyChirho::char_chirho();
    let other_id_chirho = source_id_chirho("Other", "G");
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    registry_chirho
        .insert_chirho(definition_chirho(
            &id_chirho,
            TypeFamilyReductionChirho::OpenChirho,
            vec![first_chirho.clone()],
        ))
        .unwrap();
    let mut incoming_chirho = TypeFamilyRegistryChirho::new_chirho();
    incoming_chirho
        .insert_chirho(definition_chirho(
            &id_chirho,
            TypeFamilyReductionChirho::OpenChirho,
            vec![changed_chirho],
        ))
        .unwrap();
    incoming_chirho
        .insert_chirho(definition_chirho(
            &other_id_chirho,
            TypeFamilyReductionChirho::OpenChirho,
            Vec::new(),
        ))
        .unwrap();
    incoming_chirho
        .export_alias_chirho("Incoming.F".to_owned(), &id_chirho)
        .unwrap();
    assert!(registry_chirho.merge_chirho(&incoming_chirho).is_err());
    assert_eq!(registry_chirho.definition_count_chirho(), 1);
    assert_eq!(
        registry_chirho
            .definition_chirho(&id_chirho.key_chirho())
            .unwrap()
            .branches_chirho(),
        &[first_chirho]
    );
    assert!(matches!(
        registry_chirho.resolve_export_chirho("Incoming.F"),
        TypeFamilyAliasResolutionChirho::MissingChirho
    ));
}

#[test]
fn immutable_closed_snapshots_never_concatenate_chirho() {
    let id_chirho = source_id_chirho("Definition", "F");
    let first_chirho = branch_chirho(&id_chirho, "Definition", 10, "Int", "Bool");
    let mut second_chirho = branch_chirho(&id_chirho, "Definition", 10, "Char", "Int");
    second_chirho.id_chirho.equation_index_chirho = 1;
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    registry_chirho
        .insert_chirho(definition_chirho(
            &id_chirho,
            TypeFamilyReductionChirho::ClosedChirho,
            vec![first_chirho.clone()],
        ))
        .unwrap();
    assert!(
        registry_chirho
            .insert_chirho(definition_chirho(
                &id_chirho,
                TypeFamilyReductionChirho::ClosedChirho,
                vec![first_chirho, second_chirho]
            ))
            .is_err()
    );
    assert_eq!(
        registry_chirho
            .definition_chirho(&id_chirho.key_chirho())
            .unwrap()
            .branches_chirho()
            .len(),
        1
    );
}

#[test]
fn arity_and_blocker_ownership_are_validated_before_registration_chirho() {
    let id_chirho = source_id_chirho("Definition", "F");
    let mut invalid_chirho = branch_chirho(&id_chirho, "Definition", 10, "Int", "Bool");
    invalid_chirho.blockers_chirho.push(0);
    assert!(
        TypeFamilyDefinitionChirho::new_chirho(
            id_chirho.clone(),
            1,
            TypeFamilyReductionChirho::ClosedChirho,
            vec![invalid_chirho]
        )
        .is_err()
    );
    let valid_chirho = branch_chirho(&id_chirho, "Definition", 10, "Int", "Bool");
    assert!(
        TypeFamilyDefinitionChirho::new_chirho(
            id_chirho.clone(),
            2,
            TypeFamilyReductionChirho::ClosedChirho,
            vec![valid_chirho.clone()]
        )
        .is_err()
    );
    assert!(
        TypeFamilyDefinitionChirho::new_chirho(
            source_id_chirho("Other", "F"),
            1,
            TypeFamilyReductionChirho::ClosedChirho,
            vec![valid_chirho]
        )
        .is_err()
    );
}

#[test]
fn empty_families_are_real_definitions_and_absent_targets_cannot_be_exported_chirho() {
    let id_chirho = source_id_chirho("Local", "+");
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    registry_chirho
        .insert_chirho(
            TypeFamilyDefinitionChirho::new_chirho(
                id_chirho.clone(),
                2,
                TypeFamilyReductionChirho::ClosedChirho,
                Vec::new(),
            )
            .unwrap(),
        )
        .unwrap();
    registry_chirho
        .export_alias_chirho("Local.+".to_owned(), &id_chirho)
        .unwrap();
    let TypeFamilyAliasResolutionChirho::UniqueChirho(definition_chirho) =
        registry_chirho.resolve_export_chirho("Local.+")
    else {
        panic!("empty declaration must resolve")
    };
    assert_eq!(definition_chirho.arity_chirho(), 2);
    assert_eq!(
        definition_chirho.reduction_chirho(),
        TypeFamilyReductionChirho::ClosedChirho
    );
    assert!(definition_chirho.branches_chirho().is_empty());
    assert!(
        registry_chirho
            .export_alias_chirho("Missing.F".to_owned(), &source_id_chirho("Missing", "F"))
            .is_err()
    );
}

#[test]
fn intrinsic_reduction_requires_its_own_identity_and_arity_chirho() {
    let intrinsic_chirho = IntrinsicTypeFamilyChirho::AddNatChirho;
    let intrinsic_id_chirho = TypeFamilyIdChirho::IntrinsicChirho(intrinsic_chirho);
    assert!(
        TypeFamilyDefinitionChirho::new_chirho(
            source_id_chirho("Local", "+"),
            2,
            TypeFamilyReductionChirho::IntrinsicChirho(intrinsic_chirho),
            Vec::new()
        )
        .is_err()
    );
    assert!(
        TypeFamilyDefinitionChirho::new_chirho(
            intrinsic_id_chirho.clone(),
            1,
            TypeFamilyReductionChirho::IntrinsicChirho(intrinsic_chirho),
            Vec::new()
        )
        .is_err()
    );
    assert!(
        TypeFamilyDefinitionChirho::new_chirho(
            intrinsic_id_chirho,
            2,
            TypeFamilyReductionChirho::IntrinsicChirho(intrinsic_chirho),
            Vec::new()
        )
        .is_ok()
    );
}

#[test]
fn closed_branches_cannot_join_different_declarations_chirho() {
    let id_chirho = source_id_chirho("Definition", "F");
    let first_chirho = branch_chirho(&id_chirho, "Definition", 10, "Int", "Bool");
    for (producer_chirho, offset_chirho) in [("Other", 10), ("Definition", 20)] {
        let mut second_chirho =
            branch_chirho(&id_chirho, producer_chirho, offset_chirho, "Char", "Int");
        second_chirho.id_chirho.equation_index_chirho = 1;
        assert!(
            TypeFamilyDefinitionChirho::new_chirho(
                id_chirho.clone(),
                1,
                TypeFamilyReductionChirho::ClosedChirho,
                vec![first_chirho.clone(), second_chirho]
            )
            .is_err()
        );
    }
    assert!(
        TypeFamilyDefinitionChirho::new_chirho(
            id_chirho.clone(),
            1,
            TypeFamilyReductionChirho::ClosedChirho,
            vec![branch_chirho(&id_chirho, "Other", 10, "Int", "Bool")]
        )
        .is_err()
    );
}

#[test]
fn held_branch_stops_later_reduction_without_copied_blocker_lists_chirho() {
    let id_chirho = source_id_chirho("Definition", "F");
    let mut first_chirho = branch_chirho(&id_chirho, "Definition", 10, "Int", "Char");
    first_chirho.held_chirho = true;
    let mut second_chirho = branch_chirho(&id_chirho, "Definition", 10, "Char", "Bool");
    second_chirho.id_chirho.equation_index_chirho = 1;
    let definition_chirho = definition_chirho(
        &id_chirho,
        TypeFamilyReductionChirho::ClosedChirho,
        vec![first_chirho, second_chirho],
    );
    assert_eq!(definition_chirho.reduction_candidates_chirho().count(), 0);
    assert!(
        definition_chirho.branches_chirho()[1]
            .blockers_chirho
            .is_empty()
    );
}

#[test]
fn republication_replaces_a_reexports_origin_instead_of_creating_ambiguity_chirho() {
    let first_chirho = source_id_chirho("First", "F");
    let second_chirho = source_id_chirho("Second", "F");
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    for id_chirho in [&first_chirho, &second_chirho] {
        registry_chirho
            .insert_chirho(definition_chirho(
                id_chirho,
                TypeFamilyReductionChirho::ClosedChirho,
                Vec::new(),
            ))
            .unwrap();
    }
    registry_chirho
        .replace_module_exports_chirho(
            "Forwarder",
            &[("Forwarder.F".to_owned(), BTreeSet::from([first_chirho]))],
        )
        .unwrap();
    registry_chirho
        .replace_module_exports_chirho(
            "Forwarder",
            &[(
                "Forwarder.F".to_owned(),
                BTreeSet::from([second_chirho.clone()]),
            )],
        )
        .unwrap();
    let TypeFamilyAliasResolutionChirho::UniqueChirho(definition_chirho) =
        registry_chirho.resolve_export_chirho("Forwarder.F")
    else {
        panic!("replacement must have exactly its new origin")
    };
    assert_eq!(definition_chirho.id_chirho(), &second_chirho);
}

#[test]
fn removed_exports_retire_without_clearing_a_nested_modules_exports_chirho() {
    let id_chirho = source_id_chirho("Origin", "F");
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    registry_chirho
        .insert_chirho(definition_chirho(
            &id_chirho,
            TypeFamilyReductionChirho::ClosedChirho,
            Vec::new(),
        ))
        .unwrap();
    registry_chirho
        .replace_module_exports_chirho(
            "Forwarder",
            &[(
                "Forwarder.F".to_owned(),
                BTreeSet::from([id_chirho.clone()]),
            )],
        )
        .unwrap();
    registry_chirho
        .replace_module_exports_chirho(
            "Forwarder.Inner",
            &[("Forwarder.Inner.F".to_owned(), BTreeSet::from([id_chirho]))],
        )
        .unwrap();
    registry_chirho
        .replace_module_exports_chirho("Forwarder", &[])
        .unwrap();
    assert!(matches!(
        registry_chirho.resolve_export_chirho("Forwarder.F"),
        TypeFamilyAliasResolutionChirho::MissingChirho
    ));
    assert!(matches!(
        registry_chirho.resolve_export_chirho("Forwarder.Inner.F"),
        TypeFamilyAliasResolutionChirho::UniqueChirho(_)
    ));
}

#[test]
fn invalid_republication_is_atomic_and_an_ordinary_type_clears_an_alias_chirho() {
    let id_chirho = source_id_chirho("Origin", "F");
    let missing_chirho = source_id_chirho("Missing", "F");
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    registry_chirho
        .insert_chirho(definition_chirho(
            &id_chirho,
            TypeFamilyReductionChirho::ClosedChirho,
            Vec::new(),
        ))
        .unwrap();
    registry_chirho
        .replace_module_exports_chirho(
            "Forwarder",
            &[("Forwarder.F".to_owned(), BTreeSet::from([id_chirho]))],
        )
        .unwrap();
    assert!(
        registry_chirho
            .replace_module_exports_chirho(
                "Forwarder",
                &[
                    ("Forwarder.F".to_owned(), BTreeSet::new()),
                    ("Forwarder.G".to_owned(), BTreeSet::from([missing_chirho]))
                ]
            )
            .is_err()
    );
    assert!(matches!(
        registry_chirho.resolve_export_chirho("Forwarder.F"),
        TypeFamilyAliasResolutionChirho::UniqueChirho(_)
    ));
    assert!(matches!(
        registry_chirho.resolve_export_chirho("Forwarder.G"),
        TypeFamilyAliasResolutionChirho::MissingChirho
    ));
    registry_chirho
        .replace_module_exports_chirho("Forwarder", &[("Forwarder.F".to_owned(), BTreeSet::new())])
        .unwrap();
    assert!(matches!(
        registry_chirho.resolve_export_chirho("Forwarder.F"),
        TypeFamilyAliasResolutionChirho::MissingChirho
    ));
}

#[test]
fn imported_hidden_type_declarations_cannot_borrow_builtin_comparison_authority_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    registry_chirho
        .constructor_origins_mut_chirho()
        .insert_compiler_chirho("Int".to_owned(), "GHC.Types.Int".to_owned());
    assert!(
        registry_chirho
            .constructor_origins_chirho()
            .identity_chirho("Int")
            .is_some()
    );
    let mut incoming_chirho = TypeFamilyRegistryChirho::new_chirho();
    incoming_chirho
        .constructor_origins_mut_chirho()
        .replace_source_module_chirho("Hidden", &[("Int".to_owned(), true)])
        .unwrap();
    assert!(!incoming_chirho.is_empty_chirho());
    let hidden_identity_chirho = incoming_chirho
        .constructor_origins_chirho()
        .identity_chirho("Int")
        .unwrap();
    registry_chirho.merge_chirho(&incoming_chirho).unwrap();
    assert_eq!(
        registry_chirho
            .constructor_origins_chirho()
            .identity_chirho("Int"),
        None
    );
    assert!(
        registry_chirho
            .constructor_origins_chirho()
            .identity_chirho(&hidden_identity_chirho)
            .is_some()
    );
}

#[test]
fn open_overlap_requires_a_bounded_compatibility_certificate_chirho() {
    let id_chirho = source_id_chirho("Definition", "F");
    let first_chirho = branch_chirho(&id_chirho, "InstanceA", 10, "Int", "Bool");
    let second_chirho = branch_chirho(&id_chirho, "InstanceB", 10, "Int", "Char");
    let definition_chirho = definition_chirho(
        &id_chirho,
        TypeFamilyReductionChirho::OpenChirho,
        vec![first_chirho, second_chirho],
    );
    assert!(!definition_chirho.open_coherence_verified_chirho());
    assert!(
        definition_chirho
            .verify_open_coherence_chirho(&|_left_chirho, _right_chirho| false, 1)
            .is_err()
    );
    assert!(
        definition_chirho
            .verify_open_coherence_chirho(&|_left_chirho, _right_chirho| true, 0)
            .is_err()
    );
}

#[test]
fn merging_new_open_rows_invalidates_a_prior_certificate_chirho() {
    let id_chirho = source_id_chirho("Definition", "F");
    let first_chirho = branch_chirho(&id_chirho, "InstanceA", 10, "Int", "Bool");
    let second_chirho = branch_chirho(&id_chirho, "InstanceB", 10, "Char", "Int");
    let verified_chirho = definition_chirho(
        &id_chirho,
        TypeFamilyReductionChirho::OpenChirho,
        vec![first_chirho],
    )
    .verify_open_coherence_chirho(&|_left_chirho, _right_chirho| true, 0)
    .unwrap();
    assert!(verified_chirho.open_coherence_verified_chirho());
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    registry_chirho.insert_chirho(verified_chirho).unwrap();
    registry_chirho
        .insert_chirho(definition_chirho(
            &id_chirho,
            TypeFamilyReductionChirho::OpenChirho,
            vec![second_chirho],
        ))
        .unwrap();
    assert!(
        !registry_chirho
            .definition_chirho(&id_chirho.key_chirho())
            .unwrap()
            .open_coherence_verified_chirho()
    );
}

#[test]
fn verifying_the_current_open_snapshot_updates_the_stored_certificate_chirho() {
    let id_chirho = source_id_chirho("Definition", "F");
    let branch_chirho = branch_chirho(&id_chirho, "InstanceA", 10, "Int", "Bool");
    let unverified_chirho = definition_chirho(
        &id_chirho,
        TypeFamilyReductionChirho::OpenChirho,
        vec![branch_chirho],
    );
    let verified_chirho = unverified_chirho
        .verify_open_coherence_chirho(&|_left_chirho, _right_chirho| true, 0)
        .unwrap();
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    registry_chirho
        .insert_chirho(unverified_chirho.clone())
        .unwrap();
    registry_chirho.insert_chirho(verified_chirho).unwrap();
    assert!(
        registry_chirho
            .definition_chirho(&id_chirho.key_chirho())
            .unwrap()
            .open_coherence_verified_chirho()
    );
    registry_chirho.insert_chirho(unverified_chirho).unwrap();
    assert!(
        registry_chirho
            .definition_chirho(&id_chirho.key_chirho())
            .unwrap()
            .open_coherence_verified_chirho(),
        "an older duplicate must not erase the proof"
    );
}

#[test]
fn importing_a_verified_superset_keeps_its_exact_snapshot_certificate_chirho() {
    let id_chirho = source_id_chirho("Definition", "F");
    let first_chirho = branch_chirho(&id_chirho, "InstanceA", 10, "Int", "Bool");
    let second_chirho = branch_chirho(&id_chirho, "InstanceB", 10, "Char", "Int");
    let subset_chirho = definition_chirho(
        &id_chirho,
        TypeFamilyReductionChirho::OpenChirho,
        vec![first_chirho.clone()],
    );
    let verified_chirho = definition_chirho(
        &id_chirho,
        TypeFamilyReductionChirho::OpenChirho,
        vec![first_chirho, second_chirho],
    )
    .verify_open_coherence_chirho(&|_left_chirho, _right_chirho| true, 1)
    .unwrap();
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    registry_chirho
        .insert_chirho(subset_chirho.clone())
        .unwrap();
    let mut incoming_chirho = TypeFamilyRegistryChirho::new_chirho();
    incoming_chirho.insert_chirho(verified_chirho).unwrap();
    registry_chirho.merge_chirho(&incoming_chirho).unwrap();
    registry_chirho.insert_chirho(subset_chirho).unwrap();
    let final_chirho = registry_chirho
        .definition_chirho(&id_chirho.key_chirho())
        .unwrap();
    assert_eq!(final_chirho.branches_chirho().len(), 2);
    assert!(final_chirho.open_coherence_verified_chirho());
}

#[test]
fn diagnostics_render_family_names_without_changing_internal_keys_chirho() {
    let left_chirho = source_id_chirho("LeftChirho", "FChirho");
    let right_chirho = source_id_chirho("RightChirho", "FChirho");
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    for id_chirho in [&left_chirho, &right_chirho] {
        registry_chirho
            .insert_chirho(definition_chirho(
                id_chirho,
                TypeFamilyReductionChirho::ClosedChirho,
                Vec::new(),
            ))
            .unwrap();
    }
    let before_chirho = format!(
        "expected `{} Int`, found `{} Bool`",
        left_chirho.key_chirho(),
        right_chirho.key_chirho()
    );
    assert_eq!(
        registry_chirho.readable_keys_chirho(&before_chirho),
        "expected `LeftChirho.FChirho Int`, found `RightChirho.FChirho Bool`"
    );
    assert!(
        registry_chirho
            .definition_chirho(&left_chirho.key_chirho())
            .is_some()
    );
    assert!(
        registry_chirho
            .definition_chirho("LeftChirho.FChirho")
            .is_none()
    );
    assert_eq!(
        registry_chirho.readable_keys_chirho("%family:source:99:Short"),
        "%family:source:99:Short"
    );
    assert_eq!(
        registry_chirho.readable_keys_chirho("%family:unresolved:4:Main:11:QChirho.Foo"),
        "QChirho.Foo"
    );
}

#[test]
fn open_pair_refusals_and_exhaustion_keep_family_and_source_branch_provenance_chirho() {
    let id_chirho = source_id_chirho("DefinitionChirho", "FChirho");
    let first_chirho = branch_chirho(&id_chirho, "LeftProducerChirho", 10, "Int", "Bool");
    let second_chirho = branch_chirho(&id_chirho, "RightProducerChirho", 30, "Int", "Char");
    let definition_chirho = definition_chirho(
        &id_chirho,
        TypeFamilyReductionChirho::OpenChirho,
        vec![first_chirho.clone(), second_chirho.clone()],
    );
    let expected_chirho = TypeFamilyDefinitionContextChirho::PairChirho {
        family_chirho: id_chirho.clone(),
        left_index_chirho: 0,
        left_id_chirho: first_chirho.id_chirho,
        right_index_chirho: 1,
        right_id_chirho: second_chirho.id_chirho,
    };
    let conflict_chirho = definition_chirho
        .verify_open_coherence_chirho(
            &|left_chirho, right_chirho| left_chirho.rhs_chirho == right_chirho.rhs_chirho,
            1,
        )
        .unwrap_err();
    let exhausted_chirho = definition_chirho
        .verify_open_coherence_chirho(&|_, _| true, 0)
        .unwrap_err();
    for error_chirho in [conflict_chirho, exhausted_chirho] {
        assert_eq!(error_chirho.context_chirho(), Some(&expected_chirho));
        let rendered_chirho = error_chirho.to_string();
        assert!(rendered_chirho.contains("DefinitionChirho.FChirho"));
        assert!(rendered_chirho.contains("LeftProducerChirho:10#0"));
        assert!(rendered_chirho.contains("RightProducerChirho:30#0"));
        assert!(!rendered_chirho.contains("%family:"));
    }
    assert!(!definition_chirho.open_coherence_verified_chirho());
}

#[test]
fn unsupported_open_branch_keeps_single_branch_provenance_chirho() {
    let id_chirho = source_id_chirho("DefinitionChirho", "HeldChirho");
    let mut branch_chirho = branch_chirho(&id_chirho, "ProducerChirho", 50, "Int", "Bool");
    branch_chirho.held_chirho = true;
    let definition_chirho = definition_chirho(
        &id_chirho,
        TypeFamilyReductionChirho::OpenChirho,
        vec![branch_chirho.clone()],
    );
    let error_chirho = definition_chirho
        .verify_open_coherence_chirho(&|_, _| true, 1)
        .unwrap_err();
    assert_eq!(
        error_chirho.context_chirho(),
        Some(&TypeFamilyDefinitionContextChirho::BranchChirho {
            family_chirho: id_chirho,
            index_chirho: 0,
            id_chirho: branch_chirho.id_chirho
        })
    );
}
