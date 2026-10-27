// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Tuple `Show`, `Eq` and `Ord` at every arity GHC's base covers, 2 to 15 (row 604). Before,
//! only pairs and triples had instances: `print` of a 4-tuple, or `==` and `compare` on one,
//! failed to type-check on every engine. Every program below is GHC 9.14.1's, byte for byte
//! as measured.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::assert_ghc_everywhere_chirho;

/// `print` and `show` at arities 5 to 8 and 15, mixed and nested fields, negative numbers
/// unparenthesized inside a tuple as GHC prints them.
const SHOWS_CHIRHO: [(&str, &str, &str); 7] = [
    (
        "TS5Chirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (1 :: Int, True, 'c', \"s\", 2.5 :: Double)\n",
        "(1,True,'c',\"s\",2.5)\n",
    ),
    (
        "TS6Chirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (1 :: Int, True, 'c', \"s\", 2.5 :: Double, [1 :: Int])\n",
        "(1,True,'c',\"s\",2.5,[1])\n",
    ),
    (
        "TS7Chirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (1 :: Int, True, 'c', \"s\", 2.5 :: Double, [1 :: Int], Just (3 :: Int))\n",
        "(1,True,'c',\"s\",2.5,[1],Just 3)\n",
    ),
    (
        "TS8Chirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (1 :: Int, 2 :: Int, 3 :: Int, 4 :: Int, 5 :: Int, 6 :: Int, 7 :: Int, 8 :: Int)\n",
        "(1,2,3,4,5,6,7,8)\n",
    ),
    (
        "TS15Chirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (1 :: Int, 2 :: Int, 3 :: Int, 4 :: Int, 5 :: Int, 6 :: Int, 7 :: Int, 8 :: Int, 9 :: Int, 10 :: Int, 11 :: Int, 12 :: Int, 13 :: Int, 14 :: Int, 15 :: Int)\n",
        "(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15)\n",
    ),
    (
        "TSS6Chirho.hs",
        "module Main where\nmain :: IO ()\nmain = putStrLn (show (Just (1 :: Int), [2 :: Int], \"x\", 4.0 :: Double, False, LT))\n",
        "(Just 1,[2],\"x\",4.0,False,LT)\n",
    ),
    (
        "TShowNestedChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print (-1 :: Int, Just (-2 :: Int), [(-3 :: Int, 'x')], (4 :: Int, (5 :: Int, 6 :: Int)), -7.5 :: Double)\n  print (Just (1 :: Int, 2 :: Int, 3 :: Int, 4 :: Int), [(1 :: Int, 2 :: Int, 3 :: Int, 4 :: Int, 5 :: Int)])\n",
        "(-1,Just (-2),[(-3,'x')],(4,(5,6)),-7.5)\n(Just (1,2,3,4),[(1,2,3,4,5)])\n",
    ),
];

/// `==`, `compare`, `<` and `max` at arities 3, 4, 6 and 15, and row 603's `elem`, `lookup`
/// and `nub` on 4-tuples, through the Eq and Ord rows composed at the wider arity.
const COMPARISONS_CHIRHO: [(&str, &str, &str); 4] = [
    (
        "TEq34Chirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print ((1 :: Int, True, 'c') == (1, True, 'c'), compare (1 :: Int, True, 'c') (1, True, 'd'))\n  print ((1 :: Int, 2 :: Int, 3 :: Int, 4 :: Int) == (1, 2, 3, 4), compare (1 :: Int, 2 :: Int, 3 :: Int, 4 :: Int) (1, 2, 3, 5))\n",
        "(True,LT)\n(True,LT)\n",
    ),
    (
        "TE6Chirho.hs",
        "module Main where\nsixChirho :: (Int, Int, Int, Int, Int, Int)\nsixChirho = (1, 2, 3, 4, 5, 6)\nmain :: IO ()\nmain = do\n  print (sixChirho == (1, 2, 3, 4, 5, 6), sixChirho == (1, 2, 3, 4, 5, 7))\n  print (compare sixChirho (1, 2, 3, 4, 5, 7), sixChirho < (0, 9, 9, 9, 9, 9))\n",
        "(True,False)\n(LT,False)\n",
    ),
    (
        "TEqOrd4Chirho.hs",
        "module Main where\nimport Data.List (nub)\nmain :: IO ()\nmain = do\n  print ((1 :: Int, True, 'c', \"s\") == (1, True, 'c', \"s\"), compare (1 :: Int, 2 :: Int, 3 :: Int, 4 :: Int) (1, 2, 3, 5))\n  print (elem (1 :: Int, 2 :: Int, 3 :: Int, 4 :: Int) [(0, 0, 0, 0), (1, 2, 3, 4)], lookup (1 :: Int, 'a', True, 2.5 :: Double) [((1, 'a', True, 2.5), \"hit\")])\n  print (nub [(1 :: Int, 2 :: Int, 3 :: Int, 4 :: Int), (1, 2, 3, 4)], max (1 :: Int, 9 :: Int, 9 :: Int, 9 :: Int) (2, 0, 0, 0))\n",
        "(True,LT)\n(True,Just \"hit\")\n([(1,2,3,4)],(2,0,0,0))\n",
    ),
    (
        "TEqOrd15Chirho.hs",
        "module Main where\nfifteenChirho :: (Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int)\nfifteenChirho = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)\nmain :: IO ()\nmain = print (fifteenChirho == fifteenChirho, compare fifteenChirho (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16), fifteenChirho < fifteenChirho)\n",
        "(True,LT,False)\n",
    ),
];

#[test]
fn tuples_show_as_ghc_at_every_arity_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in SHOWS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn tuples_compare_as_ghc_at_every_arity_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in COMPARISONS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}
