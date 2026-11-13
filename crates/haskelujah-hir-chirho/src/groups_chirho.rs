// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Deterministic dependency-first SCCs over resolved HIR identities.
//! Retains legacy dependency ordering, with an iterative Tarjan walk instead of recursive DFS.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use crate::{BindingChirho, ExprKindChirho};
use std::collections::{BTreeSet, HashMap};

pub fn binding_groups_chirho(bindings_chirho: &[BindingChirho]) -> Vec<Vec<usize>> {
    let indices_chirho: HashMap<_, _> = bindings_chirho
        .iter()
        .enumerate()
        .map(|(index_chirho, binding_chirho)| (binding_chirho.name_chirho, index_chirho))
        .collect();
    let mut edges_chirho = vec![Vec::new(); bindings_chirho.len()];
    for (index_chirho, binding_chirho) in bindings_chirho.iter().enumerate() {
        let mut references_chirho = BTreeSet::new();
        let mut work_chirho = vec![&binding_chirho.body_chirho];
        while let Some(expression_chirho) = work_chirho.pop() {
            match &expression_chirho.kind_chirho {
                ExprKindChirho::VarChirho(name_chirho) => {
                    if let Some(target_chirho) = indices_chirho.get(name_chirho)
                        && bindings_chirho[*target_chirho].signature_chirho.is_none()
                    {
                        references_chirho.insert(*target_chirho);
                    }
                }
                ExprKindChirho::AppChirho(function_chirho, argument_chirho) => {
                    work_chirho.push(function_chirho);
                    work_chirho.push(argument_chirho);
                }
                ExprKindChirho::LamChirho(_, body_chirho)
                | ExprKindChirho::AnnChirho(body_chirho, _) => work_chirho.push(body_chirho),
                ExprKindChirho::LetChirho(locals_chirho, body_chirho) => {
                    work_chirho.extend(
                        locals_chirho
                            .iter()
                            .map(|local_chirho| &local_chirho.body_chirho),
                    );
                    work_chirho.push(body_chirho);
                }
                ExprKindChirho::IfChirho(condition_chirho, yes_chirho, no_chirho) => {
                    work_chirho.extend([
                        condition_chirho.as_ref(),
                        yes_chirho.as_ref(),
                        no_chirho.as_ref(),
                    ]);
                }
                ExprKindChirho::ListChirho(items_chirho)
                | ExprKindChirho::TupleChirho(items_chirho) => work_chirho.extend(items_chirho),
                ExprKindChirho::MatchChirho(inputs_chirho, arms_chirho) => {
                    work_chirho.extend(inputs_chirho);
                    work_chirho
                        .extend(arms_chirho.iter().map(|arm_chirho| &arm_chirho.body_chirho));
                }
                ExprKindChirho::CharChirho(_)
                | ExprKindChirho::IntegerChirho(_)
                | ExprKindChirho::StringChirho(_) => {}
            }
        }
        edges_chirho[index_chirho].extend(references_chirho);
    }
    components_chirho(&edges_chirho)
}

fn components_chirho(edges_chirho: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let count_chirho = edges_chirho.len();
    let mut indices_chirho = vec![None; count_chirho];
    let mut low_chirho = vec![0; count_chirho];
    let mut live_chirho = vec![false; count_chirho];
    let mut stack_chirho = Vec::new();
    let mut groups_chirho = Vec::new();
    let mut cursor_chirho = 0;
    for root_chirho in 0..count_chirho {
        if indices_chirho[root_chirho].is_some() {
            continue;
        }
        let mut frames_chirho = vec![(root_chirho, 0)];
        indices_chirho[root_chirho] = Some(cursor_chirho);
        low_chirho[root_chirho] = cursor_chirho;
        cursor_chirho += 1;
        live_chirho[root_chirho] = true;
        stack_chirho.push(root_chirho);
        while let Some((node_chirho, next_chirho)) = frames_chirho.last_mut() {
            let node_chirho = *node_chirho;
            if let Some(&dependency_chirho) = edges_chirho[node_chirho].get(*next_chirho) {
                *next_chirho += 1;
                if indices_chirho[dependency_chirho].is_none() {
                    indices_chirho[dependency_chirho] = Some(cursor_chirho);
                    low_chirho[dependency_chirho] = cursor_chirho;
                    cursor_chirho += 1;
                    live_chirho[dependency_chirho] = true;
                    stack_chirho.push(dependency_chirho);
                    frames_chirho.push((dependency_chirho, 0));
                } else if live_chirho[dependency_chirho] {
                    low_chirho[node_chirho] =
                        low_chirho[node_chirho].min(indices_chirho[dependency_chirho].unwrap());
                }
            } else {
                frames_chirho.pop();
                if low_chirho[node_chirho] == indices_chirho[node_chirho].unwrap() {
                    let mut group_chirho = Vec::new();
                    loop {
                        let member_chirho = stack_chirho.pop().unwrap();
                        live_chirho[member_chirho] = false;
                        group_chirho.push(member_chirho);
                        if member_chirho == node_chirho {
                            break;
                        }
                    }
                    group_chirho.sort_unstable();
                    groups_chirho.push(group_chirho);
                }
                if let Some((parent_chirho, _)) = frames_chirho.last() {
                    low_chirho[*parent_chirho] =
                        low_chirho[*parent_chirho].min(low_chirho[node_chirho]);
                }
            }
        }
    }
    groups_chirho
}
