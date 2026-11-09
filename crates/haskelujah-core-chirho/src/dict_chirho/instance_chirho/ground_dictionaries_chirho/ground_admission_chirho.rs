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

use std::collections::{HashMap, HashSet};

use super::ground_selection_chirho::{GroundRefusalChirho, readable_instance_chirho};
use super::instance_rows_chirho::RowKeyAbsenceChirho;
use super::{DictLayoutChirho, DictPassCtxChirho};
use crate::expr_chirho::{BinderChirho, CoreIdChirho};
use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho, PredChirho};

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

/// The existing preregistered dictionary payload, in its original field/move order.
/// Construction does not allocate an identifier or change canonical ownership.
pub(super) struct EagerInstanceRegistrationChirho {
    pub(super) class_name_chirho: String,
    pub(super) type_key_chirho: String,
    pub(super) layout_chirho: DictLayoutChirho,
    pub(super) dict_binder_chirho: BinderChirho,
    pub(super) instance_chirho: Option<std::sync::Arc<InstDeclChirho>>,
}

/// Pass-local lookup data from the already-proven S3 join. Pointer keys compare
/// actual canonical objects only; no pointer is dereferenced or exported.
#[derive(Default)]
pub(super) struct EagerInstanceRowsChirho {
    canonical_chirho: HashMap<*const InstDeclChirho, String>,
    written_owners_chirho: HashMap<String, HashMap<String, Option<*const InstDeclChirho>>>,
    fallback_claims_chirho: HashMap<String, HashSet<String>>,
}

impl DictPassCtxChirho {
    /// Index the existing proven rows once. Its existing join construction cost
    /// is unchanged; no per-method scan is added. Carried Arcs keep all indexed
    /// canonical objects live until eager generation finishes.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn eager_instance_rows_chirho(&self) -> EagerInstanceRowsChirho {
        let mut rows_chirho = EagerInstanceRowsChirho::default();
        for (instance_chirho, written_key_chirho) in self.instance_rows_chirho.proven_rows_chirho()
        {
            let pointer_chirho = instance_chirho as *const InstDeclChirho;
            rows_chirho
                .canonical_chirho
                .insert(pointer_chirho, written_key_chirho.to_string());
            rows_chirho
                .written_owners_chirho
                .entry(instance_chirho.class_name_chirho.clone())
                .or_default()
                .entry(written_key_chirho.to_string())
                .and_modify(|owner_chirho| {
                    if owner_chirho
                        .is_some_and(|known_chirho| !std::ptr::eq(known_chirho, instance_chirho))
                    {
                        *owner_chirho = None;
                    }
                })
                .or_insert(Some(pointer_chirho));
        }
        // Denial must retain ambiguous/unpaired source claims too. Only the
        // proven iterator above can fill the positive canonical/owner maps.
        for (class_chirho, route_chirho) in self.instance_rows_chirho.source_route_claims_chirho() {
            rows_chirho
                .fallback_claims_chirho
                .entry(class_chirho.to_string())
                .or_default()
                .insert(route_chirho.to_string());
        }
        rows_chirho
    }

    /// Decide once per dictionary. Structural equality cannot transfer another
    /// canonical object's row. Claimed source routes only deny legacy fallback;
    /// their text never grants a declaration or class identity.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn eager_instance_route_chirho(
        &self,
        rows_chirho: &EagerInstanceRowsChirho,
        instance_chirho: Option<&InstDeclChirho>,
        class_name_chirho: &str,
        type_key_chirho: &str,
        written_key_chirho: Option<&str>,
    ) -> Result<Option<String>, GroundRefusalChirho> {
        let description_chirho = || {
            instance_chirho.map_or_else(
                || format!("{class_name_chirho} {type_key_chirho}"),
                readable_instance_chirho,
            )
        };
        if let Some(instance_chirho) = instance_chirho {
            let pointer_chirho = instance_chirho as *const InstDeclChirho;
            if let Some(key_chirho) = rows_chirho.canonical_chirho.get(&pointer_chirho) {
                let owner_chirho = rows_chirho
                    .written_owners_chirho
                    .get(class_name_chirho)
                    .and_then(|owners_chirho| owners_chirho.get(key_chirho))
                    .copied()
                    .flatten();
                return if owner_chirho
                    .is_some_and(|owner_chirho| std::ptr::eq(owner_chirho, instance_chirho))
                {
                    Ok(Some(key_chirho.clone()))
                } else {
                    Err(GroundRefusalChirho::RowsAmbiguousChirho(
                        description_chirho(),
                    ))
                };
            }
            // Only an absent canonical object needs the old ambiguity check.
            // Its structural Ok may name a DIFFERENT object, never this owner.
            match self.instance_rows_chirho.row_key_chirho(instance_chirho) {
                Ok(_) => {
                    return Err(GroundRefusalChirho::RowsElsewhereChirho(
                        description_chirho(),
                    ));
                }
                Err(RowKeyAbsenceChirho::AmbiguousChirho) => {
                    return Err(GroundRefusalChirho::RowsAmbiguousChirho(
                        description_chirho(),
                    ));
                }
                Err(RowKeyAbsenceChirho::ElsewhereChirho) => {}
            }
        }
        if rows_chirho
            .fallback_claims_chirho
            .get(class_name_chirho)
            .is_some_and(|claims_chirho| {
                claims_chirho.contains(type_key_chirho)
                    || written_key_chirho
                        .is_some_and(|key_chirho| claims_chirho.contains(key_chirho))
            })
        {
            return Err(GroundRefusalChirho::RowsElsewhereChirho(
                description_chirho(),
            ));
        }
        Ok(None)
    }

    /// Read the chosen declaration's body-backed row or its own default donor.
    /// Otherwise retain the legacy seed route or a callable named refusal; an
    /// unused missing-method row does not fail the module globally.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn eager_instance_method_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        method_name_chirho: &str,
        type_key_chirho: &str,
        written_key_chirho: Option<&str>,
        route_chirho: &Result<Option<String>, GroundRefusalChirho>,
    ) -> CoreIdChirho {
        let prim_name_chirho =
            format!("$prim_{class_name_chirho}_{method_name_chirho}_{type_key_chirho}");
        match route_chirho {
            Ok(Some(key_chirho)) => {
                let row_name_chirho =
                    format!("$prim_{class_name_chirho}_{method_name_chirho}_{key_chirho}");
                if let Some(row_chirho) = self.body_backed_names_chirho.get(&row_name_chirho) {
                    return *row_chirho;
                }
                if let Some(default_chirho) = self.method_default_row_chirho(
                    class_name_chirho,
                    method_name_chirho,
                    key_chirho,
                    &prim_name_chirho,
                ) {
                    return default_chirho;
                }
                self.generate_missing_method_binding_chirho(
                    class_env_chirho,
                    class_name_chirho,
                    method_name_chirho,
                    type_key_chirho,
                    &prim_name_chirho,
                )
            }
            Ok(None) => self.resolve_or_missing_method_id_chirho(
                class_env_chirho,
                class_name_chirho,
                method_name_chirho,
                type_key_chirho,
                &prim_name_chirho,
                written_key_chirho,
            ),
            Err(refusal_chirho) => {
                let scheme_chirho = class_env_chirho
                    .classes_chirho()
                    .get_chirho(class_name_chirho)
                    .and_then(|class_chirho| class_chirho.methods_chirho.get(method_name_chirho));
                self.missing_method_row_chirho(
                    None,
                    &prim_name_chirho,
                    format!(
                        "missing method {class_name_chirho}.{method_name_chirho} for {type_key_chirho}: {}",
                        refusal_chirho.describe_chirho()
                    ),
                    scheme_chirho.map(Self::scheme_arity_chirho).unwrap_or(0),
                    scheme_chirho.map(|scheme_chirho| scheme_chirho.ty_chirho.clone()),
                )
            }
        }
    }
}
