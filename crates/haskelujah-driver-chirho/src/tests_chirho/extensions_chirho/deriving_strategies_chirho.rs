// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 694 deriving-strategy controls on the interpreter alone, so a unit stage can measure
//! them before any native stage. The programs are the shared fixtures of
//! `tests/typing_integration_chirho/declarations_chirho/deriving_strategies_chirho/`, which
//! the three-engine controls run too. Only `newtype_show` and the two refusals are measured
//! with GHC 9.14.1 (root's D01 controls); the other outputs are GHC's semantics, still to be
//! measured, and `parametric_newtype_show` is a pinned prediction (#31740).
//! workflow: language-features-chirho/deriving-strategies-chirho

use crate::{
    compile_source_chirho, eval_modules_with_machine_chirho, eval_source_with_machine_chirho,
};
use haskelujah_span_chirho::SourceMapChirho;

macro_rules! fixture_chirho {
    ($name_chirho:literal) => {
        include_str!(concat!(
            "../../../tests/typing_integration_chirho/declarations_chirho/deriving_strategies_chirho/",
            $name_chirho
        ))
    };
}

/// What the interpreter prints for a one-module program.
fn interpreted_chirho(file_name_chirho: &str, source_chirho: &str) -> String {
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
        None,
    )
    .unwrap_or_else(|error_chirho| panic!("{file_name_chirho}: interpreter: {error_chirho}"));
    machine_chirho.io_output_chirho
}

#[test]
fn newtype_show_prints_the_representation_chirho() {
    assert_eq!(
        interpreted_chirho(
            "NewtypeShowChirho.hs",
            fixture_chirho!("newtype_show_chirho.hs")
        ),
        "7\n"
    );
}

#[test]
fn stock_show_prints_the_constructor_chirho() {
    assert_eq!(
        interpreted_chirho(
            "StockShowChirho.hs",
            fixture_chirho!("stock_show_chirho.hs")
        ),
        "AgeChirho 7\n"
    );
}

#[test]
fn standalone_newtype_show_prints_the_representation_chirho() {
    assert_eq!(
        interpreted_chirho(
            "StandaloneNewtypeShowChirho.hs",
            fixture_chirho!("standalone_newtype_show_chirho.hs")
        ),
        "7\n"
    );
}

#[test]
fn newtype_num_computes_with_the_representation_chirho() {
    assert_eq!(
        interpreted_chirho(
            "NewtypeNumChirho.hs",
            fixture_chirho!("newtype_num_chirho.hs")
        ),
        "7\n"
    );
}

#[test]
fn stock_show_beside_newtype_num_keeps_the_constructor_chirho() {
    assert_eq!(
        interpreted_chirho(
            "MixedStockShowNewtypeNumChirho.hs",
            fixture_chirho!("mixed_stock_show_newtype_num_chirho.hs")
        ),
        "AgeChirho 7\n"
    );
}

#[test]
fn nested_newtype_and_data_fields_agree_chirho() {
    assert_eq!(
        interpreted_chirho(
            "NestedNewtypeFieldsChirho.hs",
            fixture_chirho!("nested_newtype_fields_chirho.hs")
        ),
        "PersonChirho \"Ann\" 7\n8\n6\n"
    );
}

#[test]
fn parametric_newtype_show_takes_its_context_chirho() {
    assert_eq!(
        interpreted_chirho(
            "ParametricNewtypeShowChirho.hs",
            fixture_chirho!("parametric_newtype_show_chirho.hs")
        ),
        "7\n"
    );
}

#[test]
fn written_parametric_show_prints_chirho() {
    assert_eq!(
        interpreted_chirho(
            "WrittenParametricShowChirho.hs",
            fixture_chirho!("written_parametric_show_chirho.hs")
        ),
        "7\n"
    );
}

#[test]
fn contextual_user_instance_keeps_its_own_method_chirho() {
    assert_eq!(
        interpreted_chirho(
            "ContextualUserInstanceChirho.hs",
            fixture_chirho!("contextual_user_instance_chirho.hs")
        ),
        "user\n"
    );
}

#[test]
fn imported_gnd_newtype_keeps_one_representation_chirho() {
    let (_, machine_chirho) = eval_modules_with_machine_chirho(
        &[
            ("AgesChirho.hs", fixture_chirho!("imported_ages_chirho.hs")),
            ("Main.hs", fixture_chirho!("imported_main_chirho.hs")),
        ],
        &mut SourceMapChirho::new_chirho(),
        None,
    )
    .unwrap_or_else(|error_chirho| panic!("imported GND newtype: interpreter: {error_chirho}"));
    assert_eq!(machine_chirho.io_output_chirho, "6\n7\n8\n90\n7\n");
}

/// The frontend refuses a program, with every reason named.
fn assert_refused_chirho(file_name_chirho: &str, source_chirho: &str, reasons_chirho: &[&str]) {
    let message_chirho = match compile_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
    ) {
        Ok(_) => panic!("{file_name_chirho}: GHC rejects this program; it compiled"),
        Err(error_chirho) => error_chirho.to_string(),
    };
    for reason_chirho in reasons_chirho {
        assert!(
            message_chirho.contains(reason_chirho),
            "{file_name_chirho}: refused, but not for `{reason_chirho}`: {message_chirho}"
        );
    }
}

#[test]
fn newtype_strategy_on_data_is_refused_chirho() {
    assert_refused_chirho(
        "DataNewtypeStrategyChirho.hs",
        fixture_chirho!("data_newtype_strategy_chirho.hs"),
        &[
            "E0209",
            "`Show BadChirho` with the newtype strategy",
            "non-newtypes",
        ],
    );
}

#[test]
fn newtype_strategy_without_the_extension_is_refused_chirho() {
    assert_refused_chirho(
        "MissingGndExtensionChirho.hs",
        fixture_chirho!("missing_gnd_extension_chirho.hs"),
        &[
            "E0209",
            "`Eq AgeChirho` with the newtype strategy",
            "is not enabled",
        ],
    );
}
