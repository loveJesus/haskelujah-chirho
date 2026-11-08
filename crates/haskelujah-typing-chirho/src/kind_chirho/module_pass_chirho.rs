// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The module kind pass and its public entry points: phase 1 (declarations and
//! signatures, in source order), phase 1.5 (instance heads) and phase 2 (finalize).
//! workflow: language-features-chirho/declaration-kinds-chirho

use super::*;

/// Result of kind inference on a module.
#[derive(Debug)]
pub struct KindResultChirho {
    /// The kind environment after inference (type constructors → kinds).
    pub env_chirho: KindEnvChirho,
    /// Diagnostics (errors and warnings) from kind checking.
    pub diagnostics_chirho: DiagnosticBundleChirho,
}

/// What a module's imports tell the kind pass (K01 C): the builtin classes its class
/// spellings name, as naming proved them, and how many of its declarations were written.
/// workflow: compiler-pipeline-chirho/builtin-class-identity-chirho
#[derive(Debug, Clone, Copy)]
pub struct KindImportsChirho<'a> {
    /// The builtin classes the module's spellings name.
    pub builtin_classes_chirho: &'a BuiltinClassScopeChirho,
    /// The declarations written in source (after splices). Deriving appends its
    /// instances after them, and only written builtin instance heads are judged: a
    /// derived head applies every type parameter, which is the deriver's own boundary.
    pub written_decl_count_chirho: usize,
}

/// Run kind inference on a module's type declarations and type signatures, knowing
/// nothing from its imports: no spelling names a builtin class.
pub fn infer_module_kinds_chirho(module_chirho: &ModuleChirho) -> KindResultChirho {
    infer_module_kinds_with_imports_chirho(
        module_chirho,
        KindImportsChirho {
            builtin_classes_chirho: &BuiltinClassScopeChirho::default(),
            written_decl_count_chirho: module_chirho.decls_chirho.len(),
        },
    )
}

/// Run kind inference on a module's type declarations and type signatures, with what its
/// imports tell the kind pass.
pub fn infer_module_kinds_with_imports_chirho(
    module_chirho: &ModuleChirho,
    imports_chirho: KindImportsChirho<'_>,
) -> KindResultChirho {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());
    let mut projection_budget_chirho =
        haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho();
    let local_kind_decl_names_chirho: std::collections::HashSet<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho { name_chirho, .. }
            | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
            | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
            | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. }
            | DeclChirho::ClassDeclChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect();
    for local_kind_decl_name_chirho in &local_kind_decl_names_chirho {
        ctx_chirho
            .env_chirho
            .kinds_chirho
            .remove(local_kind_decl_name_chirho);
    }
    ctx_chirho.local_kind_decl_names_chirho = local_kind_decl_names_chirho.clone();
    ctx_chirho.seed_builtin_kinds_chirho(imports_chirho.builtin_classes_chirho);
    let poly_kinds_enabled_chirho = module_chirho
        .extensions_chirho
        .iter()
        .any(|e_chirho| e_chirho == "PolyKinds" || e_chirho == "TypeInType");

    // Phase 1: Process all type/data/newtype/class declarations to establish
    // the kind of each type constructor.
    for decl_chirho in &module_chirho.decls_chirho {
        if let Err(error_chirho) =
            haskelujah_ast_chirho::decl_chirho::check_declaration_projection_chirho(
                decl_chirho,
                &mut projection_budget_chirho,
                1,
            )
        {
            ctx_chirho
                .diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_chirho(
                    error_chirho.to_string(),
                    error_chirho.span_chirho,
                ));
            continue;
        }
        // Reset kind variable cache between declarations so that kind
        // variables from one declaration don't leak into the next.
        ctx_chirho.kind_var_cache_chirho.clear();

        // Snapshot the env keys so we can remove per-declaration locals
        // (type variables) after processing this declaration, preventing
        // scope leaks into subsequent declarations.
        let env_keys_before_chirho: std::collections::HashSet<String> =
            ctx_chirho.env_chirho.kinds_chirho.keys().cloned().collect();
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                kind_sig_chirho,
                span_chirho,
                ..
            } => {
                ctx_chirho.infer_data_decl_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    kind_sig_chirho.as_ref(),
                    *span_chirho,
                );
                // Kind-check constructor field types.
                for con_chirho in constructors_chirho {
                    match con_chirho {
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            fields_chirho,
                            ..
                        } => {
                            for (_strictness_chirho, field_ty_chirho) in fields_chirho {
                                let k_chirho = ctx_chirho.infer_type_kind_chirho(field_ty_chirho);
                                ctx_chirho.unify_chirho(
                                    &k_chirho,
                                    &KindChirho::StarChirho,
                                    "data constructor field",
                                    *span_chirho,
                                );
                                ctx_chirho.report_constraint_at_type_position_chirho(
                                    field_ty_chirho,
                                    &k_chirho,
                                );
                            }
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                            fields_chirho,
                            ..
                        } => {
                            for field_decl_chirho in fields_chirho {
                                let k_chirho =
                                    ctx_chirho.infer_type_kind_chirho(&field_decl_chirho.ty_chirho);
                                ctx_chirho.unify_chirho(
                                    &k_chirho,
                                    &KindChirho::StarChirho,
                                    "data constructor field",
                                    *span_chirho,
                                );
                                ctx_chirho.report_constraint_at_type_position_chirho(
                                    &field_decl_chirho.ty_chirho,
                                    &k_chirho,
                                );
                            }
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                            signature_chirho,
                            ..
                        } => {
                            let ty_chirho = &match signature_chirho
                                .try_as_type_chirho(&mut projection_budget_chirho)
                            {
                                Ok(ty_chirho) => ty_chirho,
                                Err(error_chirho) => {
                                    ctx_chirho.diagnostics_chirho.push_chirho(
                                        DiagnosticChirho::error_chirho(
                                            error_chirho.to_string(),
                                            error_chirho.span_chirho(),
                                        ),
                                    );
                                    continue;
                                }
                            };
                            // Kind-check the full GADT type signature.
                            let k_chirho = ctx_chirho.infer_type_kind_chirho(ty_chirho);
                            ctx_chirho.unify_chirho(
                                &k_chirho,
                                &KindChirho::StarChirho,
                                "GADT constructor type",
                                *span_chirho,
                            );
                        }
                    }
                }
            }
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructor_chirho,
                kind_sig_chirho,
                span_chirho,
                ..
            } => {
                ctx_chirho.infer_data_decl_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    kind_sig_chirho.as_ref(),
                    *span_chirho,
                );
                // Kind-check the newtype constructor field.
                match constructor_chirho.as_ref() {
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                        fields_chirho,
                        ..
                    } => {
                        for (_strictness_chirho, field_ty_chirho) in fields_chirho {
                            let k_chirho = ctx_chirho.infer_type_kind_chirho(field_ty_chirho);
                            ctx_chirho.unify_chirho(
                                &k_chirho,
                                &KindChirho::StarChirho,
                                "newtype constructor field",
                                *span_chirho,
                            );
                            ctx_chirho.report_constraint_at_type_position_chirho(
                                field_ty_chirho,
                                &k_chirho,
                            );
                        }
                    }
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                        fields_chirho,
                        ..
                    } => {
                        for field_decl_chirho in fields_chirho {
                            let k_chirho =
                                ctx_chirho.infer_type_kind_chirho(&field_decl_chirho.ty_chirho);
                            ctx_chirho.unify_chirho(
                                &k_chirho,
                                &KindChirho::StarChirho,
                                "newtype constructor field",
                                *span_chirho,
                            );
                            ctx_chirho.report_constraint_at_type_position_chirho(
                                &field_decl_chirho.ty_chirho,
                                &k_chirho,
                            );
                        }
                    }
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                        signature_chirho,
                        ..
                    } => {
                        let ty_chirho = &match signature_chirho
                            .try_as_type_chirho(&mut projection_budget_chirho)
                        {
                            Ok(ty_chirho) => ty_chirho,
                            Err(error_chirho) => {
                                ctx_chirho.diagnostics_chirho.push_chirho(
                                    DiagnosticChirho::error_chirho(
                                        error_chirho.to_string(),
                                        error_chirho.span_chirho(),
                                    ),
                                );
                                continue;
                            }
                        };
                        // Kind-check the full GADT type signature.
                        let k_chirho = ctx_chirho.infer_type_kind_chirho(ty_chirho);
                        ctx_chirho.unify_chirho(
                            &k_chirho,
                            &KindChirho::StarChirho,
                            "newtype GADT constructor type",
                            *span_chirho,
                        );
                    }
                }
            }
            DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                rhs_chirho,
                span_chirho,
            } => {
                ctx_chirho.infer_type_alias_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    rhs_chirho,
                    *span_chirho,
                );
            }
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                result_kind_chirho,
                standalone_kind_chirho,
                span_chirho,
                ..
            } => {
                ctx_chirho.infer_type_family_decl_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    result_kind_chirho.as_ref(),
                    standalone_kind_chirho.as_ref(),
                    *span_chirho,
                    poly_kinds_enabled_chirho,
                );
            }
            DeclChirho::ClassDeclChirho {
                context_chirho,
                name_chirho,
                type_vars_chirho,
                methods_chirho,
                associated_tfs_chirho,
                span_chirho,
                ..
            } => {
                ctx_chirho.infer_class_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    *span_chirho,
                );
                for assoc_tf_chirho in associated_tfs_chirho {
                    let mut param_kinds_chirho = Vec::new();
                    for type_var_chirho in &assoc_tf_chirho.type_vars_chirho {
                        let text_chirho = type_var_chirho.name_chirho.text_chirho();
                        let kind_chirho = if let Some(annotation_chirho) =
                            &type_var_chirho.kind_annotation_chirho
                        {
                            ctx_chirho.type_to_kind_chirho(annotation_chirho)
                        } else if let Some(existing_chirho) =
                            ctx_chirho.env_chirho.lookup_chirho(text_chirho)
                        {
                            existing_chirho.clone()
                        } else {
                            ctx_chirho.fresh_kind_chirho()
                        };
                        ctx_chirho
                            .env_chirho
                            .bind_chirho(text_chirho.to_string(), kind_chirho.clone());
                        if type_var_chirho.is_visible_chirho() {
                            param_kinds_chirho.push(kind_chirho);
                        }
                    }

                    let result_kind_chirho = if let Some(result_kind_chirho) =
                        &assoc_tf_chirho.result_kind_chirho
                    {
                        ctx_chirho.type_to_kind_chirho(result_kind_chirho)
                    } else if let Some(default_rhs_chirho) = &assoc_tf_chirho.default_rhs_chirho {
                        ctx_chirho.infer_type_kind_chirho(default_rhs_chirho)
                    } else {
                        ctx_chirho.fresh_kind_chirho()
                    };

                    let family_kind_chirho =
                        KindChirho::arrow_n_chirho(param_kinds_chirho, result_kind_chirho.clone());
                    let assoc_name_text_chirho = assoc_tf_chirho.name_chirho.text_chirho();
                    ctx_chirho
                        .env_chirho
                        .bind_chirho(assoc_name_text_chirho.to_string(), family_kind_chirho);
                }
                // Kind-check superclass constraints — do NOT force args to *,
                // since constraint args like `f` in `Applicative f` may be `* -> *`.
                for constraint_chirho in context_chirho {
                    let _k_chirho = ctx_chirho.infer_constraint_kind_chirho(constraint_chirho);
                }
                // Kind-check method type signatures.
                for method_chirho in methods_chirho {
                    let k_chirho = ctx_chirho.infer_type_kind_chirho(&method_chirho.ty_chirho);
                    ctx_chirho.unify_chirho(
                        &k_chirho,
                        &KindChirho::StarChirho,
                        "class method type",
                        method_chirho.span_chirho,
                    );
                    ctx_chirho.report_constraint_at_type_position_chirho(
                        &method_chirho.ty_chirho,
                        &k_chirho,
                    );
                }
            }
            DeclChirho::TypeSigChirho {
                ty_chirho,
                span_chirho,
                ..
            } => {
                // Kind-check standalone type signatures.
                let k_chirho = ctx_chirho.infer_type_kind_chirho(ty_chirho);
                ctx_chirho.unify_chirho(
                    &k_chirho,
                    &KindChirho::StarChirho,
                    "type signature",
                    *span_chirho,
                );
                ctx_chirho.report_constraint_at_type_position_chirho(ty_chirho, &k_chirho);
            }
            _ => {}
        }

        // Remove per-declaration locals (type variables like `a`, `b`, `k`)
        // that were added during this declaration but shouldn't persist.
        // Keep only type constructor names that were in the env before or
        // that look like type constructors (uppercase first char).
        let keys_to_remove_chirho: Vec<String> = ctx_chirho
            .env_chirho
            .kinds_chirho
            .keys()
            .filter(|k_chirho| {
                !env_keys_before_chirho.contains(*k_chirho)
                    && k_chirho
                        .chars()
                        .next()
                        .is_none_or(|c_chirho| c_chirho.is_lowercase())
            })
            .cloned()
            .collect();
        for key_chirho in keys_to_remove_chirho {
            ctx_chirho.env_chirho.kinds_chirho.remove(&key_chirho);
        }
    }

    // Phase 1.5: instance heads. Runs as its own pass so every class kind is
    // established regardless of declaration order (an instance may precede its
    // class in the source). Instance heads were never kind-checked at all, so
    // `class MonadReader a b` + `instance MonadReader Int` was accepted.
    //
    // Under-application is only reported when every head form is one our
    // lowering represents faithfully. It is NOT, under these extensions: a head
    // argument may be a type-level literal (`instance A 0`), a promoted list
    // (`instance All c '[]`), an operator type (`instance Category (->)`), or a
    // variable-headed application (`instance MonadReader r (Reader r)`, which
    // lowers to ONE argument instead of two). None of those reach
    // `types_chirho`, so the head looks short and we would report OUR gap as
    // the program's error. Over-application stays sound either way — a dropped
    // argument can only shorten the head.
    let instance_head_forms_representable_chirho =
        !module_chirho.extensions_chirho.iter().any(|e_chirho| {
            e_chirho == "DataKinds"
                || e_chirho == "PolyKinds"
                || e_chirho == "TypeInType"
                || e_chirho == "TypeOperators"
                || e_chirho == "FlexibleInstances"
        });
    for (index_chirho, decl_chirho) in module_chirho.decls_chirho.iter().enumerate() {
        let DeclChirho::InstanceDeclChirho {
            class_chirho,
            types_chirho,
            span_chirho,
            ..
        } = decl_chirho
        else {
            continue;
        };
        if index_chirho < imports_chirho.written_decl_count_chirho
            && ctx_chirho.check_builtin_instance_head_chirho(
                &class_chirho.full_name_chirho(),
                types_chirho,
                *span_chirho,
            )
        {
            continue;
        }
        ctx_chirho.check_instance_head_arity_chirho(
            class_chirho.text_chirho(),
            types_chirho,
            instance_head_forms_representable_chirho,
            *span_chirho,
        );
    }

    // Phase 2: Finalize — apply substitution and default unconstrained vars.
    ctx_chirho.finalize_chirho(poly_kinds_enabled_chirho);

    KindResultChirho {
        env_chirho: ctx_chirho.env_chirho,
        diagnostics_chirho: ctx_chirho.diagnostics_chirho,
    }
}
