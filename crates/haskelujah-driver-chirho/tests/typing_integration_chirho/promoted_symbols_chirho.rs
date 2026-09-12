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
