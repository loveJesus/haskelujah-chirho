// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor pattern images keep their complete checked binder prefix.

use super::*;

impl InferCtxChirho {
    pub(in crate::infer_chirho) fn instantiate_constructor_pattern_with_variables_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
    ) -> (Vec<TyChirho>, TyChirho, Vec<PredChirho>, SubstChirho) {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        let mut budget_chirho = match self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)
        {
            Ok(budget_chirho) => budget_chirho,
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                return (
                    Vec::new(),
                    self.fresh_var_chirho(),
                    Vec::new(),
                    SubstChirho::empty_chirho(),
                );
            }
        };
        let initial_chirho = budget_chirho.remaining_chirho();
        let result_chirho = self.instantiate_constructor_pattern_prefix_chirho(
            scheme_chirho,
            &mut budget_chirho,
            span_chirho,
        );
        if let Err(error_chirho) = self.elaboration_state_chirho.settle_equality_budget_chirho(
            initial_chirho,
            &budget_chirho,
            result_chirho.is_none(),
            span_chirho,
        ) {
            self.record_body_kind_error_chirho(error_chirho);
            return (
                Vec::new(),
                self.fresh_var_chirho(),
                Vec::new(),
                SubstChirho::empty_chirho(),
            );
        }
        result_chirho.unwrap_or_else(|| {
            (
                Vec::new(),
                self.fresh_var_chirho(),
                Vec::new(),
                SubstChirho::empty_chirho(),
            )
        })
    }

    fn instantiate_constructor_pattern_prefix_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        budget_chirho: &mut super::super::equality_normalization_chirho::EqualityBudgetChirho,
        span_chirho: SpanChirho,
    ) -> Option<(Vec<TyChirho>, TyChirho, Vec<PredChirho>, SubstChirho)> {
        budget_chirho.node_chirho(scheme_chirho.vars_chirho.len())?;
        let mut binders_chirho = scheme_chirho.vars_chirho.records_chirho().to_vec();
        let mut fields_chirho = Vec::new();
        let mut remaining_chirho = &scheme_chirho.ty_chirho;
        let mut depth_chirho = 0_usize;
        loop {
            budget_chirho.work_chirho(1)?;
            if depth_chirho >= 128 {
                return None;
            }
            depth_chirho += 1;
            match remaining_chirho {
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                }
                | TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    budget_chirho.node_chirho(vars_chirho.len())?;
                    binders_chirho.extend(vars_chirho.records_chirho().iter().cloned());
                    remaining_chirho = body_chirho;
                }
                TyChirho::FunChirho(field_chirho, result_chirho, _) => {
                    budget_chirho.node_chirho(1)?;
                    fields_chirho.push(field_chirho.as_ref());
                    remaining_chirho = result_chirho;
                }
                _ => break,
            }
        }
        budget_chirho.node_chirho(binders_chirho.len())?;
        let mut seen_chirho = HashSet::new();
        binders_chirho.retain(|record_chirho| seen_chirho.insert(record_chirho.variable_chirho));
        let mut result_parameters_chirho = HashSet::new();
        let mut pending_chirho = Vec::new();
        let mut remaining_work_chirho = 16_384_usize;
        let mut head_chirho = remaining_chirho;
        while let TyChirho::AppChirho(fun_chirho, argument_chirho) = head_chirho {
            let next_work_chirho = remaining_work_chirho.checked_sub(1)?;
            budget_chirho.work_chirho(1)?;
            remaining_work_chirho = next_work_chirho;
            budget_chirho.node_chirho(1)?;
            pending_chirho.push(argument_chirho.as_ref());
            head_chirho = fun_chirho;
        }
        match remaining_chirho {
            TyChirho::ListChirho(element_chirho) => {
                budget_chirho.node_chirho(1)?;
                pending_chirho.push(element_chirho);
            }
            TyChirho::TupleChirho(elements_chirho) => {
                remaining_work_chirho = remaining_work_chirho.checked_sub(elements_chirho.len())?;
                budget_chirho.node_chirho(elements_chirho.len())?;
                pending_chirho.extend(elements_chirho);
            }
            _ => {}
        }
        while let Some(component_chirho) = pending_chirho.pop() {
            let next_work_chirho = remaining_work_chirho.checked_sub(1)?;
            budget_chirho.work_chirho(1)?;
            remaining_work_chirho = next_work_chirho;
            match component_chirho {
                TyChirho::VarChirho(var_chirho) => {
                    budget_chirho.node_chirho(1)?;
                    result_parameters_chirho.insert(*var_chirho);
                }
                TyChirho::TupleChirho(elements_chirho) => {
                    remaining_work_chirho =
                        remaining_work_chirho.checked_sub(elements_chirho.len())?;
                    budget_chirho.node_chirho(elements_chirho.len())?;
                    pending_chirho.extend(elements_chirho);
                }
                TyChirho::ListChirho(element_chirho) => {
                    budget_chirho.node_chirho(1)?;
                    pending_chirho.push(element_chirho);
                }
                TyChirho::FunChirho(argument_chirho, result_chirho, _) => {
                    budget_chirho.node_chirho(2)?;
                    pending_chirho.extend([argument_chirho.as_ref(), result_chirho.as_ref()])
                }
                TyChirho::AppChirho(..) => {
                    let mut head_chirho = component_chirho;
                    let mut nested_chirho = Vec::new();
                    while let TyChirho::AppChirho(fun_chirho, argument_chirho) = head_chirho {
                        let next_work_chirho = remaining_work_chirho.checked_sub(1)?;
                        budget_chirho.work_chirho(1)?;
                        remaining_work_chirho = next_work_chirho;
                        budget_chirho.node_chirho(1)?;
                        nested_chirho.push(argument_chirho.as_ref());
                        head_chirho = fun_chirho;
                    }
                    if let TyChirho::ConChirho(name_chirho) = head_chirho
                        && matches!(super::family_registration_chirho::family_head_role_chirho(&self.type_families_chirho, name_chirho), super::closed_family_apartness_chirho::FamilyHeadRoleChirho::GenerativeChirho { .. })
                    { budget_chirho.node_chirho(nested_chirho.len())?; pending_chirho.extend(nested_chirho); }
                }
                _ => {}
            }
        }
        let mut instantiation_chirho = SubstChirho::empty_chirho();
        for record_chirho in binders_chirho {
            budget_chirho.node_chirho(1)?;
            let image_chirho = if result_parameters_chirho.contains(&record_chirho.variable_chirho)
            {
                let kind_chirho = self.instantiate_quantifier_kind_chirho(
                    &record_chirho,
                    &instantiation_chirho,
                    budget_chirho,
                )?;
                let stored_kind_chirho = match &kind_chirho {
                    Some(kind_chirho) => Some(
                        super::super::equality_normalization_chirho::copy_term_with_budget_chirho(
                            kind_chirho,
                            budget_chirho,
                        )?,
                    ),
                    None => None,
                };
                let variable_chirho = self
                    .elaboration_state_chirho
                    .fresh_chirho(stored_kind_chirho, span_chirho)
                    .ok()?;
                crate::subst_chirho::SubstitutionImageChirho {
                    image_chirho: TyChirho::VarChirho(variable_chirho),
                    kind_chirho,
                    source_chirho: record_chirho.source_chirho.clone(),
                }
            } else {
                let image_chirho = self.fresh_rigid_quantifier_image_chirho(
                    &record_chirho,
                    &instantiation_chirho,
                    budget_chirho,
                )?;
                if let TyChirho::ForallVarChirho(name_chirho) = &image_chirho.image_chirho {
                    budget_chirho.node_chirho(name_chirho.len().saturating_add(1))?;
                    self.equality_scopes_chirho[self.active_equality_scope_chirho]
                        .skolems_chirho
                        .insert(name_chirho.clone());
                }
                image_chirho
            };
            instantiation_chirho.insert_image_chirho(record_chirho.variable_chirho, image_chirho);
        }
        let (result_chirho, predicates_chirho) = self.instantiate_scheme_terms_chirho(
            scheme_chirho,
            remaining_chirho,
            &instantiation_chirho,
            budget_chirho,
        )?;
        let mut mapped_fields_chirho = Vec::new();
        for field_chirho in fields_chirho {
            budget_chirho.node_chirho(1)?;
            mapped_fields_chirho.push(instantiation_chirho.apply_scheme_term_bounded_chirho(
                field_chirho,
                &mut self.elaboration_state_chirho.next_variable_chirho,
                &mut |work_chirho, output_chirho| {
                    budget_chirho.node_chirho(output_chirho)?;
                    budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                },
            )?);
        }
        Some((
            mapped_fields_chirho,
            result_chirho,
            predicates_chirho,
            instantiation_chirho,
        ))
    }
}
