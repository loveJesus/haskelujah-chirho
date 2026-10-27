// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC 9.14.1's `properFraction` answers at Double on all three engines.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

const PROPER_FRACTION_CHIRHO: &str = r#"module Main where
fractionsChirho :: [Double] -> [(Int, Double)]
fractionsChirho = map properFraction
main :: IO ()
main = do
  print (properFraction (3.75 :: Double) :: (Int, Double))
  print (properFraction (-3.75 :: Double) :: (Int, Double))
  print (properFraction (2.0 :: Double) :: (Int, Double))
  print (properFraction (-0.0 :: Double) :: (Int, Double))
  print (properFraction (5.0e-324 :: Double) :: (Int, Double))
  print (properFraction (1 / 0 :: Double) :: (Int, Double))
  print (properFraction (-1 / 0 :: Double) :: (Int, Double))
  print (properFraction (0 / 0 :: Double) :: (Int, Double))
  print (properFraction (1.0e20 :: Double) :: (Int, Double))
  print (properFraction (-1.0e20 :: Double) :: (Int, Double))
  print (properFraction (3.75 :: Double) :: (Integer, Double))
  print (fractionsChirho [1.25, -2.5])
  print (truncate (1 / 0 :: Double) :: Int)
  print (truncate (-1 / 0 :: Double) :: Int)
  print (truncate (1.0e20 :: Double) :: Int)
"#;

const PROPER_FRACTION_GHC_CHIRHO: &str = "(3,0.75)\n(-3,-0.75)\n(2,0.0)\n(0,0.0)\n(0,5.0e-324)\n(0,0.0)\n(0,0.0)\n(0,0.0)\n(7766279631452241920,0.0)\n(-7766279631452241920,0.0)\n(3,0.75)\n[(1,0.25),(-2,-0.5)]\n0\n0\n7766279631452241920\n";

const OWN_PROPER_FRACTION_CHIRHO: &str = r#"module Main where
import Prelude hiding (properFraction)
properFraction :: Double -> (Int, Double)
properFraction valueChirho = (7, valueChirho)
main :: IO ()
main = print (properFraction 1.25)
"#;

fn interpreter_output_chirho(name_chirho: &str, source_chirho: &str) -> String {
    let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        name_chirho,
        None,
    )
    .unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho}"));
    machine_chirho.io_output_chirho
}

fn assert_everywhere_chirho(name_chirho: &str, source_chirho: &str, expected_chirho: &str) {
    assert_eq!(
        interpreter_output_chirho(name_chirho, source_chirho),
        expected_chirho,
        "interpreter"
    );
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        assert_eq!(
            native_round_trip_chirho(source_chirho, backend_chirho, "").unwrap_or_else(
                |error_chirho| panic!("{name_chirho} {backend_chirho:?}: {error_chirho}")
            ),
            (0, expected_chirho.to_string()),
            "{backend_chirho:?}"
        );
    }
}

#[test]
fn proper_fraction_answers_as_ghc_everywhere_chirho() {
    assert_everywhere_chirho(
        "ProperFractionChirho.hs",
        PROPER_FRACTION_CHIRHO,
        PROPER_FRACTION_GHC_CHIRHO,
    );
}

#[test]
fn a_programs_own_proper_fraction_stays_its_own_everywhere_chirho() {
    assert_everywhere_chirho(
        "OwnProperFractionChirho.hs",
        OWN_PROPER_FRACTION_CHIRHO,
        "(7,1.25)\n",
    );
}
