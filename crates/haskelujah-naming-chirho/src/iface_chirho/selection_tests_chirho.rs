// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The selected provider controls every imported view; no spelling creates exposure proof.
use super::*;
use haskelujah_ast_chirho::module_chirho::ImportDeclChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

fn import_chirho(package_chirho: Option<&str>) -> ImportDeclChirho {
    ImportDeclChirho {
        package_chirho: package_chirho.map(str::to_owned),
        module_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "Data.Proxy",
            SpanChirho::DUMMY_CHIRHO,
        )),
        qualified_chirho: false,
        alias_chirho: None,
        spec_chirho: None,
        source_chirho: false,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn iface_chirho(exposure_chirho: Option<IfaceExposureChirho>) -> ModuleIfaceChirho {
    ModuleIfaceChirho {
        exposure_chirho,
        name_chirho: "Data.Proxy".into(),
        exports_chirho: IfaceExportsChirho::default(),
    }
}

#[test]
fn selected_home_and_package_survive_merging_in_both_orders_chirho() {
    let home_chirho = iface_chirho(Some(IfaceExposureChirho::HomeChirho));
    let package_chirho = builtin_module_ifaces_chirho()
        .into_iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Proxy")
        .expect("positive builtin producer");
    for available_chirho in [
        vec![home_chirho.clone(), package_chirho.clone()],
        vec![package_chirho.clone(), home_chirho.clone()],
    ] {
        let available_chirho = merge_module_ifaces_chirho(available_chirho);
        assert_eq!(available_chirho.len(), 2);
        for selector_chirho in [None, Some("this")] {
            let selected_chirho =
                select_import_iface_chirho(&available_chirho, &import_chirho(selector_chirho))
                    .expect("admitted home selected");
            assert_eq!(
                selected_chirho.exposure_chirho,
                Some(IfaceExposureChirho::HomeChirho)
            );
            assert!(selected_chirho.exports_chirho.types_chirho.is_empty());
        }
        let selected_chirho =
            select_import_iface_chirho(&available_chirho, &import_chirho(Some("base")))
                .expect("actual base provider selected");
        assert!(
            selected_chirho
                .exports_chirho
                .types_chirho
                .contains_key("Proxy")
        );
    }
}

#[test]
fn unknown_and_builtin_origin_cannot_supply_package_selection_chirho() {
    let mut unknown_chirho = iface_chirho(None);
    unknown_chirho.exports_chirho.origin_chirho = IfaceOriginChirho::BuiltinChirho;
    let available_chirho = [unknown_chirho];
    assert!(select_import_iface_chirho(&available_chirho, &import_chirho(None)).is_some());
    for selector_chirho in ["base", "this", "unknown-chirho"] {
        assert!(
            select_import_iface_chirho(&available_chirho, &import_chirho(Some(selector_chirho)))
                .is_none()
        );
    }
}

#[test]
fn ambiguous_positive_package_exposures_do_not_choose_an_order_chirho() {
    let first_chirho = iface_chirho(Some(IfaceExposureChirho::packages_from_producer_chirho(&[
        "base",
    ])));
    let second_chirho = iface_chirho(Some(IfaceExposureChirho::packages_from_producer_chirho(&[
        "base",
        "second-chirho",
    ])));
    for available_chirho in [
        vec![first_chirho.clone(), second_chirho.clone()],
        vec![second_chirho.clone(), first_chirho.clone()],
    ] {
        assert!(
            select_import_iface_chirho(&available_chirho, &import_chirho(Some("base"))).is_none()
        );
        assert!(
            select_import_iface_chirho(&available_chirho, &import_chirho(Some("second-chirho")))
                .is_some()
        );
    }
}

#[test]
fn actual_package_reexport_producer_retains_both_exposures_chirho() {
    let available_chirho = merge_module_ifaces_chirho(builtin_module_ifaces_chirho());
    let mut import_chirho = import_chirho(None);
    import_chirho.module_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        "GHC.Num.Natural",
        SpanChirho::DUMMY_CHIRHO,
    ));
    let mut selected_chirho = Vec::new();
    for package_chirho in ["base", "ghc-bignum"] {
        import_chirho.package_chirho = Some(package_chirho.into());
        selected_chirho.push(
            select_import_iface_chirho(&available_chirho, &import_chirho)
                .expect("actual exposed reexport producer"),
        );
    }
    assert!(std::ptr::eq(selected_chirho[0], selected_chirho[1]));
}
