// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Reification — translating compiler state to TH `Info`
//!
//! When a TH splice calls `reify someName`, the compiler must look up
//! `someName` in its internal state and return a `ThInfoChirho` value
//! describing the name's declaration, type, and other metadata.
//!
//! This module provides the translation from haskelujah's AST types
//! (`DeclChirho`, `TypeChirho`, etc.) to TH types (`ThDecChirho`, `ThTypeChirho`, etc.).

use haskelujah_ast_chirho::decl_chirho::{
    ClassMethodChirho, ConDeclChirho, DeclChirho, FieldDeclChirho, TyVarChirho,
};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

use crate::th_ast_chirho::*;

/// Convert a haskelujah AST type to a TH type.
pub fn ast_type_to_th_chirho(ty_chirho: &TypeChirho) -> ThTypeChirho {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => {
            ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(name_chirho.text_chirho()))
        }
        TypeChirho::VarChirho(name_chirho) => {
            ThTypeChirho::VarTChirho(ThNameChirho::mk_name_chirho(name_chirho.text_chirho()))
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => ThTypeChirho::AppTChirho(
            Box::new(ast_type_to_th_chirho(fun_chirho)),
            Box::new(ast_type_to_th_chirho(arg_chirho)),
        ),
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => ThTypeChirho::AppTChirho(
            Box::new(ThTypeChirho::AppTChirho(
                Box::new(ThTypeChirho::ArrowTChirho),
                Box::new(ast_type_to_th_chirho(arg_chirho)),
            )),
            Box::new(ast_type_to_th_chirho(result_chirho)),
        ),
        TypeChirho::ListChirho { element_chirho, .. } => ThTypeChirho::AppTChirho(
            Box::new(ThTypeChirho::ListTChirho),
            Box::new(ast_type_to_th_chirho(element_chirho)),
        ),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => {
            let arity_chirho = elements_chirho.len() as i32;
            let mut result_chirho = ThTypeChirho::TupleTChirho(arity_chirho);
            for elt_chirho in elements_chirho {
                result_chirho = ThTypeChirho::AppTChirho(
                    Box::new(result_chirho),
                    Box::new(ast_type_to_th_chirho(elt_chirho)),
                );
            }
            result_chirho
        }
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            let bndrs_chirho: Vec<ThTyVarBndrChirho> = vars_chirho
                .iter()
                .map(|v_chirho| ast_tyvar_to_th_chirho(v_chirho))
                .collect();
            ThTypeChirho::ForallTChirho(
                bndrs_chirho,
                vec![],
                Box::new(ast_type_to_th_chirho(body_chirho)),
            )
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            let cxt_chirho: Vec<ThTypeChirho> = context_chirho
                .iter()
                .map(|c_chirho| {
                    let mut result_chirho = ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
                        c_chirho.class_chirho.text_chirho(),
                    ));
                    for arg_chirho in &c_chirho.args_chirho {
                        result_chirho = ThTypeChirho::AppTChirho(
                            Box::new(result_chirho),
                            Box::new(ast_type_to_th_chirho(arg_chirho)),
                        );
                    }
                    result_chirho
                })
                .collect();
            ThTypeChirho::ForallTChirho(
                vec![],
                cxt_chirho,
                Box::new(ast_type_to_th_chirho(body_chirho)),
            )
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => ast_type_to_th_chirho(inner_chirho),
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            ThTypeChirho::PromotedTChirho(ThNameChirho::mk_name_chirho(name_chirho.text_chirho()))
        }
        TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => ThTypeChirho::PromotedListTChirho(
            elements_chirho
                .iter()
                .map(|e_chirho| ast_type_to_th_chirho(e_chirho))
                .collect(),
        ),
        // PartialTypeSignatures: `_` wildcard becomes a fresh anonymous TH type
        // variable named `_wildcard_chirho` at the TH level.
        TypeChirho::WildcardChirho { .. } => {
            ThTypeChirho::VarTChirho(ThNameChirho::mk_name_chirho("_wildcard_chirho"))
        }
        // Type-level literal (DataKinds): treat as an opaque type constructor.
        TypeChirho::LitChirho { value_chirho, .. } => {
            ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(value_chirho))
        }
    }
}

/// Convert a haskelujah AST type variable to a TH type variable binder.
pub fn ast_tyvar_to_th_chirho(tv_chirho: &TyVarChirho) -> ThTyVarBndrChirho {
    match &tv_chirho.kind_annotation_chirho {
        None => ThTyVarBndrChirho::PlainTVChirho(ThNameChirho::mk_name_chirho(
            tv_chirho.name_chirho.text_chirho(),
        )),
        Some(_kind_chirho) => {
            // Kind annotations require mapping AstKindChirho to ThTypeChirho
            // For now, treat as plain since kind annotations are rare in user code
            ThTyVarBndrChirho::PlainTVChirho(ThNameChirho::mk_name_chirho(
                tv_chirho.name_chirho.text_chirho(),
            ))
        }
    }
}

/// Convert a haskelujah constructor declaration to a TH constructor.
pub fn ast_con_to_th_chirho(con_chirho: &ConDeclChirho) -> ThConChirho {
    match con_chirho {
        ConDeclChirho::OrdinaryChirho {
            name_chirho,
            fields_chirho,
            ..
        } => {
            let bang_types_chirho: Vec<ThBangTypeChirho> = fields_chirho
                .iter()
                .map(|(_strictness_chirho, ty_chirho)| ThBangTypeChirho {
                    bang_chirho: ThBangChirho::default_bang_chirho(),
                    ty_chirho: ast_type_to_th_chirho(ty_chirho),
                })
                .collect();
            ThConChirho::NormalCChirho(
                ThNameChirho::mk_name_chirho(name_chirho.text_chirho()),
                bang_types_chirho,
            )
        }
        ConDeclChirho::RecordChirho {
            name_chirho,
            fields_chirho,
            ..
        } => {
            let var_bang_types_chirho: Vec<ThVarBangTypeChirho> = fields_chirho
                .iter()
                .flat_map(|fd_chirho: &FieldDeclChirho| {
                    fd_chirho
                        .names_chirho
                        .iter()
                        .map(move |fn_chirho| ThVarBangTypeChirho {
                            name_chirho: ThNameChirho::mk_name_chirho(fn_chirho.text_chirho()),
                            bang_chirho: ThBangChirho::default_bang_chirho(),
                            ty_chirho: ast_type_to_th_chirho(&fd_chirho.ty_chirho),
                        })
                })
                .collect();
            ThConChirho::RecCChirho(
                ThNameChirho::mk_name_chirho(name_chirho.text_chirho()),
                var_bang_types_chirho,
            )
        }
        ConDeclChirho::GadtChirho {
            name_chirho,
            ty_chirho,
            ..
        } => {
            // Extract argument types and return type from the GADT type signature.
            let mut arg_types_chirho: Vec<TypeChirho> = Vec::new();
            extract_gadt_fun_args_chirho(ty_chirho, &mut arg_types_chirho);
            let ret_ty_chirho = extract_gadt_return_type_chirho(ty_chirho);
            let bang_types_chirho: Vec<ThBangTypeChirho> = arg_types_chirho
                .iter()
                .map(|arg_chirho| ThBangTypeChirho {
                    bang_chirho: ThBangChirho::default_bang_chirho(),
                    ty_chirho: ast_type_to_th_chirho(arg_chirho),
                })
                .collect();
            ThConChirho::GadtCChirho(
                vec![ThNameChirho::mk_name_chirho(name_chirho.text_chirho())],
                bang_types_chirho,
                Box::new(ast_type_to_th_chirho(&ret_ty_chirho)),
            )
        }
    }
}

/// Extract argument types from a GADT function type chain, peeling forall/qual.
fn extract_gadt_fun_args_chirho(ty_chirho: &TypeChirho, out_chirho: &mut Vec<TypeChirho>) {
    match ty_chirho {
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            out_chirho.push((**arg_chirho).clone());
            extract_gadt_fun_args_chirho(result_chirho, out_chirho);
        }
        TypeChirho::ForallChirho { body_chirho, .. } => {
            extract_gadt_fun_args_chirho(body_chirho, out_chirho);
        }
        TypeChirho::QualChirho { body_chirho, .. } => {
            extract_gadt_fun_args_chirho(body_chirho, out_chirho);
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            extract_gadt_fun_args_chirho(inner_chirho, out_chirho);
        }
        _ => {} // Return type — not an argument
    }
}

/// Extract the final return type from a GADT function type chain.
fn extract_gadt_return_type_chirho(ty_chirho: &TypeChirho) -> TypeChirho {
    match ty_chirho {
        TypeChirho::FunChirho { result_chirho, .. } => {
            extract_gadt_return_type_chirho(result_chirho)
        }
        TypeChirho::ForallChirho { body_chirho, .. } => {
            extract_gadt_return_type_chirho(body_chirho)
        }
        TypeChirho::QualChirho { body_chirho, .. } => extract_gadt_return_type_chirho(body_chirho),
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            extract_gadt_return_type_chirho(inner_chirho)
        }
        other_chirho => other_chirho.clone(),
    }
}

/// Reify a top-level declaration into TH `Info`.
///
/// This is the main entry point for `reify` when the target is a locally-defined name.
pub fn reify_decl_chirho(decl_chirho: &DeclChirho) -> Option<ThInfoChirho> {
    match decl_chirho {
        DeclChirho::DataDeclChirho {
            name_chirho,
            type_vars_chirho,
            constructors_chirho,
            deriving_chirho,
            ..
        } => {
            let th_name_chirho = ThNameChirho::mk_name_chirho(name_chirho.text_chirho());
            let th_tyvars_chirho: Vec<ThTyVarBndrChirho> = type_vars_chirho
                .iter()
                .map(|tv_chirho| ast_tyvar_to_th_chirho(tv_chirho))
                .collect();
            let th_cons_chirho: Vec<ThConChirho> = constructors_chirho
                .iter()
                .map(|c_chirho| ast_con_to_th_chirho(c_chirho))
                .collect();
            let th_deriving_chirho: Vec<ThDerivClauseChirho> = if deriving_chirho.is_empty() {
                vec![]
            } else {
                vec![ThDerivClauseChirho {
                    strategy_chirho: None,
                    classes_chirho: deriving_chirho
                        .iter()
                        .map(|d_chirho| {
                            ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
                                d_chirho.text_chirho(),
                            ))
                        })
                        .collect(),
                }]
            };

            Some(ThInfoChirho::TyConIChirho(ThDecChirho::DataDChirho(
                vec![],
                th_name_chirho,
                th_tyvars_chirho,
                None,
                th_cons_chirho,
                th_deriving_chirho,
            )))
        }
        DeclChirho::NewtypeDeclChirho {
            name_chirho,
            type_vars_chirho,
            constructor_chirho,
            deriving_chirho,
            ..
        } => {
            let th_name_chirho = ThNameChirho::mk_name_chirho(name_chirho.text_chirho());
            let th_tyvars_chirho: Vec<ThTyVarBndrChirho> = type_vars_chirho
                .iter()
                .map(|tv_chirho| ast_tyvar_to_th_chirho(tv_chirho))
                .collect();
            let th_con_chirho = ast_con_to_th_chirho(constructor_chirho);
            let th_deriving_chirho: Vec<ThDerivClauseChirho> = if deriving_chirho.is_empty() {
                vec![]
            } else {
                vec![ThDerivClauseChirho {
                    strategy_chirho: None,
                    classes_chirho: deriving_chirho
                        .iter()
                        .map(|d_chirho| {
                            ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
                                d_chirho.text_chirho(),
                            ))
                        })
                        .collect(),
                }]
            };

            Some(ThInfoChirho::TyConIChirho(ThDecChirho::NewtypeDChirho(
                vec![],
                th_name_chirho,
                th_tyvars_chirho,
                None,
                th_con_chirho,
                th_deriving_chirho,
            )))
        }
        DeclChirho::ClassDeclChirho {
            name_chirho,
            type_vars_chirho,
            methods_chirho,
            context_chirho,
            fundeps_chirho,
            ..
        } => {
            let th_name_chirho = ThNameChirho::mk_name_chirho(name_chirho.text_chirho());
            let th_tyvars_chirho: Vec<ThTyVarBndrChirho> = type_vars_chirho
                .iter()
                .map(|tv_chirho| ast_tyvar_to_th_chirho(tv_chirho))
                .collect();
            let th_cxt_chirho: ThCxtChirho = context_chirho
                .iter()
                .map(|c_chirho| {
                    let mut t_chirho = ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
                        c_chirho.class_chirho.text_chirho(),
                    ));
                    for arg_chirho in &c_chirho.args_chirho {
                        t_chirho = ThTypeChirho::AppTChirho(
                            Box::new(t_chirho),
                            Box::new(ast_type_to_th_chirho(arg_chirho)),
                        );
                    }
                    t_chirho
                })
                .collect();
            let th_fundeps_chirho: Vec<ThFunDepChirho> = fundeps_chirho
                .iter()
                .map(|(from_chirho, to_chirho)| ThFunDepChirho {
                    from_chirho: from_chirho
                        .iter()
                        .map(|s_chirho| ThNameChirho::mk_name_chirho(s_chirho))
                        .collect(),
                    to_chirho: to_chirho
                        .iter()
                        .map(|s_chirho| ThNameChirho::mk_name_chirho(s_chirho))
                        .collect(),
                })
                .collect();
            let th_methods_chirho: Vec<ThDecChirho> = methods_chirho
                .iter()
                .map(|m_chirho: &ClassMethodChirho| {
                    ThDecChirho::SigDChirho(
                        ThNameChirho::mk_name_chirho(m_chirho.name_chirho.text_chirho()),
                        Box::new(ast_type_to_th_chirho(&m_chirho.ty_chirho)),
                    )
                })
                .collect();

            Some(ThInfoChirho::ClassIChirho(
                ThDecChirho::ClassDChirho(
                    th_cxt_chirho,
                    th_name_chirho,
                    th_tyvars_chirho,
                    th_fundeps_chirho,
                    th_methods_chirho,
                ),
                vec![], // Known instances would be populated from the instance environment
            ))
        }
        DeclChirho::TypeAliasDeclChirho {
            name_chirho,
            type_vars_chirho,
            rhs_chirho,
            ..
        } => {
            let th_name_chirho = ThNameChirho::mk_name_chirho(name_chirho.text_chirho());
            let th_tyvars_chirho: Vec<ThTyVarBndrChirho> = type_vars_chirho
                .iter()
                .map(|tv_chirho| ast_tyvar_to_th_chirho(tv_chirho))
                .collect();
            Some(ThInfoChirho::TyConIChirho(ThDecChirho::TySynDChirho(
                th_name_chirho,
                th_tyvars_chirho,
                Box::new(ast_type_to_th_chirho(rhs_chirho)),
            )))
        }
        _ => None, // Other declaration types not yet reifiable
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    use haskelujah_span_chirho::SpanChirho;

    fn mk_name_chirho(s_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            s_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    #[test]
    fn reify_data_decl_chirho() {
        let decl_chirho = DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Person"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::RecordChirho {
                name_chirho: mk_name_chirho("Person"),
                fields_chirho: vec![
                    FieldDeclChirho {
                        names_chirho: vec![mk_name_chirho("_name")],
                        ty_chirho: TypeChirho::ConChirho(mk_name_chirho("String")),
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    FieldDeclChirho {
                        names_chirho: vec![mk_name_chirho("_age")],
                        ty_chirho: TypeChirho::ConChirho(mk_name_chirho("Int")),
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![mk_name_chirho("Show")],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let info_chirho = reify_decl_chirho(&decl_chirho).unwrap();
        match info_chirho {
            ThInfoChirho::TyConIChirho(ThDecChirho::DataDChirho(
                _cxt,
                name_chirho,
                _tyvars,
                _kind,
                cons_chirho,
                derivs_chirho,
            )) => {
                assert_eq!(name_chirho.occ_chirho, "Person");
                assert_eq!(cons_chirho.len(), 1);
                match &cons_chirho[0] {
                    ThConChirho::RecCChirho(cn_chirho, fields_chirho) => {
                        assert_eq!(cn_chirho.occ_chirho, "Person");
                        assert_eq!(fields_chirho.len(), 2);
                        assert_eq!(fields_chirho[0].name_chirho.occ_chirho, "_name");
                        assert_eq!(fields_chirho[1].name_chirho.occ_chirho, "_age");
                    }
                    _ => panic!("expected RecC"),
                }
                assert_eq!(derivs_chirho.len(), 1);
            }
            _ => panic!("expected TyConI(DataD)"),
        }
    }

    #[test]
    fn reify_type_alias_chirho() {
        let decl_chirho = DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("MyString"),
            type_vars_chirho: vec![],
            rhs_chirho: TypeChirho::ConChirho(mk_name_chirho("String")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let info_chirho = reify_decl_chirho(&decl_chirho).unwrap();
        match info_chirho {
            ThInfoChirho::TyConIChirho(ThDecChirho::TySynDChirho(name_chirho, _, rhs_chirho)) => {
                assert_eq!(name_chirho.occ_chirho, "MyString");
                assert!(
                    matches!(*rhs_chirho, ThTypeChirho::ConTChirho(n) if n.occ_chirho == "String")
                );
            }
            _ => panic!("expected TyConI(TySynD)"),
        }
    }

    #[test]
    fn ast_type_conversion_chirho() {
        let ty_chirho = TypeChirho::FunChirho {
            arg_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Int"))),
            mult_chirho: None,
            result_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Bool"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let th_ty_chirho = ast_type_to_th_chirho(&ty_chirho);
        // Should be AppT (AppT ArrowT Int) Bool
        match th_ty_chirho {
            ThTypeChirho::AppTChirho(inner_chirho, _bool_chirho) => match *inner_chirho {
                ThTypeChirho::AppTChirho(arrow_chirho, _int_chirho) => {
                    assert!(matches!(*arrow_chirho, ThTypeChirho::ArrowTChirho));
                }
                _ => panic!("expected AppT ArrowT Int"),
            },
            _ => panic!("expected AppT"),
        }
    }
}
