// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor pattern implications. Expression uses keep ordinary instantiation.
//! Workflow: language-features-chirho/pattern-matching-chirho.md.

use super::*;

pub(super) struct PatternImplicationChirho {
    owner_chirho: usize,
    given_start_chirho: usize,
    rewrite_start_chirho: usize,
    first_meta_chirho: u32,
    event_start_chirho: usize,
}

/// A public image replacement is a domain update, not ordinary mathematical
/// substitution composition. Keep it separate until each caller accumulates
/// this implication's complete solution.
pub(super) struct PatternImplicationSolutionChirho {
    ordinary_chirho: SubstChirho,
    public_chirho: SubstChirho,
}

impl PatternImplicationSolutionChirho {
    pub(super) fn compose_chirho(&self, previous_chirho: &SubstChirho) -> SubstChirho {
        let mut step_chirho = self.public_chirho.compose_chirho(&self.ordinary_chirho);
        for (var_chirho, image_chirho) in self.public_chirho.iter_chirho() {
            step_chirho.insert_chirho(*var_chirho, image_chirho.clone());
        }
        let mut combined_chirho = step_chirho.compose_chirho(previous_chirho);
        for (var_chirho, image_chirho) in self.public_chirho.iter_chirho() {
            combined_chirho.insert_chirho(*var_chirho, image_chirho.clone());
        }
        combined_chirho
    }
}

impl InferCtxChirho {
    pub(super) fn begin_pattern_implication_chirho(&mut self) -> PatternImplicationChirho {
        self.equality_pattern_depth_chirho += 1;
        PatternImplicationChirho {
            owner_chirho: self.begin_equality_scope_chirho(
                super::equality_scopes_chirho::EqualityScopeKindChirho::PatternChirho,
            ),
            given_start_chirho: self.given_preds_chirho.len(),
            rewrite_start_chirho: self.given_rewrites_chirho.len(),
            first_meta_chirho: self.next_var_chirho,
            event_start_chirho: self.equality_substitution_events_chirho.len(),
        }
    }

    /// Solve while pattern evidence and environment refinements still exist.
    /// Pending equalities keep their owner/evidence links after the frame ends.
    pub(super) fn end_pattern_implication_chirho(
        &mut self,
        frame_chirho: PatternImplicationChirho,
        result_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> PatternImplicationSolutionChirho {
        let improvement_chirho = self.discharge_deferred_with_current_givens_chirho();
        let equalities_chirho = self.retry_deferred_equalities_chirho();
        let public_subst_chirho = self.check_pattern_equality_scope_chirho(
            frame_chirho.owner_chirho,
            result_chirho,
            frame_chirho.first_meta_chirho,
            frame_chirho.event_start_chirho,
            span_chirho,
        );
        self.given_preds_chirho
            .truncate(frame_chirho.given_start_chirho);
        self.given_rewrites_chirho
            .truncate(frame_chirho.rewrite_start_chirho);
        self.end_equality_scope_chirho(frame_chirho.owner_chirho);
        self.equality_pattern_depth_chirho -= 1;
        if self.equality_pattern_depth_chirho == 0 {
            self.equality_substitution_events_chirho.clear();
            self.equality_substitution_events_exhausted_chirho = false;
        }
        PatternImplicationSolutionChirho {
            ordinary_chirho: equalities_chirho.compose_chirho(&improvement_chirho),
            public_chirho: public_subst_chirho,
        }
    }

    /// Result parameters exposed through generative structure determine a
    /// universal. Field-only binders, and binders hidden under a non-injective
    /// family or unknown head, remain rigid in the pattern implication.
    pub(super) fn instantiate_constructor_pattern_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
    ) -> (Vec<TyChirho>, TyChirho, Vec<PredChirho>) {
        let mut binders_chirho = scheme_chirho.vars_chirho.clone();
        let mut fields_chirho = Vec::new();
        let mut remaining_chirho = &scheme_chirho.ty_chirho;
        loop {
            match remaining_chirho {
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                }
                | TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    binders_chirho.extend(vars_chirho.iter().copied());
                    remaining_chirho = body_chirho;
                }
                TyChirho::FunChirho(field_chirho, result_chirho, _) => {
                    fields_chirho.push(field_chirho.as_ref());
                    remaining_chirho = result_chirho;
                }
                _ => break,
            }
        }
        binders_chirho.sort_by_key(|var_chirho| var_chirho.0);
        binders_chirho.dedup();
        let mut result_parameters_chirho = HashSet::new();
        let mut pending_chirho = Vec::new();
        let mut remaining_work_chirho = 16_384_usize;
        let mut head_chirho = remaining_chirho;
        while let TyChirho::AppChirho(fun_chirho, argument_chirho) = head_chirho {
            let Some(next_work_chirho) = remaining_work_chirho.checked_sub(1) else {
                break;
            };
            remaining_work_chirho = next_work_chirho;
            pending_chirho.push(argument_chirho.as_ref());
            head_chirho = fun_chirho;
        }
        match remaining_chirho {
            TyChirho::ListChirho(element_chirho) => pending_chirho.push(element_chirho),
            TyChirho::TupleChirho(elements_chirho)
                if elements_chirho.len() <= remaining_work_chirho =>
            {
                remaining_work_chirho -= elements_chirho.len();
                pending_chirho.extend(elements_chirho);
            }
            _ => {}
        }
        while let Some(component_chirho) = pending_chirho.pop() {
            let Some(next_work_chirho) = remaining_work_chirho.checked_sub(1) else {
                break;
            };
            remaining_work_chirho = next_work_chirho;
            match component_chirho {
                TyChirho::VarChirho(var_chirho) => {
                    result_parameters_chirho.insert(*var_chirho);
                }
                TyChirho::TupleChirho(elements_chirho)
                    if elements_chirho.len() <= remaining_work_chirho =>
                {
                    remaining_work_chirho -= elements_chirho.len();
                    pending_chirho.extend(elements_chirho);
                }
                TyChirho::ListChirho(element_chirho) => pending_chirho.push(element_chirho),
                TyChirho::FunChirho(argument_chirho, result_chirho, _) => {
                    pending_chirho.extend([argument_chirho.as_ref(), result_chirho.as_ref()])
                }
                TyChirho::AppChirho(..) => {
                    let mut head_chirho = component_chirho;
                    let mut nested_chirho = Vec::new();
                    while let TyChirho::AppChirho(fun_chirho, argument_chirho) = head_chirho {
                        let Some(next_work_chirho) = remaining_work_chirho.checked_sub(1) else {
                            break;
                        };
                        remaining_work_chirho = next_work_chirho;
                        nested_chirho.push(argument_chirho.as_ref());
                        head_chirho = fun_chirho;
                    }
                    if let TyChirho::ConChirho(name_chirho) = head_chirho
                        && matches!(super::family_registration_chirho::family_head_role_chirho(&self.type_families_chirho, name_chirho), super::closed_family_apartness_chirho::FamilyHeadRoleChirho::GenerativeChirho { .. })
                    { pending_chirho.extend(nested_chirho); }
                }
                _ => {}
            }
        }
        let mut instantiation_chirho = SubstChirho::empty_chirho();
        for var_chirho in binders_chirho {
            let image_chirho = if result_parameters_chirho.contains(&var_chirho) {
                self.fresh_var_chirho()
            } else {
                let base_chirho = self
                    .tyvar_source_names_chirho
                    .get(&var_chirho)
                    .map(String::as_str)
                    .unwrap_or("existentialChirho");
                let name_chirho = skolem_name_chirho(base_chirho, self.next_skolem_chirho);
                self.next_skolem_chirho += 1;
                self.equality_scopes_chirho[self.active_equality_scope_chirho]
                    .skolems_chirho
                    .insert(name_chirho.clone());
                TyChirho::ForallVarChirho(name_chirho)
            };
            instantiation_chirho.insert_chirho(var_chirho, image_chirho);
        }
        let predicates_chirho = scheme_chirho
            .preds_chirho
            .iter()
            .map(|pred_chirho| PredChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: instantiation_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|ty_chirho| instantiation_chirho.apply_ty_chirho(ty_chirho))
                    .collect(),
            })
            .collect();
        (
            fields_chirho
                .into_iter()
                .map(|field_chirho| instantiation_chirho.apply_ty_chirho(field_chirho))
                .collect(),
            instantiation_chirho.apply_ty_chirho(remaining_chirho),
            predicates_chirho,
        )
    }

    pub(super) fn install_pattern_givens_chirho(&mut self, predicates_chirho: Vec<PredChirho>) {
        let start_chirho = self.given_rewrites_chirho.len();
        self.install_given_equalities_chirho(&predicates_chirho);
        self.record_equality_rewrites_chirho(self.given_rewrites_chirho[start_chirho..].to_vec());
        self.given_preds_chirho.extend(predicates_chirho);
    }

    /// Meet only equal nominal datatype heads. Their result arguments are
    /// evidence of the match, not wanted equalities. A family component stays
    /// whole: no injectivity is inferred from its spelling or arguments.
    pub(super) fn match_gadt_result_chirho(
        &mut self,
        result_chirho: &TyChirho,
        scrutinee_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Option<SubstChirho> {
        let result_chirho = self.normalize_ty_chirho(result_chirho);
        let scrutinee_chirho = self.normalize_ty_chirho(scrutinee_chirho);
        let result_head_chirho = Self::ty_outer_head_name_chirho(&result_chirho)?;
        if Self::ty_outer_head_name_chirho(&scrutinee_chirho) != Some(result_head_chirho)
            || self.ty_is_family_app_chirho(&result_chirho)
        {
            return None;
        }
        let (_, result_arguments_chirho) = collect_app_spine_chirho(&result_chirho);
        let (_, scrutinee_arguments_chirho) = collect_app_spine_chirho(&scrutinee_chirho);
        if result_arguments_chirho.len() != scrutinee_arguments_chirho.len() {
            return None;
        }
        let mut combined_chirho = SubstChirho::empty_chirho();
        for (left_chirho, right_chirho) in result_arguments_chirho
            .into_iter()
            .zip(scrutinee_arguments_chirho)
        {
            let left_chirho = combined_chirho.apply_ty_chirho(&left_chirho);
            let right_chirho = combined_chirho.apply_ty_chirho(&right_chirho);
            if left_chirho == right_chirho {
                continue;
            }
            let ordinary_chirho = if self.ty_mentions_family_chirho(&left_chirho)
                || self.ty_mentions_family_chirho(&right_chirho)
            {
                // Binding a whole flexible variable is safe; decomposing a
                // family is not. No wanted is created by result matching.
                if matches!(left_chirho, TyChirho::VarChirho(_))
                    || matches!(right_chirho, TyChirho::VarChirho(_))
                {
                    unify_chirho(&left_chirho, &right_chirho, span_chirho).ok()
                } else {
                    None
                }
            } else {
                unify_chirho(&left_chirho, &right_chirho, span_chirho)
                    .ok()
                    .or_else(|| {
                        self.refine_skolems_by_unification_chirho(&left_chirho, &right_chirho)
                    })
            };
            if let Some(subst_chirho) = ordinary_chirho {
                self.apply_subst_all_chirho(&subst_chirho);
                combined_chirho = subst_chirho.compose_chirho(&combined_chirho);
            } else {
                self.install_pattern_givens_chirho(vec![PredChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "~".to_owned(),
                    ty_chirho: left_chirho,
                    extra_tys_chirho: vec![right_chirho],
                }]);
            }
        }
        Some(combined_chirho)
    }
}
