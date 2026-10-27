// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The statement hook's boundary with occurrence identity.
//!
//! One traversal now yields two different kinds of thing, and they must not
//! bleed into each other (gpt_chirho #25088): a statement is NOT an occurrence,
//! so carrying a statement hook may not mint, remint, or change how many
//! occurrences a walk sees. These controls measure that on real lowered
//! modules rather than asserting it from the shape of the code.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::expr_chirho::StmtChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::occurrences_chirho::{
    OccurrenceMutChirho, visit_decl_and_statements_chirho, visit_decl_chirho,
};
use haskelujah_ast_chirho::provenance_chirho::OriginIdChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use crate::cst_parser_chirho::parse_to_cst_chirho;

fn lower_chirho(source_chirho: &str) -> ModuleChirho {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(source_chirho, file_chirho);
    lower_module_chirho(&cst_chirho, file_chirho)
}

/// A module with do blocks, a comprehension, a bind, a let and a tail, so the
/// walk meets every statement shape.
fn sample_chirho() -> ModuleChirho {
    lower_chirho(
        "module Main where\n\
         justsChirho ms = [y | Just y <- ms, y > 0]\n\
         readTwoChirho = do\n\
         \x20 a <- getLine\n\
         \x20 let n = length a\n\
         \x20 b <- getLine\n\
         \x20 putStrLn (a ++ b ++ show n)\n\
         main = do\n\
         \x20 readTwoChirho\n\
         \x20 putStrLn \"done\"\n",
    )
}

/// Every occurrence a walk yields, as (text, origin), in order.
fn occurrences_chirho(module_chirho: &mut ModuleChirho) -> Vec<(String, Option<OriginIdChirho>)> {
    let mut found_chirho = Vec::new();
    for decl_chirho in &mut module_chirho.decls_chirho {
        visit_decl_chirho(decl_chirho, &mut |occurrence_chirho| {
            found_chirho.push((
                text_of_chirho(&occurrence_chirho),
                occurrence_chirho.origin_chirho(),
            ));
        });
    }
    found_chirho
}

/// The same, from a walk that ALSO carries a statement hook.
fn occurrences_with_hook_chirho(
    module_chirho: &mut ModuleChirho,
    statements_seen_chirho: &mut usize,
) -> Vec<(String, Option<OriginIdChirho>)> {
    let mut found_chirho = Vec::new();
    for decl_chirho in &mut module_chirho.decls_chirho {
        let mut seen_chirho = 0usize;
        visit_decl_and_statements_chirho(
            decl_chirho,
            &mut |occurrence_chirho| {
                found_chirho.push((
                    text_of_chirho(&occurrence_chirho),
                    occurrence_chirho.origin_chirho(),
                ));
            },
            &mut |_stmt_chirho: &mut StmtChirho| {
                seen_chirho += 1;
            },
        );
        *statements_seen_chirho += seen_chirho;
    }
    found_chirho
}

fn text_of_chirho(occurrence_chirho: &OccurrenceMutChirho<'_>) -> String {
    match occurrence_chirho {
        OccurrenceMutChirho::ReferenceChirho(name_chirho) => name_chirho.text_chirho().to_string(),
        OccurrenceMutChirho::LiteralChirho(lit_chirho) => format!("{lit_chirho:?}"),
    }
}

#[test]
fn a_statement_hook_changes_no_occurrence_chirho() {
    let mut without_chirho = sample_chirho();
    let mut with_chirho = sample_chirho();
    let mut statements_chirho = 0usize;

    let plain_chirho = occurrences_chirho(&mut without_chirho);
    let hooked_chirho = occurrences_with_hook_chirho(&mut with_chirho, &mut statements_chirho);

    // Same occurrences, same order, same origins. Carrying a statement hook
    // buys statements and costs nothing.
    assert_eq!(plain_chirho, hooked_chirho);
    assert!(
        !plain_chirho.is_empty(),
        "the sample yielded no occurrences"
    );
}

#[test]
fn the_statement_hook_sees_every_statement_chirho() {
    let mut module_chirho = sample_chirho();
    let mut statements_chirho = 0usize;
    let _ = occurrences_with_hook_chirho(&mut module_chirho, &mut statements_chirho);
    // `readTwoChirho` has four, `main` two, and the comprehension two qualifiers:
    // a generator and a guard, which are statements of the same type and are
    // reached by the same walk.
    assert_eq!(statements_chirho, 8);
}

#[test]
fn the_statement_hook_mints_nothing_chirho() {
    // The supply is what a mint would move. Walking with a hook, and rewriting
    // through it, must not advance it.
    let mut module_chirho = sample_chirho();
    let before_chirho = module_chirho.origin_supply_chirho.minted_chirho();
    let mut cleared_chirho = 0usize;
    for decl_chirho in &mut module_chirho.decls_chirho {
        visit_decl_and_statements_chirho(
            decl_chirho,
            &mut |_occurrence_chirho| {},
            &mut |stmt_chirho: &mut StmtChirho| {
                // A rewriting pass, of the shape the failability refinement will
                // have: drop a selection the statement turns out not to need.
                if let StmtChirho::BindChirho { fail_chirho, .. } = stmt_chirho {
                    if fail_chirho.take().is_some() {
                        cleared_chirho += 1;
                    }
                }
            },
        );
    }
    assert_eq!(
        module_chirho.origin_supply_chirho.minted_chirho(),
        before_chirho
    );
    assert_eq!(cleared_chirho, 2, "two binds to clear in the sample");
}
