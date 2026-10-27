// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Operator sections take their written operator's evidence (row 609). Desugaring lowered
//! a section of `+`, `-` or `*` to the Int primop, so `(+ 0.5)` added a Double's bits as an
//! integer natively, and a program's own `(+)` lost to the primop. Fractional's `/` minted
//! no occurrence, so `(/ 2)` had no binding at all. An operand-first section of a
//! qualified operator, `(2 P.-)`, was lowered as one unqualified `P.-` and applied as a
//! constructor. Every program below is GHC 9.14.1's, byte for byte as measured.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_named_failure_everywhere_chirho, assert_native_chirho, run_chirho};

/// Double `+`, `-`, `*` and `/` sections in both directions, a program's own `(+)`, the NaN
/// and signed-zero comparisons a section of `0 / 0` reaches, and concrete Double `/` infix,
/// first-class and partially applied. Main gave wrong answers or failed on each.
const SECTION_MOVERS_CHIRHO: [(&str, &str, &str); 4] = [
    (
        "MDoubleArithChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print (map (+ 0.5) [1 :: Double], map (0.5 +) [1 :: Double], map (* 2) [1.5 :: Double], map (2 *) [1.5 :: Double])\n  print (map (2.0 -) [0.5 :: Double], map (/ 2) [3.0 :: Double], map (6 /) [2.0 :: Double], map (+ (1 / 2)) [1 :: Double])\n",
        "([1.5],[1.5],[3.0],[3.0])\n([1.5],[1.5],[3.0],[1.5])\n",
    ),
    (
        "HProgramOwnedChirho.hs",
        "module Main where\nimport Prelude hiding ((+))\n(+) :: Int -> Int -> Int\na + b = a * b\nmain :: IO ()\nmain = print (map (+ 2) [3 :: Int], map (2 +) [5 :: Int], 4 + 5)\n",
        "([6],[10],20)\n",
    ),
    (
        "MDoubleCompareNegativeChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print (filter (< -1.0) [-0.5, -1.5 :: Double], filter (> -1.0) [-0.5, -1.5 :: Double], filter (-1.0 >=) [-0.5, -1.5 :: Double])\n  print (filter (< (0 / 0)) [1.0 :: Double], filter (>= 0.0) [-0.0 :: Double], filter (<= 2.5) [-3.5, 2.5 :: Double])\n",
        "([-1.5],[-0.5],[-1.5])\n([],[-0.0],[-3.5,2.5])\n",
    ),
    (
        "FDivideConcreteChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ((3.0 :: Double) / 2, map (/ 4) [1.0 :: Double], map (1 /) [4.0 :: Double], zipWith (/) [3.0, 1.0] [2.0, 4.0 :: Double], foldr (/) 1 [2.0, 4.0 :: Double], map ((/) 1) [8.0 :: Double])\n",
        "(1.5,[0.25],[0.25],[1.5,0.25],0.5,[0.125])\n",
    ),
];

/// Qualified `+`, `-`, `*` and `/` sections, operator-first and operand-first, at Int and
/// Double, nested ones included.
const QUALIFIED_SECTIONS_CHIRHO: [(&str, &str, &str); 3] = [
    (
        "QSectionsIntChirho.hs",
        "module Main where\nimport qualified Prelude as P\nimport Prelude (IO, Int, print, map)\nmain :: IO ()\nmain = print (map (P.+ 1) [1 :: Int], map (1 P.+) [1 :: Int], map (P.- 1) [5 :: Int], map (10 P.-) [3 :: Int], map (P.* 3) [2 :: Int], map (3 P.*) [2 :: Int], map (1 P.+ 2 P.-) [5 :: Int])\n",
        "([2],[2],[4],[7],[6],[6],[-2])\n",
    ),
    (
        "QSectionsDoubleChirho.hs",
        "module Main where\nimport qualified Prelude as P\nimport Prelude (IO, Double, print, map)\nmain :: IO ()\nmain = print (map (P.+ 0.5) [1 :: Double], map (0.5 P.+) [1 :: Double], map (P.- 0.5) [2 :: Double], map (2.0 P.-) [0.5 :: Double], map (P.* 2) [1.5 :: Double], map (2 P.*) [1.5 :: Double], map (P./ 2) [3.0 :: Double], map (6 P./) [2.0 :: Double], map (1 P.+ 2 P.-) [0.5 :: Double])\n",
        "([1.5],[1.5],[1.5],[1.5],[3.0],[3.0],[1.5],[3.0],[2.5])\n",
    ),
    (
        "QPreludeSectionsChirho.hs",
        "module Main where\nimport qualified Prelude as P\nimport Prelude (IO, Double, Int, print, map, filter, (++))\nmain :: IO ()\nmain = print (map (P.+ 1) [1 :: Int], map (P.* 2) [1.5 :: Double], map (P./ 2) [3.0 :: Double], filter (P.< \"b\") [\"a\" ++ \"\", \"c\"], map (2 P.-) [0.5 :: Double])\n",
        "([2],[3.0],[1.5],[\"a\"],[1.5])\n",
    ),
];

/// R1's `==` sections, Int and Char sections, `++`, `^` and `**`, row 610's comparison
/// sections, a dictionary-polymorphic Fractional and a program's own `(/)`: each matched GHC
/// on main and still does.
const SECTION_HOLDS_CHIRHO: [(&str, &str, &str); 7] = [
    (
        "HEqSectionChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (filter (== [1]) [[1], [2 :: Int]], filter (\"ab\" ==) [\"a\" ++ \"b\", \"c\"], filter (== 2.5) [2.5 :: Double])\n",
        "([[1]],[\"ab\"],[2.5])\n",
    ),
    (
        "HIntCharChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (map (+ 1) [1 :: Int], map (10 -) [3 :: Int], map (* 3) [2 :: Int], filter (< 3) [1, 5 :: Int], filter (> 'b') \"abc\", filter ('b' <=) \"abc\")\n",
        "([2],[7],[6],[1],\"c\",\"bc\")\n",
    ),
    (
        "HNonClassChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (map (++ \"!\") [\"a\"], map (\"<\" ++) [\"b\"], map (^ 2) [3 :: Int], map (2 ^) [3 :: Int], map (** 2) [3.0 :: Double])\n",
        "([\"a!\"],[\"<b\"],[9],[8],[9.0])\n",
    ),
    (
        "MStringCompareChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (filter (< \"b\") [\"a\" ++ \"\", \"c\"], filter (\"b\" <) [\"a\", \"c\" ++ \"\"], filter (>= \"ab\") [\"a\" ++ \"b\", \"aa\"], filter (\"ab\" >) [\"a\" ++ \"a\", \"b\"])\n",
        "([\"a\"],[\"c\"],[\"ab\"],[\"aa\"])\n",
    ),
    (
        "MDoubleCompareChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print (filter (< 2.5) [1.5, 3.5 :: Double], filter (2.5 <) [1.5, 3.5 :: Double], filter (>= 2.0) [1.5, 2.0 :: Double])\n  print (filter (<= 1.5) [1.5, 3.5 :: Double], filter (> 1.0) [0.5, 1.5 :: Double], filter (2.0 >=) [1.5, 2.5 :: Double])\n",
        "([1.5],[3.5],[2.0])\n([1.5],[1.5],[1.5])\n",
    ),
    (
        "FDividePolymorphicChirho.hs",
        "module Main where\nhalfChirho :: Fractional a => a -> a\nhalfChirho x = x / 2\nscaleChirho :: Fractional a => [a] -> [a]\nscaleChirho = map (/ 4)\nmeanChirho :: Fractional a => [a] -> a\nmeanChirho xs = sum xs / fromIntegral (length xs)\nmain :: IO ()\nmain = print (halfChirho (3 :: Double), scaleChirho [2.0 :: Double], meanChirho [1.0, 2.0, 4.5 :: Double])\n",
        "(1.5,[0.5],2.5)\n",
    ),
    (
        "FDivideProgramOwnedChirho.hs",
        "module Main where\nimport Prelude hiding ((/))\n(/) :: Int -> Int -> Int\na / b = a * b\nmain :: IO ()\nmain = print (6 / 3, map (/ 2) [5 :: Int], map (2 /) [5 :: Int], zipWith (/) [2] [7 :: Int])\n",
        "(18,[10],[10],[14])\n",
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
fn operator_sections_take_their_evidence_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in SECTION_MOVERS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn qualified_operator_sections_answer_as_ghc_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in QUALIFIED_SECTIONS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn section_holds_stay_ghc_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in SECTION_HOLDS_CHIRHO {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

/// Float has no `/` body, so a forced `/` at Float names its missing method on every engine,
/// as the stub row does; main failed on `missing binding /`. GHC prints `1`.
#[test]
fn division_without_a_body_fails_closed_everywhere_chirho() {
    assert_named_failure_everywhere_chirho(
        "FDivideFloatForcedChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (map (/ 2) [1.5 :: Float] `seq` ((1.5 :: Float) / 2) `seq` (1 :: Int))\n",
        "missing method Fractional./ for Float",
    );
}

/// `subtract 1` is not a section, and stays outside row 609 as a named residual. GHC prints
/// `([1.5],[4])`.
#[test]
fn subtract_stays_the_named_residual_everywhere_chirho() {
    assert_named_failure_everywhere_chirho(
        "HSubtractChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (map (subtract 1) [2.5 :: Double], map (subtract 1) [5 :: Int])\n",
        "no matching alternative",
    );
}
