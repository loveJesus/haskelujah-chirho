// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Double operands at the desugarer's Int primops (row 547). Infix and section `<`,
//! `<=`, `>` and `>=`, and a prefix minus on an integer literal, were Int primops
//! whatever the type. At Double the interpreter refused them (`LtIntChirho: expected
//! Int#`), and both native backends compared IEEE bits as integers: 13 of the 21 lines
//! below printed the wrong Bool, negative operands and NaN among them. The dictionary
//! pass now gives an operand known at Double the float primop. A negated literal also
//! keys as its operand, so `(-1) / 2 :: Double` finds Fractional Double instead of a
//! missing `/`. The program's GHC 9.14.1 output is recorded beside it.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_native_chirho, run_chirho, run_result_chirho};

/// Negative operands as literals, a binding and function parameters; NaN on each
/// side of all four operators; -0.0 against 0.0; both section forms; and Int and
/// Char ordering, which keep their Int primops; and a negated literal divided at Double.
const DOUBLE_ORDERING_CHIRHO: &str = "module Main where\nxChirho :: Double\nxChirho = -1.5\nlessChirho :: Double -> Double -> Bool\nlessChirho a b = a < b\nmain :: IO ()\nmain = do\n  print ((-1.5 :: Double) < (-2.5))\n  print ((-1 :: Double) < (-2))\n  print ((-2.5 :: Double) >= (-1.5))\n  print ((-2.5 :: Double) <= (-1.5))\n  print ((-1.5 :: Double) > (-2.5))\n  print (xChirho < (-2.5))\n  print (lessChirho (-1.5) (-2.5))\n  print ((0.0 / 0.0 :: Double) < 1.0)\n  print ((0.0 / 0.0 :: Double) <= 1.0)\n  print ((0.0 / 0.0 :: Double) > 1.0)\n  print ((0.0 / 0.0 :: Double) >= 1.0)\n  print (1.0 < (0.0 / 0.0 :: Double))\n  print (1.0 >= (0.0 / 0.0 :: Double))\n  print ((-0.0 :: Double) < 0.0)\n  print ((-0.0 :: Double) <= 0.0)\n  print ((-0.0 :: Double) >= 0.0)\n  print ((< (-1.0)) (-1.5 :: Double))\n  print (((-1.0) >) (-1.5 :: Double))\n  print ((-1 :: Int) < (-2))\n  print ((3 :: Int) >= 3)\n  print ('a' < 'b')\n  print ((-1) / 2 :: Double)\n  print (compare ((-1) / 0 :: Double) 0)\n";

/// GHC 9.14.1's output for [`DOUBLE_ORDERING_CHIRHO`].
const DOUBLE_ORDERING_GHC_CHIRHO: &str = "False\nFalse\nFalse\nTrue\nTrue\nFalse\nFalse\nFalse\nFalse\nFalse\nFalse\nFalse\nFalse\nFalse\nTrue\nTrue\nTrue\nTrue\nFalse\nTrue\nTrue\n-0.5\nLT\n";

#[test]
fn double_ordering_answers_as_ghc_everywhere_chirho() {
    assert_eq!(
        run_chirho("DoubleOrderingChirho.hs", DOUBLE_ORDERING_CHIRHO),
        DOUBLE_ORDERING_GHC_CHIRHO,
        "interpreter"
    );
    assert_native_chirho(
        "DoubleOrderingChirho.hs",
        DOUBLE_ORDERING_CHIRHO,
        DOUBLE_ORDERING_GHC_CHIRHO,
    );
}

/// Row 553: under `map (+ 0)` the list's elements are rewritten at the exact key Double,
/// and that type-keyed rebuild kept the desugarer's Int `negate#`. Both native backends
/// printed `[-0.75,1.0]` and the interpreter stopped at `NegIntChirho`. The rebuild now
/// takes row 547's float counterpart; the interpreter still stops at the section's `+#`
/// (`AddIntChirho`), which row 554 owns. GHC: `[-1.5,1.0]`.
const TYPE_KEYED_NEGATION_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (map (+ 0) ([-3 / 2, 1] :: [Double]))\n";

#[test]
fn a_type_keyed_minus_takes_the_float_primop_chirho() {
    assert_native_chirho(
        "TypeKeyedNegationChirho.hs",
        TYPE_KEYED_NEGATION_CHIRHO,
        "[-1.5,1.0]\n",
    );
    let error_chirho = run_result_chirho("TypeKeyedNegationChirho.hs", TYPE_KEYED_NEGATION_CHIRHO)
        .expect_err("the interpreter holds at row 554's section boundary");
    assert!(
        error_chirho.contains("AddIntChirho") && !error_chirho.contains("NegIntChirho"),
        "{error_chirho}"
    );
}

/// The same negated division where it already worked: the list printed directly, its
/// head, and the division alone. GHC: `[-1.5,1.0]`, `-1.5` and `-1.5`.
#[test]
fn a_negated_division_holds_everywhere_chirho() {
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "NegatedListChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ([-3 / 2, 1] :: [Double])\n",
            "[-1.5,1.0]\n",
        ),
        (
            "NegatedHeadChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (head ([-3 / 2, 1] :: [Double]))\n",
            "-1.5\n",
        ),
        (
            "NegatedDivisionChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (-3 / 2 :: Double)\n",
            "-1.5\n",
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
