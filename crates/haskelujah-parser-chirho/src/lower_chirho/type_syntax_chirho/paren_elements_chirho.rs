// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Parenthesized tuples whose elements carry retained `:: k` annotations.
//! Part of CST-to-AST lowering (`lower_chirho.rs`).

use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::green_chirho::GreenElementChirho;
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho};

impl LowerCtxChirho {
    /// The elements of a parenthesized tuple in which an element carries `:: k`: a kind,
    /// or an implicit parameter's type, as in `(C t, ?x :: TF t)` (T5120, tc218, T12507).
    /// The group splits at its top-level commas, and each element is the type written
    /// together with its own `::` annotation. `None` unless the group has
    /// both a top-level comma and a top-level `::`.
    pub(super) fn annotated_tuple_elements_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        span_chirho: SpanChirho,
    ) -> Option<Vec<TypeChirho>> {
        let has_comma_chirho = children_chirho
            .iter()
            .any(|child_chirho| is_token_chirho(child_chirho, TokenKindChirho::CommaChirho));
        let has_annotation_chirho = children_chirho
            .iter()
            .any(|child_chirho| is_token_chirho(child_chirho, TokenKindChirho::DoubleColonChirho));
        if !has_comma_chirho || !has_annotation_chirho {
            return None;
        }
        let inner_chirho = if children_chirho.first().is_some_and(|child_chirho| {
            is_token_chirho(child_chirho, TokenKindChirho::LeftParenChirho)
        }) && children_chirho.last().is_some_and(|child_chirho| {
            is_token_chirho(child_chirho, TokenKindChirho::RightParenChirho)
        }) {
            &children_chirho[1..children_chirho.len() - 1]
        } else {
            children_chirho
        };
        let elements_chirho = inner_chirho
            .split(|child_chirho| is_token_chirho(child_chirho, TokenKindChirho::CommaChirho))
            .map(|element_children_chirho| {
                let element_span_chirho = element_children_chirho
                    .first()
                    .zip(element_children_chirho.last())
                    .map(|(first_chirho, last_chirho)| {
                        self.span_chirho(first_chirho.start_chirho, last_chirho.end_chirho)
                    })
                    .unwrap_or(span_chirho);
                self.type_from_flat_children_chirho(
                    &element_children_chirho.iter().collect::<Vec<_>>(),
                    element_span_chirho,
                )
            })
            .collect();
        Some(elements_chirho)
    }
}

fn is_token_chirho(child_chirho: &ChildChirho<'_>, kind_chirho: TokenKindChirho) -> bool {
    matches!(child_chirho.element_chirho, GreenElementChirho::TokenChirho(token_chirho)
        if token_chirho.kind_chirho() == kind_chirho)
}
