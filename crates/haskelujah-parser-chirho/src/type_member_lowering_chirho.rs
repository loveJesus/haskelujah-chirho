// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class-body type-member classification.
//!
//! The CST uses ordinary declaration nodes inside class bodies. This module
//! separates value methods from associated type/data-family declarations so
//! the large AST lowerer delegates one coherent responsibility.

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::{AssocTypeFamilyChirho, ClassMethodChirho, DeclChirho};
use haskelujah_ast_chirho::expr_chirho::MatchArmChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;

/// Partition lowered class-body declarations into value methods and type members.
///
/// Workflow: `spec-chirho/workflows-chirho/compiler-pipeline-chirho/
/// type-scope-resolution-chirho.md`.
pub(crate) fn partition_class_members_chirho(
    where_decls_chirho: Vec<DeclChirho>,
    default_impls_chirho: &HashMap<String, Vec<MatchArmChirho>>,
    default_sigs_chirho: &HashMap<String, String>,
    visible_kind_binder_names_chirho: &HashSet<String>,
) -> (Vec<ClassMethodChirho>, Vec<AssocTypeFamilyChirho>) {
    // `type Member :: Kind` currently reaches AST lowering as two type
    // signatures with the same span: the recovery name `type` and the real
    // member name. Use that structural marker rather than exporting either as
    // a value method.
    let associated_kind_sig_spans_chirho: HashSet<SpanChirho> = where_decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeSigChirho {
                name_chirho,
                span_chirho,
                ..
            } if name_chirho.text_chirho() == "type" => Some(*span_chirho),
            _ => None,
        })
        .collect();

    let mut methods_chirho = Vec::new();
    let mut associated_types_chirho = Vec::new();

    for decl_chirho in where_decls_chirho {
        match decl_chirho {
            DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                span_chirho,
            } => {
                if name_chirho.text_chirho() == "type" {
                    continue;
                }
                if associated_kind_sig_spans_chirho.contains(&span_chirho) {
                    associated_types_chirho.push(AssocTypeFamilyChirho {
                        name_chirho,
                        type_vars_chirho: Vec::new(),
                        default_rhs_chirho: None,
                        default_params_chirho: Vec::new(),
                        span_chirho,
                    });
                    continue;
                }
                let default_chirho = default_impls_chirho.get(name_chirho.text_chirho()).cloned();
                let default_sig_chirho =
                    default_sigs_chirho.get(name_chirho.text_chirho()).cloned();
                methods_chirho.push(ClassMethodChirho {
                    name_chirho,
                    ty_chirho,
                    default_chirho,
                    default_sig_chirho,
                    span_chirho,
                });
            }
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                equations_chirho,
                span_chirho,
                ..
            } => {
                let default_rhs_chirho =
                    (equations_chirho.len() == 1).then(|| equations_chirho[0].rhs_chirho.clone());
                let default_params_chirho = if equations_chirho.len() == 1 {
                    equations_chirho[0]
                        .lhs_types_chirho
                        .iter()
                        .map(|lhs_ty_chirho| match lhs_ty_chirho {
                            TypeChirho::VarChirho(name_chirho) => Some(name_chirho.clone()),
                            _ => None,
                        })
                        .collect::<Option<Vec<NameChirho>>>()
                        .unwrap_or_default()
                } else {
                    Vec::new()
                };
                associated_types_chirho.push(AssocTypeFamilyChirho {
                    name_chirho,
                    type_vars_chirho: type_vars_chirho
                        .into_iter()
                        .filter(|type_var_chirho| {
                            !visible_kind_binder_names_chirho
                                .contains(type_var_chirho.name_chirho.text_chirho())
                        })
                        .map(|type_var_chirho| type_var_chirho.name_chirho)
                        .collect(),
                    default_rhs_chirho,
                    default_params_chirho,
                    span_chirho,
                });
            }
            DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                rhs_chirho,
                span_chirho,
            } => {
                let type_var_names_chirho: Vec<NameChirho> = type_vars_chirho
                    .into_iter()
                    .filter(|type_var_chirho| {
                        !visible_kind_binder_names_chirho
                            .contains(type_var_chirho.name_chirho.text_chirho())
                    })
                    .map(|type_var_chirho| type_var_chirho.name_chirho)
                    .collect();
                let declared_names_chirho: HashSet<String> = type_var_names_chirho
                    .iter()
                    .map(|name_chirho| name_chirho.text_chirho().to_string())
                    .collect();
                let rhs_free_names_chirho = free_type_variable_names_chirho(&rhs_chirho);
                let default_rhs_chirho = rhs_free_names_chirho
                    .is_subset(&declared_names_chirho)
                    .then_some(rhs_chirho);
                associated_types_chirho.push(AssocTypeFamilyChirho {
                    name_chirho,
                    type_vars_chirho: type_var_names_chirho,
                    default_rhs_chirho,
                    default_params_chirho: Vec::new(),
                    span_chirho,
                });
            }
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                span_chirho,
                ..
            } => {
                associated_types_chirho.push(AssocTypeFamilyChirho {
                    name_chirho,
                    type_vars_chirho: type_vars_chirho
                        .into_iter()
                        .filter(|type_var_chirho| {
                            !visible_kind_binder_names_chirho
                                .contains(type_var_chirho.name_chirho.text_chirho())
                        })
                        .map(|type_var_chirho| type_var_chirho.name_chirho)
                        .collect(),
                    default_rhs_chirho: None,
                    default_params_chirho: Vec::new(),
                    span_chirho,
                });
            }
            _ => {}
        }
    }

    (methods_chirho, associated_types_chirho)
}

fn free_type_variable_names_chirho(ty_chirho: &TypeChirho) -> HashSet<String> {
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho) => {
            std::iter::once(name_chirho.text_chirho().to_string()).collect()
        }
        TypeChirho::ConChirho(_) => HashSet::new(),
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        }
        | TypeChirho::KindAppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            let mut names_chirho = free_type_variable_names_chirho(fun_chirho);
            names_chirho.extend(free_type_variable_names_chirho(arg_chirho));
            names_chirho
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            let mut names_chirho = free_type_variable_names_chirho(arg_chirho);
            names_chirho.extend(free_type_variable_names_chirho(result_chirho));
            names_chirho
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => elements_chirho
            .iter()
            .flat_map(free_type_variable_names_chirho)
            .collect(),
        TypeChirho::ListChirho { element_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: element_chirho,
            ..
        } => free_type_variable_names_chirho(element_chirho),
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            let mut names_chirho: HashSet<String> = context_chirho
                .iter()
                .flat_map(free_constraint_variable_names_chirho)
                .collect();
            names_chirho.extend(free_type_variable_names_chirho(body_chirho));
            names_chirho
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
            let mut names_chirho = free_type_variable_names_chirho(body_chirho);
            for type_var_chirho in vars_chirho {
                names_chirho.remove(type_var_chirho.text_chirho());
            }
            names_chirho
        }
        TypeChirho::PromotedConChirho { .. }
        | TypeChirho::WildcardChirho { .. }
        | TypeChirho::LitChirho { .. } => HashSet::new(),
    }
}

fn free_constraint_variable_names_chirho(constraint_chirho: &ConstraintChirho) -> HashSet<String> {
    match constraint_chirho {
        ConstraintChirho::ClassChirho { args_chirho, .. } => args_chirho
            .iter()
            .flat_map(free_type_variable_names_chirho)
            .collect(),
        ConstraintChirho::QuantifiedChirho {
            vars_chirho,
            context_chirho,
            body_chirho,
            ..
        } => {
            let mut names_chirho: HashSet<String> = context_chirho
                .iter()
                .flat_map(free_constraint_variable_names_chirho)
                .collect();
            names_chirho.extend(free_constraint_variable_names_chirho(body_chirho));
            for type_var_chirho in vars_chirho {
                names_chirho.remove(type_var_chirho.text_chirho());
            }
            names_chirho
        }
    }
}
