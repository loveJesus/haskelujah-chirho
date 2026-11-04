// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Producer-side transport of family identity through exported type syntax.

use super::family_definitions_chirho::TypeFamilyRegistryChirho;
use super::family_scope_chirho::{TypeFamilyScopeChirho, TypeFamilyScopeResolutionChirho};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, MultiplicityChirho, TypeChirho};

const FAMILY_TRANSPORT_WORK_CHIRHO: usize = 100_000;

pub(super) fn unresolved_family_key_chirho(module_chirho: &str, name_chirho: &str) -> String {
    format!(
        "%family:unresolved:{}:{module_chirho}:{}:{name_chirho}",
        module_chirho.len(),
        name_chirho.len()
    )
}

/// Preserve exporter's stamps, including unresolved stamps. Never resolve one in its importer.
pub fn stamp_family_references_chirho(
    ty_chirho: &mut TypeChirho,
    scope_chirho: &TypeFamilyScopeChirho,
    registry_chirho: &TypeFamilyRegistryChirho,
    module_name_chirho: &str,
) {
    fn walk_constraint_chirho(
        constraint_value_chirho: &mut ConstraintChirho,
        scope_chirho: &TypeFamilyScopeChirho,
        registry_chirho: &TypeFamilyRegistryChirho,
        module_chirho: &str,
        budget_chirho: &mut usize,
        depth_chirho: usize,
    ) -> bool {
        if depth_chirho > 128 || *budget_chirho == 0 {
            return false;
        }
        *budget_chirho -= 1;
        match constraint_value_chirho {
            ConstraintChirho::ClassChirho {
                args_chirho,
                kind_args_chirho,
                ..
            } => kind_args_chirho
                .iter_mut()
                .chain(args_chirho)
                .all(|argument_chirho| {
                    walk_chirho(
                        argument_chirho,
                        scope_chirho,
                        registry_chirho,
                        module_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )
                }),
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho,
                kind_chirho,
                ..
            } => {
                walk_constraint_chirho(
                    constraint_chirho,
                    scope_chirho,
                    registry_chirho,
                    module_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                ) && walk_chirho(
                    kind_chirho,
                    scope_chirho,
                    registry_chirho,
                    module_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )
            }
            ConstraintChirho::QuantifiedChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                context_chirho.iter_mut().all(|inner_chirho| {
                    walk_constraint_chirho(
                        inner_chirho,
                        scope_chirho,
                        registry_chirho,
                        module_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )
                }) && walk_constraint_chirho(
                    body_chirho,
                    scope_chirho,
                    registry_chirho,
                    module_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )
            }
        }
    }
    fn walk_chirho(
        ty_chirho: &mut TypeChirho,
        scope_chirho: &TypeFamilyScopeChirho,
        registry_chirho: &TypeFamilyRegistryChirho,
        module_chirho: &str,
        budget_chirho: &mut usize,
        depth_chirho: usize,
    ) -> bool {
        if depth_chirho > 128 || *budget_chirho == 0 {
            return false;
        }
        *budget_chirho -= 1;
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) if name_chirho.family_key_chirho().is_none() => {
                let written_chirho = name_chirho.full_name_chirho();
                match scope_chirho.resolve_chirho(&written_chirho, registry_chirho) {
                    TypeFamilyScopeResolutionChirho::UniqueChirho(key_chirho) => {
                        name_chirho.set_family_key_chirho(key_chirho.to_owned())
                    }
                    TypeFamilyScopeResolutionChirho::AmbiguousChirho(_) => name_chirho
                        .set_family_key_chirho(unresolved_family_key_chirho(
                            module_chirho,
                            &written_chirho,
                        )),
                    TypeFamilyScopeResolutionChirho::MissingChirho => {}
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
                return walk_chirho(
                    fun_chirho,
                    scope_chirho,
                    registry_chirho,
                    module_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                ) && walk_chirho(
                    arg_chirho,
                    scope_chirho,
                    registry_chirho,
                    module_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                );
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                return elements_chirho.iter_mut().all(|element_chirho| {
                    walk_chirho(
                        element_chirho,
                        scope_chirho,
                        registry_chirho,
                        module_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )
                });
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
                return walk_chirho(
                    element_chirho,
                    scope_chirho,
                    registry_chirho,
                    module_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                );
            }
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                return context_chirho.iter_mut().all(|constraint_value_chirho| {
                    walk_constraint_chirho(
                        constraint_value_chirho,
                        scope_chirho,
                        registry_chirho,
                        module_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )
                }) && walk_chirho(
                    body_chirho,
                    scope_chirho,
                    registry_chirho,
                    module_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                );
            }
            _ => {}
        }
        true
    }
    let mut budget_chirho = FAMILY_TRANSPORT_WORK_CHIRHO;
    if !walk_chirho(
        ty_chirho,
        scope_chirho,
        registry_chirho,
        module_name_chirho,
        &mut budget_chirho,
        0,
    ) {
        let span_chirho = ty_chirho.span_chirho();
        let key_chirho = unresolved_family_key_chirho(module_name_chirho, "resource-bound");
        let mut name_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            key_chirho.clone(),
            span_chirho,
        ));
        name_chirho.set_family_key_chirho(key_chirho);
        *ty_chirho = TypeChirho::ConChirho(name_chirho);
    }
}

pub(super) fn family_rhs_supported_chirho(
    ty_chirho: &TypeChirho,
    pattern_chirho: bool,
    work_chirho: &mut usize,
    depth_chirho: usize,
) -> bool {
    if depth_chirho > 128 || *work_chirho == 0 {
        return false;
    }
    *work_chirho -= 1;
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho)
        | TypeChirho::ConChirho(name_chirho)
        | TypeChirho::PromotedConChirho { name_chirho, .. } => {
            let bytes_chirho = name_chirho
                .text_chirho()
                .len()
                .saturating_add(name_chirho.qualifier_chirho().map_or(0, str::len))
                .saturating_add(name_chirho.family_key_chirho().map_or(0, str::len));
            if let Some(remaining_chirho) = work_chirho.checked_sub(bytes_chirho) {
                *work_chirho = remaining_chirho;
                true
            } else {
                false
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
        } => {
            family_rhs_supported_chirho(fun_chirho, pattern_chirho, work_chirho, depth_chirho + 1)
                && family_rhs_supported_chirho(
                    arg_chirho,
                    pattern_chirho,
                    work_chirho,
                    depth_chirho + 1,
                )
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            mult_chirho,
            ..
        } => {
            !matches!(mult_chirho, Some(MultiplicityChirho::MultVarChirho(_)))
                && family_rhs_supported_chirho(
                    arg_chirho,
                    pattern_chirho,
                    work_chirho,
                    depth_chirho + 1,
                )
                && family_rhs_supported_chirho(
                    result_chirho,
                    pattern_chirho,
                    work_chirho,
                    depth_chirho + 1,
                )
        }
        TypeChirho::ListChirho { element_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: element_chirho,
            ..
        } => family_rhs_supported_chirho(
            element_chirho,
            pattern_chirho,
            work_chirho,
            depth_chirho + 1,
        ),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => elements_chirho.iter().all(|element_chirho| {
            family_rhs_supported_chirho(
                element_chirho,
                pattern_chirho,
                work_chirho,
                depth_chirho + 1,
            )
        }),
        TypeChirho::LitChirho { value_chirho, .. } => {
            if let Some(remaining_chirho) = work_chirho.checked_sub(value_chirho.len()) {
                *work_chirho = remaining_chirho;
                true
            } else {
                false
            }
        }
        TypeChirho::WildcardChirho { .. } => pattern_chirho,
        TypeChirho::QualChirho { .. }
        | TypeChirho::ForallChirho { .. }
        | TypeChirho::RequiredForallChirho { .. } => false,
    }
}

pub(super) fn prepare_pattern_chirho(
    ty_chirho: &mut TypeChirho,
    equation_chirho: u32,
    wildcard_chirho: &mut u32,
    work_chirho: &mut usize,
    depth_chirho: usize,
) -> bool {
    if depth_chirho > 128 || *work_chirho == 0 {
        return false;
    }
    *work_chirho -= 1;
    match ty_chirho {
        TypeChirho::WildcardChirho { span_chirho } => {
            let name_chirho = format!("%wildcard:{equation_chirho}:{}", *wildcard_chirho);
            let Some(next_chirho) = wildcard_chirho.checked_add(1) else {
                return false;
            };
            *wildcard_chirho = next_chirho;
            *ty_chirho = TypeChirho::VarChirho(NameChirho::RawChirho(
                RawNameChirho::unqualified_chirho(name_chirho, *span_chirho),
            ));
            true
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            prepare_pattern_chirho(
                fun_chirho,
                equation_chirho,
                wildcard_chirho,
                work_chirho,
                depth_chirho + 1,
            ) && prepare_pattern_chirho(
                arg_chirho,
                equation_chirho,
                wildcard_chirho,
                work_chirho,
                depth_chirho + 1,
            )
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            mult_chirho,
            ..
        } => {
            !matches!(mult_chirho, Some(MultiplicityChirho::MultVarChirho(_)))
                && prepare_pattern_chirho(
                    arg_chirho,
                    equation_chirho,
                    wildcard_chirho,
                    work_chirho,
                    depth_chirho + 1,
                )
                && prepare_pattern_chirho(
                    result_chirho,
                    equation_chirho,
                    wildcard_chirho,
                    work_chirho,
                    depth_chirho + 1,
                )
        }
        TypeChirho::ListChirho { element_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: element_chirho,
            ..
        } => prepare_pattern_chirho(
            element_chirho,
            equation_chirho,
            wildcard_chirho,
            work_chirho,
            depth_chirho + 1,
        ),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => elements_chirho.iter_mut().all(|element_chirho| {
            prepare_pattern_chirho(
                element_chirho,
                equation_chirho,
                wildcard_chirho,
                work_chirho,
                depth_chirho + 1,
            )
        }),
        TypeChirho::QualChirho { .. }
        | TypeChirho::ForallChirho { .. }
        | TypeChirho::RequiredForallChirho { .. } => false,
        _ => true,
    }
}

/// Check before cloning: unsupported syntax must keep the whole open union held.
pub(super) fn family_rhs_is_supported_chirho(ty_chirho: &TypeChirho) -> bool {
    let mut work_chirho = FAMILY_TRANSPORT_WORK_CHIRHO;
    family_rhs_supported_chirho(ty_chirho, false, &mut work_chirho, 0)
}

/// Ordinary and associated open equations share wildcard freshness and shape guards.
pub(super) fn prepare_family_instance_syntax_chirho(
    lhs_chirho: &[TypeChirho],
    rhs_chirho: &TypeChirho,
) -> Option<(Vec<TypeChirho>, TypeChirho)> {
    let mut work_chirho = FAMILY_TRANSPORT_WORK_CHIRHO;
    if !lhs_chirho
        .iter()
        .all(|ty_chirho| family_rhs_supported_chirho(ty_chirho, true, &mut work_chirho, 0))
        || !family_rhs_supported_chirho(rhs_chirho, false, &mut work_chirho, 0)
    {
        return None;
    }
    let mut lhs_chirho = lhs_chirho.to_vec();
    let mut wildcard_chirho = 0;
    let mut work_chirho = FAMILY_TRANSPORT_WORK_CHIRHO;
    if !lhs_chirho.iter_mut().all(|ty_chirho| {
        prepare_pattern_chirho(ty_chirho, 0, &mut wildcard_chirho, &mut work_chirho, 0)
    }) {
        return None;
    }
    Some((lhs_chirho, rhs_chirho.clone()))
}
