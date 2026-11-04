// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC's error-deferral flag semantics without compiling a program (row 695): implications,
//! left-to-right order, caller-then-module precedence, and which pragma text is an option.
//! workflow: compiler-pipeline-chirho/error-deferral-chirho

use crate::DeferralPolicyChirho;
use haskelujah_parser_chirho::pragma_chirho::header_ghc_options_chirho;
use haskelujah_span_chirho::FileIdChirho;

fn words_chirho(options_chirho: &[&str]) -> Vec<String> {
    options_chirho
        .iter()
        .map(|option_chirho| (*option_chirho).to_string())
        .collect()
}

/// (type errors, typed holes, out-of-scope variables) under the caller's then the module's
/// options.
fn categories_chirho(caller_chirho: &[&str], module_chirho: &[&str]) -> (bool, bool, bool) {
    let policy_chirho = DeferralPolicyChirho::resolve_chirho(
        &words_chirho(caller_chirho),
        &words_chirho(module_chirho),
    );
    (
        policy_chirho.type_errors_chirho,
        policy_chirho.typed_holes_chirho,
        policy_chirho.out_of_scope_variables_chirho,
    )
}

fn header_chirho(source_chirho: &str) -> Vec<String> {
    header_ghc_options_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
}

#[test]
fn nothing_defers_without_an_option_chirho() {
    assert_eq!(categories_chirho(&[], &[]), (false, false, false));
}

#[test]
fn general_deferral_turns_on_all_three_categories_chirho() {
    assert_eq!(
        categories_chirho(&["-fdefer-type-errors"], &[]),
        (true, true, true)
    );
}

#[test]
fn the_general_negative_turns_off_only_itself_chirho() {
    assert_eq!(
        categories_chirho(&["-fdefer-type-errors", "-fno-defer-type-errors"], &[]),
        (false, true, true)
    );
}

#[test]
fn each_category_switch_changes_only_its_own_category_chirho() {
    assert_eq!(
        categories_chirho(&["-fdefer-typed-holes"], &[]),
        (false, true, false)
    );
    assert_eq!(
        categories_chirho(&["-fdefer-out-of-scope-variables"], &[]),
        (false, false, true)
    );
    assert_eq!(
        categories_chirho(
            &["-fdefer-type-errors", "-fno-defer-out-of-scope-variables"],
            &[]
        ),
        (true, true, false)
    );
}

#[test]
fn the_later_option_wins_in_either_order_chirho() {
    // T9497d's all.T order: the general switch, then the hole negative, leaves holes fatal.
    assert_eq!(
        categories_chirho(&["-fdefer-type-errors", "-fno-defer-typed-holes"], &[]),
        (true, false, true)
    );
    // The reverse order: the general switch turns typed holes back on.
    assert_eq!(
        categories_chirho(&["-fno-defer-typed-holes", "-fdefer-type-errors"], &[]),
        (true, true, true)
    );
}

#[test]
fn module_header_options_apply_after_the_callers_chirho() {
    assert_eq!(
        categories_chirho(&["-fdefer-typed-holes"], &["-fno-defer-typed-holes"]),
        (false, false, false)
    );
    assert_eq!(
        categories_chirho(&["-fno-defer-typed-holes"], &["-fdefer-typed-holes"]),
        (false, true, false)
    );
}

#[test]
fn other_options_change_nothing_chirho() {
    // T9497b/c pass -fno-warn-typed-holes: it silences the warning, it does not defer.
    assert_eq!(
        categories_chirho(
            &[
                "-Wall",
                "-XGADTs",
                "-fno-warn-typed-holes",
                "-fno-max-valid-hole-fits"
            ],
            &["-O2"]
        ),
        (false, false, false)
    );
}

#[test]
fn header_pragmas_give_their_option_words_in_order_chirho() {
    assert_eq!(
        header_chirho(
            "{-# LANGUAGE GADTs #-}\n-- a comment\n{-# OPTIONS_GHC -Wall\n    -fdefer-typed-holes #-}\n{-# options_ghc -fno-defer-type-errors #-}\n{-# OPTIONS -fdefer-out-of-scope-variables #-}\nmodule Main where\nmain :: IO ()\nmain = pure ()\n"
        ),
        words_chirho(&[
            "-Wall",
            "-fdefer-typed-holes",
            "-fno-defer-type-errors",
            "-fdefer-out-of-scope-variables",
        ])
    );
}

#[test]
fn flags_outside_header_option_pragmas_are_not_options_chirho() {
    for source_chirho in [
        "-- {-# OPTIONS_GHC -fdefer-type-errors #-}\nmodule Main where\n",
        "{- {-# OPTIONS_GHC -fdefer-type-errors #-} -}\nmodule Main where\n",
        "module Main where\n{-# OPTIONS_GHC -fdefer-type-errors #-}\nmain :: IO ()\nmain = pure ()\n",
        "module Main where\nbanner :: String\nbanner = \"{-# OPTIONS_GHC -fdefer-type-errors #-}\"\n",
        "{-# OPTIONS_HADDOCK hide #-}\nmodule Main where\n",
    ] {
        assert_eq!(
            header_chirho(source_chirho),
            Vec::<String>::new(),
            "{source_chirho}"
        );
    }
}
