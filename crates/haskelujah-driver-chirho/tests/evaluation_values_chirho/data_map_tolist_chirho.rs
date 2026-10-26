// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Qualified `Data.Map` `fromList` and `toList` repair (row 534, gpt_chirho #28360 and #28371).
//! Hop A preserves the `mapFromList` builtin alias in `occurrence_head_replacement_chirho` when
//! the canonical target has no dictionary parameters, instead of leaving `M.fromList` on the
//! spine for `IsList.fromList`. Hop B makes `build_pair_list_chirho` allocate `"$tuple2"` pairs
//! matching Core/STG and `ShowShapeChirho::TupleChirho`.
//! workflow: language-features-chirho/own-class-method-occurrences-chirho

use super::run_chirho;

#[test]
fn qualified_data_map_from_list_and_to_list_answer_as_ghc_chirho() {
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "D01_qualified_data_map_tolist_chirho.hs",
            "module Main where\nimport qualified Data.Map as M\nmain :: IO ()\nmain = print (M.toList (M.fromList [(2, 'b'), (1, 'a') :: (Int, Char)]))\n",
            "[(1,'a'),(2,'b')]\n",
        ),
        (
            "M01_map_tolist_int_values_chirho.hs",
            "module Main where\nimport qualified Data.Map as M\nmain :: IO ()\nmain = print (M.toList (M.fromList [(2, 20), (1, 10 :: Int)] :: M.Map Int Int))\n",
            "[(1,10),(2,20)]\n",
        ),
        (
            "M02_map_tolist_insert_chirho.hs",
            "module Main where\nimport qualified Data.Map as M\nmain :: IO ()\nmain = print (M.toList (M.insert (2 :: Int) \"b\" (M.singleton 1 \"a\")))\n",
            "[(1,\"a\"),(2,\"b\")]\n",
        ),
        (
            "M03_map_tolist_char_annotated_map_chirho.hs",
            "module Main where\nimport qualified Data.Map as M\nmain :: IO ()\nmain = print (M.toList (M.fromList [(2, 'b'), (1, 'a')] :: M.Map Int Char))\n",
            "[(1,'a'),(2,'b')]\n",
        ),
        (
            "M04_map_tolist_string_values_chirho.hs",
            "module Main where\nimport qualified Data.Map as M\nmain :: IO ()\nmain = print (M.toList (M.fromList [(2 :: Int, \"b\"), (1, \"a\")]))\n",
            "[(1,\"a\"),(2,\"b\")]\n",
        ),
        (
            "M05_map_size_fromlist_chirho.hs",
            "module Main where\nimport qualified Data.Map as M\nmain :: IO ()\nmain = print (M.size (M.fromList [(2, \"b\"), (1, \"a\") :: (Int, String)]))\n",
            "2\n",
        ),
        (
            "M06_map_tolist_fst_projection_chirho.hs",
            "module Main where\nimport qualified Data.Map as M\nmain :: IO ()\nmain = let xsChirho = M.toList (M.fromList [(2, 'b'), (1, 'a') :: (Int, Char)]) in print (map fst xsChirho)\n",
            "[1,2]\n",
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
fn list_to_list_and_insert_from_list_holds_answer_as_ghc_chirho() {
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "M07_list_tolist_non_interference_chirho.hs",
            "module Main where\nimport GHC.Exts (toList)\nmain :: IO ()\nmain = print (toList [1, 2, 3 :: Int])\n",
            "[1,2,3]\n",
        ),
        (
            "M08_map_size_insert_fromlist_hold_chirho.hs",
            "module Main where\nimport qualified Data.Map as M\nmain :: IO ()\nmain = print (M.size (M.insert 2 \"b\" (M.fromList [(1, \"a\")])))\n",
            "2\n",
        ),
    ] {
        assert_eq!(
            run_chirho(file_name_chirho, program_chirho),
            ghc_chirho,
            "{file_name_chirho}"
        );
    }
    // Open boundaries outside row 534 (#28371):
    // - D02 is an unqualified explicit import `import Data.Map (fromList, toList)` (classified
    //   STAYS-WRONG from measurement: `[(2,'b'),(1,'a')]`, since `seed_builtin_import_aliases_chirho`
    //   binds only qualified `M.fromList` / `M.toList`).
    // - T07 and Q01 expose the local `toList` binder `CoreId` collision in
    //   `DesugarCtxChirho::resolve_or_fresh_id_chirho` on held unit 517, and fail on main with
    //   `missing STG binding ':|'` (classified STAYS-LOUD).
    // GHC's true output is recorded so the oracle extracts and measures all 12 programs;
    // nothing here asserts the current non-GHC behavior as a specification.
    let _open_boundaries_chirho = [
        (
            "D02_unqualified_data_map_tolist_chirho.hs",
            "module Main where\nimport Data.Map (fromList, toList)\nmain :: IO ()\nmain = print (toList (fromList [(2, 'b'), (1, 'a') :: (Int, Char)]))\n",
            "[(1,'a'),(2,'b')]\n",
        ),
        (
            "T07_lexical_shadows_beat_the_alias_chirho.hs",
            "module Main where\nimport Data.List.NonEmpty (NonEmpty(..), toList)\ncountChirho :: [Int] -> Int\ncountChirho toList = length toList\nmain :: IO ()\nmain = do { print (countChirho [1, 2, 3]); print ((\\toList -> toList) [5 :: Int]); print (toList (7 :| [8 :: Int])) }\n",
            "3\n[5]\n[7,8]\n",
        ),
        (
            "Q01_i1_with_typed_outer_tolist_chirho.hs",
            "module Main where\nimport Data.List.NonEmpty (NonEmpty(..), toList)\nmain :: IO ()\nmain = print (let toList = length in toList [1, 2, 3 :: Int], (toList (1 :| [2, 3 :: Int]) :: [Int]))\n",
            "(3,[1,2,3])\n",
        ),
    ];
}
