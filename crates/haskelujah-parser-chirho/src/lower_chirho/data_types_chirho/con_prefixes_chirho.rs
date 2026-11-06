// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Retain one actual H98 prefix before selecting positional, infix or record fields.
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use haskelujah_ast_chirho::decl_chirho::{ConPrefixChirho, TyVarChirho};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenTokenChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho};

impl LowerCtxChirho {
    pub(super) fn con_prefix_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        span_chirho: SpanChirho,
    ) -> (ConPrefixChirho, usize) {
        let mut prefix_chirho = ConPrefixChirho::default();
        let mut cursor_chirho = 0;
        if token_at_chirho(children_chirho, 0).is_some_and(|token_chirho| {
            token_chirho.kind_chirho() == TokenKindChirho::ForallKeywordChirho
        }) {
            prefix_chirho.forall_written_chirho = true;
            cursor_chirho = 1;
            while let Some(child_chirho) = children_chirho.get(cursor_chirho) {
                match child_chirho.element_chirho {
                    GreenElementChirho::TokenChirho(token_chirho)
                        if token_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                            && token_chirho.text_chirho() == "." =>
                    {
                        cursor_chirho += 1;
                        break;
                    }
                    GreenElementChirho::TokenChirho(token_chirho)
                        if token_chirho.kind_chirho() == TokenKindChirho::VarIdChirho =>
                    {
                        prefix_chirho
                            .existential_vars_chirho
                            .push(TyVarChirho::plain_chirho(self.name_from_token_chirho(
                                token_chirho,
                                self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                ),
                            )));
                    }
                    GreenElementChirho::NodeChirho(node_chirho)
                        if node_chirho.kind_chirho() == SyntaxKindChirho::ParenTypeChirho =>
                    {
                        match self.try_lower_kind_annotated_forall_binder_chirho(
                            node_chirho,
                            child_chirho.start_chirho,
                        ) {
                            Some(binder_chirho) => {
                                prefix_chirho.existential_vars_chirho.push(binder_chirho)
                            }
                            None => prefix_chirho.unread_chirho = true,
                        }
                    }
                    _ => {
                        prefix_chirho.unread_chirho = true;
                        return (prefix_chirho, cursor_chirho);
                    }
                }
                cursor_chirho += 1;
            }
            if cursor_chirho == children_chirho.len()
                && !token_at_chirho(children_chirho, cursor_chirho.saturating_sub(1))
                    .is_some_and(|token_chirho| token_chirho.text_chirho() == ".")
            {
                prefix_chirho.unread_chirho = true;
            }
        }
        let mut depth_chirho = 0usize;
        for index_chirho in cursor_chirho..children_chirho.len() {
            let Some(token_chirho) = token_at_chirho(children_chirho, index_chirho) else {
                continue;
            };
            match token_chirho.kind_chirho() {
                TokenKindChirho::LeftParenChirho
                | TokenKindChirho::LeftBracketChirho
                | TokenKindChirho::LeftBraceChirho => depth_chirho += 1,
                TokenKindChirho::RightParenChirho
                | TokenKindChirho::RightBracketChirho
                | TokenKindChirho::RightBraceChirho => {
                    let Some(previous_chirho) = depth_chirho.checked_sub(1) else {
                        prefix_chirho.unread_chirho = true;
                        return (prefix_chirho, cursor_chirho);
                    };
                    depth_chirho = previous_chirho;
                }
                TokenKindChirho::DoubleArrowChirho if depth_chirho == 0 => {
                    let context_chirho: Vec<_> = children_chirho[cursor_chirho..index_chirho]
                        .iter()
                        .collect();
                    if context_chirho.is_empty() {
                        prefix_chirho.unread_chirho = true;
                    } else {
                        let context_ty_chirho =
                            self.type_from_flat_children_chirho(&context_chirho, span_chirho);
                        prefix_chirho.context_chirho =
                            Self::type_to_constraints_chirho(&context_ty_chirho, span_chirho);
                    }
                    return (prefix_chirho, index_chirho + 1);
                }
                _ => {}
            }
        }
        (prefix_chirho, cursor_chirho)
    }
}

fn token_at_chirho<'child_chirho>(
    children_chirho: &[ChildChirho<'child_chirho>],
    index_chirho: usize,
) -> Option<&'child_chirho GreenTokenChirho> {
    match children_chirho.get(index_chirho)?.element_chirho {
        GreenElementChirho::TokenChirho(token_chirho) => Some(token_chirho),
        GreenElementChirho::NodeChirho(_) => None,
    }
}
