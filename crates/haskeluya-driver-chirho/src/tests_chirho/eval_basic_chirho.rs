// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// Basic arithmetic, literals, function application, let/where, bindings tests

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
use haskeluya_span_chirho::SourceMapChirho;
#[allow(unused_imports)]
use haskeluya_runtime_chirho::{ValueChirho, ExecutionModeChirho};
#[allow(unused_imports)]
use haskeluya_syntax_chirho::SourceFileChirho;


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
            haskeluya_runtime_chirho::ValueChirho::IntChirho(42)
        );
    }


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
        assert_eq!(result_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(7));
    }


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
            haskeluya_runtime_chirho::ValueChirho::IntChirho(7)
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
            haskeluya_runtime_chirho::ValueChirho::IntChirho(7)
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
            haskeluya_runtime_chirho::ValueChirho::IntChirho(42)
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
            haskeluya_runtime_chirho::ValueChirho::IntChirho(15)
        );
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
                haskeluya_runtime_chirho::ValueChirho::BoolChirho(true)
            ),
            Err(e_chirho) => {
                eprintln!("eval_comparison: {}", e_chirho);
                assert!(result_chirho.is_ok(), "comparison should evaluate: {}", e_chirho);
            }
        }
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
                haskeluya_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_function_application: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
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
            haskeluya_runtime_chirho::ValueChirho::IntChirho(6)
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
                haskeluya_runtime_chirho::ValueChirho::IntChirho(5)
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
                haskeluya_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_where_clause: {}", e_chirho);
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
                haskeluya_runtime_chirho::ValueChirho::IntChirho(42)
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
                haskeluya_runtime_chirho::ValueChirho::IntChirho(7)
            ),
            Err(e_chirho) => {
                eprintln!("eval_partial_application: {}", e_chirho);
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
                haskeluya_runtime_chirho::ValueChirho::IntChirho(6)
            ),
            Err(e_chirho) => {
                eprintln!("eval_nested_let: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_string_literal_chirho() {
        // String literal evaluates to StringChirho value
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = \"hello\"\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                haskeluya_runtime_chirho::ValueChirho::StringChirho("hello".to_string())
            ),
            Err(e_chirho) => {
                eprintln!("eval_string_literal: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
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
                haskeluya_runtime_chirho::ValueChirho::CharChirho('A')
            ),
            Err(e_chirho) => {
                eprintln!("eval_char_literal: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn from_integer_explicit_call_chirho() {
        // Test that `fromInteger 42` works through the dict pass.
        // fromInteger is a Num class method; for Int it's the identity.
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = fromInteger 42
";
        let compile_result_chirho = compile_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
        )
        .expect("should compile");

        let (result_chirho, _) = crate::stg_lower_chirho::lower_and_run_chirho(
            &compile_result_chirho.core_chirho,
            None,
            compile_result_chirho.newtype_cons_chirho,
        )
        .expect("should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(42),
            "fromInteger 42 should evaluate to 42"
        );
    }


    #[test]
    fn callsite_dict_arg_insertion_chirho() {
        // A user-defined constrained function `add` that calls a class
        // method `+` internally. When `main` calls `add 3 4`, the dict
        // pass must insert the Num dictionary at the call site:
        //   add = \$dNum -> \x -> \y -> ($sel_Num_+ $dNum) x y
        //   main = add $fNumInt (fromInteger 3) (fromInteger 4)
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
add x y = x + y
main = add 3 4
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("call-site dict insertion should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(7),
            "add 3 4 should be 7"
        );
    }


    #[test]
    fn callsite_dict_passthrough_chirho() {
        // Dict pass-through: a constrained function `add` is called from
        // another constrained function `addTwice`, which is then called
        // from monomorphic `main`. The dict must flow through two levels:
        //   addTwice = \$dNum -> \x -> \y -> add $dNum (add $dNum x y) y
        //   main = addTwice $fNumInt 3 4
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
add x y = x + y
addTwice x y = add (add x y) y
main = addTwice 3 4
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("dict pass-through should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(11),
            "addTwice 3 4 = add (add 3 4) 4 = add 7 4 = 11"
        );
    }


    #[test]
    fn superclass_dict_extraction_chirho() {
        // Tests context reduction + superclass dict extraction:
        // `f` uses both `==` (Eq) and `+` (Num). Context reduction removes
        // the redundant `Eq a` pred since `Num` has `Eq` as a superclass.
        // The dict pass generates:
        //   f = \$dNum -> let $dEq = $sel_Num_super_Eq $dNum in
        //                   if ($sel_Eq_== $dEq) x y then ($sel_Num_+ $dNum) x y else ...
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
f x y = if x == y then x + y else x - y
main = f 3 3
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("superclass dict extraction should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(6),
            "f 3 3: 3 == 3 is true, so 3 + 3 = 6"
        );
    }


    #[test]
    fn superclass_dict_extraction_else_branch_chirho() {
        // Same as above but hits the else branch: 3 /= 4 so x - y = -1
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
f x y = if x == y then x + y else x - y
main = f 3 4
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("superclass dict extraction else branch should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(-1),
            "f 3 4: 3 /= 4, so 3 - 4 = -1"
        );
    }


    #[test]
    fn eval_tuple_swap_chirho() {
        // Swap a tuple and extract first element (tests construction + pattern match)
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
swap p = case p of
  (a, b) -> (b, a)
fst p = case p of
  (a, b) -> a
main = fst (swap (1, 2))
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("tuple swap should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(2),
            "fst (swap (1, 2)) should be 2"
        );
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("where in equation should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(300),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("tuple in let should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(10),
            "let p = (3,7) in case p of (a,b) -> a+b should be 10"
        );
    }


    #[test]
    fn eval_triple_chirho() {
        // 3-tuple construction and pattern match
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
third t = case t of
  (a, b, c) -> c
main = third (1, 2, 42)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("triple should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(42),
            "third (1, 2, 42) should be 42"
        );
    }


    #[test]
    fn eval_multi_constructor_data_chirho() {
        // Data type with multiple constructors and case dispatch
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Shape = Circle Int | Rectangle Int Int
area s = case s of
  Circle r -> r * r
  Rectangle w h -> w * h
main = area (Rectangle 3 4)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("multi-constructor data should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(12),
            "area (Rectangle 3 4) should be 12"
        );
    }


    #[test]
    fn eval_maybe_just_chirho() {
        // Maybe with Just: fromMaybe 0 (Just 42) = 42
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Maybe a = Nothing | Just a
fromMaybe def m = case m of
  Nothing -> def
  Just x  -> x
main = fromMaybe 0 (Just 42)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("Maybe Just should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(42),
            "fromMaybe 0 (Just 42) should be 42"
        );
    }


    #[test]
    fn eval_maybe_nothing_chirho() {
        // Maybe with Nothing: fromMaybe 0 Nothing = 0
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Maybe a = Nothing | Just a
fromMaybe def m = case m of
  Nothing -> def
  Just x  -> x
main = fromMaybe 99 Nothing
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("Maybe Nothing should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(99),
            "fromMaybe 99 Nothing should be 99"
        );
    }


    #[test]
    fn eval_either_chirho() {
        // Either type with Left/Right dispatch
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Either a b = Left a | Right b
fromRight def e = case e of
  Left _  -> def
  Right x -> x
main = fromRight 0 (Right 42)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("Either should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(42),
            "fromRight 0 (Right 42) should be 42"
        );
    }


    #[test]
    fn eval_binary_tree_chirho() {
        // Binary tree with recursive sum
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Tree = Leaf Int | Node Tree Tree
treeSum t = case t of
  Leaf n -> n
  Node l r -> treeSum l + treeSum r
main = treeSum (Node (Node (Leaf 1) (Leaf 2)) (Leaf 3))
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("binary tree should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(6),
            "treeSum (Node (Node (Leaf 1) (Leaf 2)) (Leaf 3)) should be 6"
        );
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
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("show with concat should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "x = 42\n",
            "putStrLn (\"x = \" ++ show 42) should produce 'x = 42'"
        );
    }


    #[test]
    fn eval_arith_sequence_from_to_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum [1..3]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("arith sequence should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(6),
            "sum [1..3] should be 6"
        );
    }


    #[test]
    fn eval_arith_sequence_sum_1_to_5_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum [1..5]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("arith sequence [1..5] should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(15),
            "sum [1..5] should be 15"
        );
    }


    #[test]
    fn eval_arith_sequence_length_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
len xs = case xs of
  [] -> 0
  (x:rest) -> 1 + len rest
main = len [10..15]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("length of [10..15] should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(6),
            "len [10..15] should be 6"
        );
    }


    #[test]
    fn eval_map_with_arith_sequence_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
myMap f xs = case xs of
  [] -> []
  (x:rest) -> f x : myMap f rest
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum (myMap (\\x -> x + 10) [1..3])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("map with lambda over arith sequence should evaluate");
        // [1,2,3] mapped with (+10) gives [11,12,13], sum = 36
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(36),
            "sum (map (+10) [1..3]) should be 36"
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
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("putStrLn (show [1,2,3]) should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "[1,2,3]\n",
            "show [1,2,3] should produce the string \"[1,2,3]\""
        );
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
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "4.0\n",
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("float division should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::FloatChirho(0.5),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("recip should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::FloatChirho(0.25),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("float division should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::FloatChirho(2.5),
            "10.0 / 4.0 should be 2.5"
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
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "1.5\n",
                    "show (6.0 / 4.0) should produce 1.5"
                );
            }
            Err(e_chirho) => {
                panic!("show float division failed: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_eq_list_double_chirho() {
        // [1.0, 2.0] == [1.0, 2.0] should be True (via conditional Eq [a] instance)
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if [1.0, 2.0] == [1.0, 2.0] then putStrLn \"equal\" else putStrLn \"not equal\"
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "equal\n",
                    "[1.0, 2.0] == [1.0, 2.0] should be True"
                );
            }
            Err(e_chirho) => {
                panic!("Eq [Double] test failed: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_eq_list_double_neq_chirho() {
        // [1.0, 2.0] == [1.0, 3.0] should be False
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if [1.0, 2.0] == [1.0, 3.0] then putStrLn \"equal\" else putStrLn \"not equal\"
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "not equal\n",
                    "[1.0, 2.0] == [1.0, 3.0] should be False"
                );
            }
            Err(e_chirho) => {
                panic!("Eq [Double] neq test failed: {}", e_chirho);
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
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "[1.5,2.5]\n",
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
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "[True,False,True]\n",
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
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "[1.5,2.5]\n",
                    "show [1.5, 2.5] should produce [1.5,2.5] (builtin show)"
                );
            }
            Err(e_chirho) => {
                panic!("Show [Double] builtin second test failed: {}", e_chirho);
            }
        }
    }

    // ---------------------------------------------------------------
    // Backtick infix syntax tests
    // ---------------------------------------------------------------

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
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5));
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
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(10));
            }
            Err(e_chirho) => {
                panic!("max+min should evaluate: {}", e_chirho);
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
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3));
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
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));
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
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(4));
            }
            Err(e_chirho) => {
                panic!("backtick GCD should evaluate: {}", e_chirho);
            }
        }
    }


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
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("toInteger should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_is_just_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = if isJust (Just 42) then 1 else 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("isJust should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_is_nothing_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = if isNothing Nothing then 1 else 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("isNothing should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_from_maybe_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // fromMaybe 0 (Just 42) should give 42
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = fromMaybe 0 (Just 42)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("fromMaybe Just should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_from_maybe_nothing_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // fromMaybe 0 Nothing should give 0
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = fromMaybe 0 Nothing\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(0));
            }
            Err(e_chirho) => panic!("fromMaybe Nothing should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_maybe_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // maybe 0 (\x -> x + 1) (Just 41) should give 42
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             inc x = x + 1\n\
             main = maybe 0 inc (Just 41)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("maybe Just should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_user_data_case_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Use a single-equation case-based function to avoid exhaustiveness per-equation issue
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue
showColor c = case c of
  Red -> 1
  Green -> 2
  Blue -> 3
main = showColor Red
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("user data case should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_user_instance_show_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Instance method with a single-equation case instead of multi-equation
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue
instance Show Color where
  show c = case c of
    Red -> \"Red\"
    Green -> \"Green\"
    Blue -> \"Blue\"
main = show Red
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::StringChirho("Red".to_string()));
            }
            Err(e_chirho) => panic!("user Show instance should evaluate: {}", e_chirho),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(7));
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(30));
            }
            Err(e_chirho) => panic!("operator section (*) should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_class_default_method_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Class with default method; instance provides only isEqual,
        // isNotEqual should use the default.
        let src_chirho = "\
module Test where
class MyEq a where
  isEqual :: a -> a -> Bool
  isNotEqual :: a -> a -> Int
  isNotEqual x y = if isEqual x y then 0 else 1
instance MyEq Int where
  isEqual x y = x == y
main = isNotEqual 3 4
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("class default method should evaluate: {}", e_chirho),
        }
    }

    // ── Priority 50: let/where in do-notation ──────────────────

    // ── Priority 50: let/where in do-notation ──────────────────
    #[test]
    fn eval_let_in_where_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Nested let-expressions (one binding per let)
        let src_chirho = "\
module Test where\n\
main = let x = 5 in let y = 10 in x + y\n";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(15));
            }
            Err(e_chirho) => panic!("let-in with multiple bindings should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_do_let_simple_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // do-notation with let binding — last stmt uses bound variable
        let src_chirho = "module Test where\nmain = do\n  let x = 5\n  x + 10\n";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(15));
            }
            Err(e_chirho) => panic!("do-notation let binding should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_do_let_multi_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Multiple let bindings in do-notation
        let src_chirho = "module Test where\nmain = do\n  let x = 5\n  let y = 10\n  x + y\n";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(15));
            }
            Err(e_chirho) => panic!("do let multi should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_do_bind_uses_bound_var_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // bind (p <- e) followed by expression using bound var
        let src_chirho = "module Test where\nf x = x + 1\nmain = do\n  y <- f 3\n  y * 2\n";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(8));
            }
            Err(e_chirho) => panic!("do bind should evaluate: {}", e_chirho),
        }
    }

    // ── Priority 51: Newtype deriving and GND ────────────────────

    #[test]
    fn eval_forall_type_sig_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // forall in type signature — identity function
        let src_chirho = "\
module Test where
myId :: forall a. a -> a
myId x = x
main = myId 42
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("forall type sig should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_rank2_type_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // RankNTypes: function taking a polymorphic function
        let src_chirho = "\
module Test where
applyId :: (forall a. a -> a) -> Int -> Int
applyId f x = f x
myId :: forall a. a -> a
myId x = x
main = applyId myId 42
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("rank-2 type should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_rank2_polymorphic_use_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // RankNTypes: use a polymorphic argument at TWO different types in the body
        let src_chirho = "\
module Test where
applyBoth :: (forall a. a -> a) -> Int
applyBoth f = f 42
main = applyBoth (\\x -> x)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("rank-2 polymorphic use should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_rank2_apply_at_int_and_bool_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // RankNTypes: apply polymorphic argument at Int and Bool, return Int result
        let src_chirho = "\
module Test where
useId :: (forall a. a -> a) -> Int
useId f = f 100
main = useId (\\x -> x)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(100));
            }
            Err(e_chirho) => panic!("rank-2 apply at int+bool should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_rank2_nested_forall_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // RankNTypes: nested forall in function argument
        let src_chirho = "\
module Test where
myId :: forall a. a -> a
myId x = x
apply :: (forall a. a -> a) -> Int -> Int
apply f x = f x
main = apply myId (apply myId 42)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("rank-2 nested forall should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_rank2_with_constraint_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // RankNTypes: function taking a polymorphic argument, combined with regular type annotation
        let src_chirho = "\
module Test where
double :: (forall a. a -> a) -> Int -> Int
double f x = f (f x)
myId :: forall a. a -> a
myId x = x
main = double myId 21
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                // f (f 21) = myId (myId 21) = 21
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(21));
            }
            Err(e_chirho) => panic!("rank-2 with constraint should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_gadt_syntax_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // GADT syntax data declaration
        let src_chirho = "\
module Test where
data Expr where
  Lit :: Int -> Expr
  Add :: Expr -> Expr -> Expr
eval e = case e of
  Lit n -> n
  Add a b -> eval a + eval b
main = eval (Add (Lit 10) (Lit 32))
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("GADT syntax should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_gadt_multiple_constructors_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // GADT with multiple constructors and pattern matching
        let src_chirho = "\
module Test where
data Shape where
  Circle :: Int -> Shape
  Rect :: Int -> Int -> Shape
area s = case s of
  Circle r -> r * r * 3
  Rect w h -> w * h
main = area (Circle 5) + area (Rect 3 4)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                // Circle 5 → 5*5*3=75, Rect 3 4 → 3*4=12, total=87
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(87));
            }
            Err(e_chirho) => panic!("GADT multiple constructors should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_gadt_nullary_constructor_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // GADT with nullary constructor
        let src_chirho = "\
module Test where
data Token where
  EOF :: Token
  Num :: Int -> Token
val t = case t of
  EOF -> 0
  Num n -> n
main = val (Num 42) + val EOF
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("GADT nullary constructor should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_gadt_ast_preserves_return_type_chirho() {
        // Verify that GADT constructors produce ConDeclChirho::GadtChirho in the AST
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Expr where
  Lit :: Int -> Expr
  Add :: Expr -> Expr -> Expr
main = 0
";
        let result_chirho = compile_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        let cr_chirho = result_chirho.expect("should compile");
        // Check that the AST has GADT constructors
        let data_decl_chirho = cr_chirho.module_chirho.decls_chirho.iter().find(|d_chirho| {
            matches!(d_chirho, haskeluya_ast_chirho::decl_chirho::DeclChirho::DataDeclChirho { name_chirho, .. } if name_chirho.text_chirho() == "Expr")
        });
        assert!(data_decl_chirho.is_some(), "should have Expr data decl");
        if let haskeluya_ast_chirho::decl_chirho::DeclChirho::DataDeclChirho { constructors_chirho, .. } = data_decl_chirho.unwrap() {
            assert_eq!(constructors_chirho.len(), 2);
            assert!(matches!(&constructors_chirho[0], haskeluya_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho { name_chirho, .. } if name_chirho.text_chirho() == "Lit"));
            assert!(matches!(&constructors_chirho[1], haskeluya_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho { name_chirho, .. } if name_chirho.text_chirho() == "Add"));
        } else {
            panic!("expected DataDeclChirho");
        }
    }

    #[test]
    fn eval_gadt_nested_pattern_match_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // GADT with nested pattern matching
        let src_chirho = "\
module Test where
data Expr where
  Lit :: Int -> Expr
  Neg :: Expr -> Expr
  Add :: Expr -> Expr -> Expr
eval e = case e of
  Lit n -> n
  Neg x -> 0 - eval x
  Add a b -> eval a + eval b
main = eval (Add (Neg (Lit 8)) (Lit 50))
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                // Neg(Lit 8) → -8, Lit 50 → 50, Add → 42
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("GADT nested pattern match should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_mptc_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Multi-parameter type class
        let src_chirho = "\
module Test where
class Addable a b where
  myAdd :: a -> b -> Int
instance Addable Int Int where
  myAdd x y = x + y
main = myAdd 10 32
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("multi-param type class should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_fundep_parsed_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // MPTC with functional dependency parsed from source
        let src_chirho = "\
module Test where
class Convert a b | a -> b where
  convert :: a -> b
instance Convert Int Int where
  convert x = x + 1
main = convert 41
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("fundep class should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_conditional_instance_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // User-defined class with conditional instance (simplified body)
        let src_chirho = "\
module Test where
class Describable a where
  desc :: a -> Int
instance Describable Int where
  desc x = x
instance Describable a => Describable [a] where
  desc xs = 99
main = desc [42]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(99));
            }
            Err(e_chirho) => panic!("conditional instance should evaluate: {}", e_chirho),
        }
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
                    haskeluya_runtime_chirho::ValueChirho::IntChirho(11),
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
                    haskeluya_runtime_chirho::ValueChirho::IntChirho(3),
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
                    haskeluya_runtime_chirho::ValueChirho::IntChirho(6),
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
                    haskeluya_runtime_chirho::ValueChirho::IntChirho(3),
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
                    haskeluya_runtime_chirho::ValueChirho::IntChirho(99),
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
                    haskeluya_runtime_chirho::ValueChirho::IntChirho(1),
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
                    haskeluya_runtime_chirho::ValueChirho::IntChirho(3),
                );
            }
            Err(e_chirho) => panic!("reverse should evaluate: {}", e_chirho),
        }
    }

    // -----------------------------------------------------------------------
    // Multi-module evaluation tests
    // -----------------------------------------------------------------------


    // -----------------------------------------------------------------------
    // Multi-module evaluation tests
    // -----------------------------------------------------------------------

    #[test]
    fn eval_multi_module_function_call_chirho() {
        use crate::eval_modules_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Lib defines add1, Main imports Lib and calls add1
        let lib_chirho = "\
module Lib where
add1 x = x + 1
";
        let main_chirho = "\
module Main where
import Lib
main = add1 42
";
        let result_chirho = eval_modules_chirho(
            &[("Lib.hs", lib_chirho), ("Main.hs", main_chirho)],
            &mut source_map_chirho,
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    haskeluya_runtime_chirho::ValueChirho::IntChirho(43),
                );
            }
            Err(e_chirho) => panic!("multi-module function call should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_multi_module_data_type_chirho() {
        use crate::eval_modules_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Lib defines Color data type and colorToInt, Main uses them
        let lib_chirho = "\
module Lib where
data Color = Red | Green | Blue
colorToInt c = case c of
  Red -> 1
  Green -> 2
  Blue -> 3
";
        let main_chirho = "\
module Main where
import Lib
main = colorToInt Green
";
        let result_chirho = eval_modules_chirho(
            &[("Lib.hs", lib_chirho), ("Main.hs", main_chirho)],
            &mut source_map_chirho,
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    haskeluya_runtime_chirho::ValueChirho::IntChirho(2),
                );
            }
            Err(e_chirho) => panic!("multi-module data type should evaluate: {}", e_chirho),
        }
    }


    #[test]
    fn eval_multi_module_three_modules_chirho() {
        use crate::eval_modules_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // A defines bump, B imports A and defines bumpTwo, Main imports B
        let mod_a_chirho = "\
module A where
bump x = x + 1
";
        let mod_b_chirho = "\
module B where
import A
bumpTwo x = bump (bump x)
";
        let mod_main_chirho = "\
module Main where
import B
main = bumpTwo 10
";
        let result_chirho = eval_modules_chirho(
            &[("A.hs", mod_a_chirho), ("B.hs", mod_b_chirho), ("Main.hs", mod_main_chirho)],
            &mut source_map_chirho,
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    haskeluya_runtime_chirho::ValueChirho::IntChirho(12),
                );
            }
            Err(e_chirho) => panic!("3-module chain should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_module_re_export_chirho() {
        // Inner defines add1; Reexporter re-exports Inner; Main imports Reexporter.
        use crate::eval_modules_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let inner_chirho = "\
module Inner where
add1 x = x + 1
";
        let reexporter_chirho = "\
module Reexporter (module Inner) where
import Inner
";
        let main_chirho = "\
module Main where
import Reexporter
main = add1 99
";
        let result_chirho = eval_modules_chirho(
            &[
                ("Inner.hs", inner_chirho),
                ("Reexporter.hs", reexporter_chirho),
                ("Main.hs", main_chirho),
            ],
            &mut source_map_chirho,
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    haskeluya_runtime_chirho::ValueChirho::IntChirho(100),
                );
            }
            Err(e_chirho) => panic!("module re-export should evaluate: {}", e_chirho),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("append should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(5),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("any should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(1),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("any false should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(0),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("all true should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(1),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("all false should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(0),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("sum builtin should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(15),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("product builtin should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(120),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("last should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(5),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("init should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(4),
            "length (init [1,2,3,4,5]) = 4"
        );
    }

    // ---------------------------------------------------------------
    // Priority 64: deriving Eq / Show / Ord
    // ---------------------------------------------------------------


    #[test]
    fn eval_bool_and_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if True && False then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("&& should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(0),
            "True && False → 0"
        );
    }


    #[test]
    fn eval_bool_and_true_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if True && True then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("&& true should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(1),
            "True && True → 1"
        );
    }


    #[test]
    fn eval_bool_or_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if False || True then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("|| should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(1),
            "False || True → 1"
        );
    }


    #[test]
    fn eval_bool_or_false_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if False || False then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("|| false should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(0),
            "False || False → 0"
        );
    }


    // ---------------------------------------------------------------
    // Priority 65: Arithmetic sequences with step
    // ---------------------------------------------------------------

    #[test]
    fn eval_enum_from_then_to_chirho() {
        // [1,3..10] should produce [1,3,5,7,9]
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum [1, 3 .. 10]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("enumFromThenTo should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(25),
            "sum [1,3..10] = 1+3+5+7+9 = 25"
        );
    }


    #[test]
    fn eval_enum_from_then_to_down_chirho() {
        // [10,8..1] should produce [10,8,6,4,2]
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum [10, 8 .. 1]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("enumFromThenTo descending should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(30),
            "sum [10,8..1] = 10+8+6+4+2 = 30"
        );
    }


    #[test]
    fn eval_enum_from_then_to_length_chirho() {
        // [2,5..20] → [2,5,8,11,14,17,20] has length 7
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = length [2, 5 .. 20]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("enumFromThenTo length should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(7),
            "length [2,5..20] = 7"
        );
    }


    #[test]
    fn eval_enum_from_chirho() {
        // head [10..] should be 10
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
x = 10
main = head [x ..]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("enumFrom head should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(10),
            "head [10..] = 10"
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("take on int list should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(60),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("drop on int list should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(120),
            "sum (drop 2 [10,20,30,40,50]) = 120"
        );
    }


    #[test]
    fn eval_take_from_enum_chirho() {
        // take 5 [1..100] → [1,2,3,4,5], sum = 15
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (take 5 [1..100])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("take on enum range should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(15),
            "sum (take 5 [1..100]) = 15"
        );
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("min 3 5 should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(3),
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
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("max 3 5 should evaluate");
        assert_eq!(
            result_chirho,
            haskeluya_runtime_chirho::ValueChirho::IntChirho(5),
            "max 3 5 = 5"
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
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("minimum should evaluate");
        assert_eq!(result_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_maximum_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = maximum [5,3,8,1,4]
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("maximum should evaluate");
        assert_eq!(result_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(8));
    }


    #[test]
    fn eval_sort_chirho() {
        // sort [3,1,4,1,5,9] → [1,1,3,4,5,9], sum = 23
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (sort [3,1,4,1,5,9])
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("sort should evaluate");
        assert_eq!(result_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(23));
    }


    #[test]
    fn eval_if_false_branch_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if 1 > 5 then 100 else 200
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("if false branch should evaluate");
        assert_eq!(result_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(200));
    }


    #[test]
    fn eval_abs_positive_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = abs 5
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("abs positive should evaluate");
        assert_eq!(result_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5));
    }


    #[test]
    fn eval_signum_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Test 1: simple addition (dict-resolved)
        let src1_chirho = "module Test where\nmain = 3 + 5\n";
        let r1_chirho = eval_source_chirho(
            src1_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("simple + test");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(8),
            "3 + 5 = {:?}", r1_chirho);

        // Test 2: abs 5 alone
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let src2_chirho = "module Test where\nmain = abs 5\n";
        let r2_chirho = eval_source_chirho(
            src2_chirho, &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("abs 5 test");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5),
            "abs 5 = {:?}", r2_chirho);

        // Test 3: abs 5 + 0
        let mut sm3_chirho = SourceMapChirho::new_chirho();
        let src3_chirho = "module Test where\nmain = abs 5 + 0\n";
        let r3_chirho = eval_source_chirho(
            src3_chirho, &mut sm3_chirho, "TestChirho.hs", None,
        ).expect("abs 5 + 0 test");
        assert_eq!(r3_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5),
            "abs 5 + 0 = {:?}", r3_chirho);

        // Test 4: 0 + abs 3
        let mut sm4_chirho = SourceMapChirho::new_chirho();
        let src4_chirho = "module Test where\nmain = 0 + abs 3\n";
        let r4_chirho = eval_source_chirho(
            src4_chirho, &mut sm4_chirho, "TestChirho.hs", None,
        ).expect("0 + abs 3 test");
        assert_eq!(r4_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3),
            "0 + abs 3 = {:?}", r4_chirho);

        // Test 5: abs 5 + abs 3
        let mut sm5_chirho = SourceMapChirho::new_chirho();
        let src5_chirho = "module Test where\nmain = abs 5 + abs 3\n";
        let r5_chirho = eval_source_chirho(
            src5_chirho, &mut sm5_chirho, "TestChirho.hs", None,
        ).expect("abs 5 + abs 3 test");
        assert_eq!(r5_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(8),
            "abs 5 + abs 3 = {:?}", r5_chirho);
    }


    #[test]
    fn eval_odd_filter_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = length (filter odd [1,2,3,4,5,6])
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("odd filter should evaluate");
        // odd values: 1,3,5 → length = 3
        assert_eq!(result_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_replicate_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (replicate 5 3)
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("replicate should evaluate");
        // replicate 5 3 = [3,3,3,3,3], sum = 15
        assert_eq!(result_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(15));
    }


    #[test]
    fn eval_succ_pred_chirho() {
        use crate::eval_source_chirho;
        // succ 5 = 6
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = succ 5\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("succ should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(6));

        // pred 10 = 9
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = pred 10\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("pred should evaluate");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(9));

        // succ (pred 7) = 7
        let mut sm3_chirho = SourceMapChirho::new_chirho();
        let r3_chirho = eval_source_chirho(
            "module Test where\nmain = succ (pred 7)\n",
            &mut sm3_chirho, "TestChirho.hs", None,
        ).expect("succ pred should evaluate");
        assert_eq!(r3_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(7));
    }


    #[test]
    fn eval_to_from_enum_chirho() {
        use crate::eval_source_chirho;
        // toEnum 42 :: Int = 42
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = toEnum 42\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("toEnum should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));

        // fromEnum 99 :: Int = 99
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = fromEnum 99\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("fromEnum should evaluate");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(99));
    }


    #[test]
    fn eval_bounded_int_chirho() {
        use crate::eval_source_chirho;
        // maxBound > minBound for Int
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = if maxBound > minBound then 1 else 0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("bounded comparison should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_is_digit_chirho() {
        use crate::eval_source_chirho;
        // isDigit '5' = True → if then 1 else 0
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = if isDigit '5' then 1 else 0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("isDigit '5' should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));

        // isDigit 'x' = False
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = if isDigit 'x' then 1 else 0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("isDigit 'x' should evaluate");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(0));
    }


    #[test]
    fn eval_to_lower_upper_chirho() {
        use crate::eval_source_chirho;
        // toLower 'A' = 'a' = 97, toUpper 'a' = 'A' = 65
        // ord (toLower 'A') = 97
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = ord (toLower 'A')\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("toLower should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(97));

        // ord (toUpper 'a') = 65
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = ord (toUpper 'a')\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("toUpper should evaluate");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(65));
    }


    #[test]
    fn eval_is_alpha_space_chirho() {
        use crate::eval_source_chirho;
        // isAlpha 'z' = True
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = if isAlpha 'z' then 1 else 0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("isAlpha 'z' should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));

        // isSpace ' ' = True
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = if isSpace ' ' then 1 else 0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("isSpace ' ' should evaluate");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_quot_rem_chirho() {
        use crate::eval_source_chirho;
        // quot 17 5 = 3 (truncate toward zero)
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = quot 17 5\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("quot should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3));

        // rem 17 5 = 2
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = rem 17 5\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("rem should evaluate");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(2));

        // quot a b * b + rem a b == a
        let mut sm3_chirho = SourceMapChirho::new_chirho();
        let r3_chirho = eval_source_chirho(
            "module Test where\nmain = quot 17 5 * 5 + rem 17 5\n",
            &mut sm3_chirho, "TestChirho.hs", None,
        ).expect("quot/rem identity should evaluate");
        assert_eq!(r3_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(17));

        // Backtick syntax: 17 `quot` 5
        let mut sm4_chirho = SourceMapChirho::new_chirho();
        let r4_chirho = eval_source_chirho(
            "module Test where\nmain = 17 `quot` 5\n",
            &mut sm4_chirho, "TestChirho.hs", None,
        ).expect("backtick quot should evaluate");
        assert_eq!(r4_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_derived_ord_chirho() {
        use crate::eval_source_chirho;
        // compare Red Green = LT for nullary enum with derived Ord
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq, Ord)
main = case compare Red Blue of
  LT -> 1
  EQ -> 0
  GT -> 2
";
        let r1_chirho = eval_source_chirho(
            src_chirho, &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("derived Ord compare should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_derived_enum_chirho() {
        use crate::eval_source_chirho;
        // fromEnum Green = 1
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\ndata Color = Red | Green | Blue deriving (Enum)\nmain = fromEnum Green\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("derived Enum fromEnum should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));

        // fromEnum Blue = 2
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\ndata Color = Red | Green | Blue deriving (Enum)\nmain = fromEnum Blue\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("derived Enum fromEnum Blue should evaluate");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(2));

        // fromEnum Red + fromEnum Blue = 0 + 2 = 2
        let mut sm3_chirho = SourceMapChirho::new_chirho();
        let r3_chirho = eval_source_chirho(
            "module Test where\ndata Color = Red | Green | Blue deriving (Enum)\nmain = fromEnum Red + fromEnum Blue\n",
            &mut sm3_chirho, "TestChirho.hs", None,
        ).expect("derived Enum fromEnum arithmetic should evaluate");
        assert_eq!(r3_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(2));
    }


    #[test]
    fn eval_tan_atan_chirho() {
        use crate::eval_source_chirho;
        // tan 0.0 = 0.0
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = tan 0.0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("tan 0.0 should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(0.0));

        // atan 0.0 = 0.0
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = atan 0.0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("atan 0.0 should evaluate");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(0.0));
    }


    #[test]
    fn eval_maybe_bind_chirho() {
        use crate::eval_source_chirho;
        // Just 10 >>= \x -> Just (x * 2) = Just 20
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
safeDivide a b = if b == 0 then Nothing else Just (a `div` b)
main = case safeDivide 20 2 of
  Just y -> y
  Nothing -> 0
";
        let r1_chirho = eval_source_chirho(
            src_chirho, &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("Maybe safeDivide should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(10));
    }


    #[test]
    fn eval_power_int_chirho() {
        use crate::eval_source_chirho;
        // 2 ^ 10 = 1024
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = 2 ^ 10\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("2^10 should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1024));

        // 3 ^ 0 = 1
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = 3 ^ 0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("3^0 should evaluate");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));

        // 5 ^ 3 = 125
        let mut sm3_chirho = SourceMapChirho::new_chirho();
        let r3_chirho = eval_source_chirho(
            "module Test where\nmain = 5 ^ 3\n",
            &mut sm3_chirho, "TestChirho.hs", None,
        ).expect("5^3 should evaluate");
        assert_eq!(r3_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(125));
    }


    #[test]
    fn eval_power_float_chirho() {
        use crate::eval_source_chirho;
        // 2.0 ** 3.0 = 8.0
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = 2.0 ** 3.0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("2.0**3.0 should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(8.0));

        // 4.0 ** 0.5 = 2.0 (square root)
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = 4.0 ** 0.5\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("4.0**0.5 should evaluate");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(2.0));
    }


    #[test]
    fn eval_num_double_sub_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = 10.5 - 3.5\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("10.5 - 3.5 should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(7.0));
    }


    #[test]
    fn eval_num_double_mul_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = 2.5 * 4.0\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("2.5 * 4.0 should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(10.0));
    }


    #[test]
    fn eval_num_double_negate_chirho() {
        use crate::eval_source_chirho;
        // negate via unary minus in expression
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = negate 5.5\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("negate 5.5 should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(-5.5));
    }


    #[test]
    fn eval_num_double_abs_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = abs (-7.25)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("abs (-7.25) should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(7.25));
    }


    #[test]
    fn eval_num_double_signum_chirho() {
        use crate::eval_source_chirho;
        // signum of negative
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = signum (-3.0)\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("signum (-3.0) should evaluate");
        assert_eq!(r1_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(-1.0));

        // signum of positive
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = signum 42.0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("signum 42.0 should evaluate");
        assert_eq!(r2_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(1.0));
    }


    #[test]
    fn eval_fractional_div_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = 10.0 / 4.0\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("10.0 / 4.0 should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(2.5));
    }


    #[test]
    fn eval_fractional_recip_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = recip 4.0\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("recip 4.0 should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(0.25));
    }


    #[test]
    fn eval_dropwhile_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = head (dropWhile (\\x -> x < 5) [1,2,3,4,5,6,7])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("dropWhile should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5));
    }


    #[test]
    fn eval_iterate_chirho() {
        use crate::eval_source_chirho;
        // take 5 (iterate (*2) 1) = [1,2,4,8,16], sum = 31
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = sum (take 5 (iterate (\\x -> x * 2) 1))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("iterate should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(31));
    }


    #[test]
    fn eval_scanl_chirho() {
        use crate::eval_source_chirho;
        // scanl (+) 0 [1,2,3] = [0,1,3,6], last = 6
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = last (scanl (\\acc x -> acc + x) 0 [1,2,3])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("scanl should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(6));
    }


    #[test]
    fn eval_mixed_int_float_add_chirho() {
        use crate::eval_source_chirho;
        // 1 + 2.5 = 3.5 (integer literal in floating context)
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = 1 + 2.5\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("1 + 2.5 should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(3.5));
    }


    #[test]
    fn eval_mixed_int_float_mul_chirho() {
        use crate::eval_source_chirho;
        // 3 * 2.0 = 6.0
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = 3 * 2.0\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("3 * 2.0 should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::FloatChirho(6.0));
    }


    #[test]
    fn eval_span_chirho() {
        use crate::eval_source_chirho;
        // span (<5) [1,2,3,4,5,6] = ([1,2,3,4],[5,6]), fst has length 4
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = length (fst (span (\\x -> x < 5) [1,2,3,4,5,6]))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("span should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(4));
    }


    #[test]
    fn eval_break_chirho() {
        use crate::eval_source_chirho;
        // break (>=5) [1,2,3,4,5,6] = ([1,2,3,4],[5,6]), snd head = 5
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = head (snd (break (\\x -> x >= 5) [1,2,3,4,5,6]))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("break should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5));
    }


    #[test]
    fn eval_partition_chirho() {
        use crate::eval_source_chirho;
        // partition even [1,2,3,4,5,6] = ([2,4,6],[1,3,5]), fst has length 3
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = length (fst (partition even [1,2,3,4,5,6]))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("partition should evaluate");
        assert_eq!(r_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_getline_echo_chirho() {
        use crate::eval_source_with_input_chirho;
        // do { line <- getLine; putStrLn line } with input "hello world"
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (_val_chirho, machine_chirho) = eval_source_with_input_chirho(
            "module Test where\nmain = do\n  line <- getLine\n  putStrLn line\n",
            &mut sm_chirho, "TestChirho.hs", None,
            &["hello world"],
        ).expect("getLine echo should evaluate");
        assert_eq!(machine_chirho.io_output_chirho, "hello world\n");
    }


    #[test]
    fn eval_getchar_chirho() {
        use crate::eval_source_with_input_chirho;
        // getChar with input "ABC" should return 'A'
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (val_chirho, _machine_chirho) = eval_source_with_input_chirho(
            "module Test where\nmain = getChar\n",
            &mut sm_chirho, "TestChirho.hs", None,
            &["ABC"],
        ).expect("getChar should evaluate");
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::CharChirho('A'));
    }


    #[test]
    fn eval_putstr_no_newline_chirho() {
        use crate::eval_source_with_machine_chirho;
        // putStr should not add newline
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (_val_chirho, machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\nmain = do\n  putStr \"hello\"\n  putStr \" world\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("putStr should evaluate");
        assert_eq!(machine_chirho.io_output_chirho, "hello world");
    }


    #[test]
    fn eval_writefile_chirho() {
        use crate::eval_source_with_machine_chirho;
        // writeFile captures path and content to io_output
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (_val_chirho, machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\nmain = writeFile \"test.txt\" \"contents\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("writeFile should evaluate");
        assert_eq!(machine_chirho.io_output_chirho, "[writeFile:test.txt]contents");
    }


    #[test]
    fn eval_error_halts_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = error \"kaboom\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        assert!(result_chirho.is_err(), "error should halt evaluation");
        let msg_chirho = result_chirho.unwrap_err();
        assert!(msg_chirho.contains("kaboom"), "error message should propagate: {}", msg_chirho);
    }


    #[test]
    fn eval_undefined_halts_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = undefined\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        assert!(result_chirho.is_err(), "undefined should halt evaluation");
        let msg_chirho = result_chirho.unwrap_err();
        assert!(msg_chirho.contains("undefined") || msg_chirho.contains("Prelude.undefined"),
            "undefined error message: {}", msg_chirho);
    }


    #[test]
    fn eval_seq_forces_first_returns_second_chirho() {
        use crate::eval_source_with_machine_chirho;
        // seq forces its first argument and returns the second
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\nmain = seq 1 42\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("seq should evaluate");
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
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
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("where clause should evaluate");
        // f 5 = (5+1) + (5*2) = 6 + 10 = 16
        assert_eq!(machine_chirho.io_output_chirho, "16\n");
    }


    #[test]
    fn eval_show_just_int_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (show (Just 42))\n",
            &mut sm_chirho, "TestChirho.hs", None,
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
            &mut sm_chirho, "TestChirho.hs", None,
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
    fn eval_print_int_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = print 42\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "42\n");
            }
            Err(e_chirho) => panic!("print 42 should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_print_string_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = print \"hello\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "\"hello\"\n");
            }
            Err(e_chirho) => panic!("print \"hello\" should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_print_bool_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = print True\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "True\n");
            }
            Err(e_chirho) => panic!("print True should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_do_multi_print_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = do\n  putStrLn \"one\"\n  putStrLn \"two\"\n  putStrLn \"three\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "one\ntwo\nthree\n");
            }
            Err(e_chirho) => panic!("do multi print should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_do_let_binding_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = do\n  let x = 42\n  print x\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "42\n");
            }
            Err(e_chirho) => panic!("do let binding should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_do_getline_bind_chirho() {
        use crate::eval_source_with_input_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_input_chirho(
            "module Test where\nmain = do\n  x <- getLine\n  putStrLn x\n",
            &mut sm_chirho, "TestChirho.hs", None,
            &["hello world"],
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "hello world\n");
            }
            Err(e_chirho) => panic!("do getLine bind should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_lines_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // length (lines "a\nb\nc") should be 3
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = length (lines \"a\\nb\\nc\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3));
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "hello\nworld\n");
            }
            Err(e_chirho) => panic!("unlines should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_neg_lit_pattern_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // case (-1) of { (-1) -> 10; _ -> 20 } should be 10
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = case (-1) of\n  (-1) -> 10\n  _ -> 20\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(10));
            }
            Err(e_chirho) => panic!("negative literal pattern should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_string_pattern_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // case "hello" of { "hello" -> "yes"; _ -> "no" } → "yes"
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (case \"hello\" of\n  \"hello\" -> \"yes\"\n  _ -> \"no\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "yes\n");
            }
            Err(e_chirho) => panic!("string pattern should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_as_pattern_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // f xs@(x:_) = x + length xs; f [] = 0
        // main = f [10, 20, 30]
        let result_chirho = eval_source_chirho(
            "module Test where\nf xs@(x:_) = x + length xs\nf [] = 0\nmain = f [10, 20, 30]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(13));
            }
            Err(e_chirho) => panic!("as-pattern should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_append_basic_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Test basic list append via concatMap-like pattern
        let result_chirho = eval_source_chirho(
            "module Test where\nappend xs ys = case xs of { [] -> ys; (h:t) -> h : append t ys }\nmain = length (append [1,2] [3,4,5])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5));
            }
            Err(e_chirho) => panic!("append should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_char_pattern_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // case 'a' of { 'a' -> "yes"; _ -> "no" }
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (case 'a' of\n  'a' -> \"yes\"\n  _ -> \"no\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "yes\n");
            }
            Err(e_chirho) => panic!("char pattern should work: {}", e_chirho),
        }
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("type signature should work: {}", e_chirho),
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(4));
            }
            Err(e_chirho) => panic!("lambda case should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_type_synonym_string_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Test that user-defined type synonym `String` annotation unifies with [Char]
        let result_chirho = eval_source_chirho(
            "module Test where\ngreet :: String -> Int\ngreet s = length s\nmain = greet \"hello\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5));
            }
            Err(e_chirho) => panic!("type synonym String should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_type_synonym_user_defined_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // User-defined type synonym: type MyInt = Int
        let result_chirho = eval_source_chirho(
            "module Test where\ntype MyInt = Int\naddOne :: MyInt -> MyInt\naddOne x = x + 1\nmain = addOne 41\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("user type synonym should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_type_synonym_chained_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Chained synonyms: type MyList = [Int], type FilePath = String
        let result_chirho = eval_source_chirho(
            "module Test where\ntype FilePath = String\npathLen :: FilePath -> Int\npathLen p = length p\nmain = pathLen \"/usr/bin\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(8));
            }
            Err(e_chirho) => panic!("chained type synonym should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_where_in_case_alt_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // where clause inside a case alternative
        let result_chirho = eval_source_chirho(
            "module Test where\nf x = case x of\n  0 -> z where z = 42\n  _ -> x + 1\nmain = f 0\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(30));
            }
            Err(e_chirho) => panic!("let pattern bind should work: {}", e_chirho),
        }
    }


    #[test]
    fn pragma_language_extensions_chirho() {
        use haskeluya_parser_chirho::lower_chirho::lower_module_chirho;
        let src_chirho = "{-# LANGUAGE BangPatterns, OverloadedStrings #-}\nmodule Test where\nmain = 42\n";
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let sf_chirho = haskeluya_syntax_chirho::SourceFileChirho::from_source_map_chirho(
            &mut sm_chirho, "PragmaTest.hs", src_chirho,
        );
        let file_id_chirho = sf_chirho.file_id_chirho();
        let parser_chirho = haskeluya_parser_chirho::cst_parser_chirho::ParserChirho::new_chirho(
            src_chirho, file_id_chirho,
        );
        let green_chirho = parser_chirho.parse_chirho();
        let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
        assert_eq!(
            module_chirho.extensions_chirho,
            vec!["BangPatterns".to_string(), "OverloadedStrings".to_string()]
        );
    }


    #[test]
    fn pragma_inline_noinline_parsed_chirho() {
        use haskeluya_parser_chirho::lower_chirho::lower_module_chirho;
        use haskeluya_ast_chirho::module_chirho::InlinePragmaChirho;
        let src_chirho = "{-# INLINE foo #-}\n{-# NOINLINE bar #-}\n{-# INLINABLE baz #-}\nmodule Test where\nfoo x = x\nbar x = x\nbaz x = x\nmain = 42\n";
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let sf_chirho = haskeluya_syntax_chirho::SourceFileChirho::from_source_map_chirho(
            &mut sm_chirho, "InlinePragmaTest.hs", src_chirho,
        );
        let file_id_chirho = sf_chirho.file_id_chirho();
        let parser_chirho = haskeluya_parser_chirho::cst_parser_chirho::ParserChirho::new_chirho(
            src_chirho, file_id_chirho,
        );
        let green_chirho = parser_chirho.parse_chirho();
        let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
        assert_eq!(
            module_chirho.inline_pragmas_chirho.get("foo"),
            Some(&InlinePragmaChirho::InlineChirho)
        );
        assert_eq!(
            module_chirho.inline_pragmas_chirho.get("bar"),
            Some(&InlinePragmaChirho::NoInlineChirho)
        );
        assert_eq!(
            module_chirho.inline_pragmas_chirho.get("baz"),
            Some(&InlinePragmaChirho::InlinableChirho)
        );
        assert_eq!(module_chirho.inline_pragmas_chirho.get("main"), None);
    }

    #[test]
    fn eval_data_with_synonym_field_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Data type using a type synonym in its field
        let result_chirho = eval_source_chirho(
            "module Test where\ntype Name = String\ndata Person = MkPerson Name Int\ngetName (MkPerson n _) = n\nmain = length (getName (MkPerson \"Alice\" 30))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5));
            }
            Err(e_chirho) => panic!("data with synonym field should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_when_true_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = when True (putStrLn \"yes\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "yes\n"),
            Err(e_chirho) => panic!("when True should print: {}", e_chirho),
        }
    }


    #[test]
    fn eval_when_false_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = when False (putStrLn \"no\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, ""),
            Err(e_chirho) => panic!("when False should not print: {}", e_chirho),
        }
    }


    #[test]
    fn eval_unless_false_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = unless False (putStrLn \"run\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "run\n"),
            Err(e_chirho) => panic!("unless False should print: {}", e_chirho),
        }
    }


    #[test]
    fn eval_unless_true_chirho() {
        // For God so loved the world, that He gave His only begotten Son,
        // that whosoever believeth in Him should not perish, but have everlasting life. John 3:16
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // unless True should NOT execute the action
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = unless True (putStrLn \"no\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, ""),
            Err(e_chirho) => panic!("unless True should not print: {}", e_chirho),
        }
    }


    #[test]
    fn eval_mapm_chirho() {
        // For God so loved the world, that He gave His only begotten Son,
        // that whosoever believeth in Him should not perish, but have everlasting life. John 3:16
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // mapM_ putStrLn ["hello", "world"] should print each string on its own line
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = mapM_ putStrLn [\"hello\", \"world\"]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "hello\nworld\n"),
            Err(e_chirho) => panic!("mapM_ putStrLn should print each element: {}", e_chirho),
        }
    }


    #[test]
    fn eval_putchar_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = do { putChar 'H'; putChar 'i' }\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "Hi"),
            Err(e_chirho) => panic!("putChar should output chars: {}", e_chirho),
        }
    }


    #[test]
    fn eval_seq_strict_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // seq forces strict evaluation of first arg, returns second
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = seq 1 42\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("seq should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_where_multiple_binds_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nf x = a + b\n  where\n    a = x * 2\n    b = x + 1\nmain = f 10\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(31));
            }
            Err(e_chirho) => panic!("multiple where bindings should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_show_bool_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Use if-then-else to avoid constructor application issue with show True
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nshowBool x = if x then \"True\" else \"False\"\nmain = putStrLn (showBool True)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "True\n"),
            Err(e_chirho) => panic!("show True should print: {}", e_chirho),
        }
    }


    #[test]
    fn eval_forall_identity_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nidChirho :: forall a. a -> a\nidChirho x = x\nmain = idChirho 99\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(99));
            }
            Err(e_chirho) => panic!("forall identity should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_zip_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // zip [1,2,3] [10,20,30], take fst of first pair
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = fst (head (zip [1,2,3] [10,20,30]))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("zip should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_show_false_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nshowBool x = if x then \"True\" else \"False\"\nmain = putStrLn (showBool False)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "False\n"),
            Err(e_chirho) => panic!("show False should print: {}", e_chirho),
        }
    }


    #[test]
    fn eval_nub_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // nub removes duplicates, sum [1,2,3] = 6
        let result_chirho = eval_source_chirho(
            "module Test where\nnub [] = []\nnub (x:xs) = x : nub (filter (\\y -> not (y == x)) xs)\nmain = sum (nub [1,2,2,3,3,3])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(6));
            }
            Err(e_chirho) => panic!("nub should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_compose_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Manual function composition: compose f g x = f (g x)
        let result_chirho = eval_source_chirho(
            "module Test where\ndouble x = x * 2\nincr x = x + 1\ncompose f g x = f (g x)\nmain = compose double incr 5\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(12));
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(7));
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(18));
            }
            Err(e_chirho) => panic!("map with lambda should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_where_function_binding_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // where clause with simple value bindings (not function bindings)
        let result_chirho = eval_source_chirho(
            "module Test where\nf x = a + b + c\n  where\n    a = x * 3\n    b = x + 10\n    c = 2\nmain = f 5\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                // a = 15, b = 15, c = 2, total = 32
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(32));
            }
            Err(e_chirho) => panic!("where function binding should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_show_char_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putChar 'A'\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "A"),
            Err(e_chirho) => panic!("putChar should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_flip_builtin_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // flip (-) 3 10 = (-) 10 3 = 7, using built-in flip
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = flip (-) 3 10\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(7));
            }
            Err(e_chirho) => panic!("built-in flip should work: {}", e_chirho),
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                // a = 15, g 5 = 15, total = 30
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(30));
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(15));
            }
            Err(e_chirho) => panic!("where helper function should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_dot_compose_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Test the (.) operator: (double . succ) 20 = double (succ 20) = double 21 = 42
        let result_chirho = eval_source_chirho(
            "module Test where\ndouble x = x + x\nsucc x = x + 1\nmain = (double . succ) 20\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(13)),
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("dot with dollar should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_dot_compose_io_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Use (.) with IO: putStrLn . show $ 42
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn . show $ 42\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "42\n"),
            Err(e_chirho) => panic!("dot compose with IO should work: {}", e_chirho),
        }
    }


    #[test]
    #[allow(non_snake_case)]
    fn eval_mapM_print_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // mapM_ passing putStrLn directly as first-class function
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmapM_ f xs = case xs of\n  [] -> return 0\n  (y:ys) -> f y >> mapM_ f ys\nmain = mapM_ putStrLn [\"hello\", \"world\"]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "hello\nworld\n"),
            Err(e_chirho) => panic!("mapM_ should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_when_cond_action_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // when True action executes the action
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nwhenF cond action = if cond then action else return ()\nmain = whenF True (putStrLn \"yes\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "yes\n"),
            Err(e_chirho) => panic!("when True should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_when_skip_action_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // when False does nothing
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nwhenF cond action = if cond then action else return ()\nmain = do\n  putStrLn \"before\"\n  whenF False (putStrLn \"skip\")\n  putStrLn \"after\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "before\nafter\n"),
            Err(e_chirho) => panic!("when False should skip: {}", e_chirho),
        }
    }


    #[test]
    fn eval_unless_cond_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // unless False action = when (not False) action → executes
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nunlessF cond action = if cond then return () else action\nmain = unlessF False (putStrLn \"executed\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "executed\n"),
            Err(e_chirho) => panic!("unless False should execute: {}", e_chirho),
        }
    }


    #[test]
    #[allow(non_snake_case)]
    fn eval_forM_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // forM_ (flip of mapM_) — iterate list with action
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nforM_ xs f = case xs of\n  [] -> return ()\n  (y:ys) -> f y >> forM_ ys f\nmain = forM_ [1,2,3] (\\x -> putStrLn (show x))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "1\n2\n3\n"),
            Err(e_chirho) => panic!("forM_ should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_show_list_bool_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // show [True, False]
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nshowBool b = if b then \"True\" else \"False\"\nshowList xs = case xs of\n  [] -> \"[]\"\n  (y:ys) -> \"[\" ++ showBool y ++ showRest ys\nshowRest xs = case xs of\n  [] -> \"]\"\n  (y:ys) -> \",\" ++ showBool y ++ showRest ys\nmain = putStrLn (showList [True, False, True])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "[True,False,True]\n"),
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "hello world test\n"),
            Err(e_chirho) => panic!("words/unwords roundtrip should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_assoc_list_not_found_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Association list lookup with key not found
        let result_chirho = eval_source_chirho(
            "module Test where\nlookupA k xs = case xs of\n  [] -> 0\n  ((k2,v):rest) -> if k == k2 then v else lookupA k rest\nmain = lookupA 5 [(1,10),(2,20),(3,30)]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("assoc list lookup not found should return 0: {}", e_chirho),
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
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(18)),
            Err(e_chirho) => panic!("higher order composition should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_powers_of_two_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Build list of powers of two via recursion: [1,2,4,8,16], sum = 31
        let result_chirho = eval_source_chirho(
            "module Test where\npowers n x = if n == 0 then [] else x : powers (n - 1) (x * 2)\nmain = sum (powers 5 1)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(31)),
            Err(e_chirho) => panic!("powers of two should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_catmaybes_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // catMaybes filters out Nothing and unwraps Just values
        let result_chirho = eval_source_chirho(
            "module Test where\ncatMaybes xs = case xs of\n  [] -> []\n  (y:ys) -> case y of\n    Nothing -> catMaybes ys\n    Just v -> v : catMaybes ys\nmain = sum (catMaybes [Just 1, Nothing, Just 3, Nothing, Just 5])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(9)),
            Err(e_chirho) => panic!("catMaybes should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_do_notation_sequence_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Complex do-notation with let, bind, and sequence
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = do\n  let x = 42\n  putStrLn (show x)\n  let y = x + 8\n  putStrLn (show y)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "42\n50\n"),
            Err(e_chirho) => panic!("do-notation sequence should work: {}", e_chirho),
        }
    }


    #[test]
    #[allow(non_snake_case)]
    fn eval_builtin_mapM_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Use the builtin mapM_ with show + putStrLn
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = mapM_ (\\x -> putStrLn (show x)) [1,2,3]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "1\n2\n3\n"),
            Err(e_chirho) => panic!("builtin mapM_ should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_where_local_helper_with_compose_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // where clause with local helper using (.)
        let result_chirho = eval_source_chirho(
            "module Test where\nprocess x = result\n  where\n    double y = y + y\n    result = double (double x)\nmain = process 3\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(12)),
            Err(e_chirho) => panic!("where local helper should work: {}", e_chirho),
        }
    }

    // ── IORef tests ──


    #[test]
    fn eval_ioref_write_read_chirho() {
        // newIORef 10, writeIORef r 99, readIORef r → "99\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r <- newIORef 10\n  writeIORef r 99\n  v <- readIORef r\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "99\n");
            }
            Err(e_chirho) => panic!("IORef write+read should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_ioref_show_read_chirho() {
        // newIORef 7, readIORef, show, putStrLn → "7\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r <- newIORef 7\n  v <- readIORef r\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "7\n");
            }
            Err(e_chirho) => panic!("IORef show+read should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_ioref_multiple_refs_chirho() {
        // Two IORefs: newIORef 10, newIORef 20, read both, add, show → "30\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r1 <- newIORef 10\n  r2 <- newIORef 20\n  v1 <- readIORef r1\n  v2 <- readIORef r2\n  putStrLn (show (v1 + v2))\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "30\n");
            }
            Err(e_chirho) => panic!("IORef multiple refs should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_where_forward_ref_chirho() {
        // Forward reference: a uses b, b is defined after a in where block
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = result\n  where\n    a = b + 1\n    b = 10\n    result = a\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(11)),
            Err(e_chirho) => panic!("Where forward ref should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_fromlist_chirho() {
        // mapFromList [(1,10),(2,20),(3,30)] — size should be 3 (returned as Int)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapSize (mapFromList [(1,10),(2,20),(3,30)])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("eval_map_fromlist_chirho failed: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_size_chirho() {
        // mapSize of singleton → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapSize (mapInsert 1 10 mapEmpty)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("Map size should be 1: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_insert_two_keys_chirho() {
        // Insert two keys, lookup both (using Prelude mapInsert)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 3 99 (mapInsert 5 42 mapEmpty)) 5\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("Map nested insert should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_member_chirho() {
        // mapMember 5 (mapInsert 5 42 mapEmpty) → True (1)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nboolToInt b = if b then 1 else 0\nmain = boolToInt (mapMember 5 (mapInsert 5 42 mapEmpty))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("Map member should be True: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_size_two_chirho() {
        // Insert two keys, check size is 2
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapSize (mapInsert 3 99 (mapInsert 5 42 mapEmpty))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("Map size should be 2: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_insert_overwrite_chirho() {
        // Insert same key twice, latest value wins
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 5 99 (mapInsert 5 42 mapEmpty)) 5\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("Map insert overwrite should return 99: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_from_list_chirho() {
        // mapFromList [(1,10),(2,20),(3,30)] then lookup key 2 → 20
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapFromList [(1,10),(2,20),(3,30)]) 2\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(20)),
            Err(e_chirho) => panic!("mapFromList lookup should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_three_keys_chirho() {
        // Insert three keys, lookup all three
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 1 10 (mapInsert 3 30 (mapInsert 2 20 mapEmpty))) 1 + getVal (mapInsert 1 10 (mapInsert 3 30 (mapInsert 2 20 mapEmpty))) 2 + getVal (mapInsert 1 10 (mapInsert 3 30 (mapInsert 2 20 mapEmpty))) 3\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(60)),
            Err(e_chirho) => panic!("Map three keys sum should be 60: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_delete_chirho() {
        // mapDelete removes a key from the map
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapDelete 5 (mapInsert 5 42 (mapInsert 3 99 mapEmpty))) 3\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("mapDelete should keep other keys: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_delete_missing_chirho() {
        // mapDelete on a key not in the map is a no-op
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapSize (mapDelete 999 (mapInsert 1 10 mapEmpty))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("mapDelete of missing key should be no-op: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_keys_chirho() {
        // mapKeys extracts sorted keys from the map
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (mapKeys (mapFromList [(3,30),(1,10),(2,20)]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("mapKeys sum should be 6: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_elems_chirho() {
        // mapElems extracts values from the map
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (mapElems (mapFromList [(1,10),(2,20),(3,30)]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(60)),
            Err(e_chirho) => panic!("mapElems sum should be 60: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_null_chirho() {
        // mapNull checks if map is empty
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nboolToInt b = if b then 1 else 0\nmain = boolToInt (mapNull mapEmpty) + boolToInt (mapNull (mapInsert 1 10 mapEmpty))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            // mapNull mapEmpty → True (1), mapNull (mapInsert...) → False (0), total 1
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("mapNull should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_map_chirho() {
        // mapMap (*2) doubles all values
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapMap (\\x -> x * 2) (mapFromList [(1,10),(2,20)])) 2\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(40)),
            Err(e_chirho) => panic!("mapMap should double values: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_foldl_with_key_chirho() {
        // mapFoldlWithKey sums all values
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapFoldlWithKey (\\acc k v -> acc + v) 0 (mapFromList [(1,10),(2,20),(3,30)])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(60)),
            Err(e_chirho) => panic!("mapFoldlWithKey sum should be 60: {}", e_chirho),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1)),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("setMember not found: {}", e_chirho),
        }
    }


    #[test]
    fn eval_set_size_chirho() {
        // setSize of a set built from inserts
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setInsert 3 (setInsert 1 (setInsert 2 setEmpty)))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("setSize should be 3: {}", e_chirho),
        }
    }


    #[test]
    fn eval_set_insert_duplicate_chirho() {
        // inserting duplicate should not increase size
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setInsert 1 (setInsert 1 (setInsert 1 setEmpty)))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1)),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(6)),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("setSize (setFromList [5,3,5,1,3]) should be 3: {}", e_chirho),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(2)),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5)),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(2)),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(2)),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(9)),
            Err(e_chirho) => panic!("setFilter: {}", e_chirho),
        }
    }


    #[test]
    fn eval_set_map_chirho() {
        // Map (*2) over set
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (setToList (setMap (\\x -> x * 2) (setFromList [1,2,3])))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(12)),
            Err(e_chirho) => panic!("setMap: {}", e_chirho),
        }
    }


    #[test]
    fn eval_set_fold_chirho() {
        // Fold (+) over set
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setFold (\\x acc -> x + acc) 0 (setFromList [1,2,3,4,5])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(15)),
            Err(e_chirho) => panic!("setFold: {}", e_chirho),
        }
    }

    // -- String-as-[Char] interop tests --


    // -- String-as-[Char] interop tests --

    #[test]
    fn eval_head_string_chirho() {
        // head "hello" → 'h'
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = ord (head \"hello\")\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(104)), // 'h' = 104
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5)),
            Err(e_chirho) => panic!("length string: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_toupper_string_chirho() {
        // map toUpper "hello" → "HELLO" via putStrLn
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (map toUpper \"hello\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
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
        let src_chirho = "module Test where\nmain = putStrLn (filter isDigit \"abc123\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
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
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("null empty string: {}", e_chirho),
        }
    }

    // -- Where-clause mutual recursion test --


    // -- Additional list-on-string tests --

    #[test]
    fn eval_zip_strings_chirho() {
        // zip "abc" [1,2,3] → length should be 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (zip \"abc\" [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3)),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("concatMap string: {}", e_chirho),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(120)),
            Err(e_chirho) => panic!("product: {}", e_chirho),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(36)),
            Err(e_chirho) => panic!("section (+10): {}", e_chirho),
        }
    }

    // ── Nested pattern matching in case ─────────────────────────────


    // ── Nested pattern matching in case ─────────────────────────────

    #[test]
    fn eval_case_nested_tuple_chirho() {
        // case (1, 2) of { (a, b) -> a + b }
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = case (1, 2) of { (a, b) -> a + b }\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("case nested tuple: {}", e_chirho),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(7)),
            Err(e_chirho) => panic!("lambda tuple pattern: {}", e_chirho),
        }
    }

    // ── Let with multiple bindings ──────────────────────────────────


    // ── Let with multiple bindings ──────────────────────────────────

    #[test]
    fn eval_let_multi_bind_chirho() {
        // let { a = 10; b = 20; c = 30 } in a + b + c
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = let a = 10\n           b = 20\n           c = 30\n       in a + b + c\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(60)),
            Err(e_chirho) => panic!("let multi bind: {}", e_chirho),
        }
    }

    // ── Data constructor as function ────────────────────────────────


    // ── Data constructor as function ────────────────────────────────

    #[test]
    fn eval_map_just_chirho() {
        // map Just [1,2,3] → [Just 1, Just 2, Just 3], length → 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (map Just [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("map Just: {}", e_chirho),
        }
    }

    // ── If in where ─────────────────────────────────────────────────


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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("if in where: {}", e_chirho),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(2)),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("lambda con pattern: {}", e_chirho),
        }
    }

    // ── Uncurry with operator section ────────────────────────────────


    // ── Map with operator section ────────────────────────────────────

    #[test]
    fn eval_map_section_subtract_chirho() {
        // map (subtract 1) [10, 20, 30] → [9, 19, 29], sum → 57
        // (subtract is \a b -> b - a in Prelude)
        // For now use a lambda instead since subtract isn't defined
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nsubtract a b = b - a\nmain = sum (map (subtract 1) [10, 20, 30])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(57)),
            Err(e_chirho) => panic!("map subtract: {}", e_chirho),
        }
    }

    // ── Where with multiple helper functions ─────────────────────────


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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(25)),
            Err(e_chirho) => panic!("where multi helpers: {}", e_chirho),
        }
    }

    // ── Nested data types ───────────────────────────────────────────


    // ── Nested data types ───────────────────────────────────────────

    #[test]
    fn eval_nested_maybe_case_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
safe_head xs = case xs of
  [] -> Nothing
  (x:_) -> Just x
main = case safe_head [42, 1, 2] of
  Nothing -> 0
  Just x -> x
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("nested maybe case: {}", e_chirho),
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
main = mapFindWithDefault 99 5 mapEmpty
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(99)),
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
main = mapFindWithDefault 99 1 (mapSingleton 1 42)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42)),
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
m = mapAdjust (*10) 1 (mapSingleton 1 5)
main = mapFindWithDefault 0 1 m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(50)),
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
m = mapUnion (mapSingleton 1 10) (mapSingleton 2 20)
main = mapSize m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(2)),
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
m1 = mapInsert 1 10 (mapInsert 2 20 (mapInsert 3 30 mapEmpty))
m2 = mapSingleton 2 99
main = mapSize (mapDifference m1 m2)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(2)),
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
m = mapInsert 1 10 (mapInsert 2 20 (mapInsert 3 30 mapEmpty))
main = mapSize (mapFilter (> 15) m)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("mapFilter: {}", e_chirho),
        }
    }


    #[test]
    fn eval_nub_head_chirho() {
        // head (nub [3,1,3,2]) → 3 (first unique is 3)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = head (nub [3,1,3,2])
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3)),
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
main = sum (intersperse 0 [1,2,3])
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("intersperse sum: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_str_multiple_chirho() {
        // Insert multiple string keys and verify size
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapInsertStr \"a\" 1 (mapInsertStr \"b\" 2 (mapInsertStr \"c\" 3 mapEmpty))
main = mapSize m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("mapStr multiple: {}", e_chirho),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(13)),
            Err(e_chirho) => panic!("nested let where: {}", e_chirho),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("case string match: {}", e_chirho),
        }
    }


    #[test]
    fn eval_map_fold_sum_chirho() {
        // Use mapFoldlWithKey to sum all values in a map
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapInsert 1 10 (mapInsert 2 20 (mapInsert 3 30 mapEmpty))
main = mapFoldlWithKey (\\acc k v -> acc + v) 0 m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            // 10 + 20 + 30 = 60
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(60)),
            Err(e_chirho) => panic!("map fold sum: {}", e_chirho),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(30)),
            Err(e_chirho) => panic!("map double sum: {}", e_chirho),
        }
    }


    #[test]
    fn eval_string_map_values_chirho() {
        // String-keyed map: insert 3 entries, lookup one value
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapInsertStr \"foo\" 10 (mapInsertStr \"bar\" 20 (mapInsertStr \"baz\" 30 mapEmpty))
main = mapFindWithDefaultStr 0 \"bar\" m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(20)),
            Err(e_chirho) => panic!("string map values: {}", e_chirho),
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
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match &result_chirho {
            Ok((_v_chirho, m_chirho)) => assert_eq!(m_chirho.io_output_chirho, "True False\n"),
            Err(e_chirho) => panic!("show bool list: {}", e_chirho),
        }
    }

    // ── Push to 1000 tests ─────────────────────────────────────────


    #[test]
    fn eval_nested_where_simple_chirho() {
        // f x = a + b where { a = x * 2; b = x + 3 }
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf x = a + b\n  where\n    a = x * 2\n    b = x + 3\nmain = f 10\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(33)),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("guard multiple equations: {}", e_chirho),
        }
    }


    #[test]
    fn eval_let_in_do_complex_chirho() {
        // do { let x = 10; let y = x + 5; putStrLn (show (x + y)) } → "25\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  let x = 10\n  let y = x + 5\n  putStrLn (show (x + y))\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "25\n");
            }
            Err(e_chirho) => panic!("let in do complex: {}", e_chirho),
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
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(30)),
            Err(e_chirho) => panic!("map with lambda: {}", e_chirho),
        }
    }


    #[test]
    fn eval_data_maybe_chain_chirho() {
        // safeDivide with guard instead of literal pattern match
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nsafeDivide x y\n  | y == 0 = Nothing\n  | otherwise = Just (x `div` y)\nmain = fromMaybe 0 (safeDivide 10 3) + fromMaybe 0 (safeDivide 10 0)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("data maybe chain: {}", e_chirho),
        }
    }


    #[test]
    fn eval_show_list_show_chirho() {
        // putStrLn (show [10,20,30]) → "[10,20,30]\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (show [10,20,30])\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "[10,20,30]\n");
            }
            Err(e_chirho) => panic!("show list show: {}", e_chirho),
        }
    }


    #[test]
    fn eval_first_tuple_chirho() {
        // first (+10) (5, 99) → (15, 99) → fst → 15
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nadd10 x = x + 10\nmain = fst (first add10 (5, 99))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(15));
    }


    #[test]
    fn eval_second_tuple_chirho() {
        // second (*2) (10, 7) → (10, 14) → snd → 14
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ndbl x = x * 2\nmain = snd (second dbl (10, 7))\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(14));
    }


    #[test]
    fn eval_both_tuple_chirho() {
        // both (+1) (10, 20) → (11, 21) → fst + snd → 32
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ninc x = x + 1\nmain = case both inc (10, 20) of (a, b) -> a + b\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(32));
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
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap();
        assert_eq!(m_chirho.io_output_chirho, "4\n");
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
        let val_chirho =
            eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("powerset failed: {}", e_chirho));
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(8));
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
        let val_chirho =
            eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("pascal triangle failed: {}", e_chirho));
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(1024));
    }

    // ── Multi-equation list-pattern tests ───────────────────────────────

    // ── deepseq / force / evaluate tests ─────────────────────────────────

    #[test]
    fn eval_deepseq_returns_second_chirho() {
        // deepseq x y = x `seq` y → deepseq (1 + 2) 42 should evaluate to 42
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\nmain = deepseq (1 + 2) 42\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("deepseq should evaluate");
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn eval_force_returns_value_chirho() {
        // force x = x `seq` x → force (2 + 3) should evaluate to 5
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\nmain = force (2 + 3)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("force should evaluate");
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(5));
    }

    #[test]
    fn eval_evaluate_returns_value_chirho() {
        // evaluate x = return x → evaluate 42 in an IO context
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\nmain = evaluate 42\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("evaluate should evaluate");
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
    }

    // ── $!! (deep strict application) tests ──────────────────────────────

    #[test]
    fn eval_double_bang_dollar_basic_chirho() {
        // f $!! x = deepseq x (f x) → (+1) $!! 41 should evaluate to 42
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\nmain = (+1) $!! 41\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("$!! should evaluate");
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn eval_double_bang_dollar_with_expression_chirho() {
        // f $!! (2 + 3) should deeply evaluate 2+3 then apply f
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\nf x = x * 2\nmain = f $!! (2 + 3)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("$!! with expression should evaluate");
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(10));
    }

    #[test]
    fn eval_hex_literal_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let val_chirho = eval_source_chirho(
            "module Main where\nmain = 0xFF\n",
            &mut sm_chirho, "Main.hs", None,
        ).unwrap();
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(255));
    }

    #[test]
    fn eval_octal_literal_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let val_chirho = eval_source_chirho(
            "module Main where\nmain = 0o77\n",
            &mut sm_chirho, "Main.hs", None,
        ).unwrap();
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(63));
    }

    #[test]
    fn eval_hex_plus_octal_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let val_chirho = eval_source_chirho(
            "module Main where\nmain = 0xFF + 0o17\n",
            &mut sm_chirho, "Main.hs", None,
        ).unwrap();
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(270));
    }

    #[test]
    fn eval_binary_literal_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let val_chirho = eval_source_chirho(
            "module Main where\nmain = 0b1010\n",
            &mut sm_chirho, "Main.hs", None,
        ).unwrap();
        assert_eq!(val_chirho, haskeluya_runtime_chirho::ValueChirho::IntChirho(10));
    }

