// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A rigid instance method's exact context parameters, retained with its
//! captures. No class-only or superclass lookup supplies a row parameter index.

use super::super::closed_family_apartness_chirho::FamilyHeadRoleChirho;
use super::super::family_registration_chirho::family_head_role_chirho;
use super::{InferCtxChirho, PredChirho, SubstChirho, TyChirho, own_dictionary_key_chirho};
use std::sync::Arc;

fn predicate_chirho(
    ctx_chirho: &InferCtxChirho,
    pred_chirho: &PredChirho,
    subst_chirho: &SubstChirho,
) -> PredChirho {
    PredChirho::from_proof_chirho(
        pred_chirho.class_identity_chirho().clone(),
        crate::class_chirho::PredPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: pred_chirho.class_name_chirho.clone(),
            ty_chirho: ctx_chirho
                .normalize_ty_chirho(&subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho)),
            extra_tys_chirho: pred_chirho
                .extra_tys_chirho
                .iter()
                .map(|ty_chirho| {
                    ctx_chirho.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(ty_chirho))
                })
                .collect(),
        },
    )
}

fn authoritative_rigid_type_chirho(ctx_chirho: &InferCtxChirho, ty_chirho: &TyChirho) -> bool {
    let mut pending_chirho = vec![(ty_chirho, 0_usize)];
    let mut work_chirho = 100_000_usize;
    while let Some((ty_chirho, depth_chirho)) = pending_chirho.pop() {
        if work_chirho == 0 || depth_chirho > 128 {
            return false;
        }
        work_chirho -= 1;
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => {
                if !super::ground_goals_chirho::reserved_constructor_chirho(name_chirho)
                    && !matches!(
                        family_head_role_chirho(&ctx_chirho.type_families_chirho, name_chirho),
                        FamilyHeadRoleChirho::GenerativeChirho { .. }
                            | FamilyHeadRoleChirho::FamilyChirho { .. }
                    )
                {
                    return false;
                }
            }
            TyChirho::ForallVarChirho(_) => {}
            TyChirho::AppChirho(fun_chirho, arg_chirho)
            | TyChirho::FunChirho(fun_chirho, arg_chirho, _) => {
                pending_chirho.push((fun_chirho, depth_chirho + 1));
                pending_chirho.push((arg_chirho, depth_chirho + 1));
            }
            TyChirho::TupleChirho(elements_chirho) => {
                if elements_chirho.len() > 64 {
                    return false;
                }
                pending_chirho.extend(
                    elements_chirho
                        .iter()
                        .map(|ty_chirho| (ty_chirho, depth_chirho + 1)),
                );
            }
            TyChirho::ListChirho(element_chirho) => {
                pending_chirho.push((element_chirho, depth_chirho + 1))
            }
            TyChirho::VarChirho(_)
            | TyChirho::QualChirho { .. }
            | TyChirho::ForallChirho { .. }
            | TyChirho::RequiredForallChirho { .. } => return false,
        }
    }
    true
}

impl InferCtxChirho {
    /// The supplied slice is ONLY the instance declaration's own context, not
    /// additional method predicates. Normalize while its givens are installed.
    pub(in crate::infer_chirho) fn with_instance_context_evidence_chirho<ResultChirho>(
        &mut self,
        context_chirho: &[PredChirho],
        body_chirho: impl FnOnce(&mut Self) -> ResultChirho,
    ) -> ResultChirho {
        let normalized_chirho: Arc<[PredChirho]> = context_chirho
            .iter()
            .map(|pred_chirho| predicate_chirho(self, pred_chirho, &SubstChirho::empty_chirho()))
            .collect();
        let previous_chirho = self
            .active_instance_context_chirho
            .replace(normalized_chirho);
        let result_chirho = body_chirho(self);
        self.active_instance_context_chirho = previous_chirho;
        result_chirho
    }

    /// Return a unique exact parameter, preserving the declaration context's
    /// order. Captures without all class arguments or positive name authority
    /// prove no index. In particular Eq a does not name an Ord a parameter.
    pub(in crate::infer_chirho) fn instance_context_key_chirho(
        &self,
        class_chirho: &str,
        ty_chirho: &TyChirho,
        extras_chirho: &[TyChirho],
        context_chirho: Option<&[PredChirho]>,
        final_subst_chirho: &SubstChirho,
    ) -> Option<String> {
        let context_chirho = context_chirho?;
        let declaration_chirho = self
            .class_env_chirho
            .classes_chirho()
            .get_chirho(class_chirho)?;
        if declaration_chirho.extra_vars_chirho.len() != extras_chirho.len() {
            return None;
        }
        let wanted_chirho = predicate_chirho(
            self,
            &PredChirho::legacy_chirho(crate::class_chirho::PredPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: class_chirho.to_owned(),
                ty_chirho: ty_chirho.clone(),
                extra_tys_chirho: extras_chirho.to_vec(),
            }),
            final_subst_chirho,
        );
        if !std::iter::once(&wanted_chirho.ty_chirho)
            .chain(&wanted_chirho.extra_tys_chirho)
            .all(|ty_chirho| authoritative_rigid_type_chirho(self, ty_chirho))
        {
            return None;
        }
        let mut selected_chirho = None;
        for (index_chirho, given_chirho) in context_chirho.iter().enumerate() {
            if given_chirho.class_name_chirho == class_chirho
                && given_chirho.extra_tys_chirho.len() == extras_chirho.len()
                && predicate_chirho(self, given_chirho, final_subst_chirho) == wanted_chirho
            {
                if selected_chirho.replace(index_chirho).is_some() {
                    return None;
                }
            }
        }
        selected_chirho.map(|index_chirho| own_dictionary_key_chirho(Some(index_chirho)))
    }
}

#[cfg(test)]
#[path = "instance_context_tests_chirho.rs"]
mod tests_chirho;
