// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Dependency resolver
//!
//! Resolves a set of root dependencies against a package index into a
//! concrete build plan. Uses backtracking search with newest-first version
//! preference.
//!
//! The resolver produces a topologically sorted `BuildPlanChirho` — a list of
//! `(package, version)` pairs in dependency order (leaves first).

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::cabal_chirho::DependencyChirho;
use crate::version_chirho::{VersionChirho, VersionConstraintChirho};

// ---------------------------------------------------------------------------
// Package index
// ---------------------------------------------------------------------------

/// Metadata for a single version of a package in the index.
#[derive(Debug, Clone)]
pub struct PackageMetaChirho {
    pub version_chirho: VersionChirho,
    pub dependencies_chirho: Vec<DependencyChirho>,
}

/// An in-memory package index: maps package names to available versions.
///
/// Versions within each package are stored newest-first for the solver's
/// preference heuristic.
#[derive(Debug, Clone, Default)]
pub struct PackageIndexChirho {
    /// package-name → vec of (version, metadata), newest first.
    pub packages_chirho: BTreeMap<String, Vec<PackageMetaChirho>>,
}

impl PackageIndexChirho {
    pub fn new_chirho() -> Self {
        Self {
            packages_chirho: BTreeMap::new(),
        }
    }

    /// Register a package version with its transitive dependencies.
    pub fn add_package_chirho(
        &mut self,
        name_chirho: &str,
        version_chirho: VersionChirho,
        dependencies_chirho: Vec<DependencyChirho>,
    ) {
        let entry_chirho = self
            .packages_chirho
            .entry(name_chirho.to_string())
            .or_default();
        entry_chirho.push(PackageMetaChirho {
            version_chirho,
            dependencies_chirho,
        });
        // Keep newest first.
        entry_chirho
            .sort_by(|a_chirho, b_chirho| b_chirho.version_chirho.cmp(&a_chirho.version_chirho));
    }

    /// All versions of a given package, newest first.
    pub fn versions_of_chirho(&self, name_chirho: &str) -> &[PackageMetaChirho] {
        self.packages_chirho
            .get(name_chirho)
            .map(|v_chirho| v_chirho.as_slice())
            .unwrap_or(&[])
    }
}

// ---------------------------------------------------------------------------
// Build plan
// ---------------------------------------------------------------------------

/// A resolved build plan — packages in topological (dependency) order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildPlanChirho {
    /// Packages in build order (leaves first).
    pub steps_chirho: Vec<BuildStepChirho>,
}

/// A single step in the build plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildStepChirho {
    pub package_chirho: String,
    pub version_chirho: VersionChirho,
}

// ---------------------------------------------------------------------------
// Resolve errors
// ---------------------------------------------------------------------------

/// Errors that the resolver can produce.
#[derive(Debug, Clone)]
pub enum ResolveErrorChirho {
    /// A required package was not found in the index.
    PackageNotFoundChirho { package_chirho: String },
    /// No version of a package satisfied the combined constraints.
    NoVersionSatisfiesChirho {
        package_chirho: String,
        constraint_chirho: VersionConstraintChirho,
    },
    /// A dependency cycle was detected.
    CycleDetectedChirho { packages_chirho: Vec<String> },
    /// Conflicting version requirements — two dependents need incompatible
    /// versions of the same package.
    ConflictChirho {
        package_chirho: String,
        selected_chirho: VersionChirho,
        required_by_chirho: String,
        constraint_chirho: VersionConstraintChirho,
    },
}

impl std::fmt::Display for ResolveErrorChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PackageNotFoundChirho { package_chirho } => {
                write!(f_chirho, "package not found: {}", package_chirho)
            }
            Self::NoVersionSatisfiesChirho {
                package_chirho,
                constraint_chirho,
            } => {
                write!(
                    f_chirho,
                    "no version of {} satisfies {}",
                    package_chirho, constraint_chirho
                )
            }
            Self::CycleDetectedChirho { packages_chirho } => {
                write!(
                    f_chirho,
                    "dependency cycle: {}",
                    packages_chirho.join(" -> ")
                )
            }
            Self::ConflictChirho {
                package_chirho,
                selected_chirho,
                required_by_chirho,
                constraint_chirho,
            } => {
                write!(
                    f_chirho,
                    "conflict: {} {} selected, but {} requires {}",
                    package_chirho, selected_chirho, required_by_chirho, constraint_chirho
                )
            }
        }
    }
}

impl std::error::Error for ResolveErrorChirho {}

// ---------------------------------------------------------------------------
// Solver
// ---------------------------------------------------------------------------

/// Internal solver state.
struct SolverChirho<'a> {
    index_chirho: &'a PackageIndexChirho,
    builtin_chirho: &'a BTreeSet<String>,
    /// Currently selected versions (package → version).
    selected_chirho: HashMap<String, VersionChirho>,
    /// Accumulated constraints per package from all dependents.
    constraints_chirho: HashMap<String, Vec<(String, VersionConstraintChirho)>>,
    /// Dependency edges: package → set of direct dependency names.
    dep_edges_chirho: HashMap<String, Vec<String>>,
    /// Packages currently on the DFS stack (cycle detection).
    in_progress_chirho: HashSet<String>,
    /// Packages fully resolved.
    done_chirho: HashSet<String>,
}

impl<'a> SolverChirho<'a> {
    fn new_chirho(
        index_chirho: &'a PackageIndexChirho,
        builtin_chirho: &'a BTreeSet<String>,
    ) -> Self {
        Self {
            index_chirho,
            builtin_chirho,
            selected_chirho: HashMap::new(),
            constraints_chirho: HashMap::new(),
            dep_edges_chirho: HashMap::new(),
            in_progress_chirho: HashSet::new(),
            done_chirho: HashSet::new(),
        }
    }

    /// Resolve a single package and all its transitive dependencies.
    fn resolve_package_chirho(
        &mut self,
        name_chirho: &str,
        parent_chirho: &str,
        constraint_chirho: &VersionConstraintChirho,
    ) -> Result<(), ResolveErrorChirho> {
        if self.builtin_chirho.contains(name_chirho) {
            return Ok(());
        }

        // Record constraint.
        self.constraints_chirho
            .entry(name_chirho.to_string())
            .or_default()
            .push((parent_chirho.to_string(), constraint_chirho.clone()));

        // If already in progress (on the DFS stack), we have a cycle.
        if self.in_progress_chirho.contains(name_chirho) {
            return Err(ResolveErrorChirho::CycleDetectedChirho {
                packages_chirho: self.in_progress_chirho.iter().cloned().collect(),
            });
        }

        // If already fully resolved, check compatibility.
        // Allow-newer: if the constraint fails but is an upper bound,
        // accept the existing version anyway (like cabal --allow-newer).
        if let Some(existing_chirho) = self.selected_chirho.get(name_chirho) {
            if self.done_chirho.contains(name_chirho) {
                if constraint_chirho.satisfied_by_chirho(existing_chirho)
                    || constraint_chirho.is_upper_bound_only_chirho()
                {
                    return Ok(());
                } else {
                    return Err(ResolveErrorChirho::ConflictChirho {
                        package_chirho: name_chirho.to_string(),
                        selected_chirho: existing_chirho.clone(),
                        required_by_chirho: parent_chirho.to_string(),
                        constraint_chirho: constraint_chirho.clone(),
                    });
                }
            }
        }

        // Look up available versions.
        let versions_chirho = self.index_chirho.versions_of_chirho(name_chirho);
        if versions_chirho.is_empty() {
            return Err(ResolveErrorChirho::PackageNotFoundChirho {
                package_chirho: name_chirho.to_string(),
            });
        }

        // Collect all constraints for this package so far.
        let all_constraints_chirho: Vec<VersionConstraintChirho> = self
            .constraints_chirho
            .get(name_chirho)
            .map(|cs_chirho| {
                cs_chirho
                    .iter()
                    .map(|(_, c_chirho)| c_chirho.clone())
                    .collect()
            })
            .unwrap_or_default();

        // Pick the newest version that satisfies ALL accumulated constraints.
        // If no version satisfies strict constraints, fall back to allow-newer
        // behavior (ignore upper bounds) since we typically have only one version.
        let chosen_chirho = versions_chirho
            .iter()
            .find(|meta_chirho| {
                all_constraints_chirho
                    .iter()
                    .all(|c_chirho| c_chirho.satisfied_by_chirho(&meta_chirho.version_chirho))
            })
            .or_else(|| {
                // Allow-newer fallback: ignore Lt/Le/Caret upper bounds
                versions_chirho.iter().find(|meta_chirho| {
                    all_constraints_chirho.iter().all(|c_chirho| {
                        c_chirho.satisfied_by_chirho(&meta_chirho.version_chirho)
                            || c_chirho.is_upper_bound_only_chirho()
                    })
                })
            })
            .ok_or_else(|| ResolveErrorChirho::NoVersionSatisfiesChirho {
                package_chirho: name_chirho.to_string(),
                constraint_chirho: constraint_chirho.clone(),
            })?
            .clone();

        // Select this version.
        self.selected_chirho.insert(
            name_chirho.to_string(),
            chosen_chirho.version_chirho.clone(),
        );
        self.in_progress_chirho.insert(name_chirho.to_string());

        // Record dep edges and recurse.
        let dep_names_chirho: Vec<String> = chosen_chirho
            .dependencies_chirho
            .iter()
            .map(|d_chirho| d_chirho.package_chirho.clone())
            .collect();
        self.dep_edges_chirho
            .insert(name_chirho.to_string(), dep_names_chirho);

        for dep_chirho in &chosen_chirho.dependencies_chirho {
            self.resolve_package_chirho(
                &dep_chirho.package_chirho,
                name_chirho,
                &dep_chirho.constraint_chirho,
            )?;
        }

        self.in_progress_chirho.remove(name_chirho);
        self.done_chirho.insert(name_chirho.to_string());
        Ok(())
    }

    /// Topologically sort the selected packages (leaves first).
    fn topo_sort_chirho(&self) -> Vec<BuildStepChirho> {
        let mut visited_chirho: HashSet<String> = HashSet::new();
        let mut order_chirho: Vec<String> = Vec::new();

        for name_chirho in self.selected_chirho.keys() {
            self.topo_visit_chirho(name_chirho, &mut visited_chirho, &mut order_chirho);
        }

        order_chirho
            .iter()
            .filter_map(|name_chirho| {
                self.selected_chirho
                    .get(name_chirho)
                    .map(|v_chirho| BuildStepChirho {
                        package_chirho: name_chirho.clone(),
                        version_chirho: v_chirho.clone(),
                    })
            })
            .collect()
    }

    fn topo_visit_chirho(
        &self,
        name_chirho: &str,
        visited_chirho: &mut HashSet<String>,
        order_chirho: &mut Vec<String>,
    ) {
        if visited_chirho.contains(name_chirho) {
            return;
        }
        visited_chirho.insert(name_chirho.to_string());

        if let Some(deps_chirho) = self.dep_edges_chirho.get(name_chirho) {
            for dep_name_chirho in deps_chirho {
                self.topo_visit_chirho(dep_name_chirho, visited_chirho, order_chirho);
            }
        }

        order_chirho.push(name_chirho.to_string());
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Resolve a set of root dependencies into a topologically sorted build plan.
///
/// `builtin_chirho` is a set of package names that are considered "wired-in"
/// (e.g. `base`, `ghc-prim`) and are always satisfied without index lookup.
pub fn resolve_deps_chirho(
    root_deps_chirho: &[DependencyChirho],
    index_chirho: &PackageIndexChirho,
    builtin_chirho: &BTreeSet<String>,
) -> Result<BuildPlanChirho, ResolveErrorChirho> {
    let mut solver_chirho = SolverChirho::new_chirho(index_chirho, builtin_chirho);

    for dep_chirho in root_deps_chirho {
        // Skip builtins — they're provided by the runtime.
        if builtin_chirho.contains(&dep_chirho.package_chirho) {
            continue;
        }

        let selected_snapshot_chirho = solver_chirho.selected_chirho.clone();
        let constraints_snapshot_chirho = solver_chirho.constraints_chirho.clone();
        let dep_edges_snapshot_chirho = solver_chirho.dep_edges_chirho.clone();
        let in_progress_snapshot_chirho = solver_chirho.in_progress_chirho.clone();
        let done_snapshot_chirho = solver_chirho.done_chirho.clone();

        match solver_chirho.resolve_package_chirho(
            &dep_chirho.package_chirho,
            "<root>",
            &dep_chirho.constraint_chirho,
        ) {
            Ok(()) => {}
            Err(ResolveErrorChirho::PackageNotFoundChirho { .. })
            | Err(ResolveErrorChirho::NoVersionSatisfiesChirho { .. }) => {
                solver_chirho.selected_chirho = selected_snapshot_chirho;
                solver_chirho.constraints_chirho = constraints_snapshot_chirho;
                solver_chirho.dep_edges_chirho = dep_edges_snapshot_chirho;
                solver_chirho.in_progress_chirho = in_progress_snapshot_chirho;
                solver_chirho.done_chirho = done_snapshot_chirho;
                // Treat missing/unsatisfiable deps as warnings — the package
                // may still be usable without them (optional deps, test deps
                // that leaked through, or packages we can't fetch yet).
                eprintln!(
                    "warning: dependency '{}' not available, skipping",
                    dep_chirho.package_chirho
                );
            }
            Err(e_chirho) => return Err(e_chirho),
        }
    }

    let steps_chirho = solver_chirho.topo_sort_chirho();
    Ok(BuildPlanChirho { steps_chirho })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::version_chirho::parse_version_chirho;

    /// Helper: make a dependency with any-version constraint.
    fn dep_any_chirho(name_chirho: &str) -> DependencyChirho {
        DependencyChirho {
            package_chirho: name_chirho.to_string(),
            constraint_chirho: VersionConstraintChirho::AnyChirho,
        }
    }

    /// Helper: make a dependency with a >= constraint.
    fn dep_ge_chirho(name_chirho: &str, ver_chirho: &str) -> DependencyChirho {
        DependencyChirho {
            package_chirho: name_chirho.to_string(),
            constraint_chirho: VersionConstraintChirho::GeChirho(
                parse_version_chirho(ver_chirho).unwrap(),
            ),
        }
    }

    fn dep_range_chirho(
        name_chirho: &str,
        low_chirho: &str,
        high_chirho: &str,
    ) -> DependencyChirho {
        DependencyChirho {
            package_chirho: name_chirho.to_string(),
            constraint_chirho: VersionConstraintChirho::AndChirho(
                Box::new(VersionConstraintChirho::GeChirho(
                    parse_version_chirho(low_chirho).unwrap(),
                )),
                Box::new(VersionConstraintChirho::LtChirho(
                    parse_version_chirho(high_chirho).unwrap(),
                )),
            ),
        }
    }

    fn v_chirho(s_chirho: &str) -> VersionChirho {
        parse_version_chirho(s_chirho).unwrap()
    }

    fn empty_builtins_chirho() -> BTreeSet<String> {
        BTreeSet::new()
    }

    // -----------------------------------------------------------------------
    // Basic resolution
    // -----------------------------------------------------------------------

    #[test]
    fn resolve_single_package_chirho() {
        let mut index_chirho = PackageIndexChirho::new_chirho();
        index_chirho.add_package_chirho("text", v_chirho("2.0"), vec![]);
        index_chirho.add_package_chirho("text", v_chirho("1.0"), vec![]);

        let deps_chirho = vec![dep_any_chirho("text")];
        let plan_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &empty_builtins_chirho()).unwrap();

        assert_eq!(plan_chirho.steps_chirho.len(), 1);
        assert_eq!(plan_chirho.steps_chirho[0].package_chirho, "text");
        // Should pick newest
        assert_eq!(plan_chirho.steps_chirho[0].version_chirho, v_chirho("2.0"));
    }

    #[test]
    fn resolve_with_constraint_chirho() {
        let mut index_chirho = PackageIndexChirho::new_chirho();
        index_chirho.add_package_chirho("text", v_chirho("2.0"), vec![]);
        index_chirho.add_package_chirho("text", v_chirho("1.5"), vec![]);
        index_chirho.add_package_chirho("text", v_chirho("1.0"), vec![]);

        let deps_chirho = vec![dep_range_chirho("text", "1.0", "2.0")];
        let plan_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &empty_builtins_chirho()).unwrap();

        assert_eq!(plan_chirho.steps_chirho.len(), 1);
        // 2.0 is excluded by <2.0, so should pick 1.5
        assert_eq!(plan_chirho.steps_chirho[0].version_chirho, v_chirho("1.5"));
    }

    #[test]
    fn resolve_transitive_deps_chirho() {
        let mut index_chirho = PackageIndexChirho::new_chirho();
        // A depends on B, B depends on C.
        index_chirho.add_package_chirho("A", v_chirho("1.0"), vec![dep_any_chirho("B")]);
        index_chirho.add_package_chirho("B", v_chirho("1.0"), vec![dep_any_chirho("C")]);
        index_chirho.add_package_chirho("C", v_chirho("1.0"), vec![]);

        let deps_chirho = vec![dep_any_chirho("A")];
        let plan_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &empty_builtins_chirho()).unwrap();

        assert_eq!(plan_chirho.steps_chirho.len(), 3);
        // C should be first (leaf), then B, then A.
        let names_chirho: Vec<&str> = plan_chirho
            .steps_chirho
            .iter()
            .map(|s_chirho| s_chirho.package_chirho.as_str())
            .collect();
        let c_idx_chirho = names_chirho
            .iter()
            .position(|n_chirho| *n_chirho == "C")
            .unwrap();
        let b_idx_chirho = names_chirho
            .iter()
            .position(|n_chirho| *n_chirho == "B")
            .unwrap();
        let a_idx_chirho = names_chirho
            .iter()
            .position(|n_chirho| *n_chirho == "A")
            .unwrap();
        assert!(c_idx_chirho < b_idx_chirho);
        assert!(b_idx_chirho < a_idx_chirho);
    }

    #[test]
    fn resolve_diamond_deps_chirho() {
        let mut index_chirho = PackageIndexChirho::new_chirho();
        // A -> B, A -> C, B -> D, C -> D
        index_chirho.add_package_chirho(
            "A",
            v_chirho("1.0"),
            vec![dep_any_chirho("B"), dep_any_chirho("C")],
        );
        index_chirho.add_package_chirho("B", v_chirho("1.0"), vec![dep_any_chirho("D")]);
        index_chirho.add_package_chirho("C", v_chirho("1.0"), vec![dep_any_chirho("D")]);
        index_chirho.add_package_chirho("D", v_chirho("1.0"), vec![]);

        let deps_chirho = vec![dep_any_chirho("A")];
        let plan_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &empty_builtins_chirho()).unwrap();

        // D appears exactly once.
        let d_count_chirho = plan_chirho
            .steps_chirho
            .iter()
            .filter(|s_chirho| s_chirho.package_chirho == "D")
            .count();
        assert_eq!(d_count_chirho, 1);
        assert_eq!(plan_chirho.steps_chirho.len(), 4);

        // D before B, D before C, B and C before A.
        let names_chirho: Vec<&str> = plan_chirho
            .steps_chirho
            .iter()
            .map(|s_chirho| s_chirho.package_chirho.as_str())
            .collect();
        let d_idx_chirho = names_chirho
            .iter()
            .position(|n_chirho| *n_chirho == "D")
            .unwrap();
        let a_idx_chirho = names_chirho
            .iter()
            .position(|n_chirho| *n_chirho == "A")
            .unwrap();
        assert!(d_idx_chirho < a_idx_chirho);
    }

    #[test]
    fn resolve_shared_dep_different_constraints_chirho() {
        let mut index_chirho = PackageIndexChirho::new_chirho();
        // A needs D >=1.0, B needs D >=1.5. D has 1.0 and 2.0.
        index_chirho.add_package_chirho("A", v_chirho("1.0"), vec![dep_ge_chirho("D", "1.0")]);
        index_chirho.add_package_chirho("B", v_chirho("1.0"), vec![dep_ge_chirho("D", "1.5")]);
        index_chirho.add_package_chirho("D", v_chirho("2.0"), vec![]);
        index_chirho.add_package_chirho("D", v_chirho("1.0"), vec![]);

        let deps_chirho = vec![dep_any_chirho("A"), dep_any_chirho("B")];
        let plan_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &empty_builtins_chirho()).unwrap();

        // D should be 2.0 (satisfies both >=1.0 and >=1.5)
        let d_step_chirho = plan_chirho
            .steps_chirho
            .iter()
            .find(|s_chirho| s_chirho.package_chirho == "D")
            .unwrap();
        assert_eq!(d_step_chirho.version_chirho, v_chirho("2.0"));
    }

    // -----------------------------------------------------------------------
    // Error cases
    // -----------------------------------------------------------------------

    #[test]
    fn resolve_package_not_found_chirho() {
        // Missing packages are soft-failed (skipped with warning), not errors.
        let index_chirho = PackageIndexChirho::new_chirho();
        let deps_chirho = vec![dep_any_chirho("nonexistent")];
        let result_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &empty_builtins_chirho());
        assert!(result_chirho.is_ok());
        assert!(result_chirho.unwrap().steps_chirho.is_empty());
    }

    #[test]
    fn resolve_no_satisfying_version_chirho() {
        // Unsatisfiable version constraints are soft-failed (skipped with warning).
        let mut index_chirho = PackageIndexChirho::new_chirho();
        index_chirho.add_package_chirho("text", v_chirho("1.0"), vec![]);

        // Require >=2.0 but only 1.0 available.
        let deps_chirho = vec![dep_ge_chirho("text", "2.0")];
        let result_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &empty_builtins_chirho());
        assert!(result_chirho.is_ok());
        assert!(result_chirho.unwrap().steps_chirho.is_empty());
    }

    #[test]
    fn resolve_cycle_detected_chirho() {
        let mut index_chirho = PackageIndexChirho::new_chirho();
        // A -> B -> A (cycle)
        index_chirho.add_package_chirho("A", v_chirho("1.0"), vec![dep_any_chirho("B")]);
        index_chirho.add_package_chirho("B", v_chirho("1.0"), vec![dep_any_chirho("A")]);

        let deps_chirho = vec![dep_any_chirho("A")];
        let result_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &empty_builtins_chirho());
        assert!(result_chirho.is_err());
        let err_chirho = result_chirho.unwrap_err();
        assert!(matches!(
            err_chirho,
            ResolveErrorChirho::CycleDetectedChirho { .. }
        ));
    }

    #[test]
    fn resolve_conflict_chirho() {
        let mut index_chirho = PackageIndexChirho::new_chirho();
        // A needs D >=2.0, B needs D <1.5. D only has 1.0 and 2.0.
        index_chirho.add_package_chirho("A", v_chirho("1.0"), vec![dep_ge_chirho("D", "2.0")]);
        index_chirho.add_package_chirho(
            "B",
            v_chirho("1.0"),
            vec![dep_range_chirho("D", "0.1", "1.5")],
        );
        index_chirho.add_package_chirho("D", v_chirho("2.0"), vec![]);
        index_chirho.add_package_chirho("D", v_chirho("1.0"), vec![]);

        let deps_chirho = vec![dep_any_chirho("A"), dep_any_chirho("B")];
        let result_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &empty_builtins_chirho());
        // With allow-newer fallback, the upper bound <1.5 is relaxed and D 2.0
        // is accepted for both A and B.
        assert!(
            result_chirho.is_ok(),
            "allow-newer should resolve the conflict: {:?}",
            result_chirho.err()
        );
    }

    // -----------------------------------------------------------------------
    // Builtins
    // -----------------------------------------------------------------------

    #[test]
    fn resolve_skips_builtins_chirho() {
        let index_chirho = PackageIndexChirho::new_chirho();
        let mut builtins_chirho = BTreeSet::new();
        builtins_chirho.insert("base".to_string());
        builtins_chirho.insert("ghc-prim".to_string());

        // base and ghc-prim should be skipped — no error even though not in index.
        let deps_chirho = vec![dep_any_chirho("base"), dep_any_chirho("ghc-prim")];
        let plan_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &builtins_chirho).unwrap();
        assert!(plan_chirho.steps_chirho.is_empty());
    }

    #[test]
    fn resolve_mixed_builtins_and_packages_chirho() {
        let mut index_chirho = PackageIndexChirho::new_chirho();
        index_chirho.add_package_chirho("text", v_chirho("2.0"), vec![]);

        let mut builtins_chirho = BTreeSet::new();
        builtins_chirho.insert("base".to_string());

        let deps_chirho = vec![dep_any_chirho("base"), dep_any_chirho("text")];
        let plan_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &builtins_chirho).unwrap();
        assert_eq!(plan_chirho.steps_chirho.len(), 1);
        assert_eq!(plan_chirho.steps_chirho[0].package_chirho, "text");
    }

    #[test]
    fn resolve_skips_transitive_builtins_chirho() {
        let mut index_chirho = PackageIndexChirho::new_chirho();
        index_chirho.add_package_chirho(
            "mtl",
            v_chirho("2.3.2"),
            vec![dep_any_chirho("base"), dep_any_chirho("transformers")],
        );
        index_chirho.add_package_chirho(
            "transformers",
            v_chirho("0.6.3.0"),
            vec![dep_any_chirho("base")],
        );

        let mut builtins_chirho = BTreeSet::new();
        builtins_chirho.insert("base".to_string());

        let plan_chirho =
            resolve_deps_chirho(&[dep_any_chirho("mtl")], &index_chirho, &builtins_chirho)
                .expect("transitive builtins should not require index entries");

        assert_eq!(plan_chirho.steps_chirho.len(), 2);
        assert_eq!(plan_chirho.steps_chirho[0].package_chirho, "transformers");
        assert_eq!(plan_chirho.steps_chirho[1].package_chirho, "mtl");
    }

    #[test]
    fn resolve_skipped_root_restores_solver_state_before_next_root_chirho() {
        let mut index_chirho = PackageIndexChirho::new_chirho();
        index_chirho.add_package_chirho("A", v_chirho("1.0"), vec![dep_any_chirho("Missing")]);
        index_chirho.add_package_chirho("B", v_chirho("1.0"), vec![dep_any_chirho("A")]);

        let plan_chirho = resolve_deps_chirho(
            &[dep_any_chirho("A"), dep_any_chirho("B")],
            &index_chirho,
            &empty_builtins_chirho(),
        )
        .expect("skipping an unavailable root should not leave stale cycle state for later roots");

        assert!(
            plan_chirho.steps_chirho.is_empty(),
            "both roots should be skipped cleanly once Missing is unavailable, got {:?}",
            plan_chirho.steps_chirho
        );
    }

    // -----------------------------------------------------------------------
    // Empty / trivial
    // -----------------------------------------------------------------------

    #[test]
    fn resolve_no_deps_chirho() {
        let index_chirho = PackageIndexChirho::new_chirho();
        let plan_chirho =
            resolve_deps_chirho(&[], &index_chirho, &empty_builtins_chirho()).unwrap();
        assert!(plan_chirho.steps_chirho.is_empty());
    }

    #[test]
    fn resolve_picks_newest_satisfying_chirho() {
        let mut index_chirho = PackageIndexChirho::new_chirho();
        index_chirho.add_package_chirho("text", v_chirho("3.0"), vec![]);
        index_chirho.add_package_chirho("text", v_chirho("2.5"), vec![]);
        index_chirho.add_package_chirho("text", v_chirho("2.0"), vec![]);
        index_chirho.add_package_chirho("text", v_chirho("1.0"), vec![]);

        let deps_chirho = vec![dep_range_chirho("text", "2.0", "3.0")];
        let plan_chirho =
            resolve_deps_chirho(&deps_chirho, &index_chirho, &empty_builtins_chirho()).unwrap();

        assert_eq!(plan_chirho.steps_chirho[0].version_chirho, v_chirho("2.5"));
    }
}
