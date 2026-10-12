// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Guarded right-hand sides: arms tried top to bottom, each a list of
//! qualifiers that must all succeed (Haskell 2010 §3.13, GHC's guarded
//! right-hand side).
//!
//! A boolean qualifier goes on when it is `True`; a pattern guard `p <- e`
//! matches `p` against `e` with the single-row matcher, its variables in scope
//! for the qualifiers after it and the body; a `let` binds. When any qualifier
//! fails, the NEXT arm is tried, and after the last arm the row's own
//! fall-through: the next equation or alternative, or, with no row to fall to,
//! the guard failure.
//!
//! The arms are compiled in source order, each against a placeholder for the
//! next, and joined bottom-up by the join the rows use: an arm that cannot fail
//! (`otherwise`) drops the arms after it, a continuation used once is inlined,
//! and one used more than once is let-bound, never copied.
//! workflow: language-features-chirho/pattern-matching-chirho

use haskelujah_ast_chirho::expr_chirho::{ExprChirho, GuardedExprChirho, StmtChirho};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::DesugarCtxChirho;
use super::match_rows_chirho::join_alternatives_chirho;
use crate::expr_chirho::{AltConChirho, BinderChirho, CoreAltChirho, CoreExprChirho};

impl DesugarCtxChirho {
    /// The arms of a guarded right-hand side, in order. When every arm fails,
    /// `fallthrough_chirho` (the next row), or the guard failure without one.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn desugar_guards_chirho(
        &mut self,
        guards_chirho: &[GuardedExprChirho],
        fallthrough_chirho: Option<&CoreExprChirho>,
    ) -> CoreExprChirho {
        let compiled_chirho: Vec<(BinderChirho, CoreExprChirho)> = guards_chirho
            .iter()
            .map(|arm_chirho| {
                let next_chirho = self.row_binder_chirho("$nextguard");
                let next_ref_chirho = CoreExprChirho::VarChirho(next_chirho.id_chirho);
                // What an arm's qualifiers bind is visible to the rest of that
                // arm only.
                self.push_scope_chirho();
                let arm_core_chirho = self.desugar_quals_chirho(
                    &arm_chirho.quals_chirho,
                    &arm_chirho.body_chirho,
                    &next_ref_chirho,
                );
                self.pop_scope_chirho();
                (next_chirho, arm_core_chirho)
            })
            .collect();
        let failure_chirho = match fallthrough_chirho {
            Some(fallthrough_chirho) => fallthrough_chirho.clone(),
            None => self.case_failure_chirho("Non-exhaustive guards"),
        };
        join_alternatives_chirho(compiled_chirho, failure_chirho)
    }

    /// One arm from `quals_chirho` on: each qualifier in turn, then the body.
    /// Any qualifier that fails continues with `next_chirho`.
    fn desugar_quals_chirho(
        &mut self,
        quals_chirho: &[StmtChirho],
        body_chirho: &ExprChirho,
        next_chirho: &CoreExprChirho,
    ) -> CoreExprChirho {
        let Some((qual_chirho, rest_chirho)) = quals_chirho.split_first() else {
            return self.desugar_expr_chirho(body_chirho);
        };
        match qual_chirho {
            // `otherwise` is Prelude's `True`: it cannot fail.
            StmtChirho::ExprChirho { expr_chirho, .. } if is_otherwise_chirho(expr_chirho) => {
                self.desugar_quals_chirho(rest_chirho, body_chirho, next_chirho)
            }
            StmtChirho::ExprChirho { expr_chirho, .. } => {
                let cond_chirho = self.desugar_expr_chirho(expr_chirho);
                let holds_chirho = self.desugar_quals_chirho(rest_chirho, body_chirho, next_chirho);
                let wild_chirho = self.fresh_binder_chirho(
                    "wild",
                    TyChirho::bool_chirho(),
                    SpanChirho::DUMMY_CHIRHO,
                );
                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(cond_chirho),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: self.row_ty_chirho(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: holds_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: next_chirho.clone(),
                        },
                    ],
                }
            }
            // A pattern guard is a one-pattern row against its source.
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                ..
            } => {
                let source_chirho = self.desugar_expr_chirho(expr_chirho);
                let (source_id_chirho, binding_chirho) =
                    self.scrutinee_variable_chirho(source_chirho, "$guard");
                let matched_chirho = self.match_row_chirho(
                    pat_chirho,
                    source_id_chirho,
                    next_chirho,
                    &mut |ctx_chirho| {
                        ctx_chirho.desugar_quals_chirho(rest_chirho, body_chirho, next_chirho)
                    },
                );
                Self::with_scrutinee_chirho(binding_chirho, matched_chirho)
            }
            StmtChirho::LetChirho { binds_chirho, .. } => {
                self.desugar_let_around_chirho(binds_chirho, &mut |ctx_chirho| {
                    ctx_chirho.desugar_quals_chirho(rest_chirho, body_chirho, next_chirho)
                })
            }
        }
    }
}

fn is_otherwise_chirho(expr_chirho: &ExprChirho) -> bool {
    matches!(expr_chirho, ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "otherwise")
}
