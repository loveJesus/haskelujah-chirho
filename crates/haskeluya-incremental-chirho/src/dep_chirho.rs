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

    /// Topological sort that groups mutually-dependent modules into
    /// strongly-connected components (SCCs). Each SCC with more than one
    /// member represents a circular import group that requires `.hs-boot`
    /// files to break the cycle.
    ///
    /// Returns SCCs in dependency order (leaves first).
    pub fn topo_sort_sccs_chirho(&self) -> Vec<Vec<String>> {
        // Tarjan's SCC algorithm
        let names_chirho: Vec<&str> = self.nodes_chirho.keys().map(|s_chirho| s_chirho.as_str()).collect();
        let mut index_map_chirho: HashMap<&str, usize> = HashMap::new();
        let mut lowlink_chirho: HashMap<&str, usize> = HashMap::new();
        let mut on_stack_chirho: HashSet<&str> = HashSet::new();
        let mut stack_chirho: Vec<&str> = Vec::new();
        let mut index_counter_chirho: usize = 0;
        let mut sccs_chirho: Vec<Vec<String>> = Vec::new();

        fn strongconnect_chirho<'a>(
            v_chirho: &'a str,
            edges_chirho: &HashMap<String, HashSet<String>>,
            index_map_chirho: &mut HashMap<&'a str, usize>,
            lowlink_chirho: &mut HashMap<&'a str, usize>,
            on_stack_chirho: &mut HashSet<&'a str>,
            stack_chirho: &mut Vec<&'a str>,
            index_counter_chirho: &mut usize,
            sccs_chirho: &mut Vec<Vec<String>>,
            names_chirho: &[&'a str],
        ) {
            index_map_chirho.insert(v_chirho, *index_counter_chirho);
            lowlink_chirho.insert(v_chirho, *index_counter_chirho);
            *index_counter_chirho += 1;
            stack_chirho.push(v_chirho);
            on_stack_chirho.insert(v_chirho);

            if let Some(deps_chirho) = edges_chirho.get(v_chirho) {
                for w_name_chirho in deps_chirho {
                    // Find the &'a str reference from names_chirho
                    if let Some(w_chirho) = names_chirho.iter().find(|n_chirho| **n_chirho == w_name_chirho.as_str()) {
                        if !index_map_chirho.contains_key(*w_chirho) {
                            strongconnect_chirho(
                                w_chirho,
                                edges_chirho,
                                index_map_chirho,
                                lowlink_chirho,
                                on_stack_chirho,
                                stack_chirho,
                                index_counter_chirho,
                                sccs_chirho,
                                names_chirho,
                            );
                            let w_low_chirho = lowlink_chirho[*w_chirho];
                            let v_low_chirho = lowlink_chirho[v_chirho];
                            if w_low_chirho < v_low_chirho {
                                lowlink_chirho.insert(v_chirho, w_low_chirho);
                            }
                        } else if on_stack_chirho.contains(*w_chirho) {
                            let w_idx_chirho = index_map_chirho[*w_chirho];
                            let v_low_chirho = lowlink_chirho[v_chirho];
                            if w_idx_chirho < v_low_chirho {
                                lowlink_chirho.insert(v_chirho, w_idx_chirho);
                            }
                        }
                    }
                }
            }

            if lowlink_chirho[v_chirho] == index_map_chirho[v_chirho] {
                let mut scc_chirho = Vec::new();
                loop {
                    let w_chirho = stack_chirho.pop().unwrap();
                    on_stack_chirho.remove(w_chirho);
                    scc_chirho.push(w_chirho.to_string());
                    if w_chirho == v_chirho {
                        break;
                    }
                }
                scc_chirho.reverse();
                sccs_chirho.push(scc_chirho);
            }
        }

        for name_chirho in &names_chirho {
            if !index_map_chirho.contains_key(*name_chirho) {
                strongconnect_chirho(
                    name_chirho,
                    &self.edges_chirho,
                    &mut index_map_chirho,
                    &mut lowlink_chirho,
                    &mut on_stack_chirho,
                    &mut stack_chirho,
                    &mut index_counter_chirho,
                    &mut sccs_chirho,
                    &names_chirho,
                );
            }
        }

        sccs_chirho
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

    #[test]
    fn scc_no_cycle_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("A", fp_chirho("a"));
        graph_chirho.add_module_chirho("B", fp_chirho("b"));
        graph_chirho.add_dep_chirho("B", "A");
        let sccs_chirho = graph_chirho.topo_sort_sccs_chirho();
        // No cycles: each SCC has exactly one module
        assert_eq!(sccs_chirho.len(), 2);
        assert_eq!(sccs_chirho[0].len(), 1);
        assert_eq!(sccs_chirho[1].len(), 1);
        // A before B (dependency order)
        assert_eq!(sccs_chirho[0][0], "A");
        assert_eq!(sccs_chirho[1][0], "B");
    }

    #[test]
    fn scc_simple_cycle_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("A", fp_chirho("a"));
        graph_chirho.add_module_chirho("B", fp_chirho("b"));
        graph_chirho.add_dep_chirho("A", "B");
        graph_chirho.add_dep_chirho("B", "A");
        let sccs_chirho = graph_chirho.topo_sort_sccs_chirho();
        // One SCC containing both modules
        assert_eq!(sccs_chirho.len(), 1);
        assert_eq!(sccs_chirho[0].len(), 2);
        let mut members_chirho = sccs_chirho[0].clone();
        members_chirho.sort();
        assert_eq!(members_chirho, vec!["A".to_string(), "B".to_string()]);
    }

    #[test]
    fn scc_cycle_with_leaf_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("A", fp_chirho("a"));
        graph_chirho.add_module_chirho("B", fp_chirho("b"));
        graph_chirho.add_module_chirho("C", fp_chirho("c"));
        // A↔B cycle, C imports A
        graph_chirho.add_dep_chirho("A", "B");
        graph_chirho.add_dep_chirho("B", "A");
        graph_chirho.add_dep_chirho("C", "A");
        let sccs_chirho = graph_chirho.topo_sort_sccs_chirho();
        // Two SCCs: {A, B} then {C}
        assert_eq!(sccs_chirho.len(), 2);
        // First SCC: the cycle
        let mut first_chirho = sccs_chirho[0].clone();
        first_chirho.sort();
        assert_eq!(first_chirho, vec!["A".to_string(), "B".to_string()]);
        // Second SCC: C (depends on the cycle)
        assert_eq!(sccs_chirho[1], vec!["C".to_string()]);
    }

    #[test]
    fn scc_three_node_cycle_chirho() {
        let mut graph_chirho = DepGraphChirho::new_chirho();
        graph_chirho.add_module_chirho("A", fp_chirho("a"));
        graph_chirho.add_module_chirho("B", fp_chirho("b"));
        graph_chirho.add_module_chirho("C", fp_chirho("c"));
        // A→B→C→A
        graph_chirho.add_dep_chirho("A", "B");
        graph_chirho.add_dep_chirho("B", "C");
        graph_chirho.add_dep_chirho("C", "A");
        let sccs_chirho = graph_chirho.topo_sort_sccs_chirho();
        assert_eq!(sccs_chirho.len(), 1);
        assert_eq!(sccs_chirho[0].len(), 3);
    }
}
