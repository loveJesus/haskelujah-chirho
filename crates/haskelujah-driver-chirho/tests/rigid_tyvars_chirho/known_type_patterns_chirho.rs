// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Known-type patterns: constructor shape before fields, local GADT result
//! givens and left-to-right siblings. All 43 Haskell sources were measured on
//! GHC 9.14.1 before these controls were written. Candidate tests are unrun at
//! the source checkpoint; no source-only pass or corpus gain is claimed.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::{assert_accepted_chirho, assert_output_chirho, assert_rejected_with_chirho};

#[test]
fn adversarial_chirho() {
    for (file_name_chirho, source_chirho, accepted_chirho) in [
        (
            "P01_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}

module P01NestedGadtUnderMaybePositiveChirho where

data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho

nestedGoodChirho :: Maybe (EqualChirho aChirho Bool) -> aChirho -> Bool
nestedGoodChirho (Just ReflChirho) xChirho = xChirho
nestedGoodChirho Nothing _ = False
"#,
            true,
        ),
        (
            "P02_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}

module P02NestedGadtAgainstBareSkolemNegativeChirho where

data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho

badNestedSkolemChirho :: aChirho -> Int
badNestedSkolemChirho (Just ReflChirho) = 1
"#,
            false,
        ),
        (
            "P03_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ExplicitForAll #-}

module P03ExplicitForallGadtNullaryPositiveChirho where

data EqualChirho aChirho bChirho where
  ReflChirho :: forall aChirho. EqualChirho aChirho aChirho

castExplicitForallChirho :: EqualChirho aChirho bChirho -> aChirho -> bChirho
castExplicitForallChirho ReflChirho xChirho = xChirho
"#,
            true,
        ),
        (
            "P04_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ExplicitForAll #-}

module P04ExplicitForallGadtUnaryNegativeChirho where

data GChirho aChirho where
  MkGChirho :: forall aChirho. aChirho -> GChirho Int

badExplicitForallUnaryChirho :: Bool -> Int
badExplicitForallUnaryChirho (MkGChirho xChirho) = 0
"#,
            false,
        ),
        (
            "P04b_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ExplicitForAll #-}

module P04bExplicitForallGadtUnarySkolemNegativeChirho where

data GChirho aChirho where
  MkGChirho :: forall aChirho. aChirho -> GChirho Int

badExplicitForallSkolemChirho :: bChirho -> Int
badExplicitForallSkolemChirho (MkGChirho xChirho) = 0
"#,
            false,
        ),
        (
            "P05_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}

module P05HigherRankConstructorFieldPositiveChirho where

newtype PolyChirho = MkPolyChirho (forall aChirho. aChirho -> aChirho)

usePolyChirho :: PolyChirho -> (Int, Bool)
usePolyChirho (MkPolyChirho fChirho) = (fChirho 1, fChirho True)
"#,
            true,
        ),
        (
            "P06_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module P06MonomorphicInfixConNegativeChirho where

data IntPairChirho = Int :!: Bool

badMonoInfixChirho :: Maybe aChirho -> Int
badMonoInfixChirho (xChirho :!: yChirho) = xChirho
"#,
            false,
        ),
        (
            "P07_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module P07RecordSiblingFieldSubstNegativeChirho where

data SamePairChirho aChirho = SamePairChirho { leftFChirho :: aChirho, rightFChirho :: aChirho }

badRecSiblingChirho SamePairChirho { leftFChirho = True, rightFChirho = 'x' } = (0 :: Int)
"#,
            false,
        ),
        (
            "P08_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}

module P08PartialGadtUnrefinedFieldNegativeChirho where

data PartialGadtChirho aChirho bChirho where
  MkPartialChirho :: aChirho -> PartialGadtChirho aChirho Int

badPartialChirho :: PartialGadtChirho aChirho bChirho -> Int
badPartialChirho (MkPartialChirho True) = 1
"#,
            false,
        ),
        (
            "P09_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}

module P09PartialGadtRefinedIndexPositiveChirho where

data PartialGadtChirho aChirho bChirho where
  MkPartialChirho :: aChirho -> PartialGadtChirho aChirho Int

goodPartialChirho :: PartialGadtChirho aChirho bChirho -> (aChirho, bChirho)
goodPartialChirho (MkPartialChirho xChirho) = (xChirho, 42 :: Int)
"#,
            true,
        ),
        (
            "P10_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}

module P10ReversedSiblingGadtRefinementNegativeChirho where

data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho

badOrderChirho :: (aChirho, EqualChirho aChirho Bool) -> Int
badOrderChirho (True, ReflChirho) = 1
"#,
            false,
        ),
        (
            "P10b_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}

module P10bReversedSiblingGadtCaseNegativeChirho where

data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho

badCaseOrderChirho :: (aChirho, EqualChirho aChirho Bool) -> Int
badCaseOrderChirho p = case p of
  (True, ReflChirho) -> 1
"#,
            false,
        ),
        (
            "P11_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module P11EmptyRecordPatOnPositionalConNegativeChirho where

badEmptyRecordOnPositionalChirho :: Int -> Int
badEmptyRecordOnPositionalChirho (Just {}) = 0
"#,
            false,
        ),
        (
            "P12_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module P12TupleListAgainstIntNegativeChirho where

badTuplePatChirho :: Int -> Int
badTuplePatChirho (xChirho, yChirho) = 0

badListPatChirho :: Int -> Int
badListPatChirho [xChirho] = 0
"#,
            false,
        ),
        (
            "P13_adversarial_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ExistentialQuantification #-}

module P13ExistentialAndRecordGadtPositiveChirho where

data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho

data SomeShowChirho = forall aChirho. MkSomeShowChirho aChirho (aChirho -> Int)

data RecBoxChirho aChirho = MkRecBoxChirho
  { eqFieldChirho :: EqualChirho aChirho Bool
  , valFieldChirho :: aChirho
  }

runNestedExistentialChirho :: Maybe SomeShowChirho -> Int
runNestedExistentialChirho (Just (MkSomeShowChirho xChirho toIntChirho)) = toIntChirho xChirho
runNestedExistentialChirho Nothing = 0

checkRecGadtFieldChirho :: RecBoxChirho aChirho -> Int
checkRecGadtFieldChirho MkRecBoxChirho { eqFieldChirho = ReflChirho, valFieldChirho = True } = 1
checkRecGadtFieldChirho _ = 0
"#,
            true,
        ),
    ] {
        if accepted_chirho {
            assert_accepted_chirho(file_name_chirho, source_chirho);
        } else {
            assert_rejected_with_chirho(file_name_chirho, source_chirho, "type mismatch");
        }
    }
}

#[test]
fn shapes_chirho() {
    for (file_name_chirho, source_chirho, accepted_chirho) in [
        (
            "constructor_int_equation_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
badChirho :: Int -> Int
badChirho (Just valueChirho) = valueChirho
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
        (
            "tuple_int_equation_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
badChirho :: Int -> Int
badChirho (leftChirho, rightChirho) = 0
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
        (
            "list_int_equation_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
badChirho :: Int -> Int
badChirho [valueChirho] = 0
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
        (
            "constructor_int_case_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
badChirho :: Int -> Int
badChirho valueChirho = case valueChirho of Just resultChirho -> resultChirho
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
        (
            "tuple_int_case_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
badChirho :: Int -> Int
badChirho valueChirho = case valueChirho of (leftChirho, rightChirho) -> 0
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
        (
            "list_int_case_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
badChirho :: Int -> Int
badChirho valueChirho = case valueChirho of [valueChirho] -> 0
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
        (
            "constructor_int_guard_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
badChirho :: Int -> Int
badChirho valueChirho | Just resultChirho <- valueChirho = resultChirho
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
        (
            "tuple_int_guard_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
badChirho :: Int -> Int
badChirho valueChirho | (leftChirho, rightChirho) <- valueChirho = 0
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
        (
            "list_int_guard_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
badChirho :: Int -> Int
badChirho valueChirho | [valueChirho] <- valueChirho = 0
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
    ] {
        if accepted_chirho {
            assert_accepted_chirho(file_name_chirho, source_chirho);
        } else {
            assert_rejected_with_chirho(file_name_chirho, source_chirho, "type mismatch");
        }
    }
}

#[test]
fn sibling_order_chirho() {
    for (file_name_chirho, source_chirho, accepted_chirho) in [
        (
            "generic_sibling_equation_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
castChirho :: (EqualChirho aChirho bChirho, aChirho) -> bChirho
castChirho (ReflChirho, True) = True
main :: IO ()
main = print (castChirho (ReflChirho, True))
"#,
            false,
        ),
        (
            "specialized_sibling_equation_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
castChirho :: (EqualChirho aChirho Bool, aChirho) -> Bool
castChirho (ReflChirho, True) = True
main :: IO ()
main = print (castChirho (ReflChirho, True))
"#,
            true,
        ),
        (
            "generic_sibling_case_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
castChirho :: (EqualChirho aChirho bChirho, aChirho) -> bChirho
castChirho valueChirho = case valueChirho of (ReflChirho, True) -> True
main :: IO ()
main = print (castChirho (ReflChirho, True))
"#,
            false,
        ),
        (
            "specialized_sibling_case_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
castChirho :: (EqualChirho aChirho Bool, aChirho) -> Bool
castChirho valueChirho = case valueChirho of (ReflChirho, True) -> True
main :: IO ()
main = print (castChirho (ReflChirho, True))
"#,
            true,
        ),
        (
            "generic_sibling_guard_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
castChirho :: (EqualChirho aChirho bChirho, aChirho) -> bChirho
castChirho valueChirho | (ReflChirho, True) <- valueChirho = True
main :: IO ()
main = print (castChirho (ReflChirho, True))
"#,
            false,
        ),
        (
            "specialized_sibling_guard_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
castChirho :: (EqualChirho aChirho Bool, aChirho) -> Bool
castChirho valueChirho | (ReflChirho, True) <- valueChirho = True
main :: IO ()
main = print (castChirho (ReflChirho, True))
"#,
            true,
        ),
        (
            "guard_reverse_refinement_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
badChirho :: (aChirho, EqualChirho aChirho Bool) -> Int
badChirho valueChirho | (True, ReflChirho) <- valueChirho = 0
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
    ] {
        if accepted_chirho {
            assert_accepted_chirho(file_name_chirho, source_chirho);
        } else {
            assert_rejected_with_chirho(file_name_chirho, source_chirho, "type mismatch");
        }
    }
}

#[test]
fn records_chirho() {
    for (file_name_chirho, source_chirho, accepted_chirho) in [
        (
            "record_constructor_siblings_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
data SamePairChirho aChirho = SamePairChirho { leftChirho :: aChirho, rightChirho :: aChirho }
badChirho SamePairChirho {leftChirho = True, rightChirho = Just valueChirho} = (0 :: Int)
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
        (
            "record_reverse_refinement_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
data BoxChirho aChirho = BoxChirho { proofChirho :: EqualChirho aChirho Bool, valueChirho :: aChirho }
badChirho :: BoxChirho aChirho -> Int
badChirho BoxChirho {valueChirho = True, proofChirho = ReflChirho} = 0
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
    ] {
        if accepted_chirho {
            assert_accepted_chirho(file_name_chirho, source_chirho);
        } else {
            assert_rejected_with_chirho(file_name_chirho, source_chirho, "type mismatch");
        }
    }
}

#[test]
fn literals_chirho() {
    for (file_name_chirho, source_chirho, accepted_chirho) in [
        (
            "char_against_int_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
badChirho :: Int -> Int
badChirho 'x' = 0
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
        (
            "string_against_int_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
badChirho :: Int -> Int
badChirho "value" = 0
main :: IO ()
main = putStrLn "ok"
"#,
            false,
        ),
        (
            "overloaded_string_pattern_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE OverloadedStrings #-}
module Main where
fChirho :: String -> Bool
fChirho "x" = True
fChirho _ = False
main :: IO ()
main = print (fChirho "x", fChirho "y")
"#,
            true,
        ),
        (
            "numeric_pattern_context_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
fChirho :: (Eq aChirho, Num aChirho) => aChirho -> Bool
fChirho 0 = True
fChirho _ = False
main :: IO ()
main = print (fChirho (0 :: Double), fChirho (1 :: Int))
"#,
            true,
        ),
        (
            "fractional_pattern_context_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
fChirho :: (Eq aChirho, Fractional aChirho) => aChirho -> Bool
fChirho 0.5 = True
fChirho _ = False
main :: IO ()
main = print (fChirho (0.5 :: Double), fChirho (1 :: Double))
"#,
            true,
        ),
    ] {
        if accepted_chirho {
            assert_accepted_chirho(file_name_chirho, source_chirho);
        } else {
            assert_rejected_with_chirho(file_name_chirho, source_chirho, "type mismatch");
        }
    }
}

#[test]
fn refined_and_higher_rank_fields_keep_their_runtime_values_chirho() {
    for (file_name_chirho, source_chirho, expected_chirho) in [
        (
            "nested_runtime_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
fChirho :: Maybe (EqualChirho aChirho Bool) -> aChirho -> Bool
fChirho (Just ReflChirho) valueChirho = valueChirho
fChirho Nothing _ = False
main :: IO ()
main = print (fChirho (Just ReflChirho) True, fChirho Nothing False)
"#,
            "(True,False)\n",
        ),
        (
            "explicit_forall_runtime_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ExplicitForAll #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: forall aChirho. EqualChirho aChirho aChirho
fChirho :: EqualChirho aChirho bChirho -> aChirho -> bChirho
fChirho ReflChirho valueChirho = valueChirho
main :: IO ()
main = print (fChirho ReflChirho True, fChirho ReflChirho (7 :: Int))
"#,
            "(True,7)\n",
        ),
        (
            "higher_rank_runtime_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
newtype PolyChirho = MkPolyChirho (forall aChirho. aChirho -> aChirho)
fChirho :: PolyChirho -> (Int, Bool)
fChirho (MkPolyChirho applyChirho) = (applyChirho 7, applyChirho True)
main :: IO ()
main = print (fChirho (MkPolyChirho id))
"#,
            "(7,True)\n",
        ),
        (
            "existential_runtime_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE ExistentialQuantification #-}
module Main where
data SomeChirho = forall aChirho. PackChirho aChirho (aChirho -> Int)
fChirho :: Maybe SomeChirho -> Int
fChirho (Just (PackChirho valueChirho applyChirho)) = applyChirho valueChirho
fChirho Nothing = 0
main :: IO ()
main = print (fChirho (Just (PackChirho (7 :: Int) id)), fChirho Nothing)
"#,
            "(7,0)\n",
        ),
        (
            "record_runtime_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
data BoxChirho aChirho = BoxChirho { proofChirho :: EqualChirho aChirho Bool, valueChirho :: aChirho }
fChirho :: BoxChirho aChirho -> Int
fChirho BoxChirho {proofChirho = ReflChirho, valueChirho = True} = 7
fChirho _ = 0
main :: IO ()
main = print (fChirho (BoxChirho ReflChirho True), fChirho (BoxChirho ReflChirho False))
"#,
            "(7,0)\n",
        ),
    ] {
        assert_output_chirho(file_name_chirho, source_chirho, expected_chirho);
    }
}
