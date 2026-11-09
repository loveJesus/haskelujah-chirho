// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type families: registration, equation lookup and reduction, the built-in type-level literal
//! families among them.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::equality_normalization_chirho::NormalizationChangeChirho;
use super::family_definitions_chirho::{IntrinsicTypeFamilyChirho, TypeFamilyReductionChirho};
use super::family_reduction_chirho::{
    FamilyReductionDecisionChirho, FamilyReductionLimitsChirho, MatchFailureChirho,
    decide_family_with_budget_chirho,
};
use super::family_registration_chirho::family_head_role_chirho;
use super::strip_name_qualifier_chirho;
use crate::ty_chirho::TyChirho;
use haskelujah_ast_chirho::decl_chirho::GadtProjectionLimitChirho as MatchLimitChirho;
use haskelujah_ast_chirho::lit_chirho::{
    parse_haskell_char_body_chirho, render_haskell_char_body_chirho,
};
use haskelujah_ast_chirho::promotion_chirho::{PromotedKeyChirho, is_promoted_key_chirho};
use std::collections::HashMap;

impl InferCtxChirho {
    /// Select exactly one resolved family; a blocked/empty source family never borrows
    /// another identity's equations or the compiler's intrinsic implementation.
    pub fn reduce_type_family_chirho(
        &self,
        name_chirho: &str,
        arguments_chirho: &[TyChirho],
    ) -> Option<TyChirho> {
        self.reduce_family_with_budget_chirho(name_chirho, arguments_chirho, 100_000, 16_384)
    }

    fn reduce_family_with_budget_chirho(
        &self,
        name_chirho: &str,
        arguments_chirho: &[TyChirho],
        steps_chirho: usize,
        nodes_chirho: usize,
    ) -> Option<TyChirho> {
        let mut work_chirho = steps_chirho;
        let mut output_chirho = nodes_chirho;
        reduce_family_in_context_chirho(
            self,
            name_chirho,
            arguments_chirho,
            &mut work_chirho,
            &mut output_chirho,
        )
        .flatten()
    }

    /// One budget covers the complete normalization tree, including nested redexes.
    /// Ordinary family attempts settle consumed matching work and output, including
    /// partial failures, against the same continuation allowance. Intrinsics retain
    /// their explicit legacy reservation. The output/clone allowance also counts
    /// name bytes. Exhaustion conservatively
    /// restores the original stuck input instead of expanding siblings forever.
    pub fn reduce_type_families_in_ty_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        let mut work_chirho = 10_000_000usize;
        let mut output_chirho = 16_384usize;
        self.reduce_families_bounded_chirho(ty_chirho, 0, &mut work_chirho, &mut output_chirho)
            .or_else(|| clone_stuck_type_chirho(ty_chirho, 0, &mut 16_384usize))
            .unwrap_or_else(|| {
                TyChirho::ConChirho("%family:unresolved:normalization-resource-bound".to_owned())
            })
    }

    pub(super) fn reduce_families_bounded_chirho(
        &self,
        ty_chirho: &TyChirho,
        depth_chirho: usize,
        work_chirho: &mut usize,
        output_chirho: &mut usize,
    ) -> Option<TyChirho> {
        reduce_families_with_context_chirho(
            self,
            ty_chirho,
            depth_chirho,
            work_chirho,
            output_chirho,
        )
    }
}

fn clone_stuck_type_chirho(
    ty_chirho: &TyChirho,
    depth_chirho: usize,
    nodes_chirho: &mut usize,
) -> Option<TyChirho> {
    if depth_chirho > 128 {
        return None;
    }
    *nodes_chirho = nodes_chirho.checked_sub(1)?;
    let child_chirho = depth_chirho.checked_add(1)?;
    Some(match ty_chirho {
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => TyChirho::QualChirho {
            context_chirho: context_chirho
                .iter()
                .map(|constraint_chirho| {
                    clone_stuck_type_chirho(constraint_chirho, child_chirho, nodes_chirho)
                })
                .collect::<Option<Vec<_>>>()?,
            body_chirho: Box::new(clone_stuck_type_chirho(
                body_chirho,
                child_chirho,
                nodes_chirho,
            )?),
        },
        TyChirho::VarChirho(variable_chirho) => TyChirho::VarChirho(*variable_chirho),
        TyChirho::ForallVarChirho(name_chirho) => {
            *nodes_chirho = nodes_chirho.checked_sub(name_chirho.len())?;
            TyChirho::ForallVarChirho(name_chirho.clone())
        }
        TyChirho::ConChirho(name_chirho) => {
            *nodes_chirho = nodes_chirho.checked_sub(name_chirho.len())?;
            TyChirho::ConChirho(name_chirho.clone())
        }
        TyChirho::AppChirho(function_chirho, argument_chirho) => TyChirho::AppChirho(
            Box::new(clone_stuck_type_chirho(
                function_chirho,
                child_chirho,
                nodes_chirho,
            )?),
            Box::new(clone_stuck_type_chirho(
                argument_chirho,
                child_chirho,
                nodes_chirho,
            )?),
        ),
        TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
            TyChirho::FunChirho(
                Box::new(clone_stuck_type_chirho(
                    argument_chirho,
                    child_chirho,
                    nodes_chirho,
                )?),
                Box::new(clone_stuck_type_chirho(
                    result_chirho,
                    child_chirho,
                    nodes_chirho,
                )?),
                *multiplicity_chirho,
            )
        }
        TyChirho::ListChirho(element_chirho) => TyChirho::ListChirho(Box::new(
            clone_stuck_type_chirho(element_chirho, child_chirho, nodes_chirho)?,
        )),
        TyChirho::TupleChirho(elements_chirho) => TyChirho::TupleChirho(
            elements_chirho
                .iter()
                .map(|element_chirho| {
                    clone_stuck_type_chirho(element_chirho, child_chirho, nodes_chirho)
                })
                .collect::<Option<Vec<_>>>()?,
        ),
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            *nodes_chirho = nodes_chirho.checked_sub(vars_chirho.len().saturating_mul(4))?;
            TyChirho::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(clone_stuck_type_chirho(
                    body_chirho,
                    child_chirho,
                    nodes_chirho,
                )?),
            }
        }
        TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            *nodes_chirho = nodes_chirho.checked_sub(vars_chirho.len().saturating_mul(4))?;
            TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(clone_stuck_type_chirho(
                    body_chirho,
                    child_chirho,
                    nodes_chirho,
                )?),
            }
        }
    })
}

fn nat_type_literal_chirho(ty_chirho: &TyChirho) -> Option<u128> {
    match ty_chirho {
        TyChirho::ConChirho(name_chirho)
            if !name_chirho.is_empty()
                && name_chirho
                    .chars()
                    .all(|char_chirho| char_chirho.is_ascii_digit()) =>
        {
            name_chirho.parse::<u128>().ok()
        }
        _ => None,
    }
}

fn reserve_intrinsic_name_chirho(
    length_chirho: usize,
    work_chirho: &mut usize,
    output_chirho: &mut usize,
) -> Option<()> {
    let size_chirho = length_chirho.checked_add(1)?;
    *work_chirho = work_chirho.checked_sub(size_chirho)?;
    *output_chirho = output_chirho.checked_sub(size_chirho)?;
    Some(())
}

fn nat_type_con_chirho(
    mut value_chirho: u128,
    work_chirho: &mut usize,
    output_chirho: &mut usize,
) -> Option<TyChirho> {
    // Build the actual decimal payload in a fixed stack buffer, then debit
    // its exact selected bytes before allocating the constructor name.
    let mut digits_chirho = [0_u8; 39];
    let mut start_chirho = digits_chirho.len();
    loop {
        *work_chirho = work_chirho.checked_sub(1)?;
        start_chirho = start_chirho.checked_sub(1)?;
        digits_chirho[start_chirho] = b'0' + (value_chirho % 10) as u8;
        value_chirho /= 10;
        if value_chirho == 0 {
            break;
        }
    }
    let bytes_chirho = &digits_chirho[start_chirho..];
    reserve_intrinsic_name_chirho(bytes_chirho.len(), work_chirho, output_chirho)?;
    Some(TyChirho::ConChirho(
        std::str::from_utf8(bytes_chirho).ok()?.to_owned(),
    ))
}

/// The compiler's promoted `True` or `False`: retain the actual producer key
/// when the module is keyed; debit before copying it into the AST result.
fn bool_type_con_chirho(
    value_chirho: bool,
    keyed_chirho: bool,
    work_chirho: &mut usize,
    output_chirho: &mut usize,
) -> Option<TyChirho> {
    let key_chirho = keyed_chirho.then(|| PromotedKeyChirho::builtin_bool_chirho(value_chirho));
    let name_chirho = match &key_chirho {
        Some(key_chirho) => key_chirho.as_str_chirho(),
        None => {
            if value_chirho {
                "True"
            } else {
                "False"
            }
        }
    };
    reserve_intrinsic_name_chirho(name_chirho.len(), work_chirho, output_chirho)?;
    Some(TyChirho::ConChirho(name_chirho.to_owned()))
}

fn bool_type_literal_chirho(ty_chirho: &TyChirho, keyed_chirho: bool) -> Option<bool> {
    let TyChirho::ConChirho(name_chirho) = ty_chirho else {
        return None;
    };
    // A key is opaque: compare it whole before any `.` is read as a qualifier.
    if is_promoted_key_chirho(name_chirho) {
        return [true, false].into_iter().find(|value_chirho| {
            PromotedKeyChirho::builtin_bool_chirho(*value_chirho).as_str_chirho() == name_chirho
        });
    }
    if keyed_chirho {
        return None;
    }
    match strip_name_qualifier_chirho(name_chirho).trim_start_matches('\'') {
        "True" => Some(true),
        "False" => Some(false),
        _ => None,
    }
}

fn symbol_type_literal_chirho(ty_chirho: &TyChirho) -> Option<&str> {
    match ty_chirho {
        TyChirho::ConChirho(name_chirho)
            if name_chirho.len() >= 2
                && name_chirho.starts_with('"')
                && name_chirho.ends_with('"') =>
        {
            Some(&name_chirho[1..name_chirho.len() - 1])
        }
        _ => None,
    }
}

fn symbol_type_con_chirho(
    left_chirho: &str,
    right_chirho: &str,
    work_chirho: &mut usize,
    output_chirho: &mut usize,
) -> Option<TyChirho> {
    let mut length_chirho = 2_usize;
    for character_chirho in left_chirho.chars().chain(right_chirho.chars()) {
        *work_chirho = work_chirho.checked_sub(character_chirho.len_utf8())?;
        length_chirho = length_chirho.checked_add(character_chirho.len_utf8())?;
        if matches!(character_chirho, '\\' | '"') {
            length_chirho = length_chirho.checked_add(1)?;
        }
    }
    reserve_intrinsic_name_chirho(length_chirho, work_chirho, output_chirho)?;
    let mut name_chirho = String::with_capacity(length_chirho);
    name_chirho.push('"');
    for character_chirho in left_chirho.chars().chain(right_chirho.chars()) {
        if matches!(character_chirho, '\\' | '"') {
            name_chirho.push('\\');
        }
        name_chirho.push(character_chirho);
    }
    name_chirho.push('"');
    Some(TyChirho::ConChirho(name_chirho))
}

fn char_type_literal_chirho(ty_chirho: &TyChirho) -> Option<char> {
    let TyChirho::ConChirho(name_chirho) = ty_chirho else {
        return None;
    };
    let body_chirho = name_chirho.strip_prefix('\'')?.strip_suffix('\'')?;
    parse_haskell_char_body_chirho(body_chirho)
}

fn char_type_con_chirho(
    value_chirho: char,
    work_chirho: &mut usize,
    output_chirho: &mut usize,
) -> Option<TyChirho> {
    // A single character's renderer has a fixed-size payload. Reuse its actual
    // escaped body instead of independently estimating its spelling.
    let body_chirho = render_haskell_char_body_chirho(value_chirho);
    let length_chirho = body_chirho.len().checked_add(2)?;
    reserve_intrinsic_name_chirho(length_chirho, work_chirho, output_chirho)?;
    let mut name_chirho = String::with_capacity(length_chirho);
    name_chirho.push('\'');
    name_chirho.push_str(&body_chirho);
    name_chirho.push('\'');
    Some(TyChirho::ConChirho(name_chirho))
}

fn copy_intrinsic_type_chirho(
    ty_chirho: &TyChirho,
    work_chirho: &mut usize,
    output_chirho: &mut usize,
) -> Option<TyChirho> {
    super::equality_normalization_chirho::copy_term_with_remaining_budget_chirho(
        ty_chirho,
        work_chirho,
        output_chirho,
    )
}

fn apply_extra_type_family_args_chirho(
    mut reduced_chirho: TyChirho,
    args_chirho: &[TyChirho],
    consumed_chirho: usize,
    work_chirho: &mut usize,
    output_chirho: &mut usize,
) -> Option<TyChirho> {
    for extra_arg_chirho in &args_chirho[consumed_chirho..] {
        let argument_chirho =
            copy_intrinsic_type_chirho(extra_arg_chirho, work_chirho, output_chirho)?;
        *work_chirho = work_chirho.checked_sub(1)?;
        *output_chirho = output_chirho.checked_sub(1)?;
        reduced_chirho = TyChirho::AppChirho(Box::new(reduced_chirho), Box::new(argument_chirho));
    }
    Some(reduced_chirho)
}

/// Outer None is resource exhaustion; inner None is the original stuck result.
/// Every selected name/clone/App is charged BEFORE its AST materialization.
fn reduce_builtin_type_family_application_chirho(
    intrinsic_chirho: IntrinsicTypeFamilyChirho,
    args_chirho: &[TyChirho],
    keyed_chirho: bool,
    work_chirho: &mut usize,
    output_chirho: &mut usize,
) -> Option<Option<TyChirho>> {
    let (reduced_chirho, consumed_chirho) = match (intrinsic_chirho, args_chirho) {
        // workflow: language-features-chirho/type-level-character-families-chirho
        (IntrinsicTypeFamilyChirho::CharToNatChirho, [value_chirho, ..]) => {
            let Some(value_chirho) = char_type_literal_chirho(value_chirho) else {
                return Some(None);
            };
            (
                nat_type_con_chirho(u128::from(value_chirho as u32), work_chirho, output_chirho)?,
                1,
            )
        }
        (IntrinsicTypeFamilyChirho::NatToCharChirho, [value_chirho, ..]) => {
            let Some(value_chirho) = nat_type_literal_chirho(value_chirho)
                .and_then(|value_chirho| u32::try_from(value_chirho).ok())
                .and_then(char::from_u32)
            else {
                return Some(None);
            };
            (
                char_type_con_chirho(value_chirho, work_chirho, output_chirho)?,
                1,
            )
        }
        (IntrinsicTypeFamilyChirho::AppendSymbolChirho, [left_chirho, right_chirho, ..]) => {
            let image_chirho = match (
                symbol_type_literal_chirho(left_chirho),
                symbol_type_literal_chirho(right_chirho),
            ) {
                (Some(""), _) => {
                    copy_intrinsic_type_chirho(right_chirho, work_chirho, output_chirho)?
                }
                (_, Some("")) => {
                    copy_intrinsic_type_chirho(left_chirho, work_chirho, output_chirho)?
                }
                (Some(left_value_chirho), Some(right_value_chirho)) => symbol_type_con_chirho(
                    left_value_chirho,
                    right_value_chirho,
                    work_chirho,
                    output_chirho,
                )?,
                _ => return Some(None),
            };
            (image_chirho, 2)
        }
        (IntrinsicTypeFamilyChirho::AddNatChirho, [left_chirho, right_chirho, ..]) => {
            let image_chirho = match (
                nat_type_literal_chirho(left_chirho),
                nat_type_literal_chirho(right_chirho),
            ) {
                (Some(0), _) => {
                    copy_intrinsic_type_chirho(right_chirho, work_chirho, output_chirho)?
                }
                (_, Some(0)) => {
                    copy_intrinsic_type_chirho(left_chirho, work_chirho, output_chirho)?
                }
                (Some(left_value_chirho), Some(right_value_chirho)) => {
                    let Some(value_chirho) = left_value_chirho.checked_add(right_value_chirho)
                    else {
                        return Some(None);
                    };
                    nat_type_con_chirho(value_chirho, work_chirho, output_chirho)?
                }
                _ => return Some(None),
            };
            (image_chirho, 2)
        }
        (IntrinsicTypeFamilyChirho::MultiplyNatChirho, [left_chirho, right_chirho, ..]) => {
            let image_chirho = match (
                nat_type_literal_chirho(left_chirho),
                nat_type_literal_chirho(right_chirho),
            ) {
                (Some(0), _) | (_, Some(0)) => nat_type_con_chirho(0, work_chirho, output_chirho)?,
                (Some(1), _) => {
                    copy_intrinsic_type_chirho(right_chirho, work_chirho, output_chirho)?
                }
                (_, Some(1)) => {
                    copy_intrinsic_type_chirho(left_chirho, work_chirho, output_chirho)?
                }
                (Some(left_value_chirho), Some(right_value_chirho)) => {
                    let Some(value_chirho) = left_value_chirho.checked_mul(right_value_chirho)
                    else {
                        return Some(None);
                    };
                    nat_type_con_chirho(value_chirho, work_chirho, output_chirho)?
                }
                _ => return Some(None),
            };
            (image_chirho, 2)
        }
        (IntrinsicTypeFamilyChirho::PowerNatChirho, [left_chirho, right_chirho, ..]) => {
            let image_chirho = match (
                nat_type_literal_chirho(left_chirho),
                nat_type_literal_chirho(right_chirho),
            ) {
                (_, Some(0)) | (Some(1), _) => nat_type_con_chirho(1, work_chirho, output_chirho)?,
                (_, Some(1)) => {
                    copy_intrinsic_type_chirho(left_chirho, work_chirho, output_chirho)?
                }
                (Some(left_value_chirho), Some(right_value_chirho)) => {
                    let Some(value_chirho) = u32::try_from(right_value_chirho)
                        .ok()
                        .and_then(|exponent_chirho| left_value_chirho.checked_pow(exponent_chirho))
                    else {
                        return Some(None);
                    };
                    nat_type_con_chirho(value_chirho, work_chirho, output_chirho)?
                }
                _ => return Some(None),
            };
            (image_chirho, 2)
        }
        (IntrinsicTypeFamilyChirho::SubtractNatChirho, [left_chirho, right_chirho, ..]) => {
            let image_chirho = match (
                nat_type_literal_chirho(left_chirho),
                nat_type_literal_chirho(right_chirho),
            ) {
                (_, Some(0)) => {
                    copy_intrinsic_type_chirho(left_chirho, work_chirho, output_chirho)?
                }
                (Some(left_value_chirho), Some(right_value_chirho))
                    if left_value_chirho >= right_value_chirho =>
                {
                    nat_type_con_chirho(
                        left_value_chirho - right_value_chirho,
                        work_chirho,
                        output_chirho,
                    )?
                }
                _ => return Some(None),
            };
            (image_chirho, 2)
        }
        (IntrinsicTypeFamilyChirho::DivideNatChirho, [left_chirho, right_chirho, ..]) => {
            let image_chirho = match (
                nat_type_literal_chirho(left_chirho),
                nat_type_literal_chirho(right_chirho),
            ) {
                (_, Some(1)) => {
                    copy_intrinsic_type_chirho(left_chirho, work_chirho, output_chirho)?
                }
                (Some(left_value_chirho), Some(right_value_chirho)) if right_value_chirho != 0 => {
                    nat_type_con_chirho(
                        left_value_chirho / right_value_chirho,
                        work_chirho,
                        output_chirho,
                    )?
                }
                _ => return Some(None),
            };
            (image_chirho, 2)
        }
        (IntrinsicTypeFamilyChirho::ModuloNatChirho, [left_chirho, right_chirho, ..]) => {
            let image_chirho = match (
                nat_type_literal_chirho(left_chirho),
                nat_type_literal_chirho(right_chirho),
            ) {
                (_, Some(1)) => nat_type_con_chirho(0, work_chirho, output_chirho)?,
                (Some(left_value_chirho), Some(right_value_chirho)) if right_value_chirho != 0 => {
                    nat_type_con_chirho(
                        left_value_chirho % right_value_chirho,
                        work_chirho,
                        output_chirho,
                    )?
                }
                _ => return Some(None),
            };
            (image_chirho, 2)
        }
        (IntrinsicTypeFamilyChirho::Log2NatChirho, [value_chirho, ..]) => {
            let Some(value_chirho) =
                nat_type_literal_chirho(value_chirho).filter(|value_chirho| *value_chirho != 0)
            else {
                return Some(None);
            };
            (
                nat_type_con_chirho(
                    127 - value_chirho.leading_zeros() as u128,
                    work_chirho,
                    output_chirho,
                )?,
                1,
            )
        }
        (IntrinsicTypeFamilyChirho::LessEqualNatChirho, [left_chirho, right_chirho, ..]) => {
            let value_chirho = if left_chirho == right_chirho {
                true
            } else {
                let (Some(left_value_chirho), Some(right_value_chirho)) = (
                    nat_type_literal_chirho(left_chirho),
                    nat_type_literal_chirho(right_chirho),
                ) else {
                    return Some(None);
                };
                left_value_chirho <= right_value_chirho
            };
            (
                bool_type_con_chirho(value_chirho, keyed_chirho, work_chirho, output_chirho)?,
                2,
            )
        }
        (
            IntrinsicTypeFamilyChirho::IfChirho,
            [condition_chirho, true_chirho, false_chirho, ..],
        ) => {
            let Some(condition_chirho) = bool_type_literal_chirho(condition_chirho, keyed_chirho)
            else {
                return Some(None);
            };
            (
                copy_intrinsic_type_chirho(
                    if condition_chirho {
                        true_chirho
                    } else {
                        false_chirho
                    },
                    work_chirho,
                    output_chirho,
                )?,
                3,
            )
        }
        (IntrinsicTypeFamilyChirho::NotChirho, [value_chirho, ..]) => {
            let Some(value_chirho) = bool_type_literal_chirho(value_chirho, keyed_chirho) else {
                return Some(None);
            };
            (
                bool_type_con_chirho(!value_chirho, keyed_chirho, work_chirho, output_chirho)?,
                1,
            )
        }
        (IntrinsicTypeFamilyChirho::AndChirho, [left_chirho, right_chirho, ..]) => {
            let Some(left_chirho) = bool_type_literal_chirho(left_chirho, keyed_chirho) else {
                return Some(None);
            };
            (
                if left_chirho {
                    copy_intrinsic_type_chirho(right_chirho, work_chirho, output_chirho)?
                } else {
                    bool_type_con_chirho(false, keyed_chirho, work_chirho, output_chirho)?
                },
                2,
            )
        }
        (IntrinsicTypeFamilyChirho::OrChirho, [left_chirho, right_chirho, ..]) => {
            let Some(left_chirho) = bool_type_literal_chirho(left_chirho, keyed_chirho) else {
                return Some(None);
            };
            (
                if left_chirho {
                    bool_type_con_chirho(true, keyed_chirho, work_chirho, output_chirho)?
                } else {
                    copy_intrinsic_type_chirho(right_chirho, work_chirho, output_chirho)?
                },
                2,
            )
        }
        _ => return Some(None),
    };
    Some(Some(apply_extra_type_family_args_chirho(
        reduced_chirho,
        args_chirho,
        consumed_chirho,
        work_chirho,
        output_chirho,
    )?))
}

pub(super) fn substitute_type_vars_chirho(
    ty_chirho: &TyChirho,
    bindings_chirho: &HashMap<String, TyChirho>,
) -> TyChirho {
    match ty_chirho {
        TyChirho::VarChirho(tv_chirho) => {
            let var_name_chirho = format!("tv{}", tv_chirho.0);
            bindings_chirho
                .get(&var_name_chirho)
                .cloned()
                .unwrap_or_else(|| ty_chirho.clone())
        }
        TyChirho::ForallVarChirho(name_chirho) => bindings_chirho
            .get(name_chirho)
            .cloned()
            .unwrap_or_else(|| ty_chirho.clone()),
        TyChirho::AppChirho(f_chirho, a_chirho) => TyChirho::AppChirho(
            Box::new(substitute_type_vars_chirho(f_chirho, bindings_chirho)),
            Box::new(substitute_type_vars_chirho(a_chirho, bindings_chirho)),
        ),
        TyChirho::FunChirho(a_chirho, r_chirho, mult_chirho) => TyChirho::FunChirho(
            Box::new(substitute_type_vars_chirho(a_chirho, bindings_chirho)),
            Box::new(substitute_type_vars_chirho(r_chirho, bindings_chirho)),
            *mult_chirho,
        ),
        TyChirho::ListChirho(inner_chirho) => TyChirho::ListChirho(Box::new(
            substitute_type_vars_chirho(inner_chirho, bindings_chirho),
        )),
        TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
            elems_chirho
                .iter()
                .map(|e_chirho| substitute_type_vars_chirho(e_chirho, bindings_chirho))
                .collect(),
        ),
        _ => ty_chirho.clone(),
    }
}

fn reduce_family_in_context_chirho(
    context_chirho: &dyn super::equality_normalization_chirho::NormalizationContextChirho,
    name_chirho: &str,
    arguments_chirho: &[TyChirho],
    work_chirho: &mut usize,
    output_chirho: &mut usize,
) -> Option<Option<TyChirho>> {
    let Some(key_chirho) = context_chirho.family_name_key_chirho(name_chirho) else {
        return Some(None);
    };
    let Some(definition_chirho) = context_chirho
        .catalogue_chirho()
        .definition_chirho(&key_chirho)
    else {
        return Some(None);
    };
    if let TypeFamilyReductionChirho::IntrinsicChirho(intrinsic_chirho) =
        definition_chirho.reduction_chirho()
    {
        // Retain the intrinsic work reservation and debit the selected builder's
        // actual output before it allocates. Owned reuse cannot erase that cost.
        let reservation_chirho = 100_000;
        *work_chirho = work_chirho.checked_sub(reservation_chirho)?;
        // The builder spends this already-debited reservation, not a second
        // loan from the remaining outer work. Its unused part is never refunded.
        let mut reserved_work_chirho = reservation_chirho;
        return reduce_builtin_type_family_application_chirho(
            intrinsic_chirho,
            arguments_chirho,
            context_chirho.promotion_present_chirho(),
            &mut reserved_work_chirho,
            output_chirho,
        );
    }
    *work_chirho = work_chirho.checked_sub(name_chirho.len().checked_add(1)?)?;
    let initial_work_chirho = *work_chirho;
    let loan_work_chirho = initial_work_chirho.min(100_000);
    let initial_output_chirho = *output_chirho;
    let mut limits_chirho = FamilyReductionLimitsChirho {
        steps_chirho: loan_work_chirho,
        nodes_chirho: initial_output_chirho,
        depth_chirho: 128,
    };
    let result_chirho = decide_family_with_budget_chirho(
        definition_chirho,
        arguments_chirho,
        &|head_chirho| family_head_role_chirho(context_chirho.catalogue_chirho(), head_chirho),
        &mut limits_chirho,
    );
    // Settle before interpreting ANY verdict, including unknown/unsupported and
    // typed failures. A failed or stuck match never refunds already-used work.
    let used_work_chirho = loan_work_chirho.checked_sub(limits_chirho.steps_chirho)?;
    let used_output_chirho = initial_output_chirho.checked_sub(limits_chirho.nodes_chirho)?;
    *work_chirho = work_chirho.checked_sub(used_work_chirho)?;
    *output_chirho = output_chirho.checked_sub(used_output_chirho)?;
    match result_chirho {
        Ok(FamilyReductionDecisionChirho::ReducedChirho(result_chirho)) => {
            Some(Some(result_chirho))
        }
        Err(MatchFailureChirho::LimitChirho(MatchLimitChirho::OutputChirho)) => None,
        Err(MatchFailureChirho::LimitChirho(MatchLimitChirho::WorkChirho))
            if loan_work_chirho == initial_work_chirho =>
        {
            None
        }
        // A per-match proof cap, unsupported syntax or a local comparison depth
        // refuses a reduction as before. It cannot certify a match or no-match.
        _ => Some(None),
    }
}

pub(super) fn reduce_families_with_context_chirho(
    context_chirho: &dyn super::equality_normalization_chirho::NormalizationContextChirho,
    ty_chirho: &TyChirho,
    depth_chirho: usize,
    work_chirho: &mut usize,
    output_chirho: &mut usize,
) -> Option<TyChirho> {
    match reduce_families_change_with_context_chirho(
        context_chirho,
        ty_chirho,
        depth_chirho,
        work_chirho,
        output_chirho,
    )? {
        NormalizationChangeChirho::UnchangedChirho => {
            clone_stuck_type_chirho(ty_chirho, depth_chirho, output_chirho)
        }
        NormalizationChangeChirho::RebuiltChirho(image_chirho) => Some(image_chirho),
    }
}

/// The family judgments are unchanged; this outcome describes only storage.
/// A stable call borrows its exact input instead of copying a stuck tree.
pub(super) fn reduce_families_change_with_context_chirho(
    context_chirho: &dyn super::equality_normalization_chirho::NormalizationContextChirho,
    ty_chirho: &TyChirho,
    depth_chirho: usize,
    work_chirho: &mut usize,
    output_chirho: &mut usize,
) -> Option<NormalizationChangeChirho> {
    if depth_chirho > 128 {
        return None;
    }
    *work_chirho = work_chirho.checked_sub(1)?;
    let child_chirho = depth_chirho.checked_add(1)?;
    match ty_chirho {
        TyChirho::ConChirho(name_chirho) => {
            *work_chirho = work_chirho.checked_sub(name_chirho.len())?;
            if context_chirho
                .family_name_key_chirho(name_chirho)
                .is_some_and(|key_chirho| {
                    context_chirho
                        .catalogue_chirho()
                        .definition_chirho(&key_chirho)
                        .is_some()
                })
                && let Some(reduced_chirho) = reduce_family_in_context_chirho(
                    context_chirho,
                    name_chirho,
                    &[],
                    work_chirho,
                    output_chirho,
                )?
            {
                let next_chirho = context_chirho.reduce_families_bounded_chirho(
                    &reduced_chirho,
                    child_chirho,
                    work_chirho,
                    output_chirho,
                )?;
                return Some(NormalizationChangeChirho::RebuiltChirho(
                    next_chirho.retain_owned_chirho(reduced_chirho),
                ));
            }
            Some(NormalizationChangeChirho::UnchangedChirho)
        }
        TyChirho::AppChirho(_, _) => {
            let mut head_chirho = ty_chirho;
            let mut arguments_chirho = Vec::new();
            while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
                *work_chirho = work_chirho.checked_sub(1)?;
                if depth_chirho.checked_add(arguments_chirho.len())? > 128 {
                    return None;
                }
                arguments_chirho.push(argument_chirho.as_ref());
                head_chirho = function_chirho;
            }
            arguments_chirho.reverse();
            let head_depth_chirho = depth_chirho.checked_add(arguments_chirho.len())?;
            if head_depth_chirho > 128 {
                return None;
            }
            let mut reduced_arguments_chirho = None::<Vec<TyChirho>>;
            for (index_chirho, argument_chirho) in arguments_chirho.iter().enumerate() {
                let image_chirho = context_chirho.reduce_families_bounded_chirho(
                    argument_chirho,
                    head_depth_chirho - index_chirho,
                    work_chirho,
                    output_chirho,
                )?;
                if !image_chirho.is_unchanged_chirho() && reduced_arguments_chirho.is_none() {
                    let mut prefix_chirho = Vec::new();
                    for (prefix_index_chirho, prefix_argument_chirho) in
                        arguments_chirho[..index_chirho].iter().enumerate()
                    {
                        prefix_chirho.push(clone_stuck_type_chirho(
                            prefix_argument_chirho,
                            head_depth_chirho - prefix_index_chirho,
                            output_chirho,
                        )?);
                    }
                    reduced_arguments_chirho = Some(prefix_chirho);
                }
                if let Some(reduced_arguments_chirho) = &mut reduced_arguments_chirho {
                    reduced_arguments_chirho.push(match image_chirho {
                        NormalizationChangeChirho::UnchangedChirho => clone_stuck_type_chirho(
                            argument_chirho,
                            head_depth_chirho - index_chirho,
                            output_chirho,
                        )?,
                        NormalizationChangeChirho::RebuiltChirho(image_chirho) => image_chirho,
                    });
                }
            }
            if let TyChirho::ConChirho(name_chirho) = head_chirho
                && context_chirho
                    .family_name_key_chirho(name_chirho)
                    .is_some_and(|key_chirho| {
                        context_chirho
                            .catalogue_chirho()
                            .definition_chirho(&key_chirho)
                            .is_some()
                    })
            {
                // The existing matcher takes an owned argument slice. Only an
                // actual family attempt requires these otherwise-stable copies.
                let arguments_changed_chirho = reduced_arguments_chirho.is_some();
                let actual_arguments_chirho = match reduced_arguments_chirho.take() {
                    Some(arguments_chirho) => arguments_chirho,
                    None => arguments_chirho
                        .iter()
                        .enumerate()
                        .map(|(index_chirho, argument_chirho)| {
                            clone_stuck_type_chirho(
                                argument_chirho,
                                head_depth_chirho - index_chirho,
                                output_chirho,
                            )
                        })
                        .collect::<Option<Vec<_>>>()?,
                };
                if let Some(reduced_chirho) = reduce_family_in_context_chirho(
                    context_chirho,
                    name_chirho,
                    &actual_arguments_chirho,
                    work_chirho,
                    output_chirho,
                )? {
                    let next_chirho = context_chirho.reduce_families_bounded_chirho(
                        &reduced_chirho,
                        child_chirho,
                        work_chirho,
                        output_chirho,
                    )?;
                    return Some(NormalizationChangeChirho::RebuiltChirho(
                        next_chirho.retain_owned_chirho(reduced_chirho),
                    ));
                }
                if arguments_changed_chirho {
                    reduced_arguments_chirho = Some(actual_arguments_chirho);
                }
            }
            let Some(reduced_arguments_chirho) = reduced_arguments_chirho else {
                return Some(NormalizationChangeChirho::UnchangedChirho);
            };
            let mut rebuilt_chirho =
                clone_stuck_type_chirho(head_chirho, head_depth_chirho, output_chirho)?;
            for argument_chirho in reduced_arguments_chirho {
                *output_chirho = output_chirho.checked_sub(1)?;
                rebuilt_chirho =
                    TyChirho::AppChirho(Box::new(rebuilt_chirho), Box::new(argument_chirho));
            }
            Some(NormalizationChangeChirho::RebuiltChirho(rebuilt_chirho))
        }
        TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
            let argument_image_chirho = context_chirho.reduce_families_bounded_chirho(
                argument_chirho,
                child_chirho,
                work_chirho,
                output_chirho,
            )?;
            let result_image_chirho = context_chirho.reduce_families_bounded_chirho(
                result_chirho,
                child_chirho,
                work_chirho,
                output_chirho,
            )?;
            if argument_image_chirho.is_unchanged_chirho()
                && result_image_chirho.is_unchanged_chirho()
            {
                return Some(NormalizationChangeChirho::UnchangedChirho);
            }
            *output_chirho = output_chirho.checked_sub(1)?;
            let argument_chirho = match argument_image_chirho {
                NormalizationChangeChirho::UnchangedChirho => {
                    clone_stuck_type_chirho(argument_chirho, child_chirho, output_chirho)?
                }
                NormalizationChangeChirho::RebuiltChirho(image_chirho) => image_chirho,
            };
            let result_chirho = match result_image_chirho {
                NormalizationChangeChirho::UnchangedChirho => {
                    clone_stuck_type_chirho(result_chirho, child_chirho, output_chirho)?
                }
                NormalizationChangeChirho::RebuiltChirho(image_chirho) => image_chirho,
            };
            Some(NormalizationChangeChirho::RebuiltChirho(
                TyChirho::FunChirho(
                    Box::new(argument_chirho),
                    Box::new(result_chirho),
                    *multiplicity_chirho,
                ),
            ))
        }
        TyChirho::ListChirho(element_chirho) => {
            let image_chirho = context_chirho.reduce_families_bounded_chirho(
                element_chirho,
                child_chirho,
                work_chirho,
                output_chirho,
            )?;
            match image_chirho {
                NormalizationChangeChirho::UnchangedChirho => {
                    Some(NormalizationChangeChirho::UnchangedChirho)
                }
                NormalizationChangeChirho::RebuiltChirho(element_chirho) => {
                    *output_chirho = output_chirho.checked_sub(1)?;
                    Some(NormalizationChangeChirho::RebuiltChirho(
                        TyChirho::ListChirho(Box::new(element_chirho)),
                    ))
                }
            }
        }
        TyChirho::TupleChirho(elements_chirho) => {
            let mut rebuilt_chirho = None::<Vec<TyChirho>>;
            for (index_chirho, element_chirho) in elements_chirho.iter().enumerate() {
                let image_chirho = context_chirho.reduce_families_bounded_chirho(
                    element_chirho,
                    child_chirho,
                    work_chirho,
                    output_chirho,
                )?;
                if !image_chirho.is_unchanged_chirho() && rebuilt_chirho.is_none() {
                    let mut prefix_chirho = Vec::new();
                    for prefix_chirho_element_chirho in &elements_chirho[..index_chirho] {
                        prefix_chirho.push(clone_stuck_type_chirho(
                            prefix_chirho_element_chirho,
                            child_chirho,
                            output_chirho,
                        )?);
                    }
                    rebuilt_chirho = Some(prefix_chirho);
                }
                if let Some(rebuilt_chirho) = &mut rebuilt_chirho {
                    rebuilt_chirho.push(match image_chirho {
                        NormalizationChangeChirho::UnchangedChirho => {
                            clone_stuck_type_chirho(element_chirho, child_chirho, output_chirho)?
                        }
                        NormalizationChangeChirho::RebuiltChirho(image_chirho) => image_chirho,
                    });
                }
            }
            match rebuilt_chirho {
                None => Some(NormalizationChangeChirho::UnchangedChirho),
                Some(elements_chirho) => {
                    *output_chirho = output_chirho.checked_sub(1)?;
                    Some(NormalizationChangeChirho::RebuiltChirho(
                        TyChirho::TupleChirho(elements_chirho),
                    ))
                }
            }
        }
        // This path previously only copied a stuck tree; it must retain its
        // physical-depth judgment without materializing before that judgment.
        _ => {
            inspect_stuck_type_chirho(ty_chirho, depth_chirho, work_chirho)?;
            Some(NormalizationChangeChirho::UnchangedChirho)
        }
    }
}

fn inspect_stuck_type_chirho(
    ty_chirho: &TyChirho,
    depth_chirho: usize,
    work_chirho: &mut usize,
) -> Option<()> {
    if depth_chirho > 128 {
        return None;
    }
    *work_chirho = work_chirho.checked_sub(1)?;
    let child_chirho = depth_chirho.checked_add(1)?;
    match ty_chirho {
        TyChirho::AppChirho(left_chirho, right_chirho)
        | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
            inspect_stuck_type_chirho(left_chirho, child_chirho, work_chirho)?;
            inspect_stuck_type_chirho(right_chirho, child_chirho, work_chirho)?;
        }
        TyChirho::ListChirho(element_chirho) => {
            inspect_stuck_type_chirho(element_chirho, child_chirho, work_chirho)?
        }
        TyChirho::TupleChirho(elements_chirho) => {
            for element_chirho in elements_chirho {
                inspect_stuck_type_chirho(element_chirho, child_chirho, work_chirho)?;
            }
        }
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => {
            for constraint_chirho in context_chirho {
                inspect_stuck_type_chirho(constraint_chirho, child_chirho, work_chirho)?;
            }
            inspect_stuck_type_chirho(body_chirho, child_chirho, work_chirho)?;
        }
        TyChirho::ForallChirho { body_chirho, .. }
        | TyChirho::RequiredForallChirho { body_chirho, .. } => {
            inspect_stuck_type_chirho(body_chirho, child_chirho, work_chirho)?
        }
        _ => {}
    }
    Some(())
}
