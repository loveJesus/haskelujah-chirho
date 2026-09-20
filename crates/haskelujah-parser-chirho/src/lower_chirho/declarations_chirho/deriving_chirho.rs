// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Deriving retains the same type syntax as other declaration applications.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{GreenElementChirho, GreenNodeChirho, LowerCtxChirho, TokenKindChirho, TypeChirho};

impl LowerCtxChirho {
    pub(super) fn lower_deriving_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<TypeChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut tokens_chirho = Vec::new();
        for child_chirho in &children_chirho {
            let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho else {
                continue;
            };
            // Via has a separate represented payload and must not generate a
            // second ordinary instance. Its existing lowering is independent.
            if token_chirho.text_chirho() == "via" {
                return Vec::new();
            }
            if tokens_chirho.is_empty()
                && matches!(
                    token_chirho.text_chirho(),
                    "deriving" | "stock" | "newtype" | "anyclass"
                )
            {
                continue;
            }
            tokens_chirho.push((
                token_chirho,
                self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
            ));
        }
        let Some((_, first_chirho)) = tokens_chirho.first() else {
            return Vec::new();
        };
        let span_chirho = first_chirho
            .merge_chirho(tokens_chirho.last().expect("nonempty deriving").1)
            .unwrap_or(*first_chirho);
        let application_chirho =
            self.lower_type_from_token_slice_chirho(&tokens_chirho, span_chirho);
        match application_chirho {
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => elements_chirho,
            TypeChirho::ParenChirho { inner_chirho, .. } => vec![*inner_chirho],
            TypeChirho::ConChirho(ref name_chirho)
                if name_chirho.text_chirho() == "()"
                    && tokens_chirho.first().is_some_and(|(token_chirho, _)| {
                        token_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho
                    }) =>
            {
                Vec::new()
            }
            _ => vec![application_chirho],
        }
    }
}
