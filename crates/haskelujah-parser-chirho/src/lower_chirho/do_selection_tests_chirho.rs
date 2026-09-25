// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! What each do statement selects, as lowering records it.
//!
//! Controls: a non-tail expression statement selects `>>` and the tail selects
//! nothing; a bind selects `>>=` and reserves `fail`; a `let` selects nothing;
//! `M.do` carries the qualified binding rather than a bare name; a list
//! comprehension's generator is the same statement type and selects nothing at
//! all; and every selection carries an origin of its own.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashSet;

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::provenance_chirho::{OccurrenceRoleChirho, OriginIdChirho};
use haskelujah_ast_chirho::stmt_operation_chirho::SelectedOperationChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use crate::cst_parser_chirho::parse_to_cst_chirho;

fn lower_chirho(source_chirho: &str) -> ModuleChirho {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(source_chirho, file_chirho);
    lower_module_chirho(&cst_chirho, file_chirho)
}

/// The statements of the first `do` block in the named binding.
fn do_stmts_chirho<'m>(module_chirho: &'m ModuleChirho, name_chirho: &str) -> &'m [StmtChirho] {
    for decl_chirho in &module_chirho.decls_chirho {
        let DeclChirho::FunBindChirho {
            name_chirho: bound_chirho,
            matches_chirho,
            ..
        } = decl_chirho
        else {
            continue;
        };
        if bound_chirho.text_chirho() != name_chirho {
            continue;
        }
        if let Some(RhsChirho::UnguardedChirho(ExprChirho::DoChirho { stmts_chirho, .. })) =
            matches_chirho.first().map(|arm_chirho| &arm_chirho.rhs_chirho)
        {
            return stmts_chirho;
        }
    }
    panic!("no do block in `{name_chirho}`");
}

/// The name the selection will be looked up by, qualifier included, and the role
/// it plays. The FULL name is what matters: it is the string resolution sees.
fn described_chirho(
    operation_chirho: &SelectedOperationChirho,
) -> (String, OccurrenceRoleChirho) {
    (
        operation_chirho.name_chirho.full_name_chirho(),
        operation_chirho.role_chirho,
    )
}

/// Every selection in the module, so origins can be compared across statements.
fn selections_chirho(stmts_chirho: &[StmtChirho]) -> Vec<&SelectedOperationChirho> {
    let mut found_chirho = Vec::new();
    for stmt_chirho in stmts_chirho {
        match stmt_chirho {
            StmtChirho::ExprChirho { then_chirho, .. } => found_chirho.extend(then_chirho.iter()),
            StmtChirho::BindChirho {
                bind_chirho,
                fail_chirho,
                ..
            } => {
                found_chirho.extend(bind_chirho.iter());
                found_chirho.extend(fail_chirho.iter());
            }
            StmtChirho::LetChirho { .. } => {}
        }
    }
    found_chirho
}

#[test]
fn a_non_tail_expression_selects_then_and_the_tail_selects_nothing_chirho() {
    let module_chirho = lower_chirho(
        "module Main where\n\
         main = do\n\
         \x20 putStrLn \"a\"\n\
         \x20 putStrLn \"b\"\n",
    );
    let stmts_chirho = do_stmts_chirho(&module_chirho, "main");
    assert_eq!(stmts_chirho.len(), 2);
    let StmtChirho::ExprChirho { then_chirho, .. } = &stmts_chirho[0] else {
        panic!("expected an expression statement");
    };
    assert_eq!(
        described_chirho(then_chirho.as_ref().expect("the first statement selects `>>`")),
        (">>".to_string(), OccurrenceRoleChirho::Then)
    );
    let StmtChirho::ExprChirho { then_chirho, .. } = &stmts_chirho[1] else {
        panic!("expected an expression statement");
    };
    assert!(
        then_chirho.is_none(),
        "the tail statement selects no operator"
    );
}

#[test]
fn a_bind_selects_bind_and_reserves_fail_chirho() {
    let module_chirho = lower_chirho(
        "module Main where\n\
         main = do\n\
         \x20 line <- getLine\n\
         \x20 putStrLn line\n",
    );
    let stmts_chirho = do_stmts_chirho(&module_chirho, "main");
    let StmtChirho::BindChirho {
        bind_chirho,
        fail_chirho,
        ..
    } = &stmts_chirho[0]
    else {
        panic!("expected a bind statement");
    };
    assert_eq!(
        described_chirho(bind_chirho.as_ref().expect("a bind selects `>>=`")),
        (">>=".to_string(), OccurrenceRoleChirho::Bind)
    );
    // Reserved here, because whether it is really selected needs the constructor
    // environment; the refinement pass clears it for an irrefutable pattern.
    assert_eq!(
        described_chirho(fail_chirho.as_ref().expect("a bind reserves `fail`")),
        ("fail".to_string(), OccurrenceRoleChirho::Fail)
    );
}

#[test]
fn a_let_statement_selects_nothing_chirho() {
    let module_chirho = lower_chirho(
        "module Main where\n\
         main = do\n\
         \x20 let n = 1\n\
         \x20 print n\n",
    );
    let stmts_chirho = do_stmts_chirho(&module_chirho, "main");
    assert!(
        matches!(stmts_chirho[0], StmtChirho::LetChirho { .. }),
        "{:?}",
        stmts_chirho[0]
    );
    // A `let` has nowhere to put a selection, so "selects nothing" is its shape.
    // Only the tail remains, and it selects nothing either.
    assert_eq!(selections_chirho(stmts_chirho).len(), 0);
}

#[test]
fn a_qualified_do_carries_the_qualified_binding_chirho() {
    let module_chirho = lower_chirho(
        "{-# LANGUAGE QualifiedDo #-}\n\
         module Main where\n\
         import qualified Linear as L\n\
         main = L.do\n\
         \x20 x <- act\n\
         \x20 use x\n",
    );
    let stmts_chirho = do_stmts_chirho(&module_chirho, "main");
    let StmtChirho::BindChirho {
        bind_chirho,
        fail_chirho,
        ..
    } = &stmts_chirho[0]
    else {
        panic!("expected a bind statement");
    };
    // `L.do` selects `L.>>=`, not `>>=`. Whether `L.>>=` resolves to a local
    // binding is name resolution's business, not this selection's.
    assert_eq!(
        described_chirho(bind_chirho.as_ref().expect("a bind selects `>>=`")),
        ("L.>>=".to_string(), OccurrenceRoleChirho::Bind)
    );
    assert_eq!(
        described_chirho(fail_chirho.as_ref().expect("a bind reserves `fail`")),
        ("L.fail".to_string(), OccurrenceRoleChirho::Fail)
    );
}

#[test]
fn a_list_comprehension_generator_selects_nothing_chirho() {
    // The same `StmtChirho` type carries a comprehension's qualifiers, and a
    // generator there is not a monadic bind. Nothing may be selected for it.
    let module_chirho = lower_chirho(
        "module Main where\n\
         justsChirho ms = [y | Just y <- ms, y > 0]\n",
    );
    let mut comprehension_selections_chirho = 0;
    for decl_chirho in &module_chirho.decls_chirho {
        let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho else {
            continue;
        };
        let Some(RhsChirho::UnguardedChirho(ExprChirho::ListCompChirho { quals_chirho, .. })) =
            matches_chirho.first().map(|arm_chirho| &arm_chirho.rhs_chirho)
        else {
            continue;
        };
        comprehension_selections_chirho += selections_chirho(quals_chirho).len();
    }
    assert_eq!(comprehension_selections_chirho, 0);
}

#[test]
fn every_selection_takes_an_origin_of_its_own_chirho() {
    let module_chirho = lower_chirho(
        "module Main where\n\
         main = do\n\
         \x20 a <- one\n\
         \x20 report a\n\
         \x20 b <- two\n\
         \x20 report b\n",
    );
    let stmts_chirho = do_stmts_chirho(&module_chirho, "main");
    let selections_chirho = selections_chirho(stmts_chirho);
    // Two binds (each a `>>=` and a reserved `fail`) and two non-tail `>>`: the
    // last `report b` is the tail and selects nothing.
    assert_eq!(selections_chirho.len(), 5, "{selections_chirho:?}");
    let origins_chirho: Vec<OriginIdChirho> = selections_chirho
        .iter()
        .map(|operation_chirho| {
            operation_chirho
                .origin_chirho()
                .expect("a selection carries an origin")
        })
        .collect();
    let distinct_chirho: HashSet<OriginIdChirho> = origins_chirho.iter().copied().collect();
    assert_eq!(distinct_chirho.len(), origins_chirho.len(), "{origins_chirho:?}");
}
