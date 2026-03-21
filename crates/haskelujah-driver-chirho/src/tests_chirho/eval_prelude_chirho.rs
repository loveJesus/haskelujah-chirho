// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// Prelude functions, numeric conversions, string ops, curry/uncurry, Maybe/Either tests

#[allow(unused_imports)]
use crate::{
    check_source_file_chirho, compile_modules_chirho, compile_modules_incremental_chirho,
    compile_source_chirho, discover_modules_chirho, eval_modules_chirho, eval_source_chirho,
    eval_source_with_input_chirho, eval_source_with_machine_chirho,
    eval_source_with_step_limit_chirho, render_summary_chirho,
};
#[allow(unused_imports)]
use haskelujah_runtime_chirho::{ExecutionModeChirho, ValueChirho};
#[allow(unused_imports)]
use haskelujah_span_chirho::SourceMapChirho;
#[allow(unused_imports)]
use haskelujah_syntax_chirho::SourceFileChirho;

#[test]
fn eval_dollar_operator_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // f $ x = f x  — test the $ operator
    // Define id locally since it's not a runtime builtin yet
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             id x = x\n\
             main = id $ 42\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => {
            eprintln!("eval_dollar_operator: {}", e_chirho);
            assert!(
                result_chirho.is_ok(),
                "$ operator should evaluate: {}",
                e_chirho
            );
        }
    }
}

#[test]
fn eval_mutual_recursion_even_odd_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // mutual recursion: isEven/isOdd
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             isEven n = if n == 0 then 1 else isOdd (n - 1)\n\
             isOdd n = if n == 0 then 0 else isEven (n - 1)\n\
             main = isEven 4\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
            );
        }
        Err(e_chirho) => {
            panic!("mutual recursion even/odd should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_mutual_recursion_odd_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // mutual recursion: isOdd 3 should be 1
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             isEven n = if n == 0 then 1 else isOdd (n - 1)\n\
             isOdd n = if n == 0 then 0 else isEven (n - 1)\n\
             main = isOdd 3\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
            );
        }
        Err(e_chirho) => {
            panic!("mutual recursion isOdd should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_not_chirho() {
    // `not` should negate boolean values.
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if not (3 == 4) then 1 else 0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("not should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1),
        "not (3 == 4) should be True, giving 1"
    );
}

#[test]
fn eval_double_arithmetic_chirho() {
    // Double addition through Num instance
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = 1.5 + 2.5
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("double arithmetic should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(4.0),
        "1.5 + 2.5 should be 4.0"
    );
}

#[test]
fn eval_show_double_chirho() {
    // putStrLn (show 3.14) should display "3.14"
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = putStrLn (show 3.14)
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("show double should evaluate");
    assert_eq!(
        result_chirho.1.io_output_chirho, "3.14\n",
        "show 3.14 should produce '3.14'"
    );
}

#[test]
fn eval_tuple_fst_chirho() {
    // Tuple construction and case dispatch to extract first element
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
fst p = case p of
  (a, b) -> a
main = fst (10, 20)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("tuple fst should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10),
        "fst (10, 20) should be 10"
    );
}

#[test]
fn eval_tuple_snd_chirho() {
    // Tuple construction and case dispatch to extract second element
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
snd p = case p of
  (a, b) -> b
main = snd (10, 20)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("tuple snd should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(20),
        "snd (10, 20) should be 20"
    );
}

#[test]
fn eval_show_arith_seq_chirho() {
    // show [1..3] should produce "[1,2,3]"
    // Tests Show [Int] + arithmetic sequence + showList# end-to-end.
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = putStrLn (show [1..3])
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("putStrLn (show [1..3]) should evaluate");
    assert_eq!(
        result_chirho.1.io_output_chirho, "[1,2,3]\n",
        "show [1..3] should produce the string \"[1,2,3]\""
    );
}

#[test]
fn eval_read_int_chirho() {
    // read "42" :: Int should evaluate to 42
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = read \"42\"
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("read Int should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42),
        "read \"42\" should be 42"
    );
}

#[test]
fn eval_read_int_arithmetic_chirho() {
    // (read "10") + 5 should evaluate to 15
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = (read \"10\") + 5
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("read Int + 5 should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(15),
        "read \"10\" + 5 should be 15"
    );
}

#[test]
fn eval_read_int_negation_chirho() {
    // read "-7" should parse to -7
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = read \"-7\"
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("read \"-7\" should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(-7),
        "read \"-7\" should be -7"
    );
}

// ---------------------------------------------------------------
// Backtick infix syntax tests
// ---------------------------------------------------------------

// ---------------------------------------------------------------
// Prelude numeric/predicate function tests
// ---------------------------------------------------------------

#[test]
fn eval_even_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // even 4 = True (represented as Int 1)
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = if even 4 then 1 else 0\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
            );
        }
        Err(e_chirho) => {
            panic!("even should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_odd_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = if odd 7 then 1 else 0\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
            );
        }
        Err(e_chirho) => {
            panic!("odd should evaluate: {}", e_chirho);
        }
    }
}

// ---------------------------------------------------------------
// Tuple operation tests (fst, snd)
// ---------------------------------------------------------------

#[test]
fn eval_fst_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = fst (42, 99)\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => {
            panic!("fst should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_snd_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = snd (42, 99)\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
            );
        }
        Err(e_chirho) => {
            panic!("snd should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_fst_snd_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = fst (1, 2) + snd (3, 4)\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            // fst (1, 2) = 1, snd (3, 4) = 4, 1 + 4 = 5
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
            );
        }
        Err(e_chirho) => {
            panic!("fst+snd should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_curry_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // curry takes a function on pairs and returns a curried version
    // curry fst 10 20 should give 10  (fst (10, 20) = 10)
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = curry fst 10 20\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
            );
        }
        Err(e_chirho) => {
            panic!("curry fst should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_uncurry_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // uncurry add (3, 4) should give 7
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             add x y = x + y\n\
             main = uncurry add (3, 4)\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
            );
        }
        Err(e_chirho) => {
            panic!("uncurry (+) should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_curry_snd_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // curry snd 10 20 should give 20  (snd (10, 20) = 20)
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = curry snd 10 20\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(20)
            );
        }
        Err(e_chirho) => {
            panic!("curry snd should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_take_drop_concat_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\n\
             main = putStrLn (take 3 \"abcdef\" ++ drop 3 \"abcdef\")\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("take+drop concat should evaluate");
    assert_eq!(result_chirho.1.io_output_chirho, "abcdef\n");
}

#[test]
fn eval_words_unwords_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // unwords (words "hello world") should give "hello world"
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\n\
             main = putStrLn (unwords (words \"hello world\"))\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("words/unwords should evaluate");
    assert_eq!(result_chirho.1.io_output_chirho, "hello world\n");
}

#[test]
fn eval_intercalate_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // intercalate ", " (words "one two three") should give "one, two, three"
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\n\
             main = putStrLn (intercalate \", \" (words \"one two three\"))\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("intercalate should evaluate");
    assert_eq!(result_chirho.1.io_output_chirho, "one, two, three\n");
}

#[test]
fn eval_floor_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = floor 3.7\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
            );
        }
        Err(e_chirho) => panic!("floor should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_ceiling_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = ceiling 3.2\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(4)
            );
        }
        Err(e_chirho) => panic!("ceiling should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_round_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = round 3.5\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(4)
            );
        }
        Err(e_chirho) => panic!("round should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_truncate_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = truncate 3.9\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
            );
        }
        Err(e_chirho) => panic!("truncate should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_uncurry_operator_section_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = uncurry (+) (3, 4)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
            );
        }
        Err(e_chirho) => panic!("uncurry (+) should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_take_string_still_works_chirho() {
    // take still works on strings: take 3 "hello" ++ "!" → "hel!"
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = putStrLn (take 3 \"hello\")
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("take on string should still work");
    assert_eq!(
        result_chirho.1.io_output_chirho, "hel\n",
        "take 3 \"hello\" = \"hel\""
    );
}

// ── Priority 67: Ord/compare/Ordering/min/max ─────────────────────

// ── Priority 67: Ord/compare/Ordering/min/max ─────────────────────
#[test]
fn eval_compare_int_lt_chirho() {
    // compare 3 5 → LT → putStrLn "LT"
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
showOrd x y = case compare x y of
  LT -> \"LT\"
  EQ -> \"EQ\"
  GT -> \"GT\"
main = putStrLn (showOrd 3 5)
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("compare 3 5 should evaluate");
    assert_eq!(result_chirho.1.io_output_chirho, "LT\n", "compare 3 5 = LT");
}

#[test]
fn eval_compare_int_eq_chirho() {
    // compare 5 5 → EQ
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
showOrd x y = case compare x y of
  LT -> \"LT\"
  EQ -> \"EQ\"
  GT -> \"GT\"
main = putStrLn (showOrd 5 5)
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("compare 5 5 should evaluate");
    assert_eq!(result_chirho.1.io_output_chirho, "EQ\n", "compare 5 5 = EQ");
}

#[test]
fn eval_compare_int_gt_chirho() {
    // compare 10 3 → GT
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
showOrd x y = case compare x y of
  LT -> \"LT\"
  EQ -> \"EQ\"
  GT -> \"GT\"
main = putStrLn (showOrd 10 3)
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("compare 10 3 should evaluate");
    assert_eq!(
        result_chirho.1.io_output_chirho, "GT\n",
        "compare 10 3 = GT"
    );
}

#[test]
fn eval_min_max_same_chirho() {
    // min x x = max x x = x
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = min 7 7 + max 7 7
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("min+max same should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(14),
        "min 7 7 + max 7 7 = 14"
    );
}

// ── Priority 68: elem, notElem, minimum, maximum, sort ────────────

// ── Priority 68: elem, notElem, minimum, maximum, sort ────────────
#[test]
fn eval_elem_found_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = case elem 3 [1,2,3,4,5] of
  True  -> 1
  False -> 0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("elem found should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_elem_not_found_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = case elem 9 [1,2,3] of
  True  -> 1
  False -> 0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("elem not found should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
    );
}

#[test]
fn eval_sort_head_chirho() {
    // head (sort [5,2,8,1]) = 1
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = head (sort [5,2,8,1])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("head sort should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_sort_last_chirho() {
    // last (sort [5,2,8,1]) = 8
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = last (sort [5,2,8,1])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("last sort should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(8)
    );
}

// ── Priority 69: if-then-else, abs, signum, even, odd, replicate ──

#[test]
fn eval_abs_negative_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = abs (negate 7)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("abs negative should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
    );
}

#[test]
fn eval_even_odd_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = length (filter even [1,2,3,4,5,6])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("even filter should evaluate");
    // even values: 2,4,6 → length = 3
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );
}

#[test]
fn eval_ord_chr_chirho() {
    use crate::eval_source_chirho;
    // ord 'A' = 65
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = ord 'A'\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("ord should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(65)
    );

    // chr (ord 'A' + 1) = 'B' = 66
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = ord 'A' + 1\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("ord arithmetic should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(66)
    );
}

#[test]
fn eval_sin_cos_chirho() {
    use crate::eval_source_chirho;
    // sin 0.0 = 0.0
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = sin 0.0\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("sin 0.0 should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(0.0)
    );

    // cos 0.0 = 1.0
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = cos 0.0\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("cos 0.0 should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(1.0)
    );
}

#[test]
fn eval_exp_log_chirho() {
    use crate::eval_source_chirho;
    // exp 0.0 = 1.0
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = exp 0.0\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("exp 0.0 should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(1.0)
    );

    // log 1.0 = 0.0
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = log 1.0\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("log 1.0 should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(0.0)
    );
}

#[test]
fn eval_sqrt_chirho() {
    use crate::eval_source_chirho;
    // sqrt 4.0 = 2.0
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = sqrt 4.0\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("sqrt 4.0 should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(2.0)
    );

    // sqrt 9.0 = 3.0
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = sqrt 9.0\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("sqrt 9.0 should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(3.0)
    );
}

#[test]
fn eval_asin_acos_chirho() {
    use crate::eval_source_chirho;
    // asin 0.0 = 0.0
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = asin 0.0\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("asin 0.0 should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(0.0)
    );

    // acos 1.0 = 0.0
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = acos 1.0\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("acos 1.0 should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(0.0)
    );
}

#[test]
fn eval_fmap_maybe_just_chirho() {
    use crate::eval_source_chirho;
    // fmap (+1) (Just 5) = Just 6 — extract with case
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
myFmap f mx = case mx of
  Nothing -> Nothing
  Just x  -> Just (f x)
main = case myFmap (\\y -> y + 1) (Just 5) of
  Just z -> z
  Nothing -> 0
";
    let r1_chirho = eval_source_chirho(src_chirho, &mut sm1_chirho, "TestChirho.hs", None)
        .expect("user-defined fmap Just should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
    );
}

#[test]
fn eval_fmap_maybe_nothing_chirho() {
    use crate::eval_source_chirho;
    // fmap (+1) Nothing = Nothing
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
myFmap f mx = case mx of
  Nothing -> Nothing
  Just x  -> Just (f x)
main = case myFmap (\\y -> y + 1) Nothing of
  Just _ -> 1
  Nothing -> 0
";
    let r1_chirho = eval_source_chirho(src_chirho, &mut sm1_chirho, "TestChirho.hs", None)
        .expect("user-defined fmap Nothing should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
    );
}

#[test]
fn eval_interact_chirho() {
    use crate::eval_source_with_input_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Read a line, prepend "Hello, ", print it
    let result_chirho = eval_source_with_input_chirho(
        "module Test where\nmain = do\n  name <- getLine\n  putStrLn (\"Hello, \" ++ name)\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
        &["World"],
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "Hello, World\n")
        }
        Err(e_chirho) => panic!("interact pattern should work: {}", e_chirho),
    }
}

#[test]
fn eval_assoc_list_lookup_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Simple association list lookup by key
    let result_chirho = eval_source_chirho(
        "module Test where\nlookupA k xs = case xs of\n  [] -> 0\n  ((k2,v):rest) -> if k == k2 then v else lookupA k rest\nmain = lookupA 2 [(1,10),(2,20),(3,30)]\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(20)
        ),
        Err(e_chirho) => panic!("assoc list lookup should work: {}", e_chirho),
    }
}

#[test]
fn eval_map_maybe_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // mapMaybe applies function and collects Just results
    let result_chirho = eval_source_chirho(
        "module Test where\nmapMaybe f xs = case xs of\n  [] -> []\n  (y:ys) -> case f y of\n    Nothing -> mapMaybe f ys\n    Just v -> v : mapMaybe f ys\nsafeDiv x = if x == 0 then Nothing else Just (100 `div` x)\nmain = sum (mapMaybe safeDiv [5, 0, 10, 0, 2])\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(80)
        ),
        Err(e_chirho) => panic!("mapMaybe should work: {}", e_chirho),
    }
}

#[test]
fn eval_map_insert_lookup_chirho() {
    // Insert key 5 with value 42, lookup key 5 should find Just 42
    // Use case on mapLookup result to extract Int
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 5 42 mapEmpty) 5\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("Map insert+lookup should work: {}", e_chirho),
    }
}

#[test]
fn eval_map_lookup_missing_chirho() {
    // Lookup a key that doesn't exist → Nothing → 0
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 5 42 mapEmpty) 10\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
        ),
        Err(e_chirho) => panic!("Map lookup missing should return Nothing: {}", e_chirho),
    }
}

#[test]
fn eval_map_insert_lookup_other_key_chirho() {
    // Insert two keys, lookup the other key (3→99)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 3 99 (mapInsert 5 42 mapEmpty)) 3\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
        ),
        Err(e_chirho) => panic!("Map lookup other key should return 99: {}", e_chirho),
    }
}

// ── String comparison / Ord [Char] tests ─────────────────────────

#[test]
fn eval_compare_string_lt_chirho() {
    // compare "abc" "def" should yield LT → pattern match to 1
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = case compare "abc" "def" of
         LT -> 1
         EQ -> 2
         GT -> 3
"#;
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("compare string LT: {}", e_chirho),
    }
}

#[test]
fn eval_compare_string_eq_chirho() {
    // compare "hello" "hello" should yield EQ → 2
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = case compare "hello" "hello" of
         LT -> 1
         EQ -> 2
         GT -> 3
"#;
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("compare string EQ: {}", e_chirho),
    }
}

#[test]
fn eval_compare_string_gt_chirho() {
    // compare "xyz" "abc" should yield GT → 3
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = case compare "xyz" "abc" of
         LT -> 1
         EQ -> 2
         GT -> 3
"#;
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!("compare string GT: {}", e_chirho),
    }
}

// ── interact with function application tests ─────────────────────

// ── interact with function application tests ─────────────────────

#[test]
fn eval_interact_identity_chirho() {
    // interact id should echo input to output
    use crate::eval_source_with_input_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = interact id\n";
    let result_chirho = eval_source_with_input_chirho(
        src_chirho,
        &mut sm_chirho,
        "TestChirho.hs",
        None,
        &["hello"],
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "hello");
        }
        Err(e_chirho) => panic!("interact id: {}", e_chirho),
    }
}

#[test]
fn eval_interact_map_toupper_chirho() {
    // interact (map toUpper) should uppercase all chars
    use crate::eval_source_with_input_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = interact (map toUpper)\n";
    let result_chirho = eval_source_with_input_chirho(
        src_chirho,
        &mut sm_chirho,
        "TestChirho.hs",
        None,
        &["hello"],
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "HELLO");
        }
        Err(e_chirho) => panic!("interact map toUpper: {}", e_chirho),
    }
}

#[test]
fn eval_interact_with_reverse_chirho() {
    // interact reverse should reverse the input
    use crate::eval_source_with_input_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = interact reverse\n";
    let result_chirho = eval_source_with_input_chirho(
        src_chirho,
        &mut sm_chirho,
        "TestChirho.hs",
        None,
        &["abcde"],
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "edcba");
        }
        Err(e_chirho) => panic!("interact reverse: {}", e_chirho),
    }
}

// ── mapM_ / forM_ tests ─────────────────────────

// ── mapM_ / forM_ tests ─────────────────────────

#[test]
fn eval_mapm_underscore_chirho() {
    // mapM_ putStrLn ["a","b","c"] should output "a\nb\nc\n"
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = mapM_ putStrLn ["a","b","c"]
"#;
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "a\nb\nc\n");
        }
        Err(e_chirho) => panic!("mapM_: {}", e_chirho),
    }
}

// ── lookup / additional Prelude tests ─────────────────────

// ── lookup / additional Prelude tests ─────────────────────

#[test]
fn eval_lookup_found_chirho() {
    // lookup 2 [(1,10),(2,20),(3,30)] → Just 20 → 20
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = case lookup 2 [(1,10),(2,20),(3,30)] of
         Just x -> x
         Nothing -> 0
"#;
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(20)
        ),
        Err(e_chirho) => panic!("lookup found: {}", e_chirho),
    }
}

#[test]
fn eval_lookup_not_found_chirho() {
    // lookup 5 [(1,10),(2,20)] → Nothing → 0
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = case lookup 5 [(1,10),(2,20)] of
         Just x -> x
         Nothing -> 0
"#;
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
        ),
        Err(e_chirho) => panic!("lookup not found: {}", e_chirho),
    }
}

// ── Uncurry with operator section ────────────────────────────────

#[test]
fn eval_uncurry_section_chirho() {
    // uncurry (+) (3, 4) → 7
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = uncurry (+) (3, 4)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
        ),
        Err(e_chirho) => panic!("uncurry (+): {}", e_chirho),
    }
}

// ── Map with operator section ────────────────────────────────────

// ── Numeric escape sequences ────────────────────────────────────

#[test]
fn eval_numeric_escape_decimal_chirho() {
    // \65 = 'A', \66 = 'B', \67 = 'C'
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = putStrLn \"\\65\\66\\67\"\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok((_v_chirho, m_chirho)) => assert_eq!(m_chirho.io_output_chirho, "ABC\n"),
        Err(e_chirho) => panic!("numeric escape decimal: {}", e_chirho),
    }
}

#[test]
fn eval_numeric_escape_hex_chirho() {
    // \x48 = 'H', \x69 = 'i'
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = putStrLn \"\\x48\\x69\"\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok((_v_chirho, m_chirho)) => assert_eq!(m_chirho.io_output_chirho, "Hi\n"),
        Err(e_chirho) => panic!("numeric escape hex: {}", e_chirho),
    }
}

#[test]
fn eval_numeric_escape_octal_chirho() {
    // \o110 = 'H' (72 in octal), \o151 = 'i' (105 in octal)
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = putStrLn \"\\o110\\o151\"\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok((_v_chirho, m_chirho)) => assert_eq!(m_chirho.io_output_chirho, "Hi\n"),
        Err(e_chirho) => panic!("numeric escape octal: {}", e_chirho),
    }
}

// ── Data.Map extended operations ────────────────────────────────

// ── Additional list functions ───────────────────────────────────

#[test]
fn eval_nub_length_chirho() {
    // nub [1,2,1,3,2,4] → [1,2,3,4] → length 4
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = length (nub [1,2,1,3,2,4])
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(4)
        ),
        Err(e_chirho) => panic!("nub length: {}", e_chirho),
    }
}

#[test]
fn eval_map_lookup_str_not_found_chirho() {
    // mapFindWithDefaultStr 99 "missing" mapEmpty → 99
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = mapFindWithDefaultStr 99 \"missing\" mapEmpty
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
        ),
        Err(e_chirho) => panic!("mapFindWithDefaultStr not found: {}", e_chirho),
    }
}

#[test]
fn eval_product_list_chirho() {
    // product [1..5] → 120
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = product [1..5]\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(120)
        ),
        Err(e_chirho) => panic!("product list: {}", e_chirho),
    }
}

// ── Data.Maybe extras ──

#[test]
fn eval_maybe_to_list_just_chirho() {
    // maybeToList (Just 42) → [42] → head = 42
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = head (maybeToList (Just 42))\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("maybeToList Just: {}", e_chirho),
    }
}

#[test]
fn eval_maybe_to_list_nothing_chirho() {
    // maybeToList Nothing → [] → length = 0
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = length (maybeToList Nothing)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
        ),
        Err(e_chirho) => panic!("maybeToList Nothing: {}", e_chirho),
    }
}

#[test]
fn eval_list_to_maybe_chirho() {
    // listToMaybe [10,20,30] → Just 10 → fromMaybe 0 = 10
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = fromMaybe 0 (listToMaybe [10,20,30])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
        ),
        Err(e_chirho) => panic!("listToMaybe: {}", e_chirho),
    }
}

#[test]
fn eval_list_to_maybe_empty_chirho() {
    // listToMaybe [] → Nothing → fromMaybe 99 = 99
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = fromMaybe 99 (listToMaybe [])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
        ),
        Err(e_chirho) => panic!("listToMaybe empty: {}", e_chirho),
    }
}

#[test]
fn eval_cat_maybes_chirho() {
    // catMaybes [Just 1, Nothing, Just 3, Nothing, Just 5] → [1,3,5] → sum = 9
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nmain = sum (catMaybes [Just 1, Nothing, Just 3, Nothing, Just 5])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(9)
        ),
        Err(e_chirho) => panic!("catMaybes: {}", e_chirho),
    }
}

#[test]
fn eval_map_maybe_filter_chirho() {
    // mapMaybe (\x -> if x > 3 then Just (x * 10) else Nothing) [1,2,3,4,5] → [40,50] → sum = 90
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = sum (mapMaybe (\\x -> if x > 3 then Just (x * 10) else Nothing) [1,2,3,4,5])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(90)
        ),
        Err(e_chirho) => panic!("mapMaybe filter: {}", e_chirho),
    }
}

// ── Data.IORef ──

// ── flip ──

#[test]
fn eval_flip_const_chirho() {
    // flip const 1 2 → 2
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = flip const 1 2\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("flip const: {}", e_chirho),
    }
}

// ── Data.Either extras ──

// ── Data.Either extras ──

#[test]
fn eval_either_left_chirho() {
    // either (+10) (*2) (Left 5) → 15
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = either (+10) (*2) (Left 5)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
        ),
        Err(e_chirho) => panic!("either Left: {}", e_chirho),
    }
}

#[test]
fn eval_either_right_chirho() {
    // either (+10) (*2) (Right 5) → 10
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = either (+10) (*2) (Right 5)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
        ),
        Err(e_chirho) => panic!("either Right: {}", e_chirho),
    }
}

// ── More feature tests ──

#[test]
fn eval_char_operations_chirho() {
    // ord 'A' + ord 'a' = 65 + 97 = 162
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = ord 'A' + ord 'a'\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(162)
        ),
        Err(e_chirho) => panic!("char operations: {}", e_chirho),
    }
}

#[test]
fn eval_enum_succ_pred_chirho() {
    // succ 41 + pred 43 = 42 + 42 = 84
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = succ 41 + pred 43\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(84)
        ),
        Err(e_chirho) => panic!("enum succ pred: {}", e_chirho),
    }
}

#[test]
fn eval_import_data_list_chirho() {
    // import Data.List functions
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.List (sort)
main = head (sort [3, 1, 2])
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("import Data.List: {}", e_chirho),
    }
}

#[test]
fn eval_import_data_char_chirho() {
    // import Data.Char functions
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Char (ord)
main = ord 'A'
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(65)
        ),
        Err(e_chirho) => panic!("import Data.Char: {}", e_chirho),
    }
}

#[test]
fn eval_import_data_maybe_chirho() {
    // import Data.Maybe functions
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Maybe (fromMaybe)
main = fromMaybe 0 (Just 42)
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("import Data.Maybe: {}", e_chirho),
    }
}

#[test]
fn eval_import_qualified_data_list_chirho() {
    // import qualified Data.List as L
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.List as L
main = L.head (L.sort [3, 1, 2])
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("import qualified Data.List: {}", e_chirho),
    }
}

#[test]
fn eval_import_qualified_data_list_no_alias_chirho() {
    // import qualified Data.List (no alias) → Data.List.head
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.List
main = Data.List.head (Data.List.sort [3, 1, 2])
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("import qualified Data.List no alias: {}", e_chirho),
    }
}

#[test]
fn eval_import_hiding_chirho() {
    // import Data.Map hiding (mapDelete)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Map hiding (mapDelete)
main = mapSize (mapInsert 1 10 mapEmpty)
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("import hiding: {}", e_chirho),
    }
}

// ── mapM_ / forM_ IO sequencing ────────────────────────────────────

// ── 3-tuple pattern matching ────────────────────────────────────────

#[test]
fn eval_tuple3_fst_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nfst3 t = case t of (a,b,c) -> a\nmain = fst3 (10, 20, 30)\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
    );
}

#[test]
fn eval_3tuple_snd_chirho() {
    // 3-tuple second element extraction
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\n\
             snd3 (a, b, c) = b\n\
             main = snd3 (10, 20, 30)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("3-tuple snd3 should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(20)
    );
}
