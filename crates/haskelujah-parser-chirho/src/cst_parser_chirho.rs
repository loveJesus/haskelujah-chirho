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
//! - The parser is organized by syntactic category, one file each under
//!   `cst_parser_chirho/` (row 640): the token cursor, the module header and
//!   imports, data declarations, other declarations, bindings, types,
//!   expressions, atoms, patterns and Template Haskell. This file holds the
//!   public shell: `ParserChirho`, its entry points and `parse_to_cst_chirho`.
//! - Uses the checkpoint mechanism for retroactive node wrapping (needed for
//!   infix expressions, function types, type annotations).

use std::sync::Arc;

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenBuilderChirho, GreenNodeChirho};

mod atoms_chirho;
mod bindings_chirho;
#[cfg(test)]
#[path = "cst_parser_chirho/tests_chirho/comprehension_let_tests_chirho.rs"]
mod comprehension_let_tests_chirho;
mod cursor_chirho;
mod data_decls_chirho;
mod declarations_chirho;
mod expressions_chirho;
#[cfg(test)]
#[path = "cst_parser_chirho/tests_chirho/guard_let_tests_chirho.rs"]
mod guard_let_tests_chirho;
#[cfg(test)]
#[path = "cst_parser_chirho/tests_chirho/infix_definition_spacing_tests_chirho.rs"]
mod infix_definition_spacing_tests_chirho;
mod let_qualifier_chirho;
mod module_chirho;
mod patterns_chirho;
mod template_haskell_chirho;
mod types_chirho;

use crate::layout_chirho::apply_layout_chirho;
use crate::lexer_chirho::{LexerChirho, RawTokenChirho, RawTokenKindChirho};
use crate::pragma_chirho::classify_contextual_keywords_chirho;
use haskelujah_span_chirho::FileIdChirho;

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
        let mut raw_chirho = lexer_chirho.lex_all_chirho();
        classify_contextual_keywords_chirho(source_chirho, &mut raw_chirho);
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

#[cfg(test)]
#[path = "cst_parser_chirho/tests_chirho/parser_tests_chirho.rs"]
mod tests_chirho;
