// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Demand proofs for saturated calls. Start recursive functions at the greatest
//! demand set and only remove parameters: every terminating branch must demand
//! the argument. Constructor/lambda results demand no payload. Unknown calls and
//! primitives supply no argument proof. The descending finite lattice terminates
//! after at most one removal per function parameter; this is not a runtime guard.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use crate::transform_chirho::children_chirho;
use crate::{CoreExprChirho, CoreIdChirho, CoreModuleChirho};
use std::collections::{HashMap, HashSet};

pub(super) type DemandsChirho = HashMap<CoreIdChirho, Vec<bool>>;

pub(super) fn split_application_chirho(
    mut expression_chirho: CoreExprChirho,
) -> (CoreExprChirho, Vec<CoreExprChirho>) {
    let mut arguments_chirho = Vec::new();
    while let CoreExprChirho::AppChirho {
        fun_chirho,
        arg_chirho,
    } = expression_chirho
    {
        arguments_chirho.push(*arg_chirho);
        expression_chirho = *fun_chirho;
    }
    arguments_chirho.reverse();
    (expression_chirho, arguments_chirho)
}

fn parameters_chirho(
    mut expression_chirho: &CoreExprChirho,
) -> (Vec<CoreIdChirho>, &CoreExprChirho) {
    let mut parameters_chirho = Vec::new();
    loop {
        match expression_chirho {
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                parameters_chirho.push(binder_chirho.id_chirho);
                expression_chirho = body_chirho;
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => expression_chirho = body_chirho,
            _ => return (parameters_chirho, expression_chirho),
        }
    }
}

fn definitions_chirho<'a>(
    expression_chirho: &'a CoreExprChirho,
    definitions_by_id_chirho: &mut HashMap<CoreIdChirho, (Vec<CoreIdChirho>, &'a CoreExprChirho)>,
) {
    if let CoreExprChirho::LetChirho { binds_chirho, .. } = expression_chirho {
        for (binder_chirho, rhs_chirho) in binds_chirho {
            let definition_chirho = parameters_chirho(rhs_chirho);
            if !definition_chirho.0.is_empty() {
                definitions_by_id_chirho.insert(binder_chirho.id_chirho, definition_chirho);
            }
        }
    }
    children_chirho(expression_chirho, &mut |child_chirho| {
        definitions_chirho(child_chirho, definitions_by_id_chirho)
    });
}

pub(super) fn analyze_chirho(module_chirho: &CoreModuleChirho) -> DemandsChirho {
    let mut definitions_by_id_chirho = HashMap::new();
    for binding_chirho in &module_chirho.bindings_chirho {
        let definition_chirho = parameters_chirho(&binding_chirho.rhs_chirho);
        if !definition_chirho.0.is_empty() {
            definitions_by_id_chirho
                .insert(binding_chirho.binder_chirho.id_chirho, definition_chirho);
        }
        definitions_chirho(&binding_chirho.rhs_chirho, &mut definitions_by_id_chirho);
    }
    let mut demands_chirho: DemandsChirho = definitions_by_id_chirho
        .iter()
        .map(|(id_chirho, (parameters_chirho, _))| {
            (*id_chirho, vec![true; parameters_chirho.len()])
        })
        .collect();
    loop {
        let mut changed_chirho = false;
        for (id_chirho, (parameters_chirho, body_chirho)) in &definitions_by_id_chirho {
            let required_chirho = required_chirho(body_chirho, &demands_chirho);
            for (index_chirho, parameter_chirho) in parameters_chirho.iter().enumerate() {
                let previous_chirho = &mut demands_chirho.get_mut(id_chirho).unwrap()[index_chirho];
                if *previous_chirho && !required_chirho.contains(parameter_chirho) {
                    *previous_chirho = false;
                    changed_chirho = true;
                }
            }
        }
        if !changed_chirho {
            return demands_chirho;
        }
    }
}

fn required_chirho(
    expression_chirho: &CoreExprChirho,
    demands_chirho: &DemandsChirho,
) -> HashSet<CoreIdChirho> {
    match expression_chirho {
        CoreExprChirho::VarChirho(id_chirho) => HashSet::from([*id_chirho]),
        CoreExprChirho::LitChirho(_)
        | CoreExprChirho::ConAppChirho { .. }
        | CoreExprChirho::LamChirho { .. } => HashSet::new(),
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            required_chirho(body_chirho, demands_chirho)
        }
        CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
            required_chirho(expr_chirho, demands_chirho)
        }
        CoreExprChirho::AppChirho { .. } => {
            let mut function_chirho = expression_chirho;
            let mut arguments_chirho = Vec::new();
            while let CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } = function_chirho
            {
                arguments_chirho.push(arg_chirho.as_ref());
                function_chirho = fun_chirho;
            }
            arguments_chirho.reverse();
            while let CoreExprChirho::TyAppChirho { expr_chirho, .. } = function_chirho {
                function_chirho = expr_chirho;
            }
            let mut required_set_chirho = required_chirho(function_chirho, demands_chirho);
            if let CoreExprChirho::VarChirho(id_chirho) = function_chirho
                && let Some(mask_chirho) = demands_chirho
                    .get(id_chirho)
                    .filter(|mask_chirho| arguments_chirho.len() >= mask_chirho.len())
            {
                for (argument_chirho, strict_chirho) in arguments_chirho.iter().zip(mask_chirho) {
                    if *strict_chirho {
                        required_set_chirho
                            .extend(required_chirho(argument_chirho, demands_chirho));
                    }
                }
            }
            required_set_chirho
        }
        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => {
            if matches!(
                name_chirho.as_str(),
                "+#" | "-#"
                    | "*#"
                    | "div#"
                    | "mod#"
                    | "quot#"
                    | "rem#"
                    | "negate#"
                    | "==#"
                    | "/=#"
                    | "<#"
                    | "<=#"
                    | ">#"
                    | ">=#"
                    | "force#"
                    | "isHeapObjectChirho#"
            ) {
                args_chirho
                    .iter()
                    .flat_map(|argument_chirho| required_chirho(argument_chirho, demands_chirho))
                    .collect()
            } else {
                HashSet::new()
            }
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            let mut required_set_chirho = required_chirho(scrutinee_chirho, demands_chirho);
            let mut common_chirho: Option<HashSet<CoreIdChirho>> = None;
            for alternative_chirho in alts_chirho {
                let mut branch_chirho =
                    required_chirho(&alternative_chirho.rhs_chirho, demands_chirho);
                branch_chirho.remove(&bind_chirho.id_chirho);
                for binder_chirho in &alternative_chirho.binders_chirho {
                    branch_chirho.remove(&binder_chirho.id_chirho);
                }
                match &mut common_chirho {
                    None => common_chirho = Some(branch_chirho),
                    Some(common_chirho) => {
                        common_chirho.retain(|id_chirho| branch_chirho.contains(id_chirho))
                    }
                }
            }
            required_set_chirho.extend(common_chirho.unwrap_or_default());
            required_set_chirho
        }
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            let mut required_set_chirho = required_chirho(body_chirho, demands_chirho);
            let bindings_chirho: HashMap<_, _> = binds_chirho
                .iter()
                .map(|(binder_chirho, rhs_chirho)| (binder_chirho.id_chirho, rhs_chirho))
                .collect();
            let mut pending_chirho: Vec<_> = required_set_chirho.iter().copied().collect();
            let mut visited_chirho = HashSet::new();
            while let Some(id_chirho) = pending_chirho.pop() {
                if !visited_chirho.insert(id_chirho) {
                    continue;
                }
                if let Some(rhs_chirho) = bindings_chirho.get(&id_chirho) {
                    let dependencies_chirho = required_chirho(rhs_chirho, demands_chirho);
                    pending_chirho.extend(dependencies_chirho.iter().copied());
                    required_set_chirho.extend(dependencies_chirho);
                }
            }
            required_set_chirho.retain(|id_chirho| !bindings_chirho.contains_key(id_chirho));
            required_set_chirho
        }
    }
}
