// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written operators take their occurrence's evidence (R1, rows 609 and 610).
//!
//! Desugaring lowered section operators, and infix Ord operators, to Int primops before any
//! type was known. Those primops met Doubles as integer bits and Strings and derived values as
//! heap words, and they beat a program's own operator. A written operator that carries its
//! origin resolves as a reference instead, to the program's own binder where one is in scope:
//! - a section of `==` (R1, gpt_chirho #27508), of `+`, `-` or `*` (row 609), or of `/=`
//!   (row 606) takes its method's route, as the infix operator does; `/` has no primop and
//!   always did;
//! - Ord's operators (row 610), infix or section, take a minted occurrence the dictionary pass
//!   serves by the checker's evidence. Each one is recorded, so that where no row serves its
//!   proof the pass can give it back the primop it replaced.
//!
//! Without an origin, as in generated code, the operator keeps its primop.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::provenance_chirho::OccurrenceRoleChirho;

use super::DesugarCtxChirho;
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho};

/// Class-method operators whose written section takes its method's route. Each has a method
/// binding, so the route serves it with or without a minted occurrence. `/=` joined Eq's
/// methods in row 606.
const SECTION_METHOD_OPERATORS_CHIRHO: [&str; 5] = ["==", "/=", "+", "-", "*"];

/// Ord's operators, which take their evidence infix as well. Their only binding is the one the
/// dictionary pass builds for an occurrence, so each needs one minted.
const EVIDENCED_ORDERING_OPERATORS_CHIRHO: [&str; 4] = ["<", "<=", ">", ">="];

impl DesugarCtxChirho {
    /// Whether written operator `op_chirho` resolves as a reference rather than a primop: one
    /// of the section method operators, or an Ord operator opted in to occurrences, carrying
    /// its origin either way.
    pub(super) fn takes_operator_evidence_chirho(&self, op_chirho: &NameChirho) -> bool {
        let text_chirho = op_chirho.text_chirho();
        op_chirho.origin_chirho().is_some()
            && (SECTION_METHOD_OPERATORS_CHIRHO.contains(&text_chirho)
                || (EVIDENCED_ORDERING_OPERATORS_CHIRHO.contains(&text_chirho)
                    && self.method_occurrence_names_chirho.contains(text_chirho)))
    }

    /// `left op right` through written operator `op_chirho`'s reference route: the binding
    /// its name resolves to (row 598), with its occurrence minted and its provenance recorded.
    /// It serves the infix fallback, a section's fallback, and an owned operator ahead of
    /// every Prelude shortcut (row 614).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn written_operator_application_chirho(
        &mut self,
        op_chirho: &NameChirho,
        left_chirho: CoreExprChirho,
        right_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let (op_lookup_chirho, op_id_chirho) = self.resolve_written_reference_chirho(op_chirho);
        let op_id_chirho = self.reference_occurrence_chirho(
            &op_lookup_chirho,
            op_id_chirho,
            op_chirho.span_chirho(),
        );
        self.note_operator_occurrence_chirho(op_chirho, op_id_chirho);
        self.record_occurrence_provenance_chirho(
            op_id_chirho,
            op_chirho.origin_chirho(),
            OccurrenceRoleChirho::Reference,
        );
        CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(op_id_chirho)),
                arg_chirho: Box::new(left_chirho),
            }),
            arg_chirho: Box::new(right_chirho),
        }
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
