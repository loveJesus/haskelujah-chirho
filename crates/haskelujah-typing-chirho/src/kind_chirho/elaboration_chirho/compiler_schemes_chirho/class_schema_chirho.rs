// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Materialize the compiler identity's one shared ordered telescope into the checked arena.
//! workflow: compiler-pipeline-chirho/builtin-class-identity-chirho

use super::*;
use haskelujah_ast_chirho::decl_chirho::{TyVarSpecificityChirho, TyVarVisibilityChirho};
use haskelujah_ast_chirho::name_chirho::{
    CompilerClassChirho, CompilerClassGroundKindChirho, CompilerClassKindAtomChirho,
    CompilerClassKindChirho,
};

impl TypeElaboratorChirho<'_> {
    pub(super) fn compiler_class_kind_chirho(
        &mut self,
        kind_chirho: &CompilerClassKindChirho,
        preceding_slots_chirho: usize,
        depth_chirho: usize,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        if depth_chirho >= 128 {
            return Err(self.state_chirho.resource_failure_chirho(
                "compiler class kind depth bound exhausted",
                span_chirho,
            ));
        }
        match kind_chirho {
            CompilerClassKindChirho::AtomChirho(CompilerClassKindAtomChirho::BoundSlotChirho(
                index_chirho,
            )) => {
                if *index_chirho as usize >= preceding_slots_chirho {
                    return Err(Self::error_chirho(
                        "compiler class kind references an unbound telescope slot",
                        span_chirho,
                    ));
                }
                Ok(TyChirho::VarChirho(TyVarChirho(*index_chirho)))
            }
            CompilerClassKindChirho::AtomChirho(CompilerClassKindAtomChirho::GroundChirho(
                ground_chirho,
            )) => {
                // This branch alone joins the pure shared schema to 688's existing checked
                // declaration vocabulary. Nothing recovers an identity from a spelling.
                let constructor_chirho = match *ground_chirho {
                    CompilerClassGroundKindChirho::TYPE_CHIRHO => {
                        CompilerTypeConstructorChirho::TypeUniverseChirho
                    }
                    CompilerClassGroundKindChirho::CONSTRAINT_CHIRHO => {
                        CompilerTypeConstructorChirho::ConstraintUniverseChirho
                    }
                    CompilerClassGroundKindChirho::NAT_CHIRHO => {
                        CompilerTypeConstructorChirho::NatChirho
                    }
                    CompilerClassGroundKindChirho::SYMBOL_CHIRHO => {
                        CompilerTypeConstructorChirho::SymbolChirho
                    }
                    CompilerClassGroundKindChirho::CHAR_CHIRHO => {
                        CompilerTypeConstructorChirho::CharChirho
                    }
                    _ => {
                        return Err(Self::error_chirho(
                            "unsupported compiler class ground-kind atom",
                            span_chirho,
                        ));
                    }
                };
                let proof_chirho =
                    TypeConstructorDeclarationChirho::compiler_data_chirho(constructor_chirho);
                Ok(TyChirho::ConChirho(
                    self.declaration_key_chirho(&proof_chirho, span_chirho)?
                        .to_string(),
                ))
            }
            CompilerClassKindChirho::ArrowChirho(argument_chirho, result_chirho) => {
                let argument_chirho = self.compiler_class_kind_chirho(
                    argument_chirho,
                    preceding_slots_chirho,
                    depth_chirho + 1,
                    span_chirho,
                )?;
                let result_chirho = self.compiler_class_kind_chirho(
                    result_chirho,
                    preceding_slots_chirho,
                    depth_chirho + 1,
                    span_chirho,
                )?;
                Ok(ElaborationStateChirho::arrow_chirho(
                    argument_chirho,
                    result_chirho,
                ))
            }
        }
    }

    /// No public spelling or Type-only arity can mint this compiler identity or its kind.
    pub(in crate::kind_chirho::elaboration_chirho) fn seed_class_header_chirho(
        &mut self,
        class_chirho: CompilerClassChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let proof_chirho = TypeConstructorDeclarationChirho::compiler_data_chirho(
            CompilerTypeConstructorChirho::ClassChirho(class_chirho),
        );
        let identity_chirho = self.declaration_key_chirho(&proof_chirho, span_chirho)?;
        if self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(&identity_chirho)
            .is_some()
        {
            return Ok(());
        }
        self.state_chirho.charge_chirho(128, 128, span_chirho)?;
        let schema_chirho = class_chirho.schema_chirho();
        let mut binders_chirho = Vec::new();
        for binder_chirho in schema_chirho.binders_chirho {
            let kind_chirho = self.compiler_class_kind_chirho(
                binder_chirho.kind_chirho,
                binders_chirho.len(),
                0,
                span_chirho,
            )?;
            binders_chirho.push(DeclarationBinderChirho {
                term_specificity_chirho: None,
                variable_chirho: TyVarChirho(binders_chirho.len() as u32),
                kind_chirho,
                visibility_chirho: match (
                    binder_chirho.visibility_chirho,
                    binder_chirho.specificity_chirho,
                ) {
                    (TyVarVisibilityChirho::VisibleChirho, _) => {
                        TelescopeVisibilityChirho::RequiredChirho
                    }
                    (
                        TyVarVisibilityChirho::InvisibleChirho,
                        TyVarSpecificityChirho::SpecifiedChirho,
                    ) => TelescopeVisibilityChirho::SpecifiedChirho,
                    (
                        TyVarVisibilityChirho::InvisibleChirho,
                        TyVarSpecificityChirho::InferredChirho,
                    ) => TelescopeVisibilityChirho::InferredChirho,
                },
            });
        }
        let result_kind_chirho = self.compiler_class_kind_chirho(
            schema_chirho.result_kind_chirho,
            binders_chirho.len(),
            0,
            span_chirho,
        )?;
        let visible_arity_chirho = binders_chirho
            .iter()
            .filter(|binder_chirho| {
                binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho
            })
            .count();
        self.registry_chirho
            .declaration_schemes_mut_chirho()
            .insert_chirho(DeclarationSchemeChirho {
                injectivity_chirho: None,
                association_chirho:
                    crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho,
                identity_chirho,
                owner_chirho: None,
                role_chirho: TypeConstructorRoleChirho::ClassChirho,
                declaration_chirho: Some(proof_chirho),
                reduction_arity_chirho: binders_chirho.len(),
                visible_arity_chirho,
                binders_chirho,
                source_variables_chirho: Vec::new(),
                result_kind_chirho,
                synonym_rhs_chirho: None,
                synonym_syntax_chirho: None,
                local_binders_chirho: Arc::from([]),
                promoted_chirho: None,
                held_chirho: false,
            })
            .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))
    }
}
