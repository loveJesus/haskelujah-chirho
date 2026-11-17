// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded syntax whose grouping is independent of the parser's spelling-based fixity table.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
mod do_chirho;
mod pragmas_chirho;

impl LowerChirho<'_> {
    pub(super) fn apply_name_chirho(
        &mut self,
        name_chirho: NameIdChirho,
        arguments_chirho: Vec<ExprChirho>,
        span_chirho: SpanChirho,
    ) -> ResultChirho<ExprChirho> {
        let mut expression_chirho =
            self.node_chirho(ExprKindChirho::VarChirho(name_chirho), span_chirho)?;
        for argument_chirho in arguments_chirho {
            expression_chirho = self.node_chirho(
                ExprKindChirho::AppChirho(Box::new(expression_chirho), Box::new(argument_chirho)),
                span_chirho,
            )?;
        }
        Ok(expression_chirho)
    }
}
