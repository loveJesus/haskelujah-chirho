// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written Ord operators take their occurrence's evidence (row 610).
//!
//! Desugaring lowered infix and section `<`, `<=`, `>` and `>=` to Int primops before any
//! type was known. Those primops met Strings and derived values as heap words, and signed
//! zeros in lambdas as integer bits, and they beat a program's own `(<)`. A written
//! operator that carries its origin now resolves as a reference instead: the program's own
//! binder where one is in scope, else a minted occurrence the dictionary pass serves by
//! the checker's evidence. Each minted one is recorded, so that where no row serves its
//! proof the pass can give it back the primop it replaced. Without an origin, or with the
//! name not opted in to occurrences, the operator keeps its primop.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::name_chirho::NameChirho;

use super::DesugarCtxChirho;
use crate::expr_chirho::CoreIdChirho;

/// The Ord operators whose written occurrences route by evidence.
const EVIDENCED_ORDERING_OPERATORS_CHIRHO: [&str; 4] = ["<", "<=", ">", ">="];

impl DesugarCtxChirho {
    /// Whether written operator `op_chirho` resolves as a reference rather than a primop:
    /// an Ord operator that carries its origin and is opted in to occurrences.
    pub(super) fn takes_operator_evidence_chirho(&self, op_chirho: &NameChirho) -> bool {
        let text_chirho = op_chirho.text_chirho();
        op_chirho.origin_chirho().is_some()
            && EVIDENCED_ORDERING_OPERATORS_CHIRHO.contains(&text_chirho)
            && self.method_occurrence_names_chirho.contains(text_chirho)
    }

    /// Record `id_chirho` when it is the minted occurrence of a written Ord operator, which
    /// reaches the reference route only by taking its evidence.
    pub(super) fn note_operator_occurrence_chirho(
        &mut self,
        op_chirho: &NameChirho,
        id_chirho: CoreIdChirho,
    ) {
        if EVIDENCED_ORDERING_OPERATORS_CHIRHO.contains(&op_chirho.text_chirho())
            && self.method_occurrences_chirho.contains_key(&id_chirho)
        {
            self.operator_occurrences_chirho.insert(id_chirho);
        }
    }
}
