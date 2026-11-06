// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Function application, backtick infix, lambdas, operator sections and composition (row 700).

use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn eval_identity_function_chirho() {
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let compile_result_chirho = compile_source_chirho(
        "module Test where\nid x = x\nmain = id 7\n",
        &mut source_map_chirho,
        "TestChirho.hs",
    )
    .expect("should compile");

    // Debug: print core bindings
    for b_chirho in &compile_result_chirho.core_chirho.bindings_chirho {
        eprintln!(
            "binding: {} (id={}) = {:?}",
            b_chirho.binder_chirho.name_chirho,
            b_chirho.binder_chirho.id_chirho.0,
            b_chirho.rhs_chirho
        );
    }

    let (result_chirho, _) = crate::stg_lower_chirho::lower_and_run_chirho(
        &compile_result_chirho.core_chirho,
        None,
        compile_result_chirho.newtype_cons_chirho,
    )
    .expect("should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
    );
}

#[test]
fn eval_function_with_arithmetic_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Use a function definition instead of let-in (which needs specific parse support)
    let result_chirho = eval_source_chirho(
        "module Test where\nx = 10\nmain = x + 5\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    assert_eq!(
        result_chirho.unwrap(),
        haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
    );
}

#[test]
fn eval_function_application_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Simple function application: double x = x + x; main = double 21
    let result_chirho = eval_source_chirho(
        "module Test where\ndouble x = x + x\nmain = double 21\n",
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
            eprintln!("eval_function_application: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_higher_order_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Higher-order: apply f x = f x; main = apply id 42  →  42
    // Uses id (no Num constraint) to avoid dict-pass complications
    let result_chirho = eval_source_chirho(
        "module Test where\nid x = x\napply f x = f x\nmain = apply id 42\n",
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
            eprintln!("eval_higher_order: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_partial_application_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Partial application: const x y = x; main = const 7 99  →  7
    // Uses const (no Num constraint) to test multi-arg + partial app
    let result_chirho = eval_source_chirho(
        "module Test where\nconst x y = x\nmain = const 7 99\n",
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
            eprintln!("eval_partial_application: {}", e_chirho);
            assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
        }
    }
}

// ---------------------------------------------------------------
// Tuple operation tests (fst, snd)
// ---------------------------------------------------------------

#[test]
fn eval_backtick_div_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = 10 `div` 3\n",
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
        Err(e_chirho) => {
            panic!("backtick div should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_backtick_mod_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = 10 `mod` 3\n",
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
            panic!("backtick mod should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_backtick_gcd_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // GCD using backtick syntax for mod
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             gcd a b = if b == 0 then a else gcd b (a `mod` b)\n\
             main = gcd 12 8\n",
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
        Err(e_chirho) => {
            panic!("backtick GCD should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_operator_section_plus_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // (+ ) applied to two arguments via uncurry
    let src_chirho = "\
module Test where
add = (+)
main = add 3 4
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
        Err(e_chirho) => panic!("operator section (+) should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_operator_section_multiply_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
mul = (*)
main = mul 5 6
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(30)
            );
        }
        Err(e_chirho) => panic!("operator section (*) should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_lambda_case_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // classify = \case { 0 -> "zero"; 1 -> "one"; _ -> "other" }
    // main = length (classify 0)
    let result_chirho = eval_source_chirho(
        "module Test where\nclassify = \\case\n  0 -> \"zero\"\n  1 -> \"one\"\n  _ -> \"other\"\nmain = length (classify 0)\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(4)
            );
        }
        Err(e_chirho) => panic!("lambda case should work: {}", e_chirho),
    }
}

#[test]
fn eval_compose_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Manual function composition: compose f g x = f (g x)
    let result_chirho = eval_source_chirho(
        "module Test where\ndouble x = x * 2\nincr x = x + 1\ncompose f g x = f (g x)\nmain = compose double incr 5\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(12)
            );
        }
        Err(e_chirho) => panic!("compose should work: {}", e_chirho),
    }
}

#[test]
fn eval_flip_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // flip f x y = f y x; flip (-) 3 10 = 10 - 3 = 7
    let result_chirho = eval_source_chirho(
        "module Test where\nflipF f x y = f y x\nmain = flipF (-) 3 10\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
            );
        }
        Err(e_chirho) => panic!("flip should work: {}", e_chirho),
    }
}

#[test]
fn eval_map_with_lambda_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // map (\x -> (x + 1) * 2) [1,2,3] → [4,6,8], sum → 18
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = sum (map (\\x -> (x + 1) * 2) [1,2,3])\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(18)
            );
        }
        Err(e_chirho) => panic!("map with lambda should work: {}", e_chirho),
    }
}

#[test]
fn eval_flip_builtin_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // flip (-) 3 10 = (-) 10 3 = 7, using built-in flip
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = flip (-) 3 10\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
            );
        }
        Err(e_chirho) => panic!("built-in flip should work: {}", e_chirho),
    }
}

#[test]
fn eval_dot_compose_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Test the (.) operator: (double . succ) 20 = double (succ 20) = double 21 = 42
    let result_chirho = eval_source_chirho(
        "module Test where\ndouble x = x + x\nsucc x = x + 1\nmain = (double . succ) 20\n",
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
        Err(e_chirho) => panic!("dot compose should work: {}", e_chirho),
    }
}

#[test]
fn eval_dot_compose_chain_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Chained composition: (f . g . h) x = f (g (h x))
    // add1 . double . add1 $ 5 = add1(double(add1(5))) = add1(double(6)) = add1(12) = 13
    let result_chirho = eval_source_chirho(
        "module Test where\nadd1 x = x + 1\ndouble x = x + x\nmain = (add1 . double . add1) 5\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(13)
        ),
        Err(e_chirho) => panic!("chained composition should work: {}", e_chirho),
    }
}

#[test]
fn eval_dot_compose_with_dollar_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // (.) combined with ($): double . succ $ 20 = (double . succ) 20 = 42
    let result_chirho = eval_source_chirho(
        "module Test where\ndouble x = x + x\nsucc x = x + 1\nmain = double . succ $ 20\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("dot with dollar should work: {}", e_chirho),
    }
}

#[test]
fn eval_higher_order_composition_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // map ((*2) . (+1)) [1,2,3] = [4,6,8]
    // Using user-defined compose and apply functions
    let result_chirho = eval_source_chirho(
        "module Test where\ncomp f g x = f (g x)\ntimes2 x = x * 2\nadd1 x = x + 1\nmain = sum (map (comp times2 add1) [1,2,3])\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(18)
        ),
        Err(e_chirho) => panic!("higher order composition should work: {}", e_chirho),
    }
}

#[test]
fn eval_section_addition_chirho() {
    // map (+10) [1,2,3] → [11,12,13], sum → 36
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = sum (map (+10) [1,2,3])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(36)
        ),
        Err(e_chirho) => panic!("section (+10): {}", e_chirho),
    }
}

// ── Type annotation expressions ─────────────────────────────────

// ── Lambda with tuple pattern ────────────────────────────────────

#[test]
fn eval_lambda_tuple_pattern_chirho() {
    // (\(x, y) -> x + y) (3, 4)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = (\\(x, y) -> x + y) (3, 4)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
        ),
        Err(e_chirho) => panic!("lambda tuple pattern: {}", e_chirho),
    }
}

// ── Chained function application ────────────────────────────────

// ── Chained function application ────────────────────────────────

#[test]
fn eval_chained_dollar_chirho() {
    // head $ filter even $ [1..10] → 2
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = head $ filter even $ [1..10]\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("chained $: {}", e_chirho),
    }
}

// ── Lambda with constructor pattern ──────────────────────────────

// ── Lambda with constructor pattern ──────────────────────────────

#[test]
fn eval_lambda_con_pattern_chirho() {
    // (\(Just x) -> x + 1) (Just 41) → 42
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = (\\(Just x) -> x + 1) (Just 41)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("lambda con pattern: {}", e_chirho),
    }
}

// ── Uncurry with operator section ────────────────────────────────

// ── Map with operator section ────────────────────────────────────

#[test]
fn eval_map_section_subtract_chirho() {
    // map (subtract 1) [10, 20, 30] → [9, 19, 29], sum → 57
    // (subtract is \a b -> b - a in Prelude)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = sum (map (subtract 1) [10, 20, 30])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(57)
        ),
        Err(e_chirho) => panic!("map subtract: {}", e_chirho),
    }
}

// ── Higher-order function composition ─────────────────────────────

/// Test 2: twice applied twice — higher-order composition.
/// twice (twice inc) 0 = 4
#[test]
fn eval_twice_composition_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
twice f x = f (f x)
inc x = x + 1
main = putStrLn (show (twice (twice inc) 0))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(m_chirho.io_output_chirho, "4\n");
}
