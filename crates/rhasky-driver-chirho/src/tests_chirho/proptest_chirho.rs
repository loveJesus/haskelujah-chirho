// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Property-based tests for the full compilation + evaluation pipeline.
//!
//! These tests verify that the STG runtime evaluator terminates correctly,
//! preserves integer arithmetic identities, and handles edge cases gracefully.

use proptest::prelude::*;
use rhasky_span_chirho::SourceMapChirho;
use rhasky_runtime_chirho::ValueChirho;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Integer literals should evaluate to themselves.
    #[test]
    fn eval_int_literal_identity_chirho(val_chirho in -1000i64..1000) {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Main where\nmain = {}\n", val_chirho);
        let result_chirho = crate::eval_source_chirho(
            &src_chirho,
            &mut sm_chirho,
            "PropInt.hs",
            None,
        );
        prop_assert!(result_chirho.is_ok(), "eval should succeed for literal {}", val_chirho);
        prop_assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(val_chirho));
    }

    /// Addition of two literals should produce the correct sum.
    #[test]
    fn eval_addition_correct_chirho(
        a_chirho in -500i64..500,
        b_chirho in -500i64..500
    ) {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Main where\nmain = {} + {}\n", a_chirho, b_chirho);
        let result_chirho = crate::eval_source_chirho(
            &src_chirho,
            &mut sm_chirho,
            "PropAdd.hs",
            None,
        );
        prop_assert!(result_chirho.is_ok(), "eval should succeed for {} + {}", a_chirho, b_chirho);
        prop_assert_eq!(
            result_chirho.unwrap(),
            ValueChirho::IntChirho(a_chirho + b_chirho)
        );
    }

    /// Multiplication of two literals should produce the correct product.
    #[test]
    fn eval_multiplication_correct_chirho(
        a_chirho in -50i64..50,
        b_chirho in -50i64..50
    ) {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Main where\nmain = {} * {}\n", a_chirho, b_chirho);
        let result_chirho = crate::eval_source_chirho(
            &src_chirho,
            &mut sm_chirho,
            "PropMul.hs",
            None,
        );
        prop_assert!(result_chirho.is_ok(), "eval should succeed for {} * {}", a_chirho, b_chirho);
        prop_assert_eq!(
            result_chirho.unwrap(),
            ValueChirho::IntChirho(a_chirho * b_chirho)
        );
    }

    /// `let x = N in x` should evaluate to N.
    #[test]
    fn eval_let_identity_chirho(val_chirho in -1000i64..1000) {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Main where\nmain = let x = {} in x\n", val_chirho);
        let result_chirho = crate::eval_source_chirho(
            &src_chirho,
            &mut sm_chirho,
            "PropLet.hs",
            None,
        );
        prop_assert!(result_chirho.is_ok(), "let binding should work for {}", val_chirho);
        prop_assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(val_chirho));
    }

    /// `if True then a else b` should always produce a.
    #[test]
    fn eval_if_true_chirho(
        a_chirho in -100i64..100,
        b_chirho in -100i64..100
    ) {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!(
            "module Main where\nmain = if True then {} else {}\n",
            a_chirho, b_chirho
        );
        let result_chirho = crate::eval_source_chirho(
            &src_chirho,
            &mut sm_chirho,
            "PropIfTrue.hs",
            None,
        );
        prop_assert!(result_chirho.is_ok());
        prop_assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(a_chirho));
    }

    /// `if False then a else b` should always produce b.
    #[test]
    fn eval_if_false_chirho(
        a_chirho in -100i64..100,
        b_chirho in -100i64..100
    ) {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!(
            "module Main where\nmain = if False then {} else {}\n",
            a_chirho, b_chirho
        );
        let result_chirho = crate::eval_source_chirho(
            &src_chirho,
            &mut sm_chirho,
            "PropIfFalse.hs",
            None,
        );
        prop_assert!(result_chirho.is_ok());
        prop_assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(b_chirho));
    }

    /// Function application: `(\x -> x + 1) n` should produce n + 1.
    #[test]
    fn eval_lambda_apply_chirho(val_chirho in -500i64..500) {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!(
            "module Main where\nf x = x + 1\nmain = f ({})\n",
            val_chirho
        );
        let result_chirho = crate::eval_source_chirho(
            &src_chirho,
            &mut sm_chirho,
            "PropLam.hs",
            None,
        );
        prop_assert!(result_chirho.is_ok());
        prop_assert_eq!(
            result_chirho.unwrap(),
            ValueChirho::IntChirho(val_chirho + 1)
        );
    }

    /// Subtraction identity: a - a should always be 0.
    #[test]
    fn eval_subtraction_identity_chirho(val_chirho in -1000i64..1000) {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!(
            "module Main where\nx = {}\nmain = x - x\n",
            val_chirho
        );
        let result_chirho = crate::eval_source_chirho(
            &src_chirho,
            &mut sm_chirho,
            "PropSub.hs",
            None,
        );
        prop_assert!(result_chirho.is_ok());
        prop_assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(0));
    }
}
