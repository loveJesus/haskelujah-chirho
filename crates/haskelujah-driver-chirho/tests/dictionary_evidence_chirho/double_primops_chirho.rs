// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Double operands at the desugarer's Int primops (row 547). Infix and section `<`,
//! `<=`, `>` and `>=`, and a prefix minus on an integer literal, were Int primops
//! whatever the type. At Double the interpreter refused them (`LtIntChirho: expected
//! Int#`), and both native backends compared IEEE bits as integers: 13 of the 21 lines
//! below printed the wrong Bool, negative operands and NaN among them. The dictionary
//! pass now gives an operand known at Double the float primop. The program's GHC
//! 9.14.1 output is recorded beside it.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_native_chirho, run_chirho};

/// Negative operands as literals, a binding and function parameters; NaN on each
/// side of all four operators; -0.0 against 0.0; both section forms; and Int and
/// Char ordering, which keep their Int primops.
const DOUBLE_ORDERING_CHIRHO: &str = "module Main where\nxChirho :: Double\nxChirho = -1.5\nlessChirho :: Double -> Double -> Bool\nlessChirho a b = a < b\nmain :: IO ()\nmain = do\n  print ((-1.5 :: Double) < (-2.5))\n  print ((-1 :: Double) < (-2))\n  print ((-2.5 :: Double) >= (-1.5))\n  print ((-2.5 :: Double) <= (-1.5))\n  print ((-1.5 :: Double) > (-2.5))\n  print (xChirho < (-2.5))\n  print (lessChirho (-1.5) (-2.5))\n  print ((0.0 / 0.0 :: Double) < 1.0)\n  print ((0.0 / 0.0 :: Double) <= 1.0)\n  print ((0.0 / 0.0 :: Double) > 1.0)\n  print ((0.0 / 0.0 :: Double) >= 1.0)\n  print (1.0 < (0.0 / 0.0 :: Double))\n  print (1.0 >= (0.0 / 0.0 :: Double))\n  print ((-0.0 :: Double) < 0.0)\n  print ((-0.0 :: Double) <= 0.0)\n  print ((-0.0 :: Double) >= 0.0)\n  print ((< (-1.0)) (-1.5 :: Double))\n  print (((-1.0) >) (-1.5 :: Double))\n  print ((-1 :: Int) < (-2))\n  print ((3 :: Int) >= 3)\n  print ('a' < 'b')\n";

/// GHC 9.14.1's output for [`DOUBLE_ORDERING_CHIRHO`].
const DOUBLE_ORDERING_GHC_CHIRHO: &str = "False\nFalse\nFalse\nTrue\nTrue\nFalse\nFalse\nFalse\nFalse\nFalse\nFalse\nFalse\nFalse\nFalse\nTrue\nTrue\nTrue\nTrue\nFalse\nTrue\nTrue\n";

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
