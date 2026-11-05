// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type and kind syntax share one elaborator. Hidden applications remain
//! explicit metadata nodes until runtime lowering erases their value arity.

use super::*;

mod checking_chirho;
mod prefix_chirho;
mod spine_chirho;
use crate::subst_chirho::SubstChirho;
use haskelujah_ast_chirho::decl_chirho::TyVarChirho as AstTyVarChirho;
use haskelujah_ast_chirho::name_chirho::TypeConstructorRoleChirho;
use haskelujah_ast_chirho::name_chirho::{
    CompilerTypeConstructorChirho, NameChirho, RawNameChirho, TypeConstructorDeclarationChirho,
};
use haskelujah_ast_chirho::promotion_chirho::quoted_promoted_spelling_chirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};

impl TypeElaboratorChirho<'_> {
    pub(super) fn infer_type_at_kind_chirho(
        &mut self,
        syntax_chirho: &mut TypeChirho,
        expected_chirho: &TyChirho,
    ) -> Result<ElaboratedTypeChirho, ElaborationErrorChirho> {
        let span_chirho = syntax_chirho.span_chirho();
        // The empty tuple has both a Type and a Constraint interpretation.
        // Its checked surrounding kind selects the latter; unconstrained ()
        // retains the ordinary Type default.
        let expected_chirho = self
            .state_chirho
            .resolve_chirho(expected_chirho, span_chirho)?;
        if matches!(
            &expected_chirho,
            TyChirho::ForallChirho { .. } | TyChirho::RequiredForallChirho { .. }
        ) {
            return self.check_universal_kind_argument_chirho(
                syntax_chirho,
                expected_chirho,
                span_chirho,
            );
        }
        let mut syntax_chirho = syntax_chirho;
        let mut parens_chirho = 0;
        while let TypeChirho::ParenChirho { inner_chirho, .. } = syntax_chirho {
            parens_chirho += 1;
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            if self.type_depth_chirho + parens_chirho >= 128 {
                return Err(Self::error_chirho(
                    "kind/type syntax depth bound exhausted",
                    span_chirho,
                ));
            }
            syntax_chirho = inner_chirho;
        }
        let result_chirho = match syntax_chirho {
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } if elements_chirho.is_empty()
                && expected_chirho == ElaborationStateChirho::constraint_universe_chirho() =>
            {
                self.state_chirho.charge_chirho(1, 1, span_chirho)?;
                ElaboratedTypeChirho {
                    term_chirho: TyChirho::TupleChirho(Vec::new()),
                    kind_chirho: expected_chirho.clone(),
                }
            }
            _ => self.infer_type_chirho(syntax_chirho)?,
        };
        self.check_inferred_kind_chirho(syntax_chirho, result_chirho, &expected_chirho)
    }

    pub(super) fn infer_type_chirho(
        &mut self,
        syntax_chirho: &mut TypeChirho,
    ) -> Result<ElaboratedTypeChirho, ElaborationErrorChirho> {
        if self.type_depth_chirho >= 128 {
            return Err(Self::error_chirho(
                "kind/type syntax depth bound exhausted",
                syntax_chirho.span_chirho(),
            ));
        }
        self.type_depth_chirho += 1;
        let result_chirho = self.infer_type_inner_chirho(syntax_chirho);
        self.type_depth_chirho -= 1;
        result_chirho
    }

    fn infer_type_inner_chirho(
        &mut self,
        syntax_chirho: &mut TypeChirho,
    ) -> Result<ElaboratedTypeChirho, ElaborationErrorChirho> {
        let span_chirho = syntax_chirho.span_chirho();
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        let required_forall_chirho =
            matches!(syntax_chirho, TypeChirho::RequiredForallChirho { .. });
        match syntax_chirho {
            TypeChirho::ConChirho(_)
            | TypeChirho::PromotedConChirho { .. }
            | TypeChirho::AppChirho { .. }
            | TypeChirho::KindApplicationChirho { .. } => self.infer_spine_chirho(syntax_chirho),
            TypeChirho::VarChirho(name_chirho) => {
                let (variable_chirho, kind_chirho) =
                    if let Some(variable_chirho) = self.check_variable_owner_chirho(name_chirho)? {
                        (
                            variable_chirho,
                            if let Some(image_chirho) =
                                self.lexical_images_chirho.and_then(|images_chirho| {
                                    images_chirho.lookup_image_chirho(&variable_chirho)
                                })
                            {
                                let kind_chirho =
                                    image_chirho.kind_chirho.as_ref().ok_or_else(|| {
                                        Self::error_chirho(
                                            "lexical type image has no checked kind",
                                            span_chirho,
                                        )
                                    })?;
                                self.normalize_lexical_term_chirho(kind_chirho, span_chirho)?
                            } else {
                                self.state_chirho
                                    .variable_kind_chirho(variable_chirho, span_chirho)?
                            },
                        )
                    } else {
                        if self.strict_argument_chirho
                            && !self
                                .variables_chirho
                                .contains_key(name_chirho.text_chirho())
                        {
                            return Err(Self::error_chirho(
                                "required type argument has no lexical type-variable binding",
                                span_chirho,
                            ));
                        }
                        self.variable_chirho(name_chirho.text_chirho(), span_chirho)?
                    };
                self.stamp_variable_chirho(name_chirho, variable_chirho)?;
                if let Some(image_chirho) = self
                    .lexical_images_chirho
                    .and_then(|images_chirho| images_chirho.lookup_image_chirho(&variable_chirho))
                {
                    let kind_chirho = image_chirho.kind_chirho.as_ref().ok_or_else(|| {
                        Self::error_chirho("lexical type image has no checked kind", span_chirho)
                    })?;
                    return Ok(ElaboratedTypeChirho {
                        term_chirho: self.normalize_lexical_term_chirho(
                            &image_chirho.image_chirho,
                            span_chirho,
                        )?,
                        kind_chirho: self
                            .normalize_lexical_term_chirho(kind_chirho, span_chirho)?,
                    });
                }
                Ok(ElaboratedTypeChirho {
                    term_chirho: TyChirho::VarChirho(variable_chirho),
                    kind_chirho,
                })
            }
            TypeChirho::KindAnnotationChirho {
                ty_chirho,
                kind_chirho,
                ..
            } => {
                let annotation_chirho = self.infer_kind_syntax_chirho(kind_chirho)?;
                let elaborated_chirho =
                    self.infer_type_at_kind_chirho(ty_chirho, &annotation_chirho.term_chirho)?;
                Ok(elaborated_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => self.infer_type_chirho(inner_chirho),
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                mult_chirho,
                ..
            } => {
                let argument_chirho = self.infer_type_chirho(arg_chirho)?;
                let result_chirho = self.infer_type_chirho(result_chirho)?;
                self.unify_kinds_chirho(
                    &argument_chirho.kind_chirho,
                    &ElaborationStateChirho::type_universe_chirho(),
                    span_chirho,
                )?;
                self.unify_kinds_chirho(
                    &result_chirho.kind_chirho,
                    &ElaborationStateChirho::type_universe_chirho(),
                    span_chirho,
                )?;
                Ok(ElaboratedTypeChirho {
                    term_chirho: TyChirho::FunChirho(
                        Box::new(argument_chirho.term_chirho),
                        Box::new(result_chirho.term_chirho),
                        if matches!(
                            mult_chirho,
                            Some(haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho)
                        ) {
                            crate::ty_chirho::MultChirho::OneChirho
                        } else {
                            crate::ty_chirho::MultChirho::ManyChirho
                        },
                    ),
                    kind_chirho: ElaborationStateChirho::type_universe_chirho(),
                })
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                self.state_chirho.charge_chirho(
                    elements_chirho.len(),
                    elements_chirho.len(),
                    span_chirho,
                )?;
                let mut terms_chirho = Vec::with_capacity(elements_chirho.len());
                let mut tuple_kind_chirho = None;
                for element_chirho in elements_chirho {
                    let element_chirho = self.infer_type_chirho(element_chirho)?;
                    if let Some(expected_chirho) = &tuple_kind_chirho {
                        self.unify_kinds_chirho(
                            &element_chirho.kind_chirho,
                            expected_chirho,
                            span_chirho,
                        )?;
                    } else {
                        tuple_kind_chirho = Some(element_chirho.kind_chirho);
                    }
                    terms_chirho.push(element_chirho.term_chirho);
                }
                let kind_chirho = if let Some(kind_chirho) = tuple_kind_chirho {
                    self.state_chirho
                        .resolve_chirho(&kind_chirho, span_chirho)?
                } else {
                    ElaborationStateChirho::type_universe_chirho()
                };
                if kind_chirho != ElaborationStateChirho::constraint_universe_chirho() {
                    self.unify_kinds_chirho(
                        &kind_chirho,
                        &ElaborationStateChirho::type_universe_chirho(),
                        span_chirho,
                    )?;
                }
                Ok(ElaboratedTypeChirho {
                    term_chirho: TyChirho::TupleChirho(terms_chirho),
                    kind_chirho,
                })
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                let element_chirho = self.infer_type_chirho(element_chirho)?;
                self.unify_kinds_chirho(
                    &element_chirho.kind_chirho,
                    &ElaborationStateChirho::type_universe_chirho(),
                    span_chirho,
                )?;
                Ok(ElaboratedTypeChirho {
                    term_chirho: TyChirho::ListChirho(Box::new(element_chirho.term_chirho)),
                    kind_chirho: ElaborationStateChirho::type_universe_chirho(),
                })
            }
            TypeChirho::PromotedListChirho { .. } => self.infer_promoted_list_chirho(syntax_chirho),
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                self.state_chirho.charge_chirho(
                    context_chirho.len(),
                    context_chirho.len(),
                    span_chirho,
                )?;
                let mut context_terms_chirho = Vec::with_capacity(context_chirho.len());
                for constraint_chirho in context_chirho {
                    context_terms_chirho.push(self.infer_constraint_chirho(constraint_chirho)?);
                }
                let body_chirho = self.infer_type_chirho(body_chirho)?;
                self.unify_kinds_chirho(
                    &body_chirho.kind_chirho,
                    &ElaborationStateChirho::type_universe_chirho(),
                    span_chirho,
                )?;
                Ok(ElaboratedTypeChirho {
                    term_chirho: TyChirho::QualChirho {
                        context_chirho: context_terms_chirho,
                        body_chirho: Box::new(body_chirho.term_chirho),
                    },
                    kind_chirho: body_chirho.kind_chirho,
                })
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            }
            | TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                if required_forall_chirho && vars_chirho.iter().any(|binder_chirho| binder_chirho.specificity_chirho == haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho) {
                    return Err(Self::error_chirho("inferred variables cannot bind a required forall", span_chirho));
                }
                let mark_chirho = self.scope_undo_chirho.len();
                let variables_chirho = self.bind_forall_chirho(vars_chirho)?;
                let rigid_chirho = self
                    .state_chirho
                    .open_rigid_scope_chirho(&variables_chirho, span_chirho)?;
                let body_chirho = self.infer_type_chirho(body_chirho);
                self.state_chirho.close_rigid_scope_chirho(rigid_chirho);
                self.close_forall_scope_chirho(mark_chirho, &variables_chirho, span_chirho)?;
                let body_chirho = body_chirho?;
                let variables_chirho =
                    self.introduced_quantifiers_chirho(&variables_chirho, span_chirho)?;
                Ok(ElaboratedTypeChirho {
                    term_chirho: if matches!(syntax_chirho, TypeChirho::RequiredForallChirho { .. })
                    {
                        TyChirho::RequiredForallChirho {
                            vars_chirho: variables_chirho,
                            body_chirho: Box::new(body_chirho.term_chirho),
                        }
                    } else {
                        TyChirho::ForallChirho {
                            vars_chirho: variables_chirho,
                            body_chirho: Box::new(body_chirho.term_chirho),
                        }
                    },
                    kind_chirho: body_chirho.kind_chirho,
                })
            }
            TypeChirho::WildcardChirho { .. } => {
                if self.standalone_kind_scope_chirho {
                    return Err(Self::error_chirho(
                        "wildcards are not allowed in a standalone kind signature",
                        span_chirho,
                    ));
                }
                let kind_chirho = self.fresh_kind_chirho(span_chirho)?;
                let stored_kind_chirho =
                    self.state_chirho.copy_chirho(&kind_chirho, span_chirho)?;
                let variable_chirho = self
                    .state_chirho
                    .fresh_chirho(Some(stored_kind_chirho), span_chirho)?;
                Ok(ElaboratedTypeChirho {
                    term_chirho: TyChirho::VarChirho(variable_chirho),
                    kind_chirho,
                })
            }
            TypeChirho::LitChirho { value_chirho, .. } => {
                let kind_name_chirho = if value_chirho.starts_with('"') {
                    "Symbol"
                } else if value_chirho.starts_with('\'') {
                    "Char"
                } else {
                    "Nat"
                };
                let term_chirho =
                    TyChirho::ConChirho(self.state_chirho.name_chirho(value_chirho, span_chirho)?);
                // Literals are positively produced syntax; their compiler kind
                // identity does not come from an arbitrary source type spelling.
                self.state_chirho.charge_chirho(64, 64, span_chirho)?;
                Ok(ElaboratedTypeChirho {
                    term_chirho,
                    kind_chirho: TyChirho::ConChirho(kind_name_chirho.to_owned()),
                })
            }
        }
    }

    pub(super) fn bind_forall_chirho(
        &mut self,
        binders_chirho: &mut [AstTyVarChirho],
    ) -> Result<Vec<TyVarChirho>, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(
            binders_chirho.len(),
            binders_chirho.len(),
            binders_chirho
                .first()
                .map_or(SpanChirho::DUMMY_CHIRHO, |binder_chirho| {
                    binder_chirho.name_chirho.span_chirho()
                }),
        )?;
        let mut variables_chirho = Vec::with_capacity(binders_chirho.len());
        for binder_chirho in binders_chirho.iter_mut() {
            let span_chirho = binder_chirho.name_chirho.span_chirho();
            let (variable_chirho, kind_chirho) = if let Some(variable_chirho) =
                self.check_variable_owner_chirho(&binder_chirho.name_chirho)?
            {
                (
                    variable_chirho,
                    self.state_chirho
                        .variable_kind_chirho(variable_chirho, span_chirho)?,
                )
            } else {
                let kind_chirho = self.fresh_kind_chirho(span_chirho)?;
                let stored_kind_chirho =
                    self.state_chirho.copy_chirho(&kind_chirho, span_chirho)?;
                (
                    self.state_chirho
                        .fresh_chirho(Some(stored_kind_chirho), span_chirho)?,
                    kind_chirho,
                )
            };
            self.state_chirho.note_source_variable_chirho(
                variable_chirho,
                binder_chirho.specificity_chirho,
                true,
                span_chirho,
            )?;
            self.state_chirho
                .mark_universal_chirho(variable_chirho, span_chirho)?;
            self.stamp_variable_chirho(&mut binder_chirho.name_chirho, variable_chirho)?;
            let name_chirho = self
                .state_chirho
                .name_chirho(binder_chirho.text_chirho(), span_chirho)?;
            self.bind_variable_chirho(name_chirho, variable_chirho, kind_chirho, span_chirho)?;
            variables_chirho.push(variable_chirho);
        }
        self.check_binder_annotations_chirho(binders_chirho)?;
        Ok(variables_chirho)
    }

    pub(super) fn scheme_chirho(
        &mut self,
        head_chirho: &mut TypeChirho,
    ) -> Result<Option<Arc<DeclarationSchemeChirho>>, ElaborationErrorChirho> {
        let span_chirho = head_chirho.span_chirho();
        match head_chirho {
            TypeChirho::ParenChirho { inner_chirho, .. } => self.scheme_chirho(inner_chirho),
            TypeChirho::ConChirho(name_chirho) => {
                if let Some(constructor_chirho) = syntax_constructor_chirho(name_chirho) {
                    self.seed_compiler_header_chirho(constructor_chirho, span_chirho)?;
                    name_chirho.set_type_constructor_chirho(Some(
                        TypeConstructorDeclarationChirho::compiler_data_chirho(constructor_chirho),
                    ));
                }
                if self.star_is_type_chirho
                    && name_chirho.qualifier_chirho().is_none()
                    && name_chirho.text_chirho() == "*"
                {
                    name_chirho.set_type_constructor_chirho(Some(
                        TypeConstructorDeclarationChirho::compiler_data_chirho(
                            CompilerTypeConstructorChirho::TypeUniverseChirho,
                        ),
                    ));
                }
                if let Some(proof_chirho) = self.declaration_chirho(name_chirho)? {
                    let key_chirho = self.declaration_key_chirho(&proof_chirho, span_chirho)?;
                    if matches!(
                        proof_chirho.role_chirho,
                        TypeConstructorRoleChirho::TypeFamilyChirho
                            | TypeConstructorRoleChirho::DataFamilyChirho
                    ) {
                        name_chirho.set_family_key_chirho(
                            self.state_chirho.name_chirho(&key_chirho, span_chirho)?,
                        );
                    }
                    name_chirho.set_type_constructor_chirho(Some(proof_chirho));
                    return Ok(self
                        .registry_chirho
                        .declaration_schemes_chirho()
                        .get_chirho(&key_chirho)
                        .cloned());
                }
                let written_chirho = self.written_name_chirho(name_chirho)?;
                if let crate::infer_chirho::TypeFamilyScopeResolutionChirho::UniqueChirho(
                    key_chirho,
                ) = self
                    .family_scope_chirho
                    .resolve_chirho(&written_chirho, &self.registry_chirho)
                    && let Some(scheme_chirho) = self
                        .registry_chirho
                        .declaration_schemes_chirho()
                        .get_chirho(key_chirho)
                        .cloned()
                {
                    name_chirho.set_family_key_chirho(
                        self.state_chirho.name_chirho(key_chirho, span_chirho)?,
                    );
                    return Ok(Some(scheme_chirho));
                }
                if let Some(class_chirho) =
                    self.builtin_classes_chirho.class_chirho(&written_chirho)
                {
                    self.state_chirho.charge_chirho(
                        class_chirho.module_chirho.len() + class_chirho.name_chirho.len() + 64,
                        class_chirho.module_chirho.len() + class_chirho.name_chirho.len() + 64,
                        span_chirho,
                    )?;
                    let key_chirho = format!(
                        "%class:compiler:{}",
                        class_chirho.identity_chirho.legacy_name_chirho()
                    );
                    return Ok(self
                        .registry_chirho
                        .declaration_schemes_chirho()
                        .get_chirho(&key_chirho)
                        .cloned());
                }
                let resolution_chirho = self.promotion_scope_chirho.unquoted_chirho(name_chirho);
                if let Some(resolution_chirho) = resolution_chirho
                    && !haskelujah_ast_chirho::promotion_chirho::is_unknown_promoted_key_chirho(
                        resolution_chirho.key_chirho().as_str_chirho(),
                    )
                {
                    let scheme_chirho = self
                        .registry_chirho
                        .declaration_schemes_chirho()
                        .get_chirho(resolution_chirho.key_chirho().as_str_chirho())
                        .cloned();
                    name_chirho.set_promotion_chirho(resolution_chirho);
                    return Ok(scheme_chirho);
                }
                Ok(None)
            }
            TypeChirho::PromotedConChirho { name_chirho, .. } => {
                if let Some(constructor_chirho) = syntax_constructor_chirho(name_chirho) {
                    self.seed_compiler_header_chirho(constructor_chirho, span_chirho)?;
                }
                if name_chirho.qualifier_chirho().is_none() && name_chirho.text_chirho() == ":" {
                    self.seed_compiler_header_chirho(
                        CompilerTypeConstructorChirho::ListChirho,
                        span_chirho,
                    )?;
                }
                self.state_chirho.charge_chirho(
                    name_chirho.text_chirho().len() + 128,
                    name_chirho.text_chirho().len() + 128,
                    span_chirho,
                )?;
                let Some(key_chirho) =
                    quoted_promoted_spelling_chirho(name_chirho, Some(self.promotion_scope_chirho))
                else {
                    return Ok(None);
                };
                if haskelujah_ast_chirho::promotion_chirho::is_unknown_promoted_key_chirho(
                    &key_chirho,
                ) {
                    return Ok(None);
                }
                let scheme_chirho = self
                    .registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(&key_chirho)
                    .cloned();
                Ok(scheme_chirho)
            }
            _ => Ok(None),
        }
    }

    pub(super) fn instantiate_header_chirho(
        &mut self,
        scheme_chirho: &DeclarationSchemeChirho,
        span_chirho: SpanChirho,
    ) -> Result<(Vec<DeclarationBinderChirho>, TyChirho, Option<TyChirho>), ElaborationErrorChirho>
    {
        if scheme_chirho.held_chirho {
            return Err(Self::error_chirho(
                "a held declaration telescope cannot be instantiated",
                span_chirho,
            ));
        }
        if scheme_chirho
            .owner_chirho
            .is_some_and(|owner_chirho| Some(owner_chirho) != self.state_chirho.owner_chirho)
        {
            return Err(Self::error_chirho(
                "foreign declaration contains unquantified kind variables",
                span_chirho,
            ));
        }
        self.state_chirho.charge_chirho(
            scheme_chirho.binders_chirho.len(),
            scheme_chirho.binders_chirho.len(),
            span_chirho,
        )?;
        let mut rename_chirho = SubstChirho::empty_chirho();
        for binder_chirho in &scheme_chirho.binders_chirho {
            let variable_chirho = self.state_chirho.fresh_chirho(None, span_chirho)?;
            rename_chirho.insert_chirho(
                binder_chirho.variable_chirho,
                TyChirho::VarChirho(variable_chirho),
            );
        }
        let mut binders_chirho = Vec::with_capacity(scheme_chirho.binders_chirho.len());
        for binder_chirho in &scheme_chirho.binders_chirho {
            let copied_kind_chirho = if scheme_chirho.owner_chirho.is_some() {
                self.state_chirho
                    .resolve_chirho(&binder_chirho.kind_chirho, span_chirho)?
            } else {
                self.state_chirho
                    .copy_chirho(&binder_chirho.kind_chirho, span_chirho)?
            };
            let kind_chirho = self
                .state_chirho
                .instantiate_checked_declaration_term_chirho(
                    &copied_kind_chirho,
                    &rename_chirho,
                    scheme_chirho.owner_chirho.is_some(),
                    &scheme_chirho.local_binders_chirho,
                    span_chirho,
                )?;
            let TyChirho::VarChirho(variable_chirho) = rename_chirho
                .lookup_chirho(&binder_chirho.variable_chirho)
                .expect("freshened telescope variable")
            else {
                return Err(Self::error_chirho(
                    "invalid telescope renaming",
                    span_chirho,
                ));
            };
            let variable_chirho = *variable_chirho;
            let stored_kind_chirho = self.state_chirho.copy_chirho(&kind_chirho, span_chirho)?;
            self.state_chirho
                .set_fresh_variable_kind_chirho(variable_chirho, stored_kind_chirho);
            binders_chirho.push(DeclarationBinderChirho {
                variable_chirho,
                kind_chirho,
                visibility_chirho: binder_chirho.visibility_chirho,
            });
        }
        let result_kind_chirho = if scheme_chirho.owner_chirho.is_some() {
            self.state_chirho
                .resolve_chirho(&scheme_chirho.result_kind_chirho, span_chirho)?
        } else {
            self.state_chirho
                .copy_chirho(&scheme_chirho.result_kind_chirho, span_chirho)?
        };
        let result_kind_chirho = self
            .state_chirho
            .instantiate_checked_declaration_term_chirho(
                &result_kind_chirho,
                &rename_chirho,
                scheme_chirho.owner_chirho.is_some(),
                &scheme_chirho.local_binders_chirho,
                span_chirho,
            )?;
        Ok((
            binders_chirho,
            result_kind_chirho,
            scheme_chirho
                .synonym_rhs_chirho
                .as_ref()
                .map(|rhs_chirho| {
                    self.state_chirho
                        .instantiate_checked_declaration_term_chirho(
                            rhs_chirho,
                            &rename_chirho,
                            scheme_chirho.owner_chirho.is_some(),
                            &scheme_chirho.local_binders_chirho,
                            span_chirho,
                        )
                })
                .transpose()?,
        ))
    }

    fn infer_promoted_list_chirho(
        &mut self,
        syntax_chirho: &mut TypeChirho,
    ) -> Result<ElaboratedTypeChirho, ElaborationErrorChirho> {
        let TypeChirho::PromotedListChirho {
            elements_chirho,
            span_chirho,
        } = syntax_chirho
        else {
            unreachable!()
        };
        let span_chirho = *span_chirho;
        if elements_chirho.len() >= 128 {
            return Err(self.state_chirho.resource_failure_chirho(
                "promoted list spine depth bound exhausted",
                span_chirho,
            ));
        }
        let element_kind_chirho = self.fresh_kind_chirho(span_chirho)?;
        self.state_chirho.charge_chirho(
            elements_chirho.len(),
            elements_chirho.len(),
            span_chirho,
        )?;
        let mut values_chirho = Vec::with_capacity(elements_chirho.len());
        for element_chirho in elements_chirho.iter_mut() {
            let value_chirho = self.infer_type_chirho(element_chirho)?;
            self.unify_kinds_chirho(&value_chirho.kind_chirho, &element_kind_chirho, span_chirho)?;
            values_chirho.push(value_chirho.term_chirho);
        }
        // Project only the positive grammar producers' checked prefixes. Keep
        // every already-checked source element, including forall/context/origin
        // metadata, rather than reifying elements from erased Ty terms.
        let nil_key_chirho =
            haskelujah_ast_chirho::promotion_chirho::PromotedKeyChirho::builtin_nil_chirho();
        let cons_key_chirho =
            haskelujah_ast_chirho::promotion_chirho::PromotedKeyChirho::builtin_cons_chirho();
        let nil_prefix_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(
                nil_key_chirho.as_str_chirho().to_owned(),
            )),
            Box::new(
                self.state_chirho
                    .copy_chirho(&element_kind_chirho, span_chirho)?,
            ),
        );
        let mut projected_chirho = self.surface_chirho(&nil_prefix_chirho, span_chirho)?;
        self.state_chirho.charge_chirho(
            elements_chirho.len(),
            elements_chirho.len(),
            span_chirho,
        )?;
        let mut prefixes_chirho = Vec::with_capacity(elements_chirho.len());
        for _ in 0..elements_chirho.len() {
            let prefix_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho(
                    cons_key_chirho.as_str_chirho().to_owned(),
                )),
                Box::new(
                    self.state_chirho
                        .copy_chirho(&element_kind_chirho, span_chirho)?,
                ),
            );
            prefixes_chirho.push(self.surface_chirho(&prefix_chirho, span_chirho)?);
        }
        for (element_chirho, prefix_chirho) in std::mem::take(elements_chirho)
            .into_iter()
            .zip(prefixes_chirho)
            .rev()
        {
            projected_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::AppChirho {
                    fun_chirho: Box::new(prefix_chirho),
                    arg_chirho: Box::new(element_chirho),
                    span_chirho,
                }),
                arg_chirho: Box::new(projected_chirho),
                span_chirho,
            };
        }
        let mut term_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(
                haskelujah_ast_chirho::promotion_chirho::PromotedKeyChirho::builtin_nil_chirho()
                    .as_str_chirho()
                    .to_owned(),
            )),
            Box::new(
                self.state_chirho
                    .copy_chirho(&element_kind_chirho, span_chirho)?,
            ),
        );
        for value_chirho in values_chirho.into_iter().rev() {
            term_chirho=TyChirho::AppChirho(Box::new(TyChirho::AppChirho(Box::new(TyChirho::AppChirho(Box::new(TyChirho::ConChirho(haskelujah_ast_chirho::promotion_chirho::PromotedKeyChirho::builtin_cons_chirho().as_str_chirho().to_owned())),Box::new(self.state_chirho.copy_chirho(&element_kind_chirho,span_chirho)?))),Box::new(value_chirho))),Box::new(term_chirho));
        }
        *syntax_chirho = projected_chirho;
        Ok(ElaboratedTypeChirho {
            term_chirho,
            kind_chirho: TyChirho::ListChirho(Box::new(
                self.state_chirho
                    .resolve_chirho(&element_kind_chirho, span_chirho)?,
            )),
        })
    }

    pub(super) fn infer_constraint_chirho(
        &mut self,
        constraint_chirho: &mut ConstraintChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        match constraint_chirho {
            ConstraintChirho::ClassChirho {
                class_chirho,
                args_chirho,
                kind_args_chirho,
                span_chirho,
            } => {
                let span_chirho = *span_chirho;
                let name_chirho = std::mem::replace(
                    class_chirho,
                    NameChirho::RawChirho(RawNameChirho::unqualified_chirho("", span_chirho)),
                );
                let mut syntax_chirho = TypeChirho::ConChirho(name_chirho);
                for argument_chirho in std::mem::take(kind_args_chirho) {
                    syntax_chirho = TypeChirho::KindApplicationChirho {
                        fun_chirho: Box::new(syntax_chirho),
                        arg_chirho: Box::new(argument_chirho),
                        written_chirho: !self.rechecking_chirho,
                        span_chirho,
                    };
                }
                for argument_chirho in std::mem::take(args_chirho) {
                    syntax_chirho = TypeChirho::AppChirho {
                        fun_chirho: Box::new(syntax_chirho),
                        arg_chirho: Box::new(argument_chirho),
                        span_chirho,
                    };
                }
                let typed_chirho = self.infer_type_chirho(&mut syntax_chirho)?;
                self.unify_kinds_chirho(
                    &typed_chirho.kind_chirho,
                    &ElaborationStateChirho::constraint_universe_chirho(),
                    span_chirho,
                )?;
                loop {
                    match syntax_chirho {
                        TypeChirho::AppChirho {
                            fun_chirho,
                            arg_chirho,
                            ..
                        } => {
                            args_chirho.push(*arg_chirho);
                            syntax_chirho = *fun_chirho;
                        }
                        TypeChirho::KindApplicationChirho {
                            fun_chirho,
                            arg_chirho,
                            ..
                        } => {
                            kind_args_chirho.push(*arg_chirho);
                            syntax_chirho = *fun_chirho;
                        }
                        TypeChirho::ConChirho(name_chirho) => {
                            *class_chirho = name_chirho;
                            break;
                        }
                        _ => {
                            return Err(Self::error_chirho(
                                "constraint lost its class head",
                                span_chirho,
                            ));
                        }
                    }
                }
                args_chirho.reverse();
                kind_args_chirho.reverse();
                Ok(typed_chirho.term_chirho)
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                span_chirho,
            } => {
                let span_chirho = *span_chirho;
                let mark_chirho = self.scope_undo_chirho.len();
                let variables_chirho = self.bind_forall_chirho(vars_chirho)?;
                let rigid_chirho = self
                    .state_chirho
                    .open_rigid_scope_chirho(&variables_chirho, span_chirho)?;
                let result_chirho = (|| {
                    self.state_chirho.charge_chirho(
                        context_chirho.len(),
                        context_chirho.len(),
                        span_chirho,
                    )?;
                    let mut context_terms_chirho = Vec::with_capacity(context_chirho.len());
                    for constraint_chirho in context_chirho {
                        context_terms_chirho.push(self.infer_constraint_chirho(constraint_chirho)?);
                    }
                    let body_chirho = self.infer_constraint_chirho(body_chirho)?;
                    Ok(TyChirho::ForallChirho {
                        vars_chirho: self
                            .introduced_quantifiers_chirho(&variables_chirho, span_chirho)?,
                        body_chirho: Box::new(TyChirho::QualChirho {
                            context_chirho: context_terms_chirho,
                            body_chirho: Box::new(body_chirho),
                        }),
                    })
                })();
                self.state_chirho.close_rigid_scope_chirho(rigid_chirho);
                self.close_forall_scope_chirho(mark_chirho, &variables_chirho, span_chirho)?;
                result_chirho
            }
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho,
                kind_chirho,
                span_chirho,
            } => {
                let term_chirho = self.infer_constraint_chirho(constraint_chirho)?;
                let kind_chirho = self.infer_type_chirho(kind_chirho)?;
                self.unify_kinds_chirho(
                    &kind_chirho.kind_chirho,
                    &ElaborationStateChirho::type_universe_chirho(),
                    *span_chirho,
                )?;
                self.unify_kinds_chirho(
                    &kind_chirho.term_chirho,
                    &ElaborationStateChirho::constraint_universe_chirho(),
                    *span_chirho,
                )?;
                Ok(term_chirho)
            }
        }
    }
}

/// These spellings are grammar-owned constructors that no module can define.
/// Ordinary identifiers, including Type/Constraint, never enter this path.
fn syntax_constructor_chirho(name_chirho: &NameChirho) -> Option<CompilerTypeConstructorChirho> {
    if name_chirho.qualifier_chirho().is_some() {
        return None;
    }
    match name_chirho.text_chirho() {
        "~" | "∼" => Some(CompilerTypeConstructorChirho::EqualityChirho(false)),
        "~~" => Some(CompilerTypeConstructorChirho::EqualityChirho(true)),
        "[]" => Some(CompilerTypeConstructorChirho::ListChirho),
        "->" | "(->)" => Some(CompilerTypeConstructorChirho::FunctionChirho),
        "()" => Some(CompilerTypeConstructorChirho::UnitChirho),
        text_chirho
            if text_chirho.starts_with('(')
                && text_chirho.ends_with(')')
                && text_chirho.len() > 2
                && text_chirho[1..text_chirho.len() - 1]
                    .bytes()
                    .all(|byte_chirho| byte_chirho == b',') =>
        {
            let arity_chirho = text_chirho.len() - 1;
            u8::try_from(arity_chirho)
                .ok()
                .filter(|arity_chirho| (2..=62).contains(arity_chirho))
                .map(CompilerTypeConstructorChirho::TupleChirho)
        }
        _ => None,
    }
}

mod pattern_scope_chirho;
