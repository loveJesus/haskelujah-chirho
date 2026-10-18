// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Context controls from Claude2 review #27228: equality givens, refinements
//! reaching later patterns, pattern synonyms and imported constructors. GHC
//! receipts predate embedding. The imported infix constructor control now runs
//! through the shared constructor-operator repair as well. Candidate/corpus
//! outcomes and remaining arity boundaries are recorded in the tasklist.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::{assert_accepted_chirho, assert_output_chirho, assert_rejected_with_chirho};

const CONTROLS_CHIRHO: &[(&str, &str, Option<&str>)] = &[
    (
        "kr01_equality_given_tuple_pattern_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies #-}
module Main where
fChirho :: (aChirho ~ (Int, Bool)) => aChirho -> Int
fChirho (xChirho, _) = xChirho
main :: IO ()
main = print (fChirho (3, True))
"#,
        Some("3\n"),
    ),
    (
        "kr02_equality_given_constructor_pattern_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies #-}
module Main where
gChirho :: (aChirho ~ Maybe Int) => aChirho -> Int
gChirho (Just nChirho) = nChirho
gChirho Nothing = 0
main :: IO ()
main = print (gChirho (Just 4))
"#,
        Some("4\n"),
    ),
    (
        "kr03_refinement_reaches_later_tuple_arg_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
hChirho :: EqualChirho aChirho (Int, Bool) -> aChirho -> Int
hChirho ReflChirho (nChirho, _) = nChirho
main :: IO ()
main = print (hChirho ReflChirho (5, False))
"#,
        Some("5\n"),
    ),
    (
        "kr04_refinement_reaches_later_char_literal_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
kChirho :: EqualChirho aChirho Char -> aChirho -> Int
kChirho ReflChirho 'c' = 1
kChirho ReflChirho _ = 0
main :: IO ()
main = print (kChirho ReflChirho 'c', kChirho ReflChirho 'd')
"#,
        Some("(1,0)\n"),
    ),
    (
        "kr05_refinement_in_tuple_case_sibling_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
mChirho :: EqualChirho aChirho (Int, Bool) -> aChirho -> Int
mChirho eqChirho vChirho = case (eqChirho, vChirho) of (ReflChirho, (nChirho, _)) -> nChirho
main :: IO ()
main = print (mChirho ReflChirho (6, True))
"#,
        Some("6\n"),
    ),
    (
        "kr06_pattern_synonyms_view_and_cons_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PatternSynonyms, ViewPatterns #-}
module Main where
pattern HeadChirho :: aChirho -> [aChirho]
pattern HeadChirho xChirho <- (xChirho : _)
pattern EvenChirho :: Int
pattern EvenChirho <- (even -> True)
fChirho :: [Int] -> String
fChirho (HeadChirho EvenChirho) = "even head"
fChirho (HeadChirho _) = "odd head"
fChirho [] = "empty"
main :: IO ()
main = mapM_ (putStrLn . fChirho) [[2, 3], [3], []]
"#,
        Some("even head\nodd head\nempty\n"),
    ),
    (
        "kr07_refinement_reaches_later_string_literal_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
sChirho :: EqualChirho aChirho String -> aChirho -> Int
sChirho ReflChirho "hi" = 1
sChirho ReflChirho _ = 0
main :: IO ()
main = print (sChirho ReflChirho "hi", sChirho ReflChirho "yo")
"#,
        Some("(1,0)\n"),
    ),
    (
        "kr08_type_family_scrutinee_reduces_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies #-}
module Main where
type family FChirho aChirho where
  FChirho Int = Maybe Bool
fChirho :: FChirho Int -> Bool
fChirho (Just bChirho) = bChirho
fChirho Nothing = False
main :: IO ()
main = print (fChirho (Just True))
"#,
        Some("True\n"),
    ),
    (
        "kr09_imported_constructor_patterns_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import Data.List.NonEmpty (NonEmpty(..))
import Data.Complex (Complex(..))
import Data.Monoid (Sum(..), First(..))
import Data.Functor.Identity (Identity(..))
import Data.Ord (Down(..))
import System.Exit (ExitCode(..))
f1Chirho :: NonEmpty Int -> Int
f1Chirho (xChirho :| _) = xChirho
f2Chirho :: Complex Double -> Double
f2Chirho (rChirho :+ _) = rChirho
f3Chirho :: Sum Int -> Int
f3Chirho (Sum nChirho) = nChirho
f4Chirho :: First Char -> Maybe Char
f4Chirho (First mChirho) = mChirho
f5Chirho :: Identity Bool -> Bool
f5Chirho (Identity vChirho) = vChirho
f6Chirho :: Down Int -> Int
f6Chirho (Down dChirho) = dChirho
f7Chirho :: ExitCode -> Int
f7Chirho (ExitFailure cChirho) = cChirho
f7Chirho ExitSuccess = 0
main :: IO ()
main = do
  print (f1Chirho (1 :| [2]), f2Chirho (1.5 :+ 2), f3Chirho (Sum 4))
  print (f4Chirho (First (Just 'x')), f5Chirho (Identity True), f6Chirho (Down 9), f7Chirho (ExitFailure 3))
"#,
        Some("(1,1.5,4)\n(Just 'x',True,9,3)\n"),
    ),
    (
        "kr09c_newtype_and_enum_imported_constructors_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import Data.Monoid (Sum(..), First(..))
import Data.Functor.Identity (Identity(..))
import Data.Ord (Down(..))
import System.Exit (ExitCode(..))
f3Chirho :: Sum Int -> Int
f3Chirho (Sum nChirho) = nChirho
f4Chirho :: First Char -> Maybe Char
f4Chirho (First mChirho) = mChirho
f5Chirho :: Identity Bool -> Bool
f5Chirho (Identity vChirho) = vChirho
f6Chirho :: Down Int -> Int
f6Chirho (Down dChirho) = dChirho
f7Chirho :: ExitCode -> Int
f7Chirho (ExitFailure cChirho) = cChirho
f7Chirho ExitSuccess = 0
main :: IO ()
main = do
  print (f3Chirho (Sum 4))
  print (f4Chirho (First (Just 'x')), f5Chirho (Identity True), f6Chirho (Down 9), f7Chirho (ExitFailure 3))
"#,
        Some("4\n(Just 'x',True,9,3)\n"),
    ),
    (
        "kr10_unsigned_pattern_synonym_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PatternSynonyms #-}
module Main where
pattern PairChirho aChirho bChirho = (aChirho, bChirho)
pattern SingleChirho xChirho = [xChirho]
sumPairChirho :: (Int, Int) -> Int
sumPairChirho (PairChirho aChirho bChirho) = aChirho + bChirho
onlyChirho :: [Char] -> String
onlyChirho (SingleChirho cChirho) = [cChirho, cChirho]
onlyChirho _ = "many"
main :: IO ()
main = do
  print (sumPairChirho (PairChirho 2 3))
  putStrLn (onlyChirho "z")
  putStrLn (onlyChirho "zz")
"#,
        Some("5\nzz\nmany\n"),
    ),
];

#[test]
fn givens_and_refinements_keep_valid_contextual_patterns_chirho() {
    for &(file_name_chirho, source_chirho, _) in CONTROLS_CHIRHO {
        assert_accepted_chirho(file_name_chirho, source_chirho);
    }
}

#[test]
fn contextual_patterns_keep_their_runtime_values_chirho() {
    for &(file_name_chirho, source_chirho, output_chirho) in CONTROLS_CHIRHO {
        if let Some(expected_chirho) = output_chirho {
            assert_output_chirho(file_name_chirho, source_chirho, expected_chirho);
        }
    }
}

/// Empty record syntax still matches the constructor result and keeps its
/// refinement; its higher-rank field is neither opened nor demanded here.
/// GHC accepts this exact source; its comparison against the original main
/// and the repaired route is retained in the pattern gate.
#[test]
fn an_empty_gadt_pattern_keeps_result_refinement_with_rank_n_fields_chirho() {
    assert_accepted_chirho(
        "b2_01_record_wildcard_gadt_rankn_pos_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, RankNTypes #-}
module MChirho where

data GChirho aChirho where
  MkGChirho :: Int -> (forall bChirho. bChirho -> aChirho) -> GChirho Int

fChirho :: GChirho aChirho -> aChirho -> Int
fChirho (MkGChirho {}) xChirho = xChirho
"#,
    );
}

/// A higher-kinded index keeps the payload tied to its outer rigid argument:
/// `Wrap (Maybe t)` against `Wrap (f a)` learns both f ~ Maybe and t ~ a.
#[test]
fn a_higher_kinded_gadt_index_keeps_its_field_type_chirho() {
    assert_rejected_with_chirho(
        "HigherKindResultNegativeChirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE GADTs, KindSignatures, ExplicitForAll #-}
module Main where
import Data.Kind (Type)
data WrapChirho indexChirho where
  MkWrapChirho :: tChirho -> WrapChirho (Maybe tChirho)
badChirho :: forall (fChirho :: Type -> Type) aChirho. WrapChirho (fChirho aChirho) -> Bool
badChirho (MkWrapChirho xChirho) = xChirho
main = return ()
"#,
        "type mismatch",
    );
    assert_output_chirho(
        "HigherKindResultPositiveChirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE GADTs, KindSignatures, ExplicitForAll #-}
module Main where
import Data.Kind (Type)
data WrapChirho indexChirho where
  MkWrapChirho :: tChirho -> WrapChirho (Maybe tChirho)
unwrapChirho :: forall (fChirho :: Type -> Type) aChirho. WrapChirho (fChirho aChirho) -> aChirho
unwrapChirho (MkWrapChirho xChirho) = xChirho
main = print (unwrapChirho (MkWrapChirho (7 :: Int)))
"#,
        "7\n",
    );
}
