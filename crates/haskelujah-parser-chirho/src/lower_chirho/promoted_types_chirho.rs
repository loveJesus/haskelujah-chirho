// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Structured and flat promoted parentheses retain symbols or tuple applications.
//! Workflow: language-features-chirho/flat-type-syntax-chirho.
use super::*;

impl LowerCtxChirho {
    pub(super) fn lower_promoted_type_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> TypeChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        if let Some(name_chirho) = children_chirho.iter().find_map(|child_chirho| {
            let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho else {
                return None;
            };
            matches!(
                token_chirho.kind_chirho(),
                TokenKindChirho::ConIdChirho
                    | TokenKindChirho::QualifiedConIdChirho
                    | TokenKindChirho::ConSymChirho
                    | TokenKindChirho::QualifiedConSymChirho
            )
            .then(|| {
                self.name_from_token_chirho(
                    token_chirho,
                    self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                )
            })
        }) {
            return TypeChirho::PromotedConChirho {
                name_chirho,
                span_chirho,
            };
        }
        let start_chirho = children_chirho.iter().position(|child_chirho| matches!(child_chirho.element_chirho,
            GreenElementChirho::TokenChirho(token_chirho) if token_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho));
        let end_chirho = children_chirho.iter().rposition(|child_chirho| matches!(child_chirho.element_chirho,
            GreenElementChirho::TokenChirho(token_chirho) if token_chirho.kind_chirho() == TokenKindChirho::RightParenChirho));
        let (Some(start_chirho), Some(end_chirho)) = (start_chirho, end_chirho) else {
            return self.placeholder_type_chirho();
        };
        let Some(operands_chirho) = children_chirho.get(start_chirho + 1..end_chirho) else {
            return self.placeholder_type_chirho();
        };
        self.promoted_parenthesized_from_children_chirho(
            &operands_chirho.iter().collect::<Vec<_>>(),
            span_chirho,
        )
    }

    pub(super) fn promoted_parenthesized_from_children_chirho(
        &self,
        children_chirho: &[&ChildChirho],
        span_chirho: SpanChirho,
    ) -> TypeChirho {
        if let [child_chirho] = children_chirho
            && let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho
            && matches!(
                token_chirho.kind_chirho(),
                TokenKindChirho::ConSymChirho | TokenKindChirho::QualifiedConSymChirho
            )
        {
            return TypeChirho::PromotedConChirho {
                name_chirho: self.name_from_token_chirho(
                    token_chirho,
                    self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                ),
                span_chirho,
            };
        }
        let commas_chirho = super::flat_types_chirho::top_level_commas_chirho(children_chirho);
        let constructor_only_chirho = children_chirho.len() == commas_chirho.len();
        let arity_chirho = if children_chirho.is_empty() {
            0
        } else {
            commas_chirho.len() + 1
        };
        if arity_chirho == 1 {
            return self.placeholder_type_chirho();
        }
        let name_chirho = format!("({})", ",".repeat(arity_chirho.saturating_sub(1)));
        let mut result_chirho = TypeChirho::PromotedConChirho {
            name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                &name_chirho,
                span_chirho,
            )),
            span_chirho,
        };
        if !constructor_only_chirho {
            let mut start_chirho = 0;
            for end_chirho in commas_chirho
                .into_iter()
                .chain(std::iter::once(children_chirho.len()))
            {
                if start_chirho == end_chirho {
                    return self.placeholder_type_chirho();
                }
                let operand_chirho = &children_chirho[start_chirho..end_chirho];
                let operand_span_chirho = self.span_chirho(
                    operand_chirho.first().unwrap().start_chirho,
                    operand_chirho.last().unwrap().end_chirho,
                );
                let argument_chirho =
                    self.type_from_flat_children_chirho(operand_chirho, operand_span_chirho);
                result_chirho = TypeChirho::AppChirho {
                    fun_chirho: Box::new(result_chirho),
                    arg_chirho: Box::new(argument_chirho),
                    span_chirho,
                };
                start_chirho = end_chirho + 1;
            }
        }
        result_chirho
    }
}
