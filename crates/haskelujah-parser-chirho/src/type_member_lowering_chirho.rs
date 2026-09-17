// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Class members retain kinded heads and default equations separately.
//! Workflow: compiler-pipeline-chirho/type-scope-resolution-chirho.
use haskelujah_ast_chirho::decl_chirho::{
    AssocTypeFamilyChirho, ClassMethodChirho, DeclChirho, DeclKindSigChirho, TyVarChirho,
    TypeFamilyEquationChirho, TypeFamilyResultChirho,
};
use haskelujah_ast_chirho::expr_chirho::MatchArmChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use std::collections::{HashMap, HashSet};

pub(crate) fn partition_class_members_chirho(
    where_decls_chirho: Vec<DeclChirho>,
    default_impls_chirho: &HashMap<String, Vec<MatchArmChirho>>,
    default_sigs_chirho: &HashMap<String, TypeChirho>,
    visible_kind_binder_names_chirho: &HashSet<String>,
) -> (Vec<ClassMethodChirho>, Vec<AssocTypeFamilyChirho>) {
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
    let mut families_chirho = Vec::new();
    let mut defaults_chirho: Vec<(NameChirho, TypeFamilyEquationChirho)> = Vec::new();
    let retain_chirho = |vars_chirho: Vec<TyVarChirho>| {
        vars_chirho
            .into_iter()
            .filter(|var_chirho| {
                !visible_kind_binder_names_chirho.contains(var_chirho.text_chirho())
            })
            .collect()
    };
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
                    families_chirho.push(AssocTypeFamilyChirho {
                        name_chirho,
                        type_vars_chirho: Vec::new(),
                        result_chirho: TypeFamilyResultChirho {
                            kind_sig_chirho: Some(DeclKindSigChirho::ResultChirho(ty_chirho)),
                            ..TypeFamilyResultChirho::default()
                        },
                        data_chirho: false,
                        head_declared_chirho: true,
                        defaults_chirho: Vec::new(),
                        span_chirho,
                    });
                } else {
                    methods_chirho.push(ClassMethodChirho {
                        default_chirho: default_impls_chirho
                            .get(name_chirho.text_chirho())
                            .cloned(),
                        default_sig_chirho: default_sigs_chirho
                            .get(name_chirho.text_chirho())
                            .cloned(),
                        name_chirho,
                        ty_chirho,
                        span_chirho,
                    });
                }
            }
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                result_chirho,
                body_chirho,
                span_chirho,
            } => {
                families_chirho.push(AssocTypeFamilyChirho {
                    name_chirho,
                    type_vars_chirho: retain_chirho(type_vars_chirho),
                    result_chirho,
                    data_chirho: false,
                    head_declared_chirho: true,
                    defaults_chirho: body_chirho.equations_chirho().to_vec(),
                    span_chirho,
                });
            }
            DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                rhs_chirho,
                span_chirho,
            } => {
                defaults_chirho.push((
                    name_chirho,
                    TypeFamilyEquationChirho {
                        lhs_types_chirho: type_vars_chirho
                            .into_iter()
                            .map(|var_chirho| TypeChirho::VarChirho(var_chirho.name_chirho))
                            .collect(),
                        rhs_chirho,
                        span_chirho,
                    },
                ));
            }
            DeclChirho::TypeFamilyInstanceDeclChirho {
                family_name_chirho,
                lhs_types_chirho,
                rhs_chirho,
                span_chirho,
            } => {
                defaults_chirho.push((
                    family_name_chirho,
                    TypeFamilyEquationChirho {
                        lhs_types_chirho,
                        rhs_chirho,
                        span_chirho,
                    },
                ));
            }
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                kind_sig_chirho,
                span_chirho,
                ..
            } => {
                families_chirho.push(AssocTypeFamilyChirho {
                    name_chirho,
                    type_vars_chirho: retain_chirho(type_vars_chirho),
                    result_chirho: TypeFamilyResultChirho {
                        kind_sig_chirho,
                        ..TypeFamilyResultChirho::default()
                    },
                    data_chirho: true,
                    head_declared_chirho: true,
                    defaults_chirho: Vec::new(),
                    span_chirho,
                });
            }
            _ => {}
        }
    }
    let mut indices_chirho: HashMap<String, usize> = families_chirho
        .iter()
        .enumerate()
        .map(|(index_chirho, family_chirho)| {
            (
                family_chirho.name_chirho.text_chirho().to_owned(),
                index_chirho,
            )
        })
        .collect();
    for (name_chirho, equation_chirho) in defaults_chirho {
        if let Some(index_chirho) = indices_chirho.get(name_chirho.text_chirho()) {
            families_chirho[*index_chirho]
                .defaults_chirho
                .push(equation_chirho);
        } else {
            indices_chirho.insert(name_chirho.text_chirho().to_owned(), families_chirho.len());
            families_chirho.push(AssocTypeFamilyChirho {
                name_chirho,
                type_vars_chirho: Vec::new(),
                result_chirho: TypeFamilyResultChirho::default(),
                data_chirho: false,
                head_declared_chirho: false,
                span_chirho: equation_chirho.span_chirho,
                defaults_chirho: vec![equation_chirho],
            });
        }
    }
    (methods_chirho, families_chirho)
}
