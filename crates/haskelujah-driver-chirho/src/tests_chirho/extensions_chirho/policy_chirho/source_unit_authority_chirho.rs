// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! P1 standalone source authority (gpt #33306, #33308). The driver mints the default home unit
//! `main` only on its standalone routes, and the creation-time declaration scheme joins it to a
//! source class's predicates, class record and instances. Every other route stays MissingUnit,
//! membership comes only from checked home admission, and compiler proofs are untouched.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;
use std::sync::Arc;

use haskelujah_ast_chirho::name_chirho::{
    ClassIdentityUnknownChirho, PredicateClassIdentityChirho, PredicateClassProofChirho,
    SourceUnitIdentityChirho,
};
use haskelujah_naming_chirho::iface_chirho::IfaceExposureChirho;
use haskelujah_span_chirho::{FileIdChirho, SourceMapChirho};

use crate::FrontendResultChirho;

/// A local source class with a method, an instance and a signed use, beside a compiler class.
const LOCAL_CLASS_SOURCE_CHIRHO: &str = "module Main where\n\nclass ShapeChirho a where\n  sidesChirho :: a -> Int\n\ninstance ShapeChirho Bool where\n  sidesChirho _ = 2\n\nprobeChirho :: ShapeChirho a => a -> Int\nprobeChirho valueChirho = sidesChirho valueChirho\n\nnumericChirho :: Num a => a -> a\nnumericChirho valueChirho = valueChirho + 1\n\nmain :: IO ()\nmain = print (probeChirho True)\n";

/// A sibling module that declares its own source class.
const SIBLING_SOURCE_CHIRHO: &str =
    "module SiblingChirho where\n\nclass ShapeChirho a where\n  sidesChirho :: a -> Int\n";

/// A program whose signature names the sibling's class.
const SIBLING_USER_SOURCE_CHIRHO: &str = "module Main where\n\nimport SiblingChirho\n\nprobeChirho :: ShapeChirho a => a -> Int\nprobeChirho _ = 1\n\nmain :: IO ()\nmain = pure ()\n";

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
