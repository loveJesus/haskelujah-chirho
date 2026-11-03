// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One work/output allowance covers an equality's complete normalization.
//! Every clone, substitution, expansion and evidence comparison is charged.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct EqualityNormalizationExhaustedChirho;

pub(super) struct EqualityBudgetChirho {
    work_chirho: usize,
    output_chirho: usize,
}

impl EqualityBudgetChirho {
    pub(super) fn new_chirho() -> Self {
        Self {
            work_chirho: 10_000_000,
            output_chirho: 16_384,
        }
    }
    pub(super) fn work_chirho(&mut self, amount_chirho: usize) -> Option<()> {
        self.work_chirho = self.work_chirho.checked_sub(amount_chirho)?;
        Some(())
    }
    fn node_chirho(&mut self, amount_chirho: usize) -> Option<()> {
        self.work_chirho(amount_chirho)?;
        self.output_chirho = self.output_chirho.checked_sub(amount_chirho)?;
        Some(())
    }
}

struct NamedEqualityBindingsChirho<'source_chirho> {
    parent_chirho: Option<&'source_chirho NamedEqualityBindingsChirho<'source_chirho>>,
    bindings_chirho: Vec<(&'source_chirho str, &'source_chirho TyChirho)>,
}

fn equal_chirho(
    left_chirho: &TyChirho,
    right_chirho: &TyChirho,
    budget_chirho: &mut EqualityBudgetChirho,
) -> Option<bool> {
    let mut pending_chirho = vec![(left_chirho, right_chirho, 0_usize)];
    while let Some((left_chirho, right_chirho, depth_chirho)) = pending_chirho.pop() {
        budget_chirho.work_chirho(1)?;
        if depth_chirho > 128 {
            return None;
        }
        match (left_chirho, right_chirho) {
            (TyChirho::VarChirho(left_chirho), TyChirho::VarChirho(right_chirho))
                if left_chirho == right_chirho => {}
            (TyChirho::ConChirho(left_chirho), TyChirho::ConChirho(right_chirho))
            | (TyChirho::ForallVarChirho(left_chirho), TyChirho::ForallVarChirho(right_chirho)) => {
                budget_chirho.work_chirho(left_chirho.len().saturating_add(right_chirho.len()))?;
                if left_chirho != right_chirho {
                    return Some(false);
                }
            }
            (
                TyChirho::AppChirho(left_fun_chirho, left_arg_chirho),
                TyChirho::AppChirho(right_fun_chirho, right_arg_chirho),
            ) => {
                pending_chirho.push((left_fun_chirho, right_fun_chirho, depth_chirho + 1));
                pending_chirho.push((left_arg_chirho, right_arg_chirho, depth_chirho + 1));
            }
            (
                TyChirho::FunChirho(left_arg_chirho, left_result_chirho, left_mult_chirho),
                TyChirho::FunChirho(right_arg_chirho, right_result_chirho, right_mult_chirho),
            ) if left_mult_chirho == right_mult_chirho => {
                pending_chirho.push((left_arg_chirho, right_arg_chirho, depth_chirho + 1));
                pending_chirho.push((left_result_chirho, right_result_chirho, depth_chirho + 1));
            }
            (TyChirho::TupleChirho(left_chirho), TyChirho::TupleChirho(right_chirho))
                if left_chirho.len() == right_chirho.len() =>
            {
                budget_chirho.work_chirho(left_chirho.len())?;
                pending_chirho.extend(left_chirho.iter().zip(right_chirho).map(
                    |(left_chirho, right_chirho)| (left_chirho, right_chirho, depth_chirho + 1),
                ));
            }
            (TyChirho::ListChirho(left_chirho), TyChirho::ListChirho(right_chirho)) => {
                pending_chirho.push((left_chirho, right_chirho, depth_chirho + 1))
            }
            (
                TyChirho::ForallChirho {
                    vars_chirho: left_vars_chirho,
                    body_chirho: left_body_chirho,
                },
                TyChirho::ForallChirho {
                    vars_chirho: right_vars_chirho,
                    body_chirho: right_body_chirho,
                },
            )
            | (
                TyChirho::RequiredForallChirho {
                    vars_chirho: left_vars_chirho,
                    body_chirho: left_body_chirho,
                },
                TyChirho::RequiredForallChirho {
                    vars_chirho: right_vars_chirho,
                    body_chirho: right_body_chirho,
                },
            ) => {
                budget_chirho.work_chirho(
                    left_vars_chirho
                        .len()
                        .saturating_add(right_vars_chirho.len()),
                )?;
                if left_vars_chirho != right_vars_chirho {
                    return Some(false);
                }
                pending_chirho.push((left_body_chirho, right_body_chirho, depth_chirho + 1));
            }
            _ => return Some(false),
        }
    }
    Some(true)
}

fn transform_chirho<'source_chirho>(
    ctx_chirho: &'source_chirho InferCtxChirho,
    ty_chirho: &'source_chirho TyChirho,
    rewrites_chirho: &'source_chirho [(TyChirho, TyChirho)],
    named_chirho: Option<&'source_chirho NamedEqualityBindingsChirho<'source_chirho>>,
    bound_chirho: &mut HashSet<TyVarChirho>,
    expand_chirho: bool,
    budget_chirho: &mut EqualityBudgetChirho,
    depth_chirho: usize,
) -> Option<TyChirho> {
    budget_chirho.work_chirho(1)?;
    if depth_chirho > 128 {
        return None;
    }
    if let TyChirho::ForallVarChirho(name_chirho) = ty_chirho {
        let mut frame_chirho = named_chirho;
        while let Some(current_chirho) = frame_chirho {
            budget_chirho.work_chirho(current_chirho.bindings_chirho.len().saturating_add(1))?;
            for (parameter_chirho, value_chirho) in &current_chirho.bindings_chirho {
                budget_chirho
                    .work_chirho(parameter_chirho.len().saturating_add(name_chirho.len()))?;
                if *parameter_chirho == name_chirho {
                    return transform_chirho(
                        ctx_chirho,
                        value_chirho,
                        rewrites_chirho,
                        current_chirho.parent_chirho,
                        bound_chirho,
                        expand_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    );
                }
            }
            frame_chirho = current_chirho.parent_chirho;
        }
    }
    for (lhs_chirho, rhs_chirho) in rewrites_chirho.iter().filter(|_| named_chirho.is_none()) {
        if equal_chirho(ty_chirho, lhs_chirho, budget_chirho)? {
            // Replacement is cloned through the same budget, without recursively
            // firing evidence on itself in this pass. Later passes share the allowance.
            return transform_chirho(
                ctx_chirho,
                rhs_chirho,
                &[],
                None,
                bound_chirho,
                expand_chirho,
                budget_chirho,
                depth_chirho + 1,
            );
        }
    }
    if let TyChirho::VarChirho(var_chirho) = ty_chirho
        && !bound_chirho.contains(var_chirho)
        && let Some(image_chirho) = ctx_chirho
            .equality_substitutions_chirho
            .lookup_chirho(var_chirho)
        && !matches!(image_chirho, TyChirho::VarChirho(other_chirho) if other_chirho == var_chirho)
    {
        return transform_chirho(
            ctx_chirho,
            image_chirho,
            rewrites_chirho,
            None,
            bound_chirho,
            expand_chirho,
            budget_chirho,
            depth_chirho + 1,
        );
    }
    if expand_chirho {
        let mut head_chirho = ty_chirho;
        let mut arguments_chirho = Vec::new();
        while let TyChirho::AppChirho(fun_chirho, arg_chirho) = head_chirho {
            budget_chirho.work_chirho(1)?;
            if depth_chirho.saturating_add(arguments_chirho.len()) > 128 {
                return None;
            }
            arguments_chirho.push(arg_chirho.as_ref());
            head_chirho = fun_chirho;
        }
        arguments_chirho.reverse();
        if let TyChirho::ConChirho(name_chirho) = head_chirho
            && {
                budget_chirho.work_chirho(name_chirho.len())?;
                true
            }
            && let Some((parameters_chirho, rhs_chirho)) =
                ctx_chirho.lookup_type_synonym_chirho(name_chirho)
            && parameters_chirho.len() <= arguments_chirho.len()
        {
            budget_chirho.work_chirho(parameters_chirho.len())?;
            let frame_chirho = NamedEqualityBindingsChirho {
                parent_chirho: named_chirho,
                bindings_chirho: parameters_chirho
                    .iter()
                    .zip(&arguments_chirho)
                    .map(|(parameter_chirho, argument_chirho)| {
                        (parameter_chirho.as_str(), *argument_chirho)
                    })
                    .collect(),
            };
            let mut expanded_chirho = transform_chirho(
                ctx_chirho,
                rhs_chirho,
                rewrites_chirho,
                Some(&frame_chirho),
                bound_chirho,
                true,
                budget_chirho,
                depth_chirho + 1,
            )?;
            for argument_chirho in &arguments_chirho[parameters_chirho.len()..] {
                budget_chirho.node_chirho(1)?;
                expanded_chirho = TyChirho::AppChirho(
                    Box::new(expanded_chirho),
                    Box::new(transform_chirho(
                        ctx_chirho,
                        argument_chirho,
                        rewrites_chirho,
                        named_chirho,
                        bound_chirho,
                        true,
                        budget_chirho,
                        depth_chirho + 1,
                    )?),
                );
            }
            return Some(expanded_chirho);
        }
    }
    budget_chirho.node_chirho(1)?;
    Some(match ty_chirho {
        TyChirho::VarChirho(var_chirho) => TyChirho::VarChirho(*var_chirho),
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            budget_chirho.node_chirho(name_chirho.len())?;
            if matches!(ty_chirho, TyChirho::ConChirho(_)) {
                TyChirho::ConChirho(name_chirho.clone())
            } else {
                TyChirho::ForallVarChirho(name_chirho.clone())
            }
        }
        TyChirho::AppChirho(fun_chirho, arg_chirho) => TyChirho::AppChirho(
            Box::new(transform_chirho(
                ctx_chirho,
                fun_chirho,
                rewrites_chirho,
                named_chirho,
                bound_chirho,
                expand_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?),
            Box::new(transform_chirho(
                ctx_chirho,
                arg_chirho,
                rewrites_chirho,
                named_chirho,
                bound_chirho,
                expand_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?),
        ),
        TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => TyChirho::FunChirho(
            Box::new(transform_chirho(
                ctx_chirho,
                arg_chirho,
                rewrites_chirho,
                named_chirho,
                bound_chirho,
                expand_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?),
            Box::new(transform_chirho(
                ctx_chirho,
                result_chirho,
                rewrites_chirho,
                named_chirho,
                bound_chirho,
                expand_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?),
            *mult_chirho,
        ),
        TyChirho::TupleChirho(elements_chirho) => TyChirho::TupleChirho(
            elements_chirho
                .iter()
                .map(|element_chirho| {
                    transform_chirho(
                        ctx_chirho,
                        element_chirho,
                        rewrites_chirho,
                        named_chirho,
                        bound_chirho,
                        expand_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )
                })
                .collect::<Option<Vec<_>>>()?,
        ),
        TyChirho::ListChirho(element_chirho) => TyChirho::ListChirho(Box::new(transform_chirho(
            ctx_chirho,
            element_chirho,
            rewrites_chirho,
            named_chirho,
            bound_chirho,
            expand_chirho,
            budget_chirho,
            depth_chirho + 1,
        )?)),
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            budget_chirho.node_chirho(vars_chirho.len())?;
            let inserted_chirho = vars_chirho
                .iter()
                .copied()
                .filter(|var_chirho| bound_chirho.insert(*var_chirho))
                .collect::<Vec<_>>();
            let body_chirho = transform_chirho(
                ctx_chirho,
                body_chirho,
                rewrites_chirho,
                named_chirho,
                bound_chirho,
                expand_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?;
            for var_chirho in inserted_chirho {
                bound_chirho.remove(&var_chirho);
            }
            if matches!(ty_chirho, TyChirho::ForallChirho { .. }) {
                TyChirho::ForallChirho {
                    vars_chirho: vars_chirho.clone(),
                    body_chirho: Box::new(body_chirho),
                }
            } else {
                TyChirho::RequiredForallChirho {
                    vars_chirho: vars_chirho.clone(),
                    body_chirho: Box::new(body_chirho),
                }
            }
        }
    })
}

pub(super) fn normalize_owned_equality_chirho<'evidence_chirho>(
    ctx_chirho: &InferCtxChirho,
    ty_chirho: &TyChirho,
    evidence_chirho: impl Iterator<Item = &'evidence_chirho [(TyChirho, TyChirho)]>,
) -> Result<TyChirho, EqualityNormalizationExhaustedChirho> {
    normalize_owned_equality_with_budget_chirho(
        ctx_chirho,
        ty_chirho,
        evidence_chirho,
        &mut EqualityBudgetChirho::new_chirho(),
    )
}

pub(super) fn normalize_owned_equality_with_budget_chirho<'evidence_chirho>(
    ctx_chirho: &InferCtxChirho,
    ty_chirho: &TyChirho,
    evidence_chirho: impl Iterator<Item = &'evidence_chirho [(TyChirho, TyChirho)]>,
    budget_chirho: &mut EqualityBudgetChirho,
) -> Result<TyChirho, EqualityNormalizationExhaustedChirho> {
    let mut chunks_chirho = Vec::new();
    for chunk_chirho in evidence_chirho {
        budget_chirho
            .work_chirho(1)
            .ok_or(EqualityNormalizationExhaustedChirho)?;
        if chunk_chirho.is_empty() {
            continue;
        }
        budget_chirho
            .node_chirho(1)
            .ok_or(EqualityNormalizationExhaustedChirho)?;
        let mut copied_chirho = Vec::new();
        for (lhs_chirho, rhs_chirho) in chunk_chirho {
            let lhs_chirho = transform_chirho(
                ctx_chirho,
                lhs_chirho,
                &[],
                None,
                &mut HashSet::new(),
                false,
                budget_chirho,
                0,
            )
            .ok_or(EqualityNormalizationExhaustedChirho)?;
            let rhs_chirho = transform_chirho(
                ctx_chirho,
                rhs_chirho,
                &[],
                None,
                &mut HashSet::new(),
                false,
                budget_chirho,
                0,
            )
            .ok_or(EqualityNormalizationExhaustedChirho)?;
            copied_chirho.push((lhs_chirho, rhs_chirho));
        }
        chunks_chirho.push(copied_chirho);
    }
    let rewrites_chirho = chunks_chirho
        .into_iter()
        .rev()
        .flatten()
        .collect::<Vec<_>>();
    let mut current_chirho = transform_chirho(
        ctx_chirho,
        ty_chirho,
        &[],
        None,
        &mut HashSet::new(),
        false,
        budget_chirho,
        0,
    )
    .ok_or(EqualityNormalizationExhaustedChirho)?;
    for _iteration_chirho in 0..100 {
        let transformed_chirho = transform_chirho(
            ctx_chirho,
            &current_chirho,
            &rewrites_chirho,
            None,
            &mut HashSet::new(),
            true,
            budget_chirho,
            0,
        )
        .ok_or(EqualityNormalizationExhaustedChirho)?;
        let next_chirho = ctx_chirho
            .reduce_families_bounded_chirho(
                &transformed_chirho,
                0,
                &mut budget_chirho.work_chirho,
                &mut budget_chirho.output_chirho,
            )
            .ok_or(EqualityNormalizationExhaustedChirho)?;
        if equal_chirho(&next_chirho, &current_chirho, budget_chirho)
            .ok_or(EqualityNormalizationExhaustedChirho)?
        {
            return Ok(next_chirho);
        }
        current_chirho = next_chirho;
    }
    Err(EqualityNormalizationExhaustedChirho)
}
