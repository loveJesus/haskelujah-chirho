// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One head/recursion/constructor/publication lifecycle for data and newtype.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::{DeclChirho, KindChirho, KindInferCtxChirho, ModuleChirho, TyVarChirho};
use haskelujah_ast_chirho::decl_chirho::ConDeclChirho;

pub(super) struct PreparedPromotedConstructorChirho {
    name_chirho: String,
    body_chirho: KindChirho,
    parameters_chirho: Vec<super::KindVarChirho>,
}

impl KindInferCtxChirho {
    /// A local constructor owns its promoted name even when its full promoted
    /// contract is not represented. Never inherit a same-spelled builtin row.
    /// Nullary, parameter-free ordinary constructors have a complete classifier
    /// available directly from their owner; other contracts are published only
    /// after their declaration group has checked its fields and closed its kinds.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    pub(super) fn register_local_promoted_constructor_heads_chirho(
        &mut self,
        module_chirho: &ModuleChirho,
    ) {
        for declaration_chirho in &module_chirho.decls_chirho {
            let (owner_chirho, parameters_chirho, constructors_chirho) = match declaration_chirho {
                DeclChirho::DataDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    constructors_chirho,
                    ..
                } => (
                    name_chirho,
                    type_vars_chirho,
                    constructors_chirho.as_slice(),
                ),
                DeclChirho::NewtypeDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    constructor_chirho,
                    ..
                } => (
                    name_chirho,
                    type_vars_chirho,
                    std::slice::from_ref(constructor_chirho),
                ),
                _ => continue,
            };
            for constructor_chirho in constructors_chirho {
                let (name_chirho, nullary_chirho) = match constructor_chirho {
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho,
                        fields_chirho,
                        ..
                    } => (name_chirho, fields_chirho.is_empty()),
                    ConDeclChirho::RecordChirho {
                        name_chirho,
                        fields_chirho,
                        ..
                    } => (name_chirho, fields_chirho.is_empty()),
                    ConDeclChirho::GadtChirho { name_chirho, .. } => (name_chirho, false),
                };
                let qualified_chirho = format!(
                    "{}.{}",
                    module_chirho.name_chirho.full_name_chirho(),
                    name_chirho.text_chirho(),
                );
                for spelling_chirho in [name_chirho.text_chirho(), qualified_chirho.as_str()] {
                    self.local_promoted_constructor_names_chirho
                        .insert(spelling_chirho.to_owned());
                    self.env_chirho.hide_promoted_chirho(spelling_chirho);
                    if nullary_chirho && parameters_chirho.is_empty() {
                        self.env_chirho.bind_promoted_generalized_chirho(
                            spelling_chirho,
                            KindChirho::ConChirho(owner_chirho.full_name_chirho()),
                        );
                    }
                }
            }
        }
    }

    pub(super) fn check_data_constructors_chirho(
        &mut self,
        type_vars_chirho: &[TyVarChirho],
        constructors_chirho: &[ConDeclChirho],
    ) {
        for constructor_chirho in constructors_chirho {
            match constructor_chirho {
                ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
                    for (_, field_chirho) in fields_chirho {
                        let kind_chirho = self.infer_type_kind_chirho(field_chirho);
                        self.check_runtime_kind_chirho(
                            &kind_chirho,
                            "data constructor field",
                            field_chirho.span_chirho(),
                        );
                    }
                }
                ConDeclChirho::RecordChirho { fields_chirho, .. } => {
                    for field_chirho in fields_chirho {
                        let kind_chirho = self.infer_type_kind_chirho(&field_chirho.ty_chirho);
                        self.check_runtime_kind_chirho(
                            &kind_chirho,
                            "data constructor field",
                            field_chirho.ty_chirho.span_chirho(),
                        );
                    }
                }
                ConDeclChirho::GadtChirho {
                    name_chirho,
                    ty_chirho,
                    ..
                } => {
                    // GADT constructor type variables are independently quantified;
                    // declaration-head names do not scope over their signatures.
                    self.env_chirho.begin_scope_chirho();
                    for variable_chirho in type_vars_chirho {
                        self.env_chirho.hide_chirho(variable_chirho.text_chirho());
                    }
                    let outer_names_chirho = std::mem::take(&mut self.kind_var_cache_chirho);
                    let errors_before_chirho = self.diagnostics_chirho.error_count_chirho();
                    let kind_chirho = self.infer_type_kind_chirho(ty_chirho);
                    self.check_runtime_kind_chirho(
                        &kind_chirho,
                        "GADT constructor type",
                        ty_chirho.span_chirho(),
                    );
                    // The promoted classifier is the checked signature's TERM,
                    // including its result index, not merely its result owner.
                    // Unsupported contexts/higher-rank fields remain opaque;
                    // no nearby ordinary constructor contract is fabricated.
                    if self.diagnostics_chirho.error_count_chirho() == errors_before_chirho
                        && let Some(promoted_chirho) =
                            self.promoted_gadt_signature_chirho(ty_chirho)
                    {
                        let promoted_chirho = self.subst_chirho.apply_chirho(&promoted_chirho);
                        let variables_chirho = promoted_chirho.free_vars_chirho();
                        let scheme_chirho = self.bind_source_kind_scheme_chirho(
                            promoted_chirho,
                            variables_chirho.clone(),
                            &variables_chirho.into_iter().collect(),
                        );
                        self.env_chirho.bind_promoted_scheme_chirho(
                            name_chirho.text_chirho(),
                            scheme_chirho.clone(),
                        );
                        if let Some(module_chirho) = &self.local_kind_module_chirho {
                            self.env_chirho.bind_promoted_scheme_chirho(
                                &format!("{module_chirho}.{}", name_chirho.text_chirho()),
                                scheme_chirho,
                            );
                        }
                    }
                    self.kind_var_cache_chirho = outer_names_chirho;
                    self.env_chirho.end_scope_chirho();
                }
            }
        }
    }

    /// Capture terms while the declaration's parameters are in scope, but do
    /// not generalize their classifiers until every body in the SCC is checked.
    /// This relates a promoted constructor's fields to its actual result indices.
    /// Unsupported higher-rank fields remain opaque, not guessed arrow kinds.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    pub(super) fn prepare_ordinary_promoted_contracts_chirho(
        &mut self,
        declaration_chirho: &DeclChirho,
    ) -> Vec<PreparedPromotedConstructorChirho> {
        let (owner_chirho, parameters_chirho, constructors_chirho) = match declaration_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                ..
            } => (
                name_chirho,
                type_vars_chirho,
                constructors_chirho.as_slice(),
            ),
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructor_chirho,
                ..
            } => (
                name_chirho,
                type_vars_chirho,
                std::slice::from_ref(constructor_chirho),
            ),
            _ => return Vec::new(),
        };
        let mut result_chirho = self.named_kind_term_chirho(owner_chirho);
        let mut identities_chirho = Vec::new();
        for parameter_chirho in parameters_chirho {
            let term_chirho = self.interpret_kind_term_chirho(&super::TypeChirho::VarChirho(
                parameter_chirho.name_chirho.clone(),
            ));
            identities_chirho.extend(term_chirho.free_vars_chirho());
            if parameter_chirho.visibility_chirho
                == haskelujah_ast_chirho::decl_chirho::TyVarVisibilityChirho::VisibleChirho
            {
                result_chirho = KindChirho::app_chirho(result_chirho, term_chirho);
            }
        }
        let mut contracts_chirho = Vec::new();
        for constructor_chirho in constructors_chirho {
            let (name_chirho, fields_chirho) = match constructor_chirho {
                ConDeclChirho::OrdinaryChirho {
                    name_chirho,
                    fields_chirho,
                    ..
                } => (
                    name_chirho,
                    fields_chirho
                        .iter()
                        .map(|(_, field_chirho)| self.family_term_chirho(field_chirho))
                        .collect::<Option<Vec<_>>>(),
                ),
                ConDeclChirho::RecordChirho {
                    name_chirho,
                    fields_chirho,
                    ..
                } => {
                    let mut terms_chirho = Vec::new();
                    let mut represented_chirho = true;
                    for field_chirho in fields_chirho {
                        if let Some(term_chirho) = self.family_term_chirho(&field_chirho.ty_chirho)
                        {
                            terms_chirho.extend(std::iter::repeat_n(
                                term_chirho,
                                field_chirho.names_chirho.len(),
                            ));
                        } else {
                            represented_chirho = false;
                            break;
                        }
                    }
                    (name_chirho, represented_chirho.then_some(terms_chirho))
                }
                ConDeclChirho::GadtChirho { .. } => continue,
            };
            if let Some(fields_chirho) = fields_chirho {
                contracts_chirho.push(PreparedPromotedConstructorChirho {
                    name_chirho: name_chirho.text_chirho().to_owned(),
                    body_chirho: KindChirho::arrow_n_chirho(fields_chirho, result_chirho.clone()),
                    parameters_chirho: identities_chirho.clone(),
                });
            }
        }
        contracts_chirho
    }

    pub(super) fn publish_ordinary_promoted_contracts_chirho(
        &mut self,
        contracts_chirho: Vec<PreparedPromotedConstructorChirho>,
    ) {
        for contract_chirho in contracts_chirho {
            let written_chirho = contract_chirho.parameters_chirho.iter().copied().collect();
            let scheme_chirho = self.source_kind_scheme_chirho(
                contract_chirho.body_chirho,
                contract_chirho.parameters_chirho,
                &written_chirho,
            );
            self.env_chirho
                .bind_promoted_scheme_chirho(&contract_chirho.name_chirho, scheme_chirho.clone());
            if let Some(module_chirho) = &self.local_kind_module_chirho {
                self.env_chirho.bind_promoted_scheme_chirho(
                    &format!("{module_chirho}.{}", contract_chirho.name_chirho),
                    scheme_chirho,
                );
            }
        }
    }

    fn promoted_gadt_signature_chirho(
        &mut self,
        ty_chirho: &super::TypeChirho,
    ) -> Option<KindChirho> {
        match ty_chirho {
            super::TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => self.with_kind_binders_chirho(vars_chirho, |ctx_chirho| {
                // This is a fresh quantifier scope, distinct from validity
                // checking above. Solve each binder's classifier from THIS
                // body before publishing its promoted scheme; otherwise
                // `forall a. Proxy a -> T` invents an unconstrained kind for a.
                let _classifier_chirho = ctx_chirho.infer_type_kind_chirho(body_chirho);
                ctx_chirho.promoted_gadt_signature_chirho(body_chirho)
            }),
            super::TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.promoted_gadt_signature_chirho(inner_chirho)
            }
            super::TypeChirho::KindAnnotChirho { type_chirho, .. } => {
                self.promoted_gadt_signature_chirho(type_chirho)
            }
            _ => self.family_term_chirho(ty_chirho),
        }
    }
}

pub(super) fn cusks_enabled_chirho(extensions_chirho: &[String]) -> bool {
    extensions_chirho.iter().fold(
        false,
        |enabled_chirho, extension_chirho| match extension_chirho.as_str() {
            "NoCUSKs" | "StandaloneKindSignatures" | "GHC2021" | "GHC2024" => false,
            "CUSKs" | "Haskell98" | "Haskell2010" => true,
            _ => enabled_chirho,
        },
    )
}

/// Match the default GHC2021 edition used by the other front-end passes.
/// Explicit legacy editions and NoPolyKinds still take effect in source order.
pub(super) fn poly_kinds_enabled_chirho(extensions_chirho: &[String]) -> bool {
    extensions_chirho.iter().fold(
        true,
        |enabled_chirho, extension_chirho| match extension_chirho.as_str() {
            "Haskell98" | "Haskell2010" | "NoPolyKinds" => false,
            "GHC2021" | "GHC2024" | "PolyKinds" | "TypeInType" => true,
            _ => enabled_chirho,
        },
    )
}
