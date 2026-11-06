// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pinned provider and homonym controls use the real frontend kind handoff.

use super::language_options_tests_chirho::check_chirho;

#[test]
fn selected_scalar_providers_supply_their_actual_closed_representation_kinds_chirho() {
    for source_chirho in [
        include_str!("ExtsIntHashKindChirho.hs"),
        include_str!("ExtsWordHashKindChirho.hs"),
        include_str!("PrimIntHashKindChirho.hs"),
        include_str!("PrimWordHashKindChirho.hs"),
    ] {
        if let Err(error_chirho) = check_chirho(source_chirho, &[]) {
            panic!("selected primitive kind producer: {error_chirho}");
        }
    }
}

#[test]
fn swapped_scalar_representations_are_coded_kind_errors_chirho() {
    for source_chirho in [
        include_str!("WrongIntHashKindChirho.hs"),
        include_str!("WrongWordHashKindChirho.hs"),
    ] {
        let error_chirho = check_chirho(source_chirho, &[])
            .err()
            .expect("different actual primitive representations cannot unify");
        assert!(error_chirho.to_string().contains("error[E0300]"));
    }
}

#[test]
fn source_scalar_homonyms_keep_their_own_lifted_kinds_chirho() {
    for source_chirho in [
        include_str!("LocalIntHashLiftedChirho.hs"),
        include_str!("QualifiedIntHashBesideLocalChirho.hs"),
    ] {
        if let Err(error_chirho) = check_chirho(source_chirho, &[]) {
            panic!("source and qualified primitive kinds stay distinct: {error_chirho}");
        }
    }
}

#[test]
fn a_source_scalar_homonym_cannot_borrow_the_primitive_kind_chirho() {
    let error_chirho = check_chirho(include_str!("LocalIntHashWrongKindChirho.hs"), &[])
        .err()
        .expect("a source data declaration has its own lifted kind");
    assert!(error_chirho.to_string().contains("error[E0300]"));
}

#[test]
fn original_newtype_record_rejects_different_primitive_representations_chirho() {
    let path_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../ghc-tests-chirho/typecheck-chirho/should_fail/UnliftedNewtypesMismatchedKindRecord.hs");
    let error_chirho =
        crate::check_source_path_chirho(path_chirho, crate::ExecutionModeChirho::BatchChirho)
            .expect_err("retained GHC83865: a Word# field cannot implement a TYPE IntRep newtype");
    assert!(error_chirho.to_string().contains("error[E0300]"));
}

#[test]
fn class_methods_consume_the_actual_checked_kind_beside_a_primitive_supplier_chirho() {
    if let Err(error_chirho) = check_chirho(include_str!("LocalClassMethodKindChirho.hs"), &[]) {
        panic!("fresh GHC-positive lifted source method: {error_chirho}");
    }
    let error_chirho = check_chirho(include_str!("PrimitiveClassMethodKindChirho.hs"), &[])
        .err()
        .expect("fresh GHC83865: a class method requires a lifted value kind");
    assert!(error_chirho.to_string().contains("error[E0300]"));
}

#[test]
fn a_cold_scalar_alias_retains_its_checked_supplier_kind_beside_a_caller_homonym_chirho() {
    let root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/source_chirho/scalar_kinds_chirho/transport_chirho");
    if let Err(error_chirho) = crate::check_source_path_chirho(
        root_chirho.join("ScalarAliasCallerChirho.hs"),
        crate::ExecutionModeChirho::BatchChirho,
    ) {
        panic!("supplier-owned scalar alias keeps its checked kind: {error_chirho}");
    }
    let error_chirho = crate::check_source_path_chirho(
        root_chirho.join("WrongScalarAliasCallerChirho.hs"),
        crate::ExecutionModeChirho::BatchChirho,
    )
    .expect_err("the imported scalar alias cannot acquire its caller's representation");
    assert!(error_chirho.to_string().contains("error[E0300]"));
}

#[test]
fn an_explicit_given_retains_its_full_kind_prefix_beside_an_inapplicable_instance_chirho() {
    if let Err(error_chirho) = check_chirho(
        include_str!("transport_chirho/GivenImplicitInstanceChirho.hs"),
        &[],
    ) {
        panic!("fresh GHC-positive given context retains its checked full prefix: {error_chirho}");
    }
}

#[test]
fn a_source_cycle_cannot_certify_local_instance_prefix_completeness_chirho() {
    let path_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/source_chirho/scalar_kinds_chirho/transport_chirho/cycle_chirho/OwnerChirho.hs");
    if let Err(error_chirho) =
        crate::check_source_path_chirho(path_chirho, crate::ExecutionModeChirho::BatchChirho)
    {
        panic!(
            "fresh GHC-positive SOURCE cycle carries an instance beyond the local arena: {error_chirho}"
        );
    }
}
