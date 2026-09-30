// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A pattern binding is lazy, and when it is demanded it matches the WHOLE
//! pattern.
//!
//! The three properties gpt_chirho required (#25099), each measured against GHC
//! 9.14.1 before being written here: an unused variable forces nothing, even when
//! the scrutinee is `undefined`; demanding a variable selects its own field; and
//! a mismatching constructor fails only when a variable is demanded.
//!
//! And the property the first version of this repair got wrong, silently, in
//! every binding position: demanding ANY variable matches the whole pattern
//! (Haskell 2010 §3.17.3, rule (d)), so `x` in `(x, Just y) = (1, Nothing)`
//! fails even though `x`'s own position matched. The controls below the full-match
//! heading carry that, together with the banged bindings, where the MATCH is
//! forced before the body and the variables are not.
//!
//! These exist because brick 5 exposed the defect. Stopping Core emitting a
//! `fail` an irrefutable pattern never selected turned a crash on
//! `~(Just n) <- m` into a WRONG ANSWER, since a lazy pattern was binding the
//! whole scrutinee instead of the field. A crash traded for a silent wrong answer
//! is not an improvement, which is why the repair came first.
//! workflow: language-features-chirho/pattern-matching-chirho

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

/// Run one program and return what it printed.
fn output_chirho(body_chirho: &str) -> String {
    let source_chirho = format!(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nmain :: IO ()\nmain = do\n{body_chirho}\n"
    );
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "LazyPatternsChirho.hs",
        None,
    )
    .expect("the program compiles");
    machine_chirho.io_output_chirho
}

/// Run one program that is expected to fail, and return the failure text.
fn failure_chirho(body_chirho: &str) -> String {
    let source_chirho = format!(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nmain :: IO ()\nmain = do\n{body_chirho}\n"
    );
    match eval_source_with_machine_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "LazyPatternsChirho.hs",
        None,
    ) {
        Ok((_, machine_chirho)) => panic!(
            "expected a failure on demand, printed {:?}",
            machine_chirho.io_output_chirho
        ),
        Err(diagnostics_chirho) => format!("{diagnostics_chirho:?}"),
    }
}

#[test]
fn demanding_a_lazy_variable_selects_its_own_field_chirho() {
    // GHC 9.14.1 prints 7. Before the repair this printed `Just 7`: the variable
    // was bound to the whole scrutinee.
    assert_eq!(
        output_chirho("  ~(Just n) <- pure (Just (7 :: Int))\n  print n"),
        "7\n"
    );
}

#[test]
fn each_variable_of_a_lazy_tuple_selects_its_own_field_chirho() {
    // Two selectors over one scrutinee, so a wrong index shows up as a wrong sum.
    assert_eq!(
        output_chirho("  ~(a, b) <- pure (3 :: Int, 4 :: Int)\n  print (a - b)"),
        "-1\n"
    );
}

#[test]
fn a_nested_lazy_pattern_selects_through_both_levels_chirho() {
    assert_eq!(
        output_chirho("  ~(Just ~(a, b)) <- pure (Just (3 :: Int, 4 :: Int))\n  print (a - b)"),
        "-1\n"
    );
}

#[test]
fn an_unused_lazy_variable_does_not_force_a_mismatching_scrutinee_chirho() {
    // GHC prints 5: the bind itself never matches, so `Nothing` is fine as long
    // as nothing is demanded. This is the property a case at the binding site
    // would destroy.
    assert_eq!(
        output_chirho("  ~(Just n) <- pure (Nothing :: Maybe Int)\n  print (5 :: Int)"),
        "5\n"
    );
}

#[test]
fn an_unused_lazy_variable_does_not_force_undefined_chirho() {
    // The sharpest form of the same property, and the one GHC users rely on.
    assert_eq!(
        output_chirho("  ~(Just n) <- pure (undefined :: Maybe Int)\n  print (5 :: Int)"),
        "5\n"
    );
}

#[test]
fn a_mismatching_lazy_pattern_fails_only_when_demanded_chirho() {
    // GHC raises an irrefutable-pattern-match failure at the point of DEMAND.
    // What matters here is that it fails at all, and that the previous control
    // proves it does not fail when nothing is demanded.
    let failure_chirho = failure_chirho("  ~(Just n) <- pure (Nothing :: Maybe Int)\n  print n");
    // Reason-aware, not merely non-empty (claude2_chirho, #25430): main passes a
    // "some failure happened" check today by CRASHING on a missing STG binding,
    // so only the reason can tell the repair from the defect it replaced. GHC
    // 9.14.1 raises "Non-exhaustive patterns in Just n".
    assert!(
        failure_chirho.contains("Non-exhaustive patterns"),
        "the failure must be GHC's pattern-match reason: {failure_chirho}"
    );
    assert!(
        !failure_chirho.contains("missing STG binding"),
        "a missing binding is the old crash, not a pattern failure: {failure_chirho}"
    );
}

#[test]
fn a_demanded_mismatching_tilde_let_fails_and_prints_nothing_chirho() {
    // On main this is a SILENT wrong answer in the other direction: it PRINTS
    // `Nothing`, the scrutinee standing in for the field (claude2_chirho, #25433).
    // GHC 9.14.1 fails with "Non-exhaustive patterns in Just n". `failure_chirho`
    // panics if the program succeeds, so printing anything at all fails this
    // control, and the reason must be the pattern-match reason.
    let failure_chirho = failure_chirho("  let ~(Just n) = (Nothing :: Maybe Int)\n  print n");
    assert!(
        failure_chirho.contains("Non-exhaustive patterns"),
        "the failure must be GHC's pattern-match reason: {failure_chirho}"
    );
}

#[test]
fn a_tilde_tuple_let_selects_each_field_chirho() {
    // Crashes on main: the whole tuple reached `-` as though it were a number.
    assert_eq!(
        output_chirho("  let ~(a, b) = (3 :: Int, 4 :: Int)\n  print (a - b)"),
        "-1\n"
    );
}

#[test]
fn a_tilde_let_binding_selects_the_field_not_the_whole_value_chirho() {
    // A SILENT wrong answer on main today, measured on main's CLI bfd9ad99: this
    // prints `Just 7` where GHC 9.14.1 prints 7. The `~` sent the pattern to a
    // default alternative whose variable bound the whole scrutinee. It was
    // missing from the first set of controls, which is how a review of them could
    // truthfully find "no silent wrong answer on main" among the rows it had.
    assert_eq!(
        output_chirho("  let ~(Just n) = Just (7 :: Int)\n  print n"),
        "7\n"
    );
}

/// Run a whole module and return what it printed, for the shapes that are not a
/// do statement.
fn module_output_chirho(body_chirho: &str) -> String {
    let source_chirho = format!(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\n{body_chirho}\n"
    );
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "LazyPatternsChirho.hs",
        None,
    )
    .expect("the program compiles");
    machine_chirho.io_output_chirho
}

#[test]
fn a_do_let_pattern_binding_is_lazy_chirho() {
    // Every Haskell pattern binding is lazy, `~` or not. Demanding selects the
    // field; leaving it unused forces nothing, even against a scrutinee that
    // cannot match or that is undefined. All three crashed before the repair.
    assert_eq!(
        output_chirho("  let (Just n) = Just (7 :: Int)\n  print n"),
        "7\n"
    );
    assert_eq!(
        output_chirho("  let (Just n) = (Nothing :: Maybe Int)\n  print (5 :: Int)"),
        "5\n"
    );
    assert_eq!(
        output_chirho("  let ~(Just n) = (undefined :: Maybe Int)\n  print (5 :: Int)"),
        "5\n"
    );
    assert_eq!(
        output_chirho("  let (a, b) = (3 :: Int, 4 :: Int)\n  print (a - b)"),
        "-1\n"
    );
}

#[test]
fn a_where_pattern_binding_is_lazy_chirho() {
    assert_eq!(
        module_output_chirho(
            "valChirho :: Int\nvalChirho = n where (Just n) = Just (7 :: Int)\nmain :: IO ()\nmain = print valChirho"
        ),
        "7\n"
    );
    assert_eq!(
        module_output_chirho(
            "valChirho :: Int\nvalChirho = 5 where (Just n) = (Nothing :: Maybe Int)\nmain :: IO ()\nmain = print valChirho"
        ),
        "5\n"
    );
}

#[test]
fn a_let_in_pattern_binding_is_lazy_chirho() {
    assert_eq!(
        module_output_chirho("main :: IO ()\nmain = print (let (Just n) = Just (7 :: Int) in n)"),
        "7\n"
    );
    assert_eq!(
        module_output_chirho(
            "main :: IO ()\nmain = print (let (Just n) = (Nothing :: Maybe Int) in (5 :: Int))"
        ),
        "5\n"
    );
}

#[test]
fn a_top_level_pattern_binding_is_lazy_chirho() {
    // Measured rather than assumed: this path was already correct before the
    // repair, and the control keeps it that way.
    assert_eq!(
        module_output_chirho(
            "(aChirho, bChirho) = (3 :: Int, 4 :: Int)\nmain :: IO ()\nmain = print (aChirho - bChirho)"
        ),
        "-1\n"
    );
    assert_eq!(
        module_output_chirho(
            "(Just nChirho) = (Nothing :: Maybe Int)\nmain :: IO ()\nmain = print (5 :: Int)"
        ),
        "5\n"
    );
}

#[test]
fn a_refutable_bind_still_selects_fail_chirho() {
    // The control gpt_chirho asked to keep unchanged: a genuinely failable
    // pattern still routes through the monad's `fail`, so Maybe gives Nothing
    // rather than a crash.
    let source_chirho = "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\n\nhitChirho :: Maybe Int\nhitChirho = do\n  Just x <- Just (Just 9)\n  return x\n\nmissChirho :: Maybe Int\nmissChirho = do\n  Just x <- Just Nothing\n  return x\n\nmain :: IO ()\nmain = do\n  print hitChirho\n  print missChirho\n";
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        &source_chirho.to_string(),
        &mut SourceMapChirho::new_chirho(),
        "RefutableChirho.hs",
        None,
    )
    .expect("the program compiles");
    assert_eq!(machine_chirho.io_output_chirho, "Just 9\nNothing\n");
}

// ---------------------------------------------------------------------------
// The WHOLE pattern is matched when any variable is demanded, in every binding
// position. Every expectation below was measured on GHC 9.14.1 (probes W, X and
// T in the lane's tmp-chirho/lazy-positions-chirho/). The failures print nothing
// and fail with the pattern-match reason, not with a missing binding.
// ---------------------------------------------------------------------------

/// Run a whole module, after any LANGUAGE pragmas, and return what it printed
/// or why it failed.
fn run_module_chirho(pragmas_chirho: &str, body_chirho: &str) -> Result<String, String> {
    let source_chirho = format!(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{pragmas_chirho}module Main where\n{body_chirho}\n"
    );
    eval_source_with_machine_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "PatternBindingsChirho.hs",
        None,
    )
    .map(|(_, machine_chirho)| machine_chirho.io_output_chirho)
    .map_err(|diagnostics_chirho| format!("{diagnostics_chirho:?}"))
}

fn module_prints_chirho(pragmas_chirho: &str, body_chirho: &str) -> String {
    run_module_chirho(pragmas_chirho, body_chirho)
        .unwrap_or_else(|failure_chirho| panic!("the program failed: {failure_chirho}"))
}

/// The failure of a program GHC rejects at run time. Printing anything fails
/// the control, and so does a failure that is the old missing-binding crash
/// rather than a real pattern-match or forcing failure.
fn module_fails_chirho(pragmas_chirho: &str, body_chirho: &str) -> String {
    let failure_chirho = match run_module_chirho(pragmas_chirho, body_chirho) {
        Ok(printed_chirho) => panic!("GHC fails here; this printed {printed_chirho:?}"),
        Err(failure_chirho) => failure_chirho,
    };
    assert!(
        !failure_chirho.contains("missing STG binding"),
        "a missing binding is a crash, not the failure GHC raises: {failure_chirho}"
    );
    failure_chirho
}

fn assert_pattern_failure_chirho(failure_chirho: &str) {
    assert!(
        failure_chirho.contains("Non-exhaustive patterns in"),
        "GHC fails with a pattern-match reason: {failure_chirho}"
    );
}

const MAIN_DO_CHIRHO: &str = "main :: IO ()\nmain = do\n";
const BANGS_CHIRHO: &str = "{-# LANGUAGE BangPatterns #-}\n";

#[test]
fn demanding_x_matches_the_whole_pattern_in_a_do_let_chirho() {
    // W1. The first version of this repair printed 1: it checked only x's path.
    let failure_chirho = module_fails_chirho(
        "",
        &format!("{MAIN_DO_CHIRHO}  let (x, Just y) = (1 :: Int, Nothing :: Maybe Int)\n  print x"),
    );
    assert_pattern_failure_chirho(&failure_chirho);
}

#[test]
fn demanding_x_matches_the_whole_pattern_in_a_let_in_chirho() {
    // W3.
    let failure_chirho = module_fails_chirho(
        "",
        "main :: IO ()\nmain = print (let (x, Just y) = (1 :: Int, Nothing :: Maybe Int) in x)",
    );
    assert_pattern_failure_chirho(&failure_chirho);
}

#[test]
fn demanding_x_matches_the_whole_pattern_in_a_where_chirho() {
    // W4.
    let failure_chirho = module_fails_chirho(
        "",
        "valChirho :: Int\nvalChirho = x where (x, Just y) = (1 :: Int, Nothing :: Maybe Int)\nmain :: IO ()\nmain = print valChirho",
    );
    assert_pattern_failure_chirho(&failure_chirho);
}

#[test]
fn demanding_x_matches_the_whole_pattern_in_a_lazy_do_bind_chirho() {
    // W5.
    let failure_chirho = module_fails_chirho(
        "",
        &format!(
            "{MAIN_DO_CHIRHO}  ~(x, Just y) <- pure (1 :: Int, Nothing :: Maybe Int)\n  print x"
        ),
    );
    assert_pattern_failure_chirho(&failure_chirho);
}

#[test]
fn demanding_x_matches_the_whole_pattern_at_top_level_chirho() {
    // W6. Silent on main as well, which printed 1.
    let failure_chirho = module_fails_chirho(
        "",
        "(xChirho, Just yChirho) = (1 :: Int, Nothing :: Maybe Int)\nmain :: IO ()\nmain = print xChirho",
    );
    assert_pattern_failure_chirho(&failure_chirho);
}

#[test]
fn a_literal_sibling_is_part_of_the_match_chirho() {
    // W7. Silent on main too: a literal below a constructor was never tested.
    let failure_chirho = module_fails_chirho(
        "",
        &format!("{MAIN_DO_CHIRHO}  let (x, 3) = (1 :: Int, 4 :: Int)\n  print x"),
    );
    assert_pattern_failure_chirho(&failure_chirho);
}

#[test]
fn a_lazy_sibling_is_not_part_of_the_match_chirho() {
    // W2: the `~` keeps `Just y` out of the match, so x is 1.
    assert_eq!(
        module_prints_chirho(
            "",
            &format!(
                "{MAIN_DO_CHIRHO}  let (x, ~(Just y)) = (1 :: Int, Nothing :: Maybe Int)\n  print x"
            ),
        ),
        "1\n"
    );
}

#[test]
fn an_unused_sibling_mismatch_forces_nothing_chirho() {
    // X2: the whole match runs only on demand, and nothing demands it.
    assert_eq!(
        module_prints_chirho(
            "",
            &format!(
                "{MAIN_DO_CHIRHO}  let (x, Just y) = (1 :: Int, Nothing :: Maybe Int)\n  print (5 :: Int)"
            ),
        ),
        "5\n"
    );
}

#[test]
fn a_record_pattern_binds_fields_by_name_chirho() {
    // W9 and X1: silent on main, which bound listed fields by position.
    let record_chirho = "data PairChirho = PairChirho { leftChirho :: Int, rightChirho :: Int }\n";
    assert_eq!(
        module_prints_chirho(
            "",
            &format!(
                "{record_chirho}{MAIN_DO_CHIRHO}  let PairChirho {{ rightChirho = r, leftChirho = l }} = PairChirho 1 2\n  print (l - r)"
            ),
        ),
        "-1\n"
    );
    assert_eq!(
        module_prints_chirho(
            "",
            &format!(
                "{record_chirho}{MAIN_DO_CHIRHO}  let PairChirho {{ rightChirho = r }} = PairChirho 1 2\n  print r"
            ),
        ),
        "2\n"
    );
}

#[test]
fn a_list_pattern_binding_matches_its_length_chirho() {
    // WA crashed on main; W8 is the length mismatch, which must fail.
    assert_eq!(
        module_prints_chirho(
            "",
            &format!("{MAIN_DO_CHIRHO}  let [x, y] = [1, 2 :: Int]\n  print (x - y)")
        ),
        "-1\n"
    );
    let failure_chirho = module_fails_chirho(
        "",
        &format!("{MAIN_DO_CHIRHO}  let [x, y] = [1, 2, 3 :: Int]\n  print x"),
    );
    assert_pattern_failure_chirho(&failure_chirho);
}

#[test]
fn a_recursive_pattern_binding_ties_its_knot_chirho() {
    // X4: each side is defined through the other. Crashed on main.
    assert_eq!(
        module_prints_chirho(
            "",
            &format!(
                "{MAIN_DO_CHIRHO}  let (xs, ys) = (1 : ys, 2 : take 3 xs) :: ([Int], [Int])\n  print (take 4 xs)"
            ),
        ),
        "[1,2,1,2]\n"
    );
}

#[test]
fn a_top_level_tilde_binding_is_an_ordinary_pattern_binding_chirho() {
    // T1 and T3: both looped on main (`<<loop>>`).
    assert_eq!(
        module_prints_chirho(
            "",
            "~(Just nChirho) = Just (7 :: Int)\nmain :: IO ()\nmain = print nChirho",
        ),
        "7\n"
    );
    assert_eq!(
        module_prints_chirho(
            "",
            "~(aChirho, bChirho) = (3 :: Int, 4 :: Int)\nmain :: IO ()\nmain = print (aChirho - bChirho)",
        ),
        "-1\n"
    );
}

// ---------------------------------------------------------------------------
// A bang forces the MATCH before the body, and not the variables. The parser
// used to drop a binding's leading `!` outright, so none of these held.
// ---------------------------------------------------------------------------

#[test]
fn a_banged_variable_binding_forces_its_value_chirho() {
    // B1. Silent on main: it printed 5.
    let failure_chirho = module_fails_chirho(
        BANGS_CHIRHO,
        &format!("{MAIN_DO_CHIRHO}  let !x = (undefined :: Int)\n  print (5 :: Int)"),
    );
    assert!(failure_chirho.contains("undefined"), "{failure_chirho}");
}

#[test]
fn a_banged_let_in_binding_forces_its_match_chirho() {
    // B2 and B7: the first version of this repair printed 5 for both.
    let failure_chirho = module_fails_chirho(
        BANGS_CHIRHO,
        "main :: IO ()\nmain = print (let !(a, b) = (undefined :: (Int, Int)) in (5 :: Int))",
    );
    assert!(failure_chirho.contains("undefined"), "{failure_chirho}");
    let failure_chirho = module_fails_chirho(
        BANGS_CHIRHO,
        "main :: IO ()\nmain = print (let !(Just n) = (Nothing :: Maybe Int) in (5 :: Int))",
    );
    assert_pattern_failure_chirho(&failure_chirho);
}

#[test]
fn a_banged_where_binding_forces_its_match_chirho() {
    // B3.
    let failure_chirho = module_fails_chirho(
        BANGS_CHIRHO,
        "valChirho :: Int\nvalChirho = 5 where !(a, b) = (undefined :: (Int, Int))\nmain :: IO ()\nmain = print valChirho",
    );
    assert!(failure_chirho.contains("undefined"), "{failure_chirho}");
}

#[test]
fn a_bang_forces_the_match_not_the_variables_chirho() {
    // B6 (claude2_chirho's positive control, #25468): the match succeeds and n is
    // never demanded, so GHC prints 5. Forcing n would fail here. B4 is the
    // demanded case.
    assert_eq!(
        module_prints_chirho(
            BANGS_CHIRHO,
            "main :: IO ()\nmain = print (let !(Just n) = Just (undefined :: Int) in (5 :: Int))",
        ),
        "5\n"
    );
    assert_eq!(
        module_prints_chirho(
            BANGS_CHIRHO,
            &format!("{MAIN_DO_CHIRHO}  let !(Just n) = Just (7 :: Int)\n  print n"),
        ),
        "7\n"
    );
}

#[test]
fn an_inner_bang_is_forced_when_the_match_runs_chirho() {
    // B5. Silent on main: demanding a runs the match, and the match forces b.
    let failure_chirho = module_fails_chirho(
        BANGS_CHIRHO,
        &format!("{MAIN_DO_CHIRHO}  let (a, !b) = (1 :: Int, undefined :: Int)\n  print a"),
    );
    assert!(failure_chirho.contains("undefined"), "{failure_chirho}");
}
