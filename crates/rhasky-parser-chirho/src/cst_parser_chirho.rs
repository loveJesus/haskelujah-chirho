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

use rhasky_syntax_chirho::cst_chirho::SyntaxKindChirho;
use rhasky_syntax_chirho::green_chirho::{GreenBuilderChirho, GreenNodeChirho};
use rhasky_syntax_chirho::token_chirho::TokenKindChirho;

use crate::lexer_chirho::{LexerChirho, RawTokenChirho, RawTokenKindChirho};
use crate::layout_chirho::apply_layout_chirho;
use rhasky_span_chirho::FileIdChirho;

// ---------------------------------------------------------------------------
// Token mapping: RawTokenKindChirho → TokenKindChirho
// ---------------------------------------------------------------------------

fn map_token_kind_chirho(raw_chirho: RawTokenKindChirho) -> TokenKindChirho {
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

        // Identifiers and operators
        RawTokenKindChirho::VarIdChirho => TokenKindChirho::VarIdChirho,
        RawTokenKindChirho::ConIdChirho => TokenKindChirho::ConIdChirho,
        RawTokenKindChirho::VarSymChirho => TokenKindChirho::VarSymChirho,
        RawTokenKindChirho::ConSymChirho => TokenKindChirho::ConSymChirho,
        RawTokenKindChirho::QualifiedIdChirho => TokenKindChirho::QualifiedConIdChirho,

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
        RawTokenKindChirho::FatArrowChirho => TokenKindChirho::DoubleArrowChirho,
        RawTokenKindChirho::AtChirho => TokenKindChirho::AtSignChirho,
        RawTokenKindChirho::TildeChirho => TokenKindChirho::TildeChirho,
        RawTokenKindChirho::UnderscoreChirho => TokenKindChirho::UnderscoreReservedIdChirho,

        // Trivia
        RawTokenKindChirho::WhitespaceChirho => TokenKindChirho::WhitespaceTriviaChirho,
        RawTokenKindChirho::LineCommentChirho => TokenKindChirho::LineCommentTriviaChirho,
        RawTokenKindChirho::BlockCommentChirho => TokenKindChirho::BlockCommentTriviaChirho,
        RawTokenKindChirho::DocCommentChirho => TokenKindChirho::DocCommentTriviaChirho,

        // Layout
        RawTokenKindChirho::VirtualLeftBraceChirho => TokenKindChirho::VirtualLeftBraceChirho,
        RawTokenKindChirho::VirtualRightBraceChirho => TokenKindChirho::VirtualRightBraceChirho,
        RawTokenKindChirho::VirtualSemicolonChirho => TokenKindChirho::VirtualSemicolonChirho,

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
            // Eat tokens until comma or close paren
            while !self.at_chirho(RawTokenKindChirho::CommaChirho)
                && !self.at_chirho(RawTokenKindChirho::RightParenChirho)
                && !self.at_eof_chirho()
            {
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::CommaChirho)
                    && !self.at_chirho(RawTokenKindChirho::RightParenChirho)
                    && !self.at_eof_chirho()
                {
                    self.bump_chirho();
                }
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
            Some(RawTokenKindChirho::TypeChirho) => self.parse_type_alias_decl_chirho(),
            Some(RawTokenKindChirho::NewtypeChirho) => self.parse_newtype_decl_chirho(),
            Some(RawTokenKindChirho::ClassChirho) => self.parse_class_decl_chirho(),
            Some(RawTokenKindChirho::InstanceChirho) => self.parse_instance_decl_chirho(),
            Some(RawTokenKindChirho::InfixChirho)
            | Some(RawTokenKindChirho::InfixlChirho)
            | Some(RawTokenKindChirho::InfixrChirho) => self.parse_fixity_decl_chirho(),
            Some(RawTokenKindChirho::DefaultChirho) => self.parse_default_decl_chirho(),
            Some(RawTokenKindChirho::ForeignChirho) => self.parse_foreign_decl_chirho(),
            _ => self.parse_value_decl_chirho(),
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

        // Module name
        if self.at_chirho(RawTokenKindChirho::ConIdChirho)
            || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
        {
            self.bump_chirho();
        }
        self.eat_trivia_chirho();

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

            // Eat tokens until comma or close paren
            while !self.at_chirho(RawTokenKindChirho::CommaChirho)
                && !self.at_chirho(RawTokenKindChirho::RightParenChirho)
                && !self.at_eof_chirho()
            {
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::CommaChirho)
                    && !self.at_chirho(RawTokenKindChirho::RightParenChirho)
                    && !self.at_eof_chirho()
                {
                    self.bump_chirho();
                }
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

        // = Constructor | Constructor ...
        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();

            self.parse_con_decl_chirho();

            while self.at_chirho(RawTokenKindChirho::PipeChirho) {
                self.bump_chirho(); // |
                self.eat_trivia_chirho();
                self.parse_con_decl_chirho();
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

        // Constructor name
        if self.at_chirho(RawTokenKindChirho::ConIdChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();

            // Record syntax: { field :: Type, ... }
            if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                self.parse_record_fields_chirho();
            } else {
                // Ordinary constructor: parse atomic types as fields
                while self.can_start_atype_chirho() {
                    let before_chirho = self.pos_chirho;
                    self.parse_atype_chirho();
                    self.eat_trivia_chirho();
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

            // field names, ::, type
            while !self.at_chirho(RawTokenKindChirho::CommaChirho)
                && !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                && !self.at_decl_boundary_chirho()
                && !self.at_eof_chirho()
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

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Type alias declarations
    // -----------------------------------------------------------------------

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

    fn parse_default_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::DefaultDeclChirho);
        self.bump_chirho(); // default
        self.eat_trivia_chirho();
        self.eat_until_decl_end_chirho();
        self.builder_chirho.finish_node_chirho();
    }

    fn parse_foreign_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ForeignDeclChirho);
        self.bump_chirho(); // foreign
        self.eat_trivia_chirho();
        self.eat_until_decl_end_chirho();
        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Value declarations (type sigs and bindings)
    // -----------------------------------------------------------------------

    fn parse_value_decl_chirho(&mut self) {
        if self.is_type_sig_chirho() {
            self.parse_type_sig_chirho();
        } else {
            self.parse_fun_bind_chirho();
        }
    }

    fn parse_type_sig_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeSigDeclChirho);

        // Name (or operator in parens)
        if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            // Operator in parens: (+!)
            self.bump_chirho(); // (
            self.eat_trivia_chirho();
            while !self.at_chirho(RawTokenKindChirho::RightParenChirho) && !self.at_eof_chirho() {
                self.bump_chirho();
            }
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
        } else {
            self.bump_chirho(); // name
        }
        self.eat_trivia_chirho();

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

        // Function name or pattern head
        if self.at_chirho(RawTokenKindChirho::VarIdChirho)
            || self.at_chirho(RawTokenKindChirho::ConIdChirho)
        {
            self.bump_chirho();
            self.eat_trivia_chirho();
        } else if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            // Operator in parens or tuple pattern
            self.parse_apat_chirho();
            self.eat_trivia_chirho();
        } else if self.can_start_apat_chirho() {
            self.parse_apat_chirho();
            self.eat_trivia_chirho();
        }

        // Parse argument patterns until = or |
        while self.can_start_apat_chirho() {
            let before_chirho = self.pos_chirho;
            self.parse_apat_chirho();
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

    fn parse_guarded_rhs_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::GuardedRhsChirho);

        while self.at_chirho(RawTokenKindChirho::PipeChirho) {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::GuardChirho);

            self.bump_chirho(); // |
            self.eat_trivia_chirho();

            // Guard expression (parse until = )
            self.parse_guard_expr_chirho();

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
        // Parse expression atoms until = or decl boundary
        while !self.at_chirho(RawTokenKindChirho::EqualsChirho)
            && !self.at_decl_boundary_chirho()
        {
            self.bump_chirho();
        }
    }

    // -----------------------------------------------------------------------
    // Where blocks
    // -----------------------------------------------------------------------

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

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Type parsing
    // -----------------------------------------------------------------------

    /// Parse a type. Handles qualified types (context => type) and function
    /// types (a -> b). Uses checkpoints for retroactive wrapping.
    fn parse_type_chirho(&mut self) {
        let cp_chirho = self.builder_chirho.checkpoint_chirho();

        self.parse_btype_chirho();
        self.eat_trivia_chirho();

        // Check for => (qualified type) — but only if we haven't gone
        // past the declaration boundary.
        if self.at_chirho(RawTokenKindChirho::FatArrowChirho) {
            // Retroactively wrap the parsed btype as a context
            self.builder_chirho.start_node_at_chirho(
                cp_chirho,
                SyntaxKindChirho::QualTypeChirho,
            );
            self.bump_chirho(); // =>
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // the actual type
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for -> (function type)
        if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
            self.builder_chirho.start_node_at_chirho(
                cp_chirho,
                SyntaxKindChirho::FunTypeChirho,
            );
            self.bump_chirho(); // ->
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // right-recursive
            self.builder_chirho.finish_node_chirho();
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
                self.builder_chirho.start_node_at_chirho(
                    cp_chirho,
                    SyntaxKindChirho::AppTypeChirho,
                );
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
            Some(RawTokenKindChirho::ConIdChirho)
            | Some(RawTokenKindChirho::QualifiedIdChirho) => {
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
            self.builder_chirho.start_node_at_chirho(
                cp_chirho,
                SyntaxKindChirho::TypeAnnotExprChirho,
            );
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
                self.builder_chirho.start_node_at_chirho(
                    cp_chirho,
                    SyntaxKindChirho::InfixExprChirho,
                );
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

            self.parse_lexp_chirho();
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
                if self.current_text_chirho() == "-"
                    && !self.at_decl_boundary_chirho() =>
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
    fn parse_lambda_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::LambdaExprChirho);

        self.bump_chirho(); // backslash
        self.eat_trivia_chirho();

        // Parse patterns until ->
        while self.can_start_apat_chirho() {
            let before_chirho = self.pos_chirho;
            self.parse_apat_chirho();
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
    fn parse_if_expr_chirho(&mut self) {
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

    /// Parse guard expression in case alt (between | and ->).
    fn parse_guard_expr_case_chirho(&mut self) {
        while !self.at_chirho(RawTokenKindChirho::RightArrowChirho)
            && !self.at_decl_boundary_chirho()
        {
            self.bump_chirho();
        }
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
            self.builder_chirho.start_node_at_chirho(
                cp_chirho,
                SyntaxKindChirho::BindStmtChirho,
            );
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
            self.builder_chirho.start_node_at_chirho(
                cp_chirho,
                SyntaxKindChirho::DoStmtChirho,
            );
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

        while i_chirho < self.tokens_chirho.len() {
            let k_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            match k_chirho {
                RawTokenKindChirho::LeftArrowChirho if paren_depth_chirho == 0 && bracket_depth_chirho == 0 => {
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
                // Statement boundaries
                RawTokenKindChirho::VirtualSemicolonChirho
                | RawTokenKindChirho::SemicolonChirho
                | RawTokenKindChirho::VirtualRightBraceChirho
                | RawTokenKindChirho::RightBraceChirho
                | RawTokenKindChirho::EofChirho => {
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
        while self.can_start_aexp_chirho() {
            let before_chirho = self.pos_chirho;
            if count_chirho == 1 {
                self.builder_chirho.start_node_at_chirho(
                    cp_chirho,
                    SyntaxKindChirho::AppExprChirho,
                );
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
            Some(RawTokenKindChirho::ConIdChirho)
            | Some(RawTokenKindChirho::QualifiedIdChirho) => {
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
            Some(RawTokenKindChirho::LeftParenChirho) => {
                self.parse_paren_expr_chirho();
            }
            Some(RawTokenKindChirho::LeftBracketChirho) => {
                self.parse_list_expr_chirho();
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

        // Could be a left section: (op) or (op expr) or (expr op)
        // or just (expr) or (expr, expr, ...)
        self.parse_expr_chirho();
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // Tuple
            while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho(); // ,
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::RightParenChirho)
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
                    self.parse_expr_chirho();
                    self.eat_trivia_chirho();
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

        // Check for infix constructor pattern
        if self.at_chirho(RawTokenKindChirho::ConSymChirho)
            || self.at_chirho(RawTokenKindChirho::BacktickChirho)
        {
            self.builder_chirho.start_node_at_chirho(
                cp_chirho,
                SyntaxKindChirho::InfixConPatChirho,
            );

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
                self.bump_chirho(); // ConSym
            }
            self.eat_trivia_chirho();

            self.parse_lpat_chirho();
            self.builder_chirho.finish_node_chirho();
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
                    self.builder_chirho.start_node_at_chirho(
                        cp_chirho,
                        SyntaxKindChirho::AsPatChirho,
                    );
                    self.bump_chirho(); // @
                    self.eat_trivia_chirho();
                    self.parse_apat_chirho();
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::ConIdChirho)
            | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                // Constructor pattern possibly with argument patterns
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.bump_chirho(); // ConId
                self.eat_trivia_chirho();

                // Check for record pattern
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    self.builder_chirho.start_node_at_chirho(
                        cp_chirho,
                        SyntaxKindChirho::RecordPatChirho,
                    );
                    self.parse_record_pat_fields_chirho();
                    self.builder_chirho.finish_node_chirho();
                } else if self.can_start_apat_chirho() {
                    // Constructor application pattern
                    self.builder_chirho.start_node_at_chirho(
                        cp_chirho,
                        SyntaxKindChirho::ConPatChirho,
                    );
                    while self.can_start_apat_chirho() {
                        let before_chirho = self.pos_chirho;
                        self.parse_apat_chirho();
                        self.eat_trivia_chirho();
                        if self.pos_chirho == before_chirho {
                            break;
                        }
                    }
                    self.builder_chirho.finish_node_chirho();
                }
                // Otherwise, just the ConId stands (no wrapping)
            }
            Some(RawTokenKindChirho::TildeChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LazyPatChirho);
                self.bump_chirho(); // ~
                self.eat_trivia_chirho();
                self.parse_apat_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::VarSymChirho) if self.current_text_chirho() == "!" => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::BangPatChirho);
                self.bump_chirho(); // !
                self.eat_trivia_chirho();
                self.parse_apat_chirho();
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
            Some(RawTokenKindChirho::ConIdChirho)
            | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ConPatChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
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

    /// Parse parenthesized or tuple pattern.
    fn parse_paren_pat_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ParenPatChirho);

        self.bump_chirho(); // (
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            // Unit pattern ()
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.parse_pat_chirho();
        self.eat_trivia_chirho();

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
    // Predicates — what can start various constructs
    // -----------------------------------------------------------------------

    /// Can the current token start an atomic type?
    fn can_start_atype_chirho(&self) -> bool {
        matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarIdChirho)
                | Some(RawTokenKindChirho::ConIdChirho)
                | Some(RawTokenKindChirho::QualifiedIdChirho)
                | Some(RawTokenKindChirho::LeftParenChirho)
                | Some(RawTokenKindChirho::LeftBracketChirho)
        )
    }

    /// Can the current token start an atomic expression?
    fn can_start_aexp_chirho(&self) -> bool {
        matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarIdChirho)
                | Some(RawTokenKindChirho::ConIdChirho)
                | Some(RawTokenKindChirho::QualifiedIdChirho)
                | Some(RawTokenKindChirho::IntLitChirho)
                | Some(RawTokenKindChirho::FloatLitChirho)
                | Some(RawTokenKindChirho::CharLitChirho)
                | Some(RawTokenKindChirho::StringLitChirho)
                | Some(RawTokenKindChirho::LeftParenChirho)
                | Some(RawTokenKindChirho::LeftBracketChirho)
        )
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
        )
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
        self.pos_chirho >= self.tokens_chirho.len()
            || self.at_chirho(RawTokenKindChirho::EofChirho)
    }

    /// Check if current token is a VarId with specific text.
    fn at_varid_text_chirho(&self, text_chirho: &str) -> bool {
        if !self.at_chirho(RawTokenKindChirho::VarIdChirho) {
            return false;
        }
        self.current_text_chirho() == text_chirho
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
            let kind_chirho = map_token_kind_chirho(tok_chirho.kind_chirho);
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

        // Skip the name(s) — could be `(+!)` for operator type sigs
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

        // Skip trivia
        while i_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[i_chirho].kind_chirho.is_trivia_chirho()
        {
            i_chirho += 1;
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
    use rhasky_syntax_chirho::green_chirho::GreenElementChirho;

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
        assert_eq!(root_chirho.kind_chirho(), SyntaxKindChirho::SourceFileChirho);

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
        let source_chirho = "module M where\ndata Color = Red | Green | Blue\n  deriving (Show, Eq)\n";
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
        assert_eq!(root_chirho.kind_chirho(), SyntaxKindChirho::SourceFileChirho);

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
}
