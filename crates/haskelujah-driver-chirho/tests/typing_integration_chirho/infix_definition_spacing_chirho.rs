// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! GHC's whitespace rule for `!` and `-` in a definition (row 617). After a pattern, only a
//! prefix occurrence, loose on the left and tight on the right, starts an argument pattern: a bang
//! pattern, or a negative literal. Every other `!` or `-` is the operator an infix definition
//! defines. Before this, a `!` or `-` after a name head or a constructor's arguments was always an
//! argument pattern: `x ! y = ...` defined `x`, and `V a - V b = ...` left `-` undefined. Every
//! program here is GHC 9.14.1's, byte for byte as measured.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::assert_prints_everywhere_chirho;

/// A `!` that is not a prefix occurrence defines the operator: loose, tight, suffix, after a
/// constructor pattern, and in a `where` clause.
const BANG_DEFINITIONS_CHIRHO: [(&str, &str, &str); 5] = [
    (
        "SLooseBangChirho.hs",
        "module Main where\ninfixl 7 !\n(!) :: Int -> Int -> Int\nx ! y = x * 10 + y\nmain :: IO ()\nmain = print (3 ! 4, 1 ! 2 ! 3)\n",
        "(34,123)\n",
    ),
    (
        "STightBangChirho.hs",
        "module Main where\n(!) :: Int -> Int -> Int\na!b = a - b\nmain :: IO ()\nmain = print (10 ! 3)\n",
        "7\n",
    ),
    (
        "SSuffixBangChirho.hs",
        "module Main where\n(!) :: Int -> Int -> Int\na! b = a * b\nmain :: IO ()\nmain = print (6 ! 7)\n",
        "42\n",
    ),
    (
        "SBangConLeftChirho.hs",
        "module Main where\ndata Box = Box Int\n(!) :: Box -> Int -> Int\nBox a ! i = a + i\nmain :: IO ()\nmain = print (Box 40 ! 2)\n",
        "42\n",
    ),
    (
        "SWhereBangChirho.hs",
        "module Main where\nmixChirho :: Int -> Int -> Int\nmixChirho x y = x ! y\n  where\n    a ! b = a * 100 + b\nmain :: IO ()\nmain = print (mixChirho 3 4)\n",
        "304\n",
    ),
];

/// A `-` that is not a prefix occurrence before a number defines the operator: with a signature
/// (loose, tight, and prefix before a name), unsigned, as instance methods, and in a `let`.
const MINUS_DEFINITIONS_CHIRHO: [(&str, &str, &str); 7] = [
    (
        "DHiddenMinusSigChirho.hs",
        "module Main where\nimport Prelude hiding ((-))\n(-) :: Int -> Int -> Int\na - b = a * b\nmain :: IO ()\nmain = print (3 - 4, map (2 -) [5])\n",
        "(12,[10])\n",
    ),
    (
        "STightMinusSigChirho.hs",
        "module Main where\nimport Prelude hiding ((-))\n(-) :: Int -> Int -> Int\na-b = a * b\nmain :: IO ()\nmain = print (3 - 4)\n",
        "12\n",
    ),
    (
        "SPrefixMinusVarSigChirho.hs",
        "module Main where\nimport Prelude hiding ((-))\n(-) :: Int -> Int -> Int\na -b = a * b + 1\nmain :: IO ()\nmain = print (3 - 4)\n",
        "13\n",
    ),
    (
        "DHiddenMinusChirho.hs",
        "module Main where\nimport Prelude hiding ((-))\na - b = a + b\nmain :: IO ()\nmain = print (3 - 1 :: Int)\n",
        "4\n",
    ),
    (
        "SInstanceMinusVarChirho.hs",
        "module Main where\nnewtype W = W Int deriving Show\ninstance Num W where\n  W a + W b = W (a + b)\n  x - y = x + y + W 100\n  W a * W b = W (a * b)\n  abs (W a) = W (abs a)\n  signum (W a) = W (signum a)\n  fromInteger n = W (fromInteger n)\nmain :: IO ()\nmain = print (W 1 - W 2)\n",
        "W 103\n",
    ),
    (
        "SInstanceConMinusChirho.hs",
        "module Main where\nnewtype V = V Int deriving Show\ninstance Num V where\n  V a + V b = V (a + b)\n  V a - V b = V (a - b)\n  V a * V b = V (a * b)\n  negate (V a) = V (negate a)\n  abs (V a) = V (abs a)\n  signum (V a) = V (signum a)\n  fromInteger n = V (fromInteger n)\nmain :: IO ()\nmain = print (V 5 - V 3, V 10 - V 1)\n",
        "(V 2,V 9)\n",
    ),
    (
        "SLetMinusChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (let a - b = a * b in 3 - 4 :: Int)\n",
        "12\n",
    ),
];

/// Prefix bangs stay bang patterns in a function, a lambda and a constructor argument; the prefix
/// form of `-` keeps its definition.
const ARGUMENT_PATTERN_HOLDS_CHIRHO: [(&str, &str, &str); 4] = [
    (
        "HBangArgsChirho.hs",
        "module Main where\nf :: Int -> Int -> Int\nf !x !y = x * 10 + y\nmain :: IO ()\nmain = print (f 3 4)\n",
        "34\n",
    ),
    (
        "HLambdaBangChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ((\\ !x -> x + 1) (4 :: Int), (\\ !y !z -> y * z) 6 (7 :: Int))\n",
        "(5,42)\n",
    ),
    (
        "HCaseBangConChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (case Just (3 :: Int) of Just !x -> x * 2; Nothing -> 0)\n",
        "6\n",
    ),
    (
        "XHiddenMinusPrefixChirho.hs",
        "module Main where\nimport Prelude hiding ((-))\n(-) a b = a + b\nmain :: IO ()\nmain = print (3 - 1 :: Int)\n",
        "4\n",
    ),
];

/// Infix definition shapes that already held: backticks, a new operator, a constructor pattern on
/// the left, and a `where`-bound operator.
const INFIX_DEFINITION_HOLDS_CHIRHO: [(&str, &str, &str); 4] = [
    (
        "DBacktickChirho.hs",
        "module Main where\nplusChirho :: Int -> Int -> Int\na `plusChirho` b = a + b + 100\nmain :: IO ()\nmain = print (1 `plusChirho` 2, plusChirho 3 4)\n",
        "(103,107)\n",
    ),
    (
        "DNewOperatorChirho.hs",
        "module Main where\ninfixl 6 <+>\n(<+>) :: Int -> Int -> Int\na <+> b = a * 10 + b\nmain :: IO ()\nmain = print (1 <+> 2 <+> 3)\n",
        "123\n",
    ),
    (
        "DPatternInfixChirho.hs",
        "module Main where\n(<?>) :: Maybe Int -> Int -> Int\nJust a <?> _ = a\nNothing <?> d = d\nmain :: IO ()\nmain = print (Just 5 <?> 0, Nothing <?> 7)\n",
        "(5,7)\n",
    ),
    (
        "DWhereOperatorChirho.hs",
        "module Main where\nmixChirho :: Int -> Int -> Int\nmixChirho x y = x |> y\n  where\n    a |> b = a * 2 + b\nmain :: IO ()\nmain = print (mixChirho 3 4)\n",
        "10\n",
    ),
];

#[test]
fn bang_operator_definitions_follow_ghc_spacing_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in BANG_DEFINITIONS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn minus_operator_definitions_follow_ghc_spacing_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in MINUS_DEFINITIONS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn prefix_bangs_and_the_prefix_minus_form_hold_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in ARGUMENT_PATTERN_HOLDS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn earlier_infix_definition_shapes_hold_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in INFIX_DEFINITION_HOLDS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}
