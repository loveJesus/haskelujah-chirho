// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type synonyms and imported type names: registration, expansion, normalization and
//! synonym-pattern matching.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::family_definitions_chirho::is_type_family_key_chirho;
use super::strip_name_qualifier_chirho;
use crate::ty_chirho::TyChirho;
use haskelujah_ast_chirho::promotion_chirho::is_promoted_key_chirho;
use std::collections::{HashMap, HashSet};

impl InferCtxChirho {
    pub(super) fn set_imported_type_name_preferences_chirho(
        &mut self,
        safe_unqualified_imported_type_names_chirho: &HashSet<String>,
        preferred_qualified_type_names_chirho: &HashMap<String, String>,
    ) {
        self.safe_unqualified_imported_type_names_chirho =
            safe_unqualified_imported_type_names_chirho.clone();
        self.preferred_qualified_type_names_chirho = preferred_qualified_type_names_chirho.clone();
    }

    pub(super) fn normalize_imported_type_name_chirho(&self, name_text_chirho: &str) -> String {
        // A promoted-constructor key keeps its whole identity (row 686).
        if is_promoted_key_chirho(name_text_chirho)
            || is_type_family_key_chirho(name_text_chirho)
            || self
                .type_families_chirho
                .declaration_schemes_chirho()
                .get_chirho(name_text_chirho)
                .is_some()
        {
            return name_text_chirho.to_string();
        }
        if let Some(key_chirho) = self.family_name_key_chirho(name_text_chirho) {
            return key_chirho;
        }
        if let Some(declaration_chirho) = self
            .type_constructor_scope_chirho
            .as_ref()
            .and_then(|scope_chirho| scope_chirho.declaration_chirho(name_text_chirho))
        {
            return declaration_chirho.type_key_chirho();
        }
        let bare_name_chirho = strip_name_qualifier_chirho(name_text_chirho);
        if self
            .safe_unqualified_imported_type_names_chirho
            .contains(bare_name_chirho)
            || self
                .preferred_qualified_type_names_chirho
                .contains_key(bare_name_chirho)
        {
            bare_name_chirho.to_string()
        } else {
            name_text_chirho.to_string()
        }
    }

    pub(super) fn normalize_imported_ty_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        let copied_chirho = super::equality_normalization_chirho::copy_term_with_budget_chirho(
            ty_chirho,
            &mut super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho(),
        );
        match copied_chirho {
            Some(term_chirho) => self.normalize_imported_ty_inner_chirho(&term_chirho),
            None => {
                self.type_normalization_exhausted_chirho.set(true);
                TyChirho::ConChirho("%normalization:resource-exhausted".to_owned())
            }
        }
    }

    pub(super) fn normalize_imported_ty_inner_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        match ty_chirho {
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => TyChirho::QualChirho {
                context_chirho: context_chirho
                    .iter()
                    .map(|constraint_chirho| {
                        self.normalize_imported_ty_inner_chirho(constraint_chirho)
                    })
                    .collect(),
                body_chirho: Box::new(self.normalize_imported_ty_inner_chirho(body_chirho)),
            },
            TyChirho::ConChirho(name_chirho) => {
                TyChirho::ConChirho(self.normalize_imported_type_name_chirho(name_chirho))
            }
            TyChirho::AppChirho(fun_chirho, arg_chirho) => TyChirho::AppChirho(
                Box::new(self.normalize_imported_ty_inner_chirho(fun_chirho)),
                Box::new(self.normalize_imported_ty_inner_chirho(arg_chirho)),
            ),
            TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => TyChirho::FunChirho(
                Box::new(self.normalize_imported_ty_inner_chirho(arg_chirho)),
                Box::new(self.normalize_imported_ty_inner_chirho(result_chirho)),
                *mult_chirho,
            ),
            TyChirho::TupleChirho(elements_chirho) => TyChirho::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|element_chirho| self.normalize_imported_ty_inner_chirho(element_chirho))
                    .collect(),
            ),
            TyChirho::ListChirho(inner_chirho) => TyChirho::ListChirho(Box::new(
                self.normalize_imported_ty_inner_chirho(inner_chirho),
            )),
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::ForallChirho {
                vars_chirho: vars_chirho.map_records_chirho(|record_chirho| {
                    let mut mapped_chirho = record_chirho.clone();
                    mapped_chirho.kind_chirho =
                        record_chirho.kind_chirho.as_ref().map(|kind_chirho| {
                            std::sync::Arc::new(
                                self.normalize_imported_ty_inner_chirho(kind_chirho),
                            )
                        });
                    mapped_chirho
                }),
                body_chirho: Box::new(self.normalize_imported_ty_inner_chirho(body_chirho)),
            },
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.map_records_chirho(|record_chirho| {
                    let mut mapped_chirho = record_chirho.clone();
                    mapped_chirho.kind_chirho =
                        record_chirho.kind_chirho.as_ref().map(|kind_chirho| {
                            std::sync::Arc::new(
                                self.normalize_imported_ty_inner_chirho(kind_chirho),
                            )
                        });
                    mapped_chirho
                }),
                body_chirho: Box::new(self.normalize_imported_ty_inner_chirho(body_chirho)),
            },
            _ => ty_chirho.clone(),
        }
    }

    pub(super) fn uses_checked_declaration_chirho(&self, name_chirho: &str) -> bool {
        self.type_constructor_scope_chirho
            .as_ref()
            .and_then(|scope_chirho| scope_chirho.declaration_chirho(name_chirho))
            .is_some()
            || self
                .type_families_chirho
                .declaration_schemes_chirho()
                .get_chirho(name_chirho)
                .is_some()
    }

    /// Register a type synonym from a `TypeAliasDeclChirho`.
    pub fn register_type_synonym_chirho(
        &mut self,
        name_chirho: String,
        params_chirho: Vec<String>,
        rhs_chirho: TyChirho,
    ) {
        let normalized_rhs_chirho = self.normalize_imported_ty_chirho(&rhs_chirho);
        self.type_synonyms_chirho
            .insert(name_chirho, (params_chirho, normalized_rhs_chirho));
    }

    /// Expand type synonyms in a `TyChirho`. Handles both nullary synonyms
    /// (e.g. `String` → `[Char]`) and parameterised synonyms (e.g.
    /// `Pair Int` → `(Int, Int)` for `type Pair a = (a, a)`).
    pub fn expand_type_synonyms_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        if self.elaboration_state_chirho.owner_chirho.is_some()
            || !self
                .type_families_chirho
                .declaration_schemes_chirho()
                .is_empty_chirho()
        {
            return super::synonym_expansion_chirho::expand_synonyms_with_budget_chirho(
                self,
                ty_chirho,
                &mut super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho(),
            )
            .unwrap_or_else(|| {
                self.type_normalization_exhausted_chirho.set(true);
                TyChirho::ConChirho("%normalization:resource-exhausted".to_owned())
            });
        }
        self.expand_syn_chirho(ty_chirho, 0)
    }

    pub(super) fn lookup_type_synonym_chirho(
        &self,
        name_chirho: &str,
    ) -> Option<&(Vec<String>, TyChirho)> {
        lookup_type_synonym_in_scope_chirho(
            &self.type_synonyms_chirho,
            &self.type_families_chirho,
            self.type_constructor_scope_chirho.as_deref(),
            name_chirho,
        )
    }

    fn expand_syn_chirho(&self, ty_chirho: &TyChirho, depth_chirho: usize) -> TyChirho {
        if depth_chirho > 100 {
            return ty_chirho.clone(); // guard against cycles
        }
        match ty_chirho {
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => TyChirho::QualChirho {
                context_chirho: context_chirho
                    .iter()
                    .map(|constraint_chirho| {
                        self.expand_syn_chirho(constraint_chirho, depth_chirho + 1)
                    })
                    .collect(),
                body_chirho: Box::new(self.expand_syn_chirho(body_chirho, depth_chirho + 1)),
            },
            TyChirho::ConChirho(name_chirho) => {
                if let Some((params_chirho, rhs_chirho)) =
                    self.lookup_type_synonym_chirho(name_chirho)
                    && params_chirho.is_empty()
                {
                    // Nullary synonym — expand and recurse
                    return self.expand_syn_chirho(rhs_chirho, depth_chirho + 1);
                }
                ty_chirho.clone()
            }
            TyChirho::AppChirho(fun_chirho, arg_chirho) => {
                // Collect the spine: f a1 a2 ... an
                let (head_chirho, args_chirho) = collect_app_spine_chirho(ty_chirho);
                if let TyChirho::ConChirho(name_chirho) = &head_chirho
                    && let Some((params_chirho, rhs_chirho)) =
                        self.lookup_type_synonym_chirho(name_chirho)
                    && args_chirho.len() >= params_chirho.len()
                {
                    // Saturated application — substitute params
                    let expanded_args_chirho: Vec<TyChirho> = args_chirho
                        .iter()
                        .map(|a_chirho| self.expand_syn_chirho(a_chirho, depth_chirho + 1))
                        .collect();
                    let mut body_chirho = rhs_chirho.clone();
                    for (p_chirho, a_chirho) in
                        params_chirho.iter().zip(expanded_args_chirho.iter())
                    {
                        body_chirho = subst_named_var_chirho(&body_chirho, p_chirho, a_chirho);
                    }
                    // Apply remaining args (over-saturated)
                    let mut result_chirho = self.expand_syn_chirho(&body_chirho, depth_chirho + 1);
                    for a_chirho in &expanded_args_chirho[params_chirho.len()..] {
                        result_chirho = TyChirho::AppChirho(
                            Box::new(result_chirho),
                            Box::new(a_chirho.clone()),
                        );
                    }
                    return result_chirho;
                }
                // Not a synonym application — just expand sub-parts
                let ef_chirho = self.expand_syn_chirho(fun_chirho, depth_chirho);
                let ea_chirho = self.expand_syn_chirho(arg_chirho, depth_chirho);
                TyChirho::AppChirho(Box::new(ef_chirho), Box::new(ea_chirho))
            }
            TyChirho::FunChirho(a_chirho, b_chirho, mult_chirho) => TyChirho::FunChirho(
                Box::new(self.expand_syn_chirho(a_chirho, depth_chirho)),
                Box::new(self.expand_syn_chirho(b_chirho, depth_chirho)),
                *mult_chirho,
            ),
            TyChirho::ListChirho(el_chirho) => {
                TyChirho::ListChirho(Box::new(self.expand_syn_chirho(el_chirho, depth_chirho)))
            }
            TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
                elems_chirho
                    .iter()
                    .map(|e_chirho| self.expand_syn_chirho(e_chirho, depth_chirho))
                    .collect(),
            ),
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::ForallChirho {
                vars_chirho: vars_chirho.map_records_chirho(|record_chirho| {
                    let mut mapped_chirho = record_chirho.clone();
                    mapped_chirho.kind_chirho =
                        record_chirho.kind_chirho.as_ref().map(|kind_chirho| {
                            std::sync::Arc::new(
                                self.expand_syn_chirho(kind_chirho, depth_chirho + 1),
                            )
                        });
                    mapped_chirho
                }),
                body_chirho: Box::new(self.expand_syn_chirho(body_chirho, depth_chirho)),
            },
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.map_records_chirho(|record_chirho| {
                    let mut mapped_chirho = record_chirho.clone();
                    mapped_chirho.kind_chirho =
                        record_chirho.kind_chirho.as_ref().map(|kind_chirho| {
                            std::sync::Arc::new(
                                self.expand_syn_chirho(kind_chirho, depth_chirho + 1),
                            )
                        });
                    mapped_chirho
                }),
                body_chirho: Box::new(self.expand_syn_chirho(body_chirho, depth_chirho)),
            },
            _ => ty_chirho.clone(),
        }
    }

    /// Normalize a type to a fixed point by alternating synonym expansion and
    /// type-family reduction. This catches cases where family reduction
    /// exposes a synonym head such as `Rep f` -> `GRep f`.
    pub fn normalize_ty_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        let mut current_ty_chirho = ty_chirho.clone();
        for _iteration_chirho in 0..100 {
            let expanded_ty_chirho = self.expand_type_synonyms_chirho(&current_ty_chirho);
            let reduced_ty_chirho = self.reduce_type_families_in_ty_chirho(&expanded_ty_chirho);
            let normalized_ty_chirho = self.expand_type_synonyms_chirho(&reduced_ty_chirho);
            // Local given equalities from GADT patterns rewrite rigid variables
            // in scope. workflow: language-features-chirho/rigid-type-variables-chirho
            let normalized_ty_chirho = match self
                .env_chirho
                .apply_refinements_chirho(&normalized_ty_chirho)
            {
                Ok(term_chirho) => term_chirho,
                Err(_) => {
                    self.type_normalization_exhausted_chirho.set(true);
                    return TyChirho::ConChirho("%normalization:resource-exhausted".to_owned());
                }
            };
            let normalized_ty_chirho = self.apply_given_rewrites_chirho(&normalized_ty_chirho);
            if normalized_ty_chirho == current_ty_chirho {
                return normalized_ty_chirho;
            }
            current_ty_chirho = normalized_ty_chirho;
        }
        current_ty_chirho
    }

    pub(super) fn saturated_st_inner_ty_chirho(&self, ty_chirho: &TyChirho) -> Option<TyChirho> {
        if let TyChirho::AppChirho(_, _) = ty_chirho {
            let (head_chirho, args_chirho) = collect_app_spine_chirho(ty_chirho);
            if matches!(&head_chirho, TyChirho::ConChirho(name_chirho) if name_chirho == "ST")
                && args_chirho.len() == 2
            {
                return Some(args_chirho[1].clone());
            }
        }
        None
    }

    fn type_matches_synonym_pattern_chirho(
        params_chirho: &[String],
        pattern_chirho: &TyChirho,
        actual_chirho: &TyChirho,
        bindings_chirho: &mut HashMap<String, TyChirho>,
    ) -> bool {
        match pattern_chirho {
            TyChirho::ForallVarChirho(name_chirho) if params_chirho.contains(name_chirho) => {
                if let Some(bound_chirho) = bindings_chirho.get(name_chirho) {
                    bound_chirho == actual_chirho
                } else {
                    bindings_chirho.insert(name_chirho.clone(), actual_chirho.clone());
                    true
                }
            }
            TyChirho::ConChirho(pattern_name_chirho) => {
                matches!(actual_chirho, TyChirho::ConChirho(actual_name_chirho) if actual_name_chirho == pattern_name_chirho)
            }
            TyChirho::VarChirho(pattern_var_chirho) => {
                matches!(actual_chirho, TyChirho::VarChirho(actual_var_chirho) if actual_var_chirho == pattern_var_chirho)
            }
            TyChirho::AppChirho(pattern_fun_chirho, pattern_arg_chirho) => {
                matches!(
                    actual_chirho,
                    TyChirho::AppChirho(actual_fun_chirho, actual_arg_chirho)
                        if Self::type_matches_synonym_pattern_chirho(
                            params_chirho,
                            pattern_fun_chirho,
                            actual_fun_chirho,
                            bindings_chirho,
                        ) && Self::type_matches_synonym_pattern_chirho(
                            params_chirho,
                            pattern_arg_chirho,
                            actual_arg_chirho,
                            bindings_chirho,
                        )
                )
            }
            TyChirho::FunChirho(pattern_arg_chirho, pattern_result_chirho, pattern_mult_chirho) => {
                matches!(
                    actual_chirho,
                    TyChirho::FunChirho(actual_arg_chirho, actual_result_chirho, actual_mult_chirho)
                        if actual_mult_chirho == pattern_mult_chirho
                            && Self::type_matches_synonym_pattern_chirho(
                                params_chirho,
                                pattern_arg_chirho,
                                actual_arg_chirho,
                                bindings_chirho,
                            )
                            && Self::type_matches_synonym_pattern_chirho(
                                params_chirho,
                                pattern_result_chirho,
                                actual_result_chirho,
                                bindings_chirho,
                            )
                )
            }
            TyChirho::ListChirho(pattern_elem_chirho) => {
                matches!(
                    actual_chirho,
                    TyChirho::ListChirho(actual_elem_chirho)
                        if Self::type_matches_synonym_pattern_chirho(
                            params_chirho,
                            pattern_elem_chirho,
                            actual_elem_chirho,
                            bindings_chirho,
                        )
                )
            }
            TyChirho::TupleChirho(pattern_elems_chirho) => {
                matches!(actual_chirho, TyChirho::TupleChirho(actual_elems_chirho)
                if pattern_elems_chirho.len() == actual_elems_chirho.len()
                    && pattern_elems_chirho
                        .iter()
                        .zip(actual_elems_chirho.iter())
                        .all(|(pattern_elem_chirho, actual_elem_chirho)| Self::type_matches_synonym_pattern_chirho(
                            params_chirho,
                            pattern_elem_chirho,
                            actual_elem_chirho,
                            bindings_chirho,
                        )))
            }
            TyChirho::ForallChirho {
                vars_chirho: pattern_vars_chirho,
                body_chirho: pattern_body_chirho,
            } => {
                matches!(
                    actual_chirho,
                    TyChirho::ForallChirho {
                        vars_chirho: actual_vars_chirho,
                        body_chirho: actual_body_chirho,
                    } if pattern_vars_chirho == actual_vars_chirho
                        && Self::type_matches_synonym_pattern_chirho(
                            params_chirho,
                            pattern_body_chirho,
                            actual_body_chirho,
                            bindings_chirho,
                        )
                )
            }
            _ => pattern_chirho == actual_chirho,
        }
    }

    pub(super) fn matches_function_result_alias_expansion_chirho(
        &self,
        ty_chirho: &TyChirho,
    ) -> bool {
        let normalized_ty_chirho = self.normalize_ty_chirho(ty_chirho);
        ["ShowS", "ReadS"].iter().any(|synonym_name_chirho| {
            let Some((params_chirho, rhs_chirho)) =
                self.lookup_type_synonym_chirho(synonym_name_chirho)
            else {
                return false;
            };
            let normalized_rhs_chirho = self.normalize_ty_chirho(rhs_chirho);
            let mut bindings_chirho = HashMap::new();
            Self::type_matches_synonym_pattern_chirho(
                params_chirho,
                &normalized_rhs_chirho,
                &normalized_ty_chirho,
                &mut bindings_chirho,
            )
        })
    }
}

/// Collect the spine of a left-nested `AppChirho` into (head, [args]).
pub(super) fn collect_app_spine_chirho(ty_chirho: &TyChirho) -> (TyChirho, Vec<TyChirho>) {
    let mut head_chirho = ty_chirho.clone();
    let mut args_chirho = Vec::new();
    while let TyChirho::AppChirho(f_chirho, a_chirho) = head_chirho {
        args_chirho.push(*a_chirho);
        head_chirho = *f_chirho;
    }
    args_chirho.reverse();
    (head_chirho, args_chirho)
}

/// Substitute a named type variable in a `TyChirho`. Used for expanding
/// parameterised type synonyms where the RHS contains named variables
/// (e.g. `type Pair a = (a, a)` — substitute `a` for the applied type).
pub(super) fn subst_named_var_chirho(
    ty_chirho: &TyChirho,
    name_chirho: &str,
    replacement_chirho: &TyChirho,
) -> TyChirho {
    match ty_chirho {
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => TyChirho::QualChirho {
            context_chirho: context_chirho
                .iter()
                .map(|constraint_chirho| {
                    subst_named_var_chirho(constraint_chirho, name_chirho, replacement_chirho)
                })
                .collect(),
            body_chirho: Box::new(subst_named_var_chirho(
                body_chirho,
                name_chirho,
                replacement_chirho,
            )),
        },
        TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => TyChirho::RequiredForallChirho {
            vars_chirho: vars_chirho.map_records_chirho(|record_chirho| {
                let mut mapped_chirho = record_chirho.clone();
                mapped_chirho.kind_chirho = record_chirho.kind_chirho.as_ref().map(|kind_chirho| {
                    std::sync::Arc::new(subst_named_var_chirho(
                        kind_chirho,
                        name_chirho,
                        replacement_chirho,
                    ))
                });
                mapped_chirho
            }),
            body_chirho: Box::new(subst_named_var_chirho(
                body_chirho,
                name_chirho,
                replacement_chirho,
            )),
        },
        TyChirho::ConChirho(n_chirho) if n_chirho == name_chirho => replacement_chirho.clone(),
        TyChirho::ForallVarChirho(n_chirho) if n_chirho == name_chirho => {
            replacement_chirho.clone()
        }
        TyChirho::AppChirho(f_chirho, a_chirho) => TyChirho::AppChirho(
            Box::new(subst_named_var_chirho(
                f_chirho,
                name_chirho,
                replacement_chirho,
            )),
            Box::new(subst_named_var_chirho(
                a_chirho,
                name_chirho,
                replacement_chirho,
            )),
        ),
        TyChirho::FunChirho(a_chirho, b_chirho, mult_chirho) => TyChirho::FunChirho(
            Box::new(subst_named_var_chirho(
                a_chirho,
                name_chirho,
                replacement_chirho,
            )),
            Box::new(subst_named_var_chirho(
                b_chirho,
                name_chirho,
                replacement_chirho,
            )),
            *mult_chirho,
        ),
        TyChirho::ListChirho(el_chirho) => TyChirho::ListChirho(Box::new(subst_named_var_chirho(
            el_chirho,
            name_chirho,
            replacement_chirho,
        ))),
        TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
            elems_chirho
                .iter()
                .map(|e_chirho| subst_named_var_chirho(e_chirho, name_chirho, replacement_chirho))
                .collect(),
        ),
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } => TyChirho::ForallChirho {
            vars_chirho: vars_chirho.map_records_chirho(|record_chirho| {
                let mut mapped_chirho = record_chirho.clone();
                mapped_chirho.kind_chirho = record_chirho.kind_chirho.as_ref().map(|kind_chirho| {
                    std::sync::Arc::new(subst_named_var_chirho(
                        kind_chirho,
                        name_chirho,
                        replacement_chirho,
                    ))
                });
                mapped_chirho
            }),
            body_chirho: Box::new(subst_named_var_chirho(
                body_chirho,
                name_chirho,
                replacement_chirho,
            )),
        },
        _ => ty_chirho.clone(),
    }
}

/// Build the applied type `Map k v` for Data.Map builtin signatures. (WI-005)
///
/// The Core primop bodies (dict_chirho/prelude_chirho.rs) already carry the real
/// map type; several builtin schemes historically faked the map argument/result
/// as `Int`, which typechecks in isolation but rejects any genuine `Map` value
/// (E0200 "expected `Int`, found `((Map k) v)`"). This keeps the type-env schemes
/// in agreement with the real map representation.
pub(super) fn map_ty_chirho(k_chirho: TyChirho, v_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Map".to_string())),
            Box::new(k_chirho),
        )),
        Box::new(v_chirho),
    )
}

pub(super) fn lookup_type_synonym_in_scope_chirho<'scope_chirho>(
    synonyms_chirho: &'scope_chirho HashMap<String, (Vec<String>, TyChirho)>,
    catalogue_chirho: &super::TypeFamilyEnvChirho,
    type_scope_chirho: Option<&haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho>,
    name_chirho: &str,
) -> Option<&'scope_chirho (Vec<String>, TyChirho)> {
    if is_type_family_key_chirho(name_chirho)
        || is_promoted_key_chirho(name_chirho)
        || catalogue_chirho
            .declaration_schemes_chirho()
            .get_chirho(name_chirho)
            .is_some()
        || type_scope_chirho
            .and_then(|scope_chirho| scope_chirho.declaration_chirho(name_chirho))
            .is_some()
    {
        return None;
    }
    synonyms_chirho.get(name_chirho).or_else(|| {
        name_chirho
            .rsplit_once('.')
            .and_then(|(_prefix_chirho, bare_name_chirho)| synonyms_chirho.get(bare_name_chirho))
    })
}
