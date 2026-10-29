// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Schemes: fresh variables, instantiation, deferred predicates, givens and generalization.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::{FundepGeneralizationChirho, GeneralizationScopeChirho};
use crate::class_chirho::PredChirho;
use crate::skolem_chirho::contains_skolem_chirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};
use haskelujah_span_chirho::SpanChirho;
use std::collections::{HashMap, HashSet};

impl InferCtxChirho {
    /// Generate a fresh unification variable.
    pub fn fresh_var_chirho(&mut self) -> TyChirho {
        let var_chirho = TyVarChirho(self.next_var_chirho);
        self.next_var_chirho += 1;
        TyChirho::VarChirho(var_chirho)
    }

    pub(super) fn instantiate_scheme_parts_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
    ) -> (TyChirho, Vec<PredChirho>) {
        if scheme_chirho.vars_chirho.is_empty() && scheme_chirho.preds_chirho.is_empty() {
            return (scheme_chirho.ty_chirho.clone(), Vec::new());
        }

        let mut subst_chirho = SubstChirho::empty_chirho();
        for v_chirho in &scheme_chirho.vars_chirho {
            let fresh_chirho = self.fresh_var_chirho();
            if self.local_generalized_vars_chirho.contains(v_chirho)
                && let TyChirho::VarChirho(fresh_var_chirho) = fresh_chirho
            {
                self.local_instantiations_chirho
                    .entry(*v_chirho)
                    .or_default()
                    .push(fresh_var_chirho);
            }
            subst_chirho.insert_chirho(*v_chirho, fresh_chirho);
        }

        let instantiated_preds_chirho: Vec<PredChirho> = scheme_chirho
            .preds_chirho
            .iter()
            .map(|pred_chirho| {
                let instantiated_ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
                let instantiated_extra_chirho: Vec<TyChirho> = pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
                    .collect();
                let mut pred_inst_chirho =
                    PredChirho::new_chirho(&pred_chirho.class_name_chirho, instantiated_ty_chirho);
                pred_inst_chirho.extra_tys_chirho = instantiated_extra_chirho;
                pred_inst_chirho
            })
            .collect();

        (
            subst_chirho.apply_ty_chirho(&scheme_chirho.ty_chirho),
            instantiated_preds_chirho,
        )
    }

    pub(super) fn instantiate_scheme_parts_with_type_args_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        explicit_tys_chirho: &[TyChirho],
    ) -> Option<(TyChirho, Vec<PredChirho>)> {
        let scheme_var_count_chirho = scheme_chirho.vars_chirho.len();
        let nested_arg_count_chirho = explicit_tys_chirho
            .len()
            .saturating_sub(scheme_var_count_chirho);
        let (nested_vars_chirho, nested_body_chirho) = if nested_arg_count_chirho > 0 {
            let (vars_chirho, body_chirho) =
                Self::peel_forall_chirho(scheme_chirho.ty_chirho.clone());
            if nested_arg_count_chirho > vars_chirho.len() {
                return None;
            }
            (vars_chirho, Some(body_chirho))
        } else {
            (Vec::new(), None)
        };

        let mut subst_chirho = SubstChirho::empty_chirho();
        for (index_chirho, v_chirho) in scheme_chirho.vars_chirho.iter().enumerate() {
            let inst_ty_chirho = explicit_tys_chirho
                .get(index_chirho)
                .cloned()
                .unwrap_or_else(|| self.fresh_var_chirho());
            subst_chirho.insert_chirho(*v_chirho, inst_ty_chirho);
        }
        for (index_chirho, v_chirho) in nested_vars_chirho.iter().enumerate() {
            let inst_ty_chirho = explicit_tys_chirho
                .get(scheme_var_count_chirho + index_chirho)
                .cloned()
                .unwrap_or_else(|| self.fresh_var_chirho());
            subst_chirho.insert_chirho(*v_chirho, inst_ty_chirho);
        }

        let instantiated_preds_chirho: Vec<PredChirho> = scheme_chirho
            .preds_chirho
            .iter()
            .map(|pred_chirho| {
                let instantiated_ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
                let instantiated_extra_chirho: Vec<TyChirho> = pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
                    .collect();
                let mut pred_inst_chirho =
                    PredChirho::new_chirho(&pred_chirho.class_name_chirho, instantiated_ty_chirho);
                pred_inst_chirho.extra_tys_chirho = instantiated_extra_chirho;
                pred_inst_chirho
            })
            .collect();

        // A rank-N parameter keeps its quantifier inside `TyChirho`, rather
        // than lifting it into the local scheme. Visible type application may
        // consume that quantifier only while it is outermost; peeling here
        // cannot skip a function arrow or enter a supplied polymorphic type.
        // workflow: language-features-chirho/rank-n-visible-type-application-chirho
        let result_ty_chirho = nested_body_chirho
            .as_ref()
            .unwrap_or(&scheme_chirho.ty_chirho);
        Some((
            subst_chirho.apply_ty_chirho(result_ty_chirho),
            instantiated_preds_chirho,
        ))
    }

    /// Instantiate a type scheme with fresh unification variables.
    /// Any predicates on the scheme are also instantiated and deferred.
    pub fn instantiate_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        span_chirho: SpanChirho,
    ) -> TyChirho {
        let (instantiated_ty_chirho, instantiated_preds_chirho) =
            self.instantiate_scheme_parts_chirho(scheme_chirho);
        for pred_inst_chirho in instantiated_preds_chirho {
            self.defer_pred_chirho(pred_inst_chirho, span_chirho);
        }
        instantiated_ty_chirho
    }

    /// Defer a wanted predicate: an equality (`~`) becomes a deferred
    /// equality to be unified once its stuck side is known, anything else a
    /// deferred class constraint. Every instantiation site goes through here.
    /// workflow: language-features-chirho/stuck-family-equalities-chirho
    pub(super) fn defer_pred_chirho(&mut self, pred_chirho: PredChirho, span_chirho: SpanChirho) {
        if pred_chirho.class_name_chirho == "~" {
            if let Some(rhs_chirho) = pred_chirho.extra_tys_chirho.first() {
                self.deferred_equalities_chirho.push((
                    pred_chirho.ty_chirho.clone(),
                    rhs_chirho.clone(),
                    span_chirho,
                ));
            }
            return;
        }
        self.deferred_preds_chirho.push((pred_chirho, span_chirho));
    }

    pub(super) fn with_given_preds_chirho<TChirho, FChirho>(
        &mut self,
        preds_chirho: Vec<PredChirho>,
        f_chirho: FChirho,
    ) -> TChirho
    where
        FChirho: FnOnce(&mut Self) -> TChirho,
    {
        let given_start_chirho = self.given_preds_chirho.len();
        let rewrites_start_chirho = self.given_rewrites_chirho.len();
        self.install_given_equalities_chirho(&preds_chirho);
        self.given_preds_chirho.extend(preds_chirho);
        let result_chirho = f_chirho(self);
        let _improvement_chirho = self.discharge_deferred_with_current_givens_chirho();
        let _equalities_chirho = self.retry_deferred_equalities_chirho();
        self.given_preds_chirho.truncate(given_start_chirho);
        self.given_rewrites_chirho.truncate(rewrites_start_chirho);
        result_chirho
    }

    /// Generalize a type into a scheme by quantifying over all free variables
    /// not free in the environment. Also partitions deferred predicates:
    /// predicates whose type variables are all being generalized become part
    /// of the scheme; the rest remain deferred.
    ///
    /// Deferred predicates are assumed to already have substitutions applied
    /// (via `apply_subst_all_chirho` during inference).
    pub fn generalize_chirho(&mut self, ty_chirho: &TyChirho) -> SchemeChirho {
        self.generalize_with_io_defaulting_chirho(
            ty_chirho,
            true,
            GeneralizationScopeChirho::TopLevelChirho,
        )
    }

    pub(super) fn generalize_local_chirho(&mut self, ty_chirho: &TyChirho) -> SchemeChirho {
        self.generalize_with_io_defaulting_chirho(
            ty_chirho,
            false,
            GeneralizationScopeChirho::NestedChirho,
        )
    }

    pub(super) fn generalize_with_io_defaulting_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        allow_io_defaulting_chirho: bool,
        scope_chirho: GeneralizationScopeChirho,
    ) -> SchemeChirho {
        let mut env_fvs_chirho = self.env_chirho.free_vars_chirho();
        for pred_chirho in &self.given_preds_chirho {
            env_fvs_chirho.extend(pred_chirho.ty_chirho.free_vars_chirho());
            for extra_ty_chirho in &pred_chirho.extra_tys_chirho {
                env_fvs_chirho.extend(extra_ty_chirho.free_vars_chirho());
            }
        }
        // workflow: language-features-chirho/fundep-generalization-chirho
        let newly_mono_chirho = self.fundep_newly_mono_vars_chirho(&env_fvs_chirho);
        let ty_fvs_chirho = ty_chirho.free_vars_chirho();
        let vars_chirho: Vec<TyVarChirho> = ty_fvs_chirho
            .into_iter()
            .filter(|v_chirho| {
                !env_fvs_chirho.contains(v_chirho) && !newly_mono_chirho.contains(v_chirho)
            })
            .collect();
        let fundeps_chirho = FundepGeneralizationChirho {
            newly_mono_chirho,
            quantified_chirho: &vars_chirho,
            scope_chirho,
        };

        // Partition deferred predicates: those whose type vars are all being
        // generalized go into the scheme; the rest stay deferred.
        let mut scheme_preds_chirho = Vec::new();
        let mut remaining_chirho = Vec::new();
        let drained_preds_chirho: Vec<(PredChirho, SpanChirho)> =
            self.deferred_preds_chirho.drain(..).collect();
        for wanted_chirho in drained_preds_chirho {
            let Some((pred_chirho, span_chirho)) = self.place_fundep_wanted_chirho(
                wanted_chirho,
                &fundeps_chirho,
                &mut scheme_preds_chirho,
                &mut remaining_chirho,
            ) else {
                continue;
            };
            let pred_fvs_chirho = pred_chirho.ty_chirho.free_vars_chirho();
            // A predicate over a rigid signature variable belongs to the
            // enclosing signature's context: it is discharged by those givens
            // when their scope ends, or reported. It is never quantified away.
            let mentions_rigid_chirho = contains_skolem_chirho(&pred_chirho.ty_chirho)
                || pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .any(contains_skolem_chirho);
            if !mentions_rigid_chirho
                && pred_fvs_chirho
                    .iter()
                    .all(|v_chirho| vars_chirho.contains(v_chirho))
            {
                // A predicate with no free variables passes the check above
                // vacuously and used to be absorbed into the scheme unchecked
                // — silently accepting programs GHC rejects (`Eq U` with no
                // instance anywhere in the defining module). Groundness makes
                // the verdict final: no later substitution can change it.
                if pred_fvs_chirho.is_empty()
                    && self.report_certainly_unsolvable_ground_chirho(&pred_chirho, span_chirho)
                {
                    continue;
                }
                scheme_preds_chirho.push(SchemePredChirho {
                    class_name_chirho: pred_chirho.class_name_chirho,
                    ty_chirho: pred_chirho.ty_chirho,
                    extra_tys_chirho: pred_chirho.extra_tys_chirho,
                });
            } else {
                remaining_chirho.push((pred_chirho, span_chirho));
            }
        }
        self.deferred_preds_chirho = remaining_chirho;

        // Deduplicate predicates
        scheme_preds_chirho.dedup();

        // Context reduction: remove predicates that are entailed by
        // other predicates via the superclass hierarchy.
        // E.g. if we have (Eq a, Num a), remove Eq a since Num has Eq
        // as a superclass.
        let keep_chirho: Vec<bool> = scheme_preds_chirho
            .iter()
            .map(|pred_chirho| {
                !scheme_preds_chirho.iter().any(|other_chirho| {
                    other_chirho.class_name_chirho != pred_chirho.class_name_chirho
                        && other_chirho.ty_chirho == pred_chirho.ty_chirho
                        && self.is_transitive_superclass_chirho(
                            &pred_chirho.class_name_chirho,
                            &other_chirho.class_name_chirho,
                        )
                })
            })
            .collect();
        scheme_preds_chirho = scheme_preds_chirho
            .into_iter()
            .zip(keep_chirho)
            .filter(|(_, keep_chirho)| *keep_chirho)
            .map(|(p_chirho, _)| p_chirho)
            .collect();

        // Evidence-threading P2: top-level generalization can absorb
        // `(Num a, Fractional a)` into the binding scheme before module-end
        // defaulting sees it. Preserve the fractional evidence for any method
        // occurrence captured on the same bare variable so `negate 5.5`
        // dispatches through the Double row instead of the class-only Int
        // fallback used for truly unconstrained methods.
        let fractional_default_classes_chirho: &[&str] =
            &["Fractional", "Floating", "RealFrac", "RealFloat"];
        let mut pred_classes_by_var_chirho: HashMap<TyVarChirho, Vec<&str>> = HashMap::new();
        for pred_chirho in &scheme_preds_chirho {
            if let TyChirho::VarChirho(var_chirho) = &pred_chirho.ty_chirho {
                pred_classes_by_var_chirho
                    .entry(*var_chirho)
                    .or_default()
                    .push(pred_chirho.class_name_chirho.as_str());
            }
        }
        for (var_chirho, classes_chirho) in pred_classes_by_var_chirho {
            if classes_chirho
                .iter()
                .any(|class_chirho| fractional_default_classes_chirho.contains(class_chirho))
            {
                self.occurrence_default_hints_chirho
                    .insert(var_chirho, "Double".to_string());
            } else {
                self.occurrence_generalized_vars_chirho.insert(var_chirho);
            }
        }

        // IO defaulting: when a predicate is Monad/Applicative/Functor on a
        // bare type variable with no other non-IO-defaultable constraints,
        // default the variable to IO (like GHC's ExtendedDefaultRules).
        // This ensures `f = return 42` gets type `IO Int` rather than
        // `forall m. Monad m => m Int`, avoiding dictionary abstraction.
        let io_defaultable_chirho: &[&str] = &[
            "Monad",
            "Applicative",
            "Functor",
            "MonadIO",
            "MonadFail",
            "MonadFix",
        ];
        let mut io_default_subst_chirho = SubstChirho::empty_chirho();
        // Group predicates by type variable
        let mut var_pred_classes_chirho: std::collections::HashMap<TyVarChirho, Vec<&str>> =
            std::collections::HashMap::new();
        for pred_chirho in &scheme_preds_chirho {
            if let TyChirho::VarChirho(v_chirho) = &pred_chirho.ty_chirho {
                var_pred_classes_chirho
                    .entry(*v_chirho)
                    .or_default()
                    .push(&pred_chirho.class_name_chirho);
            }
        }
        // DETERMINISM: builds the IO-defaulting substitution — same hazard as the
        // defaulting loop in check_deferred_preds_chirho. Sort so the substitution is a
        // function of the program, not of the hasher.
        let mut sorted_var_pred_classes_chirho: Vec<_> = var_pred_classes_chirho.iter().collect();
        sorted_var_pred_classes_chirho.sort_by_key(|(var_chirho, _)| **var_chirho);
        for (var_chirho, classes_chirho) in sorted_var_pred_classes_chirho {
            let all_io_chirho = classes_chirho
                .iter()
                .all(|c_chirho| io_defaultable_chirho.contains(c_chirho));
            if all_io_chirho && !classes_chirho.is_empty() {
                io_default_subst_chirho
                    .insert_chirho(*var_chirho, TyChirho::ConChirho("IO".to_string()));
            }
        }
        if allow_io_defaulting_chirho && !io_default_subst_chirho.is_empty_chirho() {
            // Apply IO defaulting: substitute in the type, remove defaulted
            // vars, and remove satisfied predicates.
            let defaulted_ty_chirho = io_default_subst_chirho.apply_ty_chirho(ty_chirho);
            let defaulted_vars_chirho: Vec<TyVarChirho> = vars_chirho
                .into_iter()
                .filter(|v_chirho| io_default_subst_chirho.lookup_chirho(v_chirho).is_none())
                .collect();
            let defaulted_preds_chirho: Vec<SchemePredChirho> = scheme_preds_chirho
                .into_iter()
                .filter(|p_chirho| {
                    if let TyChirho::VarChirho(v_chirho) = &p_chirho.ty_chirho {
                        io_default_subst_chirho.lookup_chirho(v_chirho).is_none()
                    } else {
                        true
                    }
                })
                .collect();
            return SchemeChirho {
                vars_chirho: defaulted_vars_chirho,
                preds_chirho: defaulted_preds_chirho,
                ty_chirho: defaulted_ty_chirho,
            };
        }

        SchemeChirho {
            vars_chirho,
            preds_chirho: scheme_preds_chirho,
            ty_chirho: ty_chirho.clone(),
        }
    }

    /// Check if `potential_super_chirho` is a (transitive) superclass of
    /// `sub_chirho`. For example, `Eq` is a superclass of `Ord` and also
    /// of `Num` (since Num has Eq in its superclass list). Each class is
    /// visited once, so a superclass cycle, legal under
    /// UndecidableSuperClasses (T16414's `All` and `All2`), ends the walk.
    /// workflow: language-features-chirho/rigid-type-variables-chirho
    pub(super) fn is_transitive_superclass_chirho(
        &self,
        potential_super_chirho: &str,
        sub_chirho: &str,
    ) -> bool {
        let mut visited_chirho = HashSet::from([sub_chirho.to_string()]);
        let mut pending_chirho = vec![sub_chirho.to_string()];
        while let Some(class_name_chirho) = pending_chirho.pop() {
            for super_chirho in self
                .class_env_chirho
                .superclasses_chirho(&class_name_chirho)
            {
                if super_chirho == potential_super_chirho {
                    return true;
                }
                if visited_chirho.insert(super_chirho.clone()) {
                    pending_chirho.push(super_chirho);
                }
            }
        }
        false
    }

    /// Apply a substitution to the type environment and deferred predicates.
    pub(super) fn apply_subst_all_chirho(&mut self, subst_chirho: &SubstChirho) {
        self.env_chirho.apply_subst_chirho(subst_chirho);
        self.recursive_references_chirho
            .apply_subst_chirho(subst_chirho);
        // Every argument of a multi-parameter predicate must follow the
        // substitution, not only the primary one: a functional-dependency
        // improvement or a solved variable in an extra position would
        // otherwise be lost (`MyReader r v` with `v := Int`).
        for (pred_chirho, _span_chirho) in &mut self.deferred_preds_chirho {
            pred_chirho.ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            for extra_ty_chirho in &mut pred_chirho.extra_tys_chirho {
                *extra_ty_chirho = subst_chirho.apply_ty_chirho(extra_ty_chirho);
            }
        }
        for pred_chirho in &mut self.given_preds_chirho {
            pred_chirho.ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            for extra_ty_chirho in &mut pred_chirho.extra_tys_chirho {
                *extra_ty_chirho = subst_chirho.apply_ty_chirho(extra_ty_chirho);
            }
        }
        for (left_chirho, right_chirho, _span_chirho) in &mut self.deferred_equalities_chirho {
            *left_chirho = subst_chirho.apply_ty_chirho(left_chirho);
            *right_chirho = subst_chirho.apply_ty_chirho(right_chirho);
        }
        for (lhs_chirho, rhs_chirho) in &mut self.given_rewrites_chirho {
            *lhs_chirho = subst_chirho.apply_ty_chirho(lhs_chirho);
            *rhs_chirho = subst_chirho.apply_ty_chirho(rhs_chirho);
        }
        for capture_chirho in &mut self.occurrence_captures_chirho {
            capture_chirho.ty_chirho = subst_chirho.apply_ty_chirho(&capture_chirho.ty_chirho);
        }
        if !self.occurrence_default_hints_chirho.is_empty() {
            let mut updated_hints_chirho = HashMap::new();
            // DETERMINISM: substitution can map two distinct vars onto the SAME
            // updated var, and the later insert then wins. Draining a HashMap
            // makes "later" depend on a per-process hash seed, so the surviving
            // defaulting hint — and with it the inferred type — could differ
            // between runs of the same binary on the same source.
            // See spec-chirho/bug-nondeterministic-typecheck-chirho.md
            let mut drained_hints_chirho: Vec<_> =
                self.occurrence_default_hints_chirho.drain().collect();
            drained_hints_chirho.sort_by_key(|(var_chirho, _)| *var_chirho);
            for (var_chirho, key_chirho) in drained_hints_chirho {
                if let TyChirho::VarChirho(updated_var_chirho) =
                    subst_chirho.apply_ty_chirho(&TyChirho::VarChirho(var_chirho))
                {
                    updated_hints_chirho.insert(updated_var_chirho, key_chirho);
                }
            }
            self.occurrence_default_hints_chirho = updated_hints_chirho;
        }
        if !self.occurrence_generalized_vars_chirho.is_empty() {
            let mut updated_vars_chirho = HashSet::new();
            for var_chirho in self.occurrence_generalized_vars_chirho.drain() {
                if let TyChirho::VarChirho(updated_var_chirho) =
                    subst_chirho.apply_ty_chirho(&TyChirho::VarChirho(var_chirho))
                {
                    updated_vars_chirho.insert(updated_var_chirho);
                }
            }
            self.occurrence_generalized_vars_chirho = updated_vars_chirho;
        }
        for pred_chirho in &mut self.given_preds_chirho {
            pred_chirho.ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            pred_chirho.extra_tys_chirho = pred_chirho
                .extra_tys_chirho
                .iter()
                .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
                .collect();
        }
    }
}
