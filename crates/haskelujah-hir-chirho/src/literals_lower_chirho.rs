// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rebindable operators use the lexical name at the occurrence; payloads retain producer origins.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;

impl LowerChirho<'_> {
    pub(super) fn syntax_application_chirho(
        &mut self,
        operation_chirho: &str,
        arguments_chirho: Vec<ExprChirho>,
        span_chirho: SpanChirho,
    ) -> ResultChirho<ExprChirho> {
        let name_chirho = self
            .env_chirho
            .lookup_chirho(operation_chirho, NamespaceChirho::ValueChirho)
            .map(|info_chirho| NameIdChirho(info_chirho.def_id_chirho.0))
            .ok_or_else(|| {
                self.invalid_class_chirho(
                    &format!("rebindable `{operation_chirho}` is not in scope"),
                    span_chirho,
                )
            })?;
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

    pub(super) fn rebindable_integer_chirho(
        &mut self,
        literal_chirho: &LitChirho,
    ) -> ResultChirho<ExprChirho> {
        let span_chirho = literal_chirho.span_chirho();
        let source_chirho = literal_chirho
            .origin_chirho()
            .and_then(|origin_chirho| self.integer_literals_chirho.get(&origin_chirho))
            .ok_or_else(|| {
                self.error_chirho(
                    "next integer literal lacks its producer's exact payload",
                    span_chirho,
                )
            })?;
        if !self.explicit_forall_chirho
            && (source_chirho.contains('_')
                || source_chirho.starts_with("0b")
                || source_chirho.starts_with("0B"))
        {
            return Err(self.error_chirho(
                "next does not yet check this integer literal lexical profile",
                span_chirho,
            ));
        }
        // Include retained token bytes in the HIR's aggregate module allowance.
        self.next_expr_chirho = self
            .next_expr_chirho
            .checked_add(
                u32::try_from(source_chirho.len()).map_err(|_| HirErrorChirho {
                    kind_chirho: HirErrorKindChirho::ResourceChirho,
                    message_chirho: "integer literal byte allowance exhausted".into(),
                    span_chirho,
                })?,
            )
            .ok_or_else(|| HirErrorChirho {
                kind_chirho: HirErrorKindChirho::ResourceChirho,
                message_chirho: "integer literal byte allowance exhausted".into(),
                span_chirho,
            })?;
        self.room_chirho(0, span_chirho)?;
        let payload_chirho = self.node_chirho(
            ExprKindChirho::IntegerChirho(source_chirho.clone()),
            span_chirho,
        )?;
        self.syntax_application_chirho("fromInteger", vec![payload_chirho], span_chirho)
    }
}
