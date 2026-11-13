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
        let name_chirho = if self.rebindable_chirho {
            self.lookup_text_chirho(operation_chirho, NamespaceChirho::ValueChirho, span_chirho)?
        } else {
            let original_chirho = OriginalNameChirho {
                unit_chirho: UnitIdentityChirho::CompilerChirho,
                module_chirho: "GHC.Internal.Num".into(),
                namespace_chirho: NamespaceChirho::ValueChirho,
                occurrence_chirho: operation_chirho.into(),
            };
            if self.unit_chirho == original_chirho.unit_chirho
                && self.module_chirho == original_chirho.module_chirho
            {
                let own_chirho = self
                    .env_chirho
                    .lookup_chirho(operation_chirho, NamespaceChirho::ValueChirho)
                    .map(|info_chirho| NameIdChirho(info_chirho.def_id_chirho.0))
                    .filter(|id_chirho| {
                        self.names_chirho[id_chirho.0 as usize].original_chirho() == original_chirho
                    });
                own_chirho.ok_or_else(|| {
                    self.invalid_class_chirho(
                        "defining numeric source lacks its known selector",
                        span_chirho,
                    )
                })?
            } else {
                self.imported_names_chirho
                    .get(&original_chirho)
                    .copied()
                    .ok_or_else(|| {
                        self.error_chirho(
                            "known numeric selector has no checked defining-unit interface",
                            span_chirho,
                        )
                    })?
            }
        };
        self.apply_name_chirho(name_chirho, arguments_chirho, span_chirho)
    }

    pub(super) fn integer_expression_chirho(
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
