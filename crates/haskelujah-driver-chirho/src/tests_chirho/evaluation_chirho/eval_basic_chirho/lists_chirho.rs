// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The list Prelude functions (row 700).

use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn eval_take_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\n\
             main = putStrLn (take 5 \"hello world\")\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("take should evaluate");
    assert_eq!(result_chirho.1.io_output_chirho, "hello\n");
}

#[test]
fn eval_drop_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\n\
             main = putStrLn (drop 6 \"hello world\")\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("drop should evaluate");
    assert_eq!(result_chirho.1.io_output_chirho, "world\n");
}

#[test]
fn eval_concat_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // concat (words "abc def") should give "abcdef"
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\n\
             main = putStrLn (concat (words \"abc def\"))\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("concat should evaluate");
    assert_eq!(result_chirho.1.io_output_chirho, "abcdef\n");
}

// ── Priority 59: List comprehensions ─────────────────────────

#[test]
fn eval_map_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // head (map (\x -> x + 1) [10, 20]) should be 11
    let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
main = myHead (map (\\x -> x + 1) [10, 20])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(11),
            );
        }
        Err(e_chirho) => panic!("map should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_filter_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // head (filter (\x -> x == 3) [2, 3, 4]) should be 3
    let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
main = myHead (filter (\\x -> x == 3) [2, 3, 4])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(3),
            );
        }
        Err(e_chirho) => panic!("filter should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_foldr_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // foldr (\x acc -> x + acc) 0 [1, 2, 3] should be 6
    let src_chirho = "\
module Test where
main = foldr (\\x acc -> x + acc) 0 [1, 2, 3]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(6),
            );
        }
        Err(e_chirho) => panic!("foldr should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_length_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // length [10, 20, 30] should be 3
    let src_chirho = "\
module Test where
main = length [10, 20, 30]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(3),
            );
        }
        Err(e_chirho) => panic!("length should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_head_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // head [99, 100] should be 99
    let src_chirho = "\
module Test where
main = head [99, 100]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(99),
            );
        }
        Err(e_chirho) => panic!("head should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_null_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // null [] should be True; we test by converting to int via case
    let src_chirho = "\
module Test where
boolToInt b = case b of
  True -> 1
  False -> 0
main = boolToInt (null [])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1),
            );
        }
        Err(e_chirho) => panic!("null should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_reverse_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // head (reverse [1, 2, 3]) should be 3
    let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
main = myHead (reverse [1, 2, 3])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(3),
            );
        }
        Err(e_chirho) => panic!("reverse should evaluate: {}", e_chirho),
    }
}

// -----------------------------------------------------------------------
// Priority 63: Additional Prelude list functions
// -----------------------------------------------------------------------

#[test]
fn eval_append_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = length (append [1, 2, 3] [4, 5])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("append should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5),
        "length (append [1,2,3] [4,5]) = 5"
    );
}

#[test]
fn eval_any_true_chirho() {
    // any with predicate: use if-then-else that returns an Int to verify
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if any (\\x -> x > 3) [1, 2, 4, 5] then 1 else 0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("any should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1),
    );
}

#[test]
fn eval_any_false_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if any (\\x -> x > 10) [1, 3, 5, 7] then 1 else 0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("any false should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0),
    );
}

#[test]
fn eval_all_true_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if all (\\x -> x > 0) [1, 2, 3] then 1 else 0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("all true should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1),
    );
}

#[test]
fn eval_all_false_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if all (\\x -> x > 0) [1, 0, 3] then 1 else 0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("all false should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0),
    );
}

#[test]
fn eval_sum_builtin_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = sum [1, 2, 3, 4, 5]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("sum builtin should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(15),
        "sum [1,2,3,4,5] = 15"
    );
}

#[test]
fn eval_product_builtin_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = product [1, 2, 3, 4, 5]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("product builtin should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(120),
        "product [1,2,3,4,5] = 120"
    );
}

#[test]
fn eval_last_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = last [1, 2, 3, 4, 5]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("last should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5),
        "last [1,2,3,4,5] = 5"
    );
}

#[test]
fn eval_init_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = length (init [1, 2, 3, 4, 5])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("init should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(4),
        "length (init [1,2,3,4,5]) = 4"
    );
}

// ---------------------------------------------------------------
// Priority 66: Polymorphic take/drop on lists
// ---------------------------------------------------------------

// ---------------------------------------------------------------
// Priority 66: Polymorphic take/drop on lists
// ---------------------------------------------------------------

#[test]
fn eval_take_int_list_chirho() {
    // take 3 [10,20,30,40,50] → [10,20,30], sum = 60
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = sum (take 3 [10, 20, 30, 40, 50])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("take on int list should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(60),
        "sum (take 3 [10,20,30,40,50]) = 60"
    );
}

#[test]
fn eval_drop_int_list_chirho() {
    // drop 2 [10,20,30,40,50] → [30,40,50], sum = 120
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = sum (drop 2 [10, 20, 30, 40, 50])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("drop on int list should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(120),
        "sum (drop 2 [10,20,30,40,50]) = 120"
    );
}

#[test]
fn eval_minimum_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = minimum [5,3,8,1,4]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("minimum should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_maximum_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = maximum [5,3,8,1,4]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("maximum should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(8)
    );
}

#[test]
fn eval_sort_chirho() {
    // sort [3,1,4,1,5,9] → [1,1,3,4,5,9], sum = 23
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.List (sort)
main = sum (sort [3,1,4,1,5,9])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("sort should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(23)
    );
}

#[test]
fn eval_odd_filter_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = length (filter odd [1,2,3,4,5,6])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("odd filter should evaluate");
    // odd values: 1,3,5 → length = 3
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );
}

#[test]
fn eval_replicate_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = sum (replicate 5 3)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("replicate should evaluate");
    // replicate 5 3 = [3,3,3,3,3], sum = 15
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
    );
}

#[test]
fn eval_dropwhile_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = head (dropWhile (\\x -> x < 5) [1,2,3,4,5,6,7])\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("dropWhile should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
    );
}

#[test]
fn eval_iterate_chirho() {
    use crate::eval_source_chirho;
    // take 5 (iterate (*2) 1) = [1,2,4,8,16], sum = 31
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = sum (take 5 (iterate (\\x -> x * 2) 1))\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("iterate should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(31)
    );
}

#[test]
fn eval_scanl_chirho() {
    use crate::eval_source_chirho;
    // scanl (+) 0 [1,2,3] = [0,1,3,6], last = 6
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = last (scanl (\\acc x -> acc + x) 0 [1,2,3])\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("scanl should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
    );
}

#[test]
fn eval_span_chirho() {
    use crate::eval_source_chirho;
    // span (<5) [1,2,3,4,5,6] = ([1,2,3,4],[5,6]), fst has length 4
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = length (fst (span (\\x -> x < 5) [1,2,3,4,5,6]))\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("span should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(4)
    );
}

#[test]
fn eval_break_chirho() {
    use crate::eval_source_chirho;
    // break (>=5) [1,2,3,4,5,6] = ([1,2,3,4],[5,6]), snd head = 5
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = head (snd (break (\\x -> x >= 5) [1,2,3,4,5,6]))\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("break should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
    );
}

#[test]
fn eval_partition_chirho() {
    use crate::eval_source_chirho;
    // partition even [1,2,3,4,5,6] = ([2,4,6],[1,3,5]), fst has length 3
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nimport Data.List (partition)\nmain = length (fst (partition even [1,2,3,4,5,6]))\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("partition should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );
}

#[test]
fn eval_append_basic_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Test basic list append via concatMap-like pattern
    let result_chirho = eval_source_chirho(
        "module Test where\nappend xs ys = case xs of { [] -> ys; (h:t) -> h : append t ys }\nmain = length (append [1,2] [3,4,5])\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
            );
        }
        Err(e_chirho) => panic!("append should work: {}", e_chirho),
    }
}

#[test]
fn eval_zip_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // zip [1,2,3] [10,20,30], take fst of first pair
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = fst (head (zip [1,2,3] [10,20,30]))\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
            );
        }
        Err(e_chirho) => panic!("zip should work: {}", e_chirho),
    }
}

#[test]
fn eval_nub_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // nub removes duplicates, sum [1,2,3] = 6
    let result_chirho = eval_source_chirho(
        "module Test where\nnub [] = []\nnub (x:xs) = x : nub (filter (\\y -> not (y == x)) xs)\nmain = sum (nub [1,2,2,3,3,3])\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
            );
        }
        Err(e_chirho) => panic!("nub should work: {}", e_chirho),
    }
}

#[test]
fn eval_assoc_list_not_found_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Association list lookup with key not found
    let result_chirho = eval_source_chirho(
        "module Test where\nlookupA k xs = case xs of\n  [] -> 0\n  ((k2,v):rest) -> if k == k2 then v else lookupA k rest\nmain = lookupA 5 [(1,10),(2,20),(3,30)]\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
        ),
        Err(e_chirho) => panic!("assoc list lookup not found should return 0: {}", e_chirho),
    }
}

#[test]
fn eval_powers_of_two_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Build list of powers of two via recursion: [1,2,4,8,16], sum = 31
    let result_chirho = eval_source_chirho(
        "module Test where\npowers n x = if n == 0 then [] else x : powers (n - 1) (x * 2)\nmain = sum (powers 5 1)\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(31)
        ),
        Err(e_chirho) => panic!("powers of two should work: {}", e_chirho),
    }
}

// ── String comparison / Ord [Char] tests ─────────────────────────

#[test]
fn eval_product_chirho() {
    // product [1,2,3,4,5] → 120
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = product [1,2,3,4,5]\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(120)
        ),
        Err(e_chirho) => panic!("product: {}", e_chirho),
    }
}

#[test]
fn eval_nub_head_chirho() {
    // head (nub [3,1,3,2]) → 3 (first unique is 3)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.List (nub)
main = head (nub [3,1,3,2])
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!("nub head: {}", e_chirho),
    }
}

#[test]
fn eval_intersperse_sum_chirho() {
    // sum (intersperse 0 [1,2,3]) → 1+0+2+0+3 = 6
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.List (intersperse)
main = sum (intersperse 0 [1,2,3])
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
        ),
        Err(e_chirho) => panic!("intersperse sum: {}", e_chirho),
    }
}

#[test]
fn eval_map_double_sum_chirho() {
    // map (*2) then sum: sum (map (*2) [1..5]) = 2+4+6+8+10 = 30
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = sum (map (*2) [1..5])
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(30)
        ),
        Err(e_chirho) => panic!("map double sum: {}", e_chirho),
    }
}

#[test]
fn eval_map_square_sum_chirho() {
    // map (\x -> x * x) [1,2,3,4] → [1,4,9,16] → sum = 30
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = sum (map (\\x -> x * x) [1,2,3,4])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(30)
        ),
        Err(e_chirho) => panic!("map with lambda: {}", e_chirho),
    }
}

// ── Accumulator pattern with recursive list building ──────────────

/// Test 3: digits decomposition + sum — accumulator with ++ and recursion.
/// sum (digits 12345) = 1+2+3+4+5 = 15

#[test]
fn eval_powerset_chirho() {
    // Power set of [1,2,3] has 2^3 = 8 elements
    // Tests recursive list manipulation with inline lambda (x:e)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
powerset xss = case xss of
  [] -> [[]]
  (x:xs) -> let ps = powerset xs in ps ++ map (\e -> x : e) ps
main = length (powerset [1,2,3])
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .unwrap_or_else(|e_chirho| panic!("powerset failed: {}", e_chirho));
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(8)
    );
}

#[test]
fn eval_pascal_triangle_chirho() {
    // Pascal's triangle row 10: sum = 2^10 = 1024.
    // Tests zipWith with list concatenation and prepending.
    // Uses helper function to pass prev explicitly, avoiding shared-thunk issues.
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
nextRow prev = zipWith (+) (0 : prev) (prev ++ [0])
pascal n = if n == 0 then [1] else nextRow (pascal (n - 1))
main = sum (pascal 10)
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .unwrap_or_else(|e_chirho| panic!("pascal triangle failed: {}", e_chirho));
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1024)
    );
}
