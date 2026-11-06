// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn make_enum_with_enum_bounded_chirho() -> ModuleChirho {
    ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("Dir"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("North"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("South"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("East"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("West"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
            ],
            deriving_chirho: unspecified_chirho(&["Enum", "Bounded"]),
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
        origin_supply_chirho: Default::default(),
    }
}

#[test]
fn derive_enum_generates_to_from_chirho() {
    let module_chirho = make_enum_with_enum_bounded_chirho();
    let result_chirho = derive_instances_chirho(&module_chirho);
    assert!(result_chirho.warnings_chirho.is_empty());
    assert_eq!(result_chirho.instances_chirho.len(), 2); // Enum + Bounded

    // Enum instance
    let enum_inst_chirho = &result_chirho.instances_chirho[0];
    match enum_inst_chirho {
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            methods_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "Enum");
            assert_eq!(methods_chirho.len(), 4); // toEnum, fromEnum, succ, pred

            // toEnum should have 4 constructor matches + 1 catch-all = 5
            if let LocalBindChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } = &methods_chirho[0]
            {
                assert_eq!(name_chirho.text_chirho(), "toEnum");
                assert_eq!(matches_chirho.len(), 5); // 4 cons + error catch-all
            } else {
                panic!("expected FunBindChirho for toEnum");
            }

            // fromEnum should have 4 matches
            if let LocalBindChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } = &methods_chirho[1]
            {
                assert_eq!(name_chirho.text_chirho(), "fromEnum");
                assert_eq!(matches_chirho.len(), 4);
            } else {
                panic!("expected FunBindChirho for fromEnum");
            }
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_bounded_generates_min_max_chirho() {
    let module_chirho = make_enum_with_enum_bounded_chirho();
    let result_chirho = derive_instances_chirho(&module_chirho);

    let bounded_inst_chirho = &result_chirho.instances_chirho[1];
    match bounded_inst_chirho {
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            methods_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "Bounded");
            assert_eq!(methods_chirho.len(), 2); // minBound, maxBound

            if let LocalBindChirho::FunBindChirho { name_chirho, .. } = &methods_chirho[0] {
                assert_eq!(name_chirho.text_chirho(), "minBound");
            } else {
                panic!("expected FunBindChirho for minBound");
            }

            if let LocalBindChirho::FunBindChirho { name_chirho, .. } = &methods_chirho[1] {
                assert_eq!(name_chirho.text_chirho(), "maxBound");
            } else {
                panic!("expected FunBindChirho for maxBound");
            }
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_enum_rejects_non_nullary_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("T"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkT"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::ConChirho(var_name_chirho("Int")),
                )],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
            deriving_chirho: unspecified_chirho(&["Enum"]),
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
        origin_supply_chirho: Default::default(),
    };
    let result_chirho = derive_instances_chirho(&module_chirho);
    assert!(result_chirho.instances_chirho.is_empty());
    assert_eq!(result_chirho.warnings_chirho.len(), 1);
    assert!(result_chirho.warnings_chirho[0].contains("nullary"));
}

#[test]
fn derive_enum_rejects_polymorphic_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("T"),
            type_vars_chirho: vec![var_name_chirho("a").into()],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkT"),
                fields_chirho: vec![],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
            deriving_chirho: unspecified_chirho(&["Enum"]),
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
        origin_supply_chirho: Default::default(),
    };
    let result_chirho = derive_instances_chirho(&module_chirho);
    assert!(result_chirho.instances_chirho.is_empty());
    assert_eq!(result_chirho.warnings_chirho.len(), 1);
    assert!(result_chirho.warnings_chirho[0].contains("polymorphic"));
}

// -------------------------------------------------------------------
// Read tests
// -------------------------------------------------------------------

#[test]
fn derive_read_enum_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("Color"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Red"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Green"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
            ],
            deriving_chirho: unspecified_chirho(&["Read"]),
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
        origin_supply_chirho: Default::default(),
    };
    let result_chirho = derive_instances_chirho(&module_chirho);
    assert!(result_chirho.warnings_chirho.is_empty());
    assert_eq!(result_chirho.instances_chirho.len(), 1);

    match &result_chirho.instances_chirho[0] {
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            methods_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "Read");
            assert_eq!(methods_chirho.len(), 1); // readsPrec
            if let LocalBindChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } = &methods_chirho[0]
            {
                assert_eq!(name_chirho.text_chirho(), "readsPrec");
                assert_eq!(matches_chirho.len(), 1); // single match arm
                // Two patterns: _ and s
                assert_eq!(matches_chirho[0].pats_chirho.len(), 2);
            } else {
                panic!("expected FunBindChirho for readsPrec");
            }
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_read_polymorphic_adds_context_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("Box"),
            type_vars_chirho: vec![var_name_chirho("a").into()],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkBox"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::VarChirho(var_name_chirho("a")),
                )],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
            deriving_chirho: unspecified_chirho(&["Read"]),
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
        origin_supply_chirho: Default::default(),
    };
    let result_chirho = derive_instances_chirho(&module_chirho);
    assert!(result_chirho.warnings_chirho.is_empty());
    assert_eq!(result_chirho.instances_chirho.len(), 1);

    match &result_chirho.instances_chirho[0] {
        DeclChirho::InstanceDeclChirho { context_chirho, .. } => {
            assert_eq!(context_chirho.len(), 1);
            assert!(matches!(
                &context_chirho[0],
                haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                    class_chirho,
                    ..
                } if class_chirho.text_chirho() == "Read"
            ));
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_all_six_classes_chirho() {
    // A type deriving all supported classes at once
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("Bool2"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("F2"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("T2"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
            ],
            deriving_chirho: unspecified_chirho(&["Eq", "Ord", "Show", "Read", "Enum", "Bounded"]),
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
        origin_supply_chirho: Default::default(),
    };
    let result_chirho = derive_instances_chirho(&module_chirho);
    assert!(result_chirho.warnings_chirho.is_empty());
    assert_eq!(result_chirho.instances_chirho.len(), 6);

    let class_names_chirho: Vec<&str> = result_chirho
        .instances_chirho
        .iter()
        .map(|d_chirho| match d_chirho {
            DeclChirho::InstanceDeclChirho { class_chirho, .. } => class_chirho.text_chirho(),
            _ => panic!("expected InstanceDeclChirho"),
        })
        .collect();
    assert_eq!(
        class_names_chirho,
        vec!["Eq", "Ord", "Show", "Read", "Enum", "Bounded"]
    );
}

// -------------------------------------------------------------------
// Generalized Newtype Deriving (GND) tests
// -------------------------------------------------------------------
