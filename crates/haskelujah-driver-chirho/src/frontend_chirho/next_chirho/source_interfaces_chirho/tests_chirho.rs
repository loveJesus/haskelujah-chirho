// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn hidden_namesake_type_keeps_its_defining_identity_chirho() {
    let sources_chirho = [
        include_str!("fixtures_chirho/nominal-chirho/ALibChirho.hs"),
        include_str!("fixtures_chirho/nominal-chirho/BUseChirho.hs"),
    ];
    let error_chirho = check_sources_chirho(&sources_chirho, "BUseChirho").unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::TypeChirho,
        "{error_chirho}"
    );
}

#[test]
fn dictionaries_and_hidden_defaults_cross_both_directions_chirho() {
    let sources_chirho = [
        include_str!("fixtures_chirho/dictionaries-chirho/ALibChirho.hs"),
        include_str!("fixtures_chirho/dictionaries-chirho/BUseChirho.hs"),
    ];
    let (value_chirho, _) =
        eval_sources_binding_chirho(&sources_chirho, "BUseChirho", "resultChirho", 100_000)
            .unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('t')
    );
}

#[test]
fn abstract_type_dependencies_cross_without_entering_source_scope_chirho() {
    let sources_chirho = [
        include_str!("fixtures_chirho/abstract-chirho/ALibChirho.hs"),
        include_str!("fixtures_chirho/abstract-chirho/BUseChirho.hs"),
    ];
    let (value_chirho, _) =
        eval_sources_binding_chirho(&sources_chirho, "BUseChirho", "resultChirho", 100_000)
            .unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('a')
    );
    let private_chirho = [
        include_str!("fixtures_chirho/private-chirho/ALibChirho.hs"),
        include_str!("fixtures_chirho/private-chirho/BUseChirho.hs"),
    ];
    assert_eq!(
        check_sources_chirho(&private_chirho, "BUseChirho")
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::InvalidChirho
    );
}

#[test]
fn diamond_imports_share_the_original_type_without_a_direct_import_chirho() {
    let sources_chirho = [
        include_str!("fixtures_chirho/diamond-chirho/ALibChirho.hs"),
        include_str!("fixtures_chirho/diamond-chirho/BLeftChirho.hs"),
        include_str!("fixtures_chirho/diamond-chirho/CRightChirho.hs"),
        include_str!("fixtures_chirho/diamond-chirho/BUseChirho.hs"),
    ];
    let (value_chirho, _) =
        eval_sources_binding_chirho(&sources_chirho, "BUseChirho", "resultChirho", 100_000)
            .unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('d')
    );
}

#[test]
fn imported_evidence_tracks_each_type_argument_chirho() {
    let sources_chirho = [
        include_str!("fixtures_chirho/arguments-chirho/ALibChirho.hs"),
        include_str!("fixtures_chirho/arguments-chirho/BUseChirho.hs"),
    ];
    let (value_chirho, mut machine_chirho) =
        eval_sources_binding_chirho(&sources_chirho, "BUseChirho", "resultChirho", 100_000)
            .unwrap();
    let haskelujah_runtime_chirho::ValueChirho::HeapPtrChirho(address_chirho) = value_chirho else {
        panic!("expected pair");
    };
    let fields_chirho = machine_chirho
        .heap_chirho
        .read_chirho(machine_chirho.heap_chirho.follow_ind_chirho(address_chirho))
        .payload_chirho
        .clone();
    assert_eq!(fields_chirho.len(), 2);
    for (field_chirho, expected_chirho) in fields_chirho.into_iter().zip(['t', 'u']) {
        assert_eq!(
            machine_chirho.force_to_prim_chirho(field_chirho),
            haskelujah_runtime_chirho::ValueChirho::CharChirho(expected_chirho)
        );
    }
}

#[test]
fn imported_superclass_projections_keep_their_owner_chirho() {
    let sources_chirho = [
        include_str!("fixtures_chirho/superclass-chirho/ALibChirho.hs"),
        include_str!("fixtures_chirho/superclass-chirho/BUseChirho.hs"),
    ];
    let (value_chirho, _) =
        eval_sources_binding_chirho(&sources_chirho, "BUseChirho", "resultChirho", 100_000)
            .unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('s')
    );
}

#[test]
fn imported_rebindable_conversion_retains_exact_integer_chirho() {
    let sources_chirho = [
        include_str!("fixtures_chirho/rebindable-chirho/ALibChirho.hs"),
        include_str!("fixtures_chirho/rebindable-chirho/BUseChirho.hs"),
    ];
    let (value_chirho, _) =
        eval_sources_binding_chirho(&sources_chirho, "BUseChirho", "resultChirho", 100_000)
            .unwrap();
    let haskelujah_runtime_chirho::ValueChirho::IntegerChirho(integer_chirho) = value_chirho else {
        panic!("expected Integer");
    };
    assert_eq!(
        integer_chirho.to_string(),
        "340282366920938463463374607431768211457"
    );
}

#[test]
fn instance_uniqueness_covers_the_whole_import_union_chirho() {
    let sources_chirho = [
        include_str!("fixtures_chirho/duplicate-chirho/AClassChirho.hs"),
        include_str!("fixtures_chirho/duplicate-chirho/BLeftChirho.hs"),
        include_str!("fixtures_chirho/duplicate-chirho/CRightChirho.hs"),
        include_str!("fixtures_chirho/duplicate-chirho/BUseChirho.hs"),
    ];
    let error_chirho = check_sources_chirho(&sources_chirho, "BUseChirho").unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho,
        "{error_chirho}"
    );
    assert!(error_chirho.message_chirho.contains("instance"));
}

#[test]
fn failed_producers_cycles_and_missing_imports_never_supply_a_stub_chirho() {
    let bad_chirho = "module ALibChirho where\nfChirho = True 'x'";
    let importer_chirho = "module BUseChirho where\nimport ALibChirho\nresultChirho = 'r'";
    assert_eq!(
        check_sources_chirho(&[bad_chirho, importer_chirho], "BUseChirho")
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    assert_eq!(
        check_sources_chirho(&[importer_chirho], "BUseChirho")
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    assert_eq!(
        check_sources_chirho(
            &[
                "module AChirho where\nimport BChirho\nfChirho='a'",
                "module BChirho where\nimport AChirho\ngChirho='b'"
            ],
            "AChirho"
        )
        .unwrap_err()
        .kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    let sources_chirho =
        vec!["module AChirho where\naChirho='a'"; graph_chirho::MAX_MODULES_CHIRHO + 1];
    assert_eq!(
        check_sources_chirho(&sources_chirho, "AChirho")
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::ResourceChirho
    );
}

#[test]
fn projection_requires_closed_binders_and_charges_output_chirho() {
    let mut checked_chirho = super::super::check_source_chirho(
        "module AChirho where\nfChirho valueChirho = valueChirho",
    )
    .unwrap();
    assert_eq!(
        haskelujah_tc_chirho::project_interface_chirho(&checked_chirho, &mut 0)
            .unwrap_err()
            .kind_chirho,
        haskelujah_tc_chirho::TcErrorKindChirho::ResourceChirho
    );
    let name_chirho = checked_chirho.module_chirho.bindings_chirho[0].name_chirho;
    checked_chirho
        .schemes_chirho
        .get_mut(&name_chirho)
        .unwrap()
        .quantified_chirho
        .clear();
    assert_eq!(
        haskelujah_tc_chirho::project_interface_chirho(&checked_chirho, &mut 100_000)
            .unwrap_err()
            .kind_chirho,
        haskelujah_tc_chirho::TcErrorKindChirho::InvalidReferenceChirho
    );
}

#[test]
fn file_loading_uses_the_same_checked_interface_path_chirho() {
    let path_chirho=Path::new(env!("CARGO_MANIFEST_DIR")).join("src/frontend_chirho/next_chirho/source_interfaces_chirho/fixtures_chirho/dictionaries-chirho/BUseChirho.hs");
    check_source_file_chirho(&path_chirho).unwrap();
    let core_chirho = compile_source_file_chirho(&path_chirho).unwrap();
    let (value_chirho, _) =
        super::super::eval_core_binding_chirho(core_chirho, "resultChirho", 100_000).unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('t')
    );
}

#[test]
fn direct_and_relayed_exports_are_one_original_name_chirho() {
    let sources_chirho = [
        include_str!("fixtures_chirho/relay-chirho/ALibChirho.hs"),
        include_str!("fixtures_chirho/relay-chirho/RelayChirho.hs"),
        include_str!("fixtures_chirho/relay-chirho/BUseChirho.hs"),
    ];
    let (value_chirho, _) =
        eval_sources_binding_chirho(&sources_chirho, "BUseChirho", "resultChirho", 100_000)
            .unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('r')
    );
}

#[test]
fn original_names_keep_unit_namespace_and_component_boundaries_chirho() {
    use haskelujah_hir_chirho::{NamespaceChirho, UnitIdentityChirho};
    let original_chirho = OriginalNameChirho {
        unit_chirho: UnitIdentityChirho::StandaloneHomeChirho,
        module_chirho: "GHC.Internal.Types".into(),
        namespace_chirho: NamespaceChirho::TypeChirho,
        occurrence_chirho: "Bool".into(),
    };
    let mut other_chirho = original_chirho.clone();
    other_chirho.unit_chirho = UnitIdentityChirho::CompilerChirho;
    assert_ne!(
        original_chirho.link_key_chirho("type"),
        other_chirho.link_key_chirho("type")
    );
    other_chirho = original_chirho.clone();
    other_chirho.namespace_chirho = NamespaceChirho::ValueChirho;
    assert_ne!(
        original_chirho.link_key_chirho("value"),
        other_chirho.link_key_chirho("value")
    );
    other_chirho = original_chirho.clone();
    other_chirho.module_chirho.push_str(":Bool");
    other_chirho.occurrence_chirho.clear();
    assert_ne!(
        original_chirho.link_key_chirho("value"),
        other_chirho.link_key_chirho("value")
    );
}

#[test]
fn an_unresolved_core_reference_never_becomes_a_runtime_stub_chirho() {
    use haskelujah_core_chirho::{
        BinderChirho, CoreBindingChirho, CoreExprChirho, InlineAnnotationChirho,
    };
    let core_chirho = CoreModuleChirho {
        name_chirho: "AChirho".into(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(0),
                name_chirho: "resultChirho".into(),
                ty_chirho: haskelujah_typing_chirho::ty_chirho::TyChirho::char_chirho(),
                span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(99)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: Default::default(),
        specialize_pragmas_chirho: Default::default(),
        foreign_exports_chirho: Vec::new(),
        provenance_chirho: Default::default(),
    };
    let original_chirho = OriginalNameChirho {
        unit_chirho: haskelujah_hir_chirho::UnitIdentityChirho::StandaloneHomeChirho,
        module_chirho: "AChirho".into(),
        namespace_chirho: haskelujah_hir_chirho::NamespaceChirho::ValueChirho,
        occurrence_chirho: "resultChirho".into(),
    };
    let object_chirho = ObjectChirho {
        core_chirho,
        definitions_chirho: BTreeMap::from([(CoreIdChirho(0), original_chirho)]),
        imports_chirho: BTreeMap::new(),
    };
    let error_chirho = link_chirho::link_chirho(vec![object_chirho], "AChirho").unwrap_err();
    assert_eq!(error_chirho.kind_chirho, NextErrorKindChirho::InvalidChirho);
    assert!(error_chirho.message_chirho.contains("unresolved"));
}

fn imported_hir_chirho(
    interface_chirho: Arc<haskelujah_hir_chirho::InterfaceChirho>,
) -> haskelujah_hir_chirho::ModuleChirho {
    let source_chirho = "module BUseChirho where\nimport ALibChirho\nresultChirho = fChirho 'a'";
    let parsed_chirho = super::super::parse_chirho::parse_chirho(
        source_chirho,
        haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
    )
    .unwrap();
    haskelujah_hir_chirho::lower_module_with_interfaces_chirho(
        &parsed_chirho.module_chirho,
        haskelujah_hir_chirho::UnitIdentityChirho::StandaloneHomeChirho,
        haskelujah_hir_chirho::LimitsChirho::default(),
        &parsed_chirho.integer_literals_chirho,
        &BTreeMap::from([("ALibChirho".into(), interface_chirho)]),
    )
    .unwrap()
}

#[test]
fn imported_types_have_consumer_ownership_and_failed_budgets_do_not_poison_exports_chirho() {
    let library_chirho = super::super::check_source_chirho(
        "module ALibChirho where\nfChirho valueChirho = valueChirho",
    )
    .unwrap();
    let id_chirho = library_chirho.module_chirho.bindings_chirho[0].name_chirho;
    let producer_ty_chirho = library_chirho.names_chirho[&id_chirho];
    let interface_chirho = Arc::new(
        haskelujah_tc_chirho::project_interface_chirho(&library_chirho, &mut 100_000).unwrap(),
    );
    let error_chirho = haskelujah_tc_chirho::check_module_chirho(
        imported_hir_chirho(Arc::clone(&interface_chirho)),
        1,
    )
    .unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        haskelujah_tc_chirho::TcErrorKindChirho::ResourceChirho
    );
    let consumer_chirho =
        haskelujah_tc_chirho::check_module_chirho(imported_hir_chirho(interface_chirho), 100_000)
            .unwrap();
    assert_eq!(
        consumer_chirho
            .solver_chirho
            .arena_chirho
            .node_chirho(producer_ty_chirho)
            .unwrap_err()
            .kind_chirho,
        haskelujah_tc_chirho::TcErrorKindChirho::InvalidReferenceChirho
    );
}

#[test]
fn malformed_import_maps_and_unbound_scheme_positions_refuse_before_publication_chirho() {
    let library_chirho=super::super::check_source_chirho("module ALibChirho where\nfChirho valueChirho = valueChirho\ngChirho valueChirho = valueChirho").unwrap();
    let interface_chirho =
        haskelujah_tc_chirho::project_interface_chirho(&library_chirho, &mut 100_000).unwrap();
    let mut hir_chirho = imported_hir_chirho(Arc::new(interface_chirho.clone()));
    let originals_chirho: Vec<_> = hir_chirho.imports_chirho[0]
        .names_chirho
        .keys()
        .cloned()
        .collect();
    assert_eq!(originals_chirho.len(), 2);
    let id_chirho = hir_chirho.imports_chirho[0].names_chirho[&originals_chirho[0]];
    hir_chirho.imports_chirho[0]
        .names_chirho
        .insert(originals_chirho[1].clone(), id_chirho);
    assert_eq!(
        haskelujah_tc_chirho::check_module_chirho(hir_chirho, 100_000)
            .unwrap_err()
            .kind_chirho,
        haskelujah_tc_chirho::TcErrorKindChirho::InvalidReferenceChirho
    );
    let mut malformed_chirho = interface_chirho;
    malformed_chirho
        .values_chirho
        .get_mut(&originals_chirho[0])
        .unwrap()
        .scheme_chirho
        .body_chirho = haskelujah_hir_chirho::InterfaceTypeChirho::BoundChirho(999);
    assert_eq!(
        haskelujah_tc_chirho::check_module_chirho(
            imported_hir_chirho(Arc::new(malformed_chirho)),
            100_000
        )
        .unwrap_err()
        .kind_chirho,
        haskelujah_tc_chirho::TcErrorKindChirho::InvalidReferenceChirho
    );
}

#[test]
fn unused_imported_overlap_is_unsupported_instead_of_a_false_semantic_refusal_chirho() {
    let sources_chirho = [
        include_str!("fixtures_chirho/overlap-unused-chirho/AClassChirho.hs"),
        include_str!("fixtures_chirho/overlap-unused-chirho/BLeftChirho.hs"),
        include_str!("fixtures_chirho/overlap-unused-chirho/CRightChirho.hs"),
        include_str!("fixtures_chirho/overlap-unused-chirho/BUseChirho.hs"),
    ];
    assert_eq!(
        check_sources_chirho(&sources_chirho, "BUseChirho")
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
}

#[test]
fn repeated_plain_imports_do_not_duplicate_interface_storage_chirho() {
    let library_chirho = super::super::check_source_chirho(
        "module ALibChirho where\nfChirho valueChirho = valueChirho",
    )
    .unwrap();
    let interface_chirho = Arc::new(
        haskelujah_tc_chirho::project_interface_chirho(&library_chirho, &mut 100_000).unwrap(),
    );
    let source_chirho = format!(
        "module BUseChirho where\n{}resultChirho = fChirho 'a'\n",
        "import ALibChirho\n".repeat(2000)
    );
    let parsed_chirho = super::super::parse_chirho::parse_chirho(
        &source_chirho,
        haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
    )
    .unwrap();
    let hir_chirho = haskelujah_hir_chirho::lower_module_with_interfaces_chirho(
        &parsed_chirho.module_chirho,
        haskelujah_hir_chirho::UnitIdentityChirho::StandaloneHomeChirho,
        haskelujah_hir_chirho::LimitsChirho {
            nodes_chirho: 512,
            depth_chirho: 256,
        },
        &parsed_chirho.integer_literals_chirho,
        &BTreeMap::from([("ALibChirho".into(), interface_chirho)]),
    )
    .unwrap();
    haskelujah_tc_chirho::check_module_chirho(hir_chirho, 100_000).unwrap();
}
