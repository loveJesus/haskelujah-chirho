// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Data.Map and Data.Set (row 700).

use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn eval_map_fromlist_chirho() {
    // mapFromList [(1,10),(2,20),(3,30)] — size should be 3 (returned as Int)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = Map.size (Map.fromList [(1,10),(2,20),(3,30)])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!("eval_map_fromlist_chirho failed: {}", e_chirho),
    }
}

#[test]
fn eval_map_size_chirho() {
    // mapSize of singleton → 1
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = Map.size (Map.insert 1 10 Map.empty)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("Map size should be 1: {}", e_chirho),
    }
}

#[test]
fn eval_map_insert_two_keys_chirho() {
    // Insert two keys, lookup both (using Prelude mapInsert)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\ngetVal m k = case Map.lookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (Map.insert 3 99 (Map.insert 5 42 Map.empty)) 5\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("Map nested insert should work: {}", e_chirho),
    }
}

#[test]
fn eval_map_member_chirho() {
    // mapMember 5 (mapInsert 5 42 mapEmpty) → True (1)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nboolToInt b = if b then 1 else 0\nmain = boolToInt (Map.member 5 (Map.insert 5 42 Map.empty))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("Map member should be True: {}", e_chirho),
    }
}

#[test]
fn eval_map_size_two_chirho() {
    // Insert two keys, check size is 2
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = Map.size (Map.insert 3 99 (Map.insert 5 42 Map.empty))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("Map size should be 2: {}", e_chirho),
    }
}

#[test]
fn eval_map_insert_overwrite_chirho() {
    // Insert same key twice, latest value wins
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\ngetVal m k = case Map.lookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (Map.insert 5 99 (Map.insert 5 42 Map.empty)) 5\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
        ),
        Err(e_chirho) => panic!("Map insert overwrite should return 99: {}", e_chirho),
    }
}

#[test]
fn eval_map_from_list_chirho() {
    // mapFromList [(1,10),(2,20),(3,30)] then lookup key 2 → 20
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\ngetVal m k = case Map.lookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (Map.fromList [(1,10),(2,20),(3,30)]) 2\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(20)
        ),
        Err(e_chirho) => panic!("mapFromList lookup should work: {}", e_chirho),
    }
}

#[test]
fn eval_map_three_keys_chirho() {
    // Insert three keys, lookup all three
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\ngetVal m k = case Map.lookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (Map.insert 1 10 (Map.insert 3 30 (Map.insert 2 20 Map.empty))) 1 + getVal (Map.insert 1 10 (Map.insert 3 30 (Map.insert 2 20 Map.empty))) 2 + getVal (Map.insert 1 10 (Map.insert 3 30 (Map.insert 2 20 Map.empty))) 3\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(60)
        ),
        Err(e_chirho) => panic!("Map three keys sum should be 60: {}", e_chirho),
    }
}

#[test]
fn eval_map_delete_chirho() {
    // mapDelete removes a key from the map
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\ngetVal m k = case Map.lookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (Map.delete 5 (Map.insert 5 42 (Map.insert 3 99 Map.empty))) 3\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
        ),
        Err(e_chirho) => panic!("mapDelete should keep other keys: {}", e_chirho),
    }
}

#[test]
fn eval_map_delete_missing_chirho() {
    // mapDelete on a key not in the map is a no-op
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = Map.size (Map.delete 999 (Map.insert 1 10 Map.empty))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("mapDelete of missing key should be no-op: {}", e_chirho),
    }
}

#[test]
fn eval_map_keys_chirho() {
    // mapKeys extracts sorted keys from the map
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = sum (Map.keys (Map.fromList [(3,30),(1,10),(2,20)]))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
        ),
        Err(e_chirho) => panic!("mapKeys sum should be 6: {}", e_chirho),
    }
}

#[test]
fn eval_map_elems_chirho() {
    // mapElems extracts values from the map
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = sum (Map.elems (Map.fromList [(1,10),(2,20),(3,30)]))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(60)
        ),
        Err(e_chirho) => panic!("mapElems sum should be 60: {}", e_chirho),
    }
}

#[test]
fn eval_map_null_chirho() {
    // mapNull checks if map is empty
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nboolToInt b = if b then 1 else 0\nmain = boolToInt (Map.null Map.empty) + boolToInt (Map.null (Map.insert 1 10 Map.empty))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        // mapNull mapEmpty → True (1), mapNull (mapInsert...) → False (0), total 1
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("mapNull should work: {}", e_chirho),
    }
}

#[test]
fn eval_map_map_chirho() {
    // mapMap (*2) doubles all values
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\ngetVal m k = case Map.lookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (Map.map (\\x -> x * 2) (Map.fromList [(1,10),(2,20)])) 2\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(40)
        ),
        Err(e_chirho) => panic!("mapMap should double values: {}", e_chirho),
    }
}

#[test]
fn eval_map_foldl_with_key_chirho() {
    // mapFoldlWithKey sums all values
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = Map.foldlWithKey (\\acc k v -> acc + v) 0 (Map.fromList [(1,10),(2,20),(3,30)])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(60)
        ),
        Err(e_chirho) => panic!("mapFoldlWithKey sum should be 60: {}", e_chirho),
    }
}

#[test]
fn eval_map_foldl_with_key_strict_chirho() {
    // mapFoldlWithKey' reuses the strict left-fold runtime path and keeps Map typed.
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport qualified Data.Map as Map\nmain = Map.foldlWithKey' (\\acc k v -> acc + k + v) 0 (Map.fromList [(1,10),(2,20)])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(33)
        ),
        Err(e_chirho) => panic!("mapFoldlWithKey' sum should be 33: {}", e_chirho),
    }
}

#[test]
fn eval_set_singleton_member_chirho() {
    // setSingleton + setMember — use if-then-else to convert Bool to Int
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = if setMember 5 (setSingleton 5) then 1 else 0\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("setMember singleton: {}", e_chirho),
    }
}

#[test]
fn eval_set_member_not_found_chirho() {
    // setMember for element not in set — use if-then-else
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = if setMember 99 (setSingleton 5) then 1 else 0\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
        ),
        Err(e_chirho) => panic!("setMember not found: {}", e_chirho),
    }
}

#[test]
fn eval_set_size_chirho() {
    // setSize of a set built from inserts
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nmain = setSize (setInsert 3 (setInsert 1 (setInsert 2 setEmpty)))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!("setSize should be 3: {}", e_chirho),
    }
}

#[test]
fn eval_set_insert_duplicate_chirho() {
    // inserting duplicate should not increase size
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nmain = setSize (setInsert 1 (setInsert 1 (setInsert 1 setEmpty)))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("setSize with duplicates should be 1: {}", e_chirho),
    }
}

#[test]
fn eval_set_to_list_chirho() {
    // setToList should return sorted list, sum it to verify
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = sum (setToList (setInsert 3 (setInsert 1 (setInsert 2 setEmpty))))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
        ),
        Err(e_chirho) => panic!("sum (setToList {{1,2,3}}) should be 6: {}", e_chirho),
    }
}

#[test]
fn eval_set_from_list_chirho() {
    // setFromList then setSize
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = setSize (setFromList [5,3,5,1,3])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!(
            "setSize (setFromList [5,3,5,1,3]) should be 3: {}",
            e_chirho
        ),
    }
}

#[test]
fn eval_set_delete_chirho() {
    // Delete element then check size
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = setSize (setDelete 2 (setFromList [1,2,3]))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("setDelete: {}", e_chirho),
    }
}

#[test]
fn eval_set_union_chirho() {
    // Union of two sets
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = setSize (setUnion (setFromList [1,2,3]) (setFromList [3,4,5]))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
        ),
        Err(e_chirho) => panic!("setUnion: {}", e_chirho),
    }
}

#[test]
fn eval_set_intersection_chirho() {
    // Intersection of two sets
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = setSize (setIntersection (setFromList [1,2,3,4]) (setFromList [3,4,5,6]))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("setIntersection: {}", e_chirho),
    }
}

#[test]
fn eval_set_difference_chirho() {
    // Difference: elements in first but not second
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = setSize (setDifference (setFromList [1,2,3,4]) (setFromList [3,4,5]))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("setDifference: {}", e_chirho),
    }
}

#[test]
fn eval_set_filter_chirho() {
    // Filter elements > 3
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = sum (setToList (setFilter (\\x -> x > 3) (setFromList [1,2,3,4,5])))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(9)
        ),
        Err(e_chirho) => panic!("setFilter: {}", e_chirho),
    }
}

#[test]
fn eval_set_map_chirho() {
    // Map (*2) over set
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nmain = sum (setToList (setMap (\\x -> x * 2) (setFromList [1,2,3])))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(12)
        ),
        Err(e_chirho) => panic!("setMap: {}", e_chirho),
    }
}

#[test]
fn eval_set_fold_chirho() {
    // Fold (+) over set
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nmain = setFold (\\x acc -> x + acc) 0 (setFromList [1,2,3,4,5])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
        ),
        Err(e_chirho) => panic!("setFold: {}", e_chirho),
    }
}

// ── Type synonym in user code ───────────────────────────────────

#[test]
fn eval_map_find_with_default_chirho() {
    // mapFindWithDefault 99 5 mapEmpty → 99 (key not found, returns default)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
main = Map.findWithDefault 99 5 Map.empty
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
        ),
        Err(e_chirho) => panic!("mapFindWithDefault: {}", e_chirho),
    }
}

#[test]
fn eval_map_find_with_default_found_chirho() {
    // mapFindWithDefault 99 1 (mapSingleton 1 42) → 42 (key found)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
main = Map.findWithDefault 99 1 (Map.singleton 1 42)
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("mapFindWithDefault found: {}", e_chirho),
    }
}

#[test]
fn eval_map_adjust_chirho() {
    // mapAdjust (*10) 1 (mapSingleton 1 5) → value at key 1 is 5*10=50
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
m = Map.adjust (*10) 1 (Map.singleton 1 5)
main = Map.findWithDefault 0 1 m
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(50)
        ),
        Err(e_chirho) => panic!("mapAdjust: {}", e_chirho),
    }
}

#[test]
fn eval_map_union_chirho() {
    // mapUnion (mapSingleton 1 10) (mapSingleton 2 20) → size 2
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
m = Map.union (Map.singleton 1 10) (Map.singleton 2 20)
main = Map.size m
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("mapUnion: {}", e_chirho),
    }
}

#[test]
fn eval_map_difference_chirho() {
    // mapDifference (fromList [(1,10),(2,20),(3,30)]) (mapSingleton 2 99) → size 2
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
m1 = Map.insert 1 10 (Map.insert 2 20 (Map.insert 3 30 Map.empty))
m2 = Map.singleton 2 99
main = Map.size (Map.difference m1 m2)
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("mapDifference: {}", e_chirho),
    }
}

#[test]
fn eval_map_filter_chirho() {
    // mapFilter (> 15) (fromList [(1,10),(2,20),(3,30)]) → size 2 (values 20 and 30)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
m = Map.insert 1 10 (Map.insert 2 20 (Map.insert 3 30 Map.empty))
main = Map.size (Map.filter (> 15) m)
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("mapFilter: {}", e_chirho),
    }
}

#[test]
fn eval_map_str_multiple_chirho() {
    // Insert multiple string keys and verify size
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
m = Map.insert \"a\" 1 (Map.insert \"b\" 2 (Map.insert \"c\" 3 Map.empty))
main = Map.size m
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!("mapStr multiple: {}", e_chirho),
    }
}

#[test]
fn eval_map_fold_sum_chirho() {
    // Use mapFoldlWithKey to sum all values in a map
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
m = Map.insert 1 10 (Map.insert 2 20 (Map.insert 3 30 Map.empty))
main = Map.foldlWithKey (\\acc k v -> acc + v) 0 m
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        // 10 + 20 + 30 = 60
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(60)
        ),
        Err(e_chirho) => panic!("map fold sum: {}", e_chirho),
    }
}

#[test]
fn eval_string_map_values_chirho() {
    // String-keyed map: insert 3 entries, lookup one value
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
m = Map.insert \"foo\" 10 (Map.insert \"bar\" 20 (Map.insert \"baz\" 30 Map.empty))
main = Map.findWithDefault 0 \"bar\" m
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(20)
        ),
        Err(e_chirho) => panic!("string map values: {}", e_chirho),
    }
}
