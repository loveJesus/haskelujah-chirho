// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 520: each free `toEnum` and `fromEnum` reference gets an occurrence id,
//! so the checker's evidence chooses its `Enum` instance. Both names otherwise
//! guessed the Int row from an argument or shared canonical id. GHC 9.14.1 was
//! run on every program below with the recorded output.
//! The native control observes a generated Char list through `fromEnum`; native
//! rendering of a runtime-built String remains the separately owned row 557.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_native_chirho, run_chirho};

#[test]
fn enum_conversions_follow_each_proven_instance_everywhere_chirho() {
    let program_chirho = r#"module Main where
data DirectionChirho = UpChirho | DownChirho deriving (Show, Enum)
data ColorChirho = RedChirho | GreenChirho deriving Show
instance Enum ColorChirho where
  toEnum 0 = RedChirho
  toEnum _ = GreenChirho
  fromEnum RedChirho = 40
  fromEnum GreenChirho = 41
main :: IO ()
main = do
  print (toEnum 66 :: Char)
  print (fromEnum 'B')
  print (map fromEnum (map toEnum [97, 98] :: [Char]))
  print (map fromEnum "ab")
  print (toEnum 1 :: DirectionChirho)
  print (map fromEnum [UpChirho, DownChirho])
  print (toEnum 1 :: ColorChirho)
  print (map fromEnum [RedChirho, GreenChirho])
"#;
    let ghc_chirho = "'B'\n66\n[97,98]\n[97,98]\nDownChirho\n[0,1]\nGreenChirho\n[40,41]\n";

    assert_eq!(
        run_chirho("EnumConversionsChirho.hs", program_chirho),
        ghc_chirho
    );
    assert_native_chirho("EnumConversionsChirho.hs", program_chirho, ghc_chirho);
}

#[test]
fn a_witness_carries_the_callers_enum_dictionary_everywhere_chirho() {
    let program_chirho = r#"module Main where
enumWithWitnessChirho :: Enum a => a -> Int -> a
enumWithWitnessChirho _ valueChirho = toEnum valueChirho
main :: IO ()
main = do
  print (enumWithWitnessChirho 'x' 98)
  print (fromEnum (enumWithWitnessChirho 'x' 99))
"#;
    let ghc_chirho = "'b'\n99\n";

    assert_eq!(
        run_chirho("EnumWitnessChirho.hs", program_chirho),
        ghc_chirho
    );
    assert_native_chirho("EnumWitnessChirho.hs", program_chirho, ghc_chirho);
}

#[test]
fn local_and_top_level_shadows_keep_their_own_bindings_chirho() {
    let local_chirho = r#"module Main where
main :: IO ()
main = print (let toEnum = (+ 1) in toEnum (5 :: Int))
"#;
    let top_level_chirho = r#"module Main where
import Prelude hiding (fromEnum)
fromEnum :: Int -> Int
fromEnum = (* 2)
main :: IO ()
main = print (fromEnum 21)
"#;

    assert_eq!(run_chirho("LocalEnumShadowChirho.hs", local_chirho), "6\n");
    assert_eq!(
        run_chirho("TopLevelEnumShadowChirho.hs", top_level_chirho),
        "42\n"
    );
}

#[test]
fn result_only_polymorphic_to_enum_keeps_the_named_boundary_chirho() {
    let program_chirho = r#"module Main where
enumAtChirho :: Enum a => Int -> a
enumAtChirho valueChirho = toEnum valueChirho
main :: IO ()
main = print (enumAtChirho 97 :: Char)
"#;

    // The pass still defaults a result-only class variable before occurrence
    // dispatch. Row 520 repairs occurrences whose proof reaches the pass; it
    // does not add result-type propagation.
    assert_eq!(
        run_chirho("ResultOnlyEnumChirho.hs", program_chirho),
        "97\n"
    );
}
