// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Prefix minus against the fixity table (row 555): a negation takes in the operators
//! that bind tighter than `infixl 6`, declared fixities included, and nothing else.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use crate::cst_parser_chirho::parse_to_cst_chirho;

/// The grouping `probeChirho`'s right-hand side lowers to, fully bracketed: `(- e)` is a
/// negation, `(l op r)` an operator, `[e]` parentheses and `(f x)` an application.
fn grouping_chirho(declarations_chirho: &str, expression_chirho: &str) -> String {
    let source_chirho =
        format!("module Main where\n{declarations_chirho}probeChirho = {expression_chirho}\n");
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let module_chirho = lower_module_chirho(
        &parse_to_cst_chirho(&source_chirho, file_chirho),
        file_chirho,
    );
    module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } if name_chirho.text_chirho() == "probeChirho" => {
                match matches_chirho
                    .first()
                    .map(|arm_chirho| &arm_chirho.rhs_chirho)
                {
                    Some(RhsChirho::UnguardedChirho(expr_chirho)) => {
                        Some(bracketed_chirho(expr_chirho))
                    }
                    _ => None,
                }
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("no `probeChirho` right-hand side in {source_chirho:?}"))
}

fn bracketed_chirho(expr_chirho: &ExprChirho) -> String {
    match expr_chirho {
        ExprChirho::NegChirho { expr_chirho, .. } => {
            format!("(- {})", bracketed_chirho(expr_chirho))
        }
        ExprChirho::InfixChirho {
            left_chirho,
            op_chirho,
            right_chirho,
            ..
        } => format!(
            "({} {} {})",
            bracketed_chirho(left_chirho),
            op_chirho.text_chirho(),
            bracketed_chirho(right_chirho)
        ),
        ExprChirho::ParenChirho { inner_chirho, .. } => {
            format!("[{}]", bracketed_chirho(inner_chirho))
        }
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => format!(
            "({} {})",
            bracketed_chirho(fun_chirho),
            bracketed_chirho(arg_chirho)
        ),
        ExprChirho::VarChirho(name_chirho) => name_chirho.text_chirho().to_string(),
        ExprChirho::LitChirho(LitChirho::IntChirho(value_chirho, ..)) => value_chirho.to_string(),
        other_chirho => panic!("no bracketing for {other_chirho:?}"),
    }
}

fn assert_groupings_chirho(declarations_chirho: &str, cases_chirho: &[(&str, &str)]) {
    for (expression_chirho, expected_chirho) in cases_chirho {
        assert_eq!(
            grouping_chirho(declarations_chirho, expression_chirho),
            *expected_chirho,
            "{expression_chirho}"
        );
    }
}

#[test]
fn a_negation_takes_in_the_operators_that_bind_tighter_chirho() {
    assert_groupings_chirho(
        "",
        &[
            ("-3 ^ 2", "(- (3 ^ 2))"),
            ("- 2 ^ 2", "(- (2 ^ 2))"),
            ("-7 `mod` 3", "(- (7 mod 3))"),
            ("-3 * 2 + 1", "((- (3 * 2)) + 1)"),
            ("-2 ^ 3 * 4 - 1", "((- ((2 ^ 3) * 4)) - 1)"),
            ("4 == - 2 ^ 2", "(4 == (- (2 ^ 2)))"),
            ("f $ - 3 ^ 2", "(f (- (3 ^ 2)))"),
        ],
    );
}

#[test]
fn operators_at_or_below_prefix_minus_stay_outside_it_chirho() {
    assert_groupings_chirho(
        "",
        &[
            ("-3 + 2", "((- 3) + 2)"),
            ("-3 - 2", "((- 3) - 2)"),
            ("-3 < 2", "((- 3) < 2)"),
            ("(-3) ^ 2", "([(- 3)] ^ 2)"),
            ("negate 3 ^ 2", "((negate 3) ^ 2)"),
            ("abs (-3) ^ 2", "((abs [(- 3)]) ^ 2)"),
        ],
    );
}

#[test]
fn declared_fixities_decide_what_a_negation_takes_in_chirho() {
    assert_groupings_chirho(
        "infixl 7 .*.\ninfixl 6 .+.\ninfixr 8 .^.\n",
        &[
            ("-3 .*. 2", "(- (3 .*. 2))"),
            ("-3 .+. 2", "((- 3) .+. 2)"),
            ("-3 .^. 2 .*. 4", "(- ((3 .^. 2) .*. 4))"),
            ("-3 `f` 2", "(- (3 f 2))"),
        ],
    );
}

#[test]
fn a_direct_minimum_word_literal_preserves_its_one_extra_magnitude_chirho() {
    assert_groupings_chirho(
        "",
        &[
            ("-9223372036854775808", "(- -9223372036854775808)"),
            ("-0x8000000000000000", "(- -9223372036854775808)"),
            ("9223372036854775808", "0"),
            ("9223372036854775809", "0"),
            ("-9223372036854775808 ^ 1", "(- (0 ^ 1))"),
        ],
    );
}
