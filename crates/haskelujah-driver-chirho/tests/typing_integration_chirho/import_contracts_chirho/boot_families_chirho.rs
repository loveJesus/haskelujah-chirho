// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Sources are byte-identical to source-boot-chirho/families-chirho/references-chirho.jsonl.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::boot_chirho::source_graph_paths_chirho;

fn compare_chirho(sources_chirho: &[(&str, &str)], root_chirho: &str, reason_chirho: Option<&str>) {
    for result_chirho in source_graph_paths_chirho(sources_chirho, root_chirho) {
        if let Some(reason_chirho) = reason_chirho {
            let error_chirho = result_chirho.expect_err(reason_chirho);
            assert!(error_chirho.contains(reason_chirho), "{error_chirho}");
        } else {
            result_chirho.unwrap();
        }
    }
}

#[test]
fn abstract_nonempty_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where ..\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        None,
    );
}

#[test]
fn abstract_empty_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where {}\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where ..\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        None,
    );
}

#[test]
fn empty_matching_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where {}\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where {}\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        None,
    );
}

#[test]
fn full_matching_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        None,
    );
}

#[test]
fn full_alpha_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho bChirho = bChirho\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        None,
    );
}

#[test]
fn empty_nonempty_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where {}\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        Some("boot contract"),
    );
}

#[test]
fn abstract_open_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where ..\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        Some("boot contract"),
    );
}

#[test]
fn open_closed_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        Some("boot contract"),
    );
}

#[test]
fn full_changed_result_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Char\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        Some("boot contract"),
    );
}

#[test]
fn full_missing_row_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        Some("boot contract"),
    );
}

#[test]
fn full_reordered_rows_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho aChirho = aChirho\n  FChirho Int = Bool\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        Some("boot contract"),
    );
}

#[test]
fn full_equation_readback_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nreadBackChirho :: FChirho Int -> Bool\nreadBackChirho xChirho = xChirho\n",
            ),
        ],
        "ConsumerChirho.hs",
        None,
    );
}

#[test]
fn abstract_equation_unavailable_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where ..\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nreadBackChirho :: FChirho Int -> Bool\nreadBackChirho xChirho = xChirho\n",
            ),
        ],
        "ConsumerChirho.hs",
        Some("E0200"),
    );
}

#[test]
fn abstract_injective_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) = (rChirho :: Type) | rChirho -> aChirho where\n  FChirho aChirho = [aChirho]\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) = (rChirho :: Type) | rChirho -> aChirho where ..\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        None,
    );
}

#[test]
fn abstract_injectivity_mismatch_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) = (rChirho :: Type) | rChirho -> aChirho where ..\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        Some("boot contract"),
    );
}

#[test]
fn abstract_invalid_implementation_chirho() {
    compare_chirho(
        &[
            (
                "ProviderChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where\n  FChirho aChirho = NoSuchTypeChirho\n",
            ),
            (
                "ProviderChirho.hs-boot",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where ..\n",
            ),
            (
                "ConsumerChirho.hs",
                "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n",
            ),
        ],
        "ConsumerChirho.hs",
        Some("NoSuchTypeChirho"),
    );
}

#[test]
fn abstract_outside_boot_chirho() {
    compare_chirho(
        &[(
            "ProviderChirho.hs",
            "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\nmodule ProviderChirho where\nimport Data.Kind (Type)\ntype family FChirho (aChirho :: Type) :: Type where ..\n",
        )],
        "ProviderChirho.hs",
        Some("abstract closed family"),
    );
}
