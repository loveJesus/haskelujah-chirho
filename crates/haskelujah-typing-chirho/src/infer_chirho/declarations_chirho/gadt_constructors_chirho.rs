// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor schemes and named-field inventories share the canonical GADT
//! signature. Pattern contexts are givens; expressions still demand them.
//! Workflow: compiler-pipeline-chirho/gadt-constructor-carrier-chirho.md.

use super::{InferCtxChirho, records_chirho};
use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};
use haskelujah_ast_chirho::decl_chirho::GadtConSignatureChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use std::collections::HashMap;

impl InferCtxChirho {
    pub(super) fn bind_gadt_constructor_chirho(
        &mut self,
        name_chirho: &NameChirho,
        signature_chirho: &GadtConSignatureChirho,
        seed_chirho: &HashMap<String, TyVarChirho>,
        type_constructor_scope_chirho: Option<
            &haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho,
        >,
        module_name_chirho: &str,
        projection_budget_chirho: &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho,
    ) {
        let ty_chirho = match signature_chirho.try_as_type_chirho(projection_budget_chirho) {
            Ok(ty_chirho) => ty_chirho,
            Err(error_chirho) => {
                self.diagnostics_chirho.push_chirho(
                    haskelujah_diagnostics_chirho::DiagnosticChirho::error_chirho(
                        error_chirho.to_string(),
                        error_chirho.span_chirho(),
                    ),
                );
                return;
            }
        };
        self.note_written_constructor_context_chirho(name_chirho, &ty_chirho);
        let (scheme_chirho, _) =
            self.ast_type_to_scheme_seeded_chirho(&ty_chirho, seed_chirho, false);
        if signature_chirho
            .positional_fields_chirho()
            .any(|field_chirho| field_chirho.strictness_chirho.is_strict_chirho())
        {
            self.strict_positional_constructors_chirho
                .insert(name_chirho.text_chirho().to_string());
        }
        if let Some(fields_chirho) = signature_chirho.record_fields_chirho() {
            self.con_field_names_chirho.insert(
                name_chirho.text_chirho().to_string(),
                fields_chirho
                    .iter()
                    .flat_map(|field_chirho| {
                        field_chirho
                            .names_chirho
                            .iter()
                            .map(|name_chirho| name_chirho.text_chirho().to_string())
                    })
                    .collect(),
            );
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
                    self.diagnostics_chirho.push_chirho(
                        haskelujah_diagnostics_chirho::DiagnosticChirho::error_chirho(
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
        self.env_chirho
            .bind_chirho(name_chirho.text_chirho().to_string(), scheme_chirho);
    }
}
