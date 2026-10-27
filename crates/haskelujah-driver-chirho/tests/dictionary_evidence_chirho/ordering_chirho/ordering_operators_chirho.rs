// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `<`, `<=`, `>` and `>=` at the key the checker proved (row 610). Desugaring lowered
//! written infix and section Ord operators to Int primops before any type was known, so
//! computed Strings, derived values and lambda-bound Doubles were ordered as words or bits,
//! and a program's own or local `(<)` lost to the primop. A first-class `(<)` had no binding
//! at all. Every program below is GHC 9.14.1's, byte for byte as measured.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_ghc_everywhere_chirho, assert_named_failure_everywhere_chirho};

/// Written operators at computed Strings, NaN and signed-zero Doubles, a derived enum, a
/// program's own `compare`, derived fields, a newtype, `Maybe`, lists and tuples, in
/// infix, both section directions and lambdas. Main gave wrong answers or failed on each.
const MOVERS_CHIRHO: [(&str, &str, &str); 7] = [
    (
        "OStringSectionsChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (filter (< \"b\") [\"a\" ++ \"\", \"c\"], filter (\"b\" <) [\"a\", \"c\" ++ \"\"], filter (>= \"ab\") [\"a\" ++ \"b\", \"aa\"], filter (\"ab\" >) [\"a\" ++ \"a\", \"b\"], filter (<= \"b\") [\"b\" ++ \"\", \"c\"])\n",
        "([\"a\"],[\"c\"],[\"ab\"],[\"aa\"],[\"b\"])\n",
    ),
    (
        "OStringInfixChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (filter (\\s -> s < \"b\") [\"a\" ++ \"\", \"c\"], filter (\\s -> \"b\" > s) [\"a\" ++ \"\", \"c\"], [s | s <- [\"a\" ++ \"b\", \"b\"], s >= \"ab\"], zipWith (<) [\"a\" ++ \"\", \"c\"] [\"b\", \"b\"])\n",
        "([\"a\"],[\"a\"],[\"ab\",\"b\"],[True,False])\n",
    ),
    (
        "ODoubleNanZeroChirho.hs",
        "module Main where\nnanChirho :: Double\nnanChirho = 0 / 0\nmain :: IO ()\nmain = do\n  print (filter (< nanChirho) [1.0], filter (nanChirho >) [1.0], filter (> nanChirho) [1.0], filter (nanChirho <=) [1.0])\n  print (filter (>= 0.0) [-0.0 :: Double], filter (0.0 <=) [-0.0 :: Double], filter (\\d -> d < nanChirho) [1.0], filter (\\d -> d >= 0.0) [-0.0 :: Double])\n  print (zipWith (>=) [nanChirho, -0.0] [1.0, 0.0], filter (\\d -> d < -1.0) [-0.5, -1.5 :: Double])\n",
        "([],[],[],[])\n([-0.0],[-0.0],[],[-0.0])\n([False,True],[-1.5])\n",
    ),
    (
        "OCustomOrdChirho.hs",
        "module Main where\ndata SizeChirho = SmallChirho | LargeChirho deriving (Eq, Ord, Show)\nnewtype RevChirho = RevChirho Int deriving (Eq, Show)\ninstance Ord RevChirho where\n  compare (RevChirho a) (RevChirho b) = compare b a\nmain :: IO ()\nmain = print (filter (< LargeChirho) [SmallChirho, LargeChirho], filter (\\r -> r < RevChirho 2) [RevChirho 1, RevChirho 3], RevChirho 1 > RevChirho 3, filter (RevChirho 2 <=) [RevChirho 1, RevChirho 3])\n",
        "([SmallChirho],[RevChirho 3],True,[RevChirho 1])\n",
    ),
    (
        "KDerivedFieldsChirho.hs",
        "module Main where\ndata PointChirho = PointChirho Int Int deriving (Eq, Ord, Show)\nnewtype AgeChirho = AgeChirho Int deriving (Eq, Ord, Show)\nmain :: IO ()\nmain = print (PointChirho 1 2 < PointChirho 1 3, filter (>= PointChirho 1 0) [PointChirho 0 9, PointChirho 1 1], AgeChirho 3 < AgeChirho 4, filter (> AgeChirho 3) [AgeChirho 1, AgeChirho 5])\n",
        "(True,[PointChirho 1 1],True,[AgeChirho 5])\n",
    ),
    (
        "KMaybeListChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (Just 3 < Just (4 :: Int), Nothing < Just (1 :: Int), [1, 2] < [1, 3 :: Int], filter (< [2]) [[1], [3 :: Int]], filter (\\m -> m >= Just 'b') [Just 'a', Nothing, Just 'c'])\n",
        "(True,True,True,[[1]],[Just 'c'])\n",
    ),
    (
        "KTupleChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ((1, 'a') < (1 :: Int, 'b'), filter (>= (1 :: Int, 'b')) [(1, 'a'), (2, 'a')], filter (\\p -> p > (\"a\", 2 :: Int)) [(\"a\" ++ \"\", 3), (\"a\", 1)])\n",
        "(True,[(2,'a')],[(\"a\",3)])\n",
    ),
];

/// The operator the program wrote keeps its binding: a hidden-and-redefined top-level
/// `(<)`, a `where`-bound `(<=)`, and an instance's own `(<)` beside its `compare`.
const OWN_OPERATORS_CHIRHO: [(&str, &str, &str); 3] = [
    (
        "OProgramOwnedChirho.hs",
        "module Main where\nimport Prelude hiding ((<))\n(<) :: Int -> Int -> Bool\na < b = a > b\nmain :: IO ()\nmain = print (filter (< 3) [1, 5 :: Int], filter (3 <) [1, 5 :: Int], 4 < 2, filter (\\x -> x < 3) [1, 5 :: Int])\n",
        "([5],[1],True,[5])\n",
    ),
    (
        "OLocalOperatorChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (filter (<= 2) [1, 2, 3 :: Int], [x | x <- \"abc\", x > 'a'])\n  where\n    a <= b = a >= (b :: Int)\n",
        "([2,3],\"bc\")\n",
    ),
    (
        "KOwnLessChirho.hs",
        "module Main where\nnewtype WChirho = WChirho Int deriving (Eq, Show)\ninstance Ord WChirho where\n  compare (WChirho a) (WChirho b) = compare a b\n  WChirho a < WChirho b = a > b\nmain :: IO ()\nmain = print (WChirho 1 < WChirho 2, compare (WChirho 1) (WChirho 2), WChirho 1 > WChirho 2, filter (< WChirho 2) [WChirho 1, WChirho 3], zipWith (<) [WChirho 1] [WChirho 2])\n",
        "(False,LT,False,[WChirho 3],[False])\n",
    ),
];

/// First-class and prefix uses at String, Double with a NaN, Ordering and Int, which had
/// no binding on main.
const FIRST_CLASS_CHIRHO: [(&str, &str, &str); 2] = [
    (
        "OFirstClassChirho.hs",
        "module Main where\nnanChirho :: Double\nnanChirho = 0 / 0\nmain :: IO ()\nmain = print ((<) (\"a\" ++ \"\") \"b\", map ((<=) \"b\") [\"a\", \"b\" ++ \"\", \"c\"], zipWith (>) [1.0, nanChirho] [0.5, 1.0], map (uncurry (>=)) [(LT, GT), (GT, EQ)], zipWith (<) [2, 1 :: Int] [1, 2])\n",
        "(True,[False,True,True],[True,False],[False,True],[False,True])\n",
    ),
    (
        "OOrderingKeyChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (LT < GT, filter (> EQ) [LT, EQ, GT], GT >= LT, filter (\\o -> o < EQ) [GT, LT], (<=) EQ EQ)\n",
        "(True,[GT],True,[LT],True)\n",
    ),
];

/// Int, Char and Bool primops; `compare`, extrema and structured Ord; NaN on each side of
/// each operator and the signed-zero matrix at annotated Doubles; Integer. Each matched GHC
/// on main and still does.
const HOLDS_CHIRHO: [(&str, &str, &str); 3] = [
    (
        "HOrdHoldsChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (filter (< 3) [1, 5 :: Int], filter ('b' <=) \"abc\", filter (> False) [False, True], (2 :: Int) < 3, compare \"a\" \"b\", maximum [\"a\" ++ \"\", \"c\"], max (1 :: Int, 'b') (1, 'a'), (1 :: Int, 2 :: Int, 3 :: Int, 4 :: Int) < (1, 2, 3, 5))\n",
        "([1],\"bc\",[True],True,LT,\"c\",(1,'b'),True)\n",
    ),
    (
        "ODoubleSidesChirho.hs",
        "module Main where\nnanChirho :: Double\nnanChirho = 0 / 0\nzeroesChirho :: [Double]\nzeroesChirho = [-0.0, 0.0]\nmain :: IO ()\nmain = do\n  print (nanChirho < 1, 1 < nanChirho, nanChirho <= 1, 1 <= nanChirho, nanChirho > 1, 1 > nanChirho, nanChirho >= 1, 1 >= nanChirho)\n  print (map (< nanChirho) [1], map (nanChirho <) [1], map (<= nanChirho) [1], map (nanChirho <=) [1], map (> nanChirho) [1], map (nanChirho >) [1], map (>= nanChirho) [1], map (nanChirho >=) [1])\n  print [(a < b, a <= b, a > b, a >= b) | a <- zeroesChirho, b <- zeroesChirho]\n  print (map (\\d -> d > nanChirho) [1], map (\\d -> nanChirho <= d) [1], map (\\d -> d <= 0.0) [-0.0], map (\\d -> 0.0 > d) [-0.0])\n",
        "(False,False,False,False,False,False,False,False)\n([False],[False],[False],[False],[False],[False],[False],[False])\n[(False,True,False,True),(False,True,False,True),(False,True,False,True),(False,True,False,True)]\n([False],[False],[True],[False])\n",
    ),
    (
        "KIntegerChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ((3 :: Integer) < 5, filter (< (3 :: Integer)) [1, 5, 2], (7 :: Integer) >= 7, filter (\\n -> n > (2 :: Integer)) [1, 3])\n",
        "(True,[1,2],True,[3])\n",
    ),
];

#[test]
fn written_ordering_operators_take_the_proved_key_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in MOVERS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn a_programs_own_ordering_operator_keeps_its_binding_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in OWN_OPERATORS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn first_class_ordering_operators_take_their_rows_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in FIRST_CLASS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn ordering_holds_stay_ghc_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in HOLDS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

/// An instance with neither `compare` nor `<=` leaves missing-method stubs as both rows. A
/// first-class `(<)` there stays loud, naming its own dictionary slot's stub; GHC's defaults call
/// each other forever. Its twin that writes `<=` answers GHC's `[False]` since row 626
/// (`ord_chain_chirho.rs`).
#[test]
fn a_first_class_operator_at_a_stub_compare_key_stays_loud_chirho() {
    assert_named_failure_everywhere_chirho(
        "LNeitherFirstClassChirho.hs",
        "module Main where\nnewtype VChirho = VChirho Int deriving (Eq, Show)\ninstance Ord VChirho\nmain :: IO ()\nmain = print (zipWith (<) [VChirho 1] [VChirho 2])\n",
        "missing method Ord.< for VChirho",
    );
}
