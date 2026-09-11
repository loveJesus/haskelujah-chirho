// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Kind-declaration dependencies come from type syntax, never value-name scans.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{AstKindChirho, ConstraintChirho, DeclChirho, ModuleChirho, TyVarChirho, TypeChirho};
use haskelujah_ast_chirho::decl_chirho::ConDeclChirho;
use haskelujah_ast_chirho::ty_chirho::MultiplicityChirho;
use std::collections::{HashMap, HashSet};

pub(super) struct KindDependenciesChirho<'source_chirho> {
    pub(super) declarations_chirho: Vec<&'source_chirho DeclChirho>,
    pub(super) names_chirho: Vec<Vec<&'source_chirho str>>,
    pub(super) edges_chirho: Vec<Vec<usize>>,
}

impl<'source_chirho> KindDependenciesChirho<'source_chirho> {
    pub(super) fn new_chirho(module_chirho: &'source_chirho ModuleChirho) -> Self {
        let mut declarations_chirho = Vec::new();
        let mut names_chirho = Vec::new();
        let mut owners_chirho = HashMap::new();
        for declaration_chirho in &module_chirho.decls_chirho {
            let mut declared_chirho = match declaration_chirho {
                DeclChirho::DataDeclChirho { name_chirho, .. }
                | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
                | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
                | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. }
                | DeclChirho::ClassDeclChirho { name_chirho, .. } => {
                    vec![name_chirho.text_chirho()]
                }
                _ => continue,
            };
            if let DeclChirho::ClassDeclChirho {
                associated_tfs_chirho,
                ..
            } = declaration_chirho
            {
                declared_chirho.extend(
                    associated_tfs_chirho
                        .iter()
                        .map(|family_chirho| family_chirho.name_chirho.text_chirho()),
                );
            }
            let index_chirho = declarations_chirho.len();
            for &name_chirho in &declared_chirho {
                owners_chirho.insert(name_chirho.to_owned(), index_chirho);
                owners_chirho.insert(
                    format!(
                        "{}.{}",
                        module_chirho.name_chirho.text_chirho(),
                        name_chirho
                    ),
                    index_chirho,
                );
            }
            declarations_chirho.push(declaration_chirho);
            names_chirho.push(declared_chirho);
        }
        let edges_chirho = declarations_chirho
            .iter()
            .map(|declaration_chirho| {
                let mut references_chirho = HashSet::new();
                declaration_refs_chirho(declaration_chirho, &mut references_chirho);
                let mut targets_chirho: Vec<_> = references_chirho
                    .into_iter()
                    .filter_map(|name_chirho| owners_chirho.get(&name_chirho).copied())
                    .collect();
                targets_chirho.sort_unstable();
                targets_chirho.dedup();
                targets_chirho
            })
            .collect();
        Self {
            declarations_chirho,
            names_chirho,
            edges_chirho,
        }
    }
}

fn declaration_refs_chirho(declaration_chirho: &DeclChirho, refs_chirho: &mut HashSet<String>) {
    match declaration_chirho {
        DeclChirho::DataDeclChirho {
            type_vars_chirho,
            constructors_chirho,
            kind_sig_chirho,
            ..
        } => {
            binder_refs_chirho(type_vars_chirho, refs_chirho);
            if let Some(signature_chirho) = kind_sig_chirho {
                for ty_chirho in signature_chirho
                    .standalone_chirho()
                    .into_iter()
                    .chain(signature_chirho.result_chirho())
                {
                    type_refs_chirho(ty_chirho, refs_chirho);
                }
            }
            for constructor_chirho in constructors_chirho {
                constructor_refs_chirho(constructor_chirho, refs_chirho);
            }
        }
        DeclChirho::NewtypeDeclChirho {
            type_vars_chirho,
            constructor_chirho,
            kind_sig_chirho,
            ..
        } => {
            binder_refs_chirho(type_vars_chirho, refs_chirho);
            if let Some(signature_chirho) = kind_sig_chirho {
                for ty_chirho in signature_chirho
                    .standalone_chirho()
                    .into_iter()
                    .chain(signature_chirho.result_chirho())
                {
                    type_refs_chirho(ty_chirho, refs_chirho);
                }
            }
            constructor_refs_chirho(constructor_chirho, refs_chirho);
        }
        DeclChirho::TypeAliasDeclChirho {
            type_vars_chirho,
            rhs_chirho,
            ..
        } => {
            binder_refs_chirho(type_vars_chirho, refs_chirho);
            type_refs_chirho(rhs_chirho, refs_chirho);
        }
        DeclChirho::TypeFamilyDeclChirho {
            type_vars_chirho,
            result_chirho,
            equations_chirho,
            ..
        } => {
            binder_refs_chirho(type_vars_chirho, refs_chirho);
            if let Some(kind_chirho) = &result_chirho.kind_chirho {
                type_refs_chirho(kind_chirho, refs_chirho);
            }
            for equation_chirho in equations_chirho {
                for argument_chirho in &equation_chirho.lhs_types_chirho {
                    type_refs_chirho(argument_chirho, refs_chirho);
                }
                type_refs_chirho(&equation_chirho.rhs_chirho, refs_chirho);
            }
        }
        DeclChirho::ClassDeclChirho {
            type_vars_chirho,
            context_chirho,
            methods_chirho,
            associated_tfs_chirho,
            ..
        } => {
            binder_refs_chirho(type_vars_chirho, refs_chirho);
            for constraint_chirho in context_chirho {
                constraint_refs_chirho(constraint_chirho, refs_chirho);
            }
            for method_chirho in methods_chirho {
                type_refs_chirho(&method_chirho.ty_chirho, refs_chirho);
            }
            for family_chirho in associated_tfs_chirho {
                if let Some(rhs_chirho) = &family_chirho.default_rhs_chirho {
                    type_refs_chirho(rhs_chirho, refs_chirho);
                }
            }
        }
        _ => {}
    }
}

fn constructor_refs_chirho(constructor_chirho: &ConDeclChirho, refs_chirho: &mut HashSet<String>) {
    match constructor_chirho {
        ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
            for (_, field_chirho) in fields_chirho {
                type_refs_chirho(field_chirho, refs_chirho);
            }
        }
        ConDeclChirho::RecordChirho { fields_chirho, .. } => {
            for field_chirho in fields_chirho {
                type_refs_chirho(&field_chirho.ty_chirho, refs_chirho);
            }
        }
        ConDeclChirho::GadtChirho { ty_chirho, .. } => type_refs_chirho(ty_chirho, refs_chirho),
    }
}

fn binder_refs_chirho(binders_chirho: &[TyVarChirho], refs_chirho: &mut HashSet<String>) {
    for binder_chirho in binders_chirho {
        if let Some(annotation_chirho) = &binder_chirho.kind_annotation_chirho {
            kind_refs_chirho(annotation_chirho, refs_chirho);
        }
    }
}

fn kind_refs_chirho(kind_chirho: &AstKindChirho, refs_chirho: &mut HashSet<String>) {
    match kind_chirho {
        AstKindChirho::ConChirho(name_chirho) => {
            refs_chirho.insert(name_chirho.full_name_chirho());
        }
        AstKindChirho::ArrowChirho(argument_chirho, result_chirho)
        | AstKindChirho::AppChirho(argument_chirho, result_chirho) => {
            kind_refs_chirho(argument_chirho, refs_chirho);
            kind_refs_chirho(result_chirho, refs_chirho);
        }
        AstKindChirho::StarChirho
        | AstKindChirho::ConstraintChirho
        | AstKindChirho::VarChirho(_) => {}
    }
}

fn type_refs_chirho(ty_chirho: &TypeChirho, refs_chirho: &mut HashSet<String>) {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => {
            refs_chirho.insert(name_chirho.full_name_chirho());
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            type_refs_chirho(fun_chirho, refs_chirho);
            type_refs_chirho(arg_chirho, refs_chirho);
        }
        TypeChirho::FunChirho {
            arg_chirho,
            mult_chirho,
            result_chirho,
            ..
        } => {
            type_refs_chirho(arg_chirho, refs_chirho);
            if let Some(MultiplicityChirho::ExpressionChirho(expression_chirho)) = mult_chirho {
                type_refs_chirho(expression_chirho, refs_chirho);
            }
            type_refs_chirho(result_chirho, refs_chirho);
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
            binder_refs_chirho(vars_chirho, refs_chirho);
            type_refs_chirho(body_chirho, refs_chirho);
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            for constraint_chirho in context_chirho {
                constraint_refs_chirho(constraint_chirho, refs_chirho);
            }
            type_refs_chirho(body_chirho, refs_chirho);
        }
        TypeChirho::ListChirho { element_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: element_chirho,
            ..
        } => type_refs_chirho(element_chirho, refs_chirho),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                type_refs_chirho(element_chirho, refs_chirho);
            }
        }
        TypeChirho::VarChirho(_)
        | TypeChirho::PromotedConChirho { .. }
        | TypeChirho::WildcardChirho { .. }
        | TypeChirho::LitChirho { .. } => {}
    }
}

fn constraint_refs_chirho(constraint_chirho: &ConstraintChirho, refs_chirho: &mut HashSet<String>) {
    match constraint_chirho {
        ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        } => {
            refs_chirho.insert(class_chirho.full_name_chirho());
            for argument_chirho in args_chirho {
                type_refs_chirho(argument_chirho, refs_chirho);
            }
        }
        ConstraintChirho::QuantifiedChirho {
            vars_chirho,
            context_chirho,
            body_chirho,
            ..
        } => {
            binder_refs_chirho(vars_chirho, refs_chirho);
            for constraint_chirho in context_chirho {
                constraint_refs_chirho(constraint_chirho, refs_chirho);
            }
            constraint_refs_chirho(body_chirho, refs_chirho);
        }
    }
}
