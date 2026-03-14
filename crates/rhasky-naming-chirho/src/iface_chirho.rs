// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Module interfaces
//!
//! A `ModuleIfaceChirho` represents the public API of a compiled module — the
//! names (values, types, constructors) that other modules may import from it.
//! Building an interface applies the module's export list to its declarations.

use std::collections::HashMap;

use rhasky_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use rhasky_ast_chirho::module_chirho::{ExportMembersChirho, ExportSpecChirho, ModuleChirho};
use rhasky_span_chirho::SpanChirho;

// ---------------------------------------------------------------------------
// Interface types
// ---------------------------------------------------------------------------

/// An exported value (function, variable, data constructor).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfaceValueChirho {
    pub name_chirho: String,
    pub span_chirho: SpanChirho,
}

/// An exported type with its available constructors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfaceTypeChirho {
    pub name_chirho: String,
    /// Constructor names that are exported alongside this type.
    pub constructors_chirho: Vec<String>,
    /// Class method names that are exported alongside this class.
    pub methods_chirho: Vec<String>,
    pub span_chirho: SpanChirho,
}

/// Everything a module exports.
#[derive(Debug, Clone, Default)]
pub struct IfaceExportsChirho {
    /// Value-level exports: functions, variables, data constructors.
    pub values_chirho: HashMap<String, IfaceValueChirho>,
    /// Type-level exports: type constructors, type classes.
    pub types_chirho: HashMap<String, IfaceTypeChirho>,
}

/// A compiled module's public interface.
#[derive(Debug, Clone)]
pub struct ModuleIfaceChirho {
    /// Fully qualified module name.
    pub name_chirho: String,
    /// The exported names.
    pub exports_chirho: IfaceExportsChirho,
}

// ---------------------------------------------------------------------------
// Building an interface from a module
// ---------------------------------------------------------------------------

/// Build a module interface from a parsed module. Applies the export list
/// to determine which names are publicly visible.
pub fn build_iface_chirho(module_chirho: &ModuleChirho) -> ModuleIfaceChirho {
    let module_name_chirho = module_chirho.name_chirho.text_chirho().to_string();

    // First, collect ALL definitions in the module.
    let all_exports_chirho = collect_all_definitions_chirho(module_chirho);

    // Then filter by the export list.
    let exports_chirho = match &module_chirho.exports_chirho {
        None => {
            // No export list means export everything.
            all_exports_chirho
        }
        Some(specs_chirho) => filter_exports_chirho(&all_exports_chirho, specs_chirho),
    };

    ModuleIfaceChirho {
        name_chirho: module_name_chirho,
        exports_chirho,
    }
}

/// Collect all definitions (types + values) from a module's declarations.
fn collect_all_definitions_chirho(module_chirho: &ModuleChirho) -> IfaceExportsChirho {
    let mut exports_chirho = IfaceExportsChirho::default();

    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                span_chirho,
                ..
            } => {
                let type_name_chirho = name_chirho.text_chirho().to_string();
                let con_names_chirho: Vec<String> = constructors_chirho
                    .iter()
                    .map(|c_chirho| con_decl_name_chirho(c_chirho).to_string())
                    .collect();

                // Export each constructor as a value
                for cn_chirho in &con_names_chirho {
                    exports_chirho.values_chirho.insert(
                        cn_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho: cn_chirho.clone(),
                            span_chirho: *span_chirho,
                        },
                    );
                }

                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: type_name_chirho,
                        constructors_chirho: con_names_chirho,
                        methods_chirho: vec![],
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                constructor_chirho,
                span_chirho,
                ..
            } => {
                let type_name_chirho = name_chirho.text_chirho().to_string();
                let con_name_chirho = con_decl_name_chirho(constructor_chirho).to_string();

                exports_chirho.values_chirho.insert(
                    con_name_chirho.clone(),
                    IfaceValueChirho {
                        name_chirho: con_name_chirho.clone(),
                        span_chirho: *span_chirho,
                    },
                );

                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: type_name_chirho,
                        constructors_chirho: vec![con_name_chirho],
                        methods_chirho: vec![],
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                let type_name_chirho = name_chirho.text_chirho().to_string();
                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: type_name_chirho,
                        constructors_chirho: vec![],
                        methods_chirho: vec![],
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::ClassDeclChirho {
                name_chirho,
                methods_chirho,
                span_chirho,
                ..
            } => {
                let class_name_chirho = name_chirho.text_chirho().to_string();
                let method_names_chirho: Vec<String> = methods_chirho
                    .iter()
                    .map(|m_chirho| m_chirho.name_chirho.text_chirho().to_string())
                    .collect();

                // Export each method as a value
                for mn_chirho in &method_names_chirho {
                    exports_chirho.values_chirho.insert(
                        mn_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho: mn_chirho.clone(),
                            span_chirho: *span_chirho,
                        },
                    );
                }

                exports_chirho.types_chirho.insert(
                    class_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: class_name_chirho,
                        constructors_chirho: vec![],
                        methods_chirho: method_names_chirho,
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::FunBindChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                let fn_name_chirho = name_chirho.text_chirho().to_string();
                exports_chirho.values_chirho.insert(
                    fn_name_chirho.clone(),
                    IfaceValueChirho {
                        name_chirho: fn_name_chirho,
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::PatBindChirho { .. } => {
                // Pattern bindings are not exported by name
            }
            DeclChirho::TypeSigChirho { .. }
            | DeclChirho::InstanceDeclChirho { .. }
            | DeclChirho::FixityDeclChirho { .. }
            | DeclChirho::DefaultDeclChirho { .. }
            | DeclChirho::ForeignDeclChirho { .. } => {}
        }
    }

    exports_chirho
}

/// Apply an explicit export list to filter the module's definitions.
fn filter_exports_chirho(
    all_chirho: &IfaceExportsChirho,
    specs_chirho: &[ExportSpecChirho],
) -> IfaceExportsChirho {
    let mut result_chirho = IfaceExportsChirho::default();

    for spec_chirho in specs_chirho {
        match spec_chirho {
            ExportSpecChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                if let Some(val_chirho) = all_chirho.values_chirho.get(text_chirho) {
                    result_chirho
                        .values_chirho
                        .insert(text_chirho.to_string(), val_chirho.clone());
                }
                // A bare name in an export list can also refer to a type
                if let Some(ty_chirho) = all_chirho.types_chirho.get(text_chirho) {
                    result_chirho.types_chirho.insert(
                        text_chirho.to_string(),
                        IfaceTypeChirho {
                            name_chirho: ty_chirho.name_chirho.clone(),
                            constructors_chirho: vec![], // bare name = no constructors
                            methods_chirho: vec![],
                            span_chirho: ty_chirho.span_chirho,
                        },
                    );
                }
            }
            ExportSpecChirho::TyConChirho {
                name_chirho,
                members_chirho,
            } => {
                let text_chirho = name_chirho.text_chirho();
                if let Some(ty_chirho) = all_chirho.types_chirho.get(text_chirho) {
                    let (cons_chirho, methods_chirho) = match members_chirho {
                        ExportMembersChirho::AllChirho => (
                            ty_chirho.constructors_chirho.clone(),
                            ty_chirho.methods_chirho.clone(),
                        ),
                        ExportMembersChirho::SomeChirho(names_chirho) => {
                            let selected_chirho: Vec<String> = names_chirho
                                .iter()
                                .map(|n_chirho| n_chirho.text_chirho().to_string())
                                .collect();
                            let sel_cons_chirho: Vec<String> = ty_chirho
                                .constructors_chirho
                                .iter()
                                .filter(|c_chirho| selected_chirho.contains(c_chirho))
                                .cloned()
                                .collect();
                            let sel_methods_chirho: Vec<String> = ty_chirho
                                .methods_chirho
                                .iter()
                                .filter(|m_chirho| selected_chirho.contains(m_chirho))
                                .cloned()
                                .collect();
                            (sel_cons_chirho, sel_methods_chirho)
                        }
                        ExportMembersChirho::NoneChirho => (vec![], vec![]),
                    };

                    // Export the selected constructors/methods as values
                    for cn_chirho in &cons_chirho {
                        if let Some(val_chirho) = all_chirho.values_chirho.get(cn_chirho) {
                            result_chirho
                                .values_chirho
                                .insert(cn_chirho.clone(), val_chirho.clone());
                        }
                    }
                    for mn_chirho in &methods_chirho {
                        if let Some(val_chirho) = all_chirho.values_chirho.get(mn_chirho) {
                            result_chirho
                                .values_chirho
                                .insert(mn_chirho.clone(), val_chirho.clone());
                        }
                    }

                    result_chirho.types_chirho.insert(
                        text_chirho.to_string(),
                        IfaceTypeChirho {
                            name_chirho: ty_chirho.name_chirho.clone(),
                            constructors_chirho: cons_chirho,
                            methods_chirho,
                            span_chirho: ty_chirho.span_chirho,
                        },
                    );
                }
            }
            ExportSpecChirho::ModuleChirho(_re_export_chirho) => {
                // Re-export entire module — would need the imported module's
                // interface. For now, skip.
            }
        }
    }

    result_chirho
}

fn con_decl_name_chirho(decl_chirho: &ConDeclChirho) -> &str {
    match decl_chirho {
        ConDeclChirho::OrdinaryChirho { name_chirho, .. }
        | ConDeclChirho::RecordChirho { name_chirho, .. } => name_chirho.text_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhasky_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

    fn mk_name_chirho(s_chirho: &str) -> NameChirho {
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
            span_chirho: SpanChirho::DUMMY_CHIRHO,
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
                        },
                        ConDeclChirho::OrdinaryChirho {
                            name_chirho: mk_name_chirho("Blue"),
                            fields_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    deriving_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: mk_name_chirho("paint"),
                    matches_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert_eq!(iface_chirho.name_chirho, "Lib");
        assert!(iface_chirho.exports_chirho.types_chirho.contains_key("Color"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("Red"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("Blue"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("paint"));
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
                    }],
                    deriving_chirho: vec![],
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

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert!(iface_chirho.exports_chirho.types_chirho.contains_key("Color"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("Red"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("paint"));
        // helper is NOT exported
        assert!(!iface_chirho.exports_chirho.values_chirho.contains_key("helper"));
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
                }],
                deriving_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert!(iface_chirho.exports_chirho.types_chirho.contains_key("Color"));
        // Red is NOT exported — only the type name
        assert!(!iface_chirho.exports_chirho.values_chirho.contains_key("Red"));
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
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: mk_name_chirho("Blue"),
                        fields_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                deriving_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("Red"));
        assert!(!iface_chirho.exports_chirho.values_chirho.contains_key("Blue"));
        let ty_chirho = &iface_chirho.exports_chirho.types_chirho["Color"];
        assert_eq!(ty_chirho.constructors_chirho, vec!["Red"]);
    }

    #[test]
    fn class_methods_exported_chirho() {
        let module_chirho = mk_module_chirho(
            "Lib",
            None,
            vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: mk_name_chirho("Show"),
                type_vars_chirho: vec![],
                methods_chirho: vec![rhasky_ast_chirho::decl_chirho::ClassMethodChirho {
                    name_chirho: mk_name_chirho("show"),
                    ty_chirho: rhasky_ast_chirho::ty_chirho::TypeChirho::VarChirho(
                        mk_name_chirho("a"),
                    ),
                    default_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert!(iface_chirho.exports_chirho.types_chirho.contains_key("Show"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("show"));
        assert_eq!(
            iface_chirho.exports_chirho.types_chirho["Show"].methods_chirho,
            vec!["show"]
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
                constructor_chirho: ConDeclChirho::OrdinaryChirho {
                    name_chirho: mk_name_chirho("Wrap"),
                    fields_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                deriving_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert!(iface_chirho.exports_chirho.types_chirho.contains_key("Wrapper"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("Wrap"));
    }
}
