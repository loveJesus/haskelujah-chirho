// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Retained class metadata, not text annotations interpreted by the boot checker.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::*;
use crate::lexer_chirho::{LexerChirho, RawTokenKindChirho};
use haskelujah_ast_chirho::class_chirho::MinimalFormulaChirho;

impl LowerCtxChirho {
    /// The top-level family AST shares type/data syntax. Capture the actual
    /// keyword here before class members become their own typed contracts.
    pub(super) fn associated_data_spans_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> HashSet<SpanChirho> {
        let mut spans_chirho = HashSet::new();
        for child_chirho in self.semantic_children_chirho(node_chirho, base_chirho) {
            let GreenElementChirho::NodeChirho(where_chirho) = child_chirho.element_chirho else {
                continue;
            };
            if where_chirho.kind_chirho() != SyntaxKindChirho::WhereClauseChirho {
                continue;
            }
            for member_chirho in
                self.semantic_children_chirho(where_chirho, child_chirho.start_chirho)
            {
                let GreenElementChirho::NodeChirho(declaration_chirho) =
                    member_chirho.element_chirho
                else {
                    continue;
                };
                if declaration_chirho.kind_chirho() != SyntaxKindChirho::TypeFamilyDeclChirho {
                    continue;
                }
                if self.semantic_children_chirho(declaration_chirho, member_chirho.start_chirho)
                    .first().is_some_and(|first_chirho| matches!(first_chirho.element_chirho,
                        GreenElementChirho::TokenChirho(token_chirho) if token_chirho.kind_chirho() == TokenKindChirho::DataKeywordChirho)) {
                    spans_chirho.insert(self.span_chirho(member_chirho.start_chirho, member_chirho.end_chirho));
                }
            }
        }
        spans_chirho
    }

    pub(super) fn try_extract_default_sig_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<(String, TypeChirho)> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let tokens_chirho: Vec<_> = children_chirho
            .iter()
            .filter_map(|child_chirho| {
                if let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho {
                    Some((
                        token_chirho,
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                    ))
                } else {
                    None
                }
            })
            .collect();
        let colon_chirho = tokens_chirho.iter().position(|(token_chirho, _)| {
            token_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho
        })?;
        let name_chirho = tokens_chirho[..colon_chirho]
            .iter()
            .find(|(token_chirho, _)| {
                matches!(
                    token_chirho.kind_chirho(),
                    TokenKindChirho::VarIdChirho | TokenKindChirho::VarSymChirho
                )
            })?
            .0
            .text_chirho()
            .to_owned();
        let tail_chirho = &tokens_chirho[colon_chirho + 1..];
        let first_chirho = tail_chirho.first()?.1;
        let span_chirho = first_chirho
            .merge_chirho(tail_chirho.last()?.1)
            .unwrap_or(first_chirho);
        Some((
            name_chirho,
            self.lower_type_from_token_slice_chirho(tail_chirho, span_chirho),
        ))
    }

    pub(super) fn lower_minimal_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        _base_chirho: usize,
    ) -> Option<MinimalFormulaChirho> {
        let mut result_chirho = None;
        let mut pending_chirho = vec![node_chirho];
        while let Some(current_chirho) = pending_chirho.pop() {
            for child_chirho in current_chirho.children_chirho() {
                match child_chirho {
                    GreenElementChirho::NodeChirho(node_chirho) => pending_chirho.push(node_chirho),
                    GreenElementChirho::TokenChirho(token_chirho)
                        if token_chirho.kind_chirho() == TokenKindChirho::PragmaChirho =>
                    {
                        let Some(body_chirho) = token_chirho
                            .text_chirho()
                            .strip_prefix("{-#")
                            .and_then(|text_chirho| text_chirho.strip_suffix("#-}"))
                        else {
                            continue;
                        };
                        let Some(formula_chirho) = body_chirho.trim().strip_prefix("MINIMAL")
                        else {
                            continue;
                        };
                        if formula_chirho
                            .chars()
                            .next()
                            .is_some_and(|char_chirho| !char_chirho.is_whitespace())
                        {
                            continue;
                        }
                        if result_chirho.is_some() {
                            return Some(MinimalFormulaChirho::InvalidChirho);
                        }
                        result_chirho = Some(parse_minimal_chirho(formula_chirho));
                    }
                    _ => {}
                }
            }
        }
        result_chirho
    }
}

fn parse_minimal_chirho(source_chirho: &str) -> MinimalFormulaChirho {
    use MinimalFormulaChirho::{AllChirho, InvalidChirho};
    if source_chirho.len() > 16_384 {
        return InvalidChirho;
    }
    let tokens_chirho: Vec<_> =
        LexerChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
            .lex_all_chirho()
            .into_iter()
            .filter(|token_chirho| {
                !token_chirho.kind_chirho.is_trivia_chirho()
                    && token_chirho.kind_chirho != RawTokenKindChirho::EofChirho
            })
            .map(|token_chirho| {
                (
                    token_chirho.kind_chirho,
                    &source_chirho[token_chirho.span_chirho.start_chirho().as_usize_chirho()
                        ..token_chirho.span_chirho.end_chirho().as_usize_chirho()],
                )
            })
            .collect();
    if tokens_chirho.is_empty() {
        return AllChirho(Vec::new());
    }
    let mut parser_chirho = MinimalParserChirho {
        tokens_chirho: &tokens_chirho,
        index_chirho: 0,
        budget_chirho: 4096,
    };
    match parser_chirho.disjunction_chirho(0) {
        Some(formula_chirho) if parser_chirho.index_chirho == tokens_chirho.len() => formula_chirho,
        _ => InvalidChirho,
    }
}

struct MinimalParserChirho<'source_chirho> {
    tokens_chirho: &'source_chirho [(RawTokenKindChirho, &'source_chirho str)],
    index_chirho: usize,
    budget_chirho: usize,
}

impl MinimalParserChirho<'_> {
    fn eat_chirho(&mut self, kind_chirho: RawTokenKindChirho) -> bool {
        if self
            .tokens_chirho
            .get(self.index_chirho)
            .is_some_and(|token_chirho| token_chirho.0 == kind_chirho)
        {
            self.index_chirho += 1;
            true
        } else {
            false
        }
    }

    fn disjunction_chirho(&mut self, depth_chirho: usize) -> Option<MinimalFormulaChirho> {
        let mut alternatives_chirho = vec![self.conjunction_chirho(depth_chirho)?];
        while self.eat_chirho(RawTokenKindChirho::PipeChirho) {
            alternatives_chirho.push(self.conjunction_chirho(depth_chirho)?);
        }
        Some(if alternatives_chirho.len() == 1 {
            alternatives_chirho.pop()?
        } else {
            MinimalFormulaChirho::AnyChirho(alternatives_chirho)
        })
    }

    fn conjunction_chirho(&mut self, depth_chirho: usize) -> Option<MinimalFormulaChirho> {
        let mut requirements_chirho = vec![self.atom_chirho(depth_chirho)?];
        while self.eat_chirho(RawTokenKindChirho::CommaChirho) {
            requirements_chirho.push(self.atom_chirho(depth_chirho)?);
        }
        Some(if requirements_chirho.len() == 1 {
            requirements_chirho.pop()?
        } else {
            MinimalFormulaChirho::AllChirho(requirements_chirho)
        })
    }

    fn atom_chirho(&mut self, depth_chirho: usize) -> Option<MinimalFormulaChirho> {
        self.budget_chirho = self.budget_chirho.checked_sub(1)?;
        if depth_chirho > 128 {
            return None;
        }
        let token_chirho = *self.tokens_chirho.get(self.index_chirho)?;
        if token_chirho.0 == RawTokenKindChirho::VarIdChirho {
            self.index_chirho += 1;
            return Some(MinimalFormulaChirho::MethodChirho(
                token_chirho.1.to_owned(),
            ));
        }
        if !self.eat_chirho(RawTokenKindChirho::LeftParenChirho) {
            return None;
        }
        if self.eat_chirho(RawTokenKindChirho::RightParenChirho) {
            return Some(MinimalFormulaChirho::AllChirho(Vec::new()));
        }
        if let Some(&(
            RawTokenKindChirho::VarSymChirho | RawTokenKindChirho::PipeChirho,
            name_chirho,
        )) = self.tokens_chirho.get(self.index_chirho)
        {
            self.index_chirho += 1;
            return self
                .eat_chirho(RawTokenKindChirho::RightParenChirho)
                .then(|| MinimalFormulaChirho::MethodChirho(name_chirho.to_owned()));
        }
        let nested_chirho = self.disjunction_chirho(depth_chirho + 1)?;
        self.eat_chirho(RawTokenKindChirho::RightParenChirho)
            .then_some(nested_chirho)
    }
}
