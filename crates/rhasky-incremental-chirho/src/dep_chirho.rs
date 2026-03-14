// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Dependency graph
//!
//! Tracks inter-module dependencies so the incremental session can determine
//! the transitive closure of modules that need recompilation when a source
//! file changes.
//!
//! The graph is directed: an edge `A → B` means "module A imports module B".
//! A topological sort yields a valid compilation order.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::fingerprint_chirho::FingerprintChirho;

/// A node in the dependency graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DepNodeChirho {
    /// Module name (e.g. `"Data.List"`).
    pub module_name_chirho: String,
    /// Fingerprint of the module's source text.
    pub source_fp_chirho: FingerprintChirho,
}

/// Directed dependency graph over modules.
#[derive(Debug, Clone, Default)]
pub struct DepGraphChirho {
    /// Module name → node.
    nodes_chirho: HashMap<String, DepNodeChirho>,
    /// Module name → set of module names it imports.
    edges_chirho: HashMap<String, HashSet<String>>,
    /// Reverse edges: module name → set of module names that import it.
    rev_edges_chirho: HashMap<String, HashSet<String>>,
}

impl DepGraphChirho {
    /// Create an empty graph.
    pub fn new_chirho() -> Self {
        Self::default()
    }

    /// Register a module in the graph.
    pub fn add_module_chirho(
        &mut self,
        module_name_chirho: &str,
        source_fp_chirho: FingerprintChirho,
    ) {
        let node_chirho = DepNodeChirho {
            module_name_chirho: module_name_chirho.to_string(),
            source_fp_chirho,
        };
        self.nodes_chirho
            .insert(module_name_chirho.to_string(), node_chirho);
        self.edges_chirho
            .entry(module_name_chirho.to_string())
            .or_default();
        self.rev_edges_chirho
            .entry(module_name_chirho.to_string())
            .or_default();
    }

    /// Record that `importer_chirho` imports `imported_chirho`.
    pub fn add_dep_chirho(&mut self, importer_chirho: &str, imported_chirho: &str) {
        self.edges_chirho
            .entry(importer_chirho.to_string())
            .or_default()
            .insert(imported_chirho.to_string());
        self.rev_edges_chirho
            .entry(imported_chirho.to_string())
            .or_default()
            .insert(importer_chirho.to_string());
    }

    /// Get the direct dependencies of a module.
    pub fn deps_of_chirho(&self, module_name_chirho: &str) -> Vec<&str> {
        self.edges_chirho
            .get(module_name_chirho)
            .map(|s_chirho| s_chirho.iter().map(|s_chirho| s_chirho.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get the direct reverse dependencies (dependents) of a module.
    pub fn dependents_of_chirho(&self, module_name_chirho: &str) -> Vec<&str> {
        self.rev_edges_chirho
            .get(module_name_chirho)
            .map(|s_chirho| s_chirho.iter().map(|s_chirho| s_chirho.as_str()).collect())
            .unwrap_or_default()
    }

    /// Look up a node by module name.
    pub fn node_chirho(&self, module_name_chirho: &str) -> Option<&DepNodeChirho> {
        self.nodes_chirho.get(module_name_chirho)
    }

    /// All module names in the graph.
    pub fn modules_chirho(&self) -> Vec<&str> {
        self.nodes_chirho.keys().map(|k_chirho| k_chirho.as_str()).collect()
    }

    /// Compute the transitive closure of modules that depend on `changed_chirho`.
    /// Returns the set of module names that need recompilation (includes `changed_chirho` itself).
    pub fn invalidated_by_chirho(&self, changed_chirho: &str) -> HashSet<String> {
        let mut visited_chirho = HashSet::new();
        let mut queue_chirho = VecDeque::new();
        queue_chirho.push_back(changed_chirho.to_string());
        visited_chirho.insert(changed_chirho.to_string());

        while let Some(current_chirho) = queue_chirho.pop_front() {
            if let Some(dependents_chirho) = self.rev_edges_chirho.get(&current_chirho) {
                for dep_chirho in dependents_chirho {
                    if visited_chirho.insert(dep_chirho.clone()) {
                        queue_chirho.push_back(dep_chirho.clone());
                    }
                }
            }
        }

        visited_chirho
    }

    /// Topological sort of all modules (dependencies before dependents).
    /// Returns `None` if there is a cycle.
    pub fn topo_sort_chirho(&self) -> Option<Vec<String>> {
        // Kahn's algorithm: dep_count tracks how many unresolved imports each module has.
        // Modules with 0 deps are ready to compile first.
        let mut dep_count_chirho: HashMap<&str, usize> = HashMap::new();
        for name_chirho in self.nodes_chirho.keys() {
            let count_chirho = self
                .edges_chirho
                .get(name_chirho.as_str())
                .map(|s_chirho| s_chirho.len())
                .unwrap_or(0);
            dep_count_chirho.insert(name_chirho.as_str(), count_chirho);
        }

        let mut queue_chirho: VecDeque<&str> = dep_count_chirho
            .iter()
            .filter(|(_, c_chirho)| **c_chirho == 0)
            .map(|(name_chirho, _)| *name_chirho)
            .collect();

        let mut result_chirho = Vec::new();

        while let Some(name_chirho) = queue_chirho.pop_front() {
            result_chirho.push(name_chirho.to_string());

            // For each module that imports `name_chirho`, decrement its dep count.
            if let Some(dependents_chirho) = self.rev_edges_chirho.get(name_chirho) {
                for dependent_chirho in dependents_chirho {
                    if let Some(count_chirho) = dep_count_chirho.get_mut(dependent_chirho.as_str())
                    {
                        *count_chirho -= 1;
                        if *count_chirho == 0 {
                            queue_chirho.push_back(dependent_chirho.as_str());
                        }
                    }
                }
            }
        }

        if result_chirho.len() == self.nodes_chirho.len() {
            Some(result_chirho)
        } else {
            None // cycle detected
        }
    }

    /// Number of modules in the graph.
    pub fn len_chirho(&self) -> usize {
        self.nodes_chirho.len()
    }

    /// Whether the graph is empty.
    pub fn is_empty_chirho(&self) -> bool {
        self.nodes_chirho.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn fp_chirho(s_chirho: &str) -> FingerprintChirho {
        FingerprintChirho::from_str_chirho(s_chirho)
    }

    #[test]
    fn empty_graph_chirho() {
        let graph_chirho = DepGraphChirho::new_chirho();
        assert!(graph_chirho.is_empty_chirho());
        assert_eq!(graph_chirho.len_chirho(), 0);
        assert_eq!(graph_chirho.topo_sort_chirho(), Some(vec![]));
    }

    #[test]
    fn single_module_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("Main", fp_chirho("main source"));
        assert_eq!(graph_chirho.len_chirho(), 1);
        assert!(graph_chirho.deps_of_chirho("Main").is_empty());
        assert_eq!(
            graph_chirho.topo_sort_chirho(),
            Some(vec!["Main".to_string()])
        );
    }

    #[test]
    fn linear_deps_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("A", fp_chirho("a"));
        graph_chirho.add_module_chirho("B", fp_chirho("b"));
        graph_chirho.add_module_chirho("C", fp_chirho("c"));
        // C imports B, B imports A
        graph_chirho.add_dep_chirho("C", "B");
        graph_chirho.add_dep_chirho("B", "A");

        let sorted_chirho = graph_chirho.topo_sort_chirho().unwrap();
        let pos_a_chirho = sorted_chirho.iter().position(|x| x == "A").unwrap();
        let pos_b_chirho = sorted_chirho.iter().position(|x| x == "B").unwrap();
        let pos_c_chirho = sorted_chirho.iter().position(|x| x == "C").unwrap();
        assert!(pos_a_chirho < pos_b_chirho);
        assert!(pos_b_chirho < pos_c_chirho);
    }

    #[test]
    fn cycle_returns_none_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("A", fp_chirho("a"));
        graph_chirho.add_module_chirho("B", fp_chirho("b"));
        graph_chirho.add_dep_chirho("A", "B");
        graph_chirho.add_dep_chirho("B", "A");
        assert!(graph_chirho.topo_sort_chirho().is_none());
    }

    #[test]
    fn invalidated_by_leaf_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("Base", fp_chirho("base"));
        graph_chirho.add_module_chirho("Lib", fp_chirho("lib"));
        graph_chirho.add_module_chirho("App", fp_chirho("app"));
        graph_chirho.add_dep_chirho("Lib", "Base");
        graph_chirho.add_dep_chirho("App", "Lib");

        let inv_chirho = graph_chirho.invalidated_by_chirho("Base");
        assert!(inv_chirho.contains("Base"));
        assert!(inv_chirho.contains("Lib"));
        assert!(inv_chirho.contains("App"));
        assert_eq!(inv_chirho.len(), 3);
    }

    #[test]
    fn invalidated_by_middle_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("Base", fp_chirho("base"));
        graph_chirho.add_module_chirho("Lib", fp_chirho("lib"));
        graph_chirho.add_module_chirho("App", fp_chirho("app"));
        graph_chirho.add_dep_chirho("Lib", "Base");
        graph_chirho.add_dep_chirho("App", "Lib");

        let inv_chirho = graph_chirho.invalidated_by_chirho("Lib");
        assert!(!inv_chirho.contains("Base")); // Base doesn't depend on Lib
        assert!(inv_chirho.contains("Lib"));
        assert!(inv_chirho.contains("App"));
        assert_eq!(inv_chirho.len(), 2);
    }

    #[test]
    fn diamond_deps_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("A", fp_chirho("a"));
        graph_chirho.add_module_chirho("B", fp_chirho("b"));
        graph_chirho.add_module_chirho("C", fp_chirho("c"));
        graph_chirho.add_module_chirho("D", fp_chirho("d"));
        // D imports B and C; B and C both import A
        graph_chirho.add_dep_chirho("D", "B");
        graph_chirho.add_dep_chirho("D", "C");
        graph_chirho.add_dep_chirho("B", "A");
        graph_chirho.add_dep_chirho("C", "A");

        let sorted_chirho = graph_chirho.topo_sort_chirho().unwrap();
        let pos_a_chirho = sorted_chirho.iter().position(|x| x == "A").unwrap();
        let pos_b_chirho = sorted_chirho.iter().position(|x| x == "B").unwrap();
        let pos_c_chirho = sorted_chirho.iter().position(|x| x == "C").unwrap();
        let pos_d_chirho = sorted_chirho.iter().position(|x| x == "D").unwrap();
        assert!(pos_a_chirho < pos_b_chirho);
        assert!(pos_a_chirho < pos_c_chirho);
        assert!(pos_b_chirho < pos_d_chirho);
        assert!(pos_c_chirho < pos_d_chirho);
    }

    #[test]
    fn dependents_of_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("X", fp_chirho("x"));
        graph_chirho.add_module_chirho("Y", fp_chirho("y"));
        graph_chirho.add_module_chirho("Z", fp_chirho("z"));
        graph_chirho.add_dep_chirho("Y", "X");
        graph_chirho.add_dep_chirho("Z", "X");

        let mut dependents_chirho: Vec<&str> = graph_chirho.dependents_of_chirho("X");
        dependents_chirho.sort();
        assert_eq!(dependents_chirho, vec!["Y", "Z"]);
    }

    #[test]
    fn deps_of_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("App", fp_chirho("app"));
        graph_chirho.add_module_chirho("Lib1", fp_chirho("lib1"));
        graph_chirho.add_module_chirho("Lib2", fp_chirho("lib2"));
        graph_chirho.add_dep_chirho("App", "Lib1");
        graph_chirho.add_dep_chirho("App", "Lib2");

        let mut deps_chirho: Vec<&str> = graph_chirho.deps_of_chirho("App");
        deps_chirho.sort();
        assert_eq!(deps_chirho, vec!["Lib1", "Lib2"]);
    }
}
