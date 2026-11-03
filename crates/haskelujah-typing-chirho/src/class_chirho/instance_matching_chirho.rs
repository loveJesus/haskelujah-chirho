// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A consistent one-way instance-head match. Every primary and extra head
//! shares one substitution; targets are never instantiated by the match.
//! workflow: language-features-chirho/instance-obligations-chirho

use super::{ClassEnvChirho, InstDeclChirho, PredChirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};
use std::collections::HashMap;

impl ClassEnvChirho {
    /// The instance's context under one consistent substitution over all its
    /// heads. A repeated variable must name the same target type everywhere,
    /// and function arrows must carry the same multiplicity.
    /// workflow: language-features-chirho/instance-obligations-chirho
    pub fn instance_sub_goals_chirho(
        inst_chirho: &InstDeclChirho,
        pred_chirho: &PredChirho,
    ) -> Option<Vec<PredChirho>> {
        if inst_chirho.class_name_chirho != pred_chirho.class_name_chirho
            || inst_chirho.extra_head_tys_chirho.len() != pred_chirho.extra_tys_chirho.len()
        {
            return None;
        }
        let mut bindings_chirho = HashMap::new();
        let patterns_chirho =
            std::iter::once(&inst_chirho.head_ty_chirho).chain(&inst_chirho.extra_head_tys_chirho);
        let targets_chirho =
            std::iter::once(&pred_chirho.ty_chirho).chain(&pred_chirho.extra_tys_chirho);
        for (pattern_chirho, target_chirho) in patterns_chirho.zip(targets_chirho) {
            if !match_consistently_chirho(pattern_chirho, target_chirho, &mut bindings_chirho) {
                return None;
            }
        }
        let mut subst_chirho = SubstChirho::empty_chirho();
        for (var_chirho, target_chirho) in bindings_chirho {
            subst_chirho.insert_chirho(var_chirho, target_chirho);
        }
        Some(
            inst_chirho
                .context_chirho
                .iter()
                .map(|context_chirho| PredChirho {
                    class_name_chirho: context_chirho.class_name_chirho.clone(),
                    ty_chirho: subst_chirho.apply_ty_chirho(&context_chirho.ty_chirho),
                    extra_tys_chirho: context_chirho
                        .extra_tys_chirho
                        .iter()
                        .map(|ty_chirho| subst_chirho.apply_ty_chirho(ty_chirho))
                        .collect(),
                })
                .collect(),
        )
    }
}

/// Check a sub-match against one shared pattern substitution.
/// Alpha-duplicate detection and instance selection share this matcher.
pub(super) fn match_consistently_chirho(
    pattern_chirho: &TyChirho,
    target_chirho: &TyChirho,
    bindings_chirho: &mut HashMap<TyVarChirho, TyChirho>,
) -> bool {
    match (pattern_chirho, target_chirho) {
        (TyChirho::VarChirho(var_chirho), _) => match bindings_chirho.get(var_chirho) {
            Some(bound_chirho) => bound_chirho == target_chirho,
            None => {
                bindings_chirho.insert(*var_chirho, target_chirho.clone());
                true
            }
        },
        (
            TyChirho::FunChirho(pattern_arg_chirho, pattern_res_chirho, pattern_mult_chirho),
            TyChirho::FunChirho(target_arg_chirho, target_res_chirho, target_mult_chirho),
        ) => {
            pattern_mult_chirho == target_mult_chirho
                && match_consistently_chirho(pattern_arg_chirho, target_arg_chirho, bindings_chirho)
                && match_consistently_chirho(pattern_res_chirho, target_res_chirho, bindings_chirho)
        }
        (
            TyChirho::AppChirho(pattern_fun_chirho, pattern_arg_chirho),
            TyChirho::AppChirho(target_fun_chirho, target_arg_chirho),
        ) => {
            match_consistently_chirho(pattern_fun_chirho, target_fun_chirho, bindings_chirho)
                && match_consistently_chirho(pattern_arg_chirho, target_arg_chirho, bindings_chirho)
        }
        (TyChirho::ListChirho(pattern_elem_chirho), TyChirho::ListChirho(target_elem_chirho)) => {
            match_consistently_chirho(pattern_elem_chirho, target_elem_chirho, bindings_chirho)
        }
        (
            TyChirho::ListChirho(pattern_elem_chirho),
            TyChirho::AppChirho(target_fun_chirho, target_elem_chirho),
        ) if matches!(target_fun_chirho.as_ref(), TyChirho::ConChirho(name_chirho) if name_chirho == "[]") => {
            match_consistently_chirho(pattern_elem_chirho, target_elem_chirho, bindings_chirho)
        }
        (
            TyChirho::AppChirho(pattern_fun_chirho, pattern_elem_chirho),
            TyChirho::ListChirho(target_elem_chirho),
        ) if matches!(pattern_fun_chirho.as_ref(), TyChirho::ConChirho(name_chirho) if name_chirho == "[]") => {
            match_consistently_chirho(pattern_elem_chirho, target_elem_chirho, bindings_chirho)
        }
        (
            TyChirho::TupleChirho(pattern_elems_chirho),
            TyChirho::TupleChirho(target_elems_chirho),
        ) if pattern_elems_chirho.len() == target_elems_chirho.len() => pattern_elems_chirho
            .iter()
            .zip(target_elems_chirho)
            .all(|(pattern_elem_chirho, target_elem_chirho)| {
                match_consistently_chirho(pattern_elem_chirho, target_elem_chirho, bindings_chirho)
            }),
        // Constructors, skolems and quantified types match only themselves.
        _ => pattern_chirho == target_chirho,
    }
}

#[cfg(test)]
#[path = "instance_matching_tests_chirho.rs"]
mod tests_chirho;
