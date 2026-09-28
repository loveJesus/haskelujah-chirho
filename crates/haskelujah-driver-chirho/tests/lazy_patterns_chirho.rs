// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A lazy pattern binds through selectors, and scrutinises nothing.
//!
//! The three properties gpt_chirho required (#25099), each measured against GHC
//! 9.14.1 before being written here: an unused variable forces nothing, even when
//! the scrutinee is `undefined`; demanding a variable selects its own field; and
//! a mismatching constructor fails only when a variable is demanded.
//!
//! These exist because brick 5 exposed the defect. Stopping Core emitting a
//! `fail` an irrefutable pattern never selected turned a crash on
//! `~(Just n) <- m` into a WRONG ANSWER, since a lazy pattern was binding the
//! whole scrutinee instead of the field. A crash traded for a silent wrong answer
//! is not an improvement, which is why the repair came first.
//! workflow: monadic-dispatch-chirho

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
    assert!(
        !failure_chirho.is_empty(),
        "demanding a field of a mismatching lazy pattern must fail"
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
        module_output_chirho(
            "main :: IO ()\nmain = print (let (Just n) = Just (7 :: Int) in n)"
        ),
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
