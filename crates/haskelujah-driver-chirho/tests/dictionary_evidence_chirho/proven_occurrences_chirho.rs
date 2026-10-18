// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Proven list keys at `Eq` and `Ord` method occurrences (R2, gpt_chirho
//! #27481, #27488). An occurrence the checker proved at a whole list key that
//! names no method row is served by the registry's dictionary at that key, by
//! the one at the call's argument-shape key only when that key agrees with the
//! proof, or by an error naming the obligation, forced only when the comparison
//! is. It used to fall back to a guess read off the first argument, so the
//! nested comparisons below printed `True` for both twins. Every expected
//! output is GHC 9.14.1's.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{run_chirho, run_result_chirho};

/// GHC's exact output, or a failure that names the obligation: never another
/// dictionary's answer.
fn assert_ghc_or_named_chirho(
    file_name_chirho: &str,
    source_chirho: &str,
    ghc_chirho: &str,
    obligation_chirho: &str,
) {
    match run_result_chirho(file_name_chirho, source_chirho) {
        Ok(output_chirho) => assert_eq!(
            output_chirho, ghc_chirho,
            "{file_name_chirho}: neither GHC's output nor a named failure"
        ),
        Err(err_chirho) => assert!(
            err_chirho.contains(&format!("no instance dictionary for `{obligation_chirho}`")),
            "{file_name_chirho}: the failure must name `{obligation_chirho}`: {err_chirho}"
        ),
    }
}

#[test]
fn a_nested_list_comparison_answers_as_ghc_or_names_its_dictionary_chirho() {
    // Each program pairs an unequal comparison with its equal twin. The key is
    // the checker's spelling: defaulted literals are Integer, and a nested
    // application is parenthesized. The Maybe row printed GHC's answer only by
    // the runtime unboxing a one-field constructor; it is the measured cost.
    for (file_name_chirho, program_chirho, ghc_chirho, obligation_chirho) in [
        (
            "NestedIntChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ([[1]] == [[2 :: Int]], [[1]] == [[1 :: Int]])\n",
            "(False,True)\n",
            "Eq [[Int]]",
        ),
        (
            "NestedDefaultedChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ([[1]] == [[2]], [[1]] == [[1]])\n",
            "(False,True)\n",
            "Eq [[Integer]]",
        ),
        (
            "NestedDoubleChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ([[1.5]] == [[2.5 :: Double]], [[1.5]] == [[1.5 :: Double]])\n",
            "(False,True)\n",
            "Eq [[Double]]",
        ),
        (
            "NestedDefaultedDoubleChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ([[1.5]] == [[2.5]], [[1.5]] == [[1.5]])\n",
            "(False,True)\n",
            "Eq [[Double]]",
        ),
        (
            "NestedBoolChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ([[True]] == [[False]], [[True]] == [[True]])\n",
            "(False,True)\n",
            "Eq [[Bool]]",
        ),
        (
            "NestedLongerChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ([[1, 2]] == [[1, 3 :: Int]], [[1, 2]] == [[1, 2 :: Int]])\n",
            "(False,True)\n",
            "Eq [[Int]]",
        ),
        (
            "NestedCompareChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare [[1]] [[2 :: Int]], compare [[2]] [[1 :: Int]])\n",
            "(LT,GT)\n",
            "Ord [[Int]]",
        ),
        (
            "StringListChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ([\"a\"] == [\"b\"], [\"a\"] == [\"a\"])\n",
            "(False,True)\n",
            "Eq [String]",
        ),
        (
            "MaybeListChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ([Just 1] == [Just (2 :: Int)], [Just 1] == [Just (1 :: Int)])\n",
            "(False,True)\n",
            "Eq [(Maybe Int)]",
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
fn signed_nested_lists_answer_as_ghc_or_name_their_dictionary_chirho() {
    // These failed before too, as a raw missing binding; now the failure names
    // the obligation, and a later dictionary for it would answer in its place.
    for (file_name_chirho, program_chirho, ghc_chirho, obligation_chirho) in [
        (
            "SignedFunctionChirho.hs",
            "module Main where\nnestedEqChirho :: [[Int]] -> [[Int]] -> Bool\nnestedEqChirho aChirho bChirho = aChirho == bChirho\nmain :: IO ()\nmain = print (nestedEqChirho [[1]] [[2]], nestedEqChirho [[1]] [[1]])\n",
            "(False,True)\n",
            "Eq [[Int]]",
        ),
        (
            "OuterAnnotationChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (([[1]] :: [[Int]]) == [[2]], ([[1]] :: [[Int]]) == [[1]])\n",
            "(False,True)\n",
            "Eq [[Int]]",
        ),
        (
            "SignedLetChirho.hs",
            "module Main where\nmain :: IO ()\nmain = let xsChirho = [[1]] :: [[Int]] in print (xsChirho == [[2]], xsChirho == [[1]])\n",
            "(False,True)\n",
            "Eq [[Int]]",
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
fn an_enclosing_dictionary_never_answers_for_a_proven_list_chirho() {
    // The first comparison is at the signature's own `a` and takes its
    // parameter. The nested one is proven `Eq [[Int]]`, which that `Eq a`
    // dictionary answered before (`True`).
    assert_ghc_or_named_chirho(
        "EnclosingDictionaryChirho.hs",
        "module Main where\nsameChirho :: Eq a => a -> a -> Bool\nsameChirho xChirho yChirho = xChirho == yChirho && [[1]] == [[2 :: Int]]\nmain :: IO ()\nmain = print (sameChirho 'a' 'a')\n",
        "False\n",
        "Eq [[Int]]",
    );
}

#[test]
fn agreeing_keys_and_other_classes_keep_ghcs_answer_chirho() {
    // The defaulted flat list is proven `[Integer]` and still served by
    // `$fEq[Int]` through the agreeing `[Int]`. Show keeps its own dictionary
    // path, including a shape with no generated row.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "DefaultedListChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ([1] == [2], [1] == [1])\n",
            "(False,True)\n",
        ),
        (
            "ScalarsAndFlatListsChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (\"ab\" == \"ab\", \"ab\" == \"ac\", [1.5] == [1.5 :: Double]) >> print ([True] == [False], (1 :: Int) == 1) >> print ('a' == 'b', 1.5 == (2.5 :: Double), 1 == 2)\n",
            "(True,False,True)\n(False,True)\n(False,False,False)\n",
        ),
        (
            "DoubleVariablesChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ((\\aChirho bChirho -> [aChirho] == [bChirho]) 1.5 (2.5 :: Double), (\\aChirho -> [aChirho] == [aChirho]) (1.5 :: Double))\n",
            "(False,True)\n",
        ),
        (
            "ShowNestedChirho.hs",
            "module Main where\nmain :: IO ()\nmain = putStrLn (show [[1, 2], [3 :: Int]]) >> putStrLn (show [[1], [2]]) >> putStrLn (show [(1, 'a')]) >> putStrLn (show [\"a\", \"b\"])\n",
            "[[1,2],[3]]\n[[1],[2]]\n[(1,'a')]\n[\"a\",\"b\"]\n",
        ),
        (
            "ShowWithoutRowChirho.hs",
            "module Main where\ndata ColorChirho = RedChirho | GreenChirho deriving (Eq, Show)\nmain :: IO ()\nmain = putStrLn (show [Just RedChirho]) >> putStrLn (show [RedChirho]) >> print [Just GreenChirho]\n",
            "[Just RedChirho]\n[RedChirho]\n[Just GreenChirho]\n",
        ),
        (
            "ShowSupportedShapesChirho.hs",
            "module Main where\nmain :: IO ()\nmain = putStrLn (show [[Just 1], [Nothing]]) >> putStrLn (show [Left 1, Right 'a'])\n",
            "[[Just 1],[Nothing]]\n[Left 1,Right 'a']\n",
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
fn a_comparison_nobody_forces_never_fails_chirho() {
    // The missing dictionary is a thunk: an unused binding, a function never
    // applied, an element never inspected and a short-circuited conjunction
    // cost nothing.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "UnusedLetChirho.hs",
            "module Main where\nmain :: IO ()\nmain = let unusedChirho = [[1]] == [[2 :: Int]] in print (42 :: Int)\n",
            "42\n",
        ),
        (
            "EmptyMapChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\xChirho -> xChirho == [[1 :: Int]]) [])\n",
            "[]\n",
        ),
        (
            "UnforcedElementChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (length [[[1]] == [[2 :: Int]]])\n",
            "1\n",
        ),
        (
            "ShortCircuitChirho.hs",
            "module Main where\nsameChirho :: Eq a => a -> a -> Bool\nsameChirho xChirho yChirho = xChirho == yChirho && [[1]] == [[2 :: Int]]\nmain :: IO ()\nmain = print (sameChirho 'a' 'b')\n",
            "False\n",
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
fn the_same_comparison_forced_names_its_dictionary_chirho() {
    assert_ghc_or_named_chirho(
        "ForcedNestedChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ([[1]] == [[2 :: Int]])\n",
        "False\n",
        "Eq [[Int]]",
    );
}
