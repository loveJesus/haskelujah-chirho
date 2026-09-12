// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Sources independently checked by GHC9.14.1; negative tests pin the actual phase.
use super::{assert_compile_success_chirho, assert_kind_error_chirho};

#[test]
fn tuple_projections_chirho() {
    let source_chirho = include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-tuples-chirho/sources-chirho/tuple_projections_chirho.hs"
    );
    assert_compile_success_chirho("TupleProjectionsChirho.hs", source_chirho);
}

#[test]
fn tuple_wrong_coordinate_chirho() {
    let source_chirho = include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-tuples-chirho/sources-chirho/tuple_wrong_coordinate_chirho.hs"
    );
    let error_chirho = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut haskelujah_span_chirho::SourceMapChirho::new_chirho(),
        "WrongCoordinateChirho.hs",
    )
    .err()
    .expect("the first coordinate is Int, not Char");
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| diagnostic_chirho.code_chirho
                == Some(haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(200))),
        "{error_chirho}"
    );
}

#[test]
fn tuple_prefix_chirho() {
    let source_chirho = include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-tuples-chirho/sources-chirho/tuple_prefix_chirho.hs"
    );
    assert_compile_success_chirho("TuplePrefixChirho.hs", source_chirho);
}

#[test]
fn tuple_nested_chirho() {
    let source_chirho = include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-tuples-chirho/sources-chirho/tuple_nested_chirho.hs"
    );
    assert_compile_success_chirho("TupleNestedChirho.hs", source_chirho);
}

#[test]
fn tuple_mixed_kinds_chirho() {
    let source_chirho = include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-tuples-chirho/sources-chirho/tuple_mixed_kinds_chirho.hs"
    );
    assert_compile_success_chirho("TupleMixedKindsChirho.hs", source_chirho);
}

#[test]
fn tuple_wrong_kind_chirho() {
    let source_chirho = include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-tuples-chirho/sources-chirho/tuple_wrong_kind_chirho.hs"
    );
    assert_kind_error_chirho(source_chirho);
}

#[test]
fn tuple_field_chirho() {
    let source_chirho = include_str!(
        "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/promoted-tuples-chirho/sources-chirho/tuple_field_chirho.hs"
    );
    assert_compile_success_chirho("TupleFieldChirho.hs", source_chirho);
}
