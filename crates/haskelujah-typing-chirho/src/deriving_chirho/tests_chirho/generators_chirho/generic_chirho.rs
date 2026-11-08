// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn derive_generic_enum_chirho() {
    // data Color = Red | Green | Blue deriving (Generic)
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
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Blue"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
            ],
            deriving_chirho: unspecified_chirho(&["Generic"]),
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
            assert_eq!(class_chirho.text_chirho(), "Generic");
            // from + to
            assert_eq!(methods_chirho.len(), 2);
            // from should have 3 match arms (one per constructor)
            match &methods_chirho[0] {
                LocalBindChirho::FunBindChirho { matches_chirho, .. } => {
                    assert_eq!(matches_chirho.len(), 3);
                }
                _ => panic!("expected FunBindChirho for from"),
            }
            // to should have 3 match arms
            match &methods_chirho[1] {
                LocalBindChirho::FunBindChirho { matches_chirho, .. } => {
                    assert_eq!(matches_chirho.len(), 3);
                }
                _ => panic!("expected FunBindChirho for to"),
            }
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_generic_product_chirho() {
    // data Pair = MkPair Int Bool deriving (Generic)
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("Pair"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkPair"),
                fields_chirho: vec![
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::ConChirho(var_name_chirho("Int")),
                    ),
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::ConChirho(var_name_chirho("Bool")),
                    ),
                ],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
            deriving_chirho: unspecified_chirho(&["Generic"]),
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
            assert_eq!(class_chirho.text_chirho(), "Generic");
            assert_eq!(methods_chirho.len(), 2);
            // Single constructor = 1 match arm each
            match &methods_chirho[0] {
                LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    ..
                } => {
                    assert_eq!(name_chirho.text_chirho(), "from");
                    assert_eq!(matches_chirho.len(), 1);
                }
                _ => panic!("expected FunBindChirho for from"),
            }
            match &methods_chirho[1] {
                LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    ..
                } => {
                    assert_eq!(name_chirho.text_chirho(), "to");
                    assert_eq!(matches_chirho.len(), 1);
                }
                _ => panic!("expected FunBindChirho for to"),
            }
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_generic_single_nullary_chirho() {
    // data Unit = MkUnit deriving (Generic)
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("Unit"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkUnit"),
                fields_chirho: vec![],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
            deriving_chirho: unspecified_chirho(&["Generic"]),
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
            assert_eq!(class_chirho.text_chirho(), "Generic");
            assert_eq!(methods_chirho.len(), 2);
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_generic_newtype_chirho() {
    // newtype Wrapper a = MkWrapper a deriving (Generic)
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::NewtypeDeclChirho {
            name_chirho: var_name_chirho("Wrapper"),
            type_vars_chirho: vec![var_name_chirho("a").into()],
            constructor_chirho: Box::new(ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkWrapper"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::VarChirho(var_name_chirho("a")),
                )],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }),
            deriving_chirho: unspecified_chirho(&["Generic"]),
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
            assert_eq!(class_chirho.text_chirho(), "Generic");
            assert_eq!(methods_chirho.len(), 2);
            // Single-field newtype: from (MkWrapper x1) = x1
            match &methods_chirho[0] {
                LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    ..
                } => {
                    assert_eq!(name_chirho.text_chirho(), "from");
                    assert_eq!(matches_chirho.len(), 1);
                }
                _ => panic!("expected FunBindChirho"),
            }
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}
