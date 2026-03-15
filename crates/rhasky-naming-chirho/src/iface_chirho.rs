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
    build_iface_with_imports_chirho(module_chirho, &[])
}

/// Build a module interface from a parsed module, with access to imported
/// module interfaces for handling `module Foo` re-exports in the export list.
pub fn build_iface_with_imports_chirho(
    module_chirho: &ModuleChirho,
    imported_ifaces_chirho: &[ModuleIfaceChirho],
) -> ModuleIfaceChirho {
    let module_name_chirho = module_chirho.name_chirho.text_chirho().to_string();

    // First, collect ALL definitions in the module.
    let all_exports_chirho = collect_all_definitions_chirho(module_chirho);

    // Then filter by the export list.
    let exports_chirho = match &module_chirho.exports_chirho {
        None => {
            // No export list means export everything.
            all_exports_chirho
        }
        Some(specs_chirho) => filter_exports_chirho(
            &all_exports_chirho,
            specs_chirho,
            module_chirho,
            imported_ifaces_chirho,
        ),
    };

    ModuleIfaceChirho {
        name_chirho: module_name_chirho,
        exports_chirho,
    }
}

/// Build synthetic module interfaces for built-in modules like `Data.Map`,
/// `Data.Set`, `Data.List`, `Data.Char`, `Data.Maybe`, `Data.Either`.
/// These are generated at compile time and don't correspond to source files.
pub fn builtin_module_ifaces_chirho() -> Vec<ModuleIfaceChirho> {
    let mut modules_chirho = Vec::new();

    // Helper: create an IfaceValueChirho with DUMMY span
    let mk_val_chirho = |name_chirho: &str| -> (String, IfaceValueChirho) {
        (name_chirho.to_string(), IfaceValueChirho {
            name_chirho: name_chirho.to_string(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        })
    };
    let mk_type_chirho = |name_chirho: &str, cons_chirho: &[&str]| -> (String, IfaceTypeChirho) {
        (name_chirho.to_string(), IfaceTypeChirho {
            name_chirho: name_chirho.to_string(),
            constructors_chirho: cons_chirho.iter().map(|c_chirho| c_chirho.to_string()).collect(),
            methods_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        })
    };

    // Data.Map
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "mapEmpty", "mapInsert", "mapLookup", "mapMember", "mapDelete",
            "mapFromList", "mapToList", "mapSize", "mapKeys", "mapElems",
            "mapFoldlWithKey", "mapInsertWith", "mapFindWithDefault", "mapAdjust",
            "mapUnionWith", "mapMap", "mapFilter", "mapNull",
            "mapInsertStr", "mapLookupStr", "mapMemberStr", "mapDeleteStr",
            "mapFindWithDefaultStr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Map", &["MapEmpty", "MapNode"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        // Also export constructors as values
        for con_chirho in &["MapEmpty", "MapNode"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Map".to_string(),
            exports_chirho,
        });
    }

    // Data.Set
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "setEmpty", "setInsert", "setMember", "setDelete",
            "setFromList", "setToList", "setSize", "setUnion",
            "setIntersection", "setDifference", "setNull",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Set", &["SetEmpty", "SetNode"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["SetEmpty", "SetNode"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Set".to_string(),
            exports_chirho,
        });
    }

    // Data.List
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "map", "filter", "foldr", "foldl", "head", "tail", "last", "init",
            "null", "length", "reverse", "zip", "zipWith", "unzip",
            "take", "drop", "takeWhile", "dropWhile", "span", "break",
            "elem", "notElem", "lookup", "sum", "product", "minimum", "maximum",
            "sort", "insert", "nub", "concat", "concatMap", "any", "all",
            "iterate", "scanl", "partition",
            "sortBy", "insertBy", "nubBy", "maximumBy", "minimumBy",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.List".to_string(),
            exports_chirho,
        });
    }

    // Data.Char
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ord", "chr", "isDigit", "isAlpha", "isAlphaNum", "isUpper", "isLower",
            "isSpace", "toLower", "toUpper", "digitToInt", "intToDigit",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Char".to_string(),
            exports_chirho,
        });
    }

    // Data.Maybe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "maybe", "isJust", "isNothing", "fromMaybe", "fromJust",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Maybe", &["Nothing", "Just"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["Nothing", "Just"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Maybe".to_string(),
            exports_chirho,
        });
    }

    // Data.IORef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newIORef", "readIORef", "writeIORef", "modifyIORef",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IORef".to_string(),
            exports_chirho,
        });
    }

    // Prelude — the implicit import every Haskell module gets
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        // Core types
        for (type_name_chirho, cons_chirho) in &[
            ("Bool", &["False", "True"][..]),
            ("Maybe", &["Nothing", "Just"][..]),
            ("Either", &["Left", "Right"][..]),
            ("Ordering", &["LT", "EQ", "GT"][..]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(type_name_chirho, cons_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            for con_chirho in *cons_chirho {
                let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
                exports_chirho.values_chirho.insert(k_chirho, v_chirho);
            }
        }
        // Standard Prelude functions
        for name_chirho in &[
            // Numeric
            "abs", "signum", "negate", "fromInteger", "fromIntegral", "toInteger",
            "even", "odd", "max", "min", "div", "mod", "quot", "rem",
            "succ", "pred", "toEnum", "fromEnum", "minBound", "maxBound",
            "floor", "ceiling", "round", "truncate",
            // Boolean
            "not", "otherwise",
            // Tuple
            "fst", "snd", "curry", "uncurry",
            // Function
            "id", "const", "flip", "error", "undefined",
            // List
            "map", "filter", "foldr", "foldl", "head", "tail", "last", "init",
            "null", "length", "reverse", "concat", "concatMap",
            "take", "drop", "takeWhile", "dropWhile", "span",
            "zip", "zipWith", "unzip", "elem", "notElem", "lookup",
            "sum", "product", "minimum", "maximum", "any", "all",
            "iterate", "scanl", "words", "unwords", "lines", "unlines",
            // IO
            "putStrLn", "putStr", "putChar", "print",
            "getLine", "getChar", "getContents", "interact",
            "readFile", "writeFile", "appendFile",
            // Show/Read
            "show", "read",
            // Monad/Functor
            "fmap", "return", "mapM_", "sequence_", "when", "unless",
            // Conversion
            "fromString", "fromList", "toList",
            // Comparison
            "compare",
            // String ops
            "intercalate",
            // Maybe/Either
            "maybe", "either", "fromMaybe", "isJust", "isNothing",
            // Data structures
            "sort",
            // Monad transformers
            "StateT", "runStateT", "runState", "evalState", "execState",
            "get", "put", "modify", "bindStateT", "returnStateT",
            "MaybeT", "runMaybeT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Prelude".to_string(),
            exports_chirho,
        });
    }

    modules_chirho
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
/// `module_chirho` is the parsed module (for checking imports of re-exported modules).
/// `imported_ifaces_chirho` provides the interfaces of imported modules for re-exports.
fn filter_exports_chirho(
    all_chirho: &IfaceExportsChirho,
    specs_chirho: &[ExportSpecChirho],
    module_chirho: &ModuleChirho,
    imported_ifaces_chirho: &[ModuleIfaceChirho],
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
            ExportSpecChirho::ModuleChirho(re_export_name_chirho) => {
                let target_mod_chirho = re_export_name_chirho.text_chirho();
                // Check the module is actually imported.
                let is_imported_chirho = module_chirho.imports_chirho.iter().any(|imp_chirho| {
                    imp_chirho.module_chirho.text_chirho() == target_mod_chirho
                });
                // `module M` in the export list of module M itself means
                // "export all local definitions" — this is the self-re-export pattern.
                let is_self_chirho = module_chirho.name_chirho.text_chirho() == target_mod_chirho;

                if is_self_chirho {
                    // Export all local definitions.
                    for (k_chirho, v_chirho) in &all_chirho.values_chirho {
                        result_chirho
                            .values_chirho
                            .insert(k_chirho.clone(), v_chirho.clone());
                    }
                    for (k_chirho, v_chirho) in &all_chirho.types_chirho {
                        result_chirho
                            .types_chirho
                            .insert(k_chirho.clone(), v_chirho.clone());
                    }
                } else if is_imported_chirho {
                    // Find the matching interface and re-export all its names.
                    if let Some(iface_chirho) = imported_ifaces_chirho
                        .iter()
                        .find(|i_chirho| i_chirho.name_chirho == target_mod_chirho)
                    {
                        for (k_chirho, v_chirho) in &iface_chirho.exports_chirho.values_chirho {
                            result_chirho
                                .values_chirho
                                .insert(k_chirho.clone(), v_chirho.clone());
                        }
                        for (k_chirho, v_chirho) in &iface_chirho.exports_chirho.types_chirho {
                            result_chirho
                                .types_chirho
                                .insert(k_chirho.clone(), v_chirho.clone());
                        }
                    }
                }
                // If not imported and not self, silently skip (could warn).
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

    #[test]
    fn module_re_export_chirho() {
        // module Reexporter (module Inner) where
        // import Inner
        // extra = 42
        use rhasky_ast_chirho::module_chirho::ImportDeclChirho;

        // Simulate the Inner module interface.
        let inner_iface_chirho = ModuleIfaceChirho {
            name_chirho: "Inner".to_string(),
            exports_chirho: {
                let mut e_chirho = IfaceExportsChirho::default();
                e_chirho.values_chirho.insert(
                    "innerFn".to_string(),
                    IfaceValueChirho {
                        name_chirho: "innerFn".to_string(),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                );
                e_chirho.types_chirho.insert(
                    "InnerType".to_string(),
                    IfaceTypeChirho {
                        name_chirho: "InnerType".to_string(),
                        constructors_chirho: vec!["MkInner".to_string()],
                        methods_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                );
                e_chirho.values_chirho.insert(
                    "MkInner".to_string(),
                    IfaceValueChirho {
                        name_chirho: "MkInner".to_string(),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
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
                module_chirho: mk_name_chirho("Inner"),
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
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let iface_chirho =
            build_iface_with_imports_chirho(&module_chirho, &[inner_iface_chirho]);

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
    fn self_re_export_chirho() {
        // module Lib (module Lib) where
        // foo = 1
        let module_chirho = ModuleChirho {
            name_chirho: mk_name_chirho("Lib"),
            exports_chirho: Some(vec![
                ExportSpecChirho::ModuleChirho(mk_name_chirho("Lib")),
            ]),
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: mk_name_chirho("foo"),
                matches_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let iface_chirho = build_iface_chirho(&module_chirho);
        // Self re-export means export all local definitions.
        assert!(
            iface_chirho
                .exports_chirho
                .values_chirho
                .contains_key("foo"),
            "foo should be exported via self re-export"
        );
    }
}
