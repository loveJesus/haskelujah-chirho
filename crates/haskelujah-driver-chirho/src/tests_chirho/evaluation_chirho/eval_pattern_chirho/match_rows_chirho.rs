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

/// The LANGUAGE pragmas every program here runs under.
pub(super) const PRAGMAS_CHIRHO: &str = "BangPatterns, LambdaCase, MultiWayIf";

/// Run a module under `pragmas_chirho` and return what it printed, or why it
/// failed.
fn run_chirho(pragmas_chirho: &str, body_chirho: &str) -> Result<String, String> {
    let source_chirho = format!(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{{-# LANGUAGE {pragmas_chirho} #-}}\nmodule Main where\n{body_chirho}\n"
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
    prints_under_chirho(PRAGMAS_CHIRHO, body_chirho)
}

fn fails_chirho(body_chirho: &str) -> String {
    fails_under_chirho(PRAGMAS_CHIRHO, body_chirho)
}

/// What a program printed; a failure fails the test.
pub(super) fn prints_under_chirho(pragmas_chirho: &str, body_chirho: &str) -> String {
    run_chirho(pragmas_chirho, body_chirho)
        .unwrap_or_else(|failure_chirho| panic!("failed: {failure_chirho}"))
}

/// Why a program failed, where GHC fails too; printing fails the test.
pub(super) fn fails_under_chirho(pragmas_chirho: &str, body_chirho: &str) -> String {
    let failure_chirho = match run_chirho(pragmas_chirho, body_chirho) {
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

#[test]
fn a_generator_pattern_filters_the_elements_it_does_not_match_chirho() {
    // Q1, Q2, Q5: every generator pattern but a plain variable crashed on main
    // with a missing binding; the pattern was never matched at all.
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = print [x | Just x <- [Just (1 :: Int), Nothing, Just 3]]"
        ),
        "[1,3]\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = print [a + b | (a, b) <- [(1 :: Int, 2 :: Int), (3, 4)]]"
        ),
        "[3,7]\n"
    );
    assert_eq!(
        prints_chirho("main :: IO ()\nmain = print [n | [n] <- [[1 :: Int], [], [2, 3], [4]]]"),
        "[1,4]\n"
    );
}

// Pattern guards (brick 6f): an arm's qualifiers stay qualifiers, so a failing
// pattern guard goes on to the next arm and then the next row. Each expectation
// is GHC 9.14.1's output (probe set tmp-chirho/pattern-guards-chirho/).

#[test]
fn a_failing_pattern_guard_falls_through_to_the_next_row_chirho() {
    // G4 (an equation), P02 (a case alternative), P03 (`\case`), P14 (string
    // literal patterns): the previous tip raised "Non-exhaustive guards".
    assert_eq!(
        prints_chirho(
            "fChirho :: Maybe Int -> Int\nfChirho (Just x) | Just y <- lookup x [(5, 50)] = y\nfChirho _ = 0\nmain :: IO ()\nmain = print (fChirho (Just 5), fChirho (Just 6))"
        ),
        "(50,0)\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = mapM_ (\\m -> print (case m of\n  Just x | Just y <- lookup x [(5, 50 :: Int)] -> y\n  _ -> 0)) [Just 5, Just 6, Nothing]"
        ),
        "50\n0\n0\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = mapM_ (print . (\\case { Just x | Just y <- lookup x [(5, 50 :: Int)] -> y; _ -> (0 :: Int) })) [Just 5, Just 6, Nothing :: Maybe Int]"
        ),
        "50\n0\n0\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: String -> Int\nfChirho s\n  | \"yes\" <- s = 1\n  | \"no\" <- s = 0\nfChirho _ = -1\nmain :: IO ()\nmain = print (map fChirho [\"yes\", \"no\", \"maybe\"])"
        ),
        "[1,0,-1]\n"
    );
}

#[test]
fn qualifiers_run_in_order_and_scope_over_what_follows_chirho() {
    // P01 and P18: a pattern guard's variables reach the boolean guard after
    // it and the body. L01: a `let` between them. P13: a later binding shadows.
    assert_eq!(
        prints_chirho(
            "fChirho :: Maybe Int -> Int\nfChirho m\n  | Just x <- m, x > 0 = x\n  | otherwise = -1\nmain :: IO ()\nmain = print (fChirho (Just 5), fChirho (Just (-3)), fChirho Nothing)"
        ),
        "(5,-1,-1)\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: [Int] -> Int\nfChirho xs\n  | (a : b : _) <- xs, a < b = b - a\n  | [a] <- xs = a\nfChirho _ = 0\nmain :: IO ()\nmain = print (map fChirho [[1, 5], [5, 1], [7], []])"
        ),
        "[4,0,7,0]\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Maybe Int -> Int\nfChirho m\n  | Just y <- m\n  , let z = y + 1\n  , z > 5\n  = z\nfChirho _ = 0\nmain :: IO ()\nmain = print (fChirho (Just 9), fChirho (Just 2), fChirho Nothing)"
        ),
        "(10,0,0)\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Int\nfChirho x\n  | Just x <- Just (x + 1) = x * 10\nmain :: IO ()\nmain = print (fChirho 1)"
        ),
        "20\n"
    );
}

#[test]
fn a_pattern_guard_matches_lazily_unless_banged_chirho() {
    // P08: a variable pattern binds its source unforced. P16: `~` matches
    // anything. P15: `!` forces the source.
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Int\nfChirho x\n  | y <- (undefined :: Int), x > 0 = 1\n  | otherwise = 2\nmain :: IO ()\nmain = print (fChirho 5, fChirho (-5))"
        ),
        "(1,2)\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Int\nfChirho x\n  | ~(Just y) <- (Nothing :: Maybe Int) = 1\n  | otherwise = 2\nmain :: IO ()\nmain = print (fChirho 5)"
        ),
        "1\n"
    );
    let failure_chirho = fails_chirho(
        "fChirho :: Int -> Int\nfChirho x\n  | !y <- (undefined :: Int) = 1\n  | otherwise = 2\nmain :: IO ()\nmain = print (fChirho 5)",
    );
    assert!(failure_chirho.contains("undefined"), "{failure_chirho}");
}

#[test]
fn a_guarded_alternative_with_where_falls_through_chirho() {
    // P17: both arms of the first alternative can fail; its `where` reaches both.
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = mapM_ (\\m -> print (case m of\n  Just x\n    | Just y <- lookup x tbl -> y + bonus\n    | x > 100 -> bonus\n    where bonus = 1\n          tbl = [(5, 50)]\n  _ -> 0)) [Just 5, Just 200, Just 7, Nothing :: Maybe Int]"
        ),
        "51\n1\n0\n0\n"
    );
}

#[test]
fn a_pattern_guard_in_a_top_level_pattern_binding_chirho() {
    // S02 and S03: the whole guarded right-hand side of a top-level PATTERN
    // binding is typed, not its first arm's body alone. P11: a variable binding.
    let pair_chirho = "(aChirho, bChirho)\n  | Just n <- lookup (KEY :: Int) [(1, 10 :: Int)] = (n, n + 1)\n  | otherwise = (0, 0)\nmain :: IO ()\nmain = print (aChirho + bChirho)";
    assert_eq!(prints_chirho(&pair_chirho.replace("KEY", "1")), "21\n");
    assert_eq!(prints_chirho(&pair_chirho.replace("KEY", "2")), "0\n");
    assert_eq!(
        prints_chirho(
            "xChirho :: Char\nxChirho\n  | Just v <- lookup (1 :: Int) [(1, 'a')] = v\n  | otherwise = 'z'\nmain :: IO ()\nmain = print xChirho"
        ),
        "'a'\n"
    );
}

#[test]
fn a_guard_binding_shadows_only_in_its_own_arm_chirho() {
    // S01: the previous tip REJECTED this. Its lowering placed the next arm
    // inside the pattern guard's scope, so `x + 1` saw the shadowing `Bool`.
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Maybe Bool -> Int\nfChirho x m\n  | Just x <- m, x = 1\n  | otherwise = x + 1\nmain :: IO ()\nmain = print (fChirho 5 Nothing, fChirho 5 (Just True), fChirho 5 (Just False))"
        ),
        "(6,1,6)\n"
    );
}

#[test]
fn a_failed_arms_binder_is_gone_in_the_next_arm_chirho() {
    // GF02 and GF13 (claude2_chirho, 2026-10-17): main before the match rows
    // crashed on both, and the match-row prefix printed the first arm's
    // binder in the next arm, -100 and 7. GHC prints these.
    assert_eq!(
        prints_chirho(
            "tblChirho :: [(Int, Int)]\ntblChirho = [(1, 10), (10, 100)]\nhopChirho :: Int -> Int\nhopChirho x | Just x <- lookup x tblChirho, Just y <- lookup x tblChirho = y\n            | otherwise = negate x\nmain :: IO ()\nmain = mapM_ (print . hopChirho) [1, 10, 5]"
        ),
        "100\n-10\n-5\n"
    );
    assert_eq!(
        prints_chirho(
            "yChirho :: Int\nyChirho = 5\nfChirho :: Maybe Int -> Int\nfChirho m | Just yChirho <- m, yChirho > 100 = yChirho\n          | otherwise = yChirho\nmain :: IO ()\nmain = print (fChirho (Just 7), fChirho (Just 200), fChirho Nothing)"
        ),
        "(5,200,5)\n"
    );
}

#[test]
fn every_arm_failing_with_no_row_left_fails_chirho() {
    // P10: GHC fails with a pattern-match failure and prints nothing.
    let failure_chirho = fails_chirho(
        "fChirho :: Maybe Int -> Int\nfChirho (Just x)\n  | Just y <- lookup x [] = y\nmain :: IO ()\nmain = print (fChirho (Just 1))",
    );
    assert!(
        failure_chirho.contains("Non-exhaustive"),
        "{failure_chirho}"
    );
}

#[test]
fn a_guard_binding_is_scoped_to_its_own_arm_chirho() {
    // R05: GHC rejects it; `x` is bound by the first arm's pattern guard only.
    let failure_chirho = fails_chirho(
        "fChirho :: Maybe Int -> Int\nfChirho m\n  | Just x <- m = x\n  | otherwise = x\nmain :: IO ()\nmain = print (fChirho (Just 1))",
    );
    assert!(
        failure_chirho.contains("unbound variable: `x`"),
        "{failure_chirho}"
    );
}

#[test]
fn a_boolean_guard_must_be_a_bool_chirho() {
    // R02: GHC rejects it; the previous tip accepted it and printed 0.
    let failure_chirho = fails_chirho(
        "fChirho :: Int -> Int\nfChirho x | 'c' = 1\nfChirho _ = 0\nmain :: IO ()\nmain = print (fChirho 1)",
    );
    assert!(failure_chirho.contains("type mismatch"), "{failure_chirho}");
}

#[test]
fn a_one_line_let_guard_ends_at_its_comma_chirho() {
    // T01, T02 (a case alternative), T06 (a line aligned with the closed
    // block continues the body) and T08: the previous tip looped on all four,
    // because the `let` block ran on over the next qualifier.
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Int\nfChirho x\n  | let y = x * 2, y > 10 = y\n  | otherwise = 0\nmain :: IO ()\nmain = print (fChirho 3, fChirho 7)"
        ),
        "(0,14)\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = mapM_ (\\n -> print (case n of\n  k | let d = k * 3, d > 10 -> d\n  _ -> 0)) [2, 5 :: Int]"
        ),
        "0\n15\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Int\nfChirho x\n  | let y = x * 2, y > 10 = y\n        + 1\n  | otherwise = 0\nmain :: IO ()\nmain = print (fChirho 3, fChirho 7)"
        ),
        "(0,15)\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Maybe Int -> Int\nfChirho mb\n  | Just v <- mb, let w = v * 10, w > 30, let u = w + 1 = u\nfChirho _ = 0\nmain :: IO ()\nmain = print (fChirho (Just 5), fChirho (Just 2), fChirho Nothing)"
        ),
        "(51,0,0)\n"
    );
}

// Multi-way if (brick 6i): its arms are guard arms, qualifiers and all. The
// previous tip parsed each arm as `cond -> e` only, so a pattern guard, a `let`
// or a second qualifier was rejected. Expectations are GHC 9.14.1's (M01-M10).

#[test]
fn a_multiway_if_arm_has_qualifiers_chirho() {
    // M02 (a pattern guard), M07 (two qualifiers), M03 (a one-line `let`),
    // M10 (all three, falling to later arms).
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = mapM_ (\\m -> print (if | Just x <- m, x > 0 -> x | otherwise -> (0 :: Int))) [Just 3, Just (-1), Nothing]"
        ),
        "3\n0\n0\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> String\nfChirho x = if | x > 0, even x -> \"pos even\" | x > 0 -> \"pos odd\" | otherwise -> \"non-pos\"\nmain :: IO ()\nmain = mapM_ (putStrLn . fChirho) [4, 3, 0]"
        ),
        "pos even\npos odd\nnon-pos\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Int\nfChirho x = if | let y = x * 2, y > 10 -> y | otherwise -> 0\nmain :: IO ()\nmain = print (fChirho 3, fChirho 7)"
        ),
        "(0,14)\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Maybe (Int, Int) -> Int\nfChirho m = if | Just (a, b) <- m, let s = a + b, s > 10 -> s\n               | Just (a, _) <- m -> a\n               | otherwise -> 0\nmain :: IO ()\nmain = print (map fChirho [Just (6, 7), Just (1, 2), Nothing])"
        ),
        "[13,1,0]\n"
    );
}

#[test]
fn a_multiway_if_inside_an_outer_guard_leaves_it_alone_chirho() {
    // M05: the outer `| otherwise = ...` reaches `=` first, so it is the
    // equation's guard and not another arm.
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> String\nfChirho x\n  | x > 0 = if | x > 10 -> \"big\" | otherwise -> \"small\"\n  | otherwise = \"neg\"\nmain :: IO ()\nmain = mapM_ (putStrLn . fChirho) [20, 5, -1]"
        ),
        "big\nsmall\nneg\n"
    );
}

#[test]
fn a_multiway_if_with_no_arm_left_fails_chirho() {
    // M04: GHC fails and prints nothing.
    let failure_chirho = fails_chirho(
        "fChirho :: Int -> Int\nfChirho x = if | x > 100 -> 1 | x < -100 -> 2\nmain :: IO ()\nmain = print (fChirho 5)",
    );
    assert!(
        failure_chirho.contains("Non-exhaustive guards in multi-way if"),
        "{failure_chirho}"
    );
}

#[test]
fn multiway_if_arms_must_agree_chirho() {
    // TcMultiWayIfFail's shape: GHC rejects both later arms. Unlike a guarded
    // right-hand side's, a multi-way if's arms are an `if`'s branches.
    let failure_chirho = fails_chirho(
        "xChirho :: Int\nxChirho = if | True -> 1 :: Int\n             | False -> \"2\"\n             | otherwise -> [3 :: Int]\nmain :: IO ()\nmain = print xChirho",
    );
    assert!(failure_chirho.contains("type mismatch"), "{failure_chirho}");
}
