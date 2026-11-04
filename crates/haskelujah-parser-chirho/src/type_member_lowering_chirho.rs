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
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

/// Partition lowered class-body declarations into value methods and type members.
///
/// Workflow: `spec-chirho/workflows-chirho/compiler-pipeline-chirho/
/// type-scope-resolution-chirho.md`.
pub(crate) fn partition_class_members_chirho(
    where_decls_chirho: Vec<DeclChirho>,
    default_impls_chirho: &HashMap<String, Vec<MatchArmChirho>>,
    default_sigs_chirho: &HashMap<String, String>,
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
                        data_family_chirho: false,
                        type_vars_chirho: Vec::new(),
                        result_kind_chirho: Some(ty_chirho),
                        default_rhs_chirho: None,
                        default_params_chirho: Vec::new(),
                        default_binders_chirho: None,
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
                data_family_chirho,
                result_kind_chirho,
                mut equations_chirho,
                span_chirho,
                ..
            } => {
                let default_equation_chirho = (equations_chirho.len() == 1)
                    .then(|| equations_chirho.pop().expect("one equation"));
                let default_params_chirho = default_equation_chirho
                    .as_ref()
                    .map(|equation_chirho| {
                        equation_chirho
                            .lhs_types_chirho
                            .iter()
                            .map(|lhs_ty_chirho| match lhs_ty_chirho {
                                TypeChirho::VarChirho(name_chirho) => Some(name_chirho.clone()),
                                _ => None,
                            })
                            .collect::<Option<Vec<NameChirho>>>()
                            .unwrap_or_default()
                    })
                    .unwrap_or_default();
                let default_rhs_chirho =
                    default_equation_chirho.map(|equation_chirho| equation_chirho.rhs_chirho);
                // A default owns the complete scoped CST binder account,
                // including invisible kind-only names. It is not a second
                // canonical header when a bodiless declaration accompanies it.
                let default_binders_chirho = default_rhs_chirho
                    .as_ref()
                    .map(|_| type_vars_chirho.clone());
                associated_types_chirho.push(AssocTypeFamilyChirho {
                    name_chirho,
                    data_family_chirho,
                    type_vars_chirho,
                    result_kind_chirho,
                    default_rhs_chirho,
                    default_params_chirho,
                    default_binders_chirho,
                    span_chirho,
                });
            }
            DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                rhs_chirho,
                span_chirho,
            } => {
                // A bodiless declaration's right side is lowering's placeholder,
                // a variable with an empty name. Every written default is kept,
                // even one naming another class parameter (row 686, T5481); its
                // own binders, the kind variables included, scope its right side.
                let bodiless_chirho = matches!(
                    &rhs_chirho,
                    TypeChirho::VarChirho(placeholder_chirho)
                        if placeholder_chirho.text_chirho().is_empty()
                );
                let default_binders_chirho = (!bodiless_chirho).then(|| type_vars_chirho.clone());
                let default_rhs_chirho = (!bodiless_chirho).then_some(rhs_chirho);
                associated_types_chirho.push(AssocTypeFamilyChirho {
                    name_chirho,
                    data_family_chirho: false,
                    type_vars_chirho,
                    result_kind_chirho: None,
                    default_rhs_chirho,
                    default_params_chirho: Vec::new(),
                    default_binders_chirho,
                    span_chirho,
                });
            }
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                kind_sig_chirho,
                span_chirho,
                ..
            } => {
                associated_types_chirho.push(AssocTypeFamilyChirho {
                    name_chirho,
                    data_family_chirho: true,
                    type_vars_chirho,
                    result_kind_chirho: kind_sig_chirho.and_then(|kind_chirho| match kind_chirho {
                        haskelujah_ast_chirho::decl_chirho::DataKindSigChirho::ResultChirho(result_chirho) => Some(result_chirho),
                        haskelujah_ast_chirho::decl_chirho::DataKindSigChirho::StandaloneChirho { result_chirho, .. } => result_chirho,
                    }),
                    default_rhs_chirho: None,
                    default_params_chirho: Vec::new(),
                    default_binders_chirho: None,
                    span_chirho,
                });
            }
            _ => {}
        }
    }

    (methods_chirho, associated_types_chirho)
}
