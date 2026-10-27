// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `max`, `min`, `maximum` and `minimum` through a body-backed `compare` (row 565). Their
//! generic bindings are Int-shaped, comparing with `<=#` and `>=#`. Outside Int, Char and
//! Bool the interpreter refused them (`LeIntChirho: expected Int#`), and both native
//! backends compared heap values as integers: `minimum ["pear", "apple", "zoo"]` printed
//! "pear", `maximum [Green, Red, Blue]` printed Green, and `maximum [1, 0 / 0]` printed NaN.
//! An evidenced occurrence, applied, first-class or partially applied, read its key's own
//! compare row in row 565; since row 627 it reads the key's own `max` or `min` row
//! (`extrema_methods_chirho.rs`), and these programs hold. Each program's GHC 9.14.1 output is
//! recorded beside it.
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

/// Int, Char and Bool, whose Int-shaped bindings were already exact; since row 627 they read
/// their dictionaries' `max` and `min` rows, with the same answers.
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

/// Row 565's remaining named residual, failing in the interpreter as before: the empty list
/// fails with `no matching alternative`, where GHC fails `Prelude.maximum: empty list`. A
/// polymorphic `Ord a =>` wrapper, once pinned here, folds its dictionary's `max` since row
/// 627 (`ord_methods_chirho.rs`).
#[test]
fn extrema_outside_the_compare_rows_stay_named_residuals_chirho() {
    let error_chirho = run_result_chirho(
        "EmptyMaximumChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (maximum ([] :: [Int]))\n",
    )
    .expect_err("row 565 leaves this extremum on today's path");
    assert!(
        error_chirho.contains("no matching alternative"),
        "EmptyMaximumChirho.hs: {error_chirho}"
    );
}
