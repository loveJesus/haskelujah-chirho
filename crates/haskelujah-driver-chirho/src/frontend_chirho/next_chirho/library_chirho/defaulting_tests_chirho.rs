// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::super::{check_source_chirho, eval_binding_chirho};
use super::*;

const POLYMORPHIC_CHIRHO: &str = include_str!(
    "../fixtures_chirho/numeric_defaulting_chirho/positive-chirho/PolymorphicNumericChirho.hs"
);

fn assert_integer_chirho(source_chirho: &str, entry_chirho: &str, expected_chirho: &str) {
    let (value_chirho, _) = eval_binding_chirho(source_chirho, entry_chirho, 100_000).unwrap();
    assert!(matches!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntegerChirho(_)
    ));
    assert_eq!(value_chirho.to_string(), expected_chirho);
}

#[test]
fn ordinary_literals_and_arithmetic_default_to_the_first_standard_type_chirho() {
    for (source_chirho, entry_chirho, expected_chirho) in [
        (
            include_str!(
                "../fixtures_chirho/numeric_defaulting_chirho/positive-chirho/LiteralDefaultChirho.hs"
            ),
            "resultChirho",
            "340282366920938463463374607431768211457",
        ),
        (
            include_str!(
                "../fixtures_chirho/numeric_defaulting_chirho/positive-chirho/ArithmeticDefaultChirho.hs"
            ),
            "resultChirho",
            "9",
        ),
        (
            include_str!(
                "../fixtures_chirho/numeric_defaulting_chirho/positive-chirho/ArithmeticDefaultChirho.hs"
            ),
            "negativeChirho",
            "-9223372036854775809",
        ),
        (
            include_str!(
                "../fixtures_chirho/numeric_defaulting_chirho/positive-chirho/Haskell2010DefaultChirho.hs"
            ),
            "resultChirho",
            "7",
        ),
    ] {
        assert_integer_chirho(source_chirho, entry_chirho, expected_chirho);
    }
}

#[test]
fn a_numeric_function_keeps_its_dictionary_and_works_at_two_types_chirho() {
    let checked_chirho = check_source_chirho(POLYMORPHIC_CHIRHO).unwrap();
    let name_chirho = checked_chirho
        .module_chirho
        .names_chirho
        .iter()
        .position(|name_chirho| name_chirho.occurrence_chirho == "plusOneChirho")
        .unwrap();
    let scheme_chirho =
        &checked_chirho.schemes_chirho[&haskelujah_hir_chirho::NameIdChirho(name_chirho as u32)];
    assert_eq!(scheme_chirho.quantified_chirho.len(), 1);
    assert_eq!(scheme_chirho.context_chirho.len(), 1);
    assert_integer_chirho(POLYMORPHIC_CHIRHO, "resultChirho", "3");
    let (value_chirho, _) = eval_binding_chirho(POLYMORPHIC_CHIRHO, "homeChirho", 100_000).unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('t')
    );
}

#[test]
fn restriction_and_captured_local_constraints_default_after_generalization_chirho() {
    let restricted_chirho = include_str!(
        "../fixtures_chirho/numeric_defaulting_chirho/positive-chirho/MonomorphicNumericChirho.hs"
    );
    let checked_chirho = check_source_chirho(restricted_chirho).unwrap();
    let name_chirho = checked_chirho
        .module_chirho
        .names_chirho
        .iter()
        .position(|name_chirho| name_chirho.occurrence_chirho == "restrictedChirho")
        .unwrap();
    let scheme_chirho =
        &checked_chirho.schemes_chirho[&haskelujah_hir_chirho::NameIdChirho(name_chirho as u32)];
    assert!(scheme_chirho.quantified_chirho.is_empty());
    assert!(scheme_chirho.context_chirho.is_empty());
    assert_integer_chirho(restricted_chirho, "resultChirho", "3");
    let source_chirho = include_str!(
        "../fixtures_chirho/numeric_defaulting_chirho/positive-chirho/LocalDefaultChirho.hs"
    );
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 100_000).unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::BoolChirho(true)
    );
}

#[test]
fn user_classes_and_written_ambiguity_are_not_cured_by_numeric_defaulting_chirho() {
    for source_chirho in [
        include_str!(
            "../fixtures_chirho/numeric_defaulting_chirho/boundary-chirho/HomeNumChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/numeric_defaulting_chirho/boundary-chirho/AmbiguousNumericSignatureChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/numeric_defaulting_chirho/boundary-chirho/NumFirstBlockedChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/integer_arithmetic_chirho/boundary-chirho/IntegerNoDefaultChirho.hs"
        ),
    ] {
        let error_chirho = check_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::TypeChirho,
            "{error_chirho}"
        );
        assert!(
            error_chirho.message_chirho.contains("ambiguous"),
            "{error_chirho}"
        );
    }
}

#[test]
fn a_later_group_can_fix_a_restricted_numeric_meta_before_defaulting_chirho() {
    let source_chirho = include_str!(
        "../fixtures_chirho/numeric_defaulting_chirho/positive-chirho/LaterGroupChirho.hs"
    );
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 100_000).unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('b')
    );
}

#[test]
fn custom_defaults_extension_profiles_and_missing_rigid_instances_decline_chirho() {
    for source_chirho in [
        include_str!(
            "../fixtures_chirho/numeric_defaulting_chirho/boundary-chirho/NoDefaultsChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/numeric_defaulting_chirho/boundary-chirho/ExplicitIntegerDefaultChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/numeric_defaulting_chirho/boundary-chirho/ExtendedDefaultsChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/numeric_defaulting_chirho/boundary-chirho/RigidNumericChirho.hs"
        ),
    ] {
        let error_chirho = check_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{error_chirho}"
        );
    }
}

#[test]
fn compiler_unit_modules_do_not_inherit_home_numeric_defaults_chirho() {
    // Exercise the unit-policy boundary with the same already-resolved HIR and
    // checked imported interface facts; only the module's defining unit changes.
    let source_chirho = include_str!(
        "../fixtures_chirho/numeric_defaulting_chirho/positive-chirho/LiteralDefaultChirho.hs"
    );
    let mut module_chirho = check_source_chirho(source_chirho).unwrap().module_chirho;
    module_chirho.unit_chirho = UnitIdentityChirho::CompilerChirho;
    let error_chirho =
        haskelujah_tc_chirho::check_module_chirho(module_chirho, 1 << 20).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        haskelujah_tc_chirho::TcErrorKindChirho::UnsupportedChirho
    );
    assert!(error_chirho.message_chirho.contains("defaulting"));
}
