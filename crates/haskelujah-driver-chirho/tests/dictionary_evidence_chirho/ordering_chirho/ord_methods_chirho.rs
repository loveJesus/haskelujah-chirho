// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ord's seven methods, with row 612's six body-backed slots beside `compare`. `<=`, `max` and
//! `min` follow GHC's defaults; `<`, `>` and `>=` remain compare-backed until row 626 moves them
//! onto GHC's `<=` chain. The seeded Ord class declared only `compare`, so those six methods had no
//! dictionary slot. Inside `Ord a =>` code they fell back to Int primops, silently wrong outside
//! Int, Char and Bool, and an instance's own operators were never read through a dictionary.
//! Every program below is GHC 9.14.1's, byte for byte as measured.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_ghc_everywhere_chirho, assert_named_failure_everywhere_chirho};

/// Inside `Ord a =>` code: the operators, `compare` and sections at Int, Char, Bool, Double with
/// NaN and signed zero, computed Strings, `[Int]`, Ordering, a reversed `compare` and derived
/// types; an instance's own operators beside its `compare`; `max` and `min` through the key's
/// `<=`, the instance's own `<=` included, even beside a stub `compare`; `max`, `maximum` and
/// `minimum`, the `maximum` once
/// pinned as row 565's residual; and Ord reached through `Integral`, `RealFrac` and
/// `(Show a, Ord a)`. Main gave wrong answers or failed on each.
const ORD_METHOD_MOVERS_CHIRHO: [(&str, &str, &str); 7] = [
    (
        "OPolymorphicKeysChirho.hs",
        "module Main where\ndata ColorChirho = RedChirho | GreenChirho | BlueChirho deriving (Eq, Ord, Show)\nnewtype RevChirho = RevChirho Int deriving (Eq, Show)\ninstance Ord RevChirho where\n  compare (RevChirho a) (RevChirho b) = compare b a\ndata PairChirho = PairChirho Int String deriving (Eq, Ord, Show)\nopsChirho :: Ord a => a -> a -> (Bool, Bool, Bool, Bool, Ordering)\nopsChirho x y = (x < y, x <= y, x > y, x >= y, compare x y)\nbelowChirho :: Ord a => a -> [a] -> [a]\nbelowChirho t = filter (< t)\natLeastChirho :: Ord a => a -> [a] -> [a]\natLeastChirho t = filter (t <=)\npairwiseChirho :: Ord a => [a] -> [a] -> [Bool]\npairwiseChirho = zipWith (>)\nnanChirho :: Double\nnanChirho = 0 / 0\nmain :: IO ()\nmain = do\n  print (opsChirho (3 :: Int) 2, opsChirho 'a' 'b', opsChirho True False)\n  print (opsChirho (-1.0 :: Double) (-2.0), opsChirho nanChirho 1, opsChirho (-0.0) (0.0 :: Double))\n  print (opsChirho (\"a\" ++ \"b\") \"ab\", opsChirho [1, 2 :: Int] [1, 3], opsChirho LT GT)\n  print (opsChirho (RevChirho 1) (RevChirho 2), opsChirho GreenChirho BlueChirho, opsChirho (PairChirho 1 \"b\") (PairChirho 1 \"a\"))\n  print (belowChirho \"c\" [\"a\", \"d\" ++ \"\"], belowChirho (0.0 :: Double) [-0.0, -1.5, nanChirho], belowChirho (RevChirho 2) [RevChirho 1, RevChirho 3])\n  print (atLeastChirho GreenChirho [RedChirho, BlueChirho], pairwiseChirho [\"b\", \"a\"] [\"a\" ++ \"\", \"b\"], pairwiseChirho [RevChirho 1] [RevChirho 2])\n",
        "((False,False,True,True,GT),(True,True,False,False,LT),(False,False,True,True,GT))\n((False,False,True,True,GT),(False,False,False,False,GT),(False,True,False,True,EQ))\n((False,True,False,True,EQ),(True,True,False,False,LT),(True,True,False,False,LT))\n((False,False,True,True,GT),(True,True,False,False,LT),(False,False,True,True,GT))\n([\"a\"],[-1.5],[RevChirho 3])\n([BlueChirho],[True,False],[True])\n",
    ),
    (
        "OInstanceOperatorsChirho.hs",
        "module Main where\nnewtype TChirho = TChirho Int deriving (Eq, Show)\ninstance Ord TChirho where\n  compare (TChirho a) (TChirho b) = compare a b\n  TChirho a < TChirho b = a > b\n  TChirho a >= TChirho b = a == b\nlessChirho :: Ord a => a -> a -> Bool\nlessChirho x y = x < y\ngeChirho :: Ord a => a -> a -> Bool\ngeChirho = (>=)\ngtChirho :: Ord a => a -> a -> Bool\ngtChirho x y = x > y\nmain :: IO ()\nmain = print (lessChirho (TChirho 1) (TChirho 2), TChirho 1 < TChirho 2, geChirho (TChirho 3) (TChirho 2), TChirho 3 >= TChirho 2, gtChirho (TChirho 3) (TChirho 2), max (TChirho 1) (TChirho 2))\n",
        "(False,False,False,False,True,TChirho 2)\n",
    ),
    (
        "OMaxMinDefaultsChirho.hs",
        "module Main where\nnewtype LeChirho = LeChirho Int deriving (Eq, Show)\ninstance Ord LeChirho where\n  compare (LeChirho a) (LeChirho b) = compare a b\n  LeChirho a <= LeChirho b = a >= b\npickChirho :: Ord a => a -> a -> (a, a)\npickChirho x y = (max x y, min x y)\nnanChirho :: Double\nnanChirho = 0 / 0\nmain :: IO ()\nmain = do\n  print (pickChirho (3 :: Int) 5, pickChirho 'z' 'a', pickChirho False True, pickChirho \"pear\" (\"app\" ++ \"le\"))\n  print (pickChirho nanChirho 1, pickChirho 1 nanChirho, pickChirho (-0.0) (0.0 :: Double), pickChirho 0.0 (-0.0 :: Double))\n  print (pickChirho (LeChirho 1) (LeChirho 2), pickChirho [2, 1 :: Int] [2], pickChirho GT EQ)\n",
        "((5,3),('z','a'),(True,False),(\"pear\",\"apple\"))\n((NaN,1.0),(1.0,NaN),(0.0,-0.0),(-0.0,0.0))\n((LeChirho 1,LeChirho 2),([2,1],[2]),(GT,EQ))\n",
    ),
    (
        "OLeOnlyExtremaChirho.hs",
        "module Main where\nnewtype VChirho = VChirho Int deriving (Eq, Show)\ninstance Ord VChirho where\n  VChirho a <= VChirho b = a >= b\npickChirho :: Ord a => a -> a -> (a, a)\npickChirho x y = (max x y, min x y)\nmain :: IO ()\nmain = print (pickChirho (VChirho 1) (VChirho 2), max (VChirho 1) (VChirho 2))\n",
        "((VChirho 1,VChirho 2),VChirho 1)\n",
    ),
    (
        "OPolymorphicExtremaChirho.hs",
        "module Main where\nnewtype RevChirho = RevChirho Int deriving (Eq, Show)\ninstance Ord RevChirho where\n  compare (RevChirho a) (RevChirho b) = compare b a\nbiggerChirho :: Ord a => a -> a -> a\nbiggerChirho = max\nsmallestChirho :: Ord a => [a] -> a\nsmallestChirho = minimum\nmain :: IO ()\nmain = print (biggerChirho (RevChirho 1) (RevChirho 2), smallestChirho [RevChirho 1, RevChirho 5], biggerChirho \"b\" (\"a\" ++ \"\"), smallestChirho [2.5, -1.0 :: Double])\n",
        "(RevChirho 1,RevChirho 5,\"b\",-1.0)\n",
    ),
    (
        "PolymorphicMaximumChirho.hs",
        "module Main where\nlargestChirho :: Ord a => [a] -> a\nlargestChirho = maximum\nmain :: IO ()\nmain = print (largestChirho [\"b\", \"c\", \"a\"])\n",
        "\"c\"\n",
    ),
    (
        "OSuperclassOrdChirho.hs",
        "module Main where\nclampChirho :: Integral a => a -> a -> a\nclampChirho lim x = if x > lim then lim else min x (lim - 1)\natMostChirho :: RealFrac a => a -> a -> Bool\natMostChirho x y = x <= y\nlargerChirho :: (Show a, Ord a) => a -> a -> a\nlargerChirho x y = if x == y then x else max x y\nmain :: IO ()\nmain = print (clampChirho (10 :: Int) 12, clampChirho (10 :: Int) 3, atMostChirho (2.5 :: Double) 2.5, largerChirho \"b\" \"a\", largerChirho (2 :: Int) 2)\n",
        "(10,3,True,\"b\",2)\n",
    ),
];

/// `sort`, `insert`, `maximum` and `sortBy compare` at Int and Char, and the operators, extrema
/// and `compare` at concrete keys. Each matched GHC on main.
const ORD_METHOD_HOLDS_CHIRHO: [(&str, &str, &str); 1] = [(
    "OOrdHoldsChirho.hs",
    "module Main where\nimport Data.List (sort, sortBy, insert)\nmain :: IO ()\nmain = do\n  print (sort [3, 1, 2 :: Int], sort \"banana\", insert 2 [1, 3 :: Int], maximum \"hello\", sortBy compare [3, 1, 2 :: Int])\n  print (3 < (4 :: Int), 'a' >= 'b', \"ab\" < \"b\", [1, 2] <= [1 :: Int], max \"a\" \"b\", min 2.5 (1.5 :: Double), compare (1 :: Int, 'a') (1, 'b'))\n",
    "([1,2,3],\"aaabnn\",[1,2,3],'o',[1,2,3])\n(True,False,True,False,\"b\",1.5,LT)\n",
)];

#[test]
fn ord_methods_default_from_compare_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in ORD_METHOD_MOVERS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn ord_method_holds_stay_ghc_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in ORD_METHOD_HOLDS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

/// An instance with no `compare` has no real body to default from. `compare` keeps its named
/// missing-method failure, and a polymorphic `<` now fails by name too, where the natives answered
/// True from an Int primop. GHC's defaults call each other, and the program never ends.
#[test]
fn an_instance_without_compare_fails_by_name_everywhere_chirho() {
    assert_named_failure_everywhere_chirho(
        "ONeitherLessChirho.hs",
        "module Main where\ndata TwoChirho = OneChirho | TwoChirho deriving (Eq, Show)\ninstance Ord TwoChirho\nlessChirho :: Ord a => a -> a -> Bool\nlessChirho x y = x < y\nmain :: IO ()\nmain = print (lessChirho OneChirho TwoChirho)\n",
        "missing method Ord.< for TwoChirho",
    );
    assert_named_failure_everywhere_chirho(
        "ONeitherCompareChirho.hs",
        "module Main where\ndata TwoChirho = OneChirho | TwoChirho deriving (Eq, Show)\ninstance Ord TwoChirho\nmain :: IO ()\nmain = print (compare OneChirho TwoChirho)\n",
        "missing method Ord.compare for TwoChirho",
    );
}
