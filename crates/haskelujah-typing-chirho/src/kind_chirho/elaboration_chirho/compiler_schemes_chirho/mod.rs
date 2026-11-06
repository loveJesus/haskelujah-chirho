// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typed compiler declaration producers. Syntax-only list/tuple/arrow names
//! cannot be declared by a module; ordinary names require the naming proof.

use super::*;
mod class_schema_chirho;
mod representation_chirho;
#[cfg(test)]
mod tests_chirho;
use haskelujah_ast_chirho::name_chirho::{
    CompilerTypeConstructorChirho, TypeConstructorDeclarationChirho, TypeConstructorRoleChirho,
};
use haskelujah_ast_chirho::promotion_chirho::{
    BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, BUILTIN_TUPLE_MODULE_CHIRHO, PromotedKeyChirho,
};

impl TypeElaboratorChirho<'_> {
    fn seed_equality_header_chirho(
        &mut self,
        heterogeneous_chirho: bool,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let constructor_chirho =
            CompilerTypeConstructorChirho::EqualityChirho(heterogeneous_chirho);
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
        self.state_chirho.charge_chirho(16, 16, span_chirho)?;
        let mut binders_chirho = vec![DeclarationBinderChirho {
            term_specificity_chirho: None,
            variable_chirho: TyVarChirho(0),
            kind_chirho: ElaborationStateChirho::type_universe_chirho(),
            visibility_chirho: TelescopeVisibilityChirho::InferredChirho,
        }];
        if heterogeneous_chirho {
            binders_chirho.push(DeclarationBinderChirho {
                term_specificity_chirho: None,
                variable_chirho: TyVarChirho(1),
                kind_chirho: ElaborationStateChirho::type_universe_chirho(),
                visibility_chirho: TelescopeVisibilityChirho::InferredChirho,
            });
        }
        for index_chirho in 0..2 {
            binders_chirho.push(DeclarationBinderChirho {
                term_specificity_chirho: None,
                variable_chirho: TyVarChirho(binders_chirho.len() as u32),
                kind_chirho: TyChirho::VarChirho(TyVarChirho(if heterogeneous_chirho {
                    index_chirho
                } else {
                    0
                })),
                visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
            });
        }
        self.registry_chirho
            .declaration_schemes_mut_chirho()
            .insert_chirho(DeclarationSchemeChirho {
                injectivity_chirho: None,
                association_chirho:
                    crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho,
                identity_chirho: key_chirho,
                owner_chirho: None,
                declaration_chirho: Some(proof_chirho),
                role_chirho: TypeConstructorRoleChirho::UnknownChirho,
                reduction_arity_chirho: binders_chirho.len(),
                visible_arity_chirho: 2,
                binders_chirho,
                source_variables_chirho: Vec::new(),
                result_kind_chirho: ElaborationStateChirho::constraint_universe_chirho(),
                synonym_rhs_chirho: None,
                synonym_syntax_chirho: None,
                local_binders_chirho: std::sync::Arc::from([]),
                promoted_chirho: None,
                held_chirho: false,
            })
            .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))
    }

    pub(super) fn seed_compiler_header_chirho(
        &mut self,
        constructor_chirho: CompilerTypeConstructorChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        if let CompilerTypeConstructorChirho::RepresentationChirho(representation_chirho) =
            constructor_chirho
        {
            return self.seed_representation_header_chirho(representation_chirho, span_chirho);
        }
        if let CompilerTypeConstructorChirho::ClassChirho(class_chirho) = constructor_chirho {
            return self.seed_class_header_chirho(class_chirho, span_chirho);
        }

        if matches!(constructor_chirho,CompilerTypeConstructorChirho::TupleChirho(arity_chirho) if !(2..=62).contains(&arity_chirho))
        {
            return Err(Self::error_chirho(
                "compiler tuple arity is unsupported",
                span_chirho,
            ));
        }
        if let CompilerTypeConstructorChirho::EqualityChirho(heterogeneous_chirho) =
            constructor_chirho
        {
            return self.seed_equality_header_chirho(heterogeneous_chirho, span_chirho);
        }
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
        let type_chirho = ElaborationStateChirho::type_universe_chirho();
        let mut binders_chirho = Vec::new();
        let arity_chirho = match constructor_chirho {
            CompilerTypeConstructorChirho::MaybeChirho
            | CompilerTypeConstructorChirho::ProxyChirho
            | CompilerTypeConstructorChirho::IoChirho
            | CompilerTypeConstructorChirho::ListChirho => 1,
            CompilerTypeConstructorChirho::EitherChirho
            | CompilerTypeConstructorChirho::FunctionChirho => 2,
            CompilerTypeConstructorChirho::TupleChirho(arity_chirho) => arity_chirho as usize,
            _ => 0,
        };
        self.state_chirho
            .charge_chirho(arity_chirho + 2, arity_chirho + 2, span_chirho)?;
        if constructor_chirho == CompilerTypeConstructorChirho::ProxyChirho {
            binders_chirho.push(DeclarationBinderChirho {
                term_specificity_chirho: None,
                variable_chirho: TyVarChirho(0),
                kind_chirho: type_chirho.clone(),
                visibility_chirho: TelescopeVisibilityChirho::InferredChirho,
            });
            binders_chirho.push(DeclarationBinderChirho {
                term_specificity_chirho: None,
                variable_chirho: TyVarChirho(1),
                kind_chirho: TyChirho::VarChirho(TyVarChirho(0)),
                visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
            });
        } else {
            for index_chirho in 0..arity_chirho {
                binders_chirho.push(DeclarationBinderChirho {
                    term_specificity_chirho: None,
                    variable_chirho: TyVarChirho(index_chirho as u32),
                    kind_chirho: type_chirho.clone(),
                    visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
                });
            }
        }
        let result_kind_chirho = self
            .primitive_scalar_kind_chirho(constructor_chirho, span_chirho)?
            .unwrap_or(type_chirho);
        self.registry_chirho
            .declaration_schemes_mut_chirho()
            .insert_chirho(DeclarationSchemeChirho {
                injectivity_chirho: None,
                association_chirho:
                    crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho,
                source_variables_chirho: Vec::new(),
                identity_chirho: key_chirho.clone(),
                owner_chirho: None,
                role_chirho: proof_chirho.role_chirho,
                declaration_chirho: Some(proof_chirho),
                reduction_arity_chirho: binders_chirho.len(),
                visible_arity_chirho: arity_chirho,
                binders_chirho,
                result_kind_chirho,
                synonym_rhs_chirho: None,
                synonym_syntax_chirho: None,
                local_binders_chirho: std::sync::Arc::from([]),
                promoted_chirho: None,
                held_chirho: false,
            })
            .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))?;
        let promoted_chirho: &[(&str, &str)] = match constructor_chirho {
            CompilerTypeConstructorChirho::BoolChirho => &[
                (BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, "False"),
                (BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, "True"),
            ],
            CompilerTypeConstructorChirho::OrderingChirho => &[
                ("GHC.Types", "LT"),
                ("GHC.Types", "EQ"),
                ("GHC.Types", "GT"),
            ],
            CompilerTypeConstructorChirho::MaybeChirho => {
                &[("GHC.Maybe", "Nothing"), ("GHC.Maybe", "Just")]
            }
            CompilerTypeConstructorChirho::EitherChirho => {
                &[("Data.Either", "Left"), ("Data.Either", "Right")]
            }
            CompilerTypeConstructorChirho::UnitChirho => &[(BUILTIN_TUPLE_MODULE_CHIRHO, "()")],
            CompilerTypeConstructorChirho::ProxyChirho => &[("Data.Proxy", "Proxy")],
            CompilerTypeConstructorChirho::ListChirho => &[
                (BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, "[]"),
                (BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, ":"),
            ],
            _ => &[],
        };
        for (module_chirho, name_chirho) in promoted_chirho {
            self.seed_promoted_header_chirho(
                constructor_chirho,
                &key_chirho,
                module_chirho,
                name_chirho,
                span_chirho,
            )?;
        }
        if let CompilerTypeConstructorChirho::TupleChirho(_) = constructor_chirho {
            self.seed_promoted_header_chirho(
                constructor_chirho,
                &key_chirho,
                BUILTIN_TUPLE_MODULE_CHIRHO,
                constructor_chirho.name_chirho(),
                span_chirho,
            )?;
        }
        Ok(())
    }

    fn seed_promoted_header_chirho(
        &mut self,
        parent_chirho: CompilerTypeConstructorChirho,
        parent_key_chirho: &str,
        module_chirho: &str,
        name_chirho: &str,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(
            module_chirho.len() + name_chirho.len() + 128,
            module_chirho.len() + name_chirho.len() + 128,
            span_chirho,
        )?;
        let promoted_key_chirho = PromotedKeyChirho::builtin_chirho(module_chirho, name_chirho);
        let identity_chirho: Arc<str> = Arc::from(promoted_key_chirho.as_str_chirho());
        if self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(&identity_chirho)
            .is_some()
        {
            return Ok(());
        }
        let parameter_count_chirho = match parent_chirho {
            CompilerTypeConstructorChirho::MaybeChirho
            | CompilerTypeConstructorChirho::ListChirho => 1,
            CompilerTypeConstructorChirho::EitherChirho
            | CompilerTypeConstructorChirho::ProxyChirho => 2,
            CompilerTypeConstructorChirho::TupleChirho(arity_chirho) => arity_chirho as usize,
            _ => 0,
        };
        self.state_chirho.charge_chirho(
            parameter_count_chirho * 4 + 8,
            parameter_count_chirho * 4 + 8,
            span_chirho,
        )?;
        let mut binders_chirho = Vec::new();
        for index_chirho in 0..parameter_count_chirho {
            binders_chirho.push(DeclarationBinderChirho {
                term_specificity_chirho: Some(
                    if parent_chirho == CompilerTypeConstructorChirho::ProxyChirho
                        && index_chirho == 0
                    {
                        haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho
                    } else {
                        haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho
                    },
                ),
                variable_chirho: TyVarChirho(index_chirho as u32),
                kind_chirho: if parent_chirho == CompilerTypeConstructorChirho::ProxyChirho
                    && index_chirho == 1
                {
                    TyChirho::VarChirho(TyVarChirho(0))
                } else {
                    ElaborationStateChirho::type_universe_chirho()
                },
                visibility_chirho: if parent_chirho == CompilerTypeConstructorChirho::ProxyChirho
                    && index_chirho == 0
                {
                    TelescopeVisibilityChirho::InferredChirho
                } else {
                    TelescopeVisibilityChirho::SpecifiedChirho
                },
            });
        }
        let parameter_chirho = |index_chirho| TyChirho::VarChirho(TyVarChirho(index_chirho));
        let mut result_chirho = TyChirho::ConChirho(parent_key_chirho.to_owned());
        if parent_chirho == CompilerTypeConstructorChirho::ListChirho {
            result_chirho = TyChirho::ListChirho(Box::new(parameter_chirho(0)));
        } else if matches!(parent_chirho, CompilerTypeConstructorChirho::TupleChirho(_)) {
            result_chirho = TyChirho::TupleChirho(
                (0..parameter_count_chirho)
                    .map(|index_chirho| parameter_chirho(index_chirho as u32))
                    .collect(),
            );
        } else {
            for index_chirho in 0..parameter_count_chirho {
                result_chirho = TyChirho::AppChirho(
                    Box::new(result_chirho),
                    Box::new(parameter_chirho(index_chirho as u32)),
                );
            }
        }
        let fields_chirho: Vec<TyChirho> = match name_chirho {
            "Just" | "Left" => vec![parameter_chirho(0)],
            "Right" => vec![parameter_chirho(1)],
            ":" => vec![
                parameter_chirho(0),
                TyChirho::ListChirho(Box::new(parameter_chirho(0))),
            ],
            _ if matches!(parent_chirho, CompilerTypeConstructorChirho::TupleChirho(_)) => (0
                ..parameter_count_chirho)
                .map(|index_chirho| parameter_chirho(index_chirho as u32))
                .collect(),
            _ => Vec::new(),
        };
        let visible_arity_chirho = fields_chirho.len();
        for (index_chirho, kind_chirho) in fields_chirho.into_iter().enumerate() {
            binders_chirho.push(DeclarationBinderChirho {
                term_specificity_chirho: None,
                variable_chirho: TyVarChirho((parameter_count_chirho + index_chirho) as u32),
                kind_chirho,
                visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
            });
        }
        self.registry_chirho
            .declaration_schemes_mut_chirho()
            .insert_chirho(DeclarationSchemeChirho {
                injectivity_chirho: None,
                association_chirho:
                    crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho,
                source_variables_chirho: Vec::new(),
                identity_chirho,
                owner_chirho: None,
                role_chirho: TypeConstructorRoleChirho::DataChirho,
                declaration_chirho: None,
                reduction_arity_chirho: binders_chirho.len(),
                binders_chirho,
                result_kind_chirho: result_chirho,
                synonym_rhs_chirho: None,
                synonym_syntax_chirho: None,
                local_binders_chirho: std::sync::Arc::from([]),
                visible_arity_chirho,
                promoted_chirho: Some(promoted_key_chirho),
                held_chirho: false,
            })
            .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))
    }
}
