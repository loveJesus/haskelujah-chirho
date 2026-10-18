// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Equality sections (R1, gpt_chirho #27506, #27508). An `==` section whose
//! operator carries an origin is an occurrence of Eq's `==`, dispatched by the
//! checker's evidence like an infix `==`. It used to be the Int primop `==#`,
//! which compared lists as empty string prefixes and a newtype as its raw `Int`.
//! The ordering operators, `/=` and the arithmetic sections keep their primops.
//! Every expected output is GHC 9.14.1's.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_ghc_or_named_chirho, run_chirho};

#[test]
fn an_equality_section_answers_as_ghc_chirho() {
    // List sections printed [True,True], the newtype [False,False]; the Double
    // section and the user enum failed on the Int primop.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "SectionIntListChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (== [1]) [[1], [2 :: Int]], map ([1] ==) [[1], [2 :: Int]])\n",
            "([True,False],[True,False])\n",
        ),
        (
            "SectionNewtypeEqChirho.hs",
            "module Main where\nnewtype ModChirho = ModChirho Int\ninstance Eq ModChirho where\n  ModChirho a == ModChirho b = a `mod` 3 == b `mod` 3\nmain :: IO ()\nmain = print (map (== ModChirho 1) [ModChirho 4, ModChirho 2])\n",
            "[True,False]\n",
        ),
        (
            "SectionUserEnumEqChirho.hs",
            "module Main where\ndata ShadeChirho = LightChirho | DarkChirho\ninstance Eq ShadeChirho where\n  _ == _ = True\nmain :: IO ()\nmain = print (map (== LightChirho) [DarkChirho])\n",
            "[True]\n",
        ),
        (
            "SectionScalarsChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (== 1) [1, 2 :: Int], map (== 1.5) [1.5, 2.5 :: Double]) >> print (map (== 'a') \"ab\", map (== \"ab\") [\"ab\", \"cd\"])\n",
            "([True,False],[True,False])\n([True,False],[True,False])\n",
        ),
        (
            "SectionLazyChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (== [[1 :: Int]]) [])\n",
            "[]\n",
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
fn a_nested_equality_section_answers_as_ghc_or_names_its_dictionary_chirho() {
    // A nested section is proven `Eq [[Int]]`, which R2 serves or names. The
    // second program is the ground gate's exposure: it used to print a wrong
    // first line and a wrong third.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "SectionNestedChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (map (== [[1]]) [[[1]], [[2 :: Int]]])\n",
            "[True,False]\n",
        ),
        (
            "SectionExposureChirho.hs",
            "module Main where\nnestedChirho :: [[Int]]\nnestedChirho = [[1], [2]]\nmain :: IO ()\nmain = do\n  print (map (== [1]) nestedChirho)\n  print (map (\\x -> x == [1]) nestedChirho)\n  print ([2] == [1 :: Int], [[1]] == [[2 :: Int]])\n",
            "[True,False]\n[True,False]\n(False,False)\n",
        ),
    ] {
        assert_ghc_or_named_chirho(file_name_chirho, program_chirho, ghc_chirho, "Eq [[Int]]");
    }
}
