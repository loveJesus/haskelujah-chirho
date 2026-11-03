// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::super::family_definitions_chirho::{
    TypeFamilyDefinitionChirho, TypeFamilyReductionChirho,
};
use super::*;

fn source_id_chirho(module_chirho: &str) -> TypeFamilyIdChirho {
    TypeFamilyIdChirho::SourceChirho {
        module_chirho: module_chirho.to_owned(),
        name_chirho: "F".to_owned(),
    }
}

fn fixture_chirho() -> TypeFamilyRegistryChirho {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    for module_chirho in ["A", "B"] {
        let id_chirho = source_id_chirho(module_chirho);
        registry_chirho
            .insert_chirho(
                TypeFamilyDefinitionChirho::new_chirho(
                    id_chirho.clone(),
                    1,
                    TypeFamilyReductionChirho::ClosedChirho,
                    Vec::new(),
                )
                .unwrap(),
            )
            .unwrap();
        registry_chirho
            .export_alias_chirho(format!("{module_chirho}.F"), &id_chirho)
            .unwrap();
    }
    registry_chirho
        .export_alias_chirho("Forwarder.F".to_owned(), &source_id_chirho("A"))
        .unwrap();
    registry_chirho
}

#[test]
fn qualified_namesakes_resolve_separately_without_bare_fallback_chirho() {
    let registry_chirho = fixture_chirho();
    let mut scope_chirho = TypeFamilyScopeChirho::new_chirho();
    scope_chirho.import_export_chirho("Q.F".to_owned(), "A.F", &registry_chirho);
    scope_chirho.import_export_chirho("R.F".to_owned(), "B.F", &registry_chirho);
    assert_eq!(
        scope_chirho.resolve_chirho("Q.F", &registry_chirho),
        TypeFamilyScopeResolutionChirho::UniqueChirho(&source_id_chirho("A").key_chirho())
    );
    assert_eq!(
        scope_chirho.resolve_chirho("R.F", &registry_chirho),
        TypeFamilyScopeResolutionChirho::UniqueChirho(&source_id_chirho("B").key_chirho())
    );
    assert_eq!(
        scope_chirho.resolve_chirho("F", &registry_chirho),
        TypeFamilyScopeResolutionChirho::MissingChirho
    );
    assert_eq!(
        scope_chirho.resolve_chirho("A.F", &registry_chirho),
        TypeFamilyScopeResolutionChirho::MissingChirho
    );
}

#[test]
fn two_paths_to_one_definition_are_not_ambiguous_chirho() {
    let registry_chirho = fixture_chirho();
    let mut scope_chirho = TypeFamilyScopeChirho::new_chirho();
    scope_chirho.import_export_chirho("F".to_owned(), "A.F", &registry_chirho);
    scope_chirho.import_export_chirho("F".to_owned(), "Forwarder.F", &registry_chirho);
    assert_eq!(
        scope_chirho.resolve_chirho("F", &registry_chirho),
        TypeFamilyScopeResolutionChirho::UniqueChirho(&source_id_chirho("A").key_chirho())
    );
}

#[test]
fn distinct_unqualified_origins_are_ambiguous_chirho() {
    let registry_chirho = fixture_chirho();
    let mut scope_chirho = TypeFamilyScopeChirho::new_chirho();
    scope_chirho.import_export_chirho("F".to_owned(), "A.F", &registry_chirho);
    scope_chirho.import_export_chirho("F".to_owned(), "B.F", &registry_chirho);
    let TypeFamilyScopeResolutionChirho::AmbiguousChirho(keys_chirho) =
        scope_chirho.resolve_chirho("F", &registry_chirho)
    else {
        panic!("distinct identities must remain ambiguous")
    };
    assert_eq!(
        keys_chirho,
        &BTreeSet::from([
            source_id_chirho("A").key_chirho(),
            source_id_chirho("B").key_chirho()
        ])
    );
}

#[test]
fn local_family_shadows_imports_independently_of_registration_order_chirho() {
    let registry_chirho = fixture_chirho();
    let local_chirho = source_id_chirho("Local");
    for local_first_chirho in [false, true] {
        let mut scope_chirho = TypeFamilyScopeChirho::new_chirho();
        if local_first_chirho {
            scope_chirho
                .bind_local_family_chirho("F".to_owned(), &local_chirho)
                .unwrap();
        }
        scope_chirho.import_export_chirho("F".to_owned(), "A.F", &registry_chirho);
        if !local_first_chirho {
            scope_chirho
                .bind_local_family_chirho("F".to_owned(), &local_chirho)
                .unwrap();
        }
        assert_eq!(
            scope_chirho.resolve_chirho("F", &registry_chirho),
            TypeFamilyScopeResolutionChirho::UniqueChirho(&local_chirho.key_chirho())
        );
    }
}

#[test]
fn other_local_types_block_imported_family_lookup_chirho() {
    let registry_chirho = fixture_chirho();
    let mut scope_chirho = TypeFamilyScopeChirho::new_chirho();
    scope_chirho.import_export_chirho("F".to_owned(), "A.F", &registry_chirho);
    scope_chirho
        .bind_other_local_type_chirho("F".to_owned())
        .unwrap();
    assert_eq!(
        scope_chirho.resolve_chirho("F", &registry_chirho),
        TypeFamilyScopeResolutionChirho::MissingChirho
    );
    assert!(
        scope_chirho
            .bind_local_family_chirho("F".to_owned(), &source_id_chirho("A"))
            .is_err()
    );
}

#[test]
fn canonical_keys_need_an_actual_registered_definition_chirho() {
    let registry_chirho = fixture_chirho();
    let scope_chirho = TypeFamilyScopeChirho::new_chirho();
    let present_chirho = source_id_chirho("A").key_chirho();
    let absent_chirho = source_id_chirho("Absent").key_chirho();
    assert_eq!(
        scope_chirho.resolve_chirho(&present_chirho, &registry_chirho),
        TypeFamilyScopeResolutionChirho::UniqueChirho(&present_chirho)
    );
    assert_eq!(
        scope_chirho.resolve_chirho(&absent_chirho, &registry_chirho),
        TypeFamilyScopeResolutionChirho::MissingChirho
    );
}
