// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ground dictionaries for conditional instances. A pre-pass, before the per-binding rewrite,
//! builds one top-level dictionary per admitted ground goal whose structurally selected
//! instance has a context (`KnownNat n => KnownNat (Succ n)` at `Succ (Succ Zero)`).
//!
//! A goal's dictionary id is allocated and memoized before its obligations are built, so a
//! revisit while it is still being built returns that id. Finite recursive evidence therefore
//! ties a knot, as GHC's recursive dictionaries do (`instance Loop [a] => Loop [a]` prints 7),
//! and every dictionary on such a cycle is bound as recursive. A goal that only grows
//! (`instance C (a, a) => C a`) reaches a bound of `ground_budget_chirho.rs` instead, by name.
//! A failed top-level attempt removes every binding and memo entry it made, so no binding
//! refers to a dictionary that was never built.
//!
//! An obligation that names a type constructor its goal does not is served only by a dictionary
//! already bound for exactly it. The checker proves the goal's constructors, so a constructor
//! written only in an instance context has no proven identity for a new build. The pass records
//! each refusal, and the rewrite reports one only for an occurrence that stays unserved.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{HashMap, HashSet};

use super::DictPassCtxChirho;
use super::ground_budget_chirho::GroundBudgetChirho;
use super::ground_selection_chirho::{
    GroundRefusalChirho, goal_registry_key_chirho, instance_registry_key_chirho,
    pred_constructors_chirho, pred_is_ground_chirho, readable_instance_chirho,
    select_ground_instance_chirho,
};
use super::instance_rows_chirho::RowKeyAbsenceChirho;
use crate::dict_chirho::{DictLayoutChirho, DictPassDiagnosticChirho, SuperclassSlotChirho};
use crate::expr_chirho::{CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho};
use haskelujah_ast_chirho::name_chirho::CompilerClassChirho;
use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho, PredChirho};
use haskelujah_typing_chirho::ty_chirho::TyChirho;

/// The pre-pass's state across one module: its budget, and the attempt in progress.
#[derive(Default)]
struct GroundBuildChirho {
    budget_chirho: GroundBudgetChirho,
    /// Dictionaries allocated and not yet bound, outermost first.
    in_progress_chirho: Vec<CoreIdChirho>,
    /// Dictionaries a revisit reached while they were still being built: each is recursive.
    recursive_chirho: HashSet<CoreIdChirho>,
    /// The goals this attempt memoized, removed again if it fails.
    attempt_goals_chirho: Vec<PredChirho>,
    /// Top-level goals already decided: not new to this route (`None`), or refused.
    settled_chirho: HashMap<PredChirho, Option<GroundRefusalChirho>>,
    /// The names other generators bound before this pre-pass: a row name only they answer is an
    /// alias or a stub, never an instance's own body.
    generated_names_chirho: HashSet<String>,
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
        let mut build_chirho = GroundBuildChirho {
            generated_names_chirho: self
                .generated_bindings_chirho
                .iter()
                .map(|binding_chirho| binding_chirho.binder_chirho.name_chirho.clone())
                .collect(),
            ..GroundBuildChirho::default()
        };
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
                let slot_chirho = (occurrence_chirho, goal_chirho.class_name_chirho.clone());
                if let Some(dict_id_chirho) = self.ground_dicts_by_goal_chirho.get(goal_chirho) {
                    self.ground_goal_dicts_chirho
                        .insert(slot_chirho, *dict_id_chirho);
                    continue;
                }
                let settled_chirho = build_chirho.settled_chirho.get(goal_chirho).cloned();
                let decision_chirho = match settled_chirho {
                    Some(settled_chirho) => settled_chirho.map(Err),
                    None => {
                        let attempt_chirho = self.ground_dictionary_attempt_chirho(
                            class_env_chirho,
                            goal_chirho,
                            &mut build_chirho,
                        );
                        match &attempt_chirho {
                            Ok(Some(_)) => {}
                            Ok(None) => {
                                build_chirho
                                    .settled_chirho
                                    .insert(goal_chirho.clone(), None);
                            }
                            Err(refusal_chirho) => {
                                build_chirho
                                    .settled_chirho
                                    .insert(goal_chirho.clone(), Some(refusal_chirho.clone()));
                            }
                        }
                        attempt_chirho.transpose()
                    }
                };
                match decision_chirho {
                    None => {}
                    Some(Ok(dict_id_chirho)) => {
                        self.ground_goal_dicts_chirho
                            .insert(slot_chirho, dict_id_chirho);
                    }
                    Some(Err(refusal_chirho)) => {
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

    /// One top-level goal: its dictionary, `None` when the goal is not new to this route (no
    /// instance, several, or one without a context keeps its existing routes), or a refusal.
    /// On a refusal, every binding and memo entry this attempt made is removed.
    fn ground_dictionary_attempt_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        goal_chirho: &PredChirho,
        build_chirho: &mut GroundBuildChirho,
    ) -> Result<Option<CoreIdChirho>, GroundRefusalChirho> {
        let bindings_before_chirho = self.generated_bindings_chirho.len();
        let attempt_chirho =
            self.top_level_dictionary_chirho(class_env_chirho, goal_chirho, build_chirho);
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

    fn top_level_dictionary_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        goal_chirho: &PredChirho,
        build_chirho: &mut GroundBuildChirho,
    ) -> Result<Option<CoreIdChirho>, GroundRefusalChirho> {
        let goal_weight_chirho = self.examine_goal_chirho(goal_chirho, 0, build_chirho)?;
        let selected_chirho = match select_ground_instance_chirho(
            class_env_chirho,
            goal_chirho,
            goal_weight_chirho,
            &mut build_chirho.budget_chirho,
        ) {
            Ok(selected_chirho) if !selected_chirho.0.context_chirho.is_empty() => selected_chirho,
            Ok(_)
            | Err(GroundRefusalChirho::UnresolvedChirho(_))
            | Err(GroundRefusalChirho::OverlapChirho(..)) => return Ok(None),
            Err(refusal_chirho) => return Err(refusal_chirho),
        };
        self.build_selected_chirho(
            class_env_chirho,
            goal_chirho,
            selected_chirho,
            0,
            build_chirho,
        )
        .map(Some)
    }

    /// The checks every examined goal passes, with its weight: the visit and depth bounds, a
    /// ground one-parameter goal of a class the pass lays out, and a weight one goal may have.
    fn examine_goal_chirho(
        &self,
        goal_chirho: &PredChirho,
        depth_chirho: usize,
        build_chirho: &mut GroundBuildChirho,
    ) -> Result<usize, GroundRefusalChirho> {
        build_chirho.budget_chirho.visit_chirho(depth_chirho)?;
        if !pred_is_ground_chirho(goal_chirho) {
            return Err(GroundRefusalChirho::OpenObligationChirho(Box::new(
                goal_chirho.clone(),
            )));
        }
        if !goal_chirho.extra_tys_chirho.is_empty() {
            return Err(GroundRefusalChirho::MultiParameterChirho(Box::new(
                goal_chirho.clone(),
            )));
        }
        if !self
            .layouts_chirho
            .contains_key(&goal_chirho.class_name_chirho)
        {
            return Err(GroundRefusalChirho::NoLayoutChirho(
                goal_chirho.class_name_chirho.clone(),
            ));
        }
        build_chirho.budget_chirho.goal_weight_chirho(goal_chirho)
    }

    /// A dictionary already bound for exactly this goal: one built here, or one filed under
    /// the goal's own spelling.
    fn bound_dictionary_chirho(&self, goal_chirho: &PredChirho) -> Option<CoreIdChirho> {
        self.ground_dicts_by_goal_chirho
            .get(goal_chirho)
            .copied()
            .or_else(|| self.registry_dictionary_chirho(goal_chirho))
    }

    /// The dictionary a displaced compiler superclass slot holds for `goal_chirho`: the class's
    /// own validated template for that slot, instantiated at the goal's whole head and proving
    /// the slot's class, served by its namespace's dictionary at that superclass's registry key;
    /// otherwise its named refusal. The spelling's instances are never consulted (row 692, C10,
    /// gpt #32919).
    fn displaced_ground_superclass_chirho(
        &self,
        class_env_chirho: &ClassEnvChirho,
        goal_chirho: &PredChirho,
        slot_chirho: &SuperclassSlotChirho,
        identity_chirho: CompilerClassChirho,
    ) -> Result<CoreIdChirho, GroundRefusalChirho> {
        let refusal_chirho = || {
            GroundRefusalChirho::NoGroundDictionaryChirho(format!(
                "the compiler's `{}` superclass of `{goal_chirho}`, while the program's own class takes its spelling",
                identity_chirho.legacy_name_chirho()
            ))
        };
        let superclass_chirho = class_env_chirho
            .classes_chirho()
            .get_chirho(&goal_chirho.class_name_chirho)
            .and_then(|class_chirho| {
                class_chirho.superclass_preds_at_chirho(
                    &goal_chirho.kind_tys_chirho,
                    &goal_chirho.all_tys_chirho(),
                )
            })
            .and_then(|superclasses_chirho| {
                superclasses_chirho
                    .into_iter()
                    .nth(slot_chirho.index_chirho)
            })
            .filter(|superclass_chirho| {
                superclass_chirho.class_identity_chirho() == &slot_chirho.class_proof_chirho
            })
            .ok_or_else(refusal_chirho)?;
        self.compiler_superclass_id_chirho(
            identity_chirho,
            &goal_registry_key_chirho(&superclass_chirho),
        )
        .ok_or_else(refusal_chirho)
    }

    /// The dictionary an existing generator filed under exactly this goal's spelling.
    fn registry_dictionary_chirho(&self, goal_chirho: &PredChirho) -> Option<CoreIdChirho> {
        self.instance_dicts_chirho
            .get(&(
                goal_chirho.class_name_chirho.clone(),
                goal_registry_key_chirho(goal_chirho),
            ))
            .copied()
    }

    /// The dictionary for one obligation: the one built here, or one built from the instance
    /// its head selects when that instance's rows are this module's. A dictionary an existing
    /// generator filed under the obligation's own spelling serves only where this route cannot
    /// build: no unique instance, or rows that live elsewhere. The generic list specialization
    /// files a stub for a user instance whose rows it cannot find by spelling.
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
        let goal_weight_chirho =
            self.examine_goal_chirho(goal_chirho, depth_chirho, build_chirho)?;
        let (instance_chirho, obligations_chirho) = match select_ground_instance_chirho(
            class_env_chirho,
            goal_chirho,
            goal_weight_chirho,
            &mut build_chirho.budget_chirho,
        ) {
            Ok(selected_chirho) => selected_chirho,
            Err(
                refusal_chirho @ (GroundRefusalChirho::UnresolvedChirho(_)
                | GroundRefusalChirho::OverlapChirho(..)),
            ) => {
                return self
                    .registry_dictionary_chirho(goal_chirho)
                    .ok_or(refusal_chirho);
            }
            Err(refusal_chirho) => return Err(refusal_chirho),
        };
        if instance_chirho.context_chirho.is_empty() {
            // A context-free instance has its dictionary already, under its own key.
            return self
                .instance_dicts_chirho
                .get(&(
                    goal_chirho.class_name_chirho.clone(),
                    instance_registry_key_chirho(instance_chirho),
                ))
                .copied()
                .ok_or_else(|| {
                    GroundRefusalChirho::NoGroundDictionaryChirho(readable_instance_chirho(
                        instance_chirho,
                    ))
                });
        }
        match self.build_selected_chirho(
            class_env_chirho,
            goal_chirho,
            (instance_chirho, obligations_chirho),
            depth_chirho,
            build_chirho,
        ) {
            Err(
                refusal_chirho @ (GroundRefusalChirho::RowsElsewhereChirho(_)
                | GroundRefusalChirho::RowsAmbiguousChirho(_)),
            ) => self
                .registry_dictionary_chirho(goal_chirho)
                .ok_or(refusal_chirho),
            built_chirho => built_chirho,
        }
    }

    /// Build the dictionary of a selected context instance at `goal_chirho`: its context
    /// dictionaries, its superclass dictionaries at the same type, and its rows applied to the
    /// context dictionaries.
    fn build_selected_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        goal_chirho: &PredChirho,
        (instance_chirho, obligations_chirho): (&InstDeclChirho, Vec<PredChirho>),
        depth_chirho: usize,
        build_chirho: &mut GroundBuildChirho,
    ) -> Result<CoreIdChirho, GroundRefusalChirho> {
        let class_name_chirho = goal_chirho.class_name_chirho.clone();
        let Some(layout_chirho) = self.layouts_chirho.get(&class_name_chirho).cloned() else {
            return Err(GroundRefusalChirho::NoLayoutChirho(class_name_chirho));
        };
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
        if !self.instance_rows_are_bound_chirho(
            &class_name_chirho,
            &row_key_chirho,
            &layout_chirho,
            &build_chirho.generated_names_chirho,
        ) {
            return Err(GroundRefusalChirho::RowsElsewhereChirho(
                readable_instance_chirho(instance_chirho),
            ));
        }
        let goal_constructors_chirho = pred_constructors_chirho(goal_chirho);
        for obligation_chirho in &obligations_chirho {
            let unproven_chirho = pred_constructors_chirho(obligation_chirho)
                .into_iter()
                .find(|constructor_chirho| !goal_constructors_chirho.contains(constructor_chirho));
            if let Some(unproven_chirho) = unproven_chirho
                && self.bound_dictionary_chirho(obligation_chirho).is_none()
            {
                return Err(GroundRefusalChirho::UnprovenConstructorChirho(
                    obligation_chirho.to_string(),
                    unproven_chirho.to_string(),
                ));
            }
        }
        build_chirho.budget_chirho.dictionary_chirho()?;
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
        for slot_chirho in &layout_chirho.super_slots_chirho {
            let super_id_chirho =
                match self.displaced_identity_chirho(&slot_chirho.class_proof_chirho) {
                    Some(identity_chirho) => self.displaced_ground_superclass_chirho(
                        class_env_chirho,
                        goal_chirho,
                        slot_chirho,
                        identity_chirho,
                    )?,
                    None => {
                        let super_goal_chirho = PredChirho::new_chirho(
                            &slot_chirho.namespace_chirho,
                            goal_chirho.ty_chirho.clone(),
                        );
                        self.ground_dictionary_chirho(
                            class_env_chirho,
                            &super_goal_chirho,
                            depth_chirho + 1,
                            build_chirho,
                        )?
                    }
                };
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

    /// Whether the instance's rows are this module's own under its declaration's key: at least
    /// one row the declaration wrote or the desugarer bound for it, or no methods at all. A slot
    /// that only another generator's binding answers refuses the instance: a newtype alias
    /// (`generate_gnd_dicts_chirho`) shares the written key, so counting it would let an alias
    /// or a stub stand in for the instance's body. Rows that live elsewhere are never invented
    /// here.
    fn instance_rows_are_bound_chirho(
        &self,
        class_name_chirho: &str,
        row_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
        generated_names_chirho: &HashSet<String>,
    ) -> bool {
        let mut written_chirho = layout_chirho.method_slots_chirho.is_empty();
        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            let row_name_chirho =
                format!("$prim_{class_name_chirho}_{method_name_chirho}_{row_key_chirho}");
            if self.body_backed_names_chirho.contains_key(&row_name_chirho) {
                written_chirho = true;
            } else if generated_names_chirho.contains(&row_name_chirho) {
                return false;
            }
        }
        written_chirho
    }
}
