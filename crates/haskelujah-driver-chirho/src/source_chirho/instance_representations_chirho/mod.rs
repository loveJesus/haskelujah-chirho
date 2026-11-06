// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Fresh GHC controls distinguish implicit instance defaults from written kinds.

use super::language_options_tests_chirho::check_chirho;

#[test]
fn implicit_and_explicit_lifted_instance_arrows_accept_value_binders_chirho() {
    for source_chirho in [
        include_str!("ImplicitArrowInstanceChirho.hs"),
        include_str!("ExplicitLiftedArrowInstanceChirho.hs"),
    ] {
        let result_chirho = check_chirho(source_chirho, &[]);
        if let Err(error_chirho) = result_chirho {
            panic!("fresh GHC-positive instance arrow: {error_chirho}");
        }
    }
}

#[test]
fn written_polymorphic_instance_arrows_keep_their_fixedness_error_chirho() {
    let error_chirho = check_chirho(include_str!("ExplicitArrowRepresentationChirho.hs"), &[])
        .err()
        .expect("GHC55287: an explicitly polymorphic lambda domain cannot become lifted");
    assert!(error_chirho.to_string().contains("error[E"));
}

#[test]
fn an_implicit_instance_does_not_serve_an_unlifted_class_parameter_chirho() {
    let error_chirho = check_chirho(include_str!("ImplicitPolykindedInstanceChirho.hs"), &[])
        .err()
        .expect("GHC39999: the implicit instance defaults to a lifted parameter");
    assert!(error_chirho.to_string().contains("error[E"));
}

#[test]
fn a_written_instance_kind_keeps_its_unlifted_class_parameter_chirho() {
    let result_chirho = check_chirho(include_str!("ExplicitPolykindedInstanceChirho.hs"), &[]);
    if let Err(error_chirho) = result_chirho {
        panic!("fresh GHC-positive explicitly kinded instance: {error_chirho}");
    }
}

#[test]
fn original_source_class_instance_arrows_close_before_method_rigidity_chirho() {
    let path_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../ghc-tests-chirho/typecheck-chirho/should_compile/T10592.hs");
    if let Err(error_chirho) =
        crate::check_source_path_chirho(path_chirho, crate::ExecutionModeChirho::BatchChirho)
    {
        panic!("retained fresh GHC-positive original T10592: {error_chirho}");
    }
}

#[test]
fn original_written_representation_universals_do_not_acquire_defaults_chirho() {
    let path_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../ghc-tests-chirho/typecheck-chirho/should_fail/LevPolyBounded.hs");
    assert!(
        crate::check_source_path_chirho(path_chirho, crate::ExecutionModeChirho::BatchChirho,)
            .is_err()
    );
}
