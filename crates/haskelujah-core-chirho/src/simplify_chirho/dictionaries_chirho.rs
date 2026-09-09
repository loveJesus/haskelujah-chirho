// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Evidence-preserving dictionary specialization for native backends.
//!
//! A dictionary is an ordinary constructor and a selector is a projection.
//! Reduce a selection only when both shapes are known. Unknown evidence remains
//! a runtime argument, with its lambda and all call sites intact. Names and the
//! apparent representation of an argument are not proof of an instance.

use std::collections::{HashMap, HashSet};

use crate::expr_chirho::{
    AltConChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreModuleChirho,
};

use super::free_vars_chirho;

type DefinitionsChirho<'source_chirho> = HashMap<CoreIdChirho, &'source_chirho CoreExprChirho>;
type LocalsChirho<'source_chirho> = HashMap<CoreIdChirho, Option<&'source_chirho CoreExprChirho>>;

/// Resolve aliases without unfolding calls or recursively forcing dictionaries.
/// The visited set makes recursive evidence and alias cycles terminate.
fn resolve_chirho<'source_chirho>(
    mut expr_chirho: &'source_chirho CoreExprChirho,
    definitions_chirho: &DefinitionsChirho<'source_chirho>,
    locals_chirho: &LocalsChirho<'source_chirho>,
) -> &'source_chirho CoreExprChirho {
    let mut visited_chirho = HashSet::new();
    loop {
        match expr_chirho {
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            }
            | CoreExprChirho::TyLamChirho {
                body_chirho: inner_chirho,
                ..
            } => {
                expr_chirho = inner_chirho;
            }
            CoreExprChirho::VarChirho(id_chirho) if visited_chirho.insert(*id_chirho) => {
                let rhs_chirho = locals_chirho
                    .get(id_chirho)
                    .copied()
                    .unwrap_or_else(|| definitions_chirho.get(id_chirho).copied());
                let Some(rhs_chirho) = rhs_chirho else {
                    return expr_chirho;
                };
                expr_chirho = rhs_chirho;
            }
            _ => return expr_chirho,
        }
    }
}

fn project_chirho(
    fun_chirho: &CoreExprChirho,
    arg_chirho: &CoreExprChirho,
    definitions_chirho: &DefinitionsChirho<'_>,
    locals_chirho: &LocalsChirho<'_>,
) -> Option<CoreExprChirho> {
    let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = resolve_chirho(fun_chirho, definitions_chirho, locals_chirho)
    else {
        return None;
    };
    let CoreExprChirho::CaseChirho {
        scrutinee_chirho,
        alts_chirho,
        ..
    } = body_chirho.as_ref()
    else {
        return None;
    };
    if scrutinee_chirho.as_ref() != &CoreExprChirho::VarChirho(binder_chirho.id_chirho) {
        return None;
    }
    let CoreExprChirho::ConAppChirho {
        con_name_chirho,
        args_chirho,
    } = resolve_chirho(arg_chirho, definitions_chirho, locals_chirho)
    else {
        return None;
    };
    let alt_chirho = alts_chirho.iter().find(|alt_chirho| {
        alt_chirho.con_chirho == AltConChirho::DataConChirho(con_name_chirho.clone())
    })?;
    if alt_chirho.binders_chirho.len() != args_chirho.len() {
        return None;
    }
    let CoreExprChirho::VarChirho(field_id_chirho) = &alt_chirho.rhs_chirho else {
        return None;
    };
    let index_chirho = alt_chirho
        .binders_chirho
        .iter()
        .position(|field_chirho| field_chirho.id_chirho == *field_id_chirho)?;
    Some(args_chirho[index_chirho].clone())
}

fn specialize_chirho<'source_chirho>(
    expr_chirho: &'source_chirho CoreExprChirho,
    definitions_chirho: &DefinitionsChirho<'source_chirho>,
    locals_chirho: &LocalsChirho<'source_chirho>,
) -> CoreExprChirho {
    match expr_chirho {
        CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => expr_chirho.clone(),
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            let fun_chirho = specialize_chirho(fun_chirho, definitions_chirho, locals_chirho);
            let arg_chirho = specialize_chirho(arg_chirho, definitions_chirho, locals_chirho);
            project_chirho(&fun_chirho, &arg_chirho, definitions_chirho, locals_chirho)
                .unwrap_or_else(|| CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(fun_chirho),
                    arg_chirho: Box::new(arg_chirho),
                })
        }
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            let mut scope_chirho = locals_chirho.clone();
            scope_chirho.insert(binder_chirho.id_chirho, None);
            CoreExprChirho::LamChirho {
                binder_chirho: binder_chirho.clone(),
                body_chirho: Box::new(specialize_chirho(
                    body_chirho,
                    definitions_chirho,
                    &scope_chirho,
                )),
            }
        }
        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => {
            let mut scope_chirho = locals_chirho.clone();
            if *rec_chirho {
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    scope_chirho.insert(binder_chirho.id_chirho, Some(rhs_chirho));
                }
            }
            let mut rewritten_chirho = Vec::with_capacity(binds_chirho.len());
            for (binder_chirho, rhs_chirho) in binds_chirho {
                rewritten_chirho.push((
                    binder_chirho.clone(),
                    specialize_chirho(rhs_chirho, definitions_chirho, &scope_chirho),
                ));
                scope_chirho.insert(binder_chirho.id_chirho, Some(rhs_chirho));
            }
            CoreExprChirho::LetChirho {
                rec_chirho: *rec_chirho,
                binds_chirho: rewritten_chirho,
                body_chirho: Box::new(specialize_chirho(
                    body_chirho,
                    definitions_chirho,
                    &scope_chirho,
                )),
            }
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            result_ty_chirho,
            alts_chirho,
        } => {
            let mut scope_chirho = locals_chirho.clone();
            scope_chirho.insert(bind_chirho.id_chirho, None);
            CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(specialize_chirho(
                    scrutinee_chirho,
                    definitions_chirho,
                    locals_chirho,
                )),
                bind_chirho: bind_chirho.clone(),
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: alts_chirho
                    .iter()
                    .map(|alt_chirho| {
                        let mut alt_scope_chirho = scope_chirho.clone();
                        for binder_chirho in &alt_chirho.binders_chirho {
                            alt_scope_chirho.insert(binder_chirho.id_chirho, None);
                        }
                        CoreAltChirho {
                            con_chirho: alt_chirho.con_chirho.clone(),
                            binders_chirho: alt_chirho.binders_chirho.clone(),
                            rhs_chirho: specialize_chirho(
                                &alt_chirho.rhs_chirho,
                                definitions_chirho,
                                &alt_scope_chirho,
                            ),
                        }
                    })
                    .collect(),
            }
        }
        CoreExprChirho::TyLamChirho {
            ty_var_chirho,
            body_chirho,
        } => CoreExprChirho::TyLamChirho {
            ty_var_chirho: ty_var_chirho.clone(),
            body_chirho: Box::new(specialize_chirho(
                body_chirho,
                definitions_chirho,
                locals_chirho,
            )),
        },
        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ty_chirho,
        } => CoreExprChirho::TyAppChirho {
            expr_chirho: Box::new(specialize_chirho(
                inner_chirho,
                definitions_chirho,
                locals_chirho,
            )),
            ty_chirho: ty_chirho.clone(),
        },
        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => CoreExprChirho::PrimOpChirho {
            name_chirho: name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|arg_chirho| specialize_chirho(arg_chirho, definitions_chirho, locals_chirho))
                .collect(),
        },
        CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho,
        } => CoreExprChirho::ConAppChirho {
            con_name_chirho: con_name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|arg_chirho| specialize_chirho(arg_chirho, definitions_chirho, locals_chirho))
                .collect(),
        },
    }
}

/// Specialize proven selector/dictionary pairs without changing calling conventions.
pub fn elide_dicts_chirho(
    expr_chirho: &CoreExprChirho,
    bindings_chirho: &[CoreBindingChirho],
) -> CoreExprChirho {
    let definitions_chirho = bindings_chirho
        .iter()
        .map(|binding_chirho| {
            (
                binding_chirho.binder_chirho.id_chirho,
                &binding_chirho.rhs_chirho,
            )
        })
        .collect();
    specialize_chirho(expr_chirho, &definitions_chirho, &HashMap::new())
}

/// Specialize known evidence, then retain runtime bindings reachable from main.
pub fn elide_dicts_and_filter_chirho(module_chirho: &CoreModuleChirho) -> CoreModuleChirho {
    let definitions_chirho = module_chirho
        .bindings_chirho
        .iter()
        .map(|binding_chirho| {
            (
                binding_chirho.binder_chirho.id_chirho,
                &binding_chirho.rhs_chirho,
            )
        })
        .collect();
    let bindings_chirho: Vec<_> = module_chirho
        .bindings_chirho
        .iter()
        .map(|binding_chirho| {
            let mut rewritten_chirho = binding_chirho.clone();
            rewritten_chirho.rhs_chirho = specialize_chirho(
                &binding_chirho.rhs_chirho,
                &definitions_chirho,
                &HashMap::new(),
            );
            rewritten_chirho
        })
        .collect();
    let indices_chirho: HashMap<_, _> = bindings_chirho
        .iter()
        .enumerate()
        .map(|(index_chirho, binding_chirho)| {
            (binding_chirho.binder_chirho.id_chirho, index_chirho)
        })
        .collect();
    let mut reachable_chirho = HashSet::new();
    let mut pending_chirho = Vec::new();
    for (index_chirho, binding_chirho) in bindings_chirho.iter().enumerate() {
        if binding_chirho.binder_chirho.name_chirho == "main" {
            reachable_chirho.insert(index_chirho);
            pending_chirho.push(index_chirho);
        }
    }
    while let Some(index_chirho) = pending_chirho.pop() {
        for id_chirho in free_vars_chirho(&bindings_chirho[index_chirho].rhs_chirho) {
            if let Some(&dependency_chirho) = indices_chirho.get(&id_chirho)
                && reachable_chirho.insert(dependency_chirho)
            {
                pending_chirho.push(dependency_chirho);
            }
        }
    }
    CoreModuleChirho {
        bindings_chirho: bindings_chirho
            .into_iter()
            .enumerate()
            .filter(|(index_chirho, _)| reachable_chirho.contains(index_chirho))
            .map(|(_, binding_chirho)| binding_chirho)
            .collect(),
        ..module_chirho.clone()
    }
}
