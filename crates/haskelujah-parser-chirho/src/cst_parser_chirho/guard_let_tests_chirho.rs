// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A guard's `let` ends where its bindings do: at the `,` before the next
//! qualifier, or at the `=` or `->` before the arm's body, even on one line
//! where indentation closes nothing. Each source was checked on GHC 9.14.1
//! (probe set T01-T10). Before this, the block ran on over the next qualifier
//! and `y > 10 = y` became a binding that defined `>`.
//! workflow: language-features-chirho/pattern-matching-chirho

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{
    ExprChirho, GuardedExprChirho, LocalBindChirho, RhsChirho, StmtChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::ParserChirho;
use crate::lower_chirho::lower_module_chirho;

fn lower_chirho(source_chirho: &str) -> ModuleChirho {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let green_chirho = ParserChirho::new_chirho(source_chirho, file_chirho).parse_chirho();
    lower_module_chirho(&green_chirho, file_chirho)
}

/// The guarded arms of `fChirho`'s first equation.
fn first_arms_chirho(module_chirho: &ModuleChirho) -> &[GuardedExprChirho] {
    let matches_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } if name_chirho.text_chirho() == "fChirho" => Some(matches_chirho),
            _ => None,
        })
        .expect("fChirho");
    match &matches_chirho[0].rhs_chirho {
        RhsChirho::GuardedChirho(arms_chirho) => arms_chirho,
        other_chirho => panic!("expected guarded arms, got {other_chirho:?}"),
    }
}

/// The names a `let` qualifier binds.
fn let_names_chirho(qual_chirho: &StmtChirho) -> Vec<String> {
    let StmtChirho::LetChirho { binds_chirho, .. } = qual_chirho else {
        panic!("expected a let qualifier, got {qual_chirho:?}");
    };
    binds_chirho
        .iter()
        .filter_map(|bind_chirho| match bind_chirho {
            LocalBindChirho::FunBindChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            LocalBindChirho::PatBindChirho {
                pat_chirho: PatChirho::VarChirho(name_chirho),
                ..
            } => Some(name_chirho.text_chirho().to_string()),
            _ => None,
        })
        .collect()
}

fn infix_op_chirho(expr_chirho: &ExprChirho) -> Option<&str> {
    match expr_chirho {
        ExprChirho::InfixChirho { op_chirho, .. } => Some(op_chirho.text_chirho()),
        _ => None,
    }
}

#[test]
fn a_one_line_let_guard_ends_at_its_comma_chirho() {
    // T01.
    let module_chirho = lower_chirho(
        "module M where\nfChirho x\n  | let y = x * 2, y > 10 = y\n  | otherwise = 0\n",
    );
    let arms_chirho = first_arms_chirho(&module_chirho);
    assert_eq!(arms_chirho.len(), 2, "{arms_chirho:?}");
    let quals_chirho = &arms_chirho[0].quals_chirho;
    assert_eq!(quals_chirho.len(), 2, "{quals_chirho:?}");
    assert_eq!(let_names_chirho(&quals_chirho[0]), ["y"]);
    assert!(
        matches!(&quals_chirho[1], StmtChirho::ExprChirho { expr_chirho, .. } if infix_op_chirho(expr_chirho) == Some(">")),
        "{quals_chirho:?}"
    );
    assert!(
        matches!(&arms_chirho[0].body_chirho, ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "y")
    );
}

#[test]
fn a_let_guard_before_the_body_ends_at_its_equals_chirho() {
    // T05, and T03: two bindings separated by an explicit `;`.
    let module_chirho = lower_chirho("module M where\nfChirho x\n  | let y = x * 2 = y\n");
    let arms_chirho = first_arms_chirho(&module_chirho);
    assert_eq!(let_names_chirho(&arms_chirho[0].quals_chirho[0]), ["y"]);
    assert!(
        matches!(&arms_chirho[0].body_chirho, ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "y")
    );
    let module_chirho = lower_chirho(
        "module M where\nfChirho x\n  | let a = x + 1; b = a * 2, b > 10 = b\n  | otherwise = 0\n",
    );
    let arms_chirho = first_arms_chirho(&module_chirho);
    assert_eq!(
        let_names_chirho(&arms_chirho[0].quals_chirho[0]),
        ["a", "b"]
    );
    assert_eq!(arms_chirho[0].quals_chirho.len(), 2);
}

#[test]
fn a_line_aligned_with_the_closed_block_continues_the_body_chirho() {
    // T06: `+ 1` sits at the `let` block's column. The block already closed
    // at the comma, so the line continues the arm's body, as in GHC.
    let module_chirho = lower_chirho(
        "module M where\nfChirho x\n  | let y = x * 2, y > 10 = y\n        + 1\n  | otherwise = 0\n",
    );
    let arms_chirho = first_arms_chirho(&module_chirho);
    assert_eq!(arms_chirho.len(), 2, "{arms_chirho:?}");
    assert_eq!(infix_op_chirho(&arms_chirho[0].body_chirho), Some("+"));
}

#[test]
fn a_binding_keeps_the_commas_of_its_own_guards_chirho() {
    // T04: the commas inside `g`'s guard belong to `g`; the one after `False`
    // ends the `let`.
    let module_chirho = lower_chirho(
        "module M where\nfChirho x\n  | let g | x > 0, x < 10 = True | otherwise = False, g = 1\n  | otherwise = 0\n",
    );
    let arms_chirho = first_arms_chirho(&module_chirho);
    let quals_chirho = &arms_chirho[0].quals_chirho;
    assert_eq!(let_names_chirho(&quals_chirho[0]), ["g"]);
    assert!(
        matches!(&quals_chirho[1], StmtChirho::ExprChirho { expr_chirho: ExprChirho::VarChirho(name_chirho), .. } if name_chirho.text_chirho() == "g"),
        "{quals_chirho:?}"
    );
}

#[test]
fn a_case_alternative_let_guard_ends_at_its_comma_chirho() {
    // T02.
    let module_chirho = lower_chirho(
        "module M where\nfChirho n = case n of\n  k | let d = k * 3, d > 10 -> d\n  _ -> 0\n",
    );
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::FunBindChirho { .. }))
        .expect("fChirho");
    let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho else {
        unreachable!()
    };
    let RhsChirho::UnguardedChirho(ExprChirho::CaseChirho { alts_chirho, .. }) =
        &matches_chirho[0].rhs_chirho
    else {
        panic!("expected a case, got {:?}", matches_chirho[0].rhs_chirho);
    };
    assert_eq!(alts_chirho.len(), 2, "{alts_chirho:?}");
    let RhsChirho::GuardedChirho(arms_chirho) = &alts_chirho[0].rhs_chirho else {
        panic!(
            "expected a guarded alternative, got {:?}",
            alts_chirho[0].rhs_chirho
        );
    };
    assert_eq!(let_names_chirho(&arms_chirho[0].quals_chirho[0]), ["d"]);
    assert_eq!(arms_chirho[0].quals_chirho.len(), 2);
}
