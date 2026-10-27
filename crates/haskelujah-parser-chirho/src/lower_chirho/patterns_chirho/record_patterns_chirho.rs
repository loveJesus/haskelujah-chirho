// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Record-pattern AST lowering. Assigned fields are ordinary pattern nodes;
//! the remaining tokens name the constructor, field puns and RecordWildCards.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::*;
use haskelujah_ast_chirho::pat_chirho::PatFieldChirho;

impl LowerCtxChirho {
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn lower_record_pat_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> PatChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut con_name_chirho = None;
        let mut fields_chirho = Vec::new();
        let mut current_field_name_chirho = None;
        let mut has_wildcard_chirho = false;
        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(token_chirho) => {
                    let token_span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    match token_chirho.kind_chirho() {
                        TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho
                            if con_name_chirho.is_none() =>
                        {
                            con_name_chirho =
                                Some(self.name_from_token_chirho(token_chirho, token_span_chirho));
                        }
                        TokenKindChirho::VarIdChirho | TokenKindChirho::QualifiedVarIdChirho => {
                            flush_record_pun_chirho(
                                &mut fields_chirho,
                                &mut current_field_name_chirho,
                                span_chirho,
                            );
                            current_field_name_chirho =
                                Some(self.name_from_token_chirho(token_chirho, token_span_chirho));
                        }
                        TokenKindChirho::CommaChirho => {
                            flush_record_pun_chirho(
                                &mut fields_chirho,
                                &mut current_field_name_chirho,
                                span_chirho,
                            );
                        }
                        TokenKindChirho::DotDotChirho => has_wildcard_chirho = true,
                        _ => {}
                    }
                }
                GreenElementChirho::NodeChirho(pattern_node_chirho)
                    if is_pat_kind_chirho(pattern_node_chirho.kind_chirho()) =>
                {
                    if let Some(name_chirho) = current_field_name_chirho.take() {
                        fields_chirho.push(PatFieldChirho {
                            name_chirho,
                            pattern_chirho: self
                                .lower_pat_chirho(pattern_node_chirho, child_chirho.start_chirho),
                            span_chirho: self
                                .span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                        });
                    }
                }
                _ => {}
            }
        }
        flush_record_pun_chirho(
            &mut fields_chirho,
            &mut current_field_name_chirho,
            span_chirho,
        );
        PatChirho::RecordChirho {
            con_chirho: con_name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            fields_chirho,
            has_wildcard_chirho,
            span_chirho,
        }
    }
}

/// workflow: language-features-chirho/pattern-matching-chirho
fn flush_record_pun_chirho(
    fields_chirho: &mut Vec<PatFieldChirho>,
    current_name_chirho: &mut Option<NameChirho>,
    span_chirho: SpanChirho,
) {
    if let Some(name_chirho) = current_name_chirho.take() {
        fields_chirho.push(PatFieldChirho {
            pattern_chirho: PatChirho::VarChirho(name_chirho.clone()),
            name_chirho,
            span_chirho,
        });
    }
}

#[cfg(test)]
#[path = "record_pattern_tests_chirho.rs"]
mod record_pattern_tests_chirho;
