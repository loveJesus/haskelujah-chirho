// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Parenthesized tuples whose elements carry `:: k`.
//! Part of CST-to-AST lowering (`lower_chirho.rs`).

use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::green_chirho::GreenElementChirho;
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho, is_type_kind_chirho};

impl LowerCtxChirho {
    /// The elements of a parenthesized tuple in which an element carries `:: k`: a kind,
    /// or an implicit parameter's type, as in `(C t, ?x :: TF t)` (T5120, tc218, T12507).
    /// The group splits at its top-level commas, and each element is the type written
    /// before its own `::`. The annotation is consumed and dropped, as a single `(a :: k)`
    /// drops it; no annotation gains a representation here. `None` unless the group has
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
        let elements_chirho = children_chirho
            .split(|child_chirho| is_token_chirho(child_chirho, TokenKindChirho::CommaChirho))
            .map(|element_children_chirho| {
                let written_chirho = element_children_chirho
                    .split(|child_chirho| {
                        is_token_chirho(child_chirho, TokenKindChirho::DoubleColonChirho)
                    })
                    .next()
                    .unwrap_or_default();
                let nodes_chirho: Vec<&ChildChirho<'_>> = written_chirho
                    .iter()
                    .filter(|child_chirho| is_type_node_chirho(child_chirho))
                    .collect();
                match nodes_chirho.split_first() {
                    None => self.placeholder_type_chirho(),
                    Some((head_chirho, arguments_chirho)) => arguments_chirho.iter().fold(
                        self.lower_type_from_child_chirho(head_chirho),
                        |function_chirho, argument_chirho| TypeChirho::AppChirho {
                            fun_chirho: Box::new(function_chirho),
                            arg_chirho: Box::new(
                                self.lower_type_from_child_chirho(argument_chirho),
                            ),
                            span_chirho,
                        },
                    ),
                }
            })
            .collect();
        Some(elements_chirho)
    }
}

fn is_token_chirho(child_chirho: &ChildChirho<'_>, kind_chirho: TokenKindChirho) -> bool {
    matches!(child_chirho.element_chirho, GreenElementChirho::TokenChirho(token_chirho)
        if token_chirho.kind_chirho() == kind_chirho)
}

fn is_type_node_chirho(child_chirho: &ChildChirho<'_>) -> bool {
    matches!(child_chirho.element_chirho, GreenElementChirho::NodeChirho(node_chirho)
        if is_type_kind_chirho(node_chirho.kind_chirho()))
}
