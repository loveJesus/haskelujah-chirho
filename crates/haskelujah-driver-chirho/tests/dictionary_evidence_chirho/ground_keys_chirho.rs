// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ground list keys and complete dictionary arguments. A predicate at a list
//! type whose parts are all ground keeps its whole key (`[Int]`, `[Char]`), the
//! name the dictionary registry gives list dictionaries, in reference evidence
//! and method occurrences alike; and a constrained call receives a dictionary
//! for every predicate, or an explicit error naming the one it lacks, forced
//! only when used. Before, `Ord [Int]` was keyed `[]`, which names nothing,
//! and the missing argument let the next term argument take the dictionary's
//! place: those programs printed nothing more and exited 0. Every expected
//! output is GHC 9.14.1's.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

use super::run_chirho;

const SAME_CHIRHO: &str = "module Main where\nsameChirho :: Ord a => [a] -> Bool\nsameChirho (x:y:_) = x == y\nsameChirho _ = False\ndoublesChirho :: [Double]\ndoublesChirho = [1.5, 2.5]\nnestedChirho :: [[Int]]\nnestedChirho = [[1], [2]]\nwordsChirho :: [String]\nwordsChirho = [\"ab\", \"ab\"]\ntripleChirho :: [[[Int]]]\ntripleChirho = [[[1]], [[1]]]\nemptyTripleChirho :: [[[Int]]]\nemptyTripleChirho = []\nmain :: IO ()\n";

fn program_chirho(main_chirho: &str) -> String {
    format!("{SAME_CHIRHO}{main_chirho}\n")
}

/// The run's output, or the error that stopped it.
fn run_result_chirho(file_name_chirho: &str, source_chirho: &str) -> Result<String, String> {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        None,
    )
    .map(|(_value_chirho, machine_chirho)| machine_chirho.io_output_chirho)
    .map_err(|err_chirho| format!("{err_chirho}"))
}

#[test]
fn a_signed_nested_list_reaches_its_element_dictionary_chirho() {
    // T6c: `Ord [Int]` for a `[[Int]]` variable. It used to print "before"
    // and stop, exit 0.
    assert_eq!(
        run_chirho(
            "NestedVariableChirho.hs",
            &program_chirho(
                "main = do { putStrLn \"before\"; print (sameChirho nestedChirho); putStrLn \"after\" }"
            ),
        ),
        "before\nFalse\nafter\n"
    );
}

#[test]
fn string_lists_and_scalars_keep_their_dictionaries_chirho() {
    // `String` is keyed `[Char]`, as its dictionaries are named. A signed
    // `[String]` variable printed nothing before; the others are controls.
    for (file_name_chirho, main_chirho, expected_chirho) in [
        (
            "StringListChirho.hs",
            "main = print (sameChirho wordsChirho, sameChirho [\"ab\", \"cd\"])",
            "(True,False)\n",
        ),
        (
            "DoubleVariableChirho.hs",
            "main = print (sameChirho doublesChirho)",
            "False\n",
        ),
        (
            "NestedLiteralChirho.hs",
            "main = print (sameChirho [[1], [1 :: Int]], sameChirho [[1], [2 :: Int]])",
            "(True,False)\n",
        ),
        (
            "ScalarChirho.hs",
            "main = print (sameChirho [1, 1 :: Int], sameChirho \"ab\")",
            "(True,False)\n",
        ),
    ] {
        assert_eq!(
            run_chirho(file_name_chirho, &program_chirho(main_chirho)),
            expected_chirho,
            "{file_name_chirho}"
        );
    }
}

#[test]
fn a_constrained_function_passed_on_carries_its_dictionary_chirho() {
    // Higher-order: the reference `sameChirho` is applied by `map`, at `[Int]`
    // elements. It printed nothing before.
    assert_eq!(
        run_chirho(
            "HigherOrderChirho.hs",
            &program_chirho("main = print (map sameChirho [nestedChirho, [[3], [3]]])"),
        ),
        "[False,True]\n"
    );
}

#[test]
fn a_method_at_a_ground_list_dispatches_on_the_whole_key_chirho() {
    // `== [1]` at `[Int]` elements: the occurrence keyed `[]` answered
    // `[True,True]`.
    assert_eq!(
        run_chirho(
            "SectionAtListChirho.hs",
            &program_chirho("main = print (map (== [1]) nestedChirho)"),
        ),
        "[True,False]\n"
    );
}

#[test]
fn a_dictionary_nobody_can_build_fails_by_name_only_when_forced_chirho() {
    // `Ord [[Int]]` has no dictionary yet. Forced, the run must stop with an
    // error naming that obligation, or print GHC's `True` once support exists;
    // it printed nothing and exited 0.
    match run_result_chirho(
        "MissingDictionaryForcedChirho.hs",
        &program_chirho("main = print (sameChirho tripleChirho)"),
    ) {
        Ok(output_chirho) => assert_eq!(output_chirho, "True\n"),
        Err(err_chirho) => assert!(
            err_chirho.contains("no instance dictionary for `Ord [[Int]]`"),
            "the error must name the obligation: {err_chirho}"
        ),
    }
    // Never forced: an unused call, and a call whose body never reaches the
    // dictionary (the empty list), run as in GHC.
    for (file_name_chirho, main_chirho, expected_chirho) in [
        (
            "MissingDictionaryUnusedChirho.hs",
            "main = let unusedChirho = sameChirho tripleChirho in putStrLn \"ok\"",
            "ok\n",
        ),
        (
            "MissingDictionaryUnforcedChirho.hs",
            "main = print (sameChirho emptyTripleChirho)",
            "False\n",
        ),
    ] {
        assert_eq!(
            run_chirho(file_name_chirho, &program_chirho(main_chirho)),
            expected_chirho,
            "{file_name_chirho}"
        );
    }
}

#[test]
fn a_method_at_a_list_without_a_dictionary_still_fails_loudly_chirho() {
    // `==` at `[[Int]]` has no dictionary or method row yet. GHC prints
    // `True`; until that support exists the run must fail with an error, never
    // a wrong answer or a silent exit.
    match run_result_chirho(
        "MethodWithoutDictionaryChirho.hs",
        &program_chirho("main = print (nestedChirho == nestedChirho)"),
    ) {
        Ok(output_chirho) => assert_eq!(output_chirho, "True\n"),
        Err(err_chirho) => assert!(!err_chirho.is_empty()),
    }
}
