// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Every match position is rows over one single-row matcher.
//!
//! Each program below was run on GHC 9.14.1 and its output is the expectation
//! (the lane's probe sets: C, D, F, G, K, L, N, A and E). On main every one of
//! them printed something else, crashed, or failed where GHC succeeds: a literal
//! below a constructor was never tested, record fields bound by listed position,
//! guards never fell through to the next row, `~` bound the whole scrutinee, a
//! banged alternative lost its bang in the parser, and a wildcard alternative
//! forced its scrutinee. The failure controls assert GHC's pattern-match reason
//! and that nothing was printed, so a crash cannot pass for a match failure.
//! workflow: language-features-chirho/pattern-matching-chirho

use crate::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

/// Run a module and return what it printed, or why it failed.
fn run_chirho(body_chirho: &str) -> Result<String, String> {
    let source_chirho = format!(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{{-# LANGUAGE BangPatterns, LambdaCase #-}}\nmodule Main where\n{body_chirho}\n"
    );
    eval_source_with_machine_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "MatchRowsChirho.hs",
        None,
    )
    .map(|(_, machine_chirho)| machine_chirho.io_output_chirho)
    .map_err(|diagnostics_chirho| format!("{diagnostics_chirho:?}"))
}

fn prints_chirho(body_chirho: &str) -> String {
    run_chirho(body_chirho).unwrap_or_else(|failure_chirho| panic!("failed: {failure_chirho}"))
}

fn fails_chirho(body_chirho: &str) -> String {
    let failure_chirho = match run_chirho(body_chirho) {
        Ok(printed_chirho) => panic!("GHC fails here; this printed {printed_chirho:?}"),
        Err(failure_chirho) => failure_chirho,
    };
    assert!(
        !failure_chirho.contains("missing STG binding"),
        "a missing binding is a crash, not the failure GHC raises: {failure_chirho}"
    );
    failure_chirho
}

#[test]
fn a_literal_below_a_constructor_is_tested_chirho() {
    // N1, N2, N3: main returned the first row for all three.
    assert_eq!(
        prints_chirho(
            "fChirho :: (Int, Int) -> Int\nfChirho (x, 3) = x\nfChirho _ = 0\nmain :: IO ()\nmain = print (fChirho (1, 4))"
        ),
        "0\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = case Just (4 :: Int) of\n  Just 3 -> print (1 :: Int)\n  _ -> print (0 :: Int)"
        ),
        "0\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Maybe Int -> Int\nfChirho (Just 3) = 1\nfChirho _ = 0\nmain :: IO ()\nmain = print (fChirho (Just 4))"
        ),
        "0\n"
    );
}

#[test]
fn a_later_literal_row_is_reached_chirho() {
    // N4 and N6: a second literal row, and a character literal.
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = case (1 :: Int, 4 :: Int) of\n  (x, 3) -> print x\n  (x, 4) -> print (x + 10)\n  _ -> print (0 :: Int)"
        ),
        "11\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = case (1 :: Int, 'b') of\n  (x, 'a') -> print x\n  _ -> print (0 :: Int)"
        ),
        "0\n"
    );
}

#[test]
fn a_record_pattern_matches_fields_by_name_chirho() {
    // K1 (case) and K5 (equation): main swapped the fields.
    let record_chirho = "data PairChirho = PairChirho { leftChirho :: Int, rightChirho :: Int }\n";
    assert_eq!(
        prints_chirho(&format!(
            "{record_chirho}main :: IO ()\nmain = case PairChirho 1 2 of\n  PairChirho {{ rightChirho = r, leftChirho = l }} -> print (l - r)"
        )),
        "-1\n"
    );
    assert_eq!(
        prints_chirho(&format!(
            "{record_chirho}fChirho :: PairChirho -> Int\nfChirho PairChirho {{ rightChirho = r, leftChirho = l }} = l - r\nmain :: IO ()\nmain = print (fChirho (PairChirho 1 2))"
        )),
        "-1\n"
    );
}

#[test]
fn a_list_alternative_matches_its_length_chirho() {
    // K2: crashed on main, missing STG binding `y`.
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = case [1, 2 :: Int] of\n  [x, y] -> print (x - y)\n  _ -> print (0 :: Int)"
        ),
        "-1\n"
    );
}

#[test]
fn a_failing_guard_falls_through_to_the_next_row_chirho() {
    // G2, G3, G6: main raised "Non-exhaustive guards" for all three.
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Int\nfChirho x | x > 10 = 1\nfChirho _ = 2\nmain :: IO ()\nmain = print (fChirho 5)"
        ),
        "2\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = case Just (5 :: Int) of\n  Just x | x > 10 -> print (1 :: Int)\n  Just x -> print x\n  Nothing -> print (0 :: Int)"
        ),
        "5\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Int\nfChirho x\n  | x > 10 = big\n  where big = 1\nfChirho _ = 2\nmain :: IO ()\nmain = print (fChirho 5, fChirho 20)"
        ),
        "(2,1)\n"
    );
}

#[test]
fn a_guarded_variable_alternative_is_kept_chirho() {
    // G5: SILENT on main, which printed 2/2 - the guarded alternative became a
    // second default that the later `_` shadowed.
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = mapM_ (print . (\\case { x | x > 10 -> 1; _ -> (2 :: Int) })) [5, 20 :: Int]"
        ),
        "2\n1\n"
    );
}

#[test]
fn a_lazy_alternative_binds_its_field_on_demand_chirho() {
    // C1 printed `Just 7` on main; C3 forced undefined; C5 crashed.
    assert_eq!(
        prints_chirho("main :: IO ()\nmain = case Just (7 :: Int) of\n  ~(Just n) -> print n"),
        "7\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = case (undefined :: Maybe Int) of\n  ~(Just n) -> print (5 :: Int)"
        ),
        "5\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = case Just (3 :: Int, 4 :: Int) of\n  Just ~(a, b) -> print (a - b)\n  Nothing -> print (0 :: Int)"
        ),
        "-1\n"
    );
}

#[test]
fn a_demanded_lazy_mismatch_fails_chirho() {
    // C4: SILENT on main, which printed `Nothing`.
    let failure_chirho = fails_chirho(
        "main :: IO ()\nmain = case (Nothing :: Maybe Int) of\n  ~(Just n) -> print n",
    );
    assert!(
        failure_chirho.contains("Non-exhaustive patterns in"),
        "{failure_chirho}"
    );
}

#[test]
fn a_lazy_first_equation_always_matches_chirho() {
    // F4: SILENT on main, which skipped it and printed 2. F1 crashed.
    assert_eq!(
        prints_chirho(
            "kChirho :: Maybe Int -> Int\nkChirho ~(Just n) = 1\nkChirho Nothing = 2\nmain :: IO ()\nmain = print (kChirho Nothing)"
        ),
        "1\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Maybe Int -> Int\nfChirho ~(Just n) = n\nmain :: IO ()\nmain = print (fChirho (Just 7))"
        ),
        "7\n"
    );
}

#[test]
fn a_lambda_matches_nested_patterns_chirho() {
    // K6 crashed; L1 printed `Just 7`; L3 forced undefined.
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = print ((\\(x, Just y) -> x + y) (1 :: Int, Just (2 :: Int)))"
        ),
        "3\n"
    );
    assert_eq!(
        prints_chirho("main :: IO ()\nmain = print ((\\ ~(Just n) -> n) (Just (7 :: Int)))"),
        "7\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = print ((\\ ~(Just n) -> (5 :: Int)) (undefined :: Maybe Int))"
        ),
        "5\n"
    );
}

#[test]
fn a_do_bind_matches_below_its_constructor_chirho() {
    // D1: crashed on main, missing STG binding `a`.
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = do\n  Just ~(a, b) <- pure (Just (3 :: Int, 4 :: Int))\n  print (a - b)"
        ),
        "-1\n"
    );
}

#[test]
fn a_banged_alternative_forces_and_a_wildcard_does_not_chirho() {
    // E1, E5: the parser dropped the `!`. E7, E9: every alternative used to
    // force its scrutinee, so `_` failed on undefined where GHC prints 5.
    let failure_chirho = fails_chirho(
        "main :: IO ()\nmain = print (case (undefined :: Int) of { !_ -> (5 :: Int) })",
    );
    assert!(failure_chirho.contains("undefined"), "{failure_chirho}");
    let failure_chirho = fails_chirho(
        "gChirho :: Int -> Int\ngChirho n = case n of { !_ -> 5 }\nmain :: IO ()\nmain = print (gChirho undefined)",
    );
    assert!(failure_chirho.contains("undefined"), "{failure_chirho}");
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = print (case (undefined :: Int) of { _ -> (5 :: Int) })"
        ),
        "5\n"
    );
    assert_eq!(
        prints_chirho(
            "gChirho :: Int -> Int\ngChirho n = case n of { _ -> 5 }\nmain :: IO ()\nmain = print (gChirho undefined)"
        ),
        "5\n"
    );
}

#[test]
fn a_negative_literal_alternative_matches_chirho() {
    // EA, EB.
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = print (case (-1 :: Int) of { -1 -> (5 :: Int); _ -> 6 })"
        ),
        "5\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = print (case (2 :: Int) of { -1 -> (5 :: Int); _ -> 6 })"
        ),
        "6\n"
    );
}
