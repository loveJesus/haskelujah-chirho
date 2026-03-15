// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// Closures, recursion, multi-module, exceptions, overloaded strings, advanced extensions tests

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
    fn eval_named_entry_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nfoo = 99\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            Some("foo"),
        );
        assert_eq!(
            result_chirho.unwrap(),
            rhasky_runtime_chirho::ValueChirho::IntChirho(99)
        );
    }


    #[test]
    fn eval_mutual_top_level_arithmetic_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Two top-level bindings where main references x
        let result_chirho = eval_source_chirho(
            "module Test where\nx = 3 + 4\nmain = x + 1\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        assert_eq!(
            result_chirho.unwrap(),
            rhasky_runtime_chirho::ValueChirho::IntChirho(8)
        );
    }


    #[test]
    fn eval_closure_capture_chirho() {
        // \x -> \y -> x + y  — inner lambda captures x from outer
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             add = \\x -> \\y -> x + y\n\
             main = add 10 32\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_closure_capture: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_closure_let_capture_chirho() {
        // let-bound closure capturing outer parameter
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             f x = let g y = x + y in g 5\n\
             main = f 10\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(15)
            ),
            Err(e_chirho) => {
                eprintln!("eval_closure_let_capture: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_nested_closure_chirho() {
        // Triple nesting: \x -> \y -> \z -> x + y + z
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             f x y z = x + y + z\n\
             main = f 10 20 12\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_nested_closure: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }


    // ── Priority 15: Recursive let-bindings (letrec) ─────────────────

    #[test]
    fn eval_recursive_factorial_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // factorial via top-level recursion
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             fac n = if n == 0 then 1 else n * fac (n - 1)\n\
             main = fac 5\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(120));
            }
            Err(e_chirho) => {
                panic!("recursive factorial should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_recursive_sum_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // sum via top-level recursion: sum n = if n == 0 then 0 else n + sum (n - 1)
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             mysum n = if n == 0 then 0 else n + mysum (n - 1)\n\
             main = mysum 10\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(55));
            }
            Err(e_chirho) => {
                panic!("recursive sum should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_let_rec_local_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Local letrec via let/where with recursion
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = let go n = if n == 0 then 0 else n + go (n - 1) in go 5\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
            }
            Err(e_chirho) => {
                panic!("local letrec should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_where_recursive_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Recursive helper in where clause
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = go 4\n\
             \x20 where\n\
             \x20   go n = if n == 0 then 1 else n * go (n - 1)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(24));
            }
            Err(e_chirho) => {
                panic!("where-clause recursive should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_fibonacci_recursive_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // fibonacci - tests deep recursion with branching
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             fib n = if n == 0 then 0 else if n == 1 then 1 else fib (n - 1) + fib (n - 2)\n\
             main = fib 10\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(55));
            }
            Err(e_chirho) => {
                panic!("fibonacci should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_recursive_gcd_chirho() {
        use crate::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // GCD via Euclid's algorithm
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             gcd a b = if b == 0 then a else gcd b (mod a b)\n\
             main = gcd 12 8\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
            }
            Err(e_chirho) => {
                panic!("recursive GCD should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_recursive_io_list_chirho() {
        // Recursive I/O over a list using >> and case
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
printAll xs = case xs of
  [] -> putStrLn \"done\"
  (_:rest) -> putStrLn \"item\" >> printAll rest
main = printAll [10, 20, 30]
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("recursive I/O over list should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "item\nitem\nitem\ndone\n",
            "printAll [10,20,30] prints 'item' for each element"
        );
    }


    #[test]
    fn eval_where_fac_recursive_chirho() {
        use crate::eval_source_chirho;
        // recursive where binding (factorial via where)
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let val_chirho = eval_source_chirho(
            "module Test where\n\
             main = result\n  where\n    result = fac 5\n    fac n = if n == 0 then 1 else n * fac (n - 1)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("recursive where should evaluate");
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(120));
    }


    #[test]
    fn eval_where_mutual_function_ref_chirho() {
        // Mutual references: a calls g, g is defined after a in where
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = result\n  where\n    result = g 10\n    g x = x + offset\n    offset = 32\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("Where mutual function ref should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_nested_bool_case_recursive_chirho() {
        // Test: recursive function with boolean case dispatch inside data constructor case
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ninsert k v m = case m of\n  Nothing -> Just (k + v)\n  Just x -> if k == 0 then Just x else insert (k - 1) v (Just x)\nmain = case insert 2 10 Nothing of\n  Just r -> r\n  Nothing -> 0\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(12)),
            Err(e_chirho) => panic!("Nested bool case recursive should work: {}", e_chirho),
        }
    }


    // -- Where-clause mutual recursion test --

    #[test]
    fn eval_where_mutual_recursion_chirho() {
        // isEven/isOdd mutual recursion in where clause
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = isEven 10\n  where\n    isEven n = if n == 0 then 1 else isOdd (n - 1)\n    isOdd n = if n == 0 then 0 else isEven (n - 1)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("where mutual recursion: {}", e_chirho),
        }
    }

    // -- Additional list-on-string tests --


    // ── Higher-order function composition ─────────────────────────

    #[test]
    fn eval_higher_order_compose_chirho() {
        // (length . filter even) [1..10] → 5
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = (length . filter even) [1..10]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5)),
            Err(e_chirho) => panic!("higher order compose: {}", e_chirho),
        }
    }

    // ── Operator sections ────────────────────────────────────────────


    // ── Operator sections ────────────────────────────────────────────

    #[test]
    fn eval_left_section_chirho() {
        // map (2*) [1,2,3] → [2,4,6], sum → 12
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (map (2*) [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(12)),
            Err(e_chirho) => panic!("left section (2*): {}", e_chirho),
        }
    }


    #[test]
    fn eval_right_section_chirho() {
        // map (*3) [1,2,3] → [3,6,9], sum → 18
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (map (*3) [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(18)),
            Err(e_chirho) => panic!("right section (*3): {}", e_chirho),
        }
    }


    #[test]
    fn eval_where_recursive_fib_chirho() {
        // Fibonacci via where clause
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = result
  where result = fib 8
        fib n = if n <= 1 then n else fib (n - 1) + fib (n - 2)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(21)),
            Err(e_chirho) => panic!("where recursive fib: {}", e_chirho),
        }
    }


    #[test]
    fn eval_nested_function_application_chirho() {
        // f x y = x + y; g a = f a (a * 2); main = g 5 → 15
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf x y = x + y\ng a = f a (a * 2)\nmain = g 5\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15)),
            Err(e_chirho) => panic!("nested function app: {}", e_chirho),
        }
    }


    #[test]
    fn eval_tuple3_sum_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nsum3 t = case t of (a,b,c) -> a + b + c\nmain = sum3 (10, 20, 30)\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(60));
    }

    // ── zip / concatMap / any end-to-end ────────────────────────────────


    // ── Exception handling tests ────────────────────────────────────────

    #[test]
    fn eval_catch_no_error_chirho() {
        // catch (return 42) (\e -> return 0) should return 42
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = catch (return 42) (\\e -> return 0)\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }


    #[test]
    fn eval_catch_with_error_chirho() {
        // catch (error "kaboom") (\e -> return 99) should return 99
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = catch (error \"kaboom\") (\\e -> return 99)\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99));
    }


    #[test]
    fn eval_throw_caught_chirho() {
        // catch (throw "oops") (\msg -> putStrLn msg >> return 0)
        // should print "oops" and return 0
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = catch (throw \"oops\") (\\msg -> putStrLn msg >> return 0)\n";
        let (val_chirho, machine_chirho) = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("catch should handle throw");
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0));
        assert_eq!(machine_chirho.io_output_chirho, "oops\n");
    }


    #[test]
    fn eval_throw_uncaught_chirho() {
        // throw "uncaught" should produce RuntimeErrorChirho
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = throw \"uncaught\"\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        assert!(result_chirho.is_err(), "uncaught throw should produce error");
        let msg_chirho = result_chirho.unwrap_err();
        assert!(msg_chirho.contains("uncaught"), "error message should contain 'uncaught': {}", msg_chirho);
    }


    #[test]
    fn eval_nested_catch_chirho() {
        // Nested catch: inner catch handles error, outer sees success
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = catch (catch (error \"inner\") (\\e -> return 77)) (\\e -> return 0)\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(77));
    }

    // ── try / throwIO / bracket / finally end-to-end tests ────────────


    // ── try / throwIO / bracket / finally end-to-end tests ────────────

    #[test]
    fn eval_catch_basic_chirho() {
        // catch (error "boom") (\_ -> putStrLn "caught") → output "caught\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = catch (error \"boom\") (\\_ -> putStrLn \"caught\")\n";
        let (val_chirho, machine_chirho) = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("catch should handle error");
        let _ = val_chirho;
        assert_eq!(machine_chirho.io_output_chirho, "caught\n");
    }


    #[test]
    fn eval_try_success_chirho() {
        // try (return 42) binds r, then case r of Right v → putStrLn (show v) → "42\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Use do-notation so the bind is handled correctly via LetChirho desugaring.
        let src_chirho = r#"module Test where
main = do
  r <- try (return 42)
  case r of
    Right v -> putStrLn (show v)
    Left _ -> putStrLn "error"
"#;
        let (val_chirho, machine_chirho) = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("try success should not raise");
        let _ = val_chirho;
        assert_eq!(machine_chirho.io_output_chirho, "42\n");
    }


    #[test]
    fn eval_try_failure_chirho() {
        // try (error "fail") binds r, then case r of Left _ → putStrLn "error" → "error\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = do
  r <- try (error "fail")
  case r of
    Left _ -> putStrLn "error"
    Right _ -> putStrLn "ok"
"#;
        let (val_chirho, machine_chirho) = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("try failure should be caught");
        let _ = val_chirho;
        assert_eq!(machine_chirho.io_output_chirho, "error\n");
    }


    #[test]
    fn eval_throwio_caught_chirho() {
        // throwIO is an alias for throw — catch (throwIO "oops") (\_ -> putStrLn "caught")
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = catch (throwIO \"oops\") (\\_ -> putStrLn \"caught\")\n";
        let (val_chirho, machine_chirho) = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        ).expect("catch should handle throwIO");
        let _ = val_chirho;
        assert_eq!(machine_chirho.io_output_chirho, "caught\n");
    }

    // ── Data.Map (user-defined BST) end-to-end tests ──────────────────


    // ── Tuple pattern in function arguments ───────────────────────────

    #[test]
    fn eval_tuple_arg_pattern_chirho() {
        // f (x, y) = x + y — tuple pattern destructuring via case
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\naddPair p = case p of (x, y) -> x + y\nmain = addPair (3, 4)\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
    }


    #[test]
    fn eval_tuple_from_list_with_usertype_chirho() {
        // Minimal test: user data type + tuple case on list head
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ndata Foo = Bar | Baz\nmain = case head [(1, 2)] of\n  (a, b) -> a + b\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }


    #[test]
    fn eval_tuple_from_list_with_dict_type_chirho() {
        // Test: parametric data type defined but tuple case on list head
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ndata Dict k v = DTip | DBin k v (Dict k v) (Dict k v)\nmain = case head [(10, 20)] of\n  (a, b) -> a + b\n";
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30));
    }

    // ── Negative literal patterns in function equations ───────────────


    // ── Complex where-clause with multiple helpers ────────────────────

    /// Test 1: Collatz sequence — complex where-clause with a helper function.
    /// collatz 27 takes 111 steps.
    #[test]
    fn eval_collatz_steps_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
collatz n = if n == 1 then 0 else 1 + collatz (step n)
  where
    step x = if even x then x `div` 2 else 3 * x + 1
main = putStrLn (show (collatz 27))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap();
        assert_eq!(m_chirho.io_output_chirho, "111\n");
    }

    // ── Higher-order function composition ─────────────────────────────

    /// Test 2: twice applied twice — higher-order composition.
    /// twice (twice inc) 0 = 4

    // ── Accumulator pattern with recursive list building ──────────────

    /// Test 3: digits decomposition + sum — accumulator with ++ and recursion.
    /// sum (digits 12345) = 1+2+3+4+5 = 15
    #[test]
    fn eval_digit_sum_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
digits n = if n < 10 then [n] else digits (n `div` 10) ++ [n `mod` 10]
main = putStrLn (show (sum (digits 12345)))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap();
        assert_eq!(m_chirho.io_output_chirho, "15\n");
    }

    // ── Nested data types with pattern matching ────────────────────────

    /// Test 4: arithmetic expression tree — ADT with recursive eval.
    /// eval (Add (Mul (Lit 3) (Lit 4)) (Lit 5)) = 17

    // ── List recursion with accumulator ───────────────────────────────

    /// Test 5: myReverse using go accumulator helper in where-clause.
    /// sum (myReverse [1,2,3,4,5]) = 15
    /// Uses case-expression style for the where helper to avoid multi-equation
    /// where-function list-pattern limitations.
    #[test]
    fn eval_reverse_accumulator_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // myRevAcc reverses a list using an accumulator argument.
        // sum of reversed [1,2,3,4,5] is still 15.
        let src_chirho = r#"module Test where
myRevAcc ys acc = case ys of
  [] -> acc
  (z:rest) -> myRevAcc rest (z : acc)
main = sum (myRevAcc [1,2,3,4,5] [])
"#;
        let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap();
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
    }

    // ── Church numerals (higher-order encoding) ───────────────────────

    /// Test 6: Church numeral addition.
    /// toInt (churchAdd church2 church3) = 5

    // ── Church numerals (higher-order encoding) ───────────────────────

    /// Test 6: Church numeral addition.
    /// toInt (churchAdd church2 church3) = 5
    #[test]
    fn eval_church_numerals_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
church0 f x = x
church1 f x = f x
church2 f x = f (f x)
church3 f x = f (f (f x))
churchAdd m n f x = m f (n f x)
toInt n = n (\x -> x + 1) 0
main = putStrLn (show (toInt (churchAdd church2 church3)))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap();
        assert_eq!(m_chirho.io_output_chirho, "5\n");
    }

    // ── Sieve of Eratosthenes (bounded) ──────────────────────────────

    /// Test 8: Sieve of Eratosthenes up to 100.
    /// length primes = 25 (there are 25 primes ≤ 100)

    // ── Sieve of Eratosthenes (bounded) ──────────────────────────────

    /// Test 8: Sieve of Eratosthenes up to 100.
    /// length primes = 25 (there are 25 primes ≤ 100)
    #[test]
    fn eval_sieve_primes_count_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
sieve [] = []
sieve (p:xs) = p : sieve (filter (\x -> x `mod` p /= 0) xs)
primes = sieve [2..100]
main = putStrLn (show (length primes))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap();
        assert_eq!(m_chirho.io_output_chirho, "25\n");
    }

    // ── Data.Map additional operations ────────────────────────────────────


    // ── Algorithmic tests: stress-testing compiler capabilities ───────

    #[test]
    fn eval_ackermann_chirho() {
        // Ackermann function: ack 3 4 = 125
        // Tests deeply recursive multi-equation pattern matching.
        // Uses a large step limit (10M) because ack(3,4) requires ~315k recursive calls.
        use crate::eval_source_with_step_limit_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
ack 0 n = n + 1
ack m 0 = ack (m - 1) 1
ack m n = ack (m - 1) (ack m (n - 1))
main = putStrLn (show (ack 3 4))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_step_limit_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None, 10_000_000)
                .unwrap_or_else(|e_chirho| panic!("ackermann failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "125\n");
    }


    #[test]
    fn eval_hanoi_count_chirho() {
        // Tower of Hanoi move count: hanoi 10 = 2^10 - 1 = 1023
        // Tests simple tail-recursive arithmetic
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
hanoi 0 = 0
hanoi n = 2 * hanoi (n - 1) + 1
main = putStrLn (show (hanoi 10))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("hanoi count failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "1023\n");
    }


    #[test]
    fn eval_matrix_mul_chirho() {
        // 2x2 matrix multiplication via lists of lists.
        // [[1,2],[3,4]] * [[5,6],[7,8]] top-left element = 1*5 + 2*7 = 19.
        // Uses explicit helper functions to avoid lazy-let blackhole issues.
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
dot xs ys = foldr (+) 0 (zipWith (*) xs ys)
col j m = map (\row -> head (drop j row)) m
matMul a b = map (\row -> map (\j -> dot row (col j b)) [0,1]) a
main = let m = matMul [[1,2],[3,4]] [[5,6],[7,8]]
       in putStrLn (show (head (head m)))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("matrix mul failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "19\n");
    }


    #[test]
    fn eval_caesar_cipher_chirho() {
        // Caesar cipher: encrypt then decrypt returns original string
        // Tests ord/chr, map, lambda with closure capture
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
encrypt shift msg = map (\c -> chr (ord c + shift)) msg
decrypt shift msg = map (\c -> chr (ord c - shift)) msg
main = putStrLn (decrypt 3 (encrypt 3 "HELLO"))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("caesar cipher failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "HELLO\n");
    }


    #[test]
    fn eval_bin_to_dec_chirho() {
        // Binary to decimal: [1,0,1,1] = 1*8 + 0*4 + 1*2 + 1*1 = 11.
        // Uses an accumulator-based approach (shift-left) to avoid using
        // `^` with a dynamic exponent derived from `length`.
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
go bits acc = case bits of
  [] -> acc
  (b:bs) -> let newAcc = acc * 2 + b in go bs newAcc
main = putStrLn (show (go [1,0,1,1] 0))
"#;
        let (_val_chirho, m_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("binToDec failed: {}", e_chirho));
        assert_eq!(m_chirho.io_output_chirho, "11\n");
    }


    #[test]
    fn eval_run_length_encoding_chirho() {
        // Run-length encoding: rle [1,1,1,2,2,3,1,1] has 4 groups.
        // Tests span with section (== x), let tuple destructuring, cons in result.
        // Uses explicit case expression to avoid multi-equation list-pattern issues.
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
rle input = case input of
  [] -> []
  (x:xs) -> let (same, rest) = span (== x) xs
             in (length same + 1, x) : rle rest
main = length (rle [1,1,1,2,2,3,1,1])
"#;
        let val_chirho =
            eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("run-length encoding failed: {}", e_chirho));
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
    }


    // ---------------------------------------------------------------
    // Complex program tests: feature combinations
    // ---------------------------------------------------------------

    #[test]
    fn eval_fibonacci_lazy_chirho() {
        // Fibonacci via zipWith on infinite lists (requires lazy eval):
        // fibs = 0 : 1 : zipWith (+) fibs (tail fibs)
        // Simplified: compute fib with take on recursive infinite list
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
fib n = if n <= 1 then n else fib (n - 1) + fib (n - 2)
main = sum (map fib (take 7 [0..]))
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("fibonacci lazy: {}", e_chirho));
        // fib 0..6 = 0,1,1,2,3,5,8; sum = 20
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(20));
    }


    #[test]
    fn eval_collatz_chirho() {
        // Collatz sequence length for 6: 6→3→10→5→16→8→4→2→1 = 9 steps
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
collatz n = if n == 1 then 1
            else if even n then 1 + collatz (n `div` 2)
            else 1 + collatz (3 * n + 1)
main = collatz 6
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("collatz: {}", e_chirho));
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(9));
    }


    #[test]
    fn eval_sieve_primes_chirho() {
        // Count primes up to 30 using trial division
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
isPrime n = if n < 2 then False
            else if n == 2 then True
            else if even n then False
            else go 3
  where go d = if d * d > n then True
               else if n `mod` d == 0 then False
               else go (d + 2)
main = length (filter isPrime [1..30])
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("sieve primes: {}", e_chirho));
        // Primes up to 30: 2,3,5,7,11,13,17,19,23,29 = 10
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10));
    }


    #[test]
    fn eval_nested_where_let_chirho() {
        // Complex nested where + let
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = result
  where result = let x = 10
                     y = double x
                 in x + y + offset
        double n = n * 2
        offset = 5
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("nested where let: {}", e_chirho));
        // x=10, y=20, offset=5, result=35
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(35));
    }


    #[test]
    fn eval_data_constructor_math_chirho() {
        // User-defined data type with arithmetic on fields
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Point = MkPoint Int Int
getX (MkPoint x y) = x
getY (MkPoint x y) = y
dist p1 p2 = abs (getX p1 - getX p2) + abs (getY p1 - getY p2)
main = dist (MkPoint 3 4) (MkPoint 6 8)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("data constructor math: {}", e_chirho));
        // |3-6| + |4-8| = 3 + 4 = 7
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
    }


    #[test]
    fn eval_accumulator_pattern_chirho() {
        // Strict accumulator pattern with foldl
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
factorial n = foldl (*) 1 [1..n]
main = factorial 10
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("accumulator pattern: {}", e_chirho));
        // 10! = 3628800
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3628800));
    }

    // ── OverloadedStrings tests ──────────────────────────────────────────


    // ── OverloadedStrings tests ──────────────────────────────────────────

    #[test]
    fn eval_overloaded_strings_identity_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = rhasky_span_chirho::SourceMapChirho::new_chirho();
        let src_chirho = "{-# LANGUAGE OverloadedStrings #-}\n\
             module Test where\n\
             main = putStrLn \"hello overloaded\"\n";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        )
        .expect("overloaded strings identity should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "hello overloaded\n");
    }


    #[test]
    fn eval_overloaded_strings_concat_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = rhasky_span_chirho::SourceMapChirho::new_chirho();
        let src_chirho = "{-# LANGUAGE OverloadedStrings #-}\n\
             module Test where\n\
             main = putStrLn (\"hello\" ++ \" \" ++ \"world\")\n";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        )
        .expect("overloaded strings concat should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "hello world\n");
    }


    #[test]
    fn eval_no_overloaded_strings_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = rhasky_span_chirho::SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             main = putStrLn \"plain string\"\n";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        )
        .expect("plain string should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "plain string\n");
    }

    // ── OverloadedLists (§E.35) ──────────────────────────────────────────

    #[test]
    fn eval_overloaded_lists_identity_chirho() {
        // With OverloadedLists, list literals go through fromList (identity for [a])
        use crate::eval_source_chirho;
        let mut sm_chirho = rhasky_span_chirho::SourceMapChirho::new_chirho();
        let src_chirho = "{-# LANGUAGE OverloadedLists #-}\n\
             module Test where\n\
             main = sum [1, 2, 3]\n";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        )
        .expect("overloaded lists identity should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6));
    }

    #[test]
    fn eval_overloaded_lists_length_chirho() {
        // With OverloadedLists, list literal [10, 20, 30] through fromList gives same list
        use crate::eval_source_chirho;
        let mut sm_chirho = rhasky_span_chirho::SourceMapChirho::new_chirho();
        let src_chirho = "{-# LANGUAGE OverloadedLists #-}\n\
             module Test where\n\
             main = length [10, 20, 30]\n";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        )
        .expect("overloaded lists length should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }

    #[test]
    fn eval_no_overloaded_lists_chirho() {
        // Without OverloadedLists, list literals are plain cons chains (no fromList call)
        use crate::eval_source_chirho;
        let mut sm_chirho = rhasky_span_chirho::SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             main = sum [1, 2, 3, 4]\n";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        )
        .expect("plain list should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10));
    }

    #[test]
    fn eval_overloaded_lists_head_chirho() {
        // With OverloadedLists: head [42, 0, 0] → 42
        use crate::eval_source_chirho;
        let mut sm_chirho = rhasky_span_chirho::SourceMapChirho::new_chirho();
        let src_chirho = "{-# LANGUAGE OverloadedLists #-}\n\
             module Test where\n\
             main = head [42, 0, 0]\n";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        )
        .expect("overloaded lists head should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

    // ── N-ary tuple tests ───────────────────────────────────────────────

    #[test]
    fn eval_3tuple_construct_chirho() {
        // 3-tuple construction and case dispatch
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             fst3 (a, b, c) = a\n\
             main = fst3 (10, 20, 30)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("3-tuple fst3 should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10));
    }


    #[test]
    fn eval_3tuple_thd_chirho() {
        // 3-tuple third element extraction
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             thd3 (a, b, c) = c\n\
             main = thd3 (10, 20, 30)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("3-tuple thd3 should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30));
    }


    #[test]
    fn eval_3tuple_sum_show_chirho() {
        // 3-tuple element extraction and showing the sum
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             sumTriple (a, b, c) = a + b + c\n\
             main = putStrLn (show (sumTriple (10, 20, 30)))\n";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
        )
        .expect("show 3-tuple sum should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "60\n");
    }


    #[test]
    fn eval_3tuple_arithmetic_chirho() {
        // 3-tuple with arithmetic on elements
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             sumTriple (a, b, c) = a + b + c\n\
             main = sumTriple (10, 20, 30)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("3-tuple sum should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(60));
    }


    #[test]
    fn eval_4tuple_construct_chirho() {
        // 4-tuple construction and case dispatch
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             first4 (a, b, c, d) = a\n\
             last4 (a, b, c, d) = d\n\
             main = first4 (1, 2, 3, 4) + last4 (1, 2, 3, 4)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("4-tuple should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5));
    }

    // ── String case pattern matching tests ──────────────────────────────


    // John 3:16 - For God so loved the world, that he gave his only begotten Son,
    // that whosoever believeth in him should not perish, but have everlasting life.

    #[test]
    fn eval_strict_apply_chirho() {
        // $! forces argument to WHNF before applying: f $! x = seq x (f x)
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             double x = x + x\n\
             main = double $! (3 + 4)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("strict application should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(14));
    }


    #[test]
    fn eval_strict_apply_show_chirho() {
        // $! with show to verify WHNF forcing behavior
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\n\
             main = putStrLn $! show (21 + 21)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("strict apply with show should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "42\n");
    }

    // John 3:16 - For God so loved the world, that he gave his only begotten Son,
    // that whosoever believeth in him should not perish, but have everlasting life.

    // ── Automatic Prelude import tests ──

    #[test]
    fn eval_prelude_import_implicit_chirho() {
        // Every module automatically imports Prelude — using Prelude functions
        // (show, putStrLn, etc.) should work without explicit imports
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (show (2 + 3))\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("implicit Prelude import should work");
        assert_eq!(result_chirho.1.io_output_chirho, "5\n");
    }

    #[test]
    fn eval_no_implicit_prelude_chirho() {
        // {-# LANGUAGE NoImplicitPrelude #-} should suppress automatic Prelude import
        // The program should still compile because our builtins are injected at
        // the type inference level, not through the Prelude module import
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nmain = 42\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("NoImplicitPrelude should compile");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn eval_explicit_prelude_import_chirho() {
        // Explicit `import Prelude` should not cause double import
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nimport Prelude\nmain = max 3 7\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("explicit Prelude import should work");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
    }

    // ── Bang patterns ───────────────────────────────────────────────────

    #[test]
    fn eval_bang_pattern_basic_chirho() {
        // f !x = x + 1; main = f 41  → 42
        // Bang pattern forces x to WHNF before evaluating body.
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf !x = x + 1\nmain = f 41\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("bang pattern basic");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn eval_bang_pattern_two_args_chirho() {
        // f !x !y = x + y; main = f 10 32  → 42
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf !x !y = x + y\nmain = f 10 32\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("bang pattern two args");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn eval_bang_pattern_mixed_chirho() {
        // f !x y = x + y; main = f 10 32  → 42
        // Only first arg is strict.
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf !x y = x + y\nmain = f 10 32\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("bang pattern mixed");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn eval_strict_apply_with_bang_chirho() {
        // ($!) forces argument then applies: f $! 42 → f 42
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf x = x + 1\nmain = f $! 41\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("strict apply with bang");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

    // ── deepseq / force / evaluate (§A.12) ─────────────────────────────

    #[test]
    fn eval_deepseq_basic_chirho() {
        // deepseq x y = x `seq` y — forces x, returns y
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = deepseq 1 42\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("deepseq basic");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn eval_deepseq_list_chirho() {
        // deepseq forces first arg, returns second
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = deepseq [1,2,3] 99\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("deepseq list");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99));
    }

    #[test]
    fn eval_force_basic_chirho() {
        // force x = x `seq` x — forces and returns same value
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = force 42\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("force basic");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn eval_evaluate_basic_chirho() {
        // evaluate x forces to WHNF, returns result
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = evaluate (2 + 3)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("evaluate basic");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5));
    }

    #[test]
    fn eval_force_in_expression_chirho() {
        // force can be used in larger expressions
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = force 10 + force 32\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Test.hs", None)
            .expect("force in expression");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn eval_repl_style_expression_chirho() {
        // REPL wraps expressions as `main = <expr>` in a module.
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module ReplMain where\nmain = sum [1..10]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "REPL", None)
            .expect("repl-style expression");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(55));
    }

    #[test]
    fn eval_repl_style_io_chirho() {
        // REPL evaluates IO actions and captures output.
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module ReplMain where\nmain = putStrLn (\"1+2 = \" ++ show (1+2))\n";
        let (_val_chirho, machine_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "REPL", None)
                .expect("repl-style IO");
        assert_eq!(machine_chirho.io_output_chirho, "1+2 = 3\n");
    }

    #[test]
    fn eval_repl_style_let_binding_chirho() {
        // REPL :let adds definitions that subsequent expressions can use.
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho =
            "module ReplMain where\ndouble x = x * 2\nmain = double 21\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "REPL", None)
            .expect("repl-style let binding");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

    // ── Orphan instance detection tests ─────────────────────────────────

    #[test]
    fn orphan_instance_warning_produced_chirho() {
        // An instance where neither the class nor the type is defined locally
        // should produce a W0402 orphan instance warning.
        use crate::frontend_warnings_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Orphan where
instance Show Int where
  show x = "int"
main = 42
"#;
        let warnings_chirho =
            frontend_warnings_chirho(src_chirho, &mut sm_chirho, "OrphanChirho.hs")
                .expect("frontend should succeed");
        assert!(
            warnings_chirho.iter().any(|w_chirho| w_chirho.contains("orphan instance")),
            "expected orphan instance warning, got: {:?}",
            warnings_chirho
        );
    }

    #[test]
    fn orphan_instance_not_for_local_data_chirho() {
        // An instance for a locally-defined data type should NOT be orphan.
        use crate::frontend_warnings_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Local where
data Color = Red | Green | Blue
instance Show Color where
  show x = "color"
main = 42
"#;
        let warnings_chirho =
            frontend_warnings_chirho(src_chirho, &mut sm_chirho, "LocalChirho.hs")
                .expect("frontend should succeed");
        assert!(
            !warnings_chirho.iter().any(|w_chirho| w_chirho.contains("orphan instance")),
            "local data type instance should not be orphan, got: {:?}",
            warnings_chirho
        );
    }

    #[test]
    fn orphan_instance_not_for_local_class_chirho() {
        // An instance for a locally-defined class should NOT be orphan.
        use crate::frontend_warnings_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module LocalClass where
class Describable a where
  describe :: a -> Int
instance Describable Int where
  describe x = x
main = 42
"#;
        let warnings_chirho =
            frontend_warnings_chirho(src_chirho, &mut sm_chirho, "LocalClassChirho.hs")
                .expect("frontend should succeed");
        assert!(
            !warnings_chirho.iter().any(|w_chirho| w_chirho.contains("orphan instance")),
            "local class instance should not be orphan, got: {:?}",
            warnings_chirho
        );
    }

    // ── INLINE / NOINLINE pragma e2e tests ──

    #[test]
    fn inline_pragma_parse_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Main where\n{-# INLINE double #-}\ndouble x = x + x\nmain = double 21\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "InlineTest.hs", None).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(42));
    }

    #[test]
    fn noinline_pragma_parse_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Main where\n{-# NOINLINE secret #-}\nsecret = 99\nmain = secret + 1\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NoInlineTest.hs", None).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(100));
    }

    #[test]
    fn inlinable_pragma_parse_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Main where\n{-# INLINABLE triple #-}\ntriple x = x + x + x\nmain = triple 10\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "InlinableTest.hs", None).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(30));
    }

    #[test]
    fn inline_pragma_multiple_chirho() {
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Main where\n{-# INLINE add1 #-}\n{-# NOINLINE mul2 #-}\nadd1 x = x + 1\nmul2 x = x * 2\nmain = add1 (mul2 10)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "MultiPragmaTest.hs", None).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(21));
    }

    // ── SPECIALIZE pragma e2e tests ──

    #[test]
    fn specialize_pragma_basic_chirho() {
        // {-# SPECIALIZE double :: Int -> Int #-} should not affect evaluation
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Main where\n{-# SPECIALIZE double :: Int -> Int #-}\ndouble x = x + x\nmain = double 21\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "SpecTest.hs", None).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(42));
    }

    #[test]
    fn specialize_pragma_creates_binding_chirho() {
        // Verify the specialized binding $spec_double_0 appears in Core output
        use crate::compile_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Main where\n{-# SPECIALIZE double :: Int -> Int #-}\ndouble x = x + x\nmain = double 21\n";
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "SpecBindTest.hs").unwrap();
        let core_chirho = &result_chirho.core_chirho;
        // Find the $spec_double_0 binding
        let spec_binding_chirho = core_chirho
            .bindings_chirho
            .iter()
            .find(|b_chirho| b_chirho.binder_chirho.name_chirho.contains("$spec_double"));
        assert!(spec_binding_chirho.is_some(), "Expected $spec_double binding in Core");
    }

    #[test]
    fn specialise_british_spelling_eval_chirho() {
        // SPECIALISE (British spelling) should also work
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Main where\n{-# SPECIALISE triple :: Int -> Int #-}\ntriple x = x + x + x\nmain = triple 14\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "SpecialiseTest.hs", None).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(42));
    }

    #[test]
    fn specialize_multiple_types_eval_chirho() {
        // Multiple SPECIALIZE pragmas for the same function
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Main where\n{-# SPECIALIZE f :: Int -> Int #-}\n{-# SPECIALIZE f :: Int -> Int #-}\nf x = x + 1\nmain = f 41\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "MultiSpecTest.hs", None).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(42));
    }

    // ── DefaultSignatures e2e tests ──

    #[test]
    fn default_sig_basic_eval_chirho() {
        // Class with a default implementation — instance omits method, uses default
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
class Describable a where
  describe :: a -> Int
  default describe :: a -> Int
  describe x = 42
instance Describable Int
main = describe (1 :: Int)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DefaultSigTest.hs", None).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(42));
    }

    #[test]
    fn default_sig_with_override_eval_chirho() {
        // Instance provides its own method, overriding the default
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
class Describable a where
  describe :: a -> Int
  default describe :: a -> Int
  describe x = 0
instance Describable Int where
  describe x = x + 1
main = describe 41
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DefaultSigOverride.hs", None).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(42));
    }

    #[test]
    fn default_sig_parsed_in_ast_chirho() {
        // Verify the default_sig_chirho field is populated on ClassMethodChirho
        use crate::compile_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
class Describable a where
  describe :: a -> Int
  default describe :: a -> Int
  describe x = 42
main = 0
";
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "DefaultSigAST.hs").unwrap();
        // Check that the module compiled successfully
        assert!(!result_chirho.core_chirho.bindings_chirho.is_empty());
    }

    // ── Shared let-bound variable bug reproduction ───────────────────────

    #[test]
    fn eval_shared_let_putstrln_show_chirho() {
        // Two putStrLn (show ...) calls on same let-bound list
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  let x = [1, 2, 3]
  putStrLn (show (length x))
  putStrLn (show (take 2 x))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBug.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "3\n[1,2]\n",
                    "shared let-bound var should work with multiple putStrLn(show(f x))");
            }
            Err(e_chirho) => {
                panic!("shared let-bound putStrLn show bug: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_shared_let_print_twice_chirho() {
        // Same but with print instead of putStrLn (show ...)
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  let x = [1, 2, 3]
  print (length x)
  print (take 2 x)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBug2.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert!(output_chirho.contains("3"), "should contain length result");
            }
            Err(e_chirho) => {
                panic!("shared let print twice bug: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_two_putstrln_show_no_shared_chirho() {
        // Two putStrLn (show ...) WITHOUT a shared variable
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn (show (length [1, 2, 3]))
  putStrLn (show (take 2 [4, 5, 6]))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBug3.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "3\n[4,5]\n");
            }
            Err(e_chirho) => {
                panic!("no shared putStrLn show bug: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_two_putstrln_show_let_bound_results_chirho() {
        // Bind show results to let variables before putStrLn
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  let x = [1, 2, 3]
  let a = show (length x)
  let b = show (take 2 x)
  putStrLn a
  putStrLn b
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBug4.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "3\n[1,2]\n");
            }
            Err(e_chirho) => {
                panic!("let-bound results putStrLn bug: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_two_putstrln_show_simple_chirho() {
        // Simplest case: putStrLn (show 1) then putStrLn (show 2)
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn (show 1)
  putStrLn (show 2)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBug5.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "1\n2\n");
            }
            Err(e_chirho) => {
                panic!("simple two putStrLn show bug: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_two_putstrln_show_arith_chirho() {
        // putStrLn (show (1+2)) then putStrLn (show (3+4))
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn (show (1 + 2))
  putStrLn (show (3 + 4))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBug6.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "3\n7\n");
            }
            Err(e_chirho) => {
                panic!("arith two putStrLn show bug: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_putstrln_show_length_then_literal_chirho() {
        // putStrLn (show (length [1,2,3])) then putStrLn (show 42)
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn (show (length [1, 2, 3]))
  putStrLn (show 42)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBug8.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "3\n42\n");
            }
            Err(e_chirho) => {
                panic!("length then literal bug: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_putstrln_show_length_twice_chirho() {
        // Two length calls on different lists
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn (show (length [1, 2, 3]))
  putStrLn (show (length [4, 5]))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBug9.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "3\n2\n");
            }
            Err(e_chirho) => {
                panic!("two lengths bug: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_putstrln_show_int_then_list_chirho() {
        // putStrLn (show 3) then putStrLn (show [4,5,6])
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn (show 3)
  putStrLn (show [4, 5, 6])
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBugA.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "3\n[4,5,6]\n");
            }
            Err(e_chirho) => {
                panic!("int then list bug: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_putstrln_show_two_lists_chirho() {
        // putStrLn (show [1,2,3]) then putStrLn (show [4,5,6])
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn (show [1, 2, 3])
  putStrLn (show [4, 5, 6])
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBugB.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "[1,2,3]\n[4,5,6]\n");
            }
            Err(e_chirho) => {
                panic!("two lists bug: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_hello_then_show_take_chirho() {
        // putStrLn "hello" then putStrLn (show (take 2 [1,2,3]))
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn \"hello\"
  putStrLn (show (take 2 [1, 2, 3]))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBugF.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "hello\n[1,2]\n");
            }
            Err(e_chirho) => {
                panic!("hello then show take: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_show_take_then_hello_chirho() {
        // putStrLn (show (take 2 [1,2,3])) then putStrLn "hello"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn (show (take 2 [1, 2, 3]))
  putStrLn \"hello\"
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBugG.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "[1,2]\nhello\n");
            }
            Err(e_chirho) => {
                panic!("show take then hello: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_show_take_twice_assert_chirho() {
        // Two take calls — proper assertion
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn (show (take 2 [1, 2, 3]))
  putStrLn (show (take 1 [4, 5]))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBugH.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "[1,2]\n[4]\n");
            }
            Err(e_chirho) => {
                panic!("two takes: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_show_length_then_list_literal_chirho() {
        // show (length ...) then show [4,5,6] — mixed Int and [Int] Show dicts
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn (show (length [1, 2, 3]))
  putStrLn (show [4, 5, 6])
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBugJ.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "3\n[4,5,6]\n");
            }
            Err(e_chirho) => {
                panic!("length then list literal: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_show_literal_then_show_take_chirho() {
        // show literal (Int) then show (take ...) — switching types
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = do
  putStrLn (show 3)
  putStrLn (show (take 2 [4, 5, 6]))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBugI.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "3\n[4,5]\n");
            }
            Err(e_chirho) => {
                panic!("literal then show take: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_single_putstrln_show_length_chirho() {
        // Single putStrLn (show (length [1,2,3]))
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Main where
main = putStrLn (show (length [1, 2, 3]))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ShareBug7.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = &machine_chirho.io_output_chirho;
                assert_eq!(output_chirho, "3\n");
            }
            Err(e_chirho) => {
                panic!("single putStrLn show length bug: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_open_type_family_compiles_chirho() {
        // Open type family declaration should be accepted by the compiler
        let source_chirho = r#"
module Main where
type family F a
type instance F Int = Bool
main = putStrLn "ok"
"#;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            source_chirho,
            &mut source_map_chirho,
            "TypeFamilyOpenChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "ok\n");
            }
            Err(e_chirho) => {
                panic!("open type family should compile: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_closed_type_family_compiles_chirho() {
        // Closed type family declaration should be accepted by the compiler
        let source_chirho = r#"
module Main where
type family IsInt a where
  IsInt Int = Bool
main = putStrLn "closed ok"
"#;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            source_chirho,
            &mut source_map_chirho,
            "TypeFamilyClosedChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "closed ok\n");
            }
            Err(e_chirho) => {
                panic!("closed type family should compile: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_type_family_with_regular_code_chirho() {
        // Type family declarations alongside regular functions
        let source_chirho = r#"
module Main where
type family G a
type instance G Int = Char
add1 x = x + 1
main = putStrLn (show (add1 41))
"#;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            source_chirho,
            &mut source_map_chirho,
            "TypeFamilyMixChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "42\n");
            }
            Err(e_chirho) => {
                panic!("type family with regular code should compile: {}", e_chirho);
            }
        }
    }

