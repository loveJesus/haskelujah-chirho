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

use super::{
    assert_named_failure_everywhere_chirho, assert_native_chirho, run_chirho, run_result_chirho,
};

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

/// Row 570: `^` at Double. A float-literal base took `**#`, whose `pow` is not GHC's
/// answer: `1.1 ^ 50` was 117.39085287969579 in the interpreter, and both natives refused
/// `**#`. Any other base took the Int `^#`: the interpreter stopped at `PowIntChirho`, and
/// both natives printed 0.0. Both routes now take `^.#`, GHC.Real's repeated squaring
/// from the RTS. The base can be a literal, an integer literal, a typed parameter, a
/// section, a printed `map`'s lambda or a negated power, with an `Integer` exponent or an
/// Int expression as the exponent. A top-level Double base, which row 571 keys, takes it
/// too, negated included. Int powers hold.
const DOUBLE_POWER_CHIRHO: &str = "module Main where\nkChirho :: Int\nkChirho = 2\nxChirho :: Double\nxChirho = 1.5\nsquareChirho :: Double -> Double\nsquareChirho v = v ^ 2\nmain :: IO ()\nmain = do\n  print (3 ^ 2 :: Double, 2.5 ^ 3 :: Double)\n  print (2.5 ^ 0 :: Double, 2.5 ^ 1 :: Double)\n  print (0 ^ 0 :: Double, (0 / 0) ^ 0 :: Double)\n  print (1.1 ^ 50 :: Double, 1.0000001 ^ 1000003 :: Double)\n  print (2 ^ 60 :: Double, 0.9 ^ 100 :: Double)\n  print ((-2) ^ 3 :: Double, (-0.5) ^ 4 :: Double)\n  print ((0 / 0) ^ 2 :: Double, (1 / 0) ^ 3 :: Double)\n  print ((-1 / 0) ^ 3 :: Double, (-0.0) ^ 3 :: Double)\n  print (2.5 ^ (3 :: Integer) :: Double, -3 ^ 2 :: Double)\n  print (squareChirho 1.5, map (\\v -> v ^ 3) [0.5, 2.0 :: Double])\n  print (map (^ 2) [1.5, 2.5 :: Double])\n  print (map (\\v -> v ^ (kChirho * 2)) [1.5, 2.0 :: Double])\n  print (Just (1.5 ^ 2 :: Double), [0.1 ^ 2 :: Double])\n  print (xChirho ^ 10, xChirho ^ (3 :: Integer))\n  print (-xChirho ^ 2, xChirho ^ (0 :: Integer))\n  print (2 ^ 10 :: Int, (-3) ^ 3 :: Int)\n";

/// GHC 9.14.1's output for [`DOUBLE_POWER_CHIRHO`].
const DOUBLE_POWER_GHC_CHIRHO: &str = "(9.0,15.625)\n(1.0,2.5)\n(1.0,1.0)\n(117.39085287969571,1.1051712441594597)\n(1.152921504606847e18,2.6561398887587605e-5)\n(-8.0,6.25e-2)\n(NaN,Infinity)\n(-Infinity,-0.0)\n(15.625,-9.0)\n(2.25,[0.125,8.0])\n[2.25,6.25]\n[5.0625,16.0]\n(Just 2.25,[1.0000000000000002e-2])\n(57.6650390625,3.375)\n(-2.25,1.0)\n(1024,-27)\n";

#[test]
fn double_power_answers_as_ghc_everywhere_chirho() {
    assert_eq!(
        run_chirho("DoublePowerChirho.hs", DOUBLE_POWER_CHIRHO),
        DOUBLE_POWER_GHC_CHIRHO,
        "interpreter"
    );
    assert_native_chirho(
        "DoublePowerChirho.hs",
        DOUBLE_POWER_CHIRHO,
        DOUBLE_POWER_GHC_CHIRHO,
    );
}

/// GHC.Real's `(^)` rejects a negative exponent with `Negative exponent`. At Double the
/// interpreter's `pow` printed 0.4, and both natives refused `**#`.
#[test]
fn a_negative_double_exponent_fails_named_everywhere_chirho() {
    assert_named_failure_everywhere_chirho(
        "NegativeExponentChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (2.5 ^ (-1 :: Int) :: Double)\n",
        "Negative exponent",
    );
}

/// Row 570's named residuals, where nothing keys the base as Double. GHC prints 5.0 and
/// `[8.0,16.0]`.
/// - A lambda under an aggregate: `sum (map (\v -> v ^ 2) xs)`.
/// - A left section whose list holds the Int exponents: `map (2 ^) [3, 4 :: Int]`.
///
/// The base is a Double at run time, but the power stays the Int `^#`. The interpreter
/// stops at `PowIntChirho`.
#[test]
fn an_unkeyed_double_base_stays_the_named_residual_chirho() {
    for (file_name_chirho, program_chirho) in [
        (
            "SumOfPowersChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (sum (map (\\v -> v ^ 2) [1.0, 2.0 :: Double]))\n",
        ),
        (
            "LeftPowerSectionChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (2 ^) [3, 4 :: Int] :: [Double])\n",
        ),
    ] {
        let error_chirho = run_result_chirho(file_name_chirho, program_chirho)
            .expect_err("row 570 leaves an unkeyed base on the Int power");
        assert!(
            error_chirho.contains("PowIntChirho"),
            "{file_name_chirho}: {error_chirho}"
        );
    }
}

/// Row 573: Double `**`, which is a different operation from row 570's `^`. GHC's `**##`
/// is the C library's `pow`. The interpreter already answered with `powf`, but both native
/// backends refused `**#`, so every program here failed to compile natively. They now call
/// the RTS's one `pow`.
/// - Fractional, integral and negative exponents.
/// - libm's edge cases: `0 ** 0`, NaN on either side, overflow and underflow, and a
///   negative base to a fractional power.
/// - Top-level, section, function and aggregate bases.
const FLOAT_POWER_CHIRHO: &str = "module Main where\nxChirho :: Double\nxChirho = 2.5\nrootChirho :: Double -> Double\nrootChirho v = v ** 0.5\nmain :: IO ()\nmain = do\n  print (2 ** 0.5 :: Double, 2 ** 3 :: Double)\n  print (10 ** (-2) :: Double, 0.5 ** 0.5 :: Double)\n  print ((-8) ** (1 / 3) :: Double, 0 ** 0 :: Double)\n  print (1e308 ** 2 :: Double, 0 ** (-1) :: Double)\n  print ((0 / 0) ** 0 :: Double, 1 ** (0 / 0) :: Double)\n  print (2 ** 1024 :: Double, 2 ** (-1075) :: Double)\n  print ((-2) ** 3 :: Double, (-2) ** 0.5 :: Double)\n  print (xChirho ** 1.5, xChirho ** 2)\n  print (map (** 2) [1.5, 2.5 :: Double], map (2 **) [0.5, 1.5 :: Double])\n  print (rootChirho 2, map rootChirho [4, 9])\n  print (sum (map (** 2) [1.0, 2.0 :: Double]), (1.1 ** 50) / (1.1 ^ 50) :: Double)\n";

/// GHC 9.14.1's output for [`FLOAT_POWER_CHIRHO`].
const FLOAT_POWER_GHC_CHIRHO: &str = "(1.4142135623730951,8.0)\n(1.0e-2,0.7071067811865476)\n(NaN,1.0)\n(Infinity,Infinity)\n(1.0,1.0)\n(Infinity,0.0)\n(-8.0,NaN)\n(3.952847075210474,6.25)\n([2.25,6.25],[1.4142135623730951,2.8284271247461903])\n(1.4142135623730951,[2.0,3.0])\n(5.0,1.0000000000000007)\n";

#[test]
fn double_float_power_answers_as_ghc_everywhere_chirho() {
    assert_eq!(
        run_chirho("FloatPowerChirho.hs", FLOAT_POWER_CHIRHO),
        FLOAT_POWER_GHC_CHIRHO,
        "interpreter"
    );
    assert_native_chirho(
        "FloatPowerChirho.hs",
        FLOAT_POWER_CHIRHO,
        FLOAT_POWER_GHC_CHIRHO,
    );
}

/// Row 575: the unary Floating primops at Double. GHC's are libm calls. The interpreter
/// already answered, but both native backends refused `sqrt#`, `exp#`, `log#`, `sin#`,
/// `cos#`, `tan#`, `asin#`, `acos#` and `atan#`, so every line here failed to compile
/// natively. Each now calls the RTS's libm symbol for it. The program covers NaN and
/// infinite results, a top-level base, a section and `map`. `pi` stays a missing binding,
/// so none is used.
const FLOATING_PRIMOPS_CHIRHO: &str = "module Main where\nxChirho :: Double\nxChirho = 2.25\nmain :: IO ()\nmain = do\n  print (sqrt 2 :: Double, sqrt (-1) :: Double)\n  print (exp 1 :: Double, exp 710 :: Double)\n  print (log 10 :: Double, log 0 :: Double)\n  print (sin 1 :: Double, cos 1 :: Double)\n  print (tan 1 :: Double, tan 0.5 :: Double)\n  print (asin 0.5 :: Double, asin 2 :: Double)\n  print (acos 0.5 :: Double, acos (-1) :: Double)\n  print (atan 1 :: Double, atan 1e300 :: Double)\n  print (sqrt xChirho, exp (log xChirho))\n  print (map sqrt [4, 9 :: Double], map (\\v -> sin v * sin v + cos v * cos v) [0.5 :: Double])\n";

/// GHC 9.14.1's output for [`FLOATING_PRIMOPS_CHIRHO`].
const FLOATING_PRIMOPS_GHC_CHIRHO: &str = "(1.4142135623730951,NaN)\n(2.718281828459045,Infinity)\n(2.302585092994046,-Infinity)\n(0.8414709848078965,0.5403023058681398)\n(1.557407724654902,0.5463024898437905)\n(0.5235987755982988,NaN)\n(1.0471975511965976,3.141592653589793)\n(0.7853981633974483,1.5707963267948966)\n(1.5,2.25)\n([2.0,3.0],[1.0])\n";

#[test]
fn unary_floating_primops_answer_as_ghc_everywhere_chirho() {
    assert_eq!(
        run_chirho("FloatingPrimopsChirho.hs", FLOATING_PRIMOPS_CHIRHO),
        FLOATING_PRIMOPS_GHC_CHIRHO,
        "interpreter"
    );
    assert_native_chirho(
        "FloatingPrimopsChirho.hs",
        FLOATING_PRIMOPS_CHIRHO,
        FLOATING_PRIMOPS_GHC_CHIRHO,
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
