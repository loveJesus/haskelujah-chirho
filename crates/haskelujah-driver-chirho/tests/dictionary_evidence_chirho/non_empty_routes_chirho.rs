// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! NonEmpty values and the routed `toList` (unit 517, gpt_chirho #28473-#28520), end to end
//! through the interpreter. Every program is a GHC 9.14.1-measured file, embedded byte for
//! byte by `generate_controls_chirho.py`, with GHC's own stdout. Each test isolates one
//! mechanism, so a failure names its root:
//! - M1 + M3: `:|` is typed `a -> [a] -> NonEmpty a`, and NonEmpty's Show metadata renders it;
//! - M4: Data.List.NonEmpty's `toList`, qualified or bare, reaches the dual-shape route;
//! - M5: Data.Foldable's `toList` reaches the same route;
//! - the route's list alternatives keep every list value's answer unchanged.
//! workflow: language-features-chirho/own-class-method-occurrences-chirho

use super::run_chirho;

/// Run each program and require GHC's exact stdout.
fn assert_ghc_outputs_chirho(programs_chirho: &[(&str, &str, &str)]) {
    for (file_name_chirho, program_chirho, ghc_chirho) in programs_chirho {
        assert_eq!(
            run_chirho(file_name_chirho, program_chirho),
            *ghc_chirho,
            "{file_name_chirho}"
        );
    }
}

/// M1 + M3: a `:|` value has a concrete Show key, and the NonEmpty metadata renders it
/// infixr 5: bare, inside `Just`, a list, a tuple and itself, with a negative head.
#[test]
fn non_empty_values_show_as_ghc_shows_them_chirho() {
    assert_ghc_outputs_chirho(&[
        (
            "I03N1NonEmptyChirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.List.NonEmpty (NonEmpty (..))\nmain :: IO ()\nmain = print (1 :| [2, 3 :: Int])\n",
            "1 :| [2,3]\n",
        ),
        (
            "N1_print_nonempty_value_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (1 :| [2, 3 :: Int])\n",
            "1 :| [2,3]\n",
        ),
        (
            "P04_print_cons_in_just_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (Just (1 :| [2 :: Int]))\n",
            "Just (1 :| [2])\n",
        ),
        (
            "P05_print_negative_head_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (negate 1 :| [2 :: Int])\n",
            "-1 :| [2]\n",
        ),
        (
            "P06_print_list_of_cons_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print [1 :| [2 :: Int], 3 :| []]\n",
            "[1 :| [2],3 :| []]\n",
        ),
        (
            "P09_show_cons_value_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = putStrLn (show (2 :| [3 :: Int]))\n",
            "2 :| [3]\n",
        ),
        (
            "P10_print_nested_cons_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print ((1 :| [2 :: Int]) :| [])\n",
            "(1 :| [2]) :| []\n",
        ),
        (
            "Q02_p07_with_typed_nonempty_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (((1 :: Int) :| []) :: NonEmpty Int, 'c')\n",
            "(1 :| [],'c')\n",
        ),
    ]);
}

/// M4, qualified: every qualifier an import of Data.List.NonEmpty binds `toList` under
/// (alias, full module name, explicit list, `hiding`) reaches the route, first-class too.
#[test]
fn qualified_non_empty_to_list_takes_the_route_chirho() {
    assert_ghc_outputs_chirho(&[
        (
            "B03_tolist_real_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (NE.toList (1 :| [2, 3 :: Int]))\n",
            "[1,2,3]\n",
        ),
        (
            "K07a_unqualified_operator_qualified_tolist_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (NE.toList (1 :| [2, 3 :: Int]))\n",
            "[1,2,3]\n",
        ),
        (
            "B20_unqualified_alias_import_tolist_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nimport Data.List.NonEmpty as NE hiding (head, tail, map)\nmain :: IO ()\nmain = print (NE.toList (1 :| [2, 3 :: Int]))\n",
            "[1,2,3]\n",
        ),
        (
            "C02_first_class_tolist_under_map_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\n\nmain :: IO ()\nmain = print (map NE.toList [1 :| [2], 3 :| [] :: NonEmpty Int])\n",
            "[[1,2],[3]]\n",
        ),
        (
            "I4_alias_n_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as N\nmain :: IO ()\nmain = print (N.toList (1 N.:| [2 :: Int]))\n",
            "[1,2]\n",
        ),
        (
            "I5_unaliased_full_qualifier_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty\nmain :: IO ()\nmain = print (Data.List.NonEmpty.toList (1 Data.List.NonEmpty.:| [2 :: Int]))\n",
            "[1,2]\n",
        ),
        (
            "RT06_qualified_explicit_list_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE (NonEmpty(..), toList)\nmain :: IO ()\nmain = print (NE.toList (6 NE.:| [7 :: Int]))\n",
            "[6,7]\n",
        ),
        (
            "RT07_qualified_hiding_head_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE hiding (head)\nmain :: IO ()\nmain = print (NE.toList (6 NE.:| [7 :: Int]))\n",
            "[6,7]\n",
        ),
    ]);
}

/// M4, unqualified: the bare `toList` of an unqualified import reaches the route, also
/// beside a Data.Foldable import whose own `toList` is hidden.
#[test]
fn unqualified_non_empty_to_list_takes_the_route_chirho() {
    assert_ghc_outputs_chirho(&[
        (
            "U1_unqualified_tolist_of_cons_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.List.NonEmpty (NonEmpty(..), toList)\nmain :: IO ()\nmain = print (toList (1 :| [2, 3 :: Int]))\n",
            "[1,2,3]\n",
        ),
        (
            "U1T_typed_unqualified_tolist_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.List.NonEmpty (NonEmpty(..), toList)\nmain :: IO ()\nmain = print (toList ((1 :| [2, 3]) :: NonEmpty Int))\n",
            "[1,2,3]\n",
        ),
        (
            "T06_unqualified_as_keeps_both_spellings_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.List.NonEmpty as NE (NonEmpty(..), toList)\nmain :: IO ()\nmain = do { print (toList (1 :| [2 :: Int])); print (NE.toList (3 :| [4 :: Int])) }\n",
            "[1,2]\n[3,4]\n",
        ),
        (
            "RT04_foldable_hiding_all_members_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.Foldable hiding (Foldable(..))\nimport Data.List.NonEmpty (NonEmpty(..), toList)\nmain :: IO ()\nmain = print (toList (4 :| [5 :: Int]))\n",
            "[4,5]\n",
        ),
        (
            "RT05_foldable_hiding_listed_member_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.Foldable hiding (Foldable(toList))\nimport Data.List.NonEmpty (NonEmpty(..), toList)\nmain :: IO ()\nmain = print (toList (4 :| [5 :: Int]))\n",
            "[4,5]\n",
        ),
    ]);
}

/// M5: Data.Foldable's `toList` on a NonEmpty value, however the value is bound and
/// however the import names `toList` (plain, `Foldable(..)`, `Foldable(toList)`, qualified).
#[test]
fn foldable_to_list_of_a_non_empty_takes_the_route_chirho() {
    assert_ghc_outputs_chirho(&[
        (
            "X36_foldable_tolist_of_cons_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.List.NonEmpty (NonEmpty(..))\nimport Data.Foldable (toList)\nmain :: IO ()\nmain = print (toList (1 :| [2 :: Int]))\n",
            "[1,2]\n",
        ),
        (
            "X36G_signature_bound_foldable_tolist_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.List.NonEmpty (NonEmpty(..))\nimport Data.Foldable (toList)\nvalueChirho :: NonEmpty Int\nvalueChirho = 1 :| [2]\nmain :: IO ()\nmain = print (toList valueChirho)\n",
            "[1,2]\n",
        ),
        (
            "X36L_let_bound_foldable_tolist_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.List.NonEmpty (NonEmpty(..))\nimport Data.Foldable (toList)\nmain :: IO ()\nmain = let valueChirho = 1 :| [2 :: Int] in print (toList valueChirho)\n",
            "[1,2]\n",
        ),
        (
            "X36T_typed_foldable_tolist_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.List.NonEmpty (NonEmpty(..))\nimport Data.Foldable (toList)\nmain :: IO ()\nmain = print (toList ((1 :| [2]) :: NonEmpty Int))\n",
            "[1,2]\n",
        ),
        (
            "RT01_foldable_all_members_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.Foldable (Foldable(..))\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (toList (1 :| [2, 3 :: Int]))\n",
            "[1,2,3]\n",
        ),
        (
            "RT02_foldable_listed_member_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.Foldable (Foldable(toList))\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (toList (1 :| [2, 3 :: Int]))\n",
            "[1,2,3]\n",
        ),
        (
            "RT03_qualified_foldable_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.Foldable as F\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (F.toList (1 :| [2 :: Int]))\n",
            "[1,2]\n",
        ),
    ]);
}

/// The route's list alternatives come first: a list built by `NE.fromList` or `NE.map`,
/// a String, and Data.Foldable's `toList` on a list all keep the answer they had.
#[test]
fn list_values_keep_their_to_list_answers_chirho() {
    assert_ghc_outputs_chirho(&[
        (
            "G4_1_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nmain :: IO ()\nmain = print (NE.toList (NE.fromList [1, 2, 3 :: Int]))\n",
            "[1,2,3]\n",
        ),
        (
            "B07_tolist_of_map_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (NE.toList (NE.map (+1) (NE.fromList [1, 2 :: Int])))\n",
            "[2,3]\n",
        ),
        (
            "B08_tolist_of_fromlist_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (NE.toList (NE.fromList [1, 2 :: Int]))\n",
            "[1,2]\n",
        ),
        (
            "B16_tolist_tail_compose_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\nmain :: IO ()\nmain = print (NE.toList (NE.fromList (NE.tail (NE.fromList [1, 2, 3 :: Int]))))\n",
            "[2,3]\n",
        ),
        (
            "B21_tolist_of_string_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\n\nmain :: IO ()\nmain = putStrLn (NE.toList (NE.fromList \"ab\"))\n",
            "ab\n",
        ),
        (
            "B23_tolist_reverse_composition_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\n\nmain :: IO ()\nmain = print (NE.toList (NE.reverse (NE.fromList [1, 2 :: Int])))\n",
            "[2,1]\n",
        ),
        (
            "E02_typed_tolist_of_map_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport qualified Data.List.NonEmpty as NE\nimport Data.List.NonEmpty (NonEmpty(..))\n\nneHeadChirho :: NonEmpty a -> a\nneHeadChirho = NE.head\nneTailChirho :: NonEmpty a -> [a]\nneTailChirho = NE.tail\nneToListChirho :: NonEmpty a -> [a]\nneToListChirho = NE.toList\nmain :: IO ()\nmain = print (neToListChirho (NE.map (+1) (NE.fromList [1, 2 :: Int])))\n",
            "[2,3]\n",
        ),
        (
            "L0_foldable_tolist_of_list_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.Foldable (toList)\nmain :: IO ()\nmain = print (toList [1, 2 :: Int])\n",
            "[1,2]\n",
        ),
        (
            "S01_foldable_tolist_of_string_chirho.hs",
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Data.Foldable (toList)\nmain :: IO ()\nmain = putStrLn (toList \"ab\")\n",
            "ab\n",
        ),
    ]);
}
