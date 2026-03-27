// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # CST Parser — recursive-descent parser producing a lossless green tree
//!
//! Consumes a layout-processed token stream and builds a
//! `GreenNodeChirho` tree using the `GreenBuilderChirho`.
//!
//! ## Design
//!
//! - All trivia (whitespace, comments) is attached to the *next* non-trivia
//!   token as leading trivia. This ensures round-tripping.
//! - Error recovery wraps unexpected tokens in `ErrorNodeChirho` and continues.
//! - The parser is organized by syntactic category: module, imports, decls,
//!   types, expressions, patterns.
//! - Uses the checkpoint mechanism for retroactive node wrapping (needed for
//!   infix expressions, function types, type annotations).

use std::sync::Arc;

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenBuilderChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use crate::layout_chirho::apply_layout_chirho;
use crate::lexer_chirho::{LexerChirho, RawTokenChirho, RawTokenKindChirho};
use haskelujah_span_chirho::FileIdChirho;

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

fn qualified_local_text_chirho(text_chirho: &str) -> &str {
    text_chirho.rsplit('.').next().unwrap_or(text_chirho)
}

fn qualified_name_is_operator_chirho(text_chirho: &str) -> bool {
    qualified_local_text_chirho(text_chirho)
        .chars()
        .next()
        .is_some_and(is_symbol_start_char_chirho)
}

fn qualified_name_is_consym_chirho(text_chirho: &str) -> bool {
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
        RawTokenKindChirho::VarIdChirho => TokenKindChirho::VarIdChirho,
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

// ---------------------------------------------------------------------------
// ParserChirho — the recursive-descent parser
// ---------------------------------------------------------------------------

/// A recursive-descent parser that builds a lossless CST (green tree) from
/// a layout-processed token stream.
pub struct ParserChirho<'src> {
    source_chirho: &'src str,
    tokens_chirho: Vec<RawTokenChirho>,
    pos_chirho: usize,
    builder_chirho: GreenBuilderChirho,
}

impl<'src> ParserChirho<'src> {
    /// Create a parser from source text. Lexes and applies layout rule internally.
    pub fn new_chirho(source_chirho: &'src str, file_id_chirho: FileIdChirho) -> Self {
        let mut lexer_chirho = LexerChirho::new_chirho(source_chirho, file_id_chirho);
        let raw_chirho = lexer_chirho.lex_all_chirho();
        let tokens_chirho = apply_layout_chirho(source_chirho, raw_chirho, file_id_chirho);

        Self {
            source_chirho,
            tokens_chirho,
            pos_chirho: 0,
            builder_chirho: GreenBuilderChirho::new_chirho(),
        }
    }

    /// Parse the entire source file and return the root green node.
    pub fn parse_chirho(mut self) -> Arc<GreenNodeChirho> {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::SourceFileChirho);

        // Skip leading trivia (pragmas, comments, whitespace) before module header
        self.eat_trivia_chirho();

        // Parse module header if present
        if self.at_chirho(RawTokenKindChirho::ModuleChirho) {
            self.parse_module_header_chirho();
        }

        // Parse the body (layout block of declarations)
        self.parse_top_level_decls_chirho();

        // Eat remaining trivia + EOF
        self.eat_trivia_chirho();
        if self.pos_chirho < self.tokens_chirho.len() {
            self.bump_chirho(); // EOF
        }

        self.builder_chirho.finish_node_chirho();
        self.builder_chirho.finish_chirho()
    }

    // -----------------------------------------------------------------------
    // Module header
    // -----------------------------------------------------------------------

    fn parse_module_header_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ModuleHeaderChirho);

        self.eat_trivia_chirho();
        self.expect_chirho(RawTokenKindChirho::ModuleChirho); // module

        self.eat_trivia_chirho();
        // Module name — could be qualified (A.B.C)
        if self.at_chirho(RawTokenKindChirho::ConIdChirho)
            || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
        {
            self.bump_chirho();
        }

        self.eat_trivia_chirho();

        // Optional export list
        if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            self.parse_export_list_chirho();
        }

        self.eat_trivia_chirho();
        self.expect_chirho(RawTokenKindChirho::WhereChirho); // where

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_export_list_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ExportListChirho);

        self.expect_chirho(RawTokenKindChirho::LeftParenChirho);
        self.eat_trivia_chirho();

        while !self.at_chirho(RawTokenKindChirho::RightParenChirho) && !self.at_eof_chirho() {
            let before_chirho = self.pos_chirho;
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ExportSpecChirho);
            // Eat tokens until comma or close paren at nesting depth 0.
            // Track paren nesting so `Color(Red, Green)` is one ExportSpec.
            let mut paren_depth_chirho: u32 = 0;
            loop {
                self.eat_trivia_chirho();
                if self.at_eof_chirho() {
                    break;
                }
                if paren_depth_chirho == 0
                    && (self.at_chirho(RawTokenKindChirho::CommaChirho)
                        || self.at_chirho(RawTokenKindChirho::RightParenChirho))
                {
                    break;
                }
                if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
                    paren_depth_chirho += 1;
                } else if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();

            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho(); // comma
                self.eat_trivia_chirho();
            }
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Top-level declarations
    // -----------------------------------------------------------------------

    fn parse_top_level_decls_chirho(&mut self) {
        self.eat_trivia_chirho();

        // Eat virtual left brace if present
        if self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
            || self.at_chirho(RawTokenKindChirho::LeftBraceChirho)
        {
            self.bump_chirho();
        }

        loop {
            self.eat_trivia_chirho();

            // Check for end of layout block
            if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            {
                self.bump_chirho();
                break;
            }

            if self.at_eof_chirho() {
                break;
            }

            // Eat virtual semicolons
            if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
            {
                self.bump_chirho();
                continue;
            }

            let before_chirho = self.pos_chirho;
            self.parse_decl_chirho();
            // Safety: if no progress was made, consume a token to avoid
            // infinite loops on unexpected tokens.
            if self.pos_chirho == before_chirho {
                if !self.at_eof_chirho() {
                    self.builder_chirho
                        .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                    self.bump_chirho();
                    self.builder_chirho.finish_node_chirho();
                } else {
                    break;
                }
            }
        }
    }

    fn parse_decl_chirho(&mut self) {
        self.eat_trivia_chirho();

        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::ImportChirho) => self.parse_import_decl_chirho(),
            Some(RawTokenKindChirho::DataChirho) => self.parse_data_decl_chirho(),
            Some(RawTokenKindChirho::TypeChirho) => self.parse_type_or_family_decl_chirho(),
            Some(RawTokenKindChirho::NewtypeChirho) => self.parse_newtype_decl_chirho(),
            Some(RawTokenKindChirho::ClassChirho) => self.parse_class_decl_chirho(),
            Some(RawTokenKindChirho::InstanceChirho) => self.parse_instance_decl_chirho(),
            Some(RawTokenKindChirho::InfixChirho)
            | Some(RawTokenKindChirho::InfixlChirho)
            | Some(RawTokenKindChirho::InfixrChirho) => self.parse_fixity_decl_chirho(),
            Some(RawTokenKindChirho::DefaultChirho) => self.parse_default_decl_chirho(),
            Some(RawTokenKindChirho::DerivingChirho) => self.parse_standalone_deriving_chirho(),
            Some(RawTokenKindChirho::ForeignChirho) => self.parse_foreign_decl_chirho(),
            // Template Haskell splice at top level: $(expr) or $name
            Some(RawTokenKindChirho::ThSpliceChirho) => self.parse_splice_decl_chirho(),
            _ => {
                // `pattern ConName ...` → pattern synonym declaration
                if self.at_varid_text_chirho("pattern") && self.peek_next_is_conid_chirho() {
                    self.parse_pat_syn_decl_chirho();
                } else {
                    self.parse_value_decl_chirho();
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Import declarations
    // -----------------------------------------------------------------------

    fn parse_import_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ImportDeclChirho);

        self.expect_chirho(RawTokenKindChirho::ImportChirho);
        self.eat_trivia_chirho();

        // Optional "qualified"
        if self.at_varid_text_chirho("qualified") {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        // PackageImports: skip package name string literal (e.g. "base")
        if self.at_chirho(RawTokenKindChirho::StringLitChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        // Module name
        if self.at_chirho(RawTokenKindChirho::ConIdChirho)
            || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
        {
            self.bump_chirho();
        }
        self.eat_trivia_chirho();

        // ImportQualifiedPost: "qualified" after module name
        if self.at_varid_text_chirho("qualified") {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        // Optional "as Alias"
        if self.at_varid_text_chirho("as") {
            self.bump_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::ConIdChirho) {
                self.bump_chirho();
            }
            self.eat_trivia_chirho();
        }

        // Optional import spec: (items) or hiding (items)
        if self.at_varid_text_chirho("hiding") {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            self.parse_import_spec_list_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_import_spec_list_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ImportSpecListChirho);

        self.expect_chirho(RawTokenKindChirho::LeftParenChirho);
        self.eat_trivia_chirho();

        while !self.at_chirho(RawTokenKindChirho::RightParenChirho) && !self.at_eof_chirho() {
            let before_chirho = self.pos_chirho;
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ImportSpecChirho);

            let mut nested_paren_depth_chirho = 0usize;
            let mut nested_bracket_depth_chirho = 0usize;

            // Eat tokens until comma or the outer close paren, but preserve
            // nested member lists like MonadZip(mzipWith).
            while !self.at_eof_chirho() {
                self.eat_trivia_chirho();
                if self.at_eof_chirho() {
                    break;
                }

                let should_end_spec_chirho = match self.current_kind_chirho() {
                    Some(RawTokenKindChirho::CommaChirho) => {
                        nested_paren_depth_chirho == 0 && nested_bracket_depth_chirho == 0
                    }
                    Some(RawTokenKindChirho::RightParenChirho) => {
                        nested_paren_depth_chirho == 0 && nested_bracket_depth_chirho == 0
                    }
                    _ => false,
                };
                if should_end_spec_chirho {
                    break;
                }

                match self.current_kind_chirho() {
                    Some(RawTokenKindChirho::LeftParenChirho) => {
                        nested_paren_depth_chirho += 1;
                    }
                    Some(RawTokenKindChirho::RightParenChirho) => {
                        nested_paren_depth_chirho = nested_paren_depth_chirho.saturating_sub(1);
                    }
                    Some(RawTokenKindChirho::LeftBracketChirho) => {
                        nested_bracket_depth_chirho += 1;
                    }
                    Some(RawTokenKindChirho::RightBracketChirho) => {
                        nested_bracket_depth_chirho = nested_bracket_depth_chirho.saturating_sub(1);
                    }
                    _ => {}
                }

                self.bump_chirho();
            }

            self.builder_chirho.finish_node_chirho();

            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Data type declarations
    // -----------------------------------------------------------------------

    fn parse_data_decl_chirho(&mut self) {
        // Check for `data family` or `data instance` by peeking ahead
        let mut look_chirho = self.pos_chirho + 1;
        while look_chirho < self.tokens_chirho.len()
            && matches!(
                self.tokens_chirho[look_chirho].kind_chirho,
                RawTokenKindChirho::WhitespaceChirho
                    | RawTokenKindChirho::LineCommentChirho
                    | RawTokenKindChirho::BlockCommentChirho
            )
        {
            look_chirho += 1;
        }
        let next_text_chirho = if look_chirho < self.tokens_chirho.len() {
            self.token_text_chirho(&self.tokens_chirho[look_chirho])
        } else {
            ""
        };
        if next_text_chirho == "family" || next_text_chirho == "instance" {
            // data family / data instance — parse as a type-sig-like skipped decl
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::DataDeclChirho);
            self.eat_until_decl_end_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::DataDeclChirho);

        self.expect_chirho(RawTokenKindChirho::DataChirho);
        self.eat_trivia_chirho();

        // Type name and type variables until = or where or deriving
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::EqualsChirho,
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::DerivingChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        // = Constructor | Constructor ... (ADT syntax)
        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();

            self.parse_con_decl_chirho();

            while self.at_chirho(RawTokenKindChirho::PipeChirho) {
                self.bump_chirho(); // |
                self.eat_trivia_chirho();
                self.parse_con_decl_chirho();
            }
        } else if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            // GADT syntax: data T a where { C1 :: Type; C2 :: Type }
            self.bump_chirho(); // where
            self.eat_trivia_chirho();

            // Parse layout block of GADT constructor declarations
            let has_brace_chirho = self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
                || self.at_chirho(RawTokenKindChirho::LeftBraceChirho);
            if has_brace_chirho {
                self.bump_chirho(); // {
                self.eat_trivia_chirho();
            }

            loop {
                // End conditions
                if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                    || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                {
                    self.bump_chirho();
                    break;
                }
                if self.at_eof_chirho() || self.at_chirho(RawTokenKindChirho::DerivingChirho) {
                    break;
                }
                if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                    || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
                {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                    continue;
                }

                // Parse a GADT constructor: ConName :: Type
                if self.at_chirho(RawTokenKindChirho::ConIdChirho) {
                    self.parse_gadt_con_decl_chirho();
                } else {
                    // Skip unexpected tokens
                    let before_chirho = self.pos_chirho;
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                    if self.pos_chirho == before_chirho {
                        break;
                    }
                }
                self.eat_trivia_chirho();
            }
        }

        // Optional deriving clause
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::DerivingChirho) {
            self.parse_deriving_clause_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_con_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ConDeclChirho);

        // ExistentialQuantification: `forall a. Ctx => Con ...`
        if self.at_chirho(RawTokenKindChirho::ForallChirho) {
            self.bump_chirho(); // forall
            self.eat_trivia_chirho();
            // Eat type variables until dot
            while self.at_chirho(RawTokenKindChirho::VarIdChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            // Expect '.'
            if self.at_dot_chirho() {
                self.bump_chirho(); // .
                self.eat_trivia_chirho();
            }
            // Optional context: `Show a =>`
            // Check for `ConId VarId ... =>` pattern
            let _save_pos_chirho = self.pos_chirho;
            let mut found_arrow_chirho = false;
            let mut lookahead_chirho = self.pos_chirho;
            while lookahead_chirho < self.tokens_chirho.len() {
                let tk_chirho = self.tokens_chirho[lookahead_chirho].kind_chirho;
                if tk_chirho == RawTokenKindChirho::RightArrowChirho {
                    // Check if it's => (FatArrow) — but our lexer may produce RightArrow for =>
                    // Actually let's check for FatArrow
                    break;
                }
                if tk_chirho == RawTokenKindChirho::FatArrowChirho {
                    found_arrow_chirho = true;
                    break;
                }
                if tk_chirho == RawTokenKindChirho::VirtualSemicolonChirho
                    || tk_chirho == RawTokenKindChirho::PipeChirho
                {
                    break;
                }
                lookahead_chirho += 1;
            }
            if found_arrow_chirho {
                // Parse constraint(s) before =>
                while !self.at_chirho(RawTokenKindChirho::FatArrowChirho) && !self.at_eof_chirho() {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if self.at_chirho(RawTokenKindChirho::FatArrowChirho) {
                    self.bump_chirho(); // =>
                    self.eat_trivia_chirho();
                }
            }
        }

        let starts_infix_con_decl_chirho = self.starts_infix_con_decl_chirho();

        if starts_infix_con_decl_chirho {
            while self.can_start_atype_chirho() || self.at_strict_prefix_chirho() {
                let before_chirho = self.pos_chirho;
                if self.at_strict_prefix_chirho() {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if self.can_start_atype_chirho() {
                    self.parse_atype_chirho();
                    self.eat_trivia_chirho();
                }
                let lookahead_idx_chirho = self.skip_trivia_idx_chirho(self.pos_chirho);
                if self
                    .tokens_chirho
                    .get(lookahead_idx_chirho)
                    .is_some_and(|token_chirho| {
                        token_chirho.kind_chirho == RawTokenKindChirho::ConSymChirho
                    })
                {
                    break;
                }
                if self.pos_chirho == before_chirho {
                    break;
                }
            }

            if self.at_chirho(RawTokenKindChirho::ConSymChirho) {
                self.bump_chirho(); // constructor operator
                self.eat_trivia_chirho();
                while self.can_start_atype_chirho() || self.at_strict_prefix_chirho() {
                    let before_chirho = self.pos_chirho;
                    if self.at_strict_prefix_chirho() {
                        self.bump_chirho();
                        self.eat_trivia_chirho();
                    }
                    if self.can_start_atype_chirho() {
                        self.parse_atype_chirho();
                        self.eat_trivia_chirho();
                    }
                    if self.pos_chirho == before_chirho {
                        break;
                    }
                }
            }
        // Prefix constructor name
        } else if self.at_chirho(RawTokenKindChirho::ConIdChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();

            // Record syntax: { field :: Type, ... }
            if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                self.parse_record_fields_chirho();
            } else {
                // Ordinary constructor: parse atomic types as fields
                // Handle strictness annotations: `!` before a field type
                while self.can_start_atype_chirho() || self.at_strict_prefix_chirho() {
                    let before_chirho = self.pos_chirho;
                    // Consume `!` strictness annotation if present
                    if self.at_strict_prefix_chirho() {
                        self.bump_chirho(); // !
                        self.eat_trivia_chirho();
                    }
                    if self.can_start_atype_chirho() {
                        self.parse_atype_chirho();
                        self.eat_trivia_chirho();
                    }
                    if self.pos_chirho == before_chirho {
                        break;
                    }
                }
            }
        } else if self.can_start_atype_chirho() || self.at_strict_prefix_chirho() {
            // Infix constructor declaration: `!a :*: !b`
            let before_left_chirho = self.pos_chirho;
            if self.at_strict_prefix_chirho() {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            if self.can_start_atype_chirho() {
                self.parse_atype_chirho();
                self.eat_trivia_chirho();
            }

            if self.pos_chirho != before_left_chirho
                && self.at_chirho(RawTokenKindChirho::ConSymChirho)
            {
                self.bump_chirho(); // constructor operator
                self.eat_trivia_chirho();
                if self.at_strict_prefix_chirho() {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                // Right operand may be a type application (e.g. `Seq a`
                // in `a :< Seq a`), so parse all atomic types, not just one.
                while self.can_start_atype_chirho() || self.at_strict_prefix_chirho() {
                    let before_chirho = self.pos_chirho;
                    if self.at_strict_prefix_chirho() {
                        self.bump_chirho();
                        self.eat_trivia_chirho();
                    }
                    if self.can_start_atype_chirho() {
                        self.parse_atype_chirho();
                        self.eat_trivia_chirho();
                    }
                    if self.pos_chirho == before_chirho {
                        break;
                    }
                }
            }
        } else {
            // Fallback: eat until boundary
            self.eat_until_any_chirho(&[
                RawTokenKindChirho::PipeChirho,
                RawTokenKindChirho::DerivingChirho,
                RawTokenKindChirho::VirtualSemicolonChirho,
                RawTokenKindChirho::VirtualRightBraceChirho,
                RawTokenKindChirho::SemicolonChirho,
                RawTokenKindChirho::RightBraceChirho,
            ]);
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn starts_infix_con_decl_chirho(&self) -> bool {
        let mut idx_chirho = self.skip_trivia_idx_chirho(self.pos_chirho);

        loop {
            if self
                .tokens_chirho
                .get(idx_chirho)
                .is_some_and(|token_chirho| {
                    token_chirho.kind_chirho == RawTokenKindChirho::VarSymChirho
                        && self.token_text_chirho(token_chirho) == "!"
                })
            {
                idx_chirho = self.skip_trivia_idx_chirho(idx_chirho + 1);
            }

            let Some(after_atype_chirho) = self.peek_after_apat_chirho(idx_chirho) else {
                return false;
            };

            let lookahead_idx_chirho = self.skip_trivia_idx_chirho(after_atype_chirho);
            let Some(lookahead_token_chirho) = self.tokens_chirho.get(lookahead_idx_chirho) else {
                return false;
            };

            if lookahead_token_chirho.kind_chirho == RawTokenKindChirho::ConSymChirho {
                return true;
            }

            if !self.can_start_atype_idx_chirho(lookahead_idx_chirho) {
                return false;
            }

            idx_chirho = lookahead_idx_chirho;
        }
    }

    /// Parse a GADT constructor declaration: `Con :: forall a. Ctx => Arg -> ... -> T a`
    fn parse_gadt_con_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::GadtConDeclChirho);

        // Constructor name(s) — could be `C1, C2 :: Type`
        self.bump_chirho(); // ConId
        self.eat_trivia_chirho();

        // Expect ::
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
        }

        // Parse the type signature
        self.parse_type_chirho();

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_record_fields_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::RecordFieldsChirho);

        self.expect_chirho(RawTokenKindChirho::LeftBraceChirho);
        self.eat_trivia_chirho();

        while !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            && !self.at_decl_boundary_chirho()
            && !self.at_eof_chirho()
        {
            let outer_before_chirho = self.pos_chirho;
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::FieldDeclChirho);
            let mut paren_depth_chirho = 0usize;
            let mut bracket_depth_chirho = 0usize;
            let mut brace_depth_chirho = 0usize;

            // field names, ::, type
            while !self.at_decl_boundary_chirho() && !self.at_eof_chirho() {
                self.eat_trivia_chirho();
                if self.at_decl_boundary_chirho() || self.at_eof_chirho() {
                    break;
                }

                let should_end_field_chirho =
                    match self.current_chirho().map(|t_chirho| t_chirho.kind_chirho) {
                        Some(RawTokenKindChirho::CommaChirho) => {
                            paren_depth_chirho == 0
                                && bracket_depth_chirho == 0
                                && brace_depth_chirho == 0
                        }
                        Some(RawTokenKindChirho::RightBraceChirho) => {
                            paren_depth_chirho == 0
                                && bracket_depth_chirho == 0
                                && brace_depth_chirho == 0
                        }
                        _ => false,
                    };
                if should_end_field_chirho {
                    break;
                }

                match self.current_chirho().map(|t_chirho| t_chirho.kind_chirho) {
                    Some(RawTokenKindChirho::LeftParenChirho) => paren_depth_chirho += 1,
                    Some(RawTokenKindChirho::RightParenChirho) => {
                        paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                    }
                    Some(RawTokenKindChirho::LeftBracketChirho) => bracket_depth_chirho += 1,
                    Some(RawTokenKindChirho::RightBracketChirho) => {
                        bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                    }
                    Some(RawTokenKindChirho::LeftBraceChirho) => brace_depth_chirho += 1,
                    Some(RawTokenKindChirho::RightBraceChirho) => {
                        brace_depth_chirho = brace_depth_chirho.saturating_sub(1);
                    }
                    _ => {}
                }

                self.bump_chirho();
            }

            self.builder_chirho.finish_node_chirho();

            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            if self.pos_chirho == outer_before_chirho {
                break;
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightBraceChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_deriving_clause_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::DerivingClauseChirho);

        self.expect_chirho(RawTokenKindChirho::DerivingChirho);
        self.eat_trivia_chirho();

        // DerivingStrategies: consume optional strategy keyword
        if self.at_varid_text_chirho("stock")
            || self.at_varid_text_chirho("newtype")
            || self.at_varid_text_chirho("anyclass")
        {
            self.bump_chirho(); // strategy keyword
            self.eat_trivia_chirho();
        }

        // deriving (Show, Eq) or deriving Show
        if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
            while !self.at_chirho(RawTokenKindChirho::RightParenChirho) && !self.at_eof_chirho() {
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::CommaChirho)
                    && !self.at_chirho(RawTokenKindChirho::RightParenChirho)
                {
                    self.bump_chirho();
                }
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                    self.bump_chirho();
                }
            }
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
        } else if self.at_chirho(RawTokenKindChirho::ConIdChirho) {
            self.bump_chirho();
        }

        // DerivingVia: `deriving (Class) via Type`
        // Check for `via` keyword after the class list.
        self.eat_trivia_chirho();
        if self.at_varid_text_chirho("via") {
            self.bump_chirho(); // consume "via"
            self.eat_trivia_chirho();
            // Parse the via type (could be a ConId, or parenthesized, etc.)
            self.parse_atype_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Type alias declarations
    // -----------------------------------------------------------------------

    /// Dispatch `type` keyword: check if followed by `family` or `instance`.
    fn parse_type_or_family_decl_chirho(&mut self) {
        // Peek ahead past trivia to see if next token is "family" or "instance"
        let mut look_chirho = self.pos_chirho + 1;
        while look_chirho < self.tokens_chirho.len()
            && matches!(
                self.tokens_chirho[look_chirho].kind_chirho,
                RawTokenKindChirho::WhitespaceChirho
                    | RawTokenKindChirho::LineCommentChirho
                    | RawTokenKindChirho::BlockCommentChirho
            )
        {
            look_chirho += 1;
        }
        let next_text_chirho = if look_chirho < self.tokens_chirho.len() {
            self.token_text_chirho(&self.tokens_chirho[look_chirho])
        } else {
            ""
        };
        match next_text_chirho {
            "family" => self.parse_type_family_decl_chirho(),
            "instance" => self.parse_type_family_instance_decl_chirho(),
            "role" => {
                // RoleAnnotations: `type role T nominal phantom representational`
                // Just consume the entire declaration
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::TypeSigDeclChirho);
                self.eat_until_decl_end_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            _ => {
                // StandaloneKindSignatures: `type T :: Kind`
                // Detect pattern: `type ConId ::` (no `=` before `::`)
                // by scanning ahead for `::` before `=` or decl boundary.
                let mut scan_chirho = look_chirho;
                let mut is_kind_sig_chirho = false;
                // Skip past the name token (ConId) to check for ::
                if scan_chirho < self.tokens_chirho.len() {
                    scan_chirho += 1; // past ConId
                    while scan_chirho < self.tokens_chirho.len()
                        && matches!(
                            self.tokens_chirho[scan_chirho].kind_chirho,
                            RawTokenKindChirho::WhitespaceChirho
                                | RawTokenKindChirho::LineCommentChirho
                                | RawTokenKindChirho::BlockCommentChirho
                        )
                    {
                        scan_chirho += 1;
                    }
                    if scan_chirho < self.tokens_chirho.len()
                        && self.tokens_chirho[scan_chirho].kind_chirho
                            == RawTokenKindChirho::ColonColonChirho
                    {
                        is_kind_sig_chirho = true;
                    }
                }
                if is_kind_sig_chirho {
                    // Standalone kind signature — consume as TypeSigDecl (skipped)
                    self.builder_chirho
                        .start_node_chirho(SyntaxKindChirho::TypeSigDeclChirho);
                    self.eat_until_decl_end_chirho();
                    self.builder_chirho.finish_node_chirho();
                } else {
                    self.parse_type_alias_decl_chirho();
                }
            }
        }
    }

    fn parse_type_family_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeFamilyDeclChirho);

        self.expect_chirho(RawTokenKindChirho::TypeChirho); // type
        self.eat_trivia_chirho();
        self.bump_chirho(); // family
        self.eat_trivia_chirho();

        // Family name and type variables until `where`, `::`, or end of decl
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::ColonColonChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        // Optional result kind annotation `:: *`
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // kind
        }

        // Check for `where` (closed type family)
        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.bump_chirho(); // where
            self.eat_trivia_chirho();
            // Parse equations: each is `F lhs_types = rhs_type`
            if self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho) {
                self.bump_chirho();
            }
            while !self.at_eof_chirho()
                && !self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
            {
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho) {
                    self.bump_chirho();
                    continue;
                }
                if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho) {
                    break;
                }
                // Each equation: lhs = rhs
                self.eat_until_any_chirho(&[
                    RawTokenKindChirho::EqualsChirho,
                    RawTokenKindChirho::VirtualSemicolonChirho,
                    RawTokenKindChirho::VirtualRightBraceChirho,
                ]);
                if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
                    self.bump_chirho(); // =
                    self.eat_trivia_chirho();
                    self.parse_type_chirho(); // rhs
                }
            }
            if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho) {
                self.bump_chirho();
            }
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_type_family_instance_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeFamilyInstanceDeclChirho);

        self.expect_chirho(RawTokenKindChirho::TypeChirho); // type
        self.eat_trivia_chirho();
        self.bump_chirho(); // instance
        self.eat_trivia_chirho();

        // LHS patterns until =
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::EqualsChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // RHS type
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_type_alias_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeAliasDeclChirho);

        self.expect_chirho(RawTokenKindChirho::TypeChirho);
        self.eat_trivia_chirho();

        // Type name and variables until =
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::EqualsChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_type_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Newtype declarations
    // -----------------------------------------------------------------------

    fn parse_newtype_decl_chirho(&mut self) {
        // Check for `newtype instance` — data family instance with newtype
        let mut look_chirho = self.pos_chirho + 1;
        while look_chirho < self.tokens_chirho.len()
            && matches!(
                self.tokens_chirho[look_chirho].kind_chirho,
                RawTokenKindChirho::WhitespaceChirho
                    | RawTokenKindChirho::LineCommentChirho
                    | RawTokenKindChirho::BlockCommentChirho
            )
        {
            look_chirho += 1;
        }
        if look_chirho < self.tokens_chirho.len()
            && self.token_text_chirho(&self.tokens_chirho[look_chirho]) == "instance"
        {
            // newtype instance — parse as a skipped decl
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::NewtypeDeclChirho);
            self.eat_until_decl_end_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::NewtypeDeclChirho);

        self.expect_chirho(RawTokenKindChirho::NewtypeChirho);
        self.eat_trivia_chirho();

        // Type name and variables until =
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::EqualsChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_con_decl_chirho();
        }

        // Optional deriving clause
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::DerivingChirho) {
            self.parse_deriving_clause_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Class / Instance declarations
    // -----------------------------------------------------------------------

    fn parse_class_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ClassDeclChirho);

        self.expect_chirho(RawTokenKindChirho::ClassChirho);
        self.eat_trivia_chirho();

        // Eat until where or end of decl
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.parse_where_block_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_instance_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::InstanceDeclChirho);

        self.expect_chirho(RawTokenKindChirho::InstanceChirho);
        self.eat_trivia_chirho();

        self.eat_until_any_chirho(&[
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.parse_where_block_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Fixity / Default / Foreign declarations
    // -----------------------------------------------------------------------

    fn parse_fixity_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::FixityDeclChirho);

        // infix | infixl | infixr
        self.bump_chirho();
        self.eat_trivia_chirho();

        self.eat_until_decl_end_chirho();

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse: `deriving instance [context =>] ClassName Type`
    fn parse_standalone_deriving_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::StandaloneDerivingDeclChirho);
        self.bump_chirho(); // deriving
        self.eat_trivia_chirho();

        // Expect "instance"
        if self.at_chirho(RawTokenKindChirho::InstanceChirho) {
            self.bump_chirho(); // instance
            self.eat_trivia_chirho();
        }

        // Consume the rest of the declaration (context => Class Type)
        self.eat_until_decl_end_chirho();
        self.builder_chirho.finish_node_chirho();
    }

    fn parse_default_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::DefaultDeclChirho);
        self.bump_chirho(); // default
        self.eat_trivia_chirho();
        self.eat_until_decl_end_chirho();
        self.builder_chirho.finish_node_chirho();
    }

    /// Parse: foreign (import|export) callconv [safety] [string] name :: type
    /// Parse a pattern synonym declaration:
    /// `pattern ConName args = pat`   (implicitly bidirectional)
    /// `pattern ConName args <- pat`  (unidirectional)
    /// `pattern ConName args <- pat where ConName args = expr`  (explicitly bidirectional)
    fn parse_pat_syn_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::PatSynDeclChirho);

        self.bump_chirho(); // "pattern" (VarId)
        self.eat_trivia_chirho();

        // Pattern synonym name (ConId)
        if self.at_chirho(RawTokenKindChirho::ConIdChirho) {
            self.bump_chirho();
        }
        self.eat_trivia_chirho();

        // Pattern variables (VarIds before = or <-)
        while self.at_chirho(RawTokenKindChirho::VarIdChirho) && !self.at_varid_text_chirho("where")
        {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        // Direction: = (implicitly bidirectional) or <- (unidirectional/explicitly bidirectional)
        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_pat_chirho();
        } else if self.at_chirho(RawTokenKindChirho::LeftArrowChirho) {
            self.bump_chirho(); // <-
            self.eat_trivia_chirho();
            self.parse_pat_chirho();
            // Check for explicitly bidirectional where clause
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::WhereChirho) {
                self.parse_where_block_chirho();
            }
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_foreign_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ForeignDeclChirho);
        self.bump_chirho(); // foreign
        self.eat_trivia_chirho();

        // import or export
        if self.at_chirho(RawTokenKindChirho::ImportChirho) || self.at_varid_text_chirho("export") {
            self.bump_chirho();
        }
        self.eat_trivia_chirho();

        // calling convention: ccall, capi, stdcall, prim, javascript (all VarId tokens)
        if self.at_chirho(RawTokenKindChirho::VarIdChirho) {
            let text_chirho = self.current_text_chirho();
            if matches!(
                text_chirho,
                "ccall" | "capi" | "stdcall" | "prim" | "javascript" | "cplusplus"
            ) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
        }

        // optional safety: safe, unsafe, interruptible
        if self.at_varid_text_chirho("safe")
            || self.at_varid_text_chirho("unsafe")
            || self.at_varid_text_chirho("interruptible")
        {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        // optional C foreign name as string literal
        if self.at_chirho(RawTokenKindChirho::StringLitChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        // Haskell name (VarId or ConId or operator in parens)
        if self.at_chirho(RawTokenKindChirho::VarIdChirho)
            || self.at_chirho(RawTokenKindChirho::ConIdChirho)
        {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        // :: and type signature
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
            // Parse the rest as individual tokens until decl end
            self.eat_until_decl_end_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Value declarations (type sigs and bindings)
    // -----------------------------------------------------------------------

    fn parse_value_decl_chirho(&mut self) {
        // BangPatterns: skip `!` prefix in let/where bindings (e.g. `let !y = ...`)
        if self.current_kind_chirho() == Some(RawTokenKindChirho::VarSymChirho)
            && self.current_text_chirho() == "!"
        {
            self.bump_chirho(); // skip !
            self.eat_trivia_chirho();
        }
        if self.is_type_sig_chirho() {
            self.parse_type_sig_chirho();
        } else if self.starts_pat_bind_chirho() {
            self.parse_pat_bind_cst_chirho();
        } else {
            self.parse_fun_bind_chirho();
        }
    }

    /// Detect whether the current declaration is a pattern binding rather than
    /// a function binding.  A pattern binding starts with a non-variable
    /// pattern head: tuple `(a, b) = ...`, wildcard `_ = ...`, list pattern
    /// `[x] = ...`, or literal.  A `VarIdChirho` head is always parsed as a
    /// function binding (a 0-arg function binding is semantically equivalent).
    fn starts_pat_bind_chirho(&self) -> bool {
        if self.starts_infix_fun_bind_chirho() {
            return false;
        }
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                let Some(lookahead_idx_chirho) =
                    self.peek_after_fun_arg_pat_chirho(self.pos_chirho)
                else {
                    return false;
                };
                let lookahead_idx_chirho = self.skip_trivia_idx_chirho(lookahead_idx_chirho);
                self.tokens_chirho
                    .get(lookahead_idx_chirho)
                    .is_some_and(|token_chirho| {
                        token_chirho.kind_chirho == RawTokenKindChirho::ConSymChirho
                    })
            }
            // Tuple / parenthesised pattern — but not an operator section
            // like `(+) x = ...`. Heuristic: if the token right after `(`
            // is an operator symbol and is immediately closed by `)`, treat
            // it as an operator-in-parens binding head. Otherwise it is still
            // a parenthesized pattern such as `(!x, y) = ...`.
            Some(RawTokenKindChirho::LeftParenChirho) => {
                let mut i_chirho = self.pos_chirho + 1;
                // Skip trivia
                while i_chirho < self.tokens_chirho.len()
                    && self.tokens_chirho[i_chirho].kind_chirho.is_trivia_chirho()
                {
                    i_chirho += 1;
                }
                if i_chirho >= self.tokens_chirho.len() {
                    return false;
                }
                let next_kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
                if matches!(
                    next_kind_chirho,
                    RawTokenKindChirho::VarSymChirho | RawTokenKindChirho::ConSymChirho
                ) {
                    let mut j_chirho = i_chirho + 1;
                    while j_chirho < self.tokens_chirho.len()
                        && self.tokens_chirho[j_chirho].kind_chirho.is_trivia_chirho()
                    {
                        j_chirho += 1;
                    }
                    !self.tokens_chirho.get(j_chirho).is_some_and(|token_chirho| {
                        token_chirho.kind_chirho == RawTokenKindChirho::RightParenChirho
                    })
                } else {
                    true
                }
            }
            // Constructor at the start of a binding: `Just x = ...`,
            // `A x = ...` — this is a constructor pattern binding, not a
            // function definition.  Type sigs (`A :: Type`) are already
            // caught by `is_type_sig_chirho` which runs first.
            Some(RawTokenKindChirho::ConIdChirho) => true,
            // Wildcard, list pattern, or literal — always a pattern binding.
            Some(RawTokenKindChirho::UnderscoreChirho)
            | Some(RawTokenKindChirho::LeftBracketChirho)
            | Some(RawTokenKindChirho::IntLitChirho)
            | Some(RawTokenKindChirho::FloatLitChirho)
            | Some(RawTokenKindChirho::CharLitChirho)
            | Some(RawTokenKindChirho::StringLitChirho)
            | Some(RawTokenKindChirho::TildeChirho) => true,
            _ => false,
        }
    }

    /// Parse a pattern binding: `pat = expr` wrapped in a `PatBindChirho` node.
    fn parse_pat_bind_cst_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::PatBindChirho);

        // Parse the LHS pattern
        self.parse_pat_chirho();
        self.eat_trivia_chirho();

        // `=` and RHS expression
        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
        }

        // Optional where clause
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.parse_where_block_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_type_sig_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeSigDeclChirho);

        loop {
            // Name (or operator in parens)
            if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
                // Operator in parens: (+!)
                self.bump_chirho(); // (
                self.eat_trivia_chirho();
                while !self.at_chirho(RawTokenKindChirho::RightParenChirho) && !self.at_eof_chirho()
                {
                    self.bump_chirho();
                }
                if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                    self.bump_chirho();
                }
            } else {
                self.bump_chirho(); // name
            }
            self.eat_trivia_chirho();

            if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho(); // ,
                self.eat_trivia_chirho();
                continue;
            }
            break;
        }

        // ::
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
            self.parse_type_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_fun_bind_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::FunBindChirho);

        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::MatchChirho);

        if self.starts_infix_fun_bind_chirho() {
            self.parse_lpat_chirho();
            self.eat_trivia_chirho();

            if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
                if !self.at_eof_chirho()
                    && (self.at_chirho(RawTokenKindChirho::VarIdChirho)
                        || self.at_chirho(RawTokenKindChirho::ConIdChirho)
                        || self.at_chirho(RawTokenKindChirho::VarSymChirho)
                        || self.at_chirho(RawTokenKindChirho::ConSymChirho))
                {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
            } else if self.at_chirho(RawTokenKindChirho::VarSymChirho)
                || self.at_chirho(RawTokenKindChirho::ConSymChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }

            if self.can_start_fun_arg_pat_chirho() {
                self.parse_lpat_chirho();
                self.eat_trivia_chirho();
            }
        } else {
            // Function name or pattern head
            if self.at_chirho(RawTokenKindChirho::VarIdChirho)
                || self.at_chirho(RawTokenKindChirho::ConIdChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
            } else if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
                let mut lookahead_idx_chirho = self.pos_chirho + 1;
                while lookahead_idx_chirho < self.tokens_chirho.len()
                    && self.tokens_chirho[lookahead_idx_chirho]
                        .kind_chirho
                        .is_trivia_chirho()
                {
                    lookahead_idx_chirho += 1;
                }
                let is_parenthesized_operator_name_chirho = lookahead_idx_chirho
                    < self.tokens_chirho.len()
                    && matches!(
                        self.tokens_chirho[lookahead_idx_chirho].kind_chirho,
                        RawTokenKindChirho::VarSymChirho | RawTokenKindChirho::ConSymChirho
                    );
                if is_parenthesized_operator_name_chirho {
                    self.bump_chirho(); // (
                    self.eat_trivia_chirho();
                    if self.at_chirho(RawTokenKindChirho::VarSymChirho)
                        || self.at_chirho(RawTokenKindChirho::ConSymChirho)
                    {
                        self.bump_chirho(); // operator name
                        self.eat_trivia_chirho();
                    }
                    if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                        self.bump_chirho(); // )
                        self.eat_trivia_chirho();
                    }
                } else {
                    // Tuple pattern or parenthesized argument
                    self.parse_apat_chirho();
                    self.eat_trivia_chirho();
                }
            } else if self.can_start_apat_chirho() {
                self.parse_apat_chirho();
                self.eat_trivia_chirho();
            }

            // Parse argument patterns until = or |
            // Use parse_fun_arg_pat_chirho which parses atomic patterns
            // without constructor application (so `f True True` gives two
            // separate patterns, not `True applied_to True`). Handles
            // as-patterns, negated literals, and bang patterns.
            while self.can_start_apat_chirho()
                || (self.current_kind_chirho() == Some(RawTokenKindChirho::VarSymChirho)
                    && (self.current_text_chirho() == "-" || self.current_text_chirho() == "!"))
            {
                let before_chirho = self.pos_chirho;
                self.parse_fun_arg_pat_chirho();
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    break;
                }
            }
        }

        // Additional argument patterns after an infix lhs.
        while self.can_start_apat_chirho()
            || (self.current_kind_chirho() == Some(RawTokenKindChirho::VarSymChirho)
                && (self.current_text_chirho() == "-" || self.current_text_chirho() == "!"))
        {
            let before_chirho = self.pos_chirho;
            self.parse_fun_arg_pat_chirho();
            self.eat_trivia_chirho();
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        // Guards or = RHS
        if self.at_chirho(RawTokenKindChirho::PipeChirho) {
            self.parse_guarded_rhs_chirho();
        } else if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
        }

        self.builder_chirho.finish_node_chirho(); // Match

        // Optional where clause
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.parse_where_block_chirho();
        }

        self.builder_chirho.finish_node_chirho(); // FunBind
    }

    fn starts_infix_fun_bind_chirho(&self) -> bool {
        let Some(current_kind_chirho) = self.current_kind_chirho() else {
            return false;
        };
        let starts_with_name_head_chirho = matches!(
            current_kind_chirho,
            RawTokenKindChirho::VarIdChirho | RawTokenKindChirho::ConIdChirho
        );
        let mut lookahead_idx_chirho = match self.peek_after_fun_arg_pat_chirho(self.pos_chirho) {
            Some(idx_chirho) => idx_chirho,
            None => return false,
        };
        lookahead_idx_chirho = self.skip_trivia_idx_chirho(lookahead_idx_chirho);
        if lookahead_idx_chirho >= self.tokens_chirho.len() {
            return false;
        }
        if starts_with_name_head_chirho
            && self.tokens_chirho[lookahead_idx_chirho].kind_chirho
                == RawTokenKindChirho::VarSymChirho
        {
            let op_text_chirho = self.token_text_chirho(&self.tokens_chirho[lookahead_idx_chirho]);
            if op_text_chirho == "!" || op_text_chirho == "-" {
                // Prefer prefix function bindings with bang / negated argument
                // patterns (e.g. `f !x !y = ...`, `g -1 y = ...`) over the rare
                // bare infix declaration shape `x ! y = ...`.
                return false;
            }
        }
        matches!(
            self.tokens_chirho[lookahead_idx_chirho].kind_chirho,
            RawTokenKindChirho::BacktickChirho | RawTokenKindChirho::VarSymChirho
        )
    }

    fn peek_after_fun_arg_pat_chirho(&self, start_idx_chirho: usize) -> Option<usize> {
        let current_kind_chirho = self.tokens_chirho.get(start_idx_chirho)?.kind_chirho;
        match current_kind_chirho {
            RawTokenKindChirho::VarIdChirho => {
                let mut idx_chirho = self.skip_trivia_idx_chirho(start_idx_chirho + 1);
                if self.tokens_chirho.get(idx_chirho)?.kind_chirho == RawTokenKindChirho::AtChirho {
                    idx_chirho = self.skip_trivia_idx_chirho(idx_chirho + 1);
                    self.peek_after_apat_chirho(idx_chirho)
                } else {
                    Some(idx_chirho)
                }
            }
            RawTokenKindChirho::ConIdChirho | RawTokenKindChirho::QualifiedIdChirho => {
                let mut idx_chirho = self.skip_trivia_idx_chirho(start_idx_chirho + 1);
                if self
                    .tokens_chirho
                    .get(idx_chirho)
                    .is_some_and(|token_chirho| {
                        token_chirho.kind_chirho == RawTokenKindChirho::LeftBraceChirho
                    })
                {
                    return self.peek_after_balanced_group_chirho(
                        idx_chirho,
                        RawTokenKindChirho::LeftBraceChirho,
                        RawTokenKindChirho::RightBraceChirho,
                    );
                }

                while self.can_start_fun_arg_pat_idx_chirho(idx_chirho) {
                    let after_arg_chirho = self.peek_after_fun_arg_pat_chirho(idx_chirho)?;
                    if after_arg_chirho == idx_chirho {
                        break;
                    }
                    idx_chirho = self.skip_trivia_idx_chirho(after_arg_chirho);
                }
                Some(idx_chirho)
            }
            RawTokenKindChirho::VarSymChirho => {
                let token_text_chirho =
                    self.token_text_chirho(&self.tokens_chirho[start_idx_chirho]);
                if token_text_chirho == "-" || token_text_chirho == "!" {
                    let idx_chirho = self.skip_trivia_idx_chirho(start_idx_chirho + 1);
                    self.peek_after_apat_chirho(idx_chirho)
                } else {
                    None
                }
            }
            RawTokenKindChirho::TildeChirho => {
                let idx_chirho = self.skip_trivia_idx_chirho(start_idx_chirho + 1);
                self.peek_after_apat_chirho(idx_chirho)
            }
            _ => self.peek_after_apat_chirho(start_idx_chirho),
        }
    }

    fn peek_after_apat_chirho(&self, start_idx_chirho: usize) -> Option<usize> {
        let current_kind_chirho = self.tokens_chirho.get(start_idx_chirho)?.kind_chirho;
        match current_kind_chirho {
            RawTokenKindChirho::VarIdChirho
            | RawTokenKindChirho::ConIdChirho
            | RawTokenKindChirho::QualifiedIdChirho
            | RawTokenKindChirho::UnderscoreChirho
            | RawTokenKindChirho::IntLitChirho
            | RawTokenKindChirho::FloatLitChirho
            | RawTokenKindChirho::CharLitChirho
            | RawTokenKindChirho::StringLitChirho => Some(start_idx_chirho + 1),
            RawTokenKindChirho::LeftParenChirho => {
                self.peek_after_parenthesized_apat_chirho(start_idx_chirho)
            }
            RawTokenKindChirho::LeftBracketChirho => self.peek_after_balanced_group_chirho(
                start_idx_chirho,
                RawTokenKindChirho::LeftBracketChirho,
                RawTokenKindChirho::RightBracketChirho,
            ),
            _ => None,
        }
    }

    fn peek_after_parenthesized_apat_chirho(&self, start_idx_chirho: usize) -> Option<usize> {
        self.peek_after_balanced_group_chirho(
            start_idx_chirho,
            RawTokenKindChirho::LeftParenChirho,
            RawTokenKindChirho::RightParenChirho,
        )
    }

    fn peek_after_balanced_group_chirho(
        &self,
        start_idx_chirho: usize,
        left_kind_chirho: RawTokenKindChirho,
        right_kind_chirho: RawTokenKindChirho,
    ) -> Option<usize> {
        if self.tokens_chirho.get(start_idx_chirho)?.kind_chirho != left_kind_chirho {
            return None;
        }

        let mut depth_chirho = 0usize;
        let mut idx_chirho = start_idx_chirho;
        while idx_chirho < self.tokens_chirho.len() {
            match self.tokens_chirho[idx_chirho].kind_chirho {
                kind_chirho if kind_chirho == left_kind_chirho => {
                    depth_chirho += 1;
                }
                kind_chirho if kind_chirho == right_kind_chirho => {
                    depth_chirho = depth_chirho.saturating_sub(1);
                    if depth_chirho == 0 {
                        return Some(idx_chirho + 1);
                    }
                }
                _ => {}
            }
            idx_chirho += 1;
        }

        None
    }

    fn skip_trivia_idx_chirho(&self, mut idx_chirho: usize) -> usize {
        while idx_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[idx_chirho]
                .kind_chirho
                .is_trivia_chirho()
        {
            idx_chirho += 1;
        }
        idx_chirho
    }

    fn parse_guarded_rhs_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::GuardedRhsChirho);

        while self.at_chirho(RawTokenKindChirho::PipeChirho) {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::GuardChirho);

            self.bump_chirho(); // |
            self.eat_trivia_chirho();

            // Guard qualifiers until `=`. Supports:
            //   | guardExpr = rhs
            //   | pat <- expr, guardExpr = rhs
            //   | let binds, guardExpr = rhs
            while !self.at_chirho(RawTokenKindChirho::EqualsChirho)
                && !self.at_eof_chirho()
                && !self.at_decl_boundary_chirho()
            {
                let before_chirho = self.pos_chirho;
                self.eat_trivia_chirho();

                if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                    continue;
                }

                if self.at_chirho(RawTokenKindChirho::LetChirho) {
                    self.builder_chirho
                        .start_node_chirho(SyntaxKindChirho::LetStmtChirho);
                    self.bump_chirho(); // let
                    self.eat_trivia_chirho();
                    self.parse_layout_block_chirho();
                    self.builder_chirho.finish_node_chirho();
                    self.eat_trivia_chirho();
                } else if self.scan_for_guard_bind_arrow_chirho() {
                    self.parse_pat_chirho();
                    self.eat_trivia_chirho();
                    if self.at_chirho(RawTokenKindChirho::LeftArrowChirho) {
                        self.bump_chirho(); // <-
                        self.eat_trivia_chirho();
                        self.parse_expr_chirho();
                        self.eat_trivia_chirho();
                    }
                } else {
                    self.parse_guard_expr_chirho();
                }

                if self.pos_chirho == before_chirho {
                    break;
                }
            }

            if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
                self.bump_chirho(); // =
                self.eat_trivia_chirho();
                self.parse_expr_chirho();
            }

            self.builder_chirho.finish_node_chirho(); // Guard
            self.eat_trivia_chirho();
        }

        self.builder_chirho.finish_node_chirho(); // GuardedRhs
    }

    /// Parse a guard expression (the part between | and =).
    fn parse_guard_expr_chirho(&mut self) {
        self.parse_expr_chirho();
        self.eat_trivia_chirho();
    }

    /// Scan ahead to see whether the current guard qualifier contains a
    /// top-level `<-` before the next comma or `=`.
    fn scan_for_guard_bind_arrow_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho;
        let mut paren_depth_chirho = 0u32;
        let mut bracket_depth_chirho = 0u32;
        let mut brace_depth_chirho = 0u32;

        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            match kind_chirho {
                RawTokenKindChirho::LeftArrowChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return true;
                }
                RawTokenKindChirho::CommaChirho | RawTokenKindChirho::EqualsChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return false;
                }
                RawTokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                RawTokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                RawTokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBraceChirho
                | RawTokenKindChirho::VirtualLeftBraceChirho => {
                    brace_depth_chirho += 1;
                }
                RawTokenKindChirho::RightBraceChirho
                | RawTokenKindChirho::VirtualRightBraceChirho => {
                    brace_depth_chirho = brace_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::PipeChirho if brace_depth_chirho == 0 => {
                    return false;
                }
                _ => {}
            }
            i_chirho += 1;
        }

        false
    }

    // -----------------------------------------------------------------------
    // Where blocks
    // -----------------------------------------------------------------------

    /// Lookahead: after the current `|` token, scan for `->` before `=`,
    /// `,`, or another `|` at the top level.  Returns `true` when the `|`
    /// looks like a MultiWayIf alternative (`| cond -> result`), `false`
    /// when it looks like an outer guard (`| guard = rhs` or `| guard, ...`).
    fn scan_multiway_if_alt_has_arrow_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho + 1; // skip the current `|`
        let mut paren_depth_chirho = 0u32;
        let mut bracket_depth_chirho = 0u32;
        let mut brace_depth_chirho = 0u32;

        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            match kind_chirho {
                RawTokenKindChirho::RightArrowChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return true;
                }
                RawTokenKindChirho::EqualsChirho | RawTokenKindChirho::CommaChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return false;
                }
                RawTokenKindChirho::PipeChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    // Another top-level `|` before any `->` or `=`;
                    // ambiguous, but assume this one is also a MultiWayIf alt.
                    return true;
                }
                RawTokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                RawTokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                RawTokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBraceChirho
                | RawTokenKindChirho::VirtualLeftBraceChirho => {
                    brace_depth_chirho += 1;
                }
                RawTokenKindChirho::RightBraceChirho
                | RawTokenKindChirho::VirtualRightBraceChirho => {
                    if brace_depth_chirho == 0 {
                        return false;
                    }
                    brace_depth_chirho -= 1;
                }
                RawTokenKindChirho::VirtualSemicolonChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    // Layout semicolons at top level indicate we've left the
                    // MultiWayIf context.
                    return false;
                }
                _ => {}
            }
            i_chirho += 1;
        }
        false
    }

    fn parse_where_block_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::WhereClauseChirho);

        self.expect_chirho(RawTokenKindChirho::WhereChirho);
        self.eat_trivia_chirho();

        // Eat the layout block
        if self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
            || self.at_chirho(RawTokenKindChirho::LeftBraceChirho)
        {
            self.bump_chirho(); // {

            loop {
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                    || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                {
                    self.bump_chirho(); // }
                    break;
                }
                if self.at_eof_chirho() {
                    break;
                }
                if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                    || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
                {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                    continue;
                }
                let before_chirho = self.pos_chirho;
                self.parse_decl_chirho();
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    if !self.at_eof_chirho() {
                        self.bump_chirho();
                    } else {
                        break;
                    }
                }
            }
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Type parsing
    // -----------------------------------------------------------------------

    /// Parse a type. Handles qualified types (context => type) and function
    /// types (a -> b). Uses checkpoints for retroactive wrapping.
    fn parse_type_chirho(&mut self) {
        // forall a b . Type
        if self.at_chirho(RawTokenKindChirho::ForallChirho) {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ForallTypeChirho);
            self.bump_chirho(); // forall
            self.eat_trivia_chirho();
            // Collect type variables until '.'
            // Accepts bare VarId and kind-annotated (VarId :: Kind) binders
            // where the kind is a simple kind (*, k, * -> *, etc.)
            while !self.at_dot_chirho() && !self.at_eof_chirho() && !self.at_decl_boundary_chirho()
            {
                if self.at_chirho(RawTokenKindChirho::VarIdChirho) {
                    self.bump_chirho();
                } else if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
                    // Only parse as a kind-annotated binder if we can
                    // confirm the pattern: ( VarId :: SimpleKind )
                    // Use a lookahead check to avoid consuming complex
                    // type expressions that aren't simple kind annotations.
                    if self.is_simple_kind_annotated_binder_chirho() {
                        self.parse_paren_type_chirho();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
                self.eat_trivia_chirho();
            }
            if self.at_dot_chirho() {
                self.bump_chirho(); // .
                self.eat_trivia_chirho();
            }
            self.parse_type_chirho(); // body type
            self.builder_chirho.finish_node_chirho();
            return;
        }

        let cp_chirho = self.builder_chirho.checkpoint_chirho();

        self.parse_btype_chirho();
        self.eat_trivia_chirho();

        // Check for => (qualified type) — but only if we haven't gone
        // past the declaration boundary.
        if self.at_chirho(RawTokenKindChirho::FatArrowChirho) {
            // Retroactively wrap the parsed btype as a context
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::QualTypeChirho);
            self.bump_chirho(); // =>
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // the actual type
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for ⊸ (linear arrow — LinearTypes)
        if self.at_chirho(RawTokenKindChirho::LinearArrowChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::FunTypeChirho);
            self.bump_chirho(); // ⊸
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // right-recursive
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for %1 -> or %Many -> or %m -> (multiplicity annotation — LinearTypes)
        if self.at_varsym_chirho("%") {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::FunTypeChirho);
            self.bump_chirho(); // %
            self.eat_trivia_chirho();
            // Consume multiplicity: integer 1, conid Many/One, or varid (poly)
            if self.at_chirho(RawTokenKindChirho::IntLitChirho)
                || self.at_chirho(RawTokenKindChirho::ConIdChirho)
                || self.at_chirho(RawTokenKindChirho::VarIdChirho)
            {
                self.bump_chirho(); // multiplicity token
                self.eat_trivia_chirho();
            }
            // Expect ->
            if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                self.bump_chirho(); // ->
                self.eat_trivia_chirho();
            }
            self.parse_type_chirho(); // right-recursive
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for -> (function type)
        if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::FunTypeChirho);
            self.bump_chirho(); // ->
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // right-recursive
            self.builder_chirho.finish_node_chirho();
        }
        // Type equality constraint: a ~ b (parsed as infix type)
        if self.at_chirho(RawTokenKindChirho::TildeChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::InfixTypeChirho);
            self.bump_chirho(); // ~
            self.eat_trivia_chirho();
            self.parse_btype_chirho(); // right operand (btype, not full type, to avoid consuming =>)
            self.builder_chirho.finish_node_chirho();
            // After the infix ~ type, check if this is part of a qualified type
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::FatArrowChirho) {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::QualTypeChirho);
                self.bump_chirho(); // =>
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            return;
        }
        // TypeOperators: check for infix operator in type position (e.g. `a :+: b`)
        // VarSym or ConSym that isn't a special symbol like %, !, @
        // Use btype for the right operand so that `->` binds looser than
        // type operators: `a :~: b -> ()` parses as `(a :~: b) -> ()`.
        if (self.at_chirho(RawTokenKindChirho::VarSymChirho)
            || self.at_chirho(RawTokenKindChirho::ConSymChirho))
            && !matches!(self.current_text_chirho(), "%" | "!" | "@" | "|")
        {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::InfixTypeChirho);
            self.bump_chirho(); // operator
            self.eat_trivia_chirho();
            self.parse_btype_chirho(); // right operand (btype, not full type)
            self.builder_chirho.finish_node_chirho();
            // After the infix type, check for -> / => to continue
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::FunTypeChirho);
                self.bump_chirho(); // ->
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.builder_chirho.finish_node_chirho();
            } else if self.at_chirho(RawTokenKindChirho::FatArrowChirho) {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::QualTypeChirho);
                self.bump_chirho(); // =>
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            return;
        }
        // Backtick infix type constructors: a `Either` b
        if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::InfixTypeChirho);
            self.bump_chirho(); // `
            self.eat_trivia_chirho();
            // Consume the type constructor name
            if self.at_chirho(RawTokenKindChirho::ConIdChirho)
                || self.at_chirho(RawTokenKindChirho::VarIdChirho)
                || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                self.bump_chirho(); // closing `
                self.eat_trivia_chirho();
            }
            self.parse_btype_chirho(); // right operand (btype, not full type)
            self.builder_chirho.finish_node_chirho();
            // After backtick infix, check for -> / =>
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::FunTypeChirho);
                self.bump_chirho(); // ->
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.builder_chirho.finish_node_chirho();
            } else if self.at_chirho(RawTokenKindChirho::FatArrowChirho) {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::QualTypeChirho);
                self.bump_chirho(); // =>
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            return;
        }
        // Otherwise, just the btype stands as-is (no wrapping needed).
    }

    /// Parse a "btype" — type application (juxtaposition of atomic types).
    fn parse_btype_chirho(&mut self) {
        if !self.can_start_atype_chirho() {
            return;
        }

        let cp_chirho = self.builder_chirho.checkpoint_chirho();
        self.parse_atype_chirho();
        self.eat_trivia_chirho();

        let mut count_chirho = 1u32;
        while self.can_start_atype_chirho() {
            let before_chirho = self.pos_chirho;
            if count_chirho == 1 {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::AppTypeChirho);
            }
            count_chirho += 1;
            self.parse_atype_chirho();
            self.eat_trivia_chirho();
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        if count_chirho > 1 {
            self.builder_chirho.finish_node_chirho(); // AppType
        }
    }

    /// Parse an atomic type: variable, constructor, parenthesized, tuple,
    /// list, or unit.
    fn parse_atype_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::VarTypeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::ConIdChirho) | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ConTypeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftParenChirho) => {
                self.parse_paren_type_chirho();
            }
            Some(RawTokenKindChirho::LeftBracketChirho) => {
                self.parse_list_type_chirho();
            }
            // DataKinds: 'Constructor or '[Type, ...] promoted types
            Some(RawTokenKindChirho::TickChirho) => {
                self.parse_promoted_type_chirho();
            }
            // PartialTypeSignatures: `_` in type position is a wildcard type
            Some(RawTokenKindChirho::UnderscoreChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::WildcardTypeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            // DataKinds: type-level numeric/string/char literals
            Some(RawTokenKindChirho::IntLitChirho)
            | Some(RawTokenKindChirho::StringLitChirho)
            | Some(RawTokenKindChirho::CharLitChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LitTypeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            _ => {
                // Unexpected — wrap in error node
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                if !self.at_eof_chirho() {
                    self.bump_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
        }
    }

    /// Parse a parenthesized, tuple, or function-type-constructor type.
    fn parse_paren_type_chirho(&mut self) {
        // Could be: (type), (type, type, ...), (->), ()
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ParenTypeChirho);

        self.bump_chirho(); // (
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            // Unit type ()
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Could be (->) or operator as type constructor
        if self.at_chirho(RawTokenKindChirho::RightArrowChirho)
            || self.at_chirho(RawTokenKindChirho::VarSymChirho)
            || self.at_chirho(RawTokenKindChirho::ConSymChirho)
        {
            self.bump_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
                return;
            }
        }

        // Parse first type
        self.parse_type_chirho();
        self.eat_trivia_chirho();

        // Kind annotation: (a :: k) — consume :: and the kind type
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // the kind
            self.eat_trivia_chirho();
        }

        if self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // It's a tuple type — change the node kind would be ideal,
            // but we just keep ParenType and it wraps the contents.
            while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho(); // ,
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.eat_trivia_chirho();
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a list type: [type]
    fn parse_list_type_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ListTypeChirho);

        self.bump_chirho(); // [
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            // [] — empty list type constructor
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.parse_type_chirho();
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a DataKinds promoted type: `'Constructor` or `'[Type, ...]`.
    fn parse_promoted_type_chirho(&mut self) {
        match self.peek_after_tick_chirho() {
            Some(RawTokenKindChirho::ConIdChirho) | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                // Promoted constructor: 'True, 'Just, 'Nothing
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::PromotedConTypeChirho);
                self.bump_chirho(); // tick
                self.bump_chirho(); // ConId
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftBracketChirho) => {
                // Promoted list type: '[], '[Int, Bool]
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::PromotedListTypeChirho);
                self.bump_chirho(); // tick
                self.bump_chirho(); // [
                self.eat_trivia_chirho();

                if !self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                    self.parse_type_chirho();
                    self.eat_trivia_chirho();
                    while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                        self.bump_chirho(); // ,
                        self.eat_trivia_chirho();
                        self.parse_type_chirho();
                        self.eat_trivia_chirho();
                    }
                }

                if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                    self.bump_chirho(); // ]
                }
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftParenChirho) => {
                // Promoted tuple or unit: '(), '(,), '(,,)
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::PromotedConTypeChirho);
                self.bump_chirho(); // tick
                self.bump_chirho(); // (
                self.eat_trivia_chirho();
                // Eat commas for promoted tuple constructors
                while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                    self.bump_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
            _ => {
                // Fallback: just consume the tick as an error
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
        }
    }

    /// Check if the next non-trivia token after current is a ConId.
    /// Used to distinguish `pattern ConName ...` from a variable named `pattern`.
    fn peek_next_is_conid_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho + 1;
        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            if kind_chirho.is_trivia_chirho() {
                i_chirho += 1;
                continue;
            }
            return kind_chirho == RawTokenKindChirho::ConIdChirho;
        }
        false
    }

    /// Peek at the token kind after the current tick, skipping trivia.
    fn peek_after_tick_chirho(&self) -> Option<RawTokenKindChirho> {
        let mut i_chirho = self.pos_chirho + 1;
        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            if !kind_chirho.is_trivia_chirho() {
                return Some(kind_chirho);
            }
            i_chirho += 1;
        }
        None
    }

    /// Lookahead to check if the current `(` starts a simple kind-annotated
    /// binder suitable for forall: `( VarId :: SimpleKind )` where SimpleKind
    /// is `*`, `Type`, `Constraint`, a kind variable, or arrow kinds thereof.
    /// Returns false for complex kinds like `Either x y` or `forall k. k -> Type`.
    fn is_simple_kind_annotated_binder_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho + 1; // skip `(`
        // Skip trivia after `(`
        while i_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[i_chirho].kind_chirho.is_trivia_chirho()
        {
            i_chirho += 1;
        }
        // Must be VarId
        if i_chirho >= self.tokens_chirho.len()
            || self.tokens_chirho[i_chirho].kind_chirho != RawTokenKindChirho::VarIdChirho
        {
            return false;
        }
        i_chirho += 1;
        // Skip trivia
        while i_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[i_chirho].kind_chirho.is_trivia_chirho()
        {
            i_chirho += 1;
        }
        // Must be ::
        if i_chirho >= self.tokens_chirho.len()
            || self.tokens_chirho[i_chirho].kind_chirho != RawTokenKindChirho::ColonColonChirho
        {
            return false;
        }
        i_chirho += 1;
        // Scan ahead for `)` — only simple kind tokens allowed:
        // *, Type, Constraint, VarId (kind var), ConId (named kind), ->
        // Must find `)` before any complex syntax (forall, application).
        // Consecutive name tokens (VarId/ConId) without `->` indicate type
        // application (e.g. `Either x y`), not a simple kind.
        let mut paren_depth_chirho: u32 = 0;
        let mut last_was_name_chirho = false;
        while i_chirho < self.tokens_chirho.len() {
            let k_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            if k_chirho.is_trivia_chirho() {
                i_chirho += 1;
                continue;
            }
            match k_chirho {
                RawTokenKindChirho::RightParenChirho if paren_depth_chirho == 0 => return true,
                RawTokenKindChirho::RightParenChirho => {
                    paren_depth_chirho -= 1;
                    last_was_name_chirho = false;
                }
                RawTokenKindChirho::LeftParenChirho => {
                    paren_depth_chirho += 1;
                    last_was_name_chirho = false;
                }
                RawTokenKindChirho::VarIdChirho | RawTokenKindChirho::ConIdChirho => {
                    // Two consecutive names = type application, not a kind
                    if last_was_name_chirho && paren_depth_chirho == 0 {
                        return false;
                    }
                    last_was_name_chirho = true;
                }
                RawTokenKindChirho::RightArrowChirho => {
                    last_was_name_chirho = false;
                }
                // VarSym covers `*` which is used as a kind
                RawTokenKindChirho::VarSymChirho => {
                    last_was_name_chirho = false;
                }
                // Any other token (forall, etc.) is not simple
                _ => return false,
            }
            i_chirho += 1;
        }
        false
    }

    // -----------------------------------------------------------------------
    // Expression parsing
    // -----------------------------------------------------------------------

    /// Parse an expression. Handles type annotations (expr :: type) and
    /// infix operators. Stops at expression boundaries.
    fn parse_expr_chirho(&mut self) {
        let cp_chirho = self.builder_chirho.checkpoint_chirho();

        self.parse_infixexp_chirho();
        self.eat_trivia_chirho();

        // Type annotation: expr :: Type
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::TypeAnnotExprChirho);
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
            self.parse_type_chirho();
            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Parse an infix expression: lexp op lexp op ...
    /// Kept flat for CST — precedence resolved later.
    fn parse_infixexp_chirho(&mut self) {
        let cp_chirho = self.builder_chirho.checkpoint_chirho();

        self.parse_lexp_chirho();
        self.eat_trivia_chirho();

        let mut has_infix_chirho = false;

        while self.at_infix_op_chirho() {
            let before_chirho = self.pos_chirho;
            if !has_infix_chirho {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::InfixExprChirho);
                has_infix_chirho = true;
            }

            // The operator (VarSym, ConSym, or backticked name)
            if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                self.bump_chirho(); // `
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::BacktickChirho) && !self.at_eof_chirho() {
                    self.bump_chirho(); // the name
                }
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                    self.bump_chirho(); // `
                }
            } else {
                self.bump_chirho(); // operator
            }
            self.eat_trivia_chirho();

            if self.at_chirho(RawTokenKindChirho::BackslashChirho)
                || self.at_chirho(RawTokenKindChirho::LetChirho)
                || self.at_chirho(RawTokenKindChirho::IfChirho)
                || self.at_chirho(RawTokenKindChirho::CaseChirho)
                || self.at_chirho(RawTokenKindChirho::DoChirho)
            {
                self.parse_expr_chirho();
            } else {
                self.parse_lexp_chirho();
            }
            self.eat_trivia_chirho();
            // Safety: break if no progress to avoid infinite loop
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        if has_infix_chirho {
            self.builder_chirho.finish_node_chirho(); // InfixExpr
        }
    }

    /// Parse an lexp (left-hand expression): lambda, let, if, case, do,
    /// negation, or function application.
    fn parse_lexp_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::BackslashChirho) => self.parse_lambda_chirho(),
            Some(RawTokenKindChirho::LetChirho) => self.parse_let_expr_chirho(),
            Some(RawTokenKindChirho::IfChirho) => self.parse_if_expr_chirho(),
            Some(RawTokenKindChirho::CaseChirho) => self.parse_case_expr_chirho(),
            Some(RawTokenKindChirho::DoChirho) => self.parse_do_expr_chirho(),
            Some(RawTokenKindChirho::VarSymChirho)
                if self.current_text_chirho() == "-" && !self.at_decl_boundary_chirho() =>
            {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::NegateExprChirho);
                self.bump_chirho(); // -
                self.eat_trivia_chirho();
                self.parse_fexp_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            _ => self.parse_fexp_chirho(),
        }
    }

    /// Parse a lambda expression: \pat ... -> expr
    /// Also handles LambdaCase: \case { alts }
    fn parse_lambda_chirho(&mut self) {
        // Peek ahead: if the token after `\` is `case`, this is \case
        if self.peek_after_backslash_is_case_chirho() {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::LambdaCaseExprChirho);
            self.bump_chirho(); // backslash
            self.eat_trivia_chirho();
            self.bump_chirho(); // case
            self.eat_trivia_chirho();
            // Parse `of` if present (both `\case { ... }` and `\case of { ... }` forms)
            if self.at_chirho(RawTokenKindChirho::OfChirho) {
                self.bump_chirho(); // of
                self.eat_trivia_chirho();
            }
            // Parse the case alternatives (layout block inserted by layout engine
            // because `case` after `\` is treated as a layout keyword)
            self.parse_case_alts_chirho();
            self.builder_chirho.finish_node_chirho();
        } else {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::LambdaExprChirho);

            self.bump_chirho(); // backslash
            self.eat_trivia_chirho();

            // Lambda parameters admit the same function-argument patterns as
            // equation binders, including bang patterns like `\x !y -> ...`.
            while self.can_start_fun_arg_pat_chirho() {
                let before_chirho = self.pos_chirho;
                self.parse_fun_arg_pat_chirho();
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    break;
                }
            }

            if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                self.bump_chirho(); // ->
                self.eat_trivia_chirho();
                self.parse_expr_chirho();
            }

            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Check whether the next non-trivia token after `\` is the `case` keyword.
    fn peek_after_backslash_is_case_chirho(&self) -> bool {
        // Current token is `\`; look at the next non-trivia token
        let mut i_chirho = self.pos_chirho + 1;
        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            if kind_chirho.is_trivia_chirho() {
                i_chirho += 1;
                continue;
            }
            return kind_chirho == RawTokenKindChirho::CaseChirho;
        }
        false
    }

    /// Parse a let expression: let { decls } in expr
    fn parse_let_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::LetExprChirho);

        self.bump_chirho(); // let
        self.eat_trivia_chirho();

        // Parse the let-bindings layout block
        self.parse_layout_block_chirho();
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::InChirho) {
            self.bump_chirho(); // in
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse an if expression: if expr then expr else expr
    /// Also handles MultiWayIf: if | g1 -> e1 | g2 -> e2
    fn parse_if_expr_chirho(&mut self) {
        // Peek: if the token after `if` (skipping trivia) is `|`, parse as multi-way if
        if self.peek_after_if_is_pipe_chirho() {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::MultiWayIfExprChirho);
            self.bump_chirho(); // if
            self.eat_trivia_chirho();
            // Parse guards: | cond -> expr
            // Only consume `|` if lookahead confirms it is followed by
            // `expr ->` (a MultiWayIf alt).  A `|` followed by `expr =`
            // or `expr ,` belongs to an outer guard equation.
            while self.at_chirho(RawTokenKindChirho::PipeChirho)
                && self.scan_multiway_if_alt_has_arrow_chirho()
            {
                self.bump_chirho(); // |
                self.eat_trivia_chirho();
                // Parse guard condition
                self.parse_expr_chirho();
                self.eat_trivia_chirho();
                // Expect -> ; if missing, this `|` is not a MultiWayIf alt
                // (it may be an outer guard pipe that was consumed by mistake).
                if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                } else {
                    // Not a MultiWayIf alt — stop consuming.  The parser
                    // already bumped the `|` and parsed the condition expr,
                    // so we leave them in the tree as-is and break.
                    break;
                }
                // Parse result expression
                self.parse_expr_chirho();
                self.eat_trivia_chirho();
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::IfExprChirho);

        self.bump_chirho(); // if
        self.eat_trivia_chirho();

        self.parse_expr_chirho();
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::ThenChirho) {
            self.bump_chirho(); // then
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
            self.eat_trivia_chirho();
        }

        if self.at_chirho(RawTokenKindChirho::ElseChirho) {
            self.bump_chirho(); // else
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Check if `if` is followed by `|` (multi-way if).
    fn peek_after_if_is_pipe_chirho(&self) -> bool {
        // Current token should be `if`. Look at the next non-trivia token.
        let mut i_chirho = self.pos_chirho + 1;
        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            if kind_chirho == RawTokenKindChirho::WhitespaceChirho
                || kind_chirho == RawTokenKindChirho::LineCommentChirho
                || kind_chirho == RawTokenKindChirho::BlockCommentChirho
                || kind_chirho == RawTokenKindChirho::VirtualLeftBraceChirho
                || kind_chirho == RawTokenKindChirho::VirtualSemicolonChirho
            {
                i_chirho += 1;
                continue;
            }
            return kind_chirho == RawTokenKindChirho::PipeChirho;
        }
        false
    }

    /// Parse a case expression: case expr of { alts }
    fn parse_case_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::CaseExprChirho);

        self.bump_chirho(); // case
        self.eat_trivia_chirho();

        self.parse_expr_chirho();
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::OfChirho) {
            self.bump_chirho(); // of
            self.eat_trivia_chirho();
            self.parse_case_alts_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse case alternatives layout block.
    fn parse_case_alts_chirho(&mut self) {
        if self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
            || self.at_chirho(RawTokenKindChirho::LeftBraceChirho)
        {
            self.bump_chirho(); // {
        }
        self.eat_trivia_chirho();

        loop {
            if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            {
                self.bump_chirho(); // }
                break;
            }
            if self.at_eof_chirho() {
                break;
            }
            if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
                continue;
            }

            let before_chirho = self.pos_chirho;
            self.parse_case_alt_chirho();
            self.eat_trivia_chirho();
            // Safety: if no progress, bump to avoid infinite loop
            if self.pos_chirho == before_chirho {
                if !self.at_eof_chirho() {
                    self.bump_chirho();
                } else {
                    break;
                }
            }
        }
    }

    /// Parse a single case alternative: pat -> expr or pat | guard -> expr
    fn parse_case_alt_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::CaseAltChirho);

        // Parse the pattern
        if self.can_start_apat_chirho() {
            self.parse_pat_chirho();
            self.eat_trivia_chirho();
        }

        // -> or guards
        if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
            self.bump_chirho(); // ->
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
        } else if self.at_chirho(RawTokenKindChirho::PipeChirho) {
            // Guarded case alt
            while self.at_chirho(RawTokenKindChirho::PipeChirho) {
                self.bump_chirho(); // |
                self.eat_trivia_chirho();
                self.parse_guard_expr_case_chirho();
                if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                    self.bump_chirho(); // ->
                    self.eat_trivia_chirho();
                    self.parse_expr_chirho();
                }
                self.eat_trivia_chirho();
            }
        }

        // Optional where in case alt
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.parse_where_block_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse guard qualifiers in case alt (between | and ->).
    fn parse_guard_expr_case_chirho(&mut self) {
        while !self.at_chirho(RawTokenKindChirho::RightArrowChirho)
            && !self.at_eof_chirho()
            && !self.at_decl_boundary_chirho()
        {
            let before_chirho = self.pos_chirho;
            self.eat_trivia_chirho();

            if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
                continue;
            }

            if self.at_chirho(RawTokenKindChirho::LetChirho) {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LetStmtChirho);
                self.bump_chirho(); // let
                self.eat_trivia_chirho();
                self.parse_layout_block_chirho();
                self.builder_chirho.finish_node_chirho();
                self.eat_trivia_chirho();
            } else if self.scan_for_case_guard_bind_arrow_chirho() {
                self.parse_pat_chirho();
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::LeftArrowChirho) {
                    self.bump_chirho(); // <-
                    self.eat_trivia_chirho();
                    self.parse_expr_chirho();
                    self.eat_trivia_chirho();
                }
            } else {
                self.parse_expr_chirho();
                self.eat_trivia_chirho();
            }

            if self.pos_chirho == before_chirho {
                break;
            }
        }
    }

    /// Scan ahead to see whether the current guarded case-alt qualifier
    /// contains a top-level `<-` before the next comma or `->`.
    fn scan_for_case_guard_bind_arrow_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho;
        let mut paren_depth_chirho = 0u32;
        let mut bracket_depth_chirho = 0u32;
        let mut brace_depth_chirho = 0u32;

        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            match kind_chirho {
                RawTokenKindChirho::LeftArrowChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return true;
                }
                RawTokenKindChirho::CommaChirho | RawTokenKindChirho::RightArrowChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return false;
                }
                RawTokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                RawTokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                RawTokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBraceChirho
                | RawTokenKindChirho::VirtualLeftBraceChirho => {
                    brace_depth_chirho += 1;
                }
                RawTokenKindChirho::RightBraceChirho
                | RawTokenKindChirho::VirtualRightBraceChirho => {
                    brace_depth_chirho = brace_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::PipeChirho if brace_depth_chirho == 0 => {
                    return false;
                }
                _ => {}
            }
            i_chirho += 1;
        }

        false
    }

    /// Parse a do expression: do { stmts }
    fn parse_do_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::DoExprChirho);

        self.bump_chirho(); // do
        self.eat_trivia_chirho();

        // Parse do stmts layout block
        if self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
            || self.at_chirho(RawTokenKindChirho::LeftBraceChirho)
        {
            self.bump_chirho(); // {

            loop {
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                    || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                {
                    self.bump_chirho(); // }
                    break;
                }
                if self.at_eof_chirho() {
                    break;
                }
                if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                    || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
                {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                    continue;
                }

                let before_chirho = self.pos_chirho;
                self.parse_do_stmt_chirho();
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    if !self.at_eof_chirho() {
                        self.bump_chirho();
                    } else {
                        break;
                    }
                }
            }
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a single do statement.
    fn parse_do_stmt_chirho(&mut self) {
        // Check for let statement
        if self.at_chirho(RawTokenKindChirho::LetChirho) {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::LetStmtChirho);
            self.bump_chirho(); // let
            self.eat_trivia_chirho();
            self.parse_layout_block_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Try to detect bind statement: pat <- expr
        // We use a heuristic: parse as expression, then check for <-
        let cp_chirho = self.builder_chirho.checkpoint_chirho();
        let saved_pos_chirho = self.pos_chirho;

        // Check if there's a <- by scanning ahead (without building tree)
        let has_bind_chirho = self.scan_for_bind_arrow_chirho();

        if has_bind_chirho {
            // It's a bind statement: pat <- expr
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::BindStmtChirho);
            // We need to re-parse the position for the pattern
            // Actually we haven't bumped yet, so just parse normally
            self.parse_pat_chirho();
            self.eat_trivia_chirho();

            if self.at_chirho(RawTokenKindChirho::LeftArrowChirho) {
                self.bump_chirho(); // <-
                self.eat_trivia_chirho();
                self.parse_expr_chirho();
            }
            self.builder_chirho.finish_node_chirho();
        } else {
            // Regular expression statement
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::DoStmtChirho);
            // Restore position since scan_for_bind_arrow didn't consume
            self.pos_chirho = saved_pos_chirho;
            self.parse_expr_chirho();
            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Scan ahead (without consuming) to check if there's a <- in the
    /// current statement.
    fn scan_for_bind_arrow_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho;
        let mut paren_depth_chirho = 0u32;
        let mut bracket_depth_chirho = 0u32;
        let mut brace_depth_chirho = 0u32;

        while i_chirho < self.tokens_chirho.len() {
            let k_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            match k_chirho {
                RawTokenKindChirho::LeftArrowChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return true;
                }
                RawTokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                RawTokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                RawTokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBraceChirho
                | RawTokenKindChirho::VirtualLeftBraceChirho => {
                    brace_depth_chirho += 1;
                }
                RawTokenKindChirho::RightBraceChirho
                | RawTokenKindChirho::VirtualRightBraceChirho => {
                    if brace_depth_chirho == 0 {
                        return false;
                    }
                    brace_depth_chirho = brace_depth_chirho.saturating_sub(1);
                }
                // Statement boundaries
                RawTokenKindChirho::VirtualSemicolonChirho
                | RawTokenKindChirho::SemicolonChirho
                | RawTokenKindChirho::EofChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return false;
                }
                _ => {}
            }
            i_chirho += 1;
        }
        false
    }

    /// Parse a function application: f x y z
    fn parse_fexp_chirho(&mut self) {
        if !self.can_start_aexp_chirho() {
            return;
        }

        let cp_chirho = self.builder_chirho.checkpoint_chirho();
        self.parse_aexp_chirho();
        self.eat_trivia_chirho();

        let mut count_chirho = 1u32;
        loop {
            // TypeApplications: @Type after an expression
            if self.at_chirho(RawTokenKindChirho::AtChirho) {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::TypeAppExprChirho);
                // Close any open AppExpr node first
                if count_chirho > 1 {
                    self.builder_chirho.finish_node_chirho(); // AppExpr
                    count_chirho = 1; // reset
                }
                self.bump_chirho(); // consume @
                self.eat_trivia_chirho();
                self.parse_atype_chirho();
                self.builder_chirho.finish_node_chirho(); // TypeAppExpr
                self.eat_trivia_chirho();
                continue;
            }

            if !self.can_start_aexp_chirho() {
                break;
            }
            let before_chirho = self.pos_chirho;
            if count_chirho == 1 {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::AppExprChirho);
            }
            count_chirho += 1;
            self.parse_aexp_chirho();
            self.eat_trivia_chirho();
            // Safety: break if no progress to avoid infinite loop
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        if count_chirho > 1 {
            self.builder_chirho.finish_node_chirho(); // AppExpr
        }

        // Record update: expr { field = val, ... }
        // Only explicit braces (not virtual layout braces)
        if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::RecordUpdateExprChirho);
            self.parse_record_expr_chirho();
            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Parse an atomic expression: literal, variable, constructor,
    /// parenthesized, tuple, list, section.
    fn parse_aexp_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::NameExprChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::TickChirho) => {
                self.parse_quoted_name_expr_chirho();
            }
            Some(RawTokenKindChirho::ConIdChirho) | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::NameExprChirho);
                self.bump_chirho();
                // Check for record construction: Con { field = val }
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    // Could be record construction — but in a CST we
                    // need to be careful. This might also be a layout
                    // thing. Only treat as record if it's an explicit brace.
                    self.parse_record_expr_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::IntLitChirho)
            | Some(RawTokenKindChirho::FloatLitChirho)
            | Some(RawTokenKindChirho::CharLitChirho)
            | Some(RawTokenKindChirho::StringLitChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LiteralExprChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            // Typed holes: `_` in expression position (GHC's TypedHoles)
            Some(RawTokenKindChirho::UnderscoreChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::NameExprChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftParenChirho) => {
                self.parse_paren_expr_chirho();
            }
            Some(RawTokenKindChirho::LeftBracketChirho) => {
                self.parse_list_expr_chirho();
            }
            // BlockArguments: block-like expressions valid as function arguments
            Some(RawTokenKindChirho::BackslashChirho) => {
                self.parse_lambda_chirho();
            }
            Some(RawTokenKindChirho::DoChirho) => {
                self.parse_do_expr_chirho();
            }
            Some(RawTokenKindChirho::CaseChirho) => {
                self.parse_case_expr_chirho();
            }
            Some(RawTokenKindChirho::IfChirho) => {
                self.parse_if_expr_chirho();
            }
            Some(RawTokenKindChirho::LetChirho) => {
                self.parse_let_expr_chirho();
            }
            // Template Haskell splice: $name or $(expr)
            Some(RawTokenKindChirho::ThSpliceChirho) => {
                self.parse_splice_expr_chirho();
            }
            // Template Haskell typed splice: $$name or $$(expr)
            Some(RawTokenKindChirho::ThTypedSpliceChirho) => {
                self.parse_typed_splice_expr_chirho();
            }
            // Template Haskell expression quotation: [| expr |] or [e| expr |]
            Some(RawTokenKindChirho::ThOpenExpQuoteChirho)
            | Some(RawTokenKindChirho::ThOpenExpExplicitQuoteChirho) => {
                self.parse_quote_expr_chirho();
            }
            // Template Haskell declaration quotation: [d| decls |]
            Some(RawTokenKindChirho::ThOpenDecQuoteChirho) => {
                self.parse_quote_decl_chirho();
            }
            // Template Haskell type quotation: [t| type |]
            Some(RawTokenKindChirho::ThOpenTypeQuoteChirho) => {
                self.parse_quote_type_chirho();
            }
            // Template Haskell pattern quotation: [p| pat |]
            Some(RawTokenKindChirho::ThOpenPatQuoteChirho) => {
                self.parse_quote_pat_chirho();
            }
            // Template Haskell typed expression quotation: [|| expr ||]
            Some(RawTokenKindChirho::ThOpenTypedExpQuoteChirho) => {
                self.parse_typed_quote_expr_chirho();
            }
            _ => {
                // Unexpected token — wrap in error
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                if !self.at_eof_chirho() {
                    self.bump_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
        }
    }

    fn parse_quoted_name_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::QuotedNameExprChirho);

        while self.at_chirho(RawTokenKindChirho::TickChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho)
            | Some(RawTokenKindChirho::ConIdChirho)
            | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                self.bump_chirho();
            }
            Some(RawTokenKindChirho::LeftParenChirho) => {
                self.bump_chirho();
                self.eat_trivia_chirho();
                while !self.at_chirho(RawTokenKindChirho::RightParenChirho) && !self.at_eof_chirho()
                {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                    self.bump_chirho();
                }
            }
            _ => {
                if !self.at_eof_chirho() {
                    self.bump_chirho();
                }
            }
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a parenthesized expression, tuple, section, or unit.
    fn parse_paren_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ParenExprChirho);

        self.bump_chirho(); // (
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            // Unit ()
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Detect bare operator section: (op) where op is an operator
        // symbol immediately followed by ).  Wrap the operator in a
        // NameExprChirho so it lowers to a VarChirho reference.
        if self.is_operator_section_chirho() {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::NameExprChirho);
            self.bump_chirho(); // the operator token
            self.builder_chirho.finish_node_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for left section: (op expr) — operator NOT immediately
        // followed by ')'.  Emit operator token and then the expression
        // inside the ParenExprChirho; the CST→AST lowerer will detect
        // the leading operator token and produce LeftSectionChirho.
        // IMPORTANT: exclude `-` (minus) because `(-e)` is always negation
        // in Haskell, never a section.
        let is_op_chirho = matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarSymChirho) | Some(RawTokenKindChirho::ConSymChirho)
        ) || self.current_kind_chirho()
            == Some(RawTokenKindChirho::QualifiedIdChirho)
            && qualified_name_is_operator_chirho(self.current_text_chirho());
        let is_minus_chirho = is_op_chirho && self.current_text_chirho() == "-";
        let is_backticked_left_section_chirho = self.is_backticked_left_section_start_chirho();
        if (is_op_chirho && !is_minus_chirho && !self.is_operator_section_chirho())
            || is_backticked_left_section_chirho
        {
            if is_backticked_left_section_chirho {
                self.bump_backticked_operator_chirho();
            } else {
                self.bump_chirho(); // the operator
            }
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // TupleSections: if current token is a comma, first position is a gap
        if !self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // Could be a right section: (expr op) or just (expr) or (expr, expr, ...)
            self.parse_expr_chirho();
            self.eat_trivia_chirho();

            // Check for right section: after expr, we have op followed by ')'
            let is_right_op_chirho = matches!(
                self.current_kind_chirho(),
                Some(RawTokenKindChirho::VarSymChirho) | Some(RawTokenKindChirho::ConSymChirho)
            ) || self.current_kind_chirho()
                == Some(RawTokenKindChirho::QualifiedIdChirho)
                && qualified_name_is_operator_chirho(self.current_text_chirho());
            let is_backticked_right_section_chirho = self.is_backticked_right_section_chirho();
            if is_right_op_chirho || is_backticked_right_section_chirho {
                // Look ahead past operator + trivia for ')'
                let is_right_section_chirho = if is_backticked_right_section_chirho {
                    true
                } else {
                    let mut look_chirho = self.pos_chirho + 1;
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
                };

                if is_right_section_chirho {
                    // Right section: emit the operator token; lowerer detects it
                    if is_backticked_right_section_chirho {
                        self.bump_backticked_operator_chirho();
                    } else {
                        self.bump_chirho(); // the operator
                    }
                    self.eat_trivia_chirho();
                    if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                        self.bump_chirho();
                    }
                    self.builder_chirho.finish_node_chirho();
                    return;
                }
            }
        }

        if self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // Tuple (possibly with sections/gaps)
            while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho(); // ,
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::RightParenChirho)
                    && !self.at_chirho(RawTokenKindChirho::CommaChirho)
                    && !self.at_eof_chirho()
                {
                    self.parse_expr_chirho();
                    self.eat_trivia_chirho();
                }
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a list expression, arithmetic sequence, or list comprehension.
    fn parse_list_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ListExprChirho);

        self.bump_chirho(); // [
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            // Empty list
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.parse_expr_chirho();
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::PipeChirho) {
            // List comprehension: [expr | quals]
            self.bump_chirho(); // |
            self.eat_trivia_chirho();
            // Parse qualifiers until ]
            // Each qualifier is either:
            //   - generator: pat <- expr
            //   - guard: expr
            //   - let: let binds
            // Separated by commas.
            while !self.at_chirho(RawTokenKindChirho::RightBracketChirho)
                && !self.at_eof_chirho()
                && !self.at_decl_boundary_chirho()
            {
                let before_chirho = self.pos_chirho;
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if !self.at_chirho(RawTokenKindChirho::RightBracketChirho)
                    && !self.at_eof_chirho()
                    && !self.at_decl_boundary_chirho()
                {
                    // Parse the first expression (could be a pattern for a generator)
                    self.parse_expr_chirho();
                    self.eat_trivia_chirho();
                    // If followed by <-, this is a generator: pat <- source
                    if self.at_chirho(RawTokenKindChirho::LeftArrowChirho) {
                        self.bump_chirho(); // <-
                        self.eat_trivia_chirho();
                        self.parse_expr_chirho();
                        self.eat_trivia_chirho();
                    }
                }
                if self.pos_chirho == before_chirho {
                    break;
                }
            }
        } else if self.at_chirho(RawTokenKindChirho::DotDotChirho) {
            // Arithmetic sequence: [a..], [a..b], [a,b..], [a,b..c]
            self.bump_chirho(); // ..
            self.eat_trivia_chirho();
            if !self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                self.parse_expr_chirho();
                self.eat_trivia_chirho();
            }
        } else if self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // List or arithmetic sequence with step
            self.bump_chirho(); // ,
            self.eat_trivia_chirho();

            if !self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                self.parse_expr_chirho();
                self.eat_trivia_chirho();
            }

            if self.at_chirho(RawTokenKindChirho::DotDotChirho) {
                // [a, b .. c]
                self.bump_chirho();
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                    self.parse_expr_chirho();
                    self.eat_trivia_chirho();
                }
            } else {
                // Regular list: more elements
                while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                    self.bump_chirho(); // ,
                    self.eat_trivia_chirho();
                    if !self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                        self.parse_expr_chirho();
                        self.eat_trivia_chirho();
                    }
                }
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse record expression fields: { field = expr, ... }
    fn parse_record_expr_chirho(&mut self) {
        // We're inside a NameExpr that started with ConId.
        // Just eat the record fields.
        self.bump_chirho(); // {
        self.eat_trivia_chirho();

        while !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            && !self.at_eof_chirho()
            && !self.at_decl_boundary_chirho()
        {
            let outer_before_chirho = self.pos_chirho;
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::FieldAssignChirho);

            // Parse field name
            if self.at_chirho(RawTokenKindChirho::VarIdChirho)
                || self.at_chirho(RawTokenKindChirho::ConIdChirho)
            {
                self.bump_chirho(); // field name
                self.eat_trivia_chirho();
            }

            // Parse = and value expression
            if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
                self.bump_chirho(); // =
                self.eat_trivia_chirho();
                // Parse the value as a proper expression so lambdas,
                // let-expressions etc. get correct AST nodes.
                if self.can_start_aexp_chirho()
                    || self.at_chirho(RawTokenKindChirho::BackslashChirho)
                    || self.at_chirho(RawTokenKindChirho::LetChirho)
                    || self.at_chirho(RawTokenKindChirho::IfChirho)
                    || self.at_chirho(RawTokenKindChirho::CaseChirho)
                    || self.at_chirho(RawTokenKindChirho::DoChirho)
                {
                    self.parse_expr_chirho();
                    self.eat_trivia_chirho();
                }
            } else {
                // NamedFieldPuns or other tokens — consume until , or }
                while !self.at_chirho(RawTokenKindChirho::CommaChirho)
                    && !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                    && !self.at_eof_chirho()
                    && !self.at_decl_boundary_chirho()
                {
                    self.eat_trivia_chirho();
                    if !self.at_chirho(RawTokenKindChirho::CommaChirho)
                        && !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                        && !self.at_decl_boundary_chirho()
                        && !self.at_eof_chirho()
                    {
                        self.bump_chirho();
                    }
                }
            }

            self.builder_chirho.finish_node_chirho();

            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            if self.pos_chirho == outer_before_chirho {
                break;
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightBraceChirho) {
            self.bump_chirho();
        }
    }

    /// Parse a layout block (shared by let, do, where, etc.)
    fn parse_layout_block_chirho(&mut self) {
        if self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
            || self.at_chirho(RawTokenKindChirho::LeftBraceChirho)
        {
            self.bump_chirho(); // {
            self.eat_trivia_chirho();

            loop {
                if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                    || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                {
                    self.bump_chirho(); // }
                    break;
                }
                if self.at_eof_chirho() {
                    break;
                }
                if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                    || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
                {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                    continue;
                }
                let before_chirho = self.pos_chirho;
                self.parse_decl_chirho();
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    if !self.at_eof_chirho() {
                        self.bump_chirho();
                    } else {
                        break;
                    }
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Pattern parsing
    // -----------------------------------------------------------------------

    /// Parse a pattern. Handles infix constructor patterns (p1 :+: p2).
    fn parse_pat_chirho(&mut self) {
        let cp_chirho = self.builder_chirho.checkpoint_chirho();

        self.parse_lpat_chirho();
        self.eat_trivia_chirho();

        // ScopedTypeVariables: pattern type annotation (p :: Type)
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::SigPatChirho);
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // the type annotation
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for infix constructor pattern
        if self.at_chirho(RawTokenKindChirho::ConSymChirho)
            || self.at_chirho(RawTokenKindChirho::BacktickChirho)
            || self.current_kind_chirho() == Some(RawTokenKindChirho::QualifiedIdChirho)
                && qualified_name_is_consym_chirho(self.current_text_chirho())
        {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::InfixConPatChirho);

            if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                self.bump_chirho(); // `
                self.eat_trivia_chirho();
                if !self.at_eof_chirho() {
                    self.bump_chirho(); // name
                }
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                    self.bump_chirho(); // `
                }
            } else {
                self.bump_chirho(); // ConSym or qualified constructor symbol
            }
            self.eat_trivia_chirho();

            // Use parse_pat_chirho (not parse_lpat_chirho) so that chained
            // infix constructor patterns like `x:y:zs` are right-associative:
            // parsed as `x:(y:zs)` instead of `(x:y):zs`.
            self.parse_pat_chirho();
            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Parse a function argument pattern: like `parse_lpat_chirho` but does
    /// NOT greedily consume arguments after a constructor name. This ensures
    /// `f True True = ...` is parsed as two separate patterns, not
    /// `True applied_to True`. Handles as-patterns, negated literals,
    /// bang/lazy patterns, and parenthesized patterns.
    fn parse_fun_arg_pat_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                // Variable or as-pattern
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::VarPatChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::AtChirho) {
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::AsPatChirho);
                    self.bump_chirho(); // @
                    self.eat_trivia_chirho();
                    self.parse_apat_chirho();
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::ConIdChirho) | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                // Bare constructor pattern — NO argument consumption
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.bump_chirho(); // ConId
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    // Record pattern
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::RecordPatChirho);
                    self.parse_record_pat_fields_chirho();
                    self.builder_chirho.finish_node_chirho();
                } else {
                    // Nullary constructor pattern (no arguments)
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::ConPatChirho);
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::VarSymChirho) if self.current_text_chirho() == "-" => {
                // Negated literal pattern — delegate to parse_apat_chirho
                // which handles the NegPat node construction
                self.parse_apat_chirho();
            }
            Some(RawTokenKindChirho::TildeChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LazyPatChirho);
                self.bump_chirho(); // ~
                self.eat_trivia_chirho();
                self.parse_fun_arg_pat_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::VarSymChirho) if self.current_text_chirho() == "!" => {
                // Bang pattern
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::BangPatChirho);
                self.bump_chirho(); // !
                self.eat_trivia_chirho();
                self.parse_fun_arg_pat_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            _ => {
                // Fallback to apat (parenthesized, tuple, list, literal, wildcard)
                self.parse_apat_chirho();
            }
        }
    }

    /// Parse a "left pattern": constructor application, negation, as,
    /// lazy (~pat), bang (!pat).
    fn parse_lpat_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                // Could be as-pattern: x@pat or just variable
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::VarPatChirho);
                self.bump_chirho(); // var name
                self.builder_chirho.finish_node_chirho();
                self.eat_trivia_chirho();

                if self.at_chirho(RawTokenKindChirho::AtChirho) {
                    // As pattern: x@pat
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::AsPatChirho);
                    self.bump_chirho(); // @
                    self.eat_trivia_chirho();
                    self.parse_apat_chirho();
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::ConIdChirho) | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                // Constructor pattern possibly with argument patterns
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.bump_chirho(); // ConId
                self.eat_trivia_chirho();

                // Check for record pattern
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::RecordPatChirho);
                    self.parse_record_pat_fields_chirho();
                    self.builder_chirho.finish_node_chirho();
                } else {
                    // Constructor pattern (with or without argument patterns)
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::ConPatChirho);
                    while self.can_start_apat_chirho()
                        || self.at_chirho(RawTokenKindChirho::AtChirho)
                    {
                        let before_chirho = self.pos_chirho;
                        // TyAppPat: @Type in constructor patterns
                        if self.at_chirho(RawTokenKindChirho::AtChirho) {
                            self.bump_chirho(); // @
                            self.eat_trivia_chirho();
                            // Parse the type argument (skip it for now —
                            // the type is used for type variable binding
                            // but doesn't affect the pattern structure)
                            if self.can_start_atype_chirho() {
                                self.parse_atype_chirho();
                            }
                            self.eat_trivia_chirho();
                            continue;
                        }
                        // Constructor arguments are atomic/function-argument
                        // patterns, and this path must preserve nested
                        // as-patterns like `BQ bq@(BQB _ lo)` instead of
                        // truncating them to `BQ bq`.
                        self.parse_fun_arg_pat_chirho();
                        self.eat_trivia_chirho();
                        if self.pos_chirho == before_chirho {
                            break;
                        }
                    }
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::TildeChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LazyPatChirho);
                self.bump_chirho(); // ~
                self.eat_trivia_chirho();
                self.parse_lpat_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::VarSymChirho) if self.current_text_chirho() == "!" => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::BangPatChirho);
                self.bump_chirho(); // !
                self.eat_trivia_chirho();
                self.parse_lpat_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::VarSymChirho) if self.current_text_chirho() == "-" => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::NegPatChirho);
                self.bump_chirho(); // -
                self.eat_trivia_chirho();
                self.parse_apat_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            _ => self.parse_apat_chirho(),
        }
    }

    /// Parse an atomic pattern: variable, constructor, literal, wildcard,
    /// parenthesized, tuple, list.
    fn parse_apat_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::VarPatChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::ConIdChirho) | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.bump_chirho();
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::RecordPatChirho);
                    self.parse_record_pat_fields_chirho();
                    self.builder_chirho.finish_node_chirho();
                } else {
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::ConPatChirho);
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::IntLitChirho)
            | Some(RawTokenKindChirho::FloatLitChirho)
            | Some(RawTokenKindChirho::CharLitChirho)
            | Some(RawTokenKindChirho::StringLitChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LitPatChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::UnderscoreChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::WildcardPatChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftParenChirho) => {
                self.parse_paren_pat_chirho();
            }
            Some(RawTokenKindChirho::LeftBracketChirho) => {
                self.parse_list_pat_chirho();
            }
            _ => {
                // Unexpected — wrap in error
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                if !self.at_eof_chirho() {
                    self.bump_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
        }
    }

    /// Parse parenthesized, tuple, or view pattern.
    /// View pattern: `(expr -> pat)` requires ViewPatterns extension.
    fn parse_paren_pat_chirho(&mut self) {
        // Check if this is a view pattern: scan ahead for `->` at depth 0
        let is_view_chirho = self.scan_for_view_arrow_chirho();
        if is_view_chirho {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ViewPatChirho);
            self.bump_chirho(); // (
            self.eat_trivia_chirho();
            // Parse the view expression (everything up to `->`)
            self.parse_expr_chirho();
            self.eat_trivia_chirho();
            // Consume the `->` (may be RightArrowChirho or VarSymChirho)
            if self.current_kind_chirho() == Some(RawTokenKindChirho::RightArrowChirho)
                || (self.current_kind_chirho() == Some(RawTokenKindChirho::VarSymChirho)
                    && self.current_text_chirho() == "->")
            {
                self.bump_chirho(); // ->
            }
            self.eat_trivia_chirho();
            // Parse the result pattern
            self.parse_pat_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ParenPatChirho);

        self.bump_chirho(); // (
        self.eat_trivia_chirho();

        let is_unboxed_tuple_pat_chirho =
            self.at_chirho(RawTokenKindChirho::VarSymChirho) && self.current_text_chirho() == "#";
        if is_unboxed_tuple_pat_chirho {
            self.bump_chirho(); // leading #
            self.eat_trivia_chirho();

            if !self.at_chirho(RawTokenKindChirho::VarSymChirho)
                || self.current_text_chirho() != "#"
            {
                self.parse_pat_chirho();
                self.eat_trivia_chirho();

                while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                    self.bump_chirho(); // ,
                    self.eat_trivia_chirho();
                    if !(self.at_chirho(RawTokenKindChirho::VarSymChirho)
                        && self.current_text_chirho() == "#")
                    {
                        self.parse_pat_chirho();
                        self.eat_trivia_chirho();
                    }
                }
            }

            if self.at_chirho(RawTokenKindChirho::VarSymChirho) && self.current_text_chirho() == "#"
            {
                self.bump_chirho(); // trailing #
                self.eat_trivia_chirho();
            }
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            // Unit pattern ()
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        if matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarSymChirho) | Some(RawTokenKindChirho::ConSymChirho)
        ) {
            let mut lookahead_idx_chirho = self.pos_chirho + 1;
            while lookahead_idx_chirho < self.tokens_chirho.len()
                && self.tokens_chirho[lookahead_idx_chirho]
                    .kind_chirho
                    .is_trivia_chirho()
            {
                lookahead_idx_chirho += 1;
            }
            if lookahead_idx_chirho < self.tokens_chirho.len()
                && self.tokens_chirho[lookahead_idx_chirho].kind_chirho
                    == RawTokenKindChirho::RightParenChirho
            {
                let pat_kind_chirho = if self.at_chirho(RawTokenKindChirho::VarSymChirho) {
                    SyntaxKindChirho::VarPatChirho
                } else {
                    SyntaxKindChirho::ConPatChirho
                };
                self.builder_chirho.start_node_chirho(pat_kind_chirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
                self.eat_trivia_chirho();
                self.bump_chirho(); // )
                self.builder_chirho.finish_node_chirho();
                return;
            }
        }

        self.parse_pat_chirho();
        self.eat_trivia_chirho();

        // Type-annotated pattern: (pat :: Type)
        // parse_pat_chirho already handled :: and produced a SigPatChirho
        // node inside this ParenPatChirho. Just close the paren.
        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }
        // Legacy fallback: if :: appears here (shouldn't with new parse_pat),
        // consume the type annotation tokens until closing )
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
            self.parse_type_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        if self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // Tuple pattern
            while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho(); // ,
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                    self.parse_pat_chirho();
                    self.eat_trivia_chirho();
                }
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Scan ahead from the current position to see if this is a view pattern `(expr -> pat)`.
    /// Returns true if we find a `->` at parenthesis depth 0 inside the outer parens.
    fn scan_for_view_arrow_chirho(&self) -> bool {
        if self.current_kind_chirho() != Some(RawTokenKindChirho::LeftParenChirho) {
            return false;
        }
        let mut pos_chirho = self.pos_chirho + 1;
        let mut depth_chirho: i32 = 1;
        // Track whether we've seen `::` at depth 1 — if so, any `->` after
        // it is a function arrow in a type annotation, not a view pattern arrow.
        let mut in_type_annotation_chirho = false;
        while pos_chirho < self.tokens_chirho.len() {
            let tok_chirho = &self.tokens_chirho[pos_chirho];
            match tok_chirho.kind_chirho {
                RawTokenKindChirho::LeftParenChirho | RawTokenKindChirho::LeftBracketChirho => {
                    depth_chirho += 1;
                }
                RawTokenKindChirho::RightParenChirho | RawTokenKindChirho::RightBracketChirho => {
                    depth_chirho -= 1;
                    if depth_chirho == 0 {
                        return false; // reached closing paren without finding ->
                    }
                }
                RawTokenKindChirho::ColonColonChirho if depth_chirho == 1 => {
                    // `::` at depth 1 means pattern type annotation — any
                    // subsequent `->` at this depth is part of the type, not
                    // a view pattern arrow.
                    in_type_annotation_chirho = true;
                }
                RawTokenKindChirho::RightArrowChirho if depth_chirho == 1 => {
                    if !in_type_annotation_chirho {
                        return true;
                    }
                }
                RawTokenKindChirho::VarSymChirho if depth_chirho == 1 => {
                    let text_chirho = self.token_text_chirho(tok_chirho);
                    if text_chirho == "->" && !in_type_annotation_chirho {
                        return true;
                    }
                }
                _ => {}
            }
            pos_chirho += 1;
        }
        false
    }

    /// Parse list pattern.
    fn parse_list_pat_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ListPatChirho);

        self.bump_chirho(); // [
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.parse_pat_chirho();
        self.eat_trivia_chirho();

        while self.at_chirho(RawTokenKindChirho::CommaChirho) {
            self.bump_chirho(); // ,
            self.eat_trivia_chirho();
            self.parse_pat_chirho();
            self.eat_trivia_chirho();
        }

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse record pattern fields: { field = pat, ... }
    fn parse_record_pat_fields_chirho(&mut self) {
        self.bump_chirho(); // {
        self.eat_trivia_chirho();

        while !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            && !self.at_eof_chirho()
            && !self.at_decl_boundary_chirho()
        {
            let before_chirho = self.pos_chirho;
            while !self.at_chirho(RawTokenKindChirho::CommaChirho)
                && !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                && !self.at_eof_chirho()
                && !self.at_decl_boundary_chirho()
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightBraceChirho) {
            self.bump_chirho();
        }
    }

    // -----------------------------------------------------------------------
    // Template Haskell — splice and quotation parsing
    // -----------------------------------------------------------------------

    /// Parse a splice expression: `$name` or `$(expr)`.
    /// The lexer has already produced a `ThSpliceChirho` token for the `$`.
    /// What follows is either a variable name or a parenthesized expression.
    fn parse_splice_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::SpliceExprChirho);
        self.bump_chirho(); // consume ThSpliceChirho ($)
        self.eat_trivia_chirho();

        // Either $name (VarId) or $(expr)
        if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            // $(expr) — parse the parenthesized expression
            self.bump_chirho(); // (
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho(); // )
            }
        } else if self.at_chirho(RawTokenKindChirho::VarIdChirho)
            || self.at_chirho(RawTokenKindChirho::ConIdChirho)
            || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
        {
            // $name — just consume the identifier
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::NameExprChirho);
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a typed splice expression: `$$name` or `$$(expr)`.
    fn parse_typed_splice_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypedSpliceExprChirho);
        self.bump_chirho(); // consume ThTypedSpliceChirho ($$)
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            self.bump_chirho(); // (
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho(); // )
            }
        } else if self.at_chirho(RawTokenKindChirho::VarIdChirho)
            || self.at_chirho(RawTokenKindChirho::ConIdChirho)
            || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
        {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::NameExprChirho);
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a top-level splice declaration: `$(expr)` or `$name` at declaration level.
    fn parse_splice_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::SpliceDeclChirho);
        self.bump_chirho(); // consume ThSpliceChirho ($)
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            self.bump_chirho(); // (
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho(); // )
            }
        } else if self.at_chirho(RawTokenKindChirho::VarIdChirho)
            || self.at_chirho(RawTokenKindChirho::ConIdChirho)
            || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
        {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::NameExprChirho);
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse an expression quotation: `[| expr |]` or `[e| expr |]`.
    fn parse_quote_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::QuoteExprChirho);
        self.bump_chirho(); // consume ThOpenExpQuoteChirho or ThOpenExpExplicitQuoteChirho
        self.eat_trivia_chirho();

        // Parse the inner expression until |]
        if !self.at_chirho(RawTokenKindChirho::ThCloseQuoteChirho) && !self.at_eof_chirho() {
            self.parse_expr_chirho();
            self.eat_trivia_chirho();
        }

        if self.at_chirho(RawTokenKindChirho::ThCloseQuoteChirho) {
            self.bump_chirho(); // consume |]
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a declaration quotation: `[d| decls |]`.
    fn parse_quote_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::QuoteDeclChirho);
        self.bump_chirho(); // consume ThOpenDecQuoteChirho
        self.eat_trivia_chirho();

        // Parse declarations inside the quote until |]
        while !self.at_chirho(RawTokenKindChirho::ThCloseQuoteChirho) && !self.at_eof_chirho() {
            let before_chirho = self.pos_chirho;
            self.eat_trivia_chirho();
            // Skip layout separators
            if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    break;
                }
                continue;
            }
            if self.at_chirho(RawTokenKindChirho::ThCloseQuoteChirho) || self.at_eof_chirho() {
                break;
            }
            self.parse_decl_chirho();
            self.eat_trivia_chirho();
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        if self.at_chirho(RawTokenKindChirho::ThCloseQuoteChirho) {
            self.bump_chirho(); // consume |]
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a type quotation: `[t| type |]`.
    fn parse_quote_type_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::QuoteTypeChirho);
        self.bump_chirho(); // consume ThOpenTypeQuoteChirho
        self.eat_trivia_chirho();

        // Parse the inner type until |]
        if !self.at_chirho(RawTokenKindChirho::ThCloseQuoteChirho) && !self.at_eof_chirho() {
            self.parse_type_chirho();
            self.eat_trivia_chirho();
        }

        if self.at_chirho(RawTokenKindChirho::ThCloseQuoteChirho) {
            self.bump_chirho(); // consume |]
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a pattern quotation: `[p| pat |]`.
    fn parse_quote_pat_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::QuotePatChirho);
        self.bump_chirho(); // consume ThOpenPatQuoteChirho
        self.eat_trivia_chirho();

        // Parse the inner pattern until |]
        if !self.at_chirho(RawTokenKindChirho::ThCloseQuoteChirho) && !self.at_eof_chirho() {
            self.parse_pat_chirho();
            self.eat_trivia_chirho();
        }

        if self.at_chirho(RawTokenKindChirho::ThCloseQuoteChirho) {
            self.bump_chirho(); // consume |]
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a typed expression quotation: `[|| expr ||]`.
    fn parse_typed_quote_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypedQuoteExprChirho);
        self.bump_chirho(); // consume ThOpenTypedExpQuoteChirho
        self.eat_trivia_chirho();

        // Parse the inner expression until ||]
        if !self.at_chirho(RawTokenKindChirho::ThCloseTypedQuoteChirho) && !self.at_eof_chirho() {
            self.parse_expr_chirho();
            self.eat_trivia_chirho();
        }

        if self.at_chirho(RawTokenKindChirho::ThCloseTypedQuoteChirho) {
            self.bump_chirho(); // consume ||]
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Predicates — what can start various constructs
    // -----------------------------------------------------------------------

    /// Can the current token start an atomic type?
    fn can_start_atype_chirho(&self) -> bool {
        self.can_start_atype_idx_chirho(self.pos_chirho)
    }

    fn can_start_atype_idx_chirho(&self, idx_chirho: usize) -> bool {
        matches!(
            self.tokens_chirho
                .get(idx_chirho)
                .map(|token_chirho| token_chirho.kind_chirho),
            Some(RawTokenKindChirho::VarIdChirho)
                | Some(RawTokenKindChirho::ConIdChirho)
                | Some(RawTokenKindChirho::QualifiedIdChirho)
                | Some(RawTokenKindChirho::LeftParenChirho)
                | Some(RawTokenKindChirho::LeftBracketChirho)
                | Some(RawTokenKindChirho::TickChirho)
                // PartialTypeSignatures: `_` as a wildcard type
                | Some(RawTokenKindChirho::UnderscoreChirho)
                // DataKinds: type-level literals
                | Some(RawTokenKindChirho::IntLitChirho)
                | Some(RawTokenKindChirho::StringLitChirho)
                | Some(RawTokenKindChirho::CharLitChirho)
        )
    }

    /// Is the current token a `!` strictness annotation prefix?
    fn at_strict_prefix_chirho(&self) -> bool {
        self.current_kind_chirho() == Some(RawTokenKindChirho::VarSymChirho)
            && self.current_text_chirho() == "!"
    }

    /// Can the current token start an atomic expression?
    fn can_start_aexp_chirho(&self) -> bool {
        matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarIdChirho)
                | Some(RawTokenKindChirho::ConIdChirho)
                | Some(RawTokenKindChirho::QualifiedIdChirho)
                | Some(RawTokenKindChirho::TickChirho)
                | Some(RawTokenKindChirho::IntLitChirho)
                | Some(RawTokenKindChirho::FloatLitChirho)
                | Some(RawTokenKindChirho::CharLitChirho)
                | Some(RawTokenKindChirho::StringLitChirho)
                | Some(RawTokenKindChirho::LeftParenChirho)
                | Some(RawTokenKindChirho::LeftBracketChirho)
                // BlockArguments: block-like expressions as function arguments
                | Some(RawTokenKindChirho::BackslashChirho)
                | Some(RawTokenKindChirho::DoChirho)
                | Some(RawTokenKindChirho::CaseChirho)
                | Some(RawTokenKindChirho::IfChirho)
                | Some(RawTokenKindChirho::LetChirho)
                // Template Haskell splice and quotation forms
                | Some(RawTokenKindChirho::ThSpliceChirho)
                | Some(RawTokenKindChirho::ThTypedSpliceChirho)
                | Some(RawTokenKindChirho::ThOpenExpQuoteChirho)
                | Some(RawTokenKindChirho::ThOpenExpExplicitQuoteChirho)
                | Some(RawTokenKindChirho::ThOpenDecQuoteChirho)
                | Some(RawTokenKindChirho::ThOpenTypeQuoteChirho)
                | Some(RawTokenKindChirho::ThOpenPatQuoteChirho)
                | Some(RawTokenKindChirho::ThOpenTypedExpQuoteChirho)
                // TypedHoles: `_` in expression position
                | Some(RawTokenKindChirho::UnderscoreChirho)
        ) && !self.current_is_qualified_operator_chirho()
    }

    /// Can the current token start an atomic pattern?
    fn can_start_apat_chirho(&self) -> bool {
        matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarIdChirho)
                | Some(RawTokenKindChirho::ConIdChirho)
                | Some(RawTokenKindChirho::QualifiedIdChirho)
                | Some(RawTokenKindChirho::IntLitChirho)
                | Some(RawTokenKindChirho::FloatLitChirho)
                | Some(RawTokenKindChirho::CharLitChirho)
                | Some(RawTokenKindChirho::StringLitChirho)
                | Some(RawTokenKindChirho::UnderscoreChirho)
                | Some(RawTokenKindChirho::LeftParenChirho)
                | Some(RawTokenKindChirho::LeftBracketChirho)
                | Some(RawTokenKindChirho::TildeChirho)
        )
    }

    fn can_start_fun_arg_pat_chirho(&self) -> bool {
        self.can_start_fun_arg_pat_idx_chirho(self.pos_chirho)
    }

    fn can_start_fun_arg_pat_idx_chirho(&self, idx_chirho: usize) -> bool {
        matches!(
            self.tokens_chirho
                .get(idx_chirho)
                .map(|token_chirho| token_chirho.kind_chirho),
            Some(RawTokenKindChirho::VarIdChirho)
                | Some(RawTokenKindChirho::ConIdChirho)
                | Some(RawTokenKindChirho::QualifiedIdChirho)
                | Some(RawTokenKindChirho::IntLitChirho)
                | Some(RawTokenKindChirho::FloatLitChirho)
                | Some(RawTokenKindChirho::CharLitChirho)
                | Some(RawTokenKindChirho::StringLitChirho)
                | Some(RawTokenKindChirho::UnderscoreChirho)
                | Some(RawTokenKindChirho::LeftParenChirho)
                | Some(RawTokenKindChirho::LeftBracketChirho)
                | Some(RawTokenKindChirho::TildeChirho)
                | Some(RawTokenKindChirho::VarSymChirho)
        ) && !self
            .tokens_chirho
            .get(idx_chirho)
            .is_some_and(|token_chirho| {
                token_chirho.kind_chirho == RawTokenKindChirho::VarSymChirho
                    && self.token_text_chirho(token_chirho) != "-"
                    && self.token_text_chirho(token_chirho) != "!"
            })
    }

    /// Is the current token an infix operator?
    fn at_infix_op_chirho(&self) -> bool {
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

    fn current_is_qualified_operator_chirho(&self) -> bool {
        self.current_kind_chirho() == Some(RawTokenKindChirho::QualifiedIdChirho)
            && qualified_name_is_operator_chirho(self.current_text_chirho())
    }

    /// Is the current token at a declaration boundary?
    fn at_decl_boundary_chirho(&self) -> bool {
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
            )
    }

    // -----------------------------------------------------------------------
    // Core utility methods
    // -----------------------------------------------------------------------

    fn current_chirho(&self) -> Option<&RawTokenChirho> {
        self.tokens_chirho.get(self.pos_chirho)
    }

    fn current_kind_chirho(&self) -> Option<RawTokenKindChirho> {
        self.current_chirho().map(|t_chirho| t_chirho.kind_chirho)
    }

    fn at_chirho(&self, kind_chirho: RawTokenKindChirho) -> bool {
        self.current_kind_chirho() == Some(kind_chirho)
    }

    fn at_eof_chirho(&self) -> bool {
        self.pos_chirho >= self.tokens_chirho.len() || self.at_chirho(RawTokenKindChirho::EofChirho)
    }

    /// Check if current token is `.` (VarSym with text ".").
    fn at_dot_chirho(&self) -> bool {
        self.at_chirho(RawTokenKindChirho::VarSymChirho) && self.current_text_chirho() == "."
    }

    /// Check if current token is a VarId with specific text.
    fn at_varid_text_chirho(&self, text_chirho: &str) -> bool {
        if !self.at_chirho(RawTokenKindChirho::VarIdChirho) {
            return false;
        }
        self.current_text_chirho() == text_chirho
    }

    /// Check if the current token is a VarSym with the given text.
    fn at_varsym_chirho(&self, text_chirho: &str) -> bool {
        if !self.at_chirho(RawTokenKindChirho::VarSymChirho) {
            return false;
        }
        self.current_text_chirho() == text_chirho
    }

    /// Check if the current token is an operator symbol and the next
    /// non-trivia token is `)`, indicating a bare operator section `(op)`.
    fn is_operator_section_chirho(&self) -> bool {
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

    fn is_backticked_left_section_start_chirho(&self) -> bool {
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

    fn is_backticked_right_section_chirho(&self) -> bool {
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

    fn bump_backticked_operator_chirho(&mut self) {
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

    fn current_text_chirho(&self) -> &str {
        if let Some(tok_chirho) = self.current_chirho() {
            self.token_text_chirho(tok_chirho)
        } else {
            ""
        }
    }

    /// Advance one token, adding it to the current green node.
    fn bump_chirho(&mut self) {
        if let Some(tok_chirho) = self.current_chirho().copied() {
            let text_chirho = self.token_text_chirho(&tok_chirho);
            let kind_chirho = map_token_kind_chirho(tok_chirho.kind_chirho, text_chirho);
            self.builder_chirho.token_chirho(kind_chirho, text_chirho);
            self.pos_chirho += 1;
        }
    }

    /// Safely extract the source text for a token, handling multi-byte UTF-8.
    fn token_text_chirho(&self, tok_chirho: &RawTokenChirho) -> &'src str {
        let start_chirho = tok_chirho.span_chirho.start_chirho().as_usize_chirho();
        let end_chirho = tok_chirho.span_chirho.end_chirho().as_usize_chirho();
        self.source_chirho
            .get(start_chirho..end_chirho)
            .unwrap_or("")
    }

    /// Eat all trivia tokens (whitespace, comments), adding them to the tree.
    fn eat_trivia_chirho(&mut self) {
        while let Some(kind_chirho) = self.current_kind_chirho() {
            if kind_chirho.is_trivia_chirho() {
                self.bump_chirho();
            } else {
                break;
            }
        }
    }

    /// Expect a specific token kind, bump it, or create an error node.
    fn expect_chirho(&mut self, kind_chirho: RawTokenKindChirho) {
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
    fn eat_until_any_chirho(&mut self, stops_chirho: &[RawTokenKindChirho]) {
        while let Some(kind_chirho) = self.current_kind_chirho() {
            if stops_chirho.contains(&kind_chirho) || kind_chirho == RawTokenKindChirho::EofChirho {
                break;
            }
            self.bump_chirho();
        }
    }

    /// Eat tokens until the end of the current declaration.
    fn eat_until_decl_end_chirho(&mut self) {
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
            RawTokenKindChirho::SemicolonChirho,
            RawTokenKindChirho::RightBraceChirho,
        ]);
    }

    /// Lookahead: is this a type signature? (name :: ...)
    fn is_type_sig_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho;

        loop {
            // Skip one name — could be `(+!)` for operator type sigs
            if i_chirho < self.tokens_chirho.len() {
                let k_chirho = self.tokens_chirho[i_chirho].kind_chirho;
                if k_chirho == RawTokenKindChirho::VarIdChirho
                    || k_chirho == RawTokenKindChirho::ConIdChirho
                {
                    i_chirho += 1;
                } else if k_chirho == RawTokenKindChirho::LeftParenChirho {
                    // Operator in parens: (+!)
                    i_chirho += 1;
                    while i_chirho < self.tokens_chirho.len()
                        && self.tokens_chirho[i_chirho].kind_chirho
                            != RawTokenKindChirho::RightParenChirho
                    {
                        i_chirho += 1;
                    }
                    if i_chirho < self.tokens_chirho.len() {
                        i_chirho += 1; // skip )
                    }
                }
            }

            // Skip trivia after the current name.
            while i_chirho < self.tokens_chirho.len()
                && self.tokens_chirho[i_chirho].kind_chirho.is_trivia_chirho()
            {
                i_chirho += 1;
            }

            if i_chirho < self.tokens_chirho.len()
                && self.tokens_chirho[i_chirho].kind_chirho == RawTokenKindChirho::CommaChirho
            {
                i_chirho += 1;
                while i_chirho < self.tokens_chirho.len()
                    && self.tokens_chirho[i_chirho].kind_chirho.is_trivia_chirho()
                {
                    i_chirho += 1;
                }
                continue;
            }
            break;
        }

        // Check for ::
        i_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[i_chirho].kind_chirho == RawTokenKindChirho::ColonColonChirho
    }
}

// ---------------------------------------------------------------------------
// Convenience function
// ---------------------------------------------------------------------------

/// Parse Haskell source text into a lossless CST (green tree).
pub fn parse_to_cst_chirho(
    source_chirho: &str,
    file_id_chirho: FileIdChirho,
) -> Arc<GreenNodeChirho> {
    let parser_chirho = ParserChirho::new_chirho(source_chirho, file_id_chirho);
    parser_chirho.parse_chirho()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_syntax_chirho::green_chirho::GreenElementChirho;

    fn parse_chirho(source_chirho: &str) -> Arc<GreenNodeChirho> {
        parse_to_cst_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
    }

    fn collect_node_kinds_chirho(root_chirho: &GreenNodeChirho) -> Vec<SyntaxKindChirho> {
        let mut kinds_chirho = Vec::new();
        for child_chirho in root_chirho.children_chirho() {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho {
                kinds_chirho.push(n_chirho.kind_chirho());
                kinds_chirho.extend(collect_node_kinds_chirho(n_chirho));
            }
        }
        kinds_chirho
    }

    #[test]
    fn parse_simple_module_chirho() {
        let root_chirho = parse_chirho("module Main where\nmain = putStrLn \"hi\"\n");
        assert_eq!(
            root_chirho.kind_chirho(),
            SyntaxKindChirho::SourceFileChirho
        );

        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::ModuleHeaderChirho),
            "should have ModuleHeader: {:?}",
            kinds_chirho
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::FunBindChirho),
            "should have FunBind: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_import_decl_chirho() {
        let source_chirho = "module M where\nimport Data.List\nimport qualified Data.Map as Map\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        let import_count_chirho = kinds_chirho
            .iter()
            .filter(|k_chirho| **k_chirho == SyntaxKindChirho::ImportDeclChirho)
            .count();
        assert_eq!(import_count_chirho, 2, "should parse 2 imports");
    }

    #[test]
    fn parse_data_decl_chirho() {
        let source_chirho =
            "module M where\ndata Color = Red | Green | Blue\n  deriving (Show, Eq)\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::DataDeclChirho),
            "should have DataDecl"
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::DerivingClauseChirho),
            "should have DerivingClause"
        );
    }

    #[test]
    fn parse_type_sig_with_types_chirho() {
        let source_chirho = "module M where\nfoo :: Int -> String -> Bool\nfoo x y = True\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::TypeSigDeclChirho),
            "should have TypeSigDecl"
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::FunTypeChirho),
            "should have FunType for -> : {:?}",
            kinds_chirho
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::FunBindChirho),
            "should have FunBind"
        );
    }

    #[test]
    fn parse_class_with_where_chirho() {
        let source_chirho =
            "module M where\nclass Describable a where\n  describe :: a -> String\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::ClassDeclChirho),
            "should have ClassDecl"
        );
    }

    #[test]
    fn parse_script_without_module_chirho() {
        let source_chirho = "main = putStrLn \"hello\"\n";
        let root_chirho = parse_chirho(source_chirho);
        assert_eq!(
            root_chirho.kind_chirho(),
            SyntaxKindChirho::SourceFileChirho
        );

        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::FunBindChirho),
            "should have FunBind even without module header"
        );
    }

    #[test]
    fn text_len_matches_source_chirho() {
        let source_chirho = "module Main where\nmain = putStrLn \"hi\"\n";
        let root_chirho = parse_chirho(source_chirho);
        assert_eq!(
            root_chirho.text_len_chirho(),
            source_chirho.len(),
            "green tree text length should match source length"
        );
    }

    #[test]
    fn parse_imports_with_specs_chirho() {
        let source_chirho = "module M where\nimport Data.Maybe (fromMaybe, isJust)\nimport Prelude hiding (map, filter)\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        let import_count_chirho = kinds_chirho
            .iter()
            .filter(|k_chirho| **k_chirho == SyntaxKindChirho::ImportDeclChirho)
            .count();
        assert_eq!(import_count_chirho, 2);
    }

    #[test]
    fn parse_expression_nodes_chirho() {
        let source_chirho = "module M where\nresult = f x + g y\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::AppExprChirho)
                || kinds_chirho.contains(&SyntaxKindChirho::InfixExprChirho),
            "should have application or infix expression nodes: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_if_expression_chirho() {
        let source_chirho = "module M where\nx = if True then 1 else 2\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::IfExprChirho),
            "should have IfExpr: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_lambda_expression_chirho() {
        let source_chirho = "module M where\nf = \\x -> x + 1\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::LambdaExprChirho),
            "should have LambdaExpr: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_lambda_expression_with_bang_pattern_arg_chirho() {
        let source_chirho =
            "module M where\n{-# LANGUAGE BangPatterns #-}\nf = \\x !y -> x\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::LambdaExprChirho),
            "should have LambdaExpr: {:?}",
            kinds_chirho
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::BangPatChirho),
            "should have BangPat: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_fun_binding_bang_as_pattern_arg_chirho() {
        let source_chirho = "module M where\n{-# LANGUAGE BangPatterns, MagicHash #-}\nimport GHC.Exts (Int(I#))\nf !x !_y@(I# y#) = x\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::BangPatChirho),
            "should have BangPat for strict args: {:?}",
            kinds_chirho
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::AsPatChirho),
            "strict as-pattern arg should retain AsPat: {:?}",
            kinds_chirho
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::ConPatChirho),
            "strict as-pattern arg should retain inner constructor pattern: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_where_strict_tuple_pattern_binding_as_pat_bind_chirho() {
        let source_chirho = "module MChirho where\nfChirho sChirho = r0Chirho where\n  (!q0Chirho, !r0Chirho) = sChirho `divMod` 10\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::PatBindChirho),
            "strict tuple where binding should parse as PatBindChirho: {:?}",
            kinds_chirho
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::ParenPatChirho),
            "strict tuple where binding should keep parenthesized pattern: {:?}",
            kinds_chirho
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::BangPatChirho),
            "strict tuple where binding should keep bang patterns: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_do_expression_chirho() {
        let source_chirho = "module M where\nmain = do\n  putStrLn \"hello\"\n  return ()\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::DoExprChirho),
            "should have DoExpr: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_case_expression_chirho() {
        let source_chirho = "module M where\nf x = case x of\n  True -> 1\n  False -> 0\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::CaseExprChirho),
            "should have CaseExpr: {:?}",
            kinds_chirho
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::CaseAltChirho),
            "should have CaseAlt: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_list_and_tuple_chirho() {
        let source_chirho = "module M where\nxs = [1, 2, 3]\np = (1, True)\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::ListExprChirho),
            "should have ListExpr: {:?}",
            kinds_chirho
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::ParenExprChirho),
            "should have ParenExpr (tuple): {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_type_alias_with_type_nodes_chirho() {
        let source_chirho = "module M where\ntype Name = String\ntype Pair a b = (a, b)\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        let alias_count_chirho = kinds_chirho
            .iter()
            .filter(|k_chirho| **k_chirho == SyntaxKindChirho::TypeAliasDeclChirho)
            .count();
        assert_eq!(alias_count_chirho, 2, "should parse 2 type aliases");
    }

    #[test]
    fn parse_where_clause_chirho() {
        let source_chirho = "module M where\nf x = y + z\n  where\n    y = x + 1\n    z = x * 2\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::WhereClauseChirho),
            "should have WhereClause: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn roundtrip_text_len_various_chirho() {
        let sources_chirho = [
            "main = putStrLn \"hello\"\n",
            "module M where\nf :: Int -> Int\nf x = x + 1\n",
            "module M where\nimport Data.List\ndata T = A | B\n",
            "x = if True then 1 else 2\n",
            "f = \\x y -> x + y\n",
            "main = do\n  putStrLn \"hi\"\n  return 42\n",
        ];
        for src_chirho in &sources_chirho {
            let root_chirho = parse_chirho(src_chirho);
            assert_eq!(
                root_chirho.text_len_chirho(),
                src_chirho.len(),
                "text_len mismatch for: {}",
                src_chirho.trim()
            );
        }
    }

    #[test]
    fn parse_open_type_family_chirho() {
        let source_chirho = "module M where\ntype family F a :: Type\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::TypeFamilyDeclChirho),
            "should have TypeFamilyDecl: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_closed_type_family_chirho() {
        let source_chirho =
            "module M where\ntype family F a where\n  F Int = Bool\n  F Char = Int\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::TypeFamilyDeclChirho),
            "should have TypeFamilyDecl: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_type_family_instance_chirho() {
        let source_chirho = "module M where\ntype instance F Int = Bool\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::TypeFamilyInstanceDeclChirho),
            "should have TypeFamilyInstanceDecl: {:?}",
            kinds_chirho
        );
    }

    // -------------------------------------------------------------------
    // Template Haskell parsing tests
    // -------------------------------------------------------------------

    #[test]
    fn parse_th_splice_name_expr_chirho() {
        // $name as expression inside a binding
        let source_chirho = "module M where\nx = $foo\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::SpliceExprChirho),
            "should have SpliceExpr for $foo: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_th_splice_parens_expr_chirho() {
        // $(expr) as expression inside a binding
        let source_chirho = "module M where\nx = $(makeLenses foo)\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::SpliceExprChirho),
            "should have SpliceExpr for $(makeLenses foo): {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_th_typed_splice_expr_chirho() {
        // $$name as expression
        let source_chirho = "module M where\nx = $$foo\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::TypedSpliceExprChirho),
            "should have TypedSpliceExpr for $$foo: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_th_splice_decl_chirho() {
        // $(expr) at top level parsed as SpliceDeclChirho
        let source_chirho = "module M where\n$(makeLenses foo)\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::SpliceDeclChirho),
            "should have SpliceDecl for top-level $(makeLenses foo): {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_th_expr_quote_chirho() {
        // [| expr |] expression quotation
        let source_chirho = "module M where\nx = [| foo |]\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::QuoteExprChirho),
            "should have QuoteExpr for [| foo |]: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_th_decl_quote_chirho() {
        // [d| decls |] declaration quotation
        let source_chirho = "module M where\nx = [d| y = 1 |]\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::QuoteDeclChirho),
            "should have QuoteDecl for [d| y = 1 |]: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_th_type_quote_chirho() {
        // [t| type |] type quotation
        let source_chirho = "module M where\nx = [t| Int |]\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::QuoteTypeChirho),
            "should have QuoteType for [t| Int |]: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_th_pat_quote_chirho() {
        // [p| pat |] pattern quotation
        let source_chirho = "module M where\nx = [p| y |]\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::QuotePatChirho),
            "should have QuotePat for [p| y |]: {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_th_splice_decl_make_lenses_chirho() {
        // $(makeLenses ''Foo) — typical TH usage at top level
        let source_chirho = "module M where\ndata Foo = Foo\n$(makeLenses ''Foo)\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::DataDeclChirho),
            "should have DataDecl: {:?}",
            kinds_chirho
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::SpliceDeclChirho),
            "should have SpliceDecl for $(makeLenses ''Foo): {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn parse_th_text_len_matches_chirho() {
        // Green tree text length must match source length for TH sources
        let source_chirho = "module M where\nx = $foo\n";
        let root_chirho = parse_chirho(source_chirho);
        assert_eq!(
            root_chirho.text_len_chirho(),
            source_chirho.len(),
            "TH green tree text length should match source length"
        );
    }
}
