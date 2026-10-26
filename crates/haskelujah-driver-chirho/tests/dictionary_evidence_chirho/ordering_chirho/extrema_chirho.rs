// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `max`, `min`, `maximum` and `minimum` through a body-backed `compare` (row 565). Their
//! generic bindings are Int-shaped, comparing with `<=#` and `>=#`. Outside Int, Char and
//! Bool the interpreter refused them (`LeIntChirho: expected Int#`), and both native
//! backends compared heap values as integers: `minimum ["pear", "apple", "zoo"]` printed
//! "pear", `maximum [Green, Red, Blue]` printed Green, and `maximum [1, 0 / 0]` printed NaN.
//! An evidenced occurrence, applied, first-class or partially applied, now reads its key's
//! own compare row. Each program's GHC 9.14.1 output is recorded beside it.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_native_chirho, run_chirho, run_result_chirho};

/// Doubles, NaN on each side included; Strings; lists of Int; a derived enum; and
/// Ordering. They run through `maximum`, `minimum`, `max` and `min`, first-class in
/// `zipWith`, `map` and both folds, and partially applied.
const EXTREMA_CHIRHO: &str = "module Main where\ndata ColorChirho = RedChirho | GreenChirho | BlueChirho deriving (Show, Eq, Ord)\nmain :: IO ()\nmain = do\n  print (maximum [1.5, -2.5, 0.5 :: Double], minimum [1.5, -2.5, 0.5 :: Double])\n  print (maximum [0 / 0, 1 :: Double], maximum [1, 0 / 0 :: Double], minimum [0 / 0, 1 :: Double])\n  print (maximum [\"pear\", \"apple\", \"zoo\"], minimum [\"pear\", \"apple\", \"zoo\"])\n  print (maximum [[1, 2], [3], [1, 2, 3 :: Int]], minimum [[1, 2], [3], [1, 2, 3 :: Int]])\n  print (maximum [GreenChirho, RedChirho, BlueChirho], minimum [GreenChirho, BlueChirho])\n  print (maximum [LT, GT, EQ], minimum [LT, GT, EQ])\n  print (max \"ab\" \"b\", min [2, 1] [1, 2 :: Int])\n  print (max 1.5 (2.5 :: Double), max GreenChirho BlueChirho)\n  print (zipWith max [\"a\", \"c\"] [\"b\", \"a\"], map minimum [[2.5, -1.5], [0.5 :: Double]])\n  print (foldl max \"\" [\"b\", \"c\", \"a\"], foldr min \"z\" [\"b\", \"c\", \"a\"])\n  let pickChirho = max \"m\"\n  print (pickChirho \"z\", pickChirho \"a\")\n";

/// GHC 9.14.1's output for [`EXTREMA_CHIRHO`].
const EXTREMA_GHC_CHIRHO: &str = "(1.5,-2.5)\n(NaN,1.0,1.0)\n(\"zoo\",\"apple\")\n([3],[1,2])\n(BlueChirho,GreenChirho)\n(GT,LT)\n(\"b\",[1,2])\n(2.5,BlueChirho)\n([\"b\",\"c\"],[-1.5,0.5])\n(\"c\",\"a\")\n(\"z\",\"m\")\n";

#[test]
fn extrema_read_their_key_compare_everywhere_chirho() {
    assert_eq!(
        run_chirho("ExtremaChirho.hs", EXTREMA_CHIRHO),
        EXTREMA_GHC_CHIRHO,
        "interpreter"
    );
    assert_native_chirho("ExtremaChirho.hs", EXTREMA_CHIRHO, EXTREMA_GHC_CHIRHO);
}

/// Int, Char and Bool keep their Int-shaped bindings, which were already exact.
const INT_SHAPED_EXTREMA_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = do\n  print (maximum [3, 1, 2 :: Int], minimum [3, 1, 2 :: Int])\n  print (minimum \"hello\", maximum \"hello\")\n  print (maximum [True, False], minimum [True, False])\n  print (maximum (map negate [1, 2, 3 :: Int]), max 'a' 'b', min (2 :: Int) 1)\n";

/// GHC 9.14.1's output for [`INT_SHAPED_EXTREMA_CHIRHO`].
const INT_SHAPED_EXTREMA_GHC_CHIRHO: &str = "(3,1)\n('e','o')\n(True,False)\n(-1,'b',1)\n";

#[test]
fn int_shaped_extrema_keep_their_answers_everywhere_chirho() {
    assert_eq!(
        run_chirho("IntShapedExtremaChirho.hs", INT_SHAPED_EXTREMA_CHIRHO),
        INT_SHAPED_EXTREMA_GHC_CHIRHO,
        "interpreter"
    );
    assert_native_chirho(
        "IntShapedExtremaChirho.hs",
        INT_SHAPED_EXTREMA_CHIRHO,
        INT_SHAPED_EXTREMA_GHC_CHIRHO,
    );
}

/// Row 565's named residuals, each failing in the interpreter as before. GHC answers
/// `Just 3`, `(2,'a')` and `"c"`, and fails `Prelude.maximum: empty list`.
/// - Maybe and tuple keys have no body-backed compare row, so they wait for a later Ord brick.
/// - A polymorphic `Ord a =>` wrapper holds a dictionary, not a key.
/// - The empty list fails in `head`, not with GHC's message.
#[test]
fn extrema_outside_the_compare_rows_stay_named_residuals_chirho() {
    for (file_name_chirho, program_chirho, reason_chirho) in [
        (
            "MaximumMaybeChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (maximum [Just 3, Nothing, Just (1 :: Int)])\n",
            "LeIntChirho",
        ),
        (
            "MaximumTupleChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (maximum [(1, 'b'), (2, 'a'), (1, 'c') :: (Int, Char)])\n",
            "LeIntChirho",
        ),
        (
            "PolymorphicMaximumChirho.hs",
            "module Main where\nlargestChirho :: Ord a => [a] -> a\nlargestChirho = maximum\nmain :: IO ()\nmain = print (largestChirho [\"b\", \"c\", \"a\"])\n",
            "LeIntChirho",
        ),
        (
            "EmptyMaximumChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (maximum ([] :: [Int]))\n",
            "no matching alternative",
        ),
    ] {
        let error_chirho = run_result_chirho(file_name_chirho, program_chirho)
            .expect_err("row 565 leaves this extremum on today's path");
        assert!(
            error_chirho.contains(reason_chirho),
            "{file_name_chirho}: {error_chirho}"
        );
    }
}
