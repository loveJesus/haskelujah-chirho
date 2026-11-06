// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A first-class `print` at a binding's own `Show` dictionary (row 698).
//!
//! Row 697 serves an applied `print` at an indexed own `Show` key through that parameter's
//! `show`. A first-class `print` with the same proof, such as the one in `mapM_ print xs` inside
//! `f :: Show a => [a] -> IO ()`, stayed unserved: the first-class path named only a concrete
//! `$prim_Show_show_<key>` row. The measured baseline at main
//! (`tmp-chirho/row698-gate-chirho/b04`) printed GHC's output on the interpreter. LLVM and
//! Cranelift printed `97 98` for `'a' 'b'` and `1 0` for `True False`. A first-class `print`
//! now takes the same proof as a value, over that parameter's `show`.
//!
//! Each probe compares every engine with GHC 9.14.1 in one assertion. A failing run therefore
//! keeps each engine's exit status and stdout, or that engine's failure message:
//! - P1: `mapM_ print` at Char.
//! - P2: two Show predicates, Char and Bool, each printed first-class.
//! - P3: a qualified `P.print` beside the program's own `print`, the latter applied and
//!   first-class.
//!
//! Each program is embedded byte for byte from the child directory
//! `first_class_show_print_chirho/`, and every `main` has the type `IO ()`. GHC 9.14.1 measured
//! each one: `tmp-chirho/ghc-contract-refresh-chirho/first-class-show-print-r1-chirho`.
//! workflow: print-show-evidence-chirho

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

/// One engine's run: its exit status and stdout, or its failure message.
type EngineRunChirho = (&'static str, Result<(i32, String), String>);

/// P1 (sha256 49b2ad06). GHC 9.14.1 prints `'a'` then `'b'`.
const FIRST_CLASS_PRINT_CHAR_CHIRHO: &str =
    include_str!("first_class_show_print_chirho/first_class_print_char_chirho.hs");

/// P2 (sha256 d64abb8c). GHC 9.14.1 prints `'a'`, `'b'`, `True`, `False`.
const TWO_SHOW_PREDICATES_CHIRHO: &str =
    include_str!("first_class_show_print_chirho/two_show_predicates_chirho.hs");

/// P3 (sha256 92c45f93). GHC 9.14.1 prints `own hello`, `own x`, `own y`, `'a'`, `'b'`.
const QUALIFIED_PRINT_BESIDE_OWN_CHIRHO: &str =
    include_str!("first_class_show_print_chirho/qualified_print_beside_own_chirho.hs");

/// The interpreter, then LLVM and Cranelift through the native harness, each run alone with
/// no fallback.
fn every_engine_chirho(file_name_chirho: &str, source_chirho: &str) -> Vec<EngineRunChirho> {
    let interpreter_chirho = eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
        None,
    )
    .map(|(_value_chirho, machine_chirho)| (0, machine_chirho.io_output_chirho));
    vec![
        ("interpreter", interpreter_chirho),
        (
            "llvm",
            native_round_trip_chirho(source_chirho, NativeBackendChirho::LlvmChirho, ""),
        ),
        (
            "cranelift",
            native_round_trip_chirho(source_chirho, NativeBackendChirho::CraneliftChirho, ""),
        ),
    ]
}

/// GHC's stdout and exit 0 on every engine.
fn as_ghc_everywhere_chirho(ghc_chirho: &str) -> Vec<EngineRunChirho> {
    ["interpreter", "llvm", "cranelift"]
        .into_iter()
        .map(|engine_chirho| (engine_chirho, Ok((0, ghc_chirho.to_string()))))
        .collect()
}

#[test]
fn first_class_print_at_an_own_char_dictionary_matches_ghc_everywhere_chirho() {
    assert_eq!(
        every_engine_chirho(
            "FirstClassPrintCharChirho.hs",
            FIRST_CLASS_PRINT_CHAR_CHIRHO
        ),
        as_ghc_everywhere_chirho("'a'\n'b'\n"),
    );
}

#[test]
fn two_own_show_dictionaries_each_print_their_own_type_everywhere_chirho() {
    assert_eq!(
        every_engine_chirho("TwoShowPredicatesChirho.hs", TWO_SHOW_PREDICATES_CHIRHO),
        as_ghc_everywhere_chirho("'a'\n'b'\nTrue\nFalse\n"),
    );
}

#[test]
fn qualified_print_at_an_own_dictionary_beside_the_programs_own_print_everywhere_chirho() {
    assert_eq!(
        every_engine_chirho(
            "QualifiedPrintBesideOwnChirho.hs",
            QUALIFIED_PRINT_BESIDE_OWN_CHIRHO,
        ),
        as_ghc_everywhere_chirho("own hello\nown x\nown y\n'a'\n'b'\n"),
    );
}
