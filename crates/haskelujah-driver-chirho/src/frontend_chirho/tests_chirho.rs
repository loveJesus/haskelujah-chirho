// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Semantic closure and selective-import growth are module-boundary contracts.
use super::*;
use std::collections::HashMap;

fn checked_sources_chirho(
    sources_chirho: &[(&str, &str)],
) -> (
    FrontendResultChirho,
    Vec<ModuleIfaceChirho>,
    ImportedTypeContractsChirho,
) {
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let mut contracts_chirho = ImportedTypeContractsChirho::new();
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut last_chirho = None;
    for (name_chirho, source_chirho) in sources_chirho {
        let file_chirho = source_map_chirho.add_file_chirho(*name_chirho, *source_chirho);
        let result_chirho = run_frontend_with_inputs_chirho(
            source_chirho,
            file_chirho,
            FrontendInputsChirho::new_chirho(
                &ifaces_chirho,
                &HashMap::new(),
                &ImportedTypeSynonymsChirho::new(),
                &ImportedTypeFamiliesChirho::new(),
            )
            .with_type_contracts_chirho(&contracts_chirho),
        )
        .unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho}"));
        let iface_chirho =
            build_iface_with_imports_chirho(&result_chirho.module_chirho, &ifaces_chirho);
        contracts_chirho.insert(
            iface_chirho.name_chirho.clone(),
            result_chirho.type_contracts_chirho.clone(),
        );
        ifaces_chirho.push(iface_chirho);
        last_chirho = Some(result_chirho);
    }
    (last_chirho.unwrap(), ifaces_chirho, contracts_chirho)
}

#[test]
fn selective_contract_import_cost_tracks_reachable_names_not_provider_size_chirho() {
    for count_chirho in [8, 128] {
        let mut provider_chirho =
            "module SelectProviderChirho where\ndata ChosenChirho = ChosenChirho\n".to_owned();
        for index_chirho in 0..count_chirho {
            provider_chirho.push_str(&format!("type Unused{index_chirho}Chirho = Int\n"));
        }
        let consumer_chirho = "module SelectConsumerChirho where\nimport SelectProviderChirho (ChosenChirho)\ntype SelectedChirho = ChosenChirho\n";
        let (result_chirho, ifaces_chirho, contracts_chirho) = checked_sources_chirho(&[
            ("SelectProviderChirho.hs", &provider_chirho),
            ("SelectConsumerChirho.hs", consumer_chirho),
        ]);
        let selected_chirho = contracts_chirho::select_imported_contracts_chirho(
            &result_chirho.module_chirho,
            &ifaces_chirho,
            Some(&contracts_chirho),
            &HashMap::new(),
        );
        assert!(selected_chirho.kinds_chirho.contains_key("ChosenChirho"));
        assert!(
            selected_chirho.kinds_chirho.len() <= 2,
            "unrelated provider contracts were cloned: {}",
            selected_chirho.kinds_chirho.len()
        );
        assert!(
            selected_chirho.synonyms_chirho.is_empty(),
            "a selective import carried unrelated aliases"
        );
    }
}

#[test]
fn reexported_private_contracts_keep_both_defining_module_identities_chirho() {
    let left_chirho = "module OriginAChirho (PublicAChirho) where\ndata HiddenChirho aChirho = HiddenChirho aChirho\ntype PublicAChirho aChirho = HiddenChirho aChirho\n";
    let right_chirho = "module OriginBChirho (PublicBChirho) where\ndata HiddenChirho aChirho = HiddenChirho aChirho\ntype PublicBChirho aChirho = HiddenChirho aChirho\n";
    let reexport_chirho = "module OriginReexportChirho (PublicAChirho, PublicBChirho) where\nimport OriginAChirho\nimport OriginBChirho\n";
    let consumer_chirho = "module OriginConsumerChirho where\nimport qualified OriginReexportChirho as RChirho\ntype ConsumerAChirho aChirho = RChirho.PublicAChirho aChirho\ntype ConsumerBChirho aChirho = RChirho.PublicBChirho aChirho\n";
    let (result_chirho, _, _) = checked_sources_chirho(&[
        ("OriginAChirho.hs", left_chirho),
        ("OriginBChirho.hs", right_chirho),
        ("OriginReexportChirho.hs", reexport_chirho),
        ("OriginConsumerChirho.hs", consumer_chirho),
    ]);
    let contracts_chirho = &result_chirho.type_contracts_chirho;
    assert!(
        contracts_chirho
            .kinds_chirho
            .contains_key("OriginAChirho.HiddenChirho")
    );
    assert!(
        contracts_chirho
            .kinds_chirho
            .contains_key("OriginBChirho.HiddenChirho")
    );
    assert!(
        !contracts_chirho
            .kinds_chirho
            .contains_key("OriginReexportChirho.HiddenChirho")
    );
    for (alias_chirho, origin_chirho) in [
        ("ConsumerAChirho", "OriginAChirho.HiddenChirho"),
        ("ConsumerBChirho", "OriginBChirho.HiddenChirho"),
    ] {
        let mut dependencies_chirho = Vec::new();
        contracts_chirho.synonyms_chirho[alias_chirho].map_constructor_names_chirho(
            &mut |name_chirho| {
                dependencies_chirho.push(name_chirho.to_owned());
                name_chirho.to_owned()
            },
        );
        assert!(
            dependencies_chirho
                .iter()
                .any(|name_chirho| name_chirho == origin_chirho)
        );
    }
}
