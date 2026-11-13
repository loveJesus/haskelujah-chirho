// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::{
    NextErrorKindChirho, assert_character_chirho, compile_source_chirho, eval_binding_chirho,
};
use haskelujah_runtime_chirho::ValueChirho;

#[test]
fn exact_integer_payloads_exceed_machine_words_and_do_not_use_the_legacy_zero_chirho() {
    for (source_chirho, expected_chirho) in [
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/positive-chirho/ExactHugeChirho.hs"
            ),
            "340282366920938463463374607431768211457",
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/positive-chirho/ExactHexChirho.hs"
            ),
            "340282366920938463463374607431768211457",
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/positive-chirho/ExactBinaryChirho.hs"
            ),
            "4294967297",
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/positive-chirho/NegativeMinimumChirho.hs"
            ),
            "9223372036854775808",
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/positive-chirho/Haskell2010Chirho.hs"
            ),
            "18446744073709551617",
        ),
    ] {
        let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 50_000).unwrap();
        let ValueChirho::IntegerChirho(value_chirho) = value_chirho else {
            panic!("expected opaque Integer, got {value_chirho}");
        };
        assert_eq!(value_chirho.to_string(), expected_chirho);
    }
}

#[test]
fn conversion_negation_and_conditionals_resolve_at_the_lexical_occurrence_chirho() {
    for (source_chirho, expected_chirho) in [
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/positive-chirho/ScopeChirho.hs"
            ),
            'l',
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/positive-chirho/ParameterChirho.hs"
            ),
            'p',
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/positive-chirho/NegativeLexicalChirho.hs"
            ),
            'n',
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/positive-chirho/IfLexicalChirho.hs"
            ),
            'i',
        ),
    ] {
        assert_character_chirho(source_chirho, expected_chirho);
    }
}

#[test]
fn missing_syntax_names_and_wrong_or_namesake_input_types_never_publish_core_chirho() {
    for (source_chirho, expected_chirho) in [
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/negative-chirho/MissingConversionChirho.hs"
            ),
            NextErrorKindChirho::InvalidChirho,
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/negative-chirho/MissingNegateChirho.hs"
            ),
            NextErrorKindChirho::InvalidChirho,
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/negative-chirho/MissingIfChirho.hs"
            ),
            NextErrorKindChirho::InvalidChirho,
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/negative-chirho/WrongInputChirho.hs"
            ),
            NextErrorKindChirho::TypeChirho,
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/negative-chirho/NominalIntegerChirho.hs"
            ),
            NextErrorKindChirho::TypeChirho,
        ),
    ] {
        let error_chirho = compile_source_chirho(source_chirho).unwrap_err();
        assert_eq!(error_chirho.kind_chirho, expected_chirho, "{error_chirho}");
    }
}

#[test]
fn ordinary_numeric_defaults_imported_syntax_fractionals_and_patterns_stay_unsupported_chirho() {
    for source_chirho in [
        include_str!(
            "../fixtures_chirho/integer_literals_chirho/unsupported-chirho/OrdinaryChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/integer_literals_chirho/unsupported-chirho/MagicHashChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/integer_literals_chirho/unsupported-chirho/Haskell2010SeparatorsChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/integer_literals_chirho/unsupported-chirho/FractionalChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/integer_literals_chirho/unsupported-chirho/ImportedPreludeChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/integer_literals_chirho/unsupported-chirho/LiteralPatternChirho.hs"
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

#[test]
fn rebindable_syntax_does_not_supply_an_implicit_prelude_chirho() {
    let source_chirho = include_str!(
        "../fixtures_chirho/integer_literals_chirho/negative-chirho/NoPreludeChirho.hs"
    );
    assert_eq!(
        compile_source_chirho(source_chirho)
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::InvalidChirho
    );
}

#[test]
fn missing_literal_producer_data_and_oversized_payloads_refuse_chirho() {
    let source_chirho = include_str!(
        "../fixtures_chirho/integer_literals_chirho/positive-chirho/ExactHugeChirho.hs"
    );
    let parsed_chirho = super::parse_chirho::parse_chirho(
        source_chirho,
        haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
    )
    .unwrap();
    let error_chirho = haskelujah_hir_chirho::lower_module_chirho(
        &parsed_chirho.module_chirho,
        haskelujah_hir_chirho::UnitIdentityChirho::StandaloneHomeChirho,
        haskelujah_hir_chirho::LimitsChirho::default(),
    )
    .unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        haskelujah_hir_chirho::HirErrorKindChirho::UnsupportedChirho
    );
    let too_big_chirho = source_chirho.replace(
        "340282366920938463463374607431768211457",
        &"1".repeat(
            haskelujah_runtime_chirho::integer_literal_chirho::MAX_INTEGER_LITERAL_BYTES_CHIRHO + 1,
        ),
    );
    assert_eq!(
        compile_source_chirho(&too_big_chirho)
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::ResourceChirho
    );
}

#[test]
fn native_consumers_refuse_opaque_integers_before_emission_chirho() {
    let core_chirho = compile_source_chirho(include_str!(
        "../fixtures_chirho/integer_literals_chirho/positive-chirho/ExactHugeChirho.hs"
    ))
    .unwrap();
    for result_chirho in [
        haskelujah_backend_llvm_chirho::try_compile_core_to_llvm_chirho(&core_chirho).map(|_| ()),
        haskelujah_backend_llvm_chirho::try_compile_core_to_llvm_executable_chirho(&core_chirho)
            .map(|_| ()),
        haskelujah_backend_wasm_chirho::try_compile_core_to_wasm_chirho(&core_chirho).map(|_| ()),
        haskelujah_backend_wasm_chirho::try_compile_core_to_wasm_executable_chirho(&core_chirho)
            .map(|_| ()),
        haskelujah_backend_cranelift_chirho::compile_core_to_object_chirho(
            &core_chirho,
            &Default::default(),
        )
        .map(|_| ()),
        haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
            &core_chirho,
            &Default::default(),
        )
        .map(|_| ()),
        haskelujah_backend_cranelift_chirho::jit_chirho::jit_execute_chirho(&core_chirho)
            .map(|_| ()),
    ] {
        assert!(result_chirho.unwrap_err().contains("Integer"));
    }
}

#[test]
fn a_shared_integer_survives_thunk_update_and_repeated_demand_chirho() {
    let source_chirho =
        include_str!("../fixtures_chirho/integer_literals_chirho/positive-chirho/SharedChirho.hs");
    let (value_chirho, mut machine_chirho) =
        eval_binding_chirho(source_chirho, "resultChirho", 50_000).unwrap();
    let ValueChirho::HeapPtrChirho(address_chirho) = value_chirho else {
        panic!("expected tuple");
    };
    let fields_chirho = machine_chirho
        .heap_chirho
        .read_chirho(machine_chirho.heap_chirho.follow_ind_chirho(address_chirho))
        .payload_chirho
        .clone();
    for field_chirho in fields_chirho {
        let value_chirho = machine_chirho.force_to_prim_chirho(field_chirho);
        let ValueChirho::IntegerChirho(integer_chirho) = value_chirho else {
            panic!("expected shared Integer, got {value_chirho}");
        };
        assert_eq!(
            integer_chirho.to_string(),
            "340282366920938463463374607431768211457"
        );
    }
}

#[test]
fn literal_separators_and_radix_prefixes_follow_the_measured_ghc_profile_chirho() {
    for source_chirho in [
        include_str!(
            "../fixtures_chirho/integer_literals_chirho/lexical_chirho/TrailingUnderscoreChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/integer_literals_chirho/lexical_chirho/BrokenPrefixChirho.hs"
        ),
        include_str!("../fixtures_chirho/integer_literals_chirho/lexical_chirho/HashChirho.hs"),
    ] {
        assert_eq!(
            compile_source_chirho(source_chirho)
                .unwrap_err()
                .kind_chirho,
            NextErrorKindChirho::InvalidChirho
        );
    }
    for (source_chirho, expected_chirho) in [
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/lexical_chirho/DoubleUnderscoreChirho.hs"
            ),
            "12",
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/lexical_chirho/HexUnderscoreChirho.hs"
            ),
            "255",
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/lexical_chirho/BinaryUnderscoreChirho.hs"
            ),
            "1",
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/lexical_chirho/OctalUnderscoreChirho.hs"
            ),
            "7",
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_literals_chirho/lexical_chirho/LeadingZeroChirho.hs"
            ),
            "1",
        ),
    ] {
        let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 50_000).unwrap();
        let ValueChirho::IntegerChirho(integer_chirho) = value_chirho else {
            panic!("expected Integer");
        };
        assert_eq!(integer_chirho.to_string(), expected_chirho);
    }
}
