// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One work/output allowance covers an equality's complete normalization.
//! Every clone, substitution, expansion and evidence comparison is charged.

use super::*;
mod context_chirho;
pub(crate) use context_chirho::{EqualityNormalizationViewChirho, NormalizationContextChirho};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EqualityNormalizationExhaustedChirho;

pub(crate) struct EqualityBudgetChirho {
    work_chirho: usize,
    output_chirho: usize,
}

impl EqualityBudgetChirho {
    pub(crate) fn limited_chirho(work_chirho: usize, output_chirho: usize) -> Self {
        Self {
            work_chirho: work_chirho.min(10_000_000),
            output_chirho: output_chirho.min(16_384),
        }
    }
    pub(crate) fn remaining_chirho(&self) -> (usize, usize) {
        (self.work_chirho, self.output_chirho)
    }
    pub(crate) fn new_chirho() -> Self {
        Self {
            work_chirho: 10_000_000,
            output_chirho: 16_384,
        }
    }
    pub(super) fn work_chirho(&mut self, amount_chirho: usize) -> Option<()> {
        self.work_chirho = self.work_chirho.checked_sub(amount_chirho)?;
        Some(())
    }
    pub(super) fn node_chirho(&mut self, amount_chirho: usize) -> Option<()> {
        self.work_chirho(amount_chirho)?;
        self.output_chirho = self.output_chirho.checked_sub(amount_chirho)?;
        Some(())
    }
}

struct NamedEqualityBindingsChirho<'source_chirho> {
    parent_chirho: Option<&'source_chirho NamedEqualityBindingsChirho<'source_chirho>>,
    bindings_chirho: Vec<(&'source_chirho str, &'source_chirho TyChirho)>,
}

pub(super) fn equal_chirho(
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
            (
                TyChirho::QualChirho {
                    context_chirho: left_context_chirho,
                    body_chirho: left_body_chirho,
                },
                TyChirho::QualChirho {
                    context_chirho: right_context_chirho,
                    body_chirho: right_body_chirho,
                },
            ) if left_context_chirho.len() == right_context_chirho.len() => {
                budget_chirho.work_chirho(left_context_chirho.len())?;
                pending_chirho.push((left_body_chirho, right_body_chirho, depth_chirho + 1));
                pending_chirho.extend(left_context_chirho.iter().zip(right_context_chirho).map(
                    |(left_chirho, right_chirho)| (left_chirho, right_chirho, depth_chirho + 1),
                ));
            }
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
                if left_vars_chirho.len() != right_vars_chirho.len() {
                    return Some(false);
                }
                for (left_chirho, right_chirho) in left_vars_chirho
                    .records_chirho()
                    .iter()
                    .zip(right_vars_chirho.records_chirho())
                {
                    if left_chirho.variable_chirho != right_chirho.variable_chirho {
                        return Some(false);
                    }
                    match (&left_chirho.kind_chirho, &right_chirho.kind_chirho) {
                        (Some(left_chirho), Some(right_chirho)) => pending_chirho.push((
                            left_chirho.as_ref(),
                            right_chirho.as_ref(),
                            depth_chirho + 1,
                        )),
                        (None, None) => {}
                        _ => return Some(false),
                    }
                }
                pending_chirho.push((left_body_chirho, right_body_chirho, depth_chirho + 1));
            }
            _ => return Some(false),
        }
    }
    Some(true)
}

#[derive(Clone, Copy)]
struct EqualityTransformPolicyChirho<'source_chirho> {
    ctx_chirho: &'source_chirho dyn NormalizationContextChirho,
    rewrites_chirho: &'source_chirho [(TyChirho, TyChirho)],
    expand_chirho: bool,
    resolve_meta_chirho: bool,
}

fn transform_chirho<'source_chirho>(
    policy_chirho: EqualityTransformPolicyChirho<'source_chirho>,
    ty_chirho: &'source_chirho TyChirho,
    named_chirho: Option<&'source_chirho NamedEqualityBindingsChirho<'source_chirho>>,
    bound_chirho: &mut HashSet<TyVarChirho>,
    budget_chirho: &mut EqualityBudgetChirho,
    depth_chirho: usize,
) -> Option<TyChirho> {
    let EqualityTransformPolicyChirho {
        ctx_chirho,
        rewrites_chirho,
        expand_chirho,
        resolve_meta_chirho,
    } = policy_chirho;
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
                        EqualityTransformPolicyChirho {
                            ctx_chirho,
                            rewrites_chirho,
                            expand_chirho,
                            resolve_meta_chirho,
                        },
                        value_chirho,
                        current_chirho.parent_chirho,
                        bound_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    );
                }
            }
            frame_chirho = current_chirho.parent_chirho;
        }
    }
    for (lhs_chirho, rhs_chirho) in rewrites_chirho.iter().filter(|_| named_chirho.is_none()) {
        if !bound_chirho.is_empty() {
            let free_chirho = SubstChirho::free_variables_bounded_chirho(
                lhs_chirho,
                &mut |work_chirho, output_chirho| {
                    budget_chirho.node_chirho(output_chirho)?;
                    budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                },
            )?;
            budget_chirho.work_chirho(free_chirho.len())?;
            if free_chirho
                .iter()
                .any(|variable_chirho| bound_chirho.contains(variable_chirho))
            {
                continue;
            }
        }
        if equal_chirho(ty_chirho, lhs_chirho, budget_chirho)? {
            // Replacement is cloned through the same budget, without recursively
            // firing evidence on itself in this pass. Later passes share the allowance.
            return transform_chirho(
                EqualityTransformPolicyChirho {
                    ctx_chirho,
                    rewrites_chirho: &[],
                    expand_chirho,
                    resolve_meta_chirho,
                },
                rhs_chirho,
                None,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            );
        }
    }
    if resolve_meta_chirho
        && let TyChirho::VarChirho(var_chirho) = ty_chirho
        && !bound_chirho.contains(var_chirho)
        && let Some(image_chirho) = ctx_chirho
            .equality_substitutions_chirho()
            .lookup_chirho(var_chirho)
        && !matches!(image_chirho, TyChirho::VarChirho(other_chirho) if other_chirho == var_chirho)
    {
        return transform_chirho(
            EqualityTransformPolicyChirho {
                ctx_chirho,
                rewrites_chirho,
                expand_chirho,
                resolve_meta_chirho,
            },
            image_chirho,
            None,
            bound_chirho,
            budget_chirho,
            depth_chirho + 1,
        );
    }
    if expand_chirho
        && named_chirho.is_none()
        && (ctx_chirho.owner_present_chirho()
            || !ctx_chirho
                .catalogue_chirho()
                .declaration_schemes_chirho()
                .is_empty_chirho())
    {
        let expanded_chirho = super::synonym_expansion_chirho::expand_synonyms_with_budget_chirho(
            ctx_chirho,
            ty_chirho,
            budget_chirho,
        )?;
        return transform_chirho(
            EqualityTransformPolicyChirho {
                ctx_chirho,
                rewrites_chirho,
                expand_chirho: false,
                resolve_meta_chirho,
            },
            &expanded_chirho,
            None,
            bound_chirho,
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
                EqualityTransformPolicyChirho {
                    ctx_chirho,
                    rewrites_chirho,
                    expand_chirho: true,
                    resolve_meta_chirho,
                },
                rhs_chirho,
                Some(&frame_chirho),
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?;
            for argument_chirho in &arguments_chirho[parameters_chirho.len()..] {
                budget_chirho.node_chirho(1)?;
                expanded_chirho = TyChirho::AppChirho(
                    Box::new(expanded_chirho),
                    Box::new(transform_chirho(
                        EqualityTransformPolicyChirho {
                            ctx_chirho,
                            rewrites_chirho,
                            expand_chirho: true,
                            resolve_meta_chirho,
                        },
                        argument_chirho,
                        named_chirho,
                        bound_chirho,
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
                EqualityTransformPolicyChirho {
                    ctx_chirho,
                    rewrites_chirho,
                    expand_chirho,
                    resolve_meta_chirho,
                },
                fun_chirho,
                named_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?),
            Box::new(transform_chirho(
                EqualityTransformPolicyChirho {
                    ctx_chirho,
                    rewrites_chirho,
                    expand_chirho,
                    resolve_meta_chirho,
                },
                arg_chirho,
                named_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?),
        ),
        TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => TyChirho::FunChirho(
            Box::new(transform_chirho(
                EqualityTransformPolicyChirho {
                    ctx_chirho,
                    rewrites_chirho,
                    expand_chirho,
                    resolve_meta_chirho,
                },
                arg_chirho,
                named_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?),
            Box::new(transform_chirho(
                EqualityTransformPolicyChirho {
                    ctx_chirho,
                    rewrites_chirho,
                    expand_chirho,
                    resolve_meta_chirho,
                },
                result_chirho,
                named_chirho,
                bound_chirho,
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
                        EqualityTransformPolicyChirho {
                            ctx_chirho,
                            rewrites_chirho,
                            expand_chirho,
                            resolve_meta_chirho,
                        },
                        element_chirho,
                        named_chirho,
                        bound_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )
                })
                .collect::<Option<Vec<_>>>()?,
        ),
        TyChirho::ListChirho(element_chirho) => TyChirho::ListChirho(Box::new(transform_chirho(
            EqualityTransformPolicyChirho {
                ctx_chirho,
                rewrites_chirho,
                expand_chirho,
                resolve_meta_chirho,
            },
            element_chirho,
            named_chirho,
            bound_chirho,
            budget_chirho,
            depth_chirho + 1,
        )?)),
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => {
            budget_chirho.work_chirho(context_chirho.len())?;
            TyChirho::QualChirho {
                context_chirho: context_chirho
                    .iter()
                    .map(|constraint_chirho| {
                        transform_chirho(
                            EqualityTransformPolicyChirho {
                                ctx_chirho,
                                rewrites_chirho,
                                expand_chirho,
                                resolve_meta_chirho,
                            },
                            constraint_chirho,
                            named_chirho,
                            bound_chirho,
                            budget_chirho,
                            depth_chirho + 1,
                        )
                    })
                    .collect::<Option<Vec<_>>>()?,
                body_chirho: Box::new(transform_chirho(
                    EqualityTransformPolicyChirho {
                        ctx_chirho,
                        rewrites_chirho,
                        expand_chirho,
                        resolve_meta_chirho,
                    },
                    body_chirho,
                    named_chirho,
                    bound_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?),
            }
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
            let mut inserted_chirho = Vec::new();
            let result_chirho = (|| {
                let mut records_chirho = Vec::new();
                for record_chirho in vars_chirho.records_chirho() {
                    let mut mapped_chirho = record_chirho.clone();
                    let mapped_kind_chirho = match &record_chirho.kind_chirho {
                        Some(kind_chirho) => Some(std::sync::Arc::new(transform_chirho(
                            EqualityTransformPolicyChirho {
                                ctx_chirho,
                                rewrites_chirho,
                                expand_chirho,
                                resolve_meta_chirho,
                            },
                            kind_chirho,
                            named_chirho,
                            bound_chirho,
                            budget_chirho,
                            depth_chirho + 1,
                        )?)),
                        None => None,
                    };
                    mapped_chirho.kind_chirho = mapped_kind_chirho;
                    records_chirho.push(mapped_chirho);
                    if bound_chirho.insert(record_chirho.variable_chirho) {
                        inserted_chirho.push(record_chirho.variable_chirho);
                    }
                }
                Some((
                    crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(
                        records_chirho,
                    ),
                    transform_chirho(
                        EqualityTransformPolicyChirho {
                            ctx_chirho,
                            rewrites_chirho,
                            expand_chirho,
                            resolve_meta_chirho,
                        },
                        body_chirho,
                        named_chirho,
                        bound_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )?,
                ))
            })();
            for var_chirho in inserted_chirho {
                bound_chirho.remove(&var_chirho);
            }
            let (vars_chirho, body_chirho) = result_chirho?;
            if matches!(ty_chirho, TyChirho::ForallChirho { .. }) {
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            } else {
                TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            }
        }
    })
}

pub(super) fn rewrite_givens_with_budget_chirho(
    ctx_chirho: &InferCtxChirho,
    ty_chirho: &TyChirho,
    budget_chirho: &mut EqualityBudgetChirho,
) -> Option<TyChirho> {
    transform_chirho(
        EqualityTransformPolicyChirho {
            ctx_chirho,
            rewrites_chirho: &ctx_chirho.given_rewrites_chirho,
            expand_chirho: false,
            resolve_meta_chirho: false,
        },
        ty_chirho,
        None,
        &mut HashSet::new(),
        budget_chirho,
        0,
    )
}

pub(super) fn normalize_owned_equality_chirho<'evidence_chirho>(
    ctx_chirho: &dyn NormalizationContextChirho,
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
    ctx_chirho: &dyn NormalizationContextChirho,
    ty_chirho: &TyChirho,
    evidence_chirho: impl Iterator<Item = &'evidence_chirho [(TyChirho, TyChirho)]>,
    budget_chirho: &mut EqualityBudgetChirho,
) -> Result<TyChirho, EqualityNormalizationExhaustedChirho> {
    let rewrites_chirho =
        prepare_owned_equality_rewrites_chirho(ctx_chirho, evidence_chirho, budget_chirho)?;
    normalize_prepared_owned_equality_chirho(ctx_chirho, ty_chirho, &rewrites_chirho, budget_chirho)
}

/// Prepare immutable premises once when a scratch solver checks several goals
/// under the same original evidence and substitution snapshot.
pub(super) fn prepare_owned_equality_rewrites_chirho<'evidence_chirho>(
    ctx_chirho: &dyn NormalizationContextChirho,
    evidence_chirho: impl Iterator<Item = &'evidence_chirho [(TyChirho, TyChirho)]>,
    budget_chirho: &mut EqualityBudgetChirho,
) -> Result<Vec<(TyChirho, TyChirho)>, EqualityNormalizationExhaustedChirho> {
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
                EqualityTransformPolicyChirho {
                    ctx_chirho,
                    rewrites_chirho: &[],
                    expand_chirho: false,
                    resolve_meta_chirho: true,
                },
                lhs_chirho,
                None,
                &mut HashSet::new(),
                budget_chirho,
                0,
            )
            .ok_or(EqualityNormalizationExhaustedChirho)?;
            let rhs_chirho = transform_chirho(
                EqualityTransformPolicyChirho {
                    ctx_chirho,
                    rewrites_chirho: &[],
                    expand_chirho: false,
                    resolve_meta_chirho: true,
                },
                rhs_chirho,
                None,
                &mut HashSet::new(),
                budget_chirho,
                0,
            )
            .ok_or(EqualityNormalizationExhaustedChirho)?;
            copied_chirho.push((lhs_chirho, rhs_chirho));
        }
        chunks_chirho.push(copied_chirho);
    }
    Ok(chunks_chirho.into_iter().rev().flatten().collect())
}

/// Use the same normalization algorithm with already prepared immutable
/// premises. The caller must preserve their substitution/evidence snapshot.
pub(super) fn normalize_prepared_owned_equality_chirho(
    ctx_chirho: &dyn NormalizationContextChirho,
    ty_chirho: &TyChirho,
    rewrites_chirho: &[(TyChirho, TyChirho)],
    budget_chirho: &mut EqualityBudgetChirho,
) -> Result<TyChirho, EqualityNormalizationExhaustedChirho> {
    let mut current_chirho = transform_chirho(
        EqualityTransformPolicyChirho {
            ctx_chirho,
            rewrites_chirho: &[],
            expand_chirho: false,
            resolve_meta_chirho: true,
        },
        ty_chirho,
        None,
        &mut HashSet::new(),
        budget_chirho,
        0,
    )
    .ok_or(EqualityNormalizationExhaustedChirho)?;
    for _iteration_chirho in 0..100 {
        let transformed_chirho = transform_chirho(
            EqualityTransformPolicyChirho {
                ctx_chirho,
                rewrites_chirho,
                expand_chirho: true,
                resolve_meta_chirho: true,
            },
            &current_chirho,
            None,
            &mut HashSet::new(),
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

/// Borrowed preflight precedes the clone; all terms in one predicate/context
/// share the caller's allowance, including names, binders and hidden arguments.
pub(super) fn copy_term_with_budget_chirho(
    term_chirho: &TyChirho,
    budget_chirho: &mut EqualityBudgetChirho,
) -> Option<TyChirho> {
    let mut pending_chirho = vec![(term_chirho, 0usize)];
    while let Some((term_chirho, depth_chirho)) = pending_chirho.pop() {
        budget_chirho.node_chirho(1)?;
        if depth_chirho > 128 {
            return None;
        }
        match term_chirho {
            TyChirho::VarChirho(_) => {}
            TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
                budget_chirho.node_chirho(name_chirho.len())?;
            }
            TyChirho::AppChirho(left_chirho, right_chirho)
            | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
                pending_chirho.push((left_chirho, depth_chirho + 1));
                pending_chirho.push((right_chirho, depth_chirho + 1));
            }
            TyChirho::TupleChirho(elements_chirho) => {
                budget_chirho.work_chirho(elements_chirho.len())?;
                pending_chirho.extend(
                    elements_chirho
                        .iter()
                        .map(|element_chirho| (element_chirho, depth_chirho + 1)),
                );
            }
            TyChirho::ListChirho(element_chirho) => {
                pending_chirho.push((element_chirho, depth_chirho + 1))
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
                for record_chirho in vars_chirho.records_chirho() {
                    if let Some(kind_chirho) = &record_chirho.kind_chirho {
                        pending_chirho.push((kind_chirho.as_ref(), depth_chirho + 1));
                    }
                }
                pending_chirho.push((body_chirho, depth_chirho + 1));
            }
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                budget_chirho.work_chirho(context_chirho.len())?;
                pending_chirho.push((body_chirho, depth_chirho + 1));
                pending_chirho.extend(
                    context_chirho
                        .iter()
                        .map(|constraint_chirho| (constraint_chirho, depth_chirho + 1)),
                );
            }
        }
    }
    Some(term_chirho.clone())
}

/// Fused trial substitution charges each expanded image BEFORE copying it.
/// This is simultaneous substitution and respects both forall visibilities.
pub(super) fn substitute_meta_with_budget_chirho(
    term_chirho: &TyChirho,
    variable_chirho: TyVarChirho,
    image_chirho: &TyChirho,
    budget_chirho: &mut EqualityBudgetChirho,
) -> Option<TyChirho> {
    fn walk_chirho(
        term_chirho: &TyChirho,
        replacement_chirho: Option<(TyVarChirho, &TyChirho)>,
        budget_chirho: &mut EqualityBudgetChirho,
        depth_chirho: usize,
    ) -> Option<TyChirho> {
        budget_chirho.node_chirho(1)?;
        if depth_chirho > 128 {
            return None;
        }
        Some(match term_chirho {
            TyChirho::VarChirho(variable_chirho) => {
                if let Some((_target_chirho, image_chirho)) =
                    replacement_chirho.filter(|(target_chirho, _)| target_chirho == variable_chirho)
                {
                    return walk_chirho(image_chirho, None, budget_chirho, depth_chirho + 1);
                }
                TyChirho::VarChirho(*variable_chirho)
            }
            TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
                budget_chirho.node_chirho(name_chirho.len())?;
                if matches!(term_chirho, TyChirho::ConChirho(_)) {
                    TyChirho::ConChirho(name_chirho.clone())
                } else {
                    TyChirho::ForallVarChirho(name_chirho.clone())
                }
            }
            TyChirho::AppChirho(left_chirho, right_chirho) => TyChirho::AppChirho(
                Box::new(walk_chirho(
                    left_chirho,
                    replacement_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?),
                Box::new(walk_chirho(
                    right_chirho,
                    replacement_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?),
            ),
            TyChirho::FunChirho(left_chirho, right_chirho, multiplicity_chirho) => {
                TyChirho::FunChirho(
                    Box::new(walk_chirho(
                        left_chirho,
                        replacement_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )?),
                    Box::new(walk_chirho(
                        right_chirho,
                        replacement_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )?),
                    *multiplicity_chirho,
                )
            }
            TyChirho::ListChirho(element_chirho) => TyChirho::ListChirho(Box::new(walk_chirho(
                element_chirho,
                replacement_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?)),
            TyChirho::TupleChirho(elements_chirho) => {
                budget_chirho.work_chirho(elements_chirho.len())?;
                TyChirho::TupleChirho(
                    elements_chirho
                        .iter()
                        .map(|element_chirho| {
                            walk_chirho(
                                element_chirho,
                                replacement_chirho,
                                budget_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Option<Vec<_>>>()?,
                )
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
                                constraint_chirho,
                                replacement_chirho,
                                budget_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Option<Vec<_>>>()?,
                    body_chirho: Box::new(walk_chirho(
                        body_chirho,
                        replacement_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )?),
                }
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
                let mut replacement_chirho = replacement_chirho;
                let mut records_chirho = Vec::new();
                for record_chirho in vars_chirho.records_chirho() {
                    let mut mapped_chirho = record_chirho.clone();
                    mapped_chirho.kind_chirho = match &record_chirho.kind_chirho {
                        Some(kind_chirho) => Some(std::sync::Arc::new(walk_chirho(
                            kind_chirho,
                            replacement_chirho,
                            budget_chirho,
                            depth_chirho + 1,
                        )?)),
                        None => None,
                    };
                    records_chirho.push(mapped_chirho);
                    replacement_chirho = replacement_chirho.filter(|(variable_chirho, _)| {
                        *variable_chirho != record_chirho.variable_chirho
                    });
                }
                let vars_chirho = crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(
                    records_chirho,
                );
                let body_chirho = Box::new(walk_chirho(
                    body_chirho,
                    replacement_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?);
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
        })
    }
    walk_chirho(
        term_chirho,
        Some((variable_chirho, image_chirho)),
        budget_chirho,
        0,
    )
}
