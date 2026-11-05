// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Capture-safe instantiation at the checked declaration boundary. Producer
//! slots are interpreted before the consumer's substitution can see their IDs.

use super::*;
use crate::subst_chirho::SubstChirho;

impl ElaborationStateChirho {
    pub(super) fn instantiate_term_chirho(
        &mut self,
        term_chirho: &TyChirho,
        parameters_chirho: &SubstChirho,
        local_chirho: bool,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        self.instantiate_depth_chirho(
            term_chirho,
            parameters_chirho,
            local_chirho,
            span_chirho,
            0,
            &mut Vec::new(),
        )
    }

    fn instantiate_depth_chirho(
        &mut self,
        term_chirho: &TyChirho,
        parameters_chirho: &SubstChirho,
        local_chirho: bool,
        span_chirho: SpanChirho,
        depth_chirho: usize,
        bound_chirho: &mut Vec<(TyVarChirho, TyVarChirho)>,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        self.charge_chirho(1, 1, span_chirho)?;
        if depth_chirho > 128 {
            return Err(TypeElaboratorChirho::error_chirho(
                "declaration instantiation depth bound exhausted",
                span_chirho,
            ));
        }
        Ok(match term_chirho {
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                self.charge_chirho(context_chirho.len(), context_chirho.len(), span_chirho)?;
                TyChirho::QualChirho {
                    context_chirho: context_chirho
                        .iter()
                        .map(|constraint_chirho| {
                            self.instantiate_depth_chirho(
                                constraint_chirho,
                                parameters_chirho,
                                local_chirho,
                                span_chirho,
                                depth_chirho + 1,
                                bound_chirho,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                    body_chirho: Box::new(self.instantiate_depth_chirho(
                        body_chirho,
                        parameters_chirho,
                        local_chirho,
                        span_chirho,
                        depth_chirho + 1,
                        bound_chirho,
                    )?),
                }
            }
            TyChirho::VarChirho(variable_chirho) => {
                if let Some((_, renamed_chirho)) = bound_chirho
                    .iter()
                    .rev()
                    .find(|(source_chirho, _)| source_chirho == variable_chirho)
                {
                    TyChirho::VarChirho(*renamed_chirho)
                } else if let Some(image_chirho) = parameters_chirho.lookup_chirho(variable_chirho)
                {
                    // Simultaneous substitution: an image belongs to the consumer.
                    self.copy_chirho(image_chirho, span_chirho)?
                } else if local_chirho {
                    self.resolve_chirho(term_chirho, span_chirho)?
                } else {
                    return Err(TypeElaboratorChirho::error_chirho(
                        "published declaration contains an unbound type slot",
                        span_chirho,
                    ));
                }
            }
            TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {
                self.copy_chirho(term_chirho, span_chirho)?
            }
            TyChirho::AppChirho(left_chirho, right_chirho) => TyChirho::AppChirho(
                Box::new(self.instantiate_depth_chirho(
                    left_chirho,
                    parameters_chirho,
                    local_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    bound_chirho,
                )?),
                Box::new(self.instantiate_depth_chirho(
                    right_chirho,
                    parameters_chirho,
                    local_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    bound_chirho,
                )?),
            ),
            TyChirho::FunChirho(left_chirho, right_chirho, mult_chirho) => TyChirho::FunChirho(
                Box::new(self.instantiate_depth_chirho(
                    left_chirho,
                    parameters_chirho,
                    local_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    bound_chirho,
                )?),
                Box::new(self.instantiate_depth_chirho(
                    right_chirho,
                    parameters_chirho,
                    local_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    bound_chirho,
                )?),
                *mult_chirho,
            ),
            TyChirho::ListChirho(element_chirho) => {
                TyChirho::ListChirho(Box::new(self.instantiate_depth_chirho(
                    element_chirho,
                    parameters_chirho,
                    local_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    bound_chirho,
                )?))
            }
            TyChirho::TupleChirho(elements_chirho) => {
                self.charge_chirho(elements_chirho.len(), elements_chirho.len(), span_chirho)?;
                TyChirho::TupleChirho(
                    elements_chirho
                        .iter()
                        .map(|element_chirho| {
                            self.instantiate_depth_chirho(
                                element_chirho,
                                parameters_chirho,
                                local_chirho,
                                span_chirho,
                                depth_chirho + 1,
                                bound_chirho,
                            )
                        })
                        .collect::<Result<_, _>>()?,
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
                self.charge_chirho(vars_chirho.len(), vars_chirho.len(), span_chirho)?;
                let mark_chirho = bound_chirho.len();
                let mut fresh_chirho = Vec::with_capacity(vars_chirho.len());
                for variable_chirho in vars_chirho {
                    let renamed_chirho = self.fresh_chirho(None, span_chirho)?;
                    fresh_chirho.push(renamed_chirho);
                    bound_chirho.push((*variable_chirho, renamed_chirho));
                }
                let body_chirho = self.instantiate_depth_chirho(
                    body_chirho,
                    parameters_chirho,
                    local_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    bound_chirho,
                );
                bound_chirho.truncate(mark_chirho);
                let body_chirho = Box::new(body_chirho?);
                if matches!(term_chirho, TyChirho::RequiredForallChirho { .. }) {
                    TyChirho::RequiredForallChirho {
                        vars_chirho: fresh_chirho,
                        body_chirho,
                    }
                } else {
                    TyChirho::ForallChirho {
                        vars_chirho: fresh_chirho,
                        body_chirho,
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn published_nested_forall_cannot_capture_a_fresh_top_image_chirho() {
        for required_chirho in [false, true] {
            let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
            state_chirho.next_variable_chirho = 20;
            let top_chirho = state_chirho
                .fresh_chirho(None, SpanChirho::DUMMY_CHIRHO)
                .unwrap();
            let mut parameters_chirho = SubstChirho::empty_chirho();
            parameters_chirho.insert_chirho(TyVarChirho(0), TyChirho::VarChirho(top_chirho));
            let body_chirho = Box::new(TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(0)),
                TyChirho::VarChirho(TyVarChirho(20)),
            ));
            let term_chirho = if required_chirho {
                TyChirho::RequiredForallChirho {
                    vars_chirho: vec![TyVarChirho(20)],
                    body_chirho,
                }
            } else {
                TyChirho::ForallChirho {
                    vars_chirho: vec![TyVarChirho(20)],
                    body_chirho,
                }
            };
            let result_chirho = state_chirho
                .instantiate_term_chirho(
                    &term_chirho,
                    &parameters_chirho,
                    false,
                    SpanChirho::DUMMY_CHIRHO,
                )
                .unwrap();
            assert_eq!(result_chirho.free_vars_chirho(), vec![top_chirho]);
            let expected_body_chirho = Box::new(TyChirho::fun_chirho(
                TyChirho::VarChirho(top_chirho),
                TyChirho::VarChirho(TyVarChirho(21)),
            ));
            let expected_chirho = if required_chirho {
                TyChirho::RequiredForallChirho {
                    vars_chirho: vec![TyVarChirho(21)],
                    body_chirho: expected_body_chirho,
                }
            } else {
                TyChirho::ForallChirho {
                    vars_chirho: vec![TyVarChirho(21)],
                    body_chirho: expected_body_chirho,
                }
            };
            assert_eq!(result_chirho, expected_chirho);
        }
    }

    #[test]
    fn published_free_slots_never_read_the_consumers_substitution_chirho() {
        let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
        state_chirho.substitution_chirho.insert_chirho(
            TyVarChirho(99),
            ElaborationStateChirho::type_universe_chirho(),
        );
        assert!(
            state_chirho
                .instantiate_term_chirho(
                    &TyChirho::VarChirho(TyVarChirho(99)),
                    &SubstChirho::empty_chirho(),
                    false,
                    SpanChirho::DUMMY_CHIRHO
                )
                .is_err()
        );
    }
}
