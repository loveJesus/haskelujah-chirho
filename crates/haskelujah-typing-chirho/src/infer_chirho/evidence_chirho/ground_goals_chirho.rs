// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Complete closed goals for structural dictionary evidence. Existing string
//! keys remain unchanged; uncertain names never authorize a new dispatch route.

use super::super::closed_family_apartness_chirho::FamilyHeadRoleChirho;
use super::super::family_registration_chirho::family_head_role_chirho;
use super::{InferCtxChirho, PredChirho, ReferenceEvidenceChirho, TyChirho};

const GROUND_GOAL_WORK_CHIRHO: usize = 100_000;
const GROUND_GOAL_DEPTH_CHIRHO: usize = 128;

fn reserved_constructor_chirho(name_chirho: &str) -> bool {
    matches!(name_chirho, "[]" | "()" | "->")
        || name_chirho
            .strip_prefix('(')
            .and_then(|inner_chirho| inner_chirho.strip_suffix(')'))
            .is_some_and(|inner_chirho| {
                !inner_chirho.is_empty()
                    && inner_chirho.len() < 64
                    && inner_chirho.bytes().all(|byte_chirho| byte_chirho == b',')
            })
}

fn closed_authoritative_type_chirho(ctx_chirho: &InferCtxChirho, ty_chirho: &TyChirho) -> bool {
    let mut pending_chirho = vec![(ty_chirho, 0_usize)];
    let mut work_chirho = GROUND_GOAL_WORK_CHIRHO;
    while let Some((ty_chirho, depth_chirho)) = pending_chirho.pop() {
        if work_chirho == 0 || depth_chirho > GROUND_GOAL_DEPTH_CHIRHO {
            return false;
        }
        work_chirho -= 1;
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => {
                if !reserved_constructor_chirho(name_chirho)
                    && !matches!(
                        family_head_role_chirho(&ctx_chirho.type_families_chirho, name_chirho),
                        FamilyHeadRoleChirho::GenerativeChirho { .. }
                    )
                {
                    return false;
                }
            }
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
                        .map(|element_chirho| (element_chirho, depth_chirho + 1)),
                );
            }
            TyChirho::ListChirho(element_chirho) => {
                pending_chirho.push((element_chirho, depth_chirho + 1))
            }
            TyChirho::VarChirho(_)
            | TyChirho::ForallVarChirho(_)
            | TyChirho::ForallChirho { .. }
            | TyChirho::RequiredForallChirho { .. } => return false,
        }
    }
    true
}

/// Callers apply their final substitution first. Normalize every captured
/// argument here, and refuse partial MPTC captures or an unknown class arity.
pub(crate) fn ground_pred_chirho(
    ctx_chirho: &InferCtxChirho,
    class_chirho: &str,
    ty_chirho: &TyChirho,
    extras_chirho: &[TyChirho],
) -> Option<PredChirho> {
    let declaration_chirho = ctx_chirho
        .class_env_chirho
        .classes_chirho
        .get(class_chirho)?;
    if declaration_chirho.extra_vars_chirho.len() != extras_chirho.len() {
        return None;
    }
    let mut arguments_chirho =
        std::iter::once(ty_chirho)
            .chain(extras_chirho)
            .map(|argument_chirho| {
                let normalized_chirho = ctx_chirho.normalize_ty_chirho(argument_chirho);
                (!ctx_chirho.ty_mentions_family_chirho(&normalized_chirho)
                    && closed_authoritative_type_chirho(ctx_chirho, &normalized_chirho))
                .then_some(normalized_chirho)
            });
    let ty_chirho = arguments_chirho.next()??;
    let extra_tys_chirho = arguments_chirho.collect::<Option<Vec<_>>>()?;
    Some(PredChirho {
        class_name_chirho: class_chirho.to_owned(),
        ty_chirho,
        extra_tys_chirho,
    })
}

/// Structural disagreement suppresses only the new structural goal. Legacy
/// class/key consensus keeps its existing meaning and occurrence ownership.
pub(super) fn agreeing_reference_goals_chirho<KeyChirho: std::hash::Hash + Eq>(
    captures_chirho: std::collections::HashMap<KeyChirho, Vec<Vec<ReferenceEvidenceChirho>>>,
) -> std::collections::HashMap<KeyChirho, Vec<ReferenceEvidenceChirho>> {
    captures_chirho
        .into_iter()
        .filter_map(|(key_chirho, mut captures_chirho)| {
            let mut first_chirho = captures_chirho.pop()?;
            for capture_chirho in captures_chirho {
                if first_chirho.len() != capture_chirho.len() {
                    return None;
                }
                for (first_chirho, next_chirho) in first_chirho.iter_mut().zip(capture_chirho) {
                    if first_chirho.class_name_chirho != next_chirho.class_name_chirho
                        || first_chirho.ty_key_chirho != next_chirho.ty_key_chirho
                    {
                        return None;
                    }
                    if first_chirho.ground_pred_chirho != next_chirho.ground_pred_chirho {
                        first_chirho.ground_pred_chirho = None;
                    }
                }
            }
            Some((key_chirho, first_chirho))
        })
        .collect()
}

#[cfg(test)]
#[path = "ground_goals_tests_chirho.rs"]
mod tests_chirho;
