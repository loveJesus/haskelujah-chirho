// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Type environment
//!
//! Maps names to their type schemes. Supports lexical scoping via
//! a scope stack that can be pushed/popped for let bindings and lambdas.

use std::collections::HashMap;

use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, TyVarChirho};

/// A type environment: maps variable names to type schemes.
#[derive(Debug, Clone, Default)]
pub struct TyEnvChirho {
    /// Stack of scopes. The last entry is the innermost scope.
    scopes_chirho: Vec<HashMap<String, SchemeChirho>>,
}

impl TyEnvChirho {
    /// Create a new empty environment with one scope.
    pub fn new_chirho() -> Self {
        Self {
            scopes_chirho: vec![HashMap::new()],
        }
    }

    /// Push a new (empty) scope for a nested binding context.
    pub fn push_scope_chirho(&mut self) {
        self.scopes_chirho.push(HashMap::new());
    }

    /// Pop the innermost scope.
    pub fn pop_scope_chirho(&mut self) {
        if self.scopes_chirho.len() > 1 {
            self.scopes_chirho.pop();
        }
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

    /// Collect all free type variables across every binding in the environment.
    pub fn free_vars_chirho(&self) -> Vec<TyVarChirho> {
        let mut vars_chirho = Vec::new();
        for scope_chirho in &self.scopes_chirho {
            for scheme_chirho in scope_chirho.values() {
                vars_chirho.extend(scheme_chirho.free_vars_chirho());
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

    /// Apply a substitution to every type scheme in every scope.
    pub fn apply_subst_chirho(&mut self, subst_chirho: &SubstChirho) {
        for scope_chirho in &mut self.scopes_chirho {
            for scheme_chirho in scope_chirho.values_mut() {
                *scheme_chirho = subst_chirho.apply_scheme_chirho(scheme_chirho);
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
