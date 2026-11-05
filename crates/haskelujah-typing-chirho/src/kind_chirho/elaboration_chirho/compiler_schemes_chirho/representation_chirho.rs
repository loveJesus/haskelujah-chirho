// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Representation declarations use the ordinary checked catalogue. These
//! producers confer no authority on opaque names and activate no runtime path.

use super::*;
use haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho;

fn representation_term_chirho(representation_chirho: CompilerRepresentationChirho) -> TyChirho {
    TyChirho::ConChirho(representation_chirho.type_key_chirho().to_owned())
}

fn promoted_representation_term_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ConChirho(
        PromotedKeyChirho::builtin_chirho(BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, name_chirho)
            .as_str_chirho()
            .to_owned(),
    )
}

fn boxed_representation_term_chirho(lifted_chirho: bool) -> TyChirho {
    crate::ty_chirho::representation_chirho::boxed_representation_chirho(lifted_chirho).clone()
}

fn zero_bit_representation_term_chirho() -> TyChirho {
    // The checked nil application retains its inferred element-kind prefix.
    let nil_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho(
            PromotedKeyChirho::builtin_nil_chirho()
                .as_str_chirho()
                .to_owned(),
        )),
        Box::new(representation_term_chirho(
            CompilerRepresentationChirho::RuntimeRepChirho,
        )),
    );
    TyChirho::AppChirho(
        Box::new(promoted_representation_term_chirho("TupleRep")),
        Box::new(nil_chirho),
    )
}

impl TypeElaboratorChirho<'_> {
    pub(super) fn seed_representation_header_chirho(
        &mut self,
        representation_chirho: CompilerRepresentationChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let constructor_chirho =
            CompilerTypeConstructorChirho::RepresentationChirho(representation_chirho);
        let proof_chirho =
            TypeConstructorDeclarationChirho::compiler_data_chirho(constructor_chirho);
        let key_chirho = self.declaration_key_chirho(&proof_chirho, span_chirho)?;
        if self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(&key_chirho)
            .is_some()
        {
            return Ok(());
        }
        self.state_chirho.charge_chirho(128, 128, span_chirho)?;
        let runtime_rep_chirho =
            representation_term_chirho(CompilerRepresentationChirho::RuntimeRepChirho);
        let binders_chirho = if matches!(
            representation_chirho,
            CompilerRepresentationChirho::RuntimeTypeChirho
                | CompilerRepresentationChirho::RuntimeConstraintChirho
        ) {
            vec![DeclarationBinderChirho {
                variable_chirho: TyVarChirho(0),
                kind_chirho: runtime_rep_chirho.clone(),
                visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
            }]
        } else {
            Vec::new()
        };
        let synonym_rhs_chirho = match representation_chirho {
            CompilerRepresentationChirho::LiftedRepChirho => {
                Some(boxed_representation_term_chirho(true))
            }
            CompilerRepresentationChirho::UnliftedRepChirho => {
                Some(boxed_representation_term_chirho(false))
            }
            CompilerRepresentationChirho::ZeroBitRepChirho => {
                Some(zero_bit_representation_term_chirho())
            }
            CompilerRepresentationChirho::UnliftedTypeChirho => Some(TyChirho::AppChirho(
                Box::new(representation_term_chirho(
                    CompilerRepresentationChirho::RuntimeTypeChirho,
                )),
                Box::new(boxed_representation_term_chirho(false)),
            )),
            CompilerRepresentationChirho::ZeroBitTypeChirho => Some(TyChirho::AppChirho(
                Box::new(representation_term_chirho(
                    CompilerRepresentationChirho::RuntimeTypeChirho,
                )),
                Box::new(zero_bit_representation_term_chirho()),
            )),
            _ => None,
        };
        let result_kind_chirho = if matches!(
            representation_chirho,
            CompilerRepresentationChirho::LiftedRepChirho
                | CompilerRepresentationChirho::UnliftedRepChirho
                | CompilerRepresentationChirho::ZeroBitRepChirho
        ) {
            runtime_rep_chirho
        } else {
            ElaborationStateChirho::type_universe_chirho()
        };
        self.registry_chirho
            .declaration_schemes_mut_chirho()
            .insert_chirho(DeclarationSchemeChirho {
                identity_chirho: key_chirho.clone(),
                owner_chirho: None,
                declaration_chirho: Some(proof_chirho),
                role_chirho: representation_chirho.role_chirho(),
                injectivity_chirho: None,
                association_chirho:
                    crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho,
                visible_arity_chirho: binders_chirho.len(),
                reduction_arity_chirho: binders_chirho.len(),
                binders_chirho,
                source_variables_chirho: Vec::new(),
                result_kind_chirho,
                synonym_rhs_chirho,
                synonym_syntax_chirho: None,
                local_binders_chirho: Arc::from([]),
                promoted_chirho: None,
                held_chirho: false,
            })
            .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))?;
        // The closed RHS and parameter kinds carry these actual compiler
        // producers too. A consumer must not recover their authority from a
        // private key that happens to occur inside an expanded alias.
        match representation_chirho {
            CompilerRepresentationChirho::RuntimeRepChirho => {
                for dependency_chirho in [
                    CompilerRepresentationChirho::LevityChirho,
                    CompilerRepresentationChirho::VecCountChirho,
                    CompilerRepresentationChirho::VecElemChirho,
                ] {
                    self.seed_representation_header_chirho(dependency_chirho, span_chirho)?;
                }
            }
            CompilerRepresentationChirho::RuntimeTypeChirho
            | CompilerRepresentationChirho::RuntimeConstraintChirho
            | CompilerRepresentationChirho::LiftedRepChirho
            | CompilerRepresentationChirho::UnliftedRepChirho
            | CompilerRepresentationChirho::ZeroBitRepChirho => {
                self.seed_representation_header_chirho(
                    CompilerRepresentationChirho::RuntimeRepChirho,
                    span_chirho,
                )?;
            }
            CompilerRepresentationChirho::UnliftedTypeChirho
            | CompilerRepresentationChirho::ZeroBitTypeChirho => {
                self.seed_representation_header_chirho(
                    CompilerRepresentationChirho::RuntimeTypeChirho,
                    span_chirho,
                )?;
            }
            _ => {}
        }
        if matches!(
            representation_chirho,
            CompilerRepresentationChirho::ZeroBitRepChirho
                | CompilerRepresentationChirho::ZeroBitTypeChirho
        ) {
            self.seed_compiler_header_chirho(
                CompilerTypeConstructorChirho::ListChirho,
                span_chirho,
            )?;
        }
        for name_chirho in representation_chirho.constructors_chirho() {
            self.seed_representation_constructor_chirho(
                representation_chirho,
                name_chirho,
                span_chirho,
            )?;
        }
        Ok(())
    }

    fn seed_representation_constructor_chirho(
        &mut self,
        parent_chirho: CompilerRepresentationChirho,
        name_chirho: &str,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(128, 128, span_chirho)?;
        let promoted_key_chirho =
            PromotedKeyChirho::builtin_chirho(BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, name_chirho);
        let identity_chirho: Arc<str> = Arc::from(promoted_key_chirho.as_str_chirho());
        if self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(&identity_chirho)
            .is_some()
        {
            return Ok(());
        }
        let fields_chirho = match (parent_chirho, name_chirho) {
            (CompilerRepresentationChirho::RuntimeRepChirho, "BoxedRep") => {
                vec![representation_term_chirho(
                    CompilerRepresentationChirho::LevityChirho,
                )]
            }
            (CompilerRepresentationChirho::RuntimeRepChirho, "TupleRep" | "SumRep") => {
                vec![TyChirho::ListChirho(Box::new(representation_term_chirho(
                    CompilerRepresentationChirho::RuntimeRepChirho,
                )))]
            }
            (CompilerRepresentationChirho::RuntimeRepChirho, "VecRep") => vec![
                representation_term_chirho(CompilerRepresentationChirho::VecCountChirho),
                representation_term_chirho(CompilerRepresentationChirho::VecElemChirho),
            ],
            _ => Vec::new(),
        };
        let binders_chirho: Vec<_> = fields_chirho
            .into_iter()
            .enumerate()
            .map(|(index_chirho, kind_chirho)| DeclarationBinderChirho {
                variable_chirho: TyVarChirho(index_chirho as u32),
                kind_chirho,
                visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
            })
            .collect();
        self.registry_chirho
            .declaration_schemes_mut_chirho()
            .insert_chirho(DeclarationSchemeChirho {
                identity_chirho,
                owner_chirho: None,
                declaration_chirho: None,
                role_chirho: TypeConstructorRoleChirho::DataChirho,
                injectivity_chirho: None,
                association_chirho:
                    crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho,
                visible_arity_chirho: binders_chirho.len(),
                reduction_arity_chirho: binders_chirho.len(),
                binders_chirho,
                source_variables_chirho: Vec::new(),
                result_kind_chirho: representation_term_chirho(parent_chirho),
                synonym_rhs_chirho: None,
                synonym_syntax_chirho: None,
                local_binders_chirho: Arc::from([]),
                promoted_chirho: Some(promoted_key_chirho),
                held_chirho: false,
            })
            .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))
    }
}
