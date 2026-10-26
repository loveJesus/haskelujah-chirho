// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 520: each free `toEnum` and `fromEnum` reference gets an occurrence id,
//! so the checker's evidence chooses its `Enum` instance. Both names otherwise
//! guessed the Int row from an argument or shared canonical id. GHC 9.14.1 was
//! run on every program below with the recorded output.
//! The native control observes a generated Char list through `fromEnum`; native
//! rendering of a runtime-built String remains the separately owned row 557.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_named_failure_everywhere_chirho, assert_native_chirho, run_chirho};

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

#[test]
fn char_successor_and_predecessor_follow_the_enum_row_everywhere_chirho() {
    let program_chirho = r#"module Main where
data ColorChirho = RedChirho | GreenChirho | BlueChirho deriving (Show, Enum)
nextChirho :: Enum a => a -> a
nextChirho = succ
previousChirho :: Enum a => a -> a
previousChirho = pred
main :: IO ()
main = do
  print (fromEnum (succ 'a'))
  print (fromEnum (pred 'b'))
  print (map fromEnum (map succ "ab"))
  print (map fromEnum (map pred "bc"))
  print (nextChirho 'c')
  print (previousChirho 'd')
  print (succ RedChirho)
  print (pred BlueChirho)
"#;
    let ghc_chirho = "98\n97\n[98,99]\n[97,98]\n'd'\n'c'\nGreenChirho\nGreenChirho\n";

    assert_eq!(
        run_chirho("CharEnumSuccPredChirho.hs", program_chirho),
        ghc_chirho
    );
    assert_native_chirho("CharEnumSuccPredChirho.hs", program_chirho, ghc_chirho);
}

#[test]
fn char_successor_and_predecessor_keep_ghcs_endpoint_errors_everywhere_chirho() {
    for (file_name_chirho, operation_chirho, message_chirho) in [
        (
            "CharSuccMaxChirho.hs",
            "succ (maxBound :: Char)",
            "Prelude.Enum.Char.succ: bad argument",
        ),
        (
            "CharPredMinChirho.hs",
            "pred (minBound :: Char)",
            "Prelude.Enum.Char.pred: bad argument",
        ),
    ] {
        let program_chirho =
            format!("module Main where\nmain :: IO ()\nmain = print ({operation_chirho})\n");
        assert_named_failure_everywhere_chirho(file_name_chirho, &program_chirho, message_chirho);
    }
}

#[test]
fn surrogate_crossings_stay_a_named_runtime_boundary_everywhere_chirho() {
    for (file_name_chirho, operation_chirho) in [
        ("CharSuccSurrogateChirho.hs", "succ '\\xD7FF'"),
        ("CharPredSurrogateChirho.hs", "pred '\\xE000'"),
    ] {
        // GHC 9.14.1 produces '\55296' and '\57343'. The interpreter stores a
        // Rust `char`, which cannot represent either surrogate. Loud refusal is
        // retained until the shared Char representation can carry GHC values.
        let program_chirho =
            format!("module Main where\nmain :: IO ()\nmain = print ({operation_chirho})\n");
        assert_named_failure_everywhere_chirho(
            file_name_chirho,
            &program_chirho,
            "Enum Char result is not representable by the current Unicode scalar runtime",
        );
    }
}
