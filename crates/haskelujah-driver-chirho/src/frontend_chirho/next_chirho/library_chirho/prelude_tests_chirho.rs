// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::super::{check_source_chirho, eval_binding_chirho, eval_sources_binding_chirho};
use super::*;

fn fixture_chirho(name_chirho: &str, positive_chirho: bool) -> String {
    let root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/frontend_chirho/next_chirho/fixtures_chirho/prelude_exposure_chirho");
    std::fs::read_to_string(
        root_chirho
            .join(if positive_chirho {
                "positive-chirho"
            } else {
                "boundary-chirho"
            })
            .join(format!("{name_chirho}.hs")),
    )
    .unwrap()
}

#[test]
fn default_prelude_uses_checked_numeric_definitions_chirho() {
    for (name_chirho, entry_chirho, expected_chirho) in [
        ("DefaultArithmeticChirho", "resultChirho", "3"),
        ("DefaultArithmeticChirho", "negativeChirho", "-7"),
        ("DefiningImportChirho", "resultChirho", "5"),
        ("PolymorphicPreludeChirho", "resultChirho", "42"),
        ("PreludeSelectedChirho", "resultChirho", "9"),
        ("PreludeIntegerChirho", "resultChirho", "3"),
    ] {
        let (value_chirho, _) =
            eval_binding_chirho(&fixture_chirho(name_chirho, true), entry_chirho, 100_000).unwrap();
        assert_eq!(value_chirho.to_string(), expected_chirho, "{name_chirho}");
    }
}

#[test]
fn tuple_exports_and_prelude_qualification_keep_originals_chirho() {
    for (name_chirho, expected_chirho) in [
        ("DefaultTupleChirho", 't'),
        ("PreludeTupleChirho", 't'),
        ("PreludeQualifiedChirho", 'q'),
    ] {
        let (value_chirho, _) =
            eval_binding_chirho(&fixture_chirho(name_chirho, true), "resultChirho", 100_000)
                .unwrap();
        assert_eq!(
            value_chirho,
            haskelujah_runtime_chirho::ValueChirho::CharChirho(expected_chirho)
        );
    }
}

#[test]
fn spelling_cannot_supply_a_literal_or_override_the_prelude_chirho() {
    let (value_chirho, _) = eval_binding_chirho(
        &fixture_chirho("RealOriginShadowChirho", false),
        "resultChirho",
        100_000,
    )
    .unwrap();
    assert_eq!(value_chirho.to_string(), "3");
    let error_chirho =
        check_source_chirho(&fixture_chirho("HomePlusClashChirho", false)).unwrap_err();
    assert_eq!(error_chirho.kind_chirho, NextErrorKindChirho::InvalidChirho);
    let library_chirho =
        "{-# LANGUAGE NoImplicitPrelude #-}\nmodule AChirho where\nxChirho + _ = xChirho\n";
    let user_chirho = "module BChirho where\nimport AChirho\nresultChirho = 'x' + 'y'\n";
    let error_chirho =
        super::super::check_sources_chirho(&[library_chirho, user_chirho], "BChirho").unwrap_err();
    assert_eq!(error_chirho.kind_chirho, NextErrorKindChirho::InvalidChirho);
}

#[test]
fn absent_or_unchecked_prelude_exports_never_get_a_fabricated_type_chirho() {
    let error_chirho =
        check_source_chirho(&fixture_chirho("PreludeNonexportChirho", false)).unwrap_err();
    assert_eq!(error_chirho.kind_chirho, NextErrorKindChirho::InvalidChirho);
    for name_chirho in [
        "UnimplementedPreludeChirho",
        "PreludeUnsupportedSelectionChirho",
        "PreludeOpenChirho",
        "ImplicitAnnotationChirho",
    ] {
        let error_chirho = check_source_chirho(&fixture_chirho(name_chirho, false)).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{error_chirho}"
        );
    }
    for (name_chirho, kind_chirho) in [
        (
            "PreludeNotSelectedChirho",
            NextErrorKindChirho::InvalidChirho,
        ),
        (
            "UncheckedSubtractChirho",
            NextErrorKindChirho::UnsupportedChirho,
        ),
    ] {
        assert_eq!(
            check_source_chirho(&fixture_chirho(name_chirho, false))
                .unwrap_err()
                .kind_chirho,
            kind_chirho
        );
    }
}

#[test]
fn checked_prelude_schemes_cross_a_home_interface_chirho() {
    let library_chirho =
        "module AChirho (doubleChirho) where\ndoubleChirho xChirho = xChirho + xChirho\n";
    let user_chirho = "module BChirho where\nimport AChirho\nresultChirho = doubleChirho 21\n";
    let (value_chirho, _) = eval_sources_binding_chirho(
        &[library_chirho, user_chirho],
        "BChirho",
        "resultChirho",
        100_000,
    )
    .unwrap();
    assert_eq!(value_chirho.to_string(), "42");
}

#[test]
fn unrelated_modules_do_not_load_slices_or_downgrade_absence_chirho() {
    let library_chirho = "module AChirho where\naChirho = 'a'\n";
    let user_chirho = "module BChirho where\nimport AChirho\nshadowedChirho = (\\fst -> fst) aChirho\nresultChirho = missingChirho\n";
    let error_chirho =
        super::super::check_sources_chirho(&[library_chirho, user_chirho], "BChirho").unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::InvalidChirho,
        "{error_chirho}"
    );
    let checked_chirho =
        check_source_chirho("module OrdinaryChirho where\nresultChirho = 'a'\n").unwrap();
    assert!(checked_chirho.module_chirho.imports_chirho.is_empty());
}

#[test]
fn loaded_library_slices_cannot_hide_absent_home_class_instances_chirho() {
    let error_chirho =
        check_source_chirho(&fixture_chirho("HomeClassAbsenceChirho", false)).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::TypeChirho,
        "{error_chirho}"
    );
    assert!(
        error_chirho.message_chirho.contains("no instance"),
        "{error_chirho}"
    );
}

#[test]
fn an_explicit_home_prelude_is_never_replaced_by_the_upstream_facade_chirho() {
    let library_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Prelude where\nfst _ = 'h'\n";
    let user_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule UserChirho where\nimport Prelude (fst)\nresultChirho = fst ('a','b')\n";
    let error_chirho =
        super::super::check_sources_chirho(&[library_chirho, user_chirho], "UserChirho")
            .unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    assert!(error_chirho.message_chirho.contains("home Prelude"));
}
