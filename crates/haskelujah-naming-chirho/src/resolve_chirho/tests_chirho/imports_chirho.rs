// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn mk_lib_iface_chirho() -> ModuleIfaceChirho {
    let mut values_chirho = std::collections::HashMap::new();
    values_chirho.insert(
        "sort".to_string(),
        IfaceValueChirho {
            availability_chirho: crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
            name_chirho: "sort".to_string(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            defining_module_chirho: None,
            constructor_fields_chirho: None,
            compiler_value_chirho: None,
        },
    );
    values_chirho.insert(
        "map".to_string(),
        IfaceValueChirho {
            availability_chirho: crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
            name_chirho: "map".to_string(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            defining_module_chirho: None,
            constructor_fields_chirho: None,
            compiler_value_chirho: None,
        },
    );
    values_chirho.insert(
        "Red".to_string(),
        IfaceValueChirho {
            availability_chirho: crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
            name_chirho: "Red".to_string(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            defining_module_chirho: None,
            constructor_fields_chirho: None,
            compiler_value_chirho: None,
        },
    );
    values_chirho.insert(
        "Blue".to_string(),
        IfaceValueChirho {
            availability_chirho: crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
            name_chirho: "Blue".to_string(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            defining_module_chirho: None,
            constructor_fields_chirho: None,
            compiler_value_chirho: None,
        },
    );

    let mut types_chirho = std::collections::HashMap::new();
    types_chirho.insert(
        "Color".to_string(),
        IfaceTypeChirho {
            declaration_chirho: None,
            name_chirho: "Color".to_string(),
            constructors_chirho: vec!["Red".to_string(), "Blue".to_string()],
            methods_chirho: vec![],
            is_class_chirho: false,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    );

    ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: "Lib".to_string(),
        exports_chirho: crate::iface_chirho::IfaceExportsChirho {
            unary_variable_instance_heads_covered_chirho: false,
            values_chirho,
            types_chirho,
            associated_types_chirho: std::collections::HashMap::new(),
            complete_member_types_chirho: std::collections::HashSet::new(),
            origin_chirho: Default::default(),
        },
    }
}

#[test]
fn import_all_from_module_chirho() {
    let lib_chirho = mk_lib_iface_chirho();
    let module_chirho = mk_module_chirho(
        vec![],
        vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: dummy_name_chirho("Lib"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: None, // import all
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho = resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    // Unqualified lookup
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("sort")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("map")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_type_chirho("Color")
            .is_some()
    );
    // Qualified lookup
    assert!(
        result_chirho
            .env_chirho
            .lookup_qualified_chirho("Lib", "sort", NamespaceChirho::ValueChirho)
            .is_some()
    );
}

#[test]
fn import_qualified_only_chirho() {
    let lib_chirho = mk_lib_iface_chirho();
    let module_chirho = mk_module_chirho(
        vec![],
        vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: dummy_name_chirho("Lib"),
            source_chirho: false,
            qualified_chirho: true, // qualified only
            alias_chirho: None,
            spec_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho = resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

    // Unqualified lookup should NOT find imported names
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("sort")
            .is_none()
    );
    // Qualified lookup should work
    assert!(
        result_chirho
            .env_chirho
            .lookup_qualified_chirho("Lib", "sort", NamespaceChirho::ValueChirho)
            .is_some()
    );
}

#[test]
fn import_with_alias_chirho() {
    let lib_chirho = mk_lib_iface_chirho();
    let module_chirho = mk_module_chirho(
        vec![],
        vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: dummy_name_chirho("Lib"),
            source_chirho: false,
            qualified_chirho: true,
            alias_chirho: Some(dummy_name_chirho("L")),
            spec_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho = resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

    // Should be accessible via alias
    assert!(
        result_chirho
            .env_chirho
            .lookup_qualified_chirho("L", "sort", NamespaceChirho::ValueChirho)
            .is_some()
    );
    // Original module name should NOT work with alias
    assert!(
        result_chirho
            .env_chirho
            .lookup_qualified_chirho("Lib", "sort", NamespaceChirho::ValueChirho)
            .is_none()
    );
}

#[test]
fn import_explicit_list_chirho() {
    let lib_chirho = mk_lib_iface_chirho();
    let module_chirho = mk_module_chirho(
        vec![],
        vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: dummy_name_chirho("Lib"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: Some(ImportSpecChirho {
                hiding_chirho: false,
                items_chirho: vec![ImportItemChirho::VarChirho(dummy_name_chirho("sort"))],
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho = resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("sort")
            .is_some()
    );
    // map is NOT imported
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("map")
            .is_none()
    );
}

#[test]
fn import_type_all_includes_constructor_members_chirho() {
    let mut lib_chirho = mk_lib_iface_chirho();
    lib_chirho.exports_chirho.values_chirho.insert(
        "Wrap".to_string(),
        IfaceValueChirho {
            availability_chirho: crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
            name_chirho: "Wrap".to_string(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            defining_module_chirho: None,
            constructor_fields_chirho: None,
            compiler_value_chirho: None,
        },
    );
    lib_chirho.exports_chirho.values_chirho.insert(
        "unwrap".to_string(),
        IfaceValueChirho {
            availability_chirho: crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
            name_chirho: "unwrap".to_string(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            defining_module_chirho: None,
            constructor_fields_chirho: None,
            compiler_value_chirho: None,
        },
    );
    lib_chirho.exports_chirho.types_chirho.insert(
        "Wrap".to_string(),
        IfaceTypeChirho {
            declaration_chirho: None,
            name_chirho: "Wrap".to_string(),
            constructors_chirho: vec!["Wrap".to_string()],
            methods_chirho: vec!["unwrap".to_string()],
            is_class_chirho: false,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    );
    let module_chirho = mk_module_chirho(
        vec![],
        vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: dummy_name_chirho("Lib"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: Some(ImportSpecChirho {
                hiding_chirho: false,
                items_chirho: vec![ImportItemChirho::TyConChirho {
                    name_chirho: dummy_name_chirho("Wrap"),
                    members_chirho:
                        haskelujah_ast_chirho::module_chirho::ExportMembersChirho::AllChirho,
                }],
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho = resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

    assert!(
        result_chirho
            .env_chirho
            .lookup_type_chirho("Wrap")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("Wrap")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("unwrap")
            .is_some()
    );
}

#[test]
fn import_builtin_class_all_includes_methods_chirho() {
    let builtins_chirho = crate::iface_chirho::builtin_module_ifaces_chirho();
    let control_monad_iface_chirho = builtins_chirho
        .iter()
        .rev()
        .find(|iface_chirho| iface_chirho.name_chirho == "Control.Monad")
        .expect("Control.Monad builtin iface should exist");
    let monad_plus_iface_chirho = control_monad_iface_chirho
        .exports_chirho
        .types_chirho
        .get("MonadPlus")
        .expect("Control.Monad should export MonadPlus");
    assert_eq!(
        monad_plus_iface_chirho.methods_chirho,
        vec!["mzero".to_string(), "mplus".to_string()]
    );
    let spec_chirho = ImportSpecChirho {
        hiding_chirho: false,
        items_chirho: vec![ImportItemChirho::TyConChirho {
            name_chirho: dummy_name_chirho("MonadPlus"),
            members_chirho: haskelujah_ast_chirho::module_chirho::ExportMembersChirho::AllChirho,
        }],
    };
    let imported_names_chirho = compute_imported_names_chirho(
        &control_monad_iface_chirho.exports_chirho,
        &Some(spec_chirho.clone()),
    );
    let imported_value_names_chirho: Vec<String> = imported_names_chirho
        .iter()
        .filter(|(_, namespace_chirho, _)| *namespace_chirho == NamespaceChirho::ValueChirho)
        .map(|(name_chirho, _, _)| name_chirho.clone())
        .collect();
    assert!(
        imported_value_names_chirho.contains(&"mplus".to_string()),
        "expected MonadPlus(..) to expand to mplus, got {:?}",
        imported_names_chirho
    );
    assert!(
        imported_value_names_chirho.contains(&"mzero".to_string()),
        "expected MonadPlus(..) to expand to mzero, got {:?}",
        imported_names_chirho
    );
    let module_chirho = mk_module_chirho(
        vec![],
        vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: dummy_name_chirho("Control.Monad"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: Some(spec_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho = resolve_module_with_imports_chirho(&module_chirho, &builtins_chirho);

    assert!(
        result_chirho
            .env_chirho
            .lookup_type_chirho("MonadPlus")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("mplus")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("mzero")
            .is_some()
    );
}

#[test]
fn import_prefers_latest_iface_for_same_module_name_chirho() {
    let older_iface_chirho = ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: "Lib".to_string(),
        exports_chirho: crate::iface_chirho::IfaceExportsChirho {
            unary_variable_instance_heads_covered_chirho: false,
            values_chirho: [(
                "oldOnly".to_string(),
                IfaceValueChirho {
                    availability_chirho:
                        crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                    name_chirho: "oldOnly".to_string(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    defining_module_chirho: None,
                    constructor_fields_chirho: None,
                    compiler_value_chirho: None,
                },
            )]
            .into_iter()
            .collect(),
            types_chirho: std::collections::HashMap::new(),
            associated_types_chirho: std::collections::HashMap::new(),
            complete_member_types_chirho: std::collections::HashSet::new(),
            origin_chirho: Default::default(),
        },
    };
    let newer_iface_chirho = ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: "Lib".to_string(),
        exports_chirho: crate::iface_chirho::IfaceExportsChirho {
            unary_variable_instance_heads_covered_chirho: false,
            values_chirho: [(
                "newOnly".to_string(),
                IfaceValueChirho {
                    availability_chirho:
                        crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                    name_chirho: "newOnly".to_string(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    defining_module_chirho: None,
                    constructor_fields_chirho: None,
                    compiler_value_chirho: None,
                },
            )]
            .into_iter()
            .collect(),
            types_chirho: std::collections::HashMap::new(),
            associated_types_chirho: std::collections::HashMap::new(),
            complete_member_types_chirho: std::collections::HashSet::new(),
            origin_chirho: Default::default(),
        },
    };
    let module_chirho = mk_module_chirho(
        vec![],
        vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: dummy_name_chirho("Lib"),
            source_chirho: false,
            qualified_chirho: true,
            alias_chirho: Some(dummy_name_chirho("L")),
            spec_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho = resolve_module_with_imports_chirho(
        &module_chirho,
        &[older_iface_chirho, newer_iface_chirho],
    );

    assert!(
        result_chirho
            .env_chirho
            .lookup_qualified_chirho("L", "newOnly", NamespaceChirho::ValueChirho)
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_qualified_chirho("L", "oldOnly", NamespaceChirho::ValueChirho)
            .is_none()
    );
}

#[test]
fn import_hiding_chirho() {
    let lib_chirho = mk_lib_iface_chirho();
    let module_chirho = mk_module_chirho(
        vec![],
        vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: dummy_name_chirho("Lib"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: Some(ImportSpecChirho {
                hiding_chirho: true,
                items_chirho: vec![ImportItemChirho::VarChirho(dummy_name_chirho("sort"))],
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho = resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

    // sort is hidden
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("sort")
            .is_none()
    );
    // map is still imported
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("map")
            .is_some()
    );
}

#[test]
fn unknown_module_reports_error_chirho() {
    let module_chirho = mk_module_chirho(
        vec![],
        vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: dummy_name_chirho("Nonexistent"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho = resolve_module_with_imports_chirho(&module_chirho, &[]);

    // Unknown module import is an error
    assert!(result_chirho.diagnostics_chirho.has_errors_chirho());
    let msg_chirho = format!("{}", result_chirho.diagnostics_chirho);
    assert!(msg_chirho.contains("Nonexistent"));
}

#[test]
fn import_does_not_shadow_local_chirho() {
    let lib_chirho = mk_lib_iface_chirho();
    let module_chirho = mk_module_chirho(
        vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("sort"),
            matches_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: dummy_name_chirho("Lib"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho = resolve_module_with_imports_chirho(&module_chirho, &[lib_chirho]);

    // Local definition should win (it's bound after imports)
    let info_chirho = result_chirho
        .env_chirho
        .lookup_value_chirho("sort")
        .unwrap();
    assert!(!info_chirho.imported_chirho);
}

// ---------------------------------------------------------------
// Record field, pattern bind, and instance tests
// ---------------------------------------------------------------
