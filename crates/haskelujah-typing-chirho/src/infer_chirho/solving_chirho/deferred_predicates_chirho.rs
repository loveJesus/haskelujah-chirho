// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The predicates deferred during inference, checked once the module is inferred.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::UNSATISFIED_CONSTRAINT_CODE_CHIRHO;
use crate::class_chirho::PredChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

impl InferCtxChirho {
    pub(super) fn report_deferred_numeric_bool_predicates_chirho(&mut self) {
        for (pred_chirho, pred_span_chirho) in &self.deferred_preds_chirho {
            if pred_chirho.ty_chirho == TyChirho::bool_chirho()
                && matches!(
                    pred_chirho.class_name_chirho.as_str(),
                    "Num"
                        | "Integral"
                        | "Fractional"
                        | "Floating"
                        | "Real"
                        | "RealFrac"
                        | "RealFloat"
                )
            {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                        format!("no instance for `{} Bool`", pred_chirho.class_name_chirho),
                        *pred_span_chirho,
                    ));
            }
        }
    }

    /// Check remaining deferred predicates. Those that the class environment
    /// can fully entail are discharged; those that can't produce diagnostics.
    ///
    /// Implements Haskell-style numeric defaulting: ambiguous type variables
    /// constrained only by defaultable classes are resolved to `Integer` (for
    /// Num/Integral/etc.) or `Double` (for Fractional/Floating/etc.) before
    /// the final constraint check.
    pub(super) fn check_deferred_preds_chirho(
        &mut self,
        final_subst_chirho: &SubstChirho,
    ) -> SubstChirho {
        let preds_chirho: Vec<(PredChirho, SpanChirho)> =
            self.deferred_preds_chirho.drain(..).collect();
        let resolved_given_preds_chirho: Vec<PredChirho> = self
            .given_preds_chirho
            .iter()
            .map(|pred_chirho| PredChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: self.normalize_ty_chirho(
                    &final_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                ),
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| {
                        self.normalize_ty_chirho(&final_subst_chirho.apply_ty_chirho(t_chirho))
                    })
                    .collect(),
            })
            .collect();

        let defaultable_classes_chirho: &[&str] = &[
            "Num",
            "Integral",
            "Enum",
            "Bounded",
            "Eq",
            "Ord",
            "Show",
            "Read",
            "Real",
            "Fractional",
            "Floating",
            "RealFrac",
            "RealFloat",
        ];
        let numeric_default_trigger_classes_chirho: &[&str] = &[
            "Num",
            "Integral",
            "Real",
            "Fractional",
            "Floating",
            "RealFrac",
            "RealFloat",
        ];
        let fractional_classes_chirho: &[&str] =
            &["Fractional", "Floating", "RealFrac", "RealFloat"];

        // Phase 1: resolve all predicates and collect constraints per type variable
        let mut resolved_preds_chirho: Vec<(PredChirho, SpanChirho)> = Vec::new();
        let mut var_classes_chirho: std::collections::HashMap<TyVarChirho, Vec<String>> =
            std::collections::HashMap::new();

        for (pred_chirho, span_chirho) in &preds_chirho {
            let resolved_ty_chirho = final_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            let resolved_extra_tys_chirho: Vec<TyChirho> = pred_chirho
                .extra_tys_chirho
                .iter()
                .map(|t_chirho| final_subst_chirho.apply_ty_chirho(t_chirho))
                .collect();
            let resolved_ty_chirho = self.normalize_ty_chirho(&resolved_ty_chirho);
            let resolved_extra_tys_chirho: Vec<TyChirho> = resolved_extra_tys_chirho
                .into_iter()
                .map(|t_chirho| self.normalize_ty_chirho(&t_chirho))
                .collect();
            let mut resolved_pred_chirho = PredChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: resolved_ty_chirho,
                extra_tys_chirho: resolved_extra_tys_chirho,
            };

            // Apply functional dependency improvement
            let improvement_chirho = self
                .class_env_chirho
                .fundep_improve_chirho(&resolved_pred_chirho);
            if !improvement_chirho.is_empty_chirho() {
                resolved_pred_chirho.ty_chirho =
                    improvement_chirho.apply_ty_chirho(&resolved_pred_chirho.ty_chirho);
                resolved_pred_chirho.extra_tys_chirho = resolved_pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| improvement_chirho.apply_ty_chirho(t_chirho))
                    .collect();
            }

            // Collect bare type-variable constraints for defaulting
            if let TyChirho::VarChirho(v_chirho) = &resolved_pred_chirho.ty_chirho {
                if self.pred_entailed_by_givens_chirho(
                    &resolved_pred_chirho,
                    &resolved_given_preds_chirho,
                ) {
                    continue;
                }
                var_classes_chirho
                    .entry(*v_chirho)
                    .or_default()
                    .push(resolved_pred_chirho.class_name_chirho.clone());
            }
            if let Some(sub_goals_chirho) =
                self.class_env_chirho.resolve_chirho(&resolved_pred_chirho)
            {
                for sub_goal_chirho in sub_goals_chirho {
                    if self.pred_entailed_by_givens_chirho(
                        &sub_goal_chirho,
                        &resolved_given_preds_chirho,
                    ) {
                        continue;
                    }
                    if let TyChirho::VarChirho(v_chirho) = &sub_goal_chirho.ty_chirho {
                        var_classes_chirho
                            .entry(*v_chirho)
                            .or_default()
                            .push(sub_goal_chirho.class_name_chirho.clone());
                    }
                }
            }

            resolved_preds_chirho.push((resolved_pred_chirho, *span_chirho));
        }

        // Phase 2: compute defaults for ambiguous type variables
        let mut default_subst_chirho = SubstChirho::empty_chirho();
        let io_defaultable_classes_chirho: &[&str] = &[
            "Monad",
            "Applicative",
            "Functor",
            "MonadIO",
            "MonadFail",
            "MonadFix",
        ];
        // DETERMINISM: this loop BUILDS a defaulting substitution, so the order in
        // which ambiguous variables are defaulted decides the inferred types. Iterating
        // a HashMap made that order depend on a per-process hash seed.
        // See spec-chirho/bug-nondeterministic-typecheck-chirho.md
        let mut sorted_var_classes_chirho: Vec<_> = var_classes_chirho.iter().collect();
        sorted_var_classes_chirho.sort_by_key(|(var_chirho, _)| **var_chirho);
        for (var_chirho, classes_chirho) in sorted_var_classes_chirho {
            // Check if all constraints are IO-defaultable (Monad/Applicative/Functor)
            let all_io_defaultable_chirho = classes_chirho
                .iter()
                .all(|c_chirho| io_defaultable_classes_chirho.contains(&c_chirho.as_str()));
            if all_io_defaultable_chirho && !classes_chirho.is_empty() {
                default_subst_chirho
                    .insert_chirho(*var_chirho, TyChirho::ConChirho("IO".to_string()));
                continue;
            }
            let has_numeric_trigger_chirho = classes_chirho.iter().any(|c_chirho| {
                numeric_default_trigger_classes_chirho.contains(&c_chirho.as_str())
            });
            // Haskelujah leniency: if a literal-driven type variable also has
            // user-class constraints, prefer Int only when Int satisfies every
            // collected class. This keeps ordinary Num-only defaulting on the
            // Report path while accepting small local examples like
            // `Describable a => Describable [a]` with `[42]`.
            let all_defaultable_chirho = classes_chirho
                .iter()
                .all(|c_chirho| defaultable_classes_chirho.contains(&c_chirho.as_str()));
            if !all_defaultable_chirho {
                if has_numeric_trigger_chirho {
                    let int_ty_chirho = TyChirho::int_chirho();
                    let int_satisfies_all_chirho = classes_chirho.iter().all(|class_name_chirho| {
                        self.class_env_chirho
                            .entails_chirho(&PredChirho::new_chirho(
                                class_name_chirho,
                                int_ty_chirho.clone(),
                            ))
                    });
                    if int_satisfies_all_chirho {
                        default_subst_chirho.insert_chirho(*var_chirho, int_ty_chirho);
                    }
                }
                continue;
            }
            if !has_numeric_trigger_chirho {
                continue;
            }
            if !self.module_default_types_chirho.is_empty() {
                let mut matched_default_chirho = None;
                for candidate_ty_chirho in &self.module_default_types_chirho {
                    let candidate_matches_chirho = classes_chirho.iter().all(|class_name_chirho| {
                        self.class_env_chirho
                            .resolve_chirho(&PredChirho::new_chirho(
                                class_name_chirho,
                                candidate_ty_chirho.clone(),
                            ))
                            .is_some()
                    });
                    if candidate_matches_chirho {
                        matched_default_chirho = Some(candidate_ty_chirho.clone());
                        break;
                    }
                }
                if let Some(default_ty_chirho) = matched_default_chirho {
                    default_subst_chirho.insert_chirho(*var_chirho, default_ty_chirho);
                    continue;
                }
            }
            // If any constraint is a Fractional-group class, default to Double;
            // otherwise default to Integer, matching Haskell's standard default
            // declaration.
            let needs_double_chirho = classes_chirho
                .iter()
                .any(|c_chirho| fractional_classes_chirho.contains(&c_chirho.as_str()));
            let default_ty_chirho = if needs_double_chirho {
                TyChirho::double_chirho()
            } else {
                TyChirho::ConChirho("Integer".to_string())
            };
            default_subst_chirho.insert_chirho(*var_chirho, default_ty_chirho);
        }

        // Phase 3: apply defaults and check all predicates
        for (pred_chirho, span_chirho) in resolved_preds_chirho {
            if self.pred_entailed_by_givens_chirho(&pred_chirho, &resolved_given_preds_chirho) {
                continue;
            }
            let integral_bounded_integer_default_chirho = match &pred_chirho.ty_chirho {
                TyChirho::VarChirho(var_chirho)
                    if pred_chirho.class_name_chirho == "Bounded"
                        && matches!(
                            default_subst_chirho.lookup_chirho(var_chirho),
                            Some(TyChirho::ConChirho(name_chirho)) if name_chirho == "Integer"
                        ) =>
                {
                    var_classes_chirho
                        .get(var_chirho)
                        .is_some_and(|classes_chirho| {
                            classes_chirho
                                .iter()
                                .any(|class_name_chirho| class_name_chirho == "Integral")
                        })
                }
                _ => false,
            };
            if integral_bounded_integer_default_chirho {
                continue;
            }
            let defaulted_pred_chirho = PredChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: default_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| default_subst_chirho.apply_ty_chirho(t_chirho))
                    .collect(),
            };

            // workflow: language-features-chirho/fundep-generalization-chirho
            if self.report_unsatisfiable_fundep_wanted_chirho(
                &defaulted_pred_chirho,
                final_subst_chirho,
                &resolved_given_preds_chirho,
                span_chirho,
            ) {
                continue;
            }

            // If still a bare type variable after defaulting, skip (truly ambiguous
            // but no concrete check is possible).
            if matches!(defaulted_pred_chirho.ty_chirho, TyChirho::VarChirho(_)) {
                continue;
            }

            // Certainly-unsolvable predicates are errors even when the class
            // has no registered instances: for a fully-known class the empty
            // instance list IS the whole universe. Apply the final
            // substitution to extras too (apply_subst_all_chirho maintains
            // only the primary type during inference).
            let fully_resolved_pred_chirho = PredChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: defaulted_pred_chirho.class_name_chirho.clone(),
                ty_chirho: final_subst_chirho.apply_ty_chirho(&defaulted_pred_chirho.ty_chirho),
                extra_tys_chirho: defaulted_pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| final_subst_chirho.apply_ty_chirho(t_chirho))
                    .collect(),
            };
            if self.is_certainly_unsolvable_pred_chirho(&fully_resolved_pred_chirho) {
                let blamed_chirho =
                    self.unsolvable_blame_pred_chirho(&fully_resolved_pred_chirho, 0);
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                        format!("no instance for `{blamed_chirho}`"),
                        span_chirho,
                    ));
                continue;
            }
            // Skip constraints for classes with no instances in our env.
            // These are user-defined classes where instances are defined in
            // the same module, or given constraints from type signatures.
            let has_instances_chirho = self
                .class_env_chirho
                .instances_chirho
                .get(&defaulted_pred_chirho.class_name_chirho)
                .is_some_and(|insts_chirho| !insts_chirho.is_empty());
            if !has_instances_chirho {
                continue;
            }
            if Self::pred_mentions_rigid_chirho(&defaulted_pred_chirho) {
                // A wanted over a rigid signature variable was not discharged
                // by the signature's givens. Report it only when no instance
                // could ever apply (see rigid_pred_certainly_undeducible_chirho).
                if self.rigid_pred_certainly_undeducible_chirho(&fully_resolved_pred_chirho) {
                    self.diagnostics_chirho
                        .push_chirho(DiagnosticChirho::error_with_code_chirho(
                            ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                            format!(
                                "could not deduce `{fully_resolved_pred_chirho}` from the context of the type signature"
                            ),
                            span_chirho,
                        ));
                }
                continue;
            }
            if !self.class_env_chirho.entails_chirho(&defaulted_pred_chirho) {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                        format!("no instance for `{defaulted_pred_chirho}`"),
                        span_chirho,
                    ));
            }
        }
        default_subst_chirho
    }
}
