// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A case expression is rows: each alternative is one row of one pattern,
//! tried top to bottom, falling through on a failed pattern or a failed last
//! guard. Workflow: language-features-chirho/pattern-matching-chirho, and
//! testing-chirho/execution-oracles-chirho for the shared failure continuations.

use super::match_rows_chirho::{MatchRowChirho, RowBodyChirho};
use super::{CoreExprChirho, CoreLitChirho, DesugarCtxChirho};
use haskelujah_ast_chirho::expr_chirho::AltChirho;

impl DesugarCtxChirho {
    /// `case scrutinee of alts`, with the scrutinee already desugared.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn desugar_case_expr_with_scrutinee_chirho(
        &mut self,
        scrut_chirho: CoreExprChirho,
        alts_chirho: &[AltChirho],
    ) -> CoreExprChirho {
        if alts_chirho.is_empty() {
            return self.case_failure_chirho("Non-exhaustive case alternatives");
        }
        // Every row refers to the scrutinee by name. A plain variable already
        // is one; anything else is bound once, lazily - which is exactly what
        // the STG lowering does for a non-variable scrutinee anyway, and which
        // keeps `case undefined of _ -> e` from forcing, as in Haskell. A method
        // occurrence is bound too, so the dictionary pass still rewrites it once.
        let (scrutinee_id_chirho, binding_chirho) = match scrut_chirho {
            CoreExprChirho::VarChirho(id_chirho)
                if !self.method_occurrences_chirho.contains_key(&id_chirho) =>
            {
                (id_chirho, None)
            }
            other_chirho => {
                let binder_chirho = self.row_binder_chirho("$scrut");
                (binder_chirho.id_chirho, Some((binder_chirho, other_chirho)))
            }
        };
        let rows_chirho: Vec<MatchRowChirho<'_>> = alts_chirho
            .iter()
            .map(|alt_chirho| MatchRowChirho {
                pats_chirho: vec![&alt_chirho.pat_chirho],
                body_chirho: RowBodyChirho::Rhs(&alt_chirho.rhs_chirho),
                where_binds_chirho: &alt_chirho.where_binds_chirho,
            })
            .collect();
        let failure_chirho = self.case_failure_chirho("Non-exhaustive patterns in case");
        let matched_chirho =
            self.compile_rows_chirho(&[scrutinee_id_chirho], &rows_chirho, failure_chirho);
        match binding_chirho {
            Some(binding_chirho) => CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![binding_chirho],
                body_chirho: Box::new(matched_chirho),
            },
            None => matched_chirho,
        }
    }

    /// `error "<reason>"`, for a match no row accepts.
    pub(super) fn case_failure_chirho(&mut self, reason_chirho: &str) -> CoreExprChirho {
        let error_id_chirho = self.fresh_id_chirho("error");
        CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(error_id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                reason_chirho.to_string(),
            ))),
        }
    }
}
