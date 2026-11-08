// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Checked family representation equalities, with retained GHC source contracts.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::check_chirho;

#[test]
fn checked_closed_family_representation_results_are_reduced_chirho() {
    let results_chirho = [
        include_str!("UnliftedNewtypesDifficultUnification.hs"),
        include_str!("ClosedFamilyNewtypeResultChirho.hs"),
    ]
    .map(|source_chirho| {
        check_chirho(source_chirho, &[])
            .map(|_| ())
            .map_err(|error_chirho| error_chirho.to_string())
    });
    assert!(
        results_chirho.iter().all(Result::is_ok),
        "{results_chirho:?}"
    );
}

#[test]
fn cold_checked_family_representation_uses_the_same_definition_chirho() {
    let path_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/source_chirho/full_kind_arity_chirho/family_representations_chirho/cold_checked_chirho/ColdCheckedRepresentationFieldChirho.hs");
    let result_chirho = crate::check_source_path_chirho(
        path_chirho,
        haskelujah_runtime_chirho::ExecutionModeChirho::BatchChirho,
    );
    assert!(result_chirho.is_ok(), "{result_chirho:?}");
}

#[test]
fn family_representation_negatives_keep_their_full_scope_and_result_roles_chirho() {
    for source_chirho in [
        include_str!("UnliftedNewtypesMismatchedKindRecord.hs"),
        include_str!("ClosedFamilyDataResultChirho.hs"),
        include_str!("OpenFamilyFieldChirho.hs"),
    ] {
        let error_chirho = check_chirho(source_chirho, &[])
            .map(|_| ())
            .expect_err("the checked family does not supply this representation contract");
        assert!(
            error_chirho.to_string().contains("error[E"),
            "{error_chirho}"
        );
    }
    let path_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/source_chirho/full_kind_arity_chirho/family_representations_chirho/cold_wrong_chirho/ColdWrongRepresentationFieldChirho.hs");
    let error_chirho = crate::check_source_path_chirho(
        path_chirho,
        haskelujah_runtime_chirho::ExecutionModeChirho::BatchChirho,
    )
    .map(|_| ())
    .expect_err("the full hidden/visible telescope remains required");
    assert!(
        error_chirho.to_string().contains("error[E"),
        "{error_chirho}"
    );
}

#[test]
fn ordinary_recursive_family_declarations_keep_their_kind_headers_chirho() {
    let result_chirho =
        check_chirho(include_str!("OrdinaryRecursiveFamilyChirho.hs"), &[]).map(|_| ());
    assert!(result_chirho.is_ok(), "{result_chirho:?}");
}

#[test]
fn recursive_family_kind_cycles_are_not_satisfied_by_freshening_chirho() {
    for source_chirho in [
        include_str!("RecursiveFamilyKindChirho.hs"),
        include_str!("RecursiveCheckedFamilyKindChirho.hs"),
    ] {
        assert!(check_chirho(source_chirho, &[]).is_err());
        assert!(check_chirho(source_chirho, &["-fdefer-type-errors"]).is_err());
    }
}
