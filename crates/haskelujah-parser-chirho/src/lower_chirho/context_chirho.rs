// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The lowering context: spans, semantic children, pragmas, names, placeholders and the
//! syntax-kind predicates.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use std::cell::RefCell;
use std::collections::HashMap;

use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::module_chirho::InlinePragmaChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::provenance_chirho::OriginSupplyChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho, SpanChirho};
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{
    GreenElementChirho, GreenNodeChirho, GreenTokenChirho,
};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho};
use crate::pragma_chirho::pragma_extensions_from_text_chirho;

impl LowerCtxChirho {
    pub(super) fn new_chirho(file_id_chirho: FileIdChirho) -> Self {
        Self {
            file_id_chirho,
            offset_chirho: 0,
            fixity_overrides_chirho: HashMap::new(),
            origin_supply_chirho: RefCell::new(OriginSupplyChirho::new_chirho()),
        }
    }

    pub(super) fn span_chirho(&self, start_chirho: usize, end_chirho: usize) -> SpanChirho {
        SpanChirho::new_chirho(
            self.file_id_chirho,
            ByteOffsetChirho::from_usize_chirho(start_chirho),
            ByteOffsetChirho::from_usize_chirho(end_chirho),
        )
    }

    // -----------------------------------------------------------------------
    // Green tree traversal helpers
    // -----------------------------------------------------------------------

    /// Collect non-trivia children from a green node, recording each
    /// child's byte offset range.
    pub(super) fn semantic_children_chirho<'a>(
        &self,
        node_chirho: &'a GreenNodeChirho,
        base_offset_chirho: usize,
    ) -> Vec<ChildChirho<'a>> {
        let mut result_chirho = Vec::new();
        let mut off_chirho = base_offset_chirho;
        for child_chirho in node_chirho.children_chirho() {
            let start_chirho = off_chirho;
            let len_chirho = child_chirho.text_len_chirho();
            off_chirho += len_chirho;
            if is_trivia_element_chirho(child_chirho) {
                continue;
            }
            result_chirho.push(ChildChirho {
                element_chirho: child_chirho,
                start_chirho,
                end_chirho: start_chirho + len_chirho,
            });
        }
        result_chirho
    }

    /// Check if a CST node contains a specific keyword/identifier text
    /// among its first few non-trivia tokens (after the leading keyword).
    pub(super) fn node_has_keyword_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        keyword_chirho: &str,
    ) -> bool {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        // Check the first few non-trivia children for the keyword
        for child_chirho in children_chirho.iter().take(5) {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho
                && tok_chirho.text_chirho() == keyword_chirho
            {
                return true;
            }
        }
        false
    }

    // -----------------------------------------------------------------------
    // Pragma extraction
    // -----------------------------------------------------------------------

    /// Walk all pragma tokens in the CST and extract LANGUAGE extension names.
    /// Pragma tokens have the text `{-# LANGUAGE Ext1, Ext2 #-}`.
    pub(super) fn extract_pragma_extensions_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
    ) -> Vec<String> {
        let mut extensions_chirho = Vec::new();
        for child_chirho in node_chirho.children_chirho() {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho
                && tok_chirho.kind_chirho() == TokenKindChirho::PragmaChirho
            {
                extensions_chirho
                    .extend(pragma_extensions_from_text_chirho(tok_chirho.text_chirho()));
            }
        }
        extensions_chirho
    }

    /// Walk all pragma tokens and extract INLINE/NOINLINE/INLINABLE annotations.
    /// Returns a map from binding name to the inline pragma.
    pub(super) fn extract_inline_pragmas_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
    ) -> HashMap<String, InlinePragmaChirho> {
        let mut pragmas_chirho = HashMap::new();
        for child_chirho in node_chirho.children_chirho() {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho
                && tok_chirho.kind_chirho() == TokenKindChirho::PragmaChirho
            {
                let text_chirho = tok_chirho.text_chirho();
                let inner_chirho = text_chirho
                    .strip_prefix("{-#")
                    .and_then(|s_chirho| s_chirho.strip_suffix("#-}"))
                    .unwrap_or("")
                    .trim();
                // Check for NOINLINE first (longer match)
                if let Some(rest_chirho) = inner_chirho.strip_prefix("NOINLINE") {
                    let name_chirho = rest_chirho.trim();
                    if !name_chirho.is_empty() {
                        pragmas_chirho
                            .insert(name_chirho.to_string(), InlinePragmaChirho::NoInlineChirho);
                    }
                } else if let Some(rest_chirho) = inner_chirho.strip_prefix("INLINABLE") {
                    let name_chirho = rest_chirho.trim();
                    if !name_chirho.is_empty() {
                        pragmas_chirho
                            .insert(name_chirho.to_string(), InlinePragmaChirho::InlinableChirho);
                    }
                } else if let Some(rest_chirho) = inner_chirho.strip_prefix("INLINE") {
                    let name_chirho = rest_chirho.trim();
                    if !name_chirho.is_empty() {
                        pragmas_chirho
                            .insert(name_chirho.to_string(), InlinePragmaChirho::InlineChirho);
                    }
                }
            }
        }
        pragmas_chirho
    }

    /// Walk all pragma tokens and extract SPECIALIZE/SPECIALISE annotations.
    /// Returns a map from binding name to the list of specialization type strings.
    pub(super) fn extract_specialize_pragmas_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
    ) -> HashMap<String, Vec<String>> {
        let mut pragmas_chirho: HashMap<String, Vec<String>> = HashMap::new();
        for child_chirho in node_chirho.children_chirho() {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho
                && tok_chirho.kind_chirho() == TokenKindChirho::PragmaChirho
            {
                let text_chirho = tok_chirho.text_chirho();
                let inner_chirho = text_chirho
                    .strip_prefix("{-#")
                    .and_then(|s_chirho| s_chirho.strip_suffix("#-}"))
                    .unwrap_or("")
                    .trim();
                // Accept both SPECIALIZE and SPECIALISE (British spelling)
                let rest_chirho = inner_chirho
                    .strip_prefix("SPECIALIZE")
                    .or_else(|| inner_chirho.strip_prefix("SPECIALISE"));
                if let Some(rest_chirho) = rest_chirho {
                    let rest_chirho = rest_chirho.trim();
                    // Format: "functionName :: Type"
                    if let Some(colons_pos_chirho) = rest_chirho.find("::") {
                        let name_chirho = rest_chirho[..colons_pos_chirho].trim();
                        let ty_text_chirho = rest_chirho[colons_pos_chirho + 2..].trim();
                        if !name_chirho.is_empty() && !ty_text_chirho.is_empty() {
                            pragmas_chirho
                                .entry(name_chirho.to_string())
                                .or_default()
                                .push(ty_text_chirho.to_string());
                        }
                    }
                }
            }
        }
        pragmas_chirho
    }

    // -----------------------------------------------------------------------
    // Name helpers
    // -----------------------------------------------------------------------

    pub(super) fn name_from_text_chirho(
        &self,
        text_chirho: &str,
        span_chirho: SpanChirho,
    ) -> NameChirho {
        // Split qualified names like "Data.List.sort" or "Data.List.+", but
        // do NOT treat bare operator names like ":.:" as qualified just
        // because they contain a dot character.
        if let Some((qualifier_chirho, local_chirho)) =
            crate::cst_parser_chirho::split_qualified_name_chirho(text_chirho)
        {
            return NameChirho::RawChirho(RawNameChirho::qualified_chirho(
                qualifier_chirho,
                local_chirho,
                span_chirho,
            ));
        }
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(text_chirho, span_chirho))
    }

    pub(super) fn module_name_from_text_chirho(
        &self,
        text_chirho: &str,
        span_chirho: SpanChirho,
    ) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(text_chirho, span_chirho))
    }

    pub(super) fn name_from_token_chirho(
        &self,
        tok_chirho: &GreenTokenChirho,
        span_chirho: SpanChirho,
    ) -> NameChirho {
        self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho)
    }

    pub(super) fn extract_name_from_node_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> NameChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::VarIdChirho
                    | TokenKindChirho::ConIdChirho
                    | TokenKindChirho::QualifiedConIdChirho
                    | TokenKindChirho::QualifiedVarIdChirho
                    | TokenKindChirho::VarSymChirho
                    | TokenKindChirho::ConSymChirho
                    | TokenKindChirho::QualifiedVarSymChirho
                    | TokenKindChirho::QualifiedConSymChirho
                    | TokenKindChirho::UnderscoreReservedIdChirho => {
                        let span_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        return self.name_from_token_chirho(tok_chirho, span_chirho);
                    }
                    _ => {}
                }
            }
        }
        self.dummy_name_chirho()
    }

    /// A data-head binder name: a type variable, or the wildcard `_`.
    /// GHC binds `data Foo (_ :: Constraint)` as an anonymous argument, so `_` is a
    /// binder like any other — treating it as "not a binder" is what let its kind
    /// annotation be read as the declaration's own return kind.
    pub(super) fn is_head_binder_name_chirho(kind_chirho: TokenKindChirho) -> bool {
        matches!(
            kind_chirho,
            TokenKindChirho::VarIdChirho | TokenKindChirho::UnderscoreReservedIdChirho
        )
    }

    pub(super) fn dummy_name_chirho(&self) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "",
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    pub(super) fn placeholder_type_chirho(&self) -> TypeChirho {
        TypeChirho::VarChirho(self.dummy_name_chirho())
    }

    pub(super) fn placeholder_expr_chirho(&self) -> ExprChirho {
        self.reference_expr_chirho(self.dummy_name_chirho())
    }

    pub(super) fn find_top_level_token_index_in_token_slice_chirho(
        &self,
        tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
        token_kind_chirho: TokenKindChirho,
    ) -> Option<usize> {
        let mut paren_depth_chirho = 0usize;
        let mut bracket_depth_chirho = 0usize;

        for (idx_chirho, (tok_chirho, _)) in tokens_chirho.iter().enumerate() {
            match tok_chirho.kind_chirho() {
                TokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                TokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                TokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                TokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                kind_chirho
                    if kind_chirho == token_kind_chirho
                        && paren_depth_chirho == 0
                        && bracket_depth_chirho == 0 =>
                {
                    return Some(idx_chirho);
                }
                _ => {}
            }
        }

        None
    }

    pub(super) fn find_matching_token_index_chirho(
        tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
        start_idx_chirho: usize,
        open_kind_chirho: TokenKindChirho,
        close_kind_chirho: TokenKindChirho,
    ) -> Option<usize> {
        let mut depth_chirho = 0usize;
        for (idx_chirho, (tok_chirho, _)) in tokens_chirho.iter().enumerate().skip(start_idx_chirho)
        {
            let kind_chirho = tok_chirho.kind_chirho();
            if kind_chirho == open_kind_chirho {
                depth_chirho += 1;
            } else if kind_chirho == close_kind_chirho {
                depth_chirho = depth_chirho.saturating_sub(1);
                if depth_chirho == 0 {
                    return Some(idx_chirho);
                }
            }
        }
        None
    }
}

pub(super) fn is_trivia_element_chirho(element_chirho: &GreenElementChirho) -> bool {
    match element_chirho {
        GreenElementChirho::TokenChirho(tok_chirho) => {
            is_trivia_token_kind_chirho(tok_chirho.kind_chirho())
        }
        GreenElementChirho::NodeChirho(_) => false,
    }
}

pub(super) fn is_expr_kind_chirho(kind_chirho: SyntaxKindChirho) -> bool {
    matches!(
        kind_chirho,
        SyntaxKindChirho::AppExprChirho
            | SyntaxKindChirho::TypeAppExprChirho
            | SyntaxKindChirho::InfixExprChirho
            | SyntaxKindChirho::LambdaExprChirho
            | SyntaxKindChirho::LambdaCaseExprChirho
            | SyntaxKindChirho::LetExprChirho
            | SyntaxKindChirho::IfExprChirho
            | SyntaxKindChirho::MultiWayIfExprChirho
            | SyntaxKindChirho::CaseExprChirho
            | SyntaxKindChirho::DoExprChirho
            | SyntaxKindChirho::ParenExprChirho
            | SyntaxKindChirho::LeftSectionExprChirho
            | SyntaxKindChirho::RightSectionExprChirho
            | SyntaxKindChirho::TupleExprChirho
            | SyntaxKindChirho::ListExprChirho
            | SyntaxKindChirho::ArithSeqExprChirho
            | SyntaxKindChirho::ListCompExprChirho
            | SyntaxKindChirho::SectionExprChirho
            | SyntaxKindChirho::TypeAnnotExprChirho
            | SyntaxKindChirho::NegateExprChirho
            | SyntaxKindChirho::RecordConExprChirho
            | SyntaxKindChirho::RecordUpdateExprChirho
            | SyntaxKindChirho::LiteralExprChirho
            | SyntaxKindChirho::NameExprChirho
            | SyntaxKindChirho::QuotedNameExprChirho
            | SyntaxKindChirho::SpliceExprChirho
            | SyntaxKindChirho::TypedSpliceExprChirho
            | SyntaxKindChirho::QuoteExprChirho
            | SyntaxKindChirho::QuoteDeclChirho
            | SyntaxKindChirho::QuoteTypeChirho
            | SyntaxKindChirho::QuotePatChirho
            | SyntaxKindChirho::TypedQuoteExprChirho
    )
}

pub(super) fn is_pat_kind_chirho(kind_chirho: SyntaxKindChirho) -> bool {
    matches!(
        kind_chirho,
        SyntaxKindChirho::ConPatChirho
            | SyntaxKindChirho::VarPatChirho
            | SyntaxKindChirho::LitPatChirho
            | SyntaxKindChirho::WildcardPatChirho
            | SyntaxKindChirho::AsPatChirho
            | SyntaxKindChirho::ParenPatChirho
            | SyntaxKindChirho::TuplePatChirho
            | SyntaxKindChirho::ListPatChirho
            | SyntaxKindChirho::NegPatChirho
            | SyntaxKindChirho::LazyPatChirho
            | SyntaxKindChirho::BangPatChirho
            | SyntaxKindChirho::RecordPatChirho
            | SyntaxKindChirho::InfixConPatChirho
            | SyntaxKindChirho::ViewPatChirho
            | SyntaxKindChirho::SigPatChirho
    )
}

/// The tokens a declaration's meaning ignores. Pragmas are trivia here as in
/// the lexer, so `instance {-# OVERLAPPING #-} Eq (T Int)` keeps `Eq` as its
/// first head token, the class, instead of losing the class to the pragma. The
/// LANGUAGE, INLINE and SPECIALIZE readers walk a node's raw children, so they
/// still see their pragmas.
/// workflow: language-features-chirho/flat-type-syntax-chirho
fn is_trivia_token_kind_chirho(kind_chirho: TokenKindChirho) -> bool {
    matches!(
        kind_chirho,
        TokenKindChirho::WhitespaceTriviaChirho
            | TokenKindChirho::LineCommentTriviaChirho
            | TokenKindChirho::BlockCommentTriviaChirho
            | TokenKindChirho::DocCommentTriviaChirho
            | TokenKindChirho::PragmaChirho
            | TokenKindChirho::VirtualLeftBraceChirho
            | TokenKindChirho::VirtualRightBraceChirho
            | TokenKindChirho::VirtualSemicolonChirho
    )
}
