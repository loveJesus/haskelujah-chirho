// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Eq's `==` and `/=`, each defaulting to the other's negation (row 606). The seeded Eq class
//! declared only `==`, so an instance's own `(/=)` was never read, infix `/=` was rewritten to
//! `not (== …)` with an evidence-free `==`, and a section or first-class `(/=)` had no route.
//! Every program below is GHC 9.14.1's, byte for byte as measured.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_named_failure_everywhere_chirho, assert_native_chirho, run_chirho};

/// An instance with both methods, only `==` or only `/=`; first-class `(/=)`; `/=` at pairs,
/// `Maybe` and lists; sections at computed Strings, NaN and signed zero; a derived enum; and
/// generic heads that write only one method. Main gave wrong answers or failed on each.
const EQUALITY_MOVERS_CHIRHO: [(&str, &str, &str); 8] = [
    (
        "NUserOverrideChirho.hs",
        "module Main where\ndata AlwaysChirho = AlwaysChirho\ninstance Eq AlwaysChirho where\n  (==) _ _ = True\n  (/=) _ _ = True\nmain :: IO ()\nmain = print (AlwaysChirho /= AlwaysChirho, not (AlwaysChirho == AlwaysChirho))\n",
        "(True,False)\n",
    ),
    (
        "NOnlyEqChirho.hs",
        "module Main where\nnewtype ModChirho = ModChirho Int deriving Show\ninstance Eq ModChirho where\n  ModChirho a == ModChirho b = a `mod` 3 == b `mod` 3\nmain :: IO ()\nmain = print (ModChirho 1 /= ModChirho 4, ModChirho 1 /= ModChirho 2, filter (/= ModChirho 0) [ModChirho 3, ModChirho 5], zipWith (/=) [ModChirho 1] [ModChirho 7])\n",
        "(False,True,[ModChirho 5],[False])\n",
    ),
    (
        "NOnlyNotEqChirho.hs",
        "module Main where\nnewtype ParityChirho = ParityChirho Int deriving Show\ninstance Eq ParityChirho where\n  ParityChirho a /= ParityChirho b = even a /= even b\nmain :: IO ()\nmain = print (ParityChirho 1 == ParityChirho 3, ParityChirho 1 == ParityChirho 2, ParityChirho 2 /= ParityChirho 4, elem (ParityChirho 5) [ParityChirho 2, ParityChirho 7], lookup (ParityChirho 4) [(ParityChirho 1, 'a'), (ParityChirho 6, 'b')])\n",
        "(True,False,False,True,Just 'b')\n",
    ),
    (
        "NFirstClassChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (zipWith (/=) [\"a\" ++ \"b\", \"x\"] [\"ab\", \"y\"], zipWith (/=) [1 :: Int, 2] [1, 3])\n",
        "([False,True],[False,True])\n",
    ),
    (
        "NStructuredInfixChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ((1 :: Int, 2 :: Int) /= (2, 1), Just 'a' /= Just 'a', [1 :: Int] /= [1], (\"a\" ++ \"b\", 1 :: Int) /= (\"ab\", 1))\n",
        "(True,False,False,False)\n",
    ),
    (
        "NSectionsChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print (filter (/= \"ab\") [\"a\" ++ \"b\", \"c\"], filter (\"ab\" /=) [\"a\" ++ \"b\", \"c\"])\n  print (map (/= 2.5) [2.5 :: Double], map (/= (0 / 0)) [0 / 0 :: Double], map (/= 0.0) [-0.0 :: Double])\n",
        "([\"c\"],[\"c\"])\n([False],[True],[False])\n",
    ),
    (
        "NDerivedChirho.hs",
        "module Main where\ndata ColorChirho = RedChirho | GreenChirho deriving (Eq, Show)\nmain :: IO ()\nmain = print (RedChirho /= GreenChirho, RedChirho /= RedChirho, filter (/= RedChirho) [RedChirho, GreenChirho])\n",
        "(True,False,[GreenChirho])\n",
    ),
    (
        "NGenericHeadChirho.hs",
        "module Main where\ndata OneChirho a = OneChirho\ninstance Eq (OneChirho a) where\n  _ == _ = True\ndata TwoChirho a = TwoChirho\ninstance Eq (TwoChirho a) where\n  _ /= _ = True\nmain :: IO ()\nmain = print (OneChirho /= (OneChirho :: OneChirho Int), OneChirho == (OneChirho :: OneChirho Bool), TwoChirho == (TwoChirho :: TwoChirho Int), TwoChirho /= (TwoChirho :: TwoChirho Char))\n",
        "(False,True,False,True)\n",
    ),
];

/// Infix `/=` at Int, Char, Bool, computed Strings and NaN, and Eq through polymorphic code and
/// Ord's superclass dictionary, which reads the moved `==` slot. Each matched GHC on main.
const EQUALITY_HOLDS_CHIRHO: [(&str, &str, &str); 2] = [
    (
        "NInfixHoldsChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ((3 :: Int) /= 4, 'a' /= 'a', True /= False, \"a\" ++ \"b\" /= \"ab\", (0 / 0 :: Double) /= (0 / 0))\n",
        "(True,False,True,False,True)\n",
    ),
    (
        "NPolymorphicEqChirho.hs",
        "module Main where\nimport Data.List (nub)\ndata ShapeChirho = CircleChirho | SquareChirho deriving (Eq, Ord, Show)\nnewtype ModChirho = ModChirho Int deriving Show\ninstance Eq ModChirho where\n  ModChirho a == ModChirho b = a `mod` 3 == b `mod` 3\ndiffersChirho :: Eq a => a -> a -> Bool\ndiffersChirho x y = x /= y\nsameChirho :: Eq a => a -> a -> Bool\nsameChirho x y = x == y\nviaOrdChirho :: Ord a => a -> a -> (Bool, Bool, Ordering)\nviaOrdChirho x y = (x == y, x /= y, compare x y)\nmain :: IO ()\nmain = do\n  print (differsChirho (ModChirho 1) (ModChirho 4), sameChirho (ModChirho 1) (ModChirho 4), differsChirho \"a\" \"b\", sameChirho (2.5 :: Double) 2.5)\n  print (viaOrdChirho CircleChirho SquareChirho, viaOrdChirho (3 :: Int) 3, viaOrdChirho \"b\" \"a\")\n  print (nub [ModChirho 1, ModChirho 4, ModChirho 2], elem (ModChirho 7) [ModChirho 1])\n",
        "(False,True,True,True)\n((False,True,LT),(True,False,EQ),(False,True,GT))\n([ModChirho 1,ModChirho 2],True)\n",
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
fn eq_methods_default_to_each_other_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in EQUALITY_MOVERS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn equality_holds_stay_ghc_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in EQUALITY_HOLDS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

/// An instance with neither method has no real body to default from. Each method keeps its
/// named missing-method failure, and no pair of defaults is made from the two stubs. GHC's
/// two defaults call each other, and the program never ends.
#[test]
fn an_instance_with_neither_method_fails_by_name_everywhere_chirho() {
    assert_named_failure_everywhere_chirho(
        "NNeitherEqChirho.hs",
        "module Main where\ndata VoidChirho = VoidChirho\ninstance Eq VoidChirho\nmain :: IO ()\nmain = print (VoidChirho == VoidChirho)\n",
        "missing method Eq.== for VoidChirho",
    );
    assert_named_failure_everywhere_chirho(
        "NNeitherNotEqChirho.hs",
        "module Main where\ndata VoidChirho = VoidChirho\ninstance Eq VoidChirho\nmain :: IO ()\nmain = print (VoidChirho /= VoidChirho)\n",
        "missing method Eq./= for VoidChirho",
    );
}
