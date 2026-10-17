// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Record-pattern AST lowering, kept separate from the main lowering dispatch.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::*;
use haskelujah_ast_chirho::pat_chirho::PatFieldChirho;

impl LowerCtxChirho {
    pub(super) fn lower_record_pat_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> PatChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        // Con { f1 = p1, f2 = p2 }
        // CST children are all flat tokens: ConId { VarId = VarId , VarId = VarId }
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut con_name_chirho = None;
        let mut fields_chirho = Vec::new();
        let mut current_field_name_chirho: Option<NameChirho> = None;
        let mut pending_field_con_name_chirho: Option<NameChirho> = None;
        let mut pending_field_con_args_chirho: Vec<PatChirho> = Vec::new();
        let mut saw_equals_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let kind_chirho = tok_chirho.kind_chirho();
                    if con_name_chirho.is_none()
                        && (kind_chirho == TokenKindChirho::ConIdChirho
                            || kind_chirho == TokenKindChirho::QualifiedConIdChirho)
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        con_name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    } else if kind_chirho == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    } else if saw_equals_chirho
                        && current_field_name_chirho.is_some()
                        && (kind_chirho == TokenKindChirho::ConIdChirho
                            || kind_chirho == TokenKindChirho::QualifiedConIdChirho)
                        && pending_field_con_name_chirho.is_none()
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        pending_field_con_name_chirho =
                            Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    } else if kind_chirho == TokenKindChirho::VarIdChirho {
                        if saw_equals_chirho && current_field_name_chirho.is_some() {
                            let s_chirho = self
                                .span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                            let var_name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                            if pending_field_con_name_chirho.is_some() {
                                pending_field_con_args_chirho
                                    .push(PatChirho::VarChirho(var_name_chirho));
                            } else {
                                let fname_chirho = current_field_name_chirho.take().unwrap();
                                fields_chirho.push(PatFieldChirho {
                                    name_chirho: fname_chirho,
                                    pattern_chirho: PatChirho::VarChirho(var_name_chirho),
                                    span_chirho: s_chirho,
                                });
                                saw_equals_chirho = false;
                            }
                        } else {
                            // VarId before '=' — this is the field name
                            // (flush any pending punned field first)
                            if let Some(fname_chirho) = current_field_name_chirho.take() {
                                let pattern_chirho = if let Some(con_name_chirho) =
                                    pending_field_con_name_chirho.take()
                                {
                                    PatChirho::ConChirho {
                                        con_chirho: con_name_chirho,
                                        args_chirho: std::mem::take(
                                            &mut pending_field_con_args_chirho,
                                        ),
                                        span_chirho,
                                    }
                                } else {
                                    PatChirho::VarChirho(fname_chirho.clone())
                                };
                                fields_chirho.push(PatFieldChirho {
                                    pattern_chirho,
                                    name_chirho: fname_chirho,
                                    span_chirho,
                                });
                            }
                            let s_chirho = self
                                .span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                            current_field_name_chirho =
                                Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                            saw_equals_chirho = false;
                        }
                    } else if kind_chirho == TokenKindChirho::CommaChirho {
                        if let Some(fname_chirho) = current_field_name_chirho.take() {
                            let pattern_chirho = if let Some(con_name_chirho) =
                                pending_field_con_name_chirho.take()
                            {
                                PatChirho::ConChirho {
                                    con_chirho: con_name_chirho,
                                    args_chirho: std::mem::take(&mut pending_field_con_args_chirho),
                                    span_chirho,
                                }
                            } else {
                                PatChirho::VarChirho(fname_chirho.clone())
                            };
                            fields_chirho.push(PatFieldChirho {
                                pattern_chirho,
                                name_chirho: fname_chirho,
                                span_chirho,
                            });
                        }
                        saw_equals_chirho = false;
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho)
                    if is_pat_kind_chirho(n_chirho.kind_chirho()) =>
                {
                    // Pattern node after '=' (e.g. constructor pattern, literal, etc.)
                    let pat_chirho = self.lower_pat_chirho(n_chirho, child_chirho.start_chirho);
                    if let Some(fname_chirho) = current_field_name_chirho.take() {
                        let field_span_chirho = self.span_chirho(
                            child_chirho.start_chirho,
                            child_chirho.start_chirho + n_chirho.text_len_chirho(),
                        );
                        pending_field_con_name_chirho = None;
                        pending_field_con_args_chirho.clear();
                        fields_chirho.push(PatFieldChirho {
                            name_chirho: fname_chirho,
                            pattern_chirho: pat_chirho,
                            span_chirho: field_span_chirho,
                        });
                    }
                    saw_equals_chirho = false;
                }
                _ => {}
            }
        }
        // Handle trailing punned field
        if let Some(fname_chirho) = current_field_name_chirho.take() {
            let pattern_chirho = if let Some(con_name_chirho) = pending_field_con_name_chirho.take()
            {
                PatChirho::ConChirho {
                    con_chirho: con_name_chirho,
                    args_chirho: std::mem::take(&mut pending_field_con_args_chirho),
                    span_chirho,
                }
            } else {
                PatChirho::VarChirho(fname_chirho.clone())
            };
            fields_chirho.push(PatFieldChirho {
                pattern_chirho,
                name_chirho: fname_chirho,
                span_chirho,
            });
        }

        // Detect `..` wildcard token (RecordWildCards)
        let mut has_wildcard_chirho = false;
        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                if tok_chirho.kind_chirho() == TokenKindChirho::DotDotChirho {
                    has_wildcard_chirho = true;
                    break;
                }
            }
        }

        PatChirho::RecordChirho {
            con_chirho: con_name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            fields_chirho,
            has_wildcard_chirho,
            span_chirho,
        }
    }
}
