// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC 9.14.1's `decodeFloat` and `encodeFloat` answers at Double.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

const REAL_FLOAT_CODEC_CHIRHO: &str = r#"module Main where
decodeManyChirho :: [Double] -> [(Integer, Int)]
decodeManyChirho = map decodeFloat
encodeManyChirho :: [(Integer, Int)] -> [Double]
encodeManyChirho = map (uncurry encodeFloat)
main :: IO ()
main = do
  print (decodeFloat (0.0 :: Double))
  print (decodeFloat (-0.0 :: Double))
  print (decodeFloat (1.5 :: Double))
  print (decodeFloat (-2.5 :: Double))
  print (decodeFloat (5.0e-324 :: Double))
  print (decodeFloat (2.2250738585072014e-308 :: Double))
  print (decodeFloat (1.7976931348623157e308 :: Double))
  print (decodeFloat (1 / 0 :: Double))
  print (decodeFloat (-1 / 0 :: Double))
  print (decodeFloat (0 / 0 :: Double))
  print (encodeFloat 0 1000 :: Double)
  print (encodeFloat 6755399441055744 (-52) :: Double)
  print (encodeFloat (-5629499534213120) (-51) :: Double)
  print (encodeFloat 1 (-1074) :: Double)
  print (encodeFloat 4503599627370496 (-1074) :: Double)
  print (encodeFloat 9007199254740991 971 :: Double)
  print (encodeFloat 9007199254740993 (-53) :: Double)
  print (encodeFloat 9223372036854775807 (-63) :: Double, encodeFloat (-9223372036854775807) (-63) :: Double)
  print (encodeFloat 1 1024 :: Double)
  print (encodeFloat 1 (-1075) :: Double)
  print (decodeManyChirho [1.25, -3.5])
  print (encodeManyChirho [(3, -1), (-5, -1)])
"#;

const REAL_FLOAT_CODEC_GHC_CHIRHO: &str = "(0,0)\n(0,0)\n(6755399441055744,-52)\n(-5629499534213120,-51)\n(4503599627370496,-1126)\n(4503599627370496,-1074)\n(9007199254740991,971)\n(4503599627370496,972)\n(-4503599627370496,972)\n(6755399441055744,972)\n0.0\n1.5\n-2.5\n5.0e-324\n2.2250738585072014e-308\n1.7976931348623157e308\n1.0\n(1.0,-1.0)\nInfinity\n0.0\n[(5629499534213120,-52),(-7881299347898368,-51)]\n[1.5,-2.5]\n";

const OWN_REAL_FLOAT_CODEC_CHIRHO: &str = r#"module Main where
import Prelude hiding (decodeFloat, encodeFloat)
decodeFloat :: Double -> (Integer, Int)
decodeFloat _ = (7, 3)
encodeFloat :: Integer -> Int -> Double
encodeFloat _ _ = 6.0
main :: IO ()
main = print (decodeFloat 3.5, encodeFloat 4 2)
"#;

const MINIMUM_WORD_LITERAL_CHIRHO: &str = r#"module Main where
literalIntegerChirho :: Integer
literalIntegerChirho = -9223372036854775808
literalIntChirho :: Int
literalIntChirho = -9223372036854775808
literalDoubleChirho :: Double
literalDoubleChirho = -9223372036854775808
computedMinChirho :: Integer
computedMinChirho = (-9223372036854775807) - 1
main :: IO ()
main = do
  print literalIntegerChirho
  print literalIntChirho
  print literalDoubleChirho
  print (-0x8000000000000000 :: Integer)
  print (-0o1000000000000000000000 :: Integer)
  print (-0b1000000000000000000000000000000000000000000000000000000000000000 :: Integer)
  print (encodeFloat literalIntegerChirho (-63) :: Double)
  print computedMinChirho
  print (toInteger (minBound :: Int))
"#;

const MINIMUM_WORD_LITERAL_GHC_CHIRHO: &str = "-9223372036854775808\n-9223372036854775808\n-9.223372036854776e18\n-9223372036854775808\n-9223372036854775808\n-9223372036854775808\n-1.0\n-9223372036854775808\n-9223372036854775808\n";

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
fn real_float_codec_answers_as_ghc_everywhere_chirho() {
    assert_everywhere_chirho(
        "RealFloatCodecChirho.hs",
        REAL_FLOAT_CODEC_CHIRHO,
        REAL_FLOAT_CODEC_GHC_CHIRHO,
    );
}

#[test]
fn a_programs_own_real_float_codec_stays_its_own_everywhere_chirho() {
    assert_everywhere_chirho(
        "OwnRealFloatCodecChirho.hs",
        OWN_REAL_FLOAT_CODEC_CHIRHO,
        "((7,3),6.0)\n",
    );
}

#[test]
fn the_minimum_word_literal_answers_as_ghc_everywhere_chirho() {
    assert_everywhere_chirho(
        "MinimumWordLiteralChirho.hs",
        MINIMUM_WORD_LITERAL_CHIRHO,
        MINIMUM_WORD_LITERAL_GHC_CHIRHO,
    );
}
