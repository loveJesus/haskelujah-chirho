// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Type environment
//!
//! Maps names to their type schemes. Supports lexical scoping via
//! a scope stack that can be pushed/popped for let bindings and lambdas.

use std::collections::HashMap;

use crate::skolem_chirho::rewrite_skolems_chirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

/// A lexical entry is either a value or an explicit absence. A masked field
/// label must stop lookup before an outer Prelude seed can be considered.
#[derive(Debug, Clone)]
enum EnvEntryChirho {
    ValueChirho(SchemeChirho),
    MaskedChirho,
}

impl EnvEntryChirho {
    fn scheme_chirho(&self) -> Option<&SchemeChirho> {
        match self {
            Self::ValueChirho(scheme_chirho) => Some(scheme_chirho),
            Self::MaskedChirho => None,
        }
    }
}

/// A type environment: maps variable names to type schemes.
///
/// Each scope also carries the *type refinements* (local given equalities
/// `skolem ~ type`, see `skolem_chirho`) learned by the patterns bound in
/// that scope, so a GADT refinement disappears exactly when the pattern
/// variables it came with go out of scope.
#[derive(Debug, Clone, Default)]
pub struct TyEnvChirho {
    /// Stack of scopes. The last entry is the innermost scope.
    scopes_chirho: Vec<HashMap<String, EnvEntryChirho>>,
    /// One refinement list per scope, parallel to `scopes_chirho`.
    refinements_chirho: Vec<Vec<(String, TyChirho)>>,
}

impl TyEnvChirho {
    /// Create a new empty environment with one scope.
    pub fn new_chirho() -> Self {
        Self {
            scopes_chirho: vec![HashMap::new()],
            refinements_chirho: vec![Vec::new()],
        }
    }

    /// Push a new (empty) scope for a nested binding context.
    pub fn push_scope_chirho(&mut self) {
        self.scopes_chirho.push(HashMap::new());
        self.refinements_chirho.push(Vec::new());
    }

    /// Pop the innermost scope, dropping its bindings and its refinements.
    pub fn pop_scope_chirho(&mut self) {
        if self.scopes_chirho.len() > 1 {
            self.scopes_chirho.pop();
            self.refinements_chirho.pop();
        }
    }

    /// Record a local given equality `skolem ~ ty` in the innermost scope.
    /// workflow: language-features-chirho/rigid-type-variables-chirho
    pub fn add_refinement_chirho(&mut self, skolem_chirho: String, ty_chirho: TyChirho) {
        if self.refinements_chirho.is_empty() {
            self.refinements_chirho.push(Vec::new());
        }
        if let Some(scope_chirho) = self.refinements_chirho.last_mut() {
            scope_chirho.push((skolem_chirho, ty_chirho));
        }
    }

    /// Whether any scope currently holds a refinement (fast path guard).
    pub fn has_refinements_chirho(&self) -> bool {
        self.refinements_chirho
            .iter()
            .any(|scope_chirho| !scope_chirho.is_empty())
    }

    /// The type a skolem is currently refined to, innermost scope first.
    pub fn lookup_refinement_chirho(&self, skolem_chirho: &str) -> Option<&TyChirho> {
        for scope_chirho in self.refinements_chirho.iter().rev() {
            if let Some((_name_chirho, ty_chirho)) = scope_chirho
                .iter()
                .rev()
                .find(|(name_chirho, _ty_chirho)| name_chirho == skolem_chirho)
            {
                return Some(ty_chirho);
            }
        }
        None
    }

    /// Rewrite every refined skolem in `ty_chirho` to its refinement (one
    /// pass; callers iterate to a fixpoint as part of normalization).
    pub fn apply_refinements_chirho(
        &self,
        ty_chirho: &TyChirho,
    ) -> Result<TyChirho, crate::skolem_chirho::SkolemResourceExhaustedChirho> {
        if !self.has_refinements_chirho() {
            return Ok(ty_chirho.clone());
        }
        rewrite_skolems_chirho(ty_chirho, &mut |name_chirho| {
            self.lookup_refinement_chirho(name_chirho).cloned()
        })
    }

    /// Bind a name to a type scheme in the current (innermost) scope.
    pub fn bind_chirho(&mut self, name_chirho: String, scheme_chirho: SchemeChirho) {
        if let Some(scope_chirho) = self.scopes_chirho.last_mut() {
            scope_chirho.insert(name_chirho, EnvEntryChirho::ValueChirho(scheme_chirho));
        }
    }

    /// Make this name unavailable in the current scope. A later real binding
    /// replaces the mask, and an inner lexical binding shadows it normally.
    pub fn mask_chirho(&mut self, name_chirho: String) {
        if let Some(scope_chirho) = self.scopes_chirho.last_mut() {
            scope_chirho.insert(name_chirho, EnvEntryChirho::MaskedChirho);
        }
    }

    /// Whether the first lexical entry is an explicit absence. Qualified
    /// lookup uses this before considering any suffix or bare-name fallback.
    pub fn is_masked_chirho(&self, name_chirho: &str) -> bool {
        self.scopes_chirho
            .iter()
            .rev()
            .find_map(|scope_chirho| scope_chirho.get(name_chirho))
            .is_some_and(|entry_chirho| matches!(entry_chirho, EnvEntryChirho::MaskedChirho))
    }

    /// Remove a name from the current (innermost) scope.
    pub fn remove_chirho(&mut self, name_chirho: &str) {
        if let Some(scope_chirho) = self.scopes_chirho.last_mut() {
            scope_chirho.remove(name_chirho);
        }
    }

    /// Look up a name in the outermost scope alone: the module level and the
    /// builtins, never a local binder (row 598).
    /// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
    pub fn lookup_outermost_chirho(&self, name_chirho: &str) -> Option<&SchemeChirho> {
        self.scopes_chirho
            .first()
            .and_then(|scope_chirho| scope_chirho.get(name_chirho))
            .and_then(EnvEntryChirho::scheme_chirho)
    }

    /// Look up a name, searching from innermost to outermost scope.
    pub fn lookup_chirho(&self, name_chirho: &str) -> Option<&SchemeChirho> {
        for scope_chirho in self.scopes_chirho.iter().rev() {
            if let Some(entry_chirho) = scope_chirho.get(name_chirho) {
                return entry_chirho.scheme_chirho();
            }
        }
        None
    }

    /// Collect all free type variables across every binding in the environment
    /// (refinement targets included: a variable a skolem is refined to is as
    /// monomorphic as the pattern that introduced it).
    pub fn free_vars_chirho(&self) -> Vec<TyVarChirho> {
        let mut vars_chirho = Vec::new();
        for (_, scheme_chirho) in self.all_bindings_chirho() {
            vars_chirho.extend(scheme_chirho.free_vars_chirho());
        }
        for scope_chirho in &self.refinements_chirho {
            for (_name_chirho, ty_chirho) in scope_chirho {
                vars_chirho.extend(ty_chirho.free_vars_chirho());
            }
        }
        vars_chirho.sort();
        vars_chirho.dedup();
        vars_chirho
    }

    /// Borrow refinement targets for a scope-bounded captured-variable walk.
    pub(crate) fn refinement_types_chirho(&self) -> impl Iterator<Item = &TyChirho> {
        self.refinements_chirho
            .iter()
            .flat_map(|scope_chirho| scope_chirho.iter().map(|(_, ty_chirho)| ty_chirho))
    }

    /// Borrow every entry in innermost-first order, including tombstones.
    /// A budgeted caller charges and records the first name before skipping a
    /// missing scheme; otherwise a same-named outer value could reappear.
    pub(crate) fn bindings_in_scope_order_chirho(
        &self,
    ) -> impl Iterator<Item = (&String, Option<&SchemeChirho>)> {
        self.scopes_chirho.iter().rev().flat_map(|scope_chirho| {
            scope_chirho
                .iter()
                .map(|(name_chirho, entry_chirho)| (name_chirho, entry_chirho.scheme_chirho()))
        })
    }

    /// Stable locations within the existing lexical stack, used only while a
    /// checked scope projects its solved public images before evidence leaves.
    pub(crate) fn located_bindings_chirho(
        &self,
    ) -> impl Iterator<Item = (usize, &String, &SchemeChirho)> {
        self.scopes_chirho
            .iter()
            .enumerate()
            .flat_map(|(index_chirho, scope_chirho)| {
                scope_chirho
                    .iter()
                    .filter_map(move |(name_chirho, entry_chirho)| {
                        entry_chirho
                            .scheme_chirho()
                            .map(|scheme_chirho| (index_chirho, name_chirho, scheme_chirho))
                    })
            })
    }

    pub(crate) fn replace_located_binding_chirho(
        &mut self,
        index_chirho: usize,
        name_chirho: &str,
        scheme_chirho: SchemeChirho,
    ) {
        if let Some(EnvEntryChirho::ValueChirho(entry_chirho)) = self
            .scopes_chirho
            .get_mut(index_chirho)
            .and_then(|scope_chirho| scope_chirho.get_mut(name_chirho))
        {
            *entry_chirho = scheme_chirho;
        }
    }

    /// Iterate over usable visible bindings, innermost first. A masked entry
    /// suppresses an outer value without appearing as an exported scheme.
    pub fn all_bindings_chirho(&self) -> Vec<(&String, &SchemeChirho)> {
        let mut seen_chirho = std::collections::HashSet::new();
        let mut result_chirho = Vec::new();
        for (name_chirho, scheme_chirho) in self.bindings_in_scope_order_chirho() {
            if seen_chirho.insert(name_chirho)
                && let Some(scheme_chirho) = scheme_chirho
            {
                result_chirho.push((name_chirho, scheme_chirho));
            }
        }
        result_chirho
    }

    /// Collect usable visible names, with masked entries suppressing outer names.
    pub fn all_names_chirho(&self) -> Vec<&str> {
        let mut seen_chirho = std::collections::HashSet::new();
        let mut names_chirho = Vec::new();
        for (name_chirho, scheme_chirho) in self.bindings_in_scope_order_chirho() {
            if seen_chirho.insert(name_chirho.as_str()) && scheme_chirho.is_some() {
                names_chirho.push(name_chirho.as_str());
            }
        }
        names_chirho
    }

    /// Apply a substitution to every type scheme in every scope, and to the
    /// refinement targets.
    pub fn apply_subst_chirho(&mut self, subst_chirho: &SubstChirho) {
        for scope_chirho in &mut self.scopes_chirho {
            for entry_chirho in scope_chirho.values_mut() {
                if let EnvEntryChirho::ValueChirho(scheme_chirho) = entry_chirho {
                    *scheme_chirho = subst_chirho.apply_scheme_chirho(scheme_chirho);
                }
            }
        }
        for scope_chirho in &mut self.refinements_chirho {
            for (_name_chirho, ty_chirho) in scope_chirho.iter_mut() {
                *ty_chirho = subst_chirho.apply_ty_chirho(ty_chirho);
            }
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::ty_chirho::TyChirho;

    #[test]
    fn bind_and_lookup_chirho() {
        let mut env_chirho = TyEnvChirho::new_chirho();
        env_chirho.bind_chirho(
            "x".to_string(),
            SchemeChirho::mono_chirho(TyChirho::int_chirho()),
        );
        assert!(env_chirho.lookup_chirho("x").is_some());
        assert!(env_chirho.lookup_chirho("y").is_none());
    }

    #[test]
    fn scoping_shadows_chirho() {
        let mut env_chirho = TyEnvChirho::new_chirho();
        env_chirho.bind_chirho(
            "x".to_string(),
            SchemeChirho::mono_chirho(TyChirho::int_chirho()),
        );

        env_chirho.push_scope_chirho();
        env_chirho.bind_chirho(
            "x".to_string(),
            SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
        );

        // Inner scope shadows outer
        assert_eq!(
            env_chirho.lookup_chirho("x").unwrap().ty_chirho,
            TyChirho::bool_chirho()
        );

        env_chirho.pop_scope_chirho();

        // Outer binding restored
        assert_eq!(
            env_chirho.lookup_chirho("x").unwrap().ty_chirho,
            TyChirho::int_chirho()
        );
    }

    #[test]
    fn free_vars_across_scopes_chirho() {
        let mut env_chirho = TyEnvChirho::new_chirho();
        let a_chirho = TyVarChirho(0);
        env_chirho.bind_chirho(
            "x".to_string(),
            SchemeChirho::mono_chirho(TyChirho::VarChirho(a_chirho)),
        );

        env_chirho.push_scope_chirho();
        let b_chirho = TyVarChirho(1);
        env_chirho.bind_chirho(
            "y".to_string(),
            SchemeChirho::mono_chirho(TyChirho::VarChirho(b_chirho)),
        );

        let fvs_chirho = env_chirho.free_vars_chirho();
        assert!(fvs_chirho.contains(&a_chirho));
        assert!(fvs_chirho.contains(&b_chirho));
    }

    #[test]
    fn refinements_are_scoped_and_substituted_chirho() {
        let mut env_chirho = TyEnvChirho::new_chirho();
        let skolem_chirho = crate::skolem_chirho::skolem_name_chirho("a", 1);
        assert!(!env_chirho.has_refinements_chirho());
        env_chirho.push_scope_chirho();
        env_chirho
            .add_refinement_chirho(skolem_chirho.clone(), TyChirho::VarChirho(TyVarChirho(5)));
        assert!(env_chirho.has_refinements_chirho());
        let refined_chirho = env_chirho
            .apply_refinements_chirho(&TyChirho::ForallVarChirho(skolem_chirho.clone()))
            .expect("bounded refinement");
        assert_eq!(refined_chirho, TyChirho::VarChirho(TyVarChirho(5)));
        // Later substitution of the target is visible through the refinement.
        env_chirho.apply_subst_chirho(&SubstChirho::singleton_chirho(
            TyVarChirho(5),
            TyChirho::int_chirho(),
        ));
        assert_eq!(
            env_chirho.lookup_refinement_chirho(&skolem_chirho),
            Some(&TyChirho::int_chirho())
        );
        env_chirho.pop_scope_chirho();
        assert!(!env_chirho.has_refinements_chirho());
        assert_eq!(env_chirho.lookup_refinement_chirho(&skolem_chirho), None);
    }

    #[test]
    fn apply_subst_updates_all_scopes_chirho() {
        let mut env_chirho = TyEnvChirho::new_chirho();
        let a_chirho = TyVarChirho(0);
        env_chirho.bind_chirho(
            "x".to_string(),
            SchemeChirho::mono_chirho(TyChirho::VarChirho(a_chirho)),
        );

        let subst_chirho = SubstChirho::singleton_chirho(a_chirho, TyChirho::int_chirho());
        env_chirho.apply_subst_chirho(&subst_chirho);

        assert_eq!(
            env_chirho.lookup_chirho("x").unwrap().ty_chirho,
            TyChirho::int_chirho()
        );
    }
    #[test]
    fn a_mask_stops_lookup_and_export_but_a_real_lexical_value_shadows_it_chirho() {
        let mut env_chirho = TyEnvChirho::new_chirho();
        env_chirho.bind_chirho(
            "head".to_owned(),
            SchemeChirho::mono_chirho(TyChirho::VarChirho(TyVarChirho(91))),
        );
        env_chirho.push_scope_chirho();
        env_chirho.mask_chirho("head".to_owned());
        assert!(env_chirho.lookup_chirho("head").is_none());
        assert!(env_chirho.is_masked_chirho("head"));
        assert!(!env_chirho.all_names_chirho().contains(&"head"));
        assert!(env_chirho.all_bindings_chirho().is_empty());
        assert!(!env_chirho.free_vars_chirho().contains(&TyVarChirho(91)));
        let entries_chirho = env_chirho
            .bindings_in_scope_order_chirho()
            .collect::<Vec<_>>();
        assert_eq!(entries_chirho.len(), 2);
        assert!(entries_chirho[0].1.is_none());
        env_chirho.push_scope_chirho();
        env_chirho.bind_chirho(
            "head".to_owned(),
            SchemeChirho::mono_chirho(TyChirho::int_chirho()),
        );
        assert!(!env_chirho.is_masked_chirho("head"));
        assert_eq!(
            env_chirho.lookup_chirho("head").unwrap().ty_chirho,
            TyChirho::int_chirho()
        );
        env_chirho.pop_scope_chirho();
        assert!(env_chirho.is_masked_chirho("head"));
        env_chirho.bind_chirho(
            "head".to_owned(),
            SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
        );
        assert!(!env_chirho.is_masked_chirho("head"));
        assert_eq!(
            env_chirho.lookup_chirho("head").unwrap().ty_chirho,
            TyChirho::bool_chirho()
        );
    }

    #[test]
    fn a_qualified_mask_is_exact_and_substitution_preserves_it_chirho() {
        let mut env_chirho = TyEnvChirho::new_chirho();
        env_chirho.bind_chirho(
            "PChirho.head".to_owned(),
            SchemeChirho::mono_chirho(TyChirho::int_chirho()),
        );
        env_chirho.mask_chirho("HChirho.head".to_owned());
        env_chirho.apply_subst_chirho(&SubstChirho::empty_chirho());
        assert!(env_chirho.lookup_chirho("HChirho.head").is_none());
        assert!(env_chirho.is_masked_chirho("HChirho.head"));
        assert!(env_chirho.lookup_chirho("PChirho.head").is_some());
    }
}
