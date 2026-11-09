// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The ordinary monotype relation under a caller-owned allowance and binding authority.
//! No normalization, forall opening, family decomposition or state publication occurs here.

use super::{UnifyErrorChirho, bind_var_with_kind_policy_chirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};
use haskelujah_span_chirho::SpanChirho;

fn application_chirho(function_chirho: TyChirho, argument_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(Box::new(function_chirho), Box::new(argument_chirho))
}

/// Canonicalize ordinary list/tuple syntax. Arrow structure and multiplicity
/// survive until an unrestricted arrow is compared with its application form.
fn monotype_chirho(
    term_chirho: &TyChirho,
    depth_chirho: usize,
    charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
) -> Result<Option<TyChirho>, ()> {
    charge_chirho(1, 1).ok_or(())?;
    if depth_chirho > 128 {
        return Err(());
    }
    let next_chirho = depth_chirho + 1;
    Ok(Some(match term_chirho {
        TyChirho::VarChirho(variable_chirho) => TyChirho::VarChirho(*variable_chirho),
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            charge_chirho(name_chirho.len(), name_chirho.len()).ok_or(())?;
            term_chirho.clone()
        }
        TyChirho::ForallChirho { .. }
        | TyChirho::RequiredForallChirho { .. }
        | TyChirho::QualChirho { .. } => return Ok(None),
        TyChirho::AppChirho(function_chirho, argument_chirho) => {
            let Some(function_chirho) =
                monotype_chirho(function_chirho, next_chirho, charge_chirho)?
            else {
                return Ok(None);
            };
            let Some(argument_chirho) =
                monotype_chirho(argument_chirho, next_chirho, charge_chirho)?
            else {
                return Ok(None);
            };
            TyChirho::AppChirho(Box::new(function_chirho), Box::new(argument_chirho))
        }
        TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
            let Some(argument_chirho) =
                monotype_chirho(argument_chirho, next_chirho, charge_chirho)?
            else {
                return Ok(None);
            };
            let Some(result_chirho) = monotype_chirho(result_chirho, next_chirho, charge_chirho)?
            else {
                return Ok(None);
            };
            TyChirho::FunChirho(
                Box::new(argument_chirho),
                Box::new(result_chirho),
                *multiplicity_chirho,
            )
        }
        TyChirho::ListChirho(element_chirho) => {
            let Some(element_chirho) = monotype_chirho(element_chirho, next_chirho, charge_chirho)?
            else {
                return Ok(None);
            };
            charge_chirho(2, 2).ok_or(())?;
            application_chirho(TyChirho::ConChirho("[]".into()), element_chirho)
        }
        TyChirho::TupleChirho(elements_chirho) => {
            if elements_chirho.len() == 1 {
                return Ok(None);
            }
            let name_chirho = if elements_chirho.is_empty() {
                "()".to_owned()
            } else {
                charge_chirho(elements_chirho.len(), elements_chirho.len()).ok_or(())?;
                format!("({})", ",".repeat(elements_chirho.len().saturating_sub(1)))
            };
            let mut result_chirho = TyChirho::ConChirho(name_chirho);
            for element_chirho in elements_chirho {
                let Some(element_chirho) =
                    monotype_chirho(element_chirho, next_chirho, charge_chirho)?
                else {
                    return Ok(None);
                };
                charge_chirho(1, 1).ok_or(())?;
                result_chirho = application_chirho(result_chirho, element_chirho);
            }
            result_chirho
        }
    }))
}

/// An exact, nominal monotype equation. The binding callback supplies actual
/// Meta authority; it is not inferred from an ID range or the type's spelling.
/// All pending terms, images and copies share the caller's original budget.
/// workflow: language-features-chirho/fundep-generalization-chirho
pub(crate) fn unify_monomorphic_bounded_chirho(
    left_chirho: &TyChirho,
    right_chirho: &TyChirho,
    may_bind_chirho: &impl Fn(TyVarChirho) -> bool,
    charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
) -> Result<Option<SubstChirho>, ()> {
    let Some(left_chirho) = monotype_chirho(left_chirho, 0, charge_chirho)? else {
        return Ok(None);
    };
    let Some(right_chirho) = monotype_chirho(right_chirho, 0, charge_chirho)? else {
        return Ok(None);
    };
    let mut pending_chirho = vec![(left_chirho, right_chirho)];
    let mut solution_chirho = SubstChirho::empty_chirho();
    while let Some((left_chirho, right_chirho)) = pending_chirho.pop() {
        charge_chirho(1, 0).ok_or(())?;
        let left_chirho = solution_chirho
            .resolve_owned_bounded_chirho(&left_chirho, charge_chirho)
            .ok_or(())?;
        let right_chirho = solution_chirho
            .resolve_owned_bounded_chirho(&right_chirho, charge_chirho)
            .ok_or(())?;
        if left_chirho == right_chirho {
            continue;
        }
        // The same sealed TYPE/CONSTRAINT representation equation used by
        // ordinary and declaration-kind unification retains this operation's
        // actual Meta admission, occurs check and borrowed allowance.
        if let Some((representation_chirho, lifted_chirho)) =
            crate::ty_chirho::representation_chirho::lifted_representation_equation_chirho(
                &left_chirho,
                &right_chirho,
            )
        {
            let representation_chirho = SubstChirho::empty_chirho()
                .apply_bounded_chirho(representation_chirho, charge_chirho)
                .ok_or(())?;
            let lifted_chirho = SubstChirho::empty_chirho()
                .apply_bounded_chirho(lifted_chirho, charge_chirho)
                .ok_or(())?;
            charge_chirho(1, 1).ok_or(())?;
            pending_chirho.push((representation_chirho, lifted_chirho));
            continue;
        }
        let binding_chirho = match (&left_chirho, &right_chirho) {
            (TyChirho::VarChirho(variable_chirho), _) if may_bind_chirho(*variable_chirho) => {
                Some((*variable_chirho, &right_chirho))
            }
            (_, TyChirho::VarChirho(variable_chirho)) if may_bind_chirho(*variable_chirho) => {
                Some((*variable_chirho, &left_chirho))
            }
            _ => None,
        };
        if let Some((variable_chirho, image_chirho)) = binding_chirho {
            // Reserve both ordinary binding copies before their allocation.
            crate::ty_chirho::same_type_contract_chirho(
                image_chirho,
                image_chirho,
                &mut |weight_chirho| charge_chirho(weight_chirho, weight_chirho.checked_mul(2)?),
            )
            .ok_or(())?;
            // The shared ordinary binding primitive checks both monotypes and occurs.
            match bind_var_with_kind_policy_chirho(
                variable_chirho,
                image_chirho,
                SpanChirho::DUMMY_CHIRHO,
                false,
            ) {
                Ok(part_chirho) => {
                    for (variable_chirho, image_chirho) in part_chirho.iter_chirho() {
                        charge_chirho(1, 1).ok_or(())?;
                        solution_chirho.insert_chirho(*variable_chirho, image_chirho.clone());
                    }
                }
                Err(UnifyErrorChirho::ResourceExhaustedChirho { .. }) => return Err(()),
                Err(_) => return Ok(None),
            }
            continue;
        }
        match (left_chirho, right_chirho) {
            (
                TyChirho::FunChirho(argument_chirho, result_chirho, MultChirho::ManyChirho),
                other_chirho @ TyChirho::AppChirho(_, _),
            )
            | (
                other_chirho @ TyChirho::AppChirho(_, _),
                TyChirho::FunChirho(argument_chirho, result_chirho, MultChirho::ManyChirho),
            ) => {
                charge_chirho(4, 4).ok_or(())?;
                let function_chirho = application_chirho(
                    application_chirho(TyChirho::ConChirho("->".into()), *argument_chirho),
                    *result_chirho,
                );
                pending_chirho.push((function_chirho, other_chirho));
            }
            (
                TyChirho::AppChirho(left_function_chirho, left_argument_chirho),
                TyChirho::AppChirho(right_function_chirho, right_argument_chirho),
            ) => {
                charge_chirho(2, 2).ok_or(())?;
                pending_chirho.push((*left_argument_chirho, *right_argument_chirho));
                pending_chirho.push((*left_function_chirho, *right_function_chirho));
            }
            (
                TyChirho::FunChirho(left_argument_chirho, left_result_chirho, left_mult_chirho),
                TyChirho::FunChirho(right_argument_chirho, right_result_chirho, right_mult_chirho),
            ) if left_mult_chirho == right_mult_chirho => {
                charge_chirho(2, 2).ok_or(())?;
                pending_chirho.push((*left_result_chirho, *right_result_chirho));
                pending_chirho.push((*left_argument_chirho, *right_argument_chirho));
            }
            _ => return Ok(None),
        }
    }
    Ok(Some(solution_chirho))
}
