// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 694: a newtype whose classes come through the newtype strategy has one representation
//! in the whole program. Its home module erases its constructor from Core, and the multi-module
//! merge erases an importer's construction, match and selector use of it too, so a value
//! crosses a NOINLINE helper and the imported GND rows as its field. Row 629's harness runs
//! the program on the interpreter, LLVM and true Cranelift. The expected output is GHC's
//! semantics; its GHC 9.14.1 run is still to be measured.
//! workflow: language-features-chirho/deriving-strategies-chirho

use super::import_identity_chirho::{ProgramChirho, assert_exact_chirho};

const IMPORTED_AGES_CHIRHO: &str = include_str!(
    "../typing_integration_chirho/declarations_chirho/deriving_strategies_chirho/imported_ages_chirho.hs"
);
const IMPORTED_MAIN_CHIRHO: &str = include_str!(
    "../typing_integration_chirho/declarations_chirho/deriving_strategies_chirho/imported_main_chirho.hs"
);

/// `AgesChirho` derives Show and Num with the newtype strategy; Main constructs, matches and
/// selects its newtype and passes it through `bumpChirho`. GHC's semantics print 6, 7, 8, 90, 7.
const IMPORTED_GND_NEWTYPE_CHIRHO: (ProgramChirho, &str) = (
    &[
        ("AgesChirho", IMPORTED_AGES_CHIRHO),
        ("Main", IMPORTED_MAIN_CHIRHO),
    ],
    "6\n7\n8\n90\n7\n",
);

#[test]
fn an_imported_gnd_newtype_keeps_one_representation_chirho() {
    assert_exact_chirho(IMPORTED_GND_NEWTYPE_CHIRHO);
}
