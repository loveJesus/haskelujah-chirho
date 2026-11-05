// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 694 deriving-strategy controls on every engine.
//!
//! The first three sources are root's D01 controls byte for byte
//! (`26-10-30-chirho/accept-frontier-baseline-chirho.md`), measured with GHC 9.14.1:
//! `deriving newtype Show` prints `7`, `deriving newtype` on data is GHC-10333, and
//! `deriving newtype` without GeneralizedNewtypeDeriving under Haskell2010 is GHC-26557.
//! The stock twin, the standalone and `Num` positives and the contextual user instance are
//! this lane's controls; their GHC 9.14.1 runs are still to be measured.
//! workflow: language-features-chirho/deriving-strategies-chirho

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

// Root's control `newtype-show-chirho.hs`: GHC 9.14.1 prints `7`.
const NEWTYPE_SHOW_SOURCE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
newtype AgeChirho = AgeChirho Int deriving newtype Show
main = print (AgeChirho 7)
"#;

// Root's control `data-newtype-strategy-chirho.hs`: GHC 9.14.1 rejects it, GHC-10333.
const DATA_NEWTYPE_STRATEGY_SOURCE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
data BadChirho = BadChirho Int deriving newtype Show
main = return ()
"#;

// Root's control `missing-gnd-extension-chirho.hs`: GHC 9.14.1 rejects it, GHC-26557.
const MISSING_GND_EXTENSION_SOURCE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, DerivingStrategies #-}
newtype AgeChirho = AgeChirho Int deriving newtype Eq
main = return ()
"#;

// The stock twin keeps the constructor.
const STOCK_SHOW_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE DerivingStrategies #-}
newtype AgeChirho = AgeChirho Int deriving stock Show
main = print (AgeChirho 7)
"#;

// A standalone declaration takes the same route.
const STANDALONE_NEWTYPE_SHOW_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, StandaloneDeriving #-}
newtype AgeChirho = AgeChirho Int
deriving newtype instance Show AgeChirho
main = print (AgeChirho 7)
"#;

// Every `Num` method is the representation's: `+`, and `fromInteger` for the literal.
const NEWTYPE_NUM_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
newtype AgeChirho = AgeChirho Int deriving newtype (Show, Num)
main = print (AgeChirho 3 + 4)
"#;

// A written instance with a context on a newtype head is not GND: its own method stays.
const CONTEXTUAL_USER_INSTANCE_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE FlexibleContexts #-}
newtype WrapChirho = WrapChirho Int
instance Show Int => Show WrapChirho where
  show _ = "user"
main = putStrLn (show (WrapChirho 5))
"#;

#[test]
fn deriving_newtype_show_prints_the_representation_everywhere_chirho() {
    assert_prints_everywhere_chirho("NewtypeShowChirho.hs", NEWTYPE_SHOW_SOURCE_CHIRHO, "7\n");
}

#[test]
fn deriving_stock_show_prints_the_constructor_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "StockShowChirho.hs",
        STOCK_SHOW_SOURCE_CHIRHO,
        "AgeChirho 7\n",
    );
}

#[test]
fn standalone_deriving_newtype_show_prints_the_representation_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "StandaloneNewtypeShowChirho.hs",
        STANDALONE_NEWTYPE_SHOW_SOURCE_CHIRHO,
        "7\n",
    );
}

#[test]
fn deriving_newtype_num_computes_with_the_representation_everywhere_chirho() {
    assert_prints_everywhere_chirho("NewtypeNumChirho.hs", NEWTYPE_NUM_SOURCE_CHIRHO, "7\n");
}

#[test]
fn a_contextual_user_instance_keeps_its_own_method_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "ContextualUserInstanceChirho.hs",
        CONTEXTUAL_USER_INSTANCE_SOURCE_CHIRHO,
        "user\n",
    );
}

#[test]
fn deriving_newtype_on_data_is_refused_chirho() {
    rejected_like_ghc_chirho(
        DATA_NEWTYPE_STRATEGY_SOURCE_CHIRHO,
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
        MISSING_GND_EXTENSION_SOURCE_CHIRHO,
        "MissingGndExtensionChirho.hs",
        &[
            "E0209",
            "`Eq AgeChirho` with the newtype strategy",
            "GeneralizedNewtypeDeriving is not enabled",
        ],
    );
}
