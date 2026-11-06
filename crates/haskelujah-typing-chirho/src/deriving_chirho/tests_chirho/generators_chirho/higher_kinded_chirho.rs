// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn derive_functor_data_chirho() {
    // data Box a = MkBox a deriving (Functor)
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
            deriving_chirho: unspecified_chirho(&["Functor"]),
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
            assert_eq!(class_chirho.text_chirho(), "Functor");
            assert_eq!(methods_chirho.len(), 1);
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_functor_no_type_params_chirho() {
    // data Unit = MkUnit deriving (Functor) — should fail
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
            deriving_chirho: unspecified_chirho(&["Functor"]),
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
    assert_eq!(result_chirho.warnings_chirho.len(), 1);
    assert!(result_chirho.warnings_chirho[0].contains("no type parameters"));
}

#[test]
fn derive_foldable_data_chirho() {
    // data Pair a = MkPair a a deriving (Foldable)
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("Pair"),
            type_vars_chirho: vec![var_name_chirho("a").into()],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkPair"),
                fields_chirho: vec![
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::VarChirho(var_name_chirho("a")),
                    ),
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::VarChirho(var_name_chirho("a")),
                    ),
                ],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
            deriving_chirho: unspecified_chirho(&["Foldable"]),
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
            assert_eq!(class_chirho.text_chirho(), "Foldable");
            assert_eq!(methods_chirho.len(), 1);
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_traversable_data_chirho() {
    // data Maybe2 a = Nothing2 | Just2 a deriving (Traversable)
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("Maybe2"),
            type_vars_chirho: vec![var_name_chirho("a").into()],
            constructors_chirho: vec![
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Nothing2"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Just2"),
                    fields_chirho: vec![(
                        StrictnessChirho::LazyChirho,
                        TypeChirho::VarChirho(var_name_chirho("a")),
                    )],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
            ],
            deriving_chirho: unspecified_chirho(&["Traversable"]),
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
            assert_eq!(class_chirho.text_chirho(), "Traversable");
            assert_eq!(methods_chirho.len(), 1);
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn type_mentions_var_chirho_test() {
    assert!(type_mentions_var_chirho(
        &TypeChirho::VarChirho(var_name_chirho("a")),
        "a"
    ));
    assert!(!type_mentions_var_chirho(
        &TypeChirho::VarChirho(var_name_chirho("b")),
        "a"
    ));
    assert!(type_mentions_var_chirho(
        &TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::ConChirho(var_name_chirho("Maybe"))),
            arg_chirho: Box::new(TypeChirho::VarChirho(var_name_chirho("a"))),
            span_chirho: gen_span_chirho(),
        },
        "a"
    ));
    assert!(!type_mentions_var_chirho(
        &TypeChirho::ConChirho(var_name_chirho("Int")),
        "a"
    ));
}
