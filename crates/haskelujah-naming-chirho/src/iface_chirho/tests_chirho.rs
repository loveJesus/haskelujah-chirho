// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Interface building: export lists, re-exports and foreign imports.

use super::*;
use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, ForeignDirectionChirho};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

pub(super) fn mk_name_chirho(s_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        s_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn mk_module_chirho(
    name_chirho: &str,
    exports_chirho: Option<Vec<ExportSpecChirho>>,
    decls_chirho: Vec<DeclChirho>,
) -> ModuleChirho {
    ModuleChirho {
        name_chirho: mk_name_chirho(name_chirho),
        exports_chirho,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

#[test]
fn export_all_when_no_export_list_chirho() {
    let module_chirho = mk_module_chirho(
        "Lib",
        None, // no export list = export everything
        vec![
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("Color"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: mk_name_chirho("Red"),
                        fields_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                        prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(
                        ),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: mk_name_chirho("Blue"),
                        fields_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                        prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(
                        ),
                    },
                ],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: mk_name_chirho("paint"),
                matches_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
    );

    let iface_chirho = build_iface_chirho(&module_chirho)
        .expect("small source interface fixture fits constructor projection budget");
    assert_eq!(iface_chirho.name_chirho, "Lib");
    assert!(
        iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("Color")
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("Red")
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("Blue")
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("paint")
    );
}

#[test]
fn export_list_filters_chirho() {
    let module_chirho = mk_module_chirho(
        "Lib",
        Some(vec![
            // Export Color with all constructors
            ExportSpecChirho::TyConChirho {
                name_chirho: mk_name_chirho("Color"),
                members_chirho: ExportMembersChirho::AllChirho,
            },
            // Export paint function
            ExportSpecChirho::VarChirho(mk_name_chirho("paint")),
            // Do NOT export helper
        ]),
        vec![
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("Color"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: mk_name_chirho("Red"),
                    fields_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                }],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: mk_name_chirho("paint"),
                matches_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: mk_name_chirho("helper"),
                matches_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
    );

    let iface_chirho = build_iface_chirho(&module_chirho)
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("Color")
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("Red")
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("paint")
    );
    // helper is NOT exported
    assert!(
        !iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("helper")
    );
}

#[test]
fn export_type_without_constructors_chirho() {
    let module_chirho = mk_module_chirho(
        "Lib",
        Some(vec![ExportSpecChirho::TyConChirho {
            name_chirho: mk_name_chirho("Color"),
            members_chirho: ExportMembersChirho::NoneChirho,
        }]),
        vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Color"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("Red"),
                fields_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let iface_chirho = build_iface_chirho(&module_chirho)
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("Color")
    );
    // Red is NOT exported — only the type name
    assert!(
        !iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("Red")
    );
    let color_ty_chirho = &iface_chirho.exports_chirho.types_chirho["Color"];
    assert!(color_ty_chirho.constructors_chirho.is_empty());
}

#[test]
fn export_some_constructors_chirho() {
    let module_chirho = mk_module_chirho(
        "Lib",
        Some(vec![ExportSpecChirho::TyConChirho {
            name_chirho: mk_name_chirho("Color"),
            members_chirho: ExportMembersChirho::SomeChirho(vec![mk_name_chirho("Red")]),
        }]),
        vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Color"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: mk_name_chirho("Red"),
                    fields_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: mk_name_chirho("Blue"),
                    fields_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
            ],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let iface_chirho = build_iface_chirho(&module_chirho)
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("Red")
    );
    assert!(
        !iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("Blue")
    );
    let ty_chirho = &iface_chirho.exports_chirho.types_chirho["Color"];
    assert_eq!(ty_chirho.constructors_chirho, vec!["Red"]);
}

#[test]
fn export_parenthesized_operator_type_with_constructor_chirho() {
    let module_chirho = mk_module_chirho(
        "Lib",
        Some(vec![ExportSpecChirho::TyConChirho {
            name_chirho: mk_name_chirho("(:-)"),
            members_chirho: ExportMembersChirho::AllChirho,
        }]),
        vec![DeclChirho::NewtypeDeclChirho {
            name_chirho: mk_name_chirho(":-"),
            type_vars_chirho: vec![mk_name_chirho("a").into(), mk_name_chirho("b").into()],
            constructor_chirho: Box::new(ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("Sub"),
                fields_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }),
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let iface_chirho = build_iface_chirho(&module_chirho)
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho.exports_chirho.types_chirho.contains_key(":-"),
        "parenthesized operator type export should match the operator type key"
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("Sub"),
        "exporting (:-)(..) should expose the Sub constructor"
    );
    assert_eq!(
        iface_chirho.exports_chirho.types_chirho[":-"].constructors_chirho,
        vec!["Sub"]
    );
}

#[test]
fn class_methods_exported_chirho() {
    let module_chirho = mk_module_chirho(
        "Lib",
        None,
        vec![DeclChirho::ClassDeclChirho {
            standalone_kind_chirho: None,
            context_chirho: vec![],
            name_chirho: mk_name_chirho("Show"),
            type_vars_chirho: vec![],
            methods_chirho: vec![haskelujah_ast_chirho::decl_chirho::ClassMethodChirho {
                name_chirho: mk_name_chirho("show"),
                ty_chirho: haskelujah_ast_chirho::ty_chirho::TypeChirho::VarChirho(mk_name_chirho(
                    "a",
                )),
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let iface_chirho = build_iface_chirho(&module_chirho)
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("Show")
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("show")
    );
    assert_eq!(
        iface_chirho.exports_chirho.types_chirho["Show"].methods_chirho,
        vec!["show"]
    );
}

#[test]
fn record_field_selectors_exported_chirho() {
    let module_chirho = mk_module_chirho(
        "Lib",
        None,
        vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("LanguageDefChirho"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::RecordChirho {
                name_chirho: mk_name_chirho("LanguageDefChirho"),
                fields_chirho: vec![
                    haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
                        multiplicity_chirho: None,
                        names_chirho: vec![mk_name_chirho("opLetterChirho")],
                        ty_chirho: haskelujah_ast_chirho::ty_chirho::TypeChirho::ConChirho(
                            mk_name_chirho("String"),
                        ),
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
                        multiplicity_chirho: None,
                        names_chirho: vec![mk_name_chirho("reservedNamesChirho")],
                        ty_chirho: haskelujah_ast_chirho::ty_chirho::TypeChirho::ListChirho {
                            element_chirho: Box::new(
                                haskelujah_ast_chirho::ty_chirho::TypeChirho::ConChirho(
                                    mk_name_chirho("String"),
                                ),
                            ),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let iface_chirho = build_iface_chirho(&module_chirho)
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("opLetterChirho")
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("reservedNamesChirho")
    );
    assert_eq!(
        iface_chirho.exports_chirho.types_chirho["LanguageDefChirho"].methods_chirho,
        vec!["opLetterChirho", "reservedNamesChirho"]
    );
}

#[test]
fn newtype_exported_chirho() {
    let module_chirho = mk_module_chirho(
        "Lib",
        None,
        vec![DeclChirho::NewtypeDeclChirho {
            name_chirho: mk_name_chirho("Wrapper"),
            type_vars_chirho: vec![],
            constructor_chirho: Box::new(ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("Wrap"),
                fields_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }),
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let iface_chirho = build_iface_chirho(&module_chirho)
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("Wrapper")
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("Wrap")
    );
}

#[test]
fn module_re_export_chirho() {
    // module Reexporter (module Inner) where
    // import Inner
    // extra = 42
    use haskelujah_ast_chirho::module_chirho::ImportDeclChirho;

    // Simulate the Inner module interface.
    let inner_iface_chirho = ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: "Inner".to_string(),
        exports_chirho: {
            let mut e_chirho = IfaceExportsChirho::default();
            e_chirho.values_chirho.insert(
                "innerFn".to_string(),
                IfaceValueChirho {
                    availability_chirho:
                        crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                    name_chirho: "innerFn".to_string(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    defining_module_chirho: None,
                    constructor_fields_chirho: None,
                    compiler_value_chirho: None,
                },
            );
            e_chirho.types_chirho.insert(
                "InnerType".to_string(),
                IfaceTypeChirho {
                    declaration_chirho: None,
                    name_chirho: "InnerType".to_string(),
                    constructors_chirho: vec!["MkInner".to_string()],
                    methods_chirho: vec![],
                    is_class_chirho: false,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            );
            e_chirho.values_chirho.insert(
                "MkInner".to_string(),
                IfaceValueChirho {
                    availability_chirho:
                        crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                    name_chirho: "MkInner".to_string(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    defining_module_chirho: None,
                    constructor_fields_chirho: None,
                    compiler_value_chirho: None,
                },
            );
            e_chirho
        },
    };

    let module_chirho = ModuleChirho {
        name_chirho: mk_name_chirho("Reexporter"),
        exports_chirho: Some(vec![
            // Re-export everything from Inner.
            ExportSpecChirho::ModuleChirho(mk_name_chirho("Inner")),
        ]),
        imports_chirho: vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: mk_name_chirho("Inner"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: mk_name_chirho("extra"),
            matches_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &[inner_iface_chirho])
        .expect("small source interface fixture fits constructor projection budget");

    // Re-exported names from Inner should be present.
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("innerFn"),
        "innerFn should be re-exported"
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("MkInner"),
        "MkInner constructor should be re-exported"
    );
    assert!(
        iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("InnerType"),
        "InnerType should be re-exported"
    );
    // `extra` is NOT in the export list (only `module Inner` is).
    assert!(
        !iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("extra"),
        "extra should NOT be exported (not in export list)"
    );
}

#[test]
fn module_alias_re_export_chirho() {
    use haskelujah_ast_chirho::module_chirho::ImportDeclChirho;

    let inner_iface_chirho = ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: "Inner".to_string(),
        exports_chirho: {
            let mut e_chirho = IfaceExportsChirho::default();
            e_chirho.values_chirho.insert(
                "innerFn".to_string(),
                IfaceValueChirho {
                    availability_chirho:
                        crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                    name_chirho: "innerFn".to_string(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    defining_module_chirho: None,
                    constructor_fields_chirho: None,
                    compiler_value_chirho: None,
                },
            );
            e_chirho
        },
    };

    let module_chirho = ModuleChirho {
        name_chirho: mk_name_chirho("Reexporter"),
        exports_chirho: Some(vec![ExportSpecChirho::ModuleChirho(mk_name_chirho(
            "Alias",
        ))]),
        imports_chirho: vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: mk_name_chirho("Inner"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: Some(mk_name_chirho("Alias")),
            spec_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        decls_chirho: vec![],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &[inner_iface_chirho])
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("innerFn")
    );
}

#[test]
fn module_alias_re_export_unions_matching_imports_chirho() {
    use haskelujah_ast_chirho::module_chirho::{ImportDeclChirho, ImportSpecChirho};

    let left_iface_chirho = ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: "LeftMod".to_string(),
        exports_chirho: {
            let mut e_chirho = IfaceExportsChirho::default();
            e_chirho.values_chirho.insert(
                "leftFn".to_string(),
                IfaceValueChirho {
                    availability_chirho:
                        crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                    name_chirho: "leftFn".to_string(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    defining_module_chirho: None,
                    constructor_fields_chirho: None,
                    compiler_value_chirho: None,
                },
            );
            e_chirho.values_chirho.insert(
                "hiddenLeft".to_string(),
                IfaceValueChirho {
                    availability_chirho:
                        crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                    name_chirho: "hiddenLeft".to_string(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    defining_module_chirho: None,
                    constructor_fields_chirho: None,
                    compiler_value_chirho: None,
                },
            );
            e_chirho
        },
    };
    let right_iface_chirho = ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: "RightMod".to_string(),
        exports_chirho: {
            let mut e_chirho = IfaceExportsChirho::default();
            e_chirho.values_chirho.insert(
                "rightFn".to_string(),
                IfaceValueChirho {
                    availability_chirho:
                        crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                    name_chirho: "rightFn".to_string(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    defining_module_chirho: None,
                    constructor_fields_chirho: None,
                    compiler_value_chirho: None,
                },
            );
            e_chirho
        },
    };

    let module_chirho = ModuleChirho {
        name_chirho: mk_name_chirho("Reexporter"),
        exports_chirho: Some(vec![ExportSpecChirho::ModuleChirho(mk_name_chirho(
            "Alias",
        ))]),
        imports_chirho: vec![
            ImportDeclChirho {
                package_chirho: None,
                module_chirho: mk_name_chirho("LeftMod"),
                source_chirho: false,
                qualified_chirho: false,
                alias_chirho: Some(mk_name_chirho("Alias")),
                spec_chirho: Some(ImportSpecChirho {
                    hiding_chirho: true,
                    items_chirho: vec![ImportItemChirho::VarChirho(mk_name_chirho("hiddenLeft"))],
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            ImportDeclChirho {
                package_chirho: None,
                module_chirho: mk_name_chirho("RightMod"),
                source_chirho: false,
                qualified_chirho: false,
                alias_chirho: Some(mk_name_chirho("Alias")),
                spec_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        decls_chirho: vec![],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let iface_chirho =
        build_iface_with_imports_chirho(&module_chirho, &[left_iface_chirho, right_iface_chirho])
            .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("leftFn")
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("rightFn")
    );
    assert!(
        !iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("hiddenLeft")
    );
}

#[test]
fn explicit_export_list_reexports_imported_value_chirho() {
    use haskelujah_ast_chirho::module_chirho::ImportDeclChirho;

    let imported_iface_chirho = ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: "InnerChoice".to_string(),
        exports_chirho: {
            let mut exports_chirho = IfaceExportsChirho::default();
            exports_chirho.values_chirho.insert(
                "choice".to_string(),
                IfaceValueChirho {
                    availability_chirho:
                        crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                    name_chirho: "choice".to_string(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    defining_module_chirho: None,
                    constructor_fields_chirho: None,
                    compiler_value_chirho: None,
                },
            );
            exports_chirho
        },
    };

    let module_chirho = ModuleChirho {
        name_chirho: mk_name_chirho("OuterChoice"),
        exports_chirho: Some(vec![ExportSpecChirho::VarChirho(mk_name_chirho("choice"))]),
        imports_chirho: vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: mk_name_chirho("InnerChoice"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        decls_chirho: vec![],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &[imported_iface_chirho])
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("choice")
    );
}

#[test]
fn explicit_export_list_reexports_imported_type_chirho() {
    use haskelujah_ast_chirho::module_chirho::ImportDeclChirho;

    let imported_iface_chirho = ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: "InnerParsec".to_string(),
        exports_chirho: {
            let mut exports_chirho = IfaceExportsChirho::default();
            exports_chirho.types_chirho.insert(
                "Parsec".to_string(),
                IfaceTypeChirho {
                    declaration_chirho: None,
                    name_chirho: "Parsec".to_string(),
                    constructors_chirho: vec![],
                    methods_chirho: vec![],
                    is_class_chirho: false,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            );
            exports_chirho
        },
    };

    let module_chirho = ModuleChirho {
        name_chirho: mk_name_chirho("OuterParsec"),
        exports_chirho: Some(vec![ExportSpecChirho::VarChirho(mk_name_chirho("Parsec"))]),
        imports_chirho: vec![ImportDeclChirho {
            package_chirho: None,
            module_chirho: mk_name_chirho("InnerParsec"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        decls_chirho: vec![],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &[imported_iface_chirho])
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("Parsec")
    );
}

#[test]
fn self_re_export_chirho() {
    // module Lib (module Lib) where
    // foo = 1
    let module_chirho = ModuleChirho {
        name_chirho: mk_name_chirho("Lib"),
        exports_chirho: Some(vec![ExportSpecChirho::ModuleChirho(mk_name_chirho("Lib"))]),
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: mk_name_chirho("foo"),
            matches_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let iface_chirho = build_iface_chirho(&module_chirho)
        .expect("small source interface fixture fits constructor projection budget");
    // Self re-export means export all local definitions.
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("foo"),
        "foo should be exported via self re-export"
    );
}

#[test]
fn iface_exports_foreign_import_names_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: mk_name_chirho("ForeignIfaceChirho"),
        exports_chirho: Some(vec![ExportSpecChirho::VarChirho(mk_name_chirho(
            "sinChirho",
        ))]),
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::ForeignDeclChirho {
            direction_chirho: ForeignDirectionChirho::ImportChirho,
            name_chirho: mk_name_chirho("sinChirho"),
            ty_chirho: TypeChirho::ConChirho(mk_name_chirho("Double")),
            calling_conv_chirho: "ccall".to_string(),
            safety_chirho: Some("unsafe".to_string()),
            foreign_name_chirho: Some("sin".to_string()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let iface_chirho = build_iface_chirho(&module_chirho)
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("sinChirho"),
        "foreign import names should be exported through module interfaces"
    );
}

fn family_parent_iface_chirho(module_name_chirho: &str, member_chirho: &str) -> ModuleIfaceChirho {
    let mut exports_chirho = IfaceExportsChirho::default();
    for (name_chirho, is_class_chirho) in [("CChirho", true), (member_chirho, false)] {
        exports_chirho.types_chirho.insert(
            name_chirho.to_owned(),
            IfaceTypeChirho {
                declaration_chirho: None,
                is_class_chirho,
                name_chirho: name_chirho.to_owned(),
                constructors_chirho: Vec::new(),
                methods_chirho: Vec::new(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
    }
    exports_chirho
        .associated_types_chirho
        .insert("CChirho".to_owned(), vec![member_chirho.to_owned()]);
    ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: module_name_chirho.to_owned(),
        exports_chirho,
    }
}

fn family_export_import_chirho(
    module_chirho: &str,
    alias_chirho: &str,
    name_chirho: &str,
) -> haskelujah_ast_chirho::module_chirho::ImportDeclChirho {
    haskelujah_ast_chirho::module_chirho::ImportDeclChirho {
        package_chirho: None,
        module_chirho: mk_name_chirho(module_chirho),
        source_chirho: false,
        qualified_chirho: true,
        alias_chirho: Some(mk_name_chirho(alias_chirho)),
        spec_chirho: Some(haskelujah_ast_chirho::module_chirho::ImportSpecChirho {
            hiding_chirho: false,
            items_chirho: vec![
                haskelujah_ast_chirho::module_chirho::ImportItemChirho::TyConChirho {
                    name_chirho: mk_name_chirho(name_chirho),
                    members_chirho: ExportMembersChirho::NoneChirho,
                },
            ],
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn qualified_type_exports_use_the_selected_parent_and_latest_interface_chirho() {
    let mut module_chirho = mk_module_chirho(
        "UserChirho",
        Some(vec![ExportSpecChirho::TyConChirho {
            name_chirho: NameChirho::RawChirho(RawNameChirho::qualified_chirho(
                "QChirho",
                "CChirho",
                SpanChirho::DUMMY_CHIRHO,
            )),
            members_chirho: ExportMembersChirho::SomeChirho(vec![mk_name_chirho("FChirho")]),
        }]),
        vec![DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("CChirho"),
            type_vars_chirho: Vec::new(),
            rhs_chirho: TypeChirho::ConChirho(mk_name_chirho("Int")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );
    module_chirho.imports_chirho = vec![
        family_export_import_chirho("OriginChirho", "QChirho", "CChirho"),
        family_export_import_chirho("OriginChirho", "RChirho", "FChirho"),
    ];
    let ifaces_chirho = [
        family_parent_iface_chirho("OriginChirho", "OldChirho"),
        family_parent_iface_chirho("OriginChirho", "FChirho"),
    ];
    let resolve_chirho = |module_chirho: &str, name_chirho: &str| {
        if module_chirho == "OriginChirho" && name_chirho == "FChirho" {
            std::collections::BTreeSet::from(["origin-family-chirho".to_owned()])
        } else {
            std::collections::BTreeSet::new()
        }
    };
    let iface_chirho = build_iface_with_imports_and_family_identities_chirho(
        &module_chirho,
        &ifaces_chirho,
        &resolve_chirho,
    )
    .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("FChirho")
    );
    assert!(
        !iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("OldChirho")
    );
    assert_eq!(
        iface_chirho.exports_chirho.associated_types_chirho["CChirho"],
        vec!["FChirho"]
    );
}

#[test]
fn an_associated_namesake_with_another_identity_is_not_exported_chirho() {
    let mut module_chirho = mk_module_chirho(
        "UserChirho",
        Some(vec![ExportSpecChirho::TyConChirho {
            name_chirho: NameChirho::RawChirho(RawNameChirho::qualified_chirho(
                "QChirho",
                "CChirho",
                SpanChirho::DUMMY_CHIRHO,
            )),
            members_chirho: ExportMembersChirho::SomeChirho(vec![mk_name_chirho("FChirho")]),
        }]),
        Vec::new(),
    );
    module_chirho.imports_chirho = vec![
        family_export_import_chirho("OriginChirho", "QChirho", "CChirho"),
        family_export_import_chirho("OtherChirho", "RChirho", "FChirho"),
    ];
    let ifaces_chirho = [
        family_parent_iface_chirho("OriginChirho", "FChirho"),
        family_parent_iface_chirho("OtherChirho", "FChirho"),
    ];
    let resolve_chirho = |module_chirho: &str, _name_chirho: &str| {
        std::collections::BTreeSet::from([format!("{module_chirho}-family-chirho")])
    };
    let iface_chirho = build_iface_with_imports_and_family_identities_chirho(
        &module_chirho,
        &ifaces_chirho,
        &resolve_chirho,
    )
    .expect("small source interface fixture fits constructor projection budget");
    assert!(
        iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("CChirho")
    );
    assert!(
        !iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("FChirho")
    );
}

mod compiler_values_chirho;
