// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The local classes GND derives with written class arguments (row 694, #31897, T3955).
//!
//! `deriving (MonadReader a)` on `newtype T a x = T (Reader a x)` asks for
//! `MonadReader a (Reader a) => MonadReader a (T a)`: every written argument is kept, and the
//! newtype and its representation drop as many trailing variables as the class's last
//! parameter takes arguments. This module decides which classes that shape serves: a class the
//! module declares itself, with no methods, so its dictionary holds only superclass fields,
//! each the newtype's own; one parameter more than the clause writes; and a last parameter
//! whose kind arity the module proves, from its kind annotation or from a superclass applying
//! a builtin class naming proves (`Monad m` takes one argument). A shape the module does not
//! prove stays refused until its method and dictionary route exists.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    AstKindChirho, DeclChirho, DerivedClassChirho, TyVarChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{
    CompilerClassGroundKindChirho, CompilerClassKindAtomChirho, CompilerClassKindChirho,
};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};

use super::stock_identity_chirho::StockIdentityChirho;

/// What one local class's declaration proves about deriving it with class arguments.
struct LocalClassShapeChirho {
    /// Its visible parameters.
    parameters_chirho: usize,
    /// Whether it declares no method.
    methodless_chirho: bool,
    /// How many type arguments its last parameter takes, when the module proves it.
    last_arity_chirho: Option<usize>,
}

/// The module's own classes, as GND with written class arguments reads them.
pub(crate) struct LocalClassShapesChirho {
    shapes_chirho: HashMap<String, LocalClassShapeChirho>,
}

impl LocalClassShapesChirho {
    /// Every class `module_chirho` declares, judged with the builtin classes naming proved.
    pub(crate) fn of_module_chirho(
        module_chirho: &ModuleChirho,
        identity_chirho: &StockIdentityChirho<'_>,
    ) -> Self {
        let shapes_chirho = module_chirho
            .decls_chirho
            .iter()
            .filter_map(|decl_chirho| {
                let DeclChirho::ClassDeclChirho {
                    context_chirho,
                    name_chirho,
                    type_vars_chirho,
                    methods_chirho,
                    ..
                } = decl_chirho
                else {
                    return None;
                };
                let visible_chirho = TyVarChirho::visible_binders_chirho(type_vars_chirho);
                let last_arity_chirho = visible_chirho.last().and_then(|last_chirho| {
                    last_parameter_arity_chirho(last_chirho, context_chirho, identity_chirho)
                });
                Some((
                    name_chirho.text_chirho().to_string(),
                    LocalClassShapeChirho {
                        parameters_chirho: visible_chirho.len(),
                        methodless_chirho: methods_chirho.is_empty(),
                        last_arity_chirho,
                    },
                ))
            })
            .collect();
        Self { shapes_chirho }
    }

    /// The kind arity of the last parameter of the module's own class `derived_chirho` names,
    /// when GND may derive it with its written class arguments; `None` for any other request.
    pub(crate) fn class_argument_arity_chirho(
        &self,
        derived_chirho: &DerivedClassChirho,
        identity_chirho: &StockIdentityChirho<'_>,
    ) -> Option<usize> {
        if !derived_chirho.has_class_arguments_chirho()
            || !identity_chirho.names_local_chirho(&derived_chirho.class_chirho)
        {
            return None;
        }
        let shape_chirho = self
            .shapes_chirho
            .get(derived_chirho.class_chirho.text_chirho())?;
        (shape_chirho.methodless_chirho
            && shape_chirho.parameters_chirho == derived_chirho.class_arguments_chirho.len() + 1)
            .then_some(shape_chirho.last_arity_chirho)
            .flatten()
    }
}

/// How many type arguments `last_chirho` takes: its kind annotation's, or the one superclass
/// constraints fix through builtin classes naming proves. A builtin class proves its closed
/// parameter kind, read by the same rule as an annotation; its eta count never stands in for
/// that kind (`MonadTrans t` makes `t` take a constructor and then a type, so it proves no
/// count of `Type` arguments). `None` when nothing proves it or two proofs disagree.
fn last_parameter_arity_chirho(
    last_chirho: &TyVarChirho,
    context_chirho: &[ConstraintChirho],
    identity_chirho: &StockIdentityChirho<'_>,
) -> Option<usize> {
    let mut proofs_chirho: Vec<usize> = last_chirho
        .kind_annotation_chirho
        .as_ref()
        .and_then(|kind_chirho| written_type_arity_chirho(kind_chirho, identity_chirho))
        .into_iter()
        .collect();
    for constraint_chirho in context_chirho {
        let ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        } = constraint_chirho
        else {
            continue;
        };
        if identity_chirho.names_local_chirho(class_chirho) {
            continue;
        }
        let Some(builtin_chirho) = identity_chirho
            .builtin_classes_chirho()
            .class_chirho(&class_chirho.full_name_chirho())
        else {
            continue;
        };
        for (index_chirho, argument_chirho) in args_chirho.iter().enumerate() {
            if is_variable_chirho(argument_chirho, last_chirho.name_chirho.text_chirho())
                && let Some(arity_chirho) = builtin_chirho
                    .schema_chirho()
                    .closed_parameter_kind_chirho(index_chirho)
                    .and_then(written_kind_chirho)
                    .as_ref()
                    .and_then(kind_arity_chirho)
            {
                proofs_chirho.push(arity_chirho);
            }
        }
    }
    let (first_chirho, rest_chirho) = proofs_chirho.split_first()?;
    rest_chirho
        .iter()
        .all(|other_chirho| other_chirho == first_chirho)
        .then_some(*first_chirho)
}

/// The number of `Type` arguments a written kind takes: `Type -> Type` takes one. `None` for
/// a kind whose arguments are not all `Type` or whose result is not `Type`.
/// A bounded view of full written kind syntax, with no name-derived authority.
fn written_type_arity_chirho(
    kind_chirho: &TypeChirho,
    identity_chirho: &StockIdentityChirho<'_>,
) -> Option<usize> {
    let mut remaining_chirho =
        haskelujah_ast_chirho::decl_chirho::GadtProjectionLimitsChirho::default().depth_chirho;
    let mut current_chirho = kind_chirho;
    let mut arity_chirho = 0usize;
    loop {
        remaining_chirho = remaining_chirho.checked_sub(1)?;
        match current_chirho {
            TypeChirho::ParenChirho { inner_chirho, .. } => current_chirho = inner_chirho,
            TypeChirho::ConChirho(name_chirho) => {
                return identity_chirho
                    .proves_type_universe_chirho(name_chirho)
                    .then_some(arity_chirho);
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                let mut argument_chirho = arg_chirho.as_ref();
                loop {
                    remaining_chirho = remaining_chirho.checked_sub(1)?;
                    match argument_chirho {
                        TypeChirho::ParenChirho { inner_chirho, .. } => {
                            argument_chirho = inner_chirho
                        }
                        TypeChirho::ConChirho(name_chirho)
                            if identity_chirho.proves_type_universe_chirho(name_chirho) =>
                        {
                            break;
                        }
                        _ => return None,
                    }
                }
                arity_chirho = arity_chirho.checked_add(1)?;
                current_chirho = result_chirho;
            }
            _ => return None,
        }
    }
}

fn kind_arity_chirho(kind_chirho: &AstKindChirho) -> Option<usize> {
    match kind_chirho {
        AstKindChirho::StarChirho => Some(0),
        AstKindChirho::ArrowChirho(argument_chirho, result_chirho)
            if **argument_chirho == AstKindChirho::StarChirho =>
        {
            kind_arity_chirho(result_chirho).map(|rest_chirho| rest_chirho + 1)
        }
        _ => None,
    }
}

/// A builtin class's closed parameter kind as a written one, so an annotation and a builtin
/// proof share `kind_arity_chirho`. `None` for an atom other than `Type`.
fn written_kind_chirho(kind_chirho: &CompilerClassKindChirho) -> Option<AstKindChirho> {
    match kind_chirho {
        CompilerClassKindChirho::AtomChirho(CompilerClassKindAtomChirho::GroundChirho(
            CompilerClassGroundKindChirho::TYPE_CHIRHO,
        )) => Some(AstKindChirho::StarChirho),
        CompilerClassKindChirho::ArrowChirho(argument_chirho, result_chirho) => {
            Some(AstKindChirho::ArrowChirho(
                Box::new(written_kind_chirho(argument_chirho)?),
                Box::new(written_kind_chirho(result_chirho)?),
            ))
        }
        CompilerClassKindChirho::AtomChirho(_) => None,
    }
}

/// Whether `type_chirho` is the variable `name_chirho`, through parentheses.
fn is_variable_chirho(type_chirho: &TypeChirho, name_chirho: &str) -> bool {
    match type_chirho {
        TypeChirho::VarChirho(variable_chirho) => variable_chirho.text_chirho() == name_chirho,
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            is_variable_chirho(inner_chirho, name_chirho)
        }
        _ => false,
    }
}

#[cfg(test)]
#[path = "class_shapes_tests_chirho.rs"]
mod tests_chirho;
