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

/// `halfChirho` used infix (no reference evidence) inside a call keyed `Int`.
fn partial_hit_program_chirho(
    context_chirho: &str,
    body_chirho: &str,
    main_chirho: &str,
) -> String {
    format!(
        "module Main where\nhalfChirho :: ({context_chirho}) => a -> a -> String\nhalfChirho x y = {body_chirho}\nouterChirho :: Show b => Int -> b -> String\nouterChirho n x = show n ++ \"|\" ++ show x\nmain :: IO ()\n{main_chirho}\n"
    )
}

#[test]
fn a_partial_dictionary_hit_fills_every_slot_chirho() {
    // The typed-argument site found `Show` at the surrounding `Int` key and no
    // `Fractional`, and returned a spine with one dictionary: the term
    // arguments slid into the slots and GHC's `5|"1.5"` printed as
    // `5|$PAP halfChirho …`, exit 0, in BOTH predicate orders. Every slot is
    // now filled: the missing one fails by name when forced, or the run prints
    // GHC's answer once that support exists.
    for (file_name_chirho, context_chirho) in [
        ("FirstServedChirho.hs", "Show a, Fractional a"),
        ("FirstMissingChirho.hs", "Fractional a, Show a"),
    ] {
        match run_result_chirho(
            file_name_chirho,
            &partial_hit_program_chirho(
                context_chirho,
                "show (x / y)",
                "main = putStrLn (outerChirho 5 (3.0 `halfChirho` (2.0 :: Double)))",
            ),
        ) {
            Ok(output_chirho) => assert_eq!(output_chirho, "5|\"1.5\"\n", "{file_name_chirho}"),
            Err(err_chirho) => assert!(
                err_chirho.contains("no instance dictionary for `Fractional"),
                "{file_name_chirho}: the error must name the missing slot: {err_chirho}"
            ),
        }
    }
}

#[test]
fn a_filled_slot_nobody_forces_stays_silent_chirho() {
    // Laziness: a callee that never touches either dictionary, and a call that
    // is never forced, run exactly as in GHC.
    for (file_name_chirho, body_chirho, main_chirho, expected_chirho) in [
        (
            "SlotsNeverForcedChirho.hs",
            "\"label\"",
            "main = putStrLn (outerChirho 5 (3.0 `halfChirho` (2.0 :: Double)))",
            "5|\"label\"\n",
        ),
        (
            "PartialCallUnusedChirho.hs",
            "show (x / y)",
            "main = let unusedChirho = outerChirho 5 (3.0 `halfChirho` (2.0 :: Double)) in putStrLn \"ok\"",
            "ok\n",
        ),
    ] {
        assert_eq!(
            run_chirho(
                file_name_chirho,
                &partial_hit_program_chirho("Show a, Fractional a", body_chirho, main_chirho),
            ),
            expected_chirho,
            "{file_name_chirho}"
        );
    }
}

#[test]
fn local_bindings_keep_the_whole_list_key_chirho() {
    // agy_chirho #27318 A1/A2: a local `let` or `where` binding specialized at
    // one ground list type was keyed `[]` by its local specialization. A1 and
    // the `where` form printed nothing, exit 0; A2's local `==` was a missing
    // binding.
    for (file_name_chirho, source_chirho, expected_chirho) in [
        (
            "LocalLetListChirho.hs",
            "module Main where\nsameChirho :: Ord a => [a] -> Bool\nsameChirho (x : y : _) = x == y\nsameChirho _ = False\nnestedIntsChirho :: [[Int]]\nnestedIntsChirho = [[1, 2], [1, 3]]\nwordsChirho :: [String]\nwordsChirho = [\"aa\", \"aa\"]\nmain :: IO ()\nmain = do\n  let fIntChirho = sameChirho\n  let fStrChirho xs = sameChirho xs\n  print (fIntChirho nestedIntsChirho, fStrChirho wordsChirho)\n",
            "(False,True)\n",
        ),
        (
            "LocalWhereListChirho.hs",
            "module Main where\nsameChirho :: Ord a => [a] -> Bool\nsameChirho (x : y : _) = x == y\nsameChirho _ = False\nnestedIntsChirho :: [[Int]]\nnestedIntsChirho = [[1, 2], [1, 3]]\nwhereCallChirho :: Bool\nwhereCallChirho = helperChirho nestedIntsChirho\n  where\n    helperChirho = sameChirho\nmain :: IO ()\nmain = print whereCallChirho\n",
            "False\n",
        ),
        (
            "LocalOccurrenceListChirho.hs",
            "module Main where\nxsChirho :: [Int]\nxsChirho = [1, 2]\nysChirho :: [Int]\nysChirho = [1, 3]\nmain :: IO ()\nmain = do\n  let eqListChirho a b = a == b\n  print (eqListChirho xsChirho ysChirho)\n",
            "False\n",
        ),
    ] {
        assert_eq!(
            run_chirho(file_name_chirho, source_chirho),
            expected_chirho,
            "{file_name_chirho}"
        );
    }
}

const TAG_CLASS_CHIRHO: &str = "{-# LANGUAGE FlexibleInstances #-}\nmodule Main where\nclass TagChirho a where\n  tagChirho :: a -> String\nuseTagChirho :: TagChirho a => a -> String\nuseTagChirho x = tagChirho x\n";

#[test]
fn a_nested_string_meets_its_instance_under_either_spelling_chirho() {
    // agy_chirho #27318 A3: an instance head registers as it displays
    // (`instance TagChirho [[Char]]` is `$fTagChirho[[Char]]`, and so is one
    // written `[String]`), while the evidence spells the nested string
    // `[String]`. Through a constrained reference and as a direct method use,
    // either spelling must meet; a user type whose name merely contains
    // `String` keeps its own key. Each printed an empty line or failed.
    for (file_name_chirho, declarations_chirho, expected_chirho) in [
        (
            "NestedCharsByReferenceChirho.hs",
            "instance TagChirho [[Char]] where\n  tagChirho _ = \"nested-chars\"\nwordsChirho :: [String]\nwordsChirho = [\"hi\", \"there\"]\nmain :: IO ()\nmain = putStrLn (useTagChirho wordsChirho)\n",
            "nested-chars\n",
        ),
        (
            "NestedCharsByOccurrenceChirho.hs",
            "instance TagChirho [[Char]] where\n  tagChirho _ = \"nested-chars\"\nwordsChirho :: [String]\nwordsChirho = [\"hi\", \"there\"]\nmain :: IO ()\nmain = putStrLn (tagChirho wordsChirho)\n",
            "nested-chars\n",
        ),
        (
            "WrittenStringListChirho.hs",
            "instance TagChirho [String] where\n  tagChirho _ = \"string-list\"\ncharsChirho :: [[Char]]\ncharsChirho = [\"hi\"]\nmain :: IO ()\nmain = putStrLn (useTagChirho charsChirho)\n",
            "string-list\n",
        ),
        (
            "StringInATypeNameChirho.hs",
            "data StringBoxChirho = StringBoxChirho\ninstance TagChirho [StringBoxChirho] where\n  tagChirho _ = \"boxes\"\nboxesChirho :: [StringBoxChirho]\nboxesChirho = [StringBoxChirho]\nmain :: IO ()\nmain = putStrLn (useTagChirho boxesChirho)\n",
            "boxes\n",
        ),
    ] {
        assert_eq!(
            run_chirho(
                file_name_chirho,
                &format!("{TAG_CLASS_CHIRHO}{declarations_chirho}")
            ),
            expected_chirho,
            "{file_name_chirho}"
        );
    }
}

#[test]
fn unforced_missing_dictionaries_in_every_lazy_position_chirho() {
    // agy_chirho #27318 A4: the missing `Ord [[Int]]` dictionary in an unused
    // `let`, an unused higher-order `map`, under `const`, and at an empty list.
    // It printed "const-ok" and stopped, exit 0.
    assert_eq!(
        run_chirho(
            "LazyPositionsChirho.hs",
            "module Main where\nsameChirho :: Ord a => [a] -> Bool\nsameChirho (x : y : _) = x == y\nsameChirho _ = False\ntripleChirho :: [[[Int]]]\ntripleChirho = [[[1]], [[2]]]\nemptyTripleChirho :: [[[Int]]]\nemptyTripleChirho = []\nmain :: IO ()\nmain = do\n  let unusedChirho = sameChirho tripleChirho\n  let unusedHigherOrderChirho = map sameChirho [tripleChirho]\n  putStrLn (const \"const-ok\" (sameChirho tripleChirho))\n  print (sameChirho emptyTripleChirho)\n",
        ),
        "const-ok\nFalse\n"
    );
}
