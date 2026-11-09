// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! P1 standalone source authority (gpt #33306, #33308). The driver mints the default home unit
//! `main` only on its standalone routes and carries it with the exact class declarations of the
//! module being checked; the creation-time declaration scheme joins it to that class's
//! predicates, class record and instances. Every other route stays MissingUnit, and compiler
//! proofs are untouched. A defining-module account carries no unit, so no imported declaration
//! is carried, whatever the interface's exposure or container, and a current account that an
//! import also names is dropped (#33316, #33339).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;
use std::sync::Arc;

use haskelujah_ast_chirho::name_chirho::{
    ClassIdentityUnknownChirho, PredicateClassIdentityChirho, PredicateClassProofChirho,
    SourceUnitAuthorityChirho, SourceUnitIdentityChirho, TypeConstructorDeclarationChirho,
    TypeConstructorRoleChirho,
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

/// A home CollideChirho that imports the package's own CollideChirho by package and re-exports
/// its class, and a program that names that class through the home module.
const HOME_FORWARD_SOURCE_CHIRHO: &str = "{-# LANGUAGE PackageImports #-}\nmodule CollideChirho (ShapeChirho (..)) where\n\nimport \"collide-package-chirho\" CollideChirho (ShapeChirho (..))\n";
const FORWARD_USER_SOURCE_CHIRHO: &str = "module Main where\n\nimport CollideChirho\n\nprobeChirho :: ShapeChirho a => a -> Int\nprobeChirho _ = 1\n\nmain :: IO ()\nmain = pure ()\n";

/// A home relay that re-exports the package CollideChirho's class, and a checked CollideChirho
/// that declares its own class of that name and imports the relay qualified.
const PACKAGE_RELAY_SOURCE_CHIRHO: &str = "{-# LANGUAGE PackageImports #-}\nmodule RelayChirho (ShapeChirho (..)) where\n\nimport \"collide-package-chirho\" CollideChirho (ShapeChirho (..))\n";
const COLLIDING_LOCAL_SOURCE_CHIRHO: &str = "module CollideChirho where\n\nimport qualified RelayChirho as ForeignChirho\n\nclass ShapeChirho a where\n  homeSidesChirho :: a -> Int\n\nlocalChirho :: ShapeChirho a => a -> Int\nlocalChirho _ = 1\n\nrelayedChirho :: ForeignChirho.ShapeChirho a => a -> Int\nrelayedChirho _ = 2\n\nnumericChirho :: Num a => a -> a\nnumericChirho valueChirho = valueChirho + 1\n";

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

/// The proof on a signed binding's only predicate, whatever spelling that predicate keeps.
fn sole_signature_proof_chirho(
    result_chirho: &FrontendResultChirho,
    binding_chirho: &str,
) -> PredicateClassProofChirho {
    let scheme_chirho = result_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho(binding_chirho)
        .expect("signed binding retained");
    assert_eq!(
        scheme_chirho.preds_chirho.len(),
        1,
        "{binding_chirho} keeps one predicate"
    );
    scheme_chirho.preds_chirho[0]
        .class_identity_chirho()
        .clone()
}

fn class_record_proof_chirho(
    result_chirho: &FrontendResultChirho,
    class_chirho: &str,
) -> PredicateClassProofChirho {
    result_chirho
        .infer_result_chirho
        .class_env_chirho
        .classes_chirho()
        .get_chirho(class_chirho)
        .expect("class registered")
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

/// The authority the standalone route mints for a checked module over `ifaces_chirho`, from the
/// same type scope the pipeline builds for it.
fn authority_chirho(
    result_chirho: &FrontendResultChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> Option<SourceUnitAuthorityChirho> {
    let type_scope_chirho = haskelujah_naming_chirho::build_type_constructor_scope_chirho(
        &result_chirho.module_chirho,
        ifaces_chirho,
    );
    crate::pipeline_chirho::standalone_home_authority_chirho(
        &result_chirho.module_chirho,
        ifaces_chirho,
        &type_scope_chirho,
    )
}

/// The authority that carries exactly the named `(module, class)` declarations in unit `main`.
fn carried_chirho(classes_chirho: &[(&str, &str)]) -> Option<SourceUnitAuthorityChirho> {
    Some(SourceUnitAuthorityChirho::from_driver_chirho(
        SourceUnitIdentityChirho::from_producer_chirho(Arc::from("main"))
            .expect("a non-empty unit key"),
        classes_chirho.iter().map(|&(module_chirho, class_chirho)| {
            TypeConstructorDeclarationChirho::source_chirho(
                module_chirho,
                class_chirho,
                TypeConstructorRoleChirho::ClassChirho,
            )
        }),
        [],
    ))
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

/// The package `collide-package-chirho`'s CollideChirho, checked from source and exposed under
/// that package selector.
fn packaged_collide_chirho(
    ifaces_chirho: &[ModuleIfaceChirho],
) -> (FrontendResultChirho, ModuleIfaceChirho) {
    let (checked_chirho, mut iface_chirho) =
        checked_iface_chirho(PACKAGE_COLLIDE_SOURCE_CHIRHO, ifaces_chirho);
    iface_chirho.exposure_chirho = Some(IfaceExposureChirho::packages_from_producer_chirho(&[
        "collide-package-chirho",
    ]));
    (checked_chirho, iface_chirho)
}

/// The standalone route over `ifaces_chirho`, with the declaration schemes `families_chirho`
/// transports and no imported value types, synonyms or classes.
fn standalone_frontend_chirho(
    source_chirho: &str,
    ifaces_chirho: &[ModuleIfaceChirho],
    families_chirho: &haskelujah_typing_chirho::infer_chirho::TypeFamilyEnvChirho,
) -> FrontendResultChirho {
    crate::run_standalone_frontend_chirho(
        source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        crate::FrontendImportsChirho {
            ifaces_chirho,
            types_chirho: &HashMap::new(),
            type_synonyms_chirho: &crate::ImportedTypeSynonymsChirho::new(),
            type_families_chirho: families_chirho,
            class_env_chirho: &haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho(),
        },
        &crate::FrontendOptionsChirho::default(),
    )
    .expect("the module checks on the standalone route")
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
    assert_eq!(
        class_record_proof_chirho(&result_chirho, "ShapeChirho"),
        expected_chirho
    );
    let instances_chirho = result_chirho
        .infer_result_chirho
        .class_env_chirho
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
fn only_the_checked_modules_own_class_declarations_join_the_default_home_unit_chirho() {
    // #33339: no interface makes a member, whatever its exposure or container. A home sibling, a
    // home relay that re-exports the sibling's class, a home module that a package also hides
    // and the package module exposing it all sit beside the program; only the program's own
    // class is carried.
    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
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
    assert_eq!(
        authority_chirho(&main_chirho, &ifaces_chirho),
        carried_chirho(&[("Main", "ShapeChirho")])
    );
}

#[test]
fn a_home_sibling_class_stays_missing_unit_until_its_unit_travels_chirho() {
    // #33339: 7f0c9ae7 and 08e0943c expected Source(main, SiblingChirho, ShapeChirho) here, an
    // expectation no run ever measured. Home admission sets an interface's exposure; it does not
    // prove which unit defines the class, and the account carries no unit. The sibling's class
    // stays MissingUnit until its defining unit travels, while the program's authority is still
    // minted, carrying nothing.
    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let (sibling_chirho, sibling_iface_chirho) =
        checked_iface_chirho(SIBLING_SOURCE_CHIRHO, &builtin_ifaces_chirho);
    let mut home_ifaces_chirho = builtin_ifaces_chirho;
    crate::module_search_chirho::admit_home_iface_chirho(
        &mut home_ifaces_chirho,
        sibling_iface_chirho,
    );
    let user_chirho = standalone_frontend_chirho(
        SIBLING_USER_SOURCE_CHIRHO,
        &home_ifaces_chirho,
        &sibling_chirho.infer_result_chirho.type_families_chirho,
    );
    assert_eq!(
        signature_proof_chirho(&user_chirho, "probeChirho", "ShapeChirho"),
        missing_unit_chirho()
    );
    assert_eq!(
        authority_chirho(&user_chirho, &home_ifaces_chirho),
        carried_chirho(&[])
    );
}

#[test]
fn a_package_module_sharing_a_home_module_name_stays_missing_unit_chirho() {
    // #33316: GHC names an entity by package, module and name, so the home CollideChirho and a
    // package's CollideChirho coexist, here declaring the same class name. The package class the
    // program imports by package name carries the very account a home class would, and no unit
    // tells them apart. Only the program's own class declarations are carried (#33339).
    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let (_, home_iface_chirho) =
        checked_iface_chirho(HOME_COLLIDE_SOURCE_CHIRHO, &builtin_ifaces_chirho);
    let (package_chirho, package_iface_chirho) = packaged_collide_chirho(&builtin_ifaces_chirho);
    let mut ifaces_chirho = builtin_ifaces_chirho;
    crate::module_search_chirho::admit_home_iface_chirho(&mut ifaces_chirho, home_iface_chirho);
    ifaces_chirho.push(package_iface_chirho);

    let user_chirho = standalone_frontend_chirho(
        PACKAGE_USER_SOURCE_CHIRHO,
        &ifaces_chirho,
        &package_chirho.infer_result_chirho.type_families_chirho,
    );
    assert_eq!(
        signature_proof_chirho(&user_chirho, "probeChirho", "ShapeChirho"),
        missing_unit_chirho()
    );
    assert_eq!(
        authority_chirho(&user_chirho, &ifaces_chirho),
        carried_chirho(&[])
    );
}

#[test]
fn a_home_module_forwarding_a_same_named_package_class_stays_missing_unit_chirho() {
    // #33339, #33340: a home CollideChirho imports the package's own CollideChirho by package and
    // re-exports its class. The program's view then holds ONLY the home module: the package
    // interface that produced the class is absent, so nothing visible marks the account as
    // another unit's, and the home module's admission must not make it the program's.
    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let (package_chirho, package_iface_chirho) = packaged_collide_chirho(&builtin_ifaces_chirho);
    let mut package_view_chirho = builtin_ifaces_chirho.clone();
    package_view_chirho.push(package_iface_chirho);
    let (_, forward_iface_chirho) =
        checked_iface_chirho(HOME_FORWARD_SOURCE_CHIRHO, &package_view_chirho);
    let mut ifaces_chirho = builtin_ifaces_chirho;
    crate::module_search_chirho::admit_home_iface_chirho(&mut ifaces_chirho, forward_iface_chirho);

    let user_chirho = standalone_frontend_chirho(
        FORWARD_USER_SOURCE_CHIRHO,
        &ifaces_chirho,
        &package_chirho.infer_result_chirho.type_families_chirho,
    );
    assert_eq!(
        signature_proof_chirho(&user_chirho, "probeChirho", "ShapeChirho"),
        missing_unit_chirho()
    );
    assert_eq!(
        authority_chirho(&user_chirho, &ifaces_chirho),
        carried_chirho(&[])
    );
}

#[test]
fn a_current_class_sharing_a_relayed_package_account_is_never_proven_chirho() {
    // #33339, #33340: the checked CollideChirho declares its own ShapeChirho and imports,
    // qualified, a home relay that re-exports the package CollideChirho's ShapeChirho; the
    // package interface itself is absent from the view. Both accounts are the same key, so the
    // current declaration is dropped and neither the local class nor the relayed one is proven,
    // while a compiler class beside them keeps its compiler proof.
    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let (_, package_iface_chirho) = packaged_collide_chirho(&builtin_ifaces_chirho);
    let mut package_view_chirho = builtin_ifaces_chirho.clone();
    package_view_chirho.push(package_iface_chirho);
    let (_, relay_iface_chirho) =
        checked_iface_chirho(PACKAGE_RELAY_SOURCE_CHIRHO, &package_view_chirho);
    let mut ifaces_chirho = builtin_ifaces_chirho;
    crate::module_search_chirho::admit_home_iface_chirho(&mut ifaces_chirho, relay_iface_chirho);
    let families_chirho = crate::seed_builtin_type_families_chirho();

    // Without the relay import, the same module's own class is carried and proven.
    let alone_chirho =
        standalone_frontend_chirho(HOME_COLLIDE_SOURCE_CHIRHO, &ifaces_chirho, &families_chirho);
    assert_eq!(
        authority_chirho(&alone_chirho, &ifaces_chirho),
        carried_chirho(&[("CollideChirho", "ShapeChirho")])
    );
    assert_eq!(
        class_record_proof_chirho(&alone_chirho, "ShapeChirho"),
        standalone_proof_chirho("CollideChirho", "ShapeChirho")
    );

    let current_chirho = standalone_frontend_chirho(
        COLLIDING_LOCAL_SOURCE_CHIRHO,
        &ifaces_chirho,
        &families_chirho,
    );
    assert_eq!(
        authority_chirho(&current_chirho, &ifaces_chirho),
        carried_chirho(&[])
    );
    assert_eq!(
        sole_signature_proof_chirho(&current_chirho, "localChirho"),
        missing_unit_chirho()
    );
    assert_eq!(
        sole_signature_proof_chirho(&current_chirho, "relayedChirho"),
        missing_unit_chirho()
    );
    assert_eq!(
        class_record_proof_chirho(&current_chirho, "ShapeChirho"),
        missing_unit_chirho()
    );
    assert!(matches!(
        signature_proof_chirho(&current_chirho, "numericChirho", "Num"),
        PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::CompilerChirho(_))
    ));
}
