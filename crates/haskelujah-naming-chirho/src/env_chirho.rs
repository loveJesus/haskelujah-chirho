// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Name environment
//!
//! Tracks which names are in scope and their definitions. Uses a stack of
//! scopes to handle nested binding contexts (let, where, lambda, case alts).

use std::collections::HashMap;

use haskelujah_ast_chirho::name_chirho::DefIdChirho;
use haskelujah_span_chirho::SpanChirho;

/// What kind of thing a name refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceChirho {
    /// A value-level name (variable, function, data constructor).
    ValueChirho,
    /// A type-level name (type constructor, type variable, type class).
    TypeChirho,
}

/// Information about a definition site.
#[derive(Debug, Clone)]
pub struct DefInfoChirho {
    pub def_id_chirho: DefIdChirho,
    pub namespace_chirho: NamespaceChirho,
    pub span_chirho: SpanChirho,
    /// Whether this is an imported name.
    pub imported_chirho: bool,
}

/// A single scope level — maps names to their definitions.
#[derive(Debug, Clone, Default)]
pub struct ScopeChirho {
    bindings_chirho: HashMap<String, Vec<DefInfoChirho>>,
}

impl ScopeChirho {
    pub fn new_chirho() -> Self {
        Self::default()
    }

    pub fn bind_chirho(&mut self, name_chirho: String, info_chirho: DefInfoChirho) {
        self.bindings_chirho
            .entry(name_chirho)
            .or_default()
            .push(info_chirho);
    }

    pub fn lookup_chirho(&self, name_chirho: &str) -> Option<&[DefInfoChirho]> {
        self.bindings_chirho
            .get(name_chirho)
            .map(|v_chirho| v_chirho.as_slice())
    }

    pub fn lookup_in_namespace_chirho(
        &self,
        name_chirho: &str,
        namespace_chirho: NamespaceChirho,
    ) -> Option<&DefInfoChirho> {
        self.bindings_chirho
            .get(name_chirho)?
            .iter()
            .rfind(|info_chirho| info_chirho.namespace_chirho == namespace_chirho)
    }
}

/// A stack of scopes forming the name environment.
///
/// Lookup walks from the innermost scope outward, implementing
/// Haskell's lexical scoping rules. Also supports qualified name
/// lookup via module-qualified mappings (e.g. `Data.List.sort`).
#[derive(Debug, Clone)]
pub struct NameEnvChirho {
    scopes_chirho: Vec<ScopeChirho>,
    next_def_id_chirho: u32,
    /// Qualified name lookup: "Module.name" → DefInfoChirho.
    /// Populated by import resolution for qualified imports.
    qualified_chirho: HashMap<String, Vec<DefInfoChirho>>,
}

impl NameEnvChirho {
    pub fn new_chirho() -> Self {
        Self {
            scopes_chirho: vec![ScopeChirho::new_chirho()], // top-level scope
            next_def_id_chirho: 0,
            qualified_chirho: HashMap::new(),
        }
    }

    /// Push a new nested scope (for let, where, lambda, case alt).
    pub fn push_scope_chirho(&mut self) {
        self.scopes_chirho.push(ScopeChirho::new_chirho());
    }

    /// Pop the innermost scope.
    pub fn pop_scope_chirho(&mut self) {
        assert!(
            self.scopes_chirho.len() > 1,
            "cannot pop the top-level scope"
        );
        self.scopes_chirho.pop();
    }

    /// Allocate a fresh `DefIdChirho`.
    pub fn fresh_def_id_chirho(&mut self) -> DefIdChirho {
        let id_chirho = DefIdChirho::new_chirho(self.next_def_id_chirho);
        self.next_def_id_chirho += 1;
        id_chirho
    }

    /// Bind a name in the current (innermost) scope.
    pub fn bind_chirho(
        &mut self,
        name_chirho: String,
        namespace_chirho: NamespaceChirho,
        span_chirho: SpanChirho,
    ) -> DefIdChirho {
        let def_id_chirho = self.fresh_def_id_chirho();
        let info_chirho = DefInfoChirho {
            def_id_chirho,
            namespace_chirho,
            span_chirho,
            imported_chirho: false,
        };
        self.scopes_chirho
            .last_mut()
            .expect("scope stack should never be empty")
            .bind_chirho(name_chirho, info_chirho);
        def_id_chirho
    }

    /// Bind an imported name in the current scope.
    pub fn bind_import_chirho(
        &mut self,
        name_chirho: String,
        namespace_chirho: NamespaceChirho,
        span_chirho: SpanChirho,
    ) -> DefIdChirho {
        let def_id_chirho = self.fresh_def_id_chirho();
        let info_chirho = DefInfoChirho {
            def_id_chirho,
            namespace_chirho,
            span_chirho,
            imported_chirho: true,
        };
        self.scopes_chirho
            .last_mut()
            .expect("scope stack should never be empty")
            .bind_chirho(name_chirho, info_chirho);
        def_id_chirho
    }

    /// Look up a name in the given namespace, searching from innermost to
    /// outermost scope.
    pub fn lookup_chirho(
        &self,
        name_chirho: &str,
        namespace_chirho: NamespaceChirho,
    ) -> Option<&DefInfoChirho> {
        for scope_chirho in self.scopes_chirho.iter().rev() {
            if let Some(info_chirho) =
                scope_chirho.lookup_in_namespace_chirho(name_chirho, namespace_chirho)
            {
                return Some(info_chirho);
            }
        }
        None
    }

    /// Look up a value-level name.
    pub fn lookup_value_chirho(&self, name_chirho: &str) -> Option<&DefInfoChirho> {
        self.lookup_chirho(name_chirho, NamespaceChirho::ValueChirho)
    }

    /// Look up a type-level name.
    pub fn lookup_type_chirho(&self, name_chirho: &str) -> Option<&DefInfoChirho> {
        self.lookup_chirho(name_chirho, NamespaceChirho::TypeChirho)
    }

    /// Bind a qualified name (e.g. `"Data.List.sort"`) for qualified
    /// import lookup.
    pub fn bind_qualified_chirho(
        &mut self,
        qualified_name_chirho: String,
        namespace_chirho: NamespaceChirho,
        span_chirho: SpanChirho,
    ) -> DefIdChirho {
        let def_id_chirho = self.fresh_def_id_chirho();
        let info_chirho = DefInfoChirho {
            def_id_chirho,
            namespace_chirho,
            span_chirho,
            imported_chirho: true,
        };
        self.qualified_chirho
            .entry(qualified_name_chirho)
            .or_default()
            .push(info_chirho);
        def_id_chirho
    }

    /// Look up a qualified name (e.g. `"Data.List"` + `"sort"`) in the
    /// qualified namespace.
    pub fn lookup_qualified_chirho(
        &self,
        qualifier_chirho: &str,
        name_chirho: &str,
        namespace_chirho: NamespaceChirho,
    ) -> Option<&DefInfoChirho> {
        let key_chirho = format!("{qualifier_chirho}.{name_chirho}");
        self.qualified_chirho.get(&key_chirho).and_then(|infos_chirho| {
            infos_chirho
                .iter()
                .find(|i_chirho| i_chirho.namespace_chirho == namespace_chirho)
        })
    }

    /// Collect all visible names in a given namespace across all scopes.
    pub fn all_names_in_namespace_chirho(&self, namespace_chirho: NamespaceChirho) -> Vec<&str> {
        let mut seen_chirho = std::collections::HashSet::new();
        let mut names_chirho = Vec::new();
        for scope_chirho in self.scopes_chirho.iter().rev() {
            for (name_chirho, infos_chirho) in &scope_chirho.bindings_chirho {
                if infos_chirho
                    .iter()
                    .any(|i_chirho| i_chirho.namespace_chirho == namespace_chirho)
                {
                    if seen_chirho.insert(name_chirho.as_str()) {
                        names_chirho.push(name_chirho.as_str());
                    }
                }
            }
        }
        names_chirho
    }

    /// How many definitions have been allocated.
    pub fn def_count_chirho(&self) -> u32 {
        self.next_def_id_chirho
    }

    /// Current scope depth (1 = top-level).
    pub fn depth_chirho(&self) -> usize {
        self.scopes_chirho.len()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn bind_and_lookup_chirho() {
        let mut env_chirho = NameEnvChirho::new_chirho();
        let id_chirho = env_chirho.bind_chirho(
            "foo".to_string(),
            NamespaceChirho::ValueChirho,
            SpanChirho::DUMMY_CHIRHO,
        );

        let found_chirho = env_chirho.lookup_value_chirho("foo");
        assert!(found_chirho.is_some());
        assert_eq!(found_chirho.unwrap().def_id_chirho, id_chirho);
    }

    #[test]
    fn nested_scopes_shadow_chirho() {
        let mut env_chirho = NameEnvChirho::new_chirho();
        let outer_id_chirho = env_chirho.bind_chirho(
            "x".to_string(),
            NamespaceChirho::ValueChirho,
            SpanChirho::DUMMY_CHIRHO,
        );

        env_chirho.push_scope_chirho();
        let inner_id_chirho = env_chirho.bind_chirho(
            "x".to_string(),
            NamespaceChirho::ValueChirho,
            SpanChirho::DUMMY_CHIRHO,
        );

        // Inner scope shadows outer
        let found_chirho = env_chirho.lookup_value_chirho("x").unwrap();
        assert_eq!(found_chirho.def_id_chirho, inner_id_chirho);

        env_chirho.pop_scope_chirho();

        // After popping, outer is visible again
        let found_chirho = env_chirho.lookup_value_chirho("x").unwrap();
        assert_eq!(found_chirho.def_id_chirho, outer_id_chirho);
    }

    #[test]
    fn separate_namespaces_chirho() {
        let mut env_chirho = NameEnvChirho::new_chirho();
        let val_id_chirho = env_chirho.bind_chirho(
            "Foo".to_string(),
            NamespaceChirho::ValueChirho,
            SpanChirho::DUMMY_CHIRHO,
        );
        let ty_id_chirho = env_chirho.bind_chirho(
            "Foo".to_string(),
            NamespaceChirho::TypeChirho,
            SpanChirho::DUMMY_CHIRHO,
        );

        // Same name in different namespaces
        assert_eq!(
            env_chirho.lookup_value_chirho("Foo").unwrap().def_id_chirho,
            val_id_chirho
        );
        assert_eq!(
            env_chirho.lookup_type_chirho("Foo").unwrap().def_id_chirho,
            ty_id_chirho
        );
    }

    #[test]
    fn undefined_name_returns_none_chirho() {
        let env_chirho = NameEnvChirho::new_chirho();
        assert!(env_chirho.lookup_value_chirho("nonexistent").is_none());
    }

    #[test]
    fn fresh_ids_are_unique_chirho() {
        let mut env_chirho = NameEnvChirho::new_chirho();
        let id1_chirho = env_chirho.fresh_def_id_chirho();
        let id2_chirho = env_chirho.fresh_def_id_chirho();
        let id3_chirho = env_chirho.fresh_def_id_chirho();
        assert_ne!(id1_chirho, id2_chirho);
        assert_ne!(id2_chirho, id3_chirho);
    }

    #[test]
    fn imported_names_chirho() {
        let mut env_chirho = NameEnvChirho::new_chirho();
        let _id_chirho = env_chirho.bind_import_chirho(
            "sort".to_string(),
            NamespaceChirho::ValueChirho,
            SpanChirho::DUMMY_CHIRHO,
        );

        let found_chirho = env_chirho.lookup_value_chirho("sort").unwrap();
        assert!(found_chirho.imported_chirho);
    }
}
