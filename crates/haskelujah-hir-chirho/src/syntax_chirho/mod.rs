// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded syntax whose grouping is independent of the parser's spelling-based fixity table.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
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
    pub(super) fn infix_chirho(
        &mut self,
        left_chirho: &AstExprChirho,
        operator_chirho: &NameChirho,
        right_chirho: &AstExprChirho,
        span_chirho: SpanChirho,
        depth_chirho: usize,
    ) -> ResultChirho<ExprChirho> {
        if [left_chirho, right_chirho].iter().any(|expression_chirho| {
            matches!(
                expression_chirho,
                AstExprChirho::InfixChirho { .. } | AstExprChirho::NegChirho { .. }
            )
        }) {
            return Err(self.error_chirho("operator chain needs resolved-name fixity regrouping; parser grouping is not authoritative", span_chirho));
        }
        let name_chirho = self.lookup_chirho(operator_chirho, NamespaceChirho::ValueChirho)?;
        let left_chirho = self.expr_chirho(left_chirho, depth_chirho + 1)?;
        let right_chirho = self.expr_chirho(right_chirho, depth_chirho + 1)?;
        self.apply_name_chirho(name_chirho, vec![left_chirho, right_chirho], span_chirho)
    }
    pub(super) fn fixities_chirho(
        &mut self,
        declarations_chirho: &[DeclChirho],
    ) -> ResultChirho<std::collections::BTreeMap<OriginalNameChirho, InterfaceFixityChirho>> {
        let mut result_chirho = std::collections::BTreeMap::new();
        for declaration_chirho in declarations_chirho {
            let DeclChirho::FixityDeclChirho {
                fixity_chirho,
                precedence_chirho,
                ops_chirho,
                span_chirho,
            } = declaration_chirho
            else {
                continue;
            };
            let precedence_chirho = precedence_chirho.unwrap_or(9);
            if precedence_chirho > 9 || ops_chirho.is_empty() {
                return Err(self.invalid_class_chirho("invalid fixity declaration", *span_chirho));
            }
            for operator_chirho in ops_chirho {
                let id_chirho =
                    self.bound_name_chirho(operator_chirho, NamespaceChirho::ValueChirho)?;
                let info_chirho = &self.names_chirho[id_chirho.0 as usize];
                if info_chirho.local_chirho
                    || info_chirho.unit_chirho != self.unit_chirho
                    || info_chirho.module_chirho != self.module_chirho
                {
                    return Err(self.invalid_class_chirho(
                        "fixity has no declaration in its own binding group",
                        *span_chirho,
                    ));
                }
                let original_chirho = info_chirho.original_chirho();
                self.charge_names_chirho(
                    original_chirho.module_chirho.len()
                        + original_chirho.occurrence_chirho.len()
                        + 2,
                    *span_chirho,
                )?;
                let associativity_chirho = match fixity_chirho {
                    haskelujah_ast_chirho::decl_chirho::FixityChirho::InfixChirho => {
                        AssociativityChirho::NoneChirho
                    }
                    haskelujah_ast_chirho::decl_chirho::FixityChirho::InfixlChirho => {
                        AssociativityChirho::LeftChirho
                    }
                    haskelujah_ast_chirho::decl_chirho::FixityChirho::InfixrChirho => {
                        AssociativityChirho::RightChirho
                    }
                };
                if result_chirho
                    .insert(
                        original_chirho,
                        InterfaceFixityChirho {
                            associativity_chirho,
                            precedence_chirho,
                        },
                    )
                    .is_some()
                {
                    return Err(
                        self.invalid_class_chirho("repeated fixity declaration", *span_chirho)
                    );
                }
            }
        }
        Ok(result_chirho)
    }
}
