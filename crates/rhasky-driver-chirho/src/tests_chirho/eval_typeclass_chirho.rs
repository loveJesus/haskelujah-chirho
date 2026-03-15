// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// Typeclass, deriving, Show, Eq, Semigroup, Monoid, type synonym tests

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
    fn typeclass_instance_compiles_chirho() {
        // A simple typeclass with one method and one ground instance.
        // This verifies that the desugarer produces $prim_ bindings for
        // instance methods and the dict pass can build the dictionary.
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
class MyShow a where
  myShow :: a -> Int
instance MyShow Int where
  myShow x = x
main = myShow 42
";
        let result_chirho = compile_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "typeclass with instance should compile: {:?}",
            result_chirho.err()
        );
        // Verify the Core module contains a $prim_ binding
        let core_chirho = &result_chirho.unwrap().core_chirho;
        let has_prim_chirho = core_chirho
            .bindings_chirho
            .iter()
            .any(|b_chirho| b_chirho.binder_chirho.name_chirho.contains("$prim_MyShow_myShow"));
        assert!(
            has_prim_chirho,
            "Core should contain $prim_MyShow_myShow binding"
        );
    }


    #[test]
    fn typeclass_instance_eval_identity_chirho() {
        // End-to-end: class + instance + call → evaluated runtime value.
        // myShow just returns its argument, so myShow 42 == 42.
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
class MyShow a where
  myShow :: a -> Int
instance MyShow Int where
  myShow x = x
main = myShow 42
";
        let result_chirho = crate::eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("typeclass program should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(42),
            "myShow 42 should evaluate to IntChirho(42)"
        );
    }


    #[test]
    fn typeclass_instance_eval_arithmetic_chirho() {
        // Instance method that does arithmetic: myDouble x = x + x
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
class MyOp a where
  myDouble :: a -> Int
instance MyOp Int where
  myDouble x = x + x
main = myDouble 21
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("typeclass arithmetic should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(42),
            "myDouble 21 should evaluate to IntChirho(42)"
        );
    }


    #[test]
    fn typeclass_builtin_eq_eval_chirho() {
        // Test that the built-in Eq instance for Int works through the
        // dictionary-passing transform. `==` is a class method of Eq,
        // and the $prim_Eq_==_Int binding wraps the ==# primop.
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // `if 3 == 3 then 1 else 0` — tests Eq dictionary resolution
        let src_chirho = "\
module Test where
main = if 3 == 3 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        // Note: This may fail if == is not yet routed through the dict pass
        // for the built-in Eq instance. In that case it still goes through
        // the primop path directly. Either way, the result should be 1.
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(1),
                    "3 == 3 should be True, giving 1"
                );
            }
            Err(e_chirho) => {
                // If it fails, it might be because == goes through dict
                // dispatch and hits a runtime issue. Log for debugging.
                panic!("builtin Eq eval failed: {}", e_chirho);
            }
        }
    }


    #[test]
    fn typeclass_user_method_calls_builtin_chirho() {
        // Instance method that uses a built-in operator (+) through Num:
        // This tests that user-defined instance bodies can reference
        // class methods that are resolved through the built-in dict.
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
class Doubler a where
  double :: a -> Int
instance Doubler Int where
  double x = x + x
main = double 10
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("user instance calling builtin + should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(20),
            "double 10 should be 20"
        );
    }


    #[test]
    fn typeclass_derived_eq_compiles_chirho() {
        // Test that deriving Eq generates an instance that flows through
        // the full compilation pipeline.
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq)
main = if Red == Red then 1 else 0
";
        let result_chirho = compile_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "deriving Eq should compile: {:?}",
            result_chirho.err()
        );
    }


    #[test]
    fn typeclass_instance_two_methods_compiles_chirho() {
        // A typeclass with two methods and a ground instance.
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
class MyNum a where
  myAdd :: a -> a -> a
  myMul :: a -> a -> a
instance MyNum Int where
  myAdd x y = x + y
  myMul x y = x * y
main = myAdd 3 4
";
        let result_chirho = compile_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "typeclass with two methods should compile: {:?}",
            result_chirho.err()
        );
        let core_chirho = &result_chirho.unwrap().core_chirho;
        let prim_names_chirho: Vec<&str> = core_chirho
            .bindings_chirho
            .iter()
            .filter(|b_chirho| b_chirho.binder_chirho.name_chirho.starts_with("$prim_MyNum"))
            .map(|b_chirho| b_chirho.binder_chirho.name_chirho.as_str())
            .collect();
        assert!(
            prim_names_chirho.iter().any(|n_chirho| n_chirho.contains("myAdd")),
            "should have $prim_MyNum_myAdd binding, got: {:?}",
            prim_names_chirho
        );
        assert!(
            prim_names_chirho.iter().any(|n_chirho| n_chirho.contains("myMul")),
            "should have $prim_MyNum_myMul binding, got: {:?}",
            prim_names_chirho
        );
    }


    #[test]
    fn typeclass_derived_eq_eval_chirho() {
        // End-to-end: deriving Eq on a simple enum type, then evaluate
        // an equality comparison at runtime through the dictionary pass.
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq)
main = if Red == Red then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(1),
                    "Red == Red should be True, giving 1"
                );
            }
            Err(e_chirho) => {
                panic!("derived Eq eval failed: {}", e_chirho);
            }
        }
    }


    #[test]
    fn typeclass_derived_eq_neq_eval_chirho() {
        // Derived Eq: two different constructors should return False → 0.
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq)
main = if Red == Green then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(0),
                    "Red == Green should be False, giving 0"
                );
            }
            Err(e_chirho) => {
                panic!("derived Eq neq eval failed: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_neq_operator_chirho() {
        // /= desugars to not (==)
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if 3 /= 4 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("/= should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
            "3 /= 4 should be True, giving 1"
        );
    }


    #[test]
    fn eval_show_int_chirho() {
        // show for Int should convert the integer to its string
        // representation. This tests the full pipeline:
        // show 42 → ($sel_Show_show $fShowInt) (fromInteger 42)
        //         → showInt# 42
        //         → "42"
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (show 42)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("putStrLn (show 42) should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "42\n",
            "show 42 should produce the string \"42\""
        );
    }


    #[test]
    fn eval_show_fib_chirho() {
        // Full pipeline test: compute fib(10) = 89, show it, putStrLn
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
fib n = if n == 0 then 1
        else if n == 1 then 1
        else fib (n - 1) + fib (n - 2)
main = putStrLn (show (fib 10))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("fib + show should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "89\n",
            "fib 10 = 89"
        );
    }


    // ---------------------------------------------------------------
    // Priority 64: deriving Eq / Show / Ord
    // ---------------------------------------------------------------

    #[test]
    fn eval_deriving_eq_enum_chirho() {
        // deriving Eq on a simple enum: Red == Red → True, Red == Blue → False
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq)
main = if Red == Red then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq enum should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
            "Red == Red → 1"
        );
    }


    #[test]
    fn eval_deriving_eq_enum_false_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq)
main = if Red == Blue then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq enum false should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
            "Red == Blue → 0"
        );
    }


    #[test]
    fn eval_deriving_show_enum_chirho() {
        // deriving Show on a simple enum: show Green → "Green"
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Show)
main = putStrLn (show Green)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Show enum should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "Green\n",
            "show Green → \"Green\""
        );
    }


    #[test]
    fn eval_deriving_eq_with_fields_chirho() {
        // deriving Eq on a constructor with fields
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Pair = MkPair Int Int deriving (Eq)
main = if MkPair 1 2 == MkPair 1 2 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq with fields should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
            "MkPair 1 2 == MkPair 1 2 → 1"
        );
    }


    #[test]
    fn eval_deriving_eq_with_fields_false_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Pair = MkPair Int Int deriving (Eq)
main = if MkPair 1 2 == MkPair 1 3 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq with fields false should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
            "MkPair 1 2 == MkPair 1 3 → 0"
        );
    }


    #[test]
    fn eval_deriving_show_with_fields_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Pair = MkPair Int Int deriving (Show)
main = putStrLn (show (MkPair 3 4))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Show with fields should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "MkPair 3 4\n",
            "show (MkPair 3 4) → \"MkPair 3 4\""
        );
    }


    #[test]
    fn eval_deriving_eq_multi_con_chirho() {
        // Multi-constructor with fields: different constructors should not be equal
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Shape = Circle Int | Rect Int Int deriving (Eq)
main = if Circle 5 == Rect 5 5 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq multi-con should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
            "Circle 5 == Rect 5 5 → 0"
        );
    }


    #[test]
    fn eval_deriving_eq_show_combined_chirho() {
        // deriving both Eq and Show on a data type
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq, Show)
main = if Red == Red then putStrLn (show Blue) else putStrLn (show Red)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq+Show should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "Blue\n",
            "Red==Red is true, so show Blue"
        );
    }

    // ---------------------------------------------------------------
    // Priority 65: Arithmetic sequences with step
    // ---------------------------------------------------------------


    #[test]
    fn eval_show_tuple_int_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (show (1, 2))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "(1,2)\n");
            }
            Err(e_chirho) => {
                eprintln!("show (1,2) failed: {}", e_chirho);
                panic!("show (1,2) should work: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_show_negative_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (show (0 - 5))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "-5\n"),
            Err(e_chirho) => panic!("show negative should print: {}", e_chirho),
        }
    }


    #[test]
    fn eval_show_true_typeclass_chirho() {
        // show True through real typeclass Show machinery (not manual showBool)
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (show True)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "True\n"),
            Err(e_chirho) => panic!("show True through typeclass should print: {}", e_chirho),
        }
    }


    #[test]
    fn eval_show_false_typeclass_chirho() {
        // show False through real typeclass Show machinery
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (show False)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "False\n"),
            Err(e_chirho) => panic!("show False through typeclass should print: {}", e_chirho),
        }
    }


    // ── Deriving Show for product types + advanced features ─────────

    #[test]
    fn eval_deriving_show_product_chirho() {
        // data Point = Point Int Int deriving (Show)
        // show (Point 3 4) → "Point 3 4"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Point = Point Int Int deriving (Show)
main = putStrLn (show (Point 3 4))
"#;
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "Point 3 4\n");
            }
            Err(e_chirho) => panic!("deriving Show product: {}", e_chirho),
        }
    }


    #[test]
    fn eval_deriving_show_fields_chirho() {
        // data Point = Point Int Int deriving (Show)
        // putStrLn (show (Point 3 4)) → "Point 3 4\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Point = Point Int Int deriving (Show)
main = putStrLn (show (Point 3 4))
"#;
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "Point 3 4\n",
                    "show (Point 3 4) should produce \"Point 3 4\""
                );
            }
            Err(e_chirho) => panic!("deriving Show with fields: {}", e_chirho),
        }
    }


    #[test]
    fn eval_deriving_show_multi_con_chirho() {
        // data Shape = Circle Int | Rect Int Int deriving (Show)
        // do { putStrLn (show (Circle 5)); putStrLn (show (Rect 3 4)) }
        // → "Circle 5\nRect 3 4\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Shape = Circle Int | Rect Int Int deriving (Show)
main = do
  putStrLn (show (Circle 5))
  putStrLn (show (Rect 3 4))
"#;
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "Circle 5\nRect 3 4\n",
                    "show Circle 5 and show Rect 3 4 in do-block"
                );
            }
            Err(e_chirho) => panic!("deriving Show multi-con: {}", e_chirho),
        }
    }


    #[test]
    fn eval_deriving_eq_product_chirho() {
        // data Point = Point Int Int deriving (Eq)
        // Point 3 4 == Point 3 4 → True → 1
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Point = Point Int Int deriving (Eq)
main = if Point 3 4 == Point 3 4 then 1 else 0
"#;
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("deriving Eq product: {}", e_chirho),
        }
    }


    #[test]
    fn eval_deriving_eq_product_neq_chirho() {
        // Point 3 4 == Point 3 5 → False → 0
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Point = Point Int Int deriving (Eq)
main = if Point 3 4 == Point 3 5 then 1 else 0
"#;
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("deriving Eq product neq: {}", e_chirho),
        }
    }

    // ── List comprehension with guards and transforms ──────────────


    // ── Type annotation expressions ─────────────────────────────────

    #[test]
    fn eval_type_annotation_chirho() {
        // (42 :: Int) → 42
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = (42 :: Int)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("type annotation: {}", e_chirho),
        }
    }

    // ── Lambda with tuple pattern ────────────────────────────────────


    // ── Type synonym in user code ───────────────────────────────────

    #[test]
    fn eval_type_synonym_list_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
type IntList = [Int]
sumList :: IntList -> Int
sumList xs = sum xs
main = sumList [1, 2, 3, 4, 5]
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15)),
            Err(e_chirho) => panic!("type synonym list: {}", e_chirho),
        }
    }

    // ── Complex list processing ─────────────────────────────────────


    // ── More feature tests ──

    #[test]
    fn eval_type_synonym_usage_chirho() {
        // type MyList = [Int]; f :: MyList -> Int; f xs = sum xs; main = f [1,2,3]
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ntype MyList = [Int]\nf :: MyList -> Int\nf xs = sum xs\nmain = f [1,2,3]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("type synonym usage: {}", e_chirho),
        }
    }


    #[test]
    fn eval_show_negative_int_chirho() {
        // putStrLn (show (negate 42)) → "-42\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (show (negate 42))\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "-42\n");
            }
            Err(e_chirho) => panic!("show negative: {}", e_chirho),
        }
    }


    // ── Semigroup / Monoid tests ──

    #[test]
    fn eval_semigroup_append_lists_chirho() {
        // [1,2] <> [3,4] should produce a list of length 4
        // We evaluate: length ([1,2] <> [3,4])
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length ([1,2] <> [3,4])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4)),
            Err(e_chirho) => panic!("semigroup append lists: {}", e_chirho),
        }
    }


    #[test]
    fn eval_semigroup_append_strings_chirho() {
        // "hello" <> " " <> "world" via putStrLn
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (\"hello\" <> \" \" <> \"world\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "hello world\n");
            }
            Err(e_chirho) => panic!("semigroup append strings: {}", e_chirho),
        }
    }


    #[test]
    fn eval_monoid_mempty_list_chirho() {
        // mempty <> [1,2,3] should give [1,2,3] → length 3
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (mempty <> [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("monoid mempty list: {}", e_chirho),
        }
    }


    #[test]
    fn eval_monoid_mconcat_chirho() {
        // mconcat [[1,2],[3],[4,5]] should give [1,2,3,4,5] → length 5
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (mconcat [[1,2],[3],[4,5]])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5)),
            Err(e_chirho) => panic!("monoid mconcat: {}", e_chirho),
        }
    }

    // ── Higher-order *By list function tests ──


    // ── Type synonyms in instance heads ──────────────────────────────

    #[test]
    fn eval_type_synonym_instance_chirho() {
        // type String = [Char] is built-in; test that Show String resolves
        // to Show [Char] which we already have
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
greet :: String -> String
greet name = name
main = putStrLn (greet \"hello\")
";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "hello\n");
            }
            Err(e_chirho) => panic!("type synonym instance: {}", e_chirho),
        }
    }


    #[test]
    fn eval_user_type_synonym_chirho() {
        // User-defined type synonym
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
type Age = Int
addAge :: Age -> Age -> Age
addAge x y = x + y
main = addAge 25 17
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("user type synonym: {}", e_chirho),
        }
    }

    // ── Where-clause / let type annotations ────────────────────────────


    // ── Where-clause / let type annotations ────────────────────────────

    #[test]
    fn eval_where_type_annotation_chirho() {
        // where-clause with type annotation
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
f x = helper x
  where
    helper :: Int -> Int
    helper y = y + 1
main = f 41
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("where type annotation: {}", e_chirho),
        }
    }


    #[test]
    fn eval_let_type_annotation_chirho() {
        // let expression with type annotation
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = let double :: Int -> Int
           double x = x + x
       in double 21
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("let type annotation: {}", e_chirho),
        }
    }


    #[test]
    fn eval_where_multiple_annotated_chirho() {
        // where-clause with multiple annotated helper functions
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
f x = add3 (double x)
  where
    double :: Int -> Int
    double y = y + y
    add3 :: Int -> Int
    add3 z = z + 3
main = f 10
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            // double 10 = 20, add3 20 = 23
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(23)),
            Err(e_chirho) => panic!("where multiple annotated: {}", e_chirho),
        }
    }

    // ── Synthetic module imports ─────────────────────────────────────────


    // ── Show Either / Ordering ──────────────────────────────────────────

    #[test]
    fn eval_show_nested_just_chirho() {
        // show (Just (Just 42)) should give "Just (Just 42)"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (show (Just (Just 42)))
";
        let (_, machine_chirho) = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(machine_chirho.io_output_chirho, "Just (Just 42)\n");
    }

    // ── ST monad ──────────────────────────────────────────────────────────


    // ── Deriving Ord end-to-end ─────────────────────────────────────────

    #[test]
    fn eval_deriving_ord_compare_chirho() {
        // data Color = Red | Green | Blue deriving (Eq, Ord)
        // compare Red Green → LT via case dispatch
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ndata Color = Red | Green | Blue deriving (Eq, Ord)\nmain = case compare Red Green of { LT -> 1; EQ -> 0; GT -> 0 }\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_eq_ordering_eq_lt_chirho() {
        // compare 1 2 == LT → True (tests Eq Ordering instance)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if compare 1 2 == LT then 1 else 0\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }


    #[test]
    fn eval_eq_ordering_neq_chirho() {
        // compare 5 5 == LT → False
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if compare 5 5 == LT then 1 else 0\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0));
    }


    #[test]
    fn eval_show_ordering_chirho() {
        // show (compare 1 2) → "LT"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (show (compare 1 2))\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => assert_eq!(m_chirho.io_output_chirho, "LT\n"),
            Err(e_chirho) => panic!("show ordering: {}", e_chirho),
        }
    }

    // ── BST / parameterized data type tests ────────────────────────────


    // ── Type annotations in expressions ───────────────────────────────

    #[test]
    fn eval_type_annotation_expr_chirho() {
        // (42 :: Int) should evaluate to 42
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = (42 :: Int) + 1
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(43));
    }


    #[test]
    fn eval_type_annotation_let_chirho() {
        // let binding with type annotation
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = let x = (10 :: Int) in x + 5
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
    }

    // ── Show for compound types ───────────────────────────────────────


    // ── Show for compound types ───────────────────────────────────────

    #[test]
    fn eval_show_string_list_chirho() {
        // show "hello" should produce "\"hello\""
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = putStrLn (show "hello")
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap();
        assert_eq!(m_chirho.io_output_chirho, "\"hello\"\n");
    }


    #[test]
    fn eval_show_nested_list_chirho() {
        // show [[1,2],[3]] — list of lists
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = putStrLn (show [[1,2],[3]])
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap();
        assert_eq!(m_chirho.io_output_chirho, "[[1,2],[3]]\n");
    }


    #[test]
    fn eval_show_tuple_string_chirho() {
        // show (42, "hi") — tuple with mixed types
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = putStrLn (show (42, True))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap();
        assert_eq!(m_chirho.io_output_chirho, "(42,True)\n");
    }

    // ── Complex where-clause with multiple helpers ────────────────────

    /// Test 1: Collatz sequence — complex where-clause with a helper function.
    /// collatz 27 takes 111 steps.

    // ── Feature 2: type annotations in expressions (named per spec) ───

    #[test]
    fn eval_type_ann_chirho() {
        // putStrLn (show (42 :: Int)) → "42\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = putStrLn (show (42 :: Int))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("type ann show failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "42\n");
    }


    #[test]
    fn eval_type_ann_read_chirho() {
        // putStrLn (show (read "10" :: Int)) → "10\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = putStrLn (show (read "10" :: Int))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("type ann read show failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "10\n");
    }

    // ── Algorithmic tests: stress-testing compiler capabilities ───────

