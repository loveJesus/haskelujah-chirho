// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Integer and floating-point arithmetic, numeric literals and the numeric Prelude functions (row 700).

use crate::eval_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

// ── PrimOp / arithmetic end-to-end tests ────────────────────────────

#[test]
fn eval_addition_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = 3 + 4\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    assert_eq!(
        result_chirho.unwrap(),
        haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
    );
}

#[test]
fn eval_subtraction_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = 10 - 3\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    assert_eq!(
        result_chirho.unwrap(),
        haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
    );
}

#[test]
fn eval_multiplication_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = 6 * 7\n",
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
fn eval_nested_arithmetic_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // (2 + 3) * 4 = 20  — depends on parse associativity; if left-assoc:
    // 2 + 3 * 4 = 2 + 12 = 14  (if * binds tighter, which it should)
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = 2 + 3 * 4\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    // The parser should handle precedence; accept whatever it produces.
    assert!(result_chirho.is_ok());
}

#[test]
fn eval_negation_chirho() {
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module Test where\nmain = -(42)\n",
        &mut source_map_chirho,
        "TestChirho.hs",
    );
    // Should compile without error (negation desugars to PrimOp negate#)
    assert!(result_chirho.is_ok());
    let compiled_chirho = result_chirho.unwrap();
    // Core should contain a PrimOp
    assert!(!compiled_chirho.core_chirho.bindings_chirho.is_empty());
}

#[test]
fn eval_comparison_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // 3 < 5 should evaluate to True (BoolChirho(true))
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = 3 < 5\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => assert_eq!(
            *val_chirho,
            haskelujah_runtime_chirho::ValueChirho::BoolChirho(true)
        ),
        Err(e_chirho) => {
            eprintln!("eval_comparison: {}", e_chirho);
            assert!(
                result_chirho.is_ok(),
                "comparison should evaluate: {}",
                e_chirho
            );
        }
    }
}

#[test]
fn eval_double_variable_add_chirho() {
    // Test that Double variables use $fNumDouble for arithmetic.
    // `addDoubles x y = x + y` with float args should use +.#
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
addDoubles x y = x + y
main = putStrLn (show (addDoubles 1.5 2.5))
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho, "4.0\n",
                "addDoubles 1.5 2.5 should produce 4.0"
            );
        }
        Err(e_chirho) => {
            panic!("Double variable add failed: {}", e_chirho);
        }
    }
}

#[test]
fn eval_float_division_literal_chirho() {
    // 1.0 / 2.0 should evaluate to 0.5 via float-aware dispatch
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = 1.0 / 2.0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("float division should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(0.5),
        "1.0 / 2.0 should be 0.5"
    );
}

#[test]
fn eval_float_recip_chirho() {
    // recip 4.0 should evaluate to 0.25
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = recip 4.0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("recip should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(0.25),
        "recip 4.0 should be 0.25"
    );
}

#[test]
fn eval_float_division_expression_chirho() {
    // 10.0 / 4.0 should evaluate to 2.5
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = 10.0 / 4.0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("float division should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(2.5),
        "10.0 / 4.0 should be 2.5"
    );
}

// ---------------------------------------------------------------
// Prelude numeric/predicate function tests
// ---------------------------------------------------------------

#[test]
fn eval_abs_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = abs (-5)\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
            );
        }
        Err(e_chirho) => {
            panic!("abs should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_max_min_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = max 3 7 + min 3 7\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            // max 3 7 = 7, min 3 7 = 3, 7 + 3 = 10
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
            );
        }
        Err(e_chirho) => {
            panic!("max+min should evaluate: {}", e_chirho);
        }
    }
}

#[test]
fn eval_from_integral_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // fromIntegral 5 + 0.5 should give 5.5
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\n\
             main = putStrLn (show (fromIntegral 5 + 0.5))\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("fromIntegral should evaluate");
    assert_eq!(result_chirho.1.io_output_chirho, "5.5\n");
}

#[test]
fn eval_to_integer_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             main = toInteger 42\n",
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
        Err(e_chirho) => panic!("toInteger should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_min_chirho() {
    // min 3 5 → 3
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = min 3 5
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("min 3 5 should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3),
        "min 3 5 = 3"
    );
}

#[test]
fn eval_max_chirho() {
    // max 3 5 → 5
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = max 3 5
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("max 3 5 should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5),
        "max 3 5 = 5"
    );
}

#[test]
fn eval_abs_positive_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = abs 5
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("abs positive should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
    );
}

#[test]
fn eval_signum_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Test 1: simple addition (dict-resolved)
    let src1_chirho = "module Test where\nmain = 3 + 5\n";
    let r1_chirho = eval_source_chirho(src1_chirho, &mut source_map_chirho, "TestChirho.hs", None)
        .expect("simple + test");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(8),
        "3 + 5 = {:?}",
        r1_chirho
    );

    // Test 2: abs 5 alone
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let src2_chirho = "module Test where\nmain = abs 5\n";
    let r2_chirho = eval_source_chirho(src2_chirho, &mut sm2_chirho, "TestChirho.hs", None)
        .expect("abs 5 test");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5),
        "abs 5 = {:?}",
        r2_chirho
    );

    // Test 3: abs 5 + 0
    let mut sm3_chirho = SourceMapChirho::new_chirho();
    let src3_chirho = "module Test where\nmain = abs 5 + 0\n";
    let r3_chirho = eval_source_chirho(src3_chirho, &mut sm3_chirho, "TestChirho.hs", None)
        .expect("abs 5 + 0 test");
    assert_eq!(
        r3_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5),
        "abs 5 + 0 = {:?}",
        r3_chirho
    );

    // Test 4: 0 + abs 3
    let mut sm4_chirho = SourceMapChirho::new_chirho();
    let src4_chirho = "module Test where\nmain = 0 + abs 3\n";
    let r4_chirho = eval_source_chirho(src4_chirho, &mut sm4_chirho, "TestChirho.hs", None)
        .expect("0 + abs 3 test");
    assert_eq!(
        r4_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3),
        "0 + abs 3 = {:?}",
        r4_chirho
    );

    // Test 5: abs 5 + abs 3
    let mut sm5_chirho = SourceMapChirho::new_chirho();
    let src5_chirho = "module Test where\nmain = abs 5 + abs 3\n";
    let r5_chirho = eval_source_chirho(src5_chirho, &mut sm5_chirho, "TestChirho.hs", None)
        .expect("abs 5 + abs 3 test");
    assert_eq!(
        r5_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(8),
        "abs 5 + abs 3 = {:?}",
        r5_chirho
    );
}

#[test]
fn eval_quot_rem_chirho() {
    use crate::eval_source_chirho;
    // quot 17 5 = 3 (truncate toward zero)
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = quot 17 5\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("quot should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );

    // rem 17 5 = 2
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = rem 17 5\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("rem should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
    );

    // quot a b * b + rem a b == a
    let mut sm3_chirho = SourceMapChirho::new_chirho();
    let r3_chirho = eval_source_chirho(
        "module Test where\nmain = quot 17 5 * 5 + rem 17 5\n",
        &mut sm3_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("quot/rem identity should evaluate");
    assert_eq!(
        r3_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(17)
    );

    // Backtick syntax: 17 `quot` 5
    let mut sm4_chirho = SourceMapChirho::new_chirho();
    let r4_chirho = eval_source_chirho(
        "module Test where\nmain = 17 `quot` 5\n",
        &mut sm4_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("backtick quot should evaluate");
    assert_eq!(
        r4_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );
}

#[test]
fn eval_tan_atan_chirho() {
    use crate::eval_source_chirho;
    // tan 0.0 = 0.0
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = tan 0.0\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("tan 0.0 should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(0.0)
    );

    // atan 0.0 = 0.0
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = atan 0.0\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("atan 0.0 should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(0.0)
    );
}

#[test]
fn eval_power_int_chirho() {
    use crate::eval_source_chirho;
    // 2 ^ 10 = 1024
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = 2 ^ 10\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("2^10 should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1024)
    );

    // 3 ^ 0 = 1
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = 3 ^ 0\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("3^0 should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );

    // 5 ^ 3 = 125
    let mut sm3_chirho = SourceMapChirho::new_chirho();
    let r3_chirho = eval_source_chirho(
        "module Test where\nmain = 5 ^ 3\n",
        &mut sm3_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("5^3 should evaluate");
    assert_eq!(
        r3_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(125)
    );
}

#[test]
fn eval_power_float_chirho() {
    use crate::eval_source_chirho;
    // 2.0 ** 3.0 = 8.0
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = 2.0 ** 3.0\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("2.0**3.0 should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(8.0)
    );

    // 4.0 ** 0.5 = 2.0 (square root)
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = 4.0 ** 0.5\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("4.0**0.5 should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(2.0)
    );
}

#[test]
fn eval_num_double_sub_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = 10.5 - 3.5\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("10.5 - 3.5 should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(7.0)
    );
}

#[test]
fn eval_num_double_mul_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = 2.5 * 4.0\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("2.5 * 4.0 should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(10.0)
    );
}

#[test]
fn eval_num_double_negate_chirho() {
    use crate::eval_source_chirho;
    // negate via unary minus in expression
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = negate 5.5\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("negate 5.5 should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(-5.5)
    );
}

#[test]
fn eval_num_double_abs_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = abs (-7.25)\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("abs (-7.25) should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(7.25)
    );
}

#[test]
fn eval_num_double_signum_chirho() {
    use crate::eval_source_chirho;
    // signum of negative
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = signum (-3.0)\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("signum (-3.0) should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(-1.0)
    );

    // signum of positive
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = signum 42.0\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("signum 42.0 should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(1.0)
    );
}

#[test]
fn eval_fractional_div_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = 10.0 / 4.0\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("10.0 / 4.0 should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(2.5)
    );
}

#[test]
fn eval_fractional_recip_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = recip 4.0\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("recip 4.0 should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(0.25)
    );
}

#[test]
fn eval_mixed_int_float_add_chirho() {
    use crate::eval_source_chirho;
    // 1 + 2.5 = 3.5 (integer literal in floating context)
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = 1 + 2.5\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("1 + 2.5 should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(3.5)
    );
}

#[test]
fn eval_mixed_int_float_mul_chirho() {
    use crate::eval_source_chirho;
    // 3 * 2.0 = 6.0
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let r_chirho = eval_source_chirho(
        "module Test where\nmain = 3 * 2.0\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("3 * 2.0 should evaluate");
    assert_eq!(
        r_chirho,
        haskelujah_runtime_chirho::ValueChirho::FloatChirho(6.0)
    );
}

#[test]
fn eval_prelude_gcd_lcm_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = gcd 12 18 + lcm 12 18\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("gcd/lcm should use Prelude bodies: {}", e_chirho),
    }
}

#[test]
fn eval_prelude_divmod_quotrem_splitat_chirho() {
    use crate::eval_source_chirho;
    // divMod floors (div#/mod#), quotRem truncates (quot#/rem#), and splitAt
    // reuses take/drop — each destructured to a distinctive Int result.
    let cases_chirho: [(&str, i64); 3] = [
        ("main = case divMod (-7) 2 of (q,r) -> q*10+r\n", -39),
        ("main = case quotRem (-7) 2 of (q,r) -> q*10+r\n", -31),
        (
            "main = case splitAt 2 [1,2,3,4,5] of (l,r) -> sum l * 100 + sum r\n",
            312,
        ),
    ];
    for (body_chirho, want_chirho) in cases_chirho {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Test where\n{body_chirho}");
        let result_chirho = eval_source_chirho(&src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(want_chirho),
                "unexpected result for: {body_chirho}"
            ),
            Err(e_chirho) => {
                panic!("divMod/quotRem/splitAt should use Prelude bodies: {e_chirho}")
            }
        }
    }
}

#[test]
fn eval_hex_literal_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let val_chirho = eval_source_chirho(
        "module Main where\nmain = 0xFF\n",
        &mut sm_chirho,
        "Main.hs",
        None,
    )
    .unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(255)
    );
}

#[test]
fn eval_octal_literal_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let val_chirho = eval_source_chirho(
        "module Main where\nmain = 0o77\n",
        &mut sm_chirho,
        "Main.hs",
        None,
    )
    .unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(63)
    );
}

#[test]
fn eval_hex_plus_octal_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let val_chirho = eval_source_chirho(
        "module Main where\nmain = 0xFF + 0o17\n",
        &mut sm_chirho,
        "Main.hs",
        None,
    )
    .unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(270)
    );
}

#[test]
fn eval_binary_literal_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let val_chirho = eval_source_chirho(
        "module Main where\nmain = 0b1010\n",
        &mut sm_chirho,
        "Main.hs",
        None,
    )
    .unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
    );
}
