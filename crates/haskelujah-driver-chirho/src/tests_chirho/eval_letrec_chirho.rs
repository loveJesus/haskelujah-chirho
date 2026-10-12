// For God so loved the world, that he gave his only begotten Son, that whosoever believeth
// in him should not perish, but have everlasting life. — John 3:16 (KJV)

use crate::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

fn eval_output_chirho(source_chirho: &str, file_name_chirho: &str) -> String {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_value_chirho, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        None,
    )
    .expect("captured letrec program should evaluate");
    machine_chirho.io_output_chirho
}

#[test]
fn eval_captured_letrec_is_fresh_per_invocation_chirho() {
    let source_chirho = r#"module CapturedLetrecFreshChirho where
streamChirho pChirho = goChirho
  where goChirho = pChirho : goChirho
main =
  let { firstChirho = streamChirho 1; secondChirho = streamChirho 10 }
  in head firstChirho `seq`
     (head secondChirho `seq` print (head (tail firstChirho)))
"#;

    assert_eq!(
        eval_output_chirho(source_chirho, "CapturedLetrecFreshChirho.hs"),
        "1\n",
        "a later invocation must not retarget an earlier recursive closure"
    );
}

#[test]
fn eval_sieve_local_recursive_worker_chirho() {
    let source_chirho = r#"module SieveLocalWorkerChirho where
sieveChirho [] = []
sieveChirho (pChirho:xsChirho) = pChirho : sieveChirho (goChirho xsChirho)
  where
    goChirho [] = []
    goChirho (yChirho:ysChirho) =
      if yChirho `mod` pChirho /= 0
        then yChirho : goChirho ysChirho
        else goChirho ysChirho
main = print (take 6 (sieveChirho [2..]))
"#;

    assert_eq!(
        eval_output_chirho(source_chirho, "SieveLocalWorkerChirho.hs"),
        "[2,3,5,7,11,13]\n",
        "a captured recursive worker must keep each invocation's divisor"
    );
}

#[test]
fn eval_captured_mutual_letrec_group_is_fresh_chirho() {
    let source_chirho = r#"module CapturedMutualLetrecChirho where
streamChirho pChirho = leftChirho
  where
    leftChirho = pChirho : rightChirho
    rightChirho = (pChirho + 1) : leftChirho
main =
  let { firstChirho = streamChirho 1; secondChirho = streamChirho 10 }
  in head firstChirho `seq`
     (head secondChirho `seq` print (take 4 firstChirho))
"#;

    assert_eq!(
        eval_output_chirho(source_chirho, "CapturedMutualLetrecChirho.hs"),
        "[1,2,1,2]\n",
        "mutually recursive closures must point at their invocation's fresh group"
    );
}

// Every local `let` binds its names before any right-hand side (brick 6h).
// A do block's `let` and a list comprehension's `let` used their own copies of
// the binding code: the do-let desugared a function's right-hand side BEFORE
// binding its name, so it could not recurse, and the comprehension's kept only
// function bindings, so a pattern binding vanished. Each crashed with a missing
// STG binding. Expectations are GHC 9.14.1's (probes H01-H13, Q01-Q04).

#[test]
fn a_do_let_function_can_recurse_chirho() {
    // H11 (with a signature), H01 (mutual recursion), H04 (in a Maybe block).
    assert_eq!(
        eval_output_chirho(
            "module DoLetRecChirho where\nmain = do\n  let go :: Int -> Int\n      go 0 = 0\n      go n = n + go (n - 1)\n  print (go 4)\n",
            "DoLetRecChirho.hs"
        ),
        "10\n"
    );
    assert_eq!(
        eval_output_chirho(
            "module DoLetMutualChirho where\nmain = do\n  let evenChirho 0 = True\n      evenChirho n = oddChirho (n - 1 :: Int)\n      oddChirho 0 = False\n      oddChirho n = evenChirho (n - 1)\n  print (evenChirho 10, oddChirho 7)\n",
            "DoLetMutualChirho.hs"
        ),
        "(True,True)\n"
    );
    assert_eq!(
        eval_output_chirho(
            "module DoLetMaybeChirho where\nmain = print (do\n  n <- Just (5 :: Int)\n  let fact 0 = 1\n      fact k = k * fact (k - 1)\n  pure (fact n))\n",
            "DoLetMaybeChirho.hs"
        ),
        "Just 120\n"
    );
}

#[test]
fn a_do_let_binding_can_refer_forward_chirho() {
    // H02: a binding's right-hand side names a later binding of the same group.
    assert_eq!(
        eval_output_chirho(
            "module DoLetForwardChirho where\nmain = do\n  let a = b + 1\n      b = 2 :: Int\n  print a\n",
            "DoLetForwardChirho.hs"
        ),
        "3\n"
    );
}

#[test]
fn a_leading_comprehension_let_binds_patterns_and_recursion_chirho() {
    // H12 and H13: a `let` before any generator takes the comprehension's other
    // desugaring path, which had the same defect.
    assert_eq!(
        eval_output_chirho(
            "module CompLetFirstChirho where\nmain = print [a + b | let (a, b) = (1, 2 :: Int)]\n",
            "CompLetFirstChirho.hs"
        ),
        "[3]\n"
    );
    assert_eq!(
        eval_output_chirho(
            "module CompLetFirstRecChirho where\nmain = print [s | True, let sq k = if k == 0 then 0 else k + sq (k - 1 :: Int), let s = sq 4]\n",
            "CompLetFirstRecChirho.hs"
        ),
        "[10]\n"
    );
}

#[test]
fn a_comprehension_let_binds_patterns_and_recursion_chirho() {
    // Q01 (a tuple pattern), H05 (a banged binding), H08 (a recursive function
    // under a second generator, the continuation path) and H06 (an unused
    // binding whose pattern would fail stays unforced, as a let is lazy).
    assert_eq!(
        eval_output_chirho(
            "module CompLetPairChirho where\nmain = print [a + b | p <- [(1, 2), (3, 4 :: Int)], let (a, b) = p]\n",
            "CompLetPairChirho.hs"
        ),
        "[3,7]\n"
    );
    assert_eq!(
        eval_output_chirho(
            "{-# LANGUAGE BangPatterns #-}\nmodule CompLetBangChirho where\nmain = print [x | n <- [1, 2 :: Int], let !x = n * 2]\n",
            "CompLetBangChirho.hs"
        ),
        "[2,4]\n"
    );
    assert_eq!(
        eval_output_chirho(
            "module CompLetRecChirho where\nmain = print [s | x <- [1, 2, 3 :: Int], y <- [x], let sq k = if k == 0 then 0 else k + sq (k - 1), let s = sq y]\n",
            "CompLetRecChirho.hs"
        ),
        "[1,3,6]\n"
    );
    assert_eq!(
        eval_output_chirho(
            "module CompLetLazyChirho where\nmain = print [n | n <- [1, 2 :: Int], let (a, Just b) = (n, Nothing :: Maybe Int)]\n",
            "CompLetLazyChirho.hs"
        ),
        "[1,2]\n"
    );
}
