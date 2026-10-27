// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Structured Eq rows and the Eq-polymorphic list consumers (row 603), beside their Ord
//! counterpart. An Eq-evidenced operation at a concrete key calls that key's body-backed
//! `$prim_Eq_==_K`. Before, `elem`, `notElem`, `lookup`, `nub`, `group`, `isPrefixOf`,
//! `isSuffixOf` and list equality compared elements with Int `==#`. Natively that compared
//! computed Strings, tuples, `Maybe` and lists by address, and Doubles by their bits. The
//! interpreter failed on `EqIntChirho`, and tuple and `Maybe` `==` had no row at all.
//! Every program below is GHC 9.14.1's, byte for byte as measured.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_named_failure_everywhere_chirho, assert_native_chirho, run_chirho};

/// Movers, with GHC's stdout: computed Strings, `[String]` equality, Double signed zero
/// and NaN, pairs, `Maybe` and lists under `==` and the seven consumers, and a derived
/// enum (natively already right, by constructor tag; the interpreter now as well).
const MOVERS_CHIRHO: [(&str, &str, &str); 13] = [
    (
        "EComputedStringChirho.hs",
        "module Main where\nimport Data.List (nub)\nmain :: IO ()\nmain = do\n  print (elem (\"a\" ++ \"b\") [\"x\", \"ab\"], lookup (show (12 :: Int)) [(\"12\", 'y')])\n  print (nub [\"a\" ++ \"b\", \"ab\"], notElem (\"a\" ++ \"b\") [\"ab\"])\n",
        "(True,Just 'y')\n([\"ab\"],False)\n",
    ),
    (
        "EStringListFamilyChirho.hs",
        "module Main where\nimport Data.List (group, isPrefixOf, isSuffixOf)\nmain :: IO ()\nmain = do\n  print ([\"a\" ++ \"b\"] == [\"ab\"], group [\"a\" ++ \"b\", \"ab\", \"c\"])\n  print (isPrefixOf [\"a\" ++ \"b\"] [\"ab\", \"c\"], isSuffixOf [\"c\" ++ \"\"] [\"ab\", \"c\"])\n",
        "(True,[[\"ab\",\"ab\"],[\"c\"]])\n(True,True)\n",
    ),
    (
        "EDoubleEdgesChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print (elem (-0.0 :: Double) [0.0], elem (0 / 0 :: Double) [0 / 0])\n  print (lookup (0.0 :: Double) [(-0.0, \"zero\")])\n",
        "(True,False)\nJust \"zero\"\n",
    ),
    (
        "ECharDoubleListChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print (elem 'b' \"abc\", elem (2.5 :: Double) [1.5, 2.5], elem True [False, True])\n  print (elem [1 :: Int, 2] [[3], [1, 2]], lookup [1 :: Int] [([1], \"one\")])\n",
        "(True,True,True)\n(True,Just \"one\")\n",
    ),
    (
        "EStringMaybeChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print (elem \"ab\" [\"x\", \"ab\"], lookup \"k\" [(\"j\", 1 :: Int), (\"k\", 2)])\n  print (elem (Just (1 :: Int)) [Nothing, Just 1], lookup (Just 'a') [(Just 'a', True)])\n",
        "(True,Just 2)\n(True,Just True)\n",
    ),
    (
        "ENubPairChirho.hs",
        "module Main where\nimport Data.List (nub)\nmain :: IO ()\nmain = print (nub [(1 :: Int, 2 :: Int), (1, 2), (2, 1)], notElem (1 :: Int, 2 :: Int) [(1, 2)])\n",
        "([(1,2),(2,1)],False)\n",
    ),
    (
        "TElemPairChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (elem (1 :: Int, 2 :: Int) [(0, 0), (1, 2)])\n",
        "True\n",
    ),
    (
        "TLookupPairChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (lookup (1 :: Int, 'a') [((1, 'a'), \"x\")])\n",
        "Just \"x\"\n",
    ),
    (
        "TEq2Chirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ((1 :: Int, True) == (1, True), compare (1 :: Int, True) (1, False))\n",
        "(True,GT)\n",
    ),
    (
        "TEq3Chirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ((1 :: Int, True, 'c') == (1, True, 'c'))\n",
        "True\n",
    ),
    (
        "TMaybePairEqChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (Just (1 :: Int, True) == Just (1, True))\n",
        "True\n",
    ),
    (
        "TSignedPairEqChirho.hs",
        "module Main where\nsamePairChirho :: (Int, Int) -> (Int, Int) -> Bool\nsamePairChirho p q = p == q\nmain :: IO ()\nmain = print (samePairChirho (1, 2) (1, 2), samePairChirho (1, 2) (2, 1))\n",
        "(True,False)\n",
    ),
    (
        "HDerivedEnumChirho.hs",
        "module Main where\ndata ColorChirho = RedChirho | GreenChirho deriving (Eq, Show)\nmain :: IO ()\nmain = print (elem GreenChirho [RedChirho, GreenChirho], lookup RedChirho [(RedChirho, 1 :: Int)], (3 :: Int) /= 4)\n",
        "(True,Just 1,True)\n",
    ),
];

/// Holds: Int, Char and Bool keep their Int-shaped bodies, and infix `/=` at String and
/// Double was already exact. The `(/= x)` section and structured `/=` are row 606's.
const HOLDS_CHIRHO: [(&str, &str, &str); 2] = [
    (
        "EUnboxedHoldsChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (elem 'c' \"abc\", notElem True [False], lookup (3 :: Int) [(3, \"three\")])\n",
        "(True,True,Just \"three\")\n",
    ),
    (
        "NInfixHoldChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (\"a\" ++ \"b\" /= \"ab\", (0 / 0 :: Double) /= (0 / 0), (-0.0 :: Double) /= 0.0)\n",
        "(False,True,False)\n",
    ),
];

/// Still failing by name, as ruled: `==` on a parameterized derived type (its row is
/// named with its type variable), and structured `/=` (row 606).
const NAMED_FAILURES_CHIRHO: [(&str, &str); 2] = [
    (
        "HDerivedBoxChirho.hs",
        "module Main where\ndata BoxChirho a = BoxChirho a deriving (Eq, Show)\nmain :: IO ()\nmain = print (BoxChirho (1 :: Int) == BoxChirho 1, BoxChirho 'a' /= BoxChirho 'b')\n",
    ),
    (
        "NStructuredChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ((1 :: Int, 2 :: Int) /= (2, 1), Just 'a' /= Just 'a', [1 :: Int] /= [1])\n",
    ),
];

fn assert_ghc_everywhere_chirho(file_name_chirho: &str, source_chirho: &str, ghc_chirho: &str) {
    assert_eq!(
        run_chirho(file_name_chirho, source_chirho),
        ghc_chirho,
        "{file_name_chirho} interpreter"
    );
    assert_native_chirho(file_name_chirho, source_chirho, ghc_chirho);
}

#[test]
fn structured_equality_answers_as_ghc_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in MOVERS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn int_shaped_keys_and_infix_not_equal_stay_exact_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in HOLDS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn unserved_equalities_still_fail_by_name_everywhere_chirho() {
    for (file_name_chirho, source_chirho) in NAMED_FAILURES_CHIRHO {
        assert_named_failure_everywhere_chirho(file_name_chirho, source_chirho, "binding `==`");
    }
}
