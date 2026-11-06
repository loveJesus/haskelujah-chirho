// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::{SourceMapChirho, compile_source_chirho};

#[test]
fn signed_numeric_values_retain_distinct_nominal_types_chirho() {
    let cases_chirho = [
        (
            "IntToDoubleChirho.hs",
            include_str!("fixtures_chirho/negative_chirho/int_to_double_chirho.hs"),
        ),
        (
            "DoubleToIntChirho.hs",
            include_str!("fixtures_chirho/negative_chirho/double_to_int_chirho.hs"),
        ),
        (
            "NestedNumericListChirho.hs",
            include_str!("fixtures_chirho/negative_chirho/nested_numeric_list_chirho.hs"),
        ),
    ];
    // Collect every refusal before asserting, so an earlier wrong acceptance
    // cannot hide the reverse or nested case.
    let accepted_chirho: Vec<_> = cases_chirho
        .into_iter()
        .filter_map(|(file_chirho, source_chirho)| {
            compile_source_chirho(
                source_chirho,
                &mut SourceMapChirho::new_chirho(),
                file_chirho,
            )
            .is_ok()
            .then_some(file_chirho)
        })
        .collect();
    assert!(
        accepted_chirho.is_empty(),
        "distinct signed numeric types were accepted: {accepted_chirho:?}"
    );
}

#[test]
fn numeric_literals_conversions_and_polymorphic_functions_remain_checkable_chirho() {
    let cases_chirho = [
        (
            "IntLiteralChirho.hs",
            include_str!("fixtures_chirho/positive_chirho/int_literal_chirho.hs"),
        ),
        (
            "DoubleLiteralsChirho.hs",
            include_str!("fixtures_chirho/positive_chirho/double_literals_chirho.hs"),
        ),
        (
            "ExplicitConversionChirho.hs",
            include_str!("fixtures_chirho/positive_chirho/explicit_conversion_chirho.hs"),
        ),
        (
            "NumericDefaultChirho.hs",
            include_str!("fixtures_chirho/positive_chirho/numeric_default_chirho.hs"),
        ),
        (
            "PolymorphicIdentityChirho.hs",
            include_str!("fixtures_chirho/positive_chirho/polymorphic_identity_chirho.hs"),
        ),
        (
            "PolymorphicNumericFunctionChirho.hs",
            include_str!("fixtures_chirho/positive_chirho/polymorphic_numeric_function_chirho.hs"),
        ),
        (
            "NumericPatternChirho.hs",
            include_str!("fixtures_chirho/positive_chirho/numeric_pattern_chirho.hs"),
        ),
    ];
    let rejected_chirho: Vec<_> = cases_chirho
        .into_iter()
        .filter_map(|(file_chirho, source_chirho)| {
            compile_source_chirho(
                source_chirho,
                &mut SourceMapChirho::new_chirho(),
                file_chirho,
            )
            .err()
            .map(|error_chirho| (file_chirho, error_chirho.to_string()))
        })
        .collect();
    assert!(
        rejected_chirho.is_empty(),
        "valid numeric producers or conversions were refused: {rejected_chirho:?}"
    );
}
