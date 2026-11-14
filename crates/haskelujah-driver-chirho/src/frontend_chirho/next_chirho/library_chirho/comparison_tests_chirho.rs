// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

use super::super::{check_source_chirho, eval_binding_chirho};
use super::*;
use haskelujah_runtime_chirho::ValueChirho;

macro_rules! fixture_chirho {
    ($path_chirho:literal) => {
        include_str!(concat!(
            "../fixtures_chirho/integer_arithmetic_chirho/comparisons-chirho/",
            $path_chirho
        ))
    };
}

fn character_chirho(source_chirho: &str, entry_chirho: &str, expected_chirho: char) {
    let (value_chirho, _) = eval_binding_chirho(source_chirho, entry_chirho, 200_000)
        .unwrap_or_else(|error_chirho| panic!("{entry_chirho}: {error_chirho}"));
    assert_eq!(
        value_chirho,
        ValueChirho::CharChirho(expected_chirho),
        "{entry_chirho}"
    );
}

#[test]
fn prelude_comparison_results_match_the_wired_constructor_patterns_chirho() {
    let source_chirho = fixture_chirho!("positive-chirho/PreludeComparisonsChirho.hs");
    for (entry_chirho, expected_chirho) in [
        ("lessChirho", 'l'),
        ("equalChirho", 't'),
        ("greaterChirho", 'g'),
        ("sameChirho", 'e'),
        ("notChirho", 'n'),
        ("notEqualChirho", 'n'),
    ] {
        character_chirho(source_chirho, entry_chirho, expected_chirho);
    }
}

#[test]
fn comparisons_preserve_sign_and_magnitude_beyond_fixed_widths_chirho() {
    let source_chirho = fixture_chirho!("positive-chirho/ComparisonEdgesChirho.hs");
    for (entry_chirho, expected_chirho) in [
        ("resultChirho", 'e'),
        ("equalChirho", 't'),
        ("unequalChirho", 'u'),
        ("leChirho", 'l'),
        ("geChirho", 'g'),
        ("negativeOrderChirho", 'n'),
        ("zeroOrderChirho", 'z'),
    ] {
        character_chirho(source_chirho, entry_chirho, expected_chirho);
    }
    for (entry_chirho, expected_chirho) in [
        ("maximumChirho", "340282366920938463463374607431768211457"),
        ("minimumChirho", "-340282366920938463463374607431768211457"),
    ] {
        let (value_chirho, _) = eval_binding_chirho(source_chirho, entry_chirho, 200_000).unwrap();
        let ValueChirho::IntegerChirho(value_chirho) = value_chirho else {
            panic!("expected Integer")
        };
        assert_eq!(value_chirho.to_string(), expected_chirho);
    }
}

#[test]
fn checked_class_defaults_superclasses_and_polymorphism_share_evidence_chirho() {
    let source_chirho = fixture_chirho!("positive-chirho/ComparisonDefaultsChirho.hs");
    for (entry_chirho, expected_chirho) in [
        ("defaultEqChirho", 'd'),
        ("defaultLessChirho", 'l'),
        ("defaultMaxChirho", 'm'),
        ("superclassChirho", 's'),
        ("homeChirho", 'h'),
        ("integerChirho", 'i'),
        ("twoDictionariesChirho", '2'),
    ] {
        character_chirho(source_chirho, entry_chirho, expected_chirho);
    }
}

#[test]
fn comparison_bodies_demand_both_operands_only_after_saturation_chirho() {
    let source_chirho = fixture_chirho!("positive-chirho/ComparisonPartialChirho.hs");
    for (entry_chirho, expected_chirho) in [
        ("lazyChirho", 'p'),
        ("resultChirho", 'l'),
        ("equalChirho", 't'),
    ] {
        character_chirho(source_chirho, entry_chirho, expected_chirho);
    }
    let source_chirho = fixture_chirho!("boundary-chirho/ComparisonStrictChirho.hs");
    check_source_chirho(source_chirho).unwrap();
    for entry_chirho in [
        "leftChirho",
        "rightChirho",
        "compareLeftChirho",
        "compareRightChirho",
    ] {
        let error_chirho = eval_binding_chirho(source_chirho, entry_chirho, 100_000).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::ExecutionChirho
        );
        assert!(
            error_chirho
                .message_chirho
                .contains("no matching alternative"),
            "{error_chirho}"
        );
    }
}

#[test]
fn class_original_names_and_partial_instance_absence_keep_their_authority_chirho() {
    let error_chirho =
        check_source_chirho(fixture_chirho!("boundary-chirho/HomeEqDefaultChirho.hs")).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::TypeChirho,
        "{error_chirho}"
    );
    let error_chirho =
        check_source_chirho(fixture_chirho!("boundary-chirho/AbsentBoolEqChirho.hs")).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho,
        "{error_chirho}"
    );
    let error_chirho = check_source_chirho(fixture_chirho!(
        "boundary-chirho/DuplicateEqIntegerChirho.hs"
    ))
    .unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::TypeChirho,
        "{error_chirho}"
    );
    let error_chirho =
        check_source_chirho(fixture_chirho!("boundary-chirho/HomeOrderingChirho.hs")).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::TypeChirho,
        "{error_chirho}"
    );
    let (value_chirho, _) = eval_binding_chirho(
        fixture_chirho!("boundary-chirho/HomeComparisonNameChirho.hs"),
        "resultChirho",
        200_000,
    )
    .unwrap();
    let ValueChirho::IntegerChirho(value_chirho) = value_chirho else {
        panic!("expected Integer")
    };
    assert_eq!(value_chirho.to_string(), "1");
}

#[test]
fn manifest_edges_preserve_infix_and_prefix_equation_groups_chirho() {
    for source_chirho in [
        "xChirho <+> yChirho = xChirho\nxChirho <+> yChirho = yChirho\nnextChirho = 'n'\n",
        "(<+>) xChirho yChirho = xChirho\n(<+>) xChirho yChirho = yChirho\nnextChirho = 'n'\n",
    ] {
        let first_chirho = source_chirho.find('\n').unwrap() + 1;
        let second_chirho = first_chirho + source_chirho[first_chirho..].find('\n').unwrap() + 1;
        assert!(super::boundaries_chirho::validate_chirho(source_chirho, 0, first_chirho).is_err());
        assert!(
            super::boundaries_chirho::validate_chirho(source_chirho, first_chirho, second_chirho)
                .is_err()
        );
        super::boundaries_chirho::validate_chirho(source_chirho, 0, second_chirho).unwrap();
    }
}
