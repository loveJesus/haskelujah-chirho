// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! End-to-end ordinary-data and pattern semantics, including evaluation order and failure.

use super::{NextErrorKindChirho, compile_source_chirho, eval_binding_chirho};
use haskelujah_runtime_chirho::ValueChirho;

#[test]
fn constructors_instantiate_freshly_and_remain_first_class_chirho() {
    let source_chirho = include_str!(
        "fixtures_chirho/data_patterns_chirho/positive-chirho/ConstructorCaseChirho.hs"
    );
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 20_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('a'));
}

#[test]
fn recursive_and_mutually_recursive_data_use_checked_constructor_accounts_chirho() {
    let source_chirho =
        include_str!("fixtures_chirho/data_patterns_chirho/positive-chirho/RecursiveDataChirho.hs");
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 20_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('r'));
}

#[test]
fn source_order_and_left_to_right_demand_survive_nested_fallthrough_chirho() {
    let source_chirho = include_str!(
        "fixtures_chirho/data_patterns_chirho/positive-chirho/OrderedPatternsChirho.hs"
    );
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 20_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('c'));
}

#[test]
fn variables_wildcards_and_unused_fields_do_not_force_bottom_chirho() {
    let source_chirho =
        include_str!("fixtures_chirho/data_patterns_chirho/positive-chirho/LazyPatternsChirho.hs");
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 20_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('f'));
}

#[test]
fn list_tuple_and_as_patterns_preserve_aliases_and_string_list_equivalence_chirho() {
    let source_chirho =
        include_str!("fixtures_chirho/data_patterns_chirho/positive-chirho/ListTupleAsChirho.hs");
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 20_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('e'));
}

#[test]
fn nominal_data_types_and_applied_annotations_execute_chirho() {
    let source_chirho =
        include_str!("fixtures_chirho/data_patterns_chirho/positive-chirho/NominalDataChirho.hs");
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 20_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('i'));
}

#[test]
fn invalid_constructor_types_patterns_and_kinds_never_publish_core_chirho() {
    for source_chirho in [
        include_str!(
            "fixtures_chirho/data_patterns_chirho/negative-chirho/ConstructorFieldChirho.hs"
        ),
        include_str!("fixtures_chirho/data_patterns_chirho/negative-chirho/PatternArityChirho.hs"),
        include_str!(
            "fixtures_chirho/data_patterns_chirho/negative-chirho/NominalMismatchChirho.hs"
        ),
        include_str!("fixtures_chirho/data_patterns_chirho/negative-chirho/KindArityChirho.hs"),
    ] {
        assert_eq!(
            compile_source_chirho(source_chirho)
                .unwrap_err()
                .kind_chirho,
            NextErrorKindChirho::TypeChirho,
            "{source_chirho}"
        );
    }
    assert_eq!(
        compile_source_chirho(include_str!(
            "fixtures_chirho/data_patterns_chirho/negative-chirho/PatternDuplicateChirho.hs"
        ))
        .unwrap_err()
        .kind_chirho,
        NextErrorKindChirho::InvalidChirho
    );
}

#[test]
fn an_unmatched_value_fails_at_runtime_instead_of_returning_a_constructor_chirho() {
    let source_chirho =
        include_str!("fixtures_chirho/data_patterns_chirho/positive-chirho/PartialMatchChirho.hs");
    compile_source_chirho(source_chirho).unwrap();
    let error_chirho = eval_binding_chirho(source_chirho, "resultChirho", 20_000)
        .err()
        .unwrap();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::ExecutionChirho
    );
}

#[test]
fn large_core_expansions_stop_with_resource_errors_and_leave_the_checker_reusable_chirho() {
    let wide_chirho = format!(
        "module WideDataChirho where\ndata WideChirho = WideChirho {}\n",
        "Char ".repeat(600)
    );
    assert_eq!(
        compile_source_chirho(&wide_chirho).unwrap_err().kind_chirho,
        NextErrorKindChirho::ResourceChirho
    );
    let rows_chirho = format!(
        "module WideMatchChirho where\n{}",
        "pickChirho True = 'x'\n".repeat(600)
    );
    assert_eq!(
        compile_source_chirho(&rows_chirho).unwrap_err().kind_chirho,
        NextErrorKindChirho::ResourceChirho
    );
    compile_source_chirho(include_str!(
        "fixtures_chirho/data_patterns_chirho/positive-chirho/ConstructorCaseChirho.hs"
    ))
    .unwrap();
}
