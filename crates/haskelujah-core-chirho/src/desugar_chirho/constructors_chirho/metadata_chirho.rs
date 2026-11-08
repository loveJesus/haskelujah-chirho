// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Runtime constructor metadata and safe named-field accessors.
//! Workflow: compiler-pipeline-chirho/gadt-constructor-carrier-chirho.md.

use super::DesugarCtxChirho;
use haskelujah_ast_chirho::decl_chirho::{FieldDeclChirho, GadtProjectionBudgetChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;

pub(super) struct RecordSelectorPlanChirho<'module_chirho> {
    name_chirho: &'module_chirho NameChirho,
    fields_chirho: &'module_chirho [FieldDeclChirho],
    available_chirho: Vec<bool>,
}
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    InlineAnnotationChirho,
};
use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, StrictnessChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

impl DesugarCtxChirho {
    /// Populate constructor strictness and field name maps from data declarations.
    pub(super) fn collect_con_strictness_chirho(&mut self, module_chirho: &ModuleChirho) {
        for decl_chirho in &module_chirho.decls_chirho {
            let constructors_chirho: &[ConDeclChirho] = match decl_chirho {
                DeclChirho::DataDeclChirho {
                    constructors_chirho,
                    ..
                } => constructors_chirho,
                DeclChirho::NewtypeDeclChirho {
                    constructor_chirho, ..
                } => std::slice::from_ref(constructor_chirho),
                _ => continue,
            };
            for con_chirho in constructors_chirho {
                match con_chirho {
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho,
                        fields_chirho,
                        ..
                    } => {
                        self.con_arities_chirho
                            .insert(name_chirho.text_chirho().to_string(), fields_chirho.len());
                        let strictness_chirho: Vec<StrictnessChirho> = fields_chirho
                            .iter()
                            .map(|(s_chirho, _)| *s_chirho)
                            .collect();
                        if strictness_chirho
                            .iter()
                            .any(|s_chirho| s_chirho.is_strict_chirho())
                        {
                            self.con_strictness_chirho
                                .insert(name_chirho.text_chirho().to_string(), strictness_chirho);
                        }
                    }
                    ConDeclChirho::RecordChirho {
                        name_chirho,
                        fields_chirho,
                        ..
                    } => {
                        // Collect strictness
                        let strictness_chirho: Vec<StrictnessChirho> = fields_chirho
                            .iter()
                            .flat_map(|f_chirho| {
                                std::iter::repeat_n(
                                    f_chirho.strictness_chirho,
                                    f_chirho.names_chirho.len(),
                                )
                            })
                            .collect();
                        if strictness_chirho
                            .iter()
                            .any(|s_chirho| s_chirho.is_strict_chirho())
                        {
                            self.con_strictness_chirho
                                .insert(name_chirho.text_chirho().to_string(), strictness_chirho);
                        }
                        // Collect field names for RecordWildCards expansion
                        let field_names_chirho: Vec<String> = fields_chirho
                            .iter()
                            .flat_map(|f_chirho| {
                                f_chirho
                                    .names_chirho
                                    .iter()
                                    .map(|n_chirho| n_chirho.text_chirho().to_string())
                            })
                            .collect();
                        self.con_arities_chirho.insert(
                            name_chirho.text_chirho().to_string(),
                            field_names_chirho.len(),
                        );
                        self.con_field_names_chirho
                            .insert(name_chirho.text_chirho().to_string(), field_names_chirho);
                    }
                    ConDeclChirho::GadtChirho {
                        name_chirho,
                        signature_chirho,
                        ..
                    } => {
                        let strictness_chirho = signature_chirho
                            .positional_fields_chirho()
                            .map(|field_chirho| field_chirho.strictness_chirho)
                            .collect::<Vec<_>>();
                        self.con_arities_chirho.insert(
                            name_chirho.text_chirho().to_string(),
                            strictness_chirho.len(),
                        );
                        if strictness_chirho
                            .iter()
                            .any(|strictness_chirho| strictness_chirho.is_strict_chirho())
                        {
                            self.con_strictness_chirho
                                .insert(name_chirho.text_chirho().to_string(), strictness_chirho);
                        }
                        if let Some(fields_chirho) = signature_chirho.record_fields_chirho() {
                            self.con_field_names_chirho.insert(
                                name_chirho.text_chirho().to_string(),
                                fields_chirho
                                    .iter()
                                    .flat_map(|field_chirho| {
                                        field_chirho.names_chirho.iter().map(|name_chirho| {
                                            name_chirho.text_chirho().to_string()
                                        })
                                    })
                                    .collect(),
                            );
                        }
                    }
                }
            }
        }
    }

    pub(super) fn record_selector_plans_chirho<'module_chirho>(
        &self,
        module_chirho: &'module_chirho ModuleChirho,
        budget_chirho: &mut GadtProjectionBudgetChirho,
    ) -> Result<
        Vec<RecordSelectorPlanChirho<'module_chirho>>,
        haskelujah_ast_chirho::decl_chirho::ConstructorSignatureErrorChirho,
    > {
        let mut plans_chirho = Vec::new();
        for decl_chirho in &module_chirho.decls_chirho {
            let constructors_chirho = match decl_chirho {
                DeclChirho::DataDeclChirho {
                    constructors_chirho,
                    ..
                } => constructors_chirho.as_slice(),
                DeclChirho::NewtypeDeclChirho {
                    constructor_chirho, ..
                } => std::slice::from_ref(constructor_chirho),
                _ => continue,
            };
            for con_chirho in constructors_chirho {
                let Some(fields_chirho) = con_chirho.record_fields_chirho() else {
                    continue;
                };
                let (name_chirho, span_chirho) = match con_chirho {
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho,
                        span_chirho,
                        ..
                    }
                    | ConDeclChirho::RecordChirho {
                        name_chirho,
                        span_chirho,
                        ..
                    }
                    | ConDeclChirho::GadtChirho {
                        name_chirho,
                        span_chirho,
                        ..
                    } => (name_chirho, *span_chirho),
                };
                haskelujah_ast_chirho::decl_chirho::check_record_accessors_projection_chirho(
                    name_chirho,
                    fields_chirho,
                    span_chirho,
                    budget_chirho,
                )?;
                let available_chirho = if let ConDeclChirho::GadtChirho {
                    signature_chirho, ..
                } = con_chirho
                {
                    signature_chirho.selector_availability_chirho(
                        budget_chirho,
                        &|head_chirho| {
                            self.type_constructor_scope_chirho
                                .as_deref()
                                .and_then(|scope_chirho| {
                                    scope_chirho.declaration_chirho(&head_chirho.full_name_chirho())
                                })
                                .is_some_and(|declaration_chirho| {
                                    declaration_chirho.is_nominally_generative_chirho()
                                })
                        },
                    )?
                } else {
                    vec![true; fields_chirho.len()]
                };
                plans_chirho.push(RecordSelectorPlanChirho {
                    name_chirho,
                    fields_chirho,
                    available_chirho,
                });
            }
        }
        Ok(plans_chirho)
    }

    pub(super) fn prebind_record_accessors_chirho(
        &mut self,
        plans_chirho: &[RecordSelectorPlanChirho<'_>],
    ) {
        for plan_chirho in plans_chirho {
            for (field_chirho, available_chirho) in plan_chirho
                .fields_chirho
                .iter()
                .zip(&plan_chirho.available_chirho)
            {
                if !available_chirho {
                    continue;
                }
                for name_chirho in &field_chirho.names_chirho {
                    let name_chirho = name_chirho.text_chirho().to_string();
                    let binder_chirho = self.fresh_binder_chirho(
                        &name_chirho,
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    self.bind_in_scope_chirho(&name_chirho, binder_chirho.id_chirho);
                    let setter_name_chirho = format!("$setField_{name_chirho}");
                    let setter_chirho = self.fresh_binder_chirho(
                        &setter_name_chirho,
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    self.bind_in_scope_chirho(&setter_name_chirho, setter_chirho.id_chirho);
                }
            }
        }
    }

    pub(super) fn generate_record_accessors_chirho(
        &mut self,
        plans_chirho: &[RecordSelectorPlanChirho<'_>],
        bindings_chirho: &mut Vec<CoreBindingChirho>,
    ) {
        for plan_chirho in plans_chirho {
            let field_decls_chirho = plan_chirho.fields_chirho;
            let con_name_chirho = plan_chirho.name_chirho;
            // Flatten field names with their position index
            let mut flat_fields_chirho: Vec<(String, usize, bool)> = Vec::new();
            for (fd_chirho, available_chirho) in
                field_decls_chirho.iter().zip(&plan_chirho.available_chirho)
            {
                for fname_chirho in &fd_chirho.names_chirho {
                    flat_fields_chirho.push((
                        fname_chirho.text_chirho().to_string(),
                        flat_fields_chirho.len(),
                        *available_chirho,
                    ));
                }
            }
            let total_fields_chirho = flat_fields_chirho.len();
            let con_str_chirho = con_name_chirho.text_chirho().to_string();

            for (field_name_chirho, field_idx_chirho, available_chirho) in &flat_fields_chirho {
                if !available_chirho {
                    continue;
                }
                // Build: \r -> case r of { Con b0 b1 ... -> b_idx }
                let param_binder_chirho = self.fresh_binder_chirho(
                    "$r",
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let mut alt_binders_chirho = Vec::new();
                let mut result_id_chirho = CoreIdChirho(0);
                for i_chirho in 0..total_fields_chirho {
                    let b_chirho = self.fresh_binder_chirho(
                        &format!("$f{}", i_chirho),
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    if i_chirho == *field_idx_chirho {
                        result_id_chirho = b_chirho.id_chirho;
                    }
                    alt_binders_chirho.push(b_chirho);
                }

                let case_bind_chirho = self.fresh_binder_chirho(
                    "$scrut",
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let body_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                        param_binder_chirho.id_chirho,
                    )),
                    bind_chirho: case_bind_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(con_str_chirho.clone()),
                        binders_chirho: alt_binders_chirho,
                        rhs_chirho: CoreExprChirho::VarChirho(result_id_chirho),
                    }],
                };

                let accessor_rhs_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: param_binder_chirho.clone(),
                    body_chirho: Box::new(body_chirho),
                };

                // Reuse the pre-bound ID from Pass 1b
                let accessor_id_chirho = self
                    .lookup_scope_chirho(field_name_chirho)
                    .unwrap_or_else(|| self.fresh_id_chirho(field_name_chirho));
                let accessor_binder_chirho = BinderChirho {
                    id_chirho: accessor_id_chirho,
                    name_chirho: field_name_chirho.clone(),
                    ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                };
                bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: accessor_binder_chirho,
                    rhs_chirho: accessor_rhs_chirho,
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                });

                // Generate setter: $setField_f = \newVal -> \rec -> case rec of { Con b0 b1 ... -> Con ... newVal ... }
                let setter_name_chirho = format!("$setField_{}", field_name_chirho);
                let new_val_binder_chirho = self.fresh_binder_chirho(
                    "$newVal",
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let rec_binder_chirho = self.fresh_binder_chirho(
                    "$rec",
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let mut setter_alt_binders_chirho = Vec::new();
                let mut con_args_chirho = Vec::new();
                for i_chirho in 0..total_fields_chirho {
                    let b_chirho = self.fresh_binder_chirho(
                        &format!("$sf{}", i_chirho),
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    if i_chirho == *field_idx_chirho {
                        // Replace this field with newVal
                        con_args_chirho
                            .push(CoreExprChirho::VarChirho(new_val_binder_chirho.id_chirho));
                    } else {
                        con_args_chirho.push(CoreExprChirho::VarChirho(b_chirho.id_chirho));
                    }
                    setter_alt_binders_chirho.push(b_chirho);
                }
                let setter_case_bind_chirho = self.fresh_binder_chirho(
                    "$scrut",
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let setter_body_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                        rec_binder_chirho.id_chirho,
                    )),
                    bind_chirho: setter_case_bind_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(con_str_chirho.clone()),
                        binders_chirho: setter_alt_binders_chirho,
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: con_str_chirho.clone(),
                            args_chirho: con_args_chirho,
                        },
                    }],
                };
                let setter_rhs_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: new_val_binder_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: rec_binder_chirho,
                        body_chirho: Box::new(setter_body_chirho),
                    }),
                };
                // Reuse pre-bound setter ID from Pass 1b
                let setter_id_chirho = self
                    .lookup_scope_chirho(&setter_name_chirho)
                    .unwrap_or_else(|| self.fresh_id_chirho(&setter_name_chirho));
                let setter_binder_chirho = BinderChirho {
                    id_chirho: setter_id_chirho,
                    name_chirho: setter_name_chirho.clone(),
                    ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                };
                bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: setter_binder_chirho,
                    rhs_chirho: setter_rhs_chirho,
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                });
            }
        }
    }
}
