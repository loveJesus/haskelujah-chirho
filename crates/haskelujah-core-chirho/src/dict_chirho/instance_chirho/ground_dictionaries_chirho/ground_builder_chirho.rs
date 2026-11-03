// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ground dictionaries for conditional instances. A pre-pass, before the per-binding rewrite,
//! builds one top-level dictionary per admitted ground goal whose structurally selected
//! instance has a context (`KnownNat n => KnownNat (Succ n)` at `Succ (Succ Zero)`).
//!
//! A goal's dictionary id is allocated and memoized before its obligations are built, so a
//! revisit while it is still being built returns that id. Finite recursive evidence therefore
//! ties a knot, as GHC's recursive dictionaries do (`instance Loop [a] => Loop [a]` prints 7),
//! and every dictionary on such a cycle is bound as recursive. Depth, dictionary and visit
//! bounds refuse an endlessly growing chain by name. A failed top-level attempt removes every
//! binding and memo entry it made, so no binding refers to a dictionary that was never built.
//!
//! An obligation may name only type constructors its goal names. The checker proves the
//! goal's constructors, so a constructor written only in an instance context has no proven
//! identity here and is refused by name. The pass records each refusal, and the rewrite
//! reports one only for an occurrence that stays unserved.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashSet;

use super::DictPassCtxChirho;
use super::ground_selection_chirho::{
    GroundRefusalChirho, goal_registry_key_chirho, instance_registry_key_chirho,
    pred_constructors_chirho, pred_is_ground_chirho, readable_instance_chirho,
    select_ground_instance_chirho,
};
use super::instance_rows_chirho::RowKeyAbsenceChirho;
use crate::dict_chirho::{DictLayoutChirho, DictPassDiagnosticChirho};
use crate::expr_chirho::{CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho};
use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, PredChirho};
use haskelujah_typing_chirho::ty_chirho::TyChirho;

/// Nested obligations one goal may descend through.
const GROUND_DEPTH_LIMIT_CHIRHO: usize = 64;
/// Conditional dictionaries one module may construct.
const GROUND_DICTIONARY_LIMIT_CHIRHO: usize = 256;
/// Goals one module's pre-pass may examine; a memoized revisit costs nothing.
const GROUND_VISIT_LIMIT_CHIRHO: usize = 4096;

/// The module-wide allowance the pre-pass spends, and the state of the attempt in progress.
#[derive(Default)]
struct GroundBuildChirho {
    dictionaries_chirho: usize,
    visits_chirho: usize,
    /// Dictionaries allocated and not yet bound, outermost first.
    in_progress_chirho: Vec<CoreIdChirho>,
    /// Dictionaries a revisit reached while they were still being built: each is recursive.
    recursive_chirho: HashSet<CoreIdChirho>,
    /// The goals this attempt memoized, removed again if it fails.
    attempt_goals_chirho: Vec<PredChirho>,
}

impl DictPassCtxChirho {
    /// Build the ground dictionaries for the module's admitted ground goals (S1 evidence).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn generate_ground_goal_dictionaries_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
    ) {
        if self.ground_goals_chirho.is_empty() {
            return;
        }
        let mut occurrences_chirho: Vec<(CoreIdChirho, Vec<PredChirho>)> = self
            .ground_goals_chirho
            .iter()
            .map(|(occurrence_chirho, goals_chirho)| (*occurrence_chirho, goals_chirho.clone()))
            .collect();
        occurrences_chirho.sort_by_key(|(occurrence_chirho, _)| *occurrence_chirho);
        let mut build_chirho = GroundBuildChirho::default();
        for (occurrence_chirho, goals_chirho) in occurrences_chirho {
            let Some(admission_chirho) = self.ground_goal_admission_chirho(occurrence_chirho)
            else {
                continue;
            };
            for goal_chirho in &goals_chirho {
                // A class that one occurrence proves twice cannot be told apart by class.
                let same_class_chirho = goals_chirho
                    .iter()
                    .filter(|other_chirho| {
                        other_chirho.class_name_chirho == goal_chirho.class_name_chirho
                    })
                    .count();
                if same_class_chirho != 1
                    || !admission_chirho.admits_chirho(&goal_chirho.class_name_chirho)
                    || !self.ground_goal_is_eligible_chirho(goal_chirho)
                {
                    continue;
                }
                // Only a goal whose one matching instance has a context is new to this route;
                // every other goal keeps the routes it has.
                match select_ground_instance_chirho(class_env_chirho, goal_chirho) {
                    Ok((instance_chirho, _)) if !instance_chirho.context_chirho.is_empty() => {}
                    _ => continue,
                }
                let slot_chirho = (occurrence_chirho, goal_chirho.class_name_chirho.clone());
                match self.ground_dictionary_attempt_chirho(
                    class_env_chirho,
                    goal_chirho,
                    &mut build_chirho,
                ) {
                    Ok(dict_id_chirho) => {
                        self.ground_goal_dicts_chirho
                            .insert(slot_chirho, dict_id_chirho);
                    }
                    Err(refusal_chirho) => {
                        self.ground_goal_refusals_chirho.insert(
                            slot_chirho,
                            DictPassDiagnosticChirho {
                                occurrence_chirho,
                                goal_chirho: goal_chirho.clone(),
                                reason_chirho: refusal_chirho.describe_chirho(),
                            },
                        );
                    }
                }
            }
        }
    }

    /// A ground goal of a class the pass lays out, with no dictionary filed under its own
    /// spelling already (a ground list specialization, for one).
    fn ground_goal_is_eligible_chirho(&self, goal_chirho: &PredChirho) -> bool {
        pred_is_ground_chirho(goal_chirho)
            && self
                .layouts_chirho
                .contains_key(&goal_chirho.class_name_chirho)
            && !self.instance_dicts_chirho.contains_key(&(
                goal_chirho.class_name_chirho.clone(),
                goal_registry_key_chirho(goal_chirho),
            ))
    }

    /// One top-level goal. On a refusal, every binding and memo entry this attempt made is
    /// removed.
    fn ground_dictionary_attempt_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        goal_chirho: &PredChirho,
        build_chirho: &mut GroundBuildChirho,
    ) -> Result<CoreIdChirho, GroundRefusalChirho> {
        let bindings_before_chirho = self.generated_bindings_chirho.len();
        let attempt_chirho =
            self.ground_dictionary_chirho(class_env_chirho, goal_chirho, 0, build_chirho);
        if attempt_chirho.is_err() {
            self.generated_bindings_chirho
                .truncate(bindings_before_chirho);
            for attempted_goal_chirho in &build_chirho.attempt_goals_chirho {
                self.ground_dicts_by_goal_chirho
                    .remove(attempted_goal_chirho);
            }
            build_chirho.in_progress_chirho.clear();
        }
        build_chirho.attempt_goals_chirho.clear();
        attempt_chirho
    }

    /// The dictionary for one ground goal: an existing dictionary filed under the goal's own
    /// spelling, the memoized one, or one built from the selected instance's context
    /// dictionaries and its own rows.
    fn ground_dictionary_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        goal_chirho: &PredChirho,
        depth_chirho: usize,
        build_chirho: &mut GroundBuildChirho,
    ) -> Result<CoreIdChirho, GroundRefusalChirho> {
        if let Some(dict_id_chirho) = self.ground_dicts_by_goal_chirho.get(goal_chirho).copied() {
            if let Some(position_chirho) = build_chirho
                .in_progress_chirho
                .iter()
                .position(|in_progress_chirho| *in_progress_chirho == dict_id_chirho)
            {
                let cycle_chirho = build_chirho.in_progress_chirho[position_chirho..].to_vec();
                build_chirho.recursive_chirho.extend(cycle_chirho);
            }
            return Ok(dict_id_chirho);
        }
        build_chirho.visits_chirho += 1;
        if build_chirho.visits_chirho > GROUND_VISIT_LIMIT_CHIRHO {
            return Err(GroundRefusalChirho::BudgetChirho(
                "visit",
                GROUND_VISIT_LIMIT_CHIRHO,
            ));
        }
        if depth_chirho > GROUND_DEPTH_LIMIT_CHIRHO {
            return Err(GroundRefusalChirho::BudgetChirho(
                "depth",
                GROUND_DEPTH_LIMIT_CHIRHO,
            ));
        }
        if !pred_is_ground_chirho(goal_chirho) {
            return Err(GroundRefusalChirho::OpenObligationChirho(
                goal_chirho.clone(),
            ));
        }
        if !goal_chirho.extra_tys_chirho.is_empty() {
            return Err(GroundRefusalChirho::MultiParameterChirho(
                goal_chirho.clone(),
            ));
        }
        let class_name_chirho = goal_chirho.class_name_chirho.clone();
        let Some(layout_chirho) = self.layouts_chirho.get(&class_name_chirho).cloned() else {
            return Err(GroundRefusalChirho::NoLayoutChirho(class_name_chirho));
        };
        if let Some(existing_chirho) = self.instance_dicts_chirho.get(&(
            class_name_chirho.clone(),
            goal_registry_key_chirho(goal_chirho),
        )) {
            return Ok(*existing_chirho);
        }
        let (instance_chirho, obligations_chirho) =
            select_ground_instance_chirho(class_env_chirho, goal_chirho)?;
        if instance_chirho.context_chirho.is_empty() {
            // A context-free instance has its dictionary already, under its own key.
            return self
                .instance_dicts_chirho
                .get(&(
                    class_name_chirho,
                    instance_registry_key_chirho(instance_chirho),
                ))
                .copied()
                .ok_or_else(|| {
                    GroundRefusalChirho::NoGroundDictionaryChirho(readable_instance_chirho(
                        instance_chirho,
                    ))
                });
        }
        if let Some(refusal_chirho) = self.context_refusal_chirho(instance_chirho) {
            return Err(refusal_chirho);
        }
        let row_key_chirho = match self.instance_rows_chirho.row_key_chirho(instance_chirho) {
            Ok(row_key_chirho) => row_key_chirho.to_string(),
            Err(RowKeyAbsenceChirho::ElsewhereChirho) => {
                return Err(GroundRefusalChirho::RowsElsewhereChirho(
                    readable_instance_chirho(instance_chirho),
                ));
            }
            Err(RowKeyAbsenceChirho::AmbiguousChirho) => {
                return Err(GroundRefusalChirho::RowsAmbiguousChirho(
                    readable_instance_chirho(instance_chirho),
                ));
            }
        };
        if !self.instance_rows_are_bound_chirho(&class_name_chirho, &row_key_chirho, &layout_chirho)
        {
            return Err(GroundRefusalChirho::RowsElsewhereChirho(
                readable_instance_chirho(instance_chirho),
            ));
        }
        let goal_constructors_chirho = pred_constructors_chirho(goal_chirho);
        for obligation_chirho in &obligations_chirho {
            if let Some(unproven_chirho) = pred_constructors_chirho(obligation_chirho)
                .into_iter()
                .find(|constructor_chirho| !goal_constructors_chirho.contains(constructor_chirho))
            {
                return Err(GroundRefusalChirho::UnprovenConstructorChirho(
                    obligation_chirho.to_string(),
                    unproven_chirho.to_string(),
                ));
            }
        }
        build_chirho.dictionaries_chirho += 1;
        if build_chirho.dictionaries_chirho > GROUND_DICTIONARY_LIMIT_CHIRHO {
            return Err(GroundRefusalChirho::BudgetChirho(
                "dictionary",
                GROUND_DICTIONARY_LIMIT_CHIRHO,
            ));
        }
        let dict_binder_chirho = self.fresh_binder_chirho(
            &format!(
                "$fG{class_name_chirho}{}",
                self.ground_dicts_by_goal_chirho.len()
            ),
            TyChirho::ConChirho(format!("$Dict_{class_name_chirho}")),
        );
        let dict_id_chirho = dict_binder_chirho.id_chirho;
        self.ground_dicts_by_goal_chirho
            .insert(goal_chirho.clone(), dict_id_chirho);
        build_chirho.attempt_goals_chirho.push(goal_chirho.clone());
        build_chirho.in_progress_chirho.push(dict_id_chirho);
        let mut context_dicts_chirho = Vec::with_capacity(obligations_chirho.len());
        for obligation_chirho in &obligations_chirho {
            context_dicts_chirho.push(self.ground_dictionary_chirho(
                class_env_chirho,
                obligation_chirho,
                depth_chirho + 1,
                build_chirho,
            )?);
        }
        let mut field_args_chirho = Vec::with_capacity(layout_chirho.field_count_chirho);
        for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
            let super_goal_chirho =
                PredChirho::new_chirho(super_name_chirho, goal_chirho.ty_chirho.clone());
            let super_id_chirho = self.ground_dictionary_chirho(
                class_env_chirho,
                &super_goal_chirho,
                depth_chirho + 1,
                build_chirho,
            )?;
            field_args_chirho.push(CoreExprChirho::VarChirho(super_id_chirho));
        }
        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            field_args_chirho.push(self.context_slot_chirho(
                &class_name_chirho,
                method_name_chirho,
                &row_key_chirho,
                &context_dicts_chirho,
            ));
        }
        build_chirho.in_progress_chirho.pop();
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: dict_binder_chirho,
            rhs_chirho: CoreExprChirho::ConAppChirho {
                con_name_chirho: format!("$Dict_{class_name_chirho}"),
                args_chirho: field_args_chirho,
            },
            is_rec_chirho: build_chirho.recursive_chirho.contains(&dict_id_chirho),
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        Ok(dict_id_chirho)
    }

    /// Whether the instance's rows are bound in this module under its declaration's key: at
    /// least one method row is, or the class has no methods. Rows that live elsewhere are
    /// never invented here.
    fn instance_rows_are_bound_chirho(
        &self,
        class_name_chirho: &str,
        row_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
    ) -> bool {
        layout_chirho.method_slots_chirho.is_empty()
            || layout_chirho
                .method_slots_chirho
                .iter()
                .any(|(method_name_chirho, _)| {
                    self.lookup_body_backed_name_id_chirho(&format!(
                        "$prim_{class_name_chirho}_{method_name_chirho}_{row_key_chirho}"
                    ))
                    .is_some()
                })
    }
}
