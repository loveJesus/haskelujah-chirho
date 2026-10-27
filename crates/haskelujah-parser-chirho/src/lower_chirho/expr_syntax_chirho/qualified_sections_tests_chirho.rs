// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A qualified operator keeps its qualifier on every section lowering path (row 609).
//! - The direct-token path always split `P.-` with `name_from_text_chirho`.
//! - The right-section fallback is taken when the parser has folded the trailing operator into
//!   an infix node. It made one unqualified name `P.-` of that operator, and of each operator
//!   before it, and the desugarer then built a constructor application of it.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use crate::cst_parser_chirho::parse_to_cst_chirho;

/// `probeChirho`'s right-hand side, lowered.
fn probe_chirho(expression_chirho: &str) -> ExprChirho {
    let source_chirho = format!("module Main where\nprobeChirho = {expression_chirho}\n");
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let module_chirho = lower_module_chirho(
        &parse_to_cst_chirho(&source_chirho, file_chirho),
        file_chirho,
    );
    module_chirho
        .decls_chirho
        .into_iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                mut matches_chirho,
                ..
            } if name_chirho.text_chirho() == "probeChirho" => {
                match matches_chirho.remove(0).rhs_chirho {
                    RhsChirho::UnguardedChirho(expr_chirho) => Some(expr_chirho),
                    _ => None,
                }
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("no `probeChirho` right-hand side in {expression_chirho:?}"))
}

/// An operator's qualifier and its own text.
fn parts_chirho(name_chirho: &NameChirho) -> (Option<&str>, &str) {
    (name_chirho.qualifier_chirho(), name_chirho.text_chirho())
}

#[test]
fn an_operator_first_qualified_section_keeps_its_qualifier_chirho() {
    match probe_chirho("(P.- 1)") {
        ExprChirho::LeftSectionChirho { op_chirho, .. } => {
            assert_eq!(parts_chirho(&op_chirho), (Some("P"), "-"));
        }
        other_chirho => panic!("expected a left section, got {other_chirho:?}"),
    }
}

#[test]
fn an_operand_first_qualified_section_keeps_its_qualifier_chirho() {
    for (source_chirho, qualifier_chirho, operator_chirho) in [
        ("(2 P.-)", "P", "-"),
        ("(xChirho P.*)", "P", "*"),
        ("(6 Prelude./)", "Prelude", "/"),
        ("(2 `P.div`)", "P", "div"),
    ] {
        match probe_chirho(source_chirho) {
            ExprChirho::RightSectionChirho { op_chirho, .. } => assert_eq!(
                parts_chirho(&op_chirho),
                (Some(qualifier_chirho), operator_chirho),
                "{source_chirho}"
            ),
            other_chirho => {
                panic!("{source_chirho}: expected a right section, got {other_chirho:?}")
            }
        }
    }
}

#[test]
fn nested_qualified_operators_keep_their_qualifiers_chirho() {
    match probe_chirho("(1 P.+ 2 P.-)") {
        ExprChirho::RightSectionChirho {
            op_chirho,
            arg_chirho,
            ..
        } => {
            assert_eq!(parts_chirho(&op_chirho), (Some("P"), "-"));
            match *arg_chirho {
                ExprChirho::InfixChirho {
                    op_chirho: inner_chirho,
                    ..
                } => assert_eq!(parts_chirho(&inner_chirho), (Some("P"), "+")),
                other_chirho => panic!("expected the infix operand, got {other_chirho:?}"),
            }
        }
        other_chirho => panic!("expected a right section, got {other_chirho:?}"),
    }
}

#[test]
fn a_dotted_unqualified_operator_stays_unqualified_chirho() {
    match probe_chirho("(1 :.:)") {
        ExprChirho::RightSectionChirho { op_chirho, .. } => {
            assert_eq!(parts_chirho(&op_chirho), (None, ":.:"));
        }
        other_chirho => panic!("expected a right section, got {other_chirho:?}"),
    }
}
