// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Imported family identity, ordered branches and exported promoted synonyms, GHC 9.14.1 measured.
//! This target supplies the real Cabal project path on interpreter, LLVM and true Cranelift.
//! workflow: language-features-chirho/stuck-family-equalities-chirho

use super::import_identity_chirho::{ProgramChirho, assert_exact_chirho, assert_refused_chirho};

#[test]
fn b05_imported_positive_chirho() {
    let modules_chirho: ProgramChirho = &[
        (
            "SameDefChirho",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle B05ImportedPositiveChirho: the closed family lives in another module, so its order must
-- survive the import boundary (TypeFamilyEnvChirho), not only local registration.
{-# LANGUAGE DataKinds, TypeFamilies, KindSignatures #-}
module SameDefChirho (SameChirho) where

type family SameChirho aChirho bChirho :: Bool where
  SameChirho aChirho aChirho = True
  SameChirho aChirho bChirho = False
"###,
        ),
        (
            "Main",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle B05 (686b): concrete arguments reduce through an imported closed family.
-- Predicted GHC 9.14.1 output: True, False.
{-# LANGUAGE DataKinds, KindSignatures #-}
module Main where

import Data.Proxy (Proxy (..))
import SameDefChirho (SameChirho)

class KnownBoolChirho (bChirho :: Bool) where
  boolValChirho :: Proxy bChirho -> Bool

instance KnownBoolChirho True where
  boolValChirho _ = True

instance KnownBoolChirho False where
  boolValChirho _ = False

main :: IO ()
main = do
  print (boolValChirho (Proxy :: Proxy (SameChirho Int Int)))
  print (boolValChirho (Proxy :: Proxy (SameChirho Int Bool)))
"###,
        ),
    ];
    assert_exact_chirho((modules_chirho, "True\nFalse\n"));
}

#[test]
fn b06_imported_negative_chirho() {
    let modules_chirho: ProgramChirho = &[
        (
            "SameDefChirho",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle B06ImportedNegativeChirho: the closed family lives in another module, so its order must
-- survive the import boundary (TypeFamilyEnvChirho), not only local registration.
{-# LANGUAGE DataKinds, TypeFamilies, KindSignatures #-}
module SameDefChirho (SameChirho) where

type family SameChirho aChirho bChirho :: Bool where
  SameChirho aChirho aChirho = True
  SameChirho aChirho bChirho = False
"###,
        ),
        (
            "Main",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle B06 (686b negative): B04 across the import boundary. Predicted GHC 9.14.1:
-- rejected. If the family's order is lost at import, the stuck application wrongly reduces.
{-# LANGUAGE DataKinds #-}
module Main where

import Data.Proxy (Proxy (..))
import SameDefChirho (SameChirho)

gChirho :: Proxy aChirho -> Proxy bChirho -> Proxy (SameChirho aChirho bChirho)
gChirho _ _ = (Proxy :: Proxy False)

main :: IO ()
main = pure ()
"###,
        ),
    ];
    assert_refused_chirho((modules_chirho, "SameChirho"));
}

#[test]
fn c06_compatible_imported_chirho() {
    let modules_chirho: ProgramChirho = &[
        (
            "JDefChirho",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C06 (tasklist): the compatible family of C05, defined in another module.
{-# LANGUAGE TypeFamilies #-}
module JDefChirho (JChirho) where

type family JChirho aChirho where
  JChirho Int = Int
  JChirho aChirho = aChirho
"###,
        ),
        (
            "Main",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C06 (tasklist): compatibility must survive the import boundary.
-- Predicted GHC 9.14.1 output: True, 7.
{-# LANGUAGE TypeFamilies #-}
module Main where

import Data.Proxy (Proxy (..))
import JDefChirho (JChirho)

jChirho :: Proxy xChirho -> JChirho xChirho -> xChirho
jChirho _ valueChirho = valueChirho

main :: IO ()
main = do
  print (jChirho (Proxy :: Proxy Bool) True)
  print (jChirho (Proxy :: Proxy Int) 7)
"###,
        ),
    ];
    assert_exact_chirho((modules_chirho, "True\n7\n"));
}

#[test]
fn c12_qualified_homonym_chirho() {
    let modules_chirho: ProgramChirho = &[
        (
            "LeftFamChirho",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C12 (tasklist): one of two same-named closed families; this one maps Int to Bool.
{-# LANGUAGE TypeFamilies #-}
module LeftFamChirho (PickChirho) where

type family PickChirho aChirho where
  PickChirho Int = Bool
"###,
        ),
        (
            "RightFamChirho",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C12 (tasklist): one of two same-named closed families; this one maps Int to Char.
{-# LANGUAGE TypeFamilies #-}
module RightFamChirho (PickChirho) where

type family PickChirho aChirho where
  PickChirho Int = Char
"###,
        ),
        (
            "Main",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C12 (tasklist): same-named closed families imported qualified. Each
-- application must resolve to its own family identity. Predicted GHC 9.14.1 output: True, 'c'.
{-# LANGUAGE TypeFamilies #-}
module Main where

import qualified LeftFamChirho as LeftChirho
import qualified RightFamChirho as RightChirho

main :: IO ()
main = do
  print (True :: LeftChirho.PickChirho Int)
  print ('c' :: RightChirho.PickChirho Int)
"###,
        ),
    ];
    assert_exact_chirho((modules_chirho, "True\n'c'\n"));
}

#[test]
fn c18b_qualified_promoted_chirho() {
    let modules_chirho: ProgramChirho = &[
        (
            "LeftConChirho",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C18 (#30845): the same type and constructor spelling in two modules.
module LeftConChirho (SideChirho (..)) where

data SideChirho = TChirho
"###,
        ),
        (
            "RightConChirho",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C18 (#30845): the same type and constructor spelling in two modules.
module RightConChirho (SideChirho (..)) where

data SideChirho = TChirho
"###,
        ),
        (
            "Main",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C18b (#30845): CORRECTED copy of C18, preserved failed (Main sha256 abde8ef0…).
-- C18 failed GHC-25897 for the same reason as C15; correction: a standalone kind signature, with
-- StandaloneKindSignatures named explicitly. Qualified promoted constructors 'A.T and 'B.T are
-- distinct even though their type and constructor are spelled alike.
-- Predicted GHC 9.14.1 output: True, False.
{-# LANGUAGE DataKinds, TypeFamilies, PolyKinds, KindSignatures, StandaloneKindSignatures #-}
module Main where

import Data.Proxy (Proxy (..))
import qualified LeftConChirho as LeftChirho
import qualified RightConChirho as RightChirho

class KnownBoolChirho (bChirho :: Bool) where
  boolValChirho :: Proxy bChirho -> Bool

instance KnownBoolChirho 'True where
  boolValChirho _ = True

instance KnownBoolChirho False where
  boolValChirho _ = False

type WhichChirho :: kChirho -> Bool
type family WhichChirho aChirho where
  WhichChirho 'LeftChirho.TChirho = True
  WhichChirho 'RightChirho.TChirho = False

main :: IO ()
main = do
  print (boolValChirho (Proxy :: Proxy (WhichChirho 'LeftChirho.TChirho)))
  print (boolValChirho (Proxy :: Proxy (WhichChirho 'RightChirho.TChirho)))
"###,
        ),
    ];
    assert_exact_chirho((modules_chirho, "True\nFalse\n"));
}

#[test]
fn e01_alias_reexport_chirho() {
    let modules_chirho: ProgramChirho = &[
        (
            "DefChirho",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 promotion control E01 (#30849): the defining module of one constructor set.
module DefChirho (SideChirho (..)) where

data SideChirho = TChirho | UChirho
"###,
        ),
        (
            "ReexportChirho",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 promotion control E01 (#30849): re-exports DefChirho's constructors unchanged.
module ReexportChirho (SideChirho (..)) where

import DefChirho (SideChirho (..))
"###,
        ),
        (
            "Main",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 promotion control E01 (#30849): one promoted constructor reached through two
-- aliases, one of them a re-export, ticked and unticked. The equations are written with
-- one alias and used with the other, so the promoted identity must be the constructor,
-- not its spelling. Predicted GHC 9.14.1 output: True, True, False.
{-# LANGUAGE DataKinds, TypeFamilies, KindSignatures, StandaloneKindSignatures #-}
module Main where

import Data.Proxy (Proxy (..))
import qualified DefChirho as AlphaChirho
import qualified ReexportChirho as BetaChirho

class KnownBoolChirho (bChirho :: Bool) where
  boolValChirho :: Proxy bChirho -> Bool

instance KnownBoolChirho 'True where
  boolValChirho _ = True

instance KnownBoolChirho 'False where
  boolValChirho _ = False

type IsTChirho :: AlphaChirho.SideChirho -> Bool
type family IsTChirho sideChirho where
  IsTChirho 'AlphaChirho.TChirho = 'True
  IsTChirho 'AlphaChirho.UChirho = 'False

main :: IO ()
main = do
  print (boolValChirho (Proxy :: Proxy (IsTChirho 'BetaChirho.TChirho)))
  print (boolValChirho (Proxy :: Proxy (IsTChirho BetaChirho.TChirho)))
  print (boolValChirho (Proxy :: Proxy (IsTChirho 'BetaChirho.UChirho)))
"###,
        ),
    ];
    assert_exact_chirho((modules_chirho, "True\nTrue\nFalse\n"));
}

#[test]
fn f01_stamped_synonym_chirho() {
    let modules_chirho: ProgramChirho = &[
        (
            "DefChirho",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 promotion stamping control: this module's promoted 'TChirho leaves it inside
-- an exported synonym.
{-# LANGUAGE DataKinds #-}
module DefChirho (KChirho (..), SChirho) where

data KChirho = TChirho

type SChirho = 'TChirho
"###,
        ),
        (
            "Main",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 promotion control F01: an imported synonym keeps its exporter's promoted
-- constructor although this module declares its own TChirho. Predicted GHC 9.14.1
-- output: def, main.
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, FlexibleInstances #-}
module Main where

import Data.Proxy (Proxy (..))
import qualified DefChirho

data LChirho = TChirho

class WhichChirho (t :: k) where
  whichChirho :: Proxy t -> String

instance WhichChirho 'DefChirho.TChirho where
  whichChirho _ = "def"

instance WhichChirho 'TChirho where
  whichChirho _ = "main"

main :: IO ()
main = do
  putStrLn (whichChirho (Proxy :: Proxy DefChirho.SChirho))
  putStrLn (whichChirho (Proxy :: Proxy 'TChirho))
"###,
        ),
    ];
    assert_exact_chirho((modules_chirho, "def\nmain\n"));
}

#[test]
fn f02_stamped_synonym_mismatch_chirho() {
    let modules_chirho: ProgramChirho = &[
        (
            "DefChirho",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 promotion stamping control: this module's promoted 'TChirho leaves it inside
-- an exported synonym.
{-# LANGUAGE DataKinds #-}
module DefChirho (KChirho (..), SChirho) where

data KChirho = TChirho

type SChirho = 'TChirho
"###,
        ),
        (
            "Main",
            r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 promotion control F02: the checker negative for F01. The imported synonym is
-- DefChirho's 'TChirho, of kind KChirho, so it cannot stand where this module's
-- 'TChirho, of kind LChirho, is required. Predicted GHC 9.14.1: rejected.
{-# LANGUAGE DataKinds #-}
module Main where

import Data.Proxy (Proxy (..))
import qualified DefChirho

data LChirho = TChirho

needsMainChirho :: Proxy 'TChirho -> ()
needsMainChirho _ = ()

main :: IO ()
main = print (needsMainChirho (Proxy :: Proxy DefChirho.SChirho))
"###,
        ),
    ];
    assert_refused_chirho((modules_chirho, "TChirho"));
}
