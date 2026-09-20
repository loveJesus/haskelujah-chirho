// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_ast_chirho::decl_chirho::StrictnessChirho;

fn make_enum_module_chirho() -> ModuleChirho {
    ModuleChirho {
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
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Green"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Blue"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                },
            ],
            deriving_chirho: vec![
                TypeChirho::ConChirho(var_name_chirho("Eq")),
                TypeChirho::ConChirho(var_name_chirho("Ord")),
                TypeChirho::ConChirho(var_name_chirho("Show")),
            ],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
    }
}

fn make_product_module_chirho() -> ModuleChirho {
    ModuleChirho {
        name_chirho: var_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: var_name_chirho("Point"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkPoint"),
                fields_chirho: vec![
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::ConChirho(var_name_chirho("Int")),
                    ),
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::ConChirho(var_name_chirho("Int")),
                    ),
                ],
                span_chirho: gen_span_chirho(),
            }],
            deriving_chirho: vec![
                TypeChirho::ConChirho(var_name_chirho("Eq")),
                TypeChirho::ConChirho(var_name_chirho("Show")),
            ],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
    }
}

#[test]
fn derive_eq_enum_chirho() {
    let module_chirho = make_enum_module_chirho();
    let result_chirho = derive_instances_chirho(&module_chirho);

    // Should generate Eq, Ord, Show
    assert_eq!(result_chirho.instances_chirho.len(), 3);

    // First should be Eq
    let eq_inst_chirho = &result_chirho.instances_chirho[0];
    match eq_inst_chirho {
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            methods_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "Eq");
            assert_eq!(methods_chirho.len(), 1);
            // Should have 4 match arms: Red==Red, Green==Green, Blue==Blue, _ _ = False
            if let LocalBindChirho::FunBindChirho { matches_chirho, .. } = &methods_chirho[0] {
                assert_eq!(matches_chirho.len(), 4); // 3 constructors + catch-all
            } else {
                panic!("expected FunBindChirho");
            }
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_show_enum_chirho() {
    let module_chirho = make_enum_module_chirho();
    let result_chirho = derive_instances_chirho(&module_chirho);

    let show_inst_chirho = &result_chirho.instances_chirho[2];
    match show_inst_chirho {
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            methods_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "Show");
            assert_eq!(methods_chirho.len(), 1);
            // 3 match arms: show Red = "Red", show Green = "Green", show Blue = "Blue"
            if let LocalBindChirho::FunBindChirho { matches_chirho, .. } = &methods_chirho[0] {
                assert_eq!(matches_chirho.len(), 3);
            } else {
                panic!("expected FunBindChirho");
            }
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_ord_enum_chirho() {
    let module_chirho = make_enum_module_chirho();
    let result_chirho = derive_instances_chirho(&module_chirho);

    let ord_inst_chirho = &result_chirho.instances_chirho[1];
    match ord_inst_chirho {
        DeclChirho::InstanceDeclChirho { class_chirho, .. } => {
            assert_eq!(class_chirho.text_chirho(), "Ord");
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_eq_product_type_chirho() {
    let module_chirho = make_product_module_chirho();
    let result_chirho = derive_instances_chirho(&module_chirho);

    // Eq and Show
    assert_eq!(result_chirho.instances_chirho.len(), 2);

    let eq_inst_chirho = &result_chirho.instances_chirho[0];
    match eq_inst_chirho {
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            methods_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "Eq");
            // Single constructor, no catch-all needed
            if let LocalBindChirho::FunBindChirho { matches_chirho, .. } = &methods_chirho[0] {
                assert_eq!(matches_chirho.len(), 1); // just MkPoint a1 a2 == MkPoint b1 b2
                // Body should have a1 == b1 && a2 == b2
                let arm_chirho = &matches_chirho[0];
                assert_eq!(arm_chirho.pats_chirho.len(), 2);
            } else {
                panic!("expected FunBindChirho");
            }
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

#[test]
fn derive_show_product_type_chirho() {
    let module_chirho = make_product_module_chirho();
    let result_chirho = derive_instances_chirho(&module_chirho);

    let show_inst_chirho = &result_chirho.instances_chirho[1];
    match show_inst_chirho {
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            methods_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "Show");
            if let LocalBindChirho::FunBindChirho { matches_chirho, .. } = &methods_chirho[0] {
                assert_eq!(matches_chirho.len(), 1);
            } else {
                panic!("expected FunBindChirho");
            }
        }
        _ => panic!("expected InstanceDeclChirho"),
    }
}

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
            }],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Functor"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            }],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Eq"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            },
            deriving_chirho: vec![
                TypeChirho::ConChirho(var_name_chirho("Eq")),
                TypeChirho::ConChirho(var_name_chirho("Show")),
            ],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };

    let result_chirho = derive_instances_chirho(&module_chirho);
    assert_eq!(result_chirho.instances_chirho.len(), 2);
    assert!(result_chirho.warnings_chirho.is_empty());
}

// -------------------------------------------------------------------
// Enum tests
// -------------------------------------------------------------------

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
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("South"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("East"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("West"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                },
            ],
            deriving_chirho: vec![
                TypeChirho::ConChirho(var_name_chirho("Enum")),
                TypeChirho::ConChirho(var_name_chirho("Bounded")),
            ],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            }],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Enum"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            }],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Enum"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Green"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                },
            ],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Read"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            }],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Read"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("T2"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                },
            ],
            deriving_chirho: vec![
                TypeChirho::ConChirho(var_name_chirho("Eq")),
                TypeChirho::ConChirho(var_name_chirho("Ord")),
                TypeChirho::ConChirho(var_name_chirho("Show")),
                TypeChirho::ConChirho(var_name_chirho("Read")),
                TypeChirho::ConChirho(var_name_chirho("Enum")),
                TypeChirho::ConChirho(var_name_chirho("Bounded")),
            ],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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

#[test]
fn derive_newtype_gnd_chirho() {
    // newtype Age = MkAge Int deriving (Num)
    // Should produce: instance Num Int => Num Age where {}
    let mut module_chirho = ModuleChirho {
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
            },
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Num"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };
    let result_chirho = crate::kind_chirho::infer_module_kinds_with_deriving_chirho(
        &mut module_chirho,
        &std::collections::HashMap::new(),
    );
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "{:?}",
        result_chirho.diagnostics_chirho
    );
    assert_eq!(module_chirho.decls_chirho.len(), 2);

    match &module_chirho.decls_chirho[1] {
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
            },
            deriving_chirho: vec![
                TypeChirho::ConChirho(var_name_chirho("Eq")),
                TypeChirho::ConChirho(var_name_chirho("Show")),
            ],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            },
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Functor"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            }],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Functor"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            }],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Functor"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            }],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Foldable"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Just2"),
                    fields_chirho: vec![(
                        StrictnessChirho::LazyChirho,
                        TypeChirho::VarChirho(var_name_chirho("a")),
                    )],
                    span_chirho: gen_span_chirho(),
                },
            ],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Traversable"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Green"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Blue"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                },
            ],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Generic"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            }],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Generic"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            }],
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Generic"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
            constructor_chirho: ConDeclChirho::OrdinaryChirho {
                name_chirho: var_name_chirho("MkWrapper"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::VarChirho(var_name_chirho("a")),
                )],
                span_chirho: gen_span_chirho(),
            },
            deriving_chirho: vec![TypeChirho::ConChirho(var_name_chirho("Generic"))],
            kind_sig_chirho: None,
            span_chirho: gen_span_chirho(),
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
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
