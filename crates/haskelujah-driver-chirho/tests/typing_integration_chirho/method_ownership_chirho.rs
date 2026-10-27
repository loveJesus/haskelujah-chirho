// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! A program's own top-level binding of a class method's name keeps its binding (row 639).
//!
//! The dictionary pass's shadow seeding read any predicate of the method's class in the
//! binding's scheme as proof that the binding was the class method itself. An unsigned
//! `a + b = 42 :: Int` keeps its body's ground `Num Int` in its inferred scheme, so the pass
//! rewrote every use to the Prelude instance's method, silently, on every engine, for Num, Eq,
//! Ord and Show alike. Now every input binding named like a class method is the program's own,
//! whatever its scheme: the pass generates the class selectors apart from the module's
//! bindings. A program's own `fromInteger` keeps its calls, while its literals keep Num's
//! method (row 660). Every table is GHC 9.14.1's output, byte for byte as measured.
//! workflow: language-features-chirho/own-class-method-occurrences-chirho

use super::assert_prints_everywhere_chirho;

const OWNED_METHOD_MOVERS_CHIRHO: [(&str, &str, &str); 13] = [
    (
        "UInfixPlusChirho.hs",
        "module Main where\nimport Prelude hiding ((+))\na + b = 42 :: Int\nmain :: IO ()\nmain = print (1 + 2 :: Int)\n",
        "42\n",
    ),
    (
        "UPrefixPlusChirho.hs",
        "module Main where\nimport Prelude hiding ((+))\n(+) a b = 42 :: Int\nmain :: IO ()\nmain = print ((1 :: Int) + 2)\n",
        "42\n",
    ),
    (
        "UInfixTimesChirho.hs",
        "module Main where\nimport Prelude hiding ((*))\na * b = 42 :: Int\nmain :: IO ()\nmain = print (1 * 2 :: Int)\n",
        "42\n",
    ),
    (
        "UInfixMinusChirho.hs",
        "module Main where\nimport Prelude hiding ((-))\na - b = 42 :: Int\nmain :: IO ()\nmain = print (1 - 2 :: Int)\n",
        "42\n",
    ),
    (
        "UNegateChirho.hs",
        "module Main where\nimport Prelude hiding (negate)\nnegate x = 42 :: Int\nmain :: IO ()\nmain = print (negate (1 :: Int))\n",
        "42\n",
    ),
    (
        "UAbsChirho.hs",
        "module Main where\nimport Prelude hiding (abs)\nabs x = 42 :: Int\nmain :: IO ()\nmain = print (abs (-1 :: Int))\n",
        "42\n",
    ),
    (
        "USignumChirho.hs",
        "module Main where\nimport Prelude hiding (signum)\nsignum x = 42 :: Int\nmain :: IO ()\nmain = print (signum (-1 :: Int))\n",
        "42\n",
    ),
    (
        "UPlusUsesChirho.hs",
        "module Main where\nimport Prelude hiding ((+))\na + b = 42 :: Int\noneChirho :: Int\noneChirho = 1\nmain :: IO ()\nmain = print (oneChirho + oneChirho, (+) 1 2, map (+ 1) [5 :: Int])\n",
        "(42,42,[42])\n",
    ),
    (
        "UPlusHigherOrderChirho.hs",
        "module Main where\nimport Prelude hiding ((+))\na + b = 42 :: Int\nmain :: IO ()\nmain = print (zipWith (+) [1, 2] [3, 4 :: Int], foldr (+) 0 [1, 2 :: Int], (+ 1) 5, (1 +) 2)\n",
        "([42,42],42,42,42)\n",
    ),
    (
        "CEqInvertChirho.hs",
        "module Main where\nimport Prelude hiding ((==))\nimport qualified Prelude as P\na == b = (a :: Int) P./= b\nmain :: IO ()\nmain = print (1 == 2, 3 == 3)\n",
        "(True,False)\n",
    ),
    (
        "COrdFlipChirho.hs",
        "module Main where\nimport Prelude hiding ((<))\nimport qualified Prelude as P\na < b = (a :: Int) P.> b\nmain :: IO ()\nmain = print (1 < 2, 3 < 2)\n",
        "(False,True)\n",
    ),
    (
        "CShowGroundChirho.hs",
        "module Main where\nimport Prelude hiding (show)\nimport qualified Prelude as P\nshow x = P.show (42 :: Int)\nmain :: IO ()\nmain = putStrLn (show True)\n",
        "42\n",
    ),
    (
        "UFromIntegerChirho.hs",
        "module Main where\nimport Prelude hiding (fromInteger)\nfromInteger x = 42 :: Int\nmain :: IO ()\nmain = print (fromInteger 5, 5 :: Int)\n",
        "(42,5)\n",
    ),
];

const OWNED_METHOD_HOLDS_CHIRHO: [(&str, &str, &str); 8] = [
    (
        "HDivideChirho.hs",
        "module Main where\nimport Prelude hiding ((/))\na / b = 42 :: Int\nmain :: IO ()\nmain = print (1 / 2 :: Int)\n",
        "42\n",
    ),
    (
        "CDivideGroundChirho.hs",
        "module Main where\nimport Prelude hiding ((/))\na / b = 0.5 :: Double\nmain :: IO ()\nmain = print (1 / 4 :: Double)\n",
        "0.5\n",
    ),
    (
        "HShowChirho.hs",
        "module Main where\nimport Prelude hiding (show)\nshow x = \"mine\"\nmain :: IO ()\nmain = putStrLn (show (1 :: Int))\n",
        "mine\n",
    ),
    (
        "USignedPlusChirho.hs",
        "module Main where\nimport Prelude hiding ((+))\n(+) :: Int -> Int -> Int\na + b = 42\nmain :: IO ()\nmain = print (1 + 2)\n",
        "42\n",
    ),
    (
        "HWherePlusChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (1 + 2 :: Int)\n  where\n    a + b = 42 :: Int\n",
        "42\n",
    ),
    (
        "HMinusNumChirho.hs",
        "module Main where\nimport Prelude hiding ((-))\na - b = a + b\nmain :: IO ()\nmain = print (3 - 1 :: Int)\n",
        "4\n",
    ),
    (
        "UPlusListsChirho.hs",
        "module Main where\nimport Prelude hiding ((+))\na + b = a ++ b\nmain :: IO ()\nmain = putStrLn (\"ab\" + \"cd\")\n",
        "abcd\n",
    ),
    (
        "TPlusMixedChirho.hs",
        "module Main where\ninfixl 6 |+|\na |+| b = fromIntegral a * b\nmain :: IO ()\nmain = print ((3 :: Int) |+| (4.5 :: Double))\n",
        "13.5\n",
    ),
];

const POLYMORPHIC_OWNED_HOLDS_CHIRHO: [(&str, &str, &str); 5] = [
    (
        "POwnPlusSigChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Prelude hiding ((+))\nimport qualified Prelude as P\n(+) :: Num a => a -> a -> a\na + b = a P.+ b P.+ 1\nmain :: IO ()\nmain = print (1 + 2 :: Int, (+) 3 4 :: Int, map (10 +) [1, 2 :: Int], foldr (+) 0 [1, 2, 3 :: Int])\n",
        "(4,8,[12,13],9)\n",
    ),
    (
        "POwnPlusChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Prelude hiding ((+))\nimport qualified Prelude as P\na + b = a P.+ b P.+ 1\nmain :: IO ()\nmain = print (1 + 2 :: Int, (+) 3 4 :: Int, map (10 +) [1, 2 :: Int], foldr (+) 0 [1, 2, 3 :: Int])\n",
        "(4,8,[12,13],9)\n",
    ),
    (
        "POwnEqSigChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Prelude hiding ((==))\nimport qualified Prelude as P\n(==) :: Eq a => a -> a -> Bool\na == b = a P./= b\nmain :: IO ()\nmain = print (1 == (1 :: Int), (==) 1 (2 :: Int), filter (== 3) [1, 2, 3 :: Int])\n",
        "(False,True,[1,2])\n",
    ),
    (
        "POwnEqChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Prelude hiding ((==))\nimport qualified Prelude as P\na == b = a P./= b\nmain :: IO ()\nmain = print (1 == (1 :: Int), (==) 1 (2 :: Int), filter (== 3) [1, 2, 3 :: Int])\n",
        "(False,True,[1,2])\n",
    ),
    (
        "POwnShowChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Prelude hiding (show)\nimport qualified Prelude as P\nshow x = \"<\" ++ P.show x ++ \">\"\nmain :: IO ()\nmain = putStrLn (show (42 :: Int) ++ concatMap show [1, 2 :: Int])\n",
        "<42><1><2>\n",
    ),
];

const CLASS_SELECTOR_HOLDS_CHIRHO: [(&str, &str, &str); 4] = [
    (
        "SOrdinaryChirho.hs",
        "module Main where\nnewtype V = V Int deriving Show\ninstance Num V where\n  V a + V b = V (a + b)\n  V a * V b = V (a * b)\n  abs (V a) = V (abs a)\n  signum (V a) = V (signum a)\n  fromInteger n = V (fromInteger n)\n  negate (V a) = V (negate a)\nmain :: IO ()\nmain = do\n  print (1 + 2 :: Int, 3 * 4 :: Integer, negate 5 :: Int, abs (-6) :: Int)\n  print (show (7 :: Int), 8 == (8 :: Int), compare (9 :: Int) 1, max (2 :: Int) 3)\n  print (V 1 + V 2, V 3 * 4, signum (V 5))\n  print (fromIntegral (7 :: Int) / (2 :: Double))\n",
        "(3,12,-5,6)\n(\"7\",True,GT,3)\n(V 3,V 12,V 1)\n3.5\n",
    ),
    (
        "SOwnClassChirho.hs",
        "module Main where\nclass Describe a where\n  describe :: a -> String\ninstance Describe Int where\n  describe n = \"int:\" ++ show n\ninstance Describe Bool where\n  describe b = \"bool:\" ++ show b\nmain :: IO ()\nmain = do\n  putStrLn (describe (3 :: Int))\n  putStrLn (describe True)\n  mapM_ (putStrLn . describe) [False, True]\n",
        "int:3\nbool:True\nbool:False\nbool:True\n",
    ),
    (
        "SMptcArgsChirho.hs",
        "{-# LANGUAGE MultiParamTypeClasses #-}\nmodule Main where\nclass Combine a b where\n  combine :: a -> b -> String\ninstance Combine Int Bool where\n  combine n b = show n ++ \"/\" ++ show b\ninstance Combine Bool Int where\n  combine b n = show b ++ \"\\\\\" ++ show n\nmain :: IO ()\nmain = do\n  putStrLn (combine (1 :: Int) True)\n  putStrLn (combine False (2 :: Int))\n",
        "1/True\nFalse\\2\n",
    ),
    (
        "SDefaultMethodChirho.hs",
        "module Main where\nclass Describe a where\n  describe :: a -> String\n  describe _ = \"default\"\ninstance Describe Int where\n  describe n = \"int:\" ++ show n\ninstance Describe Bool\nmain :: IO ()\nmain = do\n  putStrLn (describe (3 :: Int))\n  putStrLn (describe True)\n",
        "int:3\ndefault\n",
    ),
];

/// Unsigned redefinitions of Num, Eq, Ord and Show methods take the program's own binding:
/// infix, prefix, sections and higher-order uses, and `fromInteger` beside its literal.
#[test]
fn an_owned_method_name_keeps_its_binding_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in OWNED_METHOD_MOVERS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

/// What held: Fractional's `/` with ground bodies, `show x = "mine"`, a signed and a local
/// binding, a method-shaped `-`, a list-typed `+`, and a non-method twin.
#[test]
fn owned_method_holds_stay_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in OWNED_METHOD_HOLDS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

/// A program's own polymorphic `+`, `==` and `show`, signed or inferred, keep their bodies at
/// Int: the class predicate over a quantified variable does not make them the class's method.
#[test]
fn a_polymorphic_owned_method_name_keeps_its_binding_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in POLYMORPHIC_OWNED_HOLDS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

/// Real class selectors keep dispatching: the Prelude's, a module's own class's, a
/// multi-parameter class's, and a class default's.
#[test]
fn class_selectors_keep_dispatching_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in CLASS_SELECTOR_HOLDS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}
