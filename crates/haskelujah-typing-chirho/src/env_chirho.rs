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

/// A type environment: maps variable names to type schemes.
///
/// Each scope also carries the *type refinements* (local given equalities
/// `skolem ~ type`, see `skolem_chirho`) learned by the patterns bound in
/// that scope, so a GADT refinement disappears exactly when the pattern
/// variables it came with go out of scope.
#[derive(Debug, Clone, Default)]
pub struct TyEnvChirho {
    /// Stack of scopes. The last entry is the innermost scope.
    scopes_chirho: Vec<HashMap<String, SchemeChirho>>,
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
    pub fn apply_refinements_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        if !self.has_refinements_chirho() {
            return ty_chirho.clone();
        }
        rewrite_skolems_chirho(ty_chirho, &mut |name_chirho| {
            self.lookup_refinement_chirho(name_chirho).cloned()
        })
    }

    /// Bind a name to a type scheme in the current (innermost) scope.
    pub fn bind_chirho(&mut self, name_chirho: String, scheme_chirho: SchemeChirho) {
        if let Some(scope_chirho) = self.scopes_chirho.last_mut() {
            scope_chirho.insert(name_chirho, scheme_chirho);
        }
    }

    /// Remove a name from the current (innermost) scope.
    pub fn remove_chirho(&mut self, name_chirho: &str) {
        if let Some(scope_chirho) = self.scopes_chirho.last_mut() {
            scope_chirho.remove(name_chirho);
        }
    }

    /// Look up a name, searching from innermost to outermost scope.
    pub fn lookup_chirho(&self, name_chirho: &str) -> Option<&SchemeChirho> {
        for scope_chirho in self.scopes_chirho.iter().rev() {
            if let Some(scheme_chirho) = scope_chirho.get(name_chirho) {
                return Some(scheme_chirho);
            }
        }
        None
    }

    /// Collect all free type variables across every binding in the environment
    /// (refinement targets included: a variable a skolem is refined to is as
    /// monomorphic as the pattern that introduced it).
    pub fn free_vars_chirho(&self) -> Vec<TyVarChirho> {
        let mut vars_chirho = Vec::new();
        for scope_chirho in &self.scopes_chirho {
            for scheme_chirho in scope_chirho.values() {
                vars_chirho.extend(scheme_chirho.free_vars_chirho());
            }
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

    /// Iterate over all bindings across all scopes (outermost first).
    /// If a name appears in multiple scopes, only the innermost is yielded.
    pub fn all_bindings_chirho(&self) -> Vec<(&String, &SchemeChirho)> {
        let mut seen_chirho = std::collections::HashSet::new();
        let mut result_chirho = Vec::new();
        for scope_chirho in self.scopes_chirho.iter().rev() {
            for (name_chirho, scheme_chirho) in scope_chirho {
                if seen_chirho.insert(name_chirho) {
                    result_chirho.push((name_chirho, scheme_chirho));
                }
            }
        }
        result_chirho
    }

    /// Collect all visible names across all scopes (deduplicated, innermost wins).
    pub fn all_names_chirho(&self) -> Vec<&str> {
        let mut seen_chirho = std::collections::HashSet::new();
        let mut names_chirho = Vec::new();
        for scope_chirho in self.scopes_chirho.iter().rev() {
            for name_chirho in scope_chirho.keys() {
                if seen_chirho.insert(name_chirho.as_str()) {
                    names_chirho.push(name_chirho.as_str());
                }
            }
        }
        names_chirho
    }

    /// Apply a substitution to every type scheme in every scope, and to the
    /// refinement targets.
    pub fn apply_subst_chirho(&mut self, subst_chirho: &SubstChirho) {
        for scope_chirho in &mut self.scopes_chirho {
            for scheme_chirho in scope_chirho.values_mut() {
                *scheme_chirho = subst_chirho.apply_scheme_chirho(scheme_chirho);
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
        let refined_chirho =
            env_chirho.apply_refinements_chirho(&TyChirho::ForallVarChirho(skolem_chirho.clone()));
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
}
