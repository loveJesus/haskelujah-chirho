// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The class methods the compiler names for literal syntax (row 660): `fromInteger` for an
//! integer literal (Haskell 2010 §3.2), `fromString` for a string literal under
//! OverloadedStrings, and `fromList` for a list literal under OverloadedLists.
//!
//! The program never wrote that method reference, so none of the program's bindings is it. A
//! top-level or local `fromInteger`, `fromString` or `fromList` the program defines answers the
//! program's own calls by name, never its literals. Resolving the generated name lexically made
//! every literal call such a binding, and one with a literal in its own body recursed without
//! end, to the interpreter's step limit and a native timeout or stack overflow.
//!
//! The generated name now takes an occurrence of its own past every lexical binding, minted
//! here for each literal whether or not the method is in the general evidence set. The literal's
//! origin, span and role are recorded on it as before, so the checker's proof for that literal
//! reaches the dictionary pass, and an occurrence nothing proves falls back to the name's one
//! canonical id. A reference the program writes keeps resolving normally.
//!
//! RebindableSyntax is the boundary GHC defines: under it the generated name is whatever
//! binding is in scope, so it keeps the lexical route.
//! workflow: compiler-pipeline-chirho/value-scope-resolution-chirho

use super::DesugarCtxChirho;
use crate::expr_chirho::CoreIdChirho;
use haskelujah_span_chirho::SpanChirho;

impl DesugarCtxChirho {
    /// A range always selects the standard Enum method, including under
    /// RebindableSyntax. Its endpoints are desugared separately and keep their rules.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn resolve_range_method_chirho(
        &mut self,
        name_chirho: &str,
        span_chirho: SpanChirho,
    ) -> CoreIdChirho {
        let canonical_id_chirho = self.canonical_free_id_chirho(name_chirho);
        let id_chirho = self.mint_method_occurrence_chirho(name_chirho, canonical_id_chirho);
        // A real range span also keeps the legacy join out of positional fallback.
        self.method_occurrence_spans_chirho
            .insert(id_chirho, span_chirho);
        self.range_occurrence_spans_chirho
            .insert(id_chirho, span_chirho);
        id_chirho
    }

    /// The binding a compiler-generated reference to class method `name_chirho` stands for:
    /// under ordinary syntax, a fresh occurrence of the method with the name's canonical id
    /// behind it; under RebindableSyntax, the lexical binding.
    /// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
    pub(super) fn resolve_syntax_method_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        if self.rebinds_syntax_chirho() {
            return self.resolve_var_chirho(name_chirho);
        }
        let canonical_id_chirho = self.canonical_free_id_chirho(name_chirho);
        self.mint_method_occurrence_chirho(name_chirho, canonical_id_chirho)
    }

    /// Whether the module enables RebindableSyntax.
    fn rebinds_syntax_chirho(&self) -> bool {
        self.extensions_chirho
            .iter()
            .any(|extension_chirho| extension_chirho == "RebindableSyntax")
    }
}
