// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Real source roots must supply checked dependencies, not header-only authority.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::{FAMILY_CONSUMER_CHIRHO, FAMILY_PROVIDER_CHIRHO};
use haskelujah_driver::{check_source_path_chirho, compile_source_with_search_path_chirho};
use haskelujah_runtime_chirho::ExecutionModeChirho;
use haskelujah_span_chirho::SourceMapChirho;

fn check_both_paths_chirho(source_chirho: &str, directory_chirho: &std::path::Path) {
    let path_chirho = directory_chirho.join("ConsumerChirho.hs");
    std::fs::write(&path_chirho, source_chirho).unwrap();
    check_source_path_chirho(&path_chirho, ExecutionModeChirho::BatchChirho)
        .unwrap_or_else(|error_chirho| panic!("file check: {error_chirho}"));
    compile_source_with_search_path_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ConsumerChirho.hs",
        directory_chirho,
    )
    .unwrap_or_else(|error_chirho| panic!("file compilation: {error_chirho}"));
}

#[test]
fn source_paths_check_imported_family_providers_before_the_consumer_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    std::fs::write(
        directory_chirho.path().join("ProviderChirho.hs"),
        FAMILY_PROVIDER_CHIRHO,
    )
    .unwrap();
    for source_chirho in [
        FAMILY_CONSUMER_CHIRHO.to_owned(),
        FAMILY_CONSUMER_CHIRHO
            .replace(
                "import ProviderChirho",
                "import qualified ProviderChirho as PChirho",
            )
            .replace(
                "CanDoChirho (StateChirho",
                "PChirho.CanDoChirho (StateChirho",
            ),
    ] {
        check_both_paths_chirho(&source_chirho, directory_chirho.path());
    }
}

#[test]
fn source_paths_check_reexport_dependencies_transitively_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    std::fs::write(
        directory_chirho.path().join("ProviderChirho.hs"),
        FAMILY_PROVIDER_CHIRHO,
    )
    .unwrap();
    std::fs::write(
        directory_chirho.path().join("ReexportChirho.hs"),
        "module ReexportChirho (CanDoChirho) where\nimport ProviderChirho\n",
    )
    .unwrap();
    check_both_paths_chirho(
        &FAMILY_CONSUMER_CHIRHO.replace("import ProviderChirho", "import ReexportChirho"),
        directory_chirho.path(),
    );
}

#[test]
fn source_paths_reject_wrong_imported_family_kinds_and_proofs_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    std::fs::write(
        directory_chirho.path().join("ProviderChirho.hs"),
        FAMILY_PROVIDER_CHIRHO,
    )
    .unwrap();
    for (source_chirho, expected_chirho) in [
        (
            FAMILY_CONSUMER_CHIRHO.replace("= StateCanDoChirho sChirho effChirho", "= Int"),
            "family equation result",
        ),
        (
            FAMILY_CONSUMER_CHIRHO.replace(
                "type instance CanDoChirho (StateChirho sChirho mChirho)",
                "type instance CanDoChirho (StateChirho sChirho mChirho Int)",
            ),
            "family equation argument",
        ),
        (
            FAMILY_CONSUMER_CHIRHO.replace(":~: 'True", ":~: 'False"),
            "type mismatch",
        ),
    ] {
        let path_chirho = directory_chirho.path().join("ConsumerChirho.hs");
        std::fs::write(&path_chirho, &source_chirho).unwrap();
        let check_chirho =
            check_source_path_chirho(&path_chirho, ExecutionModeChirho::BatchChirho).map(|_| ());
        let compile_chirho = compile_source_with_search_path_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "ConsumerChirho.hs",
            directory_chirho.path(),
        )
        .map(|_| ());
        for result_chirho in [check_chirho, compile_chirho] {
            let error_chirho =
                result_chirho.expect_err("the consumer must obey the checked import contract");
            assert!(
                error_chirho
                    .diagnostics_chirho()
                    .iter()
                    .any(|diagnostic_chirho| diagnostic_chirho
                        .message_chirho
                        .contains(expected_chirho)),
                "{error_chirho}"
            );
        }
    }
}

#[test]
fn source_paths_preserve_local_nominal_shadowing_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    std::fs::write(
        directory_chirho.path().join("ProviderChirho.hs"),
        FAMILY_PROVIDER_CHIRHO,
    )
    .unwrap();
    check_both_paths_chirho(
        include_str!(
            "../../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/imported-families-chirho/LocalShadowChirho.hs"
        ),
        directory_chirho.path(),
    );
}

#[test]
fn reachable_provider_failure_is_not_a_raw_interface_fallback_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    std::fs::write(
        directory_chirho.path().join("ProviderChirho.hs"),
        "module ProviderChirho where\nproviderCanaryChirho :: MissingProviderTypeChirho\nproviderCanaryChirho = ()\n",
    )
    .unwrap();
    let source_chirho = "module ConsumerChirho where\nimport ProviderChirho\nvalueChirho = ()\n";
    let path_chirho = directory_chirho.path().join("ConsumerChirho.hs");
    std::fs::write(&path_chirho, source_chirho).unwrap();
    let check_chirho =
        check_source_path_chirho(&path_chirho, ExecutionModeChirho::BatchChirho).map(|_| ());
    let compile_chirho = compile_source_with_search_path_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ConsumerChirho.hs",
        directory_chirho.path(),
    )
    .map(|_| ());
    for result_chirho in [check_chirho, compile_chirho] {
        let error_chirho =
            result_chirho.expect_err("a broken known provider must stop its consumer");
        let message_chirho = error_chirho.to_string();
        assert!(
            message_chirho.contains("ProviderChirho"),
            "{message_chirho}"
        );
        assert!(
            message_chirho.contains("MissingProviderTypeChirho"),
            "{message_chirho}"
        );
    }
}

#[test]
fn unrelated_broken_neighbor_is_not_a_dependency_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    std::fs::write(
        directory_chirho.path().join("UnrelatedChirho.hs"),
        "module UnrelatedChirho where\nbrokenChirho :: MissingUnrelatedTypeChirho\nbrokenChirho = ()\n",
    )
    .unwrap();
    check_both_paths_chirho(
        "module ConsumerChirho where\nvalueChirho :: Int\nvalueChirho = 42\n",
        directory_chirho.path(),
    );
}

#[test]
fn reachable_import_cycle_requires_a_real_boot_contract_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    for (file_chirho, source_chirho) in [
        (
            "CycleAChirho.hs",
            "module CycleAChirho where\nimport CycleBChirho\naChirho = ()\n",
        ),
        (
            "CycleBChirho.hs",
            "module CycleBChirho where\nimport CycleAChirho\nbChirho = ()\n",
        ),
    ] {
        std::fs::write(directory_chirho.path().join(file_chirho), source_chirho).unwrap();
    }
    let path_chirho = directory_chirho.path().join("ConsumerChirho.hs");
    std::fs::write(
        &path_chirho,
        "module ConsumerChirho where\nimport CycleAChirho\nvalueChirho = ()\n",
    )
    .unwrap();
    let error_chirho = check_source_path_chirho(&path_chirho, ExecutionModeChirho::BatchChirho)
        .expect_err("a source cycle has no checked interface without boot support");
    assert!(error_chirho.to_string().contains("cycle"), "{error_chirho}");
}

#[test]
fn source_paths_follow_multiline_import_syntax_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    std::fs::write(
        directory_chirho.path().join("ProviderChirho.hs"),
        FAMILY_PROVIDER_CHIRHO,
    )
    .unwrap();
    check_both_paths_chirho(
        &FAMILY_CONSUMER_CHIRHO.replace(
            "import ProviderChirho",
            "import\n  ProviderChirho\n  (CanDoChirho)",
        ),
        directory_chirho.path(),
    );
}

#[test]
fn source_paths_multiline_imports_retain_value_and_synonym_contracts_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    std::fs::write(directory_chirho.path().join("ProviderChirho.hs"),
        "module ProviderChirho where\ntype CountChirho = Int\nonlyIntChirho :: Int -> Int\nonlyIntChirho xChirho = xChirho\n").unwrap();
    for import_chirho in [
        "import ProviderChirho",
        "import\n  ProviderChirho\n  (onlyIntChirho, CountChirho)",
    ] {
        for (binding_chirho, accepts_chirho) in [
            ("valueChirho :: Int\nvalueChirho = onlyIntChirho 42\n", true),
            ("valueChirho :: CountChirho\nvalueChirho = 42\n", true),
            (
                "valueChirho :: Bool\nvalueChirho = onlyIntChirho True\n",
                false,
            ),
            ("valueChirho :: CountChirho\nvalueChirho = True\n", false),
        ] {
            let source_chirho =
                format!("module ConsumerChirho where\n{import_chirho}\n{binding_chirho}");
            let path_chirho = directory_chirho.path().join("ConsumerChirho.hs");
            std::fs::write(&path_chirho, &source_chirho).unwrap();
            let check_chirho =
                check_source_path_chirho(&path_chirho, ExecutionModeChirho::BatchChirho)
                    .map(|_| ());
            let compile_chirho = compile_source_with_search_path_chirho(
                &source_chirho,
                &mut SourceMapChirho::new_chirho(),
                "ConsumerChirho.hs",
                directory_chirho.path(),
            )
            .map(|_| ());
            for result_chirho in [check_chirho, compile_chirho] {
                if accepts_chirho {
                    result_chirho.unwrap();
                } else {
                    let error_chirho = result_chirho
                        .expect_err(
                            "an import modifier must not discard a checked value or alias contract",
                        )
                        .to_string();
                    assert!(
                        error_chirho.contains("E0200")
                            && error_chirho.contains("Bool")
                            && error_chirho.contains("Int"),
                        "{error_chirho}"
                    );
                }
            }
        }
    }
}

#[test]
fn provider_preprocessing_and_byte_limit_fail_closed_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    let source_chirho = "module ConsumerChirho where\nimport ProviderChirho\nvalueChirho = ()\n";
    let path_chirho = directory_chirho.path().join("ConsumerChirho.hs");
    std::fs::write(&path_chirho, source_chirho).unwrap();
    for (provider_chirho, expected_chirho) in [
        (
            "{-# LANGUAGE CPP #-}\nmodule ProviderChirho where\n#error ProviderCppCanaryChirho\nvalueChirho = ()\n".to_owned(),
            "ProviderCppCanaryChirho",
        ),
        (
            format!("module ProviderChirho where\n-- {}\nvalueChirho = ()\n", "x".repeat(8 * 1024 * 1024)),
            "byte budget exhausted",
        ),
    ] {
        std::fs::write(directory_chirho.path().join("ProviderChirho.hs"), provider_chirho).unwrap();
        let check_chirho = check_source_path_chirho(&path_chirho, ExecutionModeChirho::BatchChirho).map(|_| ());
        let compile_chirho = compile_source_with_search_path_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "ConsumerChirho.hs",
            directory_chirho.path(),
        ).map(|_| ());
        for result_chirho in [check_chirho, compile_chirho] {
            let error_chirho = result_chirho.expect_err("an unreadable provider cannot become a partial-success interface").to_string();
            assert!(error_chirho.contains("ProviderChirho"), "{error_chirho}");
            assert!(error_chirho.contains(expected_chirho), "{error_chirho}");
        }
    }
}
