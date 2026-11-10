// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Whole application spines and short-lived Quick Look instantiation variables.
//! Family applications stay opaque until ordinary checking. Prepared argument
//! heads retain their predicates until checking enters the argument's givens.

use super::InferCtxChirho;
use super::equality_normalization_chirho::{EqualityBudgetChirho, copy_term_with_budget_chirho};
use crate::class_chirho::{PredChirho, PredPartsChirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::name_chirho::{CompilerValueChirho, NameChirho};
use haskelujah_span_chirho::SpanChirho;
use std::collections::HashSet;

struct ApplicationArgumentChirho<'source_chirho> {
    expression_chirho: &'source_chirho ExprChirho,
    formal_chirho: TyChirho,
    prepared_chirho: Option<Box<ApplicationPlanChirho<'source_chirho>>>,
}
struct ApplicationPlanChirho<'source_chirho> {
    name_chirho: &'source_chirho NameChirho,
    variable_head_chirho: bool,
    scheme_chirho: SchemeChirho,
    compiler_value_chirho: Option<CompilerValueChirho>,
    global_quick_look_chirho: bool,
    predicates_chirho: Vec<PredChirho>,
    arguments_chirho: Vec<ApplicationArgumentChirho<'source_chirho>>,
    result_chirho: TyChirho,
}

impl ApplicationPlanChirho<'_> {
    fn quick_look_enabled_chirho(&self) -> bool {
        self.global_quick_look_chirho
            || self.compiler_value_chirho == Some(CompilerValueChirho::ApplyChirho)
    }
}
#[derive(Default)]
struct QuickLookChirho {
    variables_chirho: HashSet<TyVarChirho>,
    substitution_chirho: SubstChirho,
}

impl QuickLookChirho {
    fn apply_chirho(
        &self,
        term_chirho: &TyChirho,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<TyChirho> {
        fn walk_chirho(
            look_chirho: &QuickLookChirho,
            term_chirho: &TyChirho,
            budget_chirho: &mut EqualityBudgetChirho,
            bound_chirho: &mut Vec<TyVarChirho>,
            depth_chirho: usize,
        ) -> Option<TyChirho> {
            budget_chirho.node_chirho(1)?;
            if depth_chirho > 128 {
                return None;
            }
            Some(match term_chirho {
                TyChirho::VarChirho(variable_chirho) => {
                    if !bound_chirho.contains(variable_chirho)
                        && let Some(image_chirho) = look_chirho
                            .substitution_chirho
                            .lookup_chirho(variable_chirho)
                    {
                        return walk_chirho(
                            look_chirho,
                            image_chirho,
                            budget_chirho,
                            bound_chirho,
                            depth_chirho + 1,
                        );
                    }
                    term_chirho.clone()
                }
                TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
                    budget_chirho.node_chirho(name_chirho.len())?;
                    term_chirho.clone()
                }
                TyChirho::AppChirho(function_chirho, argument_chirho) => TyChirho::AppChirho(
                    Box::new(walk_chirho(
                        look_chirho,
                        function_chirho,
                        budget_chirho,
                        bound_chirho,
                        depth_chirho + 1,
                    )?),
                    Box::new(walk_chirho(
                        look_chirho,
                        argument_chirho,
                        budget_chirho,
                        bound_chirho,
                        depth_chirho + 1,
                    )?),
                ),
                TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
                    TyChirho::FunChirho(
                        Box::new(walk_chirho(
                            look_chirho,
                            argument_chirho,
                            budget_chirho,
                            bound_chirho,
                            depth_chirho + 1,
                        )?),
                        Box::new(walk_chirho(
                            look_chirho,
                            result_chirho,
                            budget_chirho,
                            bound_chirho,
                            depth_chirho + 1,
                        )?),
                        *multiplicity_chirho,
                    )
                }
                TyChirho::ListChirho(element_chirho) => {
                    TyChirho::ListChirho(Box::new(walk_chirho(
                        look_chirho,
                        element_chirho,
                        budget_chirho,
                        bound_chirho,
                        depth_chirho + 1,
                    )?))
                }
                TyChirho::TupleChirho(elements_chirho) => {
                    budget_chirho.work_chirho(elements_chirho.len())?;
                    TyChirho::TupleChirho(
                        elements_chirho
                            .iter()
                            .map(|element_chirho| {
                                walk_chirho(
                                    look_chirho,
                                    element_chirho,
                                    budget_chirho,
                                    bound_chirho,
                                    depth_chirho + 1,
                                )
                            })
                            .collect::<Option<Vec<_>>>()?,
                    )
                }
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                }
                | TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    budget_chirho.node_chirho(vars_chirho.len())?;
                    let previous_chirho = bound_chirho.len();
                    let result_chirho = (|| {
                        let mut records_chirho = Vec::new();
                        for record_chirho in vars_chirho.records_chirho() {
                            let mut mapped_chirho = record_chirho.clone();
                            mapped_chirho.kind_chirho = match &record_chirho.kind_chirho {
                                Some(kind_chirho) => Some(std::sync::Arc::new(walk_chirho(
                                    look_chirho,
                                    kind_chirho,
                                    budget_chirho,
                                    bound_chirho,
                                    depth_chirho + 1,
                                )?)),
                                None => None,
                            };
                            records_chirho.push(mapped_chirho);
                            bound_chirho.push(record_chirho.variable_chirho);
                        }
                        Some((
                            crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(
                                records_chirho,
                            ),
                            Box::new(walk_chirho(
                                look_chirho,
                                body_chirho,
                                budget_chirho,
                                bound_chirho,
                                depth_chirho + 1,
                            )?),
                        ))
                    })();
                    bound_chirho.truncate(previous_chirho);
                    let (vars_chirho, body_chirho) = result_chirho?;
                    if matches!(term_chirho, TyChirho::ForallChirho { .. }) {
                        TyChirho::ForallChirho {
                            vars_chirho,
                            body_chirho,
                        }
                    } else {
                        TyChirho::RequiredForallChirho {
                            vars_chirho,
                            body_chirho,
                        }
                    }
                }
                TyChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                } => {
                    budget_chirho.work_chirho(context_chirho.len())?;
                    TyChirho::QualChirho {
                        context_chirho: context_chirho
                            .iter()
                            .map(|constraint_chirho| {
                                walk_chirho(
                                    look_chirho,
                                    constraint_chirho,
                                    budget_chirho,
                                    bound_chirho,
                                    depth_chirho + 1,
                                )
                            })
                            .collect::<Option<Vec<_>>>()?,
                        body_chirho: Box::new(walk_chirho(
                            look_chirho,
                            body_chirho,
                            budget_chirho,
                            bound_chirho,
                            depth_chirho + 1,
                        )?),
                    }
                }
            })
        }
        walk_chirho(self, term_chirho, budget_chirho, &mut Vec::new(), 0)
    }
    fn unify_chirho(
        &mut self,
        context_chirho: &InferCtxChirho,
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<()> {
        let mut pending_chirho = vec![(
            copy_term_with_budget_chirho(left_chirho, budget_chirho)?,
            copy_term_with_budget_chirho(right_chirho, budget_chirho)?,
            0usize,
        )];
        while let Some((left_chirho, right_chirho, depth_chirho)) = pending_chirho.pop() {
            budget_chirho.work_chirho(1)?;
            if depth_chirho > 128 {
                return None;
            }
            let left_chirho = self.apply_chirho(&left_chirho, budget_chirho)?;
            let right_chirho = self.apply_chirho(&right_chirho, budget_chirho)?;
            let left_chirho = super::synonym_expansion_chirho::expand_synonyms_with_budget_chirho(
                context_chirho,
                &left_chirho,
                budget_chirho,
            )?;
            let right_chirho = super::synonym_expansion_chirho::expand_synonyms_with_budget_chirho(
                context_chirho,
                &right_chirho,
                budget_chirho,
            )?;
            let binding_chirho = match (&left_chirho, &right_chirho) {
                (TyChirho::VarChirho(variable_chirho), _)
                    if self.variables_chirho.contains(variable_chirho) =>
                {
                    Some((*variable_chirho, &right_chirho))
                }
                (_, TyChirho::VarChirho(variable_chirho))
                    if self.variables_chirho.contains(variable_chirho) =>
                {
                    Some((*variable_chirho, &left_chirho))
                }
                _ => None,
            };
            if let Some((variable_chirho, image_chirho)) = binding_chirho {
                if !image_chirho.free_vars_chirho().contains(&variable_chirho) {
                    self.substitution_chirho.insert_chirho(
                        variable_chirho,
                        copy_term_with_budget_chirho(image_chirho, budget_chirho)?,
                    );
                }
                continue;
            }
            // A family is neither generative nor injective. In particular, do
            // not reduce Id kappa to kappa and learn an impredicative choice.
            if context_chirho.ty_is_family_app_chirho(&left_chirho)
                || context_chirho.ty_is_family_app_chirho(&right_chirho)
            {
                continue;
            }
            match (left_chirho, right_chirho) {
                (
                    TyChirho::AppChirho(left_fun_chirho, left_arg_chirho),
                    TyChirho::AppChirho(right_fun_chirho, right_arg_chirho),
                ) => {
                    // Decompose only an equal generative head, never mismatching
                    // applications or an application whose head is a family.
                    let (mut left_head_chirho, mut right_head_chirho) =
                        (left_fun_chirho.as_ref(), right_fun_chirho.as_ref());
                    while let TyChirho::AppChirho(function_chirho, _) = left_head_chirho {
                        budget_chirho.work_chirho(1)?;
                        left_head_chirho = function_chirho;
                    }
                    while let TyChirho::AppChirho(function_chirho, _) = right_head_chirho {
                        budget_chirho.work_chirho(1)?;
                        right_head_chirho = function_chirho;
                    }
                    if left_head_chirho == right_head_chirho
                        && context_chirho.quick_look_generative_head_chirho(left_head_chirho)
                    {
                        pending_chirho.push((
                            *left_fun_chirho,
                            *right_fun_chirho,
                            depth_chirho + 1,
                        ));
                        pending_chirho.push((
                            *left_arg_chirho,
                            *right_arg_chirho,
                            depth_chirho + 1,
                        ));
                    }
                }
                (
                    TyChirho::FunChirho(left_arg_chirho, left_result_chirho, _),
                    TyChirho::FunChirho(right_arg_chirho, right_result_chirho, _),
                ) => {
                    pending_chirho.push((*left_arg_chirho, *right_arg_chirho, depth_chirho + 1));
                    pending_chirho.push((
                        *left_result_chirho,
                        *right_result_chirho,
                        depth_chirho + 1,
                    ));
                }
                (TyChirho::ListChirho(left_chirho), TyChirho::ListChirho(right_chirho)) => {
                    pending_chirho.push((*left_chirho, *right_chirho, depth_chirho + 1))
                }
                (TyChirho::TupleChirho(left_chirho), TyChirho::TupleChirho(right_chirho))
                    if left_chirho.len() == right_chirho.len() =>
                {
                    budget_chirho.work_chirho(left_chirho.len())?;
                    pending_chirho.extend(left_chirho.into_iter().zip(right_chirho).map(
                        |(left_chirho, right_chirho)| (left_chirho, right_chirho, depth_chirho + 1),
                    ));
                }
                // Quantifiers do not decompose during Quick Look.
                _ => {}
            }
        }
        Some(())
    }
}

#[derive(Clone, Copy)]
struct ApplicationPreparationChirho {
    global_quick_look_chirho: bool,
    retain_result_chirho: bool,
}

impl InferCtxChirho {
    fn quick_look_generative_head_chirho(&self, head_chirho: &TyChirho) -> bool {
        match head_chirho {
            TyChirho::VarChirho(_) | TyChirho::ForallVarChirho(_) => true,
            TyChirho::ConChirho(name_chirho) => matches!(
                super::family_registration_chirho::family_head_role_chirho(
                    &self.type_families_chirho,
                    name_chirho
                ),
                super::closed_family_apartness_chirho::FamilyHeadRoleChirho::GenerativeChirho { .. }
            ),
            _ => false,
        }
    }
    fn quick_look_guarded_chirho(&self, term_chirho: &TyChirho) -> bool {
        match term_chirho {
            TyChirho::FunChirho(..) | TyChirho::ListChirho(_) | TyChirho::TupleChirho(_) => true,
            TyChirho::AppChirho(..) => {
                let mut head_chirho = term_chirho;
                while let TyChirho::AppChirho(function_chirho, _) = head_chirho {
                    head_chirho = function_chirho;
                }
                self.quick_look_generative_head_chirho(head_chirho)
            }
            TyChirho::ConChirho(_) => self.quick_look_generative_head_chirho(term_chirho),
            _ => false,
        }
    }

    /// Prepare once, choose only the temporary instantiation variables, then
    /// check every argument using ordinary monomorphic unification variables.
    pub(super) fn infer_application_spine_chirho(
        &mut self,
        expression_chirho: &ExprChirho,
        expected_chirho: Option<&TyChirho>,
        span_chirho: SpanChirho,
    ) -> Option<(SubstChirho, TyChirho)> {
        self.infer_application_result_chirho(expression_chirho, expected_chirho, span_chirho, false)
    }

    /// A following written @ argument consumes the result's original lexical
    /// prefix. Ordinary term uses still instantiate that same result implicitly.
    pub(super) fn infer_application_sigma_chirho(
        &mut self,
        expression_chirho: &ExprChirho,
        span_chirho: SpanChirho,
    ) -> Option<(SubstChirho, TyChirho)> {
        self.infer_application_result_chirho(expression_chirho, None, span_chirho, true)
    }

    fn infer_application_result_chirho(
        &mut self,
        expression_chirho: &ExprChirho,
        expected_chirho: Option<&TyChirho>,
        span_chirho: SpanChirho,
        retain_result_chirho: bool,
    ) -> Option<(SubstChirho, TyChirho)> {
        let enabled_chirho = self
            .type_validity_license_chirho
            .is_some_and(|license_chirho| license_chirho.impredicative_enabled_chirho());
        let mut look_chirho = QuickLookChirho::default();
        let mut budget_chirho = EqualityBudgetChirho::new_chirho();
        let prepared_chirho = self.prepare_application_chirho(
            expression_chirho,
            ApplicationPreparationChirho {
                global_quick_look_chirho: enabled_chirho,
                retain_result_chirho,
            },
            &mut look_chirho,
            &mut budget_chirho,
            0,
        );
        let prepared_chirho = match prepared_chirho {
            Ok(Some(prepared_chirho)) => prepared_chirho,
            Ok(None) => return None,
            Err(()) => {
                self.type_normalization_exhausted_chirho.set(true);
                return Some((SubstChirho::empty_chirho(), self.fresh_var_chirho()));
            }
        };
        if prepared_chirho.quick_look_enabled_chirho()
            && let Some(expected_chirho) = expected_chirho
            && look_chirho
                .unify_chirho(
                    self,
                    &prepared_chirho.result_chirho,
                    expected_chirho,
                    &mut budget_chirho,
                )
                .is_none()
        {
            self.type_normalization_exhausted_chirho.set(true);
        }
        // No delta capability is stored in Scheme or the inference context.
        // From this point on, unfilled variables are ordinary monotypes.
        match self.finish_application_chirho(
            prepared_chirho,
            expected_chirho,
            &look_chirho,
            &mut budget_chirho,
            SubstChirho::empty_chirho(),
            span_chirho,
        ) {
            Some(result_chirho) => Some(result_chirho),
            None => {
                self.type_normalization_exhausted_chirho.set(true);
                Some((SubstChirho::empty_chirho(), self.fresh_var_chirho()))
            }
        }
    }

    fn prepare_application_chirho<'source_chirho>(
        &mut self,
        expression_chirho: &'source_chirho ExprChirho,
        mode_chirho: ApplicationPreparationChirho,
        look_chirho: &mut QuickLookChirho,
        budget_chirho: &mut EqualityBudgetChirho,
        depth_chirho: usize,
    ) -> Result<Option<ApplicationPlanChirho<'source_chirho>>, ()> {
        if depth_chirho > 128 {
            return Err(());
        }
        let mut head_chirho = expression_chirho;
        let mut arguments_chirho = Vec::new();
        loop {
            budget_chirho.work_chirho(1).ok_or(())?;
            match head_chirho {
                ExprChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    budget_chirho.node_chirho(1).ok_or(())?;
                    arguments_chirho.push(arg_chirho.as_ref());
                    head_chirho = fun_chirho;
                }
                ExprChirho::ParenChirho { inner_chirho, .. } => head_chirho = inner_chirho,
                _ => break,
            }
        }
        arguments_chirho.reverse();
        let mut explicit_ast_chirho = Vec::new();
        let base_chirho = Self::peel_type_app_chain_chirho(head_chirho, &mut explicit_ast_chirho);
        let (name_chirho, variable_head_chirho) = match base_chirho {
            ExprChirho::VarChirho(name_chirho) => (name_chirho, true),
            ExprChirho::ConChirho(name_chirho) => (name_chirho, false),
            _ => return Ok(None),
        };
        if !variable_head_chirho && !explicit_ast_chirho.is_empty() {
            return Ok(None);
        }
        let Some((_, scheme_chirho, compiler_value_chirho)) = self.lookup_value_entry_keyed_chirho(
            &name_chirho.full_name_chirho(),
            name_chirho.text_chirho(),
        ) else {
            return Ok(None);
        };
        let quick_look_chirho = mode_chirho.global_quick_look_chirho
            || compiler_value_chirho == Some(CompilerValueChirho::ApplyChirho);
        let mut variables_chirho = self.scoped_tyvars_chirho.clone();
        let explicit_kinds_chirho: Vec<_> = explicit_ast_chirho
            .iter()
            .map(|argument_chirho| self.checked_type_argument_kind_chirho(argument_chirho))
            .collect();
        let explicit_chirho: Vec<_> = explicit_ast_chirho
            .iter()
            .map(|argument_chirho| {
                self.ast_type_to_ty_chirho(argument_chirho, &mut variables_chirho)
            })
            .collect();
        let start_chirho = self.elaboration_state_chirho.next_variable_chirho;
        let instantiated_chirho = if explicit_chirho.is_empty() {
            Some(self.instantiate_scheme_parts_chirho(&scheme_chirho))
        } else {
            self.instantiate_scheme_parts_with_visible_type_args_chirho(
                &scheme_chirho,
                &explicit_chirho,
                &explicit_kinds_chirho,
                name_chirho.span_chirho(),
            )
        };
        let Some((mut current_chirho, mut predicates_chirho)) = instantiated_chirho else {
            return Ok(None);
        };
        budget_chirho
            .work_chirho(
                (self.elaboration_state_chirho.next_variable_chirho - start_chirho) as usize,
            )
            .ok_or(())?;
        // A parent's selected compiler head never licenses a child's fresh
        // instantiation variables. Only this resolved head, or the actual
        // global option, grants its own short-lived delta variables.
        if quick_look_chirho {
            look_chirho.variables_chirho.extend(
                (start_chirho..self.elaboration_state_chirho.next_variable_chirho).map(TyVarChirho),
            );
        }
        let mut prepared_arguments_chirho = Vec::new();
        for argument_chirho in arguments_chirho {
            let (opened_chirho, additional_chirho) = self.open_application_prefix_chirho(
                current_chirho,
                look_chirho,
                budget_chirho,
                quick_look_chirho,
            )?;
            predicates_chirho.extend(additional_chirho);
            let TyChirho::FunChirho(formal_chirho, result_chirho, _) = opened_chirho else {
                return Ok(None);
            };
            // Guardedness uses the original formal, before earlier arguments'
            // choices. This gives the same answer in either argument order.
            let exposed_formal_chirho = look_chirho
                .apply_chirho(&formal_chirho, budget_chirho)
                .ok_or(())?;
            let sigma_chirho = matches!(
                exposed_formal_chirho,
                TyChirho::ForallChirho { .. }
                    | TyChirho::RequiredForallChirho { .. }
                    | TyChirho::QualChirho { .. }
            );
            let prepared_chirho = if quick_look_chirho && !sigma_chirho {
                self.prepare_application_chirho(
                    argument_chirho,
                    ApplicationPreparationChirho {
                        retain_result_chirho: false,
                        ..mode_chirho
                    },
                    look_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?
            } else {
                None
            };
            if let Some(argument_plan_chirho) = &prepared_chirho {
                let result_chirho = look_chirho
                    .apply_chirho(&argument_plan_chirho.result_chirho, budget_chirho)
                    .ok_or(())?;
                let guarded_chirho = self.quick_look_guarded_chirho(&formal_chirho);
                let independent_chirho = !result_chirho
                    .free_vars_chirho()
                    .iter()
                    .any(|variable_chirho| look_chirho.variables_chirho.contains(variable_chirho));
                if guarded_chirho || independent_chirho {
                    look_chirho
                        .unify_chirho(self, &result_chirho, &formal_chirho, budget_chirho)
                        .ok_or(())?;
                }
            }
            prepared_arguments_chirho.push(ApplicationArgumentChirho {
                expression_chirho: argument_chirho,
                formal_chirho: *formal_chirho,
                prepared_chirho: prepared_chirho.map(Box::new),
            });
            current_chirho = *result_chirho;
        }
        if !mode_chirho.retain_result_chirho {
            let (opened_chirho, additional_chirho) = self.open_application_prefix_chirho(
                current_chirho,
                look_chirho,
                budget_chirho,
                quick_look_chirho,
            )?;
            predicates_chirho.extend(additional_chirho);
            current_chirho = opened_chirho;
        }
        Ok(Some(ApplicationPlanChirho {
            name_chirho,
            variable_head_chirho,
            scheme_chirho,
            compiler_value_chirho,
            global_quick_look_chirho: mode_chirho.global_quick_look_chirho,
            predicates_chirho,
            arguments_chirho: prepared_arguments_chirho,
            result_chirho: current_chirho,
        }))
    }

    fn open_application_prefix_chirho(
        &mut self,
        mut current_chirho: TyChirho,
        look_chirho: &mut QuickLookChirho,
        budget_chirho: &mut EqualityBudgetChirho,
        quick_look_chirho: bool,
    ) -> Result<(TyChirho, Vec<PredChirho>), ()> {
        let mut predicates_chirho = Vec::new();
        loop {
            budget_chirho.work_chirho(1).ok_or(())?;
            // Follow only the head. Original formal bodies must retain their
            // pre-Quick-Look shape for the guardedness decision.
            if let TyChirho::VarChirho(variable_chirho) = &current_chirho
                && let Some(image_chirho) = look_chirho
                    .substitution_chirho
                    .lookup_chirho(variable_chirho)
            {
                current_chirho =
                    copy_term_with_budget_chirho(image_chirho, budget_chirho).ok_or(())?;
                continue;
            }
            current_chirho = super::synonym_expansion_chirho::expand_synonyms_with_budget_chirho(
                self,
                &current_chirho,
                budget_chirho,
            )
            .ok_or(())?;
            match current_chirho {
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    budget_chirho.work_chirho(vars_chirho.len()).ok_or(())?;
                    let start_chirho = self.elaboration_state_chirho.next_variable_chirho;
                    let (opened_chirho, additional_chirho) =
                        self.instantiate_scheme_parts_chirho(&SchemeChirho {
                            vars_chirho,
                            preds_chirho: Vec::new(),
                            ty_chirho: *body_chirho,
                        });
                    if quick_look_chirho {
                        look_chirho.variables_chirho.extend(
                            (start_chirho..self.elaboration_state_chirho.next_variable_chirho)
                                .map(TyVarChirho),
                        );
                    }
                    predicates_chirho.extend(additional_chirho);
                    current_chirho = opened_chirho;
                }
                TyChirho::QualChirho { .. } => {
                    let (opened_chirho, additional_chirho) =
                        self.expose_qualified_parts_chirho(current_chirho, Vec::new());
                    predicates_chirho.extend(additional_chirho);
                    current_chirho = opened_chirho;
                }
                _ => return Ok((current_chirho, predicates_chirho)),
            }
        }
    }

    fn finish_application_chirho(
        &mut self,
        plan_chirho: ApplicationPlanChirho<'_>,
        expected_chirho: Option<&TyChirho>,
        look_chirho: &QuickLookChirho,
        budget_chirho: &mut EqualityBudgetChirho,
        carried_chirho: SubstChirho,
        span_chirho: SpanChirho,
    ) -> Option<(SubstChirho, TyChirho)> {
        if self.has_non_deferrable_failure_chirho() {
            return None;
        }
        if let Some(expected_chirho) = expected_chirho
            && matches!(
                expected_chirho,
                TyChirho::ForallChirho { .. } | TyChirho::QualChirho { .. }
            )
        {
            return self.check_expected_sigma_chirho(
                expected_chirho,
                span_chirho,
                budget_chirho,
                |context_chirho, opened_chirho, budget_chirho| {
                    context_chirho.finish_application_chirho(
                        plan_chirho,
                        Some(opened_chirho),
                        look_chirho,
                        budget_chirho,
                        carried_chirho,
                        span_chirho,
                    )
                },
            );
        }
        let mut substitution_chirho = carried_chirho;
        let mut predicates_chirho = Vec::new();
        for predicate_chirho in plan_chirho.predicates_chirho {
            let mut map_chirho = |term_chirho: &TyChirho| {
                look_chirho
                    .apply_chirho(term_chirho, budget_chirho)
                    .map(|term_chirho| substitution_chirho.apply_ty_chirho(&term_chirho))
            };
            predicates_chirho.push(PredChirho::from_proof_chirho(
                predicate_chirho.class_identity_chirho().clone(),
                PredPartsChirho {
                    class_name_chirho: predicate_chirho.class_name_chirho.clone(),
                    kind_tys_chirho: predicate_chirho
                        .kind_tys_chirho
                        .iter()
                        .map(&mut map_chirho)
                        .collect::<Option<Vec<_>>>()?,
                    ty_chirho: map_chirho(&predicate_chirho.ty_chirho)?,
                    extra_tys_chirho: predicate_chirho
                        .extra_tys_chirho
                        .iter()
                        .map(&mut map_chirho)
                        .collect::<Option<Vec<_>>>()?,
                },
            ));
        }
        if plan_chirho.variable_head_chirho {
            self.recursive_references_chirho.capture_chirho(
                plan_chirho.name_chirho.text_chirho(),
                plan_chirho.name_chirho.span_chirho(),
                plan_chirho.name_chirho.origin_chirho(),
                &plan_chirho.scheme_chirho,
            );
        }
        self.capture_occurrence_predicates_chirho(plan_chirho.name_chirho, &predicates_chirho);
        for predicate_chirho in predicates_chirho {
            self.defer_pred_chirho(predicate_chirho, plan_chirho.name_chirho.span_chirho());
        }
        let result_chirho = look_chirho.apply_chirho(&plan_chirho.result_chirho, budget_chirho)?;
        if let Some(expected_chirho) = expected_chirho
            && let Ok(mut seed_chirho) = self.unify_normalized_chirho(
                &substitution_chirho.apply_ty_chirho(&result_chirho),
                expected_chirho,
                span_chirho,
            )
        {
            if self.apply_subst_all_chirho(&mut seed_chirho).is_err() {
                return None;
            };
            substitution_chirho = seed_chirho.compose_chirho(&substitution_chirho);
        }
        for argument_chirho in plan_chirho.arguments_chirho {
            let formal_chirho = substitution_chirho.apply_ty_chirho(
                &look_chirho.apply_chirho(&argument_chirho.formal_chirho, budget_chirho)?,
            );
            let (mut checked_chirho, actual_chirho) =
                if let Some(prepared_chirho) = argument_chirho.prepared_chirho {
                    self.finish_application_chirho(
                        *prepared_chirho,
                        Some(&formal_chirho),
                        look_chirho,
                        budget_chirho,
                        substitution_chirho.clone(),
                        argument_chirho.expression_chirho.span_chirho(),
                    )?
                } else {
                    self.infer_expr_against_expected_chirho(
                        argument_chirho.expression_chirho,
                        &formal_chirho,
                        argument_chirho.expression_chirho.span_chirho(),
                    )
                };
            if self.apply_subst_all_chirho(&mut checked_chirho).is_err() {
                return None;
            };
            substitution_chirho = checked_chirho.compose_chirho(&substitution_chirho);
            match self.unify_normalized_chirho(
                &substitution_chirho.apply_ty_chirho(&actual_chirho),
                &substitution_chirho.apply_ty_chirho(&formal_chirho),
                argument_chirho.expression_chirho.span_chirho(),
            ) {
                Ok(mut checked_chirho) => {
                    if self.apply_subst_all_chirho(&mut checked_chirho).is_err() {
                        return None;
                    };
                    substitution_chirho = checked_chirho.compose_chirho(&substitution_chirho);
                }
                Err(error_chirho) => self.report_unify_error_chirho(&error_chirho),
            }
        }
        let result_chirho = substitution_chirho.apply_ty_chirho(&result_chirho);
        if let Some(expected_chirho) = expected_chirho {
            match self.unify_normalized_chirho(
                &result_chirho,
                &substitution_chirho.apply_ty_chirho(expected_chirho),
                span_chirho,
            ) {
                Ok(mut checked_chirho) => {
                    if self.apply_subst_all_chirho(&mut checked_chirho).is_err() {
                        return None;
                    };
                    substitution_chirho = checked_chirho.compose_chirho(&substitution_chirho);
                }
                Err(error_chirho) => self.report_unify_error_chirho(&error_chirho),
            }
        }
        Some((
            substitution_chirho.clone(),
            substitution_chirho.apply_ty_chirho(&result_chirho),
        ))
    }
}
