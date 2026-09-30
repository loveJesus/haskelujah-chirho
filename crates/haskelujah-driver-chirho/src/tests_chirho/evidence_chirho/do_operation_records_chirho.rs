// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The checker's records for the operations do statements SELECTED.
//!
//! Before brick 6 the checker never looked at a do statement's operation and
//! produced no record for one. Now each selected `>>=`, `>>` and `fail` is
//! looked up, instantiated from its ACTUAL scheme, and recorded under the
//! selection's own origin. These controls check that through the real front end:
//! every selection that fits carries its own record at its own instance, a
//! statement that selects nothing carries none, and an irrefutable bind gets no
//! MonadFail record — the obligation the whole unit exists to avoid inventing.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::expr_chirho::StmtChirho;
use haskelujah_ast_chirho::occurrences_chirho::visit_decl_and_statements_chirho;
use haskelujah_ast_chirho::provenance_chirho::{OccurrenceRoleChirho, OriginIdChirho};
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_syntax_chirho::SourceFileChirho;

fn frontend_chirho(source_chirho: &str) -> crate::FrontendResultChirho {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "DoOperationRecordsChirho.hs",
        source_chirho,
    );
    crate::run_frontend_chirho(
        source_chirho,
        source_file_chirho.file_id_chirho(),
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &HashMap::new(),
    )
    .unwrap_or_else(|diag_chirho| panic!("front end rejected the control: {diag_chirho:?}"))
}

/// Every selection the checked module's do statements carry, in source order.
fn selections_chirho(
    result_chirho: &mut crate::FrontendResultChirho,
) -> Vec<(OccurrenceRoleChirho, OriginIdChirho)> {
    let mut found_chirho = Vec::new();
    for decl_chirho in &mut result_chirho.module_chirho.decls_chirho {
        visit_decl_and_statements_chirho(
            decl_chirho,
            &mut |_occurrence_chirho| {},
            &mut |stmt_chirho: &mut StmtChirho| {
                let selected_chirho = match stmt_chirho {
                    StmtChirho::ExprChirho { then_chirho, .. } => vec![then_chirho.as_ref()],
                    StmtChirho::BindChirho {
                        bind_chirho,
                        fail_chirho,
                        ..
                    } => vec![bind_chirho.as_ref(), fail_chirho.as_ref()],
                    StmtChirho::LetChirho { .. } => vec![],
                };
                for selection_chirho in selected_chirho.into_iter().flatten() {
                    found_chirho.push((
                        selection_chirho.role_chirho,
                        selection_chirho
                            .origin_chirho()
                            .expect("a selection carries an origin"),
                    ));
                }
            },
        );
    }
    found_chirho
}

/// The (class, instance) records the checker holds for one origin.
fn records_chirho(
    result_chirho: &crate::FrontendResultChirho,
    origin_chirho: OriginIdChirho,
) -> Vec<(String, String)> {
    result_chirho
        .infer_result_chirho
        .method_occurrences_chirho
        .iter()
        .filter(|record_chirho| record_chirho.origin_chirho == Some(origin_chirho))
        .map(|record_chirho| {
            (
                record_chirho.class_name_chirho.clone(),
                record_chirho.ty_key_chirho.clone(),
            )
        })
        .collect()
}

fn monad_chirho(key_chirho: &str) -> Vec<(String, String)> {
    vec![("Monad".to_string(), key_chirho.to_string())]
}

#[test]
fn each_selected_operation_in_io_is_recorded_at_io_chirho() {
    let mut result_chirho = frontend_chirho(
        "module Main where\n\
         main :: IO ()\n\
         main = do\n\
         \x20 line <- getLine\n\
         \x20 putStrLn line\n\
         \x20 putStrLn \"done\"\n",
    );
    let selections_chirho = selections_chirho(&mut result_chirho);
    // A bind (with its variable pattern, which selects no `fail`) and one
    // non-tail `>>`. The tail selects nothing.
    let roles_chirho: Vec<OccurrenceRoleChirho> = selections_chirho
        .iter()
        .map(|(role_chirho, _)| *role_chirho)
        .collect();
    assert_eq!(
        roles_chirho,
        vec![OccurrenceRoleChirho::Bind, OccurrenceRoleChirho::Then],
        "{selections_chirho:?}"
    );
    for (role_chirho, origin_chirho) in selections_chirho {
        assert_eq!(
            records_chirho(&result_chirho, origin_chirho),
            monad_chirho("IO"),
            "{role_chirho:?}"
        );
    }
}

#[test]
fn an_irrefutable_bind_gets_no_monad_fail_record_chirho() {
    // The point of the whole unit: `(a, b) <- m` cannot fail, so its `fail`
    // reservation is cleared and no MonadFail evidence exists for it.
    let mut result_chirho = frontend_chirho(
        "module Main where\n\
         main :: IO ()\n\
         main = do\n\
         \x20 (a, b) <- pure (3 :: Int, 4 :: Int)\n\
         \x20 print (a + b)\n",
    );
    let selections_chirho = selections_chirho(&mut result_chirho);
    assert!(
        selections_chirho
            .iter()
            .all(|(role_chirho, _)| *role_chirho != OccurrenceRoleChirho::Fail),
        "an irrefutable pattern still selected `fail`: {selections_chirho:?}"
    );
    let monad_fail_records_chirho = result_chirho
        .infer_result_chirho
        .method_occurrences_chirho
        .iter()
        .filter(|record_chirho| record_chirho.class_name_chirho == "MonadFail")
        .count();
    assert_eq!(monad_fail_records_chirho, 0);
    let bind_chirho = selections_chirho
        .iter()
        .find(|(role_chirho, _)| *role_chirho == OccurrenceRoleChirho::Bind)
        .expect("the bind is selected");
    assert_eq!(
        records_chirho(&result_chirho, bind_chirho.1),
        monad_chirho("IO")
    );
}

#[test]
fn a_refutable_bind_in_maybe_records_its_monad_fail_chirho() {
    // A genuinely failable pattern keeps its `fail`, and that `fail` is recorded
    // at MonadFail Maybe - which is where Maybe's `Nothing` on a failed match
    // comes from.
    let mut result_chirho = frontend_chirho(
        "module Main where\n\
         firstChirho :: Maybe Int\n\
         firstChirho = do\n\
         \x20 Just x <- Just (Just 9)\n\
         \x20 return x\n\
         main :: IO ()\n\
         main = print firstChirho\n",
    );
    let selections_chirho = selections_chirho(&mut result_chirho);
    let origin_of_chirho = |wanted_chirho: OccurrenceRoleChirho| {
        selections_chirho
            .iter()
            .find(|(role_chirho, _)| *role_chirho == wanted_chirho)
            .map(|(_, origin_chirho)| *origin_chirho)
            .unwrap_or_else(|| panic!("no {wanted_chirho:?} selected: {selections_chirho:?}"))
    };
    assert_eq!(
        records_chirho(&result_chirho, origin_of_chirho(OccurrenceRoleChirho::Bind)),
        monad_chirho("Maybe")
    );
    assert_eq!(
        records_chirho(&result_chirho, origin_of_chirho(OccurrenceRoleChirho::Fail)),
        vec![("MonadFail".to_string(), "Maybe".to_string())]
    );
}

#[test]
fn repeated_operations_each_keep_their_own_record_chirho() {
    // Two binds and a `>>` in one block: three origins, three records, none
    // shared - a record that fed two statements would be the positional join
    // this whole line of work replaced.
    let mut result_chirho = frontend_chirho(
        "module Main where\n\
         main :: IO ()\n\
         main = do\n\
         \x20 a <- getLine\n\
         \x20 b <- getLine\n\
         \x20 putStrLn a\n\
         \x20 putStrLn b\n",
    );
    let selections_chirho = selections_chirho(&mut result_chirho);
    assert_eq!(selections_chirho.len(), 3, "{selections_chirho:?}");
    let origins_chirho: std::collections::HashSet<OriginIdChirho> = selections_chirho
        .iter()
        .map(|(_, origin_chirho)| *origin_chirho)
        .collect();
    assert_eq!(origins_chirho.len(), 3);
    for (role_chirho, origin_chirho) in selections_chirho {
        assert_eq!(
            records_chirho(&result_chirho, origin_chirho),
            monad_chirho("IO"),
            "{role_chirho:?}"
        );
    }
}

#[test]
fn tail_and_let_statements_select_and_record_nothing_chirho() {
    let mut result_chirho = frontend_chirho(
        "module Main where\n\
         main :: IO ()\n\
         main = do\n\
         \x20 let n = 1 :: Int\n\
         \x20 print n\n",
    );
    // A `let` and a tail: neither selects an operation, so nothing is recorded
    // for a do operator at all.
    assert!(selections_chirho(&mut result_chirho).is_empty());
}

#[test]
fn a_qualified_non_monad_operator_gets_no_invented_evidence_chirho() {
    // gpt_chirho's gate item: a qualified operator that is NOT a Monad method.
    // Here `>>=` is a plain local function, `a -> (a -> b) -> b`, used by a
    // self-qualified do over bare Ints. Its type cannot fit a monadic shape, so
    // the checker must record NOTHING for it - no Monad evidence invented for an
    // operation that has no Monad in it. (That it still runs, printing 5, is
    // eval_self_qualified_do_uses_local_bind_chirho's job.)
    //
    // WHAT THIS DOES NOT DISCRIMINATE, measured by mutation: a MANUFACTURED
    // `Monad m` recorded regardless of fit also passes here, because over bare
    // Ints the block's `m` never resolves to a concrete head, so the manufactured
    // claim never finalizes into a record. This control pins the outcome - no
    // Monad evidence for a non-Monad operator - not which scheme produced it. The
    // control that DOES catch a manufactured constraint is the refutable-Maybe one
    // above, where `fail` must come out as MonadFail rather than Monad.
    let mut result_chirho = frontend_chirho(
        "{-# LANGUAGE QualifiedDo #-}\n\
         module QualifiedDoRuntimeChirho where\n\
         valueChirho >>= nextChirho = nextChirho valueChirho\n\
         main = print (QualifiedDoRuntimeChirho.do\n\
         \x20 valueChirho <- 4\n\
         \x20 valueChirho + 1)\n",
    );
    let selections_chirho = selections_chirho(&mut result_chirho);
    assert!(
        selections_chirho
            .iter()
            .any(|(role_chirho, _)| *role_chirho == OccurrenceRoleChirho::Bind),
        "the qualified bind is still SELECTED: {selections_chirho:?}"
    );
    for (role_chirho, origin_chirho) in selections_chirho {
        let records_chirho = records_chirho(&result_chirho, origin_chirho);
        assert!(
            records_chirho
                .iter()
                .all(|(class_chirho, _)| class_chirho != "Monad" && class_chirho != "MonadFail"),
            "{role_chirho:?} was given evidence it has no class for: {records_chirho:?}"
        );
    }
}
