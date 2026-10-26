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

/// Row 571: the type environment knows a top-level value is `Double` even when
/// its Core binder still carries an inference variable. References to that value
/// now carry the module-level key into row 547's `negate#` retarget. Typed
/// parameters and locals, a negative binding, named `negate`, and Int are holds.
const TOP_LEVEL_DOUBLE_NEGATION_CHIRHO: &str = "module Main where\nxChirho :: Double\nxChirho = 1.5\ncomputedChirho :: Double\ncomputedChirho = 3 / 2\nalreadyNegatedChirho :: Double\nalreadyNegatedChirho = -1.5\nparameterNegatedChirho :: Double -> Double\nparameterNegatedChirho valueChirho = -valueChirho\nlocalNegatedChirho :: Double\nlocalNegatedChirho = let valueChirho = 1.5 in -valueChirho\nintegerChirho :: Int\nintegerChirho = 3\nmain :: IO ()\nmain = do\n  print (-xChirho)\n  print (-xChirho + 1)\n  print (parameterNegatedChirho xChirho)\n  print localNegatedChirho\n  print alreadyNegatedChirho\n  print (-computedChirho)\n  print (negate xChirho)\n  print (-integerChirho)\n";

/// GHC 9.14.1's output for [`TOP_LEVEL_DOUBLE_NEGATION_CHIRHO`].
const TOP_LEVEL_DOUBLE_NEGATION_GHC_CHIRHO: &str = "-1.5\n-0.5\n-1.5\n-1.5\n-1.5\n-1.5\n-1.5\n-3\n";

#[test]
fn a_negated_top_level_double_answers_as_ghc_everywhere_chirho() {
    assert_eq!(
        run_chirho(
            "TopLevelDoubleNegationChirho.hs",
            TOP_LEVEL_DOUBLE_NEGATION_CHIRHO,
        ),
        TOP_LEVEL_DOUBLE_NEGATION_GHC_CHIRHO,
        "interpreter"
    );
    assert_native_chirho(
        "TopLevelDoubleNegationChirho.hs",
        TOP_LEVEL_DOUBLE_NEGATION_CHIRHO,
        TOP_LEVEL_DOUBLE_NEGATION_GHC_CHIRHO,
    );
}

/// Row 553: under `map (+ 0)` the list's elements are rewritten at the exact key Double,
/// and that type-keyed rebuild kept the desugarer's Int `negate#`. Both native backends
/// printed `[-0.75,1.0]` and the interpreter stopped at `NegIntChirho`. The rebuild now
/// takes row 547's float counterpart; row 554 then moved the interpreter's remaining stop,
/// the section's `+#` (`AddIntChirho`), so all three engines answer. GHC: `[-1.5,1.0]`.
const TYPE_KEYED_NEGATION_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (map (+ 0) ([-3 / 2, 1] :: [Double]))\n";

#[test]
fn a_type_keyed_minus_takes_the_float_primop_chirho() {
    assert_eq!(
        run_chirho("TypeKeyedNegationChirho.hs", TYPE_KEYED_NEGATION_CHIRHO),
        "[-1.5,1.0]\n",
        "interpreter"
    );
    assert_native_chirho(
        "TypeKeyedNegationChirho.hs",
        TYPE_KEYED_NEGATION_CHIRHO,
        "[-1.5,1.0]\n",
    );
}

/// Row 554: sections of `+`, `-` and `*` become `\x -> +# x e` blind, so at Double the
/// interpreter refused them (`AddIntChirho`, `MulIntChirho`, `SubIntChirho`) and the native
/// backends added IEEE bits as integers (`NaN`, `[0.0,0.0]`, `1.348e308`). Where the section's
/// operand keys as Double, at either rebuild site, they now take `+.#`, `-.#` and `*.#`.
/// Right and left sections, direct and higher-order uses, a Double literal and a typed
/// integer literal; `/` sections, which take the Fractional evidence path, and Int
/// sections hold.
const NUMERIC_SECTIONS_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = do\n  print (map (+ 0.5) ([1.0, 2.0] :: [Double]))\n  print (map (* 2) ([1.5, -2.5] :: [Double]))\n  print (map (10 -) ([1.5, 2.5] :: [Double]))\n  print ((+ 1) (2.5 :: Double))\n  print (map (+ (1 :: Double)) [1.5, 2.5])\n  print (fmap (+ 1) (Just (1.5 :: Double)))\n  print (map (+ 0) ([-1.5, 1] :: [Double]))\n  print (map (/ 2) ([3.0, 5.0] :: [Double]))\n  print (map (2 /) ([4.0, 8.0] :: [Double]))\n  print ((/ 2) (3.0 :: Double))\n  print ((2 /) (4.0 :: Double))\n  print (map (* 3) [1, 2 :: Int])\n  print (map (10 -) [1, 2 :: Int])\n";

/// GHC 9.14.1's output for [`NUMERIC_SECTIONS_CHIRHO`].
const NUMERIC_SECTIONS_GHC_CHIRHO: &str = "[1.5,2.5]\n[3.0,-5.0]\n[8.5,7.5]\n3.5\n[2.5,3.5]\nJust 2.5\n[-1.5,1.0]\n[1.5,2.5]\n[0.5,0.25]\n1.5\n0.5\n[3,6]\n[9,8]\n";

#[test]
fn numeric_sections_answer_as_ghc_everywhere_chirho() {
    assert_eq!(
        run_chirho("NumericSectionsChirho.hs", NUMERIC_SECTIONS_CHIRHO),
        NUMERIC_SECTIONS_GHC_CHIRHO,
        "interpreter"
    );
    assert_native_chirho(
        "NumericSectionsChirho.hs",
        NUMERIC_SECTIONS_CHIRHO,
        NUMERIC_SECTIONS_GHC_CHIRHO,
    );
}

/// Row 554's named residual (#29012): under `iterate` the section's literal carries no
/// concrete key at either rebuild site, so `(* 2)` keeps `*#`. The interpreter still stops
/// at `MulIntChirho` (GHC: `[1.5,3.0,6.0,12.0]`); nothing is inferred from `iterate`.
#[test]
fn a_section_under_iterate_stays_the_named_residual_chirho() {
    let error_chirho = run_result_chirho(
        "IterateSectionChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (take 4 (iterate (* 2) (1.5 :: Double)))\n",
    )
    .expect_err("row 554 leaves the iterate section unchanged");
    assert!(error_chirho.contains("MulIntChirho"), "{error_chirho}");
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
