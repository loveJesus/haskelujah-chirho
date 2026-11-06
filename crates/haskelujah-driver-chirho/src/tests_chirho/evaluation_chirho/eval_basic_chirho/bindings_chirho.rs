// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! let and where bindings, guards and conditionals (row 700).

use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn eval_literal_binding_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = 42\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    assert_eq!(
        result_chirho.unwrap(),
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );
}

#[test]
fn eval_chained_bindings_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Chain: a = 1, b = a + 2, main = b + 3
    let result_chirho = eval_source_chirho(
        "module Test where\na = 1\nb = a + 2\nmain = b + 3\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    assert_eq!(
        result_chirho.unwrap(),
        haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
    );
}

#[test]
fn eval_let_expression_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // let x = 5 in x  →  5
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = let x = 5 in x\n",
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
            eprintln!("eval_let_expression: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_where_clause_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // f 42 where f x = x  →  42  (identity, avoids Num constraints)
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = f 42\n  where\n    f x = x\n",
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
            eprintln!("eval_where_clause: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_nested_let_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Nested let: let a = 1 in let b = a + 2 in b + 3  →  6
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = let a = 1 in let b = a + 2 in b + 3\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
        ),
        Err(e_chirho) => {
            eprintln!("eval_nested_let: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_where_in_equation_chirho() {
    // Where clause binding used in a function
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
circleArea r = pi * r * r
  where pi = 3
main = circleArea 10
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("where in equation should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(300),
        "circleArea 10 with pi=3 should be 300"
    );
}

#[test]
fn eval_tuple_in_let_chirho() {
    // Construct a tuple in let, then case-match it
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = let p = (3, 7) in
       case p of
         (a, b) -> a + b
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("tuple in let should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10),
        "let p = (3,7) in case p of (a,b) -> a+b should be 10"
    );
}

// ── Priority 50: let/where in do-notation ──────────────────
#[test]
fn eval_let_in_where_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Nested let-expressions (one binding per let)
    let src_chirho = "\
module Test where\n\
main = let x = 5 in let y = 10 in x + y\n";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
            );
        }
        Err(e_chirho) => panic!(
            "let-in with multiple bindings should evaluate: {}",
            e_chirho
        ),
    }
}

#[test]
fn eval_if_false_branch_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if 1 > 5 then 100 else 200
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("if false branch should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(200)
    );
}

#[test]
fn eval_where_multi_binds_arith_chirho() {
    use crate::eval_source_with_machine_chirho;
    // where clause with multiple arithmetic bindings
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_val_chirho, machine_chirho) = eval_source_with_machine_chirho(
        "module Test where\n\
             f x = a + b\n  where\n    a = x + 1\n    b = x * 2\n\
             main = putStrLn (show (f 5))\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("where clause should evaluate");
    // f 5 = (5+1) + (5*2) = 6 + 10 = 16
    assert_eq!(machine_chirho.io_output_chirho, "16\n");
}

#[test]
fn eval_type_sig_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // f :: Int -> Int
    // f x = x + 1
    // main = f 41
    let result_chirho = eval_source_chirho(
        "module Test where\nf :: Int -> Int\nf x = x + 1\nmain = f 41\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!("type signature should work: {}", e_chirho),
    }
}

#[test]
fn eval_where_in_case_alt_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // where clause inside a case alternative
    let result_chirho = eval_source_chirho(
        "module Test where\nf x = case x of\n  0 -> z where z = 42\n  _ -> x + 1\nmain = f 0\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!("where in case alt should work: {}", e_chirho),
    }
}

#[test]
fn eval_let_pattern_bind_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // let (a, b) = (10, 20) in a + b
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = let (a, b) = (10, 20) in a + b\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(30)
            );
        }
        Err(e_chirho) => panic!("let pattern bind should work: {}", e_chirho),
    }
}

#[test]
fn eval_where_multiple_binds_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\nf x = a + b\n  where\n    a = x * 2\n    b = x + 1\nmain = f 10\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(31)
            );
        }
        Err(e_chirho) => panic!("multiple where bindings should work: {}", e_chirho),
    }
}

#[test]
fn eval_where_function_binding_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // where clause with simple value bindings (not function bindings)
    let result_chirho = eval_source_chirho(
        "module Test where\nf x = a + b + c\n  where\n    a = x * 3\n    b = x + 10\n    c = 2\nmain = f 5\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            // a = 15, b = 15, c = 2, total = 32
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(32)
            );
        }
        Err(e_chirho) => panic!("where function binding should work: {}", e_chirho),
    }
}

#[test]
fn eval_where_value_and_function_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // where clause with both value bindings and function bindings
    // (function does NOT reference other where bindings — that's a separate issue)
    let result_chirho = eval_source_chirho(
        "module Test where\nf x = a + g x\n  where\n    a = x * 3\n    g y = y + 10\nmain = f 5\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            // a = 15, g 5 = 15, total = 30
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(30)
            );
        }
        Err(e_chirho) => panic!("where value and function should work: {}", e_chirho),
    }
}

#[test]
fn eval_where_helper_function_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // where clause containing a function binding (takes a parameter)
    let result_chirho = eval_source_chirho(
        "module Test where\nf x = g x\n  where\n    g y = y + 10\nmain = f 5\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
            );
        }
        Err(e_chirho) => panic!("where helper function should work: {}", e_chirho),
    }
}

#[test]
fn eval_where_local_helper_with_compose_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // where clause with local helper using (.)
    let result_chirho = eval_source_chirho(
        "module Test where\nprocess x = result\n  where\n    double y = y + y\n    result = double (double x)\nmain = process 3\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(12)
        ),
        Err(e_chirho) => panic!("where local helper should work: {}", e_chirho),
    }
}

#[test]
fn eval_where_forward_ref_chirho() {
    // Forward reference: a uses b, b is defined after a in where block
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nmain = result\n  where\n    a = b + 1\n    b = 10\n    result = a\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(11)
        ),
        Err(e_chirho) => panic!("Where forward ref should work: {}", e_chirho),
    }
}

// ── Let with multiple bindings ──────────────────────────────────

#[test]
fn eval_let_multi_bind_chirho() {
    // let { a = 10; b = 20; c = 30 } in a + b + c
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = let a = 10\n           b = 20\n           c = 30\n       in a + b + c\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(60)
        ),
        Err(e_chirho) => panic!("let multi bind: {}", e_chirho),
    }
}

// ── If in where ─────────────────────────────────────────────────

#[test]
fn eval_if_in_where_chirho() {
    // classify with where clause using if
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
classify x = result
  where result = if x > 0 then 1 else 0
main = classify 42
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("if in where: {}", e_chirho),
    }
}

// ── Where with multiple helper functions ─────────────────────────

#[test]
fn eval_where_multi_helpers_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
compute x = doubled + tripled
  where doubled = x * 2
        tripled = x * 3
main = compute 5
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(25)
        ),
        Err(e_chirho) => panic!("where multi helpers: {}", e_chirho),
    }
}

#[test]
fn eval_nested_let_where_chirho() {
    // let with where-bound helper
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = let x = double 5 in x + 3
  where double n = n * 2
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(13)
        ),
        Err(e_chirho) => panic!("nested let where: {}", e_chirho),
    }
}

#[test]
fn eval_nested_where_simple_chirho() {
    // f x = a + b where { a = x * 2; b = x + 3 }
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nf x = a + b\n  where\n    a = x * 2\n    b = x + 3\nmain = f 10\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(33)
        ),
        Err(e_chirho) => panic!("nested where simple: {}", e_chirho),
    }
}

#[test]
fn eval_guard_multiple_equations_chirho() {
    // classify n | n < 0 = -1 | n == 0 = 0 | otherwise = 1
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nclassify n\n  | n < 0 = negate 1\n  | n == 0 = 0\n  | otherwise = 1\nmain = classify (negate 5) + classify 0 + classify 10\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
        ),
        Err(e_chirho) => panic!("guard multiple equations: {}", e_chirho),
    }
}
