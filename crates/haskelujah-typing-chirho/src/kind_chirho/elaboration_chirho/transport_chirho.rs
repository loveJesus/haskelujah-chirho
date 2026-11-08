// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Owner-checked surface metadata and one canonical declaration identity.

use super::*;
use haskelujah_ast_chirho::name_chirho::{
    CompilerTypeConstructorChirho, NameChirho, RawNameChirho, TypeConstructorDeclarationChirho,
    TypeConstructorIdentityChirho, TypeConstructorRoleChirho,
};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;

/// Both live and canonical projections use the same checked telescope slots.
fn projected_application_chirho(
    function_chirho: TypeChirho,
    argument_chirho: TypeChirho,
    schema_chirho: Option<&DeclarationSchemeChirho>,
    index_chirho: usize,
    span_chirho: SpanChirho,
) -> TypeChirho {
    if schema_chirho
        .and_then(|schema_chirho| schema_chirho.binders_chirho.get(index_chirho))
        .is_some_and(|binder_chirho| {
            binder_chirho.visibility_chirho != TelescopeVisibilityChirho::RequiredChirho
        })
    {
        TypeChirho::KindApplicationChirho {
            fun_chirho: Box::new(function_chirho),
            arg_chirho: Box::new(argument_chirho),
            written_chirho: false,
            span_chirho,
        }
    } else {
        TypeChirho::AppChirho {
            fun_chirho: Box::new(function_chirho),
            arg_chirho: Box::new(argument_chirho),
            span_chirho,
        }
    }
}

impl TypeElaboratorChirho<'_> {
    fn projection_schema_chirho(
        &self,
        key_chirho: &str,
        span_chirho: SpanChirho,
    ) -> Result<Option<Arc<DeclarationSchemeChirho>>, ElaborationErrorChirho> {
        let schema_chirho = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(key_chirho)
            .cloned();
        if let Some(schema_chirho) = &schema_chirho {
            if schema_chirho.held_chirho {
                return Err(Self::error_chirho(
                    "cannot project an unresolved declaration",
                    span_chirho,
                ));
            }
            if schema_chirho.owner_chirho.is_some()
                && schema_chirho.owner_chirho != self.state_chirho.owner_chirho
            {
                return Err(Self::error_chirho(
                    "cannot project a foreign declaration owner",
                    span_chirho,
                ));
            }
            if schema_chirho
                .declaration_chirho
                .as_ref()
                .is_some_and(|proof_chirho| {
                    matches!(&proof_chirho.identity_chirho,
                    TypeConstructorIdentityChirho::CompilerChirho(constructor_chirho)
                    if constructor_chirho.scalar_representation_chirho().is_some())
                })
            {
                // A flat legacy key carries no occurrence's naming proof.
                // Actual checked AST producers retain their complete kind.
                return Ok(None);
            }
        }
        Ok(schema_chirho)
    }

    pub(super) fn copy_name_chirho(
        &mut self,
        name_chirho: &NameChirho,
    ) -> Result<NameChirho, ElaborationErrorChirho> {
        let mut weight_chirho = name_chirho
            .text_chirho()
            .len()
            .saturating_add(name_chirho.qualifier_chirho().map_or(0, str::len))
            .saturating_add(name_chirho.family_key_chirho().map_or(0, str::len))
            .saturating_add(1);
        if let Some(promotion_chirho) = name_chirho.promotion_chirho() {
            weight_chirho =
                weight_chirho.saturating_add(promotion_chirho.key_chirho().as_str_chirho().len());
        }
        if let Some(declaration_chirho) = name_chirho.type_constructor_chirho()
            && let TypeConstructorIdentityChirho::SourceChirho {
                defining_module_chirho,
                name_chirho,
            } = &declaration_chirho.identity_chirho
        {
            weight_chirho = weight_chirho
                .saturating_add(defining_module_chirho.len())
                .saturating_add(name_chirho.len());
        }
        self.state_chirho
            .charge_chirho(weight_chirho, weight_chirho, name_chirho.span_chirho())?;
        Ok(name_chirho.clone())
    }

    pub(super) fn written_name_chirho(
        &mut self,
        name_chirho: &NameChirho,
    ) -> Result<String, ElaborationErrorChirho> {
        let bytes_chirho = name_chirho
            .text_chirho()
            .len()
            .checked_add(
                name_chirho
                    .qualifier_chirho()
                    .map_or(0, |qualifier_chirho| qualifier_chirho.len() + 1),
            )
            .ok_or_else(|| {
                Self::error_chirho("type name size overflow", name_chirho.span_chirho())
            })?;
        self.state_chirho.charge_chirho(
            bytes_chirho + 1,
            bytes_chirho,
            name_chirho.span_chirho(),
        )?;
        Ok(name_chirho.full_name_chirho())
    }

    pub(super) fn declaration_chirho(
        &mut self,
        name_chirho: &NameChirho,
    ) -> Result<Option<TypeConstructorDeclarationChirho>, ElaborationErrorChirho> {
        let written_chirho = self.written_name_chirho(name_chirho)?;
        if name_chirho.type_constructor_opaque_chirho() {
            return Ok(None);
        }
        let declaration_chirho = name_chirho
            .type_constructor_chirho()
            .or_else(|| self.type_scope_chirho.declaration_chirho(&written_chirho));
        let Some(declaration_chirho) = declaration_chirho else {
            return Ok(None);
        };
        let bytes_chirho = match &declaration_chirho.identity_chirho {
            TypeConstructorIdentityChirho::SourceChirho {
                defining_module_chirho,
                name_chirho,
            } => defining_module_chirho
                .len()
                .saturating_add(name_chirho.len()),
            TypeConstructorIdentityChirho::CompilerChirho(_) => 1,
        };
        self.state_chirho.charge_chirho(
            bytes_chirho + 1,
            bytes_chirho + 1,
            name_chirho.span_chirho(),
        )?;
        Ok(Some(declaration_chirho.clone()))
    }

    pub(super) fn declaration_key_chirho(
        &mut self,
        declaration_chirho: &TypeConstructorDeclarationChirho,
        span_chirho: SpanChirho,
    ) -> Result<Arc<str>, ElaborationErrorChirho> {
        let (module_bytes_chirho, name_bytes_chirho) = match &declaration_chirho.identity_chirho {
            TypeConstructorIdentityChirho::SourceChirho {
                defining_module_chirho,
                name_chirho,
            } => (defining_module_chirho.len(), name_chirho.len()),
            TypeConstructorIdentityChirho::CompilerChirho(constructor_chirho) => {
                (0, constructor_chirho.name_chirho().len())
            }
        };
        self.state_chirho.charge_chirho(
            module_bytes_chirho + name_bytes_chirho + 128,
            module_bytes_chirho + name_bytes_chirho + 128,
            span_chirho,
        )?;
        let key_chirho = declaration_chirho.type_key_chirho();
        Ok(Arc::from(key_chirho))
    }

    pub(super) fn stamp_variable_chirho(
        &self,
        name_chirho: &mut NameChirho,
        variable_chirho: TyVarChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let owner_chirho = self
            .state_chirho
            .owner_id_chirho(name_chirho.span_chirho())?;
        let lexical_chirho = self
            .variables_chirho
            .get(name_chirho.text_chirho())
            .filter(|record_chirho| record_chirho.variable_chirho == variable_chirho);
        let source_chirho = self
            .lexical_images_chirho
            .and_then(|images_chirho| images_chirho.lookup_image_chirho(&variable_chirho))
            .and_then(|image_chirho| image_chirho.source_chirho.as_ref())
            .or_else(|| {
                lexical_chirho.and_then(|record_chirho| record_chirho.source_chirho.as_ref())
            });
        name_chirho.set_type_variable_chirho(Some(source_chirho.cloned().unwrap_or(
            ScopedTypeVariableIdChirho::InferenceChirho {
                owner_chirho,
                variable_chirho,
            },
        )));
        Ok(())
    }

    pub(super) fn check_variable_owner_chirho(
        &self,
        name_chirho: &NameChirho,
    ) -> Result<Option<TyVarChirho>, ElaborationErrorChirho> {
        // A source origin can occur in two nested instantiations of one
        // synonym. Select the actual lexical binding first, then demand its
        // exact checked proof; an origin-only map conflates those binders.
        if name_chirho.qualifier_chirho().is_none()
            && let Some(source_chirho) = name_chirho.type_variable_chirho()
            && let Some(record_chirho) = self.variables_chirho.get(name_chirho.text_chirho())
            && self
                .lexical_images_chirho
                .and_then(|images_chirho| {
                    images_chirho.lookup_image_chirho(&record_chirho.variable_chirho)
                })
                .and_then(|image_chirho| image_chirho.source_chirho.as_ref())
                .or(record_chirho.source_chirho.as_ref())
                == Some(source_chirho)
        {
            return Ok(Some(record_chirho.variable_chirho));
        }
        match name_chirho.type_variable_chirho() {
            Some(ScopedTypeVariableIdChirho::InferenceChirho {
                owner_chirho,
                variable_chirho,
            }) if Some(*owner_chirho) == self.state_chirho.owner_chirho => {
                Ok(Some(*variable_chirho))
            }
            Some(ScopedTypeVariableIdChirho::InferenceChirho { .. }) => Err(Self::error_chirho(
                "foreign unquantified kind/type variable reached module elaboration",
                name_chirho.span_chirho(),
            )),
            Some(ScopedTypeVariableIdChirho::QuantifiedChirho { .. }) => Err(Self::error_chirho(
                "exported quantifier requires its declaration telescope",
                name_chirho.span_chirho(),
            )),
            None => Ok(None),
        }
    }

    /// Generated kind syntax carries real owner-qualified metas. Its spelling
    /// is for people only; no converter recovers a variable from this text.
    pub(super) fn surface_chirho(
        &mut self,
        term_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<TypeChirho, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        if self.type_depth_chirho >= 128 {
            return Err(self.state_chirho.resource_failure_chirho(
                "kind syntax projection depth bound exhausted",
                span_chirho,
            ));
        }
        self.type_depth_chirho += 1;
        let result_chirho = self.surface_inner_chirho(term_chirho, span_chirho);
        self.type_depth_chirho -= 1;
        result_chirho
    }

    fn surface_inner_chirho(
        &mut self,
        term_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<TypeChirho, ElaborationErrorChirho> {
        let term_chirho = self.state_chirho.resolve_chirho(term_chirho, span_chirho)?;
        let required_forall_chirho = matches!(&term_chirho, TyChirho::RequiredForallChirho { .. });
        Ok(match term_chirho {
            TyChirho::VarChirho(variable_chirho) => {
                self.state_chirho.charge_chirho(32, 32, span_chirho)?;
                let mut name_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                    format!("_kind{}Chirho", variable_chirho.0),
                    span_chirho,
                ));
                self.stamp_variable_chirho(&mut name_chirho, variable_chirho)?;
                TypeChirho::VarChirho(name_chirho)
            }
            TyChirho::ConChirho(key_chirho) => {
                let text_chirho = self.state_chirho.name_chirho(&key_chirho, span_chirho)?;
                let mut name_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                    text_chirho,
                    span_chirho,
                ));
                if key_chirho == state_chirho::TYPE_UNIVERSE_KEY_CHIRHO {
                    name_chirho.set_type_constructor_chirho(Some(
                        TypeConstructorDeclarationChirho::compiler_data_chirho(
                            CompilerTypeConstructorChirho::TypeUniverseChirho,
                        ),
                    ));
                }
                if key_chirho == state_chirho::CONSTRAINT_UNIVERSE_KEY_CHIRHO {
                    name_chirho.set_type_constructor_chirho(Some(
                        TypeConstructorDeclarationChirho::compiler_data_chirho(
                            CompilerTypeConstructorChirho::ConstraintUniverseChirho,
                        ),
                    ));
                }
                if let Some(scheme_chirho) =
                    self.projection_schema_chirho(&key_chirho, span_chirho)?
                {
                    if let Some(promoted_chirho) = &scheme_chirho.promoted_chirho {
                        if scheme_chirho.held_chirho {
                            return Err(Self::error_chirho(
                                "cannot project an unresolved promoted declaration",
                                span_chirho,
                            ));
                        }
                        self.state_chirho.charge_chirho(
                            promoted_chirho.as_str_chirho().len() + 1,
                            promoted_chirho.as_str_chirho().len() + 1,
                            span_chirho,
                        )?;
                        name_chirho.set_promotion_chirho(haskelujah_ast_chirho::promotion_chirho::PromotionResolutionChirho::ResolvedChirho(promoted_chirho.clone()));
                        return Ok(TypeChirho::PromotedConChirho {
                            name_chirho,
                            span_chirho,
                        });
                    }
                    if let Some(declaration_chirho) = &scheme_chirho.declaration_chirho {
                        let bytes_chirho = match &declaration_chirho.identity_chirho {
                            TypeConstructorIdentityChirho::SourceChirho {
                                defining_module_chirho,
                                name_chirho,
                            } => defining_module_chirho
                                .len()
                                .saturating_add(name_chirho.len()),
                            TypeConstructorIdentityChirho::CompilerChirho(_) => 1,
                        };
                        self.state_chirho.charge_chirho(
                            bytes_chirho + 1,
                            bytes_chirho + 1,
                            span_chirho,
                        )?;
                        name_chirho.set_type_constructor_chirho(Some(declaration_chirho.clone()));
                    }
                    if matches!(
                        scheme_chirho.role_chirho,
                        TypeConstructorRoleChirho::TypeFamilyChirho
                            | TypeConstructorRoleChirho::DataFamilyChirho
                    ) {
                        name_chirho.set_family_key_chirho(
                            self.state_chirho.name_chirho(&key_chirho, span_chirho)?,
                        );
                    }
                }
                if name_chirho.type_constructor_chirho().is_none() {
                    name_chirho.mark_type_constructor_opaque_chirho();
                }
                TypeChirho::ConChirho(name_chirho)
            }
            TyChirho::AppChirho(_, _) => {
                let mut head_chirho = &term_chirho;
                let mut arguments_chirho = Vec::new();
                while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
                    self.state_chirho.charge_chirho(1, 1, span_chirho)?;
                    if self
                        .type_depth_chirho
                        .saturating_add(arguments_chirho.len())
                        >= 128
                    {
                        return Err(self.state_chirho.resource_failure_chirho(
                            "kind syntax spine depth bound exhausted",
                            span_chirho,
                        ));
                    }
                    arguments_chirho.push(argument_chirho.as_ref());
                    head_chirho = function_chirho;
                }
                arguments_chirho.reverse();
                let schema_chirho = if let TyChirho::ConChirho(key_chirho) = head_chirho {
                    self.projection_schema_chirho(key_chirho, span_chirho)?
                } else {
                    None
                };
                let mut result_chirho = self.surface_chirho(head_chirho, span_chirho)?;
                for (index_chirho, argument_chirho) in arguments_chirho.into_iter().enumerate() {
                    let argument_chirho = self.surface_chirho(argument_chirho, span_chirho)?;
                    result_chirho = projected_application_chirho(
                        result_chirho,
                        argument_chirho,
                        schema_chirho.as_deref(),
                        index_chirho,
                        span_chirho,
                    );
                }
                result_chirho
            }
            TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => TypeChirho::FunChirho {
                arg_chirho: Box::new(self.surface_chirho(&arg_chirho, span_chirho)?),
                mult_chirho: Some(match mult_chirho {
                    crate::ty_chirho::MultChirho::OneChirho => {
                        haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho
                    }
                    crate::ty_chirho::MultChirho::ManyChirho => {
                        haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::ManyChirho
                    }
                }),
                result_chirho: Box::new(self.surface_chirho(&result_chirho, span_chirho)?),
                span_chirho,
            },
            TyChirho::TupleChirho(elements_chirho) => TypeChirho::TupleChirho {
                elements_chirho: elements_chirho
                    .iter()
                    .map(|element_chirho| self.surface_chirho(element_chirho, span_chirho))
                    .collect::<Result<_, _>>()?,
                span_chirho,
            },
            TyChirho::ListChirho(element_chirho) => TypeChirho::ListChirho {
                element_chirho: Box::new(self.surface_chirho(&element_chirho, span_chirho)?),
                span_chirho,
            },
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                let required_chirho = required_forall_chirho;
                self.state_chirho.charge_chirho(
                    vars_chirho.len(),
                    vars_chirho.len(),
                    span_chirho,
                )?;
                let mut binders_chirho = Vec::with_capacity(vars_chirho.len());
                for variable_chirho in vars_chirho.iter().copied() {
                    let TypeChirho::VarChirho(name_chirho) =
                        self.surface_chirho(&TyChirho::VarChirho(variable_chirho), span_chirho)?
                    else {
                        return Err(Self::error_chirho(
                            "universal kind projection lost its checked variable",
                            span_chirho,
                        ));
                    };
                    let kind_chirho = self
                        .state_chirho
                        .variable_kind_chirho(variable_chirho, span_chirho)?;
                    let kind_chirho = self.surface_chirho(&kind_chirho, span_chirho)?;
                    let specificity_chirho = self
                        .state_chirho
                        .source_specificity_chirho(variable_chirho)
                        .ok_or_else(|| {
                            Self::error_chirho(
                                "universal kind projection lost its specificity",
                                span_chirho,
                            )
                        })?;
                    binders_chirho.push(haskelujah_ast_chirho::decl_chirho::TyVarChirho { name_chirho, kind_annotation_chirho: Some(Box::new(kind_chirho)), visibility_chirho: if required_chirho { haskelujah_ast_chirho::decl_chirho::TyVarVisibilityChirho::VisibleChirho } else { haskelujah_ast_chirho::decl_chirho::TyVarVisibilityChirho::InvisibleChirho }, specificity_chirho });
                }
                let body_chirho = Box::new(self.surface_chirho(&body_chirho, span_chirho)?);
                if required_chirho {
                    TypeChirho::RequiredForallChirho {
                        vars_chirho: binders_chirho,
                        body_chirho,
                        span_chirho,
                    }
                } else {
                    TypeChirho::ForallChirho {
                        vars_chirho: binders_chirho,
                        body_chirho,
                        span_chirho,
                    }
                }
            }
            TyChirho::QualChirho { .. } | TyChirho::ForallVarChirho(_) => {
                return Err(Self::error_chirho(
                    "a quantified or rigid kind escaped its declaration telescope",
                    span_chirho,
                ));
            }
        })
    }
}

impl TypeElaboratorChirho<'_> {
    /// Project an already canonical solver solution without reinterpreting its
    /// slots as this module's owner IDs. Hidden prefix syntax follows the same
    /// checked telescope used by elaboration and synonym reduction.
    pub(super) fn canonical_surface_chirho(
        &mut self,
        term_chirho: &TyChirho,
        identity_chirho: &Arc<str>,
        allowed_chirho: &std::collections::HashSet<TyVarChirho>,
        span_chirho: SpanChirho,
        depth_chirho: usize,
    ) -> Result<TypeChirho, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        if depth_chirho > 128 {
            return Err(self.state_chirho.resource_failure_chirho(
                "canonical syntax projection depth bound exhausted",
                span_chirho,
            ));
        }
        Ok(match term_chirho {
            TyChirho::VarChirho(variable_chirho) => {
                if !allowed_chirho.contains(variable_chirho) {
                    return Err(Self::error_chirho(
                        "canonical syntax solution contains an unbound slot",
                        span_chirho,
                    ));
                }
                self.state_chirho.charge_chirho(64, 64, span_chirho)?;
                let mut name_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                    format!("published{}Chirho", variable_chirho.0),
                    span_chirho,
                ));
                name_chirho.set_type_variable_chirho(Some(
                    ScopedTypeVariableIdChirho::QuantifiedChirho {
                        declaration_chirho: identity_chirho.clone(),
                        index_chirho: variable_chirho.0,
                    },
                ));
                TypeChirho::VarChirho(name_chirho)
            }
            TyChirho::ConChirho(_) => self.surface_chirho(term_chirho, span_chirho)?,
            TyChirho::AppChirho(_, _) => {
                let mut head_chirho = term_chirho;
                let mut arguments_chirho = Vec::new();
                while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
                    self.state_chirho.charge_chirho(1, 1, span_chirho)?;
                    if depth_chirho.saturating_add(arguments_chirho.len()) >= 128 {
                        return Err(self.state_chirho.resource_failure_chirho(
                            "canonical syntax spine depth bound exhausted",
                            span_chirho,
                        ));
                    }
                    arguments_chirho.push(argument_chirho.as_ref());
                    head_chirho = function_chirho;
                }
                arguments_chirho.reverse();
                let schema_chirho = if let TyChirho::ConChirho(key_chirho) = head_chirho {
                    self.projection_schema_chirho(key_chirho, span_chirho)?
                } else {
                    None
                };
                let mut result_chirho = self.canonical_surface_chirho(
                    head_chirho,
                    identity_chirho,
                    allowed_chirho,
                    span_chirho,
                    depth_chirho + arguments_chirho.len(),
                )?;
                for (index_chirho, argument_chirho) in arguments_chirho.iter().enumerate() {
                    let argument_chirho = Box::new(self.canonical_surface_chirho(
                        argument_chirho,
                        identity_chirho,
                        allowed_chirho,
                        span_chirho,
                        depth_chirho + arguments_chirho.len() - index_chirho,
                    )?);
                    result_chirho = projected_application_chirho(
                        result_chirho,
                        *argument_chirho,
                        schema_chirho.as_deref(),
                        index_chirho,
                        span_chirho,
                    );
                }
                result_chirho
            }
            TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
                TypeChirho::FunChirho {
                    arg_chirho: Box::new(self.canonical_surface_chirho(
                        argument_chirho,
                        identity_chirho,
                        allowed_chirho,
                        span_chirho,
                        depth_chirho + 1,
                    )?),
                    mult_chirho: if *multiplicity_chirho == crate::ty_chirho::MultChirho::OneChirho
                    {
                        Some(haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho)
                    } else {
                        None
                    },
                    result_chirho: Box::new(self.canonical_surface_chirho(
                        result_chirho,
                        identity_chirho,
                        allowed_chirho,
                        span_chirho,
                        depth_chirho + 1,
                    )?),
                    span_chirho,
                }
            }
            TyChirho::TupleChirho(elements_chirho) => {
                self.state_chirho.charge_chirho(
                    elements_chirho.len(),
                    elements_chirho.len(),
                    span_chirho,
                )?;
                TypeChirho::TupleChirho {
                    elements_chirho: elements_chirho
                        .iter()
                        .map(|element_chirho| {
                            self.canonical_surface_chirho(
                                element_chirho,
                                identity_chirho,
                                allowed_chirho,
                                span_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                    span_chirho,
                }
            }
            TyChirho::ListChirho(element_chirho) => TypeChirho::ListChirho {
                element_chirho: Box::new(self.canonical_surface_chirho(
                    element_chirho,
                    identity_chirho,
                    allowed_chirho,
                    span_chirho,
                    depth_chirho + 1,
                )?),
                span_chirho,
            },
            TyChirho::ForallChirho { .. }
            | TyChirho::RequiredForallChirho { .. }
            | TyChirho::QualChirho { .. }
            | TyChirho::ForallVarChirho(_) => {
                return Err(Self::error_chirho(
                    "canonical solution requires retained checked binder syntax",
                    span_chirho,
                ));
            }
        })
    }
}
