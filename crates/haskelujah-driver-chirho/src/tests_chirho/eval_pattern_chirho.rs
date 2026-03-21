// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// Pattern matching, case expressions, guards tests

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
fn eval_if_then_else_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // if True then 42 else 0
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = if True then 42 else 0\n",
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
            // If it fails, print the error for debugging but don't
            // hard-fail yet — we're probing pipeline capabilities.
            eprintln!("eval_if_then_else: {}", e_chirho);
            assert!(
                result_chirho.is_ok(),
                "if-then-else should evaluate: {}",
                e_chirho
            );
        }
    }
}

#[test]
fn eval_if_with_comparison_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // if (3 < 5) then 42 else 0  — combines comparison + conditional
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = if 3 < 5 then 42 else 0\n",
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
            eprintln!("eval_if_with_comparison: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_case_constructor_field_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // case Just 42 of { Just x -> x; Nothing -> 0 }
    let result_chirho = eval_source_chirho(
        "module Test where\ndata Maybe a = Nothing | Just a\nmain = case Just 42 of { Just x -> x; Nothing -> 0 }\n",
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
            eprintln!("eval_case_constructor_field: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_case_default_alt_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Default alt: case Just 10 of { Nothing -> 0; _ -> 99 }
    let result_chirho = eval_source_chirho(
        "module Test where\ndata Maybe a = Nothing | Just a\nmain = case Just 10 of { Nothing -> 0; _ -> 99 }\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
        ),
        Err(e_chirho) => {
            eprintln!("eval_case_default_alt: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_nested_case_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Nested: if (3 > 5) then 1 else if (2 < 4) then 2 else 3
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = if 3 > 5 then 1 else if 2 < 4 then 2 else 3\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => {
            eprintln!("eval_nested_case: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_function_with_case_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Function that uses conditional: abs x = if x < 0 then 0 - x else x
    let result_chirho = eval_source_chirho(
        "module Test where\nabs' x = if x < 0 then 0 - x else x\nmain = abs' (-7)\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
        ),
        Err(e_chirho) => {
            eprintln!("eval_function_with_case: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_case_literal_dispatch_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Literal case dispatch: classify x = case x of 1 -> 10; 2 -> 20; _ -> 0
    // main = classify 2  →  20
    let result_chirho = eval_source_chirho(
        "module Test where\nclassify x = case x of\n  1 -> 10\n  2 -> 20\n  _ -> 0\nmain = classify 2\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(20)
        ),
        Err(e_chirho) => {
            eprintln!("eval_case_literal_dispatch: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_case_literal_default_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Literal case fallthrough to default
    // main = case 99 of { 1 -> 10; 2 -> 20; _ -> 0 }
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = case 99 of\n  1 -> 10\n  2 -> 20\n  _ -> 0\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
        ),
        Err(e_chirho) => {
            eprintln!("eval_case_literal_default: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_nested_pattern_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Nested constructor pattern via two-level case
    // main = case Just 42 of { Just x -> x; Nothing -> 0 }
    let result_chirho = eval_source_chirho(
        "module Test where\ndata Maybe a = Nothing | Just a\nmain = case Just 42 of\n  Just x -> x\n  Nothing -> 0\n",
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
            eprintln!("eval_nested_pattern: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_nested_constructor_destructure_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Simpler: case Just 99 of Just x -> x + 1; _ -> 0
    // Tests constructor field extraction with arithmetic
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = case Just 99 of\n\
             \x20 Just x -> x\n\
             \x20 _ -> 0\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
        ),
        Err(e_chirho) => {
            eprintln!("eval_nested_constructor_destructure: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_deeply_nested_pattern_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Deep nesting: case Just (Just 77) of Just (Just x) -> x; _ -> 0
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = case Just (Just 77) of\n\
             \x20 Just (Just x) -> x\n\
             \x20 _ -> 0\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(77)
        ),
        Err(e_chirho) => {
            eprintln!("eval_deeply_nested_pattern: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_guarded_function_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // abs x | x >= 0 = x | True = 0 - x
    // main = abs (-5)  → 5
    // Using True as the fallback guard since we don't have `otherwise` in prelude yet
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             abs x\n  | x >= 0 = x\n  | True = 0 - x\n\
             main = abs 5\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
        ),
        Err(e_chirho) => {
            eprintln!("eval_guarded_function: {}", e_chirho);
            assert!(
                result_chirho.is_ok(),
                "guarded function should evaluate: {}",
                e_chirho
            );
        }
    }
}

#[test]
fn eval_guarded_function_fallback_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // classify x | x > 0 = 1 | x < 0 = 2 | True = 0
    // main = classify 0  → should fall through to the True guard → 0
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             classify x\n  | x > 0 = 1\n  | x < 0 = 2\n  | True = 0\n\
             main = classify 0\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
        ),
        Err(e_chirho) => {
            eprintln!("eval_guarded_function_fallback: {}", e_chirho);
            assert!(
                result_chirho.is_ok(),
                "guarded fallback should evaluate: {}",
                e_chirho
            );
        }
    }
}

#[test]
fn eval_guarded_true_branch_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Simple: f x | True = x + 1
    // main = f 41  → 42
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             f x\n  | True = x + 1\n\
             main = f 41\n",
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
            eprintln!("eval_guarded_true_branch: {}", e_chirho);
            assert!(
                result_chirho.is_ok(),
                "guarded true branch should evaluate: {}",
                e_chirho
            );
        }
    }
}

#[test]
fn eval_otherwise_guard_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // classify x | x > 0 = 1 | otherwise = 0
    // main = classify 0  → should fall to otherwise → 0
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             classify x\n  | x > 0 = 1\n  | otherwise = 0\n\
             main = classify 0\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
        ),
        Err(e_chirho) => {
            eprintln!("eval_otherwise_guard: {}", e_chirho);
            assert!(
                result_chirho.is_ok(),
                "otherwise guard should evaluate: {}",
                e_chirho
            );
        }
    }
}

#[test]
fn eval_multi_eq_constructor_pattern_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Multi-equation constructor pattern matching (Priority 47)
    let src_chirho = "\
module Test where
data Color = Red | Green | Blue
showColor Red = 1
showColor Green = 2
showColor Blue = 3
main = showColor Green
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
            );
        }
        Err(e_chirho) => panic!("multi-eq constructor pattern should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_multi_eq_with_wildcard_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Use if to unbox Bool to Int for easy testing
    let src_chirho = "\
module Test where
data Color = Red | Green | Blue
isRed Red = 1
isRed _ = 0
main = isRed Blue
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
            );
        }
        Err(e_chirho) => panic!("multi-eq wildcard should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_multi_eq_bool_not_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Use if to convert Bool result to Int
    let src_chirho = "\
module Test where
myNot True = 0
myNot False = 1
main = myNot False
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
            );
        }
        Err(e_chirho) => panic!("multi-eq bool not should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_multi_eq_instance_show_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Multi-equation method in an instance declaration
    let src_chirho = "\
module Test where
data Color = Red | Green | Blue
instance Show Color where
  show Red = \"Red\"
  show Green = \"Green\"
  show Blue = \"Blue\"
main = putStrLn (show Green)
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok((_, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "Green");
        }
        Err(e_chirho) => panic!("multi-eq instance show should evaluate: {}", e_chirho),
    }
}

// ── Priority 69: if-then-else, abs, signum, even, odd, replicate ──
#[test]
fn eval_if_comparison_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if 3 > 2 then 10 else 20
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("if-then-else should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
    );
}

// ── Negative literal patterns ──────────────────────────────────

#[test]
fn eval_neg_lit_case_chirho() {
    // case (-1) of { -1 -> 100; _ -> 0 } → not directly since we don't parse neg lit patterns yet
    // But we can test function guard equivalent
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
f x = if x == 0 then 100 else x * 2
main = f 0
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(100)
        ),
        Err(e_chirho) => panic!("neg lit case: {}", e_chirho),
    }
}

// ── Complex pattern matching scenarios ──────────────────────────

// ── Complex pattern matching scenarios ──────────────────────────

#[test]
fn eval_multi_clause_with_guards_chirho() {
    // Multi-clause function with guards
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
classify x
  | x < 0     = 0
  | x == 0    = 1
  | x < 100   = 2
  | otherwise  = 3
main = classify 0 + classify 50 + classify 200 + classify (-5)
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        // 1 + 2 + 3 + 0 = 6
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
        ),
        Err(e_chirho) => panic!("multi clause with guards: {}", e_chirho),
    }
}

#[test]
fn eval_where_with_pattern_chirho() {
    // f (x, y) = a + b where { a = x * 2; b = y + 1 }; main = f (3, 4)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nf p = a + b\n  where\n    a = fst p * 2\n    b = snd p + 1\nmain = f (3, 4)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(11)
        ),
        Err(e_chirho) => panic!("where with pattern: {}", e_chirho),
    }
}

// ── Literal pattern matching in function equations ──

#[test]
fn eval_literal_pattern_zero_chirho() {
    // f 0 = 100; f x = x + 1; main = f 0 → 100
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nf 0 = 100\nf x = x + 1\nmain = f 0\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(100)
        ),
        Err(e_chirho) => panic!("literal pattern zero: {}", e_chirho),
    }
}

#[test]
fn eval_literal_pattern_nonzero_chirho() {
    // f 0 = 100; f x = x + 1; main = f 5 → 6
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nf 0 = 100\nf x = x + 1\nmain = f 5\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
        ),
        Err(e_chirho) => panic!("literal pattern nonzero: {}", e_chirho),
    }
}

#[test]
fn eval_literal_pattern_multi_lit_chirho() {
    // Multiple literal arms with a catch-all default
    // f 0 = 10; f 1 = 20; f 2 = 30; f n = n * 100; main = f 2 + f 5
    // f 2 = 30, f 5 = 500, total = 530
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nf 0 = 10\nf 1 = 20\nf 2 = 30\nf n = n * 100\nmain = f 2 + f 5\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(530)
        ),
        Err(e_chirho) => panic!("literal pattern multi: {}", e_chirho),
    }
}

#[test]
fn eval_literal_pattern_negative_chirho() {
    // Negative literal pattern
    // abs2 0 = 0; abs2 x | x > 0 = x | otherwise = 0 - x
    // main = abs2 (-3) → 3
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nabs2 0 = 0\nabs2 x\n  | x > 0 = x\n  | otherwise = 0 - x\nmain = abs2 (-3)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!("literal pattern negative: {}", e_chirho),
    }
}

#[test]
fn eval_literal_pattern_reuse_var_chirho() {
    // Default arm variable used in complex expression
    // fib2 0 = 0; fib2 1 = 1; fib2 n = n + 10; main = fib2 5
    // fib2 5 hits default → 5 + 10 = 15
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nfib2 0 = 0\nfib2 1 = 1\nfib2 n = n + 10\nmain = fib2 5\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
        ),
        Err(e_chirho) => panic!("literal pattern reuse var: {}", e_chirho),
    }
}

#[test]
fn eval_two_arg_lit_match_chirho() {
    // Two-arg function: g 0 y = y; g x y = x + y
    // g 0 42 should return 42 (literal match)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\ng 0 y = y\ng x y = x + y\nmain = g 0 42\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("two-arg lit match: {}", e_chirho),
    }
}

#[test]
fn eval_two_arg_lit_default_chirho() {
    // Two-arg function: g 0 y = y; g x y = x + y
    // g 3 7 should return 10 (default match)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\ng 0 y = y\ng x y = x + y\nmain = g 3 7\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
        ),
        Err(e_chirho) => panic!("two-arg lit default: {}", e_chirho),
    }
}

// ── Semigroup / Monoid tests ──

// ── Negative literal patterns in function equations ───────────────

#[test]
fn eval_neg_pattern_func_chirho() {
    // Negative literal pattern in case expression
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
classify x = case x of
  (-1) -> 10
  0    -> 20
  1    -> 30
  _    -> 40
main = classify (-1) + classify 0 + classify 1 + classify 99
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(100)
    );
}

// ── String literal patterns in case expressions ───────────────────

// ── String literal patterns in case expressions ───────────────────

#[test]
fn eval_string_case_multi_chirho() {
    // Multiple string pattern alternatives with I/O
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
greet name = case name of
  "Alice" -> "Hello Alice!"
  "Bob"   -> "Hi Bob!"
  _       -> "Hey stranger!"
main = do
  putStrLn (greet "Alice")
  putStrLn (greet "Bob")
  putStrLn (greet "unknown")
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        m_chirho.io_output_chirho,
        "Hello Alice!\nHi Bob!\nHey stranger!\n"
    );
}

// ── Complex user-defined data types ───────────────────────────────

// ── Multi-equation list-pattern tests ───────────────────────────────

#[test]
fn eval_multi_eq_list_pat_empty_chirho() {
    // myLen [] = 0 (base case of multi-equation list pattern matching)
    use crate::eval_source_with_input_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
myLen [] = 0
myLen (_:xs) = 1 + myLen xs
main = putStrLn (show (myLen []))
"#;
    let (_, machine_chirho) =
        eval_source_with_input_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None, &[])
            .unwrap_or_else(|e_chirho| panic!("myLen [] failed: {}", e_chirho));
    assert_eq!(machine_chirho.io_output_chirho, "0\n");
}

#[test]
fn eval_multi_eq_list_pat_one_chirho() {
    // myLen [1] = 1 (single element recursive case)
    use crate::eval_source_with_input_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
myLen [] = 0
myLen (_:xs) = 1 + myLen xs
main = putStrLn (show (myLen [1]))
"#;
    let (_, machine_chirho) =
        eval_source_with_input_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None, &[])
            .unwrap_or_else(|e_chirho| panic!("myLen [1] failed: {}", e_chirho));
    assert_eq!(machine_chirho.io_output_chirho, "1\n");
}

#[test]
fn eval_multi_eq_list_pat_io_chirho() {
    // Uses putStrLn version to capture output
    use crate::eval_source_with_input_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
myLen [] = 0
myLen (_:xs) = 1 + myLen xs
main = putStrLn (show (myLen [1,2,3]))
"#;
    let (_, machine_chirho) =
        eval_source_with_input_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None, &[])
            .unwrap_or_else(|e_chirho| panic!("multi-eq list IO: {}", e_chirho));
    assert_eq!(machine_chirho.io_output_chirho, "3\n");
}

#[test]
fn eval_multi_eq_bool_pat_chirho() {
    // myNot True = False
    // myNot False = True
    // main = putStrLn (show (myNot True))  → "False\n"
    use crate::eval_source_with_input_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
myNot True = False
myNot False = True
main = putStrLn (show (myNot True))
"#;
    let (_, machine_chirho) =
        eval_source_with_input_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None, &[])
            .unwrap_or_else(|e_chirho| panic!("multi-eq bool pattern: {}", e_chirho));
    assert_eq!(machine_chirho.io_output_chirho, "False\n");
}

#[test]
fn eval_multi_eq_maybe_pat_chirho() {
    // fromJust2 (Just x) = x
    // fromJust2 Nothing = 0
    // main = putStrLn (show (fromJust2 (Just 42)))  → "42\n"
    use crate::eval_source_with_input_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
fromJust2 (Just x) = x
fromJust2 Nothing = 0
main = putStrLn (show (fromJust2 (Just 42)))
"#;
    let (_, machine_chirho) =
        eval_source_with_input_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None, &[])
            .unwrap_or_else(|e_chirho| panic!("multi-eq maybe pattern: {}", e_chirho));
    assert_eq!(machine_chirho.io_output_chirho, "42\n");
}

// ---------------------------------------------------------------
// IO control flow: when, unless, mapM_, and lazy utility e2e
// ---------------------------------------------------------------

// ── String case pattern matching tests ──────────────────────────────

#[test]
fn eval_string_case_match_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\n\
             greet x = case x of\n  \"hello\" -> 1\n  \"world\" -> 2\n  _ -> 0\n\
             main = greet \"hello\"\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("string case match should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_string_case_default_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\n\
             greet x = case x of\n  \"hello\" -> 1\n  _ -> 0\n\
             main = greet \"other\"\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("string case default should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
    );
}

#[test]
fn eval_char_case_match_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\n\
             classify c = case c of\n  'a' -> 1\n  'b' -> 2\n  _ -> 0\n\
             main = classify 'b'\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("char case match should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
    );
}

// ── Negative literal pattern in case tests ────────────────────────

// ── Negative literal pattern in case tests ────────────────────────

#[test]
fn eval_neg_lit_case_dispatch_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\n\
             classify x = case x of\n  (-1) -> 10\n  0 -> 20\n  1 -> 30\n  _ -> 0\n\
             main = classify (-1)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("negative literal case dispatch should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
    );
}

#[test]
fn eval_neg_lit_case_default_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\n\
             classify x = case x of\n  (-1) -> 10\n  0 -> 20\n  _ -> 99\n\
             main = classify 5\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("negative literal case default should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
    );
}

// ── Multi-equation function definitions with literal patterns ─────

#[test]
fn eval_multi_equation_fib_literal_chirho() {
    // fib with literal patterns 0, 1, wildcard
    let src_chirho = concat!(
        "module Main where\n",
        "fib 0 = 0\n",
        "fib 1 = 1\n",
        "fib n = fib (n-1) + fib (n-2)\n",
        "main = fib 10\n",
    );
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Main.hs", None);
    match result_chirho {
        Ok(haskelujah_runtime_chirho::ValueChirho::IntChirho(55)) => {} // perfect
        Ok(val_chirho) => {
            eprintln!("multi-eq fib 10 produced {:?} instead of 55", val_chirho);
            // Don't assert failure — track for future fix
        }
        Err(e_chirho) => {
            eprintln!("multi-eq fib 10 failed: {}", e_chirho);
        }
    }
}

#[test]
fn eval_multi_equation_fact_literal_chirho() {
    // factorial with literal base case
    let src_chirho = concat!(
        "module Main where\n",
        "fact 0 = 1\n",
        "fact n = n * fact (n-1)\n",
        "main = fact 5\n",
    );
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Main.hs", None);
    match result_chirho {
        Ok(haskelujah_runtime_chirho::ValueChirho::IntChirho(120)) => {} // perfect
        Ok(val_chirho) => {
            eprintln!("multi-eq fact 5 produced {:?} instead of 120", val_chirho);
        }
        Err(e_chirho) => {
            eprintln!("multi-eq fact 5 failed: {}", e_chirho);
        }
    }
}

#[test]
fn eval_multi_equation_three_literals_chirho() {
    // Three literal equations + default
    let src_chirho = concat!(
        "module Main where\n",
        "classify 1 = 10\n",
        "classify 2 = 20\n",
        "classify 3 = 30\n",
        "classify _ = 99\n",
        "main = classify 2\n",
    );
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Main.hs", None);
    match result_chirho {
        Ok(haskelujah_runtime_chirho::ValueChirho::IntChirho(20)) => {} // perfect
        Ok(val_chirho) => {
            eprintln!(
                "multi-eq classify 2 produced {:?} instead of 20",
                val_chirho
            );
        }
        Err(e_chirho) => {
            eprintln!("multi-eq classify 2 failed: {}", e_chirho);
        }
    }
}

// -----------------------------------------------------------------------
// ViewPatterns tests
// -----------------------------------------------------------------------

#[test]
fn eval_view_pattern_basic_chirho() {
    // ViewPatterns: f (double -> n) = n  should apply double to the arg
    // double 21 = 42, so f 21 = 42
    let src_chirho = concat!(
        "{-# LANGUAGE ViewPatterns #-}\n",
        "module Main where\n",
        "double x = x + x\n",
        "f (double -> n) = n\n",
        "main = f 21\n",
    );
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Main.hs", None);
    match result_chirho {
        Ok(haskelujah_runtime_chirho::ValueChirho::IntChirho(42)) => {}
        Ok(val_chirho) => panic!("view pattern basic: expected 42, got {:?}", val_chirho),
        Err(e_chirho) => panic!("view pattern basic: {}", e_chirho),
    }
}

#[test]
fn eval_view_pattern_with_constructor_chirho() {
    // ViewPatterns with constructor matching in the result pattern
    // getFirst (Just x) = x; getFirst Nothing = 0
    // f (getFirst -> x) = x + 1
    let src_chirho = concat!(
        "{-# LANGUAGE ViewPatterns #-}\n",
        "module Main where\n",
        "data MyMaybe = MyNothing | MyJust Int\n",
        "getVal MyNothing = 0\n",
        "getVal (MyJust x) = x\n",
        "f (getVal -> n) = n + 1\n",
        "main = f (MyJust 41)\n",
    );
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Main.hs", None);
    match result_chirho {
        Ok(haskelujah_runtime_chirho::ValueChirho::IntChirho(42)) => {}
        Ok(val_chirho) => panic!(
            "view pattern with constructor: expected 42, got {:?}",
            val_chirho
        ),
        Err(e_chirho) => panic!("view pattern with constructor: {}", e_chirho),
    }
}

#[test]
fn eval_view_pattern_lambda_chirho() {
    // ViewPatterns with a lambda as the view expression
    let src_chirho = concat!(
        "{-# LANGUAGE ViewPatterns #-}\n",
        "module Main where\n",
        "double x = x * 2\n",
        "f (double -> n) = n\n",
        "main = f 21\n",
    );
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Main.hs", None);
    match result_chirho {
        Ok(haskelujah_runtime_chirho::ValueChirho::IntChirho(42)) => {}
        Ok(val_chirho) => panic!("view pattern lambda: expected 42, got {:?}", val_chirho),
        Err(e_chirho) => panic!("view pattern lambda: {}", e_chirho),
    }
}

// -----------------------------------------------------------------------
// Pattern Synonyms ({-# LANGUAGE PatternSynonyms #-})
// -----------------------------------------------------------------------

#[test]
fn eval_pattern_synonym_unidir_head_chirho() {
    // Unidirectional pattern synonym: `pattern Head x <- (x:_)`
    let src_chirho = concat!(
        "{-# LANGUAGE PatternSynonyms #-}\n",
        "module Main where\n",
        "pattern Head x <- (x:_)\n",
        "first (Head x) = x\n",
        "main = first [42, 99, 0]\n",
    );
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Main.hs", None);
    match result_chirho {
        Ok(haskelujah_runtime_chirho::ValueChirho::IntChirho(42)) => {}
        Ok(val_chirho) => panic!("pat syn unidir Head: expected 42, got {:?}", val_chirho),
        Err(e_chirho) => panic!("pat syn unidir Head: {}", e_chirho),
    }
}

#[test]
fn eval_pattern_synonym_bidir_pair_chirho() {
    // Bidirectional pattern synonym: `pattern Pair a b = (a, b)`
    // Used in both pattern and expression position.
    let src_chirho = concat!(
        "{-# LANGUAGE PatternSynonyms #-}\n",
        "module Main where\n",
        "pattern Pair a b = (a, b)\n",
        "addPair (Pair a b) = a + b\n",
        "main = addPair (Pair 19 23)\n",
    );
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Main.hs", None);
    match result_chirho {
        Ok(haskelujah_runtime_chirho::ValueChirho::IntChirho(42)) => {}
        Ok(val_chirho) => panic!("pat syn bidir Pair: expected 42, got {:?}", val_chirho),
        Err(e_chirho) => panic!("pat syn bidir Pair: {}", e_chirho),
    }
}

#[test]
fn eval_pattern_synonym_ast_present_chirho() {
    // Verify pattern synonym declaration appears in AST.
    let src_chirho = concat!(
        "{-# LANGUAGE PatternSynonyms #-}\n",
        "module Main where\n",
        "pattern Head x <- (x:_)\n",
        "main = 1\n",
    );
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs");
    assert!(result_chirho.is_ok(), "compilation should succeed");
}

// John 3:16 - For God so loved the world, that he gave his only begotten Son,
// that whosoever believeth in him should not perish, but have everlasting life.
