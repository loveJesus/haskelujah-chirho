// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! A parenthesized function left side, Haskell 2010's `( funlhs ) apat { apat }` (row 618), as in
//! `(f . g) x = g (f x)`, defines its operator, as the prefix form `(.) f g x` does. Before this
//! the declaration broke into error nodes and a binding of its last argument, and every use of
//! the operator was unbound. Every program here is GHC 9.14.1's, byte for byte as measured.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

/// An operator defined by a parenthesized left side, signed and unsigned: `.` hidden from the
/// Prelude, a new operator, a backticked name and a `where`-bound operator.
const OPERATOR_DEFINITIONS_CHIRHO: [(&str, &str, &str); 5] = [
    (
        "PDotSigChirho.hs",
        "module Main where\nimport Prelude hiding ((.))\n(.) :: (Int -> Int) -> (Int -> Int) -> Int -> Int\n(f . g) x = g (f x)\nmain :: IO ()\nmain = print (((+ 1) . (* 2)) 5)\n",
        "12\n",
    ),
    (
        "PDotUnsignedChirho.hs",
        "module Main where\nimport Prelude hiding ((.))\n(f . g) x = g (f x)\nmain :: IO ()\nmain = print (((+ 1) . (* 2)) (5 :: Int))\n",
        "12\n",
    ),
    (
        "PNewOperatorChirho.hs",
        "module Main where\ninfixr 5 <+>\n(<+>) :: (Int -> Int) -> (Int -> Int) -> Int -> Int\n(f <+> g) x = f x + g x\nmain :: IO ()\nmain = print (((* 2) <+> (+ 10)) 3)\n",
        "19\n",
    ),
    (
        "PBacktickChirho.hs",
        "module Main where\nandThen :: (Int -> Int) -> (Int -> Int) -> Int -> Int\n(f `andThen` g) x = g (f x)\nmain :: IO ()\nmain = print (((+ 1) `andThen` (* 3)) 4)\n",
        "15\n",
    ),
    (
        "PWhereChirho.hs",
        "module Main where\nmixChirho :: Int -> Int\nmixChirho = (+ 1) |> (* 2)\n  where\n    (f |> g) x = g (f x)\nmain :: IO ()\nmain = print (mixChirho 5)\n",
        "12\n",
    ),
];

/// Two arguments after the `)`, a nested left side, an instance method with constructor operands,
/// and constructor operands at the top level.
const OPERAND_AND_ARITY_DEFINITIONS_CHIRHO: [(&str, &str, &str); 4] = [
    (
        "PTwoExtraArgsChirho.hs",
        "module Main where\n(<%>) :: (Int -> Int) -> (Int -> Int) -> Int -> Int -> Int\n(f <%> g) x y = f x + g y\nmain :: IO ()\nmain = print (((* 2) <%> (+ 10)) 3 4)\n",
        "20\n",
    ),
    (
        "PNestedChirho.hs",
        "module Main where\n(<%>) :: (Int -> Int) -> (Int -> Int) -> Int -> Int -> Int\n((f <%> g) x) y = f x + g y\nmain :: IO ()\nmain = print (((* 2) <%> (+ 10)) 3 4)\n",
        "20\n",
    ),
    (
        "PInstanceMethodChirho.hs",
        "module Main where\nnewtype F = F (Int -> Int)\nclass Compose a where\n  (<<>>) :: a -> a -> Int -> Int\ninstance Compose F where\n  (F f <<>> F g) x = g (f x)\nmain :: IO ()\nmain = print ((F (+ 1) <<>> F (* 2)) 5)\n",
        "12\n",
    ),
    (
        "PConOperandChirho.hs",
        "module Main where\n(<?>) :: Maybe Int -> Int -> Int -> Int\n(Just a <?> b) x = a + b + x\n(Nothing <?> b) x = b * x\nmain :: IO ()\nmain = print ((Just 1 <?> 2) 3, (Nothing <?> 4) 5)\n",
        "(6,20)\n",
    ),
];

/// What already held: the prefix form `(.) f g x`, parenthesized expressions, and parenthesized
/// pattern bindings with a tuple and a constructor operator.
const PAREN_HOLDS_CHIRHO: [(&str, &str, &str); 3] = [
    (
        "HPrefixFormChirho.hs",
        "module Main where\nimport Prelude hiding ((.))\n(.) :: (Int -> Int) -> (Int -> Int) -> Int -> Int\n(.) f g x = g (f x)\nmain :: IO ()\nmain = print (((+ 1) . (* 2)) 5)\n",
        "12\n",
    ),
    (
        "HParenExprNegateChirho.hs",
        "module Main where\nf :: Int -> Int\nf x = (x + 1) * 2\ng :: (Int, Int) -> Int\ng (a, b) = (a - b) * (a + b)\nmain :: IO ()\nmain = print (f 3, g (5, 2), (negate . (* 3)) 4)\n",
        "(8,21,-12)\n",
    ),
    (
        "HParenPatBindsChirho.hs",
        "module Main where\n(a, b) = (1 :: Int, 2 :: Int)\n(x:_) = [5 :: Int, 6]\nmain :: IO ()\nmain = print a >> print b >> print x\n",
        "1\n2\n5\n",
    ),
];

#[test]
fn parenthesized_operator_definitions_bind_their_operator_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in OPERATOR_DEFINITIONS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn nested_and_constructor_operand_definitions_bind_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in OPERAND_AND_ARITY_DEFINITIONS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn prefix_form_expressions_and_pattern_bindings_hold_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in PAREN_HOLDS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

/// With no argument after the `)`, the left side is a pattern, and `a <+> b` is not one. GHC
/// rejects the program ("Parse error in pattern: a <+> b"); this compiler rejects it too, as an
/// unbound `<+>`, with the message of its own.
#[test]
fn a_parenthesized_funlhs_without_an_argument_is_rejected_like_ghc_chirho() {
    rejected_like_ghc_chirho(
        "module Main where\n(<+>) :: Int -> Int -> Int\n(a <+> b) = a * 10 + b\nmain :: IO ()\nmain = print (1 <+> 2)\n",
        "XNoExtraArgGhcRejectsChirho.hs",
        &[],
    );
}
