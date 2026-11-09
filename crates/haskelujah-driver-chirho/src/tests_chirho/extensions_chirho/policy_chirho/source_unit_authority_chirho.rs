// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! P1 standalone source authority (gpt #33306, #33308). The driver mints the default home unit
//! `main` only on its standalone routes, and the creation-time declaration scheme joins it to a
//! source class's predicates, class record and instances. Every other route stays MissingUnit,
//! membership comes only from checked home admission, and compiler proofs are untouched. A home
//! module name that another unit could also define is withheld from the home unit (#33316).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;
use std::sync::Arc;

use haskelujah_ast_chirho::name_chirho::{
    ClassIdentityUnknownChirho, PredicateClassIdentityChirho, PredicateClassProofChirho,
    SourceUnitIdentityChirho,
};
use haskelujah_naming_chirho::iface_chirho::{IfaceExposureChirho, ModuleIfaceChirho};
use haskelujah_span_chirho::{FileIdChirho, SourceMapChirho};

use crate::FrontendResultChirho;

/// A local source class with a method, an instance and a signed use, beside a compiler class.
const LOCAL_CLASS_SOURCE_CHIRHO: &str = "module Main where\n\nclass ShapeChirho a where\n  sidesChirho :: a -> Int\n\ninstance ShapeChirho Bool where\n  sidesChirho _ = 2\n\nprobeChirho :: ShapeChirho a => a -> Int\nprobeChirho valueChirho = sidesChirho valueChirho\n\nnumericChirho :: Num a => a -> a\nnumericChirho valueChirho = valueChirho + 1\n\nmain :: IO ()\nmain = print (probeChirho True)\n";

/// A sibling module that declares its own source class.
const SIBLING_SOURCE_CHIRHO: &str =
    "module SiblingChirho where\n\nclass ShapeChirho a where\n  sidesChirho :: a -> Int\n";

/// A program whose signature names the sibling's class.
const SIBLING_USER_SOURCE_CHIRHO: &str = "module Main where\n\nimport SiblingChirho\n\nprobeChirho :: ShapeChirho a => a -> Int\nprobeChirho _ = 1\n\nmain :: IO ()\nmain = pure ()\n";

/// A home module and a package module that share the defining-module name and the class name.
const HOME_COLLIDE_SOURCE_CHIRHO: &str =
    "module CollideChirho where\n\nclass ShapeChirho a where\n  homeSidesChirho :: a -> Int\n";
const PACKAGE_COLLIDE_SOURCE_CHIRHO: &str =
    "module CollideChirho where\n\nclass ShapeChirho a where\n  sidesChirho :: a -> Int\n";

/// A program whose signature names the class of the package's module, imported by package.
const PACKAGE_USER_SOURCE_CHIRHO: &str = "{-# LANGUAGE PackageImports #-}\nmodule Main where\n\nimport \"collide-package-chirho\" CollideChirho\n\nprobeChirho :: ShapeChirho a => a -> Int\nprobeChirho _ = 1\n\nmain :: IO ()\nmain = pure ()\n";

/// A module that a package hides and the home unit also defines, a package's exposed module
/// that re-exports its class, and a home module that re-exports a home sibling's class.
const HIDDEN_SOURCE_CHIRHO: &str =
    "module HiddenChirho where\n\nclass ShapeChirho a where\n  sidesChirho :: a -> Int\n";
const EXPOSED_SOURCE_CHIRHO: &str =
    "module ExposedChirho (ShapeChirho (..)) where\n\nimport HiddenChirho\n";
const RELAY_SOURCE_CHIRHO: &str =
    "module RelayChirho (ShapeChirho (..)) where\n\nimport SiblingChirho\n";

fn signature_proof_chirho(
    result_chirho: &FrontendResultChirho,
    binding_chirho: &str,
    class_chirho: &str,
) -> PredicateClassProofChirho {
    result_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho(binding_chirho)
        .expect("signed binding retained")
        .preds_chirho
        .iter()
        .find(|predicate_chirho| predicate_chirho.class_name_chirho == class_chirho)
        .expect("signature predicate retained")
        .class_identity_chirho()
        .clone()
}

fn standalone_proof_chirho(module_chirho: &str, class_chirho: &str) -> PredicateClassProofChirho {
    PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::SourceChirho {
        unit_chirho: SourceUnitIdentityChirho::from_producer_chirho(Arc::from("main"))
            .expect("a non-empty unit key"),
        module_chirho: Arc::from(module_chirho),
        name_chirho: Arc::from(class_chirho),
    })
}

fn missing_unit_chirho() -> PredicateClassProofChirho {
    PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho)
}

fn library_frontend_chirho(source_chirho: &str) -> FrontendResultChirho {
    crate::run_frontend_chirho(
        source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &HashMap::new(),
    )
    .expect("the library route checks the program")
}

/// The library check of one module and the interface its producer builds over `ifaces_chirho`.
fn checked_iface_chirho(
    source_chirho: &str,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> (FrontendResultChirho, ModuleIfaceChirho) {
    let checked_chirho = crate::run_frontend_chirho(
        source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        ifaces_chirho,
        &HashMap::new(),
    )
    .expect("the module checks");
    let iface_chirho = haskelujah_naming_chirho::build_iface_with_imports_chirho(
        &checked_chirho.module_chirho,
        ifaces_chirho,
    )
    .expect("the module has a complete interface");
    (checked_chirho, iface_chirho)
}

#[test]
fn the_public_single_source_route_proves_a_local_class_in_the_default_home_unit_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = crate::typecheck_source_chirho(
        LOCAL_CLASS_SOURCE_CHIRHO,
        &mut source_map_chirho,
        "Main.hs",
    )
    .expect("the local class program checks");
    let expected_chirho = standalone_proof_chirho("Main", "ShapeChirho");
    assert_eq!(
        signature_proof_chirho(&result_chirho, "probeChirho", "ShapeChirho"),
        expected_chirho
    );
    // The class record and its instance take the same creation-time declaration proof.
    let class_env_chirho = &result_chirho.infer_result_chirho.class_env_chirho;
    let class_chirho = class_env_chirho
        .classes_chirho()
        .get_chirho("ShapeChirho")
        .expect("class registered");
    assert_eq!(class_chirho.class_identity_chirho(), &expected_chirho);
    let instances_chirho = class_env_chirho
        .instances_chirho()
        .get_chirho("ShapeChirho")
        .expect("instance list registered");
    let instance_chirho = instances_chirho
        .iter_chirho()
        .next()
        .expect("instance registered");
    assert_eq!(instance_chirho.class_identity_chirho(), &expected_chirho);
    // A compiler class beside it keeps its compiler proof.
    assert!(matches!(
        signature_proof_chirho(&result_chirho, "numericChirho", "Num"),
        PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::CompilerChirho(_))
    ));
}

#[test]
fn the_raw_library_and_project_entries_keep_a_local_class_missing_unit_chirho() {
    let library_chirho = library_frontend_chirho(LOCAL_CLASS_SOURCE_CHIRHO);
    assert_eq!(
        signature_proof_chirho(&library_chirho, "probeChirho", "ShapeChirho"),
        missing_unit_chirho()
    );
    // The project route's own entry (dependency_packages_chirho) never sets the standalone home.
    let project_chirho = crate::run_frontend_with_type_synonyms_families_and_class_env_chirho(
        LOCAL_CLASS_SOURCE_CHIRHO,
        FileIdChirho::SYNTHETIC_CHIRHO,
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &HashMap::new(),
        &crate::ImportedTypeSynonymsChirho::new(),
        &crate::seed_builtin_type_families_chirho(),
        &haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho(),
    )
    .expect("the project entry checks the program");
    assert_eq!(
        signature_proof_chirho(&project_chirho, "probeChirho", "ShapeChirho"),
        missing_unit_chirho()
    );
}

#[test]
fn only_checked_home_admission_joins_the_default_home_unit_chirho() {
    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let sibling_chirho = library_frontend_chirho(SIBLING_SOURCE_CHIRHO);
    let sibling_iface_chirho = haskelujah_naming_chirho::build_iface_with_imports_chirho(
        &sibling_chirho.module_chirho,
        &builtin_ifaces_chirho,
    )
    .expect("the sibling has a complete interface");
    let main_chirho = library_frontend_chirho(LOCAL_CLASS_SOURCE_CHIRHO);

    let mut home_ifaces_chirho = builtin_ifaces_chirho.clone();
    crate::module_search_chirho::admit_home_iface_chirho(
        &mut home_ifaces_chirho,
        sibling_iface_chirho.clone(),
    );
    let home_chirho = crate::pipeline_chirho::standalone_home_authority_chirho(
        &main_chirho.module_chirho,
        &home_ifaces_chirho,
    )
    .expect("the driver mints the default home unit");
    assert_eq!(home_chirho.unit_chirho().as_str_chirho(), "main");
    assert!(home_chirho.admits_module_chirho("Main"));
    assert!(home_chirho.admits_module_chirho("SiblingChirho"));
    assert!(!home_chirho.admits_module_chirho("Prelude"));

    // The same interface under a package exposure is never a member.
    let mut packaged_iface_chirho = sibling_iface_chirho;
    packaged_iface_chirho.exposure_chirho =
        Some(IfaceExposureChirho::packages_from_producer_chirho(&[
            "sibling-package-chirho",
        ]));
    let mut package_ifaces_chirho = builtin_ifaces_chirho;
    package_ifaces_chirho.push(packaged_iface_chirho);
    let package_chirho = crate::pipeline_chirho::standalone_home_authority_chirho(
        &main_chirho.module_chirho,
        &package_ifaces_chirho,
    )
    .expect("the driver mints the default home unit");
    assert!(package_chirho.admits_module_chirho("Main"));
    assert!(!package_chirho.admits_module_chirho("SiblingChirho"));
}

#[test]
fn a_home_sibling_class_is_proven_in_the_default_home_unit_chirho() {
    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let sibling_chirho = library_frontend_chirho(SIBLING_SOURCE_CHIRHO);
    let sibling_iface_chirho = haskelujah_naming_chirho::build_iface_with_imports_chirho(
        &sibling_chirho.module_chirho,
        &builtin_ifaces_chirho,
    )
    .expect("the sibling has a complete interface");
    let mut home_ifaces_chirho = builtin_ifaces_chirho;
    crate::module_search_chirho::admit_home_iface_chirho(
        &mut home_ifaces_chirho,
        sibling_iface_chirho,
    );
    let user_chirho = crate::run_standalone_frontend_chirho(
        SIBLING_USER_SOURCE_CHIRHO,
        FileIdChirho::SYNTHETIC_CHIRHO,
        crate::FrontendImportsChirho {
            ifaces_chirho: &home_ifaces_chirho,
            types_chirho: &HashMap::new(),
            type_synonyms_chirho: &crate::ImportedTypeSynonymsChirho::new(),
            type_families_chirho: &sibling_chirho.infer_result_chirho.type_families_chirho,
            class_env_chirho: &haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho(),
        },
        &crate::FrontendOptionsChirho::default(),
    )
    .expect("the sibling user checks on the standalone route");
    assert_eq!(
        signature_proof_chirho(&user_chirho, "probeChirho", "ShapeChirho"),
        standalone_proof_chirho("SiblingChirho", "ShapeChirho")
    );
}

#[test]
fn a_package_module_sharing_a_home_module_name_stays_missing_unit_chirho() {
    // #33316: GHC names an entity by package, module and name, so the home CollideChirho and a
    // package's CollideChirho coexist, here declaring the same class name. The package class the
    // program imports by package name carries the very account a home class would, and no unit
    // tells them apart, so the shared module name is withheld from the home unit.
    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let (_, home_iface_chirho) =
        checked_iface_chirho(HOME_COLLIDE_SOURCE_CHIRHO, &builtin_ifaces_chirho);
    let (package_chirho, mut package_iface_chirho) =
        checked_iface_chirho(PACKAGE_COLLIDE_SOURCE_CHIRHO, &builtin_ifaces_chirho);
    package_iface_chirho.exposure_chirho =
        Some(IfaceExposureChirho::packages_from_producer_chirho(&[
            "collide-package-chirho",
        ]));
    let mut ifaces_chirho = builtin_ifaces_chirho;
    crate::module_search_chirho::admit_home_iface_chirho(&mut ifaces_chirho, home_iface_chirho);
    ifaces_chirho.push(package_iface_chirho);

    let user_chirho = crate::run_standalone_frontend_chirho(
        PACKAGE_USER_SOURCE_CHIRHO,
        FileIdChirho::SYNTHETIC_CHIRHO,
        crate::FrontendImportsChirho {
            ifaces_chirho: &ifaces_chirho,
            types_chirho: &HashMap::new(),
            type_synonyms_chirho: &crate::ImportedTypeSynonymsChirho::new(),
            type_families_chirho: &package_chirho.infer_result_chirho.type_families_chirho,
            class_env_chirho: &haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho(),
        },
        &crate::FrontendOptionsChirho::default(),
    )
    .expect("the package user checks on the standalone route");
    assert_eq!(
        signature_proof_chirho(&user_chirho, "probeChirho", "ShapeChirho"),
        missing_unit_chirho()
    );
    let home_chirho = crate::pipeline_chirho::standalone_home_authority_chirho(
        &user_chirho.module_chirho,
        &ifaces_chirho,
    )
    .expect("the driver mints the default home unit");
    assert!(home_chirho.admits_module_chirho("Main"));
    assert!(!home_chirho.admits_module_chirho("CollideChirho"));
}

#[test]
fn a_hidden_or_re_exported_definer_is_withheld_from_the_home_unit_chirho() {
    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    // A package's exposed module re-exports the class of its hidden HiddenChirho, while the home
    // unit has its own HiddenChirho declaring the same class.
    let (_, hidden_iface_chirho) =
        checked_iface_chirho(HIDDEN_SOURCE_CHIRHO, &builtin_ifaces_chirho);
    let mut package_view_chirho = builtin_ifaces_chirho.clone();
    package_view_chirho.push(hidden_iface_chirho.clone());
    let (_, mut exposed_iface_chirho) =
        checked_iface_chirho(EXPOSED_SOURCE_CHIRHO, &package_view_chirho);
    exposed_iface_chirho.exposure_chirho =
        Some(IfaceExposureChirho::packages_from_producer_chirho(&[
            "hidden-package-chirho",
        ]));
    // A home relay re-exports the home sibling's class; no unit proves that re-export's owner.
    let (_, sibling_iface_chirho) =
        checked_iface_chirho(SIBLING_SOURCE_CHIRHO, &builtin_ifaces_chirho);
    let mut relay_view_chirho = builtin_ifaces_chirho.clone();
    crate::module_search_chirho::admit_home_iface_chirho(
        &mut relay_view_chirho,
        sibling_iface_chirho.clone(),
    );
    let (_, relay_iface_chirho) = checked_iface_chirho(RELAY_SOURCE_CHIRHO, &relay_view_chirho);

    let main_chirho = library_frontend_chirho(LOCAL_CLASS_SOURCE_CHIRHO);
    let mut ifaces_chirho = builtin_ifaces_chirho;
    for home_iface_chirho in [
        hidden_iface_chirho,
        sibling_iface_chirho,
        relay_iface_chirho,
    ] {
        crate::module_search_chirho::admit_home_iface_chirho(&mut ifaces_chirho, home_iface_chirho);
    }
    ifaces_chirho.push(exposed_iface_chirho);
    let home_chirho = crate::pipeline_chirho::standalone_home_authority_chirho(
        &main_chirho.module_chirho,
        &ifaces_chirho,
    )
    .expect("the driver mints the default home unit");
    // Direct, unambiguous home authority proceeds.
    assert!(home_chirho.admits_module_chirho("Main"));
    assert!(home_chirho.admits_module_chirho("RelayChirho"));
    // A hidden package definer and a re-exported definer stay out until their unit is carried.
    assert!(!home_chirho.admits_module_chirho("HiddenChirho"));
    assert!(!home_chirho.admits_module_chirho("SiblingChirho"));
}
