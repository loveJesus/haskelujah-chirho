// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Template Haskell splices and quotes.
//! Part of the CST parser (`cst_parser_chirho.rs`), split by syntactic category in row 640.

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;

use super::ParserChirho;
use crate::lexer_chirho::RawTokenKindChirho;

impl ParserChirho<'_> {
    // -----------------------------------------------------------------------
    // Template Haskell — splice and quotation parsing
    // -----------------------------------------------------------------------

    /// Parse a splice expression: `$name` or `$(expr)`.
    /// The lexer has already produced a `ThSpliceChirho` token for the `$`.
    /// What follows is either a variable name or a parenthesized expression.
    pub(super) fn parse_splice_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::SpliceExprChirho);
        self.parse_untyped_splice_payload_chirho();
        self.builder_chirho.finish_node_chirho();
    }

    /// The same payload grammar in a pattern keeps a distinct CST category.
    pub(super) fn parse_splice_pat_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::SplicePatChirho);
        self.parse_untyped_splice_payload_chirho();
        self.builder_chirho.finish_node_chirho();
    }

    fn parse_untyped_splice_payload_chirho(&mut self) {
        self.bump_chirho(); // $
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) || self.at_eof_chirho() {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                self.builder_chirho.finish_node_chirho();
            } else {
                self.parse_expr_chirho();
            }
            self.eat_trivia_chirho();
            self.expect_chirho(RawTokenKindChirho::RightParenChirho);
        } else if self.at_chirho(RawTokenKindChirho::VarIdChirho)
            || self.at_chirho(RawTokenKindChirho::ConIdChirho)
            || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
        {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::NameExprChirho);
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
        } else {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Parse a typed splice expression: `$$name` or `$$(expr)`.
    pub(super) fn parse_typed_splice_expr_chirho(&mut self) {
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
    pub(super) fn parse_splice_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::SpliceDeclChirho);
        self.parse_untyped_splice_payload_chirho();
        self.builder_chirho.finish_node_chirho();
    }

    /// Parse an expression quotation: `[| expr |]` or `[e| expr |]`.
    pub(super) fn parse_quote_expr_chirho(&mut self) {
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
    pub(super) fn parse_quote_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::QuoteDeclChirho);
        self.bump_chirho(); // consume ThOpenDecQuoteChirho
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
            // A written declaration block shares the ordinary declaration
            // grammar; retaining it must not turn its braces into recovery.
            self.parse_top_level_decls_chirho();
            self.eat_trivia_chirho();
            self.expect_chirho(RawTokenKindChirho::ThCloseQuoteChirho);
            self.builder_chirho.finish_node_chirho();
            return;
        }

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
    pub(super) fn parse_quote_type_chirho(&mut self) {
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
    pub(super) fn parse_quote_pat_chirho(&mut self) {
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
    pub(super) fn parse_typed_quote_expr_chirho(&mut self) {
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
}
