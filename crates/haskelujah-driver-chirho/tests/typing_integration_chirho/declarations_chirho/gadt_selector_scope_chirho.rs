// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Selector masks preserve lexical value bindings and qualified Prelude calls.
//! Root measured these exact sources with GHC 9.14.1. The two failed intended
//! positives stay unchanged as ambiguity negatives, rather than being rewritten.

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};
use haskelujah_driver::{compile_modules_chirho, eval_modules_with_machine_chirho};
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_project_round_trip_chirho,
};
use std::time::Duration;

const S01LOCALUNAVAILABLEHEADCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ExplicitForAll #-}
module Main where
data HiddenChirho where
  MkHiddenChirho :: forall aChirho. { head :: aChirho } -> HiddenChirho
badChirho :: [Int] -> Int
badChirho = head
main :: IO ()
main = print (badChirho [3])
"####;

const S02QUALIFIEDUNAVAILABLEHEADCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import qualified HiddenChirho as HChirho
badChirho :: [Int] -> Int
badChirho = HChirho.head
main :: IO ()
main = print (badChirho [3])
"####;

const S03LOCALFUNCTIONSHADOWSLABELCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import HiddenChirho (head)
head :: [Int] -> Int
head _ = 42
main :: IO ()
main = print (head [3])
"####;

const S04LEXICALBINDERSHADOWSLABELCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import HiddenChirho (head)
main :: IO ()
main = print ((\head -> head) (42 :: Int))
"####;

const S05QUALIFIEDPRELUDEHEADHOLDCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import qualified Prelude as PChirho
import HiddenChirho (head)
main :: PChirho.IO ()
main = PChirho.print (PChirho.head [7 :: PChirho.Int])
"####;

const S06IMPORTEDFIELDLABELPATTERNCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
import HiddenChirho
readChirho :: HiddenChirho -> Int
readChirho MkHiddenChirho { head = _ } = 17
main :: IO ()
main = print (readChirho (MkHiddenChirho { head = (3 :: Int) }))
"####;

const R01MAYBERESULTSELECTORCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data BoxChirho aChirho where
  MkBoxChirho :: { payloadChirho :: aChirho } -> BoxChirho (Maybe aChirho)
readChirho :: BoxChirho (Maybe aChirho) -> aChirho
readChirho = payloadChirho
main :: IO ()
main = print (readChirho (MkBoxChirho { payloadChirho = (4 :: Int) }))
"####;

const R02QUALIFIEDMAYBESELECTORCHIRHO_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
import qualified Data.Maybe as MChirho
data BoxChirho aChirho where
  MkBoxChirho :: { payloadChirho :: aChirho } -> BoxChirho (MChirho.Maybe aChirho)
readChirho :: BoxChirho (MChirho.Maybe aChirho) -> aChirho
readChirho = payloadChirho
main :: IO ()
main = print (readChirho (MkBoxChirho { payloadChirho = (4 :: Int) }))
"####;

const E01_EQUALITY_RECOVERS_SELECTOR_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, TypeOperators, ExplicitForAll #-}
module Main where
data BoxChirho aChirho where
  MkBoxChirho :: forall aChirho bChirho. (aChirho ~ bChirho) => { payloadChirho :: bChirho } -> BoxChirho aChirho
readChirho :: BoxChirho Int -> Int
readChirho = payloadChirho
main :: IO ()
main = print (readChirho (MkBoxChirho { payloadChirho = (4 :: Int) } :: BoxChirho Int))
"####;

const S03B_QUALIFIED_LOCAL_FUNCTION_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import HiddenChirho (head)
head :: [Int] -> Int
head _ = 42
main :: IO ()
main = print (Main.head [3])
"####;

const S06B_QUALIFIED_FIELD_LABELS_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
import qualified HiddenChirho as HChirho
readChirho :: HChirho.HiddenChirho -> Int
readChirho HChirho.MkHiddenChirho { HChirho.head = _ } = 17
main :: IO ()
main = print (readChirho (HChirho.MkHiddenChirho { HChirho.head = (3 :: Int) }))
"####;

const K01_ESCAPING_FIELD_KIND_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, PolyKinds, RankNTypes, ExplicitForAll, KindSignatures #-}
module Main where
import Data.Proxy
import Data.Kind (Type)

data BoxChirho where
  MkBoxChirho :: forall kChirho. { payloadChirho :: forall (xChirho :: kChirho). Proxy xChirho } -> BoxChirho
readChirho :: BoxChirho -> Proxy Int
readChirho boxChirho = payloadChirho boxChirho
main :: IO ()
main = print (readChirho (MkBoxChirho { payloadChirho = Proxy }))
"####;

const K02_RESULT_DETERMINES_FIELD_KIND_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, PolyKinds, RankNTypes, ExplicitForAll, KindSignatures #-}
module Main where
import Data.Proxy
import Data.Kind (Type)

data BoxChirho (aChirho :: kChirho) where
  MkBoxChirho :: forall kChirho (aChirho :: kChirho). { payloadChirho :: forall (xChirho :: kChirho). Proxy xChirho } -> BoxChirho aChirho
readChirho :: BoxChirho (aChirho :: Type) -> Proxy Int
readChirho boxChirho = payloadChirho boxChirho
main :: IO ()
main = print (readChirho (MkBoxChirho { payloadChirho = Proxy } :: BoxChirho Int))
"####;

const K03_INNER_FORALL_BINDS_FIELD_KIND_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, PolyKinds, RankNTypes, ExplicitForAll, KindSignatures #-}
module Main where
import Data.Proxy
import Data.Kind (Type)

data BoxChirho where
  MkBoxChirho :: { payloadChirho :: forall kChirho (xChirho :: kChirho). Proxy xChirho } -> BoxChirho
readChirho :: BoxChirho -> Proxy Int
readChirho boxChirho = payloadChirho boxChirho
main :: IO ()
main = print (readChirho (MkBoxChirho { payloadChirho = Proxy }))
"####;

const HIDDEN_HEAD_CHIRHO: &str = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ExplicitForAll #-}
module HiddenChirho where
data HiddenChirho where
  MkHiddenChirho :: forall aChirho. { head :: aChirho } -> HiddenChirho
"####;

fn assert_project_prints_chirho(file_chirho: &str, source_chirho: &str, expected_chirho: &str) {
    let sources_chirho = [
        ("HiddenChirho.hs", HIDDEN_HEAD_CHIRHO),
        (file_chirho, source_chirho),
    ];
    let (_, machine_chirho) =
        eval_modules_with_machine_chirho(&sources_chirho, &mut SourceMapChirho::new_chirho(), None)
            .unwrap_or_else(|error_chirho| panic!("{file_chirho}: interpreter: {error_chirho}"));
    assert_eq!(machine_chirho.io_output_chirho, expected_chirho);
    let native_sources_chirho = sources_chirho
        .iter()
        .map(|(name_chirho, source_chirho)| {
            (
                name_chirho.strip_suffix(".hs").unwrap_or(name_chirho),
                *source_chirho,
            )
        })
        .collect::<Vec<_>>();
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        assert_eq!(
            native_project_round_trip_chirho(
                &native_sources_chirho,
                backend_chirho,
                "",
                Duration::from_secs(15)
            )
            .unwrap_or_else(|error_chirho| panic!(
                "{file_chirho}: {backend_chirho:?}: {error_chirho}"
            )),
            (0, expected_chirho.to_owned())
        );
    }
}

#[test]
fn an_unavailable_local_or_qualified_selector_never_reads_a_seeded_function_chirho() {
    rejected_like_ghc_chirho(
        S01LOCALUNAVAILABLEHEADCHIRHO_CHIRHO,
        "S01LocalUnavailableHeadChirho.hs",
        &["head"],
    );
    assert!(
        compile_modules_chirho(
            &[
                ("HiddenChirho.hs", HIDDEN_HEAD_CHIRHO),
                ("Main.hs", S02QUALIFIEDUNAVAILABLEHEADCHIRHO_CHIRHO)
            ],
            &mut SourceMapChirho::new_chirho()
        )
        .is_err()
    );
}

#[test]
fn lexical_values_override_masks_while_ambiguous_top_level_names_are_rejected_chirho() {
    assert!(
        compile_modules_chirho(
            &[
                ("HiddenChirho.hs", HIDDEN_HEAD_CHIRHO),
                ("Main.hs", S03LOCALFUNCTIONSHADOWSLABELCHIRHO_CHIRHO)
            ],
            &mut SourceMapChirho::new_chirho()
        )
        .is_err()
    );
    assert_project_prints_chirho("Main.hs", S04LEXICALBINDERSHADOWSLABELCHIRHO_CHIRHO, "42\n");
}

#[test]
fn qualified_prelude_survives_masks_and_ambiguous_record_labels_are_rejected_chirho() {
    assert_project_prints_chirho("Main.hs", S05QUALIFIEDPRELUDEHEADHOLDCHIRHO_CHIRHO, "7\n");
    assert!(
        compile_modules_chirho(
            &[
                ("HiddenChirho.hs", HIDDEN_HEAD_CHIRHO),
                ("Main.hs", S06IMPORTEDFIELDLABELPATTERNCHIRHO_CHIRHO)
            ],
            &mut SourceMapChirho::new_chirho()
        )
        .is_err()
    );
}

#[test]
fn selector_recoverability_reads_the_actual_maybe_provider_through_its_alias_chirho() {
    assert_prints_everywhere_chirho(
        "R01MaybeResultSelectorChirho.hs",
        R01MAYBERESULTSELECTORCHIRHO_CHIRHO,
        "4\n",
    );
    assert_prints_everywhere_chirho(
        "R02QualifiedMaybeSelectorChirho.hs",
        R02QUALIFIEDMAYBESELECTORCHIRHO_CHIRHO,
        "4\n",
    );
}

#[test]
fn an_equality_given_does_not_invent_an_existential_record_selector_chirho() {
    rejected_like_ghc_chirho(
        E01_EQUALITY_RECOVERS_SELECTOR_CHIRHO,
        "E01EqualityRecoversSelectorChirho.hs",
        &["payloadChirho"],
    );
}

#[test]
fn exact_qualified_values_and_field_labels_keep_their_measured_positive_routes_chirho() {
    assert_project_prints_chirho("Main.hs", S03B_QUALIFIED_LOCAL_FUNCTION_CHIRHO, "42\n");
    assert_project_prints_chirho("Main.hs", S06B_QUALIFIED_FIELD_LABELS_CHIRHO, "17\n");
}

#[test]
fn field_kind_scope_requires_recoverability_or_its_own_inner_forall_chirho() {
    rejected_like_ghc_chirho(
        K01_ESCAPING_FIELD_KIND_CHIRHO,
        "K01EscapingFieldKindChirho.hs",
        &["payloadChirho"],
    );
    assert_prints_everywhere_chirho(
        "K02ResultDeterminesFieldKindChirho.hs",
        K02_RESULT_DETERMINES_FIELD_KIND_CHIRHO,
        "Proxy\n",
    );
    assert_prints_everywhere_chirho(
        "K03InnerForallBindsFieldKindChirho.hs",
        K03_INNER_FORALL_BINDS_FIELD_KIND_CHIRHO,
        "Proxy\n",
    );
}
