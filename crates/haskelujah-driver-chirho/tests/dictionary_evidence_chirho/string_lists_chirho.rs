// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `Eq [String]`, generated (gpt_chirho A4 disposition). The conditional
//! generator builds `Eq [[Char]]` from `Eq a => Eq [a]` beside `[Int]`, `[Char]`,
//! `[Bool]` and `[Double]`, comparing each string with `==#`, so a proven
//! `[String]` key now has the dictionary R2 looks for. Before it, a String-list
//! section named `Eq [String]`; ground's `==#` section, applied to the whole
//! `[String]` values, compared only their first strings and was wrong whenever
//! those agreed. `Ord [[Char]]` is generated the same way, comparing each
//! string with the `compareStr#` row (gpt_chirho disposition on #28041).
//! Every expected output is GHC 9.14.1's.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_ghc_or_named_chirho, run_chirho};

#[test]
fn a_string_list_comparison_answers_as_ghc_chirho() {
    // Ground printed [True,True], [True], [True] and [True,True,False] for the
    // second to fifth rows, and the combined R1/R2b CLI named `Eq [String]`.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "StringListSectionChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (== [\"a\"]) [[\"a\"], [\"b\"]])\n",
            "[True,False]\n",
        ),
        (
            "StringListSecondStringChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (== [\"a\", \"x\"]) [[\"a\", \"y\"], [\"a\", \"x\"]])\n",
            "[False,True]\n",
        ),
        (
            "StringListLaterDiffersChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (== [\"a\", \"b\"]) [[\"a\", \"c\"]])\n",
            "[False]\n",
        ),
        (
            "StringListLongerChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (== [\"a\"]) [[\"a\", \"b\"]])\n",
            "[False]\n",
        ),
        (
            "StringListLengthsChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (== [\"a\", \"b\"]) [[\"a\", \"b\"], [\"a\"], []])\n",
            "[True,False,False]\n",
        ),
        (
            "StringListInfixChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ([\"a\", \"x\"] == [\"a\", \"y\"], [\"a\", \"x\"] == [\"a\", \"x\"])\n",
            "(False,True)\n",
        ),
        (
            "StringListFilterChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (length (filter (== [\"a\"]) [[\"a\"], [\"b\"], [\"a\"]]))\n",
            "2\n",
        ),
        (
            // The generated `Ord [[Char]]` (gpt_chirho disposition on #28041):
            // ground crashed here and the combined CLI named `Ord [String]`.
            "StringListOrdChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare [\"a\"] [\"b\"])\n",
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
fn a_deeper_string_list_still_names_its_dictionary_chirho() {
    // String elements are generated, not lists of them: a list of String lists
    // keeps naming the obligation (ground printed [True,True]).
    assert_ghc_or_named_chirho(
        "NestedStringListChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print (map (== [[\"a\"]]) [[[\"a\"]], [[\"b\"]]])\n",
        "[True,False]\n",
        "Eq [[String]]",
    );
}
