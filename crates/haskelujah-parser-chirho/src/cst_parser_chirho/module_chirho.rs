// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The module header, export list and imports, and the dispatch of each top-level declaration
//! to its category.
//! Part of the CST parser (`cst_parser_chirho.rs`), split by syntactic category in row 640.

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;

use super::ParserChirho;
use crate::lexer_chirho::RawTokenKindChirho;

impl ParserChirho<'_> {
    // -----------------------------------------------------------------------
    // Module header
    // -----------------------------------------------------------------------

    pub(super) fn parse_module_header_chirho(&mut self) {
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

    pub(super) fn parse_top_level_decls_chirho(&mut self) {
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

    pub(super) fn parse_decl_chirho(&mut self) {
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

    pub(super) fn parse_import_decl_chirho(&mut self) {
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
}
