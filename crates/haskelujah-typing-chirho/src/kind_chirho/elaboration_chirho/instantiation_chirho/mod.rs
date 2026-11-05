// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Capture-safe instantiation at the checked declaration boundary. Producer
//! slots are interpreted before the consumer's substitution can see their IDs.

use super::*;
use crate::subst_chirho::SubstChirho;

struct DeclarationInstantiationChirho<'producer_chirho> {
    parameters_chirho: &'producer_chirho SubstChirho,
    local_chirho: bool,
    span_chirho: SpanChirho,
    nested_binders_chirho:
        &'producer_chirho [crate::ty_chirho::declarations_chirho::DeclarationLocalBinderChirho],
    require_kinds_chirho: bool,
}

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
            &DeclarationInstantiationChirho {
                parameters_chirho,
                local_chirho,
                span_chirho,
                nested_binders_chirho: &[],
                require_kinds_chirho: false,
            },
            0,
            &mut Vec::new(),
        )
    }

    pub(super) fn instantiate_checked_declaration_term_chirho(
        &mut self,
        term_chirho: &TyChirho,
        parameters_chirho: &SubstChirho,
        local_chirho: bool,
        nested_binders_chirho: &[crate::ty_chirho::declarations_chirho::DeclarationLocalBinderChirho],
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        self.instantiate_depth_chirho(
            term_chirho,
            &DeclarationInstantiationChirho {
                parameters_chirho,
                local_chirho,
                span_chirho,
                nested_binders_chirho,
                require_kinds_chirho: true,
            },
            0,
            &mut Vec::new(),
        )
    }

    fn instantiate_depth_chirho(
        &mut self,
        term_chirho: &TyChirho,
        context_chirho: &DeclarationInstantiationChirho<'_>,
        depth_chirho: usize,
        bound_chirho: &mut Vec<(TyVarChirho, TyVarChirho)>,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let parameters_chirho = context_chirho.parameters_chirho;
        let local_chirho = context_chirho.local_chirho;
        let span_chirho = context_chirho.span_chirho;
        self.charge_chirho(1, 1, span_chirho)?;
        if depth_chirho > 128 {
            return Err(TypeElaboratorChirho::error_chirho(
                "declaration instantiation depth bound exhausted",
                span_chirho,
            ));
        }
        Ok(match term_chirho {
            TyChirho::QualChirho {
                context_chirho: constraints_chirho,
                body_chirho,
            } => {
                self.charge_chirho(
                    constraints_chirho.len(),
                    constraints_chirho.len(),
                    span_chirho,
                )?;
                TyChirho::QualChirho {
                    context_chirho: constraints_chirho
                        .iter()
                        .map(|constraint_chirho| {
                            self.instantiate_depth_chirho(
                                constraint_chirho,
                                context_chirho,
                                depth_chirho + 1,
                                bound_chirho,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                    body_chirho: Box::new(self.instantiate_depth_chirho(
                        body_chirho,
                        context_chirho,
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
                    context_chirho,
                    depth_chirho + 1,
                    bound_chirho,
                )?),
                Box::new(self.instantiate_depth_chirho(
                    right_chirho,
                    context_chirho,
                    depth_chirho + 1,
                    bound_chirho,
                )?),
            ),
            TyChirho::FunChirho(left_chirho, right_chirho, mult_chirho) => TyChirho::FunChirho(
                Box::new(self.instantiate_depth_chirho(
                    left_chirho,
                    context_chirho,
                    depth_chirho + 1,
                    bound_chirho,
                )?),
                Box::new(self.instantiate_depth_chirho(
                    right_chirho,
                    context_chirho,
                    depth_chirho + 1,
                    bound_chirho,
                )?),
                *mult_chirho,
            ),
            TyChirho::ListChirho(element_chirho) => {
                TyChirho::ListChirho(Box::new(self.instantiate_depth_chirho(
                    element_chirho,
                    context_chirho,
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
                                context_chirho,
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
                let result_chirho = (|| {
                    let mut records_chirho = Vec::with_capacity(vars_chirho.len());
                    for source_record_chirho in vars_chirho.records_chirho() {
                        let source_chirho = source_record_chirho.variable_chirho;
                        self.charge_chirho(
                            context_chirho.nested_binders_chirho.len() + 1,
                            0,
                            span_chirho,
                        )?;
                        let provider_chirho =
                            context_chirho
                                .nested_binders_chirho
                                .iter()
                                .find(|record_chirho| {
                                    record_chirho.binder_chirho.variable_chirho == source_chirho
                                });
                        let (kind_chirho, specificity_chirho) =
                            if let Some(kind_chirho) = &source_record_chirho.kind_chirho {
                                (
                                    Some(self.instantiate_depth_chirho(
                                        kind_chirho,
                                        context_chirho,
                                        depth_chirho + 1,
                                        bound_chirho,
                                    )?),
                                    source_record_chirho.specificity_chirho,
                                )
                            } else if let Some(provider_chirho) = provider_chirho {
                                (
                                    Some(self.instantiate_depth_chirho(
                                        &provider_chirho.binder_chirho.kind_chirho,
                                        context_chirho,
                                        depth_chirho + 1,
                                        bound_chirho,
                                    )?),
                                    Some(provider_chirho.specificity_chirho),
                                )
                            } else if local_chirho
                                && self.variable_kinds_chirho.contains_key(&source_chirho)
                            {
                                let kind_chirho =
                                    self.variable_kind_chirho(source_chirho, span_chirho)?;
                                (
                                    Some(self.instantiate_depth_chirho(
                                        &kind_chirho,
                                        context_chirho,
                                        depth_chirho + 1,
                                        bound_chirho,
                                    )?),
                                    self.source_specificity_chirho(source_chirho),
                                )
                            } else {
                                (None, source_record_chirho.specificity_chirho)
                            };
                        if context_chirho.require_kinds_chirho
                            && (kind_chirho.is_none() || specificity_chirho.is_none())
                        {
                            return Err(TypeElaboratorChirho::error_chirho(
                                "checked synonym universal lost its producer-owned kind or specificity",
                                span_chirho,
                            ));
                        }
                        let renamed_chirho = self.fresh_chirho(None, span_chirho)?;
                        let stored_kind_chirho = if let Some(kind_chirho) = kind_chirho {
                            let copied_chirho = self.copy_chirho(&kind_chirho, span_chirho)?;
                            self.set_variable_kind_chirho(renamed_chirho, kind_chirho);
                            Some(Arc::new(copied_chirho))
                        } else {
                            None
                        };
                        if let Some(specificity_chirho) = specificity_chirho {
                            self.note_source_variable_chirho(
                                renamed_chirho,
                                specificity_chirho,
                                true,
                                span_chirho,
                            )?;
                        }
                        records_chirho.push(crate::ty_chirho::QuantifiedVariableChirho {
                            variable_chirho: renamed_chirho,
                            kind_chirho: stored_kind_chirho,
                            source_chirho: source_record_chirho.source_chirho.clone(),
                            specificity_chirho,
                        });
                        bound_chirho.push((source_chirho, renamed_chirho));
                    }
                    let body_chirho = self.instantiate_depth_chirho(
                        body_chirho,
                        context_chirho,
                        depth_chirho + 1,
                        bound_chirho,
                    )?;
                    Ok((
                        crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(
                            records_chirho,
                        ),
                        body_chirho,
                    ))
                })();
                bound_chirho.truncate(mark_chirho);
                let (fresh_chirho, body_chirho) = result_chirho?;
                let body_chirho = Box::new(body_chirho);
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

impl TypeElaboratorChirho<'_> {
    pub(super) fn introduced_quantifiers_chirho(
        &mut self,
        variables_chirho: &[TyVarChirho],
        span_chirho: SpanChirho,
    ) -> Result<crate::ty_chirho::QuantifiedVariablesChirho, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(
            variables_chirho.len(),
            variables_chirho.len(),
            span_chirho,
        )?;
        let mut records_chirho = Vec::with_capacity(variables_chirho.len());
        for variable_chirho in variables_chirho {
            let kind_chirho = self
                .state_chirho
                .variable_kind_chirho(*variable_chirho, span_chirho)?;
            let kind_chirho = self
                .state_chirho
                .resolve_chirho(&kind_chirho, span_chirho)?;
            let specificity_chirho = self
                .state_chirho
                .source_specificity_chirho(*variable_chirho)
                .ok_or_else(|| {
                    Self::error_chirho(
                        "introduced universal lost its actual binder specificity",
                        span_chirho,
                    )
                })?;
            records_chirho.push(crate::ty_chirho::QuantifiedVariableChirho { variable_chirho: *variable_chirho, kind_chirho: Some(Arc::new(kind_chirho)), source_chirho: Some(haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho::InferenceChirho { owner_chirho: self.state_chirho.owner_id_chirho(span_chirho)?, variable_chirho: *variable_chirho }), specificity_chirho: Some(specificity_chirho) });
        }
        Ok(crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(records_chirho))
    }

    fn with_universal_kind_scope_chirho<ResultChirho>(
        &mut self,
        variables_chirho: &[TyVarChirho],
        cutoff_chirho: u32,
        span_chirho: SpanChirho,
        action_chirho: impl FnOnce(&mut Self) -> Result<ResultChirho, ElaborationErrorChirho>,
    ) -> Result<ResultChirho, ElaborationErrorChirho> {
        let added_chirho = self
            .state_chirho
            .open_rigid_scope_chirho(variables_chirho, span_chirho)?;
        let guard_chirho = match self.state_chirho.open_kind_guard_chirho(
            variables_chirho,
            cutoff_chirho,
            span_chirho,
        ) {
            Ok(guard_chirho) => guard_chirho,
            Err(error_chirho) => {
                self.state_chirho.close_rigid_scope_chirho(added_chirho);
                return Err(error_chirho);
            }
        };
        let result_chirho = action_chirho(self);
        self.state_chirho.close_kind_guard_chirho(guard_chirho);
        self.state_chirho.close_rigid_scope_chirho(added_chirho);
        result_chirho
    }

    pub(super) fn check_universal_kind_argument_chirho(
        &mut self,
        syntax_chirho: &mut haskelujah_ast_chirho::ty_chirho::TypeChirho,
        expected_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<ElaboratedTypeChirho, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        if self.type_depth_chirho >= 128 {
            return Err(Self::error_chirho(
                "universal kind argument depth bound exhausted",
                span_chirho,
            ));
        }
        self.type_depth_chirho += 1;
        let result_chirho = self.check_universal_kind_argument_inner_chirho(
            syntax_chirho,
            expected_chirho,
            span_chirho,
        );
        self.type_depth_chirho -= 1;
        result_chirho
    }

    fn check_universal_kind_argument_inner_chirho(
        &mut self,
        syntax_chirho: &mut haskelujah_ast_chirho::ty_chirho::TypeChirho,
        expected_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<ElaboratedTypeChirho, ElaborationErrorChirho> {
        use haskelujah_ast_chirho::decl_chirho::TyVarChirho as AstTyVarChirho;
        use haskelujah_ast_chirho::ty_chirho::TypeChirho;
        let (vars_chirho, body_chirho) = match &expected_chirho {
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => (vars_chirho, body_chirho),
            _ => {
                return Err(Self::error_chirho(
                    "universal kind check requires a checked quantified kind",
                    span_chirho,
                ));
            }
        };
        let required_chirho = matches!(&expected_chirho, TyChirho::RequiredForallChirho { .. });
        if matches!(syntax_chirho, TypeChirho::VarChirho(_)) {
            let result_chirho = self.infer_type_chirho(syntax_chirho)?;
            let kind_chirho = self
                .state_chirho
                .resolve_chirho(&result_chirho.kind_chirho, span_chirho)?;
            if matches!(
                kind_chirho,
                TyChirho::ForallChirho { .. } | TyChirho::RequiredForallChirho { .. }
            ) {
                self.unify_kinds_chirho(&kind_chirho, &expected_chirho, span_chirho)?;
                return Ok(ElaboratedTypeChirho {
                    term_chirho: result_chirho.term_chirho,
                    kind_chirho: expected_chirho,
                });
            }
            return Err(Self::error_chirho(
                "kind mismatch: an inferred or monomorphic variable cannot acquire a universal kind",
                span_chirho,
            ));
        }
        if let TypeChirho::ParenChirho { inner_chirho, .. } = syntax_chirho {
            return self.check_universal_kind_argument_chirho(
                inner_chirho,
                expected_chirho,
                span_chirho,
            );
        }
        if matches!(syntax_chirho, TypeChirho::KindAnnotationChirho { .. }) {
            let result_chirho = self.infer_type_chirho(syntax_chirho)?;
            self.unify_kinds_chirho(&result_chirho.kind_chirho, &expected_chirho, span_chirho)?;
            return Ok(result_chirho);
        }
        let syntax_required_chirho =
            matches!(syntax_chirho, TypeChirho::RequiredForallChirho { .. });
        if let TypeChirho::ForallChirho {
            vars_chirho: syntax_vars_chirho,
            body_chirho: syntax_body_chirho,
            ..
        }
        | TypeChirho::RequiredForallChirho {
            vars_chirho: syntax_vars_chirho,
            body_chirho: syntax_body_chirho,
            ..
        } = syntax_chirho
        {
            if required_chirho != syntax_required_chirho
                || vars_chirho.len() != syntax_vars_chirho.len()
            {
                return Err(Self::error_chirho(
                    "universal kind argument has different quantifier visibility or arity",
                    span_chirho,
                ));
            }
            return self.scoped_chirho(|this_chirho| {
                let cutoff_chirho = this_chirho.state_chirho.next_variable_chirho;
                let variables_chirho = this_chirho.bind_forall_chirho(syntax_vars_chirho)?;
                let mut mapping_chirho = SubstChirho::empty_chirho();
                for (source_chirho, actual_chirho) in vars_chirho.iter().zip(&variables_chirho) {
                    mapping_chirho
                        .insert_chirho(*source_chirho, TyChirho::VarChirho(*actual_chirho));
                }
                let result_chirho = this_chirho.with_universal_kind_scope_chirho(
                    &variables_chirho,
                    cutoff_chirho,
                    span_chirho,
                    |this_chirho| {
                        for (source_chirho, actual_chirho) in
                            vars_chirho.iter().zip(&variables_chirho)
                        {
                            let expected_kind_chirho = this_chirho
                                .state_chirho
                                .variable_kind_chirho(*source_chirho, span_chirho)?;
                            let expected_kind_chirho =
                                this_chirho.state_chirho.instantiate_term_chirho(
                                    &expected_kind_chirho,
                                    &mapping_chirho,
                                    true,
                                    span_chirho,
                                )?;
                            let actual_kind_chirho = this_chirho
                                .state_chirho
                                .variable_kind_chirho(*actual_chirho, span_chirho)?;
                            this_chirho.unify_kinds_chirho(
                                &actual_kind_chirho,
                                &expected_kind_chirho,
                                span_chirho,
                            )?;
                        }
                        let kind_chirho = this_chirho.state_chirho.instantiate_term_chirho(
                            body_chirho,
                            &mapping_chirho,
                            true,
                            span_chirho,
                        )?;
                        this_chirho.infer_type_at_kind_chirho(syntax_body_chirho, &kind_chirho)
                    },
                )?;
                let variables_chirho =
                    this_chirho.introduced_quantifiers_chirho(&variables_chirho, span_chirho)?;
                Ok(ElaboratedTypeChirho {
                    term_chirho: if required_chirho {
                        TyChirho::RequiredForallChirho {
                            vars_chirho: variables_chirho,
                            body_chirho: Box::new(result_chirho.term_chirho),
                        }
                    } else {
                        TyChirho::ForallChirho {
                            vars_chirho: variables_chirho,
                            body_chirho: Box::new(result_chirho.term_chirho),
                        }
                    },
                    kind_chirho: this_chirho
                        .state_chirho
                        .copy_chirho(&expected_chirho, span_chirho)?,
                })
            });
        }
        let empty_list_chirho = matches!(syntax_chirho, TypeChirho::PromotedListChirho { elements_chirho, .. } if elements_chirho.is_empty());
        let source_chirho = if empty_list_chirho {
            // The empty promoted-list grammar is the builtin nil producer,
            // whose complete universal prefix lives in this same catalogue.
            self.seed_compiler_header_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerTypeConstructorChirho::ListChirho,
                span_chirho,
            )?;
            self.registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(
                    haskelujah_ast_chirho::promotion_chirho::PromotedKeyChirho::builtin_nil_chirho(
                    )
                    .as_str_chirho(),
                )
                .cloned()
        } else {
            if !matches!(
                syntax_chirho,
                TypeChirho::ConChirho(_) | TypeChirho::PromotedConChirho { .. }
            ) {
                return Err(Self::error_chirho(
                    "kind mismatch: a partially applied type cannot regain a universal kind",
                    span_chirho,
                ));
            }
            self.scheme_chirho(syntax_chirho)?
        }
        .ok_or_else(|| {
            Self::error_chirho(
                "universal kind argument has no checked declaration telescope",
                span_chirho,
            )
        })?;
        if source_chirho
            .binders_chirho
            .iter()
            .take_while(|binder_chirho| {
                binder_chirho.visibility_chirho != TelescopeVisibilityChirho::RequiredChirho
            })
            .count()
            != vars_chirho.len()
        {
            return Err(Self::error_chirho(
                "kind mismatch: constructor universal prefix differs from the expected kind",
                span_chirho,
            ));
        }
        // Checking abstracts over a fresh copy of the expected kind. Its
        // bound IDs must not become both a value-syntax binder and the separate
        // written annotation binder when this argument is published/rechecked.
        let abstraction_kind_chirho = self.state_chirho.instantiate_term_chirho(
            &expected_chirho,
            &SubstChirho::empty_chirho(),
            true,
            span_chirho,
        )?;
        let (vars_chirho, body_chirho) = match &abstraction_kind_chirho {
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => (vars_chirho, body_chirho),
            _ => {
                return Err(Self::error_chirho(
                    "universal abstraction lost its checked kind",
                    span_chirho,
                ));
            }
        };
        let cutoff_chirho = self.state_chirho.next_variable_chirho;
        let result_chirho = self.with_universal_kind_scope_chirho(
            vars_chirho,
            cutoff_chirho,
            span_chirho,
            |this_chirho| this_chirho.infer_type_at_kind_chirho(syntax_chirho, body_chirho),
        )?;
        self.state_chirho
            .charge_chirho(vars_chirho.len(), vars_chirho.len(), span_chirho)?;
        let mut binders_chirho = Vec::with_capacity(vars_chirho.len());
        for variable_chirho in vars_chirho {
            let TypeChirho::VarChirho(name_chirho) =
                self.surface_chirho(&TyChirho::VarChirho(*variable_chirho), span_chirho)?
            else {
                return Err(Self::error_chirho(
                    "universal kind argument lost its variable producer",
                    span_chirho,
                ));
            };
            let specificity_chirho = self
                .state_chirho
                .source_specificity_chirho(*variable_chirho)
                .ok_or_else(|| {
                    Self::error_chirho("universal kind argument lost its specificity", span_chirho)
                })?;
            binders_chirho.push(AstTyVarChirho {
                name_chirho,
                kind_annotation_chirho: None,
                visibility_chirho: if required_chirho {
                    haskelujah_ast_chirho::decl_chirho::TyVarVisibilityChirho::VisibleChirho
                } else {
                    haskelujah_ast_chirho::decl_chirho::TyVarVisibilityChirho::InvisibleChirho
                },
                specificity_chirho,
            });
        }
        let body_syntax_chirho = Box::new(std::mem::replace(
            syntax_chirho,
            TypeChirho::WildcardChirho { span_chirho },
        ));
        *syntax_chirho = if required_chirho {
            TypeChirho::RequiredForallChirho {
                vars_chirho: binders_chirho,
                body_chirho: body_syntax_chirho,
                span_chirho,
            }
        } else {
            TypeChirho::ForallChirho {
                vars_chirho: binders_chirho,
                body_chirho: body_syntax_chirho,
                span_chirho,
            }
        };
        let annotation_chirho = self.state_chirho.instantiate_term_chirho(
            &expected_chirho,
            &SubstChirho::empty_chirho(),
            true,
            span_chirho,
        )?;
        let annotation_chirho = self.surface_chirho(&annotation_chirho, span_chirho)?;
        let abstraction_chirho =
            std::mem::replace(syntax_chirho, TypeChirho::WildcardChirho { span_chirho });
        *syntax_chirho = TypeChirho::KindAnnotationChirho {
            ty_chirho: Box::new(abstraction_chirho),
            kind_chirho: Box::new(annotation_chirho),
            span_chirho,
        };
        let variables_chirho = self.introduced_quantifiers_chirho(vars_chirho, span_chirho)?;
        Ok(ElaboratedTypeChirho {
            term_chirho: if required_chirho {
                TyChirho::RequiredForallChirho {
                    vars_chirho: variables_chirho,
                    body_chirho: Box::new(result_chirho.term_chirho),
                }
            } else {
                TyChirho::ForallChirho {
                    vars_chirho: variables_chirho,
                    body_chirho: Box::new(result_chirho.term_chirho),
                }
            },
            kind_chirho: expected_chirho,
        })
    }
}

#[cfg(test)]
mod tests_chirho;
