// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC 9.14.1's `RealFloat Double` predicate answers, across all three engines.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

const PREDICATES_CHIRHO: &str = r#"module Main where
predicateRowChirho :: Double -> [Bool]
predicateRowChirho valueChirho = [isNaN valueChirho, isInfinite valueChirho, isDenormalized valueChirho, isNegativeZero valueChirho, isIEEE valueChirho]
topLevelNegativeZeroChirho :: Double
topLevelNegativeZeroChirho = -0.0
main :: IO ()
main = do
  print (predicateRowChirho 0.0)
  print (predicateRowChirho topLevelNegativeZeroChirho)
  print (predicateRowChirho 1.0)
  print (predicateRowChirho (1 / 0))
  print (predicateRowChirho (-1 / 0))
  print (predicateRowChirho (0 / 0))
  print (predicateRowChirho 5.0e-324)
  print (predicateRowChirho 2.2250738585072014e-308)
  print (map predicateRowChirho [0.0, -0.0, 1 / 0, 0 / 0])
"#;

const PREDICATES_GHC_CHIRHO: &str = "[False,False,False,False,True]\n[False,False,False,True,True]\n[False,False,False,False,True]\n[False,True,False,False,True]\n[False,True,False,False,True]\n[True,False,False,False,True]\n[False,False,True,False,True]\n[False,False,False,False,True]\n[[False,False,False,False,True],[False,False,False,True,True],[False,True,False,False,True],[True,False,False,False,True]]\n";

const PROPERTIES_CHIRHO: &str = r#"module Main where
topLevelChirho :: Double
topLevelChirho = 1.5
main :: IO ()
main = do
  print (floatRadix topLevelChirho)
  print (floatDigits (0 / 0 :: Double))
  print (floatRange (1 / 0 :: Double))
  print (map floatRadix [0.0, -0.0, 1.0 :: Double])
  print (map floatDigits [0.0, 0 / 0, 1 / 0 :: Double])
  print (map floatRange [0.0, 5.0e-324, 2.2250738585072014e-308 :: Double])
"#;

const PROPERTIES_GHC_CHIRHO: &str =
    "2\n53\n(-1021,1024)\n[2,2,2]\n[53,53,53]\n[(-1021,1024),(-1021,1024),(-1021,1024)]\n";

const POLYMORPHIC_CHIRHO: &str = r#"module Main where
inspectChirho :: RealFloat aChirho => aChirho -> [Bool]
inspectChirho valueChirho = [isNaN valueChirho, isInfinite valueChirho, isDenormalized valueChirho, isNegativeZero valueChirho, isIEEE valueChirho]
main :: IO ()
main = do
  print (inspectChirho (0 / 0 :: Double))
  print (inspectChirho (-0.0 :: Double))
"#;

const OWN_NAMES_CHIRHO: &str = r#"module Main where
import Prelude hiding (isDenormalized, isIEEE, isInfinite, isNaN, isNegativeZero)
isNaN :: Double -> Bool
isNaN _ = False
isInfinite :: Double -> Bool
isInfinite _ = False
isDenormalized :: Double -> Bool
isDenormalized _ = False
isNegativeZero :: Double -> Bool
isNegativeZero _ = False
isIEEE :: Double -> Bool
isIEEE _ = False
main :: IO ()
main = print [isNaN (0 / 0), isInfinite (1 / 0), isDenormalized 5.0e-324, isNegativeZero (-0.0), isIEEE 1.0]
"#;

const OWN_PROPERTY_NAMES_CHIRHO: &str = r#"module Main where
import Prelude hiding (floatDigits, floatRadix, floatRange)
floatRadix :: Double -> Integer
floatRadix _ = 9
floatDigits :: Double -> Int
floatDigits _ = 8
floatRange :: Double -> (Int, Int)
floatRange _ = (7, 6)
main :: IO ()
main = do
  print (floatRadix 1.0)
  print (floatDigits 1.0)
  print (floatRange 1.0)
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
fn real_float_predicates_answer_as_ghc_everywhere_chirho() {
    assert_everywhere_chirho(
        "RealFloatPredicatesChirho.hs",
        PREDICATES_CHIRHO,
        PREDICATES_GHC_CHIRHO,
    );
}

#[test]
fn real_float_properties_answer_as_ghc_everywhere_chirho() {
    assert_everywhere_chirho(
        "RealFloatPropertiesChirho.hs",
        PROPERTIES_CHIRHO,
        PROPERTIES_GHC_CHIRHO,
    );
}

#[test]
fn real_float_predicates_work_through_a_passed_dictionary_in_the_interpreter_chirho() {
    assert_eq!(
        interpreter_output_chirho("PolymorphicPredicatesChirho.hs", POLYMORPHIC_CHIRHO),
        "[True,False,False,False,True]\n[False,False,False,True,True]\n"
    );
}

#[test]
fn a_programs_own_real_float_predicate_names_stay_its_own_everywhere_chirho() {
    assert_everywhere_chirho(
        "OwnRealFloatPredicatesChirho.hs",
        OWN_NAMES_CHIRHO,
        "[False,False,False,False,False]\n",
    );
}

#[test]
fn a_programs_own_real_float_property_names_stay_its_own_everywhere_chirho() {
    assert_everywhere_chirho(
        "OwnRealFloatPropertiesChirho.hs",
        OWN_PROPERTY_NAMES_CHIRHO,
        "9\n8\n(7,6)\n",
    );
}
