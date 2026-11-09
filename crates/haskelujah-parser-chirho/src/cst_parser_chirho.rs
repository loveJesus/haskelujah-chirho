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

mod annotated_bindings_chirho;
#[cfg(test)]
#[path = "cst_parser_chirho/tests_chirho/annotated_bindings_tests_chirho.rs"]
mod annotated_bindings_tests_chirho;
mod atoms_chirho;
mod bindings_chirho;
#[cfg(test)]
#[path = "cst_parser_chirho/tests_chirho/comprehension_let_tests_chirho.rs"]
mod comprehension_let_tests_chirho;
mod cursor_chirho;
pub(crate) use cursor_chirho::split_qualified_name_chirho;
mod data_decls_chirho;
mod declarations_chirho;
mod expressions_chirho;
mod gadt_signatures_chirho;
#[cfg(test)]
#[path = "cst_parser_chirho/tests_chirho/guard_let_tests_chirho.rs"]
mod guard_let_tests_chirho;
#[cfg(test)]
#[path = "cst_parser_chirho/tests_chirho/infix_definition_spacing_tests_chirho.rs"]
mod infix_definition_spacing_tests_chirho;
mod let_qualifier_chirho;
mod module_chirho;
#[cfg(test)]
#[path = "cst_parser_chirho/tests_chirho/paren_funlhs_tests_chirho.rs"]
mod paren_funlhs_tests_chirho;
mod patterns_chirho;
mod template_haskell_chirho;
mod types_chirho;

use crate::layout_chirho::apply_layout_chirho;
use crate::lexer_chirho::{LexerChirho, RawTokenChirho, RawTokenKindChirho};
use crate::pragma_chirho::{
    EffectiveExtensionsChirho, classify_contextual_keywords_with_extensions_chirho,
};
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
    /// Set while a closed type-family equation's left side is parsed. Layout
    /// separates those equations, so a variable that opens a continuation line
    /// before `=` belongs to the left side rather than starting a value binding.
    family_equation_lhs_chirho: bool,
    /// Set while an instance's `where` block is parsed. `parse_decl_chirho` takes it
    /// for that block's own members only, whose `type` declarations are associated
    /// instances; a declaration nested inside a member parses as usual (row 686).
    instance_body_chirho: bool,
    /// The resolved source profile licenses invisible written quantification.
    explicit_for_all_chirho: bool,
}

impl<'src> ParserChirho<'src> {
    /// Create a parser from source text. Lexes and applies layout rule internally.
    pub fn new_chirho(source_chirho: &'src str, file_id_chirho: FileIdChirho) -> Self {
        let extensions_chirho =
            EffectiveExtensionsChirho::from_source_chirho(source_chirho, file_id_chirho);
        Self::with_extensions_chirho(source_chirho, file_id_chirho, &extensions_chirho)
    }

    /// Parse under the same effective extensions that CST lowering will publish.
    pub fn with_extensions_chirho(
        source_chirho: &'src str,
        file_id_chirho: FileIdChirho,
        extensions_chirho: &EffectiveExtensionsChirho,
    ) -> Self {
        let mut lexer_chirho = LexerChirho::new_chirho(source_chirho, file_id_chirho);
        let mut raw_chirho = lexer_chirho.lex_all_chirho();
        classify_contextual_keywords_with_extensions_chirho(
            source_chirho,
            &mut raw_chirho,
            extensions_chirho,
        );
        let tokens_chirho = apply_layout_chirho(source_chirho, raw_chirho, file_id_chirho);

        Self {
            source_chirho,
            tokens_chirho,
            pos_chirho: 0,
            builder_chirho: GreenBuilderChirho::new_chirho(),
            family_equation_lhs_chirho: false,
            instance_body_chirho: false,
            explicit_for_all_chirho: extensions_chirho.enabled_chirho("ExplicitForAll"),
        }
    }

    /// Parse a compilation input without lowering recovered syntax errors.
    /// The lossless CST entry point remains available to editors and tooling.
    pub fn parse_checked_chirho(
        self,
    ) -> Result<Arc<GreenNodeChirho>, haskelujah_diagnostics_chirho::DiagnosticBundleChirho> {
        let file_id_chirho = self
            .tokens_chirho
            .first()
            .map(|token_chirho| token_chirho.span_chirho.file_id_chirho())
            .unwrap_or(FileIdChirho::SYNTHETIC_CHIRHO);
        let root_chirho = self.parse_chirho();
        check_cst_syntax_chirho(&root_chirho, file_id_chirho)?;
        Ok(root_chirho)
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

/// An error node is a typed parser refusal, never an ordinary type error.
/// Traverse the immutable parser output iteratively, with memory bounded by
/// that tree; compilation stops at the first error, preserving source offsets.
pub fn check_cst_syntax_chirho(
    root_chirho: &GreenNodeChirho,
    file_id_chirho: FileIdChirho,
) -> Result<(), haskelujah_diagnostics_chirho::DiagnosticBundleChirho> {
    use haskelujah_span_chirho::{ByteOffsetChirho, SpanChirho};
    use haskelujah_syntax_chirho::green_chirho::GreenElementChirho;
    use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;
    let mut pending_chirho = vec![(root_chirho, 0usize)];
    while let Some((node_chirho, start_chirho)) = pending_chirho.pop() {
        if node_chirho.kind_chirho() == SyntaxKindChirho::ErrorNodeChirho {
            // The shared producer wraps precisely `forall <binders> .` when
            // the effective profile forbids it. Module headers remain normal.
            let mut tokens_chirho =
                node_chirho
                    .children_chirho()
                    .iter()
                    .filter_map(|element_chirho| match element_chirho {
                        GreenElementChirho::TokenChirho(token_chirho) => Some(token_chirho),
                        GreenElementChirho::NodeChirho(_) => None,
                    });
            let forbidden_forall_chirho = tokens_chirho.next().is_some_and(|token_chirho| {
                token_chirho.kind_chirho() == TokenKindChirho::ForallKeywordChirho
            }) && tokens_chirho
                .next_back()
                .is_some_and(|token_chirho| token_chirho.text_chirho() == ".");
            let message_chirho = if forbidden_forall_chirho {
                "illegal forall syntax: the effective source profile requires ExplicitForAll"
            } else {
                "invalid source syntax"
            };
            return Err(
                haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                    haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(2),
                    message_chirho,
                    SpanChirho::new_chirho(
                        file_id_chirho,
                        ByteOffsetChirho::from_usize_chirho(start_chirho),
                        ByteOffsetChirho::from_usize_chirho(
                            start_chirho + node_chirho.text_len_chirho(),
                        ),
                    ),
                )
                .into(),
            );
        }
        let mut end_chirho = start_chirho + node_chirho.text_len_chirho();
        for element_chirho in node_chirho.children_chirho().iter().rev() {
            end_chirho -= element_chirho.text_len_chirho();
            if let GreenElementChirho::NodeChirho(child_chirho) = element_chirho {
                pending_chirho.push((child_chirho.as_ref(), end_chirho));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "cst_parser_chirho/tests_chirho/checked_syntax_tests_chirho.rs"]
mod checked_syntax_tests_chirho;

#[cfg(test)]
#[path = "cst_parser_chirho/tests_chirho/parser_tests_chirho.rs"]
mod tests_chirho;
