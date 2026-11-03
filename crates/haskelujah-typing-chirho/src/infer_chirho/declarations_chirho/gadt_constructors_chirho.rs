// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor schemes and named-field inventories share the canonical GADT
//! signature. Pattern contexts are givens; expressions still demand them.
//! Workflow: compiler-pipeline-chirho/gadt-constructor-carrier-chirho.md.

use super::{InferCtxChirho, records_chirho};
use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};
use haskelujah_ast_chirho::decl_chirho::{GadtConSignatureChirho, StrictnessChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use std::collections::{HashMap, HashSet};

impl InferCtxChirho {
    pub(super) fn bind_gadt_constructor_chirho(
        &mut self,
        name_chirho: &NameChirho,
        signature_chirho: &GadtConSignatureChirho,
        seed_chirho: &HashMap<String, TyVarChirho>,
        generative_names_chirho: &HashSet<String>,
    ) {
        let ty_chirho = signature_chirho.as_type_chirho();
        self.note_written_constructor_context_chirho(name_chirho, &ty_chirho);
        let (scheme_chirho, _) =
            self.ast_type_to_scheme_seeded_chirho(&ty_chirho, seed_chirho, false);
        if signature_chirho
            .positional_fields_chirho()
            .any(|field_chirho| field_chirho.strictness_chirho != StrictnessChirho::LazyChirho)
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
            let mut field_index_chirho = 0_usize;
            for field_chirho in fields_chirho {
                let available_chirho = signature_chirho.field_selector_available_chirho(
                    field_chirho,
                    &|head_chirho| {
                        generative_names_chirho.contains(&head_chirho.full_name_chirho())
                    },
                );
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
                            field_name_chirho.text_chirho().to_string(),
                            selector_scheme_chirho,
                        );
                    }
                    field_index_chirho += 1;
                }
            }
        }
        self.env_chirho
            .bind_chirho(name_chirho.text_chirho().to_string(), scheme_chirho);
    }
}
