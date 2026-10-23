// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! How generated order rows answer: the conditional list generator's rows and
//! the constructor-order rows of Bool and Ordering (gpt_chirho disposition on
//! #28041). A row named for a list key keeps its method's own calling
//! convention, so a caller that finds it by name gets either the method's
//! answer or a failure naming the method, never a partial application.
//! Each program's GHC 9.14.1 behavior is recorded beside it.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

use super::{assert_ghc_or_named_chirho, run_chirho};

#[test]
fn a_list_order_answers_as_ghc_chirho() {
    // `Ord [T]` is generated for Int, Bool, Double and String elements from one
    // recursive comparison. Before it every row here crashed or named a
    // missing `Ord.compare`, except sortBy and maximumBy, which printed
    // [[2],[1]] and [1]: the compare slot was the stub whose calling
    // convention the missing-method control below guards.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "ListCompareLessChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare [1] [2 :: Int])\n",
            "LT\n",
        ),
        (
            "ListComparePrefixChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare [1, 2] [1 :: Int])\n",
            "GT\n",
        ),
        (
            "ListCompareRowsChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare [1, 2] [1, 3 :: Int], compare [] [1 :: Int], compare [1] [1 :: Int])\n",
            "(LT,LT,EQ)\n",
        ),
        (
            // Only the first elements are compared; the tail is never forced.
            "ListCompareLazyTailChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare (1 : undefined) [2 :: Int])\n",
            "LT\n",
        ),
        (
            "ListCompareConstrainedChirho.hs",
            "module Main where\ncmpChirho :: Ord a => a -> a -> Ordering\ncmpChirho = compare\nmain :: IO ()\nmain = print (cmpChirho [1] [2 :: Int])\n",
            "LT\n",
        ),
        (
            "ListSortByCompareChirho.hs",
            "module Main where\nimport Data.List (sortBy)\nmain :: IO ()\nmain = print (sortBy compare [[2], [1 :: Int]])\n",
            "[[1],[2]]\n",
        ),
        (
            "ListMaximumByCompareChirho.hs",
            "module Main where\nimport Data.List (maximumBy)\nmain :: IO ()\nmain = print (maximumBy compare [[1], [3], [2 :: Int]])\n",
            "[3]\n",
        ),
        (
            "DoubleListCompareChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare [1.5] [2.5 :: Double])\n",
            "LT\n",
        ),
        (
            "StringListSortByCompareChirho.hs",
            "module Main where\nimport Data.List (sortBy)\nmain :: IO ()\nmain = print (sortBy compare [[\"b\"], [\"a\"]])\n",
            "[[\"a\"],[\"b\"]]\n",
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
fn a_bool_order_answers_as_ghc_and_ordering_keeps_its_answer_chirho() {
    // Bool's compare row was `compare#`, an Int primop, on constructors: it
    // returned the integer 0, which crashed `print` (tag 0) and sent sortBy's
    // default branch the wrong way ([True,False]). Bool now compares by
    // constructor order through the generator Ordering always used, and
    // Ordering's own row, reached through a compare result, keeps its answer.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "BoolCompareChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare False True, compare True False)\n",
            "(LT,GT)\n",
        ),
        (
            "BoolSortByCompareChirho.hs",
            "module Main where\nimport Data.List (sortBy)\nmain :: IO ()\nmain = print (sortBy compare [True, False])\n",
            "[False,True]\n",
        ),
        (
            "BoolListCompareChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare [False] [True])\n",
            "LT\n",
        ),
        (
            "OrderingResultCompareChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare (compare 1 2) (compare 2 (1 :: Int)))\n",
            "LT\n",
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
fn a_nested_list_order_still_names_its_dictionary_chirho() {
    // Element types are generated, not lists of lists: R2's proven `[[Int]]`
    // key keeps naming the obligation.
    assert_ghc_or_named_chirho(
        "NestedListCompareChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (compare [[1]] [[2 :: Int]])\n",
        "LT\n",
        "Ord [[Int]]",
    );
}

#[test]
fn a_missing_list_method_fails_by_name_where_its_row_is_found_by_name_chirho() {
    // GHC 9.14.1 compiles this with -Wmissing-methods, prints `[` and stops with
    // "No instance nor default method for class operation neededChirho". The
    // stub used to take the element dictionary first: `map` passed each list
    // as that dictionary, and the program printed [[1]] and succeeded. The
    // program is written before its file name, the oracle's GHC-fails shape.
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let outcome_chirho = eval_source_with_machine_chirho(
        "module Main where\nclass NeedsChirho a where\n  neededChirho :: a -> Int\ninstance NeedsChirho Int where\n  neededChirho x = x\ninstance NeedsChirho a => NeedsChirho [a]\nmain :: IO ()\nmain = print (map neededChirho [[1 :: Int]])\n",
        &mut source_map_chirho,
        "MissingListMethodUnderMapChirho.hs",
        None,
    );
    match outcome_chirho {
        Ok((_value_chirho, machine_chirho)) => panic!(
            "GHC fails at run time; we answered {:?}",
            machine_chirho.io_output_chirho
        ),
        Err(err_chirho) => assert!(
            err_chirho
                .to_string()
                .contains("missing method NeedsChirho.neededChirho for [Int]"),
            "the failure must name the missing method: {err_chirho}"
        ),
    }
}
