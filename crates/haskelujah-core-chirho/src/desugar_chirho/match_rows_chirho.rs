// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rows of patterns, tried top to bottom, as Haskell specifies.
//!
//! A case expression, a function's equations and a lambda are all the same
//! thing underneath: rows, each a list of patterns matched left to right against
//! the scrutinee variables, then optional `where` bindings, then guards. The
//! first row whose patterns match AND one of whose guards holds wins. A row that
//! fails ANYWHERE - a constructor, a nested literal, or its last guard - goes on
//! to the next row. That is the whole rule, and it is what this module builds.
//!
//! Each row is compiled by the single-row matcher against a placeholder for
//! "the next row". The rows are compiled in SOURCE order, because method
//! evidence joins occurrences by their source-order ordinal. They are then
//! joined bottom-up, and the join decides what the continuation costs:
//!
//! - a row that never falls through (irrefutable and unguarded) makes every row
//!   after it unreachable, and they are dropped;
//! - a continuation used once is inlined, so a plain `f [] = ..; f (x:xs) = ..`
//!   allocates nothing; and when the row is a `case` on the same variable whose
//!   default is that continuation, the two cases merge into one switch;
//! - a continuation used more than once (a guard and a nested pattern that can
//!   both fall through) is bound once with `let`, never duplicated.
//! workflow: language-features-chirho/pattern-matching-chirho

use haskelujah_ast_chirho::expr_chirho::{LocalBindChirho, RhsChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;

use super::DesugarCtxChirho;
use crate::expr_chirho::{AltConChirho, BinderChirho, CoreAltChirho, CoreExprChirho, CoreIdChirho};
use crate::simplify_chirho::{UsageChirho, count_usage_chirho, subst_var_chirho};

/// One row: its patterns, one per scrutinee, and what it yields.
pub(super) struct MatchRowChirho<'a> {
    pub(super) pats_chirho: Vec<&'a PatChirho>,
    pub(super) rhs_chirho: &'a RhsChirho,
    pub(super) where_binds_chirho: &'a [LocalBindChirho],
}

impl DesugarCtxChirho {
    /// Try `rows_chirho` in order against `scrutinees_chirho`; `failure_chirho`
    /// is what happens when no row matches.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn compile_rows_chirho(
        &mut self,
        scrutinees_chirho: &[CoreIdChirho],
        rows_chirho: &[MatchRowChirho<'_>],
        failure_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let compiled_chirho: Vec<(BinderChirho, CoreExprChirho)> = rows_chirho
            .iter()
            .map(|row_chirho| {
                let next_chirho = self.row_binder_chirho("$nextrow");
                let body_chirho = self.compile_row_chirho(
                    row_chirho,
                    scrutinees_chirho,
                    &CoreExprChirho::VarChirho(next_chirho.id_chirho),
                );
                (next_chirho, body_chirho)
            })
            .collect();
        compiled_chirho.into_iter().rev().fold(
            failure_chirho,
            |rest_chirho, (next_chirho, row_chirho)| {
                join_row_chirho(row_chirho, next_chirho, rest_chirho)
            },
        )
    }

    /// One row: its patterns in order, then its `where` bindings over its
    /// guards; anything that fails continues with `next_chirho`.
    fn compile_row_chirho(
        &mut self,
        row_chirho: &MatchRowChirho<'_>,
        scrutinees_chirho: &[CoreIdChirho],
        next_chirho: &CoreExprChirho,
    ) -> CoreExprChirho {
        self.push_scope_chirho();
        let pending_chirho: Vec<(&PatChirho, CoreIdChirho)> = row_chirho
            .pats_chirho
            .iter()
            .copied()
            .zip(scrutinees_chirho.iter().copied())
            .collect();
        let matched_chirho =
            self.match_sequence_chirho(&pending_chirho, next_chirho, &mut |ctx_chirho| {
                ctx_chirho.desugar_rhs_in_where_chirho(
                    row_chirho.rhs_chirho,
                    row_chirho.where_binds_chirho,
                    Some(next_chirho),
                )
            });
        self.pop_scope_chirho();
        matched_chirho
    }
}

/// Put `rest_chirho` where `row_chirho` falls through to `next_chirho`.
fn join_row_chirho(
    row_chirho: CoreExprChirho,
    next_chirho: BinderChirho,
    rest_chirho: CoreExprChirho,
) -> CoreExprChirho {
    match count_usage_chirho(&next_chirho.id_chirho, &row_chirho) {
        // The row never falls through: every later row is unreachable.
        UsageChirho::AbsentChirho => row_chirho,
        UsageChirho::UsedOnceChirho => {
            merge_cases_chirho(row_chirho, next_chirho.id_chirho, rest_chirho).unwrap_or_else(
                |(row_chirho, rest_chirho)| {
                    subst_var_chirho(&row_chirho, next_chirho.id_chirho, &rest_chirho)
                },
            )
        }
        UsageChirho::UsedManyChirho => CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(next_chirho, rest_chirho)],
            body_chirho: Box::new(row_chirho),
        },
    }
}

/// `case v of { alts; _ -> next }` followed by `case v of { alts' }` is one
/// switch: `case v of { alts; alts' not already covered }`. Only taken when the
/// row's single use of `next` IS its default alternative, so no branch of the
/// row can fall through anywhere else, and an alternative the row already
/// covers is unreachable in the rest. Hands both back when the shape differs.
fn merge_cases_chirho(
    row_chirho: CoreExprChirho,
    next_id_chirho: CoreIdChirho,
    rest_chirho: CoreExprChirho,
) -> Result<CoreExprChirho, (CoreExprChirho, CoreExprChirho)> {
    let (
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: row_scrutinee_chirho,
            bind_chirho,
            result_ty_chirho,
            alts_chirho: row_alts_chirho,
        },
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: rest_scrutinee_chirho,
            alts_chirho: rest_alts_chirho,
            ..
        },
    ) = (&row_chirho, &rest_chirho)
    else {
        return Err((row_chirho, rest_chirho));
    };
    let same_variable_chirho = matches!(
        (row_scrutinee_chirho.as_ref(), rest_scrutinee_chirho.as_ref()),
        (CoreExprChirho::VarChirho(a_chirho), CoreExprChirho::VarChirho(b_chirho)) if a_chirho == b_chirho
    );
    let falls_through_by_default_chirho = matches!(
        row_alts_chirho.last(),
        Some(CoreAltChirho {
            con_chirho: AltConChirho::DefaultChirho,
            rhs_chirho: CoreExprChirho::VarChirho(id_chirho),
            ..
        }) if *id_chirho == next_id_chirho
    );
    if !same_variable_chirho || !falls_through_by_default_chirho {
        return Err((row_chirho, rest_chirho));
    }
    let mut alts_chirho: Vec<CoreAltChirho> = row_alts_chirho[..row_alts_chirho.len() - 1].to_vec();
    for alt_chirho in rest_alts_chirho {
        let covered_chirho = alt_chirho.con_chirho != AltConChirho::DefaultChirho
            && alts_chirho
                .iter()
                .any(|existing_chirho| existing_chirho.con_chirho == alt_chirho.con_chirho);
        if !covered_chirho {
            alts_chirho.push(alt_chirho.clone());
        }
    }
    Ok(CoreExprChirho::CaseChirho {
        scrutinee_chirho: row_scrutinee_chirho.clone(),
        bind_chirho: bind_chirho.clone(),
        result_ty_chirho: result_ty_chirho.clone(),
        alts_chirho,
    })
}
