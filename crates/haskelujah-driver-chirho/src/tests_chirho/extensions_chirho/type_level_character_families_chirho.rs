// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! End-to-end TypeLits character-family regressions.
//!
//! workflow: language-features-chirho/type-level-character-families-chirho

use super::{SourceMapChirho, compile_source_chirho};

const T14934A_SOURCE_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../ghc-tests-chirho/typecheck-chirho/should_compile/T14934a.hs"
));
const T19535_SOURCE_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../ghc-tests-chirho/typecheck-chirho/should_compile/T19535.hs"
));

#[test]
fn typecheck_t14934a_char_to_nat_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(T14934A_SOURCE_CHIRHO, &mut source_map_chirho, "T14934a.hs");

    assert!(
        result_chirho.is_ok(),
        "T14934a CharToNat regression should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn typecheck_t19535_bidirectional_character_families_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(T19535_SOURCE_CHIRHO, &mut source_map_chirho, "T19535.hs");

    assert!(
        result_chirho.is_ok(),
        "T19535 CharToNat/NatToChar regression should type-check: {:?}",
        result_chirho.err()
    );
}
