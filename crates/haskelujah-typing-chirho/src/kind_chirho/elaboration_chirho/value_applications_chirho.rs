// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor value applications consume the same checked telescope as
//! promotion, while inferred kind binders stay invisible to written @ arguments.

use super::*;
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

impl TypeElaboratorChirho<'_> {
    pub(super) fn elaborate_constructor_type_application_chirho(
        &mut self,
        expression_chirho: &mut ExprChirho,
    ) -> Result<bool, ElaborationErrorChirho> {
        let span_chirho = expression_chirho.span_chirho();
        let mut base_chirho = expression_chirho;
        let mut arguments_chirho = Vec::new();
        loop {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            match base_chirho {
                ExprChirho::TypeAppChirho {
                    expr_chirho,
                    ty_chirho,
                    ..
                } => {
                    arguments_chirho.push(ty_chirho);
                    base_chirho = expr_chirho;
                }
                ExprChirho::ParenChirho { inner_chirho, .. } => base_chirho = inner_chirho,
                _ => break,
            }
        }
        let ExprChirho::ConChirho(name_chirho) = base_chirho else {
            return Ok(false);
        };
        let Some(key_chirho) = self.constructor_key_chirho(name_chirho)? else {
            return Ok(false);
        };
        let Some(scheme_chirho) = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(&key_chirho)
            .filter(|scheme_chirho| scheme_chirho.promoted_chirho && !scheme_chirho.held_chirho)
            .cloned()
        else {
            return Ok(false);
        };
        let (binders_chirho, _, _) = self.instantiate_header_chirho(&scheme_chirho, span_chirho)?;
        let mut cursor_chirho = 0;
        for argument_chirho in arguments_chirho.into_iter().rev() {
            while binders_chirho
                .get(cursor_chirho)
                .is_some_and(|binder_chirho| {
                    binder_chirho.visibility_chirho == TelescopeVisibilityChirho::InferredChirho
                })
            {
                cursor_chirho += 1;
            }
            let Some(binder_chirho) = binders_chirho.get(cursor_chirho).filter(|binder_chirho| {
                binder_chirho.visibility_chirho == TelescopeVisibilityChirho::SpecifiedChirho
            }) else {
                return Err(Self::error_chirho(
                    "no specified constructor type binder remains for written type application",
                    argument_chirho.span_chirho(),
                ));
            };
            let typed_chirho = self.infer_type_chirho(argument_chirho)?;
            self.state_chirho.unify_chirho(
                &typed_chirho.kind_chirho,
                &binder_chirho.kind_chirho,
                argument_chirho.span_chirho(),
            )?;
            self.state_chirho.unify_chirho(
                &TyChirho::VarChirho(binder_chirho.variable_chirho),
                &typed_chirho.term_chirho,
                argument_chirho.span_chirho(),
            )?;
            // This is the already checked kind, not another inference pass in
            // the term converter. It shares this module's owner and supply.
            let argument_span_chirho = argument_chirho.span_chirho();
            let kind_chirho =
                self.surface_chirho(&typed_chirho.kind_chirho, argument_span_chirho)?;
            let original_chirho = std::mem::replace(
                argument_chirho,
                TypeChirho::WildcardChirho {
                    span_chirho: argument_span_chirho,
                },
            );
            *argument_chirho = TypeChirho::KindAnnotationChirho {
                ty_chirho: Box::new(original_chirho),
                kind_chirho: Box::new(kind_chirho),
                span_chirho: argument_span_chirho,
            };
            cursor_chirho += 1;
        }
        Ok(true)
    }

    /// Pattern type binders use the actual constructor's checked telescope.
    /// No term field is consumed and no runtime dictionary route is activated.
    pub(super) fn elaborate_constructor_pattern_types_chirho(
        &mut self,
        constructor_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
        arguments_chirho: &mut [haskelujah_ast_chirho::pat_chirho::ConstructorTypePatternChirho],
    ) -> Result<(), ElaborationErrorChirho> {
        if arguments_chirho.is_empty() {
            return Ok(());
        }
        let span_chirho = constructor_chirho.span_chirho();
        let scheme_chirho = if let Some(key_chirho) =
            self.constructor_key_chirho(constructor_chirho)?
        {
            self.registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(&key_chirho)
                .filter(|scheme_chirho| scheme_chirho.promoted_chirho && !scheme_chirho.held_chirho)
                .cloned()
        } else {
            None
        };
        let binders_chirho = if let Some(scheme_chirho) = &scheme_chirho {
            Some(
                self.instantiate_header_chirho(scheme_chirho, span_chirho)?
                    .0,
            )
        } else {
            None
        };
        let mut cursor_chirho = 0;
        for pattern_chirho in arguments_chirho {
            let argument_chirho = &mut pattern_chirho.ty_chirho;
            let expected_chirho = if let (Some(scheme_chirho), Some(binders_chirho)) =
                (&scheme_chirho, &binders_chirho)
            {
                let mut sources_chirho = scheme_chirho
                    .source_variables_chirho
                    .iter()
                    .collect::<Vec<_>>();
                self.state_chirho.charge_chirho(
                    sources_chirho.len(),
                    sources_chirho.len(),
                    span_chirho,
                )?;
                sources_chirho.sort_by_key(|source_chirho| source_chirho.header_index_chirho);
                while sources_chirho.get(cursor_chirho).is_some_and(|source_chirho| source_chirho.specificity_chirho == haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho) { cursor_chirho += 1; }
                if sources_chirho.is_empty() {
                    None
                } else {
                    let source_chirho = sources_chirho.get(cursor_chirho).ok_or_else(|| {
                        Self::error_chirho(
                            "no source constructor type binder remains",
                            argument_chirho.span_chirho(),
                        )
                    })?;
                    let position_chirho = scheme_chirho
                        .binders_chirho
                        .iter()
                        .position(|binder_chirho| {
                            binder_chirho.variable_chirho == source_chirho.variable_chirho
                        })
                        .ok_or_else(|| {
                            Self::error_chirho(
                                "checked constructor source binder has no telescope slot",
                                span_chirho,
                            )
                        })?;
                    // instantiate_header preserves this exact typed telescope's
                    // order. Source-written order comes solely from its mapping.
                    pattern_chirho.checked_binder_chirho =
                        Some(source_chirho.source_chirho.clone());
                    Some(&binders_chirho[position_chirho])
                }
            } else {
                None
            };
            let kind_chirho = if let Some(binder_chirho) = expected_chirho {
                self.state_chirho
                    .copy_chirho(&binder_chirho.kind_chirho, span_chirho)?
            } else {
                self.fresh_kind_chirho(argument_chirho.span_chirho())?
            };
            let argument_term_chirho =
                self.bind_pattern_type_argument_chirho(argument_chirho, &kind_chirho)?;
            if let Some(binder_chirho) = expected_chirho {
                self.state_chirho.unify_chirho(
                    &TyChirho::VarChirho(binder_chirho.variable_chirho),
                    &argument_term_chirho,
                    argument_chirho.span_chirho(),
                )?;
            }
            cursor_chirho += 1;
        }
        Ok(())
    }

    fn bind_pattern_type_argument_chirho(
        &mut self,
        argument_chirho: &mut TypeChirho,
        expected_chirho: &TyChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let span_chirho = argument_chirho.span_chirho();
        self.with_syntax_depth_chirho(span_chirho, |this_chirho| match argument_chirho {
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                this_chirho.bind_pattern_type_argument_chirho(inner_chirho, expected_chirho)
            }
            TypeChirho::KindAnnotationChirho {
                ty_chirho,
                kind_chirho,
                ..
            } => {
                let annotation_chirho = this_chirho
                    .scoped_chirho(|this_chirho| this_chirho.infer_type_chirho(kind_chirho))?;
                this_chirho.state_chirho.unify_chirho(
                    &annotation_chirho.kind_chirho,
                    &ElaborationStateChirho::type_universe_chirho(),
                    span_chirho,
                )?;
                this_chirho.state_chirho.unify_chirho(
                    &annotation_chirho.term_chirho,
                    expected_chirho,
                    span_chirho,
                )?;
                this_chirho.bind_pattern_type_argument_chirho(ty_chirho, expected_chirho)
            }
            TypeChirho::VarChirho(name_chirho) => {
                let variable_chirho = if let Some(variable_chirho) =
                    this_chirho.check_variable_owner_chirho(name_chirho)?
                {
                    let kind_chirho = this_chirho
                        .state_chirho
                        .variable_kind_chirho(variable_chirho, span_chirho)?;
                    this_chirho.state_chirho.unify_chirho(
                        &kind_chirho,
                        expected_chirho,
                        span_chirho,
                    )?;
                    variable_chirho
                } else {
                    let kind_chirho = this_chirho
                        .state_chirho
                        .copy_chirho(expected_chirho, span_chirho)?;
                    this_chirho
                        .state_chirho
                        .fresh_chirho(Some(kind_chirho), span_chirho)?
                };
                this_chirho.stamp_variable_chirho(name_chirho, variable_chirho);
                let name_chirho = this_chirho
                    .state_chirho
                    .name_chirho(name_chirho.text_chirho(), span_chirho)?;
                let kind_chirho = this_chirho
                    .state_chirho
                    .copy_chirho(expected_chirho, span_chirho)?;
                this_chirho.bind_variable_chirho(
                    name_chirho,
                    variable_chirho,
                    kind_chirho,
                    span_chirho,
                )?;
                Ok(TyChirho::VarChirho(variable_chirho))
            }
            TypeChirho::WildcardChirho { .. } => {
                let kind_chirho = this_chirho
                    .state_chirho
                    .copy_chirho(expected_chirho, span_chirho)?;
                Ok(TyChirho::VarChirho(
                    this_chirho
                        .state_chirho
                        .fresh_chirho(Some(kind_chirho), span_chirho)?,
                ))
            }
            _ => {
                this_chirho.seed_type_pattern_scope_chirho(argument_chirho)?;
                this_chirho
                    .infer_type_at_kind_chirho(argument_chirho, expected_chirho)
                    .map(|typed_chirho| typed_chirho.term_chirho)
            }
        })
    }
}
