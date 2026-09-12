// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! SOURCE edges select checked boot contracts, not implementation lookalikes.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use haskelujah_driver::{check_source_path_chirho, compile_source_with_search_path_chirho};
use haskelujah_runtime_chirho::ExecutionModeChirho;
use haskelujah_span_chirho::SourceMapChirho;

const PROVIDER_CHIRHO: &str = "module ProviderChirho where\nimport ConsumerChirho\ndata BoxChirho aChirho = BoxChirho aChirho\nsameChirho :: BoxChirho aChirho -> BoxChirho aChirho\nsameChirho xChirho = xChirho\n";
const BOOT_CHIRHO: &str = "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ndata BoxChirho (aChirho :: Type)\nsameChirho :: BoxChirho aChirho -> BoxChirho aChirho\n";
const CONSUMER_CHIRHO: &str = "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\npingChirho :: BoxChirho Int -> BoxChirho Int\npingChirho = sameChirho\n";

fn boot_paths_chirho(
    provider_chirho: &str,
    boot_chirho: Option<&str>,
    consumer_chirho: &str,
) -> Vec<Result<(), String>> {
    optional_implementation_paths_chirho(Some(provider_chirho), boot_chirho, consumer_chirho)
}

fn optional_implementation_paths_chirho(
    provider_chirho: Option<&str>,
    boot_chirho: Option<&str>,
    consumer_chirho: &str,
) -> Vec<Result<(), String>> {
    let directory_chirho = tempfile::tempdir().unwrap();
    for (name_chirho, source_chirho) in [
        ("ProviderChirho.hs", provider_chirho),
        ("ConsumerChirho.hs", Some(consumer_chirho)),
    ] {
        if let Some(source_chirho) = source_chirho {
            std::fs::write(directory_chirho.path().join(name_chirho), source_chirho).unwrap();
        }
    }
    if let Some(boot_chirho) = boot_chirho {
        std::fs::write(
            directory_chirho.path().join("ProviderChirho.hs-boot"),
            boot_chirho,
        )
        .unwrap();
    }
    vec![
        check_source_path_chirho(
            directory_chirho.path().join("ConsumerChirho.hs"),
            ExecutionModeChirho::BatchChirho,
        )
        .map(|_| ())
        .map_err(|error_chirho| error_chirho.to_string()),
        compile_source_with_search_path_chirho(
            consumer_chirho,
            &mut SourceMapChirho::new_chirho(),
            "ConsumerChirho.hs",
            directory_chirho.path(),
        )
        .map(|_| ())
        .map_err(|error_chirho| error_chirho.to_string()),
    ]
}

#[test]
fn source_boot_cycle_checks_both_providers_and_the_consumer_chirho() {
    for result_chirho in boot_paths_chirho(PROVIDER_CHIRHO, Some(BOOT_CHIRHO), CONSUMER_CHIRHO) {
        result_chirho.unwrap();
    }
}

#[test]
fn source_boot_missing_contract_is_not_an_implementation_fallback_chirho() {
    for result_chirho in boot_paths_chirho(PROVIDER_CHIRHO, None, CONSUMER_CHIRHO) {
        let error_chirho = result_chirho.expect_err("SOURCE requires an actual boot file");
        assert!(
            error_chirho.contains("ProviderChirho.hs-boot"),
            "{error_chirho}"
        );
    }
}

#[test]
fn source_boot_invalid_contract_preserves_its_own_diagnostic_chirho() {
    let boot_chirho = BOOT_CHIRHO.replace("BoxChirho aChirho ->", "MissingBootTypeChirho ->");
    for result_chirho in boot_paths_chirho(PROVIDER_CHIRHO, Some(&boot_chirho), CONSUMER_CHIRHO) {
        let error_chirho = result_chirho.expect_err("the boot declaration itself must be checked");
        assert!(
            error_chirho.contains("MissingBootTypeChirho"),
            "{error_chirho}"
        );
        assert!(
            error_chirho.contains("ProviderChirho.hs-boot"),
            "{error_chirho}"
        );
    }
}

#[test]
fn source_boot_implementation_must_not_specialize_a_polymorphic_promise_chirho() {
    let provider_chirho = PROVIDER_CHIRHO.replace(
        "sameChirho :: BoxChirho aChirho -> BoxChirho aChirho",
        "sameChirho :: BoxChirho Int -> BoxChirho Int",
    );
    for result_chirho in boot_paths_chirho(&provider_chirho, Some(BOOT_CHIRHO), CONSUMER_CHIRHO) {
        let error_chirho =
            result_chirho.expect_err("matching uses cannot validate a polymorphic promise");
        assert!(
            error_chirho.contains("boot contract") && error_chirho.contains("sameChirho"),
            "{error_chirho}"
        );
    }
}

#[test]
fn source_boot_does_not_hide_an_invalid_available_implementation_chirho() {
    let provider_chirho = format!(
        "{PROVIDER_CHIRHO}canaryChirho :: MissingImplementationTypeChirho\ncanaryChirho = ()\n"
    );
    for result_chirho in boot_paths_chirho(&provider_chirho, Some(BOOT_CHIRHO), CONSUMER_CHIRHO) {
        let error_chirho = result_chirho.expect_err("a valid boot is not a valid implementation");
        assert!(
            error_chirho.contains("MissingImplementationTypeChirho"),
            "{error_chirho}"
        );
    }
}

#[test]
fn source_boot_kind_agreement_is_not_proved_by_one_valid_application_chirho() {
    let boot_chirho = BOOT_CHIRHO
        .replace(
            "{-# LANGUAGE KindSignatures #-}",
            "{-# LANGUAGE KindSignatures, PolyKinds #-}",
        )
        .replace("(aChirho :: Type)", "(aChirho :: kChirho)");
    for result_chirho in boot_paths_chirho(PROVIDER_CHIRHO, Some(&boot_chirho), CONSUMER_CHIRHO) {
        let error_chirho = result_chirho
            .expect_err("the implementation kind must fulfill the complete boot promise");
        assert!(
            error_chirho.contains("boot contract") && error_chirho.contains("BoxChirho"),
            "{error_chirho}"
        );
    }
}

#[test]
fn source_boot_consumers_cannot_see_implementation_only_names_chirho() {
    let provider_chirho = format!("{PROVIDER_CHIRHO}secretChirho :: Int\nsecretChirho = 1\n");
    let consumer_chirho =
        format!("{CONSUMER_CHIRHO}leakChirho :: Int\nleakChirho = secretChirho\n");
    for result_chirho in boot_paths_chirho(&provider_chirho, Some(BOOT_CHIRHO), &consumer_chirho) {
        let error_chirho =
            result_chirho.expect_err("boot imports expose only their own declarations");
        assert!(error_chirho.contains("secretChirho"), "{error_chirho}");
    }
}

#[test]
fn source_boot_is_not_inferred_from_an_unused_boot_file_chirho() {
    let consumer_chirho = CONSUMER_CHIRHO.replace("{-# SOURCE #-} ", "");
    for result_chirho in boot_paths_chirho(PROVIDER_CHIRHO, Some(BOOT_CHIRHO), &consumer_chirho) {
        let error_chirho = result_chirho.expect_err("ordinary import cycles remain cycles");
        assert!(error_chirho.contains("cycle"), "{error_chirho}");
    }
}

#[test]
fn source_boot_value_scheme_is_checked_at_the_consumer_chirho() {
    let consumer_chirho = CONSUMER_CHIRHO.replace(
        "BoxChirho Int -> BoxChirho Int",
        "BoxChirho Int -> BoxChirho Bool",
    );
    for result_chirho in boot_paths_chirho(PROVIDER_CHIRHO, Some(BOOT_CHIRHO), &consumer_chirho) {
        let error_chirho =
            result_chirho.expect_err("a boot name must not receive a placeholder forall a. a");
        assert!(
            error_chirho.contains("E0200")
                && error_chirho.contains("Int")
                && error_chirho.contains("Bool"),
            "{error_chirho}"
        );
    }
}

#[test]
fn source_boot_definition_cannot_replace_a_declaration_chirho() {
    let boot_chirho = format!("{BOOT_CHIRHO}sameChirho xChirho = xChirho\n");
    for result_chirho in boot_paths_chirho(PROVIDER_CHIRHO, Some(&boot_chirho), CONSUMER_CHIRHO) {
        let error_chirho =
            result_chirho.expect_err("boot values declare a promise, not a definition");
        assert!(
            error_chirho.contains("boot contract") && error_chirho.contains("sameChirho"),
            "{error_chirho}"
        );
    }
}

#[test]
fn source_boot_agreement_uses_an_inferred_implementation_scheme_chirho() {
    let provider_chirho = PROVIDER_CHIRHO.replace(
        "sameChirho :: BoxChirho aChirho -> BoxChirho aChirho\nsameChirho xChirho = xChirho",
        "sameChirho (BoxChirho xChirho) = BoxChirho xChirho",
    );
    for result_chirho in boot_paths_chirho(&provider_chirho, Some(BOOT_CHIRHO), CONSUMER_CHIRHO) {
        result_chirho.unwrap();
    }
}

#[test]
fn source_boot_requires_an_implementation_source_chirho() {
    for result_chirho in
        optional_implementation_paths_chirho(None, Some(BOOT_CHIRHO), CONSUMER_CHIRHO)
    {
        let error_chirho =
            result_chirho.expect_err("a source graph cannot complete with only a boot promise");
        assert!(
            error_chirho.contains("ProviderChirho")
                && error_chirho.contains("no implementation source"),
            "{error_chirho}"
        );
    }
}

#[test]
fn source_boot_exports_must_exist_in_the_implementation_interface_chirho() {
    for (exports_chirho, name_chirho) in
        [("(BoxChirho)", "sameChirho"), ("(sameChirho)", "BoxChirho")]
    {
        let provider_chirho = PROVIDER_CHIRHO.replace(
            "module ProviderChirho where",
            &format!("module ProviderChirho {exports_chirho} where"),
        );
        for result_chirho in boot_paths_chirho(&provider_chirho, Some(BOOT_CHIRHO), CONSUMER_CHIRHO)
        {
            let error_chirho = result_chirho.expect_err(
                "a private implementation declaration does not fulfill a public boot export",
            );
            assert!(
                error_chirho.contains("boot contract")
                    && error_chirho.contains(name_chirho)
                    && error_chirho.contains("export"),
                "{error_chirho}"
            );
        }
    }
    let provider_chirho = PROVIDER_CHIRHO.replace(
        "module ProviderChirho where",
        "module ProviderChirho (BoxChirho, sameChirho) where",
    );
    for result_chirho in boot_paths_chirho(&provider_chirho, Some(BOOT_CHIRHO), CONSUMER_CHIRHO) {
        result_chirho.unwrap();
    }
}

#[test]
fn source_boot_constructor_members_must_be_exported_by_the_implementation_chirho() {
    let boot_chirho = BOOT_CHIRHO.replace(
        "data BoxChirho (aChirho :: Type)",
        "data BoxChirho (aChirho :: Type) = BoxChirho aChirho",
    );
    for (members_chirho, accepts_chirho) in [("BoxChirho", false), ("BoxChirho(..)", true)] {
        let provider_chirho = PROVIDER_CHIRHO.replace(
            "module ProviderChirho where",
            &format!("module ProviderChirho ({members_chirho}, sameChirho) where"),
        );
        for result_chirho in
            boot_paths_chirho(&provider_chirho, Some(&boot_chirho), CONSUMER_CHIRHO)
        {
            if accepts_chirho {
                result_chirho.unwrap();
            } else {
                let error_chirho = result_chirho.expect_err(
                    "boot constructor export requires an implementation constructor export",
                );
                assert!(
                    error_chirho.contains("boot contract")
                        && error_chirho.contains("BoxChirho")
                        && error_chirho.contains("export"),
                    "{error_chirho}"
                );
            }
        }
    }
}
