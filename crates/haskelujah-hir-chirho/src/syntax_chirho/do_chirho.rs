// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ordinary do uses GHC's known-key Monad selectors, with lexical statement scope.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_ast_chirho::expr_chirho::StmtChirho;

impl LowerChirho<'_> {
    pub(in crate::lower_chirho) fn do_chirho(
        &mut self,
        statements_chirho: &[StmtChirho],
        span_chirho: SpanChirho,
        depth_chirho: usize,
    ) -> ResultChirho<ExprChirho> {
        self.room_chirho(depth_chirho, span_chirho)?;
        let Some((first_chirho, rest_chirho)) = statements_chirho.split_first() else {
            return Err(self.invalid_class_chirho("do requires a final expression", span_chirho));
        };
        if rest_chirho.is_empty() {
            return match first_chirho {
                StmtChirho::ExprChirho { expr_chirho, .. } => {
                    self.expr_chirho(expr_chirho, depth_chirho + 1)
                }
                _ => Err(self
                    .invalid_class_chirho("last do statement must be an expression", span_chirho)),
            };
        }
        match first_chirho {
            StmtChirho::ExprChirho { expr_chirho, .. } => {
                let first_chirho = self.expr_chirho(expr_chirho, depth_chirho + 1)?;
                let rest_chirho = self.do_chirho(rest_chirho, span_chirho, depth_chirho + 1)?;
                let selector_chirho =
                    self.known_syntax_name_chirho("GHC.Internal.Base", ">>", span_chirho)?;
                self.apply_name_chirho(
                    selector_chirho,
                    vec![first_chirho, rest_chirho],
                    span_chirho,
                )
            }
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                ..
            } => {
                let first_chirho = self.expr_chirho(expr_chirho, depth_chirho + 1)?;
                let mut pattern_chirho = pat_chirho;
                while let PatChirho::ParenChirho { inner_chirho, .. } = pattern_chirho {
                    pattern_chirho = inner_chirho;
                }
                self.env_chirho.push_scope_chirho();
                let result_chirho = (|| {
                    let parameter_chirho = match pattern_chirho {
                        PatChirho::VarChirho(name_chirho) => self.bind_chirho(name_chirho.text_chirho(),NamespaceChirho::ValueChirho,name_chirho.span_chirho(),true,None)?,
                        PatChirho::WildcardChirho(_) => self.bind_chirho(&format!("%next:do-ignored:{}Chirho",self.names_chirho.len()),NamespaceChirho::ValueChirho,span_chirho,true,None)?,
                        _ => return Err(self.error_chirho("next do binds require a variable or wildcard; refutable patterns need MonadFail",pat_chirho.span_chirho())),
                    };
                    let body_chirho = self.do_chirho(rest_chirho, span_chirho, depth_chirho + 1)?;
                    let continuation_chirho = self.node_chirho(
                        ExprKindChirho::LamChirho(vec![parameter_chirho], Box::new(body_chirho)),
                        span_chirho,
                    )?;
                    let selector_chirho =
                        self.known_syntax_name_chirho("GHC.Internal.Base", ">>=", span_chirho)?;
                    self.apply_name_chirho(
                        selector_chirho,
                        vec![first_chirho, continuation_chirho],
                        span_chirho,
                    )
                })();
                self.env_chirho.pop_scope_chirho();
                result_chirho
            }
            StmtChirho::LetChirho { binds_chirho, .. } => {
                self.env_chirho.push_scope_chirho();
                let result_chirho = (|| {
                    let bindings_chirho =
                        self.local_bindings_chirho(binds_chirho, depth_chirho + 1)?;
                    let body_chirho = self.do_chirho(rest_chirho, span_chirho, depth_chirho + 1)?;
                    self.node_chirho(
                        ExprKindChirho::LetChirho(bindings_chirho, Box::new(body_chirho)),
                        span_chirho,
                    )
                })();
                self.env_chirho.pop_scope_chirho();
                result_chirho
            }
        }
    }
}
