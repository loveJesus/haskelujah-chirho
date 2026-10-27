// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC's `<=` chain (row 626). GHC.Classes defines `x < y = not (y <= x)`, `x > y = not (x <= y)`
//! and `x >= y = y <= x`, `<=` itself from `compare`, and `compare` from `==` and `<=`. Rows 610
//! and 612 read `compare` for every operator, so an instance whose own `<=` disagrees with its
//! `compare`, or whose `compare` is not antisymmetric, answered differently, and an instance
//! that writes only `<=` failed with `missing method Ord.compare`. Every program below is GHC
//! 9.14.1's, byte for byte as measured.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::assert_ghc_everywhere_chirho;

/// An ordinary `compare` beside a reversed `<=`, written, sectioned, first-class and inside
/// `Ord a =>` code; an instance that writes only `<=`, directly and through `Ord a =>` code,
/// `sortBy compare`, `maximum`, `minimum` and first-class `(<)`, the last once row 610's
/// stub-key pin; a `compare` that always answers LT; and `compare`'s default through the
/// instance's own `==`. Main gave wrong answers, or failed by a missing method, on each.
const ORD_CHAIN_MOVERS_CHIRHO: [(&str, &str, &str); 6] = [
    (
        "LMixedChainChirho.hs",
        "module Main where\nnewtype MixChirho = MixChirho Int deriving (Eq, Show)\ninstance Ord MixChirho where\n  compare (MixChirho a) (MixChirho b) = compare a b\n  MixChirho a <= MixChirho b = a >= b\nlessChirho :: Ord a => a -> a -> Bool\nlessChirho x y = x < y\nbiggerChirho :: Ord a => a -> a -> a\nbiggerChirho = max\nmain :: IO ()\nmain = do\n  print (MixChirho 1 < MixChirho 2, MixChirho 1 > MixChirho 2, MixChirho 1 >= MixChirho 2, compare (MixChirho 1) (MixChirho 2))\n  print (lessChirho (MixChirho 1) (MixChirho 2), biggerChirho (MixChirho 1) (MixChirho 2), filter (> MixChirho 2) [MixChirho 1, MixChirho 3], zipWith (>=) [MixChirho 1] [MixChirho 2])\n",
        "(False,True,True,LT)\n(False,MixChirho 1,[MixChirho 1],[True])\n",
    ),
    (
        "LLeOnlyCoreChirho.hs",
        "module Main where\nnewtype VChirho = VChirho Int deriving (Eq, Show)\ninstance Ord VChirho where\n  VChirho a <= VChirho b = a >= b\nmain :: IO ()\nmain = print (VChirho 1 < VChirho 2, VChirho 1 > VChirho 2, compare (VChirho 1) (VChirho 2), max (VChirho 1) (VChirho 2), VChirho 2 >= VChirho 2)\n",
        "(False,True,GT,VChirho 1,True)\n",
    ),
    (
        "LLeOnlyPolymorphicChirho.hs",
        "module Main where\nimport Data.List (sortBy)\nnewtype DescChirho = DescChirho Int deriving (Eq, Show)\ninstance Ord DescChirho where\n  DescChirho a <= DescChirho b = a >= b\nopsChirho :: Ord a => a -> a -> (Bool, Bool, Bool, Bool, Ordering)\nopsChirho x y = (x < y, x <= y, x > y, x >= y, compare x y)\nmain :: IO ()\nmain = do\n  print (opsChirho (DescChirho 1) (DescChirho 2), opsChirho (DescChirho 2) (DescChirho 2))\n  print (sortBy compare [DescChirho 2, DescChirho 5, DescChirho 3], maximum [DescChirho 4, DescChirho 1, DescChirho 9], minimum [DescChirho 4, DescChirho 1, DescChirho 9])\n  print (filter (< DescChirho 2) [DescChirho 1, DescChirho 3], zipWith compare [DescChirho 1] [DescChirho 2])\n",
        "((False,False,True,True,GT),(False,True,False,True,EQ))\n([DescChirho 5,DescChirho 3,DescChirho 2],DescChirho 1,DescChirho 9)\n([DescChirho 3],[GT])\n",
    ),
    (
        "LFirstClassLeOnlyChirho.hs",
        "module Main where\nnewtype VChirho = VChirho Int deriving (Eq, Show)\ninstance Ord VChirho where\n  VChirho a <= VChirho b = a >= b\nmain :: IO ()\nmain = print (zipWith (<) [VChirho 1] [VChirho 2])\n",
        "[False]\n",
    ),
    (
        "LNonAntisymmetricChirho.hs",
        "module Main where\ndata FlatChirho = FlatChirho Int deriving (Eq, Show)\ninstance Ord FlatChirho where\n  compare _ _ = LT\nopsChirho :: Ord a => a -> a -> (Bool, Bool, Bool, Bool)\nopsChirho x y = (x < y, x <= y, x > y, x >= y)\nmain :: IO ()\nmain = print (opsChirho (FlatChirho 1) (FlatChirho 2), FlatChirho 1 < FlatChirho 2, FlatChirho 1 >= FlatChirho 2)\n",
        "((False,True,False,True),False,True)\n",
    ),
    (
        "LOwnEqualityChirho.hs",
        "module Main where\nnewtype ModChirho = ModChirho Int deriving Show\ninstance Eq ModChirho where\n  ModChirho a == ModChirho b = a `mod` 3 == b `mod` 3\ninstance Ord ModChirho where\n  ModChirho a <= ModChirho b = a `mod` 3 <= b `mod` 3\nmain :: IO ()\nmain = print (compare (ModChirho 1) (ModChirho 4), compare (ModChirho 2) (ModChirho 4), ModChirho 5 > ModChirho 3)\n",
        "(EQ,GT,True)\n",
    ),
];

/// An instance that writes only `<`: its own `<` answers written and through `Ord a =>` code,
/// as it did since row 612.
const ORD_CHAIN_HOLDS_CHIRHO: [(&str, &str, &str); 1] = [(
    "LOnlyLessChirho.hs",
    "module Main where\nnewtype LtChirho = LtChirho Int deriving (Eq, Show)\ninstance Ord LtChirho where\n  LtChirho a < LtChirho b = a > b\nlessChirho :: Ord a => a -> a -> Bool\nlessChirho x y = x < y\nmain :: IO ()\nmain = print (LtChirho 1 < LtChirho 2, lessChirho (LtChirho 2) (LtChirho 1))\n",
    "(False,True)\n",
)];

#[test]
fn ordering_operators_follow_ghcs_at_most_chain_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in ORD_CHAIN_MOVERS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn ord_chain_holds_stay_ghc_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in ORD_CHAIN_HOLDS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}
