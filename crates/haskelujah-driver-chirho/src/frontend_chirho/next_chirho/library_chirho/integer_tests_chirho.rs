// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::super::{check_source_chirho, compile_source_chirho, eval_binding_chirho};
use super::*;
use haskelujah_runtime_chirho::ValueChirho;

const EDGES_CHIRHO: &str = include_str!(
    "../fixtures_chirho/integer_arithmetic_chirho/positive-chirho/IntegerEdgesChirho.hs"
);

fn assert_integer_chirho(source_chirho: &str, entry_chirho: &str, expected_chirho: &str) {
    let (value_chirho, _) = eval_binding_chirho(source_chirho, entry_chirho, 100_000).unwrap();
    let ValueChirho::IntegerChirho(value_chirho) = value_chirho else {
        panic!("expected opaque Integer, got {value_chirho}");
    };
    assert_eq!(value_chirho.to_string(), expected_chirho, "{entry_chirho}");
}

#[test]
fn actual_num_integer_dispatch_matches_the_ghc_representation_edges_chirho() {
    for (entry_chirho, expected_chirho) in [
        ("overflowChirho", "9223372036854775808"),
        ("negativeEdgeChirho", "9223372036854775808"),
        ("absoluteEdgeChirho", "9223372036854775808"),
        ("cancelChirho", "0"),
        ("productChirho", "340282366920938463463374607431768211456"),
        (
            "negativeProductChirho",
            "-340282366920938463463374607431768211456",
        ),
        ("negativeSignChirho", "-1"),
        ("zeroSignChirho", "0"),
        ("positiveSignChirho", "1"),
        ("subtractChirho", "-2"),
    ] {
        assert_integer_chirho(EDGES_CHIRHO, entry_chirho, expected_chirho);
    }
}

#[test]
fn unsaturated_functions_and_home_names_keep_their_original_identity_chirho() {
    let source_chirho = include_str!(
        "../fixtures_chirho/integer_arithmetic_chirho/positive-chirho/IntegerUnsaturatedChirho.hs"
    );
    assert_integer_chirho(source_chirho, "resultChirho", "42");
    assert_integer_chirho(source_chirho, "negativeChirho", "-41");
    assert_integer_chirho(
        include_str!(
            "../fixtures_chirho/integer_arithmetic_chirho/positive-chirho/IntegerNamesChirho.hs"
        ),
        "resultChirho",
        "11",
    );
}

#[test]
fn arithmetic_results_survive_shared_thunk_update_chirho() {
    assert_integer_chirho(
        include_str!(
            "../fixtures_chirho/integer_arithmetic_chirho/positive-chirho/IntegerSharingChirho.hs"
        ),
        "resultChirho",
        "680564733841876926926749214863536422916",
    );
}

#[test]
fn zero_multiplication_demands_both_arguments_and_propagates_failure_chirho() {
    for source_chirho in [
        include_str!(
            "../fixtures_chirho/integer_arithmetic_chirho/boundary-chirho/IntegerFailureLeftChirho.hs"
        ),
        include_str!(
            "../fixtures_chirho/integer_arithmetic_chirho/boundary-chirho/IntegerFailureRightChirho.hs"
        ),
    ] {
        check_source_chirho(source_chirho).unwrap();
        let error_chirho = eval_binding_chirho(source_chirho, "resultChirho", 50_000).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::ExecutionChirho,
            "{error_chirho}"
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
fn visible_presence_refuses_but_slice_absence_and_defaulting_still_decline_chirho() {
    for (source_chirho, expected_chirho) in [
        (
            include_str!(
                "../fixtures_chirho/integer_arithmetic_chirho/boundary-chirho/DuplicateNumIntegerChirho.hs"
            ),
            NextErrorKindChirho::TypeChirho,
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_arithmetic_chirho/boundary-chirho/IntegerTypeMismatchChirho.hs"
            ),
            NextErrorKindChirho::TypeChirho,
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_arithmetic_chirho/boundary-chirho/IntegerNoDefaultChirho.hs"
            ),
            NextErrorKindChirho::TypeChirho,
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_arithmetic_chirho/boundary-chirho/IntegerClosedLayoutChirho.hs"
            ),
            NextErrorKindChirho::UnsupportedChirho,
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_arithmetic_chirho/boundary-chirho/IntegerMissingQuotChirho.hs"
            ),
            NextErrorKindChirho::UnsupportedChirho,
        ),
    ] {
        let error_chirho = compile_source_chirho(source_chirho).unwrap_err();
        assert_eq!(error_chirho.kind_chirho, expected_chirho, "{error_chirho}");
    }
}

#[test]
fn arithmetic_exhaustion_is_a_resource_result_without_a_published_value_chirho() {
    let source_chirho = include_str!(
        "../fixtures_chirho/integer_arithmetic_chirho/boundary-chirho/IntegerResourceChirho.hs"
    );
    check_source_chirho(source_chirho).unwrap();
    let error_chirho = eval_binding_chirho(source_chirho, "resultChirho", 100_000).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::ResourceChirho,
        "{error_chirho}"
    );
}

#[test]
fn primitive_bodies_require_a_defining_manifest_and_the_exact_checked_signature_chirho() {
    let source_chirho = INTEGER_CHIRHO.source_chirho().unwrap();
    for source_chirho in [
        source_chirho.replace(
            "integerAdd :: Integer -> Integer -> Integer",
            "integerAdd :: Integer -> Integer",
        ),
        source_chirho.replace(
            "integerAdd :: Integer -> Integer -> Integer",
            "integerAdd :: Integer -> Integer -> (Integer, Integer)",
        ),
        source_chirho.replace(
            "integerAdd :: Integer -> Integer -> Integer",
            "integerAdd :: aChirho -> aChirho -> aChirho",
        ),
        source_chirho.replace("integerAdd :: Integer -> Integer -> Integer\n", ""),
    ] {
        let parsed_chirho = super::super::parse_chirho::parse_chirho(
            &source_chirho,
            haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
        )
        .unwrap();
        let mut policy_chirho = LoweringPolicyChirho::default();
        install_integer_policy_chirho(&mut policy_chirho);
        let error_chirho = super::super::check_parsed_with_policy_chirho(
            parsed_chirho,
            UnitIdentityChirho::CompilerChirho,
            &Default::default(),
            &policy_chirho,
        )
        .unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{error_chirho}"
        );
    }
    let error_chirho = check_source_chirho(&source_chirho).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
}

#[test]
fn arithmetic_without_a_literal_is_refused_by_every_unimplemented_native_bridge_chirho() {
    let source_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule PrimitiveOnlyChirho where\nimport GHC.Internal.Bignum.Integer (Integer, integerAdd)\nplusChirho :: Integer -> Integer -> Integer\nplusChirho = integerAdd\n";
    let core_chirho = compile_source_chirho(source_chirho).unwrap();
    assert!(
        !core_chirho.uses_primitive_chirho(
            haskelujah_core_chirho::expr_chirho::INTEGER_LITERAL_PRIMOP_CHIRHO
        )
    );
    assert!(core_chirho.requires_opaque_integer_runtime_chirho());
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
fn gc_and_deep_strict_demand_preserve_roots_without_host_recursion_chirho() {
    for (source_chirho, expected_chirho) in [
        (
            include_str!(
                "../fixtures_chirho/integer_arithmetic_chirho/stress-chirho/IntegerGcDemandChirho.hs"
            ),
            "12",
        ),
        (
            include_str!(
                "../fixtures_chirho/integer_arithmetic_chirho/stress-chirho/IntegerDeepDemandChirho.hs"
            ),
            "10000",
        ),
    ] {
        let (value_chirho, machine_chirho) =
            eval_binding_chirho(source_chirho, "resultChirho", 5_000_000).unwrap();
        assert_eq!(value_chirho.to_string(), expected_chirho);
        assert!(
            machine_chirho.last_gc_stats_chirho.is_some(),
            "stress control never crossed a collection"
        );
        assert!(!machine_chirho.integer_budget_chirho.exhausted_chirho());
        println!(
            "INTEGER_STRESS_CHIRHO result_chirho={expected_chirho} steps_chirho={} allocations_chirho={} code_entries_chirho={} last_gc_chirho={:?}",
            machine_chirho.steps_chirho,
            machine_chirho.heap_chirho.alloc_count_chirho(),
            machine_chirho.code_table_chirho.len(),
            machine_chirho.last_gc_stats_chirho
        );
    }
    assert_integer_chirho(
        include_str!(
            "../fixtures_chirho/integer_arithmetic_chirho/stress-chirho/IntegerLazyPartialChirho.hs"
        ),
        "resultChirho",
        "7",
    );
}
