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
            let (name_chirho, binders_chirho, result_constraint_chirho) = match declaration_chirho {
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
                }
                | DeclChirho::TypeFamilyDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    ..
                } => (name_chirho, type_vars_chirho, false),
                DeclChirho::ClassDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    ..
                } => (name_chirho, type_vars_chirho, true),
                _ => continue,
            };
            self.predeclare_header_chirho(name_chirho, binders_chirho, result_constraint_chirho)?;
        }
        for declaration_chirho in &mut module_chirho.decls_chirho {
            if let DeclChirho::ClassDeclChirho {
                associated_tfs_chirho,
                ..
            } = declaration_chirho
            {
                for family_chirho in associated_tfs_chirho {
                    self.predeclare_header_chirho(
                        &mut family_chirho.name_chirho,
                        &mut family_chirho.type_vars_chirho,
                        false,
                    )?;
                }
            }
        }
        Ok(())
    }

    fn predeclare_header_chirho(
        &mut self,
        name_chirho: &mut haskelujah_ast_chirho::name_chirho::NameChirho,
        binders_chirho: &mut [AstTyVarChirho],
        result_constraint_chirho: bool,
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
            self.stamp_variable_chirho(&mut binder_chirho.name_chirho, variable_chirho);
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
                variable_chirho,
                kind_chirho,
                visibility_chirho: if binder_chirho.is_visible_chirho() {
                    TelescopeVisibilityChirho::RequiredChirho
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
            source_variables_chirho: binders_chirho.iter().enumerate().zip(telescope_chirho.iter()).map(|((index_chirho, binder_chirho), typed_chirho)| crate::ty_chirho::declarations_chirho::SourceDeclarationVariableChirho {
                source_chirho: haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho::QuantifiedChirho { declaration_chirho: key_chirho.clone(), index_chirho: index_chirho as u32 },
                variable_chirho: typed_chirho.variable_chirho,
                specificity_chirho: binder_chirho.specificity_chirho,
            }).collect(),
            identity_chirho: key_chirho.clone(),
            owner_chirho: Some(self.state_chirho.owner_chirho),
            role_chirho: proof_chirho.role_chirho,
            declaration_chirho: Some(proof_chirho.clone()),
            binders_chirho: telescope_chirho,
            result_kind_chirho,
            synonym_rhs_chirho: None,
            visible_arity_chirho,
            reduction_arity_chirho: binders_chirho.len(),
            promoted_chirho: false,
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
        self.declaration_variables_chirho
            .insert(key_chirho, source_binders_chirho);
        self.close_scope_chirho(mark_chirho);
        Ok(())
    }

    fn seed_headers_chirho(
        &mut self,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        // A canonical compiler declaration may exist without a public source
        // alias. Lookup still requires the naming/grammar producer's identity.
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
        for index_chirho in 0..variables_chirho.len() {
            let (name_chirho, variable_chirho, kind_chirho) =
                &self.declaration_variables_chirho[key_chirho][index_chirho];
            let name_chirho = self.state_chirho.name_chirho(name_chirho, span_chirho)?;
            let variable_chirho = *variable_chirho;
            let kind_chirho = self.state_chirho.copy_chirho(kind_chirho, span_chirho)?;
            self.bind_variable_chirho(name_chirho, variable_chirho, kind_chirho, span_chirho)?;
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
            self.stamp_variable_chirho(&mut binder_chirho.name_chirho, variable_chirho);
            if let Some(kind_chirho) = &mut binder_chirho.kind_annotation_chirho {
                let annotation_chirho = self.infer_type_chirho(kind_chirho)?;
                self.state_chirho.unify_chirho(
                    &annotation_chirho.kind_chirho,
                    &ElaborationStateChirho::type_universe_chirho(),
                    kind_chirho.span_chirho(),
                )?;
                self.state_chirho.unify_chirho(
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
        keys_chirho: Vec<Arc<str>>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        for key_chirho in keys_chirho {
            let Some(scheme_chirho) = self
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(&key_chirho)
                .cloned()
            else {
                return Err(Self::error_chirho(
                    "missing declaration at publication",
                    span_chirho,
                ));
            };
            let mut binders_chirho = Vec::new();
            let mut quantified_chirho = std::collections::BTreeSet::new();
            let result_kind_chirho = if scheme_chirho.promoted_chirho {
                self.promoted_result_prefix_chirho(&scheme_chirho.result_kind_chirho, span_chirho)?
            } else {
                self.state_chirho
                    .copy_chirho(&scheme_chirho.result_kind_chirho, span_chirho)?
            };
            let result_kind_chirho = self
                .state_chirho
                .resolve_chirho(&result_kind_chirho, span_chirho)?;
            quantified_chirho.extend(result_kind_chirho.free_vars_chirho());
            for binder_chirho in &scheme_chirho.binders_chirho {
                let kind_chirho = self
                    .state_chirho
                    .resolve_chirho(&binder_chirho.kind_chirho, span_chirho)?;
                quantified_chirho.extend(kind_chirho.free_vars_chirho());
                binders_chirho.push(DeclarationBinderChirho {
                    variable_chirho: binder_chirho.variable_chirho,
                    kind_chirho,
                    visibility_chirho: binder_chirho.visibility_chirho,
                });
            }
            for binder_chirho in &binders_chirho {
                quantified_chirho.remove(&binder_chirho.variable_chirho);
            }
            let mut pending_chirho = quantified_chirho.iter().copied().collect::<Vec<_>>();
            while let Some(variable_chirho) = pending_chirho.pop() {
                let kind_chirho = self
                    .state_chirho
                    .variable_kind_chirho(variable_chirho, span_chirho)?;
                let kind_chirho = self
                    .state_chirho
                    .resolve_chirho(&kind_chirho, span_chirho)?;
                for dependency_chirho in kind_chirho.free_vars_chirho() {
                    if !binders_chirho
                        .iter()
                        .any(|binder_chirho| binder_chirho.variable_chirho == dependency_chirho)
                        && quantified_chirho.insert(dependency_chirho)
                    {
                        pending_chirho.push(dependency_chirho);
                    }
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
                inferred_chirho.push(DeclarationBinderChirho {
                    variable_chirho,
                    kind_chirho,
                    visibility_chirho: TelescopeVisibilityChirho::InferredChirho,
                });
            }
            inferred_chirho.extend(binders_chirho);
            let mut rename_chirho = SubstChirho::empty_chirho();
            for (index_chirho, binder_chirho) in inferred_chirho.iter().enumerate() {
                rename_chirho.insert_chirho(
                    binder_chirho.variable_chirho,
                    TyChirho::VarChirho(TyVarChirho(index_chirho as u32)),
                );
            }
            let reduction_arity_chirho = inferred_chirho.len();
            let binders_chirho = inferred_chirho
                .into_iter()
                .enumerate()
                .map(|(index_chirho, binder_chirho)| DeclarationBinderChirho {
                    variable_chirho: TyVarChirho(index_chirho as u32),
                    kind_chirho: rename_chirho.apply_ty_chirho(&binder_chirho.kind_chirho),
                    visibility_chirho: binder_chirho.visibility_chirho,
                })
                .collect();
            let result_kind_chirho = rename_chirho.apply_ty_chirho(&result_kind_chirho);
            let synonym_rhs_chirho = scheme_chirho
                .synonym_rhs_chirho
                .as_ref()
                .map(|rhs_chirho| {
                    self.state_chirho
                        .resolve_chirho(rhs_chirho, span_chirho)
                        .map(|rhs_chirho| rename_chirho.apply_ty_chirho(&rhs_chirho))
                })
                .transpose()?;
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
                            source_chirho: source_chirho.source_chirho.clone(),
                            variable_chirho: *renamed_chirho,
                            specificity_chirho: source_chirho.specificity_chirho,
                        },
                    )
                })
                .collect::<Result<Vec<_>, ElaborationErrorChirho>>()?;
            let exported_chirho = DeclarationSchemeChirho {
                source_variables_chirho,
                identity_chirho: key_chirho.clone(),
                owner_chirho: None,
                role_chirho: scheme_chirho.role_chirho,
                declaration_chirho: scheme_chirho.declaration_chirho.clone(),
                binders_chirho,
                result_kind_chirho,
                synonym_rhs_chirho,
                visible_arity_chirho: scheme_chirho.visible_arity_chirho,
                reduction_arity_chirho,
                promoted_chirho: scheme_chirho.promoted_chirho,
                held_chirho: scheme_chirho.held_chirho,
            };
            if let Some(contract_chirho) = self.declaration_contracts_chirho.get(&key_chirho)
                && contract_chirho.as_ref() != &exported_chirho
            {
                return Err(Self::error_chirho(
                    "source declaration does not match its imported or boot telescope",
                    span_chirho,
                ));
            }
            self.registry_chirho
                .declaration_schemes_mut_chirho()
                .publish_chirho(exported_chirho)
                .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))?;
        }
        Ok(())
    }
}
