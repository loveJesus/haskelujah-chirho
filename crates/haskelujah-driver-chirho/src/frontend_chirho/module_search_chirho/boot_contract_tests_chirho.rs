// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Filesystem controls for reached SOURCE contracts. No external tools are run.

use crate::module_search_chirho::{
    DiscoveredFamilyContextChirho, DiscoveredSourceModulesChirho,
    register_discovered_type_families_chirho, scan_sibling_modules_with_sources_chirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_diagnostics_chirho::DiagnosticBundleChirho;
use haskelujah_naming_chirho::iface_chirho::ModuleIfaceChirho;
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::{FileIdChirho, SourceMapChirho};
use haskelujah_typing_chirho::infer_chirho::{
    TypeFamilyRegistryChirho, seed_type_family_registry_chirho,
};
use std::path::{Path, PathBuf};

struct BootProjectChirho {
    path_chirho: PathBuf,
}

impl BootProjectChirho {
    fn new_chirho() -> Self {
        static NEXT_CHIRHO: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let path_chirho = std::env::temp_dir().join(format!(
            "reached_boot_contract_chirho_{}_{}",
            std::process::id(),
            NEXT_CHIRHO.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&path_chirho).expect("create boot contract fixture");
        Self { path_chirho }
    }

    fn write_chirho(&self, name_chirho: &str, source_chirho: &str) -> PathBuf {
        let path_chirho = self.path_chirho.join(name_chirho);
        std::fs::write(&path_chirho, source_chirho).expect("write boot contract fixture");
        path_chirho
    }

    fn prepare_chirho(&self, checked_name_chirho: &str) -> BootStateChirho {
        let source_chirho = std::fs::read_to_string(self.path_chirho.join(checked_name_chirho))
            .expect("read checked fixture");
        let tree_chirho =
            ParserChirho::new_chirho(&source_chirho, FileIdChirho::SYNTHETIC_CHIRHO).parse_chirho();
        let mut module_chirho = lower_module_chirho(&tree_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
        crate::inject_prelude_import_chirho(&mut module_chirho);
        let mut sources_chirho = DiscoveredSourceModulesChirho::new();
        let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
        scan_sibling_modules_with_sources_chirho(
            &self.path_chirho,
            &mut SourceMapChirho::new_chirho(),
            &mut ifaces_chirho,
            checked_name_chirho,
            &mut sources_chirho,
        );
        BootStateChirho {
            module_chirho,
            sources_chirho,
            ifaces_chirho,
            registry_chirho: seed_type_family_registry_chirho(),
        }
    }
}

impl Drop for BootProjectChirho {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path_chirho);
    }
}

struct BootStateChirho {
    module_chirho: ModuleChirho,
    sources_chirho: DiscoveredSourceModulesChirho,
    ifaces_chirho: Vec<ModuleIfaceChirho>,
    registry_chirho: TypeFamilyRegistryChirho,
}

impl BootStateChirho {
    fn register_chirho(
        &mut self,
        checked_path_chirho: Option<&Path>,
    ) -> Result<(), DiagnosticBundleChirho> {
        register_discovered_type_families_chirho(
            &self.module_chirho,
            DiscoveredFamilyContextChirho {
                imported_types_chirho: None,
                modules_chirho: &self.sources_chirho,
                ifaces_chirho: &mut self.ifaces_chirho,
                registry_chirho: &mut self.registry_chirho,
                imported_synonyms_chirho: &crate::ImportedTypeSynonymsChirho::new(),
                checked_path_chirho,
            },
        )
    }

    fn assert_atomic_failure_chirho(&mut self, checked_path_chirho: &Path, expected_chirho: &str) {
        let registry_before_chirho = format!("{:?}", self.registry_chirho);
        let ifaces_before_chirho = format!("{:?}", self.ifaces_chirho);
        let diagnostics_chirho = self
            .register_chirho(Some(checked_path_chirho))
            .expect_err("boot validation must fail");
        assert!(
            format!("{diagnostics_chirho}").contains(expected_chirho),
            "{diagnostics_chirho}"
        );
        assert_eq!(
            format!("{:?}", self.registry_chirho),
            registry_before_chirho
        );
        assert_eq!(format!("{:?}", self.ifaces_chirho), ifaces_before_chirho);
    }
}

#[test]
fn exact_t12042_rejects_the_explicit_data_synonym_contract_conflict_chirho() {
    let project_chirho = BootProjectChirho::new_chirho();
    let checked_path_chirho = project_chirho.write_chirho(
        "T12042.hs",
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/T12042.hs"),
    );
    project_chirho.write_chirho(
        "T12042a.hs",
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/T12042a.hs"),
    );
    project_chirho.write_chirho(
        "T12042.hs-boot",
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/T12042.hs-boot"),
    );
    project_chirho
        .prepare_chirho("T12042.hs")
        .assert_atomic_failure_chirho(&checked_path_chirho, "GHC-15843");
}

#[test]
fn matching_abstract_data_source_cycle_is_admitted_chirho() {
    let project_chirho = BootProjectChirho::new_chirho();
    let checked_path_chirho = project_chirho.write_chirho("OwnerChirho.hs", "module OwnerChirho where\nimport PeerChirho\ndata SChirho = SChirho\ntype RChirho = SChirho\n");
    project_chirho.write_chirho(
        "PeerChirho.hs",
        "module PeerChirho where\nimport {-# SOURCE #-} OwnerChirho\ntype UChirho = SChirho\n",
    );
    project_chirho.write_chirho(
        "OwnerChirho.hs-boot",
        "module OwnerChirho where\ndata SChirho\ntype RChirho = SChirho\n",
    );
    project_chirho
        .prepare_chirho("OwnerChirho.hs")
        .register_chirho(Some(&checked_path_chirho))
        .expect("matching abstract data contract");
}

#[test]
fn matching_cycle_file_run_preserves_the_checked_path_chirho() {
    let project_chirho = BootProjectChirho::new_chirho();
    let source_chirho =
        "module Main where\nimport PeerChirho\ndata SChirho = SChirho\nmain = print (3 :: Int)\n";
    let checked_path_chirho = project_chirho.write_chirho("Main.hs", source_chirho);
    project_chirho.write_chirho(
        "PeerChirho.hs",
        "module PeerChirho where\nimport {-# SOURCE #-} Main\ntype UChirho = SChirho\n",
    );
    project_chirho.write_chirho("Main.hs-boot", "module Main where\ndata SChirho\n");
    let outcome_chirho = crate::eval_source_file_outcome_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        &checked_path_chirho,
        None,
    )
    .expect("real file SOURCE cycle keeps path authority");
    assert!(
        outcome_chirho.result_chirho.is_ok(),
        "{:?}",
        outcome_chirho.result_chirho
    );
    assert_eq!(outcome_chirho.machine_chirho.io_output_chirho, "3\n");
}

#[test]
fn conflicting_nearby_boot_without_a_reached_source_edge_is_ignored_chirho() {
    let project_chirho = BootProjectChirho::new_chirho();
    let checked_path_chirho = project_chirho.write_chirho(
        "CheckedChirho.hs",
        "module CheckedChirho where\nimport AliasChirho\nvalueChirho :: Int\nvalueChirho = 3\n",
    );
    project_chirho.write_chirho(
        "AliasChirho.hs",
        "module AliasChirho where\ntype SChirho = Int\n",
    );
    project_chirho.write_chirho(
        "AliasChirho.hs-boot",
        "module AliasChirho where\ndata SChirho\n",
    );
    project_chirho.write_chirho(
        "UnusedChirho.hs",
        "module UnusedChirho where\nimport {-# SOURCE #-} AliasChirho\n",
    );
    project_chirho
        .prepare_chirho("CheckedChirho.hs")
        .register_chirho(Some(&checked_path_chirho))
        .expect("unused boot and unreachable SOURCE edge have no contract effect");
}

#[test]
fn reached_boot_must_declare_the_actual_target_module_chirho() {
    let project_chirho = BootProjectChirho::new_chirho();
    let checked_path_chirho = project_chirho.write_chirho(
        "CheckedChirho.hs",
        "module CheckedChirho where\nimport {-# SOURCE #-} OwnerChirho\n",
    );
    project_chirho.write_chirho(
        "OwnerChirho.hs",
        "module OwnerChirho where\ndata SChirho = SChirho\n",
    );
    project_chirho.write_chirho(
        "OwnerChirho.hs-boot",
        "module WrongChirho where\ndata SChirho\n",
    );
    project_chirho
        .prepare_chirho("CheckedChirho.hs")
        .assert_atomic_failure_chirho(
            &checked_path_chirho,
            "declares module WrongChirho, expected OwnerChirho",
        );
}

#[test]
fn missing_reached_boot_preserves_registry_and_interfaces_chirho() {
    let project_chirho = BootProjectChirho::new_chirho();
    let checked_path_chirho = project_chirho.write_chirho(
        "CheckedChirho.hs",
        "module CheckedChirho where\nimport {-# SOURCE #-} OwnerChirho\n",
    );
    project_chirho.write_chirho(
        "OwnerChirho.hs",
        "module OwnerChirho where\ndata SChirho = SChirho\n",
    );
    project_chirho
        .prepare_chirho("CheckedChirho.hs")
        .assert_atomic_failure_chirho(&checked_path_chirho, "cannot load reached SOURCE contract");
}

#[test]
fn malformed_reached_boot_preserves_registry_and_interfaces_chirho() {
    let project_chirho = BootProjectChirho::new_chirho();
    let checked_path_chirho = project_chirho.write_chirho(
        "CheckedChirho.hs",
        "module CheckedChirho where\nimport {-# SOURCE #-} OwnerChirho\n",
    );
    project_chirho.write_chirho(
        "OwnerChirho.hs",
        "module OwnerChirho where\ndata SChirho = SChirho\n",
    );
    project_chirho.write_chirho("OwnerChirho.hs-boot", "module OwnerChirho where\n)\n");
    project_chirho
        .prepare_chirho("CheckedChirho.hs")
        .assert_atomic_failure_chirho(
            &checked_path_chirho,
            "cannot validate reached SOURCE contract",
        );
}

#[test]
fn absent_lowered_declaration_is_not_used_as_a_contract_refutation_chirho() {
    let project_chirho = BootProjectChirho::new_chirho();
    let checked_path_chirho = project_chirho.write_chirho(
        "CheckedChirho.hs",
        "module CheckedChirho where\nimport {-# SOURCE #-} OwnerChirho\n",
    );
    project_chirho.write_chirho("OwnerChirho.hs", "module OwnerChirho where\n");
    project_chirho.write_chirho(
        "OwnerChirho.hs-boot",
        "module OwnerChirho where\ndata SChirho\n",
    );
    project_chirho
        .prepare_chirho("CheckedChirho.hs")
        .register_chirho(Some(&checked_path_chirho))
        .expect(
            "this cut proves explicit contradictions only, not absence from incomplete metadata",
        );
}

#[test]
fn synthetic_checked_name_cannot_supply_a_source_back_edges_boot_path_chirho() {
    let project_chirho = BootProjectChirho::new_chirho();
    project_chirho.write_chirho(
        "OwnerChirho.hs",
        "module OwnerChirho where\nimport PeerChirho\ndata SChirho = SChirho\n",
    );
    project_chirho.write_chirho(
        "PeerChirho.hs",
        "module PeerChirho where\nimport {-# SOURCE #-} OwnerChirho\n",
    );
    project_chirho.write_chirho(
        "OwnerChirho.hs-boot",
        "module OwnerChirho where\ndata SChirho\n",
    );
    let diagnostics_chirho = project_chirho
        .prepare_chirho("OwnerChirho.hs")
        .register_chirho(None)
        .expect_err("a source name alone is not path authority");
    assert!(format!("{diagnostics_chirho}").contains("no authoritative source path"));
}
