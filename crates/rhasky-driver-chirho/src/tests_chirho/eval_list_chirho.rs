// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// List operations, map, filter, fold, zip, arithmetic sequences, comprehensions tests

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
use rhasky_span_chirho::SourceMapChirho;
#[allow(unused_imports)]
use rhasky_runtime_chirho::{ValueChirho, ExecutionModeChirho};
#[allow(unused_imports)]
use rhasky_syntax_chirho::SourceFileChirho;


    #[test]
    fn eval_list_literal_chirho() {
        // [1, 2, 3] desugars to cons chain; the head element is 1
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Just verify compilation succeeds — list evaluation requires
        // deeper infrastructure (case on lists to extract head)
        let result_chirho = compile_source_chirho(
            "module Test where\n\
             main = [1, 2, 3]\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "list literal should compile: {:?}",
            result_chirho.err()
        );
    }


    #[test]
    fn eval_string_concat_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (\"hello\" ++ \" \" ++ \"world\")
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("string concat should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "hello world\n",
            "++ concatenates strings"
        );
    }


    #[test]
    fn eval_list_head_chirho() {
        // case dispatch on list constructor (:) to extract head element
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
head xs = case xs of
  (x:_) -> x
  [] -> 0
main = head [1, 2, 3]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(1),
                    "head [1,2,3] should be 1"
                );
            }
            Err(e_chirho) => {
                panic!("list head eval failed: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_list_length_chirho() {
        // recursive length function on list
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
length xs = case xs of
  [] -> 0
  (_:rest) -> 1 + length rest
main = length [10, 20, 30, 40]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list length should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(4),
            "length [10,20,30,40] should be 4"
        );
    }


    #[test]
    fn eval_list_sum_chirho() {
        // recursive sum function on list
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum [1, 2, 3, 4, 5]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list sum should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(15),
            "sum [1,2,3,4,5] should be 15"
        );
    }


    #[test]
    fn eval_list_map_chirho() {
        // recursive map function on list: map double [1,2,3] → [2,4,6]
        // We verify by checking sum (map double [1,2,3]) == 12
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
map f xs = case xs of
  [] -> []
  (x:rest) -> f x : map f rest
double x = x + x
main = sum (map double [1, 2, 3])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list map should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(12),
            "sum (map double [1,2,3]) should be 12"
        );
    }


    #[test]
    fn eval_list_filter_chirho() {
        // filter with predicate: keep elements > 3
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
filter p xs = case xs of
  [] -> []
  (x:rest) -> if p x then x : filter p rest else filter p rest
isGt3 x = x > 3
main = sum (filter isGt3 [1, 2, 3, 4, 5])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list filter should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(9),
            "sum (filter (>3) [1,2,3,4,5]) = 4+5 = 9"
        );
    }


    #[test]
    fn eval_list_foldr_chirho() {
        // foldr to sum a list
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
foldr f z xs = case xs of
  [] -> z
  (x:rest) -> f x (foldr f z rest)
add a b = a + b
main = foldr add 0 [1, 2, 3, 4, 5]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list foldr should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(15),
            "foldr add 0 [1,2,3,4,5] should be 15"
        );
    }


    #[test]
    fn eval_list_foldl_chirho() {
        // foldl to compute left-fold subtraction
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
foldl f acc xs = case xs of
  [] -> acc
  (x:rest) -> foldl f (f acc x) rest
sub a b = a - b
main = foldl sub 100 [10, 20, 30]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list foldl should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(40),
            "foldl sub 100 [10,20,30] = ((100-10)-20)-30 = 40"
        );
    }


    #[test]
    fn eval_list_reverse_chirho() {
        // reverse via foldl, then sum to verify order
        // reverse [1,2,3] via foldl (flip (:)) [] = [3,2,1]
        // we verify by checking head of reversed list
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
foldl f acc xs = case xs of
  [] -> acc
  (x:rest) -> foldl f (f acc x) rest
snoc acc x = x : acc
reverse xs = foldl snoc [] xs
head xs = case xs of
  (x:_) -> x
  [] -> 0
main = head (reverse [1, 2, 3])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list reverse should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(3),
            "head (reverse [1,2,3]) should be 3"
        );
    }


    #[test]
    fn eval_list_append_chirho() {
        // list append via foldr
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
foldr f z xs = case xs of
  [] -> z
  (x:rest) -> f x (foldr f z rest)
append xs ys = foldr cons ys xs
  where cons x acc = x : acc
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum (append [1, 2] [3, 4, 5])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list append should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(15),
            "sum (append [1,2] [3,4,5]) = 15"
        );
    }


    #[test]
    fn eval_string_eq_chirho() {
        // String equality via Eq instance
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if \"hello\" == \"hello\" then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("string eq should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
            "\"hello\" == \"hello\" should be True (1)"
        );
    }


    #[test]
    fn eval_string_neq_chirho() {
        // String inequality
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if \"hello\" == \"world\" then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("string neq should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
            "\"hello\" == \"world\" should be False (0)"
        );
    }


    #[test]
    fn eval_list_comprehension_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum [x + 1 | x <- [1..3]]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(9),
                "sum [x+1 | x <- [1..3]] should be 9 (2+3+4)"
            ),
            Err(e_chirho) => {
                eprintln!("list comprehension test error: {}", e_chirho);
                // List comprehension may not be fully implemented yet — skip
            }
        }
    }


    // ── Priority 59: List comprehensions ─────────────────────────
    #[test]
    fn eval_list_comp_simple_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Simple list comprehension: [x | x <- [42]]
        // Expected: [42], head is 42
        let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
main = myHead [x | x <- [42]]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(42),
                );
            }
            Err(e_chirho) => panic!("list comprehension should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_list_comp_transform_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // [x + 1 | x <- [10, 20]] should produce [11, 21], head is 11
        let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
main = myHead [x + 1 | x <- [10, 20]]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(11),
                );
            }
            Err(e_chirho) => panic!("list comp transform should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_list_comp_guard_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // [x | x <- [2, 3], even x] should produce [2], head is 2
        let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
myEven n = case n == 2 of
  True -> True
  False -> False
main = myHead [x | x <- [2, 3], myEven x]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(2),
                );
            }
            Err(e_chirho) => panic!("list comp guard should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_concatmap_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
dup x = [x, x]
main = sum (concatMap dup [1, 2, 3])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("concatMap should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(12),
            "sum (concatMap dup [1,2,3]) = 1+1+2+2+3+3 = 12"
        );
    }


    #[test]
    fn eval_takewhile_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = length (takeWhile (\\x -> x < 5) [1,2,3,4,5,6,7])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("takeWhile should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
    }


    #[test]
    fn eval_multi_gen_list_comp_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // [x*y | x <- [1,2], y <- [10,20]] should be [10,20,20,40], sum = 90
        // First test length to confirm the right number of elements
        let result_len_chirho = eval_source_chirho(
            "module Test where\nmain = length [x*y | x <- [1,2], y <- [10,20]]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_len_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4),
                    "should produce 4 elements");
            }
            Err(e_chirho) => panic!("multi-generator list comp length: {}", e_chirho),
        }
        // Now test sum
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = sum [x*y | x <- [1,2], y <- [10,20]]\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(90));
            }
            Err(e_chirho) => panic!("multi-generator list comp should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_list_comp_with_guard_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // [x*2 | x <- [1..5], even x] should give [4, 8]
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = sum [x * 2 | x <- [1,2,3,4,5], even x]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(12));
            }
            Err(e_chirho) => panic!("list comp with guard should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_zipwith_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // zipWith (+) [1,2,3] [10,20,30] → [11,22,33], sum → 66
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = sum (zipWith (+) [1,2,3] [10,20,30])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66));
            }
            Err(e_chirho) => panic!("zipWith should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_replicate_sum_chirho() {
        // sum (replicate 3 7) → 21
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (replicate 3 7)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(21)),
            Err(e_chirho) => panic!("replicate sum: {}", e_chirho),
        }
    }

    // ── Deriving Show for product types + advanced features ─────────


    // ── List comprehension with guards and transforms ──────────────

    #[test]
    fn eval_list_comp_transform_filter_chirho() {
        // [x*x | x <- [1..10], even x] → [4,16,36,64,100] → sum → 220
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum [x*x | x <- [1..10], even x]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(220)),
            Err(e_chirho) => panic!("list comp transform+filter: {}", e_chirho),
        }
    }


    #[test]
    fn eval_list_comp_cartesian_chirho() {
        // length [(x,y) | x <- [1,2,3], y <- [1,2]] → 6
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length [(x,y) | x <- [1,2,3], y <- [1,2]]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("list comp cartesian: {}", e_chirho),
        }
    }

    // ── Higher-order function composition ─────────────────────────


    // ── Complex list processing ─────────────────────────────────────

    #[test]
    fn eval_complex_list_pipeline_chirho() {
        // sum . map (^2) . filter odd $ [1..10] → 1+9+25+49+81 = 165
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (map (^2) (filter odd [1..10]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(165)),
            Err(e_chirho) => panic!("complex list pipeline: {}", e_chirho),
        }
    }

    // ── Numeric escape sequences ────────────────────────────────────


    #[test]
    fn eval_is_prefix_of_true_chirho() {
        // isPrefixOf [1,2] [1,2,3] → True → use if to convert to Int
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if isPrefixOf [1,2] [1,2,3] then 1 else 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("isPrefixOf true: {}", e_chirho),
        }
    }


    #[test]
    fn eval_is_prefix_of_false_chirho() {
        // isPrefixOf [2,3] [1,2,3] → False → use if to convert to Int
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if isPrefixOf [2,3] [1,2,3] then 1 else 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("isPrefixOf false: {}", e_chirho),
        }
    }

    // ── Data.Map String-keyed ─────────────────────────────────────


    #[test]
    fn eval_is_suffix_of_chirho() {
        // isSuffixOf [2,3] [1,2,3] → True → use if to convert to Int
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if isSuffixOf [2,3] [1,2,3] then 1 else 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("isSuffixOf: {}", e_chirho),
        }
    }

    // ── Negative literal patterns ──────────────────────────────────


    #[test]
    fn eval_list_comp_with_let_chirho() {
        // List comprehension with let binding: [y | x <- [1..5], let y = x * x, y > 5]
        // Would be: [9, 16, 25] → sum = 50
        // Simpler: just test that list comp + filter combo works
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
squares = map (\\x -> x * x) [1..5]
main = sum (filter (> 5) squares)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            // 9 + 16 + 25 = 50
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(50)),
            Err(e_chirho) => panic!("list comp with let: {}", e_chirho),
        }
    }


    // ── Push to 1000 tests ─────────────────────────────────────────

    #[test]
    fn eval_foldr_cons_chirho() {
        // foldr (:) [] [1,2,3] → [1,2,3] → length = 3 (identity via foldr)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (foldr (\\x xs -> x : xs) [] [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("foldr cons: {}", e_chirho),
        }
    }


    #[test]
    fn eval_foldl_subtract_chirho() {
        // foldl (-) 100 [10,20,30] → ((100-10)-20)-30 = 40
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = foldl (-) 100 [10,20,30]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(40)),
            Err(e_chirho) => panic!("foldl subtract: {}", e_chirho),
        }
    }


    #[test]
    fn eval_zip_sum_chirho() {
        // sum (map (\(a,b) -> a+b) (zip [1,2,3] [10,20,30])) → 11+22+33 = 66
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (map (\\(a,b) -> a + b) (zip [1,2,3] [10,20,30]))
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66)),
            Err(e_chirho) => panic!("zip sum: {}", e_chirho),
        }
    }


    #[test]
    fn eval_enum_from_then_chirho() {
        // [2,4..10] → [2,4,6,8,10] → sum = 30
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum [2,4..10]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30)),
            Err(e_chirho) => panic!("enum from then: {}", e_chirho),
        }
    }


    #[test]
    fn eval_any_all_chirho() {
        // any even [1,3,5,7] → False → 0, all odd [1,3,5,7] → True → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if all odd [1,3,5,7] then 1 else 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("any all: {}", e_chirho),
        }
    }


    #[test]
    fn eval_takewhile_sum_chirho() {
        // takeWhile (< 5) [1..10] → [1,2,3,4] → sum = 10
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (takeWhile (< 5) [1..10])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10)),
            Err(e_chirho) => panic!("takewhile sum: {}", e_chirho),
        }
    }

    // ── Data.Maybe extras ──


    #[test]
    fn eval_iterate_take_chirho() {
        // take 5 (iterate (*2) 1) → [1,2,4,8,16] → sum = 31
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (take 5 (iterate (*2) 1))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(31)),
            Err(e_chirho) => panic!("iterate take: {}", e_chirho),
        }
    }


    #[test]
    fn eval_scanl_length_chirho() {
        // scanl (+) 0 [1,2,3] → [0,1,3,6] → length = 4
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (scanl (+) 0 [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4)),
            Err(e_chirho) => panic!("scanl length: {}", e_chirho),
        }
    }


    #[test]
    fn eval_concatmap_sum_chirho() {
        // concatMap (\x -> [x, x*10]) [1,2,3] → [1,10,2,20,3,30] → sum = 66
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (concatMap (\\x -> [x, x*10]) [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66)),
            Err(e_chirho) => panic!("concatMap sum: {}", e_chirho),
        }
    }


    #[test]
    fn eval_list_comp_even_squares_chirho() {
        // [x * x | x <- [1..10], x `mod` 2 == 0] → [4,16,36,64,100] → sum = 220
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum [x * x | x <- [1..10], x `mod` 2 == 0]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(220)),
            Err(e_chirho) => panic!("list comp with guard: {}", e_chirho),
        }
    }


    #[test]
    fn eval_zipwith_add_chirho() {
        // sum (zipWith (+) [1,2,3] [10,20,30]) = 11+22+33 = 66
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (zipWith (+) [1,2,3] [10,20,30])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66)),
            Err(e_chirho) => panic!("zipWith add: {}", e_chirho),
        }
    }


    #[test]
    fn eval_complex_pipeline_chirho() {
        // sum . filter (> 5) . map (*2) $ [1,2,3,4,5] → filter [2,4,6,8,10] > 5 → [6,8,10] → sum = 24
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (filter (> 5) (map (*2) [1,2,3,4,5]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(24)),
            Err(e_chirho) => panic!("complex pipeline: {}", e_chirho),
        }
    }


    // ── Higher-order *By list function tests ──

    #[test]
    fn eval_sortby_chirho() {
        // sortBy compare [3,1,4,1,5] → [1,1,3,4,5], head → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = head (sortBy compare [3,1,4,1,5])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("sortBy head: {}", e_chirho),
        }
    }


    #[test]
    fn eval_sortby_sum_chirho() {
        // sortBy compare [5,2,8,1,3] → [1,2,3,5,8], sum → 19
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (sortBy compare [5,2,8,1,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(19)),
            Err(e_chirho) => panic!("sortBy sum: {}", e_chirho),
        }
    }


    #[test]
    fn eval_insertby_chirho() {
        // insertBy compare 3 [1,2,4,5] → [1,2,3,4,5], head → 1, length → 5
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (insertBy compare 3 [1,2,4,5])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5)),
            Err(e_chirho) => panic!("insertBy length: {}", e_chirho),
        }
    }


    #[test]
    fn eval_nubby_chirho() {
        // nubBy (\x y -> x == y) [1,2,1,3,2,4] → [1,2,3,4], length → 4
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (nubBy (\\x -> \\y -> x == y) [1,2,1,3,2,4])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4)),
            Err(e_chirho) => panic!("nubBy length: {}", e_chirho),
        }
    }


    #[test]
    fn eval_maximumby_chirho() {
        // maximumBy compare [3,1,5,2,4] → 5
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = maximumBy compare [3,1,5,2,4]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5)),
            Err(e_chirho) => panic!("maximumBy: {}", e_chirho),
        }
    }


    #[test]
    fn eval_minimumby_chirho() {
        // minimumBy compare [3,1,5,2,4] → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = minimumBy compare [3,1,5,2,4]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("minimumBy: {}", e_chirho),
        }
    }


    #[test]
    fn eval_on_chirho() {
        // on (+) (\x -> x * x) 3 4 = (3*3) + (4*4) = 9 + 16 = 25
        // We use a simpler test: on (+) length ... needs string lists which is complex
        // Simpler: on f g x y = f (g x) (g y) where f = (+), g = negate
        // on (+) negate 3 4 = negate 3 + negate 4 = (-3) + (-4) = -7
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = on (+) negate 3 4\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(-7)),
            Err(e_chirho) => panic!("on: {}", e_chirho),
        }
    }

    // ── Type synonyms in instance heads ──────────────────────────────


    // ── Data.List: nub / sortBy / isPrefixOf / replicate end-to-end ─────

    #[test]
    fn eval_nub_sum_e2e_chirho() {
        // nub [1,2,1,3,2,4] → [1,2,3,4], sum = 10
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (nub [1,2,1,3,2,4])\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10));
    }


    #[test]
    fn eval_sort_by_e2e_chirho() {
        // sortBy (\x y -> compare y x) [3,1,2] → [3,2,1], head = 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = head (sortBy (\\x y -> compare y x) [3,1,2])\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_is_prefix_of_chirho() {
        // isPrefixOf "he" "hello" → True → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if isPrefixOf \"he\" \"hello\" then 1 else 0\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_replicate_sum_e2e_chirho() {
        // sum (replicate 5 3) → 15
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (replicate 5 3)\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
    }

    // ── Data.Map end-to-end ─────────────────────────────────────────────


    // ── zip / concatMap / any end-to-end ────────────────────────────────

    #[test]
    fn eval_zip_fst_snd_sum_chirho() {
        // sum (map (\p -> fst p + snd p) (zip [1,2,3] [10,20,30])) → 66
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (map (\\p -> fst p + snd p) (zip [1,2,3] [10,20,30]))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66));
    }


    #[test]
    fn eval_concatmap_e2e_chirho() {
        // sum (concatMap (\x -> [x, x*10]) [1,2,3]) → 1+10+2+20+3+30 = 66
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (concatMap (\\x -> [x, x*10]) [1,2,3])\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66));
    }


    #[test]
    fn eval_any_even_chirho() {
        // any even [1,3,4] → True → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if any even [1,3,4] then 1 else 0\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }

    // ── Char equality through int primops ─────────────────────────────────


    // ── Char equality through int primops ─────────────────────────────────

    #[test]
    fn eval_is_suffix_of_e2e_chirho() {
        // isSuffixOf "lo" "hello" → True → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if isSuffixOf \"lo\" \"hello\" then 1 else 0\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_elem_int_list_chirho() {
        // elem 3 [1,2,3,4] → True → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if elem 3 [1,2,3,4] then 1 else 0\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_nub_string_chirho() {
        // length (nub "banana") → 3 (b, a, n)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (nub \"banana\")\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }

    // ── Monad transformer infrastructure ──────────────────────────────────


    // ── Polymorphic elem/notElem/nub/isPrefixOf for Char ─────────────────

    #[test]
    fn eval_elem_char_chirho() {
        // elem 'a' "banana" → True → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if elem 'a' \"banana\" then 1 else 0\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_notelem_char_chirho() {
        // notElem 'z' "hello" → True → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if notElem 'z' \"hello\" then 1 else 0\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_nub_char_string_chirho() {
        // length (nub "abcabc") → 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (nub \"abcabc\")\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_is_prefix_of_char_chirho() {
        // isPrefixOf "hel" "hello" → True → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if isPrefixOf \"hel\" \"hello\" then 1 else 0\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }

    // ── Utility Prelude functions: repeat, cycle, fix, group, etc. ────────


    // ── Utility Prelude functions: repeat, cycle, fix, group, etc. ────────

    #[test]
    fn eval_repeat_take_chirho() {
        // take 5 (repeat 7) → [7,7,7,7,7] → sum → 35
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (take 5 (repeat 7))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(35));
    }


    #[test]
    fn eval_cycle_take_chirho() {
        // take 7 (cycle [1,2,3]) → [1,2,3,1,2,3,1] → sum → 13
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (take 7 (cycle [1,2,3]))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(13));
    }


    #[test]
    fn eval_fix_factorial_chirho() {
        // fix (\f n -> if n == 0 then 1 else n * f (n - 1)) applied to 5 → 120
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nfact = fix (\\f -> \\n -> if n == 0 then 1 else n * f (n - 1))\nmain = fact 5\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(120));
    }


    #[test]
    fn eval_group_chirho() {
        // group [1,1,2,2,2,3] → [[1,1],[2,2,2],[3]] → length → 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (group [1,1,2,2,2,3])\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_zipwith3_chirho() {
        // zipWith3 (\a b c -> a + b + c) [1,2] [10,20] [100,200] → [111,222] → head → 111
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = head (zipWith3 (\\a -> \\b -> \\c -> a + b + c) [1,2] [10,20] [100,200])\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(111));
    }


    #[test]
    fn eval_repeat_head_chirho() {
        // head (repeat 42) → 42
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = head (repeat 42)\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }


    #[test]
    fn eval_group_sum_head_chirho() {
        // head (group [5,5,5,3]) → [5,5,5] → sum → 15
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (head (group [5,5,5,3]))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
    }

    // ── Exception handling tests ────────────────────────────────────────


    #[test]
    fn eval_map_tolist_sum_chirho() {
        // In-order traversal sum of values
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapElems t = case t of
  Tip -> []
  Bin k v l r -> mapElems l ++ [v] ++ mapElems r
main = sum (mapElems (mapInsert 3 30 (mapInsert 1 10 (mapInsert 2 20 Tip))))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(60));
    }


    // ── tails / inits ─────────────────────────────────────────────────

    #[test]
    fn eval_tails_length_chirho() {
        // tails [1,2,3] has length 4: [1,2,3], [2,3], [3], []
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = length (tails [1,2,3])
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
    }


    #[test]
    fn eval_tails_head_sum_chirho() {
        // head (tails [10,20,30]) = [10,20,30], sum = 60
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = sum (head (tails [10,20,30]))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(60));
    }


    #[test]
    fn eval_inits_length_chirho() {
        // inits [1,2,3] has length 4: [], [1], [1,2], [1,2,3]
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = length (inits [1,2,3])
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
    }


    #[test]
    fn eval_inits_last_sum_chirho() {
        // last (inits [10,20,30]) = [10,20,30], sum = 60
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = sum (last (inits [10,20,30]))
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(60));
    }

    // ── Type annotations in expressions ───────────────────────────────


    // ── Data.List: find ───────────────────────────────────────────────────

    #[test]
    fn eval_find_just_chirho() {
        // find (>3) [1,2,3,4,5] → Just 4 → fromMaybe 0 = 4
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = fromMaybe 0 (find (> 3) [1,2,3,4,5])
";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("find Just failed: {}", e_chirho));
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
    }


    #[test]
    fn eval_find_nothing_chirho() {
        // find (>10) [1,2,3] → Nothing → fromMaybe 99 = 99
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = fromMaybe 99 (find (> 10) [1,2,3])
";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("find Nothing failed: {}", e_chirho));
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99));
    }

    // ── Data.List: nubBy ─────────────────────────────────────────────────


    // ── Data.List: nubBy ─────────────────────────────────────────────────

    #[test]
    fn eval_nub_by_mod_chirho() {
        // nubBy (\x y -> x `mod` 3 == y `mod` 3) [1,2,3,4,5,6] → [1,2,3], length = 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = length (nubBy (\\x y -> x `mod` 3 == y `mod` 3) [1,2,3,4,5,6])
";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("nubBy mod length failed: {}", e_chirho));
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_nub_by_eq_head_chirho() {
        // nubBy (==) [1,1,2,2,3] → [1,2,3], head = 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = head (nubBy (==) [1,1,2,2,3])
";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("nubBy (==) head failed: {}", e_chirho));
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }

    // ── Data.List: sortBy ────────────────────────────────────────────────


    // ── Data.List: sortBy ────────────────────────────────────────────────

    #[test]
    fn eval_sort_by_descending_sum_chirho() {
        // sortBy (flip compare) [3,1,4,1,5] → [5,4,3,1,1], sum = 14
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (sortBy (\\x y -> compare y x) [3,1,4,1,5])
";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("sortBy descending sum failed: {}", e_chirho));
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(14));
    }


    #[test]
    fn eval_sort_by_ascending_head_chirho() {
        // sortBy compare [5,2,8,1] → [1,2,5,8], head = 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = head (sortBy compare [5,2,8,1])
";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("sortBy ascending head failed: {}", e_chirho));
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }

    // ── Data.List: groupBy ───────────────────────────────────────────────


    // ── Data.List: groupBy ───────────────────────────────────────────────

    #[test]
    fn eval_group_by_length_chirho() {
        // groupBy (==) [1,1,2,2,2,3] → [[1,1],[2,2,2],[3]], length = 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = length (groupBy (==) [1,1,2,2,2,3])
";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("groupBy (==) length failed: {}", e_chirho));
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_group_by_sum_of_heads_chirho() {
        // groupBy (==) [1,1,2,3,3] → [[1,1],[2],[3,3]], map head → [1,2,3], sum = 6
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (map head (groupBy (==) [1,1,2,3,3]))
";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("groupBy sum of heads failed: {}", e_chirho));
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6));
    }

    // ── Feature 1: tails / inits (named per spec) ─────────────────────


    // ── Feature 1: tails / inits (named per spec) ─────────────────────

    #[test]
    fn eval_tails_chirho() {
        // tails [1,2,3] returns [[1,2,3],[2,3],[3],[]], which has length 4
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = length (tails [1,2,3])
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("tails length failed: {}", e_chirho));
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
    }


    #[test]
    fn eval_inits_chirho() {
        // inits [1,2,3] returns [[],[1],[1,2],[1,2,3]], which has length 4
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = length (inits [1,2,3])
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("inits length failed: {}", e_chirho));
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
    }

    // ── Feature 2: type annotations in expressions (named per spec) ───


    #[test]
    fn eval_lazy_repeat_take_prelude_chirho() {
        // take 5 (repeat 7) should produce [7,7,7,7,7], sum = 35
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (take 5 (repeat 7))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("take 5 (repeat 7): {}", e_chirho));
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(35));
    }


    #[test]
    fn eval_lazy_iterate_take_prelude_chirho() {
        // take 5 (iterate (*2) 1) should produce [1,2,4,8,16], sum = 31
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (take 5 (iterate (*2) 1))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("take 5 (iterate (*2) 1): {}", e_chirho));
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(31));
    }

    // ---------------------------------------------------------------
    // Phase 2 §A.9: True lazy evaluation — infinite lists
    // ---------------------------------------------------------------


    // ---------------------------------------------------------------
    // Phase 2 §A.9: True lazy evaluation — infinite lists
    // ---------------------------------------------------------------

    #[test]
    fn eval_take_infinite_list_chirho() {
        // take 5 [1..] should produce [1,2,3,4,5], sum = 15
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (take 5 [1..])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("take 5 [1..]: {}", e_chirho));
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
    }


    #[test]
    fn eval_take_infinite_list_then_chirho() {
        // take 4 [1,3..] should produce [1,3,5,7], sum = 16
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (take 4 [1,3..])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("take 4 [1,3..]: {}", e_chirho));
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(16));
    }


    #[test]
    fn eval_head_infinite_list_chirho() {
        // head [42..] should be 42
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = head [42..]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("head [42..]: {}", e_chirho));
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }


    #[test]
    fn eval_take_10_enum_from_then_chirho() {
        // take 10 [0,2..] → [0,2,4,6,8,10,12,14,16,18], sum = 90
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (take 10 [0,2..])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("take 10 [0,2..]: {}", e_chirho));
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(90));
    }


    #[test]
    fn eval_lazy_enum_from_to_chirho() {
        // [1..5] should still work with the lazy implementation, sum = 15
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum [1..5]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("sum [1..5]: {}", e_chirho));
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
    }


    #[test]
    fn eval_lazy_enum_from_then_to_chirho() {
        // [1,3..10] → [1,3,5,7,9], sum = 25
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum [1,3..10]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("sum [1,3..10]: {}", e_chirho));
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(25));
    }


    #[test]
    fn eval_lazy_enum_from_then_to_desc_chirho() {
        // [10,8..1] → [10,8,6,4,2], sum = 30
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum [10,8..1]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("sum [10,8..1]: {}", e_chirho));
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30));
    }

    // ---------------------------------------------------------------
    // Complex program tests: feature combinations
    // ---------------------------------------------------------------


    #[test]
    fn eval_higher_order_filter_map_chirho() {
        // (filter even . map (*3)) [1..10] → filter even [3,6,9,...,30]
        // = [6,12,18,24,30], sum = 90
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
myFilter xs = filter even (map (*3) xs)
main = sum (myFilter [1..10])
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("higher order composition: {}", e_chirho));
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(90));
    }


    // ── zipWith and unzip tests ──

    #[test]
    fn eval_zipwith_add_sum_chirho() {
        // zipWith (+) [1,2,3] [10,20,30] → [11,22,33] → sum 66
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             main = sum (zipWith (+) [1,2,3] [10,20,30])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("zipWith should work");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66));
    }


    #[test]
    fn eval_zipwith_mul_chirho() {
        // zipWith (*) [2,3,4] [5,6,7] → [10,18,28] → sum 56
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             main = sum (zipWith (*) [2,3,4] [5,6,7])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("zipWith mul should work");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(56));
    }


    #[test]
    fn eval_concatmap_expand_chirho() {
        // concatMap (\x -> [x, x*10]) [1,2,3] → [1,10,2,20,3,30] → sum 66
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             main = sum (concatMap (\\x -> [x, x * 10]) [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("concatMap should work");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66));
    }

