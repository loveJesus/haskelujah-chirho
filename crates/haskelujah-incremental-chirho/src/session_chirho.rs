// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Incremental compilation session
//!
//! Tracks per-module compilation state and answers "should I recompile?"
//! queries. The session keeps a [`ModuleRecordChirho`] for each module
//! it has seen, storing the source fingerprint, dependency fingerprints,
//! and per-phase artifact fingerprints.
//!
//! The basic recompilation check is:
//! 1. Has the source fingerprint changed?
//! 2. Has any dependency's interface fingerprint changed?
//!
//! If neither changed, the cached artifacts for that module are still valid.

use std::collections::HashMap;

use crate::artifact_chirho::ArtifactStoreChirho;
use crate::dep_chirho::DepGraphChirho;
use crate::fingerprint_chirho::FingerprintChirho;

/// Tags for the different compilation phases whose outputs can be cached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhaseTagChirho {
    /// Parsed CST / AST.
    ParseChirho,
    /// Name resolution result.
    NameResolveChirho,
    /// Kind inference result.
    KindInferChirho,
    /// Type inference result.
    TypeInferChirho,
    /// Exhaustiveness check result.
    ExhaustChirho,
    /// Desugared Core IR.
    CoreChirho,
    /// Dictionary-passing transform.
    DictPassChirho,
    /// Simplified Core IR.
    SimplifiedCoreChirho,
    /// LLVM IR text.
    LlvmIrChirho,
    /// WebAssembly binary.
    WasmChirho,
    /// Module interface (for downstream imports).
    IfaceChirho,
}

impl PhaseTagChirho {
    /// Short string label used in diagnostic messages.
    pub fn label_chirho(&self) -> &'static str {
        match self {
            Self::ParseChirho => "parse",
            Self::NameResolveChirho => "name-resolve",
            Self::KindInferChirho => "kind-infer",
            Self::TypeInferChirho => "type-infer",
            Self::ExhaustChirho => "exhaust",
            Self::CoreChirho => "core",
            Self::DictPassChirho => "dict-pass",
            Self::SimplifiedCoreChirho => "simplified-core",
            Self::LlvmIrChirho => "llvm-ir",
            Self::WasmChirho => "wasm",
            Self::IfaceChirho => "iface",
        }
    }
}

/// Why a module needs recompilation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RebuildReasonChirho {
    /// No previous record exists.
    NoPriorRecordChirho,
    /// Source file changed.
    SourceChangedChirho,
    /// A dependency's interface changed.
    DepChangedChirho { dep_name_chirho: String },
    /// A specific phase artifact is missing from the cache.
    MissingArtifactChirho { phase_chirho: PhaseTagChirho },
    /// Forced rebuild (e.g. `--force` flag).
    ForcedChirho,
}

impl std::fmt::Display for RebuildReasonChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoPriorRecordChirho => write!(f_chirho, "no prior record"),
            Self::SourceChangedChirho => write!(f_chirho, "source changed"),
            Self::DepChangedChirho { dep_name_chirho } => {
                write!(f_chirho, "dependency {} changed", dep_name_chirho)
            }
            Self::MissingArtifactChirho { phase_chirho } => {
                write!(f_chirho, "missing {} artifact", phase_chirho.label_chirho())
            }
            Self::ForcedChirho => write!(f_chirho, "forced rebuild"),
        }
    }
}

/// Cached metadata for a single module.
#[derive(Debug, Clone)]
pub struct ModuleRecordChirho {
    /// Module name.
    pub module_name_chirho: String,
    /// Fingerprint of the source text.
    pub source_fp_chirho: FingerprintChirho,
    /// Fingerprints of per-phase artifacts.
    pub phase_fps_chirho: HashMap<PhaseTagChirho, FingerprintChirho>,
    /// Combined fingerprint of all dependency interfaces at the time
    /// this module was last compiled.
    pub deps_fp_chirho: FingerprintChirho,
}

impl ModuleRecordChirho {
    /// Create a fresh record with no cached phases.
    pub fn new_chirho(
        module_name_chirho: &str,
        source_fp_chirho: FingerprintChirho,
        deps_fp_chirho: FingerprintChirho,
    ) -> Self {
        Self {
            module_name_chirho: module_name_chirho.to_string(),
            source_fp_chirho,
            phase_fps_chirho: HashMap::new(),
            deps_fp_chirho,
        }
    }

    /// Record a phase artifact fingerprint.
    pub fn set_phase_chirho(&mut self, phase_chirho: PhaseTagChirho, fp_chirho: FingerprintChirho) {
        self.phase_fps_chirho.insert(phase_chirho, fp_chirho);
    }

    /// Get a phase artifact fingerprint.
    pub fn get_phase_chirho(&self, phase_chirho: PhaseTagChirho) -> Option<FingerprintChirho> {
        self.phase_fps_chirho.get(&phase_chirho).copied()
    }
}

/// The incremental compilation session.
///
/// Holds the dependency graph, per-module records, and artifact store.
/// The driver uses this to decide which modules need recompilation and
/// to store/retrieve cached artifacts.
#[derive(Debug)]
pub struct IncrementalSessionChirho {
    /// Dependency graph.
    pub dep_graph_chirho: DepGraphChirho,
    /// Per-module compilation records from the prior session.
    records_chirho: HashMap<String, ModuleRecordChirho>,
    /// Artifact store for cached blobs.
    pub store_chirho: ArtifactStoreChirho,
}

impl IncrementalSessionChirho {
    /// Create a new session with an in-memory artifact store.
    pub fn new_chirho() -> Self {
        Self {
            dep_graph_chirho: DepGraphChirho::new_chirho(),
            records_chirho: HashMap::new(),
            store_chirho: ArtifactStoreChirho::in_memory_chirho(),
        }
    }

    /// Create a session backed by an on-disk cache directory.
    pub fn with_cache_dir_chirho(
        cache_dir_chirho: impl AsRef<std::path::Path>,
    ) -> Result<Self, crate::artifact_chirho::ArtifactErrorChirho> {
        Ok(Self {
            dep_graph_chirho: DepGraphChirho::new_chirho(),
            records_chirho: HashMap::new(),
            store_chirho: ArtifactStoreChirho::open_chirho(cache_dir_chirho)?,
        })
    }

    /// Register a module's source and its imports.
    pub fn register_module_chirho(
        &mut self,
        module_name_chirho: &str,
        source_chirho: &str,
        imports_chirho: &[&str],
    ) {
        let source_fp_chirho = FingerprintChirho::from_str_chirho(source_chirho);
        self.dep_graph_chirho
            .add_module_chirho(module_name_chirho, source_fp_chirho);
        for imp_chirho in imports_chirho {
            self.dep_graph_chirho
                .add_dep_chirho(module_name_chirho, imp_chirho);
        }
    }

    /// Compute the combined dependency fingerprint for a module.
    /// This is the combination of all dependency interface fingerprints.
    pub fn compute_deps_fp_chirho(&self, module_name_chirho: &str) -> FingerprintChirho {
        let deps_chirho = self.dep_graph_chirho.deps_of_chirho(module_name_chirho);
        let mut fps_chirho: Vec<FingerprintChirho> = deps_chirho
            .iter()
            .filter_map(|dep_chirho| {
                self.records_chirho
                    .get(*dep_chirho)
                    .and_then(|r_chirho| r_chirho.get_phase_chirho(PhaseTagChirho::IfaceChirho))
            })
            .collect();
        fps_chirho.sort(); // deterministic order
        FingerprintChirho::combine_many_chirho(&fps_chirho)
    }

    /// Check whether a module needs recompilation.
    ///
    /// Returns `None` if the module is up-to-date, or `Some(reason)` if
    /// it needs to be rebuilt.
    pub fn needs_rebuild_chirho(
        &self,
        module_name_chirho: &str,
        current_source_fp_chirho: FingerprintChirho,
    ) -> Option<RebuildReasonChirho> {
        let record_chirho = match self.records_chirho.get(module_name_chirho) {
            Some(r_chirho) => r_chirho,
            None => return Some(RebuildReasonChirho::NoPriorRecordChirho),
        };

        // 1. Source changed?
        if record_chirho.source_fp_chirho != current_source_fp_chirho {
            return Some(RebuildReasonChirho::SourceChangedChirho);
        }

        // 2. Dependencies changed?
        let current_deps_fp_chirho = self.compute_deps_fp_chirho(module_name_chirho);
        if record_chirho.deps_fp_chirho != current_deps_fp_chirho {
            // Find which dep changed for diagnostic purposes.
            let deps_chirho = self.dep_graph_chirho.deps_of_chirho(module_name_chirho);
            for dep_chirho in deps_chirho {
                // Compare dep's current iface fp with what was recorded.
                let current_dep_iface_chirho = self
                    .records_chirho
                    .get(dep_chirho)
                    .and_then(|r_chirho| r_chirho.get_phase_chirho(PhaseTagChirho::IfaceChirho));
                // If the dep doesn't have a record or its iface changed, it's the culprit.
                if current_dep_iface_chirho.is_none() {
                    return Some(RebuildReasonChirho::DepChangedChirho {
                        dep_name_chirho: dep_chirho.to_string(),
                    });
                }
            }
            // Generic dep-changed if we can't pinpoint.
            return Some(RebuildReasonChirho::DepChangedChirho {
                dep_name_chirho: "<unknown>".to_string(),
            });
        }

        // 3. All good.
        None
    }

    /// Record the result of compiling a module.
    pub fn record_compilation_chirho(
        &mut self,
        module_name_chirho: &str,
        source_fp_chirho: FingerprintChirho,
    ) -> &mut ModuleRecordChirho {
        let deps_fp_chirho = self.compute_deps_fp_chirho(module_name_chirho);
        let record_chirho =
            ModuleRecordChirho::new_chirho(module_name_chirho, source_fp_chirho, deps_fp_chirho);
        self.records_chirho
            .insert(module_name_chirho.to_string(), record_chirho);
        self.records_chirho.get_mut(module_name_chirho).unwrap()
    }

    /// Get the record for a module.
    pub fn get_record_chirho(&self, module_name_chirho: &str) -> Option<&ModuleRecordChirho> {
        self.records_chirho.get(module_name_chirho)
    }

    /// Get a mutable record for a module.
    pub fn get_record_mut_chirho(
        &mut self,
        module_name_chirho: &str,
    ) -> Option<&mut ModuleRecordChirho> {
        self.records_chirho.get_mut(module_name_chirho)
    }

    /// Determine the compilation order (topological sort).
    pub fn compilation_order_chirho(&self) -> Option<Vec<String>> {
        self.dep_graph_chirho.topo_sort_chirho()
    }

    /// Determine which modules need recompilation given current sources.
    /// Returns a list of (module_name, reason) pairs in compilation order.
    pub fn plan_rebuild_chirho(
        &self,
        sources_chirho: &HashMap<String, String>,
    ) -> Vec<(String, RebuildReasonChirho)> {
        let order_chirho = match self.dep_graph_chirho.topo_sort_chirho() {
            Some(o_chirho) => o_chirho,
            None => {
                // Cycle detected — rebuild everything.
                return sources_chirho
                    .keys()
                    .map(|name_chirho| {
                        (
                            name_chirho.clone(),
                            RebuildReasonChirho::NoPriorRecordChirho,
                        )
                    })
                    .collect();
            }
        };

        let mut result_chirho = Vec::new();
        let mut dirty_chirho = std::collections::HashSet::new();

        for module_chirho in &order_chirho {
            if !sources_chirho.contains_key(module_chirho) {
                continue;
            }
            let source_fp_chirho =
                FingerprintChirho::from_str_chirho(&sources_chirho[module_chirho]);

            // Check if any dependency is dirty.
            let deps_chirho = self.dep_graph_chirho.deps_of_chirho(module_chirho);
            let dep_dirty_chirho = deps_chirho
                .iter()
                .find(|d_chirho| dirty_chirho.contains(**d_chirho));

            if let Some(dirty_dep_chirho) = dep_dirty_chirho {
                dirty_chirho.insert(module_chirho.clone());
                result_chirho.push((
                    module_chirho.clone(),
                    RebuildReasonChirho::DepChangedChirho {
                        dep_name_chirho: dirty_dep_chirho.to_string(),
                    },
                ));
            } else if let Some(reason_chirho) =
                self.needs_rebuild_chirho(module_chirho, source_fp_chirho)
            {
                dirty_chirho.insert(module_chirho.clone());
                result_chirho.push((module_chirho.clone(), reason_chirho));
            }
        }

        result_chirho
    }

    /// Number of tracked module records.
    pub fn record_count_chirho(&self) -> usize {
        self.records_chirho.len()
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
    fn fresh_session_needs_rebuild_chirho() {
        let session_chirho = IncrementalSessionChirho::new_chirho();
        let reason_chirho = session_chirho.needs_rebuild_chirho("Main", fp_chirho("source"));
        assert_eq!(
            reason_chirho,
            Some(RebuildReasonChirho::NoPriorRecordChirho)
        );
    }

    #[test]
    fn unchanged_source_no_rebuild_chirho() {
        let mut session_chirho = IncrementalSessionChirho::new_chirho();
        session_chirho.register_module_chirho("Main", "source code", &[]);
        let source_fp_chirho = fp_chirho("source code");
        session_chirho.record_compilation_chirho("Main", source_fp_chirho);

        let reason_chirho = session_chirho.needs_rebuild_chirho("Main", source_fp_chirho);
        assert_eq!(reason_chirho, None);
    }

    #[test]
    fn changed_source_needs_rebuild_chirho() {
        let mut session_chirho = IncrementalSessionChirho::new_chirho();
        session_chirho.register_module_chirho("Main", "v1", &[]);
        session_chirho.record_compilation_chirho("Main", fp_chirho("v1"));

        let reason_chirho = session_chirho.needs_rebuild_chirho("Main", fp_chirho("v2"));
        assert_eq!(
            reason_chirho,
            Some(RebuildReasonChirho::SourceChangedChirho)
        );
    }

    #[test]
    fn dep_change_triggers_rebuild_chirho() {
        let mut session_chirho = IncrementalSessionChirho::new_chirho();
        session_chirho.register_module_chirho("Base", "base-v1", &[]);
        session_chirho.register_module_chirho("App", "app-v1", &["Base"]);

        // Compile Base with an interface fingerprint.
        let base_record_chirho =
            session_chirho.record_compilation_chirho("Base", fp_chirho("base-v1"));
        base_record_chirho
            .set_phase_chirho(PhaseTagChirho::IfaceChirho, fp_chirho("base-iface-v1"));

        // Compile App (records deps_fp based on Base's iface).
        session_chirho.record_compilation_chirho("App", fp_chirho("app-v1"));

        // App is up-to-date.
        assert_eq!(
            session_chirho.needs_rebuild_chirho("App", fp_chirho("app-v1")),
            None
        );

        // Now recompile Base with a different interface.
        let base_record_chirho =
            session_chirho.record_compilation_chirho("Base", fp_chirho("base-v2"));
        base_record_chirho
            .set_phase_chirho(PhaseTagChirho::IfaceChirho, fp_chirho("base-iface-v2"));

        // App needs rebuild because Base's iface changed.
        let reason_chirho = session_chirho.needs_rebuild_chirho("App", fp_chirho("app-v1"));
        assert!(matches!(
            reason_chirho,
            Some(RebuildReasonChirho::DepChangedChirho { .. })
        ));
    }

    #[test]
    fn plan_rebuild_all_fresh_chirho() {
        let mut session_chirho = IncrementalSessionChirho::new_chirho();
        session_chirho.register_module_chirho("A", "a-src", &[]);
        session_chirho.register_module_chirho("B", "b-src", &["A"]);

        let mut sources_chirho = HashMap::new();
        sources_chirho.insert("A".to_string(), "a-src".to_string());
        sources_chirho.insert("B".to_string(), "b-src".to_string());

        let plan_chirho = session_chirho.plan_rebuild_chirho(&sources_chirho);
        assert_eq!(plan_chirho.len(), 2);
        // A should come before B (topo order).
        assert_eq!(plan_chirho[0].0, "A");
        assert_eq!(plan_chirho[1].0, "B");
    }

    #[test]
    fn plan_rebuild_only_changed_chirho() {
        let mut session_chirho = IncrementalSessionChirho::new_chirho();
        session_chirho.register_module_chirho("Lib", "lib-v1", &[]);
        session_chirho.register_module_chirho("App", "app-v1", &["Lib"]);

        // Compile both.
        let lib_record_chirho =
            session_chirho.record_compilation_chirho("Lib", fp_chirho("lib-v1"));
        lib_record_chirho.set_phase_chirho(PhaseTagChirho::IfaceChirho, fp_chirho("lib-iface-v1"));
        session_chirho.record_compilation_chirho("App", fp_chirho("app-v1"));

        // Only App's source changes, Lib stays the same.
        let mut sources_chirho = HashMap::new();
        sources_chirho.insert("Lib".to_string(), "lib-v1".to_string());
        sources_chirho.insert("App".to_string(), "app-v2".to_string());

        let plan_chirho = session_chirho.plan_rebuild_chirho(&sources_chirho);
        assert_eq!(plan_chirho.len(), 1);
        assert_eq!(plan_chirho[0].0, "App");
        assert_eq!(plan_chirho[0].1, RebuildReasonChirho::SourceChangedChirho);
    }

    #[test]
    fn plan_rebuild_cascading_chirho() {
        let mut session_chirho = IncrementalSessionChirho::new_chirho();
        session_chirho.register_module_chirho("Base", "base-v1", &[]);
        session_chirho.register_module_chirho("Mid", "mid-v1", &["Base"]);
        session_chirho.register_module_chirho("Top", "top-v1", &["Mid"]);

        // Compile all.
        let base_rec_chirho =
            session_chirho.record_compilation_chirho("Base", fp_chirho("base-v1"));
        base_rec_chirho.set_phase_chirho(PhaseTagChirho::IfaceChirho, fp_chirho("base-iface-v1"));
        let mid_rec_chirho = session_chirho.record_compilation_chirho("Mid", fp_chirho("mid-v1"));
        mid_rec_chirho.set_phase_chirho(PhaseTagChirho::IfaceChirho, fp_chirho("mid-iface-v1"));
        session_chirho.record_compilation_chirho("Top", fp_chirho("top-v1"));

        // Base source changes → cascade to Mid and Top.
        let mut sources_chirho = HashMap::new();
        sources_chirho.insert("Base".to_string(), "base-v2".to_string());
        sources_chirho.insert("Mid".to_string(), "mid-v1".to_string());
        sources_chirho.insert("Top".to_string(), "top-v1".to_string());

        let plan_chirho = session_chirho.plan_rebuild_chirho(&sources_chirho);
        assert_eq!(plan_chirho.len(), 3);
        let names_chirho: Vec<&str> = plan_chirho
            .iter()
            .map(|(n_chirho, _)| n_chirho.as_str())
            .collect();
        assert_eq!(names_chirho, vec!["Base", "Mid", "Top"]);
    }

    #[test]
    fn module_record_phases_chirho() {
        let mut record_chirho = ModuleRecordChirho::new_chirho(
            "Test",
            fp_chirho("src"),
            FingerprintChirho::ZERO_CHIRHO,
        );
        assert!(
            record_chirho
                .get_phase_chirho(PhaseTagChirho::CoreChirho)
                .is_none()
        );

        record_chirho.set_phase_chirho(PhaseTagChirho::CoreChirho, fp_chirho("core-data"));
        assert_eq!(
            record_chirho.get_phase_chirho(PhaseTagChirho::CoreChirho),
            Some(fp_chirho("core-data"))
        );
    }

    #[test]
    fn phase_tag_labels_chirho() {
        assert_eq!(PhaseTagChirho::ParseChirho.label_chirho(), "parse");
        assert_eq!(PhaseTagChirho::LlvmIrChirho.label_chirho(), "llvm-ir");
        assert_eq!(PhaseTagChirho::WasmChirho.label_chirho(), "wasm");
        assert_eq!(PhaseTagChirho::IfaceChirho.label_chirho(), "iface");
    }

    #[test]
    fn rebuild_reason_display_chirho() {
        let reason_chirho = RebuildReasonChirho::SourceChangedChirho;
        assert_eq!(format!("{}", reason_chirho), "source changed");

        let reason_chirho = RebuildReasonChirho::DepChangedChirho {
            dep_name_chirho: "Data.List".to_string(),
        };
        assert_eq!(format!("{}", reason_chirho), "dependency Data.List changed");
    }

    #[test]
    fn compilation_order_chirho() {
        let mut session_chirho = IncrementalSessionChirho::new_chirho();
        session_chirho.register_module_chirho("C", "c", &["B"]);
        session_chirho.register_module_chirho("B", "b", &["A"]);
        session_chirho.register_module_chirho("A", "a", &[]);

        let order_chirho = session_chirho.compilation_order_chirho().unwrap();
        let pos_a_chirho = order_chirho.iter().position(|x| x == "A").unwrap();
        let pos_b_chirho = order_chirho.iter().position(|x| x == "B").unwrap();
        let pos_c_chirho = order_chirho.iter().position(|x| x == "C").unwrap();
        assert!(pos_a_chirho < pos_b_chirho);
        assert!(pos_b_chirho < pos_c_chirho);
    }
}
