// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class/instance contexts use the same type syntax as qualified signatures.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{
    ChildChirho, ConstraintChirho, GreenElementChirho, GreenTokenChirho, LowerCtxChirho, SpanChirho,
};

impl LowerCtxChirho {
    pub(super) fn build_instance_context_chirho(
        &self,
        tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
    ) -> Vec<ConstraintChirho> {
        let Some((_, first_span_chirho)) = tokens_chirho.first() else {
            return Vec::new();
        };
        let span_chirho = first_span_chirho
            .merge_chirho(tokens_chirho.last().expect("nonempty context").1)
            .unwrap_or(*first_span_chirho);
        // Token clones share their Arc<str>; no source reparse or fabricated
        // offsets. Parentheses, applications and quantified constraints retain
        // the same representation on each entry path.
        let elements_chirho: Vec<_> = tokens_chirho
            .iter()
            .map(|(token_chirho, _)| GreenElementChirho::TokenChirho((*token_chirho).clone()))
            .collect();
        let children_chirho: Vec<_> = elements_chirho
            .iter()
            .zip(tokens_chirho)
            .map(|(element_chirho, (_, token_span_chirho))| ChildChirho {
                element_chirho,
                start_chirho: token_span_chirho.start_chirho().as_usize_chirho(),
                end_chirho: token_span_chirho.end_chirho().as_usize_chirho(),
            })
            .collect();
        let refs_chirho: Vec<_> = children_chirho.iter().collect();
        let context_chirho = self.type_from_flat_children_chirho(&refs_chirho, span_chirho);
        Self::type_to_constraints_chirho(&context_chirho, span_chirho)
    }
}
