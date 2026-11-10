// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor schemes and named-field inventories share the canonical GADT
//! signature. Pattern contexts are givens; expressions still demand them.
//! Workflow: compiler-pipeline-chirho/gadt-constructor-carrier-chirho.md.

use super::{InferCtxChirho, records_chirho};
use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};
use haskelujah_ast_chirho::decl_chirho::GadtConSignatureChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use std::collections::HashMap;

/// The real parent declaration supplies the owner; a result spelling never does.
#[derive(Clone, Copy)]
pub(crate) struct LocalConstructorFieldContextChirho<'owner_chirho> {
    pub(crate) module_name_chirho: &'owner_chirho str,
    pub(crate) owner_declaration_chirho:
        Option<&'owner_chirho haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho>,
}

impl InferCtxChirho {
    /// Both keys name the same actual local declaration. Qualified pattern
    /// consumers never borrow a field vector from an unrelated bare namesake.
    /// Workflow: compiler-pipeline-chirho/value-scope-resolution-chirho.
    pub(crate) fn bind_constructor_field_names_chirho(
        &mut self,
        field_context_chirho: LocalConstructorFieldContextChirho<'_>,
        constructor_name_chirho: &str,
        fields_chirho: Vec<String>,
    ) {
        let module_name_chirho = field_context_chirho.module_name_chirho;
        let scope_chirho = match field_context_chirho.owner_declaration_chirho {
            Some(owner_chirho) => super::ConstructorFieldScopeChirho::selected_chirho(
                owner_chirho.clone(),
                fields_chirho.clone(),
                fields_chirho,
            ),
            None => super::ConstructorFieldScopeChirho::legacy_unproven_chirho(fields_chirho),
        };
        self.con_field_names_chirho.insert(
            format!("{module_name_chirho}.{constructor_name_chirho}"),
            scope_chirho.clone(),
        );
        self.con_field_names_chirho
            .insert(constructor_name_chirho.to_string(), scope_chirho);
    }

    /// The declaration gateway already checked the actual parent header. Retain an H98
    /// prefix through the same scheme, source-correspondence and selector consumers as GADTs.
    pub(super) fn bind_written_h98_constructor_chirho(
        &mut self,
        constructor_chirho: &haskelujah_ast_chirho::decl_chirho::ConDeclChirho,
        parent_header_chirho: (
            &NameChirho,
            &[haskelujah_ast_chirho::decl_chirho::TyVarChirho],
        ),
        seed_chirho: &HashMap<String, TyVarChirho>,
        type_constructor_scope_chirho: Option<
            &haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho,
        >,
        field_context_chirho: LocalConstructorFieldContextChirho<'_>,
        projection_budget_chirho: &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho,
    ) -> bool {
        use haskelujah_ast_chirho::decl_chirho::{
            ConDeclChirho, GadtConArgsChirho, GadtConFieldChirho, GadtConPrefixChirho,
            check_constructor_projection_chirho, check_name_projection_chirho,
            check_type_projection_chirho,
        };
        use haskelujah_ast_chirho::ty_chirho::TypeChirho;
        let (name_chirho, prefix_chirho, span_chirho) = match constructor_chirho {
            ConDeclChirho::OrdinaryChirho {
                name_chirho,
                prefix_chirho,
                span_chirho,
                ..
            }
            | ConDeclChirho::RecordChirho {
                name_chirho,
                prefix_chirho,
                span_chirho,
                ..
            } => (name_chirho, prefix_chirho, *span_chirho),
            ConDeclChirho::GadtChirho { .. } => return false,
        };
        if prefix_chirho.is_empty_chirho() {
            return false;
        }
        if prefix_chirho.unread_chirho {
            self.non_deferrable_diagnostics_chirho.push_chirho(
                haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                    haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
                    "constructor quantification was not faithfully lowered",
                    span_chirho,
                ),
            );
            return true;
        }
        let checked_chirho = (|| {
            check_constructor_projection_chirho(constructor_chirho, projection_budget_chirho, 1)?;
            check_name_projection_chirho(parent_header_chirho.0, projection_budget_chirho)?;
            for binder_chirho in parent_header_chirho.1 {
                check_name_projection_chirho(&binder_chirho.name_chirho, projection_budget_chirho)?;
                if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                    check_type_projection_chirho(kind_chirho, projection_budget_chirho)?;
                }
                projection_budget_chirho.reserve_transport_chirho(1, 1, span_chirho)?;
            }
            Ok::<_, haskelujah_ast_chirho::decl_chirho::GadtProjectionErrorChirho>(())
        })();
        if let Err(error_chirho) = checked_chirho {
            self.non_deferrable_diagnostics_chirho.push_chirho(
                haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                    haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
                    error_chirho.to_string(),
                    error_chirho.span_chirho,
                ),
            );
            return true;
        }
        let mut result_ty_chirho = TypeChirho::ConChirho(parent_header_chirho.0.clone());
        let parent_scheme_chirho = type_constructor_scope_chirho
            .and_then(|scope_chirho| {
                scope_chirho.declaration_chirho(&parent_header_chirho.0.full_name_chirho())
            })
            .and_then(|declaration_chirho| {
                self.type_families_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(&declaration_chirho.identity_chirho.key_chirho())
            });
        if let Some(parent_scheme_chirho) = parent_scheme_chirho {
            if let Err(error_chirho) = projection_budget_chirho.reserve_transport_chirho(
                parent_scheme_chirho.binders_chirho.len(),
                parent_scheme_chirho.binders_chirho.len().saturating_mul(3),
                span_chirho,
            ) {
                self.non_deferrable_diagnostics_chirho.push_chirho(
                    haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                        haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
                        error_chirho.to_string(),
                        span_chirho,
                    ),
                );
                return true;
            }
            // Use every actual checked slot, including inferred kind arguments.
            // data_head_type installed these same canonical slots in the seed.
            for binder_chirho in &parent_scheme_chirho.binders_chirho {
                let mut argument_name_chirho = NameChirho::RawChirho(
                    haskelujah_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
                        format!("parent_slot_{}_chirho", binder_chirho.variable_chirho.0),
                        span_chirho,
                    ),
                );
                argument_name_chirho.set_type_variable_chirho(Some(
                    haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho::QuantifiedChirho {
                        declaration_chirho: parent_scheme_chirho.identity_chirho.clone(),
                        index_chirho: binder_chirho.variable_chirho.0,
                    },
                ));
                let argument_chirho = Box::new(TypeChirho::VarChirho(argument_name_chirho));
                result_ty_chirho = if binder_chirho.visibility_chirho
                    == crate::ty_chirho::declarations_chirho::TelescopeVisibilityChirho::RequiredChirho
                {
                    TypeChirho::AppChirho {
                        fun_chirho: Box::new(result_ty_chirho), arg_chirho: argument_chirho, span_chirho,
                    }
                } else {
                    TypeChirho::KindApplicationChirho {
                        fun_chirho: Box::new(result_ty_chirho), arg_chirho: argument_chirho,
                        written_chirho: false, span_chirho,
                    }
                };
            }
        } else {
            // Legacy standalone inference has no checked catalogue. Preserve its
            // written header without upgrading absent metadata to a telescope.
            for binder_chirho in parent_header_chirho
                .1
                .iter()
                .filter(|binder_chirho| binder_chirho.is_visible_chirho())
            {
                result_ty_chirho = TypeChirho::AppChirho {
                    fun_chirho: Box::new(result_ty_chirho),
                    arg_chirho: Box::new(TypeChirho::VarChirho(binder_chirho.name_chirho.clone())),
                    span_chirho,
                };
            }
        }
        let mut prefixes_chirho = Vec::new();
        // The datatype producer owns this order: its universals precede the
        // constructor's existentials, even when a field mentions only the latter.
        // Both groups retain their stamped source owners through conversion.
        if !parent_header_chirho.1.is_empty() {
            prefixes_chirho.push(GadtConPrefixChirho::ForallChirho {
                vars_chirho: parent_header_chirho.1.to_vec(),
                required_chirho: false,
                span_chirho,
            });
        }
        if prefix_chirho.forall_written_chirho || !prefix_chirho.existential_vars_chirho.is_empty()
        {
            prefixes_chirho.push(GadtConPrefixChirho::ForallChirho {
                vars_chirho: prefix_chirho.existential_vars_chirho.clone(),
                required_chirho: false,
                span_chirho,
            });
        }
        if !prefix_chirho.context_chirho.is_empty() {
            prefixes_chirho.push(GadtConPrefixChirho::ContextChirho {
                constraints_chirho: prefix_chirho.context_chirho.clone(),
                span_chirho,
            });
        }
        let args_chirho = match constructor_chirho {
            ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
                GadtConArgsChirho::PositionalChirho(
                    fields_chirho
                        .iter()
                        .map(|(strictness_chirho, ty_chirho)| GadtConFieldChirho {
                            strictness_chirho: *strictness_chirho,
                            multiplicity_chirho: None,
                            ty_chirho: ty_chirho.clone(),
                            span_chirho: ty_chirho.span_chirho(),
                        })
                        .collect(),
                )
            }
            ConDeclChirho::RecordChirho { fields_chirho, .. } => {
                GadtConArgsChirho::RecordChirho(fields_chirho.clone())
            }
            ConDeclChirho::GadtChirho { .. } => return false,
        };
        let signature_chirho = GadtConSignatureChirho {
            prefix_chirho: prefixes_chirho,
            args_chirho,
            result_ty_chirho,
            malformed_chirho: None,
            span_chirho,
        };
        let source_binders_chirho = prefix_chirho
            .existential_vars_chirho
            .iter()
            .collect::<Vec<_>>();
        self.bind_constructor_signature_chirho(
            name_chirho,
            (&signature_chirho, &source_binders_chirho),
            seed_chirho,
            type_constructor_scope_chirho,
            field_context_chirho,
            projection_budget_chirho,
        );
        true
    }

    pub(super) fn bind_gadt_constructor_chirho(
        &mut self,
        name_chirho: &NameChirho,
        signature_chirho: &GadtConSignatureChirho,
        seed_chirho: &HashMap<String, TyVarChirho>,
        type_constructor_scope_chirho: Option<
            &haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho,
        >,
        field_context_chirho: LocalConstructorFieldContextChirho<'_>,
        projection_budget_chirho: &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho,
    ) {
        let source_binders_chirho = signature_chirho
            .prefix_chirho
            .iter()
            .filter_map(|prefix_chirho| match prefix_chirho {
                haskelujah_ast_chirho::decl_chirho::GadtConPrefixChirho::ForallChirho {
                    vars_chirho,
                    ..
                } => Some(vars_chirho.as_slice()),
                _ => None,
            })
            .flatten()
            .collect::<Vec<_>>();
        self.bind_constructor_signature_chirho(
            name_chirho,
            (signature_chirho, &source_binders_chirho),
            seed_chirho,
            type_constructor_scope_chirho,
            field_context_chirho,
            projection_budget_chirho,
        );
    }

    /// The correspondence slice comes from the actual written constructor
    /// producer; generated datatype universals never change its source indices.
    fn bind_constructor_signature_chirho(
        &mut self,
        name_chirho: &NameChirho,
        source_chirho: (
            &GadtConSignatureChirho,
            &[&haskelujah_ast_chirho::decl_chirho::TyVarChirho],
        ),
        seed_chirho: &HashMap<String, TyVarChirho>,
        type_constructor_scope_chirho: Option<
            &haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho,
        >,
        field_context_chirho: LocalConstructorFieldContextChirho<'_>,
        projection_budget_chirho: &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho,
    ) {
        let module_name_chirho = field_context_chirho.module_name_chirho;
        let (signature_chirho, written_chirho) = source_chirho;
        let ty_chirho = match signature_chirho.try_as_type_chirho(projection_budget_chirho) {
            Ok(ty_chirho) => ty_chirho,
            Err(error_chirho) => {
                self.non_deferrable_diagnostics_chirho.push_chirho(
                    haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                        haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
                        error_chirho.to_string(),
                        error_chirho.span_chirho(),
                    ),
                );
                return;
            }
        };
        self.note_written_constructor_context_chirho(name_chirho, &ty_chirho);
        let (mut scheme_chirho, _) =
            self.ast_type_to_scheme_seeded_chirho(&ty_chirho, seed_chirho, false);
        let constructor_key_chirho = self.promotion_scope_chirho.as_ref().map(|scope_chirho| {
            scope_chirho
                .quoted_chirho(name_chirho)
                .key_chirho()
                .as_str_chirho()
                .to_owned()
        });
        if let Some(header_chirho) = constructor_key_chirho.as_ref().and_then(|key_chirho| {
            self.type_families_chirho
                .declaration_schemes_chirho()
                .get_chirho(key_chirho)
        }) {
            let mut correspondence_chirho = HashMap::new();
            for source_chirho in &header_chirho.source_variables_chirho {
                let original_chirho = written_chirho
                    .get(source_chirho.header_index_chirho as usize)
                    .and_then(|binder_chirho| binder_chirho.name_chirho.type_variable_chirho());
                let Some(original_chirho) = original_chirho.filter(|original_chirho| {
                    scheme_chirho
                        .vars_chirho
                        .records_chirho()
                        .iter()
                        .any(|record_chirho| {
                            record_chirho.source_chirho.as_ref() == Some(original_chirho)
                        })
                }) else {
                    self.non_deferrable_diagnostics_chirho.push_chirho(haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                        haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
                        "checked constructor source correspondence is absent from its term scheme",
                        name_chirho.span_chirho(),
                    ));
                    return;
                };
                correspondence_chirho
                    .insert(original_chirho.clone(), source_chirho.source_chirho.clone());
            }
            scheme_chirho.vars_chirho =
                scheme_chirho
                    .vars_chirho
                    .map_records_chirho(|record_chirho| {
                        let mut record_chirho = record_chirho.clone();
                        if let Some(source_chirho) = record_chirho
                            .source_chirho
                            .as_ref()
                            .and_then(|source_chirho| correspondence_chirho.get(source_chirho))
                        {
                            record_chirho.source_chirho = Some(source_chirho.clone());
                        }
                        record_chirho
                    });
        }
        if signature_chirho
            .positional_fields_chirho()
            .any(|field_chirho| field_chirho.strictness_chirho.is_strict_chirho())
        {
            self.strict_positional_constructors_chirho
                .insert(name_chirho.text_chirho().to_string());
        }
        self.bind_constructor_field_names_chirho(
            field_context_chirho,
            name_chirho.text_chirho(),
            signature_chirho
                .record_fields_chirho()
                .unwrap_or_default()
                .iter()
                .flat_map(|field_chirho| &field_chirho.names_chirho)
                .map(|name_chirho| name_chirho.text_chirho().to_string())
                .collect(),
        );
        if let Some(fields_chirho) = signature_chirho.record_fields_chirho() {
            self.con_strict_fields_chirho.insert(
                name_chirho.text_chirho().to_string(),
                records_chirho::strict_field_names_chirho(fields_chirho),
            );
            let mut result_ty_chirho = &scheme_chirho.ty_chirho;
            let mut argument_tys_chirho = Vec::new();
            while let TyChirho::FunChirho(arg_chirho, result_chirho, _) = result_ty_chirho {
                argument_tys_chirho.push(arg_chirho.as_ref());
                result_ty_chirho = result_chirho;
            }
            let availability_chirho = match signature_chirho.selector_availability_chirho(
                projection_budget_chirho,
                &|head_chirho| {
                    type_constructor_scope_chirho
                        .and_then(|scope_chirho| {
                            scope_chirho.declaration_chirho(&head_chirho.full_name_chirho())
                        })
                        .is_some_and(|declaration_chirho| {
                            declaration_chirho.is_nominally_generative_chirho()
                        })
                },
            ) {
                Ok(availability_chirho) => availability_chirho,
                Err(error_chirho) => {
                    self.non_deferrable_diagnostics_chirho.push_chirho(
                        haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                            haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
                            error_chirho.to_string(),
                            error_chirho.span_chirho(),
                        ),
                    );
                    return;
                }
            };
            let mut field_index_chirho = 0_usize;
            for (field_chirho, available_chirho) in fields_chirho.iter().zip(availability_chirho) {
                for field_name_chirho in &field_chirho.names_chirho {
                    if available_chirho
                        && let Some(field_ty_chirho) = argument_tys_chirho.get(field_index_chirho)
                    {
                        let selector_ty_chirho = TyChirho::FunChirho(
                            Box::new(result_ty_chirho.clone()),
                            Box::new((*field_ty_chirho).clone()),
                            MultChirho::ManyChirho,
                        );
                        let selector_scheme_chirho = self.generalize_chirho(&selector_ty_chirho);
                        self.env_chirho.bind_chirho(
                            format!("{module_name_chirho}.{}", field_name_chirho.text_chirho()),
                            selector_scheme_chirho.clone(),
                        );
                        self.env_chirho.bind_chirho(
                            field_name_chirho.text_chirho().to_string(),
                            selector_scheme_chirho,
                        );
                    } else if !available_chirho {
                        self.env_chirho
                            .mask_chirho(field_name_chirho.text_chirho().to_string());
                        self.env_chirho.mask_chirho(format!(
                            "{module_name_chirho}.{}",
                            field_name_chirho.text_chirho()
                        ));
                    }
                    field_index_chirho += 1;
                }
            }
        }
        self.env_chirho.bind_pattern_constructor_chirho(
            name_chirho.text_chirho().to_string(),
            scheme_chirho,
            true,
        );
    }
}
