// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! SCRATCH measurement probe for P1 consequence A (gpt #33363, #33367). It is not test-suite source and claims nothing.
//! Root mounts it in a scratch copy of haskelujah-driver beside the source_unit_authority module and runs only it, single
//! threaded, with output shown. It asserts only its own setup: the package and relay accounts come from their checked
//! sources through the real interface producer, every row reads one selected view, and each `alone` source is its product
//! source minus exactly the relay import, byte for byte the GHC twin. It then prints one CONSEQUENCE_A_ROW line per variant,
//! source and route. Every outcome is UNCLAIMED until that receipt exists.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::name_chirho::TypeConstructorIdentityChirho;
use haskelujah_naming_chirho::iface_chirho::{IfaceExposureChirho, ModuleIfaceChirho};
use haskelujah_span_chirho::FileIdChirho;

use crate::FrontendResultChirho;

/// The package `collide-package-chirho`: its CollideChirho declares the class the relay re-exports.
const PACKAGE_COLLIDE_SOURCE_CHIRHO: &str = "module CollideChirho where\n\nclass ShapeChirho aChirho where\n  sidesChirho :: aChirho -> Int\n";
/// The same package's RelayChirho, which re-exports that class through a package import.
const PACKAGE_RELAY_SOURCE_CHIRHO: &str = "{-# LANGUAGE PackageImports #-}\nmodule RelayChirho (ShapeChirho (..)) where\n\nimport \"collide-package-chirho\" CollideChirho (ShapeChirho (..))\n";
/// The one line each product source carries and each GHC twin drops.
const RELAY_IMPORT_CHIRHO: &str = "import qualified RelayChirho as ForeignChirho\n\n";
/// (variant, product source, GHC twin). A1 is the shape named in #33366: its probe's visible head has no instance. A2 keeps
/// the head and changes only the hidden kind, so only the prefix certificate can refuse it.
const VARIANTS_CHIRHO: [(&str, &str, &str); 2] = [
    (
        "a1-chirho",
        include_str!("a1-product-chirho.hs"),
        include_str!("../ghc-reference-chirho/ghc-twin-a1-chirho/CollideChirho.hs"),
    ),
    (
        "a2-chirho",
        include_str!("a2-product-chirho.hs"),
        include_str!("../ghc-reference-chirho/ghc-twin-a2-chirho/CollideChirho.hs"),
    ),
];

/// One package module checked by the library route and its interface from the real producer, under the package's exposure.
fn packaged_iface_chirho(
    source_chirho: &str,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> ModuleIfaceChirho {
    let checked_chirho = crate::run_frontend_chirho(
        source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        ifaces_chirho,
        &HashMap::new(),
    )
    .expect("the package module checks");
    let mut iface_chirho = haskelujah_naming_chirho::build_iface_with_imports_chirho(
        &checked_chirho.module_chirho,
        ifaces_chirho,
    )
    .expect("the package module has a complete interface");
    iface_chirho.exposure_chirho = Some(IfaceExposureChirho::packages_from_producer_chirho(&[
        "collide-package-chirho",
    ]));
    iface_chirho
}

/// An accepted module as data: the class record's proof, whether the probe binding exists, and what the standalone route
/// would carry for this module over the same view.
fn accepted_chirho(
    result_chirho: &FrontendResultChirho,
    view_chirho: &[ModuleIfaceChirho],
) -> String {
    let class_record_chirho = result_chirho
        .infer_result_chirho
        .class_env_chirho
        .classes_chirho()
        .get_chirho("ShapeChirho")
        .map(|class_chirho| class_chirho.class_identity_chirho().clone());
    let probe_bound_chirho = result_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho("probeChirho")
        .is_some();
    let scope_chirho = haskelujah_naming_chirho::build_type_constructor_scope_chirho(
        &result_chirho.module_chirho,
        view_chirho,
    );
    let authority_chirho = crate::pipeline_chirho::standalone_home_authority_chirho(
        &result_chirho.module_chirho,
        view_chirho,
        &scope_chirho,
    );
    format!(
        "ACCEPT class_record={class_record_chirho:?} probe_bound={probe_bound_chirho} standalone_authority_over_view={authority_chirho:?}"
    )
}

/// A route's outcome as one line: ACCEPT as above, or REJECT with every rendered diagnostic.
macro_rules! described_chirho {
    ($result_chirho:expr, $view_chirho:expr) => {
        match $result_chirho {
            Ok(result_chirho) => accepted_chirho(&result_chirho, $view_chirho),
            Err(diagnostics_chirho) => format!(
                "REJECT {:?}",
                diagnostics_chirho
                    .diagnostics_chirho()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            ),
        }
    };
}

#[test]
fn consequence_a_paired_route_measurement_chirho() {
    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let package_chirho =
        packaged_iface_chirho(PACKAGE_COLLIDE_SOURCE_CHIRHO, &builtin_ifaces_chirho);
    let mut package_view_chirho = builtin_ifaces_chirho.clone();
    package_view_chirho.push(package_chirho);
    let relay_chirho = packaged_iface_chirho(PACKAGE_RELAY_SOURCE_CHIRHO, &package_view_chirho);
    // Setup only: the relay re-exports the package's own CollideChirho.ShapeChirho account, as its producer recorded it.
    let relayed_chirho = relay_chirho
        .exports_chirho
        .types_chirho
        .get("ShapeChirho")
        .and_then(|type_chirho| type_chirho.declaration_chirho.as_ref())
        .map(|declaration_chirho| declaration_chirho.identity_chirho.clone());
    assert!(
        matches!(
            &relayed_chirho,
            Some(TypeConstructorIdentityChirho::SourceChirho { defining_module_chirho, name_chirho })
                if defining_module_chirho == "CollideChirho" && name_chirho == "ShapeChirho"
        ),
        "{relayed_chirho:?}"
    );
    // One selected view for every row: the builtins and the package's relay; the origin interface is absent.
    let mut view_chirho = builtin_ifaces_chirho;
    view_chirho.push(relay_chirho);
    let families_chirho = crate::seed_builtin_type_families_chirho();
    for (variant_chirho, product_chirho, twin_chirho) in VARIANTS_CHIRHO {
        assert_eq!(
            product_chirho.matches(RELAY_IMPORT_CHIRHO).count(),
            1,
            "{variant_chirho}"
        );
        let alone_chirho = product_chirho.replacen(RELAY_IMPORT_CHIRHO, "", 1);
        assert_eq!(
            alone_chirho, twin_chirho,
            "{variant_chirho}: the twin is the product minus the relay import"
        );
        for (source_name_chirho, source_chirho) in [
            ("colliding", product_chirho),
            ("alone", alone_chirho.as_str()),
        ] {
            let library_chirho = described_chirho!(
                crate::run_frontend_chirho(
                    source_chirho,
                    FileIdChirho::SYNTHETIC_CHIRHO,
                    &view_chirho,
                    &HashMap::new(),
                ),
                &view_chirho
            );
            let standalone_chirho = described_chirho!(
                crate::run_standalone_frontend_chirho(
                    source_chirho,
                    FileIdChirho::SYNTHETIC_CHIRHO,
                    crate::FrontendImportsChirho {
                        ifaces_chirho: &view_chirho,
                        types_chirho: &HashMap::new(),
                        type_synonyms_chirho: &crate::ImportedTypeSynonymsChirho::new(),
                        type_families_chirho: &families_chirho,
                        class_env_chirho:
                            &haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho(),
                    },
                    &crate::FrontendOptionsChirho::default(),
                ),
                &view_chirho
            );
            println!(
                "CONSEQUENCE_A_ROW {variant_chirho} {source_name_chirho} library {library_chirho}"
            );
            println!(
                "CONSEQUENCE_A_ROW {variant_chirho} {source_name_chirho} standalone {standalone_chirho}"
            );
        }
    }
}
