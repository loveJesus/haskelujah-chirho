// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 671: GHC's eight Enum methods are dictionary slots, so a polymorphic `Enum a =>` call runs its
//! instance's own enumeration: at the builtin keys, an instance's written method or GHC's default, and
//! a derived type's bounded rows. GHC 9.14.1 was run on every program below, and each is checked byte
//! for byte on the interpreter, LLVM and true Cranelift. A program's own `enumFrom` or `enumFromTo` stays
//! its own, and range syntax keeps the Prelude's. Float and `()` have no enumeration rows and fail by
//! the method's name. Enum Double is held row 670's.
//!
//! Measured residuals stay out of these controls: a context instance's dictionary at a call site, Char
//! `enumFrom` in the interpreter, and Float's representation (row 676).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_ghc_everywhere_chirho, assert_named_failure_everywhere_chirho};

/// Polymorphic enumeration at Int, Char, Integer, Bool and Ordering.
const POLYMORPHIC_BUILTINS_CHIRHO: (&str, &str) = (
    r#"module Main where

rangeChirho :: Enum a => a -> a -> [a]
rangeChirho = enumFromTo

startChirho :: Enum a => a -> [a]
startChirho = enumFrom

stepsChirho :: Enum a => a -> a -> [a]
stepsChirho = enumFromThen

spanChirho :: Enum a => a -> a -> a -> [a]
spanChirho = enumFromThenTo

main :: IO ()
main = do
  print (rangeChirho (1 :: Int) 5)
  print (take 3 (startChirho (7 :: Int)))
  print (take 4 (stepsChirho (10 :: Int) 7))
  print (spanChirho (1 :: Int) 4 12)
  print (rangeChirho 'a' 'e')
  print (spanChirho 'a' 'c' 'i')
  print (rangeChirho (2 :: Integer) 6)
  print (startChirho False)
  print (rangeChirho False True)
  print (stepsChirho True False)
  print (startChirho LT)
  print (stepsChirho GT EQ)
  print (spanChirho LT EQ GT)
"#,
    r#"[1,2,3,4,5]
[7,8,9]
[10,7,4,1]
[1,4,7,10]
"abcde"
"acegi"
[2,3,4,5,6]
[False,True]
[False,True]
[True,False]
[LT,EQ,GT]
[GT,EQ,LT]
[LT,EQ,GT]
"#,
);

/// An instance's own `enumFromTo` through `Enum a =>`, beside GHC's defaults for the rest.
const USER_OVERRIDE_CHIRHO: (&str, &str) = (
    r#"module Main where

data ColorChirho = RedChirho | GreenChirho | BlueChirho deriving (Show, Eq)

instance Enum ColorChirho where
  toEnum 0 = RedChirho
  toEnum 1 = GreenChirho
  toEnum _ = BlueChirho
  fromEnum RedChirho = 0
  fromEnum GreenChirho = 1
  fromEnum BlueChirho = 2
  enumFromTo lowChirho highChirho = [highChirho, lowChirho]

rangeChirho :: Enum a => a -> a -> [a]
rangeChirho = enumFromTo

startChirho :: Enum a => a -> [a]
startChirho = enumFrom

stepsChirho :: Enum a => a -> a -> [a]
stepsChirho = enumFromThen

spanChirho :: Enum a => a -> a -> a -> [a]
spanChirho = enumFromThenTo

main :: IO ()
main = do
  print (rangeChirho RedChirho BlueChirho)
  print (enumFromTo GreenChirho BlueChirho)
  print (take 2 (startChirho GreenChirho))
  print (spanChirho RedChirho GreenChirho BlueChirho)
"#,
    r#"[BlueChirho,RedChirho]
[BlueChirho,GreenChirho]
[GreenChirho,BlueChirho]
[RedChirho,GreenChirho,BlueChirho]
"#,
);

/// An instance that writes only `toEnum` and `fromEnum`, through `Enum a =>`.
const USER_DEFAULTS_CHIRHO: (&str, &str) = (
    r#"module Main where

data StageChirho = SeedChirho | SproutChirho | TreeChirho deriving Show

instance Enum StageChirho where
  toEnum 0 = SeedChirho
  toEnum 1 = SproutChirho
  toEnum 2 = TreeChirho
  toEnum n = error ("StageChirho.toEnum: " ++ show n)
  fromEnum SeedChirho = 0
  fromEnum SproutChirho = 1
  fromEnum TreeChirho = 2

rangeChirho :: Enum a => a -> a -> [a]
rangeChirho = enumFromTo

startChirho :: Enum a => a -> [a]
startChirho = enumFrom

stepsChirho :: Enum a => a -> a -> [a]
stepsChirho = enumFromThen

spanChirho :: Enum a => a -> a -> a -> [a]
spanChirho = enumFromThenTo

main :: IO ()
main = do
  print (rangeChirho SeedChirho TreeChirho)
  print (take 2 (startChirho SproutChirho))
  print (take 2 (stepsChirho SeedChirho TreeChirho))
  print (spanChirho TreeChirho SproutChirho SeedChirho)
"#,
    r#"[SeedChirho,SproutChirho,TreeChirho]
[SproutChirho,TreeChirho]
[SeedChirho,TreeChirho]
[TreeChirho,SproutChirho,SeedChirho]
"#,
);

/// A derived type's bounded enumerations through `Enum a =>`.
const DERIVED_CHIRHO: (&str, &str) = (
    r#"module Main where

data DayChirho = MonChirho | TueChirho | WedChirho | ThuChirho deriving (Show, Enum, Bounded)

rangeChirho :: Enum a => a -> a -> [a]
rangeChirho = enumFromTo

startChirho :: Enum a => a -> [a]
startChirho = enumFrom

stepsChirho :: Enum a => a -> a -> [a]
stepsChirho = enumFromThen

spanChirho :: Enum a => a -> a -> a -> [a]
spanChirho = enumFromThenTo

main :: IO ()
main = do
  print (startChirho TueChirho)
  print (stepsChirho MonChirho WedChirho)
  print (stepsChirho ThuChirho WedChirho)
  print (rangeChirho TueChirho ThuChirho)
  print (spanChirho MonChirho TueChirho ThuChirho)
"#,
    r#"[TueChirho,WedChirho,ThuChirho]
[MonChirho,WedChirho]
[ThuChirho,WedChirho,TueChirho,MonChirho]
[TueChirho,WedChirho,ThuChirho]
[MonChirho,TueChirho,WedChirho,ThuChirho]
"#,
);

/// Local functions named `enumFromTo` and `enumFrom` beside range syntax, which keeps the Prelude's.
const LOCAL_SHADOW_CHIRHO: (&str, &str) = (
    r#"module Main where

main :: IO ()
main = do
  let enumFromTo lowChirho highChirho = [highChirho, lowChirho]
  print (enumFromTo (1 :: Int) 3)
  print [1 .. 3 :: Int]
  print (twiceChirho 'q')
  where
    twiceChirho c = enumFrom c c
    enumFrom a b = [a, b]
"#,
    r#"[3,1]
[1,2,3]
"qq"
"#,
);

/// Top-level `enumFrom` and `enumFromTo` of the program's own under `hiding`, beside range syntax.
const HIDING_OWN_CHIRHO: (&str, &str) = (
    r#"module Main where

import Prelude hiding (enumFrom, enumFromTo)

enumFromTo :: Int -> Int -> [Int]
enumFromTo lowChirho highChirho = [lowChirho * 10, highChirho * 10]

enumFrom :: Char -> String
enumFrom c = [c, c]

main :: IO ()
main = do
  print (enumFromTo 1 2)
  putStrLn (enumFrom 'z')
  print [1 .. 3 :: Int]
  print (take 2 [5 :: Int ..])
"#,
    r#"[10,20]
zz
[1,2,3]
[5,6]
"#,
);

/// A qualified `P.enumFromTo` reaching an instance's own method beside a program-owned `enumFromTo` (row 673).
const QUALIFIED_BESIDE_OWNED_CHIRHO: (&str, &str) = (
    r#"module Main where

import Prelude hiding (enumFromTo)
import qualified Prelude as P

data ColorChirho = RedChirho | GreenChirho | BlueChirho deriving Show

instance P.Enum ColorChirho where
  fromEnum RedChirho = 0
  fromEnum GreenChirho = 1
  fromEnum BlueChirho = 2
  toEnum 0 = RedChirho
  toEnum 1 = GreenChirho
  toEnum _ = BlueChirho
  enumFromTo lowChirho highChirho = [highChirho, lowChirho]

enumFromTo :: Int -> Int -> [Int]
enumFromTo _ _ = []

main :: IO ()
main = do
  print (P.enumFromTo RedChirho BlueChirho)
  print (enumFromTo 1 3)
"#,
    r#"[BlueChirho,RedChirho]
[]
"#,
);

/// Written enumeration calls and ranges at Int, Char, Integer, Ordering and a derived type.
const WRITTEN_AND_RANGES_CHIRHO: (&str, &str) = (
    r#"module Main where

data DayChirho = MonChirho | TueChirho | WedChirho deriving (Show, Enum)

main :: IO ()
main = do
  print [1 .. 5 :: Int]
  print ['a' .. 'e']
  print [1, 3 .. 9 :: Int]
  print (take 3 [10 :: Int ..])
  print [10, 8 .. 1 :: Int]
  print (enumFromTo (1 :: Integer) 4)
  print (enumFromThenTo LT EQ GT)
  print (enumFrom TueChirho)
  print [MonChirho ..]
  print (map fromEnum [MonChirho .. WedChirho])
"#,
    r#"[1,2,3,4,5]
"abcde"
[1,3,5,7,9]
[10,11,12]
[10,8,6,4,2]
[1,2,3,4]
[LT,EQ,GT]
[TueChirho,WedChirho]
[MonChirho,TueChirho,WedChirho]
[0,1,2]
"#,
);

/// An inferred function whose range endpoints are arithmetic, at Int and Integer.
const INFERRED_NUMERIC_RANGE_CHIRHO: (&str, &str) = (
    r#"module Main where

rangeChirho x = enumFromTo (x + 1) (x + 3)

main :: IO ()
main = do
  print (rangeChirho (2 :: Int))
  print (rangeChirho (5 :: Integer))
"#,
    r#"[3,4,5]
[6,7,8]
"#,
);

/// A signed `(Num a, Enum a)` function whose range endpoints are arithmetic, at Int and Integer.
const SIGNED_NUMERIC_RANGE_CHIRHO: (&str, &str) = (
    r#"module Main where

rangeChirho :: (Num a, Enum a) => a -> [a]
rangeChirho x = enumFromTo (x + 1) (x + 3)

main :: IO ()
main = do
  print (rangeChirho (2 :: Int))
  print (rangeChirho (5 :: Integer))
"#,
    r#"[3,4,5]
[6,7,8]
"#,
);

/// An inferred function enumerating its own arguments, at Int and Char.
const INFERRED_ENUMERATION_CHIRHO: (&str, &str) = (
    r#"module Main where

spanChirho x y = enumFromTo x y

main :: IO ()
main = do
  print (spanChirho (2 :: Int) 4)
  print (spanChirho 'a' 'd')
"#,
    r#"[2,3,4]
"abcd"
"#,
);

/// Float has no enumeration rows, so its slot fails by the method's name.
const FLOAT_CHIRHO: (&str, &str) = (
    r#"module Main where

rangeChirho :: Enum a => a -> a -> [a]
rangeChirho = enumFromTo

startChirho :: Enum a => a -> [a]
startChirho = enumFrom

stepsChirho :: Enum a => a -> a -> [a]
stepsChirho = enumFromThen

spanChirho :: Enum a => a -> a -> a -> [a]
spanChirho = enumFromThenTo

main :: IO ()
main = print (rangeChirho (1.0 :: Float) 3.0)
"#,
    r#"missing method Enum.enumFromTo for Float"#,
);

/// `()` has no enumeration rows, so its slot fails by the method's name.
const UNIT_CHIRHO: (&str, &str) = (
    r#"module Main where

rangeChirho :: Enum a => a -> a -> [a]
rangeChirho = enumFromTo

startChirho :: Enum a => a -> [a]
startChirho = enumFrom

stepsChirho :: Enum a => a -> a -> [a]
stepsChirho = enumFromThen

spanChirho :: Enum a => a -> a -> a -> [a]
spanChirho = enumFromThenTo

main :: IO ()
main = print (startChirho ())
"#,
    r#"missing method Enum.enumFrom for ()"#,
);

#[test]
fn enumerations_answer_as_ghcs_do_everywhere_chirho() {
    for (file_name_chirho, (source_chirho, expected_chirho)) in [
        ("PolymorphicBuiltinsChirho.hs", POLYMORPHIC_BUILTINS_CHIRHO),
        ("UserOverrideChirho.hs", USER_OVERRIDE_CHIRHO),
        ("UserDefaultsChirho.hs", USER_DEFAULTS_CHIRHO),
        ("DerivedChirho.hs", DERIVED_CHIRHO),
        ("LocalShadowChirho.hs", LOCAL_SHADOW_CHIRHO),
        ("HidingOwnChirho.hs", HIDING_OWN_CHIRHO),
        (
            "QualifiedBesideOwnedChirho.hs",
            QUALIFIED_BESIDE_OWNED_CHIRHO,
        ),
        ("WrittenAndRangesChirho.hs", WRITTEN_AND_RANGES_CHIRHO),
        (
            "InferredNumericRangeChirho.hs",
            INFERRED_NUMERIC_RANGE_CHIRHO,
        ),
        ("SignedNumericRangeChirho.hs", SIGNED_NUMERIC_RANGE_CHIRHO),
        ("InferredEnumerationChirho.hs", INFERRED_ENUMERATION_CHIRHO),
    ] {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, expected_chirho);
    }
}

#[test]
fn unsupported_enumeration_slots_fail_by_name_everywhere_chirho() {
    for (file_name_chirho, (source_chirho, expected_chirho)) in [
        ("FloatChirho.hs", FLOAT_CHIRHO),
        ("UnitChirho.hs", UNIT_CHIRHO),
    ] {
        assert_named_failure_everywhere_chirho(file_name_chirho, source_chirho, expected_chirho);
    }
}
