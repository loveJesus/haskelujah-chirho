// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! An operator the program owns keeps its own binding over every desugarer shortcut (row 614).
//! The infix chain, the section table and the prefix division primops each chose a Prelude
//! primop or rewrite by the operator's spelling alone, so a program's hidden-and-redefined
//! operator, or a local one, gave the Prelude's answer silently on every engine.
//!
//! An infix `$` is outside this file: the parser's fixity resolution rewrites `f $ x` to an
//! application before naming, which is row 619's. An operator imported from another user
//! module is outside it too: a single-file run links no sibling module (#29717). Every
//! program here is GHC 9.14.1's, byte for byte as measured.
//! workflow: compiler-pipeline-chirho/value-scope-resolution-chirho

use super::assert_prints_everywhere_chirho;

/// Top-level owners in the infix chain: the float-literal arithmetic, `**` and `^`, `&&` and
/// `||`, `$!` and `.`, `$!!`, `/=` and `++`.
const INFIX_OWNERS_CHIRHO: [(&str, &str, &str); 7] = [
    (
        "TFloatLiteralOwnChirho.hs",
        "module Main where\nimport Prelude hiding ((+), (-), (*), (/))\nimport qualified Prelude as P\n(+) :: Double -> Double -> Double\n(+) a b = a P.* b\n(-) :: Double -> Double -> Double\n(-) a b = a P.+ b\n(*) :: Double -> Double -> Double\n(*) a b = a P.- b\n(/) :: Double -> Double -> Double\n(/) a b = a P.* b\nmain :: IO ()\nmain = print (1.5 + 2.0, 1.5 - 2.0, 1.5 * 2.0, 6.0 / 3.0, 2 + 1.5)\n",
        "(3.0,3.5,-0.5,18.0,3.0)\n",
    ),
    (
        "TPowerOwnChirho.hs",
        "module Main where\nimport Prelude hiding ((**), (^))\n(**) :: Double -> Double -> Double\na ** b = a + b\n(^) :: Double -> Int -> Double\na ^ n = a * fromIntegral n\nmain :: IO ()\nmain = print (2.0 ** 3.0, 2.0 ^ (3 :: Int), 2.5 ** 2)\n",
        "(5.0,6.0,4.5)\n",
    ),
    (
        "TBooleanOwnChirho.hs",
        "module Main where\nimport Prelude hiding ((&&), (||))\n(&&), (||) :: Bool -> Bool -> Bool\na && b = a /= b\na || b = a == b\nmain :: IO ()\nmain = print (True && True, True || False, False && False)\n",
        "(False,False,False)\n",
    ),
    (
        "TStrictComposeOwnChirho.hs",
        "module Main where\nimport Prelude hiding (($!), (.))\n($!) :: (Int -> Int) -> Int -> Int\n($!) f x = f (f (f x))\n(.) :: (Int -> Int) -> (Int -> Int) -> Int -> Int\n(.) f g = \\x -> g (f x)\nmain :: IO ()\nmain = print ((+ 1) $! 5, ((* 2) . (+ 1)) 3)\n",
        "(8,7)\n",
    ),
    (
        "TDeepApplyOwnChirho.hs",
        "module Main where\n($!!) :: (Int -> Int) -> Int -> Int\nf $!! x = f (f x)\nmain :: IO ()\nmain = print ((+ 1) $!! 5)\n",
        "7\n",
    ),
    (
        "TNotEqualOwnChirho.hs",
        "module Main where\nimport Prelude hiding ((/=))\n(/=) :: Int -> Int -> Bool\na /= b = a == b\nmain :: IO ()\nmain = print (1 /= (1 :: Int), 1 /= (2 :: Int))\n",
        "(True,False)\n",
    ),
    (
        "TListOwnChirho.hs",
        "module Main where\nimport Prelude hiding ((++))\n(++) :: [Int] -> [Int] -> [Int]\na ++ b = b\nmain :: IO ()\nmain = print ([1] ++ [2])\n",
        "[2]\n",
    ),
];

/// Owners in `where` clauses across every family, and a self-qualified owner (`Main.&&`,
/// `Main.div`).
const LOCAL_AND_QUALIFIED_OWNERS_CHIRHO: [(&str, &str, &str); 3] = [
    (
        "TFloatLiteralLocalChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (1.5 + 2.0, 6.0 / 3.0)\n  where\n    a + b = a * (b :: Double)\n    a / b = a - (b :: Double)\n",
        "(3.0,3.0)\n",
    ),
    (
        "TLocalOwnersChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print (2.0 ** 3.0, 2.0 ^ (3 :: Int), True && True, True || False, (+ 1) $! (5 :: Int), ((* 2) . (+ 1)) (3 :: Int), (1 :: Int) /= 1, [1 :: Int] ++ [2])\n  print (7 `div` (2 :: Int), div 7 (2 :: Int), mod 7 (2 :: Int), quot 7 (2 :: Int), rem 7 (2 :: Int), map (++ [9]) [[1 :: Int]], map (2.0 ^) [3 :: Int], filter (/= 1) [1, 2 :: Int], map (** 2.0) [3.0])\n  where\n    a ** b = a + (b :: Double)\n    a ^ n = a * fromIntegral (n :: Int)\n    a && b = not (a == b)\n    a || b = a == (b :: Bool)\n    f $! x = f (f (x :: Int))\n    (.) f g = \\x -> g (f (x :: Int))\n    a /= b = a == (b :: Int)\n    a ++ b = (b :: [Int])\n    div a b = a + (b :: Int)\n    mod a b = a * (b :: Int)\n    quot a b = a - (b :: Int)\n    rem a b = b + (a :: Int)\n",
        "(5.0,6.0,False,False,7,7,True,[2])\n(9,9,14,5,9,[[9]],[6.0],[1],[5.0])\n",
    ),
    (
        "TSelfQualifiedChirho.hs",
        "module Main where\nimport Prelude hiding ((&&), div)\n(&&) :: Bool -> Bool -> Bool\na && b = not (a == b)\ndiv :: Int -> Int -> Int\ndiv a b = a + b\nmain :: IO ()\nmain = print (True Main.&& True, Main.div 7 2, 7 `Main.div` 2, map (Main.&& True) [True])\n",
        "(False,9,9,[False])\n",
    ),
];

/// Owned sections of `++`, `^`, `**` and `/=`, a backticked `div` section, and `div` and
/// `mod` infix with `div`, `mod`, `quot` and `rem` prefix.
const SECTION_AND_PREFIX_OWNERS_CHIRHO: [(&str, &str, &str); 2] = [
    (
        "TSectionsOwnChirho.hs",
        "module Main where\nimport Prelude hiding ((++), (^), (**), (/=), div)\n(++) :: [Int] -> [Int] -> [Int]\na ++ b = b\n(^) :: Int -> Int -> Int\na ^ b = a + b\n(**) :: Double -> Double -> Double\na ** b = a * b\n(/=) :: Int -> Int -> Bool\na /= b = a == b\ndiv :: Int -> Int -> Int\ndiv a b = a + b\nmain :: IO ()\nmain = print (map (++ [9]) [[1]], map (2 ^) [3], map (** 2.0) [3.0], filter (/= 1) [1, 2], map (`div` 2) [7])\n",
        "([[9]],[5],[6.0],[1],[9])\n",
    ),
    (
        "TDivModOwnChirho.hs",
        "module Main where\nimport Prelude hiding (div, mod, quot, rem)\ndiv, mod, quot, rem :: Int -> Int -> Int\ndiv a b = a + b\nmod a b = a * b\nquot a b = a - b\nrem a b = b\nmain :: IO ()\nmain = print (7 `div` 2, 7 `mod` 2, div 7 2, mod 7 2, quot 7 2, rem 7 2)\n",
        "(9,14,9,14,5,2)\n",
    ),
];

/// Every Prelude use keeps its shortcut: the float-literal arithmetic, `**`, `^`, `&&`, `++`,
/// `$`, `$!` and `.`, the sections and the division forms. A program's own `>>=` and `>>`
/// already took their binding.
const PRELUDE_HOLDS_CHIRHO: [(&str, &str, &str); 3] = [
    (
        "HPreludeOperatorsChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (1.5 + 2.0, 1.5 - 2.0, 1.5 * 2.0, 6.0 / 3.0, 2 + 1.5, 2.0 ** 3.0, 2.0 ^ (3 :: Int), True && False, [1] ++ [2 :: Int], 7 `div` (2 :: Int), div 7 (2 :: Int), quot (-7) (2 :: Int), (+ 1) $ (5 :: Int), ((* 2) . (+ 1)) (3 :: Int))\n",
        "(3.5,-0.5,3.0,2.0,3.5,8.0,8.0,False,[1,2],3,3,-3,6,8)\n",
    ),
    (
        "HPreludeSectionsChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (map (++ \"!\") [\"a\"], map (2 ^) [3 :: Int], map (** 2) [3.0 :: Double], filter (/= 1) [1, 2 :: Int], map (`div` 2) [7 :: Int], map (`mod` 3) [7 :: Int], map (7 `div`) [2 :: Int], mod 7 (3 :: Int), rem (-7) (2 :: Int), (+ 1) $! (5 :: Int), [x | x <- [1, 2, 3 :: Int], x /= 2])\n",
        "([\"a!\"],[8],[9.0],[2],[3],[1],[3],1,-1,6,[1,3])\n",
    ),
    (
        "TBindOwnChirho.hs",
        "module Main where\nimport Prelude hiding ((>>=), (>>))\n(>>=) :: Maybe Int -> (Int -> Maybe Int) -> Maybe Int\nm >>= f = Nothing\n(>>) :: Maybe Int -> Maybe Int -> Maybe Int\na >> b = a\nmain :: IO ()\nmain = print (Just 1 >>= (\\x -> Just (x + 1)), Just 1 >> Just 2)\n",
        "(Nothing,Just 1)\n",
    ),
];

#[test]
fn owned_infix_operators_keep_their_bindings_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in INFIX_OWNERS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn local_and_self_qualified_owners_keep_their_bindings_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in LOCAL_AND_QUALIFIED_OWNERS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn owned_sections_and_prefix_division_keep_their_bindings_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in SECTION_AND_PREFIX_OWNERS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn prelude_operators_keep_their_shortcuts_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in PRELUDE_HOLDS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}
