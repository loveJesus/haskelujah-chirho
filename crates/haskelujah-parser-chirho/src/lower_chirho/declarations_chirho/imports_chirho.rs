// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lossless import modifiers through lowering.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::*;

impl LowerCtxChirho {
    pub(super) fn lower_import_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ImportDeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let source_chirho = node_chirho
            .children_chirho()
            .iter()
            .filter_map(|element_chirho| {
                if let GreenElementChirho::TokenChirho(token_chirho) = element_chirho {
                    Some(token_chirho)
                } else {
                    None
                }
            })
            .take_while(|token_chirho| {
                !matches!(
                    token_chirho.kind_chirho(),
                    TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho
                )
            })
            .any(|token_chirho| {
                token_chirho.kind_chirho() == TokenKindChirho::PragmaChirho
                    && token_chirho
                        .text_chirho()
                        .strip_prefix("{-#")
                        .and_then(|text_chirho| text_chirho.strip_suffix("#-}"))
                        .is_some_and(|text_chirho| text_chirho.trim() == "SOURCE")
            });
        let mut qualified_chirho = false;
        let mut module_name_chirho = None;
        let mut alias_chirho = None;
        let mut saw_as_chirho = false;

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::VarIdChirho if tok_chirho.text_chirho() == "qualified" => {
                        qualified_chirho = true;
                    }
                    TokenKindChirho::VarIdChirho if tok_chirho.text_chirho() == "as" => {
                        saw_as_chirho = true;
                    }
                    TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                        let span_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        let name_chirho = if saw_as_chirho {
                            self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho)
                        } else {
                            self.module_name_from_text_chirho(tok_chirho.text_chirho(), span_chirho)
                        };
                        if saw_as_chirho {
                            alias_chirho = Some(name_chirho);
                            saw_as_chirho = false;
                        } else if module_name_chirho.is_none() {
                            module_name_chirho = Some(name_chirho);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Lower import specification if present
        let mut spec_chirho = None;
        let mut hiding_chirho = false;
        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho)
                    if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                        && tok_chirho.text_chirho() == "hiding" =>
                {
                    hiding_chirho = true;
                }
                GreenElementChirho::NodeChirho(n_chirho)
                    if n_chirho.kind_chirho() == SyntaxKindChirho::ImportSpecListChirho =>
                {
                    let items_chirho =
                        self.lower_import_spec_list_chirho(n_chirho, child_chirho.start_chirho);
                    spec_chirho = Some(ImportSpecChirho {
                        hiding_chirho,
                        items_chirho,
                    });
                }
                _ => {}
            }
        }

        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        ImportDeclChirho {
            module_chirho: module_name_chirho.unwrap_or_else(|| {
                NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                    "",
                    SpanChirho::DUMMY_CHIRHO,
                ))
            }),
            source_chirho,
            qualified_chirho,
            alias_chirho,
            spec_chirho,
            span_chirho,
        }
    }

    fn lower_import_spec_list_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<ImportItemChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut items_chirho = Vec::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::ImportSpecChirho {
                    if let Some(item_chirho) =
                        self.lower_import_spec_item_chirho(n_chirho, child_chirho.start_chirho)
                    {
                        items_chirho.push(item_chirho);
                    }
                }
            }
        }

        items_chirho
    }

    fn lower_import_spec_item_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<ImportItemChirho> {
        let mut first_name_chirho: Option<(String, SpanChirho, bool)> = None; // (text, span, is_con)
        let mut has_parens_chirho = false;
        let mut has_dotdot_chirho = false;
        let mut members_chirho: Vec<NameChirho> = Vec::new();
        let mut in_parens_chirho = false;

        let mut offset_chirho = base_chirho;
        for elem_chirho in node_chirho.children_chirho() {
            let elem_start_chirho = offset_chirho;
            let elem_end_chirho = offset_chirho + elem_chirho.text_len_chirho();
            match elem_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => match tok_chirho.kind_chirho() {
                    TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                        let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                        let is_con_chirho = true;
                        if first_name_chirho.is_none() {
                            first_name_chirho = Some((
                                tok_chirho.text_chirho().to_string(),
                                span_chirho,
                                is_con_chirho,
                            ));
                        } else if in_parens_chirho {
                            members_chirho.push(
                                self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho),
                            );
                        }
                    }
                    TokenKindChirho::VarIdChirho => {
                        let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                        if first_name_chirho.is_none() {
                            first_name_chirho =
                                Some((tok_chirho.text_chirho().to_string(), span_chirho, false));
                        } else if in_parens_chirho {
                            members_chirho.push(
                                self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho),
                            );
                        }
                    }
                    TokenKindChirho::VarSymChirho
                    | TokenKindChirho::ConSymChirho
                    | TokenKindChirho::QualifiedVarSymChirho
                    | TokenKindChirho::QualifiedConSymChirho => {
                        let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                        if first_name_chirho.is_none() {
                            first_name_chirho = Some((
                                tok_chirho.text_chirho().to_string(),
                                span_chirho,
                                matches!(
                                    tok_chirho.kind_chirho(),
                                    TokenKindChirho::ConSymChirho
                                        | TokenKindChirho::QualifiedConSymChirho
                                ),
                            ));
                        } else if in_parens_chirho {
                            members_chirho.push(
                                self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho),
                            );
                        }
                    }
                    TokenKindChirho::LeftParenChirho => {
                        has_parens_chirho = true;
                        in_parens_chirho = true;
                    }
                    TokenKindChirho::RightParenChirho => {
                        in_parens_chirho = false;
                    }
                    TokenKindChirho::DotDotChirho => {
                        has_dotdot_chirho = true;
                    }
                    _ => {}
                },
                _ => {}
            }
            offset_chirho = elem_end_chirho;
        }

        let (text_chirho, span_chirho, is_con_chirho) = first_name_chirho?;
        let name_chirho = self.name_from_text_chirho(&text_chirho, span_chirho);

        if is_con_chirho && has_parens_chirho {
            // Type/class with members: T(..) or T(Con1, Con2)
            let members_spec_chirho = if has_dotdot_chirho {
                ExportMembersChirho::AllChirho
            } else if members_chirho.is_empty() {
                ExportMembersChirho::NoneChirho
            } else {
                ExportMembersChirho::SomeChirho(members_chirho)
            };
            Some(ImportItemChirho::TyConChirho {
                name_chirho,
                members_chirho: members_spec_chirho,
            })
        } else if is_con_chirho {
            // Type/class with no parens — just the name, no members
            Some(ImportItemChirho::TyConChirho {
                name_chirho,
                members_chirho: ExportMembersChirho::NoneChirho,
            })
        } else {
            // Variable
            Some(ImportItemChirho::VarChirho(name_chirho))
        }
    }
}
