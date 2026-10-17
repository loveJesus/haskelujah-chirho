// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A list comprehension's `let` ends where its bindings do: at the `,` before
//! the next qualifier, the `|` of a parallel branch, or the closing `]`, even on
//! one line where indentation closes nothing. Each source was checked on GHC
//! 9.14.1 (claude2_chirho's controls CL01-CL16). Before this, the block's close
//! stayed after the `]`, where it closed the enclosing do block, `where` or
//! module: the next statement or declaration was lost or misread.
//! workflow: parallel-list-comprehensions-chirho

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{
    ExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho, StmtChirho,
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

/// The first clause of the top-level binding `wanted_chirho`.
fn clause_chirho<'m>(module_chirho: &'m ModuleChirho, wanted_chirho: &str) -> &'m MatchArmChirho {
    module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } if name_chirho.text_chirho() == wanted_chirho => matches_chirho.first(),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no binding named {wanted_chirho}: {module_chirho:?}"))
}

/// The body of an unguarded clause.
fn body_chirho(clause_chirho: &MatchArmChirho) -> &ExprChirho {
    match &clause_chirho.rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
        other_chirho => panic!("expected an unguarded body, got {other_chirho:?}"),
    }
}

/// The statements of `main`'s do block.
fn main_statements_chirho(module_chirho: &ModuleChirho) -> &[StmtChirho] {
    match body_chirho(clause_chirho(module_chirho, "main")) {
        ExprChirho::DoChirho { stmts_chirho, .. } => stmts_chirho,
        other_chirho => panic!("expected main's do block, got {other_chirho:?}"),
    }
}

/// A comprehension's qualifiers, one group per parallel branch.
fn qualifier_groups_chirho(expr_chirho: &ExprChirho) -> Vec<&[StmtChirho]> {
    match expr_chirho {
        ExprChirho::ListCompChirho {
            quals_chirho,
            parallel_quals_chirho,
            ..
        } if parallel_quals_chirho.is_empty() => vec![quals_chirho.as_slice()],
        ExprChirho::ListCompChirho {
            parallel_quals_chirho,
            ..
        } => parallel_quals_chirho.iter().map(Vec::as_slice).collect(),
        other_chirho => panic!("expected a list comprehension, got {other_chirho:?}"),
    }
}

/// The names a binding list defines.
fn bound_names_chirho(binds_chirho: &[LocalBindChirho]) -> Vec<String> {
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

/// The names a `let` qualifier binds.
fn let_names_chirho(qual_chirho: &StmtChirho) -> Vec<String> {
    match qual_chirho {
        StmtChirho::LetChirho { binds_chirho, .. } => bound_names_chirho(binds_chirho),
        other_chirho => panic!("expected a let qualifier, got {other_chirho:?}"),
    }
}

/// The qualifier groups of the top-level comprehension `ysChirho`.
fn ys_groups_chirho(source_chirho: &str) -> Vec<Vec<StmtChirho>> {
    let module_chirho = lower_chirho(source_chirho);
    qualifier_groups_chirho(body_chirho(clause_chirho(&module_chirho, "ysChirho")))
        .into_iter()
        .map(<[StmtChirho]>::to_vec)
        .collect()
}

#[test]
fn the_statement_after_a_comprehension_let_stays_in_its_do_block_chirho() {
    // CL01.
    let module_chirho = lower_chirho(
        "module Main where\nmain = do\n  print [y | x <- [1, 2], let y = x + 1]\n  print 7\n",
    );
    assert_eq!(main_statements_chirho(&module_chirho).len(), 2);
    // CL05: two lets and a filter; this next statement was silently dropped.
    let module_chirho = lower_chirho(
        "module Main where\nmain = do\n  print [z | x <- [1 .. 6], let y = x + 1, let z = y * 2, z > 6]\n  putStrLn \"next\"\n",
    );
    assert_eq!(main_statements_chirho(&module_chirho).len(), 2);
    // CL04: a do-let holding the comprehension, then two statements.
    let module_chirho = lower_chirho(
        "module Main where\nmain = do\n  let ysChirho = [y | x <- [1, 2], let y = x + 1]\n  print ysChirho\n  print (length ysChirho)\n",
    );
    assert_eq!(main_statements_chirho(&module_chirho).len(), 3);
}

#[test]
fn the_declarations_after_a_comprehension_let_stay_in_their_block_chirho() {
    // CL02: at top level the following declarations were swallowed.
    let module_chirho = lower_chirho(
        "module Main where\nysChirho = [y | x <- [1, 2], let y = x + 1]\nzsChirho = map negate ysChirho\nmain = print (ysChirho, zsChirho)\n",
    );
    let groups_chirho =
        qualifier_groups_chirho(body_chirho(clause_chirho(&module_chirho, "ysChirho")));
    assert_eq!(groups_chirho.len(), 1);
    assert_eq!(groups_chirho[0].len(), 2);
    assert_eq!(let_names_chirho(&groups_chirho[0][1]), ["y"]);
    clause_chirho(&module_chirho, "zsChirho");
    clause_chirho(&module_chirho, "main");
    // CL11: the where block goes on past the comprehension.
    let module_chirho = lower_chirho(
        "module Main where\nfChirho = zs\n  where ys = [y | x <- [1, 2], let y = x + 1]\n        zs = ys ++ ys\nmain = print fChirho\n",
    );
    assert_eq!(
        bound_names_chirho(&clause_chirho(&module_chirho, "fChirho").where_binds_chirho),
        ["ys", "zs"]
    );
    clause_chirho(&module_chirho, "main");
}

#[test]
fn a_comprehension_let_ends_at_each_of_its_delimiters_chirho() {
    // CL06: two bindings on one line, then `]`.
    let groups_chirho = ys_groups_chirho(
        "module Main where\nysChirho = [y + w | x <- [1, 2], let y = x; w = 10]\n",
    );
    assert_eq!(let_names_chirho(&groups_chirho[0][1]), ["y", "w"]);
    // CL12: the `,` before a filter.
    let groups_chirho = ys_groups_chirho(
        "module Main where\nysChirho = [y | x <- [1 .. 5], let y = x * 3, odd y]\n",
    );
    assert_eq!(groups_chirho[0].len(), 3);
    assert_eq!(let_names_chirho(&groups_chirho[0][1]), ["y"]);
    assert!(matches!(groups_chirho[0][2], StmtChirho::ExprChirho { .. }));
    // CL08: the `|` of a parallel branch.
    let groups_chirho = ys_groups_chirho(
        "{-# LANGUAGE ParallelListComp #-}\nmodule Main where\nysChirho = [y + z | x <- [1, 2], let y = x * 10 | z <- [3, 4]]\n",
    );
    assert_eq!(groups_chirho.len(), 2);
    assert_eq!(let_names_chirho(&groups_chirho[0][1]), ["y"]);
    assert_eq!(groups_chirho[1].len(), 1);
    // CL13: a binding's own guards keep their commas and bars.
    let groups_chirho = ys_groups_chirho(
        "module Main where\nysChirho = [f x | x <- [1, 5, 50], let f n | n > 0, n < 10 = 'a' | otherwise = 'b']\n",
    );
    assert_eq!(groups_chirho[0].len(), 2);
    assert_eq!(let_names_chirho(&groups_chirho[0][1]), ["f"]);
    // CL09: explicit braces close themselves.
    let groups_chirho =
        ys_groups_chirho("module Main where\nysChirho = [y | x <- [1, 2], let { y = x + 1 }]\n");
    assert_eq!(let_names_chirho(&groups_chirho[0][1]), ["y"]);
}
