// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A transient application cursor over a checked kind. It never changes a
//! declaration's telescope or its family/synonym reduction boundary.

use super::*;
use haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho;

struct ResultKindBinderChirho {
    record_chirho: crate::ty_chirho::QuantifiedVariableChirho,
    visibility_chirho: TelescopeVisibilityChirho,
}

pub(super) struct ResultKindPrefixChirho {
    kind_chirho: TyChirho,
    binders_chirho: Vec<ResultKindBinderChirho>,
    cursor_chirho: usize,
    pub(super) applications_chirho: Option<Vec<(bool, TyChirho)>>,
}

impl ResultKindPrefixChirho {
    pub(super) fn new_chirho(kind_chirho: TyChirho, record_chirho: bool) -> Self {
        Self {
            kind_chirho,
            binders_chirho: Vec::new(),
            cursor_chirho: 0,
            applications_chirho: record_chirho.then(Vec::new),
        }
    }

    fn next_chirho(
        &mut self,
        context_chirho: &mut TypeElaboratorChirho<'_>,
        span_chirho: SpanChirho,
    ) -> Result<Option<DeclarationBinderChirho>, ElaborationErrorChirho> {
        if self.cursor_chirho == self.binders_chirho.len() {
            let kind_chirho = context_chirho
                .state_chirho
                .resolve_chirho(&self.kind_chirho, span_chirho)?;
            let (variables_chirho, body_chirho, required_chirho) = match kind_chirho {
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                } => (vars_chirho, body_chirho, false),
                TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => (vars_chirho, body_chirho, true),
                _ => return Ok(None),
            };
            context_chirho.state_chirho.charge_chirho(
                variables_chirho.len(),
                variables_chirho.len(),
                span_chirho,
            )?;
            self.binders_chirho.clear();
            self.cursor_chirho = 0;
            let mut opening_chirho = SubstChirho::empty_chirho();
            for source_record_chirho in variables_chirho.records_chirho() {
                let source_chirho = source_record_chirho.variable_chirho;
                let specificity_chirho = source_record_chirho
                    .specificity_chirho
                    .or_else(|| {
                        context_chirho
                            .state_chirho
                            .source_specificity_chirho(source_chirho)
                    })
                    .ok_or_else(|| {
                        TypeElaboratorChirho::error_chirho(
                            "kind application binder lost its producer-owned specificity",
                            span_chirho,
                        )
                    })?;
                if required_chirho && specificity_chirho == TyVarSpecificityChirho::InferredChirho {
                    return Err(TypeElaboratorChirho::error_chirho(
                        "a required kind binder cannot be inferred",
                        span_chirho,
                    ));
                }
                let visibility_chirho = if required_chirho {
                    TelescopeVisibilityChirho::RequiredChirho
                } else if specificity_chirho == TyVarSpecificityChirho::InferredChirho {
                    TelescopeVisibilityChirho::InferredChirho
                } else {
                    TelescopeVisibilityChirho::SpecifiedChirho
                };
                let kind_chirho = if let Some(kind_chirho) = &source_record_chirho.kind_chirho {
                    context_chirho
                        .state_chirho
                        .copy_chirho(kind_chirho, span_chirho)?
                } else {
                    context_chirho
                        .state_chirho
                        .variable_kind_chirho(source_chirho, span_chirho)?
                };
                let kind_chirho = context_chirho.state_chirho.instantiate_term_chirho(
                    &kind_chirho,
                    &opening_chirho,
                    true,
                    span_chirho,
                )?;
                let stored_kind_chirho = context_chirho
                    .state_chirho
                    .copy_chirho(&kind_chirho, span_chirho)?;
                let variable_chirho = context_chirho
                    .state_chirho
                    .fresh_chirho(Some(stored_kind_chirho), span_chirho)?;
                context_chirho
                    .state_chirho
                    .charge_chirho(1, 1, span_chirho)?;
                opening_chirho.insert_chirho(source_chirho, TyChirho::VarChirho(variable_chirho));
                self.binders_chirho.push(ResultKindBinderChirho {
                    record_chirho: crate::ty_chirho::QuantifiedVariableChirho {
                        variable_chirho,
                        kind_chirho: Some(Arc::new(kind_chirho)),
                        source_chirho: None,
                        specificity_chirho: source_record_chirho
                            .specificity_chirho
                            .or(Some(specificity_chirho)),
                    },
                    visibility_chirho,
                });
            }
            self.kind_chirho = context_chirho.state_chirho.instantiate_term_chirho(
                &body_chirho,
                &opening_chirho,
                true,
                span_chirho,
            )?;
            if self.binders_chirho.is_empty() {
                return self.next_chirho(context_chirho, span_chirho);
            }
        }
        let binder_chirho = &self.binders_chirho[self.cursor_chirho];
        Ok(Some(DeclarationBinderChirho {
            term_specificity_chirho: None,
            variable_chirho: binder_chirho.record_chirho.variable_chirho,
            kind_chirho: context_chirho
                .state_chirho
                .variable_kind_chirho(binder_chirho.record_chirho.variable_chirho, span_chirho)?,
            visibility_chirho: binder_chirho.visibility_chirho,
        }))
    }

    fn remember_chirho(
        &mut self,
        context_chirho: &mut TypeElaboratorChirho<'_>,
        quantified_chirho: bool,
        argument_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        if let Some(applications_chirho) = &mut self.applications_chirho {
            context_chirho
                .state_chirho
                .charge_chirho(1, 1, span_chirho)?;
            applications_chirho.push((
                quantified_chirho,
                context_chirho
                    .state_chirho
                    .copy_chirho(argument_chirho, span_chirho)?,
            ));
        }
        Ok(())
    }

    pub(super) fn finish_chirho(
        self,
        context_chirho: &mut TypeElaboratorChirho<'_>,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let mut kind_chirho = self.kind_chirho;
        for binder_chirho in self.binders_chirho[self.cursor_chirho..].iter().rev() {
            context_chirho
                .state_chirho
                .charge_chirho(1, 1, span_chirho)?;
            let mut record_chirho = binder_chirho.record_chirho.clone();
            context_chirho
                .state_chirho
                .mark_universal_chirho(record_chirho.variable_chirho, span_chirho)?;
            let binder_kind_chirho = context_chirho
                .state_chirho
                .variable_kind_chirho(record_chirho.variable_chirho, span_chirho)?;
            record_chirho.kind_chirho = Some(Arc::new(
                context_chirho
                    .state_chirho
                    .resolve_chirho(&binder_kind_chirho, span_chirho)?,
            ));
            let variables_chirho =
                crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(vec![
                    record_chirho,
                ]);
            kind_chirho =
                if binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho {
                    TyChirho::RequiredForallChirho {
                        vars_chirho: variables_chirho,
                        body_chirho: Box::new(kind_chirho),
                    }
                } else {
                    TyChirho::ForallChirho {
                        vars_chirho: variables_chirho,
                        body_chirho: Box::new(kind_chirho),
                    }
                };
        }
        context_chirho
            .state_chirho
            .resolve_chirho(&kind_chirho, span_chirho)
    }
}

impl TypeElaboratorChirho<'_> {
    fn collect_beta_binders_chirho(
        &mut self,
        root_chirho: &TyChirho,
        variables_chirho: &mut Vec<TyVarChirho>,
        seen_chirho: &mut std::collections::HashSet<TyVarChirho>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let mut pending_chirho = vec![(root_chirho, 0_usize)];
        while let Some((term_chirho, depth_chirho)) = pending_chirho.pop() {
            self.state_chirho.charge_chirho(1, 0, span_chirho)?;
            if depth_chirho > 128 {
                return Err(Self::error_chirho(
                    "kind beta binder metadata depth bound exhausted",
                    span_chirho,
                ));
            }
            match term_chirho {
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
                    for variable_chirho in vars_chirho {
                        if seen_chirho.insert(*variable_chirho) {
                            variables_chirho.push(*variable_chirho);
                        }
                    }
                    pending_chirho.push((body_chirho, depth_chirho + 1));
                }
                TyChirho::AppChirho(left_chirho, right_chirho)
                | TyChirho::FunChirho(left_chirho, right_chirho, _) => pending_chirho.extend([
                    (left_chirho.as_ref(), depth_chirho + 1),
                    (right_chirho.as_ref(), depth_chirho + 1),
                ]),
                TyChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                } => {
                    self.state_chirho.charge_chirho(
                        context_chirho.len(),
                        context_chirho.len(),
                        span_chirho,
                    )?;
                    pending_chirho.extend(
                        context_chirho
                            .iter()
                            .map(|term_chirho| (term_chirho, depth_chirho + 1)),
                    );
                    pending_chirho.push((body_chirho, depth_chirho + 1));
                }
                TyChirho::ListChirho(element_chirho) => {
                    pending_chirho.push((element_chirho, depth_chirho + 1))
                }
                TyChirho::TupleChirho(elements_chirho) => {
                    self.state_chirho.charge_chirho(
                        elements_chirho.len(),
                        elements_chirho.len(),
                        span_chirho,
                    )?;
                    pending_chirho.extend(
                        elements_chirho
                            .iter()
                            .map(|term_chirho| (term_chirho, depth_chirho + 1)),
                    );
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub(super) fn apply_checked_kind_argument_chirho(
        &mut self,
        function_chirho: TyChirho,
        argument_chirho: TyChirho,
        quantified_chirho: bool,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let mut failure_chirho = None;
        let result_chirho = SubstChirho::reduce_checked_argument_chirho(
            &function_chirho,
            &argument_chirho,
            quantified_chirho,
            &mut |work_chirho, output_chirho| match self.state_chirho.charge_chirho(
                work_chirho,
                output_chirho,
                span_chirho,
            ) {
                Ok(()) => Some(()),
                Err(error_chirho) => {
                    failure_chirho = Some(error_chirho);
                    None
                }
            },
        );
        let reduced_chirho = result_chirho.ok_or_else(|| {
            failure_chirho.unwrap_or_else(|| {
                Self::error_chirho(
                    "checked kind beta substitution capture or resource bound refused",
                    span_chirho,
                )
            })
        })?;
        let Some((result_chirho, replacement_chirho)) = reduced_chirho else {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            return Ok(TyChirho::AppChirho(
                Box::new(function_chirho),
                Box::new(argument_chirho),
            ));
        };
        let mut variables_chirho = Vec::new();
        let mut seen_chirho = std::collections::HashSet::new();
        self.collect_beta_binders_chirho(
            &result_chirho,
            &mut variables_chirho,
            &mut seen_chirho,
            span_chirho,
        )?;
        let mut cursor_chirho = 0;
        while let Some(variable_chirho) = variables_chirho.get(cursor_chirho).copied() {
            cursor_chirho += 1;
            let kind_chirho = self
                .state_chirho
                .variable_kind_chirho(variable_chirho, span_chirho)?;
            self.collect_beta_binders_chirho(
                &kind_chirho,
                &mut variables_chirho,
                &mut seen_chirho,
                span_chirho,
            )?;
            let mut failure_chirho = None;
            let kind_chirho = replacement_chirho
                .apply_bounded_chirho(&kind_chirho, &mut |work_chirho, output_chirho| match self
                    .state_chirho
                    .charge_chirho(work_chirho, output_chirho, span_chirho)
                {
                    Ok(()) => Some(()),
                    Err(error_chirho) => {
                        failure_chirho = Some(error_chirho);
                        None
                    }
                })
                .ok_or_else(|| {
                    failure_chirho.unwrap_or_else(|| {
                        Self::error_chirho(
                            "kind beta dependent binder metadata capture refused",
                            span_chirho,
                        )
                    })
                })?;
            self.state_chirho
                .set_variable_kind_chirho(variable_chirho, kind_chirho);
        }
        Ok(result_chirho)
    }

    fn push_implicit_kind_argument_chirho(
        &mut self,
        head_chirho: &mut TypeChirho,
        term_chirho: &mut TyChirho,
        argument_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let syntax_chirho = self.surface_chirho(&argument_chirho, span_chirho)?;
        let previous_chirho =
            std::mem::replace(head_chirho, TypeChirho::WildcardChirho { span_chirho });
        *head_chirho = TypeChirho::KindApplicationChirho {
            fun_chirho: Box::new(previous_chirho),
            arg_chirho: Box::new(syntax_chirho),
            written_chirho: false,
            span_chirho,
        };
        let previous_chirho = std::mem::replace(term_chirho, TyChirho::TupleChirho(Vec::new()));
        *term_chirho = self.apply_checked_kind_argument_chirho(
            previous_chirho,
            argument_chirho,
            true,
            span_chirho,
        )?;
        Ok(())
    }

    /// Ordinary checking instantiates a variable's universal kind. A universal
    /// expected kind takes the separate bidirectional path and retains it.
    pub(super) fn instantiate_invisible_kind_chirho(
        &mut self,
        syntax_chirho: &mut TypeChirho,
        typed_chirho: &mut ElaboratedTypeChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let kind_chirho = self
            .state_chirho
            .resolve_chirho(&typed_chirho.kind_chirho, span_chirho)?;
        if !matches!(kind_chirho, TyChirho::ForallChirho { .. }) {
            return Ok(());
        }
        let kind_chirho = self
            .state_chirho
            .instantiate_checked_declaration_term_chirho(
                &kind_chirho,
                &SubstChirho::empty_chirho(),
                true,
                &[],
                span_chirho,
            )?;
        let mut prefix_chirho = ResultKindPrefixChirho::new_chirho(kind_chirho, false);
        loop {
            let Some(binder_chirho) = prefix_chirho.next_chirho(self, span_chirho)? else {
                break;
            };
            if binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho {
                break;
            }
            self.push_implicit_kind_argument_chirho(
                syntax_chirho,
                &mut typed_chirho.term_chirho,
                TyChirho::VarChirho(binder_chirho.variable_chirho),
                span_chirho,
            )?;
            prefix_chirho.cursor_chirho += 1;
        }
        typed_chirho.kind_chirho = prefix_chirho.finish_chirho(self, span_chirho)?;
        Ok(())
    }

    pub(super) fn apply_result_kind_argument_chirho(
        &mut self,
        prefix_chirho: &mut ResultKindPrefixChirho,
        head_chirho: &mut TypeChirho,
        term_chirho: &mut TyChirho,
        argument_chirho: &mut TypeChirho,
        application_chirho: (bool, bool),
    ) -> Result<(), ElaborationErrorChirho> {
        let (kind_argument_chirho, written_chirho) = application_chirho;
        let span_chirho = argument_chirho.span_chirho();
        loop {
            let Some(binder_chirho) = prefix_chirho.next_chirho(self, span_chirho)? else {
                break;
            };
            let automatic_chirho = (binder_chirho.visibility_chirho
                == TelescopeVisibilityChirho::InferredChirho
                && (!kind_argument_chirho || written_chirho))
                || (!kind_argument_chirho
                    && binder_chirho.visibility_chirho
                        != TelescopeVisibilityChirho::RequiredChirho);
            if !automatic_chirho {
                break;
            }
            let argument_term_chirho = TyChirho::VarChirho(binder_chirho.variable_chirho);
            prefix_chirho.remember_chirho(self, true, &argument_term_chirho, span_chirho)?;
            self.push_implicit_kind_argument_chirho(
                head_chirho,
                term_chirho,
                argument_term_chirho,
                span_chirho,
            )?;
            prefix_chirho.cursor_chirho += 1;
        }
        let binder_chirho = prefix_chirho.next_chirho(self, span_chirho)?;
        let elaborated_chirho = if let Some(binder_chirho) = &binder_chirho {
            self.infer_application_argument_chirho(
                argument_chirho,
                &binder_chirho.kind_chirho,
                kind_argument_chirho,
            )?
        } else {
            if kind_argument_chirho {
                return Err(Self::error_chirho(
                    "no proven invisible kind binder remains",
                    span_chirho,
                ));
            }
            let expected_chirho = self
                .state_chirho
                .resolve_chirho(&prefix_chirho.kind_chirho, span_chirho)?;
            if let TyChirho::FunChirho(domain_chirho, _, _) = expected_chirho {
                self.infer_type_at_kind_chirho(argument_chirho, &domain_chirho)?
            } else {
                self.infer_type_chirho(argument_chirho)?
            }
        };
        let quantified_chirho = binder_chirho.is_some();
        prefix_chirho.remember_chirho(
            self,
            quantified_chirho,
            &elaborated_chirho.term_chirho,
            span_chirho,
        )?;
        if let Some(binder_chirho) = binder_chirho {
            if kind_argument_chirho
                == (binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho)
            {
                return Err(Self::error_chirho(
                    "type/kind argument visibility does not match the checked kind",
                    span_chirho,
                ));
            }
            self.unify_kinds_chirho(
                &elaborated_chirho.kind_chirho,
                &binder_chirho.kind_chirho,
                span_chirho,
            )?;
            self.unify_kinds_chirho(
                &TyChirho::VarChirho(binder_chirho.variable_chirho),
                &elaborated_chirho.term_chirho,
                span_chirho,
            )?;
            prefix_chirho.cursor_chirho += 1;
        } else {
            let next_kind_chirho = self.fresh_kind_chirho(span_chirho)?;
            let argument_kind_chirho = self
                .state_chirho
                .copy_chirho(&elaborated_chirho.kind_chirho, span_chirho)?;
            let expected_chirho = ElaborationStateChirho::arrow_chirho(
                argument_kind_chirho,
                self.state_chirho
                    .copy_chirho(&next_kind_chirho, span_chirho)?,
            );
            self.unify_kinds_chirho(&prefix_chirho.kind_chirho, &expected_chirho, span_chirho)?;
            prefix_chirho.kind_chirho = next_kind_chirho;
        }
        let previous_chirho = std::mem::replace(term_chirho, TyChirho::TupleChirho(Vec::new()));
        *term_chirho = self.apply_checked_kind_argument_chirho(
            previous_chirho,
            elaborated_chirho.term_chirho,
            quantified_chirho,
            span_chirho,
        )?;
        let previous_chirho =
            std::mem::replace(head_chirho, TypeChirho::WildcardChirho { span_chirho });
        *head_chirho = if kind_argument_chirho {
            TypeChirho::KindApplicationChirho {
                fun_chirho: Box::new(previous_chirho),
                arg_chirho: Box::new(std::mem::replace(
                    argument_chirho,
                    TypeChirho::WildcardChirho { span_chirho },
                )),
                written_chirho,
                span_chirho,
            }
        } else {
            TypeChirho::AppChirho {
                fun_chirho: Box::new(previous_chirho),
                arg_chirho: Box::new(std::mem::replace(
                    argument_chirho,
                    TypeChirho::WildcardChirho { span_chirho },
                )),
                span_chirho,
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn checked_beta_updates_transitive_retained_binder_kinds_chirho() {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        let module_chirho = ModuleChirho {
            name_chirho: haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(
                haskelujah_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
                    "BetaChirho",
                    span_chirho,
                ),
            ),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![],
            extensions_chirho: vec![],
            inline_pragmas_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho,
            origin_supply_chirho: Default::default(),
        };
        let registry_chirho = TypeFamilyEnvChirho::default();
        let type_scope_chirho = TypeConstructorScopeChirho::default();
        let family_scope_chirho = TypeFamilyScopeChirho::default();
        let promotion_scope_chirho = PromotionScopeChirho::default();
        let builtin_classes_chirho = BuiltinClassScopeChirho::default();
        let mut context_chirho = TypeElaboratorChirho::new_chirho(
            &module_chirho,
            super::super::super::ElaborationImportsChirho {
                registry_chirho: &registry_chirho,
                type_scope_chirho: &type_scope_chirho,
                family_scope_chirho: &family_scope_chirho,
                promotion_scope_chirho: &promotion_scope_chirho,
                builtin_classes_chirho: &builtin_classes_chirho,
                source_unit_chirho: None,
            },
        )
        .unwrap();
        let consumed_chirho = context_chirho
            .state_chirho
            .fresh_chirho(
                Some(ElaborationStateChirho::type_universe_chirho()),
                span_chirho,
            )
            .unwrap();
        let nested_chirho = context_chirho
            .state_chirho
            .fresh_chirho(Some(TyChirho::VarChirho(consumed_chirho)), span_chirho)
            .unwrap();
        let nested_kind_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![nested_chirho].into(),
            body_chirho: Box::new(ElaborationStateChirho::type_universe_chirho()),
        };
        let retained_chirho = context_chirho
            .state_chirho
            .fresh_chirho(Some(nested_kind_chirho), span_chirho)
            .unwrap();
        for variable_chirho in [nested_chirho, retained_chirho] {
            context_chirho
                .state_chirho
                .note_source_variable_chirho(
                    variable_chirho,
                    TyVarSpecificityChirho::SpecifiedChirho,
                    true,
                    span_chirho,
                )
                .unwrap();
        }
        let term_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![consumed_chirho, retained_chirho].into(),
            body_chirho: Box::new(TyChirho::VarChirho(retained_chirho)),
        };
        let result_chirho = context_chirho
            .apply_checked_kind_argument_chirho(
                term_chirho,
                ElaborationStateChirho::type_universe_chirho(),
                true,
                span_chirho,
            )
            .unwrap();
        assert_eq!(
            result_chirho,
            TyChirho::ForallChirho {
                vars_chirho: vec![retained_chirho].into(),
                body_chirho: Box::new(TyChirho::VarChirho(retained_chirho))
            }
        );
        assert_eq!(
            context_chirho
                .state_chirho
                .variable_kind_chirho(nested_chirho, span_chirho)
                .unwrap(),
            ElaborationStateChirho::type_universe_chirho()
        );
        let kind_chirho = context_chirho
            .state_chirho
            .variable_kind_chirho(retained_chirho, span_chirho)
            .unwrap();
        let freshened_chirho = context_chirho
            .state_chirho
            .instantiate_checked_declaration_term_chirho(
                &kind_chirho,
                &SubstChirho::empty_chirho(),
                true,
                &[],
                span_chirho,
            )
            .unwrap();
        let TyChirho::ForallChirho { vars_chirho, .. } = freshened_chirho else {
            panic!("retained nested forall")
        };
        assert_eq!(
            context_chirho
                .state_chirho
                .variable_kind_chirho(vars_chirho[0], span_chirho)
                .unwrap(),
            ElaborationStateChirho::type_universe_chirho()
        );
    }
}
