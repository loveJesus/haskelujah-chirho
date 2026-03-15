// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// IO operations, do-notation, putStrLn, IORef, interact, mapM_, when/unless tests

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
    fn eval_do_notation_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // do { putStrLn "hello"; putStrLn "world" }
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             main = do\n  putStrLn \"hello\"\n  putStrLn \"world\"\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "hello\nworld\n");
            }
            Err(e_chirho) => {
                eprintln!("eval_do_notation: {}", e_chirho);
                assert!(result_chirho.is_ok(), "do notation should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_composition_compile_chirho() {
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // (f . g) x = f (g x) — verify composition compiles
        let result_chirho = compile_source_chirho(
            "module Test where\n\
             double x = x + x\n\
             succ x = x + 1\n\
             main = (double . succ) 20\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "composition should compile: {:?}",
            result_chirho.err()
        );
    }


    #[test]
    fn eval_putstrln_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // main = putStrLn "hello"
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             main = putStrLn \"hello\"\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok((val_chirho, machine_chirho)) => {
                // putStrLn returns IO () which we model as IntChirho(0)
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0));
                // Check captured I/O output
                assert_eq!(machine_chirho.io_output_chirho, "hello\n");
            }
            Err(e_chirho) => {
                eprintln!("eval_putstrln: {}", e_chirho);
                assert!(result_chirho.is_ok(), "putStrLn should evaluate: {}", e_chirho);
            }
        }
    }


    #[test]
    fn eval_putstrln_variable_chirho() {
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // greet msg = putStrLn msg
        // main = greet "world"
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             greet msg = putStrLn msg\n\
             main = greet \"world\"\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "world\n");
            }
            Err(e_chirho) => {
                eprintln!("eval_putstrln_variable: {}", e_chirho);
                assert!(result_chirho.is_ok(), "putStrLn with variable should evaluate: {}", e_chirho);
            }
        }
    }

    // ── Priority 15: Recursive let-bindings (letrec) ─────────────────


    #[test]
    fn eval_infix_then_operator_chirho() {
        // >> as an infix operator chains I/O actions
        use crate::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn \"10\" >> putStrLn \"20\" >> putStrLn \"30\"
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect(">> chain should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "10\n20\n30\n",
            ">> chains putStrLn calls"
        );
    }


    // ── IORef tests ──

    #[test]
    fn eval_ioref_new_read_chirho() {
        // newIORef 42, readIORef, show → "42\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r <- newIORef 42\n  v <- readIORef r\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "42\n");
            }
            Err(e_chirho) => panic!("IORef new+read should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_ioref_modify_chirho() {
        // modifyIORef r add1 should increment: newIORef 41, modifyIORef, readIORef → "42\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nadd1 x = x + 1\nmain = do\n  r <- newIORef 41\n  modifyIORef r add1\n  v <- readIORef r\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "42\n");
            }
            Err(e_chirho) => panic!("IORef modify should work: {}", e_chirho),
        }
    }

    // ── Data.Map tests ──


    // ── Data.IORef ──

    #[test]
    fn eval_ioref_new_read_show_chirho() {
        // newIORef 42, readIORef, show → "42\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r <- newIORef 42\n  v <- readIORef r\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "42\n");
            }
            Err(e_chirho) => panic!("ioref new/read/show: {}", e_chirho),
        }
    }


    #[test]
    fn eval_ioref_write_overwrite_chirho() {
        // newIORef 1, writeIORef r 99, readIORef r → 99
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r <- newIORef 1\n  writeIORef r 99\n  v <- readIORef r\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "99\n");
            }
            Err(e_chirho) => panic!("ioref write overwrite: {}", e_chirho),
        }
    }


    #[test]
    fn eval_ioref_do_modify_lambda_chirho() {
        // modifyIORef r (\x -> x * 2), newIORef 21, readIORef → 42
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r <- newIORef 21\n  modifyIORef r (\\x -> x + x)\n  v <- readIORef r\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "42\n");
            }
            Err(e_chirho) => panic!("ioref do modify lambda: {}", e_chirho),
        }
    }


    #[test]
    fn eval_ioref_two_refs_independent_chirho() {
        // Two separate IORefs hold independent values
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r1 <- newIORef 10\n  r2 <- newIORef 20\n  writeIORef r1 99\n  v1 <- readIORef r1\n  v2 <- readIORef r2\n  putStrLn (show (v1 + v2))\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "119\n");
            }
            Err(e_chirho) => panic!("ioref two refs independent: {}", e_chirho),
        }
    }


    #[test]
    fn eval_ioref_counter_increment_chirho() {
        // IORef counter: start at 0, increment 3 times via modifyIORef, read → 3
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nincr x = x + 1\nmain = do\n  c <- newIORef 0\n  modifyIORef c incr\n  modifyIORef c incr\n  modifyIORef c incr\n  v <- readIORef c\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "3\n");
            }
            Err(e_chirho) => panic!("ioref counter increment: {}", e_chirho),
        }
    }

    // ── when/unless ──


    // ── when/unless ──

    #[test]
    fn eval_when_true_output_chirho() {
        // when True (putStrLn "yes") → "yes\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = when True (putStrLn \"yes\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "yes\n");
            }
            Err(e_chirho) => panic!("when True output: {}", e_chirho),
        }
    }


    #[test]
    fn eval_when_false_silent_chirho() {
        // when False (putStrLn "no") → ""
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = when False (putStrLn \"no\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "");
            }
            Err(e_chirho) => panic!("when False silent: {}", e_chirho),
        }
    }


    #[test]
    fn eval_unless_false_output_chirho() {
        // unless False (putStrLn "ran") → "ran\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = unless False (putStrLn \"ran\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "ran\n");
            }
            Err(e_chirho) => panic!("unless False output: {}", e_chirho),
        }
    }

    // ── flip ──


    #[test]
    fn eval_multiple_io_operations_chirho() {
        // do { putStr "a"; putStr "b"; putStrLn "c" } → "abc\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  putStr \"a\"\n  putStr \"b\"\n  putStrLn \"c\"\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "abc\n");
            }
            Err(e_chirho) => panic!("multiple io: {}", e_chirho),
        }
    }


    #[test]
    fn eval_multi_line_do_io_chirho() {
        // do { putStrLn (show 1); putStrLn (show 2); putStrLn (show 3) } → "1\n2\n3\n"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  putStrLn (show 1)\n  putStrLn (show 2)\n  putStrLn (show 3)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "1\n2\n3\n");
            }
            Err(e_chirho) => panic!("multi line do io: {}", e_chirho),
        }
    }

    // ── fromJust / swap / mapDelete fix ──


    #[test]
    fn eval_import_data_ioref_chirho() {
        // import Data.IORef for mutable state
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import Data.IORef (newIORef, readIORef, writeIORef)
main = do
  ref <- newIORef 10
  writeIORef ref 42
  v <- readIORef ref
  putStrLn (show v)
";
        let (_, machine_chirho) = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        let output_chirho = machine_chirho.io_output_chirho.clone();
        assert_eq!(output_chirho, "42\n");
    }


    // ── mapM_ / forM_ IO sequencing ────────────────────────────────────

    #[test]
    #[allow(non_snake_case)]
    fn eval_builtin_mapM_io_chirho() {
        // mapM_ with putStrLn over a list
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
printItem x = putStrLn (show x)
main = mapM_ printItem [1, 2, 3]
";
        let (_, machine_chirho) = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        let output_chirho = machine_chirho.io_output_chirho.clone();
        assert_eq!(output_chirho, "1\n2\n3\n");
    }

    // ── sequence_ / void / guard / interact tests ──────────────────────


    // ── sequence_ / void / guard / interact tests ──────────────────────

    #[test]
    fn eval_sequence_io_chirho() {
        // sequence_ [putStrLn "a", putStrLn "b", putStrLn "c"]
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sequence_ [putStrLn \"a\", putStrLn \"b\", putStrLn \"c\"]
";
        let (_, machine_chirho) = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        let output_chirho = machine_chirho.io_output_chirho.clone();
        assert_eq!(output_chirho, "a\nb\nc\n");
    }


    #[test]
    fn eval_void_io_chirho() {
        // void (putStrLn "hello") — should still print, result discarded
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = void (putStrLn \"hello\")
";
        let (_, machine_chirho) = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        let output_chirho = machine_chirho.io_output_chirho.clone();
        assert_eq!(output_chirho, "hello\n");
    }


    #[test]
    fn eval_builtin_when_true_io_chirho() {
        // when True (putStrLn "yes") — uses builtin when binding
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = when True (putStrLn \"yes\")
";
        let (_, machine_chirho) = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        let output_chirho = machine_chirho.io_output_chirho.clone();
        assert_eq!(output_chirho, "yes\n");
    }


    #[test]
    fn eval_builtin_unless_true_io_chirho() {
        // unless True (putStrLn "skipped") — should produce no output
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = unless True (putStrLn \"skipped\")
";
        let (_, machine_chirho) = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        let output_chirho = machine_chirho.io_output_chirho.clone();
        assert_eq!(output_chirho, "");
    }


    #[test]
    fn eval_guard_true_chirho() {
        // guard True >> putStrLn "passed"
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = guard True >> putStrLn \"passed\"
";
        let (_, machine_chirho) = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
        let output_chirho = machine_chirho.io_output_chirho.clone();
        assert_eq!(output_chirho, "passed\n");
    }

    // ── Show Either / Ordering ──────────────────────────────────────────


    // ── ST monad ──────────────────────────────────────────────────────────

    #[test]
    fn eval_st_new_read_chirho() {
        // newSTRef 42 then readSTRef → 42
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = let ref = newSTRef 42 in readSTRef ref\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("ST new+read should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_st_write_read_chirho() {
        // newSTRef 10, writeSTRef ref 99, readSTRef ref → 99
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = let r = newSTRef 10 in seq (writeSTRef r 99) (readSTRef r)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("ST write+read should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_runst_chirho() {
        // runST (let ref = newSTRef 0 in seq (writeSTRef ref 42) (readSTRef ref)) → 42
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = runST (let ref = newSTRef 0 in seq (writeSTRef ref 42) (readSTRef ref))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("runST should work: {}", e_chirho),
        }
    }


    #[test]
    fn eval_stref_basic_chirho() {
        // runST (do { ref <- newSTRef 0; writeSTRef ref 42; readSTRef ref }) → 42
        // Expressed without do-notation via let+seq for evaluation ordering:
        // runST (let ref = newSTRef 0 in seq (writeSTRef ref 42) (readSTRef ref))
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = runST (let ref = newSTRef 0 in seq (writeSTRef ref 42) (readSTRef ref))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("eval_stref_basic_chirho failed: {}", e_chirho),
        }
    }


    #[test]
    fn eval_stref_modify_chirho() {
        // runST (do { ref <- newSTRef 10; modifySTRef ref (+5); readSTRef ref }) → 15
        // Expressed via let+seq for evaluation ordering
        use crate::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\naddFive x = x + 5\nmain = runST (let ref = newSTRef 10 in seq (modifySTRef ref addFive) (readSTRef ref))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15)),
            Err(e_chirho) => panic!("eval_stref_modify_chirho failed: {}", e_chirho),
        }
    }

    // ── Data.List: nub / sortBy / isPrefixOf / replicate end-to-end ─────


    // ---------------------------------------------------------------
    // IO control flow: when, unless, mapM_, and lazy utility e2e
    // ---------------------------------------------------------------

    #[test]
    fn eval_io_when_true_prelude_chirho() {
        // when True (putStrLn "yes") should print "yes"
        use crate::eval_source_with_input_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = when True (putStrLn "yes")
"#;
        let (_, machine_chirho) =
            eval_source_with_input_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None, &[])
                .unwrap_or_else(|e_chirho| panic!("when True: {}", e_chirho));
        assert_eq!(machine_chirho.io_output_chirho, "yes\n");
    }


    #[test]
    fn eval_io_when_false_prelude_chirho() {
        // when False (putStrLn "no") should print nothing
        use crate::eval_source_with_input_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = when False (putStrLn "no")
"#;
        let (_, machine_chirho) =
            eval_source_with_input_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None, &[])
                .unwrap_or_else(|e_chirho| panic!("when False: {}", e_chirho));
        assert_eq!(machine_chirho.io_output_chirho, "");
    }


    #[test]
    fn eval_io_unless_false_prelude_chirho() {
        // unless False (putStrLn "yes") should print "yes"
        use crate::eval_source_with_input_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = unless False (putStrLn "yes")
"#;
        let (_, machine_chirho) =
            eval_source_with_input_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None, &[])
                .unwrap_or_else(|e_chirho| panic!("unless False: {}", e_chirho));
        assert_eq!(machine_chirho.io_output_chirho, "yes\n");
    }


    #[test]
    fn eval_io_mapm_list_prelude_chirho() {
        // mapM_ putStrLn ["a","b","c"] should print "a\nb\nc\n"
        use crate::eval_source_with_input_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = mapM_ putStrLn ["a","b","c"]
"#;
        let (_, machine_chirho) =
            eval_source_with_input_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None, &[])
                .unwrap_or_else(|e_chirho| panic!("mapM_ putStrLn: {}", e_chirho));
        assert_eq!(machine_chirho.io_output_chirho, "a\nb\nc\n");
    }


    #[test]
    fn eval_io_show_computed_list_chirho() {
        // show a computed list of Ints
        use crate::eval_source_with_input_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = putStrLn (show [2 * 1, 2 * 2, 2 * 3])
"#;
        let (_, machine_chirho) =
            eval_source_with_input_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None, &[])
                .unwrap_or_else(|e_chirho| panic!("show computed list: {}", e_chirho));
        assert_eq!(machine_chirho.io_output_chirho, "[2,4,6]\n");
    }

    // ── STM (Software Transactional Memory) e2e tests ──

    #[test]
    fn eval_stm_new_read_tvar_chirho() {
        // newTVar 42 >>= readTVar → 42
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = do
  tv <- newTVarIO 42
  v <- readTVarIO tv
  putStrLn (show v)
"#;
        let (_, machine_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("STM newTVar/readTVar: {}", e_chirho));
        assert_eq!(machine_chirho.io_output_chirho, "42\n");
    }

    #[test]
    fn eval_stm_write_read_tvar_chirho() {
        // newTVar 0 >>= writeTVar 99 >>= readTVar → 99
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = do
  tv <- newTVarIO 0
  writeTVar tv 99
  v <- readTVarIO tv
  putStrLn (show v)
"#;
        let (_, machine_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("STM writeTVar: {}", e_chirho));
        assert_eq!(machine_chirho.io_output_chirho, "99\n");
    }

    #[test]
    fn eval_stm_atomically_chirho() {
        // atomically (newTVar 10 >>= readTVar) → 10
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = do
  v <- atomically (do { tv <- newTVar 10; readTVar tv })
  putStrLn (show v)
"#;
        let (_, machine_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("STM atomically: {}", e_chirho));
        assert_eq!(machine_chirho.io_output_chirho, "10\n");
    }

    #[test]
    fn eval_stm_multiple_tvars_chirho() {
        // Create two TVars, write to both, read and sum
        use crate::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = do
  tv1 <- newTVarIO 30
  tv2 <- newTVarIO 12
  v1 <- readTVarIO tv1
  v2 <- readTVarIO tv2
  putStrLn (show (v1 + v2))
"#;
        let (_, machine_chirho) =
            eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
                .unwrap_or_else(|e_chirho| panic!("STM multiple TVars: {}", e_chirho));
        assert_eq!(machine_chirho.io_output_chirho, "42\n");
    }

