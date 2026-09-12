// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Prefix promotion must retain the constructor and its actual argument kinds.
use super::{assert_compile_success_chirho, assert_kind_error_chirho};

#[test]
fn promoted_symbol_prefix_agrees_with_list_syntax_chirho() {
    assert_compile_success_chirho(
        "PromotedPrefixChirho.hs",
        include_str!(
            "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/sources-chirho/prefix_chirho.hs"
        ),
    );
}

#[test]
fn promoted_symbol_field_agrees_with_list_syntax_chirho() {
    assert_compile_success_chirho(
        "PromotedFieldChirho.hs",
        include_str!(
            "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/sources-chirho/field_chirho.hs"
        ),
    );
}

#[test]
fn qualified_promoted_symbol_agrees_with_infix_syntax_chirho() {
    assert_compile_success_chirho(
        "PromotedQualifiedChirho.hs",
        include_str!(
            "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/sources-chirho/qualified_chirho.hs"
        ),
    );
}

#[test]
fn promoted_symbol_tail_must_be_a_list_chirho() {
    assert_kind_error_chirho(include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/sources-chirho/wrong_tail_chirho.hs"
    ));
}

#[test]
fn promoted_symbol_field_tail_must_be_a_list_chirho() {
    assert_kind_error_chirho(include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/sources-chirho/wrong_field_chirho.hs"
    ));
}

#[test]
fn qualified_promoted_family_pattern_matches_the_local_constructor_chirho() {
    assert_compile_success_chirho(
        "QualifiedFamilyChirho.hs",
        include_str!(
            "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/sources-chirho/qualified_family_chirho.hs"
        ),
    );
}

#[test]
fn qualified_promoted_associated_pattern_reduces_chirho() {
    assert_compile_success_chirho(
        "AssociatedPatternChirho.hs",
        include_str!(
            "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/associated-chirho/pattern_chirho.hs"
        ),
    );
}

#[test]
fn qualified_promoted_associated_result_reduces_chirho() {
    assert_compile_success_chirho(
        "AssociatedResultChirho.hs",
        include_str!(
            "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/associated-chirho/result_chirho.hs"
        ),
    );
}

#[test]
fn qualified_promoted_associated_default_reduces_chirho() {
    assert_compile_success_chirho(
        "AssociatedDefaultChirho.hs",
        include_str!(
            "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/associated-chirho/default_chirho.hs"
        ),
    );
}

fn assert_coordinate_error_chirho(source_chirho: &str) {
    let error_chirho = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut super::SourceMapChirho::new_chirho(),
        "WrongAssociatedChirho.hs",
    )
    .map(|_| ())
    .expect_err("the selected coordinate must constrain the result");
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| {
                diagnostic_chirho.code_chirho
                    == Some(haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(200))
                    && diagnostic_chirho.message_chirho.contains("Int")
                    && diagnostic_chirho.message_chirho.contains("Bool")
            }),
        "wrong coordinate must fail at its actual type, not qualified spelling: {error_chirho}",
    );
}

#[test]
fn qualified_promoted_associated_pattern_checks_the_coordinate_chirho() {
    assert_coordinate_error_chirho(include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/associated-chirho/wrong_pattern_chirho.hs"
    ));
}

#[test]
fn qualified_promoted_associated_result_checks_the_coordinate_chirho() {
    assert_coordinate_error_chirho(include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/associated-chirho/wrong_result_chirho.hs"
    ));
}

#[test]
fn qualified_promoted_associated_default_checks_the_coordinate_chirho() {
    assert_coordinate_error_chirho(include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-symbols-chirho/associated-chirho/wrong_default_chirho.hs"
    ));
}
