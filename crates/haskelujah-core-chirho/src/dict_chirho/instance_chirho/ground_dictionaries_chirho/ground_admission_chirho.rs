// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! What the conditional-dictionary route admits, decided by the rewrite's own rules before it
//! runs.
//! - An occurrence is admitted as the rewrite would dispatch it. A constrained reference
//!   whose target takes dictionary parameters admits each of its predicates. A class-method
//!   occurrence admits only its method's class, unless it names a lexical binding: a shadowing
//!   binder, a program-owned name, or an imported user reference. Those keep their binding,
//!   and their evidence is never built for or reported.
//! - The rows of a module instance whose declaration the producers pair (S3), and whose
//!   context the pass indexes faithfully, lead with that context, whatever their head.
//!
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::DictPassCtxChirho;
use super::ground_selection_chirho::{GroundRefusalChirho, readable_instance_chirho};
use crate::expr_chirho::CoreIdChirho;
use haskelujah_typing_chirho::class_chirho::{InstDeclChirho, PredChirho};

/// Which of an occurrence's ground goals the rewrite would serve with a dictionary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::dict_chirho) enum GroundAdmissionChirho {
    /// A class-method occurrence: only its method's class.
    MethodChirho(String),
    /// A constrained reference whose target takes dictionary parameters: each predicate.
    ReferenceChirho,
}

impl GroundAdmissionChirho {
    /// Whether a goal of `class_name_chirho` is one this occurrence's dispatch serves.
    pub(in crate::dict_chirho) fn admits_chirho(&self, class_name_chirho: &str) -> bool {
        match self {
            Self::MethodChirho(method_class_chirho) => method_class_chirho == class_name_chirho,
            Self::ReferenceChirho => true,
        }
    }
}

impl DictPassCtxChirho {
    /// The rewrite's admission of an occurrence, in the rewrite's order: the reference route
    /// first (`occurrence_head_replacement_chirho`), then the lexical-binding checks and the
    /// method selector (`try_rewrite_method_var_chirho`).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn ground_goal_admission_chirho(
        &self,
        occurrence_chirho: CoreIdChirho,
    ) -> Option<GroundAdmissionChirho> {
        let canonical_id_chirho = self.canonical_id_chirho(occurrence_chirho);
        if self
            .reference_evidence_chirho
            .contains_key(&occurrence_chirho)
        {
            return self
                .dict_param_bindings_chirho
                .contains_key(&canonical_id_chirho)
                .then_some(GroundAdmissionChirho::ReferenceChirho);
        }
        if self
            .local_shadow_ids_chirho
            .borrow()
            .contains(&canonical_id_chirho)
            || self.names_programs_own_binder_chirho(occurrence_chirho)
        {
            return None;
        }
        let name_chirho = self.names_chirho.get(&occurrence_chirho)?;
        let (class_name_chirho, _selector_chirho) =
            self.class_method_selector_for_name_chirho(name_chirho)?;
        Some(GroundAdmissionChirho::MethodChirho(class_name_chirho))
    }

    /// Why an instance's context cannot be indexed faithfully, if it cannot. Each predicate
    /// must be a one-parameter class predicate that this pass lays out. An equality, or an
    /// extra class argument the evidence does not capture, is refused by name. The old
    /// class-only map never chooses for one.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn context_refusal_chirho(
        &self,
        instance_chirho: &InstDeclChirho,
    ) -> Option<GroundRefusalChirho> {
        instance_chirho
            .context_chirho
            .iter()
            .find_map(|context_pred_chirho| {
                self.context_pred_refusal_chirho(instance_chirho, context_pred_chirho)
            })
    }

    fn context_pred_refusal_chirho(
        &self,
        instance_chirho: &InstDeclChirho,
        context_pred_chirho: &PredChirho,
    ) -> Option<GroundRefusalChirho> {
        if context_pred_chirho.class_name_chirho == "~" {
            return Some(GroundRefusalChirho::EqualityContextChirho(
                readable_instance_chirho(instance_chirho),
            ));
        }
        if !context_pred_chirho.extra_tys_chirho.is_empty() {
            return Some(GroundRefusalChirho::UnindexableContextChirho(
                readable_instance_chirho(instance_chirho),
            ));
        }
        (!self
            .layouts_chirho
            .contains_key(&context_pred_chirho.class_name_chirho))
        .then(|| GroundRefusalChirho::NoLayoutChirho(context_pred_chirho.class_name_chirho.clone()))
    }

    /// Mark each row of every paired module instance (S3) whose context is non-empty and
    /// indexed faithfully: it leads with that context, in the instance's own order, whatever
    /// its head. The row is named from its declaration's own key and looked up, never parsed.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    /// workflow: language-features-chirho/own-class-method-occurrences-chirho
    pub(in crate::dict_chirho) fn record_structural_context_rows_chirho(&mut self) {
        let mut marks_chirho: Vec<(CoreIdChirho, Vec<PredChirho>)> = Vec::new();
        for (instance_chirho, row_key_chirho) in self.instance_rows_chirho.proven_rows_chirho() {
            if instance_chirho.context_chirho.is_empty()
                || self.context_refusal_chirho(instance_chirho).is_some()
            {
                continue;
            }
            let Some(layout_chirho) = self.layouts_chirho.get(&instance_chirho.class_name_chirho)
            else {
                continue;
            };
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let row_name_chirho = format!(
                    "$prim_{}_{method_name_chirho}_{row_key_chirho}",
                    instance_chirho.class_name_chirho
                );
                if let Some(row_id_chirho) = self.body_backed_names_chirho.get(&row_name_chirho) {
                    marks_chirho.push((*row_id_chirho, instance_chirho.context_chirho.clone()));
                }
            }
        }
        self.context_leading_rows_chirho.extend(marks_chirho);
    }
}
