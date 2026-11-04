// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written types as synonym right-hand sides, and their free type variables.
//! Moved from `infer_chirho.rs` in row 661.

use super::subst_named_var_chirho;
use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};
use haskelujah_ast_chirho::promotion_chirho::{
    PromotionScopeChirho, quoted_promoted_spelling_chirho, unquoted_promoted_spelling_chirho,
};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

/// Actual compiler Proxy producers apply its complete checked telescope.
pub(super) fn mk_proxy_ty_chirho(kind_chirho: TyChirho, argument_chirho: TyChirho) -> TyChirho {
    let declaration_chirho =
        haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho::compiler_data_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerTypeConstructorChirho::ProxyChirho,
        );
    TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(declaration_chirho.type_key_chirho())),
            Box::new(kind_chirho),
        )),
        Box::new(argument_chirho),
    )
}

pub(super) fn mk_type_rep_ty_chirho(arg_ty_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("TypeRep".to_string())),
        Box::new(arg_ty_chirho),
    )
}

/// Convert an AST `TypeChirho` to a `TyChirho` suitable for storing as a type
/// synonym RHS. Synonym parameters are represented as `ForallVarChirho` so
/// they can be substituted during expansion.
pub(super) fn ast_type_to_syn_rhs_chirho(
    ty_chirho: &TypeChirho,
    params_chirho: &[String],
    scope_chirho: Option<&PromotionScopeChirho>,
) -> TyChirho {
    match ty_chirho {
        TypeChirho::KindAnnotationChirho { ty_chirho, .. } => {
            ast_type_to_syn_rhs_chirho(ty_chirho, params_chirho, scope_chirho)
        }
        TypeChirho::KindApplicationChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => TyChirho::AppChirho(
            Box::new(ast_type_to_syn_rhs_chirho(
                fun_chirho,
                params_chirho,
                scope_chirho,
            )),
            Box::new(ast_type_to_syn_rhs_chirho(
                arg_chirho,
                params_chirho,
                scope_chirho,
            )),
        ),
        TypeChirho::VarChirho(name_chirho) => {
            let text_chirho = name_chirho.text_chirho().to_string();
            if params_chirho.contains(&text_chirho) {
                TyChirho::ForallVarChirho(text_chirho)
            } else {
                // Unknown type variable — treat as Con (might be a bug upstream)
                TyChirho::ConChirho(text_chirho)
            }
        }
        TypeChirho::ConChirho(name_chirho) => TyChirho::ConChirho(
            name_chirho
                .family_key_chirho()
                .map(str::to_owned)
                .or_else(|| {
                    name_chirho
                        .type_constructor_chirho()
                        .map(|declaration_chirho| declaration_chirho.type_key_chirho())
                })
                .or_else(|| unquoted_promoted_spelling_chirho(name_chirho, scope_chirho))
                .unwrap_or_else(|| name_chirho.full_name_chirho()),
        ),
        TypeChirho::ListChirho { element_chirho, .. } => TyChirho::ListChirho(Box::new(
            ast_type_to_syn_rhs_chirho(element_chirho, params_chirho, scope_chirho),
        )),
        TypeChirho::FunChirho {
            arg_chirho,
            mult_chirho,
            result_chirho,
            ..
        } => {
            let m_chirho = match mult_chirho {
                Some(haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho) => {
                    MultChirho::OneChirho
                }
                _ => MultChirho::ManyChirho,
            };
            TyChirho::FunChirho(
                Box::new(ast_type_to_syn_rhs_chirho(
                    arg_chirho,
                    params_chirho,
                    scope_chirho,
                )),
                Box::new(ast_type_to_syn_rhs_chirho(
                    result_chirho,
                    params_chirho,
                    scope_chirho,
                )),
                m_chirho,
            )
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => TyChirho::TupleChirho(
            elements_chirho
                .iter()
                .map(|e_chirho| ast_type_to_syn_rhs_chirho(e_chirho, params_chirho, scope_chirho))
                .collect(),
        ),
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => TyChirho::AppChirho(
            Box::new(ast_type_to_syn_rhs_chirho(
                fun_chirho,
                params_chirho,
                scope_chirho,
            )),
            Box::new(ast_type_to_syn_rhs_chirho(
                arg_chirho,
                params_chirho,
                scope_chirho,
            )),
        ),
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            ast_type_to_syn_rhs_chirho(inner_chirho, params_chirho, scope_chirho)
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            // Preserve constraint context but convert the body.
            // For now, just pass through the body since constraints in
            // synonym RHS are handled during instantiation.
            // TODO: properly preserve constraints as deferred predicates
            let _ = context_chirho;
            ast_type_to_syn_rhs_chirho(body_chirho, params_chirho, scope_chirho)
        }
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            // Preserve the forall structure in the synonym RHS so that
            // higher-rank type synonyms like `type Locker = forall a. IO a -> IO a`
            // retain their polymorphic structure when expanded.
            let forall_var_names_chirho: Vec<String> = vars_chirho
                .iter()
                .map(|v_chirho| v_chirho.text_chirho().to_string())
                .collect();
            // Extend params with forall-bound vars so they become ForallVarChirho
            let mut extended_params_chirho = params_chirho.to_vec();
            extended_params_chirho.extend(forall_var_names_chirho.iter().cloned());
            let body_ty_chirho =
                ast_type_to_syn_rhs_chirho(body_chirho, &extended_params_chirho, scope_chirho);
            // Create ForallChirho with TyVarChirho identifiers for the bound vars
            let bound_vars_chirho: Vec<TyVarChirho> = forall_var_names_chirho
                .iter()
                .enumerate()
                .map(|(i_chirho, _)| TyVarChirho(8000 + i_chirho as u32))
                .collect();
            // Replace ForallVarChirho names with the TyVarChirho references
            let mut renamed_body_chirho = body_ty_chirho;
            for (name_chirho, tv_chirho) in
                forall_var_names_chirho.iter().zip(bound_vars_chirho.iter())
            {
                renamed_body_chirho = subst_named_var_chirho(
                    &renamed_body_chirho,
                    name_chirho,
                    &TyChirho::VarChirho(*tv_chirho),
                );
            }
            TyChirho::ForallChirho {
                vars_chirho: bound_vars_chirho,
                body_chirho: Box::new(renamed_body_chirho),
            }
        }
        TypeChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            let binder_names_chirho: Vec<String> = vars_chirho
                .iter()
                .map(|var_chirho| var_chirho.text_chirho().to_string())
                .collect();
            let mut extended_params_chirho = params_chirho.to_vec();
            extended_params_chirho.extend(binder_names_chirho.iter().cloned());
            let body_ty_chirho =
                ast_type_to_syn_rhs_chirho(body_chirho, &extended_params_chirho, scope_chirho);
            let bound_vars_chirho: Vec<TyVarChirho> = binder_names_chirho
                .iter()
                .enumerate()
                .map(|(index_chirho, _)| TyVarChirho(8500 + index_chirho as u32))
                .collect();
            let mut renamed_body_chirho = body_ty_chirho;
            for (name_chirho, ty_var_chirho) in
                binder_names_chirho.iter().zip(bound_vars_chirho.iter())
            {
                renamed_body_chirho = subst_named_var_chirho(
                    &renamed_body_chirho,
                    name_chirho,
                    &TyChirho::VarChirho(*ty_var_chirho),
                );
            }
            TyChirho::RequiredForallChirho {
                vars_chirho: bound_vars_chirho,
                body_chirho: Box::new(renamed_body_chirho),
            }
        }
        TypeChirho::PromotedConChirho { name_chirho, .. } => TyChirho::ConChirho(
            quoted_promoted_spelling_chirho(name_chirho, scope_chirho).unwrap_or_else(|| {
                // Legacy spelling for a caller that never ran naming: not the corrected
                // identity, and it differs from the use-site converter's.
                if name_chirho.text_chirho() == ":" {
                    ":".to_string()
                } else {
                    format!("'{}", name_chirho.text_chirho())
                }
            }),
        ),
        TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => TyChirho::promoted_list_chirho(
            elements_chirho.iter().map(|element_chirho| {
                ast_type_to_syn_rhs_chirho(element_chirho, params_chirho, scope_chirho)
            }),
            scope_chirho.is_some(),
        ),
        // PartialTypeSignatures: wildcard in a type synonym RHS is a fresh
        // anonymous variable (synthesised as "_wildcard_chirho").
        TypeChirho::WildcardChirho { .. } => TyChirho::ConChirho("_wildcard_chirho".to_string()),
        // Type-level literal (DataKinds): treat as a type-level constant.
        TypeChirho::LitChirho { value_chirho, .. } => TyChirho::ConChirho(value_chirho.clone()),
    }
}

/// Collect free type variable names from AST types (for type family instances).
pub(super) fn collect_free_type_vars_from_ast_chirho(types_chirho: &[TypeChirho]) -> Vec<String> {
    fn walk_chirho(
        ty_chirho: &TypeChirho,
        variables_chirho: &mut Vec<String>,
        seen_chirho: &mut std::collections::HashSet<String>,
        bound_chirho: &mut Vec<String>,
    ) {
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                let name_chirho = name_chirho.text_chirho();
                if !bound_chirho
                    .iter()
                    .any(|bound_chirho| bound_chirho == name_chirho)
                    && seen_chirho.insert(name_chirho.to_owned())
                {
                    variables_chirho.push(name_chirho.to_owned());
                }
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            }
            | TypeChirho::KindApplicationChirho {
                fun_chirho,
                arg_chirho,
                ..
            }
            | TypeChirho::KindAnnotationChirho {
                ty_chirho: fun_chirho,
                kind_chirho: arg_chirho,
                ..
            }
            | TypeChirho::FunChirho {
                arg_chirho: fun_chirho,
                result_chirho: arg_chirho,
                ..
            } => {
                walk_chirho(fun_chirho, variables_chirho, seen_chirho, bound_chirho);
                walk_chirho(arg_chirho, variables_chirho, seen_chirho, bound_chirho);
            }
            TypeChirho::ListChirho { element_chirho, .. }
            | TypeChirho::ParenChirho {
                inner_chirho: element_chirho,
                ..
            }
            | TypeChirho::QualChirho {
                body_chirho: element_chirho,
                ..
            } => walk_chirho(element_chirho, variables_chirho, seen_chirho, bound_chirho),
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    walk_chirho(element_chirho, variables_chirho, seen_chirho, bound_chirho);
                }
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            }
            | TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let prior_chirho = bound_chirho.len();
                bound_chirho.extend(
                    vars_chirho
                        .iter()
                        .map(|variable_chirho| variable_chirho.text_chirho().to_owned()),
                );
                for binder_chirho in vars_chirho {
                    if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                        walk_chirho(kind_chirho, variables_chirho, seen_chirho, bound_chirho);
                    }
                }
                walk_chirho(body_chirho, variables_chirho, seen_chirho, bound_chirho);
                bound_chirho.truncate(prior_chirho);
            }
            _ => {}
        }
    }
    let mut variables_chirho = Vec::new();
    let mut seen_chirho = std::collections::HashSet::new();
    for ty_chirho in types_chirho {
        walk_chirho(
            ty_chirho,
            &mut variables_chirho,
            &mut seen_chirho,
            &mut Vec::new(),
        );
    }
    variables_chirho
}
