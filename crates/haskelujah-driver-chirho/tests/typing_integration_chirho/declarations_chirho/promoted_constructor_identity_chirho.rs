// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 686 promoted-constructor identity controls.
//!
//! Each program is the row 686 oracle file GHC 9.14.1 measured, embedded byte for byte
//! (its sha256 prefix is noted), and must print GHC's output on the interpreter, LLVM
//! and Cranelift. The frozen baseline CLI (ce999556) refused A04, C17 and E03 and printed
//! `True True` for C15b; the 686a equation carrier alone still refused A04 and C17,
//! failed E03 at run time and printed `False False` for C15b. The multi-module controls
//! (C18b, E01) need native project builds and are measured as build-run cells.
//! workflow: compiler-pipeline-chirho/promoted-constructor-identity-chirho

use super::assert_prints_everywhere_chirho;
use haskelujah_span_chirho::SourceMapChirho;

/// `A04PromotedChirho.hs`, sha256 d0726ed41883cf42: ticked constructor and promoted-list patterns, ticked and unquoted uses.
const A04_PROMOTED_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle A04 (686a): ticked constructor and promoted-list patterns, used ticked and
-- unticked. Probes the tick hazard: use sites drop the tick, equation conversion keeps it.
-- Predicted GHC 9.14.1 output: False, False, False, True, False.
{-# LANGUAGE DataKinds, TypeFamilies, KindSignatures, ScopedTypeVariables, PolyKinds #-}
module Main where

import Data.Kind (Type)
import Data.Proxy (Proxy (..))

class KnownBoolChirho (bChirho :: Bool) where
  boolValChirho :: Proxy bChirho -> Bool

instance KnownBoolChirho 'True where
  boolValChirho _ = True

instance KnownBoolChirho 'False where
  boolValChirho _ = False

type family NotChirho (bChirho :: Bool) :: Bool where
  NotChirho 'True = 'False
  NotChirho 'False = 'True

type family IsNilChirho (xsChirho :: [Type]) :: Bool where
  IsNilChirho '[] = 'True
  IsNilChirho xsChirho = 'False

main :: IO ()
main = do
  print (boolValChirho (Proxy :: Proxy (NotChirho 'True)))
  print (boolValChirho (Proxy :: Proxy (NotChirho True)))
  print (boolValChirho (Proxy :: Proxy (NotChirho (NotChirho 'False))))
  print (boolValChirho (Proxy :: Proxy (IsNilChirho '[])))
  print (boolValChirho (Proxy :: Proxy (IsNilChirho '[Int])))
"#;

/// `C17QuotedTrueChirho.hs`, sha256 7be4e50330dfb922: quoted and unquoted True in patterns, right sides, uses and instance heads.
const C17_QUOTED_TRUE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C17 (#30845): quoted and unquoted True/False are the same promoted
-- constructors when no type of that name exists. Patterns, right sides, uses and instance
-- heads mix both spellings. Predicted GHC 9.14.1 output: False, False, True, True.
{-# LANGUAGE DataKinds, TypeFamilies, KindSignatures #-}
module Main where

import Data.Proxy (Proxy (..))

class KnownBoolChirho (bChirho :: Bool) where
  boolValChirho :: Proxy bChirho -> Bool

instance KnownBoolChirho 'True where
  boolValChirho _ = True

instance KnownBoolChirho False where
  boolValChirho _ = False

type family FlipChirho (bChirho :: Bool) :: Bool where
  FlipChirho True = 'False
  FlipChirho 'False = True

main :: IO ()
main = do
  print (boolValChirho (Proxy :: Proxy (FlipChirho 'True)))
  print (boolValChirho (Proxy :: Proxy (FlipChirho True)))
  print (boolValChirho (Proxy :: Proxy (FlipChirho 'False)))
  print (boolValChirho (Proxy :: Proxy (FlipChirho False)))
"#;

/// `C15bPromotedHomonymChirho.hs`, sha256 c1996d20ff83f2ce: `data TChirho = TChirho`: the promoted constructor is not the type.
const C15B_PROMOTED_HOMONYM_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C15b (tasklist, promotion identity): CORRECTED copy of C15, preserved failed (sha256 da60b25b…).
-- C15 failed GHC-25897: without a standalone kind signature the equations fix kChirho to one kind.
-- Correction: a standalone kind signature, with StandaloneKindSignatures named explicitly (GHC2021
-- already implies it). `data TChirho = TChirho`; the family separates 'TChirho from TChirho.
-- Predicted GHC 9.14.1 output: True, False.
{-# LANGUAGE DataKinds, TypeFamilies, PolyKinds, KindSignatures, StandaloneKindSignatures #-}
module Main where

import Data.Proxy (Proxy (..))

data TChirho = TChirho

class KnownBoolChirho (bChirho :: Bool) where
  boolValChirho :: Proxy bChirho -> Bool

instance KnownBoolChirho True where
  boolValChirho _ = True

instance KnownBoolChirho False where
  boolValChirho _ = False

type IsPromotedChirho :: kChirho -> Bool
type family IsPromotedChirho aChirho where
  IsPromotedChirho 'TChirho = True
  IsPromotedChirho TChirho = False

main :: IO ()
main = do
  print (boolValChirho (Proxy :: Proxy (IsPromotedChirho 'TChirho)))
  print (boolValChirho (Proxy :: Proxy (IsPromotedChirho TChirho)))
"#;

/// `E03SourceTrueChirho.hs`, sha256 5a31bb5c6a803ad9: a source constructor named True beside Prelude.True.
const E03_SOURCE_TRUE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 promotion control E03 (#30849): a source constructor named True beside the
-- standard promoted True. With Prelude's True hidden, 'True and an unticked True name the
-- source constructor; 'Prelude.True names the standard one. A family over both kinds must
-- keep them apart. Predicted GHC 9.14.1 output: False, False, True.
{-# LANGUAGE DataKinds, TypeFamilies, PolyKinds, KindSignatures, StandaloneKindSignatures #-}
module Main where

import Prelude hiding (True, False)
import qualified Prelude
import Data.Proxy (Proxy (..))

data MyBoolChirho = True | False

class KnownStdChirho (bChirho :: Prelude.Bool) where
  stdValChirho :: Proxy bChirho -> Prelude.Bool

instance KnownStdChirho 'Prelude.True where
  stdValChirho _ = Prelude.True

instance KnownStdChirho 'Prelude.False where
  stdValChirho _ = Prelude.False

type WhichTrueChirho :: kChirho -> Prelude.Bool
type family WhichTrueChirho aChirho where
  WhichTrueChirho 'True = 'Prelude.False
  WhichTrueChirho 'Prelude.True = 'Prelude.True

main :: IO ()
main = do
  print (stdValChirho (Proxy :: Proxy (WhichTrueChirho 'True)))
  print (stdValChirho (Proxy :: Proxy (WhichTrueChirho True)))
  print (stdValChirho (Proxy :: Proxy (WhichTrueChirho 'Prelude.True)))
"#;

#[test]
fn a04_promoted_prints_ghcs_output_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "A04PromotedChirho.hs",
        A04_PROMOTED_CHIRHO,
        "False\nFalse\nFalse\nTrue\nFalse\n",
    );
}

#[test]
fn c17_quoted_true_prints_ghcs_output_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "C17QuotedTrueChirho.hs",
        C17_QUOTED_TRUE_CHIRHO,
        "False\nFalse\nTrue\nTrue\n",
    );
}

#[test]
fn c15b_promoted_homonym_prints_ghcs_output_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "C15bPromotedHomonymChirho.hs",
        C15B_PROMOTED_HOMONYM_CHIRHO,
        "True\nFalse\n",
    );
}

#[test]
fn e03_source_true_prints_ghcs_output_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "E03SourceTrueChirho.hs",
        E03_SOURCE_TRUE_CHIRHO,
        "False\nFalse\nTrue\n",
    );
}

/// `P04PromotedMismatchChirho.hs`, sha256 8b56fa6b4b0d31c1: GHC 9.14.1 rejects it (GHC-83865,
/// "Couldn't match type 'SChirho ZChirho' with 'ZChirho'").
const P04_PROMOTED_MISMATCH_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 probe P04: a promoted-constructor mismatch GHC rejects. The diagnostic must
-- name the constructors as written, never the checker's internal keys.
{-# LANGUAGE DataKinds #-}
module Main where

import Data.Proxy (Proxy (..))

data NatChirho = ZChirho | SChirho NatChirho

needsZeroChirho :: Proxy 'ZChirho -> ()
needsZeroChirho _ = ()

main :: IO ()
main = print (needsZeroChirho (Proxy :: Proxy ('SChirho 'ZChirho)))
"#;

#[test]
fn a_promoted_mismatch_names_constructors_not_keys_chirho() {
    let Err(error_chirho) = haskelujah_driver::typecheck_source_chirho(
        P04_PROMOTED_MISMATCH_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "P04PromotedMismatchChirho.hs",
    ) else {
        panic!("P04PromotedMismatchChirho.hs: GHC rejects this program; we accepted it");
    };
    let error_chirho = error_chirho.to_string();
    assert!(
        !error_chirho.contains("%promoted:"),
        "an internal key reached the diagnostic: {error_chirho}"
    );
    assert!(
        error_chirho.contains("'ZChirho"),
        "the diagnostic names the constructor: {error_chirho}"
    );
}
