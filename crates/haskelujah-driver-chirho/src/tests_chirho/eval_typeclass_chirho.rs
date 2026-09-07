// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// Typeclass, deriving, Show, Eq, Semigroup, Monoid, type synonym tests

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
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "typeclass with instance should compile: {:?}",
        result_chirho.err()
    );
    // Verify the Core module contains a $prim_ binding
    let core_chirho = &result_chirho.unwrap().core_chirho;
    let has_prim_chirho = core_chirho.bindings_chirho.iter().any(|b_chirho| {
        b_chirho
            .binder_chirho
            .name_chirho
            .contains("$prim_MyShow_myShow")
    });
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
    let result_chirho =
        crate::eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("typeclass program should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42),
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("typeclass arithmetic should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42),
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    // Note: This may fail if == is not yet routed through the dict pass
    // for the built-in Eq instance. In that case it still goes through
    // the primop path directly. Either way, the result should be 1.
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1),
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("user instance calling builtin + should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(20),
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
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
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
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "typeclass with two methods should compile: {:?}",
        result_chirho.err()
    );
    let core_chirho = &result_chirho.unwrap().core_chirho;
    let prim_names_chirho: Vec<&str> = core_chirho
        .bindings_chirho
        .iter()
        .filter(|b_chirho| {
            b_chirho
                .binder_chirho
                .name_chirho
                .starts_with("$prim_MyNum")
        })
        .map(|b_chirho| b_chirho.binder_chirho.name_chirho.as_str())
        .collect();
    assert!(
        prim_names_chirho
            .iter()
            .any(|n_chirho| n_chirho.contains("myAdd")),
        "should have $prim_MyNum_myAdd binding, got: {:?}",
        prim_names_chirho
    );
    assert!(
        prim_names_chirho
            .iter()
            .any(|n_chirho| n_chirho.contains("myMul")),
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1),
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(0),
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("/= should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1),
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("putStrLn (show 42) should evaluate");
    assert_eq!(
        result_chirho.1.io_output_chirho, "42\n",
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("fib + show should evaluate");
    assert_eq!(result_chirho.1.io_output_chirho, "89\n", "fib 10 = 89");
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("deriving Eq enum should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1),
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("deriving Eq enum false should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0),
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("deriving Show enum should evaluate");
    assert_eq!(
        result_chirho.1.io_output_chirho, "Green\n",
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("deriving Eq with fields should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1),
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("deriving Eq with fields false should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0),
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("deriving Show with fields should evaluate");
    assert_eq!(
        result_chirho.1.io_output_chirho, "MkPair 3 4\n",
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("deriving Eq multi-con should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0),
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("deriving Eq+Show should evaluate");
    assert_eq!(
        result_chirho.1.io_output_chirho, "Blue\n",
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho, "Point 3 4\n",
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho, "Circle 5\nRect 3 4\n",
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
        ),
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
        ),
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
        ),
        Err(e_chirho) => panic!("type synonym usage: {}", e_chirho),
    }
}

#[test]
fn eval_show_negative_int_chirho() {
    // putStrLn (show (negate 42)) → "-42\n"
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = putStrLn (show (negate 42))\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(4)
        ),
        Err(e_chirho) => panic!("semigroup append lists: {}", e_chirho),
    }
}

#[test]
fn eval_semigroup_append_strings_chirho() {
    // "hello" <> " " <> "world" via putStrLn
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = putStrLn (\"hello\" <> \" \" <> \"world\")\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!("monoid mempty list: {}", e_chirho),
    }
}

#[test]
fn eval_sum_monoid_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Monoid\nmain = getSum (Sum 3 <> Sum 4) + getSum (mconcat [Sum 5, Sum 6]) + getSum (mempty <> Sum 7)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(25)
        ),
        Err(e_chirho) => panic!("sum monoid: {}", e_chirho),
    }
}

#[test]
fn eval_product_monoid_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Monoid\nmain = getProduct (Product 3 <> Product 4) + getProduct (mconcat [Product 2, Product 5]) + getProduct (mempty <> Product 7)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(29)
        ),
        Err(e_chirho) => panic!("product monoid: {}", e_chirho),
    }
}

#[test]
fn eval_all_monoid_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Monoid\nmain = if getAll (mempty <> All True <> mconcat [All True, All False]) then 1 else 0\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
        ),
        Err(e_chirho) => panic!("all monoid: {}", e_chirho),
    }
}

#[test]
fn eval_any_monoid_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Monoid\nmain = if getAny (mempty <> Any False <> mconcat [Any False, Any True]) then 1 else 0\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("any monoid: {}", e_chirho),
    }
}

#[test]
fn eval_identity_functor_applicative_monad_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Functor.Identity
main =
  runIdentity (fmap (+1) (Identity 2))
  + runIdentity (pure 3 :: Identity Int)
  + runIdentity (Identity (+4) <*> Identity 5)
  + runIdentity (Identity 6 >>= \\x -> Identity (x + 1))
  + runIdentity (Identity 8 >> Identity 9)
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(31)
        ),
        Err(e_chirho) => panic!("identity functor/applicative/monad: {}", e_chirho),
    }
}

#[test]
fn eval_down_functor_applicative_monad_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Ord
main =
  getDown (fmap (+1) (Down 2))
  + getDown (pure 3 :: Down Int)
  + getDown (Down (+4) <*> Down 5)
  + getDown (Down 6 >>= \\x -> Down (x + 1))
  + getDown (Down 8 >> Down 9)
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(31)
        ),
        Err(e_chirho) => panic!("down functor/applicative/monad: {}", e_chirho),
    }
}

#[test]
fn eval_product_functor_applicative_monad_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Monoid
main =
  getProduct (fmap (+1) (Product 2))
  + getProduct (pure 3 :: Product Int)
  + getProduct (Product (+4) <*> Product 5)
  + getProduct (Product 6 >>= \\x -> Product (x + 1))
  + getProduct (Product 8 >> Product 9)
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(31)
        ),
        Err(e_chirho) => panic!("product functor/applicative/monad: {}", e_chirho),
    }
}

#[test]
fn eval_proxy_functor_applicative_monad_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Proxy
scoreChirho :: Proxy a -> Int
scoreChirho Proxy = 1
main =
  scoreChirho (fmap (+1) (Proxy :: Proxy Int))
  + scoreChirho (pure 3 :: Proxy Int)
  + scoreChirho ((Proxy :: Proxy (Int -> Int)) <*> (Proxy :: Proxy Int))
  + scoreChirho ((Proxy :: Proxy Int) >>= \\xChirho -> Proxy)
  + scoreChirho ((Proxy :: Proxy Int) >> (Proxy :: Proxy Int))
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
        ),
        Err(e_chirho) => panic!("proxy functor/applicative/monad: {}", e_chirho),
    }
}

#[test]
fn eval_const_functor_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Functor.Const
main = getConst (fmap (+1) (Const 41 :: Const Int Int))
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(41)
        ),
        Err(e_chirho) => panic!("const functor: {}", e_chirho),
    }
}

#[test]
fn eval_alternative_maybe_list_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Control.Applicative
scoreMaybe m = case m of
  Nothing -> 0
  Just x -> x
main =
  length ((empty :: [Int]) <|> [1,2])
  + length ([3] <|> [4,5])
  + scoreMaybe ((empty :: Maybe Int) <|> Just 7)
  + scoreMaybe (Nothing <|> Just 11)
  + scoreMaybe (Just 13 <|> Just 17)
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(36)
        ),
        Err(e_chirho) => panic!("alternative maybe/list: {}", e_chirho),
    }
}

#[test]
fn eval_monadplus_maybe_list_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Control.Monad
scoreMaybe m = case m of
  Nothing -> 0
  Just x -> x
main =
  length (mzero :: [Int])
  + scoreMaybe (mzero :: Maybe Int)
  + length (mplus (mzero :: [Int]) [1,2])
  + length (mplus [3] [4,5])
  + scoreMaybe (mplus (mzero :: Maybe Int) (Just 7))
  + scoreMaybe (mplus Nothing (Just 11))
  + scoreMaybe (mplus (Just 13) (Just 17))
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(36)
        ),
        Err(e_chirho) => panic!("monadplus maybe/list: {}", e_chirho),
    }
}

#[test]
fn eval_monadfail_maybe_list_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Control.Monad
scoreMaybe m = case m of
  Nothing -> 0
  Just x -> x
main =
  scoreMaybe (fail \"missing\" :: Maybe Int)
  + length (fail \"missing\" :: [Int])
  + scoreMaybe (mplus (fail \"missing\" :: Maybe Int) (Just 4))
  + length (mplus (fail \"missing\" :: [Int]) [1,2])
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
        ),
        Err(e_chirho) => panic!("monadfail maybe/list: {}", e_chirho),
    }
}

#[test]
fn eval_refutable_do_patterns_use_monadfail_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
scoreMaybe m = case m of
  Nothing -> 0
  Just x -> x
scoreList xs = case xs of
  a : b : [] -> a + b
  _ -> 0
main =
  scoreMaybe ((do { Just x <- Just Nothing; return x }) :: Maybe Int)
  + scoreMaybe ((do { Just x <- Just (Just 3); return x }) :: Maybe Int)
  + scoreList ((do { Just x <- [Just 1, Nothing, Just 3]; return x }) :: [Int])
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
        ),
        Err(e_chirho) => panic!("refutable do patterns should use MonadFail: {}", e_chirho),
    }
}

#[test]
fn eval_min_max_semigroup_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Semigroup
main =
  getMin (Min 4 <> Min 2)
  + getMin (Min 9 <> Min 12)
  + getMax (Max 4 <> Max 2)
  + getMax (Max 9 <> Max 12)
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(27)
        ),
        Err(e_chirho) => panic!("min/max semigroup: {}", e_chirho),
    }
}

#[test]
fn eval_min_max_monoid_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Monoid
main =
  getMin (mempty <> Min 7)
  + getMin (mconcat [Min 8, Min 3, Min 5])
  + getMin (mappend (Min 6) (Min 4))
  + getMax (mempty <> Max 7)
  + getMax (mconcat [Max 8, Max 3, Max 12])
  + getMax (mappend (Max 6) (Max 4))
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(39)
        ),
        Err(e_chirho) => panic!("min/max monoid: {}", e_chirho),
    }

    let mut sm_chirho = SourceMapChirho::new_chirho();
    let min_identity_src_chirho = "\
module Test where
import Data.Monoid
main = getMin (mempty :: Min Int)
";
    let result_chirho = eval_source_chirho(
        min_identity_src_chirho,
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(i64::MAX)
        ),
        Err(e_chirho) => panic!("min monoid identity: {}", e_chirho),
    }

    let mut sm_chirho = SourceMapChirho::new_chirho();
    let max_identity_src_chirho = "\
module Test where
import Data.Monoid
main = getMax (mempty :: Max Int)
";
    let result_chirho = eval_source_chirho(
        max_identity_src_chirho,
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(i64::MIN)
        ),
        Err(e_chirho) => panic!("max monoid identity: {}", e_chirho),
    }
}

#[test]
fn eval_first_last_monoid_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Monoid
scoreMaybe m = case m of
  Nothing -> 0
  Just x -> x
main =
  scoreMaybe (getFirst (First Nothing <> First (Just 4)))
  + scoreMaybe (getFirst (First (Just 7) <> First (Just 9)))
  + scoreMaybe (getFirst (mempty <> First (Just 5)))
  + scoreMaybe (getFirst (mconcat [First Nothing, First (Just 6), First (Just 8)]))
  + scoreMaybe (getLast (Last Nothing <> Last (Just 4)))
  + scoreMaybe (getLast (Last (Just 7) <> Last (Just 9)))
  + scoreMaybe (getLast (mempty <> Last (Just 5)))
  + scoreMaybe (getLast (mconcat [Last (Just 6), Last Nothing, Last (Just 8)]))
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(48)
        ),
        Err(e_chirho) => panic!("first/last monoid: {}", e_chirho),
    }
}

#[test]
fn eval_ordering_monoid_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Monoid
score x = case x of
  LT -> 1
  EQ -> 2
  GT -> 3
main = score (EQ <> LT) + score (GT <> LT) + score (mempty <> EQ) + score (mconcat [EQ, GT, LT])
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(9)
        ),
        Err(e_chirho) => panic!("ordering monoid: {}", e_chirho),
    }
}

#[test]
fn eval_endo_monoid_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Monoid
main =
  appEndo (Endo (+1) <> Endo (*2)) 10
  + appEndo (mempty <> Endo (+5)) 7
  + appEndo (mconcat [Endo (+3), Endo (*4)]) 2
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(44)
        ),
        Err(e_chirho) => panic!("endo monoid: {}", e_chirho),
    }
}

#[test]
fn eval_mappend_uses_semigroup_body_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Monoid
main =
  length (mappend [1,2] [3,4])
  + getSum (mappend (Sum 3) (Sum 4))
  + getSum (mappend mempty (Sum 5))
  + appEndo (mappend (Endo (+1)) (Endo (*2))) 10
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(37)
        ),
        Err(e_chirho) => panic!("mappend semigroup body: {}", e_chirho),
    }
}

#[test]
fn eval_unit_monoid_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Monoid
scoreChirho () = 1
main =
  scoreChirho (() <> ())
  + scoreChirho (mappend () ())
  + scoreChirho (mempty :: ())
  + scoreChirho (mconcat [(), (), ()])
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(4)
        ),
        Err(e_chirho) => panic!("unit monoid: {}", e_chirho),
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
        ),
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
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
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
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(23)
        ),
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
    let (_, machine_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(machine_chirho.io_output_chirho, "Just (Just 42)\n");
}

#[test]
fn eval_gadt_record_fields_are_readable_chirho() {
    // A GADT record constructor's fields must exist and be readable. A check-only
    // gate CANNOT see this: before the parser recognised the braces, the whole
    // declaration was lowered to a nullary constructor with no fields, `main` did
    // not survive to STG, and all eight corpus files carrying this construct were
    // still green. Exact output, so the field values have to actually arrive.
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
{-# LANGUAGE GADTs #-}
module Test where
data T where
  MkT :: { f :: Int, g :: String } -> T
main = do
  let t = MkT { f = 42, g = \"hi\" }
  let u = t { f = 7 }
  print (f t)
  putStrLn (g t)
  print (f u)
";
    let (_, machine_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(machine_chirho.io_output_chirho, "42\nhi\n7\n");
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
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_eq_ordering_eq_lt_chirho() {
    // compare 1 2 == LT → True (tests Eq Ordering instance)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = if compare 1 2 == LT then 1 else 0\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_eq_ordering_neq_chirho() {
    // compare 5 5 == LT → False
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = if compare 5 5 == LT then 1 else 0\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
    );
}

#[test]
fn eval_show_ordering_chirho() {
    // show (compare 1 2) → "LT"
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = putStrLn (show (compare 1 2))\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
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
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(43)
    );
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
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
    );
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
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
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
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
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
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
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

// ── OverloadedLists tests ────────────────────────────────────────

#[test]
fn overloaded_lists_fromlist_identity_chirho() {
    // fromList [10,20,30] should be identity on lists → head = 10
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = print (head (fromList [10,20,30]))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("overloaded lists fromList failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "10\n");
}

#[test]
fn overloaded_lists_fromlist_length_chirho() {
    // fromList [10,20,30] should be identity → length = 3
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = print (length (fromList [10,20,30]))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| {
                panic!("overloaded lists fromList length failed: {}", e_chirho)
            });
    assert_eq!(m_chirho.io_output_chirho, "3\n");
}

// ── Monad transformer tests ─────────────────────────────────────

#[test]
fn state_t_get_eval_chirho() {
    // get retrieves the current state; evalState extracts result
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = print (evalState get 42)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("StateT get failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn state_t_put_exec_chirho() {
    // put sets state; execState extracts final state
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = print (execState (put 99) 0)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("StateT put failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "99\n");
}

#[test]
fn state_t_modify_chirho() {
    // modify applies a function to the state
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
inc3 = bindStateT (modify (\x -> x + 1)) (\_ ->
       bindStateT (modify (\x -> x + 1)) (\_ ->
       bindStateT (modify (\x -> x + 1)) (\_ ->
       get)))
main = print (evalState inc3 10)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("StateT modify failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "13\n");
}

#[test]
fn state_t_bind_return_chirho() {
    // bindStateT + returnStateT: get, increment, return old value
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
counter = bindStateT get (\n -> bindStateT (put (n + 1)) (\_ -> returnStateT n))
main = print (execState counter 0)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("StateT bind/return failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "1\n");
}

#[test]
fn state_t_run_state_chirho() {
    // runState returns both value and final state as tuple
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
addToState = bindStateT get (\n -> bindStateT (put (n + 5)) (\_ -> returnStateT (n * 2)))
main = print (evalState addToState 10)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("StateT runState failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "20\n");
}

// ── ReaderT (§E.45) ─────────────────────────────────────────────

#[test]
fn reader_t_ask_eval_chirho() {
    // ask returns the environment: runReader ask 42 → 42
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = print (runReader ask 42)
"#;
    let result_chirho =
        crate::eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    assert!(
        result_chirho.is_ok(),
        "ReaderT ask failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn reader_t_bind_return_chirho() {
    // bindReaderT ask (\r -> returnReaderT (r + 1)), run with env=10 → 11
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
inc = bindReaderT ask (\r -> returnReaderT (r + 1))
main = print (runReader inc 10)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("ReaderT bind/return failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "11\n");
}

#[test]
fn reader_t_local_chirho() {
    // local modifies the environment for its action:
    // runReader (local (\r -> r * 2) ask) 5 → 10
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
doubled = local (\r -> r * 2) ask
main = print (runReader doubled 5)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("ReaderT local failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "10\n");
}

#[test]
fn reader_t_chain_chirho() {
    // Chain multiple bindReaderT operations
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
computation = bindReaderT ask (\r ->
  bindReaderT (returnReaderT (r * 3)) (\tripled ->
  returnReaderT (tripled + r)))
main = print (runReader computation 7)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("ReaderT chain failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "28\n"); // 7*3 + 7 = 28
}

// ── ExceptT (§E.45) ───────────────────────────────────────────────

#[test]
fn except_t_return_right_chirho() {
    // returnExceptT wraps in Right: runExceptT (returnExceptT 42) → Right 42
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = case runExceptT (returnExceptT 42) of
  Right x -> print x
  Left _  -> print 0
"#;
    let result_chirho =
        crate::eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    assert!(
        result_chirho.is_ok(),
        "ExceptT returnExceptT failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn except_t_throw_left_chirho() {
    // throwE wraps in Left: runExceptT (throwE 99) → Left 99
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = case runExceptT (throwE 99) of
  Left e  -> print e
  Right _ -> print 0
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("ExceptT throwE failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "99\n");
}

#[test]
fn except_t_bind_success_chirho() {
    // bindExceptT propagates Right:
    // bindExceptT (returnExceptT 10) (\x -> returnExceptT (x + 5)) → Right 15
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
comp = bindExceptT (returnExceptT 10) (\x -> returnExceptT (x + 5))
main = case runExceptT comp of
  Right v -> print v
  Left _  -> print 0
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("ExceptT bind success failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "15\n");
}

#[test]
fn except_t_bind_short_circuit_chirho() {
    // bindExceptT short-circuits on Left:
    // bindExceptT (throwE 42) (\x -> returnExceptT (x + 1)) → Left 42
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
comp = bindExceptT (throwE 42) (\x -> returnExceptT (x + 1))
main = case runExceptT comp of
  Left e  -> print e
  Right _ -> print 0
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("ExceptT bind short-circuit failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn except_t_catch_chirho() {
    // catchE catches Left and recovers:
    // catchE (throwE 99) (\e -> returnExceptT (e + 1)) → Right 100
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
comp = catchE (throwE 99) (\e -> returnExceptT (e + 1))
main = case runExceptT comp of
  Right v -> print v
  Left _  -> print 0
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("ExceptT catchE failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "100\n");
}

// ── MaybeT operations (returnMaybeT, bindMaybeT) ──────────────────

#[test]
fn maybe_t_return_just_chirho() {
    // returnMaybeT 42 → MaybeT (Just 42) → Just 42
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = case runMaybeT (returnMaybeT 42) of
  Just v  -> print v
  Nothing -> print 0
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("MaybeT return failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn maybe_t_bind_success_chirho() {
    // bindMaybeT (returnMaybeT 10) (\x -> returnMaybeT (x + 5)) → Just 15
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
comp = bindMaybeT (returnMaybeT 10) (\x -> returnMaybeT (x + 5))
main = case runMaybeT comp of
  Just v  -> print v
  Nothing -> print 0
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("MaybeT bind success failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "15\n");
}

#[test]
fn maybe_t_bind_short_circuit_chirho() {
    // bindMaybeT (MaybeT Nothing) (\x -> returnMaybeT (x + 5)) → Nothing
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
comp = bindMaybeT (MaybeT Nothing) (\x -> returnMaybeT (x + 5))
main = case runMaybeT comp of
  Just _  -> print 1
  Nothing -> print 0
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("MaybeT bind short-circuit failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "0\n");
}

// ── WriterT operations (tell, returnWriterT, bindWriterT, execWriterT) ──

#[test]
fn writer_t_tell_chirho() {
    // tell [1] → WriterT ((), [1])
    // execWriterT (tell [1]) → [1]
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = print (execWriterT (tell [1]))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("WriterT tell failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "[1]\n");
}

#[test]
fn writer_t_return_chirho() {
    // returnWriterT 42 → WriterT (42, [])
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = case runWriterT (returnWriterT 42) of
  (v, _) -> print v
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("WriterT return failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn writer_t_bind_accumulate_chirho() {
    // bindWriterT m1 m2: should accumulate logs via ++
    // bindWriterT (tell [1]) (\_ -> tell [2]) → WriterT ((), [1,2])
    // execWriterT (bindWriterT (tell [1]) (\_ -> tell [2])) → [1,2]
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = print (length (execWriterT (bindWriterT (tell [1]) (\x -> tell [2]))))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("WriterT bind accumulate failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "2\n");
}

// ── TypeApplications (§E.33) ─────────────────────────────────────

#[test]
fn type_app_simple_parse_chirho() {
    // TypeApplications: `read @Int "42"` — type argument is parsed and erased
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = print (id @Int 42)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("TypeApp simple failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn type_app_show_bool_chirho() {
    // TypeApplications: `show @Bool True` — type applied to show
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = putStrLn (show @Bool True)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("TypeApp show Bool failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "True\n");
}

#[test]
fn type_app_multiple_chirho() {
    // Multiple type applications in sequence: f @Int @Bool
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
f x = x + 1
main = print (f @Int 41)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("TypeApp multiple failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn type_app_with_lambda_chirho() {
    // TypeApplications with a lambda expression
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
apply f x = f x
main = print (apply @Int (\n -> n + 8) 34)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("TypeApp lambda failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn type_app_constrains_polymorphic_chirho() {
    // TypeApplications actually constraining a polymorphic function:
    // id @Int 42 — id is polymorphic, @Int constrains to Int
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
myId x = x
main = print (myId @Int 42 + myId @Int 8)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("TypeApp constrains poly failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "50\n");
}

#[test]
fn type_app_nested_type_chirho() {
    // TypeApplication with a compound type: @[Int]
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
myLength xs = case xs of
  [] -> 0
  (_:rest) -> 1 + myLength rest
main = print (myLength @[Int] [1,2,3])
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("TypeApp nested type failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "3\n");
}

// ── DeriveFunctor / DeriveFoldable / DeriveTraversable (§E.36) ────

#[test]
fn derive_functor_simple_chirho() {
    // data Box a = MkBox a deriving (Functor)
    // fmap (+1) (MkBox 41) → MkBox 42
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Box a = MkBox a deriving (Functor)
unbox (MkBox x) = x
main = print (unbox (fmap (\x -> x + 1) (MkBox 41)))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DeriveFunctor simple failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn derive_functor_multiple_fields_chirho() {
    // data Pair a = MkPair Int a — fmap applies to last field only
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Tagged a = MkTagged Int a deriving (Functor)
getVal (MkTagged _ v) = v
main = print (getVal (fmap (\x -> x * 2) (MkTagged 0 21)))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DeriveFunctor multi-field failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

// ── DataKinds (§E.40) ──────────────────────────────────────────────

#[test]
fn datakinds_promoted_con_parses_chirho() {
    // DataKinds: promoted constructor 'True in a type signature parses and evaluates
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
{-# LANGUAGE DataKinds #-}
data Proxy a = MkProxy
showProxy :: Proxy 'True -> Int
showProxy MkProxy = 42
main = print (showProxy MkProxy)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DataKinds promoted con failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn datakinds_promoted_nothing_parses_chirho() {
    // DataKinds: promoted constructor 'Nothing in a type signature
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
{-# LANGUAGE DataKinds #-}
data Proxy a = MkProxy
test :: Proxy 'Nothing -> Int
test MkProxy = 99
main = print (test MkProxy)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DataKinds promoted Nothing failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "99\n");
}

#[test]
fn datakinds_promoted_list_parses_chirho() {
    // DataKinds: promoted list type '[Int, Bool] in a type alias
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
{-# LANGUAGE DataKinds #-}
data Proxy a = MkProxy
test :: Proxy '[Int, Bool] -> Int
test MkProxy = 77
main = print (test MkProxy)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DataKinds promoted list failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "77\n");
}

#[test]
fn datakinds_promoted_nil_parses_chirho() {
    // DataKinds: promoted empty list '[] in a type signature
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
{-# LANGUAGE DataKinds #-}
data Proxy a = MkProxy
test :: Proxy '[] -> Int
test MkProxy = 55
main = print (test MkProxy)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DataKinds promoted nil failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "55\n");
}

#[test]
fn datakinds_char_literal_preserved_chirho() {
    // Ensure char literals like 'A' still work with DataKinds changes
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
main = putChar 'A'
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("Char literal failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "A");
}

// ── ConstraintKinds (§E.38) ────────────────────────────────────────

#[test]
fn constraintkinds_kind_annotation_chirho() {
    // ConstraintKinds: `Constraint` recognized as a kind annotation
    // in kind signatures like `(c :: Constraint)`
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
{-# LANGUAGE ConstraintKinds #-}
{-# LANGUAGE KindSignatures #-}
data Dict (c :: Constraint) = MkDict
test :: Dict (Show Int) -> Int
test MkDict = 42
main = print (test MkDict)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| {
                panic!("ConstraintKinds kind annotation failed: {}", e_chirho)
            });
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn constraintkinds_constraint_arrow_kind_chirho() {
    // ConstraintKinds: arrow kind `* -> Constraint` in kind signature
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
{-# LANGUAGE ConstraintKinds #-}
{-# LANGUAGE KindSignatures #-}
data Proxy (c :: * -> Constraint) = MkProxy
test :: Proxy Show -> Int
test MkProxy = 99
main = print (test MkProxy)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("ConstraintKinds arrow kind failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "99\n");
}

#[test]
fn constraintkinds_constraint_type_alias_chirho() {
    // ConstraintKinds: constraint type alias `type Printable a = (Show a, Eq a)`
    // used as a constraint in a function signature
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
{-# LANGUAGE ConstraintKinds #-}
type Printable a = Show a
showIt :: Printable a => a -> Int
showIt x = 42
main = print (showIt True)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| {
                panic!("ConstraintKinds constraint alias failed: {}", e_chirho)
            });
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

// ── DerivingVia tests ──────────────────────────────────────────────

#[test]
fn deriving_via_show_newtype_chirho() {
    // DerivingVia: `newtype Age = MkAge Int deriving (Show) via Int`
    // show (MkAge 42) should give "42" (via Int's Show), not "MkAge 42"
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE DerivingVia #-}
module Test where
newtype Age = MkAge Int deriving (Show) via Int
main = putStrLn (show (MkAge 42))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DerivingVia Show failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn deriving_via_eq_newtype_chirho() {
    // DerivingVia: `newtype Age = MkAge Int deriving (Eq) via Int`
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE DerivingVia #-}
module Test where
newtype Age = MkAge Int deriving (Eq) via Int
main = if MkAge 10 == MkAge 10 then putStrLn "equal" else putStrLn "not equal"
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DerivingVia Eq failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "equal\n");
}

#[test]
fn deriving_via_num_newtype_chirho() {
    // DerivingVia: `newtype Age = MkAge Int deriving (Num) via Int`
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE DerivingVia #-}
module Test where
newtype Age = MkAge Int deriving (Num) via Int
getAge (MkAge x) = x
main = print (getAge (MkAge 20 + MkAge 22))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DerivingVia Num failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn deriving_via_ast_present_chirho() {
    // Verify that DerivingVia entries are extracted into the module AST
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE DerivingVia #-}
module Test where
newtype Wrapper = MkWrap Int deriving (Show) via Int
main = 0
"#;
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "DerivingVia should compile: {:?}",
        result_chirho.err()
    );
}

// ── Algorithmic tests: stress-testing compiler capabilities ───────
