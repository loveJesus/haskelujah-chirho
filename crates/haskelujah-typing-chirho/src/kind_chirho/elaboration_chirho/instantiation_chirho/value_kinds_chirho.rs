// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Project checked term kinds from the same catalogue and lexical images.
//! This reads full telescopes; an opaque term has no positive kind proof.

use super::*;
use crate::ty_chirho::{QuantifiedVariableChirho, QuantifiedVariablesChirho};
use haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho;

#[cfg(test)]
#[path = "value_kinds_tests_chirho.rs"]
mod tests_chirho;

impl TypeElaboratorChirho<'_> {
    pub(in crate::kind_chirho::elaboration_chirho) fn checked_term_kind_chirho(
        &mut self,
        term_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<Option<TyChirho>, ElaborationErrorChirho> {
        let term_chirho = self
            .state_chirho
            .instantiate_lexical_term_chirho(term_chirho, span_chirho)?;
        let term_chirho = self.normalize_value_kind_term_chirho(&term_chirho, span_chirho)?;
        self.project_term_kind_chirho(&term_chirho, &mut Vec::new(), span_chirho, 0)
    }

    pub(in crate::kind_chirho::elaboration_chirho) fn normalize_value_kind_term_chirho(
        &mut self,
        term_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let term_chirho = self.normalize_lexical_term_chirho(term_chirho, span_chirho)?;
        let Some(view_chirho) = self.lexical_equality_chirho else {
            return Ok(term_chirho);
        };
        let mut budget_chirho = self.state_chirho.lend_equality_budget_chirho(span_chirho)?;
        let initial_chirho = budget_chirho.remaining_chirho();
        let normalized_chirho = view_chirho.normalize_chirho(
            &self.registry_chirho,
            self.state_chirho.owner_chirho.is_some(),
            self.state_chirho.next_variable_chirho,
            &term_chirho,
            &mut budget_chirho,
        );
        self.state_chirho.settle_equality_budget_chirho(
            initial_chirho,
            &budget_chirho,
            normalized_chirho.is_err(),
            span_chirho,
        )?;
        normalized_chirho.map_err(|_| {
            self.state_chirho.resource_failure_chirho(
                "value parameter kind normalization bound exhausted",
                span_chirho,
            )
        })
    }

    fn project_term_kind_chirho(
        &mut self,
        term_chirho: &TyChirho,
        bound_chirho: &mut Vec<QuantifiedVariableChirho>,
        span_chirho: SpanChirho,
        depth_chirho: usize,
    ) -> Result<Option<TyChirho>, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        if depth_chirho >= 128 {
            return Err(self.state_chirho.resource_failure_chirho(
                "value parameter kind projection depth bound exhausted",
                span_chirho,
            ));
        }
        match term_chirho {
            TyChirho::VarChirho(variable_chirho) => {
                self.state_chirho
                    .charge_chirho(bound_chirho.len(), 0, span_chirho)?;
                let local_chirho = bound_chirho
                    .iter()
                    .rev()
                    .find(|record_chirho| record_chirho.variable_chirho == *variable_chirho);
                let kind_chirho = match local_chirho {
                    Some(record_chirho) => record_chirho.kind_chirho.as_ref(),
                    None => self
                        .state_chirho
                        .variable_kinds_chirho
                        .get(variable_chirho)
                        .and_then(|record_chirho| record_chirho.kind_chirho.as_ref()),
                }
                .cloned();
                kind_chirho
                    .map(|kind_chirho| {
                        self.normalize_value_kind_term_chirho(&kind_chirho, span_chirho)
                    })
                    .transpose()
            }
            TyChirho::ForallVarChirho(_) => {
                let Some(images_chirho) = self.lexical_images_chirho else {
                    return Ok(None);
                };
                self.state_chirho
                    .charge_chirho(images_chirho.len_chirho(), 0, span_chirho)?;
                let mut checked_chirho = None;
                for (variable_chirho, image_chirho) in images_chirho.iter_chirho() {
                    let equal_chirho = crate::ty_chirho::same_type_contract_chirho(
                        image_chirho,
                        term_chirho,
                        &mut |weight_chirho| {
                            self.state_chirho
                                .charge_chirho(weight_chirho, 0, span_chirho)
                                .ok()
                        },
                    )
                    .ok_or_else(|| {
                        self.state_chirho.resource_failure_chirho(
                            "value parameter rigid-image comparison bound exhausted",
                            span_chirho,
                        )
                    })?;
                    if equal_chirho {
                        let Some(kind_chirho) = images_chirho
                            .lookup_image_chirho(variable_chirho)
                            .and_then(|image_chirho| image_chirho.kind_chirho.as_ref())
                        else {
                            continue;
                        };
                        let kind_chirho =
                            self.normalize_value_kind_term_chirho(kind_chirho, span_chirho)?;
                        if let Some(previous_chirho) = &checked_chirho {
                            let agrees_chirho = crate::ty_chirho::same_type_contract_chirho(
                                previous_chirho,
                                &kind_chirho,
                                &mut |weight_chirho| {
                                    self.state_chirho
                                        .charge_chirho(weight_chirho, 0, span_chirho)
                                        .ok()
                                },
                            )
                            .ok_or_else(|| {
                                self.state_chirho.resource_failure_chirho(
                                    "value parameter image-kind comparison bound exhausted",
                                    span_chirho,
                                )
                            })?;
                            if !agrees_chirho {
                                return Err(Self::error_chirho(
                                    "rigid value parameter has conflicting checked image kinds",
                                    span_chirho,
                                ));
                            }
                        } else {
                            checked_chirho = Some(kind_chirho);
                        }
                    }
                }
                Ok(checked_chirho)
            }
            TyChirho::ConChirho(key_chirho) => {
                self.project_declaration_kind_chirho(key_chirho, &[], span_chirho)
            }
            TyChirho::AppChirho(_, _) => {
                let mut arguments_chirho = Vec::new();
                let mut head_chirho = term_chirho;
                while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
                    self.state_chirho.charge_chirho(1, 1, span_chirho)?;
                    arguments_chirho.push(argument_chirho.as_ref());
                    head_chirho = function_chirho;
                }
                arguments_chirho.reverse();
                if let TyChirho::ConChirho(key_chirho) = head_chirho {
                    return self.project_declaration_kind_chirho(
                        key_chirho,
                        &arguments_chirho,
                        span_chirho,
                    );
                }
                let Some(mut kind_chirho) = self.project_term_kind_chirho(
                    head_chirho,
                    bound_chirho,
                    span_chirho,
                    depth_chirho + 1,
                )?
                else {
                    return Ok(None);
                };
                for argument_chirho in arguments_chirho {
                    let Some(next_chirho) = self.apply_projected_kind_chirho(
                        kind_chirho,
                        argument_chirho,
                        span_chirho,
                    )?
                    else {
                        return Ok(None);
                    };
                    kind_chirho = next_chirho;
                }
                Ok(Some(kind_chirho))
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                self.state_chirho.charge_chirho(
                    vars_chirho.len(),
                    vars_chirho.len(),
                    span_chirho,
                )?;
                let mark_chirho = bound_chirho.len();
                bound_chirho.extend(vars_chirho.records_chirho().iter().cloned());
                let result_chirho = self.project_term_kind_chirho(
                    body_chirho,
                    bound_chirho,
                    span_chirho,
                    depth_chirho + 1,
                );
                bound_chirho.truncate(mark_chirho);
                result_chirho
            }
            TyChirho::TupleChirho(elements_chirho) => {
                if let Some(element_chirho) = elements_chirho.first() {
                    self.project_term_kind_chirho(
                        element_chirho,
                        bound_chirho,
                        span_chirho,
                        depth_chirho + 1,
                    )
                } else {
                    Ok(Some(ElaborationStateChirho::type_universe_chirho()))
                }
            }
            TyChirho::FunChirho(..) | TyChirho::ListChirho(_) | TyChirho::QualChirho { .. } => {
                Ok(Some(ElaborationStateChirho::type_universe_chirho()))
            }
        }
    }

    /// Interpret every declaration slot before a consumer substitution can
    /// see canonical producer IDs. Required slots are dependent parameters too.
    fn project_declaration_kind_chirho(
        &mut self,
        key_chirho: &str,
        arguments_chirho: &[&TyChirho],
        span_chirho: SpanChirho,
    ) -> Result<Option<TyChirho>, ElaborationErrorChirho> {
        let Some(scheme_chirho) = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(key_chirho)
            .cloned()
            .filter(|scheme_chirho| {
                !scheme_chirho.held_chirho
                    && (scheme_chirho.declaration_chirho.is_some()
                        || scheme_chirho.promoted_chirho.is_some())
            })
        else {
            return Ok(None);
        };
        if scheme_chirho
            .owner_chirho
            .is_some_and(|owner_chirho| Some(owner_chirho) != self.state_chirho.owner_chirho)
        {
            return Err(Self::error_chirho(
                "foreign declaration contains unquantified kind variables",
                span_chirho,
            ));
        }
        if scheme_chirho.declaration_chirho.as_ref().is_some_and(|proof_chirho| {
            matches!(&proof_chirho.identity_chirho,
                haskelujah_ast_chirho::name_chirho::TypeConstructorIdentityChirho::CompilerChirho(constructor_chirho)
                if constructor_chirho.scalar_representation_chirho().is_some())
        }) {
            // Preserve Unknown when only the legacy primitive key survived.
            // The source consumer must retain its actual ElaboratedType kind.
            return Ok(None);
        }
        if arguments_chirho.len() >= scheme_chirho.binders_chirho.len() {
            // A checked full application supplies every slot, including hidden
            // kinds. Interpret those slots simultaneously, without freshening
            // an unused synonym RHS or visiting unrelated catalogue entries.
            self.state_chirho.charge_chirho(
                arguments_chirho.len(),
                arguments_chirho.len(),
                span_chirho,
            )?;
            let mut mapping_chirho = SubstChirho::empty_chirho();
            for (binder_chirho, argument_chirho) in
                scheme_chirho.binders_chirho.iter().zip(arguments_chirho)
            {
                mapping_chirho.insert_chirho(
                    binder_chirho.variable_chirho,
                    self.state_chirho
                        .copy_chirho(argument_chirho, span_chirho)?,
                );
            }
            let mut kind_chirho = self
                .state_chirho
                .instantiate_checked_declaration_term_chirho(
                    &scheme_chirho.result_kind_chirho,
                    &mapping_chirho,
                    scheme_chirho.owner_chirho.is_some(),
                    &scheme_chirho.local_binders_chirho,
                    span_chirho,
                )?;
            for argument_chirho in &arguments_chirho[scheme_chirho.binders_chirho.len()..] {
                let Some(next_chirho) =
                    self.apply_projected_kind_chirho(kind_chirho, argument_chirho, span_chirho)?
                else {
                    return Ok(None);
                };
                kind_chirho = next_chirho;
            }
            return Ok(Some(kind_chirho));
        }
        let (binders_chirho, result_chirho, _) =
            self.instantiate_header_chirho(&scheme_chirho, span_chirho)?;
        self.state_chirho.charge_chirho(
            arguments_chirho.len(),
            arguments_chirho.len(),
            span_chirho,
        )?;
        let consumed_chirho = arguments_chirho.len().min(binders_chirho.len());
        let mut mapping_chirho = SubstChirho::empty_chirho();
        for (binder_chirho, argument_chirho) in binders_chirho.iter().zip(arguments_chirho) {
            let argument_chirho = self
                .state_chirho
                .copy_chirho(argument_chirho, span_chirho)?;
            mapping_chirho.insert_chirho(binder_chirho.variable_chirho, argument_chirho);
        }
        let mut kind_chirho = self
            .state_chirho
            .instantiate_checked_declaration_term_chirho(
                &result_chirho,
                &mapping_chirho,
                true,
                &[],
                span_chirho,
            )?;
        for binder_chirho in binders_chirho[consumed_chirho..].iter().rev() {
            let binder_kind_chirho = self
                .state_chirho
                .instantiate_checked_declaration_term_chirho(
                    &binder_chirho.kind_chirho,
                    &mapping_chirho,
                    true,
                    &[],
                    span_chirho,
                )?;
            kind_chirho =
                if binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho {
                    TyChirho::fun_chirho(binder_kind_chirho, kind_chirho)
                } else {
                    let specificity_chirho = if binder_chirho.visibility_chirho
                        == TelescopeVisibilityChirho::InferredChirho
                    {
                        TyVarSpecificityChirho::InferredChirho
                    } else {
                        TyVarSpecificityChirho::SpecifiedChirho
                    };
                    TyChirho::ForallChirho {
                        vars_chirho: QuantifiedVariablesChirho::from_records_chirho(vec![
                            QuantifiedVariableChirho {
                                variable_chirho: binder_chirho.variable_chirho,
                                kind_chirho: Some(Arc::new(binder_kind_chirho)),
                                source_chirho: None,
                                specificity_chirho: Some(specificity_chirho),
                            },
                        ]),
                        body_chirho: Box::new(kind_chirho),
                    }
                };
        }
        for argument_chirho in &arguments_chirho[consumed_chirho..] {
            let Some(next_chirho) =
                self.apply_projected_kind_chirho(kind_chirho, argument_chirho, span_chirho)?
            else {
                return Ok(None);
            };
            kind_chirho = next_chirho;
        }
        Ok(Some(kind_chirho))
    }

    fn apply_projected_kind_chirho(
        &mut self,
        kind_chirho: TyChirho,
        argument_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<Option<TyChirho>, ElaborationErrorChirho> {
        let kind_chirho = self.normalize_value_kind_term_chirho(&kind_chirho, span_chirho)?;
        let required_chirho = matches!(&kind_chirho, TyChirho::RequiredForallChirho { .. });
        match kind_chirho {
            TyChirho::FunChirho(_, result_chirho, _) => Ok(Some(*result_chirho)),
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                let Some(variable_chirho) = vars_chirho.first() else {
                    return Ok(None);
                };
                let body_chirho = if vars_chirho.len() == 1 {
                    *body_chirho
                } else if required_chirho {
                    TyChirho::RequiredForallChirho {
                        vars_chirho: vars_chirho.slice_chirho(1..vars_chirho.len()),
                        body_chirho,
                    }
                } else {
                    TyChirho::ForallChirho {
                        vars_chirho: vars_chirho.slice_chirho(1..vars_chirho.len()),
                        body_chirho,
                    }
                };
                let argument_chirho = self
                    .state_chirho
                    .copy_chirho(argument_chirho, span_chirho)?;
                let mapping_chirho =
                    SubstChirho::singleton_chirho(*variable_chirho, argument_chirho);
                self.state_chirho
                    .instantiate_checked_declaration_term_chirho(
                        &body_chirho,
                        &mapping_chirho,
                        true,
                        &[],
                        span_chirho,
                    )
                    .map(Some)
            }
            _ => Ok(None),
        }
    }
}
