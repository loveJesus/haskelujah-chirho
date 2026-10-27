// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `max`, `min`, `maximum` and `minimum` read Ord's methods at every served key (row 627). GHC.List
//! folds `maximum` and `minimum` with `max` and `min`, and an instance may write either method.
//! Row 565's compare-backed helpers decided every concrete key, so an instance's own `max` or
//! `min`, and an own `<=` that disagrees with `compare`, were ignored there and by the folds.
//! Every program below is GHC 9.14.1's, byte for byte as measured.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::assert_ghc_everywhere_chirho;

/// The measured ordinary-`compare`/reversed-`<=` program, whose concrete `max` was row 626's
/// last residual; an instance's own `max` at a concrete key and through `maximum`; an
/// instance's own `min` at a concrete key, through `minimum` in and outside `Ord a =>` code and
/// through `zipWith min`; and an own `max` through polymorphic `maximum`, a section and
/// `foldr1 max`. Main gave a wrong answer on each.
const EXTREMA_METHOD_MOVERS_CHIRHO: [(&str, &str, &str); 4] = [
    (
        "XMixedDefaultsChirho.hs",
        "module Main where\nnewtype MixChirho = MixChirho Int deriving (Eq, Show)\ninstance Ord MixChirho where\n  compare (MixChirho a) (MixChirho b) = compare a b\n  MixChirho a <= MixChirho b = a >= b\nlessChirho :: Ord a => a -> a -> Bool\nlessChirho x y = x < y\nmain :: IO ()\nmain = do\n  print (MixChirho 1 < MixChirho 2, MixChirho 1 > MixChirho 2, MixChirho 1 >= MixChirho 2, compare (MixChirho 1) (MixChirho 2))\n  print (lessChirho (MixChirho 1) (MixChirho 2), max (MixChirho 1) (MixChirho 2))\n",
        "(False,True,True,LT)\n(False,MixChirho 1)\n",
    ),
    (
        "XOwnMaxChirho.hs",
        "module Main where\nnewtype WChirho = WChirho Int deriving (Eq, Show)\ninstance Ord WChirho where\n  compare (WChirho a) (WChirho b) = compare a b\n  max _ _ = WChirho 0\nmain :: IO ()\nmain = print (max (WChirho 1) (WChirho 2), min (WChirho 1) (WChirho 2), maximum [WChirho 1, WChirho 5])\n",
        "(WChirho 0,WChirho 1,WChirho 0)\n",
    ),
    (
        "XOwnMinChirho.hs",
        "module Main where\nnewtype UChirho = UChirho Int deriving (Eq, Show)\ninstance Ord UChirho where\n  compare (UChirho a) (UChirho b) = compare a b\n  min _ _ = UChirho 7\nsmallestChirho :: Ord a => [a] -> a\nsmallestChirho = minimum\nmain :: IO ()\nmain = print (min (UChirho 1) (UChirho 2), max (UChirho 1) (UChirho 2), minimum [UChirho 3, UChirho 4], smallestChirho [UChirho 5, UChirho 6], zipWith min [UChirho 1] [UChirho 2])\n",
        "(UChirho 7,UChirho 2,UChirho 7,UChirho 7,[UChirho 7])\n",
    ),
    (
        "XPolymorphicOverrideChirho.hs",
        "module Main where\nnewtype WChirho = WChirho Int deriving (Eq, Show)\ninstance Ord WChirho where\n  compare (WChirho a) (WChirho b) = compare a b\n  max _ _ = WChirho 0\nlargestChirho :: Ord a => [a] -> a\nlargestChirho = maximum\nbiggerChirho :: Ord a => a -> a -> a\nbiggerChirho = max\nmain :: IO ()\nmain = print (largestChirho [WChirho 1, WChirho 5], biggerChirho (WChirho 1) (WChirho 2), map (max (WChirho 3)) [WChirho 4], foldr1 max [WChirho 8, WChirho 9])\n",
        "(WChirho 0,WChirho 0,[WChirho 0],WChirho 0)\n",
    ),
];

/// `max`, `min`, `maximum` and `minimum` at Int, Char, Bool, Strings and a derived type; at
/// Double with NaN and signed zero; at lists, `Maybe` and tuples, which have no dictionary; and
/// first-class and partially applied. Each matched GHC on main.
const EXTREMA_METHOD_HOLDS_CHIRHO: [(&str, &str, &str); 1] = [(
    "XExtremaHoldsChirho.hs",
    "module Main where\ndata SizeChirho = SmallChirho | LargeChirho deriving (Eq, Ord, Show)\nnanChirho :: Double\nnanChirho = 0 / 0\nmain :: IO ()\nmain = do\n  print (max (3 :: Int) 5, min 'z' 'a', maximum [True, False], minimum \"hello\", max \"pear\" (\"app\" ++ \"le\"), maximum [LargeChirho, SmallChirho])\n  print (maximum [1, nanChirho, 2], minimum [nanChirho, 1], max nanChirho 1, min 1 nanChirho, maximum [-0.0, 0.0 :: Double])\n  print (max [2, 1 :: Int] [2], minimum [[3], [1, 2 :: Int]], max (Just 'a') Nothing, maximum [(1 :: Int, 'a'), (1, 'b')], minimum [Just (3 :: Int), Nothing])\n  print (zipWith max [1, 5 :: Int] [4, 2], map (min 'm') \"az\", foldr1 min [LargeChirho, SmallChirho])\n",
    "(5,'a',True,'e',\"pear\",LargeChirho)\n(2.0,1.0,NaN,NaN,0.0)\n([2,1],[1,2],Just 'a',(1,'b'),Nothing)\n([4,5],\"am\",SmallChirho)\n",
)];

#[test]
fn extrema_follow_ord_methods_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in EXTREMA_METHOD_MOVERS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn extrema_method_holds_stay_ghc_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in EXTREMA_METHOD_HOLDS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}
