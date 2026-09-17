// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Sources are byte-identical to private-chirho/observations-chirho.jsonl, GHC9.14.1.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::boot_chirho::boot_paths_chirho;

fn check_reference_chirho(
    provider_chirho: &str,
    boot_chirho: &str,
    consumer_chirho: &str,
    reason_chirho: &[&str],
) {
    for result_chirho in boot_paths_chirho(provider_chirho, Some(boot_chirho), consumer_chirho) {
        if reason_chirho.is_empty() {
            result_chirho.unwrap();
        } else {
            let error_chirho =
                result_chirho.expect_err("the reference requires rejection for this contract");
            for fragment_chirho in reason_chirho {
                assert!(
                    error_chirho.contains(fragment_chirho),
                    "{fragment_chirho}: {error_chirho}"
                );
            }
        }
    }
}

#[test]
fn a1_private_absent_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &[],
    );
}

#[test]
fn a2_private_mismatched_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\ndata SecretChirho = SecretChirho\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &["boot contract", "SecretChirho", "kind"],
    );
}

#[test]
fn a3_exported_absent_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (SecretChirho) where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &["boot contract", "export"],
    );
}

#[test]
fn a4_exported_defined_but_unexported_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type) = SecretChirho aChirho\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (SecretChirho) where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &["boot contract", "export"],
    );
}

#[test]
fn a5_exported_matching_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (SecretChirho) where\nimport ConsumerChirho\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type) = SecretChirho aChirho\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (SecretChirho) where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &[],
    );
}

#[test]
fn b1_nolist_absent_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &["boot contract", "export"],
    );
}

#[test]
fn b2_nolist_matching_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type) = SecretChirho aChirho\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &[],
    );
}

#[test]
fn b3_nolist_kind_mismatch_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ndata SecretChirho = SecretChirho\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &["boot contract", "SecretChirho", "kind"],
    );
}

#[test]
fn b4_nolist_defined_but_unexported_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type) = SecretChirho aChirho\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &["boot contract", "export"],
    );
}

#[test]
fn c1_via_signature_matching_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (fChirho) where\nimport ConsumerChirho\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type) = SecretChirho aChirho\nfChirho :: SecretChirho Int -> Int\nfChirho _ = 0\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (fChirho) where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\nfChirho :: SecretChirho Int -> Int\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &[],
    );
}

#[test]
fn c2_via_signature_type_absent_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (fChirho) where\nimport ConsumerChirho\nimport Data.Kind (Type)\nfChirho :: Int -> Int\nfChirho _ = 0\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (fChirho) where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\nfChirho :: SecretChirho Int -> Int\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &["boot contract", "fChirho"],
    );
}

#[test]
fn c3_via_signature_private_mismatched_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (fChirho) where\nimport ConsumerChirho\nimport Data.Kind (Type)\ndata SecretChirho = SecretChirho\nfChirho :: Int -> Int\nfChirho _ = 0\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (fChirho) where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\nfChirho :: SecretChirho Int -> Int\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &["boot contract", "SecretChirho", "kind"],
    );
}

#[test]
fn d1_private_class_absent_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nclass CChirho aChirho\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &[],
    );
}

#[test]
fn d2_exported_class_absent_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (CChirho) where\nclass CChirho aChirho\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &["boot contract", "export"],
    );
}

#[test]
fn e1_private_value_absent_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\n",
        "module ProviderChirho () where\ngChirho :: Int\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &[],
    );
}

#[test]
fn e2_exported_value_absent_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\n",
        "module ProviderChirho (gChirho) where\ngChirho :: Int\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &["boot contract", "export"],
    );
}

#[test]
fn f1_consumer_uses_private_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nxChirho :: SecretChirho Int\nxChirho = undefined\nvalueChirho = ()\n",
        &["not in scope", "SecretChirho"],
    );
}

#[test]
fn f2_consumer_uses_exported_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (SecretChirho) where\nimport ConsumerChirho\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type) = SecretChirho aChirho\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho (SecretChirho) where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nxChirho :: SecretChirho Int\nxChirho = undefined\nvalueChirho = ()\n",
        &[],
    );
}

#[test]
fn g1_private_but_illformed_chirho() {
    check_reference_chirho(
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\nimport ConsumerChirho\nimport Data.Kind (Type)\n",
        "{-# LANGUAGE KindSignatures #-}\nmodule ProviderChirho () where\ndata SecretChirho (aChirho :: NoSuchKindChirho)\n",
        "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
        &["not in scope", "NoSuchKindChirho"],
    );
}
