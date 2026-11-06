// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

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
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
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
