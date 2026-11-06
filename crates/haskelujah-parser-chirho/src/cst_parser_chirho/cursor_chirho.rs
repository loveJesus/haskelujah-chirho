// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The token cursor: raw-token mapping into the green tree, the current-token tests and bumps,
//! trivia, recovery scans, declaration and expression boundaries, and the operator and
//! backtick-section predicates.
//! Part of the CST parser (`cst_parser_chirho.rs`), split by syntactic category in row 640.

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::ParserChirho;
use crate::lexer_chirho::{RawTokenChirho, RawTokenKindChirho};

// ---------------------------------------------------------------------------
// Token mapping: RawTokenKindChirho → TokenKindChirho
// ---------------------------------------------------------------------------

fn is_symbol_start_char_chirho(ch_chirho: char) -> bool {
    matches!(
        ch_chirho,
        '!' | '#'
            | '$'
            | '%'
            | '&'
            | '*'
            | '+'
            | '.'
            | '/'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '\\'
            | '^'
            | '|'
            | '-'
            | '~'
            | ':'
    )
}

pub(super) fn qualified_local_text_chirho(text_chirho: &str) -> &str {
    split_qualified_name_chirho(text_chirho).map_or(text_chirho, |(_, local_chirho)| local_chirho)
}

/// Qualification ends before an operator's dots; `Q..` names the operator `.`.
pub(crate) fn split_qualified_name_chirho(text_chirho: &str) -> Option<(&str, &str)> {
    let mut segment_start_chirho = 0;
    let mut split_chirho = None;
    for (index_chirho, character_chirho) in text_chirho.char_indices() {
        if character_chirho != '.' {
            continue;
        }
        let mut segment_chirho = text_chirho[segment_start_chirho..index_chirho].chars();
        if !segment_chirho.next().is_some_and(char::is_uppercase)
            || !segment_chirho.all(|character_chirho| {
                character_chirho.is_alphanumeric() || matches!(character_chirho, '_' | '\'')
            })
        {
            break;
        }
        let local_start_chirho = index_chirho + 1;
        if local_start_chirho < text_chirho.len() {
            split_chirho = Some((
                &text_chirho[..index_chirho],
                &text_chirho[local_start_chirho..],
            ));
        }
        segment_start_chirho = local_start_chirho;
    }
    split_chirho
}

pub(super) fn qualified_name_is_operator_chirho(text_chirho: &str) -> bool {
    qualified_local_text_chirho(text_chirho)
        .chars()
        .next()
        .is_some_and(is_symbol_start_char_chirho)
}

pub(super) fn qualified_name_is_consym_chirho(text_chirho: &str) -> bool {
    qualified_local_text_chirho(text_chirho).starts_with(':')
}

fn map_token_kind_chirho(raw_chirho: RawTokenKindChirho, text_chirho: &str) -> TokenKindChirho {
    match raw_chirho {
        // Keywords
        RawTokenKindChirho::CaseChirho => TokenKindChirho::CaseKeywordChirho,
        RawTokenKindChirho::ClassChirho => TokenKindChirho::ClassKeywordChirho,
        RawTokenKindChirho::DataChirho => TokenKindChirho::DataKeywordChirho,
        RawTokenKindChirho::DefaultChirho => TokenKindChirho::DefaultKeywordChirho,
        RawTokenKindChirho::DerivingChirho => TokenKindChirho::DerivingKeywordChirho,
        RawTokenKindChirho::DoChirho => TokenKindChirho::DoKeywordChirho,
        RawTokenKindChirho::RecChirho => TokenKindChirho::RecKeywordChirho,
        RawTokenKindChirho::ElseChirho => TokenKindChirho::ElseKeywordChirho,
        RawTokenKindChirho::ForeignChirho => TokenKindChirho::ForeignKeywordChirho,
        RawTokenKindChirho::IfChirho => TokenKindChirho::IfKeywordChirho,
        RawTokenKindChirho::ImportChirho => TokenKindChirho::ImportKeywordChirho,
        RawTokenKindChirho::InChirho => TokenKindChirho::InKeywordChirho,
        RawTokenKindChirho::InfixChirho => TokenKindChirho::InfixKeywordChirho,
        RawTokenKindChirho::InfixlChirho => TokenKindChirho::InfixlKeywordChirho,
        RawTokenKindChirho::InfixrChirho => TokenKindChirho::InfixrKeywordChirho,
        RawTokenKindChirho::InstanceChirho => TokenKindChirho::InstanceKeywordChirho,
        RawTokenKindChirho::LetChirho => TokenKindChirho::LetKeywordChirho,
        RawTokenKindChirho::ModuleChirho => TokenKindChirho::ModuleKeywordChirho,
        RawTokenKindChirho::NewtypeChirho => TokenKindChirho::NewtypeKeywordChirho,
        RawTokenKindChirho::OfChirho => TokenKindChirho::OfKeywordChirho,
        RawTokenKindChirho::ThenChirho => TokenKindChirho::ThenKeywordChirho,
        RawTokenKindChirho::TypeChirho => TokenKindChirho::TypeKeywordChirho,
        RawTokenKindChirho::WhereChirho => TokenKindChirho::WhereKeywordChirho,
        RawTokenKindChirho::ForallChirho => TokenKindChirho::ForallKeywordChirho,

        // Identifiers and operators
        RawTokenKindChirho::VarIdChirho
        | RawTokenKindChirho::TransformByChirho
        | RawTokenKindChirho::TransformUsingChirho => TokenKindChirho::VarIdChirho,
        RawTokenKindChirho::ConIdChirho => TokenKindChirho::ConIdChirho,
        RawTokenKindChirho::VarSymChirho => TokenKindChirho::VarSymChirho,
        RawTokenKindChirho::ConSymChirho => TokenKindChirho::ConSymChirho,
        RawTokenKindChirho::QualifiedIdChirho => {
            let local_chirho = qualified_local_text_chirho(text_chirho);
            if local_chirho
                .chars()
                .next()
                .is_some_and(is_symbol_start_char_chirho)
            {
                if local_chirho.starts_with(':') {
                    TokenKindChirho::QualifiedConSymChirho
                } else {
                    TokenKindChirho::QualifiedVarSymChirho
                }
            } else if local_chirho
                .starts_with(|c_chirho: char| c_chirho.is_ascii_lowercase() || c_chirho == '_')
            {
                TokenKindChirho::QualifiedVarIdChirho
            } else {
                TokenKindChirho::QualifiedConIdChirho
            }
        }

        // Literals
        RawTokenKindChirho::IntLitChirho => TokenKindChirho::IntegerLiteralChirho,
        RawTokenKindChirho::FloatLitChirho => TokenKindChirho::FloatLiteralChirho,
        RawTokenKindChirho::CharLitChirho => TokenKindChirho::CharLiteralChirho,
        RawTokenKindChirho::StringLitChirho => TokenKindChirho::StringLiteralChirho,

        // Punctuation
        RawTokenKindChirho::LeftParenChirho => TokenKindChirho::LeftParenChirho,
        RawTokenKindChirho::RightParenChirho => TokenKindChirho::RightParenChirho,
        RawTokenKindChirho::LeftBracketChirho => TokenKindChirho::LeftBracketChirho,
        RawTokenKindChirho::RightBracketChirho => TokenKindChirho::RightBracketChirho,
        RawTokenKindChirho::LeftBraceChirho => TokenKindChirho::LeftBraceChirho,
        RawTokenKindChirho::RightBraceChirho => TokenKindChirho::RightBraceChirho,
        RawTokenKindChirho::CommaChirho => TokenKindChirho::CommaChirho,
        RawTokenKindChirho::SemicolonChirho => TokenKindChirho::SemicolonChirho,
        RawTokenKindChirho::BacktickChirho => TokenKindChirho::BacktickChirho,

        // Special symbols
        RawTokenKindChirho::DotDotChirho => TokenKindChirho::DotDotChirho,
        RawTokenKindChirho::ColonColonChirho => TokenKindChirho::DoubleColonChirho,
        RawTokenKindChirho::EqualsChirho => TokenKindChirho::EqualsChirho,
        RawTokenKindChirho::BackslashChirho => TokenKindChirho::BackslashChirho,
        RawTokenKindChirho::PipeChirho => TokenKindChirho::PipeChirho,
        RawTokenKindChirho::LeftArrowChirho => TokenKindChirho::LeftArrowChirho,
        RawTokenKindChirho::RightArrowChirho => TokenKindChirho::RightArrowChirho,
        RawTokenKindChirho::LinearArrowChirho => TokenKindChirho::LinearArrowChirho,
        RawTokenKindChirho::FatArrowChirho => TokenKindChirho::DoubleArrowChirho,
        RawTokenKindChirho::AtChirho => TokenKindChirho::AtSignChirho,
        RawTokenKindChirho::TildeChirho => TokenKindChirho::TildeChirho,
        RawTokenKindChirho::UnderscoreChirho => TokenKindChirho::UnderscoreReservedIdChirho,
        RawTokenKindChirho::TickChirho => TokenKindChirho::TickChirho,

        // Trivia
        RawTokenKindChirho::WhitespaceChirho => TokenKindChirho::WhitespaceTriviaChirho,
        RawTokenKindChirho::LineCommentChirho => TokenKindChirho::LineCommentTriviaChirho,
        RawTokenKindChirho::BlockCommentChirho => TokenKindChirho::BlockCommentTriviaChirho,
        RawTokenKindChirho::DocCommentChirho => TokenKindChirho::DocCommentTriviaChirho,
        RawTokenKindChirho::PragmaChirho => TokenKindChirho::PragmaChirho,

        // Layout
        RawTokenKindChirho::VirtualLeftBraceChirho => TokenKindChirho::VirtualLeftBraceChirho,
        RawTokenKindChirho::VirtualRightBraceChirho => TokenKindChirho::VirtualRightBraceChirho,
        RawTokenKindChirho::VirtualSemicolonChirho => TokenKindChirho::VirtualSemicolonChirho,

        // Template Haskell
        RawTokenKindChirho::ThSpliceChirho => TokenKindChirho::ThSpliceChirho,
        RawTokenKindChirho::ThTypedSpliceChirho => TokenKindChirho::ThTypedSpliceChirho,
        RawTokenKindChirho::ThOpenExpQuoteChirho => TokenKindChirho::ThOpenExpQuoteChirho,
        RawTokenKindChirho::ThCloseQuoteChirho => TokenKindChirho::ThCloseQuoteChirho,
        RawTokenKindChirho::ThOpenDecQuoteChirho => TokenKindChirho::ThOpenDecQuoteChirho,
        RawTokenKindChirho::ThOpenTypeQuoteChirho => TokenKindChirho::ThOpenTypeQuoteChirho,
        RawTokenKindChirho::ThOpenPatQuoteChirho => TokenKindChirho::ThOpenPatQuoteChirho,
        RawTokenKindChirho::ThOpenExpExplicitQuoteChirho => {
            TokenKindChirho::ThOpenExpExplicitQuoteChirho
        }
        RawTokenKindChirho::ThOpenTypedExpQuoteChirho => TokenKindChirho::ThOpenTypedExpQuoteChirho,
        RawTokenKindChirho::ThCloseTypedQuoteChirho => TokenKindChirho::ThCloseTypedQuoteChirho,

        // Special
        RawTokenKindChirho::EofChirho => TokenKindChirho::VarIdChirho, // placeholder, EOF is handled separately
        RawTokenKindChirho::ErrorChirho => TokenKindChirho::VarIdChirho, // error tokens
    }
}

impl<'src> ParserChirho<'src> {
    pub(super) fn skip_trivia_idx_chirho(&self, mut idx_chirho: usize) -> usize {
        while idx_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[idx_chirho]
                .kind_chirho
                .is_trivia_chirho()
        {
            idx_chirho += 1;
        }
        idx_chirho
    }

    /// Is the current token an infix operator?
    pub(super) fn at_infix_op_chirho(&self) -> bool {
        if self.at_decl_boundary_chirho() || self.at_expr_boundary_chirho() {
            return false;
        }
        matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarSymChirho)
                | Some(RawTokenKindChirho::ConSymChirho)
                | Some(RawTokenKindChirho::BacktickChirho)
        ) || self.current_is_qualified_operator_chirho()
    }

    pub(super) fn current_is_qualified_operator_chirho(&self) -> bool {
        self.current_kind_chirho() == Some(RawTokenKindChirho::QualifiedIdChirho)
            && qualified_name_is_operator_chirho(self.current_text_chirho())
    }

    /// Is the current token at a declaration boundary?
    pub(super) fn at_decl_boundary_chirho(&self) -> bool {
        self.at_eof_chirho()
            || matches!(
                self.current_kind_chirho(),
                Some(RawTokenKindChirho::VirtualSemicolonChirho)
                    | Some(RawTokenKindChirho::VirtualRightBraceChirho)
                    | Some(RawTokenKindChirho::SemicolonChirho)
                    | Some(RawTokenKindChirho::RightBraceChirho)
            )
    }

    /// Is the current token at an expression boundary?
    /// These are keywords/tokens that cannot continue an expression.
    fn at_expr_boundary_chirho(&self) -> bool {
        self.at_decl_boundary_chirho()
            || matches!(
                self.current_kind_chirho(),
                Some(RawTokenKindChirho::WhereChirho)
                    | Some(RawTokenKindChirho::InChirho)
                    | Some(RawTokenKindChirho::ThenChirho)
                    | Some(RawTokenKindChirho::ElseChirho)
                    | Some(RawTokenKindChirho::OfChirho)
                    | Some(RawTokenKindChirho::TransformByChirho)
                    | Some(RawTokenKindChirho::TransformUsingChirho)
            )
    }

    // -----------------------------------------------------------------------
    // Core utility methods
    // -----------------------------------------------------------------------

    pub(super) fn current_chirho(&self) -> Option<&RawTokenChirho> {
        self.tokens_chirho.get(self.pos_chirho)
    }

    pub(super) fn current_kind_chirho(&self) -> Option<RawTokenKindChirho> {
        self.current_chirho().map(|t_chirho| t_chirho.kind_chirho)
    }

    pub(super) fn at_chirho(&self, kind_chirho: RawTokenKindChirho) -> bool {
        self.current_kind_chirho() == Some(kind_chirho)
    }

    pub(super) fn at_eof_chirho(&self) -> bool {
        self.pos_chirho >= self.tokens_chirho.len() || self.at_chirho(RawTokenKindChirho::EofChirho)
    }

    /// Check if current token is `.` (VarSym with text ".").
    pub(super) fn at_dot_chirho(&self) -> bool {
        self.at_chirho(RawTokenKindChirho::VarSymChirho) && self.current_text_chirho() == "."
    }

    /// Check if current token is a VarId with specific text.
    pub(super) fn at_varid_text_chirho(&self, text_chirho: &str) -> bool {
        if !self.at_chirho(RawTokenKindChirho::VarIdChirho) {
            return false;
        }
        self.current_text_chirho() == text_chirho
    }

    /// Check if the current token is a VarSym with the given text.
    pub(super) fn at_varsym_chirho(&self, text_chirho: &str) -> bool {
        if !self.at_chirho(RawTokenKindChirho::VarSymChirho) {
            return false;
        }
        self.current_text_chirho() == text_chirho
    }

    /// Check if the current token is an operator symbol and the next
    /// non-trivia token is `)`, indicating a bare operator section `(op)`.
    pub(super) fn is_operator_section_chirho(&self) -> bool {
        let is_op_chirho = matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarSymChirho) | Some(RawTokenKindChirho::ConSymChirho)
        ) || self.current_is_qualified_operator_chirho();
        if !is_op_chirho {
            return false;
        }
        // Look ahead past the operator and any trivia for ')'
        let mut look_chirho = self.pos_chirho + 1;
        while look_chirho < self.tokens_chirho.len() {
            if self.tokens_chirho[look_chirho]
                .kind_chirho
                .is_trivia_chirho()
            {
                look_chirho += 1;
            } else {
                break;
            }
        }
        look_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[look_chirho].kind_chirho == RawTokenKindChirho::RightParenChirho
    }

    fn peek_backticked_operator_end_chirho(&self, start_pos_chirho: usize) -> Option<usize> {
        if self
            .tokens_chirho
            .get(start_pos_chirho)
            .is_none_or(|token_chirho| {
                token_chirho.kind_chirho != RawTokenKindChirho::BacktickChirho
            })
        {
            return None;
        }

        let mut look_chirho = start_pos_chirho + 1;
        while look_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[look_chirho]
                .kind_chirho
                .is_trivia_chirho()
        {
            look_chirho += 1;
        }
        if look_chirho >= self.tokens_chirho.len()
            || !matches!(
                self.tokens_chirho[look_chirho].kind_chirho,
                RawTokenKindChirho::VarIdChirho
                    | RawTokenKindChirho::ConIdChirho
                    | RawTokenKindChirho::QualifiedIdChirho
                    | RawTokenKindChirho::VarSymChirho
                    | RawTokenKindChirho::ConSymChirho
            )
        {
            return None;
        }

        look_chirho += 1;
        while look_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[look_chirho]
                .kind_chirho
                .is_trivia_chirho()
        {
            look_chirho += 1;
        }

        if look_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[look_chirho].kind_chirho == RawTokenKindChirho::BacktickChirho
        {
            Some(look_chirho)
        } else {
            None
        }
    }

    pub(super) fn is_backticked_left_section_start_chirho(&self) -> bool {
        self.peek_backticked_operator_end_chirho(self.pos_chirho)
            .is_some_and(|end_pos_chirho| {
                let mut look_chirho = end_pos_chirho + 1;
                while look_chirho < self.tokens_chirho.len()
                    && self.tokens_chirho[look_chirho]
                        .kind_chirho
                        .is_trivia_chirho()
                {
                    look_chirho += 1;
                }
                look_chirho < self.tokens_chirho.len()
                    && self.tokens_chirho[look_chirho].kind_chirho
                        != RawTokenKindChirho::RightParenChirho
            })
    }

    pub(super) fn is_backticked_right_section_chirho(&self) -> bool {
        self.peek_backticked_operator_end_chirho(self.pos_chirho)
            .is_some_and(|end_pos_chirho| {
                let mut look_chirho = end_pos_chirho + 1;
                while look_chirho < self.tokens_chirho.len()
                    && self.tokens_chirho[look_chirho]
                        .kind_chirho
                        .is_trivia_chirho()
                {
                    look_chirho += 1;
                }
                look_chirho < self.tokens_chirho.len()
                    && self.tokens_chirho[look_chirho].kind_chirho
                        == RawTokenKindChirho::RightParenChirho
            })
    }

    pub(super) fn bump_backticked_operator_chirho(&mut self) {
        if !self.at_chirho(RawTokenKindChirho::BacktickChirho) {
            return;
        }
        self.bump_chirho();
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::VarIdChirho)
            || self.at_chirho(RawTokenKindChirho::ConIdChirho)
            || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
            || self.at_chirho(RawTokenKindChirho::VarSymChirho)
            || self.at_chirho(RawTokenKindChirho::ConSymChirho)
        {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }
        if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
            self.bump_chirho();
        }
    }

    pub(super) fn current_text_chirho(&self) -> &str {
        if let Some(tok_chirho) = self.current_chirho() {
            self.token_text_chirho(tok_chirho)
        } else {
            ""
        }
    }

    pub(super) fn previous_non_trivia_kind_chirho(&self) -> Option<RawTokenKindChirho> {
        let mut idx_chirho = self.pos_chirho;
        while idx_chirho > 0 {
            idx_chirho -= 1;
            let kind_chirho = self.tokens_chirho[idx_chirho].kind_chirho;
            if !kind_chirho.is_trivia_chirho() {
                return Some(kind_chirho);
            }
        }
        None
    }

    /// Advance one token, adding it to the current green node.
    pub(super) fn bump_chirho(&mut self) {
        if let Some(tok_chirho) = self.current_chirho().copied() {
            let text_chirho = self.token_text_chirho(&tok_chirho);
            let kind_chirho = map_token_kind_chirho(tok_chirho.kind_chirho, text_chirho);
            self.builder_chirho.token_chirho(kind_chirho, text_chirho);
            self.pos_chirho += 1;
        }
    }

    /// Safely extract the source text for a token, handling multi-byte UTF-8.
    pub(super) fn token_text_chirho(&self, tok_chirho: &RawTokenChirho) -> &'src str {
        let start_chirho = tok_chirho.span_chirho.start_chirho().as_usize_chirho();
        let end_chirho = tok_chirho.span_chirho.end_chirho().as_usize_chirho();
        self.source_chirho
            .get(start_chirho..end_chirho)
            .unwrap_or("")
    }

    /// Eat all trivia tokens (whitespace, comments), adding them to the tree.
    pub(super) fn eat_trivia_chirho(&mut self) {
        while let Some(kind_chirho) = self.current_kind_chirho() {
            if kind_chirho.is_trivia_chirho() {
                self.bump_chirho();
            } else {
                break;
            }
        }
    }

    /// Expect a specific token kind, bump it, or create an error node.
    pub(super) fn expect_chirho(&mut self, kind_chirho: RawTokenKindChirho) {
        if self.at_chirho(kind_chirho) {
            self.bump_chirho();
        } else {
            // Error recovery: wrap unexpected token in error node
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
            if !self.at_eof_chirho() {
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Eat tokens until we reach one of the given kinds.
    pub(super) fn eat_until_any_chirho(&mut self, stops_chirho: &[RawTokenKindChirho]) {
        while let Some(kind_chirho) = self.current_kind_chirho() {
            if stops_chirho.contains(&kind_chirho) || kind_chirho == RawTokenKindChirho::EofChirho {
                break;
            }
            self.bump_chirho();
        }
    }

    pub(super) fn eat_until_any_top_level_chirho(&mut self, stops_chirho: &[RawTokenKindChirho]) {
        let mut paren_depth_chirho = 0usize;
        let mut bracket_depth_chirho = 0usize;
        let mut brace_depth_chirho = 0usize;
        while let Some(kind_chirho) = self.current_kind_chirho() {
            if kind_chirho == RawTokenKindChirho::EofChirho {
                break;
            }
            if paren_depth_chirho == 0
                && bracket_depth_chirho == 0
                && brace_depth_chirho == 0
                && stops_chirho.contains(&kind_chirho)
            {
                break;
            }
            match kind_chirho {
                RawTokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                RawTokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                RawTokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBraceChirho
                | RawTokenKindChirho::VirtualLeftBraceChirho => brace_depth_chirho += 1,
                RawTokenKindChirho::RightBraceChirho
                | RawTokenKindChirho::VirtualRightBraceChirho => {
                    brace_depth_chirho = brace_depth_chirho.saturating_sub(1);
                }
                _ => {}
            }
            self.bump_chirho();
        }
    }

    /// Eat tokens until the end of the current declaration.
    pub(super) fn eat_until_decl_end_chirho(&mut self) {
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
            RawTokenKindChirho::SemicolonChirho,
            RawTokenKindChirho::RightBraceChirho,
        ]);
    }
}
