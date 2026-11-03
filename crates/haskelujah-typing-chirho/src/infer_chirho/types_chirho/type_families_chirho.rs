// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type families: registration, equation lookup and reduction, the built-in type-level literal
//! families among them.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::family_definitions_chirho::{IntrinsicTypeFamilyChirho, TypeFamilyReductionChirho};
use super::family_reduction_chirho::{
    FamilyReductionLimitsChirho, reduce_family_with_limits_chirho,
};
use super::family_registration_chirho::family_head_role_chirho;
use super::strip_name_qualifier_chirho;
use crate::ty_chirho::{MultChirho, TyChirho};
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
        let key_chirho = self.family_name_key_chirho(name_chirho)?;
        let definition_chirho = self.type_families_chirho.definition_chirho(&key_chirho)?;
        match definition_chirho.reduction_chirho() {
            TypeFamilyReductionChirho::IntrinsicChirho(intrinsic_chirho) => {
                reduce_builtin_type_family_application_chirho(
                    intrinsic_chirho,
                    arguments_chirho,
                    self.promotion_scope_chirho.is_some(),
                )
            }
            _ => reduce_family_with_limits_chirho(
                definition_chirho,
                arguments_chirho,
                &|name_chirho| family_head_role_chirho(&self.type_families_chirho, name_chirho),
                FamilyReductionLimitsChirho {
                    steps_chirho,
                    nodes_chirho,
                    depth_chirho: 128,
                },
            ),
        }
    }

    /// One budget covers the complete normalization tree, including nested redexes.
    /// Each leaf reserves 100,000 steps from a 10,000,000-step whole-tree allowance;
    /// the output/clone allowance also counts name bytes. Exhaustion conservatively
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

    fn reduce_families_bounded_chirho(
        &self,
        ty_chirho: &TyChirho,
        depth_chirho: usize,
        work_chirho: &mut usize,
        output_chirho: &mut usize,
    ) -> Option<TyChirho> {
        if depth_chirho > 128 {
            return None;
        }
        *work_chirho = work_chirho.checked_sub(1)?;
        *output_chirho = output_chirho.checked_sub(1)?;
        let child_chirho = depth_chirho.checked_add(1)?;
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => {
                *work_chirho = work_chirho.checked_sub(name_chirho.len())?;
                *output_chirho = output_chirho.checked_sub(name_chirho.len())?;
                if self.is_registered_family_chirho(name_chirho) {
                    *work_chirho = work_chirho.checked_sub(100_000)?;
                    if let Some(reduced_chirho) = self.reduce_family_with_budget_chirho(
                        name_chirho,
                        &[],
                        100_000,
                        *output_chirho,
                    ) {
                        return self.reduce_families_bounded_chirho(
                            &reduced_chirho,
                            child_chirho,
                            work_chirho,
                            output_chirho,
                        );
                    }
                }
                Some(TyChirho::ConChirho(name_chirho.clone()))
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
                let mut reduced_arguments_chirho = Vec::new();
                for (index_chirho, argument_chirho) in arguments_chirho.into_iter().enumerate() {
                    reduced_arguments_chirho.push(self.reduce_families_bounded_chirho(
                        argument_chirho,
                        head_depth_chirho - index_chirho,
                        work_chirho,
                        output_chirho,
                    )?);
                }
                if let TyChirho::ConChirho(name_chirho) = head_chirho
                    && self.is_registered_family_chirho(name_chirho)
                {
                    *work_chirho = work_chirho.checked_sub(100_000)?;
                    if let Some(reduced_chirho) = self.reduce_family_with_budget_chirho(
                        name_chirho,
                        &reduced_arguments_chirho,
                        100_000,
                        *output_chirho,
                    ) {
                        return self.reduce_families_bounded_chirho(
                            &reduced_chirho,
                            child_chirho,
                            work_chirho,
                            output_chirho,
                        );
                    }
                }
                let mut rebuilt_chirho =
                    clone_stuck_type_chirho(head_chirho, head_depth_chirho, output_chirho)?;
                for argument_chirho in reduced_arguments_chirho {
                    *output_chirho = output_chirho.checked_sub(1)?;
                    rebuilt_chirho =
                        TyChirho::AppChirho(Box::new(rebuilt_chirho), Box::new(argument_chirho));
                }
                Some(rebuilt_chirho)
            }
            TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
                Some(TyChirho::FunChirho(
                    Box::new(self.reduce_families_bounded_chirho(
                        argument_chirho,
                        child_chirho,
                        work_chirho,
                        output_chirho,
                    )?),
                    Box::new(self.reduce_families_bounded_chirho(
                        result_chirho,
                        child_chirho,
                        work_chirho,
                        output_chirho,
                    )?),
                    *multiplicity_chirho,
                ))
            }
            TyChirho::ListChirho(element_chirho) => Some(TyChirho::ListChirho(Box::new(
                self.reduce_families_bounded_chirho(
                    element_chirho,
                    child_chirho,
                    work_chirho,
                    output_chirho,
                )?,
            ))),
            TyChirho::TupleChirho(elements_chirho) => Some(TyChirho::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|element_chirho| {
                        self.reduce_families_bounded_chirho(
                            element_chirho,
                            child_chirho,
                            work_chirho,
                            output_chirho,
                        )
                    })
                    .collect::<Option<Vec<_>>>()?,
            )),
            _ => clone_stuck_type_chirho(ty_chirho, depth_chirho, output_chirho),
        }
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

fn nat_type_con_chirho(value_chirho: u128) -> TyChirho {
    TyChirho::ConChirho(value_chirho.to_string())
}

/// The compiler's promoted `True` or `False`: its builtin identity key when the module's
/// promoted names are keyed (row 686), the legacy spelling otherwise.
fn bool_type_con_chirho(value_chirho: bool, keyed_chirho: bool) -> TyChirho {
    TyChirho::ConChirho(if keyed_chirho {
        PromotedKeyChirho::builtin_bool_chirho(value_chirho)
            .as_str_chirho()
            .to_string()
    } else {
        if value_chirho { "True" } else { "False" }.to_string()
    })
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

fn symbol_type_con_chirho(value_chirho: &str) -> TyChirho {
    let escaped_chirho = value_chirho.replace('\\', "\\\\").replace('"', "\\\"");
    TyChirho::ConChirho(format!("\"{escaped_chirho}\""))
}

fn char_type_literal_chirho(ty_chirho: &TyChirho) -> Option<char> {
    let TyChirho::ConChirho(name_chirho) = ty_chirho else {
        return None;
    };
    let body_chirho = name_chirho.strip_prefix('\'')?.strip_suffix('\'')?;
    parse_haskell_char_body_chirho(body_chirho)
}

fn char_type_con_chirho(value_chirho: char) -> TyChirho {
    TyChirho::ConChirho(format!(
        "'{}'",
        render_haskell_char_body_chirho(value_chirho)
    ))
}

fn apply_extra_type_family_args_chirho(
    mut reduced_chirho: TyChirho,
    args_chirho: &[TyChirho],
    consumed_chirho: usize,
) -> TyChirho {
    for extra_arg_chirho in &args_chirho[consumed_chirho..] {
        reduced_chirho =
            TyChirho::AppChirho(Box::new(reduced_chirho), Box::new(extra_arg_chirho.clone()));
    }
    reduced_chirho
}

fn reduce_builtin_type_family_application_chirho(
    intrinsic_chirho: IntrinsicTypeFamilyChirho,
    args_chirho: &[TyChirho],
    keyed_chirho: bool,
) -> Option<TyChirho> {
    match (intrinsic_chirho, args_chirho) {
        // workflow: language-features-chirho/type-level-character-families-chirho
        (IntrinsicTypeFamilyChirho::CharToNatChirho, [value_chirho, ..]) => {
            char_type_literal_chirho(value_chirho)
                .map(|value_chirho| nat_type_con_chirho(u128::from(value_chirho as u32)))
                .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1))
        }
        (IntrinsicTypeFamilyChirho::NatToCharChirho, [value_chirho, ..]) => {
            nat_type_literal_chirho(value_chirho)
                .and_then(|value_chirho| u32::try_from(value_chirho).ok())
                .and_then(char::from_u32)
                .map(char_type_con_chirho)
                .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1))
        }
        (IntrinsicTypeFamilyChirho::AppendSymbolChirho, [left_chirho, right_chirho, ..]) => {
            let left_symbol_chirho = symbol_type_literal_chirho(left_chirho);
            let right_symbol_chirho = symbol_type_literal_chirho(right_chirho);
            match (left_symbol_chirho, right_symbol_chirho) {
                (Some(""), _) => Some(right_chirho.clone()),
                (_, Some("")) => Some(left_chirho.clone()),
                (Some(left_value_chirho), Some(right_value_chirho)) => Some(
                    symbol_type_con_chirho(&format!("{left_value_chirho}{right_value_chirho}")),
                ),
                _ => None,
            }
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2))
        }
        (IntrinsicTypeFamilyChirho::AddNatChirho, [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (Some(0), _) => Some(right_chirho.clone()),
            (_, Some(0)) => Some(left_chirho.clone()),
            (Some(left_value_chirho), Some(right_value_chirho)) => left_value_chirho
                .checked_add(right_value_chirho)
                .map(nat_type_con_chirho),
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        (IntrinsicTypeFamilyChirho::MultiplyNatChirho, [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (Some(0), _) | (_, Some(0)) => Some(nat_type_con_chirho(0)),
            (Some(1), _) => Some(right_chirho.clone()),
            (_, Some(1)) => Some(left_chirho.clone()),
            (Some(left_value_chirho), Some(right_value_chirho)) => left_value_chirho
                .checked_mul(right_value_chirho)
                .map(nat_type_con_chirho),
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        (IntrinsicTypeFamilyChirho::PowerNatChirho, [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (_, Some(0)) => Some(nat_type_con_chirho(1)),
            (Some(1), _) => Some(nat_type_con_chirho(1)),
            (_, Some(1)) => Some(left_chirho.clone()),
            (Some(left_value_chirho), Some(right_value_chirho)) => {
                u32::try_from(right_value_chirho)
                    .ok()
                    .and_then(|exp_chirho| left_value_chirho.checked_pow(exp_chirho))
                    .map(nat_type_con_chirho)
            }
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        (IntrinsicTypeFamilyChirho::SubtractNatChirho, [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (_, Some(0)) => Some(left_chirho.clone()),
            (Some(left_value_chirho), Some(right_value_chirho))
                if left_value_chirho >= right_value_chirho =>
            {
                Some(nat_type_con_chirho(left_value_chirho - right_value_chirho))
            }
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        (IntrinsicTypeFamilyChirho::DivideNatChirho, [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (_, Some(1)) => Some(left_chirho.clone()),
            (Some(left_value_chirho), Some(right_value_chirho)) if right_value_chirho != 0 => {
                Some(nat_type_con_chirho(left_value_chirho / right_value_chirho))
            }
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        (IntrinsicTypeFamilyChirho::ModuloNatChirho, [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (_, Some(1)) => Some(nat_type_con_chirho(0)),
            (Some(left_value_chirho), Some(right_value_chirho)) if right_value_chirho != 0 => {
                Some(nat_type_con_chirho(left_value_chirho % right_value_chirho))
            }
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        (IntrinsicTypeFamilyChirho::Log2NatChirho, [value_chirho, ..]) => {
            nat_type_literal_chirho(value_chirho)
                .and_then(|value_chirho| {
                    if value_chirho == 0 {
                        None
                    } else {
                        Some(nat_type_con_chirho(
                            127 - value_chirho.leading_zeros() as u128,
                        ))
                    }
                })
                .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1))
        }
        (IntrinsicTypeFamilyChirho::LessEqualNatChirho, [left_chirho, right_chirho, ..]) => {
            if left_chirho == right_chirho {
                Some(bool_type_con_chirho(true, keyed_chirho))
            } else {
                match (
                    nat_type_literal_chirho(left_chirho),
                    nat_type_literal_chirho(right_chirho),
                ) {
                    (Some(left_value_chirho), Some(right_value_chirho)) => Some(
                        bool_type_con_chirho(left_value_chirho <= right_value_chirho, keyed_chirho),
                    ),
                    _ => None,
                }
            }
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2))
        }
        (
            IntrinsicTypeFamilyChirho::IfChirho,
            [condition_chirho, true_chirho, false_chirho, ..],
        ) => {
            bool_type_literal_chirho(condition_chirho, keyed_chirho).map(|condition_chirho| {
                if condition_chirho {
                    true_chirho.clone()
                } else {
                    false_chirho.clone()
                }
            })
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 3)),
        (IntrinsicTypeFamilyChirho::NotChirho, [value_chirho, ..]) => {
            bool_type_literal_chirho(value_chirho, keyed_chirho)
                .map(|value_chirho| bool_type_con_chirho(!value_chirho, keyed_chirho))
                .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1))
        }
        (IntrinsicTypeFamilyChirho::AndChirho, [left_chirho, right_chirho, ..]) => {
            bool_type_literal_chirho(left_chirho, keyed_chirho)
                .map(|left_chirho| {
                    if left_chirho {
                        right_chirho.clone()
                    } else {
                        bool_type_con_chirho(false, keyed_chirho)
                    }
                })
                .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2))
        }
        (IntrinsicTypeFamilyChirho::OrChirho, [left_chirho, right_chirho, ..]) => {
            bool_type_literal_chirho(left_chirho, keyed_chirho)
                .map(|left_chirho| {
                    if left_chirho {
                        bool_type_con_chirho(true, keyed_chirho)
                    } else {
                        right_chirho.clone()
                    }
                })
                .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2))
        }
        _ => None,
    }
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
        TyChirho::FunChirho(a_chirho, r_chirho, _) => TyChirho::FunChirho(
            Box::new(substitute_type_vars_chirho(a_chirho, bindings_chirho)),
            Box::new(substitute_type_vars_chirho(r_chirho, bindings_chirho)),
            MultChirho::ManyChirho,
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
