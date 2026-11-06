// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! IO actions, do-notation, IORef and Control.Monad combinators (row 700).

use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn eval_do_let_simple_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // do-notation with let binding — last stmt uses bound variable
    let src_chirho = "module Test where\nmain = do\n  let x = 5\n  x + 10\n";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
            );
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
            );
        }
        Err(e_chirho) => panic!("do let multi should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_do_bind_uses_bound_var_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // GHC rejects binding a plain number as a monadic action. The intended
    // program returns the computed number from IO; GHC's observable result is 8.
    let src_chirho = "module Test where\nfChirho xChirho = pure (xChirho + 1)\nmain :: IO Int\nmain = do\n  yChirho <- fChirho 3\n  pure (yChirho * 2)\n";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(8)
            );
        }
        Err(e_chirho) => panic!("do bind should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_getline_echo_chirho() {
    use crate::eval_source_with_input_chirho;
    // do { line <- getLine; putStrLn line } with input "hello world"
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_val_chirho, machine_chirho) = eval_source_with_input_chirho(
        "module Test where\nmain = do\n  line <- getLine\n  putStrLn line\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
        &["hello world"],
    )
    .expect("getLine echo should evaluate");
    assert_eq!(machine_chirho.io_output_chirho, "hello world\n");
}

#[test]
fn eval_getline_bind_putstrln_chirho() {
    use crate::eval_source_with_input_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_val_chirho, machine_chirho) = eval_source_with_input_chirho(
        "module Test where\nmain = getLine >>= putStrLn\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
        &["bind echo"],
    )
    .expect("getLine >>= putStrLn should evaluate");
    assert_eq!(machine_chirho.io_output_chirho, "bind echo\n");
}

#[test]
fn eval_getchar_chirho() {
    use crate::eval_source_with_input_chirho;
    // getChar with input "ABC" should return 'A'
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (val_chirho, _machine_chirho) = eval_source_with_input_chirho(
        "module Test where\nmain = getChar\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
        &["ABC"],
    )
    .expect("getChar should evaluate");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('A')
    );
}

#[test]
fn eval_putstr_no_newline_chirho() {
    use crate::eval_source_with_machine_chirho;
    // putStr should not add newline
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_val_chirho, machine_chirho) = eval_source_with_machine_chirho(
        "module Test where\nmain = do\n  putStr \"hello\"\n  putStr \" world\"\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("putStr should evaluate");
    assert_eq!(machine_chirho.io_output_chirho, "hello world");
}

#[test]
fn eval_writefile_chirho() {
    use crate::eval_source_with_machine_chirho;
    // writeFile captures path and content to io_output
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_val_chirho, machine_chirho) = eval_source_with_machine_chirho(
        "module Test where\nmain = writeFile \"test.txt\" \"contents\"\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("writeFile should evaluate");
    assert_eq!(
        machine_chirho.io_output_chirho,
        "[writeFile:test.txt]contents"
    );
}

#[test]
fn eval_print_int_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = print 42\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "True\n");
        }
        Err(e_chirho) => panic!("print True should work: {}", e_chirho),
    }
}

#[test]
fn eval_print_char_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_value_chirho, machine_chirho) = eval_source_with_machine_chirho(
        "module Test where\nmain = print 'x'\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("print Char should evaluate");
    assert_eq!(machine_chirho.io_output_chirho, "'x'\n");
}

#[test]
fn eval_do_multi_print_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = do\n  putStrLn \"one\"\n  putStrLn \"two\"\n  putStrLn \"three\"\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
fn eval_when_true_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nimport Control.Monad (when)\nmain = when True (putStrLn \"yes\")\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        "module Test where\nimport Control.Monad (when)\nmain = when False (putStrLn \"no\")\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        "module Test where\nimport Control.Monad (unless)\nmain = unless False (putStrLn \"run\")\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        "module Test where\nimport Control.Monad (unless)\nmain = unless True (putStrLn \"no\")\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "hello\nworld\n")
        }
        Err(e_chirho) => panic!("mapM_ putStrLn should print each element: {}", e_chirho),
    }
}

#[test]
fn eval_putchar_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = do { putChar 'H'; putChar 'i' }\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "Hi"),
        Err(e_chirho) => panic!("putChar should output chars: {}", e_chirho),
    }
}

#[test]
fn eval_dot_compose_io_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Use (.) with IO: putStrLn . show $ 42
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = putStrLn . show $ 42\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "hello\nworld\n")
        }
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "before\nafter\n")
        }
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "executed\n")
        }
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "1\n2\n3\n")
        }
        Err(e_chirho) => panic!("forM_ should work: {}", e_chirho),
    }
}

#[test]
fn eval_do_notation_sequence_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Complex do-notation with let, bind, and sequence
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = do\n  let x = 42\n  putStrLn (show x)\n  let y = x + 8\n  putStrLn (show y)\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "42\n50\n")
        }
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
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "1\n2\n3\n")
        }
        Err(e_chirho) => panic!("builtin mapM_ should work: {}", e_chirho),
    }
}

// ── IORef tests ──

#[test]
fn eval_ioref_write_read_chirho() {
    // newIORef 10, writeIORef r 99, readIORef r → "99\n"
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.IORef (newIORef, readIORef, writeIORef)\nmain = do\n  r <- newIORef 10\n  writeIORef r 99\n  v <- readIORef r\n  putStrLn (show v)\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
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
    let src_chirho = "module Test where\nimport Data.IORef (newIORef, readIORef)\nmain = do\n  r <- newIORef 7\n  v <- readIORef r\n  putStrLn (show v)\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
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
    let src_chirho = "module Test where\nimport Data.IORef (newIORef, readIORef)\nmain = do\n  r1 <- newIORef 10\n  r2 <- newIORef 20\n  v1 <- readIORef r1\n  v2 <- readIORef r2\n  putStrLn (show (v1 + v2))\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_val_chirho, m_chirho)) => {
            let out_chirho = &m_chirho.io_output_chirho;
            assert_eq!(out_chirho, "30\n");
        }
        Err(e_chirho) => panic!("IORef multiple refs should work: {}", e_chirho),
    }
}

// workflow: monadic-dispatch-chirho — INV-001 stdout regression pin: IO
// do-blocks whose first statement is a real effect (putStrLn/print) must NOT
// be routed into another monad's body by blind key inference (the bug where
// `putStrLn "a" >> rest` inferred [Char] -> [] and ran the list-monad body).
#[test]
fn eval_io_do_effect_chains_chirho() {
    use crate::eval_source_with_machine_chirho;
    let cases_chirho: [(&str, &str); 4] = [
        ("main = do\n  putStrLn \"a\"\n  putStrLn \"b\"\n", "a\nb\n"),
        ("main = do\n  putStrLn \"a\"\n  print 3\n", "a\n3\n"),
        (
            "main = do\n  putStrLn \"a\"\n  x <- return (1 + 2)\n  print x\n",
            "a\n3\n",
        ),
        // Direct-argument dispatch inside an IO do-block. NOTE deliberately
        // NOT pinned here: binds on function PARAMETERS (`f e = e >>= k`)
        // have no proven type key yet and take the BindIO value fallback —
        // that is WI-003 signature-driven dispatch territory (prd_chirho.json).
        (
            "main = do\n  print ((Right 4 :: Either String Int) >>= (\\x -> Right (x + x)))\n  print (Just 10 >>= (\\x -> Just (x - 1)))\n",
            "Right 8\nJust 9\n",
        ),
    ];
    for (body_chirho, want_chirho) in cases_chirho {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Test where\n{body_chirho}");
        let result_chirho =
            eval_source_with_machine_chirho(&src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_, machine_chirho)) => assert_eq!(
                machine_chirho.io_output_chirho, want_chirho,
                "IO do-block stdout changed for: {body_chirho}"
            ),
            Err(e_chirho) => panic!("IO do-block must evaluate: {e_chirho}"),
        }
    }
}

// workflow: monadic-dispatch-chirho — INV-001 regression pin (prd_chirho.json)
#[test]
fn eval_io_bind_unchanged_chirho() {
    use crate::eval_source_chirho;
    // IO return/>>= keep their primop fast path (ReturnIOChirho/BindIOChirho)
    // because no $prim_Monad_*_IO body exists — the names fall through.
    let cases_chirho: [(&str, i64); 2] = [
        ("main = return (3 + 4)\n", 7),
        ("main = return 3 >>= (\\x -> return (x + 4))\n", 7),
    ];
    for (body_chirho, want_chirho) in cases_chirho {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Test where\n{body_chirho}");
        let result_chirho = eval_source_chirho(&src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(want_chirho),
                "IO bind fast path changed for: {body_chirho}"
            ),
            Err(e_chirho) => panic!("IO bind fast path must not regress: {e_chirho}"),
        }
    }
}

#[test]
fn eval_let_in_do_complex_chirho() {
    // do { let x = 10; let y = x + 5; putStrLn (show (x + y)) } → "25\n"
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\nmain = do\n  let x = 10\n  let y = x + 5\n  putStrLn (show (x + y))\n";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_val_chirho, m_chirho)) => {
            let out_chirho = &m_chirho.io_output_chirho;
            assert_eq!(out_chirho, "25\n");
        }
        Err(e_chirho) => panic!("let in do complex: {}", e_chirho),
    }
}
