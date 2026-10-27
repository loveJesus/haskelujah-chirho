// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Integer literals divided at Double (row 543). An integer literal desugars to
//! `fromInteger n`, and the dictionary pass keyed that application `Int` by name. So a
//! division whose first argument was such a literal found no `Fractional Int` row and
//! left `/` unbound: `print (7 / 2)` failed with `missing STG binding /` in the
//! interpreter and `missing native binding /` on both native backends. The key is now
//! the type the checker solved for that literal occurrence, and `Int` only without one.
//! A literal at a signature's rigid variable takes the binding's own dictionary
//! (row 584). Each program's GHC 9.14.1 output is recorded beside it.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_native_chirho, run_chirho};

#[test]
fn integer_literals_divide_at_double_everywhere_chirho() {
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "D02LitHalfChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (1 / 2 :: Double)\n",
            "0.5\n",
        ),
        (
            "D05TopLevelChirho.hs",
            "module Main where\nxChirho :: Double\nxChirho = 1 / 2\nmain :: IO ()\nmain = print xChirho\n",
            "0.5\n",
        ),
        (
            "D06DefaultedChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (7 / 2)\n",
            "3.5\n",
        ),
        // With row 539's native compareFloat# and row 542's interpreter NaN ordering.
        (
            "D04CompareLitChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare (0 / 0 :: Double) 1)\n",
            "GT\n",
        ),
    ] {
        assert_eq!(
            run_chirho(file_name_chirho, program_chirho),
            ghc_chirho,
            "{file_name_chirho} interpreter"
        );
        assert_native_chirho(file_name_chirho, program_chirho, ghc_chirho);
    }
}

#[test]
fn integer_literal_non_finite_values_render_in_the_interpreter_chirho() {
    for (file_name_chirho, expression_chirho, ghc_chirho) in [
        ("D01NaNChirho.hs", "0 / 0", "NaN\n"),
        ("D01InfinityChirho.hs", "1 / 0", "Infinity\n"),
        (
            "D01NegativeInfinityChirho.hs",
            "negate (1 / 0 :: Double)",
            "-Infinity\n",
        ),
    ] {
        let program_chirho = format!(
            "module Main where\nmain :: IO ()\nmain = print ({expression_chirho} :: Double)\n"
        );
        assert_eq!(
            run_chirho(file_name_chirho, &program_chirho),
            ghc_chirho,
            "{file_name_chirho} interpreter"
        );
        if expression_chirho == "0 / 0" {
            assert_native_chirho(file_name_chirho, &program_chirho, ghc_chirho);
        }
    }
}

/// GHC 9.14.1 resolves exact half ties to the even integer. Rust's plain
/// `f64::round` instead resolves them away from zero, which made `round#`
/// silently disagree in the interpreter (row 559).
#[test]
fn double_round_uses_ghcs_ties_to_even_rule_in_the_interpreter_chirho() {
    let program_chirho = "module Main where\nmain :: IO ()\nmain = print (map round ([0.5,1.5,2.5,3.5,-0.5,-1.5,-2.5,-3.5,0.49,0.51,-0.49,-0.51,0.0,-0.0,2.0,-2.0] :: [Double]) :: [Int])\n";
    assert_eq!(
        run_chirho("DoubleRoundTiesEvenChirho.hs", program_chirho),
        "[0,2,2,4,0,-2,-2,-4,0,1,0,-1,0,0,2,-2]\n"
    );
}

/// Row 562 routes every native `Double -> Int` primitive through the shared
/// RTS conversion. Each list is GHC 9.14.1's output for the source as written.
#[test]
fn double_to_int_directions_match_ghc_natively_chirho() {
    let program_chirho = "module Main where\nmain :: IO ()\nmain = do\n  print (map round ([0.5,1.5,2.5,3.5,-0.5,-1.5,-2.5,-3.5,0.49,0.51,-0.49,-0.51,0.0,-0.0,2.0,-2.0] :: [Double]) :: [Int])\n  print (map truncate ([-2.7,-2.0,-0.5,0.0,0.5,2.0,2.7] :: [Double]) :: [Int])\n  print (map floor ([-2.7,-2.0,-0.5,0.0,0.5,2.0,2.7] :: [Double]) :: [Int])\n  print (map ceiling ([-2.7,-2.0,-0.5,0.0,0.5,2.0,2.7] :: [Double]) :: [Int])\n";
    let ghc_chirho = "[0,2,2,4,0,-2,-2,-4,0,1,0,-1,0,0,2,-2]\n[-2,-2,0,0,0,2,2]\n[-3,-2,-1,0,0,2,2]\n[-2,-2,0,0,1,2,3]\n";
    assert_native_chirho("DoubleToIntDirectionsChirho.hs", program_chirho, ghc_chirho);
}

/// GHC converts non-finite values to zero and finite values modulo the `Int`
/// word size. Rows 589 and 591 make the interpreter share the same four RTS
/// conversions that row 562 established for both native backends.
#[test]
fn exceptional_double_to_int_inputs_match_ghc_everywhere_chirho() {
    let program_chirho = "module Main where\nmain :: IO ()\nmain = do\n  print (map round ([0 / 0, 1 / 0, (-1) / 0, 1.0e20, -1.0e20] :: [Double]) :: [Int])\n  print (map truncate ([0 / 0, 1 / 0, (-1) / 0, 1.0e20, -1.0e20] :: [Double]) :: [Int])\n  print (map floor ([0 / 0, 1 / 0, (-1) / 0, 1.0e20, -1.0e20] :: [Double]) :: [Int])\n  print (map ceiling ([0 / 0, 1 / 0, (-1) / 0, 1.0e20, -1.0e20] :: [Double]) :: [Int])\n";
    let ghc_line_chirho = "[0,0,0,7766279631452241920,-7766279631452241920]\n";
    assert_native_chirho(
        "ExceptionalDoubleToIntChirho.hs",
        program_chirho,
        &ghc_line_chirho.repeat(4),
    );
    assert_eq!(
        run_chirho("ExceptionalDoubleToIntChirho.hs", program_chirho),
        ghc_line_chirho.repeat(4)
    );
}

/// GHC 9.14.1 changes from fixed to scientific notation below exponent -1 and above
/// exponent 6. Native whole-Double formatting remains a separate residual.
#[test]
fn finite_double_show_uses_ghcs_exponent_boundaries_in_the_interpreter_chirho() {
    let program_chirho = "module Main where\nmain :: IO ()\nmain = do\n  print (9999999.0 :: Double)\n  print (10000000.0 :: Double)\n  print (0.1 :: Double)\n  print (0.01 :: Double)\n  print (1.0e20 :: Double)\n  print (1.0e-20 :: Double)\n";
    assert_eq!(
        run_chirho("FiniteDoubleShowChirho.hs", program_chirho),
        "9999999.0\n1.0e7\n0.1\n1.0e-2\n1.0e20\n1.0e-20\n"
    );
}

/// A literal the checker solved at Int still dispatches at Int.
#[test]
fn integer_literals_at_int_still_dispatch_at_int_chirho() {
    let program_chirho = "module Main where\nmain :: IO ()\nmain = do\n  print (7 `div` 2 :: Int)\n  print (2 ^ (10 :: Int) :: Int)\n";
    assert_eq!(
        run_chirho("IntLiteralsChirho.hs", program_chirho),
        "3\n1024\n"
    );
    assert_native_chirho("IntLiteralsChirho.hs", program_chirho, "3\n1024\n");
}

/// `(^^) :: (Fractional a, Integral b) => a -> b -> a` needs both predicate
/// proofs. GHC 9.14.1's output for this source is recorded exactly; the precision
/// line also proves that the implementation reuses `(^)` rather than libm `pow`.
#[test]
fn fractional_power_answers_as_ghc_everywhere_chirho() {
    let program_chirho = "module Main where\nbaseChirho :: Double\nbaseChirho = 1.5\nexponentChirho :: Integer\nexponentChirho = -2\nmain :: IO ()\nmain = do\n  print (2 ^^ 3 :: Double)\n  print (2 ^^ (-3) :: Double)\n  print (0 ^^ 0 :: Double)\n  print (0 ^^ (-1) :: Double)\n  print ((-2) ^^ (-3) :: Double)\n  print (1.1 ^^ 50 :: Double)\n  print (map (2 ^^) ([-2, 0, 3] :: [Int]) :: [Double])\n  print (map (^^ 2) ([1.5, 2.5] :: [Double]))\n  print (baseChirho ^^ exponentChirho)\n";
    let ghc_chirho = "8.0\n0.125\n1.0\nInfinity\n-0.125\n117.39085287969571\n[0.25,1.0,8.0]\n[2.25,6.25]\n0.4444444444444444\n";
    assert_eq!(
        run_chirho("FractionalPowerChirho.hs", program_chirho),
        ghc_chirho,
        "FractionalPowerChirho.hs interpreter"
    );
    assert_native_chirho("FractionalPowerChirho.hs", program_chirho, ghc_chirho);
}

/// Row 584: a literal at a signature's rigid variable. The checker published no key for
/// it, so the pass defaulted it to Int, and the natives read the Int's bits as a Double:
/// `incChirho (2.5 :: Double)` printed 2.5 and `twiceChirho 1.25` printed 1.0e-323.
/// Such a literal now takes the binding's own dictionary, as `+` beside it does. With
/// two predicates of one class, each literal takes its own variable's parameter.
#[test]
fn literals_at_a_signature_variable_take_its_dictionary_everywhere_chirho() {
    let program_chirho = "module Main where\nincChirho :: Num a => a -> a\nincChirho x = x + 1\ntwiceChirho :: Num a => a -> a\ntwiceChirho x = 2 * x\npairChirho :: (Num a, Num b) => a -> b -> (a, b)\npairChirho x y = (x + 1, y * 3)\nmain :: IO ()\nmain = do\n  print (incChirho (2.5 :: Double), incChirho (4 :: Int))\n  print (twiceChirho (1.25 :: Double), twiceChirho (7 :: Int))\n  print (pairChirho (0.5 :: Double) (5 :: Int), pairChirho (2 :: Int) (1.5 :: Double))\n";
    let ghc_chirho = "(3.5,5)\n(2.5,14)\n((1.5,15),(3,4.5))\n";
    assert_eq!(
        run_chirho("SignatureLiteralsChirho.hs", program_chirho),
        ghc_chirho,
        "interpreter"
    );
    assert_native_chirho("SignatureLiteralsChirho.hs", program_chirho, ghc_chirho);
}

/// Row 584: under `Fractional a` no predicate names `Num`, so the literal takes the
/// class-only own key, and the pass extracts the Num dictionary from `$dFractional`. On
/// main the literal took `$fNumInt`. A returned literal shows it: the interpreter then
/// stopped at `ShowFloatChirho: expected Double#, got 2#`. The natives refused the
/// Fractional Double dictionary's `recip#` until row 583.
#[test]
fn a_fractional_signatures_literal_takes_its_dictionary_chirho() {
    let program_chirho = "module Main where\nhalfChirho :: Fractional a => a -> a\nhalfChirho r = r / 2\ntwoChirho :: Fractional a => a -> a\ntwoChirho _ = 2\nmain :: IO ()\nmain = do\n  print (halfChirho (3 :: Double), halfChirho (-0.5 :: Double))\n  print (twoChirho (1 :: Double), twoChirho 0.5 + 1 :: Double)\n";
    let ghc_chirho = "(1.5,-0.25)\n(2.0,3.0)\n";
    assert_eq!(
        run_chirho("FractionalSignatureLiteralChirho.hs", program_chirho),
        ghc_chirho,
        "interpreter"
    );
    assert_native_chirho(
        "FractionalSignatureLiteralChirho.hs",
        program_chirho,
        ghc_chirho,
    );
}

/// Row 596: `fromIntegral` is the Report composition `fromInteger . toInteger`, with
/// the source `Integral` and result `Num` dictionaries selected independently from the
/// checker's proof. The former monomorphic `Int -> Double` binding rounded the minimum
/// word even when the result was `Integer`; both natives also exposed its Double bits as
/// an integer. Every line below is GHC 9.14.1's output for the source as written.
#[test]
fn integral_conversions_use_both_proven_dictionaries_everywhere_chirho() {
    let program_chirho = "module Main where\nconvertIntegerChirho :: Int -> Integer\nconvertIntegerChirho = fromIntegral\nconvertDoubleChirho :: Int -> Double\nconvertDoubleChirho = fromIntegral\nmain :: IO ()\nmain = do\n  print (fromIntegral (minBound :: Int) :: Integer)\n  print (encodeFloat (fromIntegral (minBound :: Int) :: Integer) (-63) :: Double)\n  print (fromIntegral (minBound :: Int) :: Double)\n  print (map fromIntegral ([minBound, -1, 0, maxBound] :: [Int]) :: [Integer])\n  print (map fromIntegral ([minBound, -1, 0, maxBound] :: [Int]) :: [Double])\n  print (convertIntegerChirho minBound, convertDoubleChirho 42)\n";
    let ghc_chirho = "-9223372036854775808\n-1.0\n-9.223372036854776e18\n[-9223372036854775808,-1,0,9223372036854775807]\n[-9.223372036854776e18,-1.0,0.0,9.223372036854776e18]\n(-9223372036854775808,42.0)\n";
    assert_eq!(
        run_chirho("IntegralConversionsChirho.hs", program_chirho),
        ghc_chirho,
        "interpreter"
    );
    assert_native_chirho("IntegralConversionsChirho.hs", program_chirho, ghc_chirho);
}
