// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Equality evidence, substitution and scoped synonym transformation.

use super::*;

pub(super) struct NamedEqualityBindingsChirho<'source_chirho> {
    parent_chirho: Option<&'source_chirho NamedEqualityBindingsChirho<'source_chirho>>,
    bindings_chirho: Vec<(&'source_chirho str, &'source_chirho TyChirho)>,
}

#[derive(Clone, Copy)]
pub(super) struct EqualityTransformPolicyChirho<'source_chirho> {
    pub(super) ctx_chirho: &'source_chirho dyn NormalizationContextChirho,
    pub(super) rewrites_chirho: &'source_chirho [(TyChirho, TyChirho)],
    pub(super) expand_chirho: bool,
    pub(super) resolve_meta_chirho: bool,
}

pub(super) fn transform_chirho<'source_chirho>(
    policy_chirho: EqualityTransformPolicyChirho<'source_chirho>,
    ty_chirho: &'source_chirho TyChirho,
    named_chirho: Option<&'source_chirho NamedEqualityBindingsChirho<'source_chirho>>,
    bound_chirho: &mut HashSet<TyVarChirho>,
    budget_chirho: &mut EqualityBudgetChirho,
    depth_chirho: usize,
) -> Option<NormalizationChangeChirho> {
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
                    let image_chirho = transform_chirho(
                        policy_chirho,
                        value_chirho,
                        current_chirho.parent_chirho,
                        bound_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )?
                    .materialize_chirho(value_chirho, budget_chirho)?;
                    return Some(NormalizationChangeChirho::RebuiltChirho(image_chirho));
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
            // A selected replacement is a different input, even if equality
            // ignores some metadata. Materialize its actual selected image.
            let image_chirho = transform_chirho(
                EqualityTransformPolicyChirho {
                    rewrites_chirho: &[],
                    ..policy_chirho
                },
                rhs_chirho,
                None,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?
            .materialize_chirho(rhs_chirho, budget_chirho)?;
            return Some(NormalizationChangeChirho::RebuiltChirho(image_chirho));
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
        let image_chirho = transform_chirho(
            policy_chirho,
            image_chirho,
            None,
            bound_chirho,
            budget_chirho,
            depth_chirho + 1,
        )?
        .materialize_chirho(image_chirho, budget_chirho)?;
        return Some(NormalizationChangeChirho::RebuiltChirho(image_chirho));
    }
    if expand_chirho
        && named_chirho.is_none()
        && (ctx_chirho.owner_present_chirho()
            || !ctx_chirho
                .catalogue_chirho()
                .declaration_schemes_chirho()
                .is_empty_chirho())
    {
        let expanded_chirho =
            super::super::synonym_expansion_chirho::expand_synonyms_change_with_budget_chirho(
                ctx_chirho,
                ty_chirho,
                budget_chirho,
            )?;
        let mapped_chirho = transform_chirho(
            EqualityTransformPolicyChirho {
                expand_chirho: false,
                ..policy_chirho
            },
            expanded_chirho.image_chirho(ty_chirho),
            None,
            bound_chirho,
            budget_chirho,
            depth_chirho + 1,
        )?;
        return Some(expanded_chirho.then_chirho(mapped_chirho));
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
                policy_chirho,
                rhs_chirho,
                Some(&frame_chirho),
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?
            .materialize_chirho(rhs_chirho, budget_chirho)?;
            for argument_chirho in &arguments_chirho[parameters_chirho.len()..] {
                let argument_chirho = transform_chirho(
                    policy_chirho,
                    argument_chirho,
                    named_chirho,
                    bound_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?
                .materialize_chirho(argument_chirho, budget_chirho)?;
                budget_chirho.node_chirho(1)?;
                expanded_chirho = if let Some(key_chirho) =
                    crate::ty_chirho::representation_chirho::folded_universe_key_chirho(
                        &expanded_chirho,
                        &argument_chirho,
                    ) {
                    budget_chirho.node_chirho(key_chirho.len())?;
                    TyChirho::ConChirho(key_chirho.to_owned())
                } else {
                    TyChirho::AppChirho(Box::new(expanded_chirho), Box::new(argument_chirho))
                };
            }
            return Some(NormalizationChangeChirho::RebuiltChirho(expanded_chirho));
        }
    }
    match ty_chirho {
        TyChirho::VarChirho(_) => Some(NormalizationChangeChirho::UnchangedChirho),
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            budget_chirho.work_chirho(name_chirho.len())?;
            Some(NormalizationChangeChirho::UnchangedChirho)
        }
        TyChirho::AppChirho(fun_chirho, arg_chirho) => {
            let fun_image_chirho = transform_chirho(
                policy_chirho,
                fun_chirho,
                named_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?;
            let arg_image_chirho = transform_chirho(
                policy_chirho,
                arg_chirho,
                named_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?;
            if let Some(key_chirho) =
                crate::ty_chirho::representation_chirho::folded_universe_key_chirho(
                    fun_image_chirho.image_chirho(fun_chirho),
                    arg_image_chirho.image_chirho(arg_chirho),
                )
            {
                budget_chirho.node_chirho(key_chirho.len().saturating_add(1))?;
                return Some(NormalizationChangeChirho::RebuiltChirho(
                    TyChirho::ConChirho(key_chirho.to_owned()),
                ));
            }
            if fun_image_chirho.is_unchanged_chirho() && arg_image_chirho.is_unchanged_chirho() {
                return Some(NormalizationChangeChirho::UnchangedChirho);
            }
            budget_chirho.node_chirho(1)?;
            Some(NormalizationChangeChirho::RebuiltChirho(
                TyChirho::AppChirho(
                    Box::new(fun_image_chirho.materialize_chirho(fun_chirho, budget_chirho)?),
                    Box::new(arg_image_chirho.materialize_chirho(arg_chirho, budget_chirho)?),
                ),
            ))
        }
        TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => {
            let arg_image_chirho = transform_chirho(
                policy_chirho,
                arg_chirho,
                named_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?;
            let result_image_chirho = transform_chirho(
                policy_chirho,
                result_chirho,
                named_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?;
            if arg_image_chirho.is_unchanged_chirho() && result_image_chirho.is_unchanged_chirho() {
                return Some(NormalizationChangeChirho::UnchangedChirho);
            }
            budget_chirho.node_chirho(1)?;
            Some(NormalizationChangeChirho::RebuiltChirho(
                TyChirho::FunChirho(
                    Box::new(arg_image_chirho.materialize_chirho(arg_chirho, budget_chirho)?),
                    Box::new(result_image_chirho.materialize_chirho(result_chirho, budget_chirho)?),
                    *mult_chirho,
                ),
            ))
        }
        TyChirho::TupleChirho(elements_chirho) => {
            let mut rebuilt_chirho = None;
            for (index_chirho, element_chirho) in elements_chirho.iter().enumerate() {
                transform_chirho(
                    policy_chirho,
                    element_chirho,
                    named_chirho,
                    bound_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?
                .append_chirho(
                    elements_chirho,
                    index_chirho,
                    &mut rebuilt_chirho,
                    budget_chirho,
                )?;
            }
            match rebuilt_chirho {
                None => Some(NormalizationChangeChirho::UnchangedChirho),
                Some(elements_chirho) => {
                    budget_chirho.node_chirho(1)?;
                    Some(NormalizationChangeChirho::RebuiltChirho(
                        TyChirho::TupleChirho(elements_chirho),
                    ))
                }
            }
        }
        TyChirho::ListChirho(element_chirho) => {
            let image_chirho = transform_chirho(
                policy_chirho,
                element_chirho,
                named_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?;
            if image_chirho.is_unchanged_chirho() {
                return Some(NormalizationChangeChirho::UnchangedChirho);
            }
            budget_chirho.node_chirho(1)?;
            Some(NormalizationChangeChirho::RebuiltChirho(
                TyChirho::ListChirho(Box::new(
                    image_chirho.materialize_chirho(element_chirho, budget_chirho)?,
                )),
            ))
        }
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => {
            budget_chirho.work_chirho(context_chirho.len())?;
            let mut context_image_chirho = None;
            for (index_chirho, constraint_chirho) in context_chirho.iter().enumerate() {
                transform_chirho(
                    policy_chirho,
                    constraint_chirho,
                    named_chirho,
                    bound_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?
                .append_chirho(
                    context_chirho,
                    index_chirho,
                    &mut context_image_chirho,
                    budget_chirho,
                )?;
            }
            let body_image_chirho = transform_chirho(
                policy_chirho,
                body_chirho,
                named_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?;
            if context_image_chirho.is_none() && body_image_chirho.is_unchanged_chirho() {
                return Some(NormalizationChangeChirho::UnchangedChirho);
            }
            budget_chirho.node_chirho(1)?;
            let context_image_chirho = match context_image_chirho {
                Some(context_chirho) => context_chirho,
                None => context_chirho
                    .iter()
                    .map(|constraint_chirho| {
                        copy_term_with_budget_chirho(constraint_chirho, budget_chirho)
                    })
                    .collect::<Option<Vec<_>>>()?,
            };
            Some(NormalizationChangeChirho::RebuiltChirho(
                TyChirho::QualChirho {
                    context_chirho: context_image_chirho,
                    body_chirho: Box::new(
                        body_image_chirho.materialize_chirho(body_chirho, budget_chirho)?,
                    ),
                },
            ))
        }
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            budget_chirho.work_chirho(vars_chirho.len())?;
            let mut inserted_chirho = Vec::new();
            let result_chirho = (|| {
                let mut records_chirho = None::<Vec<crate::ty_chirho::QuantifiedVariableChirho>>;
                for (index_chirho, record_chirho) in vars_chirho.records_chirho().iter().enumerate()
                {
                    let kind_image_chirho = match &record_chirho.kind_chirho {
                        Some(kind_chirho) => transform_chirho(
                            policy_chirho,
                            kind_chirho,
                            named_chirho,
                            bound_chirho,
                            budget_chirho,
                            depth_chirho + 1,
                        )?,
                        None => NormalizationChangeChirho::UnchangedChirho,
                    };
                    if !kind_image_chirho.is_unchanged_chirho() && records_chirho.is_none() {
                        budget_chirho.node_chirho(vars_chirho.len())?;
                        records_chirho =
                            Some(vars_chirho.records_chirho()[..index_chirho].to_vec());
                    }
                    if let Some(records_chirho) = &mut records_chirho {
                        let mut mapped_chirho = record_chirho.clone();
                        if let NormalizationChangeChirho::RebuiltChirho(kind_chirho) =
                            kind_image_chirho
                        {
                            mapped_chirho.kind_chirho = Some(std::sync::Arc::new(kind_chirho));
                        }
                        records_chirho.push(mapped_chirho);
                    }
                    if bound_chirho.insert(record_chirho.variable_chirho) {
                        inserted_chirho.push(record_chirho.variable_chirho);
                    }
                }
                let body_image_chirho = transform_chirho(
                    policy_chirho,
                    body_chirho,
                    named_chirho,
                    bound_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?;
                if records_chirho.is_none() && body_image_chirho.is_unchanged_chirho() {
                    return Some(NormalizationChangeChirho::UnchangedChirho);
                }
                budget_chirho.node_chirho(1)?;
                let variables_chirho = records_chirho
                    .map(crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho)
                    .unwrap_or_else(|| vars_chirho.clone());
                let body_chirho =
                    Box::new(body_image_chirho.materialize_chirho(body_chirho, budget_chirho)?);
                Some(NormalizationChangeChirho::RebuiltChirho(
                    if matches!(ty_chirho, TyChirho::ForallChirho { .. }) {
                        TyChirho::ForallChirho {
                            vars_chirho: variables_chirho,
                            body_chirho,
                        }
                    } else {
                        TyChirho::RequiredForallChirho {
                            vars_chirho: variables_chirho,
                            body_chirho,
                        }
                    },
                ))
            })();
            for variable_chirho in inserted_chirho {
                bound_chirho.remove(&variable_chirho);
            }
            result_chirho
        }
    }
}
