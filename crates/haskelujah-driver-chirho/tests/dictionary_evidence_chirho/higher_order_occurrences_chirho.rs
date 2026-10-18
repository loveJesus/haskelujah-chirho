// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! R2 at every occurrence entry (R2b, gpt_chirho #27663). An `Eq` or `Ord`
//! occurrence the checker proved at a whole list key keeps R2's dispatch when it
//! sits inside a typed higher-order argument (`map`, `filter`, `any`, `zipWith`,
//! a user's `applyChirho`). That route handed the occurrence to the
//! method-variable rewrite, which left a bare `==` or `compare` ("missing STG
//! binding") and, inside an `Eq a =>` binding, let the enclosing dictionary
//! answer for `Eq [[Int]]`. Every expected output is GHC 9.14.1's.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_ghc_or_named_chirho, run_chirho};

#[test]
fn a_comparison_inside_a_higher_order_argument_answers_as_ghc_or_names_its_dictionary_chirho() {
    // Each failed as a bare `==` or `compare` on the R1/R2 and ground CLIs.
    // applyChirho's argument key is `[Int]`, which disagrees with the proven
    // `[[Int]]` and must never serve it.
    for (file_name_chirho, program_chirho, ghc_chirho, obligation_chirho) in [
        (
            "MapLambdaLeftChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\x -> x == [[1]]) [[[1]], [[2 :: Int]]])\n",
            "[True,False]\n",
            "Eq [[Int]]",
        ),
        (
            "MapLambdaRightChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\x -> [[1]] == x) [[[1]], [[2 :: Int]]])\n",
            "[True,False]\n",
            "Eq [[Int]]",
        ),
        (
            "FilterNestedChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (filter (\\x -> x == [[1]]) [[[1]], [[2 :: Int]]])\n",
            "[[[1]]]\n",
            "Eq [[Int]]",
        ),
        (
            "AnyNestedChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (any (\\x -> x == [[1]]) [[[2]], [[3 :: Int]]])\n",
            "False\n",
            "Eq [[Int]]",
        ),
        (
            "ZipWithBareChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (zipWith (==) [[[1]], [[2]]] [[[1]], [[3 :: Int]]])\n",
            "[True,False]\n",
            "Eq [[Int]]",
        ),
        (
            "DisagreeingPayloadChirho.hs",
            "module Main where\napplyChirho :: (a -> b) -> a -> b\napplyChirho f x = f x\nmain :: IO ()\nmain = print (applyChirho (\\x -> x == [[1]]) [[2 :: Int]])\n",
            "False\n",
            "Eq [[Int]]",
        ),
        (
            "MapCompareNestedChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\x -> compare x [[1]]) [[[1]], [[2 :: Int]]])\n",
            "[EQ,GT]\n",
            "Ord [[Int]]",
        ),
    ] {
        assert_ghc_or_named_chirho(
            file_name_chirho,
            program_chirho,
            ghc_chirho,
            obligation_chirho,
        );
    }
}

#[test]
fn an_enclosing_dictionary_never_answers_inside_a_higher_order_argument_chirho() {
    // The R1/R2 and ground CLIs printed True: the binding's own `Eq a`
    // dictionary, here Char's, answered for the lambda's `Eq [[Int]]`.
    assert_ghc_or_named_chirho(
        "EnclosingDictionaryChirho.hs",
        "module Main where\npickChirho :: Eq a => a -> [[Int]] -> Bool\npickChirho _ ys = any (\\x -> x == [[2]]) [ys]\nmain :: IO ()\nmain = print (pickChirho 'c' [[1]])\n",
        "False\n",
        "Eq [[Int]]",
    );
}

#[test]
fn a_higher_order_comparison_with_a_method_row_keeps_ghcs_answer_chirho() {
    // Exact on both CLIs before R2b: a method row serves these keys first.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "ListLambdaChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\x -> x == [1]) [[1], [2 :: Int]])\n",
            "[True,False]\n",
        ),
        (
            "ListSelfChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\x -> x == x) [[1], [2 :: Int]])\n",
            "[True,True]\n",
        ),
        (
            "DoubleLambdaChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\x -> x == 1.5) [1.5, 2.5 :: Double])\n",
            "[True,False]\n",
        ),
        (
            "UserHigherIntChirho.hs",
            "module Main where\napplyChirho :: (a -> b) -> a -> b\napplyChirho f x = f x\nmain :: IO ()\nmain = print (applyChirho (\\x -> x == x) [1 :: Int])\n",
            "True\n",
        ),
    ] {
        assert_eq!(
            run_chirho(file_name_chirho, program_chirho),
            ghc_chirho,
            "{file_name_chirho}"
        );
    }
}

#[test]
fn a_defaulted_comparison_inside_a_higher_order_argument_answers_as_ghc_or_names_its_dictionary_chirho()
 {
    // Defaulted literals are proven `[Integer]`. Each failed as a bare `==` or
    // `compare` on the R1/R2 CLI, and ground printed the section [True,True].
    // The route's own key is the element of a collapsed `[Int]`, which does
    // not agree with `[Integer]`, so a named failure is the expected floor.
    for (file_name_chirho, program_chirho, ghc_chirho, obligation_chirho) in [
        (
            "DefaultedLambdaChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\x -> x == [1]) [[1], [2]])\n",
            "[True,False]\n",
            "Eq [Integer]",
        ),
        (
            "DefaultedSectionChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (== [1]) [[1], [2]])\n",
            "[True,False]\n",
            "Eq [Integer]",
        ),
        (
            "DefaultedSelfChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\x -> x == x) [[1], [2]])\n",
            "[True,True]\n",
            "Eq [Integer]",
        ),
        (
            "DefaultedCompareChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\x -> compare x [1]) [[1], [2]])\n",
            "[EQ,GT]\n",
            "Ord [Integer]",
        ),
    ] {
        assert_ghc_or_named_chirho(
            file_name_chirho,
            program_chirho,
            ghc_chirho,
            obligation_chirho,
        );
    }
}
