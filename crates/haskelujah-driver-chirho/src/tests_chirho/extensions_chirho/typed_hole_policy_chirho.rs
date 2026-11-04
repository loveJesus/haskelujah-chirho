// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typed holes are errors by default (GHC-88464). GHC's deferral switches, read from real
//! options, decide which inference errors become warnings instead (row 695). Every program
//! here has a typed `main :: IO ()`, so the separate unsigned numeric `main` defect (row 692)
//! never decides a verdict.
//! workflow: compiler-pipeline-chirho/error-deferral-chirho

use crate::{
    FrontendOptionsChirho, eval_source_with_options_chirho, frontend_warnings_with_options_chirho,
};
use haskelujah_span_chirho::SourceMapChirho;

const UNUSED_HOLE_CHIRHO: &str = "\
module Main where
unused :: Int -> Int
unused = _
main :: IO ()
main = putStrLn \"ran\"
";

const UNUSED_NAMED_HOLE_CHIRHO: &str = "\
module Main where
unused :: Int -> Int
unused x = _missing x
main :: IO ()
main = putStrLn \"ran\"
";

const BOUND_UNDERSCORE_NAME_CHIRHO: &str = "\
module Main where
twice :: Int -> Int
twice _x = _x + _x
main :: IO ()
main = print (twice 21)
";

const UNUSED_UNBOUND_VARIABLE_CHIRHO: &str = "\
module Main where
unused :: Int -> Int
unused x = missingName x
main :: IO ()
main = putStrLn \"ran\"
";

/// A solver type error: reported as a mismatch (E0200) or a signature mismatch (E0205),
/// both deferred only by `-fdefer-type-errors`.
const UNUSED_TYPE_MISMATCH_CHIRHO: &str = "\
module Main where
wrong :: Int
wrong = True
main :: IO ()
main = putStrLn \"ran\"
";

const ALL_THREE_CATEGORIES_CHIRHO: &str = "\
module Main where
hole :: Int -> Int
hole = _
unbound :: Int -> Int
unbound x = missingName x
wrong :: Int
wrong = True
main :: IO ()
main = putStrLn \"ran\"
";

/// GHC-83865: a `Type` variable cannot also take an argument.
const KIND_ERROR_CHIRHO: &str = "\
{-# LANGUAGE RankNTypes #-}
module Main where
bad :: (forall f. f -> f Int) -> Bool
bad _ = True
main :: IO ()
main = putStrLn \"ran\"
";

const DUPLICATE_INSTANCE_CHIRHO: &str = "\
module Main where
class C a where
  op :: a -> Bool
data B = MkB
instance C B where
  op MkB = True
instance C B where
  op MkB = False
main :: IO ()
main = print (op MkB)
";

/// T23739c: `Int` names a type, not a data constructor.
const UNBOUND_RECORD_CONSTRUCTOR_CHIRHO: &str = "\
module Main where
g :: Int
g = Int{}
main :: IO ()
main = print g
";

/// A CPP module whose header defers typed holes. The source path reads that header before
/// preprocessing; `#if 1` keeps the branch that prints, with or without a system `cpp`.
const CPP_DEFERRED_HOLE_CHIRHO: &str = "\
{-# LANGUAGE CPP #-}
{-# OPTIONS_GHC -fdefer-typed-holes #-}
module Main where
unused :: Int -> Int
unused = _
main :: IO ()
#if 1
main = putStrLn \"ran\"
#else
main = putStrLn \"wrong branch\"
#endif
";

fn options_chirho(options_chirho: &[&str]) -> FrontendOptionsChirho {
    FrontendOptionsChirho::from_ghc_options_chirho(options_chirho.iter().copied())
}

/// The unused-hole module under one header `OPTIONS_GHC` pragma with this body.
fn with_options_pragma_chirho(body_chirho: &str) -> String {
    format!("{{-# OPTIONS_GHC {body_chirho} #-}}\n{UNUSED_HOLE_CHIRHO}")
}

/// The frontend's verdict under the caller's options: its warnings, or its rendered errors.
fn check_chirho(source_chirho: &str, caller_chirho: &[&str]) -> Result<Vec<String>, String> {
    frontend_warnings_with_options_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "TypedHolePolicyChirho.hs",
        &options_chirho(caller_chirho),
    )
    .map_err(|diagnostics_chirho| diagnostics_chirho.to_string())
}

fn assert_rejected_chirho(source_chirho: &str, caller_chirho: &[&str], expected_chirho: &str) {
    match check_chirho(source_chirho, caller_chirho) {
        Err(errors_chirho) => assert!(
            errors_chirho.contains(expected_chirho),
            "{caller_chirho:?} rejected it without `{expected_chirho}`: {errors_chirho}"
        ),
        Ok(warnings_chirho) => {
            panic!("{caller_chirho:?} accepted it: {warnings_chirho:?}\n{source_chirho}")
        }
    }
}

fn assert_deferred_chirho(source_chirho: &str, caller_chirho: &[&str], expected_chirho: &str) {
    match check_chirho(source_chirho, caller_chirho) {
        Ok(warnings_chirho) => assert!(
            warnings_chirho.iter().any(|warning_chirho| {
                warning_chirho.starts_with("warning") && warning_chirho.contains(expected_chirho)
            }),
            "{caller_chirho:?} accepted it without a `{expected_chirho}` warning: {warnings_chirho:?}"
        ),
        Err(errors_chirho) => {
            panic!("{caller_chirho:?} rejected it: {errors_chirho}\n{source_chirho}")
        }
    }
}

#[test]
fn a_hole_is_an_error_by_default_chirho() {
    assert_rejected_chirho(UNUSED_HOLE_CHIRHO, &[], "found hole: `_`");
    assert_rejected_chirho(UNUSED_NAMED_HOLE_CHIRHO, &[], "found hole: `_missing`");
    assert_rejected_chirho(UNUSED_HOLE_CHIRHO, &[], "E0210");
}

#[test]
fn a_bound_underscore_name_is_an_ordinary_variable_chirho() {
    let warnings_chirho = check_chirho(BOUND_UNDERSCORE_NAME_CHIRHO, &[])
        .expect("a bound `_x` is a variable, never a hole");
    assert!(
        !warnings_chirho
            .iter()
            .any(|warning_chirho| warning_chirho.contains("hole")),
        "{warnings_chirho:?}"
    );
    let (_value_chirho, machine_chirho) = eval_source_with_options_chirho(
        BOUND_UNDERSCORE_NAME_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "BoundUnderscoreNameChirho.hs",
        None,
        &FrontendOptionsChirho::default(),
    )
    .expect("a bound `_x` runs as the variable it is");
    assert_eq!(machine_chirho.io_output_chirho, "42\n");
}

#[test]
fn an_explicitly_deferred_unused_hole_still_runs_chirho() {
    assert_deferred_chirho(UNUSED_HOLE_CHIRHO, &["-fdefer-typed-holes"], "found hole");
    let (_value_chirho, machine_chirho) = eval_source_with_options_chirho(
        UNUSED_HOLE_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "DeferredUnusedHoleChirho.hs",
        None,
        &options_chirho(&["-fdefer-typed-holes"]),
    )
    .expect("a deferred hole the program never reaches does not stop it");
    assert_eq!(machine_chirho.io_output_chirho, "ran\n");
    let refused_chirho = eval_source_with_options_chirho(
        UNUSED_HOLE_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "UndeferredUnusedHoleChirho.hs",
        None,
        &FrontendOptionsChirho::default(),
    )
    .map(|_| ())
    .expect_err("without deferral the hole stops compilation");
    assert!(refused_chirho.contains("found hole"), "{refused_chirho}");
}

#[test]
fn the_later_option_wins_in_either_order_chirho() {
    // T9497d: general deferral, then -fno-defer-typed-holes, leaves the hole an error.
    assert_rejected_chirho(
        UNUSED_HOLE_CHIRHO,
        &["-fdefer-type-errors", "-fno-defer-typed-holes"],
        "found hole",
    );
    assert_deferred_chirho(
        UNUSED_HOLE_CHIRHO,
        &["-fno-defer-typed-holes", "-fdefer-type-errors"],
        "found hole",
    );
}

#[test]
fn module_header_options_follow_the_callers_chirho() {
    assert_rejected_chirho(
        &with_options_pragma_chirho("-fno-defer-typed-holes"),
        &["-fdefer-typed-holes"],
        "found hole",
    );
    assert_deferred_chirho(
        &with_options_pragma_chirho("-fdefer-typed-holes"),
        &["-fno-defer-typed-holes"],
        "found hole",
    );
    assert_rejected_chirho(
        &with_options_pragma_chirho("-fdefer-type-errors -fno-defer-typed-holes"),
        &[],
        "found hole",
    );
}

#[test]
fn a_double_quoted_option_defers_and_a_single_quoted_one_does_not_chirho() {
    // GHC 9.14.1: the double-quoted switch is accepted; the single-quoted one is rejected.
    assert_deferred_chirho(
        &with_options_pragma_chirho("\"-fdefer-typed-holes\""),
        &[],
        "found hole",
    );
    assert_rejected_chirho(
        &with_options_pragma_chirho("'-fdefer-typed-holes'"),
        &[],
        "found hole",
    );
}

#[test]
fn flags_in_comments_strings_or_after_the_header_defer_nothing_chirho() {
    for source_chirho in [
        format!("-- {{-# OPTIONS_GHC -fdefer-type-errors #-}}\n{UNUSED_HOLE_CHIRHO}"),
        format!("{{- {{-# OPTIONS_GHC -fdefer-type-errors #-}} -}}\n{UNUSED_HOLE_CHIRHO}"),
        UNUSED_HOLE_CHIRHO.replacen(
            "module Main where\n",
            "module Main where\n{-# OPTIONS_GHC -fdefer-type-errors #-}\n",
            1,
        ),
        format!(
            "{UNUSED_HOLE_CHIRHO}banner :: String\nbanner = \"{{-# OPTIONS_GHC -fdefer-type-errors #-}}\"\n"
        ),
    ] {
        assert_rejected_chirho(&source_chirho, &[], "found hole");
    }
}

#[test]
fn deferring_holes_leaves_unbound_variables_and_type_errors_fatal_chirho() {
    assert_rejected_chirho(
        UNUSED_UNBOUND_VARIABLE_CHIRHO,
        &["-fdefer-typed-holes"],
        "unbound variable: `missingName`",
    );
    assert_rejected_chirho(
        UNUSED_TYPE_MISMATCH_CHIRHO,
        &["-fdefer-typed-holes"],
        "mismatch",
    );
}

#[test]
fn deferring_out_of_scope_variables_leaves_holes_and_type_errors_fatal_chirho() {
    assert_deferred_chirho(
        UNUSED_UNBOUND_VARIABLE_CHIRHO,
        &["-fdefer-out-of-scope-variables"],
        "unbound variable: `missingName`",
    );
    assert_rejected_chirho(
        UNUSED_HOLE_CHIRHO,
        &["-fdefer-out-of-scope-variables"],
        "found hole",
    );
    assert_rejected_chirho(
        UNUSED_TYPE_MISMATCH_CHIRHO,
        &["-fdefer-out-of-scope-variables"],
        "mismatch",
    );
}

#[test]
fn general_deferral_reaches_all_three_categories_chirho() {
    for expected_chirho in ["found hole", "unbound variable: `missingName`", "mismatch"] {
        assert_deferred_chirho(
            ALL_THREE_CATEGORIES_CHIRHO,
            &["-fdefer-type-errors"],
            expected_chirho,
        );
    }
}

#[test]
fn turning_general_deferral_off_keeps_the_switches_it_implied_chirho() {
    let caller_chirho = ["-fdefer-type-errors", "-fno-defer-type-errors"];
    assert_deferred_chirho(UNUSED_HOLE_CHIRHO, &caller_chirho, "found hole");
    assert_deferred_chirho(
        UNUSED_UNBOUND_VARIABLE_CHIRHO,
        &caller_chirho,
        "unbound variable: `missingName`",
    );
    assert_rejected_chirho(UNUSED_TYPE_MISMATCH_CHIRHO, &caller_chirho, "mismatch");
    assert_rejected_chirho(
        UNUSED_UNBOUND_VARIABLE_CHIRHO,
        &["-fdefer-type-errors", "-fno-defer-out-of-scope-variables"],
        "unbound variable: `missingName`",
    );
}

#[test]
fn kind_and_structural_errors_stay_fatal_under_general_deferral_chirho() {
    assert_rejected_chirho(KIND_ERROR_CHIRHO, &["-fdefer-type-errors"], "kind mismatch");
    assert_rejected_chirho(
        DUPLICATE_INSTANCE_CHIRHO,
        &["-fdefer-type-errors"],
        "duplicate instance declarations",
    );
    assert_rejected_chirho(
        UNBOUND_RECORD_CONSTRUCTOR_CHIRHO,
        &["-fdefer-type-errors"],
        "unbound constructor",
    );
}

#[test]
fn the_source_path_keeps_a_cpp_modules_header_options_chirho() {
    let run_chirho = |source_chirho: &str, caller_chirho: &[&str]| {
        eval_source_with_options_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "CppHeaderOptionsChirho.hs",
            None,
            &options_chirho(caller_chirho),
        )
        .map(|(_value_chirho, machine_chirho)| machine_chirho.io_output_chirho)
    };
    assert_eq!(
        run_chirho(CPP_DEFERRED_HOLE_CHIRHO, &[]).as_deref(),
        Ok("ran\n"),
        "the header's -fdefer-typed-holes survives preprocessing"
    );
    assert_eq!(
        run_chirho(CPP_DEFERRED_HOLE_CHIRHO, &["-fno-defer-typed-holes"]).as_deref(),
        Ok("ran\n"),
        "the module's option follows the caller's"
    );
    let refused_chirho = run_chirho(
        &CPP_DEFERRED_HOLE_CHIRHO.replace("-fdefer-typed-holes", "-fno-defer-typed-holes"),
        &["-fdefer-typed-holes"],
    )
    .expect_err("the module's -fno-defer-typed-holes follows the caller's deferral");
    assert!(refused_chirho.contains("found hole"), "{refused_chirho}");
}
