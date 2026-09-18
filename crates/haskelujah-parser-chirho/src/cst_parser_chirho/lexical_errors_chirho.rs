// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Compiler entry preserving lexical failure; recovery parsing stays available to tooling.
//! Workflow: language-features-chirho/flat-type-syntax-chirho.md.
use super::{GreenNodeChirho, ParserChirho, RawTokenKindChirho};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use std::sync::Arc;

impl ParserChirho<'_> {
    /// Lexical errors are not recoverable declarations or deferrable type errors.
    /// Reuse the already-lexed token stream: no second scan of the source.
    /// General grammar-recovery diagnostics are a separate contract.
    pub fn parse_lexically_checked_chirho(
        self,
    ) -> Result<Arc<GreenNodeChirho>, DiagnosticBundleChirho> {
        if let Some(token_chirho) = self
            .tokens_chirho
            .iter()
            .find(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::ErrorChirho)
        {
            return Err(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(1),
                "invalid lexical token",
                token_chirho.span_chirho,
            )
            .into());
        }
        Ok(self.parse_chirho())
    }
}
