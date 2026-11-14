// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Boolean guards with no later equation to fall through to.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_ast_chirho::expr_chirho::StmtChirho;

impl LowerChirho<'_> {
    pub(super) fn single_equation_rhs_chirho(
        &mut self,
        rhs_chirho: &RhsChirho,
        depth_chirho: usize,
    ) -> ResultChirho<ExprChirho> {
        let RhsChirho::GuardedChirho(rows_chirho) = rhs_chirho else {
            return self.rhs_chirho(rhs_chirho, depth_chirho);
        };
        let span_chirho = rows_chirho
            .first()
            .map_or(SpanChirho::DUMMY_CHIRHO, |row_chirho| {
                row_chirho.span_chirho
            });
        self.room_chirho(depth_chirho, span_chirho)?;
        if rows_chirho.is_empty()
            || rows_chirho.len() > self.limits_chirho.depth_chirho.saturating_sub(depth_chirho)
        {
            return Err(self.error_chirho("empty or too deep guarded right-hand side", span_chirho));
        }
        // One chain, with no copied continuation. The empty match is the real
        // runtime pattern failure, not a semantic refusal or a successful placeholder.
        let mut body_chirho = self.node_chirho(
            ExprKindChirho::MatchChirho(Vec::new(), Vec::new()),
            span_chirho,
        )?;
        for row_chirho in rows_chirho.iter().rev() {
            let [
                StmtChirho::ExprChirho {
                    expr_chirho,
                    then_chirho: None,
                },
            ] = row_chirho.quals_chirho.as_slice()
            else {
                return Err(self.error_chirho(
                    "next supports one Boolean guard per row of a single equation",
                    row_chirho.span_chirho,
                ));
            };
            let condition_chirho = self.expr_chirho(expr_chirho, depth_chirho + 1)?;
            let yes_chirho = self.expr_chirho(&row_chirho.body_chirho, depth_chirho + 1)?;
            body_chirho = self.node_chirho(
                ExprKindChirho::IfChirho(
                    Box::new(condition_chirho),
                    Box::new(yes_chirho),
                    Box::new(body_chirho),
                ),
                row_chirho.span_chirho,
            )?;
        }
        Ok(body_chirho)
    }
}
