// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! End-to-end required arguments and rank-N visible-application regressions.
//!
//! workflow: language-features-chirho/rank-n-visible-type-application-chirho

use crate::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

const T17594F_SOURCE_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../ghc-tests-chirho/typecheck-chirho/should_compile/T17594f.hs"
));
const VTA1_SOURCE_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../ghc-tests-chirho/typecheck-chirho/should_compile/Vta1.hs"
));
const VTA2_SOURCE_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../ghc-tests-chirho/typecheck-chirho/should_compile/Vta2.hs"
));

#[test]
fn typecheck_t17594f_required_and_visible_type_arguments_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(T17594F_SOURCE_CHIRHO, &mut source_map_chirho, "T17594f.hs");

    assert!(
        result_chirho.is_ok(),
        "T17594f RequiredTypeArguments regression should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn typecheck_vta1_type_family_result_specialization_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(VTA1_SOURCE_CHIRHO, &mut source_map_chirho, "Vta1.hs");

    assert!(
        result_chirho.is_ok(),
        "Vta1 type-family/VTA regression should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn typecheck_vta2_rank_n_expected_control_flow_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(VTA2_SOURCE_CHIRHO, &mut source_map_chirho, "Vta2.hs");

    assert!(
        result_chirho.is_ok(),
        "Vta2 rank-N expected-type regression should type-check: {:?}",
        result_chirho.err()
    );
}
