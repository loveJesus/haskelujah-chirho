// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 694: a newtype whose classes come through the newtype strategy has one representation
//! in the whole program. Its home module erases its constructor from Core, and the multi-module
//! merge erases an importer's construction, match and selector use of it too, so a value
//! crosses a NOINLINE helper and the imported GND rows as its field. Row 629's harness runs
//! the program on the interpreter, LLVM and true Cranelift. The expected output is GHC's
//! semantics; its GHC 9.14.1 run is still to be measured.
//! workflow: language-features-chirho/deriving-strategies-chirho

use std::time::Duration;

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_project_round_trip_chirho,
};

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

const HOMONYM_KEYS_CHIRHO: &str = include_str!(
    "../typing_integration_chirho/declarations_chirho/deriving_strategies_chirho/homonym_chirho/homonym_keys_chirho.hs"
);
const HOMONYM_MAIN_CHIRHO: &str = include_str!(
    "../typing_integration_chirho/declarations_chirho/deriving_strategies_chirho/homonym_chirho/homonym_main_chirho.hs"
);

/// `KeysChirho` derives through its newtype constructor `KeyChirho`; Main imports it qualified
/// and declares its own two-field data constructor `KeyChirho`. GHC's semantics print 7 twice
/// (its GHC 9.14.1 run is still to be measured). Merged Core names constructors unqualified, so
/// a native merge must keep Main's constructor intact or refuse the spelling by name; it may
/// never erase it as if it were the newtype's (#31757).
#[test]
fn a_data_homonym_of_a_gnd_newtype_constructor_is_never_erased_chirho() {
    let modules_chirho: ProgramChirho = &[
        ("KeysChirho", HOMONYM_KEYS_CHIRHO),
        ("Main", HOMONYM_MAIN_CHIRHO),
    ];
    let (_, machine_chirho) = haskelujah_driver::eval_modules_with_machine_chirho(
        &[
            ("KeysChirho.hs", HOMONYM_KEYS_CHIRHO),
            ("Main.hs", HOMONYM_MAIN_CHIRHO),
        ],
        &mut SourceMapChirho::new_chirho(),
        None,
    )
    .unwrap_or_else(|error_chirho| panic!("interpreter: {error_chirho}"));
    assert_eq!(machine_chirho.io_output_chirho, "7\n7\n", "interpreter");
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        match native_project_round_trip_chirho(
            modules_chirho,
            backend_chirho,
            "",
            Duration::from_secs(30),
        ) {
            Ok(outcome_chirho) => assert_eq!(
                outcome_chirho,
                (0, "7\n7\n".to_string()),
                "{backend_chirho:?}"
            ),
            Err(error_chirho) => assert!(
                error_chirho
                    .contains("cannot give newtype constructor `KeyChirho` one representation"),
                "{backend_chirho:?}: {error_chirho}"
            ),
        }
    }
}
