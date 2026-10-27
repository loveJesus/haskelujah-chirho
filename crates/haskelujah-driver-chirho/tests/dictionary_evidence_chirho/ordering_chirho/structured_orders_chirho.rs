// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Concrete `Maybe` and tuple `Ord` rows (row 572). Before them, `compare`
//! failed by name on every engine, infix ordering crashed the interpreter and
//! compared native heap addresses, and extrema silently chose the wrong value.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_native_chirho, run_chirho};

/// GHC 9.14.1's exact program, covering direct compare, infix defaults and
/// extrema at Maybe, pairs, triples and nested body-backed component keys.
const STRUCTURED_ORDERS_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = do\n  print (compare (Nothing :: Maybe Int) (Just 1), compare (Just 2 :: Maybe Int) (Just 1))\n  print (compare ((1, 3) :: (Int, Int)) (1, 2), compare ((1, 'a') :: (Int, Char)) (1, 'b'))\n  print (compare ((1, 2, 4) :: (Int, Int, Int)) (1, 2, 3))\n  print (compare (Just [1, 3 :: Int]) (Just [1, 2]), compare ((\"b\", [1 :: Int])) (\"a\", [9]))\n  print ((Just 3 :: Maybe Int) >= Just 2, ((1, 'a') :: (Int, Char)) < (1, 'b'))\n  print (maximum [Nothing, Just 3, Just (1 :: Int)], minimum [(2, 'b'), (2, 'a'), (3, 'z') :: (Int, Char)])\n";

/// `/opt/homebrew/bin/runghc` 9.14.1 on [`STRUCTURED_ORDERS_CHIRHO`].
const STRUCTURED_ORDERS_GHC_CHIRHO: &str =
    "(LT,GT)\n(GT,LT)\nGT\n(GT,GT)\n(True,True)\n(Just 3,(2,'a'))\n";

#[test]
fn maybe_and_tuple_orders_answer_as_ghc_everywhere_chirho() {
    assert_eq!(
        run_chirho("StructuredOrdersChirho.hs", STRUCTURED_ORDERS_CHIRHO),
        STRUCTURED_ORDERS_GHC_CHIRHO,
        "interpreter"
    );
    assert_native_chirho(
        "StructuredOrdersChirho.hs",
        STRUCTURED_ORDERS_CHIRHO,
        STRUCTURED_ORDERS_GHC_CHIRHO,
    );
}
