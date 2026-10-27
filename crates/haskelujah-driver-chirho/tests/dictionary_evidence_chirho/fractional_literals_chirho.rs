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

/// Row 599: a result-only `Num b` remains a real parameter when the binding body's
/// constrained reference carries the exact `$own:1` proof. The old result-only
/// default replaced that dictionary with Num Int: the interpreter stopped when the
/// result was shown as Double, and both natives silently reinterpreted Int words as
/// Double bits. Every line below is GHC 9.14.1's output for the source as written.
#[test]
fn a_result_only_num_dictionary_reaches_from_integral_everywhere_chirho() {
    let program_chirho = "module Main where\nconvertChirho :: (Integral a, Num b) => a -> b\nconvertChirho = fromIntegral\nmain :: IO ()\nmain = do\n  print (convertChirho (minBound :: Int) :: Integer)\n  print (convertChirho (42 :: Int) :: Double)\n  print (map convertChirho ([minBound, -1, 0, maxBound] :: [Int]) :: [Integer])\n  print (map convertChirho ([minBound, -1, 0, maxBound] :: [Int]) :: [Double])\n";
    let ghc_chirho = "-9223372036854775808\n42.0\n[-9223372036854775808,-1,0,9223372036854775807]\n[-9.223372036854776e18,-1.0,0.0,9.223372036854776e18]\n";
    assert_eq!(
        run_chirho("ResultOnlyNumChirho.hs", program_chirho),
        ghc_chirho,
        "interpreter"
    );
    assert_native_chirho("ResultOnlyNumChirho.hs", program_chirho, ghc_chirho);
}

/// Row 600: a locally generalized result literal is specialized at the enclosing
/// signature's rigid variable. That single instantiation proves the same `$own:0`
/// dictionary the outer binding already carries. Previously the literal had no
/// evidence, so the pass chose Num Int: the interpreter rejected `42#` as a Double,
/// and both natives silently printed the Int word as `2.08e-322`.
#[test]
fn a_local_result_literal_takes_the_enclosing_dictionary_everywhere_chirho() {
    let program_chirho = "module Main where\ntotalChirho :: Num a => [a] -> a\ntotalChirho xsChirho = sum xsChirho\n  where\n    sum _ = 42\nmain :: IO ()\nmain = print (totalChirho [1.5, 2.5 :: Double], totalChirho [1, 2 :: Int])\n";
    let ghc_chirho = "(42.0,42)\n";
    assert_eq!(
        run_chirho("LocalResultLiteralChirho.hs", program_chirho),
        ghc_chirho,
        "interpreter"
    );
    assert_native_chirho("LocalResultLiteralChirho.hs", program_chirho, ghc_chirho);
}

// Row 660: a literal's method is the class's, never a binding of the program's own.

// The generated `fromInteger`, `fromString` (OverloadedStrings) and `fromList` (OverloadedLists)

// used to resolve lexically, so a program that defined one, at top level or in a `where`,

// captured its literals: the interpreter hit its step limit and the natives ran to a timeout or

// died with a signal, or printed the wrong answer. Under RebindableSyntax the binding in scope

// is the literal's, as GHC defines. Every table is GHC 9.14.1's output, byte for byte as

// measured. workflow: compiler-pipeline-chirho/value-scope-resolution-chirho

const INTEGER_LITERAL_MOVERS_CHIRHO: [(&str, &str, &str); 6] = [
    (
        "LSignedFromIntegerChirho.hs",
        "module Main where\nimport Prelude hiding (fromInteger)\nfromInteger :: Integer -> Int\nfromInteger x = 42\nmain :: IO ()\nmain = print (fromInteger 5, 5 :: Int)\n",
        "(42,5)\n",
    ),
    (
        "LSignedIntegerChirho.hs",
        "module Main where\nimport Prelude hiding (fromInteger)\nimport qualified Prelude as P\nfromInteger :: Integer -> Integer\nfromInteger n = n P.* 10\nmain :: IO ()\nmain = print (fromInteger 4, 4 :: Integer)\n",
        "(40,4)\n",
    ),
    (
        "LLiteralsOnlyChirho.hs",
        "module Main where\nimport Prelude hiding (fromInteger)\nfromInteger :: Integer -> Int\nfromInteger _ = 7\nmain :: IO ()\nmain = print (1 + 2 :: Int)\n",
        "3\n",
    ),
    (
        "LWhereFromIntegerChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (x :: Int)\n  where\n    fromInteger _ = 9\n    x = 3\n",
        "3\n",
    ),
    (
        "LWhereExplicitChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (fromInteger 5 :: Int, 3 :: Int)\n  where\n    fromInteger _ = 9\n",
        "(9,3)\n",
    ),
    (
        "LNoImplicitPreludeChirho.hs",
        "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Main where\nimport Prelude (IO, Int, Integer, print)\nfromInteger :: Integer -> Int\nfromInteger _ = 7\nmain :: IO ()\nmain = print (21 :: Int)\n",
        "21\n",
    ),
];

const STRING_LITERAL_MOVERS_CHIRHO: [(&str, &str, &str); 5] = [
    (
        "LFromStringChirho.hs",
        "{-# LANGUAGE OverloadedStrings #-}\nmodule Main where\nfromString :: String -> Int\nfromString _ = 7\nmain :: IO ()\nmain = putStrLn \"literal\"\n",
        "literal\n",
    ),
    (
        "SStringExplicitChirho.hs",
        "{-# LANGUAGE OverloadedStrings #-}\nmodule Main where\nfromString :: String -> Int\nfromString _ = 7\nmain :: IO ()\nmain = do\n  print (fromString \"x\")\n  putStrLn \"literal\"\n",
        "7\nliteral\n",
    ),
    (
        "SStringWhereChirho.hs",
        "{-# LANGUAGE OverloadedStrings #-}\nmodule Main where\nmain :: IO ()\nmain = putStrLn s\n  where\n    fromString _ = \"own\"\n    s = \"literal\"\n",
        "literal\n",
    ),
    (
        "SStringNoImplicitPreludeChirho.hs",
        "{-# LANGUAGE NoImplicitPrelude, OverloadedStrings #-}\nmodule Main where\nimport Prelude (IO, Int, String, putStrLn)\nfromString :: String -> Int\nfromString _ = 7\nmain :: IO ()\nmain = putStrLn \"literal\"\n",
        "literal\n",
    ),
    (
        "SCustomIsStringChirho.hs",
        "{-# LANGUAGE OverloadedStrings #-}\nmodule Main where\nimport Data.String (IsString (..))\nnewtype Name = Name String deriving Show\ninstance IsString Name where\n  fromString s = Name (s ++ \"!\")\nmain :: IO ()\nmain = do\n  print (\"ab\" :: Name)\n  putStrLn \"plain\"\n",
        "Name \"ab!\"\nplain\n",
    ),
];

const LIST_LITERAL_MOVERS_CHIRHO: [(&str, &str, &str); 3] = [
    (
        "LFromListChirho.hs",
        "{-# LANGUAGE OverloadedLists #-}\nmodule Main where\nfromList :: [Int] -> [Int]\nfromList _ = []\nmain :: IO ()\nmain = print ([1, 2, 3] :: [Int])\n",
        "[1,2,3]\n",
    ),
    (
        "LListExplicitChirho.hs",
        "{-# LANGUAGE OverloadedLists #-}\nmodule Main where\nfromList :: [Int] -> [Int]\nfromList = reverse\nmain :: IO ()\nmain = do\n  print (fromList [1, 2, 3])\n  print ([4, 5, 6] :: [Int])\n",
        "[3,2,1]\n[4,5,6]\n",
    ),
    (
        "LListWhereChirho.hs",
        "{-# LANGUAGE OverloadedLists #-}\nmodule Main where\nmain :: IO ()\nmain = print xs\n  where\n    fromList _ = [9 :: Int]\n    xs = [1, 2] :: [Int]\n",
        "[1,2]\n",
    ),
];

const REBINDABLE_LITERAL_HOLDS_CHIRHO: [(&str, &str, &str); 3] = [
    (
        "LRebindableChirho.hs",
        "{-# LANGUAGE RebindableSyntax #-}\nmodule Main where\nimport Prelude hiding (fromInteger)\nimport qualified Prelude as P\nfromInteger :: Integer -> Int\nfromInteger n = P.fromInteger (n P.+ n)\nmain :: IO ()\nmain = print (21 :: Int)\n",
        "42\n",
    ),
    (
        "LFromStringRebindableChirho.hs",
        "{-# LANGUAGE RebindableSyntax, OverloadedStrings #-}\nmodule Main where\nimport Prelude\nfromString :: String -> String\nfromString s = s ++ ['!']\nmain :: IO ()\nmain = putStrLn \"literal\"\n",
        "literal!\n",
    ),
    (
        "LFromListRebindableChirho.hs",
        "{-# LANGUAGE RebindableSyntax, OverloadedLists #-}\nmodule Main where\nimport Prelude\nfromListN :: Int -> [Int] -> [Int]\nfromListN _ xs = reverse xs\nfromList :: [Int] -> [Int]\nfromList = reverse\nmain :: IO ()\nmain = print ([1, 2, 3] :: [Int])\n",
        "[3,2,1]\n",
    ),
];

const ORDINARY_LITERAL_HOLDS_CHIRHO: [(&str, &str, &str); 6] = [
    (
        "HLiteralsChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print (3 :: Int, 4 :: Integer, 2 :: Double, 7 / 2)\n  print (-5 :: Int, minBound :: Int, -9223372036854775808 :: Int)\n  print [1, 2, 3 :: Int]\n  print (sum [1 .. 3 :: Integer], product [4, 5 :: Integer])\n  print (map (\\x -> x + 1) [1.5, 2 :: Double])\n",
        "(3,4,2.0,3.5)\n(-5,-9223372036854775808,-9223372036854775808)\n[1,2,3]\n(6,20)\n[2.5,3.0]\n",
    ),
    (
        "HUserNumChirho.hs",
        "module Main where\nnewtype V = V Int deriving Show\ninstance Num V where\n  V a + V b = V (a + b)\n  V a * V b = V (a * b)\n  abs (V a) = V (abs a)\n  signum (V a) = V (signum a)\n  fromInteger n = V (fromInteger n * 10)\n  negate (V a) = V (negate a)\nmain :: IO ()\nmain = print (3 :: V, 2 + 3 :: V, [1, 2] :: [V])\n",
        "(V 30,V 50,[V 10,V 20])\n",
    ),
    (
        "LFromRationalChirho.hs",
        "module Main where\nimport Prelude hiding (fromRational)\nfromRational :: Rational -> Double\nfromRational _ = 0\nmain :: IO ()\nmain = print (fromRational 1, 2.5 :: Double)\n",
        "(0.0,2.5)\n",
    ),
    (
        "LPrefixNegateChirho.hs",
        "module Main where\nimport Prelude hiding (negate)\nnegate :: Int -> Int\nnegate _ = 99\nmain :: IO ()\nmain = print (-5 :: Int, negate 1)\n",
        "(-5,99)\n",
    ),
    (
        "SStringHoldChirho.hs",
        "{-# LANGUAGE OverloadedStrings #-}\nmodule Main where\nmain :: IO ()\nmain = do\n  putStrLn \"plain\"\n  print (\"a\" ++ \"b\" :: String)\n  print (words \"x y\")\n",
        "plain\n\"ab\"\n[\"x\",\"y\"]\n",
    ),
    (
        "LListHoldChirho.hs",
        "{-# LANGUAGE OverloadedLists #-}\nmodule Main where\nmain :: IO ()\nmain = do\n  print ([1, 2, 3] :: [Int])\n  print (sum ([1, 2, 3] :: [Int]))\n  print (map (* 2) [1, 2 :: Int])\n",
        "[1,2,3]\n6\n[2,4]\n",
    ),
];

/// Each program prints GHC's output on the interpreter, LLVM and Cranelift. A native run is
/// bounded by the harness's deadline, so a literal that recursed fails the test, never hangs it.
fn assert_everywhere_chirho(table_chirho: &[(&str, &str, &str)]) {
    for (file_name_chirho, program_chirho, ghc_chirho) in table_chirho {
        assert_eq!(
            run_chirho(file_name_chirho, program_chirho),
            *ghc_chirho,
            "{file_name_chirho} interpreter"
        );
        assert_native_chirho(file_name_chirho, program_chirho, ghc_chirho);
    }
}

/// An integer literal takes Num's `fromInteger` past a program's own, signed or not, top-level
/// or `where`-bound, and under NoImplicitPrelude; a written call keeps the program's.
#[test]
fn integer_literals_never_take_a_programs_from_integer_everywhere_chirho() {
    assert_everywhere_chirho(&INTEGER_LITERAL_MOVERS_CHIRHO);
}

/// A string literal under OverloadedStrings takes IsString's `fromString`, including a custom
/// instance's, through its own occurrence's proof.
#[test]
fn string_literals_never_take_a_programs_from_string_everywhere_chirho() {
    assert_everywhere_chirho(&STRING_LITERAL_MOVERS_CHIRHO);
}

/// A list literal under OverloadedLists takes IsList's `fromList` at the list type. A custom
/// IsList instance is not claimed here: the checker types the literal as a list (row 667).
#[test]
fn list_literals_never_take_a_programs_from_list_everywhere_chirho() {
    assert_everywhere_chirho(&LIST_LITERAL_MOVERS_CHIRHO);
}

/// Under RebindableSyntax each literal takes the binding in scope.
#[test]
fn rebindable_syntax_literals_take_the_binding_in_scope_everywhere_chirho() {
    assert_everywhere_chirho(&REBINDABLE_LITERAL_HOLDS_CHIRHO);
}

/// Literals of programs that bind none of these methods answer as before: Int, Integer, Double
/// and defaulted Fractional, negatives and `minBound`, a user `Num` instance, a list, a range
/// under `sum`, strings and lists under their extensions, and a program's own `fromRational`
/// and `negate`, which no literal names.
#[test]
fn ordinary_literals_hold_everywhere_chirho() {
    assert_everywhere_chirho(&ORDINARY_LITERAL_HOLDS_CHIRHO);
}
