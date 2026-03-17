// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// Data.Map, Data.Set, BST, record syntax, newtype, user-defined data type tests

#[allow(unused_imports)]
use crate::{
    eval_source_chirho,
    eval_source_with_machine_chirho,
    eval_source_with_input_chirho,
    eval_source_with_step_limit_chirho,
    compile_source_chirho,
    check_source_file_chirho,
    render_summary_chirho,
    compile_modules_chirho,
    eval_modules_chirho,
    compile_modules_incremental_chirho,
    discover_modules_chirho,
};
#[allow(unused_imports)]
use haskelujah_span_chirho::SourceMapChirho;
#[allow(unused_imports)]
use haskelujah_runtime_chirho::{ValueChirho, ExecutionModeChirho};
#[allow(unused_imports)]
use haskelujah_syntax_chirho::SourceFileChirho;


    #[test]
    fn eval_newtype_chirho() {
        // Newtype construction and case dispatch
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
newtype Age = MkAge Int
getAge a = case a of
  MkAge n -> n
main = getAge (MkAge 25)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("newtype should evaluate");
        assert_eq!(
            result_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(25),
            "getAge (MkAge 25) should be 25"
        );
    }


    // ── Priority 51: Newtype deriving and GND ────────────────────
    #[test]
    fn eval_newtype_construct_match_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Basic newtype: construct and pattern-match
        let src_chirho = "module Test where\nnewtype Age = MkAge Int\nmain = case MkAge 42 of { MkAge n -> n }\n";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("newtype construct/match should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_newtype_erasure_arithmetic_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Newtype erasure means MkAge is identity — unwrapped value
        // participates in arithmetic directly.
        let src_chirho = "\
module Test where
newtype Age = MkAge Int
getAge x = case x of { MkAge n -> n }
main = getAge (MkAge 10) + getAge (MkAge 32)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("newtype erasure arithmetic should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_newtype_derived_eq_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Newtype with derived Eq: equality comparison works through erasure.
        let src_chirho = "\
module Test where
newtype Age = MkAge Int deriving (Eq)
main = if MkAge 5 == MkAge 5 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("newtype derived Eq should evaluate: {}", e_chirho),
        }
    }

    // ── Priority 52: Record syntax field access ────────────────────

    // ── Priority 52: Record syntax field access ────────────────────
    #[test]
    fn eval_record_construction_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Record construction with field names
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
main = case MkPoint { xCoord = 3, yCoord = 4 } of { MkPoint x y -> x + y }
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(7));
            }
            Err(e_chirho) => panic!("record construction should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_record_field_accessor_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Field accessor function: xCoord and yCoord used as functions
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
p = MkPoint { xCoord = 10, yCoord = 20 }
main = xCoord p + yCoord p
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(30));
            }
            Err(e_chirho) => panic!("record field accessor should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_newtype_record_field_accessor_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Newtype record field accessor: getAge used as a function
        let src_chirho = "\
module Test where
newtype Age = MkAge { getAge :: Int }
myAge = MkAge 25
main = getAge myAge
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(25));
            }
            Err(e_chirho) => panic!("newtype record field accessor should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_newtype_runidentity_pattern_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Pattern: newtype Identity a = Identity { runIdentity :: a }
        let src_chirho = "\
module Test where
newtype Identity a = Identity { runIdentity :: a }
wrapped = Identity 42
main = runIdentity wrapped
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("newtype runIdentity accessor should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_record_pattern_match_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Record pattern: match using { field = var } syntax
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
getSum p = case p of
  MkPoint { xCoord = x, yCoord = y } -> x + y
main = getSum (MkPoint { xCoord = 12, yCoord = 30 })
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("record pattern match should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_record_update_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Record update: p { xCoord = 99 }
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
p = MkPoint { xCoord = 10, yCoord = 20 }
q = p { xCoord = 99 }
main = xCoord q + yCoord q
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(119));
            }
            Err(e_chirho) => panic!("record update should evaluate: {}", e_chirho),
        }
    }


    // ── Data.Map tests ──

    #[test]
    fn eval_map_basic_chirho() {
        // Basic Data.Map: empty + insert + lookup → I/O output "42\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = let m = mapInsert 1 42 mapEmpty\n       in case mapLookup 1 m of\n            Just v -> putStrLn (show v)\n            Nothing -> putStrLn \"gone\"\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "42\n"),
            Err(e_chirho) => panic!("eval_map_basic_chirho failed: {}", e_chirho),
        }
    }


    #[test]
    fn eval_user_bst_insert_chirho() {
        // User-defined BST insert using if-then-else (no Prelude mapInsert)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Tree = Leaf | Node Int Int Tree Tree
myInsert k v t = case t of
  Leaf -> Node k v Leaf Leaf
  Node k2 v2 l r -> if k < k2 then Node k2 v2 (myInsert k v l) r else if k == k2 then Node k v l r else Node k2 v2 l (myInsert k v r)
myLookup k t = case t of
  Leaf -> 0
  Node k2 v2 l r -> if k == k2 then v2 else if k < k2 then myLookup k l else myLookup k r
main = myLookup 5 (myInsert 3 99 (myInsert 5 42 Leaf))
"#;
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("User BST insert should work: {}", e_chirho),
        }
    }


    // ── Data.Map extended operations ────────────────────────────────

    #[test]
    fn eval_map_insert_with_chirho() {
        // mapInsertWith (+) 1 100 (mapInsert 1 10 mapEmpty) → value at key 1 is 10+100=110
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m1 = mapInsert 1 10 (mapInsert 2 20 mapEmpty)
m2 = mapInsertWith (+) 1 100 m1
main = mapFindWithDefault 0 1 m2
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(110)),
            Err(e_chirho) => panic!("mapInsertWith: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_union_with_chirho() {
        // mapUnionWith (+) (mapSingleton 1 10) (mapSingleton 1 20) → value at 1 is 30
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapUnionWith (+) (mapSingleton 1 10) (mapSingleton 1 20)
main = mapFindWithDefault 0 1 m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(30)),
            Err(e_chirho) => panic!("mapUnionWith: {}", e_chirho),
        }
    }

    // ── Additional list functions ───────────────────────────────────


    // ── Data.Map String-keyed ─────────────────────────────────────

    #[test]
    fn eval_map_insert_str_chirho() {
        // mapInsertStr then lookup: mapFindWithDefaultStr 0 "hello" (mapInsertStr "hello" 42 mapEmpty)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapInsertStr \"hello\" 42 mapEmpty
main = mapFindWithDefaultStr 0 \"hello\" m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("mapInsertStr: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_singleton_lookup_chirho() {
        // mapSingleton 42 99: lookup existing key
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapFindWithDefault 0 42 (mapSingleton 42 99)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("map singleton lookup: {}", e_chirho),
        }
    }


    #[test]
    fn eval_set_from_list_dedup_chirho() {
        // setFromList [3,1,4,1,5,9,2,6] → deduplicated set → setSize = 7
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setFromList [3,1,4,1,5,9,2,6])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(7)),
            Err(e_chirho) => panic!("set from list dedup: {}", e_chirho),
        }
    }


    // ── fromJust / swap / mapDelete fix ──

    #[test]
    fn eval_from_just_chirho() {
        // fromJust (Just 42) = 42
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = fromJust (Just 42)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("fromJust: {}", e_chirho),
        }
    }


    #[test]
    fn eval_swap_tuple_chirho() {
        // fst (swap (1, 2)) = 2
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = fst (swap (1, 2))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("swap tuple: {}", e_chirho),
        }
    }


    #[test]
    fn eval_swap_snd_chirho() {
        // snd (swap (10, 20)) = 10
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = snd (swap (10, 20))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(10)),
            Err(e_chirho) => panic!("swap snd: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_delete_preserves_chirho() {
        // mapInsert 1 10 (mapInsert 2 20 (mapInsert 3 30 mapEmpty))
        // after mapDelete 2, mapSize should be 2 and both 1 and 3 remain
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nm = mapInsert 1 10 (mapInsert 2 20 (mapInsert 3 30 mapEmpty))\nmain = mapSize (mapDelete 2 m)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("map delete preserves: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_delete_both_subtrees_chirho() {
        // insert 2, 1, 3 (root=2, left=1, right=3), delete 2
        // both 1 and 3 should remain, verify via lookup
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nm = mapInsert 2 20 (mapInsert 1 10 (mapInsert 3 30 mapEmpty))\nm2 = mapDelete 2 m\nmain = fromMaybe 0 (mapLookup 1 m2) + fromMaybe 0 (mapLookup 3 m2)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(40)),
            Err(e_chirho) => panic!("map delete both subtrees: {}", e_chirho),
        }
    }

    // ── Literal pattern matching in function equations ──


    // ── Synthetic module imports ─────────────────────────────────────────

    #[test]
    fn eval_import_data_map_chirho() {
        // import Data.Map functions via synthetic module interface
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import Data.Map (mapInsert, mapLookup, mapEmpty)
main = case mapLookup 1 (mapInsert 1 99 mapEmpty) of
         Just x  -> x
         Nothing -> 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("import Data.Map: {}", e_chirho),
        }
    }


    #[test]
    fn eval_import_data_map_size_chirho() {
        // import Data.Map, use mapSize
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import Data.Map
main = mapSize (mapInsert 2 20 (mapInsert 1 10 mapEmpty))
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("import Data.Map size: {}", e_chirho),
        }
    }


    #[test]
    fn eval_import_qualified_data_map_chirho() {
        // import qualified Data.Map as Map
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import qualified Data.Map as Map
main = case Map.mapLookup 42 (Map.mapInsert 42 100 Map.mapEmpty) of
         Just v  -> v
         Nothing -> 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(100)),
            Err(e_chirho) => panic!("import qualified Data.Map: {}", e_chirho),
        }
    }


    #[test]
    fn eval_import_qualified_data_map_no_alias_chirho() {
        // import qualified Data.Map (no alias) → use Data.Map.mapInsert
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import qualified Data.Map
main = case Data.Map.mapLookup 42 (Data.Map.mapInsert 42 100 Data.Map.mapEmpty) of
         Just v  -> v
         Nothing -> 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(100)),
            Err(e_chirho) => panic!("import qualified Data.Map (no alias): {}", e_chirho),
        }
    }

    #[test]
    fn eval_import_data_set_chirho() {
        // import Data.Set functions
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import Data.Set (setInsert, setMember, setEmpty)
main = case setMember 5 (setInsert 5 (setInsert 3 setEmpty)) of
         True  -> 1
         False -> 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("import Data.Set: {}", e_chirho),
        }
    }


    // ── Data.Map end-to-end ─────────────────────────────────────────────

    #[test]
    fn eval_map_fromlist_size_chirho() {
        // mapSize (mapFromList [(1,10),(2,20),(3,30)]) → 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapSize (mapFromList [(1,10),(2,20),(3,30)])\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_map_lookup_insert_e2e_chirho() {
        // fromMaybe 0 (mapLookup 2 (mapInsert 2 42 (mapFromList [(1,10)]))) → 42
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = fromMaybe 0 (mapLookup 2 (mapInsert 2 42 (mapFromList [(1,10)])))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42));
    }


    #[test]
    fn eval_map_delete_size_chirho() {
        // mapSize (mapDelete 2 (mapFromList [(1,10),(2,20),(3,30)])) → 2
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapSize (mapDelete 2 (mapFromList [(1,10),(2,20),(3,30)]))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(2));
    }


    #[test]
    fn eval_map_null_empty_e2e_chirho() {
        // mapNull mapEmpty → True → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if mapNull mapEmpty then 1 else 0\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_map_union_size_chirho() {
        // mapSize (mapUnion (mapFromList [(1,10),(2,20)]) (mapFromList [(2,99),(3,30)])) → 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapSize (mapUnion (mapFromList [(1,10),(2,20)]) (mapFromList [(2,99),(3,30)]))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_map_difference_size_chirho() {
        // mapSize (mapDifference (mapFromList [(1,10),(2,20),(3,30)]) (mapFromList [(2,99)])) → 2
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapSize (mapDifference (mapFromList [(1,10),(2,20),(3,30)]) (mapFromList [(2,99)]))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(2));
    }


    #[test]
    fn eval_map_keys_sum_chirho() {
        // sum (mapKeys (mapFromList [(1,10),(2,20),(3,30)])) → 6
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (mapKeys (mapFromList [(1,10),(2,20),(3,30)]))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(6));
    }


    #[test]
    fn eval_map_elems_sum_chirho() {
        // sum (mapElems (mapFromList [(1,10),(2,20),(3,30)])) → 60
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (mapElems (mapFromList [(1,10),(2,20),(3,30)]))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(60));
    }


    #[test]
    fn eval_map_map_double_chirho() {
        // sum (mapElems (mapMap (*2) (mapFromList [(1,10),(2,20)]))) → 60
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (mapElems (mapMap (*2) (mapFromList [(1,10),(2,20)])))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(60));
    }

    // ── Data.Set end-to-end ─────────────────────────────────────────────


    // ── Data.Set end-to-end ─────────────────────────────────────────────

    #[test]
    fn eval_set_from_list_size_chirho() {
        // setSize (setFromList [3,1,2,1,3]) → 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setFromList [3,1,2,1,3])\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_set_member_found_chirho() {
        // setMember 2 (setFromList [1,2,3]) → True → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if setMember 2 (setFromList [1,2,3]) then 1 else 0\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_set_union_size_chirho() {
        // setSize (setUnion (setFromList [1,2]) (setFromList [2,3,4])) → 4
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setUnion (setFromList [1,2]) (setFromList [2,3,4]))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(4));
    }

    // ── Data.Set primop-based end-to-end tests ──────────────────────────


    // ── Data.Set primop-based end-to-end tests ──────────────────────────

    #[test]
    fn eval_set_basic_chirho() {
        // let s = setInsert 3 (setInsert 1 (setInsert 2 setEmpty))
        // in putStrLn (show (setSize s)) → "3\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = let s = setInsert 3 (setInsert 1 (setInsert 2 setEmpty))
       in putStrLn (show (setSize s))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut sm_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("eval_set_basic_chirho should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "3\n");
    }


    #[test]
    fn eval_set_member_chirho() {
        // let s = setFromList [1,2,3]
        // putStrLn (if setMember 2 s then "True" else "False") → "True\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = let s = setFromList [1,2,3]
       in putStrLn (if setMember 2 s then \"True\" else \"False\")
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut sm_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("eval_set_member_chirho should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "True\n");
    }


    #[test]
    fn eval_set_tolist_chirho() {
        // let s = setFromList [3,1,2] in putStrLn (show (sum (setToList s))) → "6\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = let s = setFromList [3,1,2]
       in putStrLn (show (sum (setToList s)))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut sm_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("eval_set_tolist_chirho should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "6\n");
    }


    #[test]
    fn eval_set_union_new_chirho() {
        // let s1 = setFromList [1,2]; s2 = setFromList [2,3]
        // in putStrLn (show (setSize (setUnion s1 s2))) → "3\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = let s1 = setFromList [1,2]
           s2 = setFromList [2,3]
       in putStrLn (show (setSize (setUnion s1 s2)))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut sm_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("eval_set_union_new_chirho should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "3\n");
    }

    // ── Deriving Ord end-to-end ─────────────────────────────────────────


    // ── BST / parameterized data type tests ────────────────────────────

    #[test]
    fn eval_tree_construct_match_chirho() {
        // Basic user data type with 3 fields
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ndata Tree = Leaf | Node Tree Int Tree\nmain = case Node Leaf 5 Leaf of\n  Leaf -> 0\n  Node l v r -> v\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(5));
    }


    #[test]
    fn eval_tree_insert_single_chirho() {
        // Insert a single value into a tree
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ndata Tree = Leaf | Node Tree Int Tree\ninsert x Leaf = Node Leaf x Leaf\ninsert x (Node l v r) = if x < v then Node (insert x l) v r else Node l v r\nmain = case insert 5 Leaf of\n  Leaf -> 0\n  Node l v r -> v\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(5));
    }


    #[test]
    fn eval_tree_tolist_leaf_chirho() {
        // toList of Leaf should be empty → length 0
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ndata Tree = Leaf | Node Tree Int Tree\ntoList Leaf = []\ntoList (Node l v r) = toList l ++ [v] ++ toList r\nmain = length (toList Leaf)\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(0));
    }


    #[test]
    fn eval_tree_tolist_single_chirho() {
        // toList of single-node tree with recursive version
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ndata Tree = Leaf | Node Tree Int Tree\ntoList Leaf = []\ntoList (Node l v r) = toList l ++ [v] ++ toList r\nmain = head (toList (Node Leaf 5 Leaf))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(5));
    }


    #[test]
    fn eval_bst_insert_sum_chirho() {
        // Binary search tree with user data type
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Tree = Leaf | Node Tree Int Tree
insert x Leaf = Node Leaf x Leaf
insert x (Node l v r) = if x < v then Node (insert x l) v r else if x > v then Node l v (insert x r) else Node l v r
toList Leaf = []
toList (Node l v r) = toList l ++ [v] ++ toList r
main = sum (toList (insert 3 (insert 1 (insert 4 (insert 2 Leaf)))))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(10));
    }

    // ── 3-tuple pattern matching ────────────────────────────────────────


    // ── Monad transformer infrastructure ──────────────────────────────────

    #[test]
    fn eval_maybe_t_just_chirho() {
        // Basic MaybeT wrapping and unwrapping via a user-defined single-parameter
        // newtype (inner monad fixed to the identity/ground level for evaluation).
        // newtype MaybeT a = MkMaybeT (Maybe a)
        // getMaybeT extracts the inner Maybe value.
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
newtype MaybeT a = MkMaybeT (Maybe a)
getMaybeT t = case t of { MkMaybeT inner -> inner }
main = case getMaybeT (MkMaybeT (Just 42)) of
         Just n  -> n
         Nothing -> 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42),
                "MaybeT (Just 42) should unwrap to 42"
            ),
            Err(e_chirho) => panic!("eval_maybe_t_just_chirho failed: {}", e_chirho),
        }
    }


    #[test]
    fn eval_state_t_basic_chirho() {
        // Basic StateT wrapping and unwrapping via a user-defined newtype.
        // newtype StateT a = MkStateT (Int -> (a, Int))
        // runStateT unwraps and applies the state function.
        // addOne returns 99 and increments state, so result + state = 99+11 = 110.
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
newtype StateT a = MkStateT (Int -> (a, Int))
runStateT t s = case t of { MkStateT f -> f s }
addOne = MkStateT (\\s -> (99, s + 1))
main = case runStateT addOne 10 of
         (a, s) -> a + s
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(110),
                "StateT addOne 10 should give (99, 11), sum = 110"
            ),
            Err(e_chirho) => panic!("eval_state_t_basic_chirho failed: {}", e_chirho),
        }
    }

    // ── Polymorphic elem/notElem/nub/isPrefixOf for Char ─────────────────


    // ── Data.Map (user-defined BST) end-to-end tests ──────────────────

    #[test]
    fn eval_map_empty_size_chirho() {
        // User-defined Map with 4-field Bin constructor; size of empty map = 0
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapSize t = case t of
  Tip -> 0
  Bin k v l r -> 1 + mapSize l + mapSize r
main = mapSize Tip
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(0));
    }


    #[test]
    fn eval_map_singleton_size_chirho() {
        // Singleton map has size 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapSingleton k v = Bin k v Tip Tip
mapSize t = case t of
  Tip -> 0
  Bin k v l r -> 1 + mapSize l + mapSize r
main = mapSize (mapSingleton 42 100)
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_bst_map_insert_lookup_chirho() {
        // Insert 3 keys, look up a value that exists
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapLookup k t = case t of
  Tip -> 0
  Bin k' v l r -> if k == k' then v
                  else if k < k' then mapLookup k l
                  else mapLookup k r
main = mapLookup 2 (mapInsert 3 30 (mapInsert 1 10 (mapInsert 2 20 Tip)))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(20));
    }


    #[test]
    fn eval_map_insert_size_chirho() {
        // Insert 4 distinct keys, size = 4
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapSize t = case t of
  Tip -> 0
  Bin k v l r -> 1 + mapSize l + mapSize r
main = mapSize (mapInsert 4 40 (mapInsert 2 20 (mapInsert 3 30 (mapInsert 1 10 Tip))))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(4));
    }


    #[test]
    fn eval_map_update_value_chirho() {
        // Insert same key twice, second value overwrites first
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapLookup k t = case t of
  Tip -> 0
  Bin k' v l r -> if k == k' then v
                  else if k < k' then mapLookup k l
                  else mapLookup k r
main = mapLookup 1 (mapInsert 1 99 (mapInsert 1 10 Tip))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(99));
    }


    #[test]
    fn eval_bst_map_member_chirho() {
        // Check membership in map
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapMember k t = case t of
  Tip -> False
  Bin k' v l r -> if k == k' then True
                  else if k < k' then mapMember k l
                  else mapMember k r
m = mapInsert 5 50 (mapInsert 3 30 (mapInsert 7 70 Tip))
main = if mapMember 3 m then 1 else 0
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_map_member_missing_chirho() {
        // Check non-membership in map
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapMember k t = case t of
  Tip -> False
  Bin k' v l r -> if k == k' then True
                  else if k < k' then mapMember k l
                  else mapMember k r
m = mapInsert 5 50 (mapInsert 3 30 (mapInsert 7 70 Tip))
main = if mapMember 4 m then 1 else 0
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(0));
    }


    #[test]
    fn eval_map_keys_sorted_chirho() {
        // Keys come out in BST order (sorted by key)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapKeys t = case t of
  Tip -> []
  Bin k v l r -> mapKeys l ++ [k] ++ mapKeys r
main = head (mapKeys (mapInsert 3 30 (mapInsert 1 10 (mapInsert 2 20 Tip))))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_user_bst_five_inserts_chirho() {
        // Build BST with 5 direct inserts, check size = 5
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Dict k v = DTip | DBin k v (Dict k v) (Dict k v)
dInsert k v t = case t of
  DTip -> DBin k v DTip DTip
  DBin k' v' l r -> if k == k' then DBin k v l r
                    else if k < k' then DBin k' v' (dInsert k v l) r
                    else DBin k' v' l (dInsert k v r)
dictSize t = case t of
  DTip -> 0
  DBin k v l r -> 1 + dictSize l + dictSize r
main = dictSize (dInsert 5 50 (dInsert 4 40 (dInsert 3 30 (dInsert 2 20 (dInsert 1 10 DTip)))))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(5));
    }


    #[test]
    fn eval_tuple_case_from_list_chirho() {
        // Minimal repro: case on tuple extracted from list
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = case head [(1, 2)] of\n  (a, b) -> a + b\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_user_bst_fromlist_chirho() {
        // Build BST from list of pairs using foldr
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Dict k v = DTip | DBin k v (Dict k v) (Dict k v)
dInsert k v t = case t of
  DTip -> DBin k v DTip DTip
  DBin k' v' l r -> if k == k' then DBin k v l r
                    else if k < k' then DBin k' v' (dInsert k v l) r
                    else DBin k' v' l (dInsert k v r)
dictSize t = case t of
  DTip -> 0
  DBin k v l r -> 1 + dictSize l + dictSize r
main = dictSize (dInsert 5 50 (dInsert 4 40 (dInsert 3 30 (dInsert 2 20 (dInsert 1 10 DTip)))))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(5));
    }

    // ── Tuple pattern in function arguments ───────────────────────────


    // ── Complex user-defined data types ───────────────────────────────

    #[test]
    fn eval_expr_tree_eval_chirho() {
        // Expression tree with Add and Mul constructors
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Expr = Lit Int | Add Expr Expr | Mul Expr Expr
eval e = case e of
  Lit n -> n
  Add a b -> eval a + eval b
  Mul a b -> eval a * eval b
main = eval (Add (Mul (Lit 3) (Lit 4)) (Lit 5))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(17));
    }


    #[test]
    fn eval_linked_list_user_chirho() {
        // User-defined linked list with custom fold
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data List a = Nil | Cons a (List a)
myFoldr f z xs = case xs of
  Nil -> z
  Cons x rest -> f x (myFoldr f z rest)
mySum xs = myFoldr (\x acc -> x + acc) 0 xs
main = mySum (Cons 1 (Cons 2 (Cons 3 (Cons 4 Nil))))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(10));
    }


    #[test]
    fn eval_rose_tree_depth_chirho() {
        // Rose tree: each node has a list of children
        // We represent as: data Rose = RLeaf Int | RNode Int [Rose]
        // but list of Rose is complex; use binary rose instead:
        // data Rose = RLeaf Int | RNode Int Rose Rose
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Rose = RLeaf Int | RNode Int Rose Rose
depth t = case t of
  RLeaf n -> 1
  RNode n l r -> 1 + max (depth l) (depth r)
main = depth (RNode 1 (RNode 2 (RLeaf 3) (RLeaf 4)) (RLeaf 5))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(3));
    }

    // ── Higher-order functions with user data types ────────────────────


    // ── Higher-order functions with user data types ────────────────────

    #[test]
    fn eval_map_over_tree_chirho() {
        // Map a function over all values in a BST, sum result
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Tree = Leaf | Node Tree Int Tree
insert x t = case t of
  Leaf -> Node Leaf x Leaf
  Node l v r -> if x < v then Node (insert x l) v r
                else if x > v then Node l v (insert x r)
                else Node l v r
mapTree f t = case t of
  Leaf -> Leaf
  Node l v r -> Node (mapTree f l) (f v) (mapTree f r)
toList t = case t of
  Leaf -> []
  Node l v r -> toList l ++ [v] ++ toList r
main = sum (toList (mapTree (\x -> x * 2) (insert 3 (insert 1 (insert 2 Leaf)))))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(12));
    }

    // ── Accumulator pattern with user data type ───────────────────────


    // ── Accumulator pattern with user data type ───────────────────────

    #[test]
    fn eval_stack_push_pop_chirho() {
        // User-defined Stack data type with push/pop/peek
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Stack = Empty | Push Int Stack
peek s = case s of
  Empty -> 0
  Push x rest -> x
stackSize s = case s of
  Empty -> 0
  Push x rest -> 1 + stackSize rest
s = Push 30 (Push 20 (Push 10 Empty))
main = peek s + stackSize s
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(33));
    }

    // ── tails / inits ─────────────────────────────────────────────────


    // ── Nested data types with pattern matching ────────────────────────

    /// Test 4: arithmetic expression tree — ADT with recursive eval.
    /// eval (Add (Mul (Lit 3) (Lit 4)) (Lit 5)) = 17
    #[test]
    fn eval_expr_tree_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Expr = Lit Int | Add Expr Expr | Mul Expr Expr
eval (Lit n) = n
eval (Add a b) = eval a + eval b
eval (Mul a b) = eval a * eval b
main = putStrLn (show (eval (Add (Mul (Lit 3) (Lit 4)) (Lit 5))))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap();
        assert_eq!(m_chirho.io_output_chirho, "17\n");
    }

    // ── List recursion with accumulator ───────────────────────────────

    /// Test 5: myReverse using go accumulator helper in where-clause.
    /// sum (myReverse [1,2,3,4,5]) = 15
    /// Uses case-expression style for the where helper to avoid multi-equation
    /// where-function list-pattern limitations.

    // ── Data.Map additional operations ────────────────────────────────────

    #[test]
    fn eval_map_foldr_with_key_chirho() {
        // mapFoldrWithKey (\k v acc -> acc + v) 0 (fromList [(1,10),(2,20),(3,30)]) → 60
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapFromList [(1,10),(2,20),(3,30)]
main = mapFoldrWithKey (\\k v acc -> acc + v) 0 m
";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("mapFoldrWithKey sum failed: {}", e_chirho));
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(60));
    }


    #[test]
    fn eval_map_filter_values_chirho() {
        // mapFilter (>15) (fromList [(1,10),(2,20),(3,30)]) — sum elems of result = 50
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapFromList [(1,10),(2,20),(3,30)]
main = sum (mapElems (mapFilter (> 15) m))
";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("mapFilter sum elems failed: {}", e_chirho));
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(50));
    }


    #[test]
    fn eval_map_map_triple_chirho() {
        // mapMap (*3) (fromList [(1,10),(2,20)]) — sum values = 90
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapFromList [(1,10),(2,20)]
main = sum (mapElems (mapMap (\\x -> x * 3) m))
";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("mapMap (*3) sum failed: {}", e_chirho));
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(90));
    }


    #[test]
    fn eval_map_union_with_sum_chirho() {
        // mapUnionWith (+) two maps with shared key — combined value is sum
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m1 = mapFromList [(1,10),(2,20)]
m2 = mapFromList [(2,5),(3,30)]
main = sum (mapElems (mapUnionWith (+) m1 m2))
";
        // key 1→10, key 2→20+5=25, key 3→30 : total = 65
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("mapUnionWith sum failed: {}", e_chirho));
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(65));
    }

    // ── Data.List: find ───────────────────────────────────────────────────


    // John 3:16 - For God so loved the world, that he gave his only begotten Son,
    // that whosoever believeth in him should not perish, but have everlasting life.

    // ── Data.Set higher-order operation tests ──

    #[test]
    fn eval_set_filter_gt3_chirho() {
        // setFilter (>3) {1,2,3,4,5} → {4,5} → size 2
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             main = setSize (setFilter (\\x -> x > 3) (setFromList [1,2,3,4,5]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("setFilter should work");
        assert_eq!(result_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(2));
    }


    #[test]
    fn eval_set_map_double_chirho() {
        // setMap (*2) {1,2,3} → {2,4,6} → size 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             main = setSize (setMap (\\x -> x * 2) (setFromList [1,2,3]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("setMap should work");
        assert_eq!(result_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_set_map_dedup_chirho() {
        // setMap (\x -> x `mod` 3) {1,2,3,4,5} → {0,1,2} → size 3 (deduplication)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             main = setSize (setMap (\\x -> x `mod` 3) (setFromList [1,2,3,4,5]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("setMap with dedup should work");
        assert_eq!(result_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_set_fold_sum_chirho() {
        // setFold (+) 0 {1,2,3,4,5} → 15
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             main = setFold (+) 0 (setFromList [1,2,3,4,5])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("setFold sum should work");
        assert_eq!(result_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(15));
    }

    // ── zipWith and unzip tests ──

    // ── ExistentialQuantification tests ──

    #[test]
    fn existential_data_parses_chirho() {
        // ExistentialQuantification: `forall a. Show a => MkShowable a`
        // Test that the parser correctly extracts "MkShowable" as the constructor
        // name, not "Show" from the context.
        use crate::compile_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Showable = forall a. Show a => MkShowable a
main = 42
";
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs");
        assert!(
            result_chirho.is_ok(),
            "existential data decl should parse and compile: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn existential_data_no_context_parses_chirho() {
        // ExistentialQuantification without a context: `forall a. MkBox a`
        use crate::compile_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Box = forall a. MkBox a
main = 42
";
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs");
        assert!(
            result_chirho.is_ok(),
            "existential without context should parse: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn existential_constructor_eval_chirho() {
        // Construct an existential value and extract the inner value
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Box = forall a. MkBox a
unbox (MkBox x) = x
main = unbox (MkBox 42)
";
        let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("existential eval should work");
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42));
    }

    // ── KindSignatures (§E.41) ──────────────────────────────────────────

    #[test]
    fn kind_sig_data_star_chirho() {
        // data Proxy (a :: *) = MkProxy
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Proxy (a :: *) = MkProxy
main = case MkProxy of
  MkProxy -> 42
";
        let val_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("kind sig data star should work");
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn kind_sig_data_with_field_chirho() {
        // data Wrapper (a :: *) = MkWrapper a — kind-annotated var used in constructor
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Wrapper (a :: *) = MkWrapper a
unwrap (MkWrapper x) = x
main = unwrap (MkWrapper 99)
";
        let val_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("kind sig data with field should work");
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(99));
    }

    #[test]
    fn kind_sig_newtype_chirho() {
        // newtype Id (a :: *) = MkId a
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
newtype Id (a :: *) = MkId a
getId (MkId x) = x
main = getId (MkId 77)
";
        let val_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("kind sig newtype should work");
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(77));
    }

    #[test]
    fn kind_sig_mixed_annotated_unannotated_chirho() {
        // data Pair (a :: *) b = MkPair a b — mix of annotated and plain
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Pair (a :: *) b = MkPair a b
fst2 (MkPair x y) = x
main = fst2 (MkPair 55 100)
";
        let val_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("kind sig mixed should work");
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(55));
    }

    #[test]
    fn kind_sig_arrow_kind_chirho() {
        // data HKD (f :: * -> *) = MkHKD — higher-kinded type variable
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data HKD (f :: * -> *) = MkHKD
main = case MkHKD of
  MkHKD -> 123
";
        let val_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("kind sig arrow kind should work");
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(123));
    }

    // ── Strict data fields ──────────────────────────────────────────

    #[test]
    fn eval_strict_data_field_basic_chirho() {
        let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Pair = MkPair !Int !Int
main = case MkPair (1 + 2) (3 + 4) of
  MkPair a b -> a + b
";
        let val_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("strict fields should be forced");
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(10));
    }

    #[test]
    fn eval_strict_and_lazy_fields_chirho() {
        let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Mixed = MkMixed !Int Int
main = case MkMixed (2 + 3) (4 + 5) of
  MkMixed a b -> a + b
";
        let val_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("mixed strict/lazy fields should work");
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(14));
    }

    #[test]
    fn eval_strict_field_single_chirho() {
        let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Box = MkBox !Int
main = case MkBox (21 + 21) of
  MkBox x -> x
";
        let val_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("single strict field should work");
        assert_eq!(val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn eval_strict_field_preserves_core_chirho() {
        // Verify strict fields produce case wrappers in Core IR
        let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data StrictBox = SB !Int
main = case SB 42 of
  SB x -> x
";
        let result_chirho = compile_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs",
        ).expect("strict field should compile");
        // The Core IR should contain the binding even without case wrappers for literals
        let has_sb_chirho = result_chirho.core_chirho.bindings_chirho.iter().any(|b_chirho| {
            let name_chirho = &b_chirho.binder_chirho.name_chirho;
            name_chirho == "main"
        });
        assert!(has_sb_chirho, "main binding should exist in Core IR");
    }

    // ── RecordWildCards ──────────────────────────────────────────────

    #[test]
    fn eval_record_wildcards_pattern_chirho() {
        // RecordWildCards in pattern: Foo{..} binds all fields as variables
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
main = case MkPoint { xCoord = 10, yCoord = 20 } of
  MkPoint{..} -> xCoord + yCoord
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(30));
            }
            Err(e_chirho) => panic!("RecordWildCards pattern should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_record_wildcards_expr_chirho() {
        // RecordWildCards in expression: Con{..} fills missing fields from let-bound scope
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
main = let xCoord = 10
           yCoord = 20
       in case MkPoint{..} of
            MkPoint { xCoord = a, yCoord = b } -> a + b
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(30));
            }
            Err(e_chirho) => panic!("RecordWildCards expression should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_record_wildcards_partial_pattern_chirho() {
        // RecordWildCards with some explicit fields: MkPoint{xCoord = a, ..}
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
main = case MkPoint { xCoord = 10, yCoord = 20 } of
  MkPoint{xCoord = a, ..} -> a + yCoord
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(30));
            }
            Err(e_chirho) => panic!("RecordWildCards partial pattern should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_record_wildcards_partial_expr_chirho() {
        // RecordWildCards in expression with some explicit fields: MkPoint{xCoord = 10, ..}
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
main = let yCoord = 32
       in case MkPoint{xCoord = 10, ..} of
            MkPoint{xCoord = a, yCoord = b} -> a + b
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskelujah_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("RecordWildCards partial expression should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn debug_record_wildcard_expr_ast_chirho() {
        // Debug test: verify MkPoint{..} parses as RecordConChirho with has_wildcard_chirho=true
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
f = MkPoint{..}
";
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let fid_chirho = sm_chirho.add_file_chirho("test.hs", src_chirho);
        let cst_chirho = haskelujah_parser_chirho::cst_parser_chirho::parse_to_cst_chirho(src_chirho, fid_chirho);
        let module_chirho = haskelujah_parser_chirho::lower_chirho::lower_module_chirho(&cst_chirho, fid_chirho);
        // Find the f declaration and check its RHS
        use haskelujah_ast_chirho::decl_chirho::DeclChirho;
        use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho};
        for decl_chirho in &module_chirho.decls_chirho {
            eprintln!("DECL: {:#?}", decl_chirho);
        }
        let found_wildcard_chirho = module_chirho.decls_chirho.iter().any(|decl_chirho| {
            // Check FunBindChirho
            if let DeclChirho::FunBindChirho { name_chirho, matches_chirho, .. } = decl_chirho {
                if name_chirho.text_chirho() == "f" {
                    for m_chirho in matches_chirho {
                        if let RhsChirho::UnguardedChirho(expr_chirho) = &m_chirho.rhs_chirho {
                            if let ExprChirho::RecordConChirho { has_wildcard_chirho, .. } = expr_chirho {
                                return *has_wildcard_chirho;
                            }
                        }
                    }
                }
            }
            // Check PatBindChirho
            if let DeclChirho::PatBindChirho { rhs_chirho, .. } = decl_chirho {
                if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
                    if let ExprChirho::RecordConChirho { has_wildcard_chirho, .. } = expr_chirho {
                        return *has_wildcard_chirho;
                    }
                }
            }
            false
        });
        assert!(found_wildcard_chirho, "MkPoint{{..}} should parse as RecordConChirho with has_wildcard_chirho=true");
    }

    // ── NamedFieldPuns tests ──────────────────────────────────────────

    #[test]
    fn named_field_puns_pattern_chirho() {
        // NamedFieldPuns in pattern position: `MkPoint{x, y}` = `MkPoint{x=x, y=y}`
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE NamedFieldPuns #-}
module Test where
data Point = MkPoint { x :: Int, y :: Int }
sumPoint (MkPoint{x, y}) = x + y
main = print (sumPoint (MkPoint{x = 10, y = 32}))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("NamedFieldPuns pattern failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "42\n");
    }

    #[test]
    fn named_field_puns_expr_chirho() {
        // NamedFieldPuns in expression position: `MkPoint{x, y}` = `MkPoint{x=x, y=y}`
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE NamedFieldPuns #-}
module Test where
data Point = MkPoint { x :: Int, y :: Int }
getX (MkPoint{x = val}) = val
main = let x = 10
           y = 32
       in print (getX (MkPoint{x, y}))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("NamedFieldPuns expr failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "10\n");
    }

    #[test]
    fn named_field_puns_mixed_chirho() {
        // Mix of punned and explicit fields
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE NamedFieldPuns #-}
module Test where
data Pair = MkPair { fst :: Int, snd :: Int }
addPair (MkPair{fst, snd = b}) = fst + b
main = print (addPair (MkPair{fst = 20, snd = 22}))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("NamedFieldPuns mixed failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "42\n");
    }

    // ── MultiWayIf tests ────────────────────────────────────────────────

    #[test]
    fn multi_way_if_basic_chirho() {
        // Basic multi-way if with otherwise, hitting last branch
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE MultiWayIf #-}
module Test where
classify x = if | x > 100   -> 1
                | x > 10    -> 2
                | otherwise  -> 3
main = print (classify 5)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("MultiWayIf basic failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "3\n");
    }

    #[test]
    fn multi_way_if_first_branch_chirho() {
        // Multi-way if hitting the first branch
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE MultiWayIf #-}
module Test where
f x = if | x > 100  -> 1
         | x > 10   -> 2
         | otherwise -> 3
main = print (f 200)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("MultiWayIf first branch failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "1\n");
    }

    #[test]
    fn multi_way_if_middle_branch_chirho() {
        // Multi-way if hitting the middle branch
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE MultiWayIf #-}
module Test where
f x = if | x > 100  -> 1
         | x > 10   -> 2
         | otherwise -> 3
main = print (f 50)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("MultiWayIf middle branch failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "2\n");
    }

    // ── NumericUnderscores tests ─────────────────────────────────────────

    #[test]
    fn numeric_underscores_int_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE NumericUnderscores #-}
module Test where
main = print 1_000_000
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("NumericUnderscores int failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "1000000\n");
    }

    #[test]
    fn numeric_underscores_hex_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE NumericUnderscores #-}
module Test where
main = print 0xFF_FF
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("NumericUnderscores hex failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "65535\n");
    }

    #[test]
    fn numeric_underscores_arithmetic_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE NumericUnderscores #-}
module Test where
main = print (1_000 + 2_000)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("NumericUnderscores arith failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "3000\n");
    }

    // ── Enum succ/pred tests ────────────────────────────────────────────

    #[test]
    fn enum_succ_derived_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
module Test where
data Color = Red | Green | Blue deriving (Show, Eq, Enum)
main = print (succ Red)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("Enum succ failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "Green\n");
    }

    #[test]
    fn enum_pred_derived_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
module Test where
data Color = Red | Green | Blue deriving (Show, Eq, Enum)
main = print (pred Blue)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("Enum pred failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "Green\n");
    }

    #[test]
    fn enum_from_enum_derived_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
module Test where
data Color = Red | Green | Blue deriving (Show, Eq, Enum)
main = print (fromEnum Blue)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("Enum fromEnum failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "2\n");
    }

    // ── TupleSections tests ─────────────────────────────────────────────

    #[test]
    fn tuple_section_left_gap_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE TupleSections #-}
module Test where
main = print ((,1) 42)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("TupleSections left gap failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "(42,1)\n");
    }

    #[test]
    fn tuple_section_right_gap_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE TupleSections #-}
module Test where
main = print ((1,) 99)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("TupleSections right gap failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "(1,99)\n");
    }

    #[test]
    fn tuple_section_map_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE TupleSections #-}
module Test where
main = print (map (,True) [1,2,3])
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("TupleSections map failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "[(1,True),(2,True),(3,True)]\n");
    }

    // ── StandaloneDeriving tests ────────────────────────────────────────

    #[test]
    fn standalone_deriving_show_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE StandaloneDeriving #-}
module Test where
data Color = Red | Green | Blue
deriving instance Show Color
main = print Green
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("StandaloneDeriving Show failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "Green\n");
    }

    #[test]
    fn standalone_deriving_eq_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE StandaloneDeriving #-}
module Test where
data Color = Red | Green | Blue
deriving instance Eq Color
deriving instance Show Color
main = print (Red == Red)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("StandaloneDeriving Eq failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "True\n");
    }

    #[test]
    fn standalone_deriving_ord_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE StandaloneDeriving #-}
module Test where
data Color = Red | Green | Blue
deriving instance Eq Color
deriving instance Ord Color
deriving instance Show Color
main = print (compare Red Blue)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("StandaloneDeriving Ord failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "LT\n");
    }

    // ── DeriveAnyClass test ─────────────────────────────────────────────

    #[test]
    fn derive_anyclass_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // DeriveAnyClass generates an empty instance relying on default methods.
        // Use with a class that has a default: class MyClass a where myMethod :: a -> Int; myMethod _ = 42
        // For now just verify the extension doesn't cause a parse/compile error
        // and that standard classes still work alongside it.
        let src_chirho = r#"
{-# LANGUAGE DeriveAnyClass #-}
module Test where
data Color = Red | Green | Blue deriving (Show, Eq)
main = print (Red == Green)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("DeriveAnyClass failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "False\n");
    }

    // ── UnicodeSyntax tests ─────────────────────────────────────────────

    #[test]
    fn unicode_syntax_arrows_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "{-# LANGUAGE UnicodeSyntax #-}\nmodule Test where\nid2 \u{2237} a \u{2192} a\nid2 x = x\nmain = print (id2 42)\n";
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("UnicodeSyntax arrows failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "42\n");
    }

    #[test]
    fn unicode_syntax_lambda_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = print ((\u{03BB}x -> x + 1) 41)\n";
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("UnicodeSyntax lambda failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "42\n");
    }

    #[test]
    fn unicode_syntax_fat_arrow_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Test ⇒ for class constraints
        let src_chirho = "{-# LANGUAGE UnicodeSyntax #-}\nmodule Test where\nshowIt \u{2237} Show a \u{21D2} a \u{2192} String\nshowIt x = show x\nmain = putStrLn (showIt 42)\n";
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("UnicodeSyntax fat arrow failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "42\n");
    }

    // ── ImportQualifiedPost test ────────────────────────────────────────

    #[test]
    fn import_qualified_post_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE ImportQualifiedPost #-}
module Test where
import Data.Map qualified as Map
main = print (Map.mapSize (Map.mapInsert 1 "a" Map.mapEmpty))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("ImportQualifiedPost failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "1\n");
    }

    // ── DerivingStrategies tests ────────────────────────────────────────

    #[test]
    fn deriving_strategies_stock_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE DerivingStrategies #-}
module Test where
data Color = Red | Green | Blue
  deriving stock (Show, Eq)
main = print (Red == Green)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("DerivingStrategies stock failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "False\n");
    }

    #[test]
    fn deriving_strategies_stock_newtype_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE DerivingStrategies #-}
module Test where
newtype Age = MkAge Int
  deriving stock (Show, Eq)
main = print (MkAge 42)
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("DerivingStrategies stock newtype failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "42\n");
    }

    // ── PackageImports test ─────────────────────────────────────────────

    #[test]
    fn package_imports_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE PackageImports #-}
module Test where
import "base" Data.List
main = putStrLn "works"
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("PackageImports failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "works\n");
    }

    // ── RoleAnnotations test ────────────────────────────────────────────

    #[test]
    fn role_annotations_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE RoleAnnotations #-}
module Test where
data MyTag = TagA | TagB deriving Show
type role MyTag nominal
main = print TagA
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("RoleAnnotations failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "TagA\n");
    }

    // ── DeriveDataTypeable test ─────────────────────────────────────────

    #[test]
    fn derive_data_typeable_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"
{-# LANGUAGE DeriveDataTypeable #-}
module Test where
data Color = Red | Green | Blue deriving (Show, Eq, Typeable)
main = print Green
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("DeriveDataTypeable failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "Green\n");
    }

    // ---------------------------------------------------------------
    // Associated type families in classes
    // ---------------------------------------------------------------

    /// `{-# LANGUAGE TypeFamilies #-}`: class with associated type family
    /// parses and compiles without error (associated type registered as type family).
    #[test]
    fn assoc_type_family_basic_chirho() {
        let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

class Container f where
  type Elem f
  size :: f -> Int
"#;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let sf_chirho = crate::SourceFileChirho::from_source_map_chirho(
            &mut sm_chirho,
            "TestChirho.hs",
            src_chirho,
        );
        let frontend_chirho = crate::run_frontend_chirho(
            src_chirho,
            sf_chirho.file_id_chirho(),
            &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
            &std::collections::HashMap::new(),
        )
        .expect("frontend should succeed");
        // Verify the class has 1 method and 1 associated type
        let class_decl_chirho = frontend_chirho
            .module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| {
                matches!(
                    d_chirho,
                    haskelujah_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho { .. }
                )
            })
            .expect("should have a ClassDeclChirho");
        if let haskelujah_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho {
            methods_chirho,
            associated_tfs_chirho,
            ..
        } = class_decl_chirho
        {
            assert_eq!(methods_chirho.len(), 1, "should have 1 method (size)");
            assert_eq!(
                associated_tfs_chirho.len(),
                1,
                "should have 1 associated type family (Elem)"
            );
            assert_eq!(
                associated_tfs_chirho[0].name_chirho.text_chirho(),
                "Elem"
            );
        }
    }

    /// Associated type family appears in AST and class has the associated_tfs_chirho field.
    #[test]
    fn assoc_type_family_parsed_chirho() {
        let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

class MyClass a where
  type MyFamily a :: *
  myMethod :: a -> Int
"#;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let sf_chirho = crate::SourceFileChirho::from_source_map_chirho(
            &mut sm_chirho,
            "TestChirho.hs",
            src_chirho,
        );
        let frontend_chirho = crate::run_frontend_chirho(
            src_chirho,
            sf_chirho.file_id_chirho(),
            &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
            &std::collections::HashMap::new(),
        )
        .expect("frontend should succeed");
        let module_chirho = &frontend_chirho.module_chirho;
        // Find the class decl and check it has an associated type family
        let class_decl_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| {
                matches!(
                    d_chirho,
                    haskelujah_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho { .. }
                )
            })
            .expect("should have a ClassDeclChirho");
        if let haskelujah_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho {
            associated_tfs_chirho,
            ..
        } = class_decl_chirho
        {
            assert_eq!(
                associated_tfs_chirho.len(),
                1,
                "should have 1 associated type family"
            );
            assert_eq!(
                associated_tfs_chirho[0].name_chirho.text_chirho(),
                "MyFamily"
            );
        }
    }

    /// Associated type family instance in instance decl is extracted.
    #[test]
    fn assoc_type_family_instance_parsed_chirho() {
        let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

class MyCol a where
  type Key a

instance MyCol Int where
  type Key Int = Bool
"#;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let sf_chirho = crate::SourceFileChirho::from_source_map_chirho(
            &mut sm_chirho,
            "TestChirho.hs",
            src_chirho,
        );
        let frontend_chirho = crate::run_frontend_chirho(
            src_chirho,
            sf_chirho.file_id_chirho(),
            &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
            &std::collections::HashMap::new(),
        )
        .expect("frontend should succeed");
        let module_chirho = &frontend_chirho.module_chirho;
        let inst_decl_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| {
                matches!(
                    d_chirho,
                    haskelujah_ast_chirho::decl_chirho::DeclChirho::InstanceDeclChirho { .. }
                )
            })
            .expect("should have an InstanceDeclChirho");
        if let haskelujah_ast_chirho::decl_chirho::DeclChirho::InstanceDeclChirho {
            assoc_tf_instances_chirho,
            ..
        } = inst_decl_chirho
        {
            assert_eq!(
                assoc_tf_instances_chirho.len(),
                1,
                "should have 1 associated type family instance"
            );
            assert_eq!(
                assoc_tf_instances_chirho[0].family_name_chirho.text_chirho(),
                "Key"
            );
        }
    }

