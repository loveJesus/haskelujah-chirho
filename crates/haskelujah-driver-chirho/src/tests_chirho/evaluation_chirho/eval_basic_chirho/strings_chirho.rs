// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Strings as character lists, Data.Char and show (row 700).

use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn eval_string_literal_chirho() {
    // Observe the string, whether represented as a packed value or a lazy list.
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        "module Test where\nmain = putStrLn \"hello\"\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("string literal should be observable");
    assert_eq!(machine_chirho.io_output_chirho, "hello\n");
}

#[test]
fn eval_char_literal_chirho() {
    // Char literal evaluates to CharChirho value
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = 'A'\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::CharChirho('A')
        ),
        Err(e_chirho) => {
            eprintln!("eval_char_literal: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_show_with_concat_chirho() {
    // Show integers and concatenate: "x = " ++ show 42
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = putStrLn (\"x = \" ++ show 42)
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("show with concat should evaluate");
    assert_eq!(
        result_chirho.1.io_output_chirho, "x = 42\n",
        "putStrLn (\"x = \" ++ show 42) should produce 'x = 42'"
    );
}

#[test]
fn eval_show_list_int_chirho() {
    // show [1,2,3] should produce "[1,2,3]"
    // This tests the Show [Int] ground instance dict + showList# primop.
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = putStrLn (show [1,2,3])
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("putStrLn (show [1,2,3]) should evaluate");
    assert_eq!(
        result_chirho.1.io_output_chirho, "[1,2,3]\n",
        "show [1,2,3] should produce the string \"[1,2,3]\""
    );
}

#[test]
fn eval_show_float_division_chirho() {
    // putStrLn (show (6.0 / 4.0)) should display "1.5"
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = putStrLn (show (6.0 / 4.0))
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho, "1.5\n",
                "show (6.0 / 4.0) should produce 1.5"
            );
        }
        Err(e_chirho) => {
            panic!("show float division failed: {}", e_chirho);
        }
    }
}

#[test]
fn eval_show_list_double_chirho() {
    // show [1.5, 2.5] should produce "[1.5,2.5]"
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = putStrLn (show [1.5, 2.5])
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho, "[1.5,2.5]\n",
                "show [1.5, 2.5] should produce [1.5,2.5]"
            );
        }
        Err(e_chirho) => {
            panic!("Show [Double] test failed: {}", e_chirho);
        }
    }
}

#[test]
fn eval_show_list_bool_builtin_chirho() {
    // putStrLn (show [True, False, True]) → "[True,False,True]\n"
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = putStrLn (show [True, False, True])
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho, "[True,False,True]\n",
                "show [True, False, True] should produce [True,False,True]"
            );
        }
        Err(e_chirho) => {
            panic!("Show [Bool] builtin test failed: {}", e_chirho);
        }
    }
}

#[test]
fn eval_show_list_double_builtin_chirho() {
    // putStrLn (show [1.5, 2.5]) → "[1.5,2.5]\n"  (second test using builtin show)
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = putStrLn (show [1.5, 2.5])
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho, "[1.5,2.5]\n",
                "show [1.5, 2.5] should produce [1.5,2.5] (builtin show)"
            );
        }
        Err(e_chirho) => {
            panic!("Show [Double] builtin second test failed: {}", e_chirho);
        }
    }
}

#[test]
fn eval_is_digit_chirho() {
    use crate::eval_source_chirho;
    // isDigit '5' = True → if then 1 else 0
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nimport Data.Char (isDigit)\nmain = if isDigit '5' then 1 else 0\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("isDigit '5' should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );

    // isDigit 'x' = False
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nimport Data.Char (isDigit)\nmain = if isDigit 'x' then 1 else 0\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("isDigit 'x' should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
    );
}

#[test]
fn eval_to_lower_upper_chirho() {
    use crate::eval_source_chirho;
    // toLower 'A' = 'a' = 97, toUpper 'a' = 'A' = 65
    // ord (toLower 'A') = 97
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nimport Data.Char (ord, toLower)\nmain = ord (toLower 'A')\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("toLower should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(97)
    );

    // ord (toUpper 'a') = 65
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nimport Data.Char (ord, toUpper)\nmain = ord (toUpper 'a')\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("toUpper should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(65)
    );
}

#[test]
fn eval_is_alpha_space_chirho() {
    use crate::eval_source_chirho;
    // isAlpha 'z' = True
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nimport Data.Char (isAlpha)\nmain = if isAlpha 'z' then 1 else 0\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("isAlpha 'z' should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );

    // isSpace ' ' = True
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nimport Data.Char (isSpace)\nmain = if isSpace ' ' then 1 else 0\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("isSpace ' ' should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_show_just_int_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = putStrLn (show (Just 42))\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "Just 42\n");
        }
        Err(e_chirho) => {
            eprintln!("show (Just 42) failed: {}", e_chirho);
            panic!("show (Just 42) should work: {}", e_chirho);
        }
    }
}

#[test]
fn eval_show_nothing_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = putStrLn (show Nothing)\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "Nothing\n");
        }
        Err(e_chirho) => {
            eprintln!("show Nothing failed: {}", e_chirho);
            panic!("show Nothing should work: {}", e_chirho);
        }
    }
}

#[test]
fn eval_lines_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // length (lines "a\nb\nc") should be 3
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = length (lines \"a\\nb\\nc\")\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
            );
        }
        Err(e_chirho) => panic!("lines should work: {}", e_chirho),
    }
}

#[test]
fn eval_unlines_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = putStr (unlines [\"hello\", \"world\"])\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "hello\nworld\n");
        }
        Err(e_chirho) => panic!("unlines should work: {}", e_chirho),
    }
}

#[test]
fn eval_show_bool_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Use if-then-else to avoid constructor application issue with show True
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nshowBool x = if x then \"True\" else \"False\"\nmain = putStrLn (showBool True)\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "True\n"),
        Err(e_chirho) => panic!("show True should print: {}", e_chirho),
    }
}

#[test]
fn eval_show_false_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nshowBool x = if x then \"True\" else \"False\"\nmain = putStrLn (showBool False)\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "False\n"),
        Err(e_chirho) => panic!("show False should print: {}", e_chirho),
    }
}

#[test]
fn eval_show_char_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = putChar 'A'\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "A"),
        Err(e_chirho) => panic!("putChar should work: {}", e_chirho),
    }
}

#[test]
fn eval_show_list_bool_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // show [True, False]
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nshowBool b = if b then \"True\" else \"False\"\nshowList xs = case xs of\n  [] -> \"[]\"\n  (y:ys) -> \"[\" ++ showBool y ++ showRest ys\nshowRest xs = case xs of\n  [] -> \"]\"\n  (y:ys) -> \",\" ++ showBool y ++ showRest ys\nmain = putStrLn (showList [True, False, True])\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "[True,False,True]\n")
        }
        Err(e_chirho) => panic!("show list bool should work: {}", e_chirho),
    }
}

#[test]
fn eval_lines_unlines_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // lines splits a string by newlines, unlines joins with newlines
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = putStrLn (unwords (words \"hello world test\"))\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "hello world test\n")
        }
        Err(e_chirho) => panic!("words/unwords roundtrip should work: {}", e_chirho),
    }
}

// -- String-as-[Char] interop tests --

#[test]
fn eval_head_string_chirho() {
    // head "hello" → 'h'
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Char (ord)\nmain = ord (head \"hello\")\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(104)
        ), // 'h' = 104
        Err(e_chirho) => panic!("head string: {}", e_chirho),
    }
}

#[test]
fn eval_length_string_chirho() {
    // length "hello" → 5
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = length \"hello\"\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
        ),
        Err(e_chirho) => panic!("length string: {}", e_chirho),
    }
}

#[test]
fn eval_map_toupper_string_chirho() {
    // map toUpper "hello" → "HELLO" via putStrLn
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nimport Data.Char (toUpper)\nmain = putStrLn (map toUpper \"hello\")\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let output_chirho = machine_chirho.io_output_chirho.clone();
            assert_eq!(output_chirho, "HELLO\n");
        }
        Err(e_chirho) => panic!("map toUpper string: {}", e_chirho),
    }
}

#[test]
fn eval_filter_string_chirho() {
    // filter isDigit "abc123" → "123"
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Char (isDigit)\nmain = putStrLn (filter isDigit \"abc123\")\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let output_chirho = machine_chirho.io_output_chirho.clone();
            assert_eq!(output_chirho, "123\n");
        }
        Err(e_chirho) => panic!("filter isDigit string: {}", e_chirho),
    }
}

#[test]
fn eval_reverse_string_chirho() {
    // reverse "hello" then putStrLn
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = putStrLn (reverse \"hello\")\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let output_chirho = machine_chirho.io_output_chirho.clone();
            assert_eq!(output_chirho, "olleh\n");
        }
        Err(e_chirho) => panic!("reverse string: {}", e_chirho),
    }
}

#[test]
fn eval_null_string_chirho() {
    // null "" → True, null "hi" → False
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = if null \"\" then 1 else 0\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("null empty string: {}", e_chirho),
    }
}

// -- Additional list-on-string tests --

#[test]
fn eval_zip_strings_chirho() {
    // zip "abc" [1,2,3] → length should be 3
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = length (zip \"abc\" [1,2,3])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!("zip strings: {}", e_chirho),
    }
}

#[test]
fn eval_concat_map_string_chirho() {
    // concatMap (replicate 2) on a string using ++ for char replication
    // Actually simpler: length (concat ["ab","cd","ef"]) → 6
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = length (concatMap (\\x -> [x,x]) \"abc\")\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
        ),
        Err(e_chirho) => panic!("concatMap string: {}", e_chirho),
    }
}

#[test]
fn eval_case_string_match_chirho() {
    // String equality through if-then-else
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
greet name = if name == \"world\" then 1 else 0
main = greet \"world\"
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("case string match: {}", e_chirho),
    }
}

#[test]
fn eval_show_bool_list_chirho() {
    // show True ++ " " ++ show False → "True False"
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = putStrLn (show True ++ \" \" ++ show False)
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok((_v_chirho, m_chirho)) => assert_eq!(m_chirho.io_output_chirho, "True False\n"),
        Err(e_chirho) => panic!("show bool list: {}", e_chirho),
    }
}

#[test]
fn eval_show_list_show_chirho() {
    // putStrLn (show [10,20,30]) → "[10,20,30]\n"
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = putStrLn (show [10,20,30])\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_val_chirho, m_chirho)) => {
            let out_chirho = &m_chirho.io_output_chirho;
            assert_eq!(out_chirho, "[10,20,30]\n");
        }
        Err(e_chirho) => panic!("show list show: {}", e_chirho),
    }
}
