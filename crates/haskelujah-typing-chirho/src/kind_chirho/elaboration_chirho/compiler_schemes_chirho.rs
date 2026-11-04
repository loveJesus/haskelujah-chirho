// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typed compiler declaration producers. Syntax-only list/tuple/arrow names
//! cannot be declared by a module; ordinary names require the naming proof.

use super::*;
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
            variable_chirho: TyVarChirho(0),
            kind_chirho: ElaborationStateChirho::type_universe_chirho(),
            visibility_chirho: TelescopeVisibilityChirho::InferredChirho,
        }];
        if heterogeneous_chirho {
            binders_chirho.push(DeclarationBinderChirho {
                variable_chirho: TyVarChirho(1),
                kind_chirho: ElaborationStateChirho::type_universe_chirho(),
                visibility_chirho: TelescopeVisibilityChirho::InferredChirho,
            });
        }
        for index_chirho in 0..2 {
            binders_chirho.push(DeclarationBinderChirho {
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
                promoted_chirho: false,
                held_chirho: false,
            })
            .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))
    }

    /// All positive compiler class producers use this same Ty-based telescope.
    /// Public names and legacy arity views cannot manufacture an identity.
    pub(super) fn seed_class_header_chirho(
        &mut self,
        class_chirho: haskelujah_ast_chirho::name_chirho::CompilerClassChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        use haskelujah_ast_chirho::name_chirho::CompilerClassChirho;
        let identity_chirho: Arc<str> = Arc::from(format!(
            "%class:compiler:{}",
            class_chirho.legacy_name_chirho()
        ));
        if self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(&identity_chirho)
            .is_some()
        {
            return Ok(());
        }
        self.state_chirho.charge_chirho(128, 128, span_chirho)?;
        let type_chirho = ElaborationStateChirho::type_universe_chirho();
        let mut binders_chirho = Vec::new();
        let mut parameter_kinds_chirho = match class_chirho {
            CompilerClassChirho::KnownNatChirho => vec![TyChirho::ConChirho("Nat".to_owned())],
            CompilerClassChirho::KnownSymbolChirho => {
                vec![TyChirho::ConChirho("Symbol".to_owned())]
            }
            CompilerClassChirho::KnownCharChirho => vec![TyChirho::ConChirho("Char".to_owned())],
            CompilerClassChirho::TypeableChirho | CompilerClassChirho::CoercibleChirho => {
                binders_chirho.push(DeclarationBinderChirho {
                    variable_chirho: TyVarChirho(0),
                    kind_chirho: type_chirho.clone(),
                    visibility_chirho: TelescopeVisibilityChirho::InferredChirho,
                });
                let kind_chirho = TyChirho::VarChirho(TyVarChirho(0));
                if class_chirho == CompilerClassChirho::CoercibleChirho {
                    vec![kind_chirho.clone(), kind_chirho]
                } else {
                    vec![kind_chirho]
                }
            }
            CompilerClassChirho::FunctorChirho
            | CompilerClassChirho::ApplicativeChirho
            | CompilerClassChirho::MonadChirho
            | CompilerClassChirho::FoldableChirho
            | CompilerClassChirho::TraversableChirho
            | CompilerClassChirho::MonadIoChirho
            | CompilerClassChirho::MonadFailChirho
            | CompilerClassChirho::MonadFixChirho
            | CompilerClassChirho::MonadZipChirho
            | CompilerClassChirho::ContravariantChirho
            | CompilerClassChirho::AlternativeChirho
            | CompilerClassChirho::MonadPlusChirho => vec![ElaborationStateChirho::arrow_chirho(
                type_chirho.clone(),
                type_chirho.clone(),
            )],
            CompilerClassChirho::MonadTransChirho => vec![ElaborationStateChirho::arrow_chirho(
                ElaborationStateChirho::arrow_chirho(type_chirho.clone(), type_chirho.clone()),
                ElaborationStateChirho::arrow_chirho(type_chirho.clone(), type_chirho.clone()),
            )],
            CompilerClassChirho::HasFieldChirho => vec![
                TyChirho::ConChirho("Symbol".to_owned()),
                type_chirho.clone(),
                type_chirho.clone(),
            ],
            _ => vec![type_chirho],
        };
        let visible_arity_chirho = parameter_kinds_chirho.len();
        for kind_chirho in parameter_kinds_chirho.drain(..) {
            binders_chirho.push(DeclarationBinderChirho {
                variable_chirho: TyVarChirho(binders_chirho.len() as u32),
                kind_chirho,
                visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
            });
        }
        self.registry_chirho
            .declaration_schemes_mut_chirho()
            .insert_chirho(DeclarationSchemeChirho {
                identity_chirho,
                owner_chirho: None,
                role_chirho: TypeConstructorRoleChirho::ClassChirho,
                declaration_chirho: Some(TypeConstructorDeclarationChirho::compiler_data_chirho(
                    CompilerTypeConstructorChirho::ClassChirho(class_chirho),
                )),
                reduction_arity_chirho: binders_chirho.len(),
                visible_arity_chirho,
                binders_chirho,
                source_variables_chirho: Vec::new(),
                result_kind_chirho: ElaborationStateChirho::constraint_universe_chirho(),
                synonym_rhs_chirho: None,
                promoted_chirho: false,
                held_chirho: false,
            })
            .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))
    }

    pub(super) fn seed_compiler_header_chirho(
        &mut self,
        constructor_chirho: CompilerTypeConstructorChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
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
                variable_chirho: TyVarChirho(0),
                kind_chirho: type_chirho.clone(),
                visibility_chirho: TelescopeVisibilityChirho::InferredChirho,
            });
            binders_chirho.push(DeclarationBinderChirho {
                variable_chirho: TyVarChirho(1),
                kind_chirho: TyChirho::VarChirho(TyVarChirho(0)),
                visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
            });
        } else {
            for index_chirho in 0..arity_chirho {
                binders_chirho.push(DeclarationBinderChirho {
                    variable_chirho: TyVarChirho(index_chirho as u32),
                    kind_chirho: type_chirho.clone(),
                    visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
                });
            }
        }
        self.registry_chirho
            .declaration_schemes_mut_chirho()
            .insert_chirho(DeclarationSchemeChirho {
                source_variables_chirho: Vec::new(),
                identity_chirho: key_chirho.clone(),
                owner_chirho: None,
                role_chirho: proof_chirho.role_chirho,
                declaration_chirho: Some(proof_chirho),
                reduction_arity_chirho: binders_chirho.len(),
                visible_arity_chirho: arity_chirho,
                binders_chirho,
                result_kind_chirho: type_chirho,
                synonym_rhs_chirho: None,
                promoted_chirho: false,
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
        let identity_chirho: Arc<str> = Arc::from(
            PromotedKeyChirho::builtin_chirho(module_chirho, name_chirho).as_str_chirho(),
        );
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
                variable_chirho: TyVarChirho((parameter_count_chirho + index_chirho) as u32),
                kind_chirho,
                visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
            });
        }
        self.registry_chirho
            .declaration_schemes_mut_chirho()
            .insert_chirho(DeclarationSchemeChirho {
                source_variables_chirho: Vec::new(),
                identity_chirho,
                owner_chirho: None,
                role_chirho: TypeConstructorRoleChirho::DataChirho,
                declaration_chirho: None,
                reduction_arity_chirho: binders_chirho.len(),
                binders_chirho,
                result_kind_chirho: result_chirho,
                synonym_rhs_chirho: None,
                visible_arity_chirho,
                promoted_chirho: true,
                held_chirho: false,
            })
            .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))
    }
}
