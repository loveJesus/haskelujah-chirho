// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A kind dependency is not a declaration promised by its importing boot file.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::boot_chirho::source_graph_paths_chirho;

const CYCLE_CHIRHO: &[(&str, &str)] = &[
    (
        "ProviderChirho.hs",
        "module ProviderChirho where\nimport {-# SOURCE #-} ConsumerChirho\ndata SeedChirho = SeedChirho\ncopiedChirho = valueChirho\n",
    ),
    (
        "ProviderChirho.hs-boot",
        "module ProviderChirho where\ndata SeedChirho\n",
    ),
    (
        "ConsumerChirho.hs",
        "module ConsumerChirho where\nimport ProviderChirho\nvalueChirho = SeedChirho\n",
    ),
    (
        "ConsumerChirho.hs-boot",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho :: SeedChirho\n",
    ),
];

const PRIVATE_HEAD_CHIRHO: &[(&str, &str)] = &[
    (
        "ProviderChirho.hs",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type) = SecretChirho aChirho\n",
    ),
    (
        "ProviderChirho.hs-boot",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
    ),
    (
        "ConsumerChirho.hs",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
    ),
];

#[test]
fn imported_boot_kind_dependency_is_not_a_local_declaration_promise_chirho() {
    // The identical four-file program accepts with GHC9.14.1 from either root.
    for root_chirho in ["ProviderChirho.hs", "ConsumerChirho.hs"] {
        for result_chirho in source_graph_paths_chirho(CYCLE_CHIRHO, root_chirho) {
            result_chirho.unwrap_or_else(|error_chirho| panic!("{root_chirho}: {error_chirho}"));
        }
    }
}

#[test]
fn private_matching_boot_kind_does_not_require_a_public_export_chirho() {
    for result_chirho in source_graph_paths_chirho(PRIVATE_HEAD_CHIRHO, "ConsumerChirho.hs") {
        result_chirho.unwrap();
    }
}

#[test]
fn private_declared_boot_kind_is_checked_without_a_public_use_chirho() {
    // GHC9.14.1 rejects this for GHC-15843: the declared kinds differ even
    // though neither module exports SecretChirho or uses it in a value type.
    let mismatched_chirho = PRIVATE_HEAD_CHIRHO[1]
        .1
        .replace("(aChirho :: Type)", "(aChirho :: Type -> Type)");
    let mut sources_chirho = PRIVATE_HEAD_CHIRHO.to_vec();
    sources_chirho[1].1 = &mismatched_chirho;
    for result_chirho in source_graph_paths_chirho(&sources_chirho, "ConsumerChirho.hs") {
        let error_chirho = result_chirho.expect_err("an unused local promise still has a kind");
        assert!(
            error_chirho.contains("boot contract") && error_chirho.contains("SecretChirho"),
            "{error_chirho}"
        );
    }
}
