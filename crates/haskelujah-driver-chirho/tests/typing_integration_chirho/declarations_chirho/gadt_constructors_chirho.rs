// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Exact GHC 9.14.1 oracle sources, measured by root on 2026-11-03.
//! Constructor class-context runtime evidence remains a separately measured
//! boundary; the class-context case below asserts only its frontend verdict.

use super::{assert_prints_everywhere_chirho, checks_like_ghc_chirho, rejected_like_ghc_chirho};
use haskelujah_driver::{compile_modules_chirho, eval_source_with_machine_chirho};
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

const G01RECORDCONTEXTCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho
type instance OpaqueChirho Bool = Int

data BoxChirho aChirho where
  MkBoxChirho :: (OpaqueChirho aChirho ~ Int) => { payloadChirho :: OpaqueChirho aChirho } -> BoxChirho aChirho
readChirho :: forall aChirho. BoxChirho aChirho -> Int
readChirho MkBoxChirho { payloadChirho = valueChirho } = valueChirho
main :: IO ()
main = print (readChirho (MkBoxChirho { payloadChirho = (4 :: Int) } :: BoxChirho Bool))
"####;

const G02FORALLRECORDCONTEXTCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho
type instance OpaqueChirho Bool = Int

data BoxChirho aChirho where
  MkBoxChirho :: forall aChirho. (OpaqueChirho aChirho ~ Int) => { payloadChirho :: OpaqueChirho aChirho } -> BoxChirho aChirho
readChirho :: forall aChirho. BoxChirho aChirho -> Int
readChirho MkBoxChirho { payloadChirho = valueChirho } = valueChirho
main :: IO ()
main = print (readChirho (MkBoxChirho { payloadChirho = (4 :: Int) } :: BoxChirho Bool))
"####;

const G03FOLLOWINGSENTINELCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, FlexibleContexts #-}
module Main where

data BoxChirho aChirho where
  MkBoxChirho :: Eq aChirho => { payloadChirho :: aChirho } -> BoxChirho aChirho
sentinelChirho :: Int
sentinelChirho = 17
main :: IO ()
main = print sentinelChirho
"####;

const G04NAMEDFIELDORDERCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where

data PairChirho aChirho where
  PairChirho :: { firstChirho, secondChirho :: aChirho, thirdChirho :: Int } -> PairChirho aChirho
main :: IO ()
main = case (PairChirho { thirdChirho = 3, secondChirho = True, firstChirho = False } :: PairChirho Bool) of
  PairChirho { secondChirho = yChirho, firstChirho = xChirho, thirdChirho = nChirho } -> print (xChirho, yChirho, nChirho)
"####;

const G05REFINEDRECORDRESULTCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where

data TaggedChirho aChirho where
  TaggedChirho :: { countChirho :: Int } -> TaggedChirho Bool
readChirho :: TaggedChirho aChirho -> Int
readChirho TaggedChirho { countChirho = valueChirho } = valueChirho
main :: IO ()
main = print (readChirho (TaggedChirho { countChirho = 4 }))
"####;

const G06WRONGRECORDRESULTCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where

data TaggedChirho aChirho where
  TaggedChirho :: { countChirho :: Int } -> TaggedChirho Bool
main :: IO ()
main = case (TaggedChirho { countChirho = 4 } :: TaggedChirho Char) of
  TaggedChirho { countChirho = valueChirho } -> print valueChirho
"####;

const G07EXISTENTIALSELECTORCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ExplicitForAll #-}
module Main where

data HiddenChirho where
  HideChirho :: forall aChirho. { hiddenChirho :: aChirho } -> HiddenChirho
main :: IO ()
main = print (hiddenChirho (HideChirho (3 :: Int)))
"####;

const G08EXISTENTIALRECORDESCAPECHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ExplicitForAll #-}
module Main where

data HiddenChirho where
  HideChirho :: forall aChirho. { hiddenChirho :: aChirho } -> HiddenChirho
leakChirho :: HiddenChirho -> Int
leakChirho HideChirho { hiddenChirho = valueChirho } = valueChirho
main :: IO ()
main = print (leakChirho (HideChirho (3 :: Int)))
"####;

const G09LAZYPOSITIONALBOTTOMCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where

data LazyChirho where
  LazyChirho :: Int -> LazyChirho
main :: IO ()
main = print (case LazyChirho (error "lazy-field-chirho") of LazyChirho _ -> (1 :: Int))
"####;

const G10STRICTPOSITIONALBOTTOMCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where

data StrictChirho where
  StrictChirho :: !Int -> StrictChirho
main :: IO ()
main = print (case StrictChirho (error "strict-field-chirho") of StrictChirho _ -> (1 :: Int))
"####;

const G11STRICTRECORDBOTTOMCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, TypeFamilies #-}
module Main where

data StrictChirho aChirho where
  StrictChirho :: (aChirho ~ Int) => { payloadChirho :: !aChirho } -> StrictChirho aChirho
main :: IO ()
main = print (case (StrictChirho { payloadChirho = error "strict-field-chirho" } :: StrictChirho Int) of StrictChirho {} -> (1 :: Int))
"####;

const G12LINEARPOSITIONALCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, LinearTypes #-}
module Main where

data LinearChirho aChirho where
  LinearChirho :: aChirho %1 -> LinearChirho aChirho
unwrapChirho :: LinearChirho aChirho %1 -> aChirho
unwrapChirho (LinearChirho valueChirho) = valueChirho
main :: IO ()
main = print (unwrapChirho (LinearChirho (4 :: Int)))
"####;

const G13MANYCONSTRUCTORASLINEARCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, LinearTypes, DataKinds #-}
module Main where

import GHC.Types (Multiplicity (Many))

data ManyChirho aChirho where
  ManyChirho :: aChirho %Many -> ManyChirho aChirho
makerChirho :: Int %1 -> ManyChirho Int
makerChirho = ManyChirho
main :: IO ()
main = case makerChirho 4 of ManyChirho valueChirho -> print valueChirho
"####;

const G14LINEARRECORDFIELDCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, LinearTypes #-}
module Main where

data LinearRecordChirho aChirho where
  LinearRecordChirho :: { payloadChirho %1 :: aChirho } -> LinearRecordChirho aChirho
unwrapChirho :: LinearRecordChirho aChirho %1 -> aChirho
unwrapChirho LinearRecordChirho { payloadChirho = valueChirho } = valueChirho
main :: IO ()
main = print (unwrapChirho (LinearRecordChirho { payloadChirho = (4 :: Int) }))
"####;

const G15ILLEGALRECORDARROWCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, LinearTypes #-}
module Main where

data BadRecordChirho aChirho where
  BadRecordChirho :: { payloadChirho :: aChirho } %1 -> BadRecordChirho aChirho
main :: IO ()
main = pure ()
"####;

const G16CLASSCONTEXTBOUNDARYCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where

data NumberBoxChirho aChirho where
  NumberBoxChirho :: Num aChirho => { numberChirho :: aChirho } -> NumberBoxChirho aChirho
bumpChirho :: NumberBoxChirho aChirho -> aChirho
bumpChirho NumberBoxChirho { numberChirho = valueChirho } = valueChirho + 1
main :: IO ()
main = print (bumpChirho (NumberBoxChirho (3 :: Int)), bumpChirho (NumberBoxChirho (1.5 :: Double)))
"####;

const G17IMPORTEDEXISTENTIALSELECTORCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import HiddenChirho
main :: IO ()
main = print (hiddenChirho (HideChirho (3 :: Int)))
"####;

const HIDDEN_SELECTOR_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ExplicitForAll #-}
module HiddenChirho where
data HiddenChirho where
  HideChirho :: forall aChirho. { hiddenChirho :: aChirho } -> HiddenChirho
"####;

#[test]
fn gadt_g02_prints_like_ghc_on_every_engine_chirho() {
    assert_prints_everywhere_chirho(
        "G02ForallRecordContextChirho.hs",
        G02FORALLRECORDCONTEXTCHIRHO_CHIRHO,
        "4\n",
    );
}

#[test]
fn gadt_g03_prints_like_ghc_on_every_engine_chirho() {
    assert_prints_everywhere_chirho(
        "G03FollowingSentinelChirho.hs",
        G03FOLLOWINGSENTINELCHIRHO_CHIRHO,
        "17\n",
    );
}

#[test]
fn gadt_g04_prints_like_ghc_on_every_engine_chirho() {
    assert_prints_everywhere_chirho(
        "G04NamedFieldOrderChirho.hs",
        G04NAMEDFIELDORDERCHIRHO_CHIRHO,
        "(False,True,3)\n",
    );
}

#[test]
fn gadt_g05_prints_like_ghc_on_every_engine_chirho() {
    assert_prints_everywhere_chirho(
        "G05RefinedRecordResultChirho.hs",
        G05REFINEDRECORDRESULTCHIRHO_CHIRHO,
        "4\n",
    );
}

#[test]
fn gadt_g09_prints_like_ghc_on_every_engine_chirho() {
    assert_prints_everywhere_chirho(
        "G09LazyPositionalBottomChirho.hs",
        G09LAZYPOSITIONALBOTTOMCHIRHO_CHIRHO,
        "1\n",
    );
}

#[test]
fn gadt_g12_prints_like_ghc_on_every_engine_chirho() {
    assert_prints_everywhere_chirho(
        "G12LinearPositionalChirho.hs",
        G12LINEARPOSITIONALCHIRHO_CHIRHO,
        "4\n",
    );
}

#[test]
fn gadt_g14_prints_like_ghc_on_every_engine_chirho() {
    assert_prints_everywhere_chirho(
        "G14LinearRecordFieldChirho.hs",
        G14LINEARRECORDFIELDCHIRHO_CHIRHO,
        "4\n",
    );
}

#[test]
fn record_context_matches_the_exact_frontend_oracle_chirho() {
    checks_like_ghc_chirho("G01RecordContextChirho.hs", G01RECORDCONTEXTCHIRHO_CHIRHO);
}

#[test]
fn class_context_metadata_is_retained_without_claiming_runtime_dictionary_storage_chirho() {
    checks_like_ghc_chirho(
        "G16ClassContextBoundaryChirho.hs",
        G16CLASSCONTEXTBOUNDARYCHIRHO_CHIRHO,
    );
}

#[test]
fn wrong_results_escaping_existentials_and_many_as_linear_are_rejected_chirho() {
    rejected_like_ghc_chirho(
        G06WRONGRECORDRESULTCHIRHO_CHIRHO,
        "G06WrongRecordResultChirho.hs",
        &[],
    );

    rejected_like_ghc_chirho(
        G07EXISTENTIALSELECTORCHIRHO_CHIRHO,
        "G07ExistentialSelectorChirho.hs",
        &[],
    );

    rejected_like_ghc_chirho(
        G08EXISTENTIALRECORDESCAPECHIRHO_CHIRHO,
        "G08ExistentialRecordEscapeChirho.hs",
        &[],
    );

    rejected_like_ghc_chirho(
        G13MANYCONSTRUCTORASLINEARCHIRHO_CHIRHO,
        "G13ManyConstructorAsLinearChirho.hs",
        &[],
    );

    rejected_like_ghc_chirho(
        G15ILLEGALRECORDARROWCHIRHO_CHIRHO,
        "G15IllegalRecordArrowChirho.hs",
        &["ordinary, non-linear arrow", "GHC-37475"],
    );
}

fn assert_strict_bottom_chirho(file_chirho: &str, source_chirho: &str) {
    let interpreter_chirho = match eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_chirho,
        None,
    ) {
        Ok(_) => panic!("forcing the constructor must force its strict field"),
        Err(error_chirho) => error_chirho,
    };
    assert!(
        interpreter_chirho
            .to_string()
            .contains("strict-field-chirho"),
        "{interpreter_chirho}"
    );
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        match native_round_trip_chirho(source_chirho, backend_chirho, "") {
            Ok((exit_chirho, _)) => assert_ne!(
                exit_chirho, 0,
                "{file_chirho}: {backend_chirho:?} returned through bottom"
            ),
            Err(error_chirho) => assert!(
                error_chirho.contains("strict-field-chirho")
                    && !error_chirho.starts_with("frontend"),
                "{file_chirho}: {backend_chirho:?}: {error_chirho}"
            ),
        }
    }
}

#[test]
fn strict_gadt_fields_force_bottom_in_positional_and_record_construction_chirho() {
    assert_strict_bottom_chirho(
        "G10StrictPositionalBottomChirho.hs",
        G10STRICTPOSITIONALBOTTOMCHIRHO_CHIRHO,
    );
    assert_strict_bottom_chirho(
        "G11StrictRecordBottomChirho.hs",
        G11STRICTRECORDBOTTOMCHIRHO_CHIRHO,
    );
}

#[test]
fn an_imported_existential_label_never_gets_a_phantom_selector_scheme_chirho() {
    let sources_chirho = [
        ("HiddenChirho.hs", HIDDEN_SELECTOR_CHIRHO),
        ("Main.hs", G17IMPORTEDEXISTENTIALSELECTORCHIRHO_CHIRHO),
    ];
    assert!(
        compile_modules_chirho(&sources_chirho, &mut SourceMapChirho::new_chirho()).is_err(),
        "an imported field label is not a callable existential selector"
    );
}

const ORDINARYLAZYRECORDBOTTOMCHIRHO_CHIRHO: &str = r####"{-# LANGUAGE NamedFieldPuns #-}
module Main where
data BoxChirho = MkBoxChirho { payloadChirho :: Int }
readChirho :: BoxChirho -> Int
readChirho MkBoxChirho {} = 1
main :: IO ()
main = print (readChirho (MkBoxChirho { payloadChirho = error "strict-field-chirho" }))
"####;

const ORDINARYSTRICTRECORDBOTTOMCHIRHO_CHIRHO: &str = r####"{-# LANGUAGE NamedFieldPuns #-}
module Main where
data BoxChirho = MkBoxChirho { payloadChirho :: !Int }
readChirho :: BoxChirho -> Int
readChirho MkBoxChirho {} = 1
main :: IO ()
main = print (readChirho (MkBoxChirho { payloadChirho = error "strict-field-chirho" }))
"####;

#[test]
fn ordinary_record_strictness_uses_the_same_constructor_hook_chirho() {
    assert_prints_everywhere_chirho(
        "OrdinaryLazyRecordBottomChirho.hs",
        ORDINARYLAZYRECORDBOTTOMCHIRHO_CHIRHO,
        "1\n",
    );
    assert_strict_bottom_chirho(
        "OrdinaryStrictRecordBottomChirho.hs",
        ORDINARYSTRICTRECORDBOTTOMCHIRHO_CHIRHO,
    );
}
