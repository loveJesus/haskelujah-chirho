// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type families: registration, equation lookup and reduction, the built-in type-level literal
//! families among them.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::{collect_app_spine_chirho, strip_name_qualifier_chirho};
use crate::ty_chirho::{MultChirho, TyChirho};
use haskelujah_ast_chirho::lit_chirho::{
    parse_haskell_char_body_chirho, render_haskell_char_body_chirho,
};
use haskelujah_ast_chirho::promotion_chirho::{PromotedKeyChirho, is_promoted_key_chirho};
use std::collections::{HashMap, HashSet};

impl InferCtxChirho {
    /// Register a type family (open or closed). For open families this
    /// creates an empty equation list; for closed families it stores all
    /// equations immediately.
    pub fn register_type_family_chirho(
        &mut self,
        name_chirho: String,
        equations_chirho: Vec<(Vec<TyChirho>, TyChirho)>,
    ) {
        let normalized_equations_chirho: Vec<(Vec<TyChirho>, TyChirho)> = equations_chirho
            .into_iter()
            .map(|(lhs_types_chirho, rhs_chirho)| {
                (
                    lhs_types_chirho
                        .into_iter()
                        .map(|lhs_ty_chirho| self.normalize_imported_ty_chirho(&lhs_ty_chirho))
                        .collect(),
                    self.normalize_imported_ty_chirho(&rhs_chirho),
                )
            })
            .collect();
        let registered_equations_chirho = self.type_families_chirho.entry(name_chirho).or_default();
        if normalized_equations_chirho.is_empty() {
            return;
        }

        // Newer registrations should win over older imported equations, but
        // preserve the source order within each registration batch.
        let prior_equations_chirho = std::mem::take(registered_equations_chirho);
        registered_equations_chirho.extend(normalized_equations_chirho);
        registered_equations_chirho.extend(prior_equations_chirho);
    }

    /// Add an equation to an open type family from a `type instance` decl.
    pub fn register_type_family_instance_chirho(
        &mut self,
        family_name_chirho: String,
        lhs_types_chirho: Vec<TyChirho>,
        rhs_chirho: TyChirho,
    ) {
        let normalized_lhs_types_chirho: Vec<TyChirho> = lhs_types_chirho
            .iter()
            .map(|lhs_ty_chirho| self.normalize_imported_ty_chirho(lhs_ty_chirho))
            .collect();
        let normalized_rhs_chirho = self.normalize_imported_ty_chirho(&rhs_chirho);
        self.type_families_chirho
            .entry(family_name_chirho)
            .or_default()
            .insert(0, (normalized_lhs_types_chirho, normalized_rhs_chirho));
    }

    /// Try to reduce a type family application `F args...` by matching
    /// against registered equations. Returns `Some(reduced)` if a match
    /// is found, `None` otherwise (stuck family application).
    pub fn reduce_type_family_chirho(
        &self,
        family_name_chirho: &str,
        args_chirho: &[TyChirho],
    ) -> Option<TyChirho> {
        self.reduce_type_family_application_chirho(family_name_chirho, args_chirho)
    }

    fn reduce_type_family_application_chirho(
        &self,
        family_name_chirho: &str,
        args_chirho: &[TyChirho],
    ) -> Option<TyChirho> {
        let equation_sets_chirho = self.lookup_type_family_equation_sets_chirho(family_name_chirho);
        for equations_chirho in equation_sets_chirho {
            for (lhs_chirho, rhs_chirho) in equations_chirho {
                if lhs_chirho.len() > args_chirho.len() {
                    continue;
                }
                // Try to match each LHS pattern against the corresponding arg
                let mut bindings_chirho: HashMap<String, TyChirho> = HashMap::new();
                let mut matched_chirho = true;
                for (pat_chirho, arg_chirho) in lhs_chirho.iter().zip(args_chirho.iter()) {
                    if !match_type_pattern_chirho(pat_chirho, arg_chirho, &mut bindings_chirho) {
                        matched_chirho = false;
                        break;
                    }
                }
                if matched_chirho {
                    let mut reduced_chirho =
                        substitute_type_vars_chirho(rhs_chirho, &bindings_chirho);
                    for extra_arg_chirho in &args_chirho[lhs_chirho.len()..] {
                        reduced_chirho = TyChirho::AppChirho(
                            Box::new(reduced_chirho),
                            Box::new(extra_arg_chirho.clone()),
                        );
                    }
                    return Some(reduced_chirho);
                }
            }
        }
        reduce_builtin_type_family_application_chirho(
            family_name_chirho,
            args_chirho,
            self.promotion_scope_chirho.is_some(),
        )
    }

    fn lookup_type_family_equation_sets_chirho(
        &self,
        family_name_chirho: &str,
    ) -> Vec<&[(Vec<TyChirho>, TyChirho)]> {
        let bare_name_chirho = family_name_chirho
            .rsplit_once('.')
            .map(|(_prefix_chirho, bare_name_chirho)| bare_name_chirho)
            .unwrap_or(family_name_chirho);
        let mut seen_names_chirho: HashSet<String> = HashSet::new();
        let mut equation_sets_chirho: Vec<&[(Vec<TyChirho>, TyChirho)]> = Vec::new();

        // Prefer the bare family name so local equations can win over empty
        // imported qualified declarations for the same family.
        if seen_names_chirho.insert(bare_name_chirho.to_string())
            && let Some(equations_chirho) = self.type_families_chirho.get(bare_name_chirho)
        {
            equation_sets_chirho.push(equations_chirho.as_slice());
        }
        if seen_names_chirho.insert(family_name_chirho.to_string())
            && let Some(equations_chirho) = self.type_families_chirho.get(family_name_chirho)
        {
            equation_sets_chirho.push(equations_chirho.as_slice());
        }

        let suffix_chirho = format!(".{bare_name_chirho}");
        let suffix_match_names_chirho: Vec<String> = self
            .type_families_chirho
            .keys()
            .filter(|name_chirho| {
                name_chirho.ends_with(&suffix_chirho)
                    && !seen_names_chirho.contains(name_chirho.as_str())
            })
            .cloned()
            .collect();
        if suffix_match_names_chirho.len() == 1 {
            let suffix_name_chirho = &suffix_match_names_chirho[0];
            if seen_names_chirho.insert(suffix_name_chirho.clone())
                && let Some(equations_chirho) = self.type_families_chirho.get(suffix_name_chirho)
            {
                equation_sets_chirho.push(equations_chirho.as_slice());
            }
        }

        equation_sets_chirho
    }

    /// Reduce type family applications in a `TyChirho`. Walks the type and
    /// replaces saturated type family applications with their reduced form.
    /// For example, `F Int` where `type instance F Int = Bool` becomes `Bool`.
    pub fn reduce_type_families_in_ty_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        self.reduce_families_chirho(ty_chirho, 0)
    }

    fn reduce_families_chirho(&self, ty_chirho: &TyChirho, depth_chirho: usize) -> TyChirho {
        if depth_chirho > 100 {
            return ty_chirho.clone();
        }
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => {
                // Nullary type family (no arguments)
                if let Some(reduced_chirho) = self.reduce_type_family_chirho(name_chirho, &[]) {
                    return self.reduce_families_chirho(&reduced_chirho, depth_chirho + 1);
                }
                ty_chirho.clone()
            }
            TyChirho::AppChirho(_, _) => {
                // Collect spine and check if head is a type family
                let (head_chirho, args_chirho) = collect_app_spine_chirho(ty_chirho);
                if let TyChirho::ConChirho(name_chirho) = &head_chirho {
                    // Recursively reduce arguments first
                    let reduced_args_chirho: Vec<TyChirho> = args_chirho
                        .iter()
                        .map(|a_chirho| self.reduce_families_chirho(a_chirho, depth_chirho + 1))
                        .collect();
                    if let Some(result_chirho) = self
                        .reduce_type_family_application_chirho(name_chirho, &reduced_args_chirho)
                    {
                        return self.reduce_families_chirho(&result_chirho, depth_chirho + 1);
                    }
                    // Not a family — rebuild with reduced args
                    let mut result_chirho = head_chirho.clone();
                    for a_chirho in &reduced_args_chirho {
                        result_chirho = TyChirho::AppChirho(
                            Box::new(result_chirho),
                            Box::new(a_chirho.clone()),
                        );
                    }
                    return result_chirho;
                }
                // Head is not a Con — just reduce sub-parts
                let ef_chirho = self.reduce_families_chirho(
                    match ty_chirho {
                        TyChirho::AppChirho(f, _) => f,
                        _ => unreachable!(),
                    },
                    depth_chirho,
                );
                let ea_chirho = self.reduce_families_chirho(
                    match ty_chirho {
                        TyChirho::AppChirho(_, a) => a,
                        _ => unreachable!(),
                    },
                    depth_chirho,
                );
                TyChirho::AppChirho(Box::new(ef_chirho), Box::new(ea_chirho))
            }
            TyChirho::FunChirho(a_chirho, b_chirho, m_chirho) => TyChirho::FunChirho(
                Box::new(self.reduce_families_chirho(a_chirho, depth_chirho)),
                Box::new(self.reduce_families_chirho(b_chirho, depth_chirho)),
                *m_chirho,
            ),
            TyChirho::ListChirho(el_chirho) => TyChirho::ListChirho(Box::new(
                self.reduce_families_chirho(el_chirho, depth_chirho),
            )),
            TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
                elems_chirho
                    .iter()
                    .map(|e_chirho| self.reduce_families_chirho(e_chirho, depth_chirho))
                    .collect(),
            ),
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.reduce_families_chirho(body_chirho, depth_chirho)),
            },
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.reduce_families_chirho(body_chirho, depth_chirho)),
            },
            _ => ty_chirho.clone(),
        }
    }
}

pub(super) fn canonical_type_family_name_chirho(name_chirho: &str) -> String {
    // A promoted-constructor key is no family name, and its dots are no qualifier.
    if is_promoted_key_chirho(name_chirho) {
        return name_chirho.to_string();
    }
    let bare_name_chirho = strip_name_qualifier_chirho(name_chirho);
    if bare_name_chirho.starts_with('(')
        && bare_name_chirho.ends_with(')')
        && bare_name_chirho.len() > 2
    {
        bare_name_chirho[1..bare_name_chirho.len() - 1].to_string()
    } else {
        bare_name_chirho.to_string()
    }
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

fn bool_type_literal_chirho(ty_chirho: &TyChirho) -> Option<bool> {
    let TyChirho::ConChirho(name_chirho) = ty_chirho else {
        return None;
    };
    // A key is opaque: compare it whole before any `.` is read as a qualifier.
    if is_promoted_key_chirho(name_chirho) {
        return [true, false].into_iter().find(|value_chirho| {
            PromotedKeyChirho::builtin_bool_chirho(*value_chirho).as_str_chirho() == name_chirho
        });
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
    family_name_chirho: &str,
    args_chirho: &[TyChirho],
    keyed_chirho: bool,
) -> Option<TyChirho> {
    let family_chirho = canonical_type_family_name_chirho(family_name_chirho);
    match (family_chirho.as_str(), args_chirho) {
        // workflow: language-features-chirho/type-level-character-families-chirho
        ("CharToNat", [value_chirho, ..]) => char_type_literal_chirho(value_chirho)
            .map(|value_chirho| nat_type_con_chirho(u128::from(value_chirho as u32)))
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1)),
        ("NatToChar", [value_chirho, ..]) => nat_type_literal_chirho(value_chirho)
            .and_then(|value_chirho| u32::try_from(value_chirho).ok())
            .and_then(char::from_u32)
            .map(char_type_con_chirho)
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1)),
        ("AppendSymbol", [left_chirho, right_chirho, ..]) => {
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
        ("+", [left_chirho, right_chirho, ..]) => match (
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
        ("*", [left_chirho, right_chirho, ..]) => match (
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
        ("^", [left_chirho, right_chirho, ..]) => match (
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
        ("-", [left_chirho, right_chirho, ..]) => match (
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
        ("Div", [left_chirho, right_chirho, ..]) => match (
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
        ("Mod", [left_chirho, right_chirho, ..]) => match (
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
        ("Log2", [value_chirho, ..]) => nat_type_literal_chirho(value_chirho)
            .and_then(|value_chirho| {
                if value_chirho == 0 {
                    None
                } else {
                    Some(nat_type_con_chirho(
                        127 - value_chirho.leading_zeros() as u128,
                    ))
                }
            })
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1)),
        ("<=?", [left_chirho, right_chirho, ..]) => if left_chirho == right_chirho {
            Some(bool_type_con_chirho(true, keyed_chirho))
        } else {
            match (
                nat_type_literal_chirho(left_chirho),
                nat_type_literal_chirho(right_chirho),
            ) {
                (Some(left_value_chirho), Some(right_value_chirho)) => Some(bool_type_con_chirho(
                    left_value_chirho <= right_value_chirho,
                    keyed_chirho,
                )),
                _ => None,
            }
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        ("If", [condition_chirho, true_chirho, false_chirho, ..]) => {
            bool_type_literal_chirho(condition_chirho).map(|condition_chirho| {
                if condition_chirho {
                    true_chirho.clone()
                } else {
                    false_chirho.clone()
                }
            })
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 3)),
        ("Not", [value_chirho, ..]) => bool_type_literal_chirho(value_chirho)
            .map(|value_chirho| bool_type_con_chirho(!value_chirho, keyed_chirho))
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1)),
        ("&&", [left_chirho, right_chirho, ..]) => bool_type_literal_chirho(left_chirho)
            .map(|left_chirho| {
                if left_chirho {
                    right_chirho.clone()
                } else {
                    bool_type_con_chirho(false, keyed_chirho)
                }
            })
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        ("||", [left_chirho, right_chirho, ..]) => bool_type_literal_chirho(left_chirho)
            .map(|left_chirho| {
                if left_chirho {
                    bool_type_con_chirho(true, keyed_chirho)
                } else {
                    right_chirho.clone()
                }
            })
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        _ => None,
    }
}

/// Match a type family LHS pattern against a concrete type argument.
/// Type variables in the pattern bind to the corresponding argument types.
/// Type constructors must match exactly.
fn match_type_pattern_chirho(
    pat_chirho: &TyChirho,
    arg_chirho: &TyChirho,
    bindings_chirho: &mut HashMap<String, TyChirho>,
) -> bool {
    match pat_chirho {
        // A type variable in the pattern matches anything
        TyChirho::VarChirho(tv_chirho) => {
            let var_name_chirho = format!("tv{}", tv_chirho.0);
            if let Some(existing_chirho) = bindings_chirho.get(&var_name_chirho) {
                existing_chirho == arg_chirho
            } else {
                bindings_chirho.insert(var_name_chirho, arg_chirho.clone());
                true
            }
        }
        // ForallVar in the pattern — a named type variable from type family LHS
        // (e.g. `a` in `F (Maybe a) = [a]`). Matches any argument type.
        TyChirho::ForallVarChirho(name_chirho) => {
            if let Some(existing_chirho) = bindings_chirho.get(name_chirho) {
                existing_chirho == arg_chirho
            } else {
                bindings_chirho.insert(name_chirho.clone(), arg_chirho.clone());
                true
            }
        }
        // A constructor must match the same constructor
        TyChirho::ConChirho(name_chirho) => {
            matches!(arg_chirho, TyChirho::ConChirho(arg_name_chirho) if arg_name_chirho == name_chirho)
        }
        // Type application: both sides must be apps with matching structure
        TyChirho::AppChirho(f_chirho, a_chirho) => {
            if let TyChirho::AppChirho(af_chirho, aa_chirho) = arg_chirho {
                match_type_pattern_chirho(f_chirho, af_chirho, bindings_chirho)
                    && match_type_pattern_chirho(a_chirho, aa_chirho, bindings_chirho)
            } else {
                false
            }
        }
        TyChirho::ListChirho(inner_chirho) => {
            if let TyChirho::ListChirho(arg_inner_chirho) = arg_chirho {
                match_type_pattern_chirho(inner_chirho, arg_inner_chirho, bindings_chirho)
            } else {
                false
            }
        }
        TyChirho::TupleChirho(elems_chirho) => {
            if let TyChirho::TupleChirho(arg_elems_chirho) = arg_chirho {
                if elems_chirho.len() != arg_elems_chirho.len() {
                    return false;
                }
                elems_chirho
                    .iter()
                    .zip(arg_elems_chirho.iter())
                    .all(|(p_chirho, a_chirho)| {
                        match_type_pattern_chirho(p_chirho, a_chirho, bindings_chirho)
                    })
            } else {
                false
            }
        }
        TyChirho::FunChirho(a_chirho, r_chirho, _) => {
            if let TyChirho::FunChirho(aa_chirho, ar_chirho, _) = arg_chirho {
                match_type_pattern_chirho(a_chirho, aa_chirho, bindings_chirho)
                    && match_type_pattern_chirho(r_chirho, ar_chirho, bindings_chirho)
            } else {
                false
            }
        }
        // Int, Bool, Char literals — exact match only
        _ => pat_chirho == arg_chirho,
    }
}

/// Substitute type variables in a type family equation RHS using the
/// bindings collected during pattern matching.
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
