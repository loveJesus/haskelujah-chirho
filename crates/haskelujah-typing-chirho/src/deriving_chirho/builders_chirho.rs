// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared AST builders and constructor/type inspection for derived instances.
use super::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Dummy span for generated code.
pub(super) fn gen_span_chirho() -> SpanChirho {
    SpanChirho::DUMMY_CHIRHO
}

/// Create a simple variable name.
pub(super) fn var_name_chirho(s_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        s_chirho,
        gen_span_chirho(),
    ))
}

/// Create a variable expression.
pub(super) fn var_expr_chirho(s_chirho: &str) -> ExprChirho {
    ExprChirho::VarChirho(var_name_chirho(s_chirho))
}

/// Create a constructor expression.
pub(super) fn con_expr_chirho(s_chirho: &str) -> ExprChirho {
    ExprChirho::ConChirho(var_name_chirho(s_chirho))
}

/// Create a simple application `f x`.
pub(super) fn app_chirho(f_chirho: ExprChirho, x_chirho: ExprChirho) -> ExprChirho {
    ExprChirho::AppChirho {
        fun_chirho: Box::new(f_chirho),
        arg_chirho: Box::new(x_chirho),
        span_chirho: gen_span_chirho(),
    }
}

/// Create an infix application `a op b`.
pub(super) fn infix_chirho(
    left_chirho: ExprChirho,
    op_chirho: &str,
    right_chirho: ExprChirho,
) -> ExprChirho {
    ExprChirho::InfixChirho {
        left_chirho: Box::new(left_chirho),
        op_chirho: var_name_chirho(op_chirho),
        right_chirho: Box::new(right_chirho),
        span_chirho: gen_span_chirho(),
    }
}

pub(super) fn eq_primop_for_type_chirho(ty_chirho: &TypeChirho) -> Option<&'static str> {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => match name_chirho.text_chirho() {
            "Int" | "Integer" => Some("==#"),
            "Double" | "Float" => Some("eqFloat#"),
            "String" => Some("eqStr#"),
            _ => None,
        },
        TypeChirho::ListChirho { element_chirho, .. } => match element_chirho.as_ref() {
            TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Char" => {
                Some("eqStr#")
            }
            _ => None,
        },
        _ => None,
    }
}

pub(super) fn eq_expr_for_type_chirho(
    left_chirho: ExprChirho,
    right_chirho: ExprChirho,
    ty_chirho: &TypeChirho,
) -> ExprChirho {
    let op_chirho = eq_primop_for_type_chirho(ty_chirho).unwrap_or("==");
    infix_chirho(left_chirho, op_chirho, right_chirho)
}

/// Get the constructor name from a ConDeclChirho.
pub(super) fn con_name_chirho(con_chirho: &ConDeclChirho) -> &str {
    match con_chirho {
        ConDeclChirho::OrdinaryChirho { name_chirho, .. }
        | ConDeclChirho::RecordChirho { name_chirho, .. }
        | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho.text_chirho(),
    }
}

/// Get the number of fields for a constructor.
pub(super) fn con_field_count_chirho(con_chirho: &ConDeclChirho) -> usize {
    match con_chirho {
        ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => fields_chirho.len(),
        ConDeclChirho::RecordChirho { fields_chirho, .. } => fields_chirho.len(),
        ConDeclChirho::GadtChirho { ty_chirho, .. } => {
            // Count function arguments in the GADT type signature.
            extract_gadt_arg_count_chirho(ty_chirho)
        }
    }
}

/// Count function arguments in a GADT type signature.
/// Walks through FunChirho, ForallChirho, and QualChirho to count arrow arguments.
pub(super) fn extract_gadt_arg_count_chirho(ty_chirho: &TypeChirho) -> usize {
    match ty_chirho.unannotated_chirho() {
        TypeChirho::FunChirho { result_chirho, .. } => {
            1 + extract_gadt_arg_count_chirho(result_chirho)
        }
        TypeChirho::ForallChirho { body_chirho, .. } => extract_gadt_arg_count_chirho(body_chirho),
        TypeChirho::QualChirho { body_chirho, .. } => extract_gadt_arg_count_chirho(body_chirho),
        _ => 0, // Return type — not an argument
    }
}

/// Extract argument types from a GADT type signature (everything before the final return type).
pub(super) fn extract_gadt_args_chirho(ty_chirho: &TypeChirho) -> Vec<TypeChirho> {
    match ty_chirho.unannotated_chirho() {
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            let mut args_chirho = vec![(**arg_chirho).clone()];
            args_chirho.extend(extract_gadt_args_chirho(result_chirho));
            args_chirho
        }
        TypeChirho::ForallChirho { body_chirho, .. } => extract_gadt_args_chirho(body_chirho),
        TypeChirho::QualChirho { body_chirho, .. } => extract_gadt_args_chirho(body_chirho),
        _ => vec![], // Return type — not an argument
    }
}

/// Create field variable names like `a1`, `a2`, `b1`, `b2`.
pub(super) fn field_vars_chirho(prefix_chirho: &str, count_chirho: usize) -> Vec<String> {
    (1..=count_chirho)
        .map(|i_chirho| format!("{}{}", prefix_chirho, i_chirho))
        .collect()
}

/// Build a constructor pattern with field bindings.
pub(super) fn con_pat_chirho(
    con_name_str_chirho: &str,
    field_names_chirho: &[String],
) -> PatChirho {
    if field_names_chirho.is_empty() {
        PatChirho::ConChirho {
            con_chirho: var_name_chirho(con_name_str_chirho),
            args_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }
    } else {
        PatChirho::ConChirho {
            con_chirho: var_name_chirho(con_name_str_chirho),
            args_chirho: field_names_chirho
                .iter()
                .map(|n_chirho| PatChirho::VarChirho(var_name_chirho(n_chirho)))
                .collect(),
            span_chirho: gen_span_chirho(),
        }
    }
}

/// Build the instance head type: `T a b c` from type name and type vars.
pub(super) fn instance_type_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
) -> TypeChirho {
    if type_vars_chirho.is_empty() {
        TypeChirho::ConChirho(type_name_chirho.clone())
    } else {
        let mut result_chirho = TypeChirho::ConChirho(type_name_chirho.clone());
        for tv_chirho in type_vars_chirho {
            result_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(TypeChirho::VarChirho(tv_chirho.name_chirho.clone())),
                span_chirho: gen_span_chirho(),
            };
        }
        result_chirho
    }
}

/// Get the field types for a constructor.
pub(super) fn con_field_types_chirho(con_chirho: &ConDeclChirho) -> Vec<TypeChirho> {
    match con_chirho {
        ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => fields_chirho
            .iter()
            .map(|(_s_chirho, ty_chirho)| ty_chirho.clone())
            .collect(),
        ConDeclChirho::RecordChirho { fields_chirho, .. } => fields_chirho
            .iter()
            .map(|f_chirho| f_chirho.ty_chirho.clone())
            .collect(),
        ConDeclChirho::GadtChirho { ty_chirho, .. } => extract_gadt_args_chirho(ty_chirho),
    }
}

/// Check whether a type IS exactly the given type variable (by name, ignoring span).
pub(super) fn type_is_var_chirho(ty_chirho: &TypeChirho, var_chirho: &str) -> bool {
    matches!(ty_chirho, TypeChirho::VarChirho(n_chirho) if n_chirho.text_chirho() == var_chirho)
}

/// Check whether a type mentions a given type variable name.
pub(super) fn type_mentions_var_chirho(ty_chirho: &TypeChirho, var_chirho: &str) -> bool {
    match ty_chirho {
        TypeChirho::VarChirho(n_chirho) => n_chirho.text_chirho() == var_chirho,
        TypeChirho::ConChirho(_) => false,
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            type_mentions_var_chirho(fun_chirho, var_chirho)
                || type_mentions_var_chirho(arg_chirho, var_chirho)
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            type_mentions_var_chirho(arg_chirho, var_chirho)
                || type_mentions_var_chirho(result_chirho, var_chirho)
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => elements_chirho
            .iter()
            .any(|e_chirho| type_mentions_var_chirho(e_chirho, var_chirho)),
        TypeChirho::ListChirho { element_chirho, .. } => {
            type_mentions_var_chirho(element_chirho, var_chirho)
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            type_mentions_var_chirho(inner_chirho, var_chirho)
        }
        _ => false,
    }
}
