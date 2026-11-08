// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Producer-owned quantifier correspondence shared by ordinary and GADT constructors.

use super::*;

impl TypeElaboratorChirho<'_> {
    /// The provisional promoted scheme already owns its parent prefix and field
    /// boundary. Retain written local quantifiers there, so publication discovers
    /// only genuinely inferred dependencies rather than moving locals ahead of parents.
    pub(super) fn retain_h98_quantifier_telescope_chirho(
        &mut self,
        scheme_chirho: &mut DeclarationSchemeChirho,
        sources_chirho: &[crate::ty_chirho::declarations_chirho::SourceDeclarationVariableChirho],
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(sources_chirho.len(), sources_chirho.len(), span_chirho)?;
        let mut locals_chirho = Vec::new();
        for source_chirho in sources_chirho {
            let kind_chirho = self
                .state_chirho
                .variable_kind_chirho(source_chirho.variable_chirho, span_chirho)?;
            locals_chirho.push(DeclarationBinderChirho {
                term_specificity_chirho: Some(source_chirho.specificity_chirho),
                variable_chirho: source_chirho.variable_chirho,
                kind_chirho,
                visibility_chirho: match source_chirho.specificity_chirho {
                    haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho => {
                        TelescopeVisibilityChirho::SpecifiedChirho
                    }
                    haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho => {
                        TelescopeVisibilityChirho::InferredChirho
                    }
                },
            });
        }
        let boundary_chirho = scheme_chirho
            .binders_chirho
            .iter()
            .position(|binder_chirho| {
                binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho
            })
            .unwrap_or(scheme_chirho.binders_chirho.len());
        scheme_chirho
            .binders_chirho
            .splice(boundary_chirho..boundary_chirho, locals_chirho);
        Ok(())
    }

    pub(super) fn bind_h98_constructor_prefix_chirho(
        &mut self,
        prefix_chirho: &mut haskelujah_ast_chirho::decl_chirho::ConPrefixChirho,
        explicit_chirho: &mut std::collections::HashSet<TyVarChirho>,
        source_variables_chirho: &mut Vec<
            crate::ty_chirho::declarations_chirho::SourceDeclarationVariableChirho,
        >,
        span_chirho: SpanChirho,
    ) -> Result<bool, ElaborationErrorChirho> {
        if prefix_chirho.unread_chirho {
            return Err(Self::error_chirho(
                "constructor quantification was not faithfully lowered",
                span_chirho,
            ));
        }
        self.bind_constructor_quantifiers_chirho(
            &mut prefix_chirho.existential_vars_chirho,
            explicit_chirho,
            source_variables_chirho,
            span_chirho,
        )?;
        for constraint_chirho in &mut prefix_chirho.context_chirho {
            self.infer_constraint_chirho(constraint_chirho)?;
        }
        Ok(!prefix_chirho.context_chirho.is_empty())
    }

    /// Bind an actual constructor prefix once and retain its producer-owned AST correspondence.
    pub(super) fn bind_constructor_quantifiers_chirho(
        &mut self,
        binders_chirho: &mut [haskelujah_ast_chirho::decl_chirho::TyVarChirho],
        explicit_chirho: &mut std::collections::HashSet<TyVarChirho>,
        source_variables_chirho: &mut Vec<
            crate::ty_chirho::declarations_chirho::SourceDeclarationVariableChirho,
        >,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let bound_chirho = self.bind_forall_chirho(binders_chirho)?;
        for (binder_chirho, variable_chirho) in binders_chirho.iter().zip(&bound_chirho) {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            let source_chirho = binder_chirho
                .name_chirho
                .type_variable_chirho()
                .cloned()
                .ok_or_else(|| {
                    Self::error_chirho(
                        "checked constructor forall lost its source owner",
                        span_chirho,
                    )
                })?;
            source_variables_chirho.push(
                crate::ty_chirho::declarations_chirho::SourceDeclarationVariableChirho {
                    header_index_chirho: u32::try_from(source_variables_chirho.len()).map_err(
                        |_| Self::error_chirho("constructor binder count overflow", span_chirho),
                    )?,
                    source_chirho,
                    variable_chirho: *variable_chirho,
                    specificity_chirho: binder_chirho.specificity_chirho,
                },
            );
        }
        explicit_chirho.extend(bound_chirho);
        Ok(())
    }
}
