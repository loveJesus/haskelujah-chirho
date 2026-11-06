// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declaration telescopes precede bodies, so later and recursive heads have
//! the same identity and kind variables as earlier heads.

use super::*;
use crate::subst_chirho::SubstChirho;
use haskelujah_ast_chirho::decl_chirho::{DeclChirho, TyVarChirho as AstTyVarChirho};
use haskelujah_ast_chirho::name_chirho::{
    CompilerTypeConstructorChirho, TypeConstructorIdentityChirho,
};

impl TypeElaboratorChirho<'_> {
    pub(super) fn predeclare_chirho(
        &mut self,
        module_chirho: &mut ModuleChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.seed_headers_chirho(module_chirho.span_chirho)?;
        for declaration_chirho in &mut module_chirho.decls_chirho {
            let (name_chirho, binders_chirho, result_constraint_chirho, injectivity_chirho) =
                match declaration_chirho {
                    DeclChirho::DataDeclChirho {
                        name_chirho,
                        type_vars_chirho,
                        ..
                    }
                    | DeclChirho::NewtypeDeclChirho {
                        name_chirho,
                        type_vars_chirho,
                        ..
                    }
                    | DeclChirho::TypeAliasDeclChirho {
                        name_chirho,
                        type_vars_chirho,
                        ..
                    } => (name_chirho, type_vars_chirho, false, None),
                    DeclChirho::TypeFamilyDeclChirho {
                        injectivity_chirho,
                        name_chirho,
                        type_vars_chirho,
                        ..
                    } => (name_chirho, type_vars_chirho, false, *injectivity_chirho),
                    DeclChirho::ClassDeclChirho {
                        name_chirho,
                        type_vars_chirho,
                        ..
                    } => (name_chirho, type_vars_chirho, true, None),
                    _ => continue,
                };
            self.predeclare_header_chirho(
                name_chirho,
                binders_chirho,
                result_constraint_chirho,
                injectivity_chirho,
            )?;
        }
        for declaration_chirho in &mut module_chirho.decls_chirho {
            if let DeclChirho::ClassDeclChirho {
                name_chirho,
                associated_tfs_chirho,
                ..
            } = declaration_chirho
            {
                self.predeclare_associated_chirho(name_chirho, associated_tfs_chirho)?;
            }
        }
        Ok(())
    }

    pub(super) fn predeclare_header_chirho(
        &mut self,
        name_chirho: &mut haskelujah_ast_chirho::name_chirho::NameChirho,
        binders_chirho: &mut [AstTyVarChirho],
        result_constraint_chirho: bool,
        injectivity_chirho: Option<bool>,
    ) -> Result<(), ElaborationErrorChirho> {
        let Some(proof_chirho) = self.declaration_chirho(name_chirho)? else {
            return Ok(());
        };
        let key_chirho = self.declaration_key_chirho(&proof_chirho, name_chirho.span_chirho())?;
        let mark_chirho = self.scope_undo_chirho.len();
        let mut telescope_chirho = Vec::new();
        let mut source_binders_chirho = Vec::new();
        for binder_chirho in binders_chirho.iter_mut() {
            let kind_chirho = self.fresh_kind_chirho(binder_chirho.name_chirho.span_chirho())?;
            let stored_kind_chirho = self
                .state_chirho
                .copy_chirho(&kind_chirho, binder_chirho.name_chirho.span_chirho())?;
            let variable_chirho = self.state_chirho.fresh_chirho(
                Some(stored_kind_chirho),
                binder_chirho.name_chirho.span_chirho(),
            )?;
            self.state_chirho
                .mark_universal_chirho(variable_chirho, binder_chirho.name_chirho.span_chirho())?;
            self.stamp_variable_chirho(&mut binder_chirho.name_chirho, variable_chirho)?;
            let text_chirho = self.state_chirho.name_chirho(
                binder_chirho.text_chirho(),
                binder_chirho.name_chirho.span_chirho(),
            )?;
            let stored_kind_chirho = self
                .state_chirho
                .copy_chirho(&kind_chirho, binder_chirho.name_chirho.span_chirho())?;
            self.bind_variable_chirho(
                text_chirho,
                variable_chirho,
                stored_kind_chirho,
                binder_chirho.name_chirho.span_chirho(),
            )?;
            let text_chirho = self.state_chirho.name_chirho(
                binder_chirho.text_chirho(),
                binder_chirho.name_chirho.span_chirho(),
            )?;
            let stored_kind_chirho = self
                .state_chirho
                .copy_chirho(&kind_chirho, binder_chirho.name_chirho.span_chirho())?;
            source_binders_chirho.push((text_chirho, variable_chirho, stored_kind_chirho));
            telescope_chirho.push(DeclarationBinderChirho {
                term_specificity_chirho: None,
                variable_chirho,
                kind_chirho,
                visibility_chirho: if binder_chirho.is_visible_chirho() {
                    TelescopeVisibilityChirho::RequiredChirho
                } else if binder_chirho.specificity_chirho
                    == haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho
                {
                    TelescopeVisibilityChirho::InferredChirho
                } else {
                    TelescopeVisibilityChirho::SpecifiedChirho
                },
            });
        }
        let result_kind_chirho = if result_constraint_chirho {
            ElaborationStateChirho::constraint_universe_chirho()
        } else {
            self.fresh_kind_chirho(name_chirho.span_chirho())?
        };
        let visible_arity_chirho = telescope_chirho
            .iter()
            .filter(|binder_chirho| {
                binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho
            })
            .count();
        let scheme_chirho = DeclarationSchemeChirho {
            injectivity_chirho,
                association_chirho: crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho,
            source_variables_chirho: binders_chirho.iter().enumerate().zip(telescope_chirho.iter()).map(|((index_chirho, binder_chirho), typed_chirho)| crate::ty_chirho::declarations_chirho::SourceDeclarationVariableChirho {
                header_index_chirho: index_chirho as u32,
                source_chirho: haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho::QuantifiedChirho { declaration_chirho: key_chirho.clone(), index_chirho: typed_chirho.variable_chirho.0 },
                variable_chirho: typed_chirho.variable_chirho,
                specificity_chirho: binder_chirho.specificity_chirho,
            }).collect(),
            identity_chirho: key_chirho.clone(),
            owner_chirho: self.state_chirho.owner_chirho,
            role_chirho: proof_chirho.role_chirho,
            declaration_chirho: Some(proof_chirho.clone()),
            binders_chirho: telescope_chirho,
            result_kind_chirho,
            synonym_rhs_chirho: None,
            synonym_syntax_chirho: None,
            local_binders_chirho: std::sync::Arc::from([]),
            visible_arity_chirho,
            reduction_arity_chirho: binders_chirho.len(),
            promoted_chirho: None,
            held_chirho: false,
        };
        self.state_chirho
            .charge_chirho(1, 1, name_chirho.span_chirho())?;
        if let Some(previous_chirho) = self
            .registry_chirho
            .declaration_schemes_mut_chirho()
            .stage_chirho(scheme_chirho)
        {
            self.declaration_contracts_chirho
                .insert(key_chirho.clone(), previous_chirho);
        }
        name_chirho.set_type_constructor_chirho(Some(proof_chirho));
        self.declaration_variables_chirho.insert(
            key_chirho,
            DeclarationVariablesChirho {
                constructor_result_origin_chirho: None,
                written_signature_chirho: None,
                owned_result_kind_chirho: None,
                written_chirho: source_binders_chirho,
                publication_chirho: SubstChirho::empty_chirho(),
                constructor_inferred_chirho: std::collections::HashSet::new(),
                fixed_field_kinds_chirho: Vec::new(),
            },
        );
        self.close_scope_chirho(mark_chirho);
        Ok(())
    }

    fn seed_headers_chirho(
        &mut self,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        // A canonical compiler declaration may exist without a public source
        // alias. Lookup still requires the naming/grammar producer's identity.
        for representation_chirho in
            haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho::ALL_CHIRHO
        {
            self.seed_compiler_header_chirho(
                CompilerTypeConstructorChirho::RepresentationChirho(representation_chirho),
                span_chirho,
            )?;
        }
        for constructor_chirho in [
            CompilerTypeConstructorChirho::TypeUniverseChirho,
            CompilerTypeConstructorChirho::ConstraintUniverseChirho,
            CompilerTypeConstructorChirho::BoolChirho,
            CompilerTypeConstructorChirho::OrderingChirho,
            CompilerTypeConstructorChirho::MaybeChirho,
            CompilerTypeConstructorChirho::EitherChirho,
            CompilerTypeConstructorChirho::UnitChirho,
            CompilerTypeConstructorChirho::ProxyChirho,
            CompilerTypeConstructorChirho::IntChirho,
            CompilerTypeConstructorChirho::IntegerChirho,
            CompilerTypeConstructorChirho::CharChirho,
            CompilerTypeConstructorChirho::FloatChirho,
            CompilerTypeConstructorChirho::DoubleChirho,
            CompilerTypeConstructorChirho::IoChirho,
            CompilerTypeConstructorChirho::ListChirho,
            CompilerTypeConstructorChirho::FunctionChirho,
        ] {
            self.seed_compiler_header_chirho(constructor_chirho, span_chirho)?;
        }
        // Only actual typed compiler data producers and the naming-proven class
        // supplier scope seed these schemes. No unfamiliar spelling gains one.
        for (_, proof_chirho) in self.type_scope_chirho.iter_chirho() {
            let TypeConstructorIdentityChirho::CompilerChirho(constructor_chirho) =
                &proof_chirho.identity_chirho
            else {
                continue;
            };
            let constructor_chirho = *constructor_chirho;
            self.seed_compiler_header_chirho(constructor_chirho, span_chirho)?;
        }

        for (_, class_chirho) in self.builtin_classes_chirho.iter_chirho() {
            self.seed_class_header_chirho(class_chirho.identity_chirho, span_chirho)?;
        }
        Ok(())
    }

    pub(super) fn bind_header_scope_chirho(
        &mut self,
        key_chirho: &Arc<str>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let Some(variables_chirho) = self.declaration_variables_chirho.get(key_chirho) else {
            return Err(Self::error_chirho(
                "missing local declaration telescope",
                span_chirho,
            ));
        };
        // Copy one bounded entry at a time; no per-scope map snapshot.
        for index_chirho in 0..variables_chirho.written_chirho.len() {
            let (name_chirho, variable_chirho, kind_chirho) =
                &self.declaration_variables_chirho[key_chirho].written_chirho[index_chirho];
            let name_chirho = self.state_chirho.name_chirho(name_chirho, span_chirho)?;
            let variable_chirho = *variable_chirho;
            let kind_chirho = self.state_chirho.copy_chirho(kind_chirho, span_chirho)?;
            self.bind_variable_chirho(name_chirho, variable_chirho, kind_chirho, span_chirho)?;
        }
        Ok(())
    }

    pub(super) fn bind_published_header_scope_chirho(
        &mut self,
        key_chirho: &Arc<str>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.bind_header_scope_chirho(key_chirho, span_chirho)?;
        let variables_chirho = &self.declaration_variables_chirho[key_chirho];
        for (name_chirho, variable_chirho, _) in &variables_chirho.written_chirho {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            if let Some(TyChirho::VarChirho(published_chirho)) = variables_chirho
                .publication_chirho
                .lookup_chirho(variable_chirho)
            {
                let record_chirho =
                    self.variables_chirho.get_mut(name_chirho).ok_or_else(|| {
                        Self::error_chirho(
                            "published header lost its lexical variable",
                            span_chirho,
                        )
                    })?;
                record_chirho.source_chirho = Some(haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho::QuantifiedChirho { declaration_chirho: key_chirho.clone(), index_chirho: published_chirho.0 });
            }
        }
        Ok(())
    }

    pub(super) fn check_binder_annotations_chirho(
        &mut self,
        binders_chirho: &mut [AstTyVarChirho],
    ) -> Result<(), ElaborationErrorChirho> {
        for binder_chirho in binders_chirho {
            let (variable_chirho, expected_kind_chirho) = self.variable_chirho(
                binder_chirho.text_chirho(),
                binder_chirho.name_chirho.span_chirho(),
            )?;
            self.stamp_variable_chirho(&mut binder_chirho.name_chirho, variable_chirho)?;
            if let Some(kind_chirho) = &mut binder_chirho.kind_annotation_chirho {
                let annotation_chirho = self.infer_kind_syntax_chirho(kind_chirho)?;
                self.unify_kinds_chirho(
                    &expected_kind_chirho,
                    &annotation_chirho.term_chirho,
                    kind_chirho.span_chirho(),
                )?;
            }
        }
        Ok(())
    }

    pub(super) fn finish_catalogue_chirho(
        &mut self,
        keys_chirho: &[Arc<str>],
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let mut associations_chirho = Vec::new();
        for key_chirho in keys_chirho {
            let Some(scheme_chirho) = self
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(key_chirho)
                .cloned()
            else {
                return Err(Self::error_chirho(
                    "missing declaration at publication",
                    span_chirho,
                ));
            };
            if scheme_chirho.owner_chirho.is_none() {
                continue;
            }
            let mut binders_chirho = Vec::new();
            let mut quantified_chirho = std::collections::BTreeSet::new();
            let result_kind_chirho = if scheme_chirho.promoted_chirho.is_some() {
                self.publish_constructor_result_chirho(
                    key_chirho,
                    &scheme_chirho.result_kind_chirho,
                    span_chirho,
                )?
            } else {
                self.state_chirho
                    .copy_chirho(&scheme_chirho.result_kind_chirho, span_chirho)?
            };
            let mut result_kind_chirho = self
                .state_chirho
                .resolve_chirho(&result_kind_chirho, span_chirho)?;
            quantified_chirho.extend(result_kind_chirho.free_vars_chirho());
            quantified_chirho.extend(
                scheme_chirho
                    .source_variables_chirho
                    .iter()
                    .map(|source_chirho| source_chirho.variable_chirho),
            );
            for binder_chirho in &scheme_chirho.binders_chirho {
                let kind_chirho = self
                    .state_chirho
                    .resolve_chirho(&binder_chirho.kind_chirho, span_chirho)?;
                quantified_chirho.extend(kind_chirho.free_vars_chirho());
                binders_chirho.push(DeclarationBinderChirho {
                    term_specificity_chirho: binder_chirho.term_specificity_chirho,
                    variable_chirho: binder_chirho.variable_chirho,
                    kind_chirho,
                    visibility_chirho: binder_chirho.visibility_chirho,
                });
            }
            for binder_chirho in &binders_chirho {
                quantified_chirho.remove(&binder_chirho.variable_chirho);
            }
            self.state_chirho.charge_chirho(
                quantified_chirho.len(),
                quantified_chirho.len(),
                span_chirho,
            )?;
            let roots_chirho = quantified_chirho.iter().copied().collect::<Vec<_>>();
            let header_dependencies_chirho = self
                .state_chirho
                .kind_dependency_closure_chirho(&roots_chirho, span_chirho)?;
            quantified_chirho.extend(header_dependencies_chirho);
            // A nested forall can introduce a value-kind Meta used only by its
            // binder kind, absent from the alias's outer result kind. Default
            // that actual RHS dependency before publication; it does not invent
            // an extra public declaration parameter.
            let mut representation_roots_chirho = roots_chirho;
            if let Some(rhs_chirho) = &scheme_chirho.synonym_rhs_chirho {
                let rhs_chirho = self.state_chirho.resolve_chirho(rhs_chirho, span_chirho)?;
                let rhs_variables_chirho = rhs_chirho.free_vars_chirho();
                self.state_chirho.charge_chirho(
                    rhs_variables_chirho.len(),
                    rhs_variables_chirho.len(),
                    span_chirho,
                )?;
                representation_roots_chirho.extend(rhs_variables_chirho);
            }
            let candidates_chirho = self
                .state_chirho
                .kind_dependency_closure_chirho(&representation_roots_chirho, span_chirho)?;
            let defaults_chirho = self.state_chirho.default_representation_variables_chirho(
                &candidates_chirho,
                &std::collections::HashSet::new(),
                span_chirho,
            )?;
            if !defaults_chirho.is_empty_chirho() {
                result_kind_chirho = self
                    .state_chirho
                    .resolve_chirho(&result_kind_chirho, span_chirho)?;
                quantified_chirho.clear();
                quantified_chirho.extend(result_kind_chirho.free_vars_chirho());
                quantified_chirho.extend(
                    scheme_chirho
                        .source_variables_chirho
                        .iter()
                        .map(|source_chirho| source_chirho.variable_chirho),
                );
                for binder_chirho in &mut binders_chirho {
                    binder_chirho.kind_chirho = self
                        .state_chirho
                        .resolve_chirho(&binder_chirho.kind_chirho, span_chirho)?;
                    quantified_chirho.extend(binder_chirho.kind_chirho.free_vars_chirho());
                }
                self.state_chirho.charge_chirho(
                    quantified_chirho.len(),
                    quantified_chirho.len(),
                    span_chirho,
                )?;
                let roots_chirho = quantified_chirho.iter().copied().collect::<Vec<_>>();
                quantified_chirho.extend(
                    self.state_chirho
                        .kind_dependency_closure_chirho(&roots_chirho, span_chirho)?,
                );
            }
            for binder_chirho in &binders_chirho {
                quantified_chirho.remove(&binder_chirho.variable_chirho);
            }
            if !self.poly_kinds_chirho {
                // A promoted constructor's field/result terms mention ordinary
                // type universals too. Only their producer-recorded inferred
                // KIND dependencies default; the universals remain quantified.
                let defaultable_chirho = if scheme_chirho.promoted_chirho.is_some() {
                    quantified_chirho
                        .iter()
                        .filter(|variable_chirho| {
                            self.declaration_variables_chirho
                                .get(key_chirho)
                                .is_some_and(|variables_chirho| {
                                    variables_chirho
                                        .constructor_inferred_chirho
                                        .contains(variable_chirho)
                                })
                        })
                        .copied()
                        .collect::<std::collections::BTreeSet<_>>()
                } else {
                    quantified_chirho.clone()
                };
                self.state_chirho.charge_chirho(
                    defaultable_chirho.len(),
                    defaultable_chirho.len(),
                    span_chirho,
                )?;
                for variable_chirho in defaultable_chirho {
                    self.unify_kinds_chirho(
                        &TyChirho::VarChirho(variable_chirho),
                        &ElaborationStateChirho::type_universe_chirho(),
                        span_chirho,
                    )?;
                }
                quantified_chirho.clear();
                result_kind_chirho = self
                    .state_chirho
                    .resolve_chirho(&result_kind_chirho, span_chirho)?;
                quantified_chirho.extend(result_kind_chirho.free_vars_chirho());
                for binder_chirho in &mut binders_chirho {
                    binder_chirho.kind_chirho = self
                        .state_chirho
                        .resolve_chirho(&binder_chirho.kind_chirho, span_chirho)?;
                    quantified_chirho.extend(binder_chirho.kind_chirho.free_vars_chirho());
                }
                for binder_chirho in &binders_chirho {
                    quantified_chirho.remove(&binder_chirho.variable_chirho);
                }
            }
            let mut inferred_chirho = Vec::new();
            while !quantified_chirho.is_empty() {
                let mut next_chirho = None;
                for variable_chirho in &quantified_chirho {
                    let kind_chirho = self
                        .state_chirho
                        .variable_kind_chirho(*variable_chirho, span_chirho)?;
                    let kind_chirho = self
                        .state_chirho
                        .resolve_chirho(&kind_chirho, span_chirho)?;
                    if kind_chirho
                        .free_vars_chirho()
                        .iter()
                        .all(|dependency_chirho| !quantified_chirho.contains(dependency_chirho))
                    {
                        next_chirho = Some((*variable_chirho, kind_chirho));
                        break;
                    }
                }
                let Some((variable_chirho, kind_chirho)) = next_chirho else {
                    return Err(Self::error_chirho(
                        "cyclic dependency in inferred kind telescope",
                        span_chirho,
                    ));
                };
                quantified_chirho.remove(&variable_chirho);
                inferred_chirho.push(DeclarationBinderChirho { term_specificity_chirho: scheme_chirho.promoted_chirho.as_ref().map(|_| {
                    self.state_chirho.source_specificity_chirho(variable_chirho)
                        .unwrap_or(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho)
                }),
                    variable_chirho,
                    kind_chirho,
                    visibility_chirho: if !self.declaration_variables_chirho.get(key_chirho).is_some_and(|variables_chirho| variables_chirho.constructor_inferred_chirho.contains(&variable_chirho)) && self.state_chirho.source_specificity_chirho(variable_chirho) == Some(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho) { TelescopeVisibilityChirho::SpecifiedChirho } else { TelescopeVisibilityChirho::InferredChirho },
                });
            }
            inferred_chirho.extend(binders_chirho);
            if scheme_chirho.promoted_chirho.is_some() {
                for binder_chirho in &mut inferred_chirho {
                    if binder_chirho.visibility_chirho != TelescopeVisibilityChirho::RequiredChirho
                    {
                        binder_chirho.term_specificity_chirho =
                            binder_chirho.term_specificity_chirho.or_else(|| {
                                self.state_chirho
                                    .source_specificity_chirho(binder_chirho.variable_chirho)
                            });
                    }
                }
            }
            for binder_chirho in &inferred_chirho {
                self.state_chirho
                    .mark_universal_chirho(binder_chirho.variable_chirho, span_chirho)?;
            }
            let mut rename_chirho = SubstChirho::empty_chirho();
            for (index_chirho, binder_chirho) in inferred_chirho.iter().enumerate() {
                rename_chirho.insert_chirho(
                    binder_chirho.variable_chirho,
                    TyChirho::VarChirho(TyVarChirho(index_chirho as u32)),
                );
            }
            let reduction_arity_chirho = inferred_chirho.len();
            // Alpha-publish nested kind quantifiers before any owner IDs are
            // erased. Alias bodies complete later in transparent dependency order.
            let owned_result_kind_chirho = self
                .state_chirho
                .copy_chirho(&result_kind_chirho, span_chirho)?;
            self.declaration_variables_chirho
                .entry(key_chirho.clone())
                .or_default()
                .owned_result_kind_chirho = Some(owned_result_kind_chirho);
            let published_chirho = self.publish_header_terms_chirho(
                key_chirho,
                &inferred_chirho,
                &result_kind_chirho,
                &rename_chirho,
                span_chirho,
            )?;
            let binders_chirho = published_chirho.binders_chirho;
            let result_kind_chirho = published_chirho.result_kind_chirho;
            let local_binders_chirho = published_chirho.local_binders_chirho;
            let rename_chirho = published_chirho.mapping_chirho;
            let synonym_rhs_chirho = None;
            let source_variables_chirho = scheme_chirho
                .source_variables_chirho
                .iter()
                .map(|source_chirho| {
                    let Some(TyChirho::VarChirho(renamed_chirho)) =
                        rename_chirho.lookup_chirho(&source_chirho.variable_chirho)
                    else {
                        return Err(Self::error_chirho(
                            "source binder is absent from the published telescope",
                            span_chirho,
                        ));
                    };
                    Ok(
                        crate::ty_chirho::declarations_chirho::SourceDeclarationVariableChirho {
                            header_index_chirho: source_chirho.header_index_chirho,
                            source_chirho: haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho::QuantifiedChirho { declaration_chirho: key_chirho.clone(), index_chirho: renamed_chirho.0 },
                            variable_chirho: *renamed_chirho,
                            specificity_chirho: source_chirho.specificity_chirho,
                        },
                    )
                })
                .collect::<Result<Vec<_>, ElaborationErrorChirho>>()?;
            let association_weight_chirho = match &scheme_chirho.association_chirho {
                crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::ClassChirho { families_chirho } => families_chirho.len(),
                crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::FamilyChirho { correspondence_chirho, default_chirho: None, .. } => correspondence_chirho.len(),
                crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::FamilyChirho { default_chirho: Some(_), .. } => return Err(Self::error_chirho("associated default precedes its checked telescope", span_chirho)),
                crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho => 0,
            };
            self.state_chirho.charge_chirho(
                association_weight_chirho + 1,
                association_weight_chirho + 1,
                span_chirho,
            )?;
            associations_chirho
                .push((key_chirho.clone(), scheme_chirho.association_chirho.clone()));
            if let Some(promoted_chirho) = &scheme_chirho.promoted_chirho {
                self.state_chirho.charge_chirho(
                    promoted_chirho.as_str_chirho().len() + 1,
                    promoted_chirho.as_str_chirho().len() + 1,
                    span_chirho,
                )?;
            }
            let exported_chirho = DeclarationSchemeChirho {
                injectivity_chirho: scheme_chirho.injectivity_chirho,
                association_chirho:
                    crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho,
                source_variables_chirho,
                identity_chirho: key_chirho.clone(),
                owner_chirho: None,
                role_chirho: scheme_chirho.role_chirho,
                declaration_chirho: scheme_chirho.declaration_chirho.clone(),
                binders_chirho,
                result_kind_chirho,
                synonym_rhs_chirho,
                synonym_syntax_chirho: None,
                local_binders_chirho,
                visible_arity_chirho: scheme_chirho.visible_arity_chirho,
                reduction_arity_chirho,
                promoted_chirho: scheme_chirho.promoted_chirho.clone(),
                held_chirho: scheme_chirho.held_chirho,
            };
            self.declaration_variables_chirho
                .entry(key_chirho.clone())
                .or_default()
                .publication_chirho = rename_chirho;
            self.registry_chirho
                .declaration_schemes_mut_chirho()
                .publish_chirho(exported_chirho)
                .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))?;
        }
        self.publish_associations_chirho(associations_chirho, span_chirho)
    }
}
