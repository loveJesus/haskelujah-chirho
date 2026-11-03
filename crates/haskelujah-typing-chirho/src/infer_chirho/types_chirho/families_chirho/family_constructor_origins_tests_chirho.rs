// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn unknown_and_ambiguous_spellings_never_supply_constructor_proof_chirho() {
    let mut origins_chirho = FamilyConstructorOriginsChirho::default();
    assert_eq!(origins_chirho.identity_chirho("T"), None);
    origins_chirho
        .replace_source_module_chirho("A", &[("T".to_owned(), true)])
        .unwrap();
    let identity_a_chirho = origins_chirho.identity_chirho("T").unwrap();
    assert_eq!(
        origins_chirho.identity_chirho(&identity_a_chirho),
        Some(identity_a_chirho.clone())
    );
    origins_chirho
        .replace_source_module_chirho("B", &[("T".to_owned(), true)])
        .unwrap();
    assert_eq!(origins_chirho.identity_chirho("T"), None);
    assert_eq!(
        origins_chirho.identity_chirho(&identity_a_chirho),
        Some(identity_a_chirho)
    );
    assert_eq!(origins_chirho.identity_chirho("A.T"), None);
    assert_eq!(origins_chirho.identity_chirho("B.T"), None);
}

#[test]
fn source_shadowing_does_not_inherit_compiler_identity_chirho() {
    let mut origins_chirho = FamilyConstructorOriginsChirho::default();
    origins_chirho.insert_compiler_chirho("Int".to_owned(), "GHC.Types.Int".to_owned());
    origins_chirho.insert_compiler_chirho("Prelude.Int".to_owned(), "GHC.Types.Int".to_owned());
    let standard_chirho = origins_chirho.identity_chirho("Int").unwrap();
    origins_chirho
        .replace_source_module_chirho("User", &[("Int".to_owned(), true)])
        .unwrap();
    assert_eq!(origins_chirho.identity_chirho("Int"), None);
    assert_eq!(
        origins_chirho.identity_chirho(&standard_chirho),
        Some(standard_chirho.clone())
    );
    assert_ne!(
        origins_chirho.identity_chirho("User.Int"),
        Some(standard_chirho)
    );
}

#[test]
fn synonyms_and_classes_are_not_claimed_generative_chirho() {
    let mut origins_chirho = FamilyConstructorOriginsChirho::default();
    origins_chirho
        .replace_source_module_chirho(
            "A",
            &[("Alias".to_owned(), false), ("Class".to_owned(), false)],
        )
        .unwrap();
    assert_eq!(origins_chirho.identity_chirho("A.Alias"), None);
    assert_eq!(origins_chirho.identity_chirho("Class"), None);
}

#[test]
fn a_source_declaration_replacement_retires_removed_names_chirho() {
    let mut origins_chirho = FamilyConstructorOriginsChirho::default();
    origins_chirho
        .replace_source_module_chirho("A", &[("Old".to_owned(), true)])
        .unwrap();
    origins_chirho
        .replace_source_module_chirho("A", &[("New".to_owned(), true)])
        .unwrap();
    assert_eq!(origins_chirho.identity_chirho("A.Old"), None);
    assert_eq!(origins_chirho.identity_chirho("Old"), None);
    assert!(origins_chirho.identity_chirho("New").is_some());
}

#[test]
fn invalid_source_replacement_and_conflicting_snapshot_merge_are_atomic_chirho() {
    let mut origins_chirho = FamilyConstructorOriginsChirho::default();
    origins_chirho
        .replace_source_module_chirho("A", &[("T".to_owned(), true)])
        .unwrap();
    let prior_chirho = origins_chirho.identity_chirho("T");
    assert!(
        origins_chirho
            .replace_source_module_chirho(
                "A",
                &[
                    ("Duplicate".to_owned(), true),
                    ("Duplicate".to_owned(), false)
                ]
            )
            .is_err()
    );
    assert_eq!(origins_chirho.identity_chirho("T"), prior_chirho);
    let mut incoming_chirho = FamilyConstructorOriginsChirho::default();
    incoming_chirho
        .replace_source_module_chirho("A", &[("U".to_owned(), true)])
        .unwrap();
    incoming_chirho
        .replace_source_module_chirho("B", &[("Extra".to_owned(), true)])
        .unwrap();
    assert!(origins_chirho.merge_chirho(&incoming_chirho).is_err());
    assert_eq!(origins_chirho.identity_chirho("T"), prior_chirho);
    assert_eq!(origins_chirho.identity_chirho("B.Extra"), None);
}

#[test]
fn a_raw_import_alias_cannot_borrow_a_transitive_modules_origin_chirho() {
    let mut origins_chirho = FamilyConstructorOriginsChirho::default();
    origins_chirho
        .replace_source_module_chirho("ActualChirho", &[("TChirho".to_owned(), true)])
        .unwrap();
    let actual_chirho = origins_chirho.identity_chirho("TChirho").unwrap();
    origins_chirho
        .replace_source_module_chirho("AliasChirho", &[("TChirho".to_owned(), true)])
        .unwrap();
    // A consumer can write `import qualified ActualChirho as AliasChirho`.
    // The raw spelling then names ActualChirho's T, not the transitive AliasChirho.T.
    assert_eq!(origins_chirho.identity_chirho("AliasChirho.TChirho"), None);
    assert_eq!(
        origins_chirho.identity_chirho(&actual_chirho),
        Some(actual_chirho)
    );
    origins_chirho.insert_compiler_chirho("Prelude.Int".to_owned(), "GHC.Types.Int".to_owned());
    assert_eq!(
        origins_chirho.identity_chirho("Prelude.Int"),
        None,
        "Prelude can be a source alias too"
    );
}
