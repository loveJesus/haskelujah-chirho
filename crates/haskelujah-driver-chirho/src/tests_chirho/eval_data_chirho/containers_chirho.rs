// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Data.Map and Data.Set evaluation tests (row 700).

use haskelujah_span_chirho::SourceMapChirho;

// ── Data.Map tests ──

#[test]
fn eval_map_basic_chirho() {
    // Basic Data.Map: empty + insert + lookup → I/O output "42\n"
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = let m = Map.insert 1 42 Map.empty\n       in case Map.lookup 1 m of\n            Just v -> putStrLn (show v)\n            Nothing -> putStrLn \"gone\"\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
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
import qualified Data.Map as Map
m1 = Map.insert 1 10 (Map.insert 2 20 Map.empty)
m2 = Map.insertWith (+) 1 100 m1
main = Map.findWithDefault 0 1 m2
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(110)
        ),
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
import qualified Data.Map as Map
m = Map.unionWith (+) (Map.singleton 1 10) (Map.singleton 1 20)
main = Map.findWithDefault 0 1 m
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(30)
        ),
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
import qualified Data.Map as Map
m = Map.insert \"hello\" 42 Map.empty
main = Map.findWithDefault 0 \"hello\" m
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("mapInsertStr: {}", e_chirho),
    }
}

#[test]
fn eval_map_singleton_lookup_chirho() {
    // mapSingleton 42 99: lookup existing key
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = Map.findWithDefault 0 42 (Map.singleton 42 99)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
        ),
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
        ),
        Err(e_chirho) => panic!("set from list dedup: {}", e_chirho),
    }
}

// ── fromJust / swap / mapDelete fix ──

#[test]
fn eval_from_just_chirho() {
    // fromJust (Just 42) = 42
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Maybe (fromJust)\nmain = fromJust (Just 42)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("fromJust: {}", e_chirho),
    }
}

#[test]
fn eval_swap_tuple_chirho() {
    // fst (swap (1, 2)) = 2
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Tuple (swap)\nmain = fst (swap (1, 2))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("swap tuple: {}", e_chirho),
    }
}

#[test]
fn eval_swap_snd_chirho() {
    // snd (swap (10, 20)) = 10
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Tuple (swap)\nmain = snd (swap (10, 20))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
        ),
        Err(e_chirho) => panic!("swap snd: {}", e_chirho),
    }
}

#[test]
fn eval_map_delete_preserves_chirho() {
    // mapInsert 1 10 (mapInsert 2 20 (mapInsert 3 30 mapEmpty))
    // after mapDelete 2, mapSize should be 2 and both 1 and 3 remain
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nm = Map.insert 1 10 (Map.insert 2 20 (Map.insert 3 30 Map.empty))\nmain = Map.size (Map.delete 2 m)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("map delete preserves: {}", e_chirho),
    }
}

#[test]
fn eval_map_delete_both_subtrees_chirho() {
    // insert 2, 1, 3 (root=2, left=1, right=3), delete 2
    // both 1 and 3 should remain, verify via lookup
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Maybe (fromMaybe)\nimport qualified Data.Map as Map\nm = Map.insert 2 20 (Map.insert 1 10 (Map.insert 3 30 Map.empty))\nm2 = Map.delete 2 m\nmain = fromMaybe 0 (Map.lookup 1 m2) + fromMaybe 0 (Map.lookup 3 m2)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(40)
        ),
        Err(e_chirho) => panic!("map delete both subtrees: {}", e_chirho),
    }
}

// ── Data.Map end-to-end ─────────────────────────────────────────────

#[test]
fn eval_map_fromlist_size_chirho() {
    // mapSize (mapFromList [(1,10),(2,20),(3,30)]) → 3
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = Map.size (Map.fromList [(1,10),(2,20),(3,30)])\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );
}

#[test]
fn eval_map_lookup_insert_e2e_chirho() {
    // fromMaybe 0 (mapLookup 2 (mapInsert 2 42 (mapFromList [(1,10)]))) → 42
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Maybe (fromMaybe)\nimport qualified Data.Map as Map\nmain = fromMaybe 0 (Map.lookup 2 (Map.insert 2 42 (Map.fromList [(1,10)])))\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );
}

#[test]
fn eval_map_delete_size_chirho() {
    // mapSize (mapDelete 2 (mapFromList [(1,10),(2,20),(3,30)])) → 2
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = Map.size (Map.delete 2 (Map.fromList [(1,10),(2,20),(3,30)]))\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
    );
}

#[test]
fn eval_map_null_empty_e2e_chirho() {
    // mapNull mapEmpty → True → 1
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = if Map.null Map.empty then 1 else 0\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_map_union_size_chirho() {
    // mapSize (mapUnion (mapFromList [(1,10),(2,20)]) (mapFromList [(2,99),(3,30)])) → 3
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = Map.size (Map.union (Map.fromList [(1,10),(2,20)]) (Map.fromList [(2,99),(3,30)]))\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );
}

#[test]
fn eval_map_difference_size_chirho() {
    // mapSize (mapDifference (mapFromList [(1,10),(2,20),(3,30)]) (mapFromList [(2,99)])) → 2
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = Map.size (Map.difference (Map.fromList [(1,10),(2,20),(3,30)]) (Map.fromList [(2,99)]))\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
    );
}

#[test]
fn eval_map_keys_sum_chirho() {
    // sum (mapKeys (mapFromList [(1,10),(2,20),(3,30)])) → 6
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = sum (Map.keys (Map.fromList [(1,10),(2,20),(3,30)]))\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
    );
}

#[test]
fn eval_map_elems_sum_chirho() {
    // sum (mapElems (mapFromList [(1,10),(2,20),(3,30)])) → 60
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = sum (Map.elems (Map.fromList [(1,10),(2,20),(3,30)]))\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(60)
    );
}

#[test]
fn eval_map_map_double_chirho() {
    // sum (mapElems (mapMap (*2) (mapFromList [(1,10),(2,20)]))) → 60
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = sum (Map.elems (Map.map (*2) (Map.fromList [(1,10),(2,20)])))\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(60)
    );
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
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );
}

#[test]
fn eval_set_member_found_chirho() {
    // setMember 2 (setFromList [1,2,3]) → True → 1
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nmain = if setMember 2 (setFromList [1,2,3]) then 1 else 0\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_set_union_size_chirho() {
    // setSize (setUnion (setFromList [1,2]) (setFromList [2,3,4])) → 4
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nmain = setSize (setUnion (setFromList [1,2]) (setFromList [2,3,4]))\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(4)
    );
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("eval_set_union_new_chirho should evaluate");
    assert_eq!(result_chirho.1.io_output_chirho, "3\n");
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
import qualified Data.Map as Map
m = Map.fromList [(1,10),(2,20),(3,30)]
main = Map.foldrWithKey (\\k v acc -> acc + v) 0 m
";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .unwrap_or_else(|e_chirho| panic!("mapFoldrWithKey sum failed: {}", e_chirho));
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(60)
    );
}

#[test]
fn eval_map_filter_values_chirho() {
    // mapFilter (>15) (fromList [(1,10),(2,20),(3,30)]) — sum elems of result = 50
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
m = Map.fromList [(1,10),(2,20),(3,30)]
main = sum (Map.elems (Map.filter (> 15) m))
";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .unwrap_or_else(|e_chirho| panic!("mapFilter sum elems failed: {}", e_chirho));
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(50)
    );
}

#[test]
fn eval_map_map_triple_chirho() {
    // mapMap (*3) (fromList [(1,10),(2,20)]) — sum values = 90
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
m = Map.fromList [(1,10),(2,20)]
main = sum (Map.elems (Map.map (\\x -> x * 3) m))
";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .unwrap_or_else(|e_chirho| panic!("mapMap (*3) sum failed: {}", e_chirho));
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(90)
    );
}

#[test]
fn eval_map_union_with_sum_chirho() {
    // mapUnionWith (+) two maps with shared key — combined value is sum
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
m1 = Map.fromList [(1,10),(2,20)]
m2 = Map.fromList [(2,5),(3,30)]
main = sum (Map.elems (Map.unionWith (+) m1 m2))
";
    // key 1→10, key 2→20+5=25, key 3→30 : total = 65
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .unwrap_or_else(|e_chirho| panic!("mapUnionWith sum failed: {}", e_chirho));
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(65)
    );
}

#[test]
fn eval_map_intersection_with_sum_chirho() {
    // mapIntersectionWith (+) keeps only shared keys and combines their values.
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
m1 = Map.fromList [(1,10),(2,20),(4,40)]
m2 = Map.fromList [(2,5),(3,30),(4,7)]
main = sum (Map.elems (Map.intersectionWith (+) m1 m2))
";
    // Shared keys: 2 -> 25 and 4 -> 47. Non-overlapping keys are omitted.
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .unwrap_or_else(|e_chirho| panic!("mapIntersectionWith sum failed: {}", e_chirho));
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(72)
    );
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
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
    );
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
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );
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
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );
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
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
    );
}
