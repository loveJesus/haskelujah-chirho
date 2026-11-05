// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 694 deriving-strategy controls on every engine.
//!
//! The programs are the fixtures beside this file, shared with the interpreter controls
//! (`src/tests_chirho/extensions_chirho/deriving_strategies_chirho.rs`) and the multi-module
//! controls (`tests/native_symbol_identity_chirho/newtype_representation_chirho.rs`).
//! `newtype_show`, `data_newtype_strategy` and `missing_gnd_extension` are root's D01 controls
//! byte for byte (`26-10-30-chirho/accept-frontier-baseline-chirho.md`), measured with GHC
//! 9.14.1: `7`, GHC-10333 and GHC-26557. Every other expected output is GHC's semantics,
//! still to be measured with GHC 9.14.1. `parametric_newtype_show` is a pinned prediction
//! (#31740): its GND instance has a context, so its dictionary must take that context.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

const NEWTYPE_SHOW_CHIRHO: &str = include_str!("deriving_strategies_chirho/newtype_show_chirho.hs");
const DATA_NEWTYPE_STRATEGY_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/data_newtype_strategy_chirho.hs");
const MISSING_GND_EXTENSION_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/missing_gnd_extension_chirho.hs");
const STOCK_SHOW_CHIRHO: &str = include_str!("deriving_strategies_chirho/stock_show_chirho.hs");
const STANDALONE_NEWTYPE_SHOW_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/standalone_newtype_show_chirho.hs");
const NEWTYPE_NUM_CHIRHO: &str = include_str!("deriving_strategies_chirho/newtype_num_chirho.hs");
const MIXED_STOCK_SHOW_NEWTYPE_NUM_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/mixed_stock_show_newtype_num_chirho.hs");
const NESTED_NEWTYPE_FIELDS_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/nested_newtype_fields_chirho.hs");
const PARAMETRIC_NEWTYPE_SHOW_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/parametric_newtype_show_chirho.hs");
const WRITTEN_PARAMETRIC_SHOW_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/written_parametric_show_chirho.hs");
const CONTEXTUAL_USER_INSTANCE_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/contextual_user_instance_chirho.hs");

#[test]
fn deriving_newtype_show_prints_the_representation_everywhere_chirho() {
    assert_prints_everywhere_chirho("NewtypeShowChirho.hs", NEWTYPE_SHOW_CHIRHO, "7\n");
}

#[test]
fn deriving_stock_show_prints_the_constructor_everywhere_chirho() {
    assert_prints_everywhere_chirho("StockShowChirho.hs", STOCK_SHOW_CHIRHO, "AgeChirho 7\n");
}

#[test]
fn standalone_deriving_newtype_show_prints_the_representation_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "StandaloneNewtypeShowChirho.hs",
        STANDALONE_NEWTYPE_SHOW_CHIRHO,
        "7\n",
    );
}

#[test]
fn deriving_newtype_num_computes_with_the_representation_everywhere_chirho() {
    assert_prints_everywhere_chirho("NewtypeNumChirho.hs", NEWTYPE_NUM_CHIRHO, "7\n");
}

#[test]
fn stock_show_beside_newtype_num_keeps_the_constructor_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "MixedStockShowNewtypeNumChirho.hs",
        MIXED_STOCK_SHOW_NEWTYPE_NUM_CHIRHO,
        "AgeChirho 7\n",
    );
}

#[test]
fn nested_newtype_and_data_fields_agree_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "NestedNewtypeFieldsChirho.hs",
        NESTED_NEWTYPE_FIELDS_CHIRHO,
        "PersonChirho \"Ann\" 7\n8\n6\n",
    );
}

#[test]
fn parametric_deriving_newtype_show_takes_its_context_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "ParametricNewtypeShowChirho.hs",
        PARAMETRIC_NEWTYPE_SHOW_CHIRHO,
        "7\n",
    );
}

#[test]
fn a_written_parametric_show_prints_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "WrittenParametricShowChirho.hs",
        WRITTEN_PARAMETRIC_SHOW_CHIRHO,
        "7\n",
    );
}

#[test]
fn a_contextual_user_instance_keeps_its_own_method_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "ContextualUserInstanceChirho.hs",
        CONTEXTUAL_USER_INSTANCE_CHIRHO,
        "user\n",
    );
}

#[test]
fn deriving_newtype_on_data_is_refused_chirho() {
    rejected_like_ghc_chirho(
        DATA_NEWTYPE_STRATEGY_CHIRHO,
        "DataNewtypeStrategyChirho.hs",
        &[
            "E0209",
            "`Show BadChirho` with the newtype strategy",
            "cannot be used on non-newtypes",
        ],
    );
}

#[test]
fn deriving_newtype_without_the_extension_is_refused_chirho() {
    rejected_like_ghc_chirho(
        MISSING_GND_EXTENSION_CHIRHO,
        "MissingGndExtensionChirho.hs",
        &[
            "E0209",
            "`Eq AgeChirho` with the newtype strategy",
            "GeneralizedNewtypeDeriving is not enabled",
        ],
    );
}
