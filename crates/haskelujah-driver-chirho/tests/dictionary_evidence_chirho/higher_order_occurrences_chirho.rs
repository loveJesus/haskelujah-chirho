// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! R2 at every occurrence entry (R2b, gpt_chirho #27663). An `Eq` or `Ord`
//! occurrence the checker proved at a whole list key keeps R2's dispatch when it
//! sits inside a typed higher-order argument (`map`, `filter`, `any`, `zipWith`,
//! a user's `applyChirho`). That route handed the occurrence to the
//! method-variable rewrite, which left a bare `==` or `compare` ("missing STG
//! binding") and, inside an `Eq a =>` binding, let the enclosing dictionary
//! answer for `Eq [[Int]]`. Every expected output is GHC 9.14.1's.
//! workflow: language-features-chirho/dictionary-evidence-chirho
//!
//! A method of a class the program declares gets the same per-occurrence
//! proof (gpt_chirho #28091). Every use of it shared one id, so under `map`
//! the dictionary pass guessed `NeedsChirho Int` from the nested literal.
//! A proof never borrows the enclosing context dictionary, and a row whose
//! instance has a context is called with that context (gpt_chirho #28111).
//! workflow: language-features-chirho/own-class-method-occurrences-chirho

use super::{assert_ghc_or_named_chirho, run_chirho, run_result_chirho};

const NEEDS_CHIRHO: &str =
    "module Main where\nclass NeedsChirho a where\n  neededChirho :: a -> Int\n";

const DESCRIBABLE_CHIRHO: &str = "module Main where\nclass DescribableChirho a where\n  describeChirho :: a -> String\ninstance DescribableChirho Int where\n  describeChirho x = show x\n";

const MARK_CHIRHO: &str = "{-# LANGUAGE FlexibleContexts, FlexibleInstances, UndecidableInstances #-}\nmodule Main where\ndata FooChirho = FooChirho\nclass MarkChirho a where\n  markChirho :: a -> Int\n";

/// GHC 9.14.1 stops these with "No instance nor default method for class
/// operation neededChirho"; the run must fail too, and name the method.
fn assert_fails_naming_chirho(source_chirho: &str, file_name_chirho: &str, missing_chirho: &str) {
    match run_result_chirho(file_name_chirho, source_chirho) {
        Ok(output_chirho) => {
            panic!("{file_name_chirho}: GHC fails at run time; we answered {output_chirho:?}")
        }
        Err(err_chirho) => assert!(
            err_chirho.contains(missing_chirho),
            "{file_name_chirho}: the failure must name `{missing_chirho}`: {err_chirho}"
        ),
    }
}

#[test]
fn a_comparison_inside_a_higher_order_argument_answers_as_ghc_or_names_its_dictionary_chirho() {
    // Each failed as a bare `==` or `compare` on the R1/R2 and ground CLIs.
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
fn a_disagreeing_argument_key_never_serves_a_proven_list_chirho() {
    // The typed route offers applyChirho's argument key, `[Int]`, which
    // disagrees with the proven `[[Int]]`; `$fEq[Int]` would answer True.
    assert_ghc_or_named_chirho(
        "DisagreeingPayloadChirho.hs",
        "module Main where\napplyChirho :: (a -> b) -> a -> b\napplyChirho f x = f x\nmain :: IO ()\nmain = print (applyChirho (\\x -> x == [[1]]) [[2 :: Int]])\n",
        "False\n",
        "Eq [[Int]]",
    );
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
fn a_defaulted_comparison_inside_a_higher_order_argument_answers_as_ghc_chirho() {
    // Defaulted literals are proven `[Integer]`, which the engine spells
    // `[Int]`: `$fEq[Int]` serves them. Each failed as a bare `==` on the
    // R1/R2 CLI, and ground printed the section [True,True].
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "DefaultedLambdaChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\x -> x == [1]) [[1], [2]])\n",
            "[True,False]\n",
        ),
        (
            "DefaultedSectionChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (== [1]) [[1], [2]])\n",
            "[True,False]\n",
        ),
        (
            "DefaultedSelfChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (\\x -> x == x) [[1], [2]])\n",
            "[True,True]\n",
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
fn a_method_of_the_programs_class_runs_at_the_type_its_occurrence_was_proven_at_chirho() {
    // Before, the base and Ord CLIs printed [[1]] for the first and crashed on
    // the third, where Int's `+ 1` met a list. The second puts two types in one
    // program; inside an instance or a constrained binding the enclosing
    // dictionary still answers; and the direct call already printed 99.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "DefinedListUnderMapChirho.hs",
            "instance NeedsChirho Int where\n  neededChirho x = x\ninstance NeedsChirho a => NeedsChirho [a] where\n  neededChirho _ = 99\nmain :: IO ()\nmain = print (map neededChirho [[1 :: Int]])\n",
            "[99]\n",
        ),
        (
            "TwoTypesUnderMapChirho.hs",
            "instance NeedsChirho Int where\n  neededChirho x = x\ninstance NeedsChirho a => NeedsChirho [a] where\n  neededChirho _ = 99\nmain :: IO ()\nmain = print (map neededChirho [[1 :: Int]], map neededChirho [2 :: Int])\n",
            "([99],[2])\n",
        ),
        (
            "ElementMethodInsideChirho.hs",
            "instance NeedsChirho Int where\n  neededChirho x = x + 1\ninstance NeedsChirho a => NeedsChirho [a] where\n  neededChirho xs = sum (map neededChirho xs) + 1000\nmain :: IO ()\nmain = print (map neededChirho [[1, 2 :: Int]])\n",
            "[1005]\n",
        ),
        (
            "OwnDictionaryChirho.hs",
            "instance NeedsChirho Int where\n  neededChirho x = x\ninstance NeedsChirho a => NeedsChirho [a] where\n  neededChirho xs = sum (map neededChirho xs) * 10\nuseChirho :: NeedsChirho a => a -> Int\nuseChirho x = neededChirho x + 1\nmain :: IO ()\nmain = print (useChirho (2 :: Int), useChirho [3 :: Int])\n",
            "(3,31)\n",
        ),
        (
            "DefinedListDirectChirho.hs",
            "instance NeedsChirho Int where\n  neededChirho x = x\ninstance NeedsChirho a => NeedsChirho [a] where\n  neededChirho _ = 99\nmain :: IO ()\nmain = print (neededChirho [1 :: Int])\n",
            "99\n",
        ),
        (
            // Self-qualified: the desugarer and the checker both key it by text.
            "SelfQualifiedUnderMapChirho.hs",
            "instance NeedsChirho Int where\n  neededChirho x = x\ninstance NeedsChirho a => NeedsChirho [a] where\n  neededChirho _ = 99\nmain :: IO ()\nmain = print (map Main.neededChirho [[1 :: Int]], Main.neededChirho (5 :: Int))\n",
            "([99],5)\n",
        ),
    ] {
        let source_chirho = format!("{NEEDS_CHIRHO}{program_chirho}");
        assert_eq!(
            run_chirho(file_name_chirho, &source_chirho),
            ghc_chirho,
            "{file_name_chirho}"
        );
    }
}

#[test]
fn a_missing_method_of_the_programs_class_fails_by_name_under_map_chirho() {
    // Before, each printed [[1]] or [42] and exited 0.
    assert_fails_naming_chirho(
        &format!(
            "{NEEDS_CHIRHO}instance NeedsChirho Int where\n  neededChirho _ = 42\ninstance NeedsChirho a => NeedsChirho [a]\nmain :: IO ()\nmain = print (map neededChirho [[1 :: Int]])\n"
        ),
        "ConstantElementUnderMapChirho.hs",
        "missing method NeedsChirho.neededChirho for [Int]",
    );
    assert_fails_naming_chirho(
        &format!(
            "{NEEDS_CHIRHO}instance NeedsChirho Int where\n  neededChirho x = x\ninstance NeedsChirho a => NeedsChirho [a]\nmain :: IO ()\nmain = print (map (\\x -> neededChirho x) [[1 :: Int]])\n"
        ),
        "EtaUnderMapChirho.hs",
        "missing method NeedsChirho.neededChirho for [Int]",
    );
    // A context-free head: the method is named; its row's key spelling is not
    // pinned here.
    assert_fails_naming_chirho(
        &format!(
            "{NEEDS_CHIRHO}instance NeedsChirho Int where\n  neededChirho x = x\ninstance NeedsChirho [a]\nmain :: IO ()\nmain = print (map neededChirho [[1 :: Int]])\n"
        ),
        "PlainHeadUnderMapChirho.hs",
        "missing method NeedsChirho.neededChirho",
    );
}

#[test]
fn a_binder_named_like_the_programs_method_is_that_binder_chirho() {
    // The desugarer finds a lexical binding in scope before it would mint a
    // method occurrence, so each name below is the binder at the binder's type.
    // A top-level function or a field cannot share a name with a method of the
    // same module.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "LambdaShadowChirho.hs",
            "main :: IO ()\nmain = print ((\\neededChirho -> neededChirho 3) (+ (1 :: Int)))\n",
            "4\n",
        ),
        (
            "LetShadowChirho.hs",
            "main :: IO ()\nmain = print (let neededChirho = length in neededChirho \"abc\")\n",
            "3\n",
        ),
        (
            "WhereShadowChirho.hs",
            "countChirho :: [Bool] -> Int\ncountChirho xs = neededChirho xs\n  where\n    neededChirho = length\nmain :: IO ()\nmain = print (countChirho [True, False])\n",
            "2\n",
        ),
        (
            "CaseShadowChirho.hs",
            "main :: IO ()\nmain = print (case Just (+ (2 :: Int)) of { Just neededChirho -> neededChirho 5; Nothing -> 0 })\n",
            "7\n",
        ),
    ] {
        let source_chirho = format!("{NEEDS_CHIRHO}{program_chirho}");
        assert_eq!(
            run_chirho(file_name_chirho, &source_chirho),
            ghc_chirho,
            "{file_name_chirho}"
        );
    }
}

#[test]
fn a_recursive_call_of_the_programs_method_keeps_its_instances_own_dictionary_chirho() {
    // The checker proves `describeChirho xs` only as `[]`. The Ord-plus-opt-in
    // CLI found no dictionary under that key and served the element's context
    // dictionary: 1:[2,3], on the interpreter and as a pointer on LLVM and
    // Cranelift (compile_chirho's recursive custom-list round trips). Main
    // prints GHC's answer through the self dictionary its argument's key names.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "RecursiveConditionalOccurrenceChirho.hs",
            "instance DescribableChirho a => DescribableChirho [a] where\n  describeChirho [] = \"[]\"\n  describeChirho (x:xs) = describeChirho x ++ \":\" ++ describeChirho xs\nmain :: IO ()\nmain = putStrLn (describeChirho [1,2,3 :: Int])\n",
            "1:2:3:[]\n",
        ),
        (
            "RecursiveWhereOccurrenceChirho.hs",
            "instance DescribableChirho a => DescribableChirho [a] where\n  describeChirho [] = \"[]\"\n  describeChirho (x:xs) = describeChirho x ++ \":\" ++ restChirho\n    where\n      restChirho = describeChirho xs\nmain :: IO ()\nmain = putStrLn (describeChirho [1,2,3 :: Int])\n",
            "1:2:3:[]\n",
        ),
    ] {
        let source_chirho = format!("{DESCRIBABLE_CHIRHO}{program_chirho}");
        assert_eq!(
            run_chirho(file_name_chirho, &source_chirho),
            ghc_chirho,
            "{file_name_chirho}"
        );
    }
}

#[test]
fn an_occurrence_at_another_list_type_never_borrows_the_self_dictionary_chirho() {
    // Inside the `[a]` instance, `tagChirho [True]` is proved at `[Bool]`: its
    // own dictionary answers `b`, where the self dictionary (here `[Int]`)
    // would answer `i`. The opt-in CLI printed 1:[2]b; main prints GHC's.
    assert_eq!(
        run_chirho(
            "AgreementGroundInBodyChirho.hs",
            "module Main where\nclass DescribableChirho a where\n  describeChirho :: a -> String\n  tagChirho :: a -> String\ninstance DescribableChirho Int where\n  describeChirho x = show x\n  tagChirho _ = \"i\"\ninstance DescribableChirho Bool where\n  describeChirho b = if b then \"T\" else \"F\"\n  tagChirho _ = \"b\"\ninstance DescribableChirho a => DescribableChirho [a] where\n  describeChirho [] = \"[]\"\n  describeChirho (x:xs) = describeChirho x ++ \":\" ++ describeChirho xs ++ tagChirho [True]\n  tagChirho [] = \"l\"\n  tagChirho (y:_) = tagChirho y\nmain :: IO ()\nmain = putStrLn (describeChirho [1,2 :: Int])\n",
        ),
        "1:2:[]bb\n"
    );
}

#[test]
fn a_row_whose_instance_has_a_context_is_called_with_that_context_chirho() {
    // `$prim_MarkChirho_markChirho_FooChirho` takes `$dShow` first. Called bare,
    // FooChirho bound as that dictionary and the run printed a `$PAP` with exit
    // 0; main failed loudly ("missing STG binding `markChirho`").
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "ContextualGroundOccurrenceChirho.hs",
            "instance Show Int => MarkChirho FooChirho where\n  markChirho _ = 7\nmain :: IO ()\nmain = print (markChirho FooChirho)\n",
            "7\n",
        ),
        (
            "ContextualGroundUnderMapChirho.hs",
            "instance Show Int => MarkChirho FooChirho where\n  markChirho _ = 7\nmain :: IO ()\nmain = print (map markChirho [FooChirho, FooChirho])\n",
            "[7,7]\n",
        ),
        (
            "ContextualGroundTwoChirho.hs",
            "instance (Show Int, Num Int) => MarkChirho FooChirho where\n  markChirho _ = length (show (40 + 2 :: Int))\nmain :: IO ()\nmain = print (markChirho FooChirho)\n",
            "2\n",
        ),
    ] {
        let source_chirho = format!("{MARK_CHIRHO}{program_chirho}");
        assert_eq!(
            run_chirho(file_name_chirho, &source_chirho),
            ghc_chirho,
            "{file_name_chirho}"
        );
    }
}
