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

fn append_module_chirho(source_chirho: &str) -> ModuleChirho {
    haskelujah_parser_chirho::lower_chirho::lower_module_chirho(
        &haskelujah_parser_chirho::cst_parser_chirho::ParserChirho::new_chirho(
            source_chirho,
            FileIdChirho::SYNTHETIC_CHIRHO,
        )
        .parse_chirho(),
        FileIdChirho::SYNTHETIC_CHIRHO,
    )
}

#[test]
fn selected_export_origin_reaches_core_but_unknown_or_hidden_exports_do_not_chirho() {
    let prelude_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho()
        .into_iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Prelude")
        .unwrap();
    for certified_chirho in [false, true] {
        let mut forwarded_chirho = prelude_chirho.clone();
        forwarded_chirho.name_chirho = "ForwardAppendChirho".to_string();
        forwarded_chirho.exposure_chirho = Some(IfaceExposureChirho::HomeChirho);
        forwarded_chirho.exports_chirho.origin_chirho = IfaceOriginChirho::SourceChirho;
        if !certified_chirho {
            forwarded_chirho
                .exports_chirho
                .values_chirho
                .get_mut("++")
                .unwrap()
                .compiler_value_chirho = None;
        }
        for hidden_chirho in [false, true] {
            let hiding_chirho = if hidden_chirho { " hiding ((++))" } else { "" };
            let input_chirho = append_module_chirho(&format!(
                "module Main where\nimport qualified ForwardAppendChirho as F{hiding_chirho}\nvalueChirho = [\"a\"] F.++ [\"b\"]\n"
            ));
            let suppliers_chirho = import_suppliers_chirho(
                &input_chirho,
                &[forwarded_chirho.clone()],
                &HashSet::from(["ForwardAppendChirho".to_string()]),
            );
            let core_chirho = haskelujah_core_chirho::desugar_module_with_inputs_chirho(
                &input_chirho,
                haskelujah_core_chirho::DesugarInputsChirho {
                    import_suppliers_chirho: suppliers_chirho,
                    ..Default::default()
                },
            )
            .unwrap()
            .module_chirho;
            assert_eq!(
                core_chirho
                    .provenance_chirho
                    .compiler_value_references_chirho
                    .len(),
                usize::from(certified_chirho && !hidden_chirho),
            );
        }
    }
}

#[test]
fn authoritative_source_admission_replaces_only_its_actual_exposure_chirho() {
    let builtin_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho()
        .into_iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Prelude")
        .unwrap();
    let mut other_exposure_chirho = builtin_chirho.clone();
    other_exposure_chirho.exposure_chirho = Some(IfaceExposureChirho::HomeChirho);
    let mut replacement_chirho = builtin_chirho.clone();
    replacement_chirho.exports_chirho.origin_chirho = IfaceOriginChirho::SourceChirho;
    let value_chirho = replacement_chirho
        .exports_chirho
        .values_chirho
        .get_mut("++")
        .unwrap();
    value_chirho.compiler_value_chirho = None;
    value_chirho.defining_module_chirho = Some("Prelude".to_string());
    let mut ifaces_chirho = vec![builtin_chirho, other_exposure_chirho];
    crate::module_search_chirho::admit_authoritative_iface_chirho(
        &mut ifaces_chirho,
        replacement_chirho,
    );
    assert_eq!(ifaces_chirho.len(), 2);
    assert_eq!(
        ifaces_chirho[0].exports_chirho.values_chirho["++"].compiler_value_chirho,
        None
    );
    assert!(
        ifaces_chirho[1].exports_chirho.values_chirho["++"]
            .compiler_value_chirho
            .is_some()
    );
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
                compiler_value_chirho: None,
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
