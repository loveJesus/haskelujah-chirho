// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Defaults use the same signed-binding and instance evidence path as written methods.

use super::{NextErrorKindChirho, assert_character_chirho, compile_source_chirho};

#[test]
fn defaults_dispatch_through_the_actual_instance_and_written_overrides_win_chirho() {
    assert_character_chirho(
        include_str!(
            "../fixtures_chirho/default_methods_chirho/positive-chirho/ActualInstanceChirho.hs"
        ),
        'a',
    );
    assert_character_chirho(
        include_str!("../fixtures_chirho/default_methods_chirho/positive-chirho/OverrideChirho.hs"),
        'o',
    );
    assert_character_chirho(
        include_str!(
            "../fixtures_chirho/default_methods_chirho/positive-chirho/DifferentTypeChirho.hs"
        ),
        'd',
    );
}

#[test]
fn mutual_defaults_remain_lazy_and_use_the_instances_selected_method_chirho() {
    assert_character_chirho(
        include_str!(
            "../fixtures_chirho/default_methods_chirho/positive-chirho/ValueDefaultChirho.hs"
        ),
        'n',
    );
    assert_character_chirho(
        include_str!("../fixtures_chirho/default_methods_chirho/positive-chirho/MutualChirho.hs"),
        'm',
    );
    assert_character_chirho(
        include_str!("../fixtures_chirho/default_methods_chirho/positive-chirho/LazyChirho.hs"),
        'z',
    );
}

#[test]
fn defaults_receive_superclasses_and_constructor_instance_contexts_chirho() {
    assert_character_chirho(
        include_str!(
            "../fixtures_chirho/default_methods_chirho/positive-chirho/SuperclassChirho.hs"
        ),
        's',
    );
    assert_character_chirho(
        include_str!("../fixtures_chirho/default_methods_chirho/positive-chirho/ContextChirho.hs"),
        'c',
    );
}

#[test]
fn defaults_see_later_names_all_equations_and_scoped_class_variables_chirho() {
    assert_character_chirho(
        include_str!(
            "../fixtures_chirho/default_methods_chirho/positive-chirho/LaterNamesChirho.hs"
        ),
        'l',
    );
    assert_character_chirho(
        include_str!(
            "../fixtures_chirho/default_methods_chirho/positive-chirho/EquationsChirho.hs"
        ),
        'e',
    );
    assert_character_chirho(
        include_str!("../fixtures_chirho/default_methods_chirho/positive-chirho/ScopedChirho.hs"),
        'v',
    );
}

#[test]
fn generic_defaults_are_checked_even_when_every_instance_overrides_them_chirho() {
    for source_chirho in [
        include_str!(
            "../fixtures_chirho/default_methods_chirho/negative-chirho/BadGenericChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/default_methods_chirho/negative-chirho/MissingGivenChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/default_methods_chirho/negative-chirho/NoSelfSuperclassChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/default_methods_chirho/negative-chirho/ScopedHaskell2010Chirho.hs"
        ),
    ] {
        let error_chirho = compile_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::TypeChirho,
            "{error_chirho}"
        );
    }
}

#[test]
fn unknown_members_separate_definitions_and_inconsistent_arities_are_invalid_chirho() {
    for source_chirho in [
        include_str!(
            "../fixtures_chirho/default_methods_chirho/negative-chirho/ClassPatternChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/default_methods_chirho/negative-chirho/InstancePatternChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/default_methods_chirho/negative-chirho/UnknownDefaultSignatureChirho.hs"
        ),
        include_str!("../fixtures_chirho/default_methods_chirho/negative-chirho/OrphanChirho.hs"),
        include_str!(
            "../fixtures_chirho/default_methods_chirho/negative-chirho/SeparateEquationsChirho.hs"
        ),
        include_str!("../fixtures_chirho/default_methods_chirho/negative-chirho/ArityChirho.hs"),
    ] {
        let error_chirho = compile_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::InvalidChirho,
            "{error_chirho}"
        );
    }
}

#[test]
fn default_signatures_and_missing_method_failure_values_stay_explicitly_unsupported_chirho() {
    for source_chirho in [
        include_str!(
            "../fixtures_chirho/default_methods_chirho/unsupported-chirho/DefaultSignatureChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/default_methods_chirho/unsupported-chirho/MissingMethodChirho.hs"
        ),
    ] {
        assert_eq!(
            compile_source_chirho(source_chirho)
                .unwrap_err()
                .kind_chirho,
            NextErrorKindChirho::UnsupportedChirho
        );
    }
}
