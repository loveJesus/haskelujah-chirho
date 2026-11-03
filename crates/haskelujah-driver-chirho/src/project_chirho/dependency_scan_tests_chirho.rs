// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Behavioral tests of reached-only package scan errors and selection evidence.

use super::*;

struct PackageScratchChirho(PathBuf);

impl PackageScratchChirho {
    fn new_chirho() -> Self {
        static NEXT_CHIRHO: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let path_chirho = std::env::temp_dir().join(format!(
            "iface_package_scan_chirho_{}_{}",
            std::process::id(),
            NEXT_CHIRHO.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        std::fs::create_dir_all(path_chirho.join(".haskelujah-packages-chirho/current-chirho"))
            .expect("create package fixture");
        Self(path_chirho)
    }
    fn current_chirho(&self) -> PathBuf {
        self.0.join(".haskelujah-packages-chirho/current-chirho")
    }
    fn write_chirho(
        &self,
        package_chirho: &str,
        file_chirho: &str,
        source_chirho: &str,
    ) -> PathBuf {
        let path_chirho = self
            .0
            .join(".haskelujah-packages-chirho")
            .join(package_chirho)
            .join(file_chirho);
        std::fs::create_dir_all(path_chirho.parent().expect("source parent"))
            .expect("create source package");
        std::fs::write(&path_chirho, source_chirho).expect("write source package");
        std::fs::canonicalize(path_chirho).expect("canonical fixture source")
    }
}
impl Drop for PackageScratchChirho {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

const BAD_SOURCE_CHIRHO: &str = "{-# LANGUAGE GADTs, LinearTypes #-}\nmodule BadChirho where\ndata BoxChirho aChirho where\n  MkBoxChirho :: { payloadChirho :: aChirho } %1 -> BoxChirho aChirho\n";

fn source_chirho(import_chirho: &str) -> Vec<(String, String, String)> {
    vec![(
        "MainChirho".to_owned(),
        "MainChirho.hs".to_owned(),
        format!("module MainChirho where\n{import_chirho}\nvalueChirho :: Int\nvalueChirho = 3\n"),
    )]
}

#[test]
fn unused_malformed_cached_source_is_retained_then_reported_only_when_reached_chirho() {
    let scratch_chirho = PackageScratchChirho::new_chirho();
    let bad_path_chirho =
        scratch_chirho.write_chirho("bad-1-chirho", "BadChirho.hs", BAD_SOURCE_CHIRHO);
    let scan_chirho = scan_dependency_package_ifaces_chirho(&scratch_chirho.current_chirho());
    assert!(
        !scan_chirho
            .ifaces_chirho()
            .any(|iface_chirho| iface_chirho.name_chirho == "BadChirho")
    );
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let artifacts_chirho = super::super::collect_frontend_artifacts_from_module_sources_chirho(
        source_chirho(""),
        &mut source_map_chirho,
        scan_chirho,
        std::collections::HashMap::new(),
        crate::ImportedTypeSynonymsChirho::new(),
        crate::seed_builtin_type_families_chirho(),
        false,
    )
    .expect("an unused malformed cache module must not poison source checking");
    let errors_chirho = artifacts_chirho
        .validate_scanned_source_chirho(
            "NextChirho",
            "NextChirho.hs",
            "module NextChirho where\nimport BadChirho\n",
        )
        .expect_err("the retained seed must report a newly reached failed source");
    assert!(
        errors_chirho
            .to_string()
            .contains(bad_path_chirho.to_str().expect("source path"))
    );
    assert!(
        errors_chirho
            .diagnostics_chirho()
            .iter()
            .all(|diagnostic_chirho| diagnostic_chirho.labels_chirho.is_empty())
    );
}

#[test]
fn reached_constructor_failure_prevents_frontend_seed_publication_chirho() {
    let scratch_chirho = PackageScratchChirho::new_chirho();
    let bad_path_chirho =
        scratch_chirho.write_chirho("bad-1-chirho", "BadChirho.hs", BAD_SOURCE_CHIRHO);
    let scan_chirho = scan_dependency_package_ifaces_chirho(&scratch_chirho.current_chirho());
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let error_chirho = super::super::collect_frontend_artifacts_from_module_sources_chirho(
        source_chirho("import BadChirho"),
        &mut source_map_chirho,
        scan_chirho,
        std::collections::HashMap::new(),
        crate::ImportedTypeSynonymsChirho::new(),
        crate::seed_builtin_type_families_chirho(),
        false,
    )
    .expect_err("a selected reachable constructor error must fail the actual collector");
    assert!(error_chirho.contains("constructor signature in module BadChirho"));
    assert!(error_chirho.contains(bad_path_chirho.to_str().expect("source path")));
    assert!(error_chirho.contains("signature byte range"));
}

#[test]
fn selected_success_is_not_poisoned_by_an_unselected_failed_version_chirho() {
    let scratch_chirho = PackageScratchChirho::new_chirho();
    scratch_chirho.write_chirho("old-1-chirho", "BadChirho.hs", BAD_SOURCE_CHIRHO);
    let good_path_chirho = scratch_chirho.write_chirho(
        "new-2-chirho",
        "BadChirho.hs",
        "module BadChirho where\nvalueChirho = 3\n",
    );
    let mut scan_chirho = scan_dependency_package_ifaces_chirho(&scratch_chirho.current_chirho());
    let ambiguous_chirho = scan_chirho
        .validate_sources_chirho(&source_chirho("import BadChirho"))
        .expect_err("scan order is not version authority");
    assert!(
        ambiguous_chirho
            .to_string()
            .contains("selection is unproved for BadChirho")
    );
    scan_chirho.select_source_chirho("BadChirho".to_owned(), good_path_chirho);
    scan_chirho
        .validate_sources_chirho(&source_chirho("import BadChirho"))
        .expect("the selected successful source excludes the old failed candidate");
    assert_eq!(
        scan_chirho
            .ifaces_chirho()
            .filter(|iface_chirho| iface_chirho.name_chirho == "BadChirho")
            .count(),
        1
    );
}

#[test]
fn a_transitive_error_cannot_be_attributed_through_an_unselected_version_chirho() {
    let scratch_chirho = PackageScratchChirho::new_chirho();
    scratch_chirho.write_chirho("bad-1-chirho", "BadChirho.hs", BAD_SOURCE_CHIRHO);
    scratch_chirho.write_chirho(
        "old-1-chirho",
        "ForwardChirho.hs",
        "module ForwardChirho where\nimport BadChirho\n",
    );
    let selected_chirho = scratch_chirho.write_chirho(
        "new-2-chirho",
        "ForwardChirho.hs",
        "module ForwardChirho where\nvalueChirho = 3\n",
    );
    let mut scan_chirho = scan_dependency_package_ifaces_chirho(&scratch_chirho.current_chirho());
    let errors_chirho = scan_chirho
        .validate_sources_chirho(&source_chirho("import ForwardChirho"))
        .expect_err("a merged interface cannot prove which import edge was selected");
    assert!(
        errors_chirho
            .to_string()
            .contains("selection is unproved for ForwardChirho")
    );
    scan_chirho.select_source_chirho("ForwardChirho".to_owned(), selected_chirho);
    scan_chirho
        .validate_sources_chirho(&source_chirho("import ForwardChirho"))
        .expect("the selected forwarder never reaches the failed old dependency");
}

#[test]
fn checked_source_identity_overrides_its_failed_cached_copy_across_followups_chirho() {
    let scratch_chirho = PackageScratchChirho::new_chirho();
    scratch_chirho.write_chirho("old-1-chirho", "BadChirho.hs", BAD_SOURCE_CHIRHO);
    let scan_chirho = scan_dependency_package_ifaces_chirho(&scratch_chirho.current_chirho());
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let artifacts_chirho = super::super::collect_frontend_artifacts_from_module_sources_chirho(
        vec![(
            "BadChirho".to_owned(),
            "ActualBadChirho.hs".to_owned(),
            "module BadChirho where\nvalueChirho :: Int\nvalueChirho = 3\n".to_owned(),
        )],
        &mut source_map_chirho,
        scan_chirho,
        std::collections::HashMap::new(),
        crate::ImportedTypeSynonymsChirho::new(),
        crate::seed_builtin_type_families_chirho(),
        false,
    )
    .expect("an actual checked source overrides a cached candidate");
    artifacts_chirho
        .validate_scanned_source_chirho(
            "NextChirho",
            "NextChirho.hs",
            "module NextChirho where\nimport BadChirho\n",
        )
        .expect("the successful source override stays attached to frontend artifacts");
}
