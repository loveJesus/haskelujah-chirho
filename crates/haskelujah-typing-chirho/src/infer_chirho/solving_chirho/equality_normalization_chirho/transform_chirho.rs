// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Equality evidence, substitution and scoped synonym transformation.

use super::*;

struct NamedEqualityBindingsChirho<'source_chirho> {
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
                let argument_chirho = transform_chirho(
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
                )?;
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
        TyChirho::AppChirho(fun_chirho, arg_chirho) => {
            let fun_chirho = transform_chirho(
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
            )?;
            let arg_chirho = transform_chirho(
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
            )?;
            if let Some(key_chirho) =
                crate::ty_chirho::representation_chirho::folded_universe_key_chirho(
                    &fun_chirho,
                    &arg_chirho,
                )
            {
                budget_chirho.node_chirho(key_chirho.len())?;
                TyChirho::ConChirho(key_chirho.to_owned())
            } else {
                TyChirho::AppChirho(Box::new(fun_chirho), Box::new(arg_chirho))
            }
        }
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
