// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Type substitution
//!
//! A substitution maps type variables to types. Applying a substitution
//! replaces every occurrence of each mapped variable with its image.

use std::collections::HashMap;

use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

/// A substitution: a finite mapping from type variables to types.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SubstChirho {
    map_chirho: HashMap<TyVarChirho, TyChirho>,
}

impl SubstChirho {
    /// The empty substitution.
    pub fn empty_chirho() -> Self {
        Self {
            map_chirho: HashMap::new(),
        }
    }

    /// Create a singleton substitution `[v ↦ ty]`.
    pub fn singleton_chirho(var_chirho: TyVarChirho, ty_chirho: TyChirho) -> Self {
        let mut map_chirho = HashMap::new();
        map_chirho.insert(var_chirho, ty_chirho);
        Self { map_chirho }
    }

    /// Look up a variable in this substitution.
    pub fn lookup_chirho(&self, var_chirho: &TyVarChirho) -> Option<&TyChirho> {
        self.map_chirho.get(var_chirho)
    }

    /// Insert a mapping `var ↦ ty`.
    pub fn insert_chirho(&mut self, var_chirho: TyVarChirho, ty_chirho: TyChirho) {
        self.map_chirho.insert(var_chirho, ty_chirho);
    }

    /// Number of bindings in this substitution.
    pub fn len_chirho(&self) -> usize {
        self.map_chirho.len()
    }

    /// Whether this substitution is empty.
    pub fn is_empty_chirho(&self) -> bool {
        self.map_chirho.is_empty()
    }

    /// Compose two substitutions: `self ∘ other`.
    ///
    /// `(s1 ∘ s2)(t) = s1(s2(t))`
    ///
    /// This applies `self` to every image in `other`, then merges in
    /// bindings from `self` that aren't overridden by `other`.
    pub fn compose_chirho(&self, other_chirho: &SubstChirho) -> SubstChirho {
        let mut result_chirho = SubstChirho::empty_chirho();

        // Apply self to every image in other
        for (var_chirho, ty_chirho) in &other_chirho.map_chirho {
            result_chirho
                .map_chirho
                .insert(*var_chirho, self.apply_ty_chirho(ty_chirho));
        }

        // Add bindings from self that aren't in other
        for (var_chirho, ty_chirho) in &self.map_chirho {
            result_chirho
                .map_chirho
                .entry(*var_chirho)
                .or_insert_with(|| ty_chirho.clone());
        }

        result_chirho
    }

    /// Merge another substitution into this one. Returns `true` if
    /// consistent (no conflicting bindings), `false` otherwise.
    /// On success, `self` contains the union of both substitutions.
    pub fn merge_chirho(&mut self, other_chirho: &SubstChirho) -> bool {
        for (var_chirho, ty_chirho) in &other_chirho.map_chirho {
            if let Some(existing_chirho) = self.map_chirho.get(var_chirho) {
                if existing_chirho != ty_chirho {
                    return false;
                }
            } else {
                self.map_chirho.insert(*var_chirho, ty_chirho.clone());
            }
        }
        true
    }

    /// Apply this substitution to a type.
    pub fn apply_ty_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        match ty_chirho {
            TyChirho::VarChirho(v_chirho) => self
                .map_chirho
                .get(v_chirho)
                .cloned()
                .unwrap_or_else(|| ty_chirho.clone()),
            TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => ty_chirho.clone(),
            TyChirho::AppChirho(f_chirho, a_chirho) => TyChirho::AppChirho(
                Box::new(self.apply_ty_chirho(f_chirho)),
                Box::new(self.apply_ty_chirho(a_chirho)),
            ),
            TyChirho::FunChirho(a_chirho, b_chirho, m_chirho) => TyChirho::FunChirho(
                Box::new(self.apply_ty_chirho(a_chirho)),
                Box::new(self.apply_ty_chirho(b_chirho)),
                *m_chirho,
            ),
            TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
                elems_chirho
                    .iter()
                    .map(|e_chirho| self.apply_ty_chirho(e_chirho))
                    .collect(),
            ),
            TyChirho::ListChirho(inner_chirho) => {
                TyChirho::ListChirho(Box::new(self.apply_ty_chirho(inner_chirho)))
            }
            TyChirho::ForallChirho { vars_chirho, body_chirho } => {
                // Don't substitute bound variables — restrict the substitution
                let mut restricted_chirho = self.clone();
                for v_chirho in vars_chirho {
                    restricted_chirho.map_chirho.remove(v_chirho);
                }
                TyChirho::ForallChirho {
                    vars_chirho: vars_chirho.clone(),
                    body_chirho: Box::new(restricted_chirho.apply_ty_chirho(body_chirho)),
                }
            }
        }
    }

    /// Apply this substitution to a type scheme (only substitutes free vars).
    pub fn apply_scheme_chirho(&self, scheme_chirho: &SchemeChirho) -> SchemeChirho {
        // Remove bound variables from the substitution before applying
        let mut restricted_chirho = self.clone();
        for v_chirho in &scheme_chirho.vars_chirho {
            restricted_chirho.map_chirho.remove(v_chirho);
        }
        SchemeChirho {
            vars_chirho: scheme_chirho.vars_chirho.clone(),
            preds_chirho: scheme_chirho
                .preds_chirho
                .iter()
                .map(|p_chirho| crate::ty_chirho::SchemePredChirho {
                    class_name_chirho: p_chirho.class_name_chirho.clone(),
                    ty_chirho: restricted_chirho.apply_ty_chirho(&p_chirho.ty_chirho),
                    extra_tys_chirho: p_chirho.extra_tys_chirho.iter().map(|t_chirho| restricted_chirho.apply_ty_chirho(t_chirho)).collect(),
                })
                .collect(),
            ty_chirho: restricted_chirho.apply_ty_chirho(&scheme_chirho.ty_chirho),
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn empty_subst_is_identity_chirho() {
        let subst_chirho = SubstChirho::empty_chirho();
        let ty_chirho = TyChirho::VarChirho(TyVarChirho(0));
        assert_eq!(subst_chirho.apply_ty_chirho(&ty_chirho), ty_chirho);
    }

    #[test]
    fn singleton_replaces_var_chirho() {
        let subst_chirho =
            SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::int_chirho());
        let ty_chirho = TyChirho::VarChirho(TyVarChirho(0));
        assert_eq!(subst_chirho.apply_ty_chirho(&ty_chirho), TyChirho::int_chirho());
    }

    #[test]
    fn subst_leaves_other_vars_alone_chirho() {
        let subst_chirho =
            SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::int_chirho());
        let ty_chirho = TyChirho::VarChirho(TyVarChirho(1));
        assert_eq!(
            subst_chirho.apply_ty_chirho(&ty_chirho),
            TyChirho::VarChirho(TyVarChirho(1))
        );
    }

    #[test]
    fn subst_through_fun_chirho() {
        let subst_chirho =
            SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::int_chirho());
        let ty_chirho = TyChirho::fun_chirho(
            TyChirho::VarChirho(TyVarChirho(0)),
            TyChirho::VarChirho(TyVarChirho(1)),
        );
        let result_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(
            result_chirho,
            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::VarChirho(TyVarChirho(1)))
        );
    }

    #[test]
    fn compose_applies_outer_to_inner_chirho() {
        // s1 = [t1 ↦ Int], s2 = [t0 ↦ t1]
        // (s1 ∘ s2)(t0) = s1(s2(t0)) = s1(t1) = Int
        let s1_chirho =
            SubstChirho::singleton_chirho(TyVarChirho(1), TyChirho::int_chirho());
        let s2_chirho = SubstChirho::singleton_chirho(
            TyVarChirho(0),
            TyChirho::VarChirho(TyVarChirho(1)),
        );
        let composed_chirho = s1_chirho.compose_chirho(&s2_chirho);

        let ty_chirho = TyChirho::VarChirho(TyVarChirho(0));
        assert_eq!(
            composed_chirho.apply_ty_chirho(&ty_chirho),
            TyChirho::int_chirho()
        );
    }

    #[test]
    fn apply_scheme_respects_bound_vars_chirho() {
        // forall t0. t0 -> t1, substitution [t0 ↦ Int, t1 ↦ Bool]
        // Should become forall t0. t0 -> Bool (t0 is bound, not substituted)
        let subst_chirho = {
            let mut s_chirho = SubstChirho::empty_chirho();
            s_chirho.insert_chirho(TyVarChirho(0), TyChirho::int_chirho());
            s_chirho.insert_chirho(TyVarChirho(1), TyChirho::bool_chirho());
            s_chirho
        };
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![TyVarChirho(0)],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(0)),
                TyChirho::VarChirho(TyVarChirho(1)),
            ),
        };
        let result_chirho = subst_chirho.apply_scheme_chirho(&scheme_chirho);
        assert_eq!(
            result_chirho.ty_chirho,
            TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(0)),
                TyChirho::bool_chirho(),
            )
        );
    }

    #[test]
    fn merge_consistent_chirho() {
        let mut s1_chirho =
            SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::int_chirho());
        let s2_chirho =
            SubstChirho::singleton_chirho(TyVarChirho(1), TyChirho::bool_chirho());
        assert!(s1_chirho.merge_chirho(&s2_chirho));
        assert_eq!(
            s1_chirho.apply_ty_chirho(&TyChirho::VarChirho(TyVarChirho(0))),
            TyChirho::int_chirho()
        );
        assert_eq!(
            s1_chirho.apply_ty_chirho(&TyChirho::VarChirho(TyVarChirho(1))),
            TyChirho::bool_chirho()
        );
    }

    #[test]
    fn merge_same_binding_ok_chirho() {
        let mut s1_chirho =
            SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::int_chirho());
        let s2_chirho =
            SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::int_chirho());
        assert!(s1_chirho.merge_chirho(&s2_chirho));
    }

    #[test]
    fn merge_conflicting_fails_chirho() {
        let mut s1_chirho =
            SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::int_chirho());
        let s2_chirho =
            SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::bool_chirho());
        assert!(!s1_chirho.merge_chirho(&s2_chirho));
    }
}
