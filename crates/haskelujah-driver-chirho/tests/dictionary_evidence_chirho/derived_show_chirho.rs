// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Derived Show at GHC's precedence (row 533, gpt_chirho #28285 and #28295). Derived Show
//! used to build `show` by concatenation, so a nested constructor, a negative field or a
//! record printed differently from GHC. Each program's GHC 9.14.1 output is recorded beside
//! it. Infix constructors cannot be built end to end on this main; their structure is proved
//! by the Core unit tests in dict_chirho/show_chirho/derived_tests_chirho.rs.
//! workflow: print-show-evidence-chirho

use super::run_chirho;

#[test]
fn derived_show_answers_as_ghc_at_every_precedence_chirho() {
    // Before: PChirho TChirho 1 TChirho 2, TChirho -1, TChirho Just 1, RChirho 1 True,
    // TChirho -1.5, Just (RChirho 1 True), [TChirho -3] and RChirho -1.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "S01NestedPrefixArgChirho.hs",
            "module Main where\ndata TChirho = TChirho Int deriving Show\ndata PChirho = PChirho TChirho TChirho deriving Show\nmain :: IO ()\nmain = print (PChirho (TChirho 1) (TChirho 2))\n",
            "PChirho (TChirho 1) (TChirho 2)\n",
        ),
        (
            "S03NegativeIntFieldChirho.hs",
            "module Main where\ndata TChirho = TChirho Int deriving Show\nmain :: IO ()\nmain = print (TChirho (-1))\n",
            "TChirho (-1)\n",
        ),
        (
            "S04MaybeFieldChirho.hs",
            "module Main where\ndata TChirho = TChirho (Maybe Int) deriving Show\nmain :: IO ()\nmain = print (TChirho (Just 1))\n",
            "TChirho (Just 1)\n",
        ),
        (
            "S06RecordChirho.hs",
            "module Main where\ndata RChirho = RChirho { xChirho :: Int, yChirho :: Bool } deriving Show\nmain :: IO ()\nmain = print (RChirho 1 True)\n",
            "RChirho {xChirho = 1, yChirho = True}\n",
        ),
        (
            "S11NegativeDoubleFieldChirho.hs",
            "module Main where\ndata TChirho = TChirho Double deriving Show\nmain :: IO ()\nmain = print (TChirho (-1.5))\n",
            "TChirho (-1.5)\n",
        ),
        (
            "S18RecordInMaybeChirho.hs",
            "module Main where\ndata RChirho = RChirho { xChirho :: Int, yChirho :: Bool } deriving Show\nmain :: IO ()\nmain = print (Just (RChirho 1 True))\n",
            "Just (RChirho {xChirho = 1, yChirho = True})\n",
        ),
        (
            "C05DerivedInListNegativeChirho.hs",
            "module Main where\ndata TChirho = TChirho Int deriving Show\nmain :: IO ()\nmain = print [TChirho (-3)]\n",
            "[TChirho (-3)]\n",
        ),
        (
            "D03RecordNegativeFieldChirho.hs",
            "module Main where\ndata RChirho = RChirho { xChirho :: Int } deriving Show\nmain :: IO ()\nmain = print (RChirho (-1))\n",
            "RChirho {xChirho = -1}\n",
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
fn a_monomorphic_derived_show_reached_through_a_dictionary_answers_as_ghc_chirho() {
    // The instance dictionary's show is the type's public row, so polymorphic code reaches
    // the same rendering. Before: TChirho 1TChirho -2. A type with parameters reached
    // through a dictionary stays outside this unit.
    assert_eq!(
        run_chirho(
            "C04DerivedViaPolymorphicChirho.hs",
            "module Main where\ndata TChirho = TChirho Int deriving Show\nshowAllChirho :: Show a => [a] -> String\nshowAllChirho = concatMap show\nmain :: IO ()\nmain = putStrLn (showAllChirho [TChirho 1, TChirho (-2)])\n"
        ),
        "TChirho 1TChirho (-2)\n"
    );
}

#[test]
fn derived_show_rows_that_already_answered_as_ghc_still_do_chirho() {
    // These printed GHC's text before, through the evidenced shapes, the space heuristic of
    // the runtime renderer, or the plain show; each now renders through the derived helper.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "S02MaybeOfPrefixChirho.hs",
            "module Main where\ndata TChirho = TChirho Int deriving Show\nmain :: IO ()\nmain = print (Just (TChirho 1))\n",
            "Just (TChirho 1)\n",
        ),
        (
            "S05ListFieldChirho.hs",
            "module Main where\ndata TChirho = TChirho [Int] deriving Show\nmain :: IO ()\nmain = print (TChirho [1, 2])\n",
            "TChirho [1,2]\n",
        ),
        (
            "S07ListOfConsChirho.hs",
            "module Main where\ndata TChirho = AChirho | BChirho Int deriving Show\nmain :: IO ()\nmain = print [AChirho, BChirho 2]\n",
            "[AChirho,BChirho 2]\n",
        ),
        (
            "S08TupleFieldChirho.hs",
            "module Main where\ndata TChirho = TChirho (Int, Int) deriving Show\nmain :: IO ()\nmain = print (TChirho (1, 2))\n",
            "TChirho (1,2)\n",
        ),
        (
            "S10PolyNestedChirho.hs",
            "module Main where\ndata TChirho a = TChirho a deriving Show\nmain :: IO ()\nmain = print (TChirho (TChirho 'c'))\n",
            "TChirho (TChirho 'c')\n",
        ),
        (
            "S12StringFieldChirho.hs",
            "module Main where\ndata TChirho = TChirho String deriving Show\nmain :: IO ()\nmain = print (TChirho \"hi\")\n",
            "TChirho \"hi\"\n",
        ),
        (
            "S14EitherOfPrefixChirho.hs",
            "module Main where\ndata TChirho = TChirho Int deriving Show\nmain :: IO ()\nmain = print (Left (TChirho 1) :: Either TChirho Int)\n",
            "Left (TChirho 1)\n",
        ),
        (
            "S15TupleOfPrefixChirho.hs",
            "module Main where\ndata TChirho = TChirho Int deriving Show\nmain :: IO ()\nmain = print (TChirho 1, TChirho 2)\n",
            "(TChirho 1,TChirho 2)\n",
        ),
        (
            "S16NullaryFieldChirho.hs",
            "module Main where\ndata CChirho = RedChirho | GreenChirho deriving Show\ndata TChirho = TChirho CChirho deriving Show\nmain :: IO ()\nmain = print (TChirho RedChirho)\n",
            "TChirho RedChirho\n",
        ),
        (
            "S17ShowStringChirho.hs",
            "module Main where\ndata TChirho = TChirho Int deriving Show\nmain :: IO ()\nmain = print (show (TChirho 1))\n",
            "\"TChirho 1\"\n",
        ),
    ] {
        assert_eq!(
            run_chirho(file_name_chirho, program_chirho),
            ghc_chirho,
            "{file_name_chirho}"
        );
    }
}
