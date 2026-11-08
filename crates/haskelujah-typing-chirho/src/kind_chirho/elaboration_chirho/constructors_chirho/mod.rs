// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor field types and promoted telescopes derive from the same source
//! declaration and naming proof. Runtime packing remains a later consumer.

use super::*;
mod origins_chirho;
mod prefixes_chirho;
use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DeclChirho, GadtConArgsChirho, GadtConPrefixChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, TypeConstructorRoleChirho};
pub(super) use origins_chirho::{ConstructorResultOriginChirho, SignatureCheckingChirho};

impl TypeElaboratorChirho<'_> {
    /// Check the completed producer result, rather than specializing a written
    /// universal to satisfy datatype validity. Data return kinds require the
    /// actual BoxedRep constructor even when a family could reduce to it;
    /// newtype results may retain arbitrary checked TYPE representations.
    pub(super) fn check_datatype_result_roles_chirho(
        &mut self,
        keys_chirho: &[Arc<str>],
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        use haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho;
        use haskelujah_ast_chirho::promotion_chirho::{
            BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, PromotedKeyChirho,
        };

        for key_chirho in keys_chirho {
            self.state_chirho.charge_chirho(1, 0, span_chirho)?;
            let Some(role_chirho) = self
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(key_chirho)
                .map(|scheme_chirho| scheme_chirho.role_chirho)
            else {
                continue;
            };
            let enabled_chirho = match role_chirho {
                TypeConstructorRoleChirho::DataChirho => self.unlifted_datatypes_chirho,
                TypeConstructorRoleChirho::NewtypeChirho => self.unlifted_newtypes_chirho,
                _ => continue,
            };
            let kind_chirho = self.result_kind_chirho(key_chirho, span_chirho)?;
            let kind_chirho = self
                .state_chirho
                .resolve_chirho(&kind_chirho, span_chirho)?;
            let mut tail_chirho = &kind_chirho;
            while let TyChirho::FunChirho(_, body_chirho, _)
            | TyChirho::ForallChirho { body_chirho, .. }
            | TyChirho::RequiredForallChirho { body_chirho, .. } = tail_chirho
            {
                self.state_chirho.charge_chirho(1, 0, span_chirho)?;
                tail_chirho = body_chirho;
            }
            if *tail_chirho == ElaborationStateChirho::type_universe_chirho() {
                continue;
            }
            let allowed_chirho = if enabled_chirho {
                match tail_chirho {
                    TyChirho::AppChirho(function_chirho, representation_chirho)
                        if matches!(&**function_chirho, TyChirho::ConChirho(key_chirho)
                            if key_chirho == CompilerRepresentationChirho::RuntimeTypeChirho.type_key_chirho()) =>
                    {
                        role_chirho == TypeConstructorRoleChirho::NewtypeChirho
                            || matches!(&**representation_chirho,
                                TyChirho::AppChirho(boxed_chirho, _)
                                    if matches!(&**boxed_chirho, TyChirho::ConChirho(key_chirho)
                                        if key_chirho == PromotedKeyChirho::builtin_chirho(
                                            BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, "BoxedRep").as_str_chirho()))
                    }
                    _ => false,
                }
            } else {
                false
            };
            if !allowed_chirho {
                return Err(Self::error_chirho(
                    match (role_chirho, enabled_chirho) {
                        (TypeConstructorRoleChirho::DataChirho, true) => {
                            "data type return kind must be TYPE with a BoxedRep representation"
                        }
                        (TypeConstructorRoleChirho::DataChirho, false) => {
                            "data type return kind requires UnliftedDatatypes"
                        }
                        _ => "newtype return kind requires UnliftedNewtypes",
                    },
                    span_chirho,
                ));
            }
        }
        Ok(())
    }

    pub(super) fn queue_declaration_field_kinds_chirho(
        &mut self,
        keys_chirho: &[Arc<str>],
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        for key_chirho in keys_chirho {
            let fixed_chirho = self
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(key_chirho)
                .is_some_and(|scheme_chirho| {
                    scheme_chirho.role_chirho == TypeConstructorRoleChirho::DataChirho
                });
            let Some(variables_chirho) = self.declaration_variables_chirho.get_mut(key_chirho)
            else {
                continue;
            };
            let constructors_chirho =
                std::mem::take(&mut variables_chirho.fixed_field_kinds_chirho);
            if !fixed_chirho {
                continue;
            }
            for fields_chirho in constructors_chirho {
                let fields_chirho = fields_chirho.ok_or_else(|| {
                    Self::error_chirho(
                        "data field validity lost its checked constructor producer",
                        span_chirho,
                    )
                })?;
                self.state_chirho
                    .queue_constructor_field_kinds_chirho(fields_chirho, span_chirho)?;
            }
        }
        Ok(())
    }

    pub(super) fn publish_constructor_result_chirho(
        &mut self,
        key_chirho: &str,
        term_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let origin_chirho = self
            .declaration_variables_chirho
            .get(key_chirho)
            .and_then(|variables_chirho| variables_chirho.constructor_result_origin_chirho.clone())
            .ok_or_else(|| {
                Self::error_chirho(
                    "promoted constructor lost its result producer contract",
                    span_chirho,
                )
            })?;
        let (parent_chirho, checked_chirho) = match origin_chirho {
            ConstructorResultOriginChirho::SignatureApplicationChirho {
                checking_chirho: SignatureCheckingChirho::ProvisionalChirho,
                ..
            } => {
                return Err(Self::error_chirho(
                    "GADT signature was not checked against its published parent telescope",
                    span_chirho,
                ));
            }
            ConstructorResultOriginChirho::ParentHeaderTemplateChirho { parent_chirho } => {
                (parent_chirho, false)
            }
            ConstructorResultOriginChirho::SignatureApplicationChirho {
                parent_chirho,
                checking_chirho: SignatureCheckingChirho::CompleteChirho,
            } => (parent_chirho, true),
        };
        let mut head_chirho = term_chirho;
        while let TyChirho::AppChirho(function_chirho, _) = head_chirho {
            self.state_chirho.charge_chirho(1, 0, span_chirho)?;
            head_chirho = function_chirho;
        }
        if !matches!(head_chirho, TyChirho::ConChirho(key_chirho) if key_chirho == parent_chirho.as_ref())
        {
            return Err(Self::error_chirho(
                "constructor result names a different defining datatype",
                span_chirho,
            ));
        }
        if checked_chirho {
            self.state_chirho.copy_chirho(term_chirho, span_chirho)
        } else {
            self.promoted_result_prefix_chirho(term_chirho, span_chirho)
        }
    }

    pub(super) fn promoted_result_prefix_chirho(
        &mut self,
        term_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let mut head_chirho = term_chirho;
        let mut arguments_chirho = Vec::new();
        while let TyChirho::AppChirho(fun_chirho, argument_chirho) = head_chirho {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            arguments_chirho.push(argument_chirho.as_ref());
            head_chirho = fun_chirho;
        }
        arguments_chirho.reverse();
        let TyChirho::ConChirho(key_chirho) = head_chirho else {
            return self.state_chirho.copy_chirho(term_chirho, span_chirho);
        };
        let Some(parent_chirho) = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(key_chirho)
            .cloned()
        else {
            return self.state_chirho.copy_chirho(term_chirho, span_chirho);
        };
        if arguments_chirho.len() != parent_chirho.source_variables_chirho.len() {
            return Err(Self::error_chirho(
                "promoted result does not match its parent declaration boundary",
                span_chirho,
            ));
        }
        let mut mapping_chirho = crate::subst_chirho::SubstChirho::empty_chirho();
        for source_chirho in &parent_chirho.source_variables_chirho {
            let argument_chirho = arguments_chirho
                .get(source_chirho.header_index_chirho as usize)
                .ok_or_else(|| {
                    Self::error_chirho("missing promoted result argument", span_chirho)
                })?;
            mapping_chirho.insert_chirho(
                source_chirho.variable_chirho,
                self.state_chirho
                    .copy_chirho(argument_chirho, span_chirho)?,
            );
            if let TyChirho::VarChirho(variable_chirho) = argument_chirho {
                let actual_kind_chirho = self
                    .state_chirho
                    .variable_kind_chirho(*variable_chirho, span_chirho)?;
                let actual_kind_chirho = self
                    .state_chirho
                    .resolve_chirho(&actual_kind_chirho, span_chirho)?;
                if let Some(binder_chirho) =
                    parent_chirho.binders_chirho.iter().find(|binder_chirho| {
                        binder_chirho.variable_chirho == source_chirho.variable_chirho
                    })
                {
                    crate::ty_chirho::declarations_chirho::map_telescope_kind_variables_chirho(
                        &binder_chirho.kind_chirho,
                        &actual_kind_chirho,
                        &mut mapping_chirho,
                    );
                }
            }
        }
        let mut result_chirho = self.state_chirho.copy_chirho(head_chirho, span_chirho)?;
        for binder_chirho in &parent_chirho.binders_chirho {
            let argument_chirho = if let Some(argument_chirho) =
                mapping_chirho.lookup_chirho(&binder_chirho.variable_chirho)
            {
                self.state_chirho
                    .copy_chirho(argument_chirho, span_chirho)?
            } else {
                let kind_chirho = mapping_chirho.apply_ty_chirho(&binder_chirho.kind_chirho);
                let variable_chirho = self
                    .state_chirho
                    .fresh_chirho(Some(kind_chirho), span_chirho)?;
                let argument_chirho = TyChirho::VarChirho(variable_chirho);
                mapping_chirho
                    .insert_chirho(binder_chirho.variable_chirho, argument_chirho.clone());
                argument_chirho
            };
            result_chirho = TyChirho::AppChirho(Box::new(result_chirho), Box::new(argument_chirho));
        }
        Ok(result_chirho)
    }
    pub(super) fn constructor_key_chirho(
        &mut self,
        name_chirho: &NameChirho,
    ) -> Result<Option<Arc<str>>, ElaborationErrorChirho> {
        Ok(self
            .constructor_promotion_chirho(name_chirho)?
            .map(|key_chirho| Arc::from(key_chirho.as_str_chirho())))
    }

    fn constructor_promotion_chirho(
        &mut self,
        name_chirho: &NameChirho,
    ) -> Result<
        Option<haskelujah_ast_chirho::promotion_chirho::PromotedKeyChirho>,
        ElaborationErrorChirho,
    > {
        let resolution_chirho = self.promotion_scope_chirho.quoted_chirho(name_chirho);
        let key_chirho = resolution_chirho.key_chirho().as_str_chirho();
        if haskelujah_ast_chirho::promotion_chirho::is_unknown_promoted_key_chirho(key_chirho) {
            return Ok(None);
        }
        self.state_chirho.charge_chirho(
            key_chirho.len() + 1,
            key_chirho.len() + 1,
            name_chirho.span_chirho(),
        )?;
        Ok(Some(match resolution_chirho {
            haskelujah_ast_chirho::promotion_chirho::PromotionResolutionChirho::ResolvedChirho(key_chirho)
            | haskelujah_ast_chirho::promotion_chirho::PromotionResolutionChirho::UnresolvedChirho(key_chirho) => key_chirho,
        }))
    }

    pub(super) fn predeclare_constructors_chirho(
        &mut self,
        module_chirho: &ModuleChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let mut snapshot_budget_chirho =
            haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho();
        for declaration_chirho in &module_chirho.decls_chirho {
            let (name_chirho, constructors_chirho) = match declaration_chirho {
                DeclChirho::DataDeclChirho {
                    name_chirho,
                    constructors_chirho,
                    ..
                } => (name_chirho, constructors_chirho.as_slice()),
                DeclChirho::NewtypeDeclChirho {
                    name_chirho,
                    constructor_chirho,
                    ..
                } => (
                    name_chirho,
                    std::slice::from_ref(constructor_chirho.as_ref()),
                ),
                _ => continue,
            };
            let Some(proof_chirho) = self.declaration_chirho(name_chirho)? else {
                continue;
            };
            let parent_key_chirho =
                self.declaration_key_chirho(&proof_chirho, name_chirho.span_chirho())?;
            self.state_chirho.charge_chirho(
                constructors_chirho.len(),
                constructors_chirho.len(),
                name_chirho.span_chirho(),
            )?;
            self.declaration_variables_chirho
                .get_mut(&parent_key_chirho)
                .ok_or_else(|| {
                    Self::error_chirho(
                        "constructor producer lost its live parent",
                        name_chirho.span_chirho(),
                    )
                })?
                .fixed_field_kinds_chirho = vec![None; constructors_chirho.len()];
            let Some(parent_chirho) = self
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(&parent_key_chirho)
                .cloned()
            else {
                continue;
            };
            for constructor_chirho in constructors_chirho {
                let (name_chirho, count_chirho) = match constructor_chirho {
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho,
                        fields_chirho,
                        ..
                    } => (name_chirho, fields_chirho.len()),
                    ConDeclChirho::RecordChirho {
                        name_chirho,
                        fields_chirho,
                        ..
                    } => (
                        name_chirho,
                        fields_chirho
                            .iter()
                            .map(|field_chirho| field_chirho.names_chirho.len())
                            .sum(),
                    ),
                    ConDeclChirho::GadtChirho {
                        name_chirho,
                        signature_chirho,
                        ..
                    } => (
                        name_chirho,
                        signature_chirho.positional_fields_chirho().count(),
                    ),
                };
                let Some(promoted_key_chirho) = self.constructor_promotion_chirho(name_chirho)?
                else {
                    continue;
                };
                let key_chirho: Arc<str> = Arc::from(promoted_key_chirho.as_str_chirho());
                let span_chirho = name_chirho.span_chirho();
                self.state_chirho.charge_chirho(
                    parent_chirho.binders_chirho.len() + count_chirho,
                    parent_chirho.binders_chirho.len() + count_chirho,
                    span_chirho,
                )?;
                let mut binders_chirho = Vec::new();
                let mut result_kind_chirho = TyChirho::ConChirho(
                    self.state_chirho
                        .name_chirho(&parent_key_chirho, span_chirho)?,
                );
                for binder_chirho in &parent_chirho.binders_chirho {
                    binders_chirho.push(DeclarationBinderChirho {
                        term_specificity_chirho: None,
                        variable_chirho: binder_chirho.variable_chirho,
                        kind_chirho: self
                            .state_chirho
                            .copy_chirho(&binder_chirho.kind_chirho, span_chirho)?,
                        visibility_chirho: TelescopeVisibilityChirho::SpecifiedChirho,
                    });
                    result_kind_chirho = TyChirho::AppChirho(
                        Box::new(result_kind_chirho),
                        Box::new(TyChirho::VarChirho(binder_chirho.variable_chirho)),
                    );
                }
                for _ in 0..count_chirho {
                    let field_chirho = self.state_chirho.fresh_chirho(
                        Some(ElaborationStateChirho::type_universe_chirho()),
                        span_chirho,
                    )?;
                    let variable_chirho = self
                        .state_chirho
                        .fresh_chirho(Some(TyChirho::VarChirho(field_chirho)), span_chirho)?;
                    binders_chirho.push(DeclarationBinderChirho {
                        term_specificity_chirho: None,
                        variable_chirho,
                        kind_chirho: TyChirho::VarChirho(field_chirho),
                        visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
                    });
                }
                self.registry_chirho
                    .declaration_schemes_mut_chirho()
                    .stage_chirho(DeclarationSchemeChirho {
            injectivity_chirho: None,
                association_chirho: crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho,
                        identity_chirho: key_chirho.clone(),
                        owner_chirho: self.state_chirho.owner_chirho,
                        declaration_chirho: None,
                        role_chirho: TypeConstructorRoleChirho::DataChirho,
                        reduction_arity_chirho: binders_chirho.len(),
                        visible_arity_chirho: count_chirho,
                        binders_chirho,
                        source_variables_chirho: Vec::new(),
                        result_kind_chirho,
                        synonym_rhs_chirho: None,
                        synonym_syntax_chirho: None,
                        local_binders_chirho: std::sync::Arc::from([]),
                        promoted_chirho: Some(promoted_key_chirho),
                        held_chirho: false,
                    });
                let written_signature_chirho = if let ConDeclChirho::GadtChirho {
                    signature_chirho,
                    ..
                } = constructor_chirho
                {
                    haskelujah_ast_chirho::decl_chirho::check_projection_chirho(
                        signature_chirho,
                        &mut snapshot_budget_chirho,
                        1,
                    )
                    .map_err(|_| {
                        Self::error_chirho(
                            "GADT written signature snapshot resource bound exhausted",
                            span_chirho,
                        )
                    })?;
                    Some(signature_chirho.as_ref().clone())
                } else {
                    None
                };
                let variables_chirho = self
                    .declaration_variables_chirho
                    .entry(key_chirho)
                    .or_default();
                variables_chirho.written_signature_chirho = written_signature_chirho;
                variables_chirho.constructor_result_origin_chirho =
                    Some(ConstructorResultOriginChirho::ParentHeaderTemplateChirho {
                        parent_chirho: parent_key_chirho.clone(),
                    });
            }
        }
        Ok(())
    }

    pub(super) fn replay_gadt_constructor_chirho(
        &mut self,
        constructor_chirho: &mut ConDeclChirho,
        parent_chirho: &str,
        constructor_index_chirho: usize,
    ) -> Result<(), ElaborationErrorChirho> {
        let ConDeclChirho::GadtChirho {
            name_chirho,
            signature_chirho,
            ..
        } = constructor_chirho
        else {
            return Ok(());
        };
        let Some(key_chirho) = self.constructor_key_chirho(name_chirho)? else {
            return Ok(());
        };
        let original_chirho = self
            .declaration_variables_chirho
            .get_mut(&key_chirho)
            .and_then(|variables_chirho| variables_chirho.written_signature_chirho.take())
            .ok_or_else(|| {
                Self::error_chirho(
                    "GADT replay lost its written signature producer",
                    name_chirho.span_chirho(),
                )
            })?;
        **signature_chirho = original_chirho;
        self.elaborate_constructor_chirho(
            constructor_chirho,
            parent_chirho,
            constructor_index_chirho,
            &mut Vec::new(),
        )
    }

    /// A newtype's actual checked result may rename its parameters, but cannot
    /// refine them or hide existential field variables. Data GADTs keep their
    /// ordinary refinement rule. Result-kind applications remain in this spine.
    fn check_newtype_result_parameters_chirho(
        &mut self,
        result_chirho: &TyChirho,
        fields_chirho: &[TyChirho],
        parent_chirho: &str,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let result_chirho = self
            .state_chirho
            .resolve_chirho(result_chirho, span_chirho)?;
        let mut head_chirho = &result_chirho;
        let mut parameters_chirho = std::collections::HashSet::new();
        while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            let TyChirho::VarChirho(variable_chirho) = &**argument_chirho else {
                return Err(Self::error_chirho(
                    "newtype constructor result cannot refine its parent parameters",
                    span_chirho,
                ));
            };
            if !parameters_chirho.insert(*variable_chirho) {
                return Err(Self::error_chirho(
                    "newtype constructor result must have distinct parameter variables",
                    span_chirho,
                ));
            }
            head_chirho = function_chirho;
        }
        if !matches!(head_chirho, TyChirho::ConChirho(key_chirho) if key_chirho == parent_chirho) {
            return Err(Self::error_chirho(
                "newtype constructor result has a different defining type",
                span_chirho,
            ));
        }
        let roots_chirho = parameters_chirho.into_iter().collect::<Vec<_>>();
        let parameters_chirho = self
            .state_chirho
            .kind_dependency_closure_chirho(&roots_chirho, span_chirho)?;
        for field_chirho in fields_chirho {
            let field_chirho = self
                .state_chirho
                .resolve_chirho(field_chirho, span_chirho)?;
            let variables_chirho = field_chirho.free_vars_chirho();
            self.state_chirho
                .charge_chirho(variables_chirho.len(), 0, span_chirho)?;
            if variables_chirho
                .iter()
                .any(|variable_chirho| !parameters_chirho.contains(variable_chirho))
            {
                return Err(Self::error_chirho(
                    "newtype constructor field has an existential type variable",
                    span_chirho,
                ));
            }
        }
        Ok(())
    }

    pub(super) fn elaborate_constructor_chirho(
        &mut self,
        constructor_chirho: &mut ConDeclChirho,
        parent_key_chirho: &str,
        constructor_index_chirho: usize,
        keys_chirho: &mut Vec<Arc<str>>,
    ) -> Result<(), ElaborationErrorChirho> {
        let constructor_span_chirho = match &*constructor_chirho {
            ConDeclChirho::OrdinaryChirho { name_chirho, .. }
            | ConDeclChirho::RecordChirho { name_chirho, .. }
            | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho.span_chirho(),
        };
        let mut universals_chirho = std::collections::BTreeSet::new();
        let newtype_parent_chirho = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(parent_key_chirho)
            .is_some_and(|parent_chirho| {
                parent_chirho.role_chirho == TypeConstructorRoleChirho::NewtypeChirho
            });
        if newtype_parent_chirho {
            let variables_chirho = self
                .declaration_variables_chirho
                .get(parent_key_chirho)
                .ok_or_else(|| {
                    Self::error_chirho(
                        "newtype constructor lost its owner-local parent",
                        constructor_span_chirho,
                    )
                })?;
            self.state_chirho.charge_chirho(
                variables_chirho.written_chirho.len(),
                variables_chirho.written_chirho.len(),
                constructor_span_chirho,
            )?;
            universals_chirho.extend(
                variables_chirho
                    .written_chirho
                    .iter()
                    .map(|(_, variable_chirho, _)| *variable_chirho),
            );
            let result_chirho =
                self.result_kind_chirho(parent_key_chirho, constructor_span_chirho)?;
            let result_chirho = self
                .state_chirho
                .resolve_chirho(&result_chirho, constructor_span_chirho)?;
            let variables_chirho = result_chirho.free_vars_chirho();
            self.state_chirho.charge_chirho(
                variables_chirho.len(),
                variables_chirho.len(),
                constructor_span_chirho,
            )?;
            universals_chirho.extend(variables_chirho.into_iter().filter(|variable_chirho| {
                self.state_chirho.variable_binding_chirho(*variable_chirho)
                    == ElaborationVariableBindingChirho::UniversalChirho
            }));
        }
        if matches!(constructor_chirho, ConDeclChirho::GadtChirho { .. })
            && let Some(parent_chirho) = self
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(parent_key_chirho)
                .cloned()
                .filter(|parent_chirho| parent_chirho.owner_chirho.is_some())
        {
            for binder_chirho in &parent_chirho.binders_chirho {
                let kind_chirho = self
                    .state_chirho
                    .resolve_chirho(&binder_chirho.kind_chirho, constructor_span_chirho)?;
                for variable_chirho in kind_chirho.free_vars_chirho() {
                    if self
                        .state_chirho
                        .source_specificity_chirho(variable_chirho)
                        .is_some()
                        && !parent_chirho
                            .binders_chirho
                            .iter()
                            .any(|binder_chirho| binder_chirho.variable_chirho == variable_chirho)
                    {
                        universals_chirho.insert(variable_chirho);
                    }
                }
            }
        }
        let universals_chirho = universals_chirho.into_iter().collect::<Vec<_>>();
        let rigid_chirho = self
            .state_chirho
            .open_rigid_scope_chirho(&universals_chirho, constructor_span_chirho)?;
        let result_chirho = self.scoped_chirho(|this_chirho| {
            let name_chirho = match constructor_chirho {
                ConDeclChirho::OrdinaryChirho { name_chirho, .. }
                | ConDeclChirho::RecordChirho { name_chirho, .. }
                | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho,
            };
            let span_chirho = name_chirho.span_chirho();
            let key_chirho = this_chirho.constructor_key_chirho(name_chirho)?;
            let mut fields_chirho = Vec::new();
            let mut field_kinds_chirho = Vec::new();
            let mut gadt_result_kind_chirho = None;
            let mut gadt_result_chirho = None;
            let mut held_chirho = false;
            let mut explicit_chirho = std::collections::HashSet::new();
            let mut source_variables_chirho = Vec::new();
            if let ConDeclChirho::OrdinaryChirho { prefix_chirho, .. }
            | ConDeclChirho::RecordChirho { prefix_chirho, .. } = constructor_chirho
            {
                held_chirho |= this_chirho.bind_h98_constructor_prefix_chirho(
                    prefix_chirho,
                    &mut explicit_chirho,
                    &mut source_variables_chirho,
                    span_chirho,
                )?;
            }
            match constructor_chirho {
                ConDeclChirho::OrdinaryChirho {
                    fields_chirho: source_fields_chirho,
                    ..
                } => {
                    for (_, field_chirho) in source_fields_chirho {
                        let span_chirho = field_chirho.span_chirho();
                        let typed_chirho =
                            this_chirho.infer_defaulted_value_type_chirho(field_chirho, true)?;
                        this_chirho.state_chirho.charge_chirho(2, 2, span_chirho)?;
                        fields_chirho.push(typed_chirho.term_chirho);
                        field_kinds_chirho.push((typed_chirho.kind_chirho, span_chirho));
                    }
                }
                ConDeclChirho::RecordChirho {
                    fields_chirho: source_fields_chirho,
                    ..
                } => {
                    for field_chirho in source_fields_chirho {
                        let typed_chirho = this_chirho
                            .infer_defaulted_value_type_chirho(&mut field_chirho.ty_chirho, true)?;
                        this_chirho
                            .state_chirho
                            .charge_chirho(1, 1, field_chirho.span_chirho)?;
                        field_kinds_chirho
                            .push((typed_chirho.kind_chirho, field_chirho.span_chirho));
                        for _ in &field_chirho.names_chirho {
                            fields_chirho.push(this_chirho.state_chirho.copy_chirho(
                                &typed_chirho.term_chirho,
                                field_chirho.span_chirho,
                            )?);
                        }
                    }
                }
                ConDeclChirho::GadtChirho {
                    signature_chirho, ..
                } => {
                    let names_chirho = this_chirho
                        .declaration_variables_chirho
                        .get(parent_key_chirho)
                        .map(|variables_chirho| {
                            variables_chirho
                                .written_chirho
                                .iter()
                                .map(|(name_chirho, _, _)| name_chirho.clone())
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    for name_chirho in names_chirho {
                        this_chirho.hide_variable_chirho(&name_chirho, span_chirho)?;
                    }
                    for prefix_chirho in &mut signature_chirho.prefix_chirho {
                        match prefix_chirho {
                            GadtConPrefixChirho::ForallChirho {
                                vars_chirho: binders_chirho,
                                ..
                            } => {
                                this_chirho.bind_constructor_quantifiers_chirho(
                                    binders_chirho,
                                    &mut explicit_chirho,
                                    &mut source_variables_chirho,
                                    span_chirho,
                                )?;
                            }
                            GadtConPrefixChirho::ContextChirho {
                                constraints_chirho, ..
                            } => {
                                held_chirho |= !constraints_chirho.is_empty();
                                for constraint_chirho in constraints_chirho {
                                    this_chirho.infer_constraint_chirho(constraint_chirho)?;
                                }
                            }
                        }
                    }
                    match &mut signature_chirho.args_chirho {
                        GadtConArgsChirho::PositionalChirho(source_fields_chirho) => {
                            for field_chirho in source_fields_chirho {
                                let span_chirho = field_chirho.ty_chirho.span_chirho();
                                let typed_chirho = this_chirho.infer_defaulted_value_type_chirho(
                                    &mut field_chirho.ty_chirho,
                                    true,
                                )?;
                                this_chirho.state_chirho.charge_chirho(2, 2, span_chirho)?;
                                fields_chirho.push(typed_chirho.term_chirho);
                                field_kinds_chirho.push((typed_chirho.kind_chirho, span_chirho));
                            }
                        }
                        GadtConArgsChirho::RecordChirho(source_fields_chirho) => {
                            for field_chirho in source_fields_chirho {
                                let typed_chirho = this_chirho.infer_defaulted_value_type_chirho(
                                    &mut field_chirho.ty_chirho,
                                    true,
                                )?;
                                this_chirho.state_chirho.charge_chirho(
                                    1,
                                    1,
                                    field_chirho.span_chirho,
                                )?;
                                field_kinds_chirho
                                    .push((typed_chirho.kind_chirho, field_chirho.span_chirho));
                                for _ in &field_chirho.names_chirho {
                                    fields_chirho.push(this_chirho.state_chirho.copy_chirho(
                                        &typed_chirho.term_chirho,
                                        field_chirho.span_chirho,
                                    )?);
                                }
                            }
                        }
                    }
                    let result_chirho = this_chirho
                        .infer_value_type_chirho(&mut signature_chirho.result_ty_chirho)?;
                    gadt_result_chirho = Some(result_chirho.term_chirho);
                    gadt_result_kind_chirho = Some(result_chirho.kind_chirho);
                }
            }
            let newtype_chirho = this_chirho
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(parent_key_chirho)
                .is_some_and(|scheme_chirho| {
                    scheme_chirho.role_chirho == TypeConstructorRoleChirho::NewtypeChirho
                });
            if newtype_chirho {
                let result_kind_chirho = if let Some(kind_chirho) = gadt_result_kind_chirho {
                    kind_chirho
                } else {
                    this_chirho.result_kind_chirho(parent_key_chirho, span_chirho)?
                };
                for (kind_chirho, span_chirho) in &field_kinds_chirho {
                    this_chirho.unify_kinds_chirho(
                        kind_chirho,
                        &result_kind_chirho,
                        *span_chirho,
                    )?;
                }
                let result_chirho = if let Some(result_chirho) = &gadt_result_chirho {
                    Some(
                        this_chirho
                            .state_chirho
                            .copy_chirho(result_chirho, span_chirho)?,
                    )
                } else if let Some(key_chirho) = &key_chirho {
                    this_chirho
                        .registry_chirho
                        .declaration_schemes_chirho()
                        .get_chirho(key_chirho)
                        .map(|scheme_chirho| {
                            this_chirho
                                .state_chirho
                                .copy_chirho(&scheme_chirho.result_kind_chirho, span_chirho)
                        })
                        .transpose()?
                } else {
                    None
                };
                if let Some(result_chirho) = result_chirho {
                    this_chirho.check_newtype_result_parameters_chirho(
                        &result_chirho,
                        &fields_chirho,
                        parent_key_chirho,
                        span_chirho,
                    )?;
                }
            }
            let field_slot_chirho = this_chirho
                .declaration_variables_chirho
                .get_mut(parent_key_chirho)
                .and_then(|variables_chirho| {
                    variables_chirho
                        .fixed_field_kinds_chirho
                        .get_mut(constructor_index_chirho)
                })
                .ok_or_else(|| {
                    Self::error_chirho(
                        "checked fields lost their actual constructor position",
                        span_chirho,
                    )
                })?;
            *field_slot_chirho = Some(field_kinds_chirho);
            if let Some(key_chirho) = key_chirho {
                let mut scheme_chirho = this_chirho
                    .registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(&key_chirho)
                    .cloned()
                    .ok_or_else(|| {
                        Self::error_chirho("missing promoted constructor header", span_chirho)
                    })?
                    .as_ref()
                    .clone();
                if !matches!(constructor_chirho, ConDeclChirho::GadtChirho { .. }) {
                    this_chirho.retain_h98_quantifier_telescope_chirho(
                        &mut scheme_chirho,
                        &source_variables_chirho,
                        span_chirho,
                    )?;
                }
                scheme_chirho.source_variables_chirho = source_variables_chirho;
                // Constructor kind variables inferred from parameter kinds are
                // invisible even when the datatype header wrote their names.
                // An actual constructor forall retains its own specificity.
                let mut inferred_chirho = std::collections::HashSet::new();
                for term_chirho in fields_chirho.iter().chain(std::iter::once(
                    gadt_result_chirho
                        .as_ref()
                        .unwrap_or(&scheme_chirho.result_kind_chirho),
                )) {
                    let resolved_chirho = this_chirho
                        .state_chirho
                        .resolve_chirho(term_chirho, span_chirho)?;
                    for variable_chirho in resolved_chirho.free_vars_chirho() {
                        let kind_chirho = this_chirho
                            .state_chirho
                            .variable_kind_chirho(variable_chirho, span_chirho)?;
                        let kind_chirho = this_chirho
                            .state_chirho
                            .resolve_chirho(&kind_chirho, span_chirho)?;
                        for dependency_chirho in kind_chirho.free_vars_chirho() {
                            if !explicit_chirho.contains(&dependency_chirho) {
                                inferred_chirho.insert(dependency_chirho);
                            }
                        }
                    }
                }
                for variable_chirho in &explicit_chirho {
                    let kind_chirho = this_chirho
                        .state_chirho
                        .variable_kind_chirho(*variable_chirho, span_chirho)?;
                    let kind_chirho = this_chirho
                        .state_chirho
                        .resolve_chirho(&kind_chirho, span_chirho)?;
                    inferred_chirho.extend(
                        kind_chirho
                            .free_vars_chirho()
                            .into_iter()
                            .filter(|dependency_chirho| {
                                !explicit_chirho.contains(dependency_chirho)
                            }),
                    );
                }
                this_chirho
                    .declaration_variables_chirho
                    .entry(key_chirho.clone())
                    .or_default()
                    .constructor_inferred_chirho = inferred_chirho;
                let required_chirho =
                    scheme_chirho
                        .binders_chirho
                        .iter_mut()
                        .filter(|binder_chirho| {
                            binder_chirho.visibility_chirho
                                == TelescopeVisibilityChirho::RequiredChirho
                        });
                for (binder_chirho, field_chirho) in required_chirho.zip(fields_chirho) {
                    binder_chirho.kind_chirho = field_chirho;
                }
                if let Some(result_chirho) = gadt_result_chirho {
                    this_chirho.state_chirho.charge_chirho(
                        parent_key_chirho.len() + 1,
                        parent_key_chirho.len() + 1,
                        span_chirho,
                    )?;
                    let parent_checked_chirho = this_chirho
                        .registry_chirho
                        .declaration_schemes_chirho()
                        .get_chirho(parent_key_chirho)
                        .is_some_and(|parent_chirho| parent_chirho.owner_chirho.is_none());
                    this_chirho
                        .declaration_variables_chirho
                        .entry(key_chirho.clone())
                        .or_default()
                        .constructor_result_origin_chirho = Some(if parent_checked_chirho {
                        ConstructorResultOriginChirho::SignatureApplicationChirho {
                            parent_chirho: Arc::from(parent_key_chirho),
                            checking_chirho: SignatureCheckingChirho::CompleteChirho,
                        }
                    } else {
                        ConstructorResultOriginChirho::SignatureApplicationChirho {
                            parent_chirho: Arc::from(parent_key_chirho),
                            checking_chirho: SignatureCheckingChirho::ProvisionalChirho,
                        }
                    });
                    scheme_chirho.result_kind_chirho = result_chirho;
                    scheme_chirho.binders_chirho.retain(|binder_chirho| {
                        binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho
                    });
                }
                scheme_chirho.held_chirho = held_chirho;
                this_chirho
                    .registry_chirho
                    .declaration_schemes_mut_chirho()
                    .stage_chirho(scheme_chirho);
                // Publication discovers the complete free kind/type prefix from checked fields.
                this_chirho
                    .declaration_variables_chirho
                    .entry(key_chirho.clone())
                    .or_default();
                keys_chirho.push(key_chirho);
            }
            Ok(())
        });
        self.state_chirho.close_rigid_scope_chirho(rigid_chirho);
        result_chirho
    }
}
