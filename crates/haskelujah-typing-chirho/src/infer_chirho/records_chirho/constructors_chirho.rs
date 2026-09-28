// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared constructor and selector registration for ordinary and family data.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::*;

impl InferCtxChirho {
    pub(super) fn register_data_constructors_chirho(
        &mut self,
        owner_chirho: &str,
        constructors_chirho: &[haskelujah_ast_chirho::decl_chirho::ConDeclChirho],
        result_ty_chirho: TyChirho,
        mut tv_map_chirho: HashMap<String, TyVarChirho>,
        instance_chirho: bool,
    ) {
        let data_type_name_chirho = owner_chirho.to_owned();
        for con_chirho in constructors_chirho {
            self.data_constructors_chirho
                .entry(data_type_name_chirho.clone())
                .or_default()
                .push(
                    records_chirho::con_decl_name_chirho(con_chirho)
                        .text_chirho()
                        .to_string(),
                );
        }
        for con_chirho in constructors_chirho {
            match con_chirho {
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                    name_chirho,
                    fields_chirho,
                    ..
                } => {
                    if fields_chirho.iter().any(|(strictness_chirho, _ty_chirho)| {
                        *strictness_chirho
                            != haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho
                    }) {
                        self.strict_positional_constructors_chirho
                            .insert(name_chirho.text_chirho().to_string());
                    }
                    let field_tys_chirho: Vec<TyChirho> = fields_chirho
                        .iter()
                        .map(|(_s_chirho, ty_chirho)| {
                            self.ast_type_to_ty_chirho(ty_chirho, &mut tv_map_chirho)
                        })
                        .collect();
                    let con_ty_chirho =
                        TyChirho::fun_n_chirho(field_tys_chirho, result_ty_chirho.clone());
                    let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                    self.env_chirho
                        .bind_chirho(name_chirho.text_chirho().to_string(), gen_scheme_chirho);
                }
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    name_chirho,
                    fields_chirho,
                    ..
                } => {
                    let field_tys_chirho: Vec<TyChirho> = fields_chirho
                        .iter()
                        .flat_map(|fd_chirho| {
                            let ty_chirho = self
                                .ast_type_to_ty_chirho(&fd_chirho.ty_chirho, &mut tv_map_chirho);
                            std::iter::repeat_n(ty_chirho, fd_chirho.names_chirho.len())
                        })
                        .collect();
                    let con_ty_chirho =
                        TyChirho::fun_n_chirho(field_tys_chirho.clone(), result_ty_chirho.clone());
                    let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                    self.env_chirho
                        .bind_chirho(name_chirho.text_chirho().to_string(), gen_scheme_chirho);
                    // Store field names for RecordWildCards expansion
                    let field_names_chirho: Vec<String> = fields_chirho
                        .iter()
                        .flat_map(|fd_chirho| {
                            fd_chirho
                                .names_chirho
                                .iter()
                                .map(|n_chirho| n_chirho.text_chirho().to_string())
                        })
                        .collect();
                    self.con_field_names_chirho
                        .insert(name_chirho.text_chirho().to_string(), field_names_chirho);
                    self.con_strict_fields_chirho.insert(
                        name_chirho.text_chirho().to_string(),
                        records_chirho::strict_field_names_chirho(fields_chirho),
                    );
                    // Bind field accessor functions: fieldName :: T -> FieldType
                    // Use a running index into field_tys_chirho (which is
                    // flattened by field names, not by field declarations).
                    let mut ft_idx_chirho = 0;
                    for fd_chirho in fields_chirho {
                        for fname_chirho in &fd_chirho.names_chirho {
                            if ft_idx_chirho < field_tys_chirho.len() {
                                let accessor_ty_chirho = TyChirho::FunChirho(
                                    Box::new(result_ty_chirho.clone()),
                                    Box::new(field_tys_chirho[ft_idx_chirho].clone()),
                                    MultChirho::ManyChirho,
                                );
                                let accessor_scheme_chirho =
                                    self.generalize_chirho(&accessor_ty_chirho);
                                self.env_chirho.bind_chirho(
                                    fname_chirho.text_chirho().to_string(),
                                    accessor_scheme_chirho,
                                );
                            }
                            ft_idx_chirho += 1;
                        }
                    }
                }
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                    name_chirho,
                    ty_chirho,
                    span_chirho,
                } => {
                    // Convert the full GADT type signature to TyChirho.
                    // The return type comes from the signature itself,
                    // NOT from the auto-constructed `T a b c`.
                    let con_ty_chirho = self.ast_type_to_ty_chirho(ty_chirho, &mut tv_map_chirho);
                    if instance_chirho
                        && !self.check_data_instance_result_chirho(
                            &result_ty_chirho,
                            &con_ty_chirho,
                            *span_chirho,
                        )
                    {
                        continue;
                    }
                    let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                    self.env_chirho
                        .bind_chirho(name_chirho.text_chirho().to_string(), gen_scheme_chirho);
                }
            }
        }
    }

    /// A GADT may refine variables, but cannot change a written instance index.
    /// A kind-signature-style head (e.g. `Sing :: Bool -> Type`) leaves trailing
    /// arguments to the constructors. Compare its entire written prefix,
    /// retaining hidden arguments, without publishing the validation unifier.
    fn check_data_instance_result_chirho(
        &mut self,
        instance_head_chirho: &TyChirho,
        constructor_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> bool {
        let result_chirho = self.constructor_result_type_chirho(constructor_chirho.clone());
        let (_, expected_args_chirho) =
            super::family_declarations_chirho::family_application_spine_chirho(
                instance_head_chirho,
            );
        let (actual_head_chirho, actual_args_chirho) =
            super::family_declarations_chirho::family_application_spine_chirho(&result_chirho);
        let mut prefix_chirho = actual_head_chirho.clone();
        for (argument_chirho, hidden_chirho) in
            actual_args_chirho.iter().take(expected_args_chirho.len())
        {
            prefix_chirho = if *hidden_chirho {
                TyChirho::KindAppChirho(
                    Box::new(prefix_chirho),
                    Box::new((*argument_chirho).clone()),
                )
            } else {
                TyChirho::AppChirho(
                    Box::new(prefix_chirho),
                    Box::new((*argument_chirho).clone()),
                )
            };
        }
        match unify_chirho(instance_head_chirho, &prefix_chirho, span_chirho) {
            Ok(_) => true,
            Err(error_chirho) => {
                self.report_unify_error_chirho(&error_chirho);
                false
            }
        }
    }
}
