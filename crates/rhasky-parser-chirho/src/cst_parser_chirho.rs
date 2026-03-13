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
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ExportSpecChirho);
            // Simple: eat tokens until comma or close paren
            while !self.at_chirho(RawTokenKindChirho::CommaChirho)
                && !self.at_chirho(RawTokenKindChirho::RightParenChirho)
                && !self.at_eof_chirho()
            {
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::CommaChirho)
                    && !self.at_chirho(RawTokenKindChirho::RightParenChirho)
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

            self.parse_decl_chirho();
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

        // Eat until | or deriving or semicolon or end of block
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::PipeChirho,
            RawTokenKindChirho::DerivingChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
            RawTokenKindChirho::SemicolonChirho,
            RawTokenKindChirho::RightBraceChirho,
        ]);

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

        // Eat everything until end of declaration
        self.eat_until_decl_end_chirho();

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

        self.eat_until_decl_end_chirho();

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
    // Fixity declarations
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

    // -----------------------------------------------------------------------
    // Value declarations (type sigs and bindings)
    // -----------------------------------------------------------------------

    fn parse_value_decl_chirho(&mut self) {
        // Lookahead: if we see `name :: type`, it's a type signature.
        // Otherwise, it's a function/pattern binding.
        if self.is_type_sig_chirho() {
            self.parse_type_sig_chirho();
        } else {
            self.parse_fun_bind_chirho();
        }
    }

    fn parse_type_sig_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeSigDeclChirho);

        // Eat everything until end of declaration
        self.eat_until_decl_end_chirho();

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_fun_bind_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::FunBindChirho);

        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::MatchChirho);

        // Eat everything until end of declaration
        // This is simplified — a full parser would parse patterns, RHS, guards,
        // and where clauses properly.
        self.eat_until_decl_end_chirho();

        self.builder_chirho.finish_node_chirho(); // Match
        self.builder_chirho.finish_node_chirho(); // FunBind
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
                self.parse_decl_chirho();
                self.eat_trivia_chirho();
            }
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Utility methods
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
            let start_chirho = tok_chirho.span_chirho.start_chirho().as_usize_chirho();
            let end_chirho = tok_chirho.span_chirho.end_chirho().as_usize_chirho();
            if start_chirho < end_chirho && end_chirho <= self.source_chirho.len() {
                &self.source_chirho[start_chirho..end_chirho]
            } else {
                ""
            }
        } else {
            ""
        }
    }

    /// Advance one token, adding it to the current green node.
    fn bump_chirho(&mut self) {
        if let Some(tok_chirho) = self.current_chirho().copied() {
            let text_chirho = {
                let start_chirho = tok_chirho.span_chirho.start_chirho().as_usize_chirho();
                let end_chirho = tok_chirho.span_chirho.end_chirho().as_usize_chirho();
                if start_chirho < end_chirho && end_chirho <= self.source_chirho.len() {
                    &self.source_chirho[start_chirho..end_chirho]
                } else {
                    ""
                }
            };
            let kind_chirho = map_token_kind_chirho(tok_chirho.kind_chirho);
            self.builder_chirho.token_chirho(kind_chirho, text_chirho);
            self.pos_chirho += 1;
        }
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

    /// Eat tokens until we reach one of the given kinds (used for simplified
    /// parsing of constructs we haven't fully implemented yet).
    fn eat_until_any_chirho(&mut self, stops_chirho: &[RawTokenKindChirho]) {
        while let Some(kind_chirho) = self.current_kind_chirho() {
            if stops_chirho.contains(&kind_chirho) || kind_chirho == RawTokenKindChirho::EofChirho {
                break;
            }
            self.bump_chirho();
        }
    }

    /// Eat tokens until the end of the current declaration (virtual semicolon,
    /// virtual right brace, or EOF).
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
        // Scan forward from current position, skipping trivia
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
    fn parse_type_sig_chirho() {
        let source_chirho = "module M where\nfoo :: Int -> Int\nfoo x = x + 1\n";
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::TypeSigDeclChirho),
            "should have TypeSigDecl"
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
}
