// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn no_deriving_no_instances_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("Void"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
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
    assert!(result_chirho.warnings_chirho.is_empty());
}

#[test]
fn unsupported_class_warns_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("T"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkT"),
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
    assert!(result_chirho.instances_chirho.is_empty());
    assert_eq!(result_chirho.warnings_chirho.len(), 1);
    assert!(result_chirho.warnings_chirho[0].contains("Functor"));
}

#[test]
fn apply_deriving_mutates_module_chirho() {
    let mut module_chirho = make_enum_module_chirho();
    let initial_count_chirho = module_chirho.decls_chirho.len();
    let warnings_chirho = apply_deriving_chirho(&mut module_chirho);

    assert!(warnings_chirho.is_empty());
    // Should have added 3 instances (Eq, Ord, Show) to the decls
    assert_eq!(module_chirho.decls_chirho.len(), initial_count_chirho + 3);
}

#[test]
fn derive_polymorphic_type_adds_context_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("Pair"),
            type_vars_chirho: vec![var_name_chirho("a").into(), var_name_chirho("b").into()],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkPair"),
                fields_chirho: vec![
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::VarChirho(var_name_chirho("a")),
                    ),
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::VarChirho(var_name_chirho("b")),
                    ),
                ],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
            deriving_chirho: unspecified_chirho(&["Eq"]),
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
    assert_eq!(result_chirho.instances_chirho.len(), 1);

    match &result_chirho.instances_chirho[0] {
        DeclChirho::InstanceDeclChirho { context_chirho, .. } => {
            // Should have Eq a, Eq b context
            assert_eq!(context_chirho.len(), 2);
            assert!(matches!(
                &context_chirho[0],
                haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                    class_chirho,
                    ..
                } if class_chirho.text_chirho() == "Eq"
            ));
            assert!(matches!(
                &context_chirho[1],
                haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                    class_chirho,
                    ..
                } if class_chirho.text_chirho() == "Eq"
            ));
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_newtype_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::NewtypeDeclChirho {
            name_chirho: var_name_chirho("Age"),
            type_vars_chirho: vec![],
            constructor_chirho: ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkAge"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::ConChirho(var_name_chirho("Int")),
                )],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            },
            deriving_chirho: unspecified_chirho(&["Eq", "Show"]),
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
    assert_eq!(result_chirho.instances_chirho.len(), 2);
    assert!(result_chirho.warnings_chirho.is_empty());
}

// -------------------------------------------------------------------
// Enum tests
// -------------------------------------------------------------------

#[test]
fn derive_newtype_gnd_chirho() {
    // newtype Age = MkAge Int deriving (Num)
    // Should produce: instance Num Int => Num Age where {}
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::NewtypeDeclChirho {
            name_chirho: var_name_chirho("Age"),
            type_vars_chirho: vec![],
            constructor_chirho: ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkAge"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::ConChirho(var_name_chirho("Int")),
                )],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            },
            deriving_chirho: unspecified_chirho(&["Num"]),
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
            context_chirho,
            types_chirho,
            methods_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "Num");
            // Context should have Num Int
            assert_eq!(context_chirho.len(), 1);
            match &context_chirho[0] {
                haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                    class_chirho,
                    args_chirho,
                    ..
                } => {
                    assert_eq!(class_chirho.text_chirho(), "Num");
                    assert_eq!(args_chirho.len(), 1);
                    match &args_chirho[0] {
                        TypeChirho::ConChirho(n_chirho) => {
                            assert_eq!(n_chirho.text_chirho(), "Int")
                        }
                        other_chirho => {
                            panic!("expected ConChirho(Int), got {:?}", other_chirho)
                        }
                    }
                }
                other_chirho => {
                    panic!("expected simple Num constraint, got {:?}", other_chirho)
                }
            }
            // Instance type should be Age
            assert_eq!(types_chirho.len(), 1);
            match &types_chirho[0] {
                TypeChirho::ConChirho(n_chirho) => assert_eq!(n_chirho.text_chirho(), "Age"),
                other_chirho => panic!("expected ConChirho(Age), got {:?}", other_chirho),
            }
            // Methods empty — filled by dictionary pass
            assert!(methods_chirho.is_empty());
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_newtype_standard_classes_not_gnd_chirho() {
    // newtype Wrapper = MkWrapper Int deriving (Eq, Show)
    // Standard classes should use normal deriving, not GND
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::NewtypeDeclChirho {
            name_chirho: var_name_chirho("Wrapper"),
            type_vars_chirho: vec![],
            constructor_chirho: ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkWrapper"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::ConChirho(var_name_chirho("Int")),
                )],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            },
            deriving_chirho: unspecified_chirho(&["Eq", "Show"]),
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
    assert_eq!(result_chirho.instances_chirho.len(), 2);

    // Both should have methods (standard deriving, not GND)
    for inst_chirho in &result_chirho.instances_chirho {
        match inst_chirho {
            DeclChirho::InstanceDeclChirho { methods_chirho, .. } => {
                assert!(
                    !methods_chirho.is_empty(),
                    "standard deriving should produce methods"
                );
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }
}

#[test]
fn derive_newtype_functor_chirho() {
    // newtype App f a = MkApp (f a) deriving (Functor)
    // Now produces a real fmap implementation (not GND)
    let module_chirho = ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::NewtypeDeclChirho {
            name_chirho: var_name_chirho("App"),
            type_vars_chirho: vec![var_name_chirho("f").into(), var_name_chirho("a").into()],
            constructor_chirho: ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkApp"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::VarChirho(var_name_chirho("f"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(var_name_chirho("a"))),
                        span_chirho: gen_span_chirho(),
                    },
                )],
                span_chirho: gen_span_chirho(),
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            },
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
            // Now generates a real fmap method
            assert_eq!(methods_chirho.len(), 1);
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}
