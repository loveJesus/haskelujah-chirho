// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Selection metadata reaches the existing provenance routes without changing their authority.
use super::*;
use haskelujah_naming_chirho::iface_chirho::{
    IfaceExportsChirho, IfaceExposureChirho, IfaceValueAvailabilityChirho,
};
use haskelujah_span_chirho::{FileIdChirho, SpanChirho};

fn module_chirho(package_chirho: &str) -> ModuleChirho {
    let source_chirho =
        format!("module CallerChirho where\nimport \"{package_chirho}\" SharedChirho\n");
    haskelujah_parser_chirho::lower_chirho::lower_module_chirho(
        &haskelujah_parser_chirho::cst_parser_chirho::ParserChirho::new_chirho(
            &source_chirho,
            FileIdChirho::SYNTHETIC_CHIRHO,
        )
        .parse_chirho(),
        FileIdChirho::SYNTHETIC_CHIRHO,
    )
}
fn ifaces_chirho() -> Vec<ModuleIfaceChirho> {
    [
        (
            IfaceOriginChirho::BuiltinChirho,
            IfaceExposureChirho::packages_from_producer_chirho(&["provider-chirho"]),
        ),
        (
            IfaceOriginChirho::SourceChirho,
            IfaceExposureChirho::HomeChirho,
        ),
    ]
    .into_iter()
    .map(|(origin_chirho, exposure_chirho)| {
        let mut exports_chirho = IfaceExportsChirho {
            origin_chirho,
            ..IfaceExportsChirho::default()
        };
        exports_chirho.values_chirho.insert(
            "routeChirho".into(),
            IfaceValueChirho {
                availability_chirho: IfaceValueAvailabilityChirho::AvailableChirho,
                name_chirho: "routeChirho".into(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                defining_module_chirho: (origin_chirho == IfaceOriginChirho::SourceChirho)
                    .then(|| "SharedChirho".into()),
            },
        );
        ModuleIfaceChirho {
            name_chirho: "SharedChirho".into(),
            exposure_chirho: Some(exposure_chirho),
            exports_chirho,
        }
    })
    .collect()
}
#[test]
fn package_selection_preserves_the_selected_value_origin_chirho() {
    let ifaces_chirho = ifaces_chirho();
    for (package_chirho, source_chirho) in [("this", true), ("provider-chirho", false)] {
        let (actual_source_chirho, builtin_chirho) = imported_names_by_origin_chirho(
            &module_chirho(package_chirho),
            &ifaces_chirho,
            &HashSet::new(),
        );
        assert_eq!(actual_source_chirho.contains("routeChirho"), source_chirho);
        assert_eq!(builtin_chirho.contains("routeChirho"), !source_chirho);
    }
}
#[test]
fn package_selection_cannot_acquire_a_home_program_supplier_chirho() {
    let ifaces_chirho = ifaces_chirho();
    let program_chirho = HashSet::from(["SharedChirho".into()]);
    let package_chirho = import_suppliers_chirho(
        &module_chirho("provider-chirho"),
        &ifaces_chirho,
        &program_chirho,
    );
    assert!(
        package_chirho
            .reference_chirho(None, "routeChirho")
            .is_none()
    );
    let home_chirho =
        import_suppliers_chirho(&module_chirho("this"), &ifaces_chirho, &program_chirho);
    assert_eq!(
        home_chirho
            .reference_chirho(None, "routeChirho")
            .expect("actual home supplier")
            .defining_modules_chirho,
        ["SharedChirho"]
    );
}
