// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC's tuple size boundary on source types, including promoted constructors.
//! Workflow: language-features-chirho/stuck-family-equalities-chirho.md.

use super::{UNRESOLVED_CONSTRAINT_MARKER_CHIRHO, ValidityErrorChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;

const MAX_TUPLE_ARITY_CHIRHO: usize = 64;

/// A tuple constructor's own arity, irrespective of how many arguments it has
/// received. Only reserved comma spellings supply this source-level fact.
fn tuple_constructor_arity_chirho(text_chirho: &str) -> Option<usize> {
    let text_chirho = text_chirho.strip_prefix('\'').unwrap_or(text_chirho);
    let inner_chirho = text_chirho.strip_prefix('(')?.strip_suffix(')')?;
    if inner_chirho.is_empty() {
        Some(0)
    } else if inner_chirho.bytes().all(|byte_chirho| byte_chirho == b',') {
        Some(inner_chirho.len() + 1)
    } else {
        None
    }
}

fn report_arity_chirho(
    arity_chirho: usize,
    span_chirho: SpanChirho,
    errors_chirho: &mut Vec<ValidityErrorChirho>,
) {
    if arity_chirho > MAX_TUPLE_ARITY_CHIRHO {
        errors_chirho.push(ValidityErrorChirho {
            message_chirho: format!(
                "A {arity_chirho}-tuple is too large (maximum size is {MAX_TUPLE_ARITY_CHIRHO}; GHC-94803)"
            ),
            suggested_extension_chirho: None,
            span_chirho,
        });
    }
}

/// Check one type node. The existing validity walker owns ordinary recursion,
/// so a promoted tuple's application spine reports its constructor only once.
pub(super) fn check_type_tuple_arity_chirho(
    ty_chirho: &TypeChirho,
    errors_chirho: &mut Vec<ValidityErrorChirho>,
) {
    match ty_chirho {
        TypeChirho::TupleChirho {
            elements_chirho,
            span_chirho,
        } => {
            report_arity_chirho(elements_chirho.len(), *span_chirho, errors_chirho);
        }
        TypeChirho::ConChirho(name_chirho) | TypeChirho::PromotedConChirho { name_chirho, .. } => {
            if let Some(arity_chirho) = tuple_constructor_arity_chirho(name_chirho.text_chirho()) {
                report_arity_chirho(arity_chirho, name_chirho.span_chirho(), errors_chirho);
            }
        }
        _ => {}
    }
}

enum TupleSiteChirho<'source_chirho> {
    TypeChirho(&'source_chirho TypeChirho),
    ConstraintChirho(&'source_chirho ConstraintChirho),
}

/// The constraint gateway also sees a tuple constructor used as a constraint
/// head. Quantified constraints are not recursively extension-checked by the
/// older walker, so visit their preserved types here for tuple size alone.
/// Never infer a tuple's arity from a flattened context vector: nested legal
/// constraint tuples can contain more than 64 constraints in total.
pub(super) fn check_constraint_tuple_arity_chirho(
    constraint_chirho: &ConstraintChirho,
    errors_chirho: &mut Vec<ValidityErrorChirho>,
) {
    match constraint_chirho {
        ConstraintChirho::ClassChirho { class_chirho, .. } => {
            if let Some(arity_chirho) = tuple_constructor_arity_chirho(class_chirho.text_chirho()) {
                report_arity_chirho(arity_chirho, class_chirho.span_chirho(), errors_chirho);
            }
        }
        ConstraintChirho::KindAnnotationChirho { .. }
        | ConstraintChirho::QuantifiedChirho { .. } => {
            let mut pending_chirho = vec![TupleSiteChirho::ConstraintChirho(constraint_chirho)];
            while let Some(site_chirho) = pending_chirho.pop() {
                match site_chirho {
                    TupleSiteChirho::ConstraintChirho(ConstraintChirho::ClassChirho {
                        class_chirho,
                        args_chirho,
                        ..
                    }) => {
                        if class_chirho.text_chirho() == UNRESOLVED_CONSTRAINT_MARKER_CHIRHO {
                            continue;
                        }
                        if let Some(arity_chirho) =
                            tuple_constructor_arity_chirho(class_chirho.text_chirho())
                        {
                            report_arity_chirho(
                                arity_chirho,
                                class_chirho.span_chirho(),
                                errors_chirho,
                            );
                        }
                        pending_chirho.extend(args_chirho.iter().map(TupleSiteChirho::TypeChirho));
                    }
                    TupleSiteChirho::ConstraintChirho(ConstraintChirho::KindAnnotationChirho {
                        constraint_chirho,
                        kind_chirho,
                        ..
                    }) => {
                        pending_chirho.push(TupleSiteChirho::ConstraintChirho(constraint_chirho));
                        pending_chirho.push(TupleSiteChirho::TypeChirho(kind_chirho));
                    }
                    TupleSiteChirho::ConstraintChirho(ConstraintChirho::QuantifiedChirho {
                        context_chirho,
                        body_chirho,
                        ..
                    }) => {
                        pending_chirho.push(TupleSiteChirho::ConstraintChirho(body_chirho));
                        pending_chirho
                            .extend(context_chirho.iter().map(TupleSiteChirho::ConstraintChirho));
                    }
                    TupleSiteChirho::TypeChirho(ty_chirho) => {
                        check_type_tuple_arity_chirho(ty_chirho, errors_chirho);
                        match ty_chirho {
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
                                pending_chirho.push(TupleSiteChirho::TypeChirho(arg_chirho));
                                pending_chirho.push(TupleSiteChirho::TypeChirho(fun_chirho));
                            }
                            TypeChirho::TupleChirho {
                                elements_chirho, ..
                            }
                            | TypeChirho::PromotedListChirho {
                                elements_chirho, ..
                            } => {
                                pending_chirho.extend(
                                    elements_chirho.iter().map(TupleSiteChirho::TypeChirho),
                                );
                            }
                            TypeChirho::ListChirho { element_chirho, .. }
                            | TypeChirho::ParenChirho {
                                inner_chirho: element_chirho,
                                ..
                            }
                            | TypeChirho::ForallChirho {
                                body_chirho: element_chirho,
                                ..
                            }
                            | TypeChirho::RequiredForallChirho {
                                body_chirho: element_chirho,
                                ..
                            } => {
                                pending_chirho.push(TupleSiteChirho::TypeChirho(element_chirho));
                            }
                            TypeChirho::QualChirho {
                                context_chirho,
                                body_chirho,
                                ..
                            } => {
                                pending_chirho.push(TupleSiteChirho::TypeChirho(body_chirho));
                                pending_chirho.extend(
                                    context_chirho.iter().map(TupleSiteChirho::ConstraintChirho),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "tuple_arity_tests_chirho.rs"]
mod tests_chirho;
