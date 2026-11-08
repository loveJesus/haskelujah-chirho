// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Existing extension controls, grouped by topic.

use crate::{compile_source_chirho, eval_source_chirho, eval_source_with_machine_chirho};
use haskelujah_runtime_chirho::ValueChirho;
use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn mdo_parses_as_do_chirho() {
    // mdo keyword should be treated as do (RecursiveDo)
    let src_chirho = "\
{-# LANGUAGE RecursiveDo #-}
module Test where
main = mdo
  let x = 42
  return x
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Mdo.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn do_where_layout_basic_chirho() {
    // `where` at same/less indentation as `do` block should close the do block
    let src_chirho = "module Test where\nf = do\n  return a\n  where a = 42\nmain = f\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DoWhere.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn do_where_layout_less_indent_chirho() {
    // `where` at less indentation than `do` body should also work
    let src_chirho = "module Test where\nf = do\n  return a\n where a = 42\nmain = f\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DoWhere2.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn do_where_compiles_chirho() {
    // Standard pattern: do with where clause
    let src_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nf = do\n  return a\n  where a = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "DoWhereCmp.hs");
    assert!(
        result_chirho.is_ok(),
        "do-where should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn do_where_deeper_indent_chirho() {
    // `where` at DEEPER indent than do body should still close do block
    let src_chirho = "module Test where\nl3 = do\n  return a\n   where\n   a = 42\nmain = l3\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DoWhere3.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn let_constructor_pattern_bind_chirho() {
    // let Just x = Just 42 in x  should evaluate to 42
    let src_chirho = r#"
module Test where
data MyBox = MkBox Int
main = let MkBox x = MkBox 42 in x
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "LetConPat.hs", None);
    assert!(
        result_chirho.is_ok(),
        "let constructor pat bind should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn let_tuple_constructor_pattern_bind_chirho() {
    // let (a, b) = (10, 32) in a + b  should evaluate to 42
    let src_chirho = r#"
module Test where
main = let (a, b) = (10, 32) in a + b
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "LetTuplePat.hs", None);
    assert!(
        result_chirho.is_ok(),
        "let tuple pat bind should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn toplevel_constructor_pattern_bind_chirho() {
    // Top-level: MkBox val = MkBox 42; main = val
    let src_chirho = r#"
module Test where
data MyBox = MkBox Int
MkBox val = MkBox 42
main = val
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TopConPat.hs", None);
    assert!(
        result_chirho.is_ok(),
        "top-level constructor pat bind should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn where_clause_constructor_pattern_bind_chirho() {
    // where MkBox val = MkBox 42
    let src_chirho = r#"
module Test where
data MyBox = MkBox Int
main = val
  where MkBox val = MkBox 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "WhereConPat.hs", None);
    assert!(
        result_chirho.is_ok(),
        "where constructor pat bind should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn where_clause_tuple_pattern_bind_chirho() {
    // where (x, y) = (10, 32)
    let src_chirho = r#"
module Test where
main = x + y
  where (x, y) = (10, 32)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "WhereTuplePat.hs", None);
    assert!(
        result_chirho.is_ok(),
        "where tuple pat bind should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn where_clause_mixed_fun_and_pat_bind_chirho() {
    // Mix of function bindings and pattern bindings in where clause
    let src_chirho = r#"
module Test where
main = f x + y
  where
    f n = n * 2
    (x, y) = (10, 22)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "WhereMixed.hs", None);
    assert!(
        result_chirho.is_ok(),
        "mixed where bindings should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn do_let_tuple_pattern_bind_chirho() {
    // do { let (x, y) = (10, 32); putStrLn (show (x + y)) }
    let src_chirho = r#"
module Test where
main = do
  let (x, y) = (10, 32)
  return (x + y)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DoLetTuple.hs", None);
    assert!(
        result_chirho.is_ok(),
        "do-let tuple pat should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn do_let_constructor_pattern_bind_chirho() {
    // do { let MkBox x = MkBox 42; return x }
    let src_chirho = r#"
module Test where
data MyBox = MkBox Int
main = do
  let MkBox x = MkBox 42
  return x
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DoLetCon.hs", None);
    assert!(
        result_chirho.is_ok(),
        "do-let constructor pat should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn scc_forward_ref_id_id_chirho() {
    use crate::eval_source_chirho;
    let src_chirho = r#"module Test where
main = iD iD 42
iD x = x
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "SccForward.hs", None);
    assert!(
        result_chirho.is_ok(),
        "SCC forward ref should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn scc_forward_ref_multi_chirho() {
    use crate::eval_source_chirho;
    let src_chirho = r#"module Test where
main = compose incr incr 40
compose f g x = f (g x)
incr x = x + 1
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "SccMulti.hs", None);
    assert!(
        result_chirho.is_ok(),
        "SCC multi forward ref should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn scc_mutual_recursion_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
main = print (isEven 4)
isEven 0 = True
isEven n = isOdd (n - 1)
isOdd 0 = False
isOdd n = isEven (n - 1)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "SccMutual.hs", None)
            .unwrap_or_else(|e_chirho| panic!("SCC mutual recursion should work: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "True\n");
}

#[test]
fn letrec_multi_thunk_capture_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
f a = c where { x = a; y = a; c = x + y }
main = print (f 5)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_v_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "LetrecMulti.hs", None)
            .expect("should eval");
    assert_eq!(
        m_chirho.io_output_chirho.trim(),
        "10",
        "f 5 = x + y = a + a = 10"
    );
}

#[test]
fn letrec_thunk_arithmetic_capture_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
f a = c where { x = a + 1; y = a * 2; c = x + y }
main = print (f 10)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_v_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "LetrecArith.hs", None)
            .expect("should eval");
    assert_eq!(
        m_chirho.io_output_chirho.trim(),
        "31",
        "f 10: x=11, y=20, c=31"
    );
}

#[test]
fn letrec_multi_param_capture_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
f a b = c where { x = a * a; y = b * b; c = x + y }
main = print (f 3 4)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_v_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "LetrecMultiParam.hs", None)
            .expect("should eval");
    assert_eq!(
        m_chirho.io_output_chirho.trim(),
        "25",
        "f 3 4 = 9 + 16 = 25"
    );
}

#[test]
fn case_binder_var_pattern_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"
main = print (case (42 :: Int) of x -> x)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseVar.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho.trim(),
                "42",
                "case var pattern should bind scrutinee"
            );
        }
        Err(e_chirho) => panic!("case var pattern failed: {}", e_chirho),
    }
}

#[test]
fn case_binder_var_arithmetic_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"
f :: Int -> Int
f n = case n of
  x -> x + 1

main :: IO ()
main = print (f 41)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseVar2.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho.trim(),
                "42",
                "case var pattern should bind for arithmetic"
            );
        }
        Err(e_chirho) => panic!("case var arithmetic failed: {}", e_chirho),
    }
}

#[test]
fn case_binder_var_let_in_alt_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"
main :: IO ()
main = do
  let n = 21 :: Int
  print (case n of x -> let y = x * 2 in y)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseVar3.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho.trim(),
                "42",
                "case var should be visible in let-in-alt"
            );
        }
        Err(e_chirho) => panic!("case var let-in-alt failed: {}", e_chirho),
    }
}

#[test]
fn case_binder_con_pattern_still_works_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"
data Color = Red | Green | Blue

f :: Color -> Int
f c = case c of
  Red   -> 1
  Green -> 2
  Blue  -> 3

main :: IO ()
main = print (f Green)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseVar4.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho.trim(),
                "2",
                "constructor case alts should still work"
            );
        }
        Err(e_chirho) => panic!("constructor case alt failed: {}", e_chirho),
    }
}

#[test]
fn case_binder_default_fallthrough_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"
f :: Int -> Int
f n = case n of
  0 -> 100
  x -> x + 1

main :: IO ()
main = do
  print (f 0)
  print (f 41)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseVar5.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let lines_chirho: Vec<&str> = machine_chirho.io_output_chirho.trim().lines().collect();
            assert_eq!(lines_chirho.len(), 2, "should have two outputs");
            assert_eq!(lines_chirho[0], "100", "literal alt should match");
            assert_eq!(lines_chirho[1], "42", "default alt should bind scrutinee");
        }
        Err(e_chirho) => panic!("default fallthrough failed: {}", e_chirho),
    }
}

#[test]
fn case_alt_where_clause_chirho() {
    let src_chirho = r#"
module CaseWhere1 where

f :: Int -> Int
f n = case n of
  x -> result
    where result = x + 1

main :: IO ()
main = print (f 41)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseWhere1.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "42");
        }
        Err(e_chirho) => panic!("case alt where-clause failed: {}", e_chirho),
    }
}

#[test]
fn case_alt_where_multiple_binds_chirho() {
    let src_chirho = r#"
module CaseWhere2 where

f :: Int -> Int
f n = case n of
  x -> a + b
    where
      a = x * 2
      b = x + 1

main :: IO ()
main = print (f 10)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseWhere2.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            // 10 * 2 + (10 + 1) = 20 + 11 = 31
            assert_eq!(machine_chirho.io_output_chirho.trim(), "31");
        }
        Err(e_chirho) => panic!("case alt where multiple binds failed: {}", e_chirho),
    }
}

#[test]
fn case_alt_where_con_pattern_chirho() {
    let src_chirho = r#"
module CaseWhere3 where

data Pair = MkPair Int Int

f :: Pair -> Int
f p = case p of
  MkPair a b -> result
    where result = a + b

main :: IO ()
main = print (f (MkPair 20 22))
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseWhere3.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "42");
        }
        Err(e_chirho) => panic!("case alt where con pattern failed: {}", e_chirho),
    }
}

#[test]
fn case_alt_where_multi_alt_chirho() {
    let src_chirho = r#"
module CaseWhere4 where

f :: Int -> Int
f n = case n of
  0 -> 100
  x -> doubled
    where doubled = x * 2

main :: IO ()
main = do
  print (f 0)
  print (f 21)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseWhere4.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let lines_chirho: Vec<&str> = machine_chirho.io_output_chirho.trim().lines().collect();
            assert_eq!(lines_chirho.len(), 2);
            assert_eq!(lines_chirho[0], "100");
            assert_eq!(lines_chirho[1], "42");
        }
        Err(e_chirho) => panic!("case alt where multi-alt failed: {}", e_chirho),
    }
}

#[test]
fn case_alt_where_function_bind_chirho() {
    let src_chirho = r#"
module CaseWhere5 where

f :: Int -> Int
f n = case n of
  x -> double x
    where double y = y * 2

main :: IO ()
main = print (f 21)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseWhere5.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "42");
        }
        Err(e_chirho) => panic!("case alt where function bind failed: {}", e_chirho),
    }
}

#[test]
fn do_if_then_else_aligned_chirho() {
    let src_chirho = r#"
module DoIfAligned where

main :: IO ()
main = do if True
          then print 42
          else print 0
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "DoIfAligned.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "42");
        }
        Err(e_chirho) => panic!("do if/then/else aligned failed: {}", e_chirho),
    }
}

#[test]
fn do_if_then_else_at_do_indent_chirho() {
    let src_chirho = r#"
module DoIfDo where

main :: IO ()
main = do
  print 1
  if True
    then print 42
    else print 0
  print 3
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "DoIfDo.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let lines_chirho: Vec<&str> = machine_chirho.io_output_chirho.trim().lines().collect();
            assert_eq!(lines_chirho, vec!["1", "42", "3"]);
        }
        Err(e_chirho) => panic!("do if/then/else at do indent failed: {}", e_chirho),
    }
}

#[test]
fn do_nested_if_then_else_chirho() {
    let src_chirho = r#"
module DoNestedIf where

f :: Int -> IO ()
f n = do
  if n > 0
    then if n > 10
           then print 100
           else print n
    else print 0

main :: IO ()
main = do
  f 5
  f 20
  f 0
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "DoNestedIf.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let lines_chirho: Vec<&str> = machine_chirho.io_output_chirho.trim().lines().collect();
            assert_eq!(lines_chirho, vec!["5", "100", "0"]);
        }
        Err(e_chirho) => panic!("do nested if/then/else failed: {}", e_chirho),
    }
}

#[test]
fn case_of_next_line_chirho() {
    let src_chirho = r#"
module CaseOfNext where

f :: Int -> Int
f n = case n
        of 0 -> 100
           _ -> n + 1

main :: IO ()
main = do
  print (f 0)
  print (f 41)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseOfNext.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let lines_chirho: Vec<&str> = machine_chirho.io_output_chirho.trim().lines().collect();
            assert_eq!(lines_chirho, vec!["100", "42"]);
        }
        Err(e_chirho) => panic!("case of next line failed: {}", e_chirho),
    }
}
