// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The rewrite's read of the ground-dictionary pre-pass. Every caller asks only after each of
//! its keyed routes has missed and before any ambient one, so a ground dictionary never
//! displaces a route that already served the proof. A refusal is reported (S2) only for an
//! occurrence that stays unserved: a method occurrence still standing in the rewritten module,
//! or a constrained reference slot that ended with no dictionary.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashSet;

use super::DictPassCtxChirho;
use crate::dict_chirho::DictPassDiagnosticChirho;
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho, CoreModuleChirho};

impl DictPassCtxChirho {
    /// The dictionary the pre-pass built for an occurrence's ground goal of `class_name_chirho`.
    pub(in crate::dict_chirho) fn ground_goal_dict_chirho(
        &self,
        occurrence_chirho: CoreIdChirho,
        class_name_chirho: &str,
    ) -> Option<CoreIdChirho> {
        self.ground_goal_dicts_chirho
            .get(&(occurrence_chirho, class_name_chirho.to_string()))
            .copied()
    }

    /// `selector_chirho` applied to that dictionary.
    pub(in crate::dict_chirho) fn ground_goal_projection_chirho(
        &self,
        occurrence_chirho: CoreIdChirho,
        class_name_chirho: &str,
        selector_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        self.ground_goal_dict_chirho(occurrence_chirho, class_name_chirho)
            .map(|dict_id_chirho| CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(selector_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
            })
    }

    /// Note that a constrained reference's slot of `class_name_chirho` ended with no
    /// dictionary, so a refusal the pre-pass recorded for it is reported.
    pub(in crate::dict_chirho) fn note_unserved_reference_slot_chirho(
        &self,
        occurrence_chirho: CoreIdChirho,
        class_name_chirho: &str,
    ) {
        let slot_chirho = (occurrence_chirho, class_name_chirho.to_string());
        if self.ground_goal_refusals_chirho.contains_key(&slot_chirho) {
            self.unserved_reference_slots_chirho
                .borrow_mut()
                .insert(slot_chirho);
        }
    }

    /// The recorded refusals whose occurrence the rewrite left unserved, once each, in a stable
    /// order. Read before surviving occurrences are canonicalized.
    pub(in crate::dict_chirho) fn unserved_ground_refusals_chirho(
        &self,
        module_chirho: &CoreModuleChirho,
    ) -> Vec<DictPassDiagnosticChirho> {
        if self.ground_goal_refusals_chirho.is_empty() {
            return Vec::new();
        }
        let surviving_chirho: HashSet<CoreIdChirho> = module_chirho
            .bindings_chirho
            .iter()
            .flat_map(|binding_chirho| {
                crate::simplify_chirho::free_vars_chirho(&binding_chirho.rhs_chirho)
            })
            .collect();
        let unserved_references_chirho = self.unserved_reference_slots_chirho.borrow();
        let mut diagnostics_chirho: Vec<DictPassDiagnosticChirho> = self
            .ground_goal_refusals_chirho
            .iter()
            .filter(|(slot_chirho, _diagnostic_chirho)| {
                let (occurrence_chirho, _class_chirho) = slot_chirho;
                // A served reference keeps its occurrence id under its dictionary arguments,
                // so only the slot itself can say it went unserved.
                if self
                    .reference_evidence_chirho
                    .contains_key(occurrence_chirho)
                {
                    unserved_references_chirho.contains(*slot_chirho)
                } else {
                    surviving_chirho.contains(occurrence_chirho)
                }
            })
            .map(|(_slot_chirho, diagnostic_chirho)| diagnostic_chirho.clone())
            .collect();
        diagnostics_chirho.sort_by_cached_key(|diagnostic_chirho| diagnostic_chirho.to_string());
        diagnostics_chirho.dedup();
        diagnostics_chirho
    }
}
