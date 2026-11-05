// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Capture-avoiding synonym instantiation with one work/output allowance.
//! The catalogue supplies the complete telescope; names supply no authority.

use super::InferCtxChirho;
use super::equality_normalization_chirho::EqualityBudgetChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};

#[derive(Default)]
struct SynonymBindingsChirho {
    numeric_chirho: Vec<(TyVarChirho, TyChirho)>,
    named_chirho: Vec<(String, TyChirho)>,
}

struct SynonymExpansionChirho<'context_chirho, 'budget_chirho> {
    context_chirho: &'context_chirho InferCtxChirho,
    budget_chirho: &'budget_chirho mut EqualityBudgetChirho,
    next_bound_chirho: u32,
}

impl SynonymExpansionChirho<'_, '_> {
    fn reserve_variables_chirho(&mut self, ty_chirho: &TyChirho) -> Option<()> {
        let mut pending_chirho = vec![(ty_chirho, 0_usize)];
        while let Some((ty_chirho, depth_chirho)) = pending_chirho.pop() {
            self.budget_chirho.work_chirho(1)?;
            if depth_chirho > 128 {
                return None;
            }
            match ty_chirho {
                TyChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                } => {
                    self.budget_chirho.work_chirho(context_chirho.len())?;
                    pending_chirho.push((body_chirho, depth_chirho + 1));
                    pending_chirho.extend(
                        context_chirho
                            .iter()
                            .map(|constraint_chirho| (constraint_chirho, depth_chirho + 1)),
                    );
                }
                TyChirho::VarChirho(variable_chirho) => {
                    self.next_bound_chirho = self
                        .next_bound_chirho
                        .max(variable_chirho.0.checked_add(1)?);
                }
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                }
                | TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    self.budget_chirho.work_chirho(vars_chirho.len())?;
                    for variable_chirho in vars_chirho {
                        self.next_bound_chirho = self
                            .next_bound_chirho
                            .max(variable_chirho.0.checked_add(1)?);
                    }
                    for record_chirho in vars_chirho.records_chirho() {
                        if let Some(kind_chirho) = &record_chirho.kind_chirho {
                            pending_chirho.push((kind_chirho.as_ref(), depth_chirho + 1));
                        }
                    }
                    pending_chirho.push((body_chirho, depth_chirho + 1));
                }
                TyChirho::AppChirho(left_chirho, right_chirho)
                | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
                    pending_chirho.push((left_chirho, depth_chirho + 1));
                    pending_chirho.push((right_chirho, depth_chirho + 1));
                }
                TyChirho::TupleChirho(elements_chirho) => {
                    self.budget_chirho.work_chirho(elements_chirho.len())?;
                    pending_chirho.extend(
                        elements_chirho
                            .iter()
                            .map(|element_chirho| (element_chirho, depth_chirho + 1)),
                    );
                }
                TyChirho::ListChirho(element_chirho) => {
                    pending_chirho.push((element_chirho, depth_chirho + 1))
                }
                TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {}
            }
        }
        Some(())
    }

    fn walk_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        bindings_chirho: &SynonymBindingsChirho,
        bound_chirho: &mut Vec<(TyVarChirho, TyVarChirho)>,
        rename_chirho: bool,
        depth_chirho: usize,
    ) -> Option<TyChirho> {
        self.budget_chirho.work_chirho(1)?;
        if depth_chirho > 128 {
            return None;
        }
        if let TyChirho::VarChirho(variable_chirho) = ty_chirho {
            self.budget_chirho.work_chirho(bound_chirho.len())?;
            if let Some((_, image_chirho)) = bound_chirho
                .iter()
                .rev()
                .find(|(source_chirho, _)| source_chirho == variable_chirho)
            {
                self.budget_chirho.node_chirho(1)?;
                return Some(TyChirho::VarChirho(*image_chirho));
            }
            self.budget_chirho
                .work_chirho(bindings_chirho.numeric_chirho.len())?;
            if let Some((_, image_chirho)) = bindings_chirho
                .numeric_chirho
                .iter()
                .find(|(source_chirho, _)| source_chirho == variable_chirho)
            {
                return self.walk_chirho(
                    image_chirho,
                    &SynonymBindingsChirho::default(),
                    &mut Vec::new(),
                    false,
                    depth_chirho + 1,
                );
            }
        }
        if let TyChirho::ForallVarChirho(name_chirho) = ty_chirho {
            for (parameter_chirho, image_chirho) in &bindings_chirho.named_chirho {
                self.budget_chirho.work_chirho(
                    name_chirho
                        .len()
                        .saturating_add(parameter_chirho.len())
                        .saturating_add(1),
                )?;
                if parameter_chirho == name_chirho {
                    return self.walk_chirho(
                        image_chirho,
                        &SynonymBindingsChirho::default(),
                        &mut Vec::new(),
                        false,
                        depth_chirho + 1,
                    );
                }
            }
        }
        let mut head_chirho = ty_chirho;
        let mut arguments_chirho = Vec::new();
        while let TyChirho::AppChirho(fun_chirho, argument_chirho) = head_chirho {
            self.budget_chirho.work_chirho(1)?;
            if depth_chirho.saturating_add(arguments_chirho.len()) > 128 {
                return None;
            }
            arguments_chirho.push(argument_chirho.as_ref());
            head_chirho = fun_chirho;
        }
        arguments_chirho.reverse();
        if let TyChirho::ConChirho(name_chirho) = head_chirho {
            self.budget_chirho
                .work_chirho(name_chirho.len().saturating_add(1))?;
            // Exact producer keys reach this point from the AST boundary.
            // A checked head can never fall back to a spelling synonym.
            if let Some(scheme_chirho) = self
                .context_chirho
                .type_families_chirho
                .declaration_schemes_chirho()
                .get_chirho(name_chirho)
                && scheme_chirho.owner_chirho.is_none()
                && !scheme_chirho.held_chirho
                && scheme_chirho.reduction_arity_chirho <= arguments_chirho.len()
                && let Some(rhs_chirho) = &scheme_chirho.synonym_rhs_chirho
            {
                let mut replacement_chirho = SynonymBindingsChirho::default();
                self.budget_chirho
                    .node_chirho(scheme_chirho.binders_chirho.len())?;
                for (binder_chirho, argument_chirho) in
                    scheme_chirho.binders_chirho.iter().zip(&arguments_chirho)
                {
                    replacement_chirho.numeric_chirho.push((
                        binder_chirho.variable_chirho,
                        self.walk_chirho(
                            argument_chirho,
                            bindings_chirho,
                            bound_chirho,
                            rename_chirho,
                            depth_chirho + 1,
                        )?,
                    ));
                }
                self.reserve_variables_chirho(rhs_chirho)?;
                self.reserve_variables_chirho(&scheme_chirho.result_kind_chirho)?;
                let mut kind_chirho = Some(self.walk_chirho(
                    &scheme_chirho.result_kind_chirho,
                    &replacement_chirho,
                    &mut Vec::new(),
                    true,
                    depth_chirho + 1,
                )?);
                let mut expanded_chirho = self.walk_chirho(
                    rhs_chirho,
                    &replacement_chirho,
                    &mut Vec::new(),
                    true,
                    depth_chirho + 1,
                )?;
                for argument_chirho in &arguments_chirho[scheme_chirho.reduction_arity_chirho..] {
                    let argument_chirho = self.walk_chirho(
                        argument_chirho,
                        bindings_chirho,
                        bound_chirho,
                        rename_chirho,
                        depth_chirho + 1,
                    )?;
                    let mut quantified_chirho = false;
                    loop {
                        match kind_chirho.take() {
                            Some(TyChirho::ForallChirho {
                                vars_chirho,
                                body_chirho,
                            }) if vars_chirho.is_empty() => kind_chirho = Some(*body_chirho),
                            Some(TyChirho::RequiredForallChirho {
                                vars_chirho,
                                body_chirho,
                            }) if vars_chirho.is_empty() => kind_chirho = Some(*body_chirho),
                            Some(
                                kind_term_chirho @ (TyChirho::ForallChirho { .. }
                                | TyChirho::RequiredForallChirho { .. }),
                            ) => {
                                let reduced_chirho = crate::subst_chirho::SubstChirho::reduce_checked_argument_chirho(
                                    &kind_term_chirho, &argument_chirho, true, &mut |work_chirho, output_chirho| {
                                        self.budget_chirho.node_chirho(output_chirho)?;
                                        self.budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                                    })??;
                                kind_chirho = Some(reduced_chirho.0);
                                quantified_chirho = true;
                                break;
                            }
                            Some(TyChirho::FunChirho(_, result_chirho, _)) => {
                                kind_chirho = Some(*result_chirho);
                                break;
                            }
                            _ => {
                                kind_chirho = None;
                                break;
                            }
                        }
                    }
                    let reduced_chirho =
                        crate::subst_chirho::SubstChirho::reduce_checked_argument_chirho(
                            &expanded_chirho,
                            &argument_chirho,
                            quantified_chirho,
                            &mut |work_chirho, output_chirho| {
                                self.budget_chirho.node_chirho(output_chirho)?;
                                self.budget_chirho
                                    .work_chirho(work_chirho.saturating_sub(output_chirho))
                            },
                        )?;
                    expanded_chirho = if let Some((reduced_chirho, _)) = reduced_chirho {
                        reduced_chirho
                    } else {
                        self.budget_chirho.node_chirho(1)?;
                        TyChirho::AppChirho(Box::new(expanded_chirho), Box::new(argument_chirho))
                    };
                }
                return Some(expanded_chirho);
            }
            if let Some((parameters_chirho, rhs_chirho)) =
                self.context_chirho.lookup_type_synonym_chirho(name_chirho)
                && parameters_chirho.len() <= arguments_chirho.len()
            {
                let mut replacement_chirho = SynonymBindingsChirho::default();
                for (parameter_chirho, argument_chirho) in
                    parameters_chirho.iter().zip(&arguments_chirho)
                {
                    self.budget_chirho
                        .node_chirho(parameter_chirho.len().saturating_add(1))?;
                    replacement_chirho.named_chirho.push((
                        parameter_chirho.clone(),
                        self.walk_chirho(
                            argument_chirho,
                            bindings_chirho,
                            bound_chirho,
                            rename_chirho,
                            depth_chirho + 1,
                        )?,
                    ));
                }
                self.reserve_variables_chirho(rhs_chirho)?;
                let mut expanded_chirho = self.walk_chirho(
                    rhs_chirho,
                    &replacement_chirho,
                    &mut Vec::new(),
                    true,
                    depth_chirho + 1,
                )?;
                for argument_chirho in &arguments_chirho[parameters_chirho.len()..] {
                    self.budget_chirho.node_chirho(1)?;
                    expanded_chirho = TyChirho::AppChirho(
                        Box::new(expanded_chirho),
                        Box::new(self.walk_chirho(
                            argument_chirho,
                            bindings_chirho,
                            bound_chirho,
                            rename_chirho,
                            depth_chirho + 1,
                        )?),
                    );
                }
                return Some(expanded_chirho);
            }
        }
        self.budget_chirho.node_chirho(1)?;
        Some(match ty_chirho {
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                self.budget_chirho.work_chirho(context_chirho.len())?;
                TyChirho::QualChirho {
                    context_chirho: context_chirho
                        .iter()
                        .map(|constraint_chirho| {
                            self.walk_chirho(
                                constraint_chirho,
                                bindings_chirho,
                                bound_chirho,
                                rename_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Option<Vec<_>>>()?,
                    body_chirho: Box::new(self.walk_chirho(
                        body_chirho,
                        bindings_chirho,
                        bound_chirho,
                        rename_chirho,
                        depth_chirho + 1,
                    )?),
                }
            }
            TyChirho::VarChirho(variable_chirho) => TyChirho::VarChirho(*variable_chirho),
            TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
                self.budget_chirho.node_chirho(name_chirho.len())?;
                if matches!(ty_chirho, TyChirho::ConChirho(_)) {
                    TyChirho::ConChirho(name_chirho.clone())
                } else {
                    TyChirho::ForallVarChirho(name_chirho.clone())
                }
            }
            TyChirho::AppChirho(fun_chirho, argument_chirho) => TyChirho::AppChirho(
                Box::new(self.walk_chirho(
                    fun_chirho,
                    bindings_chirho,
                    bound_chirho,
                    rename_chirho,
                    depth_chirho + 1,
                )?),
                Box::new(self.walk_chirho(
                    argument_chirho,
                    bindings_chirho,
                    bound_chirho,
                    rename_chirho,
                    depth_chirho + 1,
                )?),
            ),
            TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
                TyChirho::FunChirho(
                    Box::new(self.walk_chirho(
                        argument_chirho,
                        bindings_chirho,
                        bound_chirho,
                        rename_chirho,
                        depth_chirho + 1,
                    )?),
                    Box::new(self.walk_chirho(
                        result_chirho,
                        bindings_chirho,
                        bound_chirho,
                        rename_chirho,
                        depth_chirho + 1,
                    )?),
                    *multiplicity_chirho,
                )
            }
            TyChirho::ListChirho(element_chirho) => {
                TyChirho::ListChirho(Box::new(self.walk_chirho(
                    element_chirho,
                    bindings_chirho,
                    bound_chirho,
                    rename_chirho,
                    depth_chirho + 1,
                )?))
            }
            TyChirho::TupleChirho(elements_chirho) => {
                self.budget_chirho.work_chirho(elements_chirho.len())?;
                TyChirho::TupleChirho(
                    elements_chirho
                        .iter()
                        .map(|element_chirho| {
                            self.walk_chirho(
                                element_chirho,
                                bindings_chirho,
                                bound_chirho,
                                rename_chirho,
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
                self.budget_chirho.node_chirho(vars_chirho.len())?;
                let mark_chirho = bound_chirho.len();
                let result_chirho = (|| {
                    let mut records_chirho = Vec::with_capacity(vars_chirho.len());
                    for record_chirho in vars_chirho.records_chirho() {
                        let image_chirho = if rename_chirho {
                            let fresh_chirho = TyVarChirho(self.next_bound_chirho);
                            self.next_bound_chirho = self.next_bound_chirho.checked_add(1)?;
                            fresh_chirho
                        } else {
                            record_chirho.variable_chirho
                        };
                        let kind_chirho = if let Some(kind_chirho) = &record_chirho.kind_chirho {
                            Some(std::sync::Arc::new(self.walk_chirho(
                                kind_chirho,
                                bindings_chirho,
                                bound_chirho,
                                rename_chirho,
                                depth_chirho + 1,
                            )?))
                        } else {
                            None
                        };
                        records_chirho.push(crate::ty_chirho::QuantifiedVariableChirho {
                            variable_chirho: image_chirho,
                            kind_chirho,
                            source_chirho: record_chirho.source_chirho.clone(),
                            specificity_chirho: record_chirho.specificity_chirho,
                        });
                        bound_chirho.push((record_chirho.variable_chirho, image_chirho));
                    }
                    let body_chirho = self.walk_chirho(
                        body_chirho,
                        bindings_chirho,
                        bound_chirho,
                        rename_chirho,
                        depth_chirho + 1,
                    )?;
                    Some((
                        crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(
                            records_chirho,
                        ),
                        body_chirho,
                    ))
                })();
                bound_chirho.truncate(mark_chirho);
                let (variables_chirho, body_chirho) = result_chirho?;
                if matches!(ty_chirho, TyChirho::ForallChirho { .. }) {
                    TyChirho::ForallChirho {
                        vars_chirho: variables_chirho,
                        body_chirho: Box::new(body_chirho),
                    }
                } else {
                    TyChirho::RequiredForallChirho {
                        vars_chirho: variables_chirho,
                        body_chirho: Box::new(body_chirho),
                    }
                }
            }
        })
    }
}

pub(super) fn expand_synonyms_with_budget_chirho(
    context_chirho: &InferCtxChirho,
    ty_chirho: &TyChirho,
    budget_chirho: &mut EqualityBudgetChirho,
) -> Option<TyChirho> {
    let mut expansion_chirho = SynonymExpansionChirho {
        context_chirho,
        budget_chirho,
        next_bound_chirho: context_chirho.next_var_chirho,
    };
    expansion_chirho.reserve_variables_chirho(ty_chirho)?;
    expansion_chirho.walk_chirho(
        ty_chirho,
        &SynonymBindingsChirho::default(),
        &mut Vec::new(),
        false,
        0,
    )
}
