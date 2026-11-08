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
    ClassMethodChirho, ConDeclChirho, DeclChirho, DerivedClassChirho, FieldDeclChirho,
    GadtConPrefixChirho, StrictnessChirho, TyVarChirho,
};
use haskelujah_ast_chirho::name_chirho::{
    CompilerTypeConstructorChirho, TypeConstructorIdentityChirho,
};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, MultiplicityChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;

use crate::th_ast_chirho::*;

/// Source validity, resource limits and TH's representational limit stay distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThReificationErrorChirho {
    ConstructorSignatureChirho(haskelujah_ast_chirho::decl_chirho::ConstructorSignatureErrorChirho),
    UnsupportedVisibleForallChirho { span_chirho: SpanChirho },
}

impl ThReificationErrorChirho {
    pub fn span_chirho(self) -> SpanChirho {
        match self {
            Self::ConstructorSignatureChirho(error_chirho) => error_chirho.span_chirho(),
            Self::UnsupportedVisibleForallChirho { span_chirho } => span_chirho,
        }
    }
    pub fn ghc_code_chirho(self) -> Option<&'static str> {
        match self {
            Self::ConstructorSignatureChirho(error_chirho) => error_chirho.ghc_code_chirho(),
            Self::UnsupportedVisibleForallChirho { .. } => Some("GHC-75721"),
        }
    }
}
impl From<haskelujah_ast_chirho::decl_chirho::ConstructorSignatureErrorChirho>
    for ThReificationErrorChirho
{
    fn from(
        error_chirho: haskelujah_ast_chirho::decl_chirho::ConstructorSignatureErrorChirho,
    ) -> Self {
        Self::ConstructorSignatureChirho(error_chirho)
    }
}
impl From<haskelujah_ast_chirho::decl_chirho::GadtProjectionErrorChirho>
    for ThReificationErrorChirho
{
    fn from(error_chirho: haskelujah_ast_chirho::decl_chirho::GadtProjectionErrorChirho) -> Self {
        Self::ConstructorSignatureChirho(error_chirho.into())
    }
}
impl std::fmt::Display for ThReificationErrorChirho {
    fn fmt(&self, formatter_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConstructorSignatureChirho(error_chirho) => {
                std::fmt::Display::fmt(error_chirho, formatter_chirho)
            }
            Self::UnsupportedVisibleForallChirho { .. } => formatter_chirho
                .write_str("Cannot represent visible forall in Template Haskell (GHC-75721)"),
        }
    }
}
impl std::error::Error for ThReificationErrorChirho {}
/// A declaration's derived classes as TH deriving clauses: consecutive classes with the
/// same strategy share one clause, as written (row 694).
fn th_deriv_clauses_chirho(deriving_chirho: &[DerivedClassChirho]) -> Vec<ThDerivClauseChirho> {
    let mut clauses_chirho: Vec<ThDerivClauseChirho> = Vec::new();
    for derived_chirho in deriving_chirho {
        let strategy_chirho = derived_chirho
            .strategy_chirho
            .as_ref()
            .map(|strategy_chirho| {
                if strategy_chirho.is_newtype_chirho() {
                    ThDerivStrategyChirho::NewtypeStrategyChirho
                } else if strategy_chirho.is_anyclass_chirho() {
                    ThDerivStrategyChirho::AnyclassStrategyChirho
                } else {
                    ThDerivStrategyChirho::StockStrategyChirho
                }
            });
        let class_chirho = ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
            derived_chirho.class_text_chirho(),
        ));
        let joins_last_chirho = clauses_chirho
            .last()
            .is_some_and(|clause_chirho| clause_chirho.strategy_chirho == strategy_chirho);
        match clauses_chirho.last_mut() {
            Some(clause_chirho) if joins_last_chirho => {
                clause_chirho.classes_chirho.push(class_chirho)
            }
            _ => clauses_chirho.push(ThDerivClauseChirho {
                strategy_chirho,
                classes_chirho: vec![class_chirho],
            }),
        }
    }
    clauses_chirho
}

fn ast_constraint_to_th_chirho(constraint_chirho: &ConstraintChirho) -> ThTypeChirho {
    match constraint_chirho {
        ConstraintChirho::ClassChirho {
            class_chirho,
            kind_args_chirho,
            args_chirho,
            ..
        } => {
            let mut result_chirho =
                ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(class_chirho.text_chirho()));
            for kind_chirho in kind_args_chirho {
                result_chirho = ThTypeChirho::AppKindTChirho(
                    Box::new(result_chirho),
                    Box::new(ast_type_to_th_chirho(kind_chirho)),
                );
            }
            for arg_chirho in args_chirho {
                result_chirho = ThTypeChirho::AppTChirho(
                    Box::new(result_chirho),
                    Box::new(ast_type_to_th_chirho(arg_chirho)),
                );
            }
            result_chirho
        }
        ConstraintChirho::KindAnnotationChirho {
            constraint_chirho,
            kind_chirho,
            ..
        } => ThTypeChirho::SigTChirho(
            Box::new(ast_constraint_to_th_chirho(constraint_chirho)),
            Box::new(ast_type_to_th_chirho(kind_chirho)),
        ),
        ConstraintChirho::QuantifiedChirho {
            vars_chirho,
            context_chirho,
            body_chirho,
            ..
        } => ThTypeChirho::ForallTChirho(
            vars_chirho.iter().map(ast_tyvar_to_th_chirho).collect(),
            context_chirho
                .iter()
                .map(ast_constraint_to_th_chirho)
                .collect(),
            Box::new(ast_constraint_to_th_chirho(body_chirho)),
        ),
    }
}

/// Convert a haskelujah AST type to a TH type.
pub fn ast_type_to_th_chirho(ty_chirho: &TypeChirho) -> ThTypeChirho {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => {
            match name_chirho
                .type_constructor_chirho()
                .map(|declaration_chirho| &declaration_chirho.identity_chirho)
            {
                Some(TypeConstructorIdentityChirho::CompilerChirho(
                    CompilerTypeConstructorChirho::TypeUniverseChirho,
                )) => ThTypeChirho::StarTChirho,
                Some(TypeConstructorIdentityChirho::CompilerChirho(
                    CompilerTypeConstructorChirho::ConstraintUniverseChirho,
                )) => ThTypeChirho::ConstraintTChirho,
                _ => ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
                    name_chirho.text_chirho(),
                )),
            }
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
        TypeChirho::KindApplicationChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => ThTypeChirho::AppKindTChirho(
            Box::new(ast_type_to_th_chirho(fun_chirho)),
            Box::new(ast_type_to_th_chirho(arg_chirho)),
        ),
        TypeChirho::KindAnnotationChirho {
            ty_chirho,
            kind_chirho,
            ..
        } => ThTypeChirho::SigTChirho(
            Box::new(ast_type_to_th_chirho(ty_chirho)),
            Box::new(ast_type_to_th_chirho(kind_chirho)),
        ),
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            mult_chirho,
            ..
        } => {
            let multiplicity_ty_chirho = match mult_chirho {
                None | Some(MultiplicityChirho::ManyChirho) => None,
                Some(MultiplicityChirho::OneChirho) => Some(ThTypeChirho::PromotedTChirho(
                    ThNameChirho::mk_name_chirho("One"),
                )),
                Some(MultiplicityChirho::MultVarChirho(name_chirho)) => {
                    Some(ThTypeChirho::VarTChirho(ThNameChirho::mk_name_chirho(
                        name_chirho.text_chirho(),
                    )))
                }
            };
            let arrow_chirho = multiplicity_ty_chirho.map_or(
                ThTypeChirho::ArrowTChirho,
                |multiplicity_ty_chirho| {
                    ThTypeChirho::AppTChirho(
                        Box::new(ThTypeChirho::MulArrowTChirho),
                        Box::new(multiplicity_ty_chirho),
                    )
                },
            );
            ThTypeChirho::AppTChirho(
                Box::new(ThTypeChirho::AppTChirho(
                    Box::new(arrow_chirho),
                    Box::new(ast_type_to_th_chirho(arg_chirho)),
                )),
                Box::new(ast_type_to_th_chirho(result_chirho)),
            )
        }
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
            let (context_chirho, body_chirho) = match body_chirho.as_ref() {
                TypeChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                    ..
                } => (
                    context_chirho
                        .iter()
                        .map(ast_constraint_to_th_chirho)
                        .collect(),
                    body_chirho.as_ref(),
                ),
                other_chirho => (vec![], other_chirho),
            };
            ThTypeChirho::ForallTChirho(
                bndrs_chirho,
                context_chirho,
                Box::new(ast_type_to_th_chirho(body_chirho)),
            )
        }
        TypeChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => ThTypeChirho::ForallVisTChirho(
            vars_chirho.iter().map(ast_tyvar_to_th_chirho).collect(),
            Box::new(ast_type_to_th_chirho(body_chirho)),
        ),
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => ThTypeChirho::ForallTChirho(
            vec![],
            context_chirho
                .iter()
                .map(ast_constraint_to_th_chirho)
                .collect(),
            Box::new(ast_type_to_th_chirho(body_chirho)),
        ),
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
        TypeChirho::WildcardChirho { .. } => ThTypeChirho::WildCardTChirho,
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
        Some(kind_chirho) => ThTyVarBndrChirho::KindedTVChirho(
            ThNameChirho::mk_name_chirho(tv_chirho.name_chirho.text_chirho()),
            Box::new(ast_type_to_th_chirho(kind_chirho)),
        ),
    }
}

/// Convert a haskelujah constructor declaration to a TH constructor.
pub fn ast_con_to_th_chirho(
    con_chirho: &ConDeclChirho,
    budget_chirho: &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho,
) -> Result<ThConChirho, ThReificationErrorChirho> {
    haskelujah_ast_chirho::decl_chirho::check_constructor_signature_chirho(con_chirho)?;
    haskelujah_ast_chirho::decl_chirho::check_constructor_projection_chirho(
        con_chirho,
        budget_chirho,
        3,
    )?;
    check_constructor_th_representation_chirho(con_chirho)?;
    Ok(project_con_to_th_chirho(con_chirho))
}

fn check_constructor_th_representation_chirho(
    con_chirho: &ConDeclChirho,
) -> Result<(), ThReificationErrorChirho> {
    if let ConDeclChirho::GadtChirho {
        signature_chirho, ..
    } = con_chirho
    {
        for prefix_chirho in &signature_chirho.prefix_chirho {
            if let GadtConPrefixChirho::ForallChirho {
                required_chirho: true,
                span_chirho,
                ..
            } = prefix_chirho
            {
                return Err(ThReificationErrorChirho::UnsupportedVisibleForallChirho {
                    span_chirho: *span_chirho,
                });
            }
        }
    }
    Ok(())
}

/// Reached only after one public constructor or declaration preflight.
fn project_con_to_th_chirho(con_chirho: &ConDeclChirho) -> ThConChirho {
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
            signature_chirho,
            ..
        } => {
            let ret_ty_chirho = Box::new(ast_type_to_th_chirho(&signature_chirho.result_ty_chirho));
            let names_chirho = vec![ThNameChirho::mk_name_chirho(name_chirho.text_chirho())];
            // GHC's TyConI BangType representation intentionally contains no
            // field-arrow multiplicity. DataConI's complete type uses the
            // shared type converter above, which retains MulArrowT.
            let mut con_chirho =
                if let Some(fields_chirho) = signature_chirho.record_fields_chirho() {
                    ThConChirho::RecGadtCChirho(
                        names_chirho,
                        fields_chirho
                            .iter()
                            .flat_map(|field_chirho| {
                                field_chirho.names_chirho.iter().map(move |name_chirho| {
                                    ThVarBangTypeChirho {
                                        name_chirho: ThNameChirho::mk_name_chirho(
                                            name_chirho.text_chirho(),
                                        ),
                                        bang_chirho: gadt_bang_to_th_chirho(
                                            field_chirho.strictness_chirho,
                                        ),
                                        ty_chirho: ast_type_to_th_chirho(&field_chirho.ty_chirho),
                                    }
                                })
                            })
                            .collect(),
                        ret_ty_chirho,
                    )
                } else {
                    ThConChirho::GadtCChirho(
                        names_chirho,
                        signature_chirho
                            .positional_fields_chirho()
                            .map(|field_chirho| ThBangTypeChirho {
                                bang_chirho: gadt_bang_to_th_chirho(field_chirho.strictness_chirho),
                                ty_chirho: ast_type_to_th_chirho(field_chirho.ty_chirho),
                            })
                            .collect(),
                        ret_ty_chirho,
                    )
                };
            for prefix_chirho in signature_chirho.prefix_chirho.iter().rev() {
                con_chirho = match prefix_chirho {
                    GadtConPrefixChirho::ForallChirho { vars_chirho, .. } => {
                        ThConChirho::ForallCChirho(
                            vars_chirho.iter().map(ast_tyvar_to_th_chirho).collect(),
                            vec![],
                            Box::new(con_chirho),
                        )
                    }
                    GadtConPrefixChirho::ContextChirho {
                        constraints_chirho, ..
                    } => ThConChirho::ForallCChirho(
                        vec![],
                        constraints_chirho
                            .iter()
                            .map(ast_constraint_to_th_chirho)
                            .collect(),
                        Box::new(con_chirho),
                    ),
                };
            }
            con_chirho
        }
    }
}

fn gadt_bang_to_th_chirho(strictness_chirho: StrictnessChirho) -> ThBangChirho {
    ThBangChirho {
        src_unpackedness_chirho: if strictness_chirho == StrictnessChirho::UnpackChirho {
            ThSourceUnpackednessChirho::SourceUnpackChirho
        } else {
            ThSourceUnpackednessChirho::NoSourceUnpackednessChirho
        },
        src_strictness_chirho: if strictness_chirho == StrictnessChirho::LazyChirho {
            ThSourceStrictnessChirho::NoSourceStrictnessChirho
        } else if strictness_chirho == StrictnessChirho::ExplicitLazyChirho {
            ThSourceStrictnessChirho::SourceLazyChirho
        } else {
            ThSourceStrictnessChirho::SourceStrictChirho
        },
    }
}

/// Reify a top-level declaration into TH `Info`.
///
/// This is the main entry point for `reify` when the target is a locally-defined name.
pub fn reify_decl_chirho(
    decl_chirho: &DeclChirho,
    budget_chirho: &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho,
) -> Result<Option<ThInfoChirho>, ThReificationErrorChirho> {
    let constructors_chirho = match decl_chirho {
        DeclChirho::DataDeclChirho {
            constructors_chirho,
            ..
        } => constructors_chirho.as_slice(),
        DeclChirho::NewtypeDeclChirho {
            constructor_chirho, ..
        } => std::slice::from_ref(constructor_chirho.as_ref()),
        _ => &[],
    };
    for constructor_chirho in constructors_chirho {
        haskelujah_ast_chirho::decl_chirho::check_constructor_signature_chirho(constructor_chirho)?;
    }
    haskelujah_ast_chirho::decl_chirho::check_declaration_projection_chirho(
        decl_chirho,
        budget_chirho,
        3,
    )?;
    for constructor_chirho in constructors_chirho {
        check_constructor_th_representation_chirho(constructor_chirho)?;
    }
    Ok(match decl_chirho {
        DeclChirho::DataDeclChirho {
            name_chirho,
            type_vars_chirho,
            constructors_chirho,
            deriving_chirho,
            kind_sig_chirho,
            ..
        } => {
            let th_name_chirho = ThNameChirho::mk_name_chirho(name_chirho.text_chirho());
            let th_tyvars_chirho: Vec<ThTyVarBndrChirho> = type_vars_chirho
                .iter()
                .map(|tv_chirho| ast_tyvar_to_th_chirho(tv_chirho))
                .collect();
            let th_cons_chirho: Vec<ThConChirho> = constructors_chirho
                .iter()
                .map(project_con_to_th_chirho)
                .collect();
            let th_deriving_chirho = th_deriv_clauses_chirho(deriving_chirho);

            Some(ThInfoChirho::TyConIChirho(ThDecChirho::DataDChirho(
                vec![],
                th_name_chirho,
                th_tyvars_chirho,
                kind_sig_chirho
                    .as_ref()
                    .and_then(|kind_chirho| kind_chirho.result_chirho())
                    .map(|kind_chirho| Box::new(ast_type_to_th_chirho(kind_chirho))),
                th_cons_chirho,
                th_deriving_chirho,
            )))
        }
        DeclChirho::NewtypeDeclChirho {
            name_chirho,
            type_vars_chirho,
            constructor_chirho,
            deriving_chirho,
            kind_sig_chirho,
            ..
        } => {
            let th_name_chirho = ThNameChirho::mk_name_chirho(name_chirho.text_chirho());
            let th_tyvars_chirho: Vec<ThTyVarBndrChirho> = type_vars_chirho
                .iter()
                .map(|tv_chirho| ast_tyvar_to_th_chirho(tv_chirho))
                .collect();
            let th_con_chirho = project_con_to_th_chirho(constructor_chirho);
            let th_deriving_chirho = th_deriv_clauses_chirho(deriving_chirho);

            Some(ThInfoChirho::TyConIChirho(ThDecChirho::NewtypeDChirho(
                vec![],
                th_name_chirho,
                th_tyvars_chirho,
                kind_sig_chirho
                    .as_ref()
                    .and_then(|kind_chirho| kind_chirho.result_chirho())
                    .map(|kind_chirho| Box::new(ast_type_to_th_chirho(kind_chirho))),
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
                .map(ast_constraint_to_th_chirho)
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
    })
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
                        multiplicity_chirho: None,
                        names_chirho: vec![mk_name_chirho("_name")],
                        ty_chirho: TypeChirho::ConChirho(mk_name_chirho("String")),
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    FieldDeclChirho {
                        multiplicity_chirho: None,
                        names_chirho: vec![mk_name_chirho("_age")],
                        ty_chirho: TypeChirho::ConChirho(mk_name_chirho("Int")),
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
            deriving_chirho: vec![DerivedClassChirho::unspecified_chirho(mk_name_chirho(
                "Show",
            ))],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let info_chirho = reify_decl_chirho(
            &decl_chirho,
            &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho(),
        )
        .expect("small TH fixture fits projection budget")
        .expect("supported TH declaration");
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

        let info_chirho = reify_decl_chirho(
            &decl_chirho,
            &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho(),
        )
        .expect("small TH fixture fits projection budget")
        .expect("supported TH declaration");
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

    #[test]
    fn complete_constructor_types_keep_multiplicity_while_bang_types_project_it_chirho() {
        let arrow_chirho = |multiplicity_chirho| TypeChirho::FunChirho {
            arg_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Int"))),
            mult_chirho: multiplicity_chirho,
            result_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("BoxChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let mut projected_chirho = Vec::new();
        for multiplicity_chirho in [
            None,
            Some(MultiplicityChirho::OneChirho),
            Some(MultiplicityChirho::MultVarChirho(mk_name_chirho("mChirho"))),
        ] {
            let ty_chirho = arrow_chirho(multiplicity_chirho.clone());
            let ThTypeChirho::AppTChirho(argument_chirho, _) = ast_type_to_th_chirho(&ty_chirho)
            else {
                panic!("function result application")
            };
            let ThTypeChirho::AppTChirho(arrow_chirho, _) = *argument_chirho else {
                panic!("function argument application")
            };
            match multiplicity_chirho {
                None => assert_eq!(*arrow_chirho, ThTypeChirho::ArrowTChirho),
                Some(multiplicity_chirho) => {
                    let ThTypeChirho::AppTChirho(operator_chirho, value_chirho) = *arrow_chirho
                    else {
                        panic!("multiplicity application")
                    };
                    assert_eq!(*operator_chirho, ThTypeChirho::MulArrowTChirho);
                    match multiplicity_chirho {
                        MultiplicityChirho::OneChirho => assert!(
                            matches!(*value_chirho, ThTypeChirho::PromotedTChirho(name_chirho) if name_chirho.occ_chirho == "One")
                        ),
                        MultiplicityChirho::MultVarChirho(_) => assert!(
                            matches!(*value_chirho, ThTypeChirho::VarTChirho(name_chirho) if name_chirho.occ_chirho == "mChirho")
                        ),
                        MultiplicityChirho::ManyChirho => panic!("not a test input"),
                    }
                }
            }
            let constructor_chirho = ConDeclChirho::GadtChirho {
                name_chirho: mk_name_chirho("MkBoxChirho"),
                signature_chirho: Box::new(
                    haskelujah_ast_chirho::decl_chirho::GadtConSignatureChirho::from_type_chirho(
                        ty_chirho,
                    ),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            projected_chirho.push(
                ast_con_to_th_chirho(
                    &constructor_chirho,
                    &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho(
                    ),
                )
                .expect("GHC projects all three into a BangType"),
            );
        }
        assert_eq!(projected_chirho[0], projected_chirho[1]);
        assert_eq!(projected_chirho[1], projected_chirho[2]);
        let unrestricted_chirho = arrow_chirho(Some(MultiplicityChirho::ManyChirho));
        assert_eq!(
            ast_type_to_th_chirho(&unrestricted_chirho),
            ast_type_to_th_chirho(&arrow_chirho(None))
        );
    }

    #[test]
    fn visible_forall_reification_failure_is_not_malformed_source_chirho() {
        let ty_chirho = TypeChirho::RequiredForallChirho {
            vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("aChirho"))],
            body_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("BoxChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        assert!(matches!(
            ast_type_to_th_chirho(&ty_chirho),
            ThTypeChirho::ForallVisTChirho(_, _)
        ));
        let signature_chirho =
            haskelujah_ast_chirho::decl_chirho::GadtConSignatureChirho::from_type_chirho(ty_chirho);
        assert!(signature_chirho.check_written_signature_chirho().is_ok());
        let constructor_chirho = ConDeclChirho::GadtChirho {
            name_chirho: mk_name_chirho("MkBoxChirho"),
            signature_chirho: Box::new(signature_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let error_chirho = ast_con_to_th_chirho(
            &constructor_chirho,
            &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho(),
        )
        .expect_err("GHC cannot represent a visible constructor forall in TH declarations");
        assert!(matches!(
            error_chirho,
            ThReificationErrorChirho::UnsupportedVisibleForallChirho { .. }
        ));
        assert_eq!(error_chirho.ghc_code_chirho(), Some("GHC-75721"));
        let declaration_chirho = DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("BoxChirho"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![constructor_chirho],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        assert!(matches!(
            reify_decl_chirho(
                &declaration_chirho,
                &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho()
            ),
            Err(ThReificationErrorChirho::UnsupportedVisibleForallChirho { .. })
        ));
    }
}
