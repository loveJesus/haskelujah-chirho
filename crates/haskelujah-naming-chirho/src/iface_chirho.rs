// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Module interfaces
//!
//! A `ModuleIfaceChirho` represents the public API of a compiled module — the
//! names (values, types, constructors) that other modules may import from it.
//! Building an interface applies the module's export list to its declarations.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ExportSpecChirho, ImportDeclChirho, ImportItemChirho, ImportSpecChirho,
    ModuleChirho,
};
use haskelujah_span_chirho::SpanChirho;

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
    /// Associated value names exported alongside this type or class.
    /// For classes this is method names; for record types/newtypes this includes
    /// field selector functions.
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

fn canonical_value_name_chirho(name_chirho: &str) -> String {
    if name_chirho.starts_with('(') && name_chirho.ends_with(')') && name_chirho.len() > 2 {
        let inner_chirho = &name_chirho[1..name_chirho.len() - 1];
        if !inner_chirho.is_empty()
            && inner_chirho
                .chars()
                .all(|char_chirho| !char_chirho.is_alphanumeric() && char_chirho != '_')
        {
            return inner_chirho.to_string();
        }
    }
    name_chirho.to_string()
}

fn builtin_class_methods_chirho(class_name_chirho: &str) -> Option<&'static [&'static str]> {
    match class_name_chirho {
        "Monad" => Some(&["return", ">>=", ">>"]),
        "Functor" => Some(&["fmap", "<$"]),
        "Applicative" => Some(&["pure", "<*>", "*>", "<*"]),
        "Foldable" => Some(&[
            "foldr", "foldl", "foldMap", "length", "null", "elem", "sum", "product", "maximum",
            "minimum", "toList",
        ]),
        "Traversable" => Some(&["traverse", "sequenceA", "mapM", "sequence"]),
        "Monoid" => Some(&["mempty", "mappend", "mconcat"]),
        "Semigroup" => Some(&["<>"]),
        _ => None,
    }
}

fn normalize_builtin_class_exports_chirho(modules_chirho: &mut [ModuleIfaceChirho]) {
    for module_chirho in modules_chirho {
        let class_names_chirho: Vec<String> = module_chirho
            .exports_chirho
            .types_chirho
            .keys()
            .cloned()
            .collect();
        for class_name_chirho in class_names_chirho {
            let Some(methods_chirho) = builtin_class_methods_chirho(&class_name_chirho) else {
                continue;
            };
            if let Some(ty_chirho) = module_chirho
                .exports_chirho
                .types_chirho
                .get_mut(&class_name_chirho)
            {
                for method_chirho in methods_chirho {
                    let method_name_chirho = method_chirho.to_string();
                    if !ty_chirho.methods_chirho.contains(&method_name_chirho) {
                        ty_chirho.methods_chirho.push(method_name_chirho.clone());
                    }
                    module_chirho
                        .exports_chirho
                        .values_chirho
                        .entry(method_name_chirho.clone())
                        .or_insert_with(|| IfaceValueChirho {
                            name_chirho: method_name_chirho,
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        });
                }
            }
        }
    }
}

fn merge_imported_exports_chirho(
    result_chirho: &mut IfaceExportsChirho,
    iface_exports_chirho: &IfaceExportsChirho,
    spec_chirho: &Option<ImportSpecChirho>,
) {
    match spec_chirho {
        None => {
            for (name_chirho, value_chirho) in &iface_exports_chirho.values_chirho {
                result_chirho
                    .values_chirho
                    .insert(name_chirho.clone(), value_chirho.clone());
            }
            for (name_chirho, ty_chirho) in &iface_exports_chirho.types_chirho {
                result_chirho
                    .types_chirho
                    .insert(name_chirho.clone(), ty_chirho.clone());
            }
        }
        Some(spec_chirho) if spec_chirho.hiding_chirho => {
            merge_imported_exports_chirho(result_chirho, iface_exports_chirho, &None);
            for item_chirho in &spec_chirho.items_chirho {
                remove_import_item_from_exports_chirho(
                    result_chirho,
                    iface_exports_chirho,
                    item_chirho,
                );
            }
        }
        Some(spec_chirho) => {
            for item_chirho in &spec_chirho.items_chirho {
                merge_import_item_into_exports_chirho(
                    result_chirho,
                    iface_exports_chirho,
                    item_chirho,
                );
            }
        }
    }
}

fn merge_import_item_into_exports_chirho(
    result_chirho: &mut IfaceExportsChirho,
    iface_exports_chirho: &IfaceExportsChirho,
    item_chirho: &ImportItemChirho,
) {
    match item_chirho {
        ImportItemChirho::VarChirho(name_chirho) => {
            let value_name_chirho = canonical_value_name_chirho(name_chirho.text_chirho());
            if let Some(value_chirho) = iface_exports_chirho.values_chirho.get(&value_name_chirho) {
                result_chirho
                    .values_chirho
                    .insert(value_name_chirho, value_chirho.clone());
            }
        }
        ImportItemChirho::TyConChirho {
            name_chirho,
            members_chirho,
        } => {
            let ty_name_chirho = name_chirho.text_chirho();
            if let Some(ty_chirho) = iface_exports_chirho.types_chirho.get(ty_name_chirho) {
                let (constructors_chirho, methods_chirho) = match members_chirho {
                    ExportMembersChirho::AllChirho => (
                        ty_chirho.constructors_chirho.clone(),
                        ty_chirho.methods_chirho.clone(),
                    ),
                    ExportMembersChirho::SomeChirho(names_chirho) => {
                        let selected_names_chirho: Vec<String> = names_chirho
                            .iter()
                            .map(|name_chirho| {
                                canonical_value_name_chirho(name_chirho.text_chirho())
                            })
                            .collect();
                        let constructors_chirho = ty_chirho
                            .constructors_chirho
                            .iter()
                            .filter(|name_chirho| selected_names_chirho.contains(name_chirho))
                            .cloned()
                            .collect();
                        let methods_chirho = ty_chirho
                            .methods_chirho
                            .iter()
                            .filter(|name_chirho| selected_names_chirho.contains(name_chirho))
                            .cloned()
                            .collect();
                        (constructors_chirho, methods_chirho)
                    }
                    ExportMembersChirho::NoneChirho => (vec![], vec![]),
                };
                for constructor_chirho in &constructors_chirho {
                    if let Some(value_chirho) =
                        iface_exports_chirho.values_chirho.get(constructor_chirho)
                    {
                        result_chirho
                            .values_chirho
                            .insert(constructor_chirho.clone(), value_chirho.clone());
                    }
                }
                for method_chirho in &methods_chirho {
                    if let Some(value_chirho) =
                        iface_exports_chirho.values_chirho.get(method_chirho)
                    {
                        result_chirho
                            .values_chirho
                            .insert(method_chirho.clone(), value_chirho.clone());
                    }
                }
                result_chirho.types_chirho.insert(
                    ty_name_chirho.to_string(),
                    IfaceTypeChirho {
                        name_chirho: ty_chirho.name_chirho.clone(),
                        constructors_chirho,
                        methods_chirho,
                        span_chirho: ty_chirho.span_chirho,
                    },
                );
            }
        }
    }
}

fn remove_import_item_from_exports_chirho(
    result_chirho: &mut IfaceExportsChirho,
    iface_exports_chirho: &IfaceExportsChirho,
    item_chirho: &ImportItemChirho,
) {
    match item_chirho {
        ImportItemChirho::VarChirho(name_chirho) => {
            let value_name_chirho = canonical_value_name_chirho(name_chirho.text_chirho());
            result_chirho.values_chirho.remove(&value_name_chirho);
        }
        ImportItemChirho::TyConChirho {
            name_chirho,
            members_chirho,
        } => {
            let ty_name_chirho = name_chirho.text_chirho();
            match members_chirho {
                ExportMembersChirho::NoneChirho => {
                    result_chirho.types_chirho.remove(ty_name_chirho);
                }
                ExportMembersChirho::AllChirho => {
                    result_chirho.types_chirho.remove(ty_name_chirho);
                    if let Some(ty_chirho) = iface_exports_chirho.types_chirho.get(ty_name_chirho) {
                        for constructor_chirho in &ty_chirho.constructors_chirho {
                            result_chirho.values_chirho.remove(constructor_chirho);
                        }
                        for method_chirho in &ty_chirho.methods_chirho {
                            result_chirho.values_chirho.remove(method_chirho);
                        }
                    }
                }
                ExportMembersChirho::SomeChirho(names_chirho) => {
                    let selected_names_chirho: Vec<String> = names_chirho
                        .iter()
                        .map(|name_chirho| canonical_value_name_chirho(name_chirho.text_chirho()))
                        .collect();
                    if let Some(ty_chirho) = result_chirho.types_chirho.get_mut(ty_name_chirho) {
                        ty_chirho
                            .constructors_chirho
                            .retain(|name_chirho| !selected_names_chirho.contains(name_chirho));
                        ty_chirho
                            .methods_chirho
                            .retain(|name_chirho| !selected_names_chirho.contains(name_chirho));
                    }
                    for selected_name_chirho in selected_names_chirho {
                        result_chirho.values_chirho.remove(&selected_name_chirho);
                    }
                }
            }
        }
    }
}

fn imported_value_in_scope_chirho(
    module_chirho: &ModuleChirho,
    imported_ifaces_chirho: &[ModuleIfaceChirho],
    value_name_chirho: &str,
) -> Option<IfaceValueChirho> {
    for import_chirho in &module_chirho.imports_chirho {
        if import_chirho.qualified_chirho {
            continue;
        }
        let Some(iface_chirho) = imported_ifaces_chirho.iter().find(|iface_chirho| {
            iface_chirho.name_chirho == import_chirho.module_chirho.full_name_chirho()
        }) else {
            continue;
        };
        let imported_names_chirho = crate::resolve_chirho::compute_imported_names_chirho(
            &iface_chirho.exports_chirho,
            &import_chirho.spec_chirho,
        );
        if imported_names_chirho
            .iter()
            .any(|(name_chirho, namespace_chirho, _span_chirho)| {
                name_chirho == value_name_chirho
                    && *namespace_chirho == crate::env_chirho::NamespaceChirho::ValueChirho
            })
        {
            if let Some(value_chirho) = iface_chirho
                .exports_chirho
                .values_chirho
                .get(value_name_chirho)
            {
                return Some(value_chirho.clone());
            }
        }
    }
    None
}

fn imported_type_in_scope_chirho(
    module_chirho: &ModuleChirho,
    imported_ifaces_chirho: &[ModuleIfaceChirho],
    type_name_chirho: &str,
    members_chirho: &ExportMembersChirho,
) -> Option<(IfaceTypeChirho, Vec<IfaceValueChirho>)> {
    for import_chirho in &module_chirho.imports_chirho {
        if import_chirho.qualified_chirho {
            continue;
        }
        let Some(iface_chirho) = imported_ifaces_chirho.iter().find(|iface_chirho| {
            iface_chirho.name_chirho == import_chirho.module_chirho.full_name_chirho()
        }) else {
            continue;
        };
        let imported_names_chirho = crate::resolve_chirho::compute_imported_names_chirho(
            &iface_chirho.exports_chirho,
            &import_chirho.spec_chirho,
        );
        let imported_type_visible_chirho =
            imported_names_chirho
                .iter()
                .any(|(name_chirho, namespace_chirho, _span_chirho)| {
                    name_chirho == type_name_chirho
                        && *namespace_chirho == crate::env_chirho::NamespaceChirho::TypeChirho
                });
        if !imported_type_visible_chirho {
            continue;
        }
        let Some(ty_chirho) = iface_chirho
            .exports_chirho
            .types_chirho
            .get(type_name_chirho)
        else {
            continue;
        };
        let (constructors_chirho, methods_chirho) = match members_chirho {
            ExportMembersChirho::AllChirho => (
                ty_chirho.constructors_chirho.clone(),
                ty_chirho.methods_chirho.clone(),
            ),
            ExportMembersChirho::SomeChirho(names_chirho) => {
                let selected_names_chirho: Vec<String> = names_chirho
                    .iter()
                    .map(|name_chirho| canonical_value_name_chirho(name_chirho.text_chirho()))
                    .collect();
                let constructors_chirho = ty_chirho
                    .constructors_chirho
                    .iter()
                    .filter(|name_chirho| selected_names_chirho.contains(name_chirho))
                    .cloned()
                    .collect();
                let methods_chirho = ty_chirho
                    .methods_chirho
                    .iter()
                    .filter(|name_chirho| selected_names_chirho.contains(name_chirho))
                    .cloned()
                    .collect();
                (constructors_chirho, methods_chirho)
            }
            ExportMembersChirho::NoneChirho => (vec![], vec![]),
        };
        let mut accompanying_values_chirho = Vec::new();
        for constructor_name_chirho in &constructors_chirho {
            if let Some(value_chirho) = iface_chirho
                .exports_chirho
                .values_chirho
                .get(constructor_name_chirho)
            {
                accompanying_values_chirho.push(value_chirho.clone());
            }
        }
        for method_name_chirho in &methods_chirho {
            if let Some(value_chirho) = iface_chirho
                .exports_chirho
                .values_chirho
                .get(method_name_chirho)
            {
                accompanying_values_chirho.push(value_chirho.clone());
            }
        }
        return Some((
            IfaceTypeChirho {
                name_chirho: ty_chirho.name_chirho.clone(),
                constructors_chirho,
                methods_chirho,
                span_chirho: ty_chirho.span_chirho,
            },
            accompanying_values_chirho,
        ));
    }
    None
}

fn con_decl_field_names_chirho(decl_chirho: &ConDeclChirho) -> Vec<String> {
    match decl_chirho {
        ConDeclChirho::RecordChirho { fields_chirho, .. } => fields_chirho
            .iter()
            .flat_map(|field_chirho| {
                field_chirho
                    .names_chirho
                    .iter()
                    .map(|name_chirho| name_chirho.text_chirho().to_string())
                    .collect::<Vec<_>>()
            })
            .collect(),
        _ => vec![],
    }
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
    // Use full_name_chirho to preserve qualified module names (e.g., "Sub.Helper"
    // instead of just "Helper"). This is critical for hierarchical module imports.
    let module_name_chirho = module_chirho.name_chirho.full_name_chirho();

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
        (
            name_chirho.to_string(),
            IfaceValueChirho {
                name_chirho: name_chirho.to_string(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        )
    };
    let mk_type_chirho = |name_chirho: &str, cons_chirho: &[&str]| -> (String, IfaceTypeChirho) {
        (
            name_chirho.to_string(),
            IfaceTypeChirho {
                name_chirho: name_chirho.to_string(),
                constructors_chirho: cons_chirho
                    .iter()
                    .map(|c_chirho| c_chirho.to_string())
                    .collect(),
                methods_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        )
    };

    // Data.Map
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "mapEmpty",
            "mapInsert",
            "mapLookup",
            "mapMember",
            "mapDelete",
            "mapFromList",
            "mapToList",
            "mapSize",
            "mapKeys",
            "mapElems",
            "mapFoldlWithKey",
            "mapInsertWith",
            "mapFindWithDefault",
            "mapAdjust",
            "mapUnionWith",
            "mapMap",
            "mapFilter",
            "mapNull",
            "mapInsertStr",
            "mapLookupStr",
            "mapMemberStr",
            "mapDeleteStr",
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
            "setEmpty",
            "setInsert",
            "setMember",
            "setDelete",
            "setFromList",
            "setToList",
            "setSize",
            "setUnion",
            "setIntersection",
            "setDifference",
            "setNull",
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
            "map",
            "filter",
            "foldr",
            "foldl",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "reverse",
            "zip",
            "zipWith",
            "unzip",
            "take",
            "drop",
            "takeWhile",
            "dropWhile",
            "dropWhileEnd",
            "span",
            "break",
            "elem",
            "notElem",
            "lookup",
            "sum",
            "product",
            "minimum",
            "maximum",
            "sort",
            "insert",
            "nub",
            "concat",
            "concatMap",
            "any",
            "all",
            "iterate",
            "scanl",
            "partition",
            "sortBy",
            "insertBy",
            "nubBy",
            "maximumBy",
            "minimumBy",
            "foldl'",
            "foldl1'",
            "genericSplitAt",
            "genericLength",
            "genericTake",
            "genericDrop",
            "intercalate",
            "transpose",
            "subsequences",
            "permutations",
            "isPrefixOf",
            "isSuffixOf",
            "isInfixOf",
            "stripPrefix",
            "group",
            "groupBy",
            "inits",
            "tails",
            "unfoldr",
            "find",
            "elemIndex",
            "findIndex",
            "!!",
            "union",
            "lines",
            "unlines",
            "words",
            "unwords",
            "uncons",
            "singleton",
            "foldl1",
            "foldr1",
            "scanl1",
            "scanr",
            "scanr1",
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
            "ord",
            "chr",
            "isDigit",
            "isAlpha",
            "isAlphaNum",
            "isUpper",
            "isLower",
            "isSpace",
            "isLetter",
            "isPrint",
            "isControl",
            "isPunctuation",
            "isSeparator",
            "isAscii",
            "isLatin1",
            "isAsciiUpper",
            "isAsciiLower",
            "isHexDigit",
            "isOctDigit",
            "toLower",
            "toUpper",
            "toTitle",
            "digitToInt",
            "intToDigit",
            "showLitChar",
            "readLitChar",
            "lexLitChar",
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
        for name_chirho in &["maybe", "isJust", "isNothing", "fromMaybe", "fromJust"] {
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
        for name_chirho in &["newIORef", "readIORef", "writeIORef", "modifyIORef"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IORef".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent.STM
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newTVar",
            "readTVar",
            "writeTVar",
            "modifyTVar",
            "modifyTVar'",
            "swapTVar",
            "newTVarIO",
            "readTVarIO",
            "newTMVar",
            "readTMVar",
            "putTMVar",
            "takeTMVar",
            "newTMVarIO",
            "newEmptyTMVar",
            "newEmptyTMVarIO",
            "atomically",
            "retry",
            "orElse",
            "check",
            "throwSTM",
            "catchSTM",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["STM", "TVar", "TMVar"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Concurrent.STM".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        // Also expose as GHC.Conc / Control.Concurrent.STM.TVar
        for sub_mod_chirho in &[
            "Control.Concurrent.STM.TVar",
            "Control.Concurrent.STM.TMVar",
            "GHC.Conc",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: sub_mod_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
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
            ("()", &["()"][..]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(type_name_chirho, cons_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            for con_chirho in *cons_chirho {
                let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
                exports_chirho.values_chirho.insert(k_chirho, v_chirho);
            }
        }
        // Prelude types without data constructors
        for name_chirho in &[
            "IO",
            "String",
            "Char",
            "Int",
            "Integer",
            "Float",
            "Double",
            "Rational",
            "ShowS",
            "ReadS",
            "FilePath",
            "IOError",
            "Num",
            "Eq",
            "Ord",
            "Show",
            "Read",
            "Enum",
            "Bounded",
            "Functor",
            "Applicative",
            "Alternative",
            "Monad",
            "MonadFail",
            "Semigroup",
            "Monoid",
            "Foldable",
            "Traversable",
            "Coercible",
            "Const",
            "Word",
            "Integral",
            "Fractional",
            "Floating",
            "Real",
            "RealFrac",
            "RealFloat",
            "IOMode",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        // Standard Prelude functions
        for name_chirho in &[
            // Numeric
            "abs",
            "signum",
            "negate",
            "fromInteger",
            "fromIntegral",
            "toInteger",
            "even",
            "odd",
            "max",
            "min",
            "div",
            "mod",
            "quot",
            "rem",
            "divMod",
            "quotRem",
            "^",
            "^^",
            "succ",
            "pred",
            "toEnum",
            "fromEnum",
            "minBound",
            "maxBound",
            "floor",
            "ceiling",
            "round",
            "truncate",
            // Boolean
            "not",
            "otherwise",
            // Tuple
            "fst",
            "snd",
            "curry",
            "uncurry",
            // Function
            "id",
            "const",
            "flip",
            "error",
            "undefined",
            "withFrozenCallStack",
            // List
            "map",
            "filter",
            "foldr",
            "foldl",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "reverse",
            "concat",
            "concatMap",
            "take",
            "drop",
            "takeWhile",
            "dropWhile",
            "span",
            "zip",
            "zipWith",
            "unzip",
            "!!",
            "elem",
            "notElem",
            "lookup",
            "sum",
            "product",
            "minimum",
            "maximum",
            "any",
            "all",
            "iterate",
            "scanl",
            "words",
            "unwords",
            "lines",
            "unlines",
            // IO
            "putStrLn",
            "putStr",
            "putChar",
            "print",
            "getLine",
            "getChar",
            "getContents",
            "interact",
            "readFile",
            "writeFile",
            "appendFile",
            // Show/Read
            "show",
            "read",
            // Monad/Functor/Foldable/Traversable
            "fmap",
            "return",
            "mapM_",
            "sequence_",
            "when",
            "unless",
            "foldMap",
            "traverse",
            // Conversion
            "fromString",
            "fromList",
            "toList",
            "bimap",
            // Comparison
            "compare",
            // String ops
            "intercalate",
            // Maybe/Either
            "maybe",
            "either",
            "fromMaybe",
            "isJust",
            "isNothing",
            // Data structures
            "sort",
            // Monad transformers
            "StateT",
            "runStateT",
            "evalStateT",
            "execStateT",
            "runState",
            "evalState",
            "execState",
            "get",
            "put",
            "modify",
            "bindStateT",
            "returnStateT",
            "MaybeT",
            "runMaybeT",
            "returnMaybeT",
            "bindMaybeT",
            // ReaderT
            "ReaderT",
            "runReaderT",
            "runReader",
            "ask",
            "local",
            "bindReaderT",
            "returnReaderT",
            // ExceptT
            "ExceptT",
            "runExceptT",
            "throwE",
            "returnExceptT",
            "bindExceptT",
            "catchE",
            // WriterT
            "WriterT",
            "runWriterT",
            "runWriter",
            "tell",
            "returnWriterT",
            "bindWriterT",
            "execWriterT",
            "execWriter",
            // NFData / deepseq
            "deepseq",
            "force",
            "evaluate",
            "$!!",
            // STM
            "newTVar",
            "readTVar",
            "writeTVar",
            "newTVarIO",
            "readTVarIO",
            "atomically",
            "retry",
            "orElse",
            // Data.Coerce
            "coerce",
            // Data.Function
            "on",
            "fix",
            // Numeric
            "realToFrac",
            // Missing standard Prelude functions
            "asTypeOf",
            "seq",
            "($)",
            "(.)",
            "(++)",
            "(&&)",
            "(||)",
            "(==)",
            "(/=)",
            "(<)",
            "(>)",
            "(<=)",
            "(>=)",
            "(+)",
            "(-)",
            "(*)",
            "(/)",
            "(**)",
            "logBase",
            "sqrt",
            "exp",
            "log",
            "sin",
            "cos",
            "tan",
            "asin",
            "acos",
            "atan",
            "atan2",
            "sinh",
            "cosh",
            "tanh",
            "asinh",
            "acosh",
            "atanh",
            "pi",
            "negate",
            "mapM",
            "sequence",
            "fail",
            "showsPrec",
            "showString",
            "showChar",
            "showParen",
            "shows",
            "readsPrec",
            "readParen",
            "reads",
            "lex",
            "break",
            "splitAt",
            "cycle",
            "repeat",
            "replicate",
            "and",
            "or",
            "enumFrom",
            "enumFromThen",
            "enumFromTo",
            "enumFromThenTo",
            "toRational",
            "fromRational",
            "recip",
            "divMod",
            "quotRem",
            "userError",
            "ioError",
            "until",
            "subtract",
            "gcd",
            "lcm",
            "rem",
            "(^)",
            "(^^)",
            "properFraction",
            "significand",
            "exponent",
            "floatRadix",
            "floatDigits",
            "decodeFloat",
            "encodeFloat",
            "scaleFloat",
            "isNaN",
            "isInfinite",
            "isDenormalized",
            "isNegativeZero",
            "isIEEE",
            "readLn",
            "readIO",
            "appendFile",
            "IOMode",
            "Word",
            "mempty",
            "mappend",
            "mconcat",
            "<>",
            "pure",
            "<*>",
            "*>",
            "<*",
            "Const",
            "getConst",
            "!!",
            "empty",
            "<|>",
            "some",
            "many",
            "optional",
            "liftA2",
            ">>",
            ">>=",
            "=<<",
            "foldr1",
            "foldl1",
            "foldl1'",
            "foldl'",
            "scanl1",
            "scanr",
            "scanr1",
            "catch",
            "throwIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Prelude".to_string(),
            exports_chirho,
        });
    }

    // Data.Kind — Type, Constraint
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Type", "Constraint"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Kind".to_string(),
            exports_chirho,
        });
    }

    // GHC.Exts — primops, coerce, IsList, etc.
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "coerce",
            "inline",
            "lazy",
            "oneShot",
            "noinline",
            "fromList",
            "fromListN",
            "toList",
            "groupWith",
            "sortWith",
            "the",
            "build",
            "augment",
            "reallyUnsafePtrEquality#",
            "withDict",
            "foldl'",
            "isTrue#",
            "proxy#",
            "unsafeCoerce#",
            "magicDict",
            // Boxed type constructors (data constructors for unboxed types)
            "I#",
            "W#",
            "D#",
            "F#",
            "C#",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "IsList",
            "Item",
            "Constraint",
            "Type",
            "RuntimeRep",
            "Int#",
            "Word#",
            "Float#",
            "Double#",
            "Char#",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Exts".to_string(),
            exports_chirho,
        });
    }

    // GHC.Exts re-exports all GHC.Prim primops. Add a second entry
    // that the deduplication pass will merge with the first GHC.Exts.
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "seq",
            "realWorld#",
            "proxy#",
            "void#",
            "coerce",
            // Bitwise/shift primops (also in GHC.Prim)
            "and#",
            "or#",
            "xor#",
            "not#",
            "uncheckedIShiftL#",
            "uncheckedIShiftRA#",
            "uncheckedIShiftRL#",
            "uncheckedShiftL#",
            "uncheckedShiftRL#",
            "popCnt#",
            "clz#",
            "ctz#",
            "byteSwap#",
            "narrow8Int#",
            "narrow16Int#",
            "narrow32Int#",
            "narrow8Word#",
            "narrow16Word#",
            "narrow32Word#",
            "I#",
            "W#",
            "D#",
            "F#",
            "C#",
            "+#",
            "-#",
            "*#",
            "negateInt#",
            "quotInt#",
            "remInt#",
            "+##",
            "-##",
            "*##",
            "/##",
            "negateDouble#",
            "plusFloat#",
            "minusFloat#",
            "timesFloat#",
            "divideFloat#",
            "negateFloat#",
            "plusWord#",
            "minusWord#",
            "timesWord#",
            "timesWord2#",
            "quotWord#",
            "remWord#",
            ">#",
            ">=#",
            "==#",
            "/=#",
            "<#",
            "<=#",
            "gtWord#",
            "geWord#",
            "eqWord#",
            "neWord#",
            "ltWord#",
            "leWord#",
            "int2Word#",
            "word2Int#",
            "int2Double#",
            "double2Int#",
            "int2Float#",
            "float2Int#",
            "float2Double#",
            "double2Float#",
            "chr#",
            "ord#",
            "word2Double#",
            "word2Float#",
            "newArray#",
            "readArray#",
            "writeArray#",
            "indexArray#",
            "sizeofArray#",
            "sizeofMutableArray#",
            "newByteArray#",
            "newPinnedByteArray#",
            "newAlignedPinnedByteArray#",
            "readIntArray#",
            "readWord8Array#",
            "writeIntArray#",
            "writeWord8Array#",
            "writeWord8ArrayAsWord64#",
            "indexIntArray#",
            "indexWord8Array#",
            "indexWord8ArrayAsWord64#",
            "sizeofByteArray#",
            "sizeofMutableByteArray#",
            "getSizeofMutableByteArray#",
            "copyByteArray#",
            "unsafeFreezeByteArray#",
            "byteArrayContents#",
            "isByteArrayPinned#",
            "unsafeFreezeArray#",
            "unsafeThawArray#",
            "newMutVar#",
            "readMutVar#",
            "writeMutVar#",
            "newMVar#",
            "takeMVar#",
            "putMVar#",
            "tryTakeMVar#",
            "tryPutMVar#",
            "mkWeak#",
            "deRefWeak#",
            "finalizeWeak#",
            "makeStableName#",
            "eqStableName#",
            "stableNameToInt#",
            "dataToTag#",
            "tagToEnum#",
            "reallyUnsafePtrEquality#",
            "touch#",
            "noDuplicate#",
            "raise#",
            "raiseIO#",
            "catch#",
            "maskAsyncExceptions#",
            "unmaskAsyncExceptions#",
            "atomically#",
            "retry#",
            "catchRetry#",
            "catchSTM#",
            "newTVar#",
            "readTVar#",
            "readTVarIO#",
            "writeTVar#",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Int#",
            "Word#",
            "Float#",
            "Double#",
            "Char#",
            "Addr#",
            "MutableByteArray#",
            "ByteArray#",
            "Array#",
            "MutableArray#",
            "SmallArray#",
            "SmallMutableArray#",
            "MutVar#",
            "TVar#",
            "MVar#",
            "State#",
            "RealWorld",
            "Weak#",
            "StableName#",
            "StablePtr#",
            "Proxy#",
            "TYPE",
            "RuntimeRep",
            "LiftedRep",
            "UnliftedRep",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Exts".to_string(),
            exports_chirho,
        });
    }

    // GHC.Types
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Type",
            "Constraint",
            "RuntimeRep",
            "Levity",
            "Multiplicity",
            "Symbol",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Types".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeLits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "natVal",
            "symbolVal",
            "sameNat",
            "sameSymbol",
            "someNatVal",
            "someSymbolVal",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Nat",
            "Symbol",
            "KnownNat",
            "KnownSymbol",
            "SomeNat",
            "SomeSymbol",
            "TypeError",
            "ErrorMessage",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.TypeLits".to_string(),
            exports_chirho,
        });
    }

    // GHC.Generics
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "from",
            "to",
            "from1",
            "to1",
            "datatypeName",
            "moduleName",
            "packageName",
            "selName",
            "conName",
            "conFixity",
            "conIsRecord",
            ":*:",
            ":+:",
            ":.:",
            "Comp1",
            "unComp1",
            "U1",
            "K1",
            "M1",
            "unK1",
            "unM1",
            "Par1",
            "unPar1",
            "Rec1",
            "unRec1",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Generic",
            "Generic1",
            "Rep",
            "Rep1",
            "V1",
            "U1",
            "K1",
            "M1",
            "Rec0",
            "Par1",
            "Rec1",
            "D1",
            "C1",
            "S1",
            "D",
            "C",
            "S",
            "R",
            ":*:",
            ":+:",
            ":.:",
            "Selector",
            "Constructor",
            "Datatype",
            "selName",
            "conName",
            "conFixity",
            "conIsRecord",
            "Fixity",
            "Prefix",
            "Infix",
            "Associativity",
            "LeftAssociative",
            "RightAssociative",
            "NotAssociative",
            "Meta",
            "MetaData",
            "MetaCons",
            "MetaSel",
            "SourceUnpackedness",
            "SourceStrictness",
            "DecidedStrictness",
            "NoSourceUnpackedness",
            "SourceNoUnpack",
            "SourceUnpack",
            "NoSourceStrictness",
            "SourceLazy",
            "SourceStrict",
            "DecidedLazy",
            "DecidedStrict",
            "DecidedUnpack",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for (type_name_chirho, constructors_chirho, methods_chirho) in [
            ("U1", vec!["U1"], vec![]),
            ("K1", vec!["K1"], vec!["unK1"]),
            ("M1", vec!["M1"], vec!["unM1"]),
            ("Par1", vec!["Par1"], vec!["unPar1"]),
            ("Rec1", vec!["Rec1"], vec!["unRec1"]),
            (":*:", vec![":*:"], vec![]),
            (":+:", vec!["L1", "R1"], vec![]),
            (":.:", vec!["Comp1"], vec!["unComp1"]),
        ] {
            exports_chirho.types_chirho.insert(
                type_name_chirho.to_string(),
                IfaceTypeChirho {
                    name_chirho: type_name_chirho.to_string(),
                    constructors_chirho: constructors_chirho
                        .into_iter()
                        .map(str::to_string)
                        .collect(),
                    methods_chirho: methods_chirho.into_iter().map(str::to_string).collect(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            );
        }
        for name_chirho in &["L1", "R1"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Generics".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "when",
            "unless",
            "guard",
            "void",
            "join",
            "forM",
            "forM_",
            "mapM",
            "mapM_",
            "sequence",
            "sequence_",
            "forever",
            "foldM",
            "foldM_",
            "filterM",
            "mplus",
            "mzero",
            "msum",
            "ap",
            "liftM",
            "liftM2",
            "return",
            ">>=",
            ">>",
            ">=>",
            "<=<",
            "fail",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Monad", "MonadPlus", "MonadFail"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Fix
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("mfix");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadFix", &["mfix"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Fix".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runST", "fixST", "stToIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.ST".to_string(),
            exports_chirho,
        });
    }

    // Control.Applicative
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "pure", "<*>", "*>", "<*", "liftA", "liftA2", "liftA3", "empty", "<|>", "some", "many",
            "optional",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Applicative", "Alternative"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Applicative".to_string(),
            exports_chirho,
        });
    }

    // Control.Arrow
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "arr", "first", "second", "***", "&&&", "returnA", "<<<", ">>>",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Arrow", "ArrowChoice", "ArrowApply", "ArrowLoop"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Arrow".to_string(),
            exports_chirho,
        });
    }

    // Data.Proxy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Proxy", &["Proxy"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Proxy");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("KProxy", &["KProxy"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("KProxy");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Proxy".to_string(),
            exports_chirho,
        });
    }

    // Data.Coerce
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("coerce");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Coercible", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Coerce".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Equality
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "castWith",
            "gcastWith",
            "testEquality",
            "sym",
            "trans",
            "inner",
            "outer",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["(:~:)", "(:~~:)", "TestEquality"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["Refl"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_val_chirho("Refl");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Equality".to_string(),
            exports_chirho,
        });
    }

    // Data.Typeable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "typeOf",
            "typeRep",
            "cast",
            "eqT",
            "gcast",
            "gcast1",
            "gcast2",
            "rnfTyCon",
            "rnfTypeRep",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Typeable", "TypeRep", "Proxy"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Typeable".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        exports_chirho.types_chirho.insert(
            "Identity".to_string(),
            IfaceTypeChirho {
                name_chirho: "Identity".to_string(),
                constructors_chirho: vec!["Identity".to_string()],
                methods_chirho: vec!["runIdentity".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        let (k_chirho, v_chirho) = mk_val_chirho("Identity");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("runIdentity");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Identity".to_string(),
            exports_chirho,
        });
    }

    // Type.Reflection
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "typeOf",
            "typeRep",
            "typeRepTyCon",
            "someTypeRep",
            "rnfTypeRep",
            "rnfModule",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Typeable", "TypeRep", "SomeTypeRep", "TyCon"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Type.Reflection".to_string(),
            exports_chirho,
        });
    }

    // System.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "putStr",
            "putStrLn",
            "print",
            "getLine",
            "getContents",
            "readFile",
            "writeFile",
            "appendFile",
            "hSetBuffering",
            "hGetBuffering",
            "hIsTerminalDevice",
            "hGetContents",
            "hPutStr",
            "hPutStrLn",
            "hFlush",
            "hClose",
            "hSetEncoding",
            "stdin",
            "stdout",
            "stderr",
            "withFile",
            "openFile",
            "openBinaryFile",
            "hSetBinaryMode",
            "hIsEOF",
            "isEOF",
            "hGetChar",
            "hGetLine",
            "hLookAhead",
            "hReady",
            "hPutChar",
            "hPrint",
            "hTell",
            "hSeek",
            "hFileSize",
            "hIsOpen",
            "hIsClosed",
            "hIsReadable",
            "hIsWritable",
            "hIsSeekable",
            "hIsTerminalDevice",
            "hSetNewlineMode",
            "hGetEncoding",
            "mkTextEncoding",
            "utf8",
            "utf16",
            "latin1",
            "char8",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "IO",
            "Handle",
            "IOMode",
            "BufferMode",
            "SeekMode",
            "NewlineMode",
            "Newline",
            "TextEncoding",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO".to_string(),
            exports_chirho,
        });
    }

    // Data.STRef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["newSTRef", "readSTRef", "writeSTRef", "modifySTRef"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("STRef", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.STRef".to_string(),
            exports_chirho,
        });
    }

    // Data.Word
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Word", "Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Word".to_string(),
            exports_chirho,
        });
    }

    // Data.Int
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Int", "Int8", "Int16", "Int32", "Int64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Int".to_string(),
            exports_chirho,
        });
    }

    // Data.List.NonEmpty
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["head", "tail", "toList", "fromList", "map", "nonEmpty"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("NonEmpty", &["(:|)"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_name_chirho in &["(:|)", ":|"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.List.NonEmpty".to_string(),
            exports_chirho,
        });
    }

    // Data.Tuple.Experimental
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Solo", "fst", "snd", "swap", "curry", "uncurry"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Solo", "Tuple0", "Tuple1", "Tuple2", "Tuple3", "Unit"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Tuple.Experimental".to_string(),
            exports_chirho,
        });
    }

    // Data.Sum.Experimental
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Sum2", "Sum3", "Sum4", "Sum5"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Sum.Experimental".to_string(),
            exports_chirho,
        });
    }

    // GHC.StaticPtr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["staticKey", "deRefStaticPtr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["StaticPtr", "IsStatic"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.StaticPtr".to_string(),
            exports_chirho,
        });
    }

    // Unsafe.Coerce
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("unsafeCoerce");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("UnsafeEquality", &["UnsafeRefl"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("UnsafeRefl");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Unsafe.Coerce".to_string(),
            exports_chirho,
        });
    }

    // Data.Void
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Void", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("absurd");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("vacuous");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Void".to_string(),
            exports_chirho,
        });
    }

    // Data.Monoid
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "mempty",
            "mappend",
            "mconcat",
            "getSum",
            "getProduct",
            "getFirst",
            "getLast",
            "getAny",
            "getAll",
            "getDual",
            "getEndo",
            "appEndo",
            "getAlt",
            "getAp",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Monoid", "Endo"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        // Newtypes with record accessors — list constructor + accessor
        for (type_chirho, ctors_chirho) in &[
            ("Sum", &["Sum", "getSum"][..]),
            ("Product", &["Product", "getProduct"][..]),
            ("First", &["First", "getFirst"][..]),
            ("Last", &["Last", "getLast"][..]),
            ("Any", &["Any", "getAny"][..]),
            ("All", &["All", "getAll"][..]),
            ("Dual", &["Dual", "getDual"][..]),
            ("Ap", &["Ap", "getAp"][..]),
            ("Alt", &["Alt", "getAlt"][..]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(type_chirho, ctors_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Monoid".to_string(),
            exports_chirho,
        });
    }

    // Data.Semigroup
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "(<>)",
            "sconcat",
            "stimes",
            "getMin",
            "getMax",
            "getFirst",
            "getLast",
            "getWrappedMonoid",
            "unwrapMonoid",
            "getOption",
            "option",
            "diff",
            "cycle1",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Semigroup"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["(<>)", "sconcat", "stimes"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for (type_chirho, ctors_chirho) in &[
            ("Min", &["Min", "getMin"][..]),
            ("Max", &["Max", "getMax"][..]),
            ("First", &["First", "getFirst"][..]),
            ("Last", &["Last", "getLast"][..]),
            (
                "WrappedMonoid",
                &["WrapMonoid", "unwrapMonoid", "getWrappedMonoid"][..],
            ),
            ("Option", &["Option", "getOption"][..]),
            ("Arg", &["Arg"][..]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(type_chirho, ctors_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Semigroup".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeNats
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "natVal",
            "natVal'",
            "someNatVal",
            "sameNat",
            "cmpNat",
            "withSomeSNat",
            "withKnownNat",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Natural", "Nat", "KnownNat", "SomeNat", "SNat", "CmpNat", "Div", "Mod", "Log2",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.TypeNats".to_string(),
            exports_chirho,
        });
    }

    // Data.Data
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "toConstr",
            "gunfold",
            "gfoldl",
            "dataTypeOf",
            "gmapT",
            "gmapQ",
            "gmapQl",
            "gmapQr",
            "gmapQi",
            "gmapM",
            "mkConstr",
            "mkDataType",
            "constrType",
            "showConstr",
            "cast",
            "gcast",
            "gcast1",
            "gcast2",
            "typeOf",
            "typeRep",
            "mkFunTy",
            "fromConstr",
            "fromConstrB",
            "fromConstrM",
            "dataTypeConstrs",
            "maxConstrIndex",
            "indexConstr",
            "dataTypeName",
            "constrFields",
            "constrFixity",
            "constrIndex",
            "constrRep",
            "repConstr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Data",
            "Typeable",
            "DataType",
            "Constr",
            "ConstrRep",
            "DataRep",
            "ConIndex",
            "Fixity",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Data".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.IO.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("liftIO");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadIO", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.IO.Class".to_string(),
            exports_chirho,
        });
    }

    // Control.Category
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["id", ".", "<<<", ">>>"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Category", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Category".to_string(),
            exports_chirho,
        });
    }

    // Data.Ord
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["compare", "comparing", "clamp", "Down"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Ord", "Ordering", "Down"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["LT", "EQ", "GT"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Ord".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("lift");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadTrans", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Class".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runIdentityT", "mapIdentityT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IdentityT", &["IdentityT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Identity".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.State / .Lazy / .Strict
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "runStateT",
            "evalStateT",
            "execStateT",
            "runState",
            "evalState",
            "execState",
            "get",
            "put",
            "modify",
            "gets",
            "state",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["StateT", "State"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &[
            "Control.Monad.Trans.State",
            "Control.Monad.Trans.State.Lazy",
            "Control.Monad.Trans.State.Strict",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Monad.Trans.Reader
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runReaderT", "mapReaderT", "withReaderT", "reader"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ReaderT", &["ReaderT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Reader".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Writer / .Lazy / .Strict
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "runWriterT",
            "execWriterT",
            "mapWriterT",
            "tell",
            "listen",
            "pass",
            "writer",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("WriterT", &["WriterT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &[
            "Control.Monad.Trans.Writer",
            "Control.Monad.Trans.Writer.Lazy",
            "Control.Monad.Trans.Writer.Strict",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Monad.Trans.Except
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "runExceptT",
            "throwE",
            "catchE",
            "mapExceptT",
            "withExceptT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ExceptT", &["ExceptT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Except".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Maybe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runMaybeT", "mapMaybeT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MaybeT", &["MaybeT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Maybe".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Reader
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ask",
            "asks",
            "local",
            "reader",
            "runReader",
            "runReaderT",
            "mapReaderT",
            "withReaderT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Reader", "ReaderT", "MonadReader"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Reader".to_string(),
            exports_chirho,
        });
    }

    // Control.Exception
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "throw",
            "throwIO",
            "catch",
            "handle",
            "try",
            "evaluate",
            "bracket",
            "bracket_",
            "finally",
            "onException",
            "throwTo",
            "mask",
            "mask_",
            "uninterruptibleMask",
            "assert",
            "mapException",
            "displayException",
            "toException",
            "fromException",
            "catches",
            "Handler",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Exception",
            "SomeException",
            "IOException",
            "ErrorCall",
            "ArithException",
            "ArrayException",
            "AsyncException",
            "NonTermination",
            "BlockedIndefinitelyOnMVar",
            "BlockedIndefinitelyOnSTM",
            "Deadlock",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Exception".to_string(),
            exports_chirho,
        });
    }

    // Data.Function
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["id", "const", "flip", "fix", "on", "&"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Function".to_string(),
            exports_chirho,
        });
    }

    // Data.Ix
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["range", "index", "inRange", "rangeSize"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Ix", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Ix".to_string(),
            exports_chirho,
        });
    }

    // GHC.Tuple
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Solo", "Unit", "Tuple0", "Tuple1", "Tuple2", "Tuple3", "Tuple4", "Tuple5", "Tuple6",
            "Tuple7",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_val_chirho("Solo");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Tuple".to_string(),
            exports_chirho,
        });
    }

    // GHC.List
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "map",
            "filter",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "foldr",
            "foldl",
            "foldl'",
            "scanl",
            "scanl'",
            "scanr",
            "iterate",
            "iterate'",
            "repeat",
            "replicate",
            "cycle",
            "take",
            "drop",
            "splitAt",
            "takeWhile",
            "dropWhile",
            "span",
            "break",
            "reverse",
            "and",
            "or",
            "any",
            "all",
            "elem",
            "notElem",
            "lookup",
            "zip",
            "zip3",
            "zipWith",
            "unzip",
            "unzip3",
            "concat",
            "concatMap",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.List".to_string(),
            exports_chirho,
        });
    }

    // GHC.Base
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "id",
            "const",
            "flip",
            ".",
            "$",
            "otherwise",
            "map",
            "foldr",
            "build",
            "augment",
            "fmap",
            "<$>",
            "pure",
            "<*>",
            "return",
            ">>=",
            ">>",
            "eqString",
            "bindIO",
            "returnIO",
            "thenIO",
            "seq",
            "maxInt",
            "minInt",
            "mkWeak#",
            "deRefWeak#",
            "finalizeWeak#",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Functor",
            "Applicative",
            "Monad",
            "Semigroup",
            "Monoid",
            "String",
            "Opaque",
            "SPEC",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Base".to_string(),
            exports_chirho,
        });
    }

    // GHC.Classes
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "==", "/=", "<", "<=", ">", ">=", "compare", "max", "min", "not", "&&", "||",
            "divInt#", "modInt#",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Eq", "Ord", "IP"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Classes".to_string(),
            exports_chirho,
        });
    }

    // GHC.Num
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "+",
            "-",
            "*",
            "negate",
            "abs",
            "signum",
            "fromInteger",
            "subtract",
            "integerToInt",
            "naturalToWord",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Num", "Integer", "Natural"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Num".to_string(),
            exports_chirho,
        });
    }

    // GHC.Show
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "show",
            "showsPrec",
            "showString",
            "showChar",
            "showParen",
            "shows",
            "showList__",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Show", "ShowS"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Show".to_string(),
            exports_chirho,
        });
    }

    // GHC.Read
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["read", "reads", "readParen", "lex", "readsPrec", "readList"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Read", "ReadS"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Read".to_string(),
            exports_chirho,
        });
    }

    // GHC.Enum
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "succ",
            "pred",
            "toEnum",
            "fromEnum",
            "enumFrom",
            "enumFromThen",
            "enumFromTo",
            "enumFromThenTo",
            "minBound",
            "maxBound",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Enum", "Bounded"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Enum".to_string(),
            exports_chirho,
        });
    }

    // GHC.Real
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "div",
            "mod",
            "quot",
            "rem",
            "divMod",
            "quotRem",
            "toInteger",
            "toRational",
            "fromIntegral",
            "realToFrac",
            "%",
            "numerator",
            "denominator",
            "reduce",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Integral",
            "Real",
            "RealFrac",
            "Fractional",
            "Ratio",
            "Rational",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Real".to_string(),
            exports_chirho,
        });
    }

    // GHC.Float
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "pi",
            "exp",
            "log",
            "sqrt",
            "sin",
            "cos",
            "tan",
            "asin",
            "acos",
            "atan",
            "sinh",
            "cosh",
            "tanh",
            "float2Double",
            "double2Float",
            "isNaN",
            "isInfinite",
            "isDenormalized",
            "isNegativeZero",
            "isIEEE",
            "integerToFloat#",
            "integerToDouble#",
            "rationalToFloat",
            "rationalToDouble",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Float", "Double", "Floating", "RealFloat"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Float".to_string(),
            exports_chirho,
        });
    }

    // GHC.Int
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Int8", "Int16", "Int32", "Int64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Int".to_string(),
            exports_chirho,
        });
    }

    // GHC.Word
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Word".to_string(),
            exports_chirho,
        });
    }

    // GHC.ST
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runST", "runSTRep"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.ST".to_string(),
            exports_chirho,
        });
    }

    // GHC.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "unsafePerformIO",
            "unsafeInterleaveIO",
            "unsafeDupablePerformIO",
            "stToIO",
            "ioToST",
            "throwIO",
            "catchException",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["IO", "MVar", "IORef"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IO".to_string(),
            exports_chirho,
        });
    }

    // GHC.IORef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newIORef",
            "readIORef",
            "writeIORef",
            "modifyIORef",
            "atomicModifyIORef",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IORef", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IORef".to_string(),
            exports_chirho,
        });
    }

    // GHC.MVar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newMVar",
            "newEmptyMVar",
            "takeMVar",
            "putMVar",
            "readMVar",
            "tryTakeMVar",
            "tryPutMVar",
            "isEmptyMVar",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MVar", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.MVar".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.State / Control.Monad.State.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "get",
            "put",
            "modify",
            "gets",
            "state",
            "runState",
            "evalState",
            "execState",
            "runStateT",
            "evalStateT",
            "execStateT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["State", "StateT"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        {
            let (k_chirho, v_chirho) = mk_type_chirho("MonadState", &["get", "put", "state"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.State".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.State.Class".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.State.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.State.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Writer / Control.Monad.Writer.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "tell",
            "listen",
            "pass",
            "writer",
            "runWriter",
            "execWriter",
            "runWriterT",
            "execWriterT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Writer", "WriterT", "MonadWriter"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Writer".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Writer.Class".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Writer.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Writer.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Except
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "throwError",
            "catchError",
            "runExcept",
            "runExceptT",
            "mapExcept",
            "mapExceptT",
            "withExcept",
            "withExceptT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Except", "ExceptT", "MonadError"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Except".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runIdentity"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Identity", &["Identity"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Identity");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Identity".to_string(),
            exports_chirho,
        });
    }

    // Data.Ratio
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["%", "numerator", "denominator", "approxRational"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Ratio", "Rational"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Ratio".to_string(),
            exports_chirho,
        });
    }

    // Data.Complex
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "realPart",
            "imagPart",
            "mkPolar",
            "cis",
            "polar",
            "magnitude",
            "phase",
            "conjugate",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Complex", &[":+"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho(":+");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Complex".to_string(),
            exports_chirho,
        });
    }

    // Data.Map.Strict (alias to Data.Map)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "insertWith",
            "delete",
            "lookup",
            "member",
            "findWithDefault",
            "adjust",
            "update",
            "union",
            "unionWith",
            "intersection",
            "intersectionWith",
            "difference",
            "map",
            "mapWithKey",
            "filter",
            "filterWithKey",
            "foldr",
            "foldl",
            "foldrWithKey",
            "foldlWithKey",
            "null",
            "size",
            "keys",
            "elems",
            "toList",
            "fromList",
            "toAscList",
            "toDescList",
            "fromAscList",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Map", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Map.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Map.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Data.Set (expanded with standard API names)
    // (original Data.Set uses custom names; this covers standard containers API)

    // Data.String
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("fromString");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("IsString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("String", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.String".to_string(),
            exports_chirho,
        });
    }

    // Data.Tuple
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["fst", "snd", "curry", "uncurry", "swap"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Solo"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["MkSolo"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            let (k_chirho, v_chirho) = mk_val_chirho("MkSolo");
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Tuple".to_string(),
            exports_chirho,
        });
    }

    // Data.Either
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "either",
            "lefts",
            "rights",
            "isLeft",
            "isRight",
            "fromLeft",
            "fromRight",
            "partitionEithers",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Either", &["Left", "Right"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["Left", "Right"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Either".to_string(),
            exports_chirho,
        });
    }

    // Data.Bool
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["bool", "not", "otherwise", "&&", "||"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Bool", &["False", "True"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["False", "True"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bool".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["fmap", "<$>", "<$", "$>", "void", "<&>"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Functor", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor".to_string(),
            exports_chirho,
        });
    }

    // Data.Foldable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "fold",
            "foldMap",
            "foldl",
            "foldr",
            "foldl'",
            "foldr'",
            "toList",
            "null",
            "length",
            "elem",
            "maximum",
            "minimum",
            "sum",
            "product",
            "any",
            "all",
            "and",
            "or",
            "concat",
            "concatMap",
            "asum",
            "find",
            "mapM_",
            "forM_",
            "sequence_",
            "for_",
            "traverse_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Foldable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Foldable".to_string(),
            exports_chirho,
        });
    }

    // Data.Traversable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "traverse",
            "sequenceA",
            "mapM",
            "sequence",
            "for",
            "forM",
            "mapAccumL",
            "mapAccumR",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Traversable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Traversable".to_string(),
            exports_chirho,
        });
    }

    // Data.Bifunctor
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["bimap", "first", "second"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Bifunctor", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bifunctor".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Signatures (transformers internal)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["CallCC", "Catch", "Listen", "Pass"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Signatures".to_string(),
            exports_chirho,
        });
    }

    // Control.Applicative (base)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "pure", "<*>", "*>", "<*", "<**>", "liftA", "liftA2", "liftA3", "empty", "<|>", "some",
            "many", "optional", "asum", "guard", "when", "unless",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Alternative", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ZipList", &["ZipList"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Applicative".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad (base)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "return",
            ">>=",
            ">>",
            "=<<",
            "join",
            "void",
            "when",
            "unless",
            "guard",
            "forever",
            "mapM",
            "mapM_",
            "forM",
            "forM_",
            "sequence",
            "sequence_",
            "foldM",
            "foldM_",
            "filterM",
            "replicateM",
            "replicateM_",
            "liftM",
            "liftM2",
            "liftM3",
            "ap",
            "MonadPlus",
            "mzero",
            "mplus",
            "msum",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        exports_chirho.types_chirho.insert(
            "Monad".to_string(),
            IfaceTypeChirho {
                name_chirho: "Monad".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["return".to_string(), ">>=".to_string(), ">>".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "MonadPlus".to_string(),
            IfaceTypeChirho {
                name_chirho: "MonadPlus".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["mzero".to_string(), "mplus".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "MonadFail".to_string(),
            IfaceTypeChirho {
                name_chirho: "MonadFail".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["fail".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad".to_string(),
            exports_chirho,
        });
    }

    // Data.Typeable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "typeOf",
            "typeRep",
            "cast",
            "eqT",
            "gcast",
            "gcast1",
            "gcast2",
            "mkTyCon",
            "mkTyConApp",
            "typeRepTyCon",
            "typeRepArgs",
            "splitTyConApp",
            "funResultTy",
            "showsTypeRep",
            "rnfTyCon",
            "rnfTypeRep",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Typeable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("TypeRep", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Proxy", &["Proxy"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Typeable".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.* (transformers package — needed by mtl, everything)
    {
        // Common transformer types and operations
        let trans_modules_chirho = [
            (
                "Control.Monad.Trans.Class",
                vec!["lift"],
                vec![("MonadTrans", &[][..])],
            ),
            (
                "Control.Monad.Trans.Identity",
                vec!["runIdentityT"],
                vec![("IdentityT", &["IdentityT"][..])],
            ),
            (
                "Control.Monad.Trans.Reader",
                vec![
                    "runReaderT",
                    "ask",
                    "local",
                    "asks",
                    "reader",
                    "withReaderT",
                    "mapReaderT",
                ],
                vec![("ReaderT", &["ReaderT"][..])],
            ),
            (
                "Control.Monad.Trans.State",
                vec![
                    "runStateT",
                    "evalStateT",
                    "execStateT",
                    "get",
                    "put",
                    "modify",
                    "gets",
                    "state",
                ],
                vec![("StateT", &["StateT"][..])],
            ),
            (
                "Control.Monad.Trans.State.Strict",
                vec![
                    "runStateT",
                    "evalStateT",
                    "execStateT",
                    "get",
                    "put",
                    "modify",
                    "gets",
                    "state",
                ],
                vec![("StateT", &["StateT"][..])],
            ),
            (
                "Control.Monad.Trans.State.Lazy",
                vec![
                    "runStateT",
                    "evalStateT",
                    "execStateT",
                    "get",
                    "put",
                    "modify",
                    "gets",
                    "state",
                ],
                vec![("StateT", &["StateT"][..])],
            ),
            (
                "Control.Monad.Trans.Writer",
                vec![
                    "runWriterT",
                    "execWriterT",
                    "tell",
                    "listen",
                    "pass",
                    "writer",
                ],
                vec![("WriterT", &["WriterT"][..])],
            ),
            (
                "Control.Monad.Trans.Writer.Strict",
                vec![
                    "runWriterT",
                    "execWriterT",
                    "tell",
                    "listen",
                    "pass",
                    "writer",
                ],
                vec![("WriterT", &["WriterT"][..])],
            ),
            (
                "Control.Monad.Trans.Writer.Lazy",
                vec![
                    "runWriterT",
                    "execWriterT",
                    "tell",
                    "listen",
                    "pass",
                    "writer",
                ],
                vec![("WriterT", &["WriterT"][..])],
            ),
            (
                "Control.Monad.Trans.Writer.CPS",
                vec![
                    "runWriterT",
                    "execWriterT",
                    "tell",
                    "listen",
                    "pass",
                    "writer",
                ],
                vec![("WriterT", &["WriterT"][..])],
            ),
            (
                "Control.Monad.Trans.Except",
                vec![
                    "runExceptT",
                    "throwE",
                    "catchE",
                    "mapExceptT",
                    "withExceptT",
                ],
                vec![("ExceptT", &["ExceptT"][..])],
            ),
            (
                "Control.Monad.Trans.Maybe",
                vec!["runMaybeT", "mapMaybeT"],
                vec![("MaybeT", &["MaybeT"][..])],
            ),
            (
                "Control.Monad.Trans.Cont",
                vec![
                    "runContT",
                    "evalContT",
                    "cont",
                    "callCC",
                    "resetT",
                    "shiftT",
                ],
                vec![("ContT", &["ContT"][..])],
            ),
            (
                "Control.Monad.Trans.RWS",
                vec!["runRWST", "evalRWST", "execRWST"],
                vec![("RWST", &["RWST"][..])],
            ),
            (
                "Control.Monad.Trans.RWS.Strict",
                vec!["runRWST", "evalRWST", "execRWST"],
                vec![("RWST", &["RWST"][..])],
            ),
            (
                "Control.Monad.Trans.RWS.Lazy",
                vec!["runRWST", "evalRWST", "execRWST"],
                vec![("RWST", &["RWST"][..])],
            ),
            (
                "Control.Monad.Trans.RWS.CPS",
                vec!["runRWST", "evalRWST", "execRWST"],
                vec![("RWST", &["RWST"][..])],
            ),
        ];
        for (mod_name_chirho, vals_chirho, types_chirho) in &trans_modules_chirho {
            let mut exports_chirho = IfaceExportsChirho::default();
            for val_chirho in vals_chirho {
                let (k_chirho, v_chirho) = mk_val_chirho(val_chirho);
                exports_chirho.values_chirho.insert(k_chirho, v_chirho);
            }
            for (ty_name_chirho, cons_chirho) in types_chirho {
                let (k_chirho, v_chirho) = mk_type_chirho(ty_name_chirho, cons_chirho);
                exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            }
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho,
            });
        }
    }

    // Data.Array / Data.Array.Unboxed
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "array",
            "listArray",
            "accumArray",
            "ixmap",
            "elems",
            "assocs",
            "indices",
            "bounds",
            "!",
            "//",
            "range",
            "index",
            "inRange",
            "rangeSize",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Array", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Ix", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        let (k_chirho, v_chirho) = mk_type_chirho("UArray", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array.Unboxed".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array.IArray".to_string(),
            exports_chirho,
        });
    }

    // GHC.Num.Natural / Numeric.Natural
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["naturalToWord", "naturalToInteger", "intToNatural"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Num.Natural".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Numeric.Natural".to_string(),
            exports_chirho,
        });
    }

    // Data.CallStack (call-stack package, used by HUnit)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "callStack",
            "HasCallStack",
            "withFrozenCallStack",
            "prettyCallStack",
            "getCallStack",
            "SrcLoc",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("CallStack", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.CallStack".to_string(),
            exports_chirho,
        });
    }

    // Network.HTTP.Types (http-types package — needed by servant, warp, etc.)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "statusCode",
            "statusMessage",
            "status200",
            "status301",
            "status302",
            "status400",
            "status401",
            "status403",
            "status404",
            "status500",
            "ok200",
            "notFound404",
            "methodGet",
            "methodPost",
            "methodPut",
            "methodDelete",
            "methodPatch",
            "methodHead",
            "methodOptions",
            "hContentType",
            "hAccept",
            "hAuthorization",
            "hCacheControl",
            "hCookie",
            "hContentLength",
            "parseQuery",
            "renderQuery",
            "simpleQueryToQuery",
            "queryToQueryBS",
            "urlDecode",
            "urlEncode",
            "decodePath",
            "encodePath",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for ty_chirho in &[
            ("Status", &["Status"][..]),
            ("Method", &[]),
            ("Header", &[]),
            ("HeaderName", &[]),
            ("Query", &[]),
            ("QueryItem", &[]),
            (
                "StdMethod",
                &["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"],
            ),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(ty_chirho.0, ty_chirho.1);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &[
            "Network.HTTP.Types",
            "Network.HTTP.Types.Status",
            "Network.HTTP.Types.Method",
            "Network.HTTP.Types.Header",
            "Network.HTTP.Types.URI",
            "Network.HTTP.Types.Version",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Gauge.Main / Criterion.Main (benchmarking)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "defaultMain",
            "bench",
            "bgroup",
            "nf",
            "whnf",
            "env",
            "nfIO",
            "whnfIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Benchmark", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &["Gauge.Main", "Gauge", "Criterion.Main", "Criterion"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Lens (lens package — massive, synthetic)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "view",
            "over",
            "set",
            "lens",
            "iso",
            "prism",
            "traverse",
            "to",
            "from",
            "review",
            "preview",
            "folded",
            "filtered",
            "mapped",
            "each",
            "both",
            "ix",
            "at",
            "contains",
            "_1",
            "_2",
            "_3",
            "_4",
            "_5",
            "_head",
            "_tail",
            "_init",
            "_last",
            "_Left",
            "_Right",
            "_Just",
            "_Nothing",
            "makeLenses",
            "makePrisms",
            "makeClassy",
            "makeFields",
            "(&)",
            "(%~)",
            "(.~)",
            "(^.)",
            "(^?)",
            "(^..)",
            "(+~)",
            "(-~)",
            "(*~)",
            "(//~)",
            "(&&~)",
            "(||~)",
            "use",
            "uses",
            "assign",
            "modifying",
            "zoom",
            "magnify",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for ty_chirho in &[
            ("Lens", &[][..]),
            ("Lens'", &[]),
            ("Traversal", &[]),
            ("Traversal'", &[]),
            ("Prism", &[]),
            ("Prism'", &[]),
            ("Iso", &[]),
            ("Iso'", &[]),
            ("Getter", &[]),
            ("Setter", &[]),
            ("Setter'", &[]),
            ("Fold", &[]),
            ("Review", &[]),
            ("Getting", &[]),
            ("ASetter", &[]),
            ("ASetter'", &[]),
            ("ALens", &[]),
            ("ALens'", &[]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(ty_chirho.0, ty_chirho.1);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        let lens_modules_chirho = [
            "Control.Lens",
            "Control.Lens.Type",
            "Control.Lens.Lens",
            "Control.Lens.Getter",
            "Control.Lens.Setter",
            "Control.Lens.Fold",
            "Control.Lens.Traversal",
            "Control.Lens.Prism",
            "Control.Lens.Iso",
            "Control.Lens.Review",
            "Control.Lens.At",
            "Control.Lens.Each",
            "Control.Lens.Indexed",
            "Control.Lens.TH",
            "Control.Lens.Combinators",
            "Control.Lens.Operators",
            "Control.Lens.Tuple",
        ];
        for mod_name_chirho in &lens_modules_chirho {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Parallel.Strategies
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "rpar",
            "rseq",
            "rdeepseq",
            "parMap",
            "parList",
            "parListChunk",
            "dot",
            "using",
            "withStrategy",
            "evalList",
            "evalTraversable",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Strategy", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Eval", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Parallel.Strategies".to_string(),
            exports_chirho,
        });
    }

    // Data.Aeson + related encoding/types modules
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "encode",
            "decode",
            "decode'",
            "eitherDecode",
            "eitherDecode'",
            "toJSON",
            "fromJSON",
            "toEncoding",
            "parseJSON",
            "encodingToLazyByteString",
            "pairs",
            "pair",
            "object",
            "withObject",
            "withArray",
            "withText",
            "withScientific",
            "withBool",
            "parseField",
            "parseFieldMaybe",
            "parseFieldMaybe'",
            "genericToJSON",
            "genericToEncoding",
            "genericParseJSON",
            "defaultOptions",
            "fieldLabelModifier",
            "constructorTagModifier",
            "allNullaryToStringTag",
            "omitNothingFields",
            "sumEncoding",
            "unwrapUnaryRecords",
            "tagSingleConstructors",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for ty_chirho in &[
            (
                "Value",
                &["Object", "Array", "String", "Number", "Bool", "Null"][..],
            ),
            ("ToJSON", &[]),
            ("FromJSON", &[]),
            ("ToJSON1", &[]),
            ("FromJSON1", &[]),
            ("Encoding", &[]),
            ("Series", &[]),
            ("Options", &["Options"]),
            (
                "SumEncoding",
                &[
                    "TaggedObject",
                    "UntaggedValue",
                    "ObjectWithSingleField",
                    "TwoElemArray",
                ],
            ),
            ("Result", &["Success", "Error"]),
            ("Parser", &[]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(ty_chirho.0, ty_chirho.1);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        // Share across all aeson sub-modules
        let aeson_modules_chirho = [
            "Data.Aeson",
            "Data.Aeson.Types",
            "Data.Aeson.Encoding",
            "Data.Aeson.Encoding.Internal",
            "Data.Aeson.Types.ToJSON",
            "Data.Aeson.Types.FromJSON",
            "Data.Aeson.Types.Internal",
            "Data.Aeson.Types.Generic",
            "Data.Aeson.Internal",
            "Data.Aeson.Key",
            "Data.Aeson.KeyMap",
        ];
        for mod_name_chirho in &aeson_modules_chirho {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Monad.Catch (exceptions package)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "throwM",
            "catch",
            "catchAll",
            "try",
            "tryJust",
            "handle",
            "handleAll",
            "handleJust",
            "onException",
            "bracket",
            "bracket_",
            "finally",
            "mask",
            "uninterruptibleMask",
            "mask_",
            "catches",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MonadThrow", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadCatch", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadMask", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("SomeException", &["SomeException"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Catch".to_string(),
            exports_chirho,
        });
    }

    // Control.Exception (base)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "throw",
            "throwIO",
            "catch",
            "try",
            "evaluate",
            "bracket",
            "bracket_",
            "finally",
            "onException",
            "handle",
            "handleJust",
            "tryJust",
            "catches",
            "mask",
            "mask_",
            "uninterruptibleMask",
            "assert",
            "throwTo",
            "ioError",
            "userError",
            "catchJust",
            "Handler",
            "mapException",
            "toException",
            "fromException",
            "displayException",
            "SomeException",
            "Exception",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        // Exception class with methods — (..) import brings these in
        exports_chirho.types_chirho.insert(
            "Exception".to_string(),
            IfaceTypeChirho {
                name_chirho: "Exception".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![
                    "toException".to_string(),
                    "fromException".to_string(),
                    "displayException".to_string(),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        let (k_chirho, v_chirho) = mk_type_chirho("SomeException", &["SomeException"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("IOException", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho(
            "AsyncException",
            &[
                "StackOverflow",
                "HeapOverflow",
                "ThreadKilled",
                "UserInterrupt",
            ],
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Exception".to_string(),
            exports_chirho,
        });
    }

    // Data.ByteString.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "c2w",
            "w2c",
            "unsafeCreate",
            "create",
            "createAndTrim",
            "mallocByteString",
            "nullForeignPtr",
            "accursedUnutterablePerformIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ByteString", &["BS", "PS"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Internal".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Internal.Type".to_string(),
            exports_chirho,
        });
    }

    // Data.ByteString.Short / Data.ByteString.Short.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "toShort",
            "fromShort",
            "pack",
            "unpack",
            "empty",
            "null",
            "length",
            "index",
            "head",
            "last",
            "tail",
            "init",
            "cons",
            "snoc",
            "append",
            "isPrefixOf",
            "isSuffixOf",
            "concat",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ShortByteString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_chirho in &["Data.ByteString.Short", "Data.ByteString.Short.Internal"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Word / Data.Int
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Word", "Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Word".to_string(),
            exports_chirho,
        });
        let mut int_exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Int", "Int8", "Int16", "Int32", "Int64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            int_exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Int".to_string(),
            exports_chirho: int_exports_chirho,
        });
    }

    // Foreign.Ptr / Foreign.ForeignPtr / Foreign.Storable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["nullPtr", "plusPtr", "castPtr", "minusPtr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Ptr", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("FunPtr", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Ptr".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        let (k_chirho, v_chirho) = mk_type_chirho("ForeignPtr", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "newForeignPtr",
            "withForeignPtr",
            "unsafeWithForeignPtr",
            "mallocForeignPtrBytes",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.ForeignPtr".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign".to_string(),
            exports_chirho,
        });
        let mut stor_exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["sizeOf", "alignment", "peek", "poke"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            stor_exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Storable", &[]);
        stor_exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Storable".to_string(),
            exports_chirho: stor_exports_chirho,
        });
    }

    // Data.IORef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newIORef",
            "readIORef",
            "writeIORef",
            "modifyIORef",
            "modifyIORef'",
            "atomicModifyIORef",
            "atomicModifyIORef'",
            "atomicWriteIORef",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IORef", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IORef".to_string(),
            exports_chirho,
        });
    }

    // System.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "hFlush",
            "hPutStr",
            "hPutStrLn",
            "hGetLine",
            "hSetBuffering",
            "hGetBuffering",
            "hSetEncoding",
            "hClose",
            "openFile",
            "withFile",
            "stdin",
            "stdout",
            "stderr",
            "utf8",
            "hGetContents",
            "hPrint",
            "putStr",
            "putStrLn",
            "print",
            "getLine",
            "getContents",
            "readFile",
            "writeFile",
            "appendFile",
            "hIsTerminalDevice",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Handle", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho(
            "IOMode",
            &["ReadMode", "WriteMode", "AppendMode", "ReadWriteMode"],
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho(
            "BufferMode",
            &["NoBuffering", "LineBuffering", "BlockBuffering"],
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO".to_string(),
            exports_chirho,
        });
    }

    // System.Exit
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["exitWith", "exitFailure", "exitSuccess", "die"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ExitCode", &["ExitSuccess", "ExitFailure"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Exit".to_string(),
            exports_chirho,
        });
    }

    // Data.Version
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "showVersion",
            "parseVersion",
            "makeVersion",
            "versionBranch",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Version", &["Version"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Version".to_string(),
            exports_chirho,
        });
    }

    // GHC.Stack.Types
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "callStack",
            "pushCallStack",
            "freezeCallStack",
            "emptyCallStack",
            "fromCallSiteList",
            "getCallStack",
            "srcLocFile",
            "srcLocModule",
            "srcLocStartLine",
            "srcLocEndLine",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("CallStack", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("SrcLoc", &["SrcLoc"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Stack.Types".to_string(),
            exports_chirho,
        });
    }

    // System.Mem.StableName
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["makeStableName", "hashStableName", "eqStableName"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("StableName", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Mem.StableName".to_string(),
            exports_chirho,
        });
    }

    // Data.Array.Byte
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("ByteArray", &["ByteArray"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MutableByteArray", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array.Byte".to_string(),
            exports_chirho,
        });
    }

    // Data.Bifoldable1
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["bifoldMap1", "bifold1"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Bifoldable1", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bifoldable1".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Contravariant
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "contramap",
            "phantom",
            ">$<",
            ">$$<",
            "Predicate",
            "getPredicate",
            "Comparison",
            "getComparison",
            "Equivalence",
            "getEquivalence",
            "Op",
            "getOp",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Contravariant", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Contravariant".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runIdentity"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        exports_chirho.types_chirho.insert(
            "Identity".to_string(),
            IfaceTypeChirho {
                name_chirho: "Identity".to_string(),
                constructors_chirho: vec!["Identity".to_string()],
                methods_chirho: vec!["runIdentity".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Identity".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Compose
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["getCompose"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Compose", &["Compose"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Compose".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Product
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Product", &["Pair"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Product".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Sum
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Sum", &["InL", "InR"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Sum".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Const
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["getConst"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Const".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent.STM / Control.Monad.STM
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "atomically",
            "retry",
            "orElse",
            "throwSTM",
            "catchSTM",
            "newTVar",
            "readTVar",
            "writeTVar",
            "modifyTVar",
            "newTMVar",
            "readTMVar",
            "putTMVar",
            "takeTMVar",
            "newTChan",
            "readTChan",
            "writeTChan",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("STM", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("TVar", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Concurrent.STM".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.STM".to_string(),
            exports_chirho,
        });
    }

    // Data.Primitive.Array / Data.Primitive
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newArray",
            "readArray",
            "writeArray",
            "indexArray",
            "sizeofArray",
            "copyArray",
            "cloneArray",
            "freezeArray",
            "thawArray",
            "runArray",
            "createArray",
            "unsafeFreezeArray",
            "unsafeThawArray",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Array", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MutableArray", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Primitive.Array".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Primitive".to_string(),
            exports_chirho,
        });
    }

    // Data.Text / Data.Text.Encoding
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "pack",
            "unpack",
            "singleton",
            "empty",
            "cons",
            "snoc",
            "append",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "map",
            "intercalate",
            "intersperse",
            "transpose",
            "reverse",
            "replace",
            "toLower",
            "toUpper",
            "toTitle",
            "strip",
            "stripStart",
            "stripEnd",
            "isPrefixOf",
            "isSuffixOf",
            "isInfixOf",
            "words",
            "unwords",
            "lines",
            "unlines",
            "splitOn",
            "splitAt",
            "take",
            "drop",
            "takeWhile",
            "dropWhile",
            "filter",
            "find",
            "partition",
            "index",
            "breakOn",
            "uncons",
            "concatMap",
            "any",
            "all",
            "foldr",
            "foldl",
            "foldl'",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Text", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        let mut enc_exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["encodeUtf8", "decodeUtf8", "decodeUtf8'"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            enc_exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Encoding".to_string(),
            exports_chirho: enc_exports_chirho,
        });
    }

    // Data.ByteString
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "pack",
            "unpack",
            "empty",
            "singleton",
            "cons",
            "snoc",
            "append",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "map",
            "reverse",
            "intercalate",
            "transpose",
            "foldl",
            "foldl'",
            "foldr",
            "concat",
            "concatMap",
            "any",
            "all",
            "take",
            "drop",
            "splitAt",
            "takeWhile",
            "dropWhile",
            "break",
            "span",
            "filter",
            "find",
            "partition",
            "index",
            "elem",
            "notElem",
            "isPrefixOf",
            "isSuffixOf",
            "hGetSome",
            "hGetNonBlocking",
            "hGet",
            "hGetContents",
            "hPut",
            "hPutNonBlocking",
            "hPutStr",
            "readFile",
            "writeFile",
            "appendFile",
            "getLine",
            "getContents",
            "putStr",
            "putStrLn",
            "interact",
            "copy",
            "replicate",
            "unfoldr",
            "unfoldrN",
            "sort",
            "group",
            "groupBy",
            "inits",
            "tails",
            "stripPrefix",
            "stripSuffix",
            "isInfixOf",
            "zip",
            "zipWith",
            "unzip",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ByteString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Data.Bifoldable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "bifold",
            "bifoldMap",
            "bifoldr",
            "bifoldl",
            "bifoldl'",
            "bifoldr'",
            "biList",
            "biany",
            "biall",
            "biconcat",
            "biconcatMap",
            "bitraverse_",
            "bifor_",
            "bimapM_",
            "bisequenceA_",
            "bisequence_",
            "binull",
            "bilength",
            "bielem",
            "bimaximum",
            "biminimum",
            "bisum",
            "biproduct",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Bifoldable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bifoldable".to_string(),
            exports_chirho,
        });
    }

    // Data.Bitraversable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "bitraverse",
            "bisequenceA",
            "bimapM",
            "bifor",
            "bimapDefault",
            "bifoldMapDefault",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Bitraversable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bitraversable".to_string(),
            exports_chirho,
        });
    }

    // Data.Foldable1
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "foldMap1",
            "fold1",
            "toNonEmpty",
            "maximum1",
            "minimum1",
            "head1",
            "last1",
            "foldrMap1",
            "foldlMap1",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Foldable1", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Foldable1".to_string(),
            exports_chirho,
        });
    }

    // Data.Semigroup.Foldable / Data.Semigroup.Traversable compat surface
    for mod_name_chirho in &[
        "Data.Semigroup.Foldable",
        "Data.Semigroup.Traversable",
        "Data.Semigroup.Traversable.Class",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "foldMap1",
            "fold1",
            "toNonEmpty",
            "maximum1",
            "minimum1",
            "head1",
            "last1",
            "foldrMap1",
            "foldlMap1",
            "traverse1",
            "sequence1",
            "traverse1Maybe",
            "gtraverse1",
            "gsequence1",
            "foldMap1Default",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Foldable1", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        exports_chirho.types_chirho.insert(
            "Traversable1".to_string(),
            IfaceTypeChirho {
                name_chirho: "Traversable1".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["traverse1".to_string(), "sequence1".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: (*mod_name_chirho).to_string(),
            exports_chirho,
        });
    }

    // Prelude.Experimental
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        // Re-export everything that Prelude exports (approximation)
        for name_chirho in &[
            "id",
            "const",
            "flip",
            "error",
            "undefined",
            "fmap",
            "pure",
            "return",
            "show",
            "print",
            "putStrLn",
            "putStr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Prelude.Experimental".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Zip
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["mzip", "mzipWith", "munzip"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MonadZip", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Zip".to_string(),
            exports_chirho,
        });
    }

    // GHC.Stack
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "callStack",
            "prettyCallStack",
            "getCallStack",
            "currentCallStack",
            "withFrozenCallStack",
            "freezeCallStack",
            "emptyCallStack",
            "pushCallStack",
            "srcLocFile",
            "srcLocModule",
            "srcLocPackage",
            "srcLocStartLine",
            "srcLocStartCol",
            "srcLocEndLine",
            "srcLocEndCol",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["CallStack", "HasCallStack"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        {
            let (k_chirho, v_chirho) = mk_type_chirho(
                "SrcLoc",
                &[
                    "SrcLoc",
                    "srcLocFile",
                    "srcLocModule",
                    "srcLocPackage",
                    "srcLocStartLine",
                    "srcLocStartCol",
                    "srcLocEndLine",
                    "srcLocEndCol",
                ],
            );
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Stack".to_string(),
            exports_chirho,
        });
    }

    // GHC.Err
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["error", "errorWithoutStackTrace", "undefined"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Err".to_string(),
            exports_chirho,
        });
    }

    // GHC.Prim
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Int#",
            "Word#",
            "Float#",
            "Double#",
            "Char#",
            "Addr#",
            "MutableByteArray#",
            "ByteArray#",
            "Array#",
            "MutableArray#",
            "SmallArray#",
            "SmallMutableArray#",
            "MutVar#",
            "TVar#",
            "MVar#",
            "State#",
            "RealWorld",
            "Weak#",
            "StableName#",
            "StablePtr#",
            "Proxy#",
            "TYPE",
            "RuntimeRep",
            "LiftedRep",
            "UnliftedRep",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "seq",
            "realWorld#",
            "proxy#",
            "void#",
            "coerce",
            // Boxed type constructors (data constructors for unboxed types)
            "I#",
            "W#",
            "D#",
            "F#",
            "C#",
            // Arithmetic primops
            "+#",
            "-#",
            "*#",
            "negateInt#",
            "quotInt#",
            "remInt#",
            "+##",
            "-##",
            "*##",
            "/##",
            "negateDouble#",
            "plusFloat#",
            "minusFloat#",
            "timesFloat#",
            "divideFloat#",
            "negateFloat#",
            "plusWord#",
            "minusWord#",
            "timesWord#",
            "timesWord2#",
            "quotWord#",
            "remWord#",
            // Comparison primops
            ">#",
            ">=#",
            "==#",
            "/=#",
            "<#",
            "<=#",
            "gtWord#",
            "geWord#",
            "eqWord#",
            "neWord#",
            "ltWord#",
            "leWord#",
            // Bitwise / shift primops
            "and#",
            "or#",
            "xor#",
            "not#",
            "uncheckedIShiftL#",
            "uncheckedIShiftRA#",
            "uncheckedIShiftRL#",
            "uncheckedShiftL#",
            "uncheckedShiftRL#",
            "popCnt#",
            "popCnt8#",
            "popCnt16#",
            "popCnt32#",
            "popCnt64#",
            "clz#",
            "clz8#",
            "clz16#",
            "clz32#",
            "clz64#",
            "ctz#",
            "ctz8#",
            "ctz16#",
            "ctz32#",
            "ctz64#",
            "byteSwap#",
            "byteSwap16#",
            "byteSwap32#",
            "byteSwap64#",
            "narrow8Int#",
            "narrow16Int#",
            "narrow32Int#",
            "narrow8Word#",
            "narrow16Word#",
            "narrow32Word#",
            // Conversion primops
            "int2Word#",
            "word2Int#",
            "int2Double#",
            "double2Int#",
            "int2Float#",
            "float2Int#",
            "float2Double#",
            "double2Float#",
            "chr#",
            "ord#",
            "word2Double#",
            "word2Float#",
            // Array primops
            "newArray#",
            "readArray#",
            "writeArray#",
            "indexArray#",
            "sizeofArray#",
            "sizeofMutableArray#",
            "newByteArray#",
            "newPinnedByteArray#",
            "newAlignedPinnedByteArray#",
            "readIntArray#",
            "readWord8Array#",
            "writeIntArray#",
            "writeWord8Array#",
            "writeWord8ArrayAsWord64#",
            "indexIntArray#",
            "indexWord8Array#",
            "indexWord8ArrayAsWord64#",
            "sizeofByteArray#",
            "sizeofMutableByteArray#",
            "getSizeofMutableByteArray#",
            "copyByteArray#",
            "unsafeFreezeByteArray#",
            "byteArrayContents#",
            "isByteArrayPinned#",
            "unsafeFreezeArray#",
            "unsafeThawArray#",
            // MutVar primops
            "newMutVar#",
            "readMutVar#",
            "writeMutVar#",
            // MVar primops
            "newMVar#",
            "takeMVar#",
            "putMVar#",
            "tryTakeMVar#",
            "tryPutMVar#",
            // Weak pointer / stable name primops
            "mkWeak#",
            "deRefWeak#",
            "finalizeWeak#",
            "makeStableName#",
            "eqStableName#",
            "stableNameToInt#",
            // Misc primops
            "dataToTag#",
            "tagToEnum#",
            "reallyUnsafePtrEquality#",
            "touch#",
            "noDuplicate#",
            "raise#",
            "raiseIO#",
            "catch#",
            "maskAsyncExceptions#",
            "unmaskAsyncExceptions#",
            "atomically#",
            "retry#",
            "catchRetry#",
            "catchSTM#",
            "newTVar#",
            "readTVar#",
            "readTVarIO#",
            "writeTVar#",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Prim".to_string(),
            exports_chirho,
        });
    }

    // GHC.Magic
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["inline", "noinline", "lazy", "oneShot", "runRW#"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Magic".to_string(),
            exports_chirho,
        });
    }

    // Data.Char (already exists but GHC.Char doesn't)
    // GHC.Char
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["chr", "ord", "eqChar", "neChar"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Char", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Char".to_string(),
            exports_chirho,
        });
    }

    // System.IO.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "unsafePerformIO",
            "inlinePerformIO",
            "unsafeInterleaveIO",
            "unsafeDupablePerformIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO.Unsafe".to_string(),
            exports_chirho,
        });
    }

    // Data.IORef (already exists via Data.IORef above, add GHC.IORef variant)

    // Control.Concurrent
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "forkIO",
            "forkOS",
            "killThread",
            "throwTo",
            "threadDelay",
            "myThreadId",
            "yield",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ThreadId", "MVar"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Concurrent".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent.MVar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newMVar",
            "newEmptyMVar",
            "takeMVar",
            "putMVar",
            "readMVar",
            "swapMVar",
            "tryTakeMVar",
            "tryPutMVar",
            "isEmptyMVar",
            "withMVar",
            "modifyMVar",
            "modifyMVar_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MVar", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Concurrent.MVar".to_string(),
            exports_chirho,
        });
    }

    // Control.DeepSeq
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["deepseq", "force", "rnf", "rwhnf", "($!!)"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("NFData", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.DeepSeq".to_string(),
            exports_chirho,
        });
    }

    // System.Exit
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["exitWith", "exitFailure", "exitSuccess", "die"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ExitCode", &["ExitSuccess", "ExitFailure"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["ExitSuccess", "ExitFailure"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Exit".to_string(),
            exports_chirho,
        });
    }

    // Data.Bits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            ".&.",
            ".|.",
            "xor",
            "complement",
            "shift",
            "shiftL",
            "shiftR",
            "rotate",
            "rotateL",
            "rotateR",
            "bit",
            "setBit",
            "clearBit",
            "complementBit",
            "testBit",
            "bitSizeMaybe",
            "bitSize",
            "isSigned",
            "popCount",
            "popCountDefault",
            "zeroBits",
            "finiteBitSize",
            "toIntegralSized",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        // Bits class with methods — (..) import brings these in
        exports_chirho.types_chirho.insert(
            "Bits".to_string(),
            IfaceTypeChirho {
                name_chirho: "Bits".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![
                    ".&.".to_string(),
                    ".|.".to_string(),
                    "xor".to_string(),
                    "complement".to_string(),
                    "shift".to_string(),
                    "shiftL".to_string(),
                    "shiftR".to_string(),
                    "rotate".to_string(),
                    "rotateL".to_string(),
                    "rotateR".to_string(),
                    "bit".to_string(),
                    "setBit".to_string(),
                    "clearBit".to_string(),
                    "complementBit".to_string(),
                    "testBit".to_string(),
                    "bitSizeMaybe".to_string(),
                    "bitSize".to_string(),
                    "isSigned".to_string(),
                    "popCount".to_string(),
                    "zeroBits".to_string(),
                    "unsafeShiftL".to_string(),
                    "unsafeShiftR".to_string(),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "FiniteBits".to_string(),
            IfaceTypeChirho {
                name_chirho: "FiniteBits".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![
                    "finiteBitSize".to_string(),
                    "countLeadingZeros".to_string(),
                    "countTrailingZeros".to_string(),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bits".to_string(),
            exports_chirho,
        });
    }

    // Foreign / Foreign.Ptr / Foreign.C / Foreign.Storable / Foreign.ForeignPtr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Ptr",
            "FunPtr",
            "IntPtr",
            "WordPtr",
            "ForeignPtr",
            "StablePtr",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "nullPtr",
            "plusPtr",
            "castPtr",
            "alignPtr",
            "ptrToIntPtr",
            "intPtrToPtr",
            "peek",
            "poke",
            "sizeOf",
            "alignment",
            "malloc",
            "free",
            "alloca",
            "allocaBytes",
            "newForeignPtr",
            "withForeignPtr",
            "unsafeWithForeignPtr",
            "mallocForeignPtr",
            "newStablePtr",
            "deRefStablePtr",
            "freeStablePtr",
            "castStablePtrToPtr",
            "castPtrToStablePtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Storable"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        // Push as multiple module names
        for mod_name_chirho in &[
            "Foreign",
            "Foreign.Ptr",
            "Foreign.ForeignPtr",
            "Foreign.Storable",
            "Foreign.StablePtr",
            "Foreign.Marshal",
            "Foreign.Marshal.Alloc",
            "Foreign.Marshal.Utils",
            "Foreign.Marshal.Pool",
            "Foreign.C",
            "Foreign.C.Types",
            "Foreign.C.String",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // GHC.Ptr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Ptr", "FunPtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["nullPtr", "plusPtr", "castPtr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Ptr".to_string(),
            exports_chirho,
        });
    }

    // Numeric
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "showInt",
            "showHex",
            "showOct",
            "showFloat",
            "readInt",
            "readHex",
            "readOct",
            "readDec",
            "showSigned",
            "readSigned",
            "floatToDigits",
            "showIntAtBase",
            "readFloat",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Numeric".to_string(),
            exports_chirho,
        });
    }

    // Text.Show
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "show",
            "showsPrec",
            "showString",
            "showChar",
            "showParen",
            "shows",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Show", "ShowS"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Show".to_string(),
            exports_chirho,
        });
    }

    // Text.Read
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "read",
            "reads",
            "readParen",
            "lex",
            "readPrec",
            "readListPrec",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Read", "ReadS", "ReadPrec"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Read".to_string(),
            exports_chirho,
        });
    }

    // GHC.Records
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("getField");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("setField");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["HasField"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Records".to_string(),
            exports_chirho,
        });
    }

    // GHC.OverloadedLabels
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("fromLabel");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("IsLabel", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.OverloadedLabels".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Bool
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["If", "Not", "type (&&)", "type (||)"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Bool".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Ord
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Compare", "OrderingI"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Ord".to_string(),
            exports_chirho,
        });
    }

    // GHC.Natural
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["naturalToInteger", "integerToNatural", "naturalToWord"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Natural".to_string(),
            exports_chirho,
        });
    }

    // Numeric.Natural
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Numeric.Natural".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Const
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Const");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("getConst");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Const".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Classes
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "eq1",
            "compare1",
            "showsPrec1",
            "readsPrec1",
            "liftEq",
            "liftCompare",
            "liftShowsPrec",
            "liftReadsPrec",
            "eq2",
            "compare2",
            "showsPrec2",
            "readsPrec2",
            "liftEq2",
            "liftCompare2",
            "liftShowsPrec2",
            "liftReadsPrec2",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Eq1", "Ord1", "Show1", "Read1", "Eq2", "Ord2", "Show2", "Read2",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Classes".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Compose
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Compose", &["Compose"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Compose");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("getCompose");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Compose".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Product
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Product", &["Pair"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Pair");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Product".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Sum
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Sum", &["InL", "InR"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["InL", "InR"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Sum".to_string(),
            exports_chirho,
        });
    }

    // Language.Haskell.TH — Template Haskell types and Q monad
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            // Q monad
            "Q",
            "runQ",
            "newName",
            "lookupTypeName",
            "lookupValueName",
            "reify",
            "reifyRoles",
            "reifyAnnotations",
            "reifyInstances",
            "reifyConStrictness",
            "reifyFixity",
            "reifyType",
            "isInstance",
            "pprint",
            "pprExp",
            "pprPat",
            "pprDec",
            "pprType",
            "location",
            // Syntax construction
            "mkName",
            "nameBase",
            "nameModule",
            "ConT",
            "VarT",
            "AppT",
            "SigT",
            "ForallT",
            "InfixT",
            "ParensT",
            "TupleT",
            "UnboxedTupleT",
            "ArrowT",
            "ListT",
            "StarT",
            "PlainTV",
            "KindedTV",
            // Expression constructors
            "litE",
            "varE",
            "conE",
            "appE",
            "appsE",
            "infixE",
            "infixApp",
            "lamE",
            "lam1E",
            "tupE",
            "unboxedTupE",
            "listE",
            "sigE",
            "recConE",
            "recUpdE",
            "letE",
            "caseE",
            "doE",
            "condE",
            "compE",
            "arithSeqE",
            "fromE",
            "fromThenE",
            "fromToE",
            "fromThenToE",
            "stringE",
            "charL",
            "stringL",
            "integerL",
            "rationalL",
            "intPrimL",
            "wordPrimL",
            // Data constructors
            "AppE",
            "ConE",
            "VarE",
            "LitE",
            "InfixE",
            "LamE",
            "TupE",
            "ListE",
            "SigE",
            // Name helpers
            "tupleDataName",
            "unboxedTupleDataName",
            "tupleTypeName",
            "unboxedTupleTypeName",
            "mkNameG_v",
            "mkNameG_tc",
            "mkNameG_d",
            // Pattern constructors
            "litP",
            "varP",
            "conP",
            "tupP",
            "listP",
            "wildP",
            "asP",
            // Type constructors
            "conT",
            "varT",
            "appT",
            "arrowT",
            "listT",
            "tupleT",
            "sigT",
            "forallT",
            // Declaration constructors
            "funD",
            "valD",
            "dataD",
            "newtypeD",
            "tySynD",
            "classD",
            "instanceD",
            "sigD",
            "pragInlD",
            // Clause and body
            "clause",
            "normalB",
            "guardedB",
            // Literals
            "integerL",
            "rationalL",
            "charL",
            "stringL",
            // Lift class
            "lift",
            "liftTyped",
            // Misc
            "reportError",
            "reportWarning",
            "recover",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Exp",
            "Pat",
            "Type",
            "Dec",
            "Body",
            "Clause",
            "Lit",
            "Name",
            "Info",
            "Loc",
            "Range",
            "Guard",
            "Stmt",
            "Match",
            "Con",
            "Strict",
            "FunDep",
            "Kind",
            "Pred",
            "Cxt",
            "TyVarBndr",
            "TypeQ",
            "ExpQ",
            "PatQ",
            "DecQ",
            "DecsQ",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Language.Haskell.TH".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        // Sub-modules that re-export from the main TH interface
        for sub_chirho in &["Language.Haskell.TH.Ppr", "Language.Haskell.TH.PprLib"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: sub_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Language.Haskell.TH.Syntax
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Lift",
            "lift",
            "liftTyped",
            "mkName",
            "nameBase",
            "ConT",
            "VarT",
            "AppT",
            "SigT",
            "ForallT",
            "InfixT",
            "ParensT",
            "TupleT",
            "UnboxedTupleT",
            "ArrowT",
            "ListT",
            "StarT",
            "PlainTV",
            "KindedTV",
            "Q",
            "runQ",
            "newName",
            "lookupTypeName",
            "lookupValueName",
            "reify",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for (name_chirho, ctors_chirho) in [
            ("Name", &[][..]),
            ("Exp", &[][..]),
            ("Pat", &[][..]),
            (
                "Type",
                &[
                    "ForallT",
                    "AppT",
                    "SigT",
                    "VarT",
                    "ConT",
                    "InfixT",
                    "ParensT",
                    "TupleT",
                    "UnboxedTupleT",
                    "ArrowT",
                    "ListT",
                    "StarT",
                ][..],
            ),
            ("Dec", &[][..]),
            ("Lit", &[][..]),
            ("Kind", &[][..]),
            ("Pred", &[][..]),
            ("Cxt", &[][..]),
            ("TyVarBndr", &["PlainTV", "KindedTV"][..]),
            ("TypeQ", &[][..]),
            ("ExpQ", &[][..]),
            ("PatQ", &[][..]),
            ("DecQ", &[][..]),
            ("DecsQ", &[][..]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, ctors_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Language.Haskell.TH.Syntax".to_string(),
            exports_chirho,
        });
    }

    // Language.Haskell.TH.Quote
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["quoteExp", "quotePat", "quoteType", "quoteDec"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("QuasiQuoter", &["QuasiQuoter"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Language.Haskell.TH.Quote".to_string(),
            exports_chirho,
        });
    }

    // Data.Hashable / Data.Hashable.Lifted
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["hashWithSalt", "hash", "hashUsing"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Hashable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Hashable".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        let mut lifted_exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["liftHashWithSalt", "hashWithSalt1", "hashWithSalt2"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            lifted_exports_chirho
                .values_chirho
                .insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Hashable1", &[]);
        lifted_exports_chirho
            .types_chirho
            .insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Hashable2", &[]);
        lifted_exports_chirho
            .types_chirho
            .insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Hashable.Lifted".to_string(),
            exports_chirho: lifted_exports_chirho,
        });
    }

    // Language.Haskell.TH.Lib
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "litE", "varE", "conE", "appE", "lamE", "tupE", "listE", "litP", "varP", "conP",
            "tupP", "wildP", "conT", "varT", "appT", "arrowT", "funD", "valD", "sigD", "clause",
            "normalB",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Language.Haskell.TH.Lib".to_string(),
            exports_chirho,
        });
    }

    // Language.Haskell.TH.Datatype (th-abstraction)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "reifyDatatype",
            "reifyConstructor",
            "reifyRecord",
            "normalizeInfo",
            "normalizeDec",
            "normalizeCon",
            "datatypeName",
            "datatypeVars",
            "datatypeCons",
            "datatypeInstTypes",
            "datatypeVariant",
            "constructorName",
            "constructorFields",
            "constructorStrictness",
            "constructorVariant",
            "constructorVars",
            "resolveTypeSynonyms",
            "quantifyType",
            "freeVariables",
            "freshenFreeVariables",
            "unifyTypes",
            "applySubstitution",
            "freeVariablesWellScoped",
            "tvName",
            "tvKind",
            "datatypeReturnKind",
            "starK",
            "isStarOrConstraint",
            "equalPred",
            "classPred",
            "asEqualPred",
            "asClassPred",
            "constructorType",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "DatatypeInfo",
            "ConstructorInfo",
            "DatatypeVariant",
            "ConstructorVariant",
            "FieldStrictness",
            "Unpackedness",
            "Strictness",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Language.Haskell.TH.Datatype".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        // Also expose as TyVarBndr sub-module
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Language.Haskell.TH.Datatype.TyVarBndr".to_string(),
            exports_chirho,
        });
    }

    // GHC.Conc — concurrent primitives
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "forkIO",
            "killThread",
            "threadDelay",
            "myThreadId",
            "STM",
            "atomically",
            "retry",
            "orElse",
            "TVar",
            "newTVar",
            "readTVar",
            "writeTVar",
            "newTVarIO",
            "readTVarIO",
            "throwSTM",
            "catchSTM",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["STM", "TVar", "ThreadId"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &["GHC.Conc", "GHC.Conc.Sync"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.IORef (already exists but add Data.IORef.Strict variant)
    // Data.Map.Internal — same exports as Data.Map
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Map",
            "empty",
            "singleton",
            "insert",
            "lookup",
            "member",
            "delete",
            "fromList",
            "toList",
            "toAscList",
            "toDescList",
            "size",
            "null",
            "keys",
            "elems",
            "union",
            "unionWith",
            "intersection",
            "intersectionWith",
            "difference",
            "map",
            "mapWithKey",
            "filter",
            "filterWithKey",
            "foldlWithKey",
            "foldrWithKey",
            "foldlWithKey'",
            "insertWith",
            "insertWithKey",
            "adjust",
            "alter",
            "findWithDefault",
            "mapKeys",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Map", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Map.Internal".to_string(),
            exports_chirho,
        });
    }

    // Data.Set.Internal — same exports as Data.Set
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Set",
            "empty",
            "singleton",
            "insert",
            "member",
            "delete",
            "fromList",
            "toList",
            "toAscList",
            "size",
            "null",
            "union",
            "intersection",
            "difference",
            "map",
            "filter",
            "foldl'",
            "foldr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Set", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Set.Internal".to_string(),
            exports_chirho,
        });
    }

    // Data.Sequence (Data.Seq)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Seq",
            "empty",
            "singleton",
            "fromList",
            "length",
            "null",
            "index",
            "adjust",
            "update",
            "take",
            "drop",
            "splitAt",
            "filter",
            "sort",
            "zip",
            "zipWith",
            "ViewL",
            "viewl",
            "ViewR",
            "viewr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Seq", &["Empty", ":<|"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Sequence".to_string(),
            exports_chirho,
        });
    }

    // Data.IntMap
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IntMap",
            "empty",
            "singleton",
            "insert",
            "lookup",
            "member",
            "delete",
            "fromList",
            "toList",
            "size",
            "null",
            "union",
            "unionWith",
            "intersection",
            "difference",
            "map",
            "filter",
            "foldlWithKey",
            "foldrWithKey",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IntMap", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntMap".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntMap.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntMap.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Data.IntSet
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IntSet",
            "empty",
            "singleton",
            "insert",
            "member",
            "delete",
            "fromList",
            "toList",
            "size",
            "null",
            "union",
            "intersection",
            "difference",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IntSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntSet".to_string(),
            exports_chirho,
        });
    }

    // Data.HashMap.Strict / Data.HashMap.Lazy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "HashMap",
            "empty",
            "singleton",
            "insert",
            "lookup",
            "member",
            "delete",
            "fromList",
            "toList",
            "size",
            "null",
            "union",
            "unionWith",
            "intersection",
            "difference",
            "map",
            "filter",
            "foldlWithKey'",
            "foldrWithKey",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("HashMap", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.HashMap.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.HashMap.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Data.HashSet
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "HashSet",
            "empty",
            "singleton",
            "insert",
            "member",
            "delete",
            "fromList",
            "toList",
            "size",
            "null",
            "union",
            "intersection",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("HashSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.HashSet".to_string(),
            exports_chirho,
        });
    }

    // Data.Text
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Text",
            "pack",
            "unpack",
            "empty",
            "singleton",
            "cons",
            "snoc",
            "append",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "map",
            "intercalate",
            "intersperse",
            "transpose",
            "reverse",
            "toLower",
            "toUpper",
            "toTitle",
            "foldl",
            "foldl'",
            "foldr",
            "concat",
            "concatMap",
            "any",
            "all",
            "maximum",
            "minimum",
            "take",
            "drop",
            "splitAt",
            "takeWhile",
            "dropWhile",
            "strip",
            "stripStart",
            "stripEnd",
            "words",
            "unwords",
            "lines",
            "unlines",
            "uncons",
            "isPrefixOf",
            "isSuffixOf",
            "isInfixOf",
            "replace",
            "breakOn",
            "splitOn",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Text", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text".to_string(),
            exports_chirho,
        });
    }

    // Data.Text.Lazy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Text",
            "pack",
            "unpack",
            "empty",
            "fromStrict",
            "toStrict",
            "uncons",
            "null",
            "length",
            "map",
            "intercalate",
            "concat",
            "take",
            "drop",
            "splitAt",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Text", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Data.Text.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["readFile", "writeFile", "putStr", "putStrLn", "getContents"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.IO".to_string(),
            exports_chirho,
        });
    }

    // Data.Text.Lazy.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["readFile", "writeFile", "putStr", "putStrLn", "getContents"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Lazy.IO".to_string(),
            exports_chirho,
        });
    }

    // Data.ByteString
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ByteString",
            "empty",
            "singleton",
            "pack",
            "unpack",
            "uncons",
            "cons",
            "snoc",
            "append",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "map",
            "reverse",
            "intercalate",
            "foldl",
            "foldl'",
            "foldr",
            "concat",
            "concatMap",
            "take",
            "drop",
            "splitAt",
            "takeWhile",
            "dropWhile",
            "isPrefixOf",
            "isSuffixOf",
            "isInfixOf",
            "readFile",
            "writeFile",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ByteString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Char8".to_string(),
            exports_chirho,
        });
    }

    // Data.ByteString.Lazy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ByteString",
            "empty",
            "pack",
            "unpack",
            "uncons",
            "fromStrict",
            "toStrict",
            "null",
            "length",
            "map",
            "concat",
            "take",
            "drop",
            "readFile",
            "writeFile",
            "getContents",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ByteString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Lazy".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Lazy.Char8".to_string(),
            exports_chirho,
        });
    }

    // Data.Vector
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Vector",
            "empty",
            "singleton",
            "fromList",
            "toList",
            "length",
            "null",
            "head",
            "tail",
            "last",
            "init",
            "map",
            "filter",
            "foldl",
            "foldl'",
            "foldr",
            "take",
            "drop",
            "slice",
            "zip",
            "zipWith",
            "generate",
            "replicate",
            "cons",
            "snoc",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Vector", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Vector".to_string(),
            exports_chirho,
        });
    }

    // GHC.ForeignPtr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ForeignPtr",
            "newForeignPtr",
            "newForeignPtr_",
            "withForeignPtr",
            "unsafeWithForeignPtr",
            "castForeignPtr",
            "mallocForeignPtr",
            "FinalizerPtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ForeignPtr", "FinalizerPtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.ForeignPtr".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.ForeignPtr".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Ptr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Ptr",
            "nullPtr",
            "castPtr",
            "plusPtr",
            "alignPtr",
            "FunPtr",
            "nullFunPtr",
            "castFunPtr",
            "WordPtr",
            "IntPtr",
            "ptrToWordPtr",
            "wordPtrToPtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Ptr", "FunPtr", "WordPtr", "IntPtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Ptr".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Storable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Storable",
            "sizeOf",
            "alignment",
            "peek",
            "poke",
            "peekByteOff",
            "pokeByteOff",
            "peekElemOff",
            "pokeElemOff",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Storable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Storable".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Marshal.Alloc
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "malloc",
            "mallocBytes",
            "calloc",
            "callocBytes",
            "realloc",
            "reallocBytes",
            "free",
            "alloca",
            "allocaBytes",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Marshal.Alloc".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.Types
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CInt", "CUInt", "CLong", "CULong", "CChar", "CUChar", "CShort", "CUShort", "CFloat",
            "CDouble", "CSize", "CLLong", "CULLong", "CBool", "CWchar",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[name_chirho]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            let (k2_chirho, v2_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k2_chirho, v2_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.Types".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.String
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CString",
            "CStringLen",
            "newCString",
            "newCStringLen",
            "withCString",
            "withCStringLen",
            "peekCString",
            "peekCStringLen",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.String".to_string(),
            exports_chirho,
        });
    }

    // Foreign (umbrella module)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Ptr",
            "nullPtr",
            "castPtr",
            "plusPtr",
            "FunPtr",
            "nullFunPtr",
            "castFunPtr",
            "ForeignPtr",
            "newForeignPtr",
            "withForeignPtr",
            "Storable",
            "sizeOf",
            "alignment",
            "peek",
            "poke",
            "malloc",
            "free",
            "alloca",
            "CInt",
            "CUInt",
            "CLong",
            "CChar",
            "CDouble",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign".to_string(),
            exports_chirho,
        });
    }

    // Data.Char (extend existing with more functions)
    // Note: Data.Char already exists above, so this adds Data.Char.Internal
    // GHC.Unicode
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "isAlpha",
            "isAlphaNum",
            "isDigit",
            "isUpper",
            "isLower",
            "isSpace",
            "isPrint",
            "isControl",
            "isPunctuation",
            "toUpper",
            "toLower",
            "toTitle",
            "generalCategory",
            "GeneralCategory",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Unicode".to_string(),
            exports_chirho,
        });
    }

    // Data.Array
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Array",
            "array",
            "listArray",
            "accumArray",
            "bounds",
            "indices",
            "elems",
            "assocs",
            "ixmap",
            "amap",
            "Ix",
            "range",
            "index",
            "inRange",
            "rangeSize",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Array", "Ix"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array".to_string(),
            exports_chirho,
        });
    }

    // Data.IORef (already exists — add Data.IORef.Strict alias)
    // System.IO.Error
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IOError",
            "ioError",
            "userError",
            "mkIOError",
            "isAlreadyExistsError",
            "isDoesNotExistError",
            "isAlreadyInUseError",
            "isFullError",
            "isEOFError",
            "isIllegalOperation",
            "isPermissionError",
            "isUserError",
            "ioeGetErrorType",
            "ioeGetLocation",
            "ioeGetErrorString",
            "ioeGetHandle",
            "ioeGetFileName",
            "tryIOError",
            "catchIOError",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IOError", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO.Error".to_string(),
            exports_chirho,
        });
    }

    // ── Type.Reflection ───────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "TypeRep",
            "SomeTypeRep",
            "Typeable",
            "typeRep",
            "typeRepFingerprint",
            "rnfTypeRep",
            "rnfModule",
            "eqTypeRep",
            "typeRepTyCon",
            "withTypeable",
            "pattern App",
            "pattern Con",
            "pattern Fun",
            "typeOf",
            "someTypeRep",
            "someTypeRepTyCon",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in [
            "TypeRep",
            "SomeTypeRep",
            "Typeable",
            "TyCon",
            "Module",
            "Fingerprint",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Type.Reflection".to_string(),
            exports_chirho,
        });
    }

    // ── Unsafe.Coerce ─────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("unsafeCoerce");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("unsafeCoerce#");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("UnsafeEquality", &["UnsafeRefl"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Unsafe.Coerce".to_string(),
            exports_chirho,
        });
    }

    // ── GHC.ForeignPtr (additional exports) ───────────────────────────
    // Already have Foreign.ForeignPtr, add GHC.ForeignPtr.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "ForeignPtr",
            "newForeignPtr",
            "withForeignPtr",
            "unsafeWithForeignPtr",
            "finalizeForeignPtr",
            "castForeignPtr",
            "plusForeignPtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ForeignPtr", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.ForeignPtr.Internal".to_string(),
            exports_chirho,
        });
    }

    // ── GHC.Fingerprint ──────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in ["Fingerprint", "fingerprintData", "fingerprintString"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Fingerprint", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Fingerprint".to_string(),
            exports_chirho,
        });
        // Also alias as GHC.Fingerprint.Type
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Fingerprint.Type".to_string(),
            exports_chirho: {
                let mut e_chirho = IfaceExportsChirho::default();
                let (k_chirho, v_chirho) = mk_type_chirho("Fingerprint", &[]);
                e_chirho.types_chirho.insert(k_chirho, v_chirho);
                e_chirho
            },
        });
    }

    // ── GHC.Exception ────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "SomeException",
            "Exception",
            "toException",
            "fromException",
            "displayException",
            "throw",
            "throwIO",
            "ErrorCall",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["SomeException", "Exception", "ErrorCall"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Exception".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Exception.Type".to_string(),
            exports_chirho,
        });
    }

    // ── GHC.IO.Exception ─────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "IOException",
            "IOError",
            "ioError",
            "userError",
            "BlockedIndefinitelyOnMVar",
            "BlockedIndefinitelyOnSTM",
            "AsyncException",
            "SomeAsyncException",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in [
            "IOException",
            "IOError",
            "AsyncException",
            "SomeAsyncException",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IO.Exception".to_string(),
            exports_chirho,
        });
    }

    // ── GHC.Arr ──────────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "Array",
            "array",
            "listArray",
            "accumArray",
            "elems",
            "indices",
            "assocs",
            "bounds",
            "(!)",
            "(//)",
            "accum",
            "ixmap",
            "range",
            "index",
            "inRange",
            "rangeSize",
            "Ix",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["Array", "Ix"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Arr".to_string(),
            exports_chirho,
        });
    }

    // ── Text.Printf ──────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "printf",
            "hPrintf",
            "PrintfArg",
            "HPrintfType",
            "PrintfType",
            "formatString",
            "formatInt",
            "formatFloat",
            "formatChar",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["PrintfArg", "PrintfType", "HPrintfType"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Printf".to_string(),
            exports_chirho,
        });
    }

    // ── Text.Read / Text.Show ────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "Read",
            "read",
            "reads",
            "readParen",
            "readPrec",
            "readMaybe",
            "readEither",
            "lex",
            "ReadPrec",
            "ReadS",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["Read", "ReadPrec", "ReadS"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Read".to_string(),
            exports_chirho: exports_chirho.clone(),
        });

        let mut show_exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "Show",
            "show",
            "showsPrec",
            "showString",
            "showParen",
            "ShowS",
            "shows",
            "showChar",
            "showList",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            show_exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["Show", "ShowS"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            show_exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Show".to_string(),
            exports_chirho: show_exports_chirho,
        });
    }

    // ── Data.Fixed ───────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "Fixed",
            "HasResolution",
            "resolution",
            "Pico",
            "Nano",
            "Micro",
            "Milli",
            "Centi",
            "Deci",
            "Uni",
            "E0",
            "E1",
            "E2",
            "E3",
            "E6",
            "E9",
            "E12",
            "showFixed",
            "mod'",
            "div'",
            "divMod'",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in [
            "Fixed",
            "HasResolution",
            "Pico",
            "Nano",
            "Micro",
            "Milli",
            "Centi",
            "Deci",
            "Uni",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Fixed".to_string(),
            exports_chirho,
        });
    }

    // ── Numeric / Numeric.Natural ────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "showInt",
            "showHex",
            "showOct",
            "readInt",
            "readHex",
            "readOct",
            "readDec",
            "readFloat",
            "readSigned",
            "Numeric",
            "fromRat",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Numeric".to_string(),
            exports_chirho,
        });

        let mut nat_exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("Natural");
        nat_exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        nat_exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Numeric.Natural".to_string(),
            exports_chirho: nat_exports_chirho,
        });
    }

    // Data.Type.Equality (21 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho(":~:", &["Refl"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("(:~~:)", &["HRefl"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("TestEquality", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "Refl",
            "HRefl",
            "castWith",
            "gcastWith",
            "apply",
            "inner",
            "outer",
            "sym",
            "trans",
            "testEquality",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Equality".to_string(),
            exports_chirho,
        });
    }

    // Control.Applicative (17 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Applicative", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Alternative", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("WrappedMonad", &["WrapMonad"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("WrappedArrow", &["WrapArrow"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ZipList", &["ZipList"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "pure",
            "liftA",
            "liftA2",
            "liftA3",
            "<*>",
            "*>",
            "<*",
            "empty",
            "<|>",
            "some",
            "many",
            "optional",
            "getConst",
            "Const",
            "ZipList",
            "getZipList",
            "WrapMonad",
            "unwrapMonad",
            "WrapArrow",
            "unwrapArrow",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Applicative".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Identity (12 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Identity", &["Identity"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["Identity", "runIdentity"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Identity".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST (9 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["runST", "fixST", "stToIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.ST".to_string(),
            exports_chirho,
        });
    }

    // GHC.Base (7 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Semigroup", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Monoid", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Functor", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Applicative", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Monad", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("NonEmpty", &["(:|)"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "id",
            "const",
            "flip",
            ".",
            "$",
            "&&",
            "||",
            "not",
            "map",
            "++",
            "foldr",
            "fmap",
            "<>",
            "mempty",
            "mappend",
            "mconcat",
            "pure",
            "return",
            ">>=",
            ">>",
            "=<<",
            "join",
            "ap",
            "otherwise",
            "error",
            "undefined",
            "seq",
            "oneShot",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Base".to_string(),
            exports_chirho,
        });
    }

    // Control.Arrow (6 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Arrow", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowChoice", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowApply", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowZero", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowPlus", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowLoop", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Kleisli", &["Kleisli"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "arr",
            "first",
            "second",
            "***",
            "&&&",
            ">>>",
            "<<<",
            "returnA",
            "left",
            "right",
            "|||",
            "+++",
            "app",
            "Kleisli",
            "runKleisli",
            "loop",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Arrow".to_string(),
            exports_chirho,
        });
    }

    // Data.Semigroup (used by some test files)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Semigroup", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Min", &["Min"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Max", &["Max"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("First", &["First"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Last", &["Last"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Arg", &["Arg"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "<>", "sconcat", "stimes", "Min", "getMin", "Max", "getMax", "First", "getFirst",
            "Last", "getLast", "Arg",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Semigroup".to_string(),
            exports_chirho,
        });
    }

    // Data.Monoid
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Monoid", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Dual", &["Dual"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Endo", &["Endo"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Sum", &["Sum"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Product", &["Product"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("All", &["All"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Any", &["Any"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Ap", &["Ap"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Alt", &["Alt"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "mempty",
            "mappend",
            "mconcat",
            "Dual",
            "getDual",
            "Endo",
            "appEndo",
            "Sum",
            "getSum",
            "Product",
            "getProduct",
            "All",
            "getAll",
            "Any",
            "getAny",
            "Ap",
            "getAp",
            "Alt",
            "getAlt",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Monoid".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor (common import)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Functor", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["fmap", "<$>", "<$", "$>", "void", "<&>"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Const
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["Const", "getConst"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Const".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Compose
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Compose", &["Compose"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["Compose", "getCompose"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Compose".to_string(),
            exports_chirho,
        });
    }

    // Data.Void
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Void", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["absurd", "vacuous"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Void".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Bool (used in DataKinds tests)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("If", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Not", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Bool".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Ord (DataKinds type-level ordering)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Compare", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("OrdCond", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Ord".to_string(),
            exports_chirho,
        });
    }

    // GHC.Stack (commonly imported)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("HasCallStack", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("CallStack", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("SrcLoc", &["SrcLoc"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "callStack",
            "getCallStack",
            "prettyCallStack",
            "prettySrcLoc",
            "currentCallStack",
            "withFrozenCallStack",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Stack".to_string(),
            exports_chirho,
        });
    }

    // Data.STRef (ST mutable references)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("STRef", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["newSTRef", "readSTRef", "writeSTRef", "modifySTRef"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.STRef".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Coercion
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Coercion", &["Coercion"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["Coercion", "coerceWith", "sym", "trans", "repr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Coercion".to_string(),
            exports_chirho,
        });
    }

    // Data.Foldable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Foldable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "fold",
            "foldMap",
            "foldMap'",
            "foldr",
            "foldl",
            "foldl'",
            "foldr'",
            "toList",
            "null",
            "length",
            "elem",
            "maximum",
            "minimum",
            "sum",
            "product",
            "and",
            "or",
            "any",
            "all",
            "concat",
            "concatMap",
            "find",
            "asum",
            "mapM_",
            "forM_",
            "sequenceA_",
            "sequence_",
            "traverse_",
            "for_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Foldable".to_string(),
            exports_chirho,
        });
    }

    // Data.Traversable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Traversable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "traverse",
            "sequenceA",
            "mapM",
            "sequence",
            "for",
            "forM",
            "mapAccumL",
            "mapAccumR",
            "fmapDefault",
            "foldMapDefault",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Traversable".to_string(),
            exports_chirho,
        });
    }

    // GHC.IO (internal module needed by some tests)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IO", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "unsafePerformIO",
            "unsafeInterleaveIO",
            "unsafeDupablePerformIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IO".to_string(),
            exports_chirho,
        });
    }

    // System.IO.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "unsafePerformIO",
            "inlinePerformIO",
            "unsafeInterleaveIO",
            "unsafeDupablePerformIO",
            "unsafeFixIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO.Unsafe".to_string(),
            exports_chirho,
        });
    }

    // Data.IORef (some tests import this directly)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IORef", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "newIORef",
            "readIORef",
            "writeIORef",
            "modifyIORef",
            "modifyIORef'",
            "atomicModifyIORef",
            "atomicModifyIORef'",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IORef".to_string(),
            exports_chirho,
        });
    }

    // GHC.List (re-exports from GHC internal)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "map",
            "filter",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "reverse",
            "foldl",
            "foldl'",
            "foldr",
            "foldr'",
            "scanl",
            "scanr",
            "iterate",
            "repeat",
            "replicate",
            "cycle",
            "take",
            "drop",
            "splitAt",
            "takeWhile",
            "dropWhile",
            "span",
            "break",
            "elem",
            "notElem",
            "lookup",
            "zip",
            "zip3",
            "zipWith",
            "zipWith3",
            "unzip",
            "unzip3",
            "lines",
            "words",
            "unlines",
            "unwords",
            "concat",
            "concatMap",
            "and",
            "or",
            "any",
            "all",
            "sum",
            "product",
            "maximum",
            "minimum",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.List".to_string(),
            exports_chirho,
        });
    }

    // ── Batch 5: additional commonly imported modules ──────────────

    // Data.Proxy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Proxy", &["Proxy"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Proxy");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("asProxyTypeOf");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Proxy".to_string(),
            exports_chirho,
        });
    }

    // Data.Typeable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Typeable", "TypeRep", "Proxy"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "typeOf",
            "typeRep",
            "cast",
            "gcast",
            "eqT",
            "typeRepTyCon",
            "mkTyCon3",
            "mkTyConApp",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Typeable".to_string(),
            exports_chirho,
        });
    }

    // Data.Coerce
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Coercible", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("coerce");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Coerce".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeLits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Nat",
            "Symbol",
            "KnownNat",
            "KnownSymbol",
            "SomeNat",
            "SomeSymbol",
            "TypeError",
            "ErrorMessage",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "natVal",
            "natVal'",
            "symbolVal",
            "symbolVal'",
            "someNatVal",
            "someSymbolVal",
            "sameNat",
            "sameSymbol",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.TypeLits".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeNats (re-exports from GHC.TypeLits plus extras)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Nat", "KnownNat", "SomeNat"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["natVal", "natVal'", "someNatVal", "sameNat"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.TypeNats".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Storable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Storable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "sizeOf",
            "alignment",
            "peek",
            "poke",
            "peekElemOff",
            "pokeElemOff",
            "peekByteOff",
            "pokeByteOff",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Storable".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Ptr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Ptr", "FunPtr", "WordPtr", "IntPtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "nullPtr",
            "castPtr",
            "plusPtr",
            "alignPtr",
            "minusPtr",
            "nullFunPtr",
            "castFunPtr",
            "freeHaskellFunPtr",
            "ptrToWordPtr",
            "wordPtrToPtr",
            "ptrToIntPtr",
            "intPtrToPtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Ptr".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent.MVar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("MVar", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "newMVar",
            "newEmptyMVar",
            "takeMVar",
            "putMVar",
            "readMVar",
            "swapMVar",
            "tryTakeMVar",
            "tryPutMVar",
            "isEmptyMVar",
            "withMVar",
            "modifyMVar",
            "modifyMVar_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Concurrent.MVar".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("MonadTrans", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("lift");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Class".to_string(),
            exports_chirho,
        });
    }

    // Text.PrettyPrint
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Doc", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "text",
            "char",
            "int",
            "integer",
            "empty",
            "nest",
            "sep",
            "fsep",
            "hsep",
            "vcat",
            "hcat",
            "hang",
            "punctuate",
            "render",
            "parens",
            "brackets",
            "braces",
            "quotes",
            "doubleQuotes",
            "comma",
            "colon",
            "semi",
            "space",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.PrettyPrint".to_string(),
            exports_chirho,
        });
    }

    // Data.Data
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Data",
            "Typeable",
            "Constr",
            "DataType",
            "DataRep",
            "ConstrRep",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "toConstr",
            "gunfold",
            "gfoldl",
            "dataTypeOf",
            "mkConstr",
            "mkDataType",
            "constrIndex",
            "showConstr",
            "dataTypeConstrs",
            "dataTypeName",
            "constrType",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Data".to_string(),
            exports_chirho,
        });
    }

    // Data.Word
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Word", "Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Word".to_string(),
            exports_chirho,
        });
    }

    // Data.Int
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Int", "Int8", "Int16", "Int32", "Int64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Int".to_string(),
            exports_chirho,
        });
    }

    // Data.Map / Data.Map.Strict / Data.Map.Lazy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Map", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "lookup",
            "member",
            "notMember",
            "findWithDefault",
            "union",
            "unionWith",
            "intersection",
            "difference",
            "map",
            "mapWithKey",
            "filter",
            "filterWithKey",
            "foldlWithKey'",
            "foldrWithKey",
            "toList",
            "fromList",
            "toAscList",
            "toDescList",
            "elems",
            "keys",
            "size",
            "null",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &["Data.Map", "Data.Map.Strict", "Data.Map.Lazy"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Set
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Set", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "member",
            "notMember",
            "size",
            "null",
            "union",
            "intersection",
            "difference",
            "map",
            "filter",
            "foldl'",
            "foldr",
            "toList",
            "fromList",
            "toAscList",
            "toDescList",
            "elems",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Set".to_string(),
            exports_chirho,
        });
    }

    // GHC.Generics
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Generic",
            "Generic1",
            "Rep",
            "Rep1",
            "V1",
            "U1",
            "K1",
            "M1",
            "Par1",
            "Rec0",
            "Rec1",
            "D1",
            "C1",
            "S1",
            "Meta",
            "Datatype",
            "Constructor",
            "Selector",
            "Fixity",
            "Associativity",
            "SourceUnpackedness",
            "SourceStrictness",
            "DecidedStrictness",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "from",
            "to",
            "from1",
            "to1",
            "datatypeName",
            "moduleName",
            "packageName",
            "conName",
            "conFixity",
            "conIsRecord",
            "selName",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Generics".to_string(),
            exports_chirho,
        });
    }

    // ── Batch 5b: missing utility modules ──────────────────────────

    // Debug.Trace
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "trace",
            "traceShow",
            "traceShowId",
            "traceIO",
            "traceM",
            "traceShowM",
            "traceStack",
            "traceEvent",
            "traceEventIO",
            "traceMarker",
            "traceMarkerIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Debug.Trace".to_string(),
            exports_chirho,
        });
    }

    // Data.Bits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Bits", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("FiniteBits", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            ".&.",
            ".|.",
            "xor",
            "complement",
            "shift",
            "rotate",
            "setBit",
            "clearBit",
            "complementBit",
            "testBit",
            "bit",
            "zeroBits",
            "shiftL",
            "shiftR",
            "rotateL",
            "rotateR",
            "popCount",
            "popCountDefault",
            "bitSize",
            "bitSizeMaybe",
            "isSigned",
            "unsafeShiftL",
            "unsafeShiftR",
            "finiteBitSize",
            "countLeadingZeros",
            "countTrailingZeros",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bits".to_string(),
            exports_chirho,
        });
    }

    // Data.Complex
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Complex", &[":+"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            ":+",
            "realPart",
            "imagPart",
            "mkPolar",
            "cis",
            "polar",
            "magnitude",
            "phase",
            "conjugate",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Complex".to_string(),
            exports_chirho,
        });
    }

    // Data.Ratio
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Ratio", &[":%"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Rational", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["(%)", "numerator", "denominator", "approxRational"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Ratio".to_string(),
            exports_chirho,
        });
    }

    // Data.Fixed
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Fixed",
            "HasResolution",
            "E0",
            "E1",
            "E2",
            "E3",
            "E6",
            "E9",
            "E12",
            "Uni",
            "Deci",
            "Centi",
            "Milli",
            "Micro",
            "Nano",
            "Pico",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["resolution", "showFixed", "mod'", "divMod'", "div'"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Fixed".to_string(),
            exports_chirho,
        });
    }

    // Data.Unique
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Unique", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["newUnique", "hashUnique"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Unique".to_string(),
            exports_chirho,
        });
    }

    // System.Exit
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("ExitCode", &["ExitSuccess", "ExitFailure"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "ExitSuccess",
            "ExitFailure",
            "exitWith",
            "exitSuccess",
            "exitFailure",
            "die",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Exit".to_string(),
            exports_chirho,
        });
    }

    // Data.Dynamic
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Dynamic", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "toDyn",
            "fromDyn",
            "fromDynamic",
            "dynTypeRep",
            "dynApply",
            "dynApp",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Dynamic".to_string(),
            exports_chirho,
        });
    }

    // Control.DeepSeq
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("NFData", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["deepseq", "rnf", "force", "($!!)", "NFData"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.DeepSeq".to_string(),
            exports_chirho,
        });
    }

    // Data.Sequence (Data.Sequence)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Seq", &["Empty", ":<|", ":|>"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ViewL", &["EmptyL", ":<"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ViewR", &["EmptyR", ":>"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "(<|)",
            "(|>)",
            "(><)",
            "fromList",
            "length",
            "null",
            "index",
            "adjust",
            "update",
            "take",
            "drop",
            "splitAt",
            "viewl",
            "viewr",
            "filter",
            "sort",
            "reverse",
            "zip",
            "zipWith",
            "unzip",
            "replicate",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Sequence".to_string(),
            exports_chirho,
        });
    }

    // Data.IntMap
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IntMap", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "lookup",
            "member",
            "size",
            "null",
            "union",
            "intersection",
            "difference",
            "map",
            "mapWithKey",
            "filter",
            "filterWithKey",
            "foldlWithKey'",
            "foldrWithKey",
            "toList",
            "fromList",
            "toAscList",
            "keys",
            "elems",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntMap".to_string(),
            exports_chirho,
        });
    }

    // Data.IntSet
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IntSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "member",
            "notMember",
            "size",
            "null",
            "union",
            "intersection",
            "difference",
            "filter",
            "foldl'",
            "foldr",
            "toList",
            "fromList",
            "toAscList",
            "elems",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntSet".to_string(),
            exports_chirho,
        });
    }

    // Data.HashMap.Strict (from unordered-containers)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("HashMap", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "lookup",
            "member",
            "size",
            "null",
            "union",
            "intersection",
            "difference",
            "map",
            "mapWithKey",
            "filter",
            "filterWithKey",
            "foldlWithKey'",
            "foldrWithKey",
            "toList",
            "fromList",
            "keys",
            "elems",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.HashMap.Strict".to_string(),
            exports_chirho,
        });
    }

    // Data.HashSet (from unordered-containers)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("HashSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "member",
            "size",
            "null",
            "union",
            "intersection",
            "difference",
            "filter",
            "foldl'",
            "foldr",
            "toList",
            "fromList",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.HashSet".to_string(),
            exports_chirho,
        });
    }

    // Data.Hashable (from hashable)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Hashable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["hash", "hashWithSalt", "hashUsing"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Hashable".to_string(),
            exports_chirho,
        });
    }

    // GHC.Records
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("HasField", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("getField");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Records".to_string(),
            exports_chirho,
        });
    }

    // GHC.Natural
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "naturalToInteger",
            "naturalFromInteger",
            "naturalToWord",
            "wordToNatural",
            "intToNatural",
            "minusNaturalMaybe",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Natural".to_string(),
            exports_chirho,
        });
    }

    // GHC.Num
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Num", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "+",
            "-",
            "*",
            "negate",
            "abs",
            "signum",
            "fromInteger",
            "subtract",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Num".to_string(),
            exports_chirho,
        });
    }

    // GHC.Real
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Integral",
            "Fractional",
            "Real",
            "RealFrac",
            "Ratio",
            "Rational",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "toInteger",
            "toRational",
            "fromIntegral",
            "realToFrac",
            "quot",
            "rem",
            "div",
            "mod",
            "quotRem",
            "divMod",
            "(%)",
            "numerator",
            "denominator",
            "ceiling",
            "floor",
            "round",
            "truncate",
            "properFraction",
            "recip",
            "fromRational",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Real".to_string(),
            exports_chirho,
        });
    }

    // GHC.Float
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Float", "Double", "Floating", "RealFloat"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "pi",
            "exp",
            "log",
            "sqrt",
            "sin",
            "cos",
            "tan",
            "asin",
            "acos",
            "atan",
            "sinh",
            "cosh",
            "tanh",
            "asinh",
            "acosh",
            "atanh",
            "(**)",
            "logBase",
            "floatRadix",
            "floatDigits",
            "floatRange",
            "decodeFloat",
            "encodeFloat",
            "exponent",
            "significand",
            "scaleFloat",
            "isNaN",
            "isInfinite",
            "isDenormalized",
            "isNegativeZero",
            "isIEEE",
            "atan2",
            "float2Double",
            "double2Float",
            "int2Double",
            "int2Float",
            "showFloat",
            "showEFloat",
            "showFFloat",
            "showGFloat",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Float".to_string(),
            exports_chirho,
        });
    }

    // GHC.Show
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        // Show class with methods for (..) import
        exports_chirho.types_chirho.insert(
            "Show".to_string(),
            IfaceTypeChirho {
                name_chirho: "Show".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![
                    "showsPrec".to_string(),
                    "show".to_string(),
                    "showList".to_string(),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        for name_chirho in &[
            "show",
            "showsPrec",
            "showString",
            "showChar",
            "showParen",
            "shows",
            "showList__",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Show".to_string(),
            exports_chirho,
        });
    }

    // GHC.Read
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Read", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["read", "reads", "readPrec", "readList", "readParen", "lex"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Read".to_string(),
            exports_chirho,
        });
    }

    // GHC.Enum
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Enum", "Bounded"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "succ",
            "pred",
            "toEnum",
            "fromEnum",
            "enumFrom",
            "enumFromThen",
            "enumFromTo",
            "enumFromThenTo",
            "minBound",
            "maxBound",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Enum".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans — umbrella re-export module
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["lift", "liftIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["MonadTrans", "MonadIO"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.State / Control.Monad.Trans.State.Lazy
    for mod_name_chirho in &[
        "Control.Monad.Trans.State",
        "Control.Monad.Trans.State.Lazy",
        "Control.Monad.Trans.State.Strict",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "get",
            "put",
            "modify",
            "modify'",
            "gets",
            "evalState",
            "execState",
            "runState",
            "evalStateT",
            "execStateT",
            "runStateT",
            "state",
            "withState",
            "mapState",
            "mapStateT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["StateT", "State"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["StateT"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Writer / Control.Monad.Trans.Writer.Lazy
    for mod_name_chirho in &[
        "Control.Monad.Trans.Writer",
        "Control.Monad.Trans.Writer.Lazy",
        "Control.Monad.Trans.Writer.Strict",
        "Control.Monad.Trans.Writer.CPS",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "tell",
            "listen",
            "pass",
            "censor",
            "runWriter",
            "execWriter",
            "runWriterT",
            "execWriterT",
            "writer",
            "mapWriter",
            "mapWriterT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["WriterT", "Writer"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["WriterT"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Fail
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("fail");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadFail", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Fail".to_string(),
            exports_chirho,
        });
    }

    // Control.Exception.Context
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["addExceptionContext", "getExceptionAnnotations"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ExceptionContext"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["ExceptionContext"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Exception.Context".to_string(),
            exports_chirho,
        });
    }

    // System.Environment
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "getArgs",
            "getProgName",
            "getEnv",
            "getEnvironment",
            "lookupEnv",
            "setEnv",
            "unsetEnv",
            "withArgs",
            "withProgName",
            "getExecutablePath",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Environment".to_string(),
            exports_chirho,
        });
    }

    // Data.Tree
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "drawTree",
            "drawForest",
            "flatten",
            "levels",
            "foldTree",
            "unfoldTree",
            "unfoldForest",
            "unfoldTreeM",
            "unfoldForestM",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Tree"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["Node"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Forest"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Tree".to_string(),
            exports_chirho,
        });
    }

    // GHC.STRef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["newSTRef", "readSTRef", "writeSTRef"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("STRef", &["STRef"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.STRef".to_string(),
            exports_chirho,
        });
    }

    // Data.Binary
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["encode", "decode", "encodeFile", "decodeFile", "put", "get"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Binary"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Binary".to_string(),
            exports_chirho,
        });
    }

    // Type.Reflection.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["mkTrApp", "mkTrCon"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Type.Reflection.Unsafe".to_string(),
            exports_chirho,
        });
    }

    // Data.Coerce
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["coerce", "Coercible"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Coercible"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Coerce".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runIdentity"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        exports_chirho.types_chirho.insert(
            "Identity".to_string(),
            IfaceTypeChirho {
                name_chirho: "Identity".to_string(),
                constructors_chirho: vec!["Identity".to_string()],
                methods_chirho: vec!["runIdentity".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Identity".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeLits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "natVal",
            "natVal'",
            "symbolVal",
            "symbolVal'",
            "sameNat",
            "sameSymbol",
            "someNatVal",
            "someSymbolVal",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Nat",
            "Symbol",
            "KnownNat",
            "KnownSymbol",
            "SomeNat",
            "SomeSymbol",
            "TypeError",
            "ErrorMessage",
            "AppendSymbol",
            "CmpNat",
            "CmpSymbol",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.TypeLits".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeNats
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["natVal", "natVal'", "sameNat", "someNatVal"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Nat", "KnownNat", "SomeNat", "CmpNat"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.TypeNats".to_string(),
            exports_chirho,
        });
    }

    // Data.Set
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "member",
            "notMember",
            "null",
            "size",
            "union",
            "intersection",
            "difference",
            "isSubsetOf",
            "toList",
            "fromList",
            "toAscList",
            "toDescList",
            "map",
            "filter",
            "foldl'",
            "foldr",
            "elems",
            "findMin",
            "findMax",
            "deleteMin",
            "deleteMax",
            "lookupMin",
            "lookupMax",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Set"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Set".to_string(),
            exports_chirho,
        });
    }

    // Data.STRef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newSTRef",
            "readSTRef",
            "writeSTRef",
            "modifySTRef",
            "modifySTRef'",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["STRef"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.STRef".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runST", "fixST", "stToIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ST"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.ST".to_string(),
            exports_chirho,
        });
    }

    // Data.Array.MArray
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newArray",
            "newArray_",
            "readArray",
            "writeArray",
            "getBounds",
            "getElems",
            "getAssocs",
            "freeze",
            "thaw",
            "mapArray",
            "mapIndices",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["MArray"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array.MArray".to_string(),
            exports_chirho,
        });
    }

    // Data.Array.ST
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runSTArray"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["STArray"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array.ST".to_string(),
            exports_chirho,
        });
    }

    // Data.Typeable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "typeOf",
            "typeRep",
            "cast",
            "gcast",
            "eqT",
            "typeRepTyCon",
            "typeRepFingerprint",
            "mkFunTy",
            "rnfTyCon",
            "rnfTypeRep",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Typeable", "TypeRep", "TyCon"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Typeable".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Storable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "sizeOf",
            "alignment",
            "peek",
            "poke",
            "peekElemOff",
            "pokeElemOff",
            "peekByteOff",
            "pokeByteOff",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Storable"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Storable".to_string(),
            exports_chirho,
        });
    }

    // GHC.Generics
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "from",
            "to",
            "from1",
            "to1",
            "datatypeName",
            "moduleName",
            "packageName",
            "conName",
            "conFixity",
            "conIsRecord",
            "selName",
            "selSourceUnpackedness",
            "selSourceStrictness",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Generic",
            "Generic1",
            "Rep",
            "Rep1",
            "V1",
            "U1",
            "K1",
            "M1",
            "Par1",
            "Rec1",
            "D1",
            "C1",
            "S1",
            "R",
            "D",
            "C",
            "S",
            "P",
            "Fixity",
            "Associativity",
            "SourceUnpackedness",
            "SourceStrictness",
            "DecidedStrictness",
            "Meta",
            "Datatype",
            "Constructor",
            "Selector",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Generics".to_string(),
            exports_chirho,
        });
    }

    // Data.Word
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Word", "Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Word".to_string(),
            exports_chirho,
        });
    }

    // Text.Parsec.Prim
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "parse",
            "runParser",
            "try",
            "lookAhead",
            "many",
            "skipMany",
            "unexpected",
            "parserFail",
            "getState",
            "putState",
            "modifyState",
            "getPosition",
            "setPosition",
            "tokenPrim",
            "tokenPrimEx",
            "tokens",
            "label",
            "(<?>)",
            "parserZero",
            "parserPlus",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Parsec", "ParsecT", "Stream"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Parsec.Prim".to_string(),
            exports_chirho,
        });
    }

    // Text.Parsec.Combinator
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "choice",
            "count",
            "between",
            "option",
            "optionMaybe",
            "optional",
            "many1",
            "skipMany1",
            "sepBy",
            "sepBy1",
            "endBy",
            "endBy1",
            "sepEndBy",
            "sepEndBy1",
            "chainl",
            "chainl1",
            "chainr",
            "chainr1",
            "eof",
            "notFollowedBy",
            "manyTill",
            "anyToken",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Parsec.Combinator".to_string(),
            exports_chirho,
        });
    }

    // Text.Parsec
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "parse",
            "parseTest",
            "try",
            "lookAhead",
            "many",
            "many1",
            "skipMany",
            "skipMany1",
            "choice",
            "option",
            "optional",
            "between",
            "sepBy",
            "sepBy1",
            "endBy",
            "endBy1",
            "chainl1",
            "chainr1",
            "eof",
            "char",
            "string",
            "anyChar",
            "oneOf",
            "noneOf",
            "digit",
            "letter",
            "spaces",
            "newline",
            "tab",
            "upper",
            "lower",
            "alphaNum",
            "satisfy",
            "(<|>)",
            "(<?>)",
            "manyTill",
            "lookAhead",
            "notFollowedBy",
            "eof",
            "between",
            "option",
            "optional",
            "skipMany",
            "skipMany1",
            "sepBy",
            "sepBy1",
            "endBy",
            "endBy1",
            "chainl",
            "chainl1",
            "chainr",
            "chainr1",
            "count",
            "choice",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Parsec", "ParsecT", "SourcePos", "ParseError"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Parsec".to_string(),
            exports_chirho,
        });
    }

    // Data.Foldable (extend existing with additional exports)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "fold",
            "foldMap",
            "foldMap'",
            "foldr",
            "foldr'",
            "foldl",
            "foldl'",
            "toList",
            "null",
            "length",
            "elem",
            "maximum",
            "minimum",
            "sum",
            "product",
            "any",
            "all",
            "and",
            "or",
            "find",
            "notElem",
            "maximumBy",
            "minimumBy",
            "concat",
            "concatMap",
            "asum",
            "traverse_",
            "for_",
            "mapM_",
            "forM_",
            "sequenceA_",
            "sequence_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Foldable"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Foldable".to_string(),
            exports_chirho,
        });
    }

    // Data.Kind
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Type", "Constraint", "FUN"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Kind".to_string(),
            exports_chirho,
        });
    }

    // Data.IntMap / Data.IntMap.Strict
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["IntMap", "Key"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "lookup",
            "member",
            "notMember",
            "findWithDefault",
            "union",
            "unionWith",
            "intersection",
            "difference",
            "map",
            "mapWithKey",
            "filter",
            "filterWithKey",
            "foldlWithKey'",
            "foldrWithKey",
            "toList",
            "fromList",
            "toAscList",
            "elems",
            "keys",
            "size",
            "null",
            "mapKeys",
            "mapKeysMonotonic",
            "adjust",
            "alter",
            "intersectionWith",
            "differenceWith",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &["Data.IntMap", "Data.IntMap.Strict", "Data.IntMap.Lazy"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Functor.Compose
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Compose"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["Compose"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Compose", "getCompose"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Compose".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Const
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Const"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["Const"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Const", "getConst"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Const".to_string(),
            exports_chirho,
        });
    }

    // Data.Monoid (extend)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Monoid", "Dual", "Endo", "All", "Any", "Sum", "Product", "First", "Last", "Ap", "Alt",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "mempty",
            "mappend",
            "mconcat",
            "Dual",
            "getDual",
            "Endo",
            "appEndo",
            "All",
            "getAll",
            "Any",
            "getAny",
            "Sum",
            "getSum",
            "Product",
            "getProduct",
            "First",
            "getFirst",
            "Last",
            "getLast",
            "Ap",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Monoid".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.IO.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["MonadIO"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["liftIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.IO.Class".to_string(),
            exports_chirho,
        });
    }

    // System.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Handle", "IOMode", "BufferMode", "SeekMode"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "stdin",
            "stdout",
            "stderr",
            "openFile",
            "hClose",
            "hFlush",
            "hPutStr",
            "hPutStrLn",
            "hPutChar",
            "hGetLine",
            "hGetContents",
            "hGetChar",
            "hSetBuffering",
            "hSetEncoding",
            "hIsEOF",
            "isEOF",
            "withFile",
            "readFile",
            "writeFile",
            "appendFile",
            "ReadMode",
            "WriteMode",
            "AppendMode",
            "ReadWriteMode",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["ThreadId", "MVar"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "forkIO",
            "forkOS",
            "killThread",
            "throwTo",
            "threadDelay",
            "myThreadId",
            "newMVar",
            "readMVar",
            "takeMVar",
            "putMVar",
            "newEmptyMVar",
            "tryTakeMVar",
            "tryPutMVar",
            "modifyMVar",
            "modifyMVar_",
            "withMVar",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Concurrent".to_string(),
            exports_chirho,
        });
    }

    // Data.Sequence
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Seq"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "empty",
            "singleton",
            "fromList",
            "toList",
            "length",
            "null",
            "index",
            "lookup",
            "(<|)",
            "(|>)",
            "(><)",
            "filter",
            "sort",
            "reverse",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Sequence".to_string(),
            exports_chirho,
        });
    }

    // Data.Map — extend with missing commonly-used exports
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "mapKeys",
            "mapKeysMonotonic",
            "adjust",
            "alter",
            "intersectionWith",
            "differenceWith",
            "isSubmapOf",
            "isProperSubmapOf",
            "restrictKeys",
            "withoutKeys",
            "(!?)",
            "(!)",
            "findMin",
            "findMax",
            "deleteMin",
            "deleteMax",
            "lookupMin",
            "lookupMax",
            "updateWithKey",
            "insertWithKey",
            "foldlWithKey",
            "foldrWithKey'",
            "mapAccum",
            "mapAccumWithKey",
            "mergeWithKey",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &["Data.Map", "Data.Map.Strict", "Data.Map.Lazy"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Foreign.C.ConstPtr (GHC 9.10+)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("ConstPtr", &["ConstPtr"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["ConstPtr", "unConstPtr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.ConstPtr".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.Types
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CInt",
            "CChar",
            "CShort",
            "CLong",
            "CLLong",
            "CUInt",
            "CUChar",
            "CUShort",
            "CULong",
            "CULLong",
            "CFloat",
            "CDouble",
            "CSize",
            "CSsize",
            "CPtrdiff",
            "CBool",
            "CWchar",
            "CSigAtomic",
            "CClock",
            "CTime",
            "CIntPtr",
            "CUIntPtr",
            "CIntMax",
            "CUIntMax",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[name_chirho]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.Types".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.String
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CString",
            "CStringLen",
            "peekCString",
            "peekCStringLen",
            "newCString",
            "newCStringLen",
            "withCString",
            "withCStringLen",
            "castCharToCChar",
            "castCCharToChar",
            "castCUCharToChar",
            "castCharToCUChar",
            "CWString",
            "CWStringLen",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.String".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.Error
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Errno",
            "eOK",
            "e2BIG",
            "eACCES",
            "eAGAIN",
            "eBADF",
            "eBUSY",
            "eCHILD",
            "eDEADLK",
            "eEXIST",
            "eFAULT",
            "eFBIG",
            "eINTR",
            "eINVAL",
            "eIO",
            "eISDIR",
            "eMFILE",
            "eMLINK",
            "eNAMETOOLONG",
            "eNFILE",
            "eNODEV",
            "eNOENT",
            "eNOEXEC",
            "eNOLCK",
            "eNOMEM",
            "eNOSPC",
            "eNOSYS",
            "eNOTDIR",
            "eNOTEMPTY",
            "eNOTTY",
            "eNXIO",
            "ePERM",
            "ePIPE",
            "eRANGE",
            "eROFS",
            "eSPIPE",
            "eSRCH",
            "eXDEV",
            "isValidErrno",
            "getErrno",
            "resetErrno",
            "throwErrno",
            "throwErrnoIf",
            "throwErrnoIf_",
            "throwErrnoIfRetry",
            "throwErrnoIfMinus1",
            "throwErrnoIfNull",
            "throwErrnoPath",
            "throwErrnoPathIf",
            "throwErrnoPathIfNull",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Errno", &["Errno"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.Error".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Marshal.Alloc / Foreign.Marshal.Array / Foreign.Marshal.Utils
    for mod_name_chirho in &[
        "Foreign.Marshal.Alloc",
        "Foreign.Marshal.Array",
        "Foreign.Marshal.Utils",
        "Foreign.Marshal",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "malloc",
            "mallocBytes",
            "alloca",
            "allocaBytes",
            "realloc",
            "reallocBytes",
            "free",
            "finalizerFree",
            "mallocArray",
            "mallocArray0",
            "allocaArray",
            "allocaArray0",
            "reallocArray",
            "reallocArray0",
            "peekArray",
            "peekArray0",
            "pokeArray",
            "pokeArray0",
            "newArray",
            "newArray0",
            "withArray",
            "withArray0",
            "withArrayLen",
            "withArrayLen0",
            "advancePtr",
            "copyArray",
            "moveArray",
            "lengthArray0",
            "new",
            "with",
            "fromBool",
            "toBool",
            "maybeNew",
            "maybeWith",
            "maybePeek",
            "withMany",
            "copyBytes",
            "moveBytes",
            "fillBytes",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Foreign.C (umbrella)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CInt",
            "CChar",
            "CShort",
            "CLong",
            "CUInt",
            "CFloat",
            "CDouble",
            "CSize",
            "CString",
            "CStringLen",
            "CWString",
            "Errno",
            "castCharToCChar",
            "castCCharToChar",
            "peekCString",
            "newCString",
            "withCString",
            "ConstPtr",
            "throwErrnoIfMinus1_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C".to_string(),
            exports_chirho,
        });
    }

    // Test.HUnit.Lang (for HUnit internals)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Assertion",
            "assertFailure",
            "assertEqual",
            "assertBool",
            "HUnitFailure",
            "FailureReason",
            "formatFailureReason",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["HUnitFailure", "FailureReason"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Test.HUnit.Lang".to_string(),
            exports_chirho,
        });
    }

    // Data.Default (data-default)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("def");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Default", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &["Data.Default", "Data.Default.Class"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Sequence / Data.Sequence.Internal (extend)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Seq",
            "empty",
            "singleton",
            "fromList",
            "toList",
            "length",
            "null",
            "index",
            "adjust",
            "update",
            "take",
            "drop",
            "splitAt",
            "viewl",
            "viewr",
            "ViewL",
            "ViewR",
            "EmptyL",
            "EmptyR",
            ":<",
            ":>",
            "|>",
            "<|",
            "><",
            "filter",
            "sort",
            "reverse",
            "zip",
            "zipWith",
            "unzip",
            "lookup",
            "insertAt",
            "deleteAt",
            "findIndexL",
            "findIndexR",
            "foldlWithIndex",
            "foldrWithIndex",
            "mapWithIndex",
            "traverseWithIndex",
            "replicate",
            "replicateA",
            "replicateM",
            "unfoldl",
            "unfoldr",
            "iterateN",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Seq", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["ViewL", "ViewR"] {
            let (k_chirho, v_chirho) =
                mk_type_chirho(name_chirho, &["EmptyL", "EmptyR", ":<", ":>"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &["Data.Sequence", "Data.Sequence.Internal"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.IntMap / Data.IntMap.Strict / Data.IntMap.Lazy / Data.IntMap.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IntMap",
            "empty",
            "singleton",
            "fromList",
            "fromListWith",
            "fromAscList",
            "toList",
            "toAscList",
            "toDescList",
            "insert",
            "insertWith",
            "insertWithKey",
            "delete",
            "adjust",
            "adjustWithKey",
            "update",
            "updateWithKey",
            "alter",
            "lookup",
            "findWithDefault",
            "member",
            "notMember",
            "null",
            "size",
            "union",
            "unionWith",
            "unionWithKey",
            "unions",
            "unionsWith",
            "difference",
            "differenceWith",
            "intersection",
            "intersectionWith",
            "map",
            "mapWithKey",
            "mapAccum",
            "mapAccumWithKey",
            "mapKeys",
            "mapKeysWith",
            "fold",
            "foldWithKey",
            "foldr",
            "foldl",
            "foldr'",
            "foldl'",
            "foldrWithKey",
            "foldlWithKey",
            "foldrWithKey'",
            "foldlWithKey'",
            "elems",
            "keys",
            "assocs",
            "keysSet",
            "filter",
            "filterWithKey",
            "partition",
            "partitionWithKey",
            "mapMaybe",
            "mapMaybeWithKey",
            "mapEither",
            "mapEitherWithKey",
            "split",
            "splitLookup",
            "isSubmapOf",
            "isSubmapOfBy",
            "isProperSubmapOf",
            "isProperSubmapOfBy",
            "mergeWithKey",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IntMap", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &[
            "Data.IntMap",
            "Data.IntMap.Strict",
            "Data.IntMap.Lazy",
            "Data.IntMap.Internal",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.IntSet / Data.IntSet.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IntSet",
            "empty",
            "singleton",
            "fromList",
            "fromAscList",
            "toList",
            "toAscList",
            "toDescList",
            "insert",
            "delete",
            "member",
            "notMember",
            "null",
            "size",
            "union",
            "unions",
            "difference",
            "intersection",
            "isSubsetOf",
            "isProperSubsetOf",
            "filter",
            "partition",
            "split",
            "splitMember",
            "findMin",
            "findMax",
            "deleteMin",
            "deleteMax",
            "elems",
            "fold",
            "foldl",
            "foldr",
            "foldl'",
            "foldr'",
            "map",
            "disjoint",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IntSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &["Data.IntSet", "Data.IntSet.Internal"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Graph
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Graph",
            "Table",
            "Bounds",
            "Edge",
            "Vertex",
            "Forest",
            "Tree",
            "graphFromEdges",
            "graphFromEdges'",
            "buildG",
            "transposeG",
            "vertices",
            "edges",
            "outdegree",
            "indegree",
            "dff",
            "dfs",
            "topSort",
            "components",
            "scc",
            "bcc",
            "reachable",
            "path",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Graph".to_string(),
            exports_chirho,
        });
    }

    // Data.Tree
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Tree",
            "Node",
            "rootLabel",
            "subForest",
            "Forest",
            "drawTree",
            "drawForest",
            "flatten",
            "levels",
            "foldTree",
            "unfoldTree",
            "unfoldForest",
            "unfoldTreeM",
            "unfoldForestM",
            "unfoldTreeM_BF",
            "unfoldForestM_BF",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Tree", &["Node"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Tree".to_string(),
            exports_chirho,
        });
    }

    // Data.Array / Data.Array.IArray / Data.Array.MArray / Data.Array.ST / Data.Array.IO / Data.Array.Unboxed
    for mod_name_chirho in &[
        "Data.Array",
        "Data.Array.IArray",
        "Data.Array.MArray",
        "Data.Array.ST",
        "Data.Array.IO",
        "Data.Array.Unboxed",
        "Data.Array.Base",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Array",
            "array",
            "listArray",
            "accumArray",
            "(!)",
            "bounds",
            "indices",
            "elems",
            "assocs",
            "ixmap",
            "accum",
            "amap",
            "//",
            "range",
            "index",
            "inRange",
            "rangeSize",
            "Ix",
            "MArray",
            "IOArray",
            "STArray",
            "IOUArray",
            "STUArray",
            "newArray",
            "newArray_",
            "newListArray",
            "readArray",
            "writeArray",
            "getElems",
            "getBounds",
            "freeze",
            "thaw",
            "IArray",
            "UArray",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Array", "Ix", "IArray", "MArray", "IOArray", "STArray", "IOUArray", "STUArray",
            "UArray",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // GHC.IO / GHC.IO.Handle / GHC.IO.Handle.Types
    for mod_name_chirho in &[
        "GHC.IO",
        "GHC.IO.Handle",
        "GHC.IO.Handle.Types",
        "GHC.IO.Handle.FD",
        "GHC.IO.Handle.Text",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IO",
            "Handle",
            "stdin",
            "stdout",
            "stderr",
            "HandleType",
            "HandlePosition",
            "hClose",
            "hFlush",
            "hGetContents",
            "hSetBuffering",
            "hGetBuffering",
            "hSetEncoding",
            "hSetNewlineMode",
            "BufferMode",
            "NoBuffering",
            "LineBuffering",
            "BlockBuffering",
            "NewlineMode",
            "universalNewlineMode",
            "nativeNewlineMode",
            "noNewlineTranslation",
            "Newline",
            "LF",
            "CRLF",
            "IOMode",
            "ReadMode",
            "WriteMode",
            "AppendMode",
            "ReadWriteMode",
            "openFile",
            "withFile",
            "hPutStr",
            "hPutStrLn",
            "hGetLine",
            "hGetChar",
            "hPutChar",
            "hPrint",
            "hIsEOF",
            "hIsTerminalDevice",
            "hSeek",
            "SeekMode",
            "AbsoluteSeek",
            "RelativeSeek",
            "SeekFromEnd",
            "hTell",
            "hFileSize",
            "hSetFileSize",
            "hIsOpen",
            "hIsClosed",
            "hIsReadable",
            "hIsWritable",
            "hIsSeekable",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Handle",
            "HandleType",
            "BufferMode",
            "IOMode",
            "SeekMode",
            "Newline",
            "NewlineMode",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST / Control.Monad.ST.Unsafe / Control.Monad.ST.Lazy
    for mod_name_chirho in &[
        "Control.Monad.ST",
        "Control.Monad.ST.Unsafe",
        "Control.Monad.ST.Lazy",
        "Control.Monad.ST.Strict",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ST",
            "runST",
            "fixST",
            "stToIO",
            "unsafeSTToIO",
            "unsafeIOToST",
            "unsafeInterleaveST",
            "RealWorld",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Data.STRef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "STRef",
            "newSTRef",
            "readSTRef",
            "writeSTRef",
            "modifySTRef",
            "modifySTRef'",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("STRef", &["STRef"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.STRef".to_string(),
            exports_chirho,
        });
    }

    // Control.Applicative.Backwards / Control.Applicative.Lift
    for mod_name_chirho in &["Control.Applicative.Backwards", "Control.Applicative.Lift"] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Backwards",
            "forwards",
            "Lift",
            "unLift",
            "runLift",
            "mapLift",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Backwards", "Lift"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Accum / Control.Monad.Trans.Select
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "AccumT",
            "runAccumT",
            "execAccumT",
            "evalAccumT",
            "mapAccumT",
            "Accum",
            "runAccum",
            "execAccum",
            "evalAccum",
            "accum",
            "look",
            "looks",
            "add",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("AccumT", &["AccumT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Accum".to_string(),
            exports_chirho,
        });
    }
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "SelectT",
            "runSelectT",
            "mapSelectT",
            "Select",
            "runSelect",
            "select",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        exports_chirho.types_chirho.insert(
            "SelectT".to_string(),
            IfaceTypeChirho {
                name_chirho: "SelectT".to_string(),
                constructors_chirho: vec!["SelectT".to_string()],
                methods_chirho: vec!["runSelectT".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Select".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Error / Control.Monad.Trans.List (deprecated but still imported)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ErrorT",
            "runErrorT",
            "mapErrorT",
            "Error",
            "noMsg",
            "strMsg",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ErrorT", &["ErrorT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Error", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Error".to_string(),
            exports_chirho,
        });
    }
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["ListT", "runListT", "mapListT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ListT", &["ListT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.List".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Apply / Data.Functor.Bind / Data.Functor.Alt / Data.Functor.Plus (semigroupoids)
    for mod_name_chirho in &[
        "Data.Functor.Apply",
        "Data.Functor.Bind",
        "Data.Functor.Bind.Class",
        "Data.Functor.Alt",
        "Data.Functor.Plus",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Apply",
            "<.>",
            ".>",
            "<.",
            "liftF2",
            "liftF3",
            "Bind",
            ">>-",
            "join",
            "Alt",
            "<!>",
            "some",
            "many",
            "Plus",
            "zero",
            "MaybeApply",
            "WrappedApplicative",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        exports_chirho.types_chirho.insert(
            "Apply".to_string(),
            IfaceTypeChirho {
                name_chirho: "Apply".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![
                    "<.>".to_string(),
                    ".>".to_string(),
                    "<.".to_string(),
                    "liftF2".to_string(),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "Bind".to_string(),
            IfaceTypeChirho {
                name_chirho: "Bind".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![">>-".to_string(), "join".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "Alt".to_string(),
            IfaceTypeChirho {
                name_chirho: "Alt".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["<!>".to_string(), "some".to_string(), "many".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "Plus".to_string(),
            IfaceTypeChirho {
                name_chirho: "Plus".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["zero".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        for name_chirho in &["MaybeApply", "WrappedApplicative"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Reverse (transformers)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Reverse", "getReverse"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Reverse", &["Reverse"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Reverse".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Constant (transformers)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Constant", "getConstant"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Constant", &["Constant"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Constant".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.RWS / Control.Monad.RWS.Strict / Control.Monad.RWS.Lazy / Control.Monad.RWS.Class
    for mod_name_chirho in &[
        "Control.Monad.RWS",
        "Control.Monad.RWS.Strict",
        "Control.Monad.RWS.Lazy",
        "Control.Monad.RWS.Class",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "RWS",
            "RWST",
            "runRWS",
            "runRWST",
            "evalRWS",
            "evalRWST",
            "execRWS",
            "execRWST",
            "mapRWS",
            "mapRWST",
            "withRWS",
            "withRWST",
            "MonadReader",
            "ask",
            "local",
            "reader",
            "asks",
            "MonadWriter",
            "tell",
            "listen",
            "pass",
            "writer",
            "censor",
            "MonadState",
            "get",
            "put",
            "state",
            "modify",
            "modify'",
            "gets",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["RWS", "RWST", "MonadReader", "MonadWriter", "MonadState"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Reader / Control.Monad.Writer / Control.Monad.State / Control.Monad.Except / Control.Monad.Error (mtl wrappers)
    for mod_name_chirho in &[
        "Control.Monad.Reader",
        "Control.Monad.Reader.Class",
        "Control.Monad.Writer",
        "Control.Monad.Writer.Strict",
        "Control.Monad.Writer.Lazy",
        "Control.Monad.Writer.Class",
        "Control.Monad.State",
        "Control.Monad.State.Strict",
        "Control.Monad.State.Lazy",
        "Control.Monad.State.Class",
        "Control.Monad.Except",
        "Control.Monad.Error",
        "Control.Monad.Error.Class",
        "Control.Monad.Cont",
        "Control.Monad.Cont.Class",
        "Control.Monad.Identity",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "MonadReader",
            "ask",
            "local",
            "reader",
            "asks",
            "ReaderT",
            "runReaderT",
            "mapReaderT",
            "withReaderT",
            "Reader",
            "runReader",
            "mapReader",
            "withReader",
            "MonadWriter",
            "tell",
            "listen",
            "pass",
            "writer",
            "censor",
            "WriterT",
            "runWriterT",
            "execWriterT",
            "mapWriterT",
            "Writer",
            "runWriter",
            "execWriter",
            "mapWriter",
            "MonadState",
            "get",
            "put",
            "state",
            "modify",
            "modify'",
            "gets",
            "StateT",
            "runStateT",
            "evalStateT",
            "execStateT",
            "mapStateT",
            "withStateT",
            "State",
            "runState",
            "evalState",
            "execState",
            "mapState",
            "withState",
            "MonadError",
            "throwError",
            "catchError",
            "ExceptT",
            "runExceptT",
            "mapExceptT",
            "withExceptT",
            "Except",
            "runExcept",
            "mapExcept",
            "withExcept",
            "ErrorT",
            "runErrorT",
            "mapErrorT",
            "Error",
            "noMsg",
            "strMsg",
            "MonadCont",
            "callCC",
            "ContT",
            "runContT",
            "mapContT",
            "withContT",
            "Cont",
            "runCont",
            "mapCont",
            "withCont",
            "Identity",
            "runIdentity",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "MonadReader",
            "ReaderT",
            "Reader",
            "MonadWriter",
            "WriterT",
            "Writer",
            "MonadState",
            "StateT",
            "State",
            "MonadError",
            "ExceptT",
            "Except",
            "ErrorT",
            "Error",
            "MonadCont",
            "ContT",
            "Cont",
            "Identity",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Comonad / Control.Comonad.Trans.Traced / etc (comonad package)
    for mod_name_chirho in &[
        "Control.Comonad",
        "Control.Comonad.Env",
        "Control.Comonad.Env.Class",
        "Control.Comonad.Store",
        "Control.Comonad.Store.Class",
        "Control.Comonad.Traced",
        "Control.Comonad.Traced.Class",
        "Control.Comonad.Trans.Env",
        "Control.Comonad.Trans.Traced",
        "Control.Comonad.Trans.Store",
        "Control.Comonad.Trans.Class",
        "Control.Comonad.Hoist.Class",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Comonad",
            "extract",
            "duplicate",
            "extend",
            "=>>",
            "liftW",
            "wfix",
            "cfix",
            "kfix",
            "ComonadApply",
            "<@>",
            "<@@>",
            "liftW2",
            "liftW3",
            "EnvT",
            "runEnvT",
            "env",
            "ask",
            "asks",
            "local",
            "StoreT",
            "runStoreT",
            "store",
            "pos",
            "peek",
            "peeks",
            "seek",
            "seeks",
            "experiment",
            "TracedT",
            "runTracedT",
            "traced",
            "trace",
            "listen",
            "listens",
            "censor",
            "ComonadTrans",
            "lower",
            "ComonadHoist",
            "cohoist",
            "ComonadEnv",
            "ComonadStore",
            "ComonadTraced",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Comonad",
            "ComonadApply",
            "ComonadTrans",
            "ComonadHoist",
            "ComonadEnv",
            "ComonadStore",
            "ComonadTraced",
            "EnvT",
            "StoreT",
            "TracedT",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Data.Tagged (tagged package)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Tagged",
            "unTagged",
            "retag",
            "untag",
            "tagSelf",
            "untagSelf",
            "asTaggedTypeOf",
            "witness",
            "proxy",
            "tagWith",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Tagged", &["Tagged"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Tagged".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Adjunction (adjunctions package)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Adjunction",
            "unit",
            "counit",
            "leftAdjunct",
            "rightAdjunct",
            "tabulateAdjunction",
            "indexAdjunction",
            "zapWithAdjunction",
            "splitL",
            "unsplitL",
            "extractL",
            "duplicateL",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Adjunction", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Adjunction".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.WithIndex / Data.Foldable.WithIndex / Data.Traversable.WithIndex (indexed-traversable)
    for mod_name_chirho in &[
        "Data.Functor.WithIndex",
        "Data.Foldable.WithIndex",
        "Data.Traversable.WithIndex",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "FunctorWithIndex",
            "imap",
            "FoldableWithIndex",
            "ifoldMap",
            "ifoldr",
            "ifoldl",
            "ifoldr'",
            "ifoldl'",
            "itraverse_",
            "ifor_",
            "TraversableWithIndex",
            "itraverse",
            "ifor",
            "imapDefault",
            "ifoldMapDefault",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "FunctorWithIndex",
            "FoldableWithIndex",
            "TraversableWithIndex",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }
    for mod_name_chirho in &[
        "Data.Functor.WithIndex.Instances",
        "Data.Foldable.WithIndex.Instances",
        "Data.Traversable.WithIndex.Instances",
    ] {
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho: IfaceExportsChirho::default(),
        });
    }

    // Data.Bifunctor.Swap (bifunctors)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Swap", "unSwap"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Swap", &["Swap"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bifunctor.Swap".to_string(),
            exports_chirho,
        });
    }

    // GHC.Integer (compatibility)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Integer",
            "mkInteger",
            "smallInteger",
            "wordToInteger",
            "integerToWord",
            "integerToInt",
            "plusInteger",
            "timesInteger",
            "minusInteger",
            "negateInteger",
            "absInteger",
            "signumInteger",
            "divInteger",
            "modInteger",
            "quotInteger",
            "remInteger",
            "quotRemInteger",
            "divModInteger",
            "eqInteger",
            "neqInteger",
            "leInteger",
            "ltInteger",
            "geInteger",
            "gtInteger",
            "compareInteger",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        // Add integer logarithm primops
        for name_chirho in &[
            "integerLog2#",
            "integerLogBase#",
            "wordLog2#",
            "integerLog2",
            "integerLogBase",
            "wordLog2",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &[
            "GHC.Integer",
            "GHC.Integer.Logarithms",
            "GHC.Integer.Logarithms.Compat",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Monad.Instances (deprecated, empty — imports kept for compat)
    {
        let exports_chirho = IfaceExportsChirho::default();
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Instances".to_string(),
            exports_chirho,
        });
    }

    // GHC.SrcLoc
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "SrcLoc",
            "srcLocPackage",
            "srcLocModule",
            "srcLocFile",
            "srcLocStartLine",
            "srcLocStartCol",
            "srcLocEndLine",
            "srcLocEndCol",
            "getCallStack",
            "callStack",
            "withFrozenCallStack",
            "HasCallStack",
            "CallStack",
            "emptyCallStack",
            "freezeCallStack",
            "pushCallStack",
            "fromCallSiteList",
            "prettySrcLoc",
            "prettyCallStack",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["SrcLoc", "CallStack", "HasCallStack"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["SrcLoc"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.SrcLoc".to_string(),
            exports_chirho,
        });
    }

    // Data.Data
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Data",
            "DataType",
            "Constr",
            "ConstrRep",
            "IntConstr",
            "FloatConstr",
            "CharConstr",
            "mkDataType",
            "mkIntType",
            "mkFloatType",
            "mkCharType",
            "mkStringType",
            "mkConstr",
            "mkIntegralConstr",
            "mkRealConstr",
            "mkCharConstr",
            "mkNoRepType",
            "dataTypeName",
            "dataTypeConstrs",
            "dataTypeRep",
            "constrType",
            "constrRep",
            "constrFields",
            "constrFixity",
            "constrIndex",
            "showConstr",
            "readConstr",
            "isAlgType",
            "maxConstrIndex",
            "toConstr",
            "gunfold",
            "gfoldl",
            "gmapT",
            "gmapQ",
            "gmapQl",
            "gmapQr",
            "gmapQi",
            "gmapM",
            "gmapMp",
            "gmapMo",
            "dataTypeOf",
            "dataCast1",
            "dataCast2",
            "cast",
            "Typeable",
            "Fixity",
            "Prefix",
            "Infix",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Data",
            "DataType",
            "Constr",
            "ConstrRep",
            "Fixity",
            "Typeable",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Data".to_string(),
            exports_chirho,
        });
    }

    // Text.ParserCombinators.ReadPrec / Text.ParserCombinators.ReadP
    for mod_name_chirho in &[
        "Text.ParserCombinators.ReadPrec",
        "Text.ParserCombinators.ReadP",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ReadPrec",
            "ReadP",
            "lift",
            "minPrec",
            "step",
            "reset",
            "prec",
            "readPrec_to_P",
            "readP_to_Prec",
            "readPrec_to_S",
            "readS_to_Prec",
            "get",
            "look",
            "pfail",
            "choice",
            "readS_to_P",
            "readP_to_S",
            "satisfy",
            "char",
            "string",
            "munch",
            "munch1",
            "skipSpaces",
            "between",
            "count",
            "option",
            "optional",
            "many",
            "many1",
            "skipMany",
            "skipMany1",
            "sepBy",
            "sepBy1",
            "endBy",
            "endBy1",
            "chainr",
            "chainl",
            "chainr1",
            "chainl1",
            "manyTill",
            "readParen",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ReadPrec", "ReadP"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // GHC.IO.Encoding.Failure
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CodingFailureMode",
            "ErrorOnCodingFailure",
            "IgnoreCodingFailure",
            "TransliterateCodingFailure",
            "RoundtripFailure",
            "isSurrogate",
            "recoverDecode",
            "recoverEncode",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho(
            "CodingFailureMode",
            &[
                "ErrorOnCodingFailure",
                "IgnoreCodingFailure",
                "TransliterateCodingFailure",
                "RoundtripFailure",
            ],
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IO.Encoding.Failure".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Concurrent
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["newForeignPtr", "addForeignPtrFinalizer"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Concurrent".to_string(),
            exports_chirho,
        });
    }

    // System.Posix.Types / System.Posix.Internals
    for mod_name_chirho in &["System.Posix.Types", "System.Posix.Internals"] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Fd",
            "FileMode",
            "FileOffset",
            "ProcessID",
            "ProcessGroupID",
            "UserID",
            "GroupID",
            "ByteCount",
            "ClockTick",
            "EpochTime",
            "DeviceID",
            "FileID",
            "Limit",
            "LinkCount",
            "CDev",
            "CIno",
            "CMode",
            "COff",
            "CPid",
            "CSsize",
            "CGid",
            "CNlink",
            "CUid",
            "CCc",
            "CSpeed",
            "CTcflag",
            "CRLim",
            "CSocklen",
            "CKey",
            "CId",
            "CFsBlkCnt",
            "CFsFilCnt",
            "CClockId",
            "CBlkSize",
            "CBlkCnt",
            "fdToHandle",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Fd",
            "FileMode",
            "FileOffset",
            "ProcessID",
            "UserID",
            "GroupID",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // System.Posix / System.Posix.Directory / System.Posix.Files
    for mod_name_chirho in &[
        "System.Posix",
        "System.Posix.Directory",
        "System.Posix.Files",
        "System.Posix.IO",
        "System.Posix.Process",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "openDirStream",
            "readDirStream",
            "closeDirStream",
            "getWorkingDirectory",
            "changeWorkingDirectory",
            "createDirectory",
            "removeDirectory",
            "getFileStatus",
            "getSymbolicLinkStatus",
            "isDirectory",
            "isRegularFile",
            "isSymbolicLink",
            "fileSize",
            "modificationTime",
            "accessTime",
            "createSymbolicLink",
            "readSymbolicLink",
            "rename",
            "removeLink",
            "setFileMode",
            "openFd",
            "closeFd",
            "fdRead",
            "fdWrite",
            "getProcessID",
            "forkProcess",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["DirStream", "FileStatus"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // System.OsString.Encoding.Internal (os-string — needed by filepath)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ucs2le",
            "mkUcs2le",
            "ucs2le_decode",
            "ucs2le_encode",
            "utf16le_b",
            "mkUTF16le_b",
            "utf16le_b_decode",
            "utf16le_b_encode",
            "encodeWithBasePosix",
            "decodeWithBasePosix",
            "encodeWithBaseWindows",
            "decodeWithBaseWindows",
            "showEncodingException",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("EncodingException", &["EncodingError"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.OsString.Encoding.Internal".to_string(),
            exports_chirho,
        });
    }

    // GHC.Foreign (needed by filepath OsPath modules)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "peekCStringLen",
            "newCStringLen",
            "withCStringLen",
            "withCString",
            "peekCString",
            "newCString",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Foreign".to_string(),
            exports_chirho,
        });
    }

    // System.OsString.Data.ByteString.Short (os-string — needed by filepath)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ShortByteString",
            "toShort",
            "fromShort",
            "pack",
            "unpack",
            "empty",
            "null",
            "length",
            "index",
            "append",
            "useAsCStringLen",
            "packCStringLen",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ShortByteString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.OsString.Data.ByteString.Short".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Resource (resourcet)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ResourceT",
            "runResourceT",
            "liftResourceT",
            "transResourceT",
            "MonadResource",
            "liftResourceT",
            "allocate",
            "register",
            "release",
            "resourceForkIO",
            "ReleaseKey",
            "MonadUnliftIO",
            "withRunInIO",
            "UnliftIO",
            "askUnliftIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "ResourceT",
            "MonadResource",
            "ReleaseKey",
            "MonadUnliftIO",
            "UnliftIO",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &[
            "Control.Monad.Trans.Resource",
            "Control.Monad.Trans.Resource.Internal",
            "UnliftIO",
            "Control.Monad.IO.Unlift",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // System.Timeout
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["timeout"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Timeout".to_string(),
            exports_chirho,
        });
    }

    // GHC internal modules needed by base-orphans
    for mod_name_chirho in &[
        "GHC.GHCi",
        "GHC.IO.Buffer",
        "GHC.IO.BufferedIO",
        "GHC.IO.Device",
        "GHC.Stats",
        "GHC.RTS.Flags",
        "GHC.Event",
        "GHC.Conc.Signal",
        "System.Console.GetOpt",
        "Text.Read.Lex",
    ] {
        let exports_chirho = IfaceExportsChirho::default();
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // GHC.Desugar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["AnnotationWrapper", "toAnnotationWrapper"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("AnnotationWrapper", &["AnnotationWrapper"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Desugar".to_string(),
            exports_chirho,
        });
    }

    // GHC.Weak
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Weak",
            "mkWeak",
            "deRefWeak",
            "finalize",
            "mkWeakPtr",
            "mkWeakPair",
            "addFinalizer",
            "mkWeakIORef",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Weak", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Weak".to_string(),
            exports_chirho,
        });
    }

    // Foreign.ForeignPtr.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["unsafeForeignPtrToPtr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.ForeignPtr.Unsafe".to_string(),
            exports_chirho,
        });
    }

    // GHC.IO.Encoding / GHC.IO.Encoding.UTF8
    for mod_name_chirho in &[
        "GHC.IO.Encoding",
        "GHC.IO.Encoding.UTF8",
        "GHC.IO.Encoding.Latin1",
        "GHC.IO.Encoding.CodePage",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "TextEncoding",
            "utf8",
            "utf8_bom",
            "utf16le",
            "utf16be",
            "utf32le",
            "utf32be",
            "latin1",
            "char8",
            "mkTextEncoding",
            "localeEncoding",
            "getLocaleEncoding",
            "setLocaleEncoding",
            "getFileSystemEncoding",
            "setFileSystemEncoding",
            "getForeignEncoding",
            "setForeignEncoding",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("TextEncoding", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // System.FilePath.Posix / System.FilePath.Windows / System.FilePath
    for mod_name_chirho in &[
        "System.FilePath",
        "System.FilePath.Posix",
        "System.FilePath.Windows",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "FilePath",
            "pathSeparator",
            "pathSeparators",
            "isPathSeparator",
            "searchPathSeparator",
            "isSearchPathSeparator",
            "extSeparator",
            "isExtSeparator",
            "splitExtension",
            "takeExtension",
            "replaceExtension",
            "dropExtension",
            "addExtension",
            "hasExtension",
            "splitExtensions",
            "takeExtensions",
            "dropExtensions",
            "splitFileName",
            "takeFileName",
            "replaceFileName",
            "dropFileName",
            "takeBaseName",
            "replaceBaseName",
            "takeDirectory",
            "replaceDirectory",
            "combine",
            "splitPath",
            "joinPath",
            "splitDirectories",
            "splitDrive",
            "joinDrive",
            "takeDrive",
            "hasDrive",
            "dropDrive",
            "isDrive",
            "hasTrailingPathSeparator",
            "addTrailingPathSeparator",
            "dropTrailingPathSeparator",
            "normalise",
            "equalFilePath",
            "makeRelative",
            "isRelative",
            "isAbsolute",
            "isValid",
            "makeValid",
            "</>",
            "<.>",
            "-<.>",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // System.Environment
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "getArgs",
            "getProgName",
            "getExecutablePath",
            "getEnvironment",
            "lookupEnv",
            "setEnv",
            "unsetEnv",
            "withArgs",
            "withProgName",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Environment".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent / Control.Concurrent.MVar
    for mod_name_chirho in &[
        "Control.Concurrent",
        "Control.Concurrent.MVar",
        "Control.Concurrent.Chan",
        "Control.Concurrent.QSem",
        "Control.Concurrent.QSemN",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ThreadId",
            "myThreadId",
            "forkIO",
            "forkFinally",
            "forkOS",
            "killThread",
            "throwTo",
            "threadDelay",
            "threadWaitRead",
            "threadWaitWrite",
            "yield",
            "MVar",
            "newMVar",
            "newEmptyMVar",
            "readMVar",
            "takeMVar",
            "putMVar",
            "tryTakeMVar",
            "tryPutMVar",
            "modifyMVar",
            "modifyMVar_",
            "swapMVar",
            "withMVar",
            "Chan",
            "newChan",
            "writeChan",
            "readChan",
            "dupChan",
            "getChanContents",
            "writeList2Chan",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ThreadId", "MVar", "Chan"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // ── Haskelujah stdlib (batteries included) ──────────────────────
    // These match the modules in stdlib-chirho/Haskelujah/*.hs
    for (mod_name_chirho, exports_list_chirho) in &[
        (
            "Haskelujah.JSON",
            &[
                "Value", "Object", "Array", "String", "Number", "Bool", "Null", "encode", "object",
                "array", ".=",
            ][..],
        ),
        (
            "Haskelujah.Test",
            &[
                "Test",
                "test",
                "runTests",
                "assertEqual",
                "assertBool",
                "assertFailure",
            ],
        ),
        (
            "Haskelujah.Args",
            &["getArgs", "getFlag", "getOption", "getPositional"],
        ),
        (
            "Haskelujah.HTTP",
            &["Response", "get", "post", "statusCode", "body", "headers"],
        ),
        (
            "Haskelujah.Prelude",
            &["trim", "words'", "unwords'", "groupBy'", "chunksOf", "nub'"],
        ),
        (
            "Haskelujah.File",
            &[
                "readFileText",
                "writeFileText",
                "appendFileText",
                "fileExists",
                "listDirectory",
            ],
        ),
        (
            "Haskelujah.Text",
            &[
                "toLower",
                "toUpper",
                "capitalize",
                "contains",
                "startsWith",
                "endsWith",
                "splitOn",
                "lines'",
                "unlines'",
                "padLeft",
                "padRight",
                "center",
            ],
        ),
        (
            "Haskelujah.Map",
            &[
                "Map",
                "empty",
                "singleton",
                "fromList",
                "toList",
                "insert",
                "lookup",
                "delete",
                "member",
                "size",
                "keys",
                "elems",
                "mapValues",
                "filterMap",
                "unionWith",
            ],
        ),
        (
            "Haskelujah.Set",
            &[
                "Set",
                "empty",
                "singleton",
                "fromList",
                "toList",
                "insert",
                "member",
                "delete",
                "size",
                "union",
                "intersection",
                "difference",
            ],
        ),
        (
            "Haskelujah.Pretty",
            &[
                "Doc", "text", "int", "nest", "line", "<+>", "$$", "hsep", "vsep", "indent",
                "render", "parens", "brackets", "braces",
            ],
        ),
        (
            "Haskelujah.Random",
            &[
                "Gen",
                "mkGen",
                "nextInt",
                "nextDouble",
                "randomList",
                "shuffle",
                "choice",
            ],
        ),
        ("Haskelujah.Process", &["shell", "exec", "ExitCode"]),
        ("Haskelujah.Time", &["now", "sleep", "measure"]),
        (
            "Haskelujah.Concurrent",
            &[
                "fork",
                "delay",
                "Chan",
                "newChan",
                "writeChan",
                "readChan",
                "MVar",
                "newMVar",
                "readMVar",
                "takeMVar",
                "putMVar",
            ],
        ),
        (
            "Haskelujah.Debug",
            &[
                "trace",
                "traceShow",
                "traceIO",
                "assert",
                "todo",
                "unreachable",
            ],
        ),
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in *exports_list_chirho {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
            if name_chirho.chars().next().is_some_and(|c| c.is_uppercase()) {
                let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
                exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            }
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Instances (transformers-compat — re-exports)
    {
        let exports_chirho = IfaceExportsChirho::default();
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Instances".to_string(),
            exports_chirho,
        });
    }

    // System.Random (random package — needed by QuickCheck)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("RandomGen", &["split", "genRange", "next"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("SplitGen", &["splitGen"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("StdGen", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "split",
            "splitGen",
            "next",
            "genRange",
            "mkStdGen",
            "newStdGen",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Random".to_string(),
            exports_chirho,
        });
    }

    // System.Random.SplitMix (splitmix package — needed by random/QuickCheck)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newSMGen",
            "initSMGen",
            "mkSMGen",
            "seedSMGen",
            "seedSMGen'",
            "unseedSMGen",
            "splitSMGen",
            "nextWord64",
            "nextWord32",
            "nextTwoWord32",
            "nextInt",
            "nextDouble",
            "nextFloat",
            "bitmaskWithRejection32",
            "bitmaskWithRejection64",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("SMGen", &["SMGen"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_chirho in &[
            "System.Random.SplitMix",
            "System.Random.SplitMix.Init",
            "System.Random.SplitMix32",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Profunctor
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["dimap", "lmap", "rmap", "arr'"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Profunctor", &["dimap", "lmap", "rmap"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for sub_mod_chirho in &[
            "Data.Profunctor",
            "Data.Profunctor.Unsafe",
            "Data.Profunctor.Types",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: sub_mod_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Bifunctor.Assoc / Data.Bifunctor.Swap (assoc package)
    for mod_name_chirho in &["Data.Bifunctor.Assoc", "Data.Bifunctor.Swap"] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["assoc", "unassoc", "swap"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Assoc", "Swap"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["assoc", "unassoc", "swap"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Data.Bifunctor.Functor (bifunctor-classes-compat)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["QualifiedBifunctor", "WrappedBifunctor"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bifunctor.Functor".to_string(),
            exports_chirho,
        });
    }

    normalize_builtin_class_exports_chirho(&mut modules_chirho);
    merge_module_ifaces_chirho(modules_chirho)
}

/// Merge module interfaces that share the same module name, preserving the
/// union of exported values and type members across all copies.
pub fn merge_module_ifaces_chirho(
    modules_chirho: Vec<ModuleIfaceChirho>,
) -> Vec<ModuleIfaceChirho> {
    let mut deduped_chirho: Vec<ModuleIfaceChirho> = Vec::with_capacity(modules_chirho.len());
    let mut index_chirho: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for module_chirho in modules_chirho {
        if let Some(&idx_chirho) = index_chirho.get(&module_chirho.name_chirho) {
            let existing_chirho = &mut deduped_chirho[idx_chirho];
            for (name_chirho, value_chirho) in module_chirho.exports_chirho.values_chirho {
                existing_chirho
                    .exports_chirho
                    .values_chirho
                    .entry(name_chirho)
                    .or_insert(value_chirho);
            }
            for (name_chirho, type_chirho) in module_chirho.exports_chirho.types_chirho {
                match existing_chirho
                    .exports_chirho
                    .types_chirho
                    .get_mut(&name_chirho)
                {
                    Some(existing_type_chirho) => {
                        for constructor_chirho in type_chirho.constructors_chirho {
                            if !existing_type_chirho
                                .constructors_chirho
                                .contains(&constructor_chirho)
                            {
                                existing_type_chirho
                                    .constructors_chirho
                                    .push(constructor_chirho);
                            }
                        }
                        for method_chirho in type_chirho.methods_chirho {
                            if !existing_type_chirho.methods_chirho.contains(&method_chirho) {
                                existing_type_chirho.methods_chirho.push(method_chirho);
                            }
                        }
                    }
                    None => {
                        existing_chirho
                            .exports_chirho
                            .types_chirho
                            .insert(name_chirho, type_chirho);
                    }
                }
            }
        } else {
            index_chirho.insert(module_chirho.name_chirho.clone(), deduped_chirho.len());
            deduped_chirho.push(module_chirho);
        }
    }

    deduped_chirho
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
                let field_names_chirho: Vec<String> = constructors_chirho
                    .iter()
                    .flat_map(con_decl_field_names_chirho)
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
                for field_name_chirho in &field_names_chirho {
                    exports_chirho.values_chirho.insert(
                        field_name_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho: field_name_chirho.clone(),
                            span_chirho: *span_chirho,
                        },
                    );
                }

                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: type_name_chirho,
                        constructors_chirho: con_names_chirho,
                        methods_chirho: field_names_chirho,
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
                let field_names_chirho = con_decl_field_names_chirho(constructor_chirho);

                exports_chirho.values_chirho.insert(
                    con_name_chirho.clone(),
                    IfaceValueChirho {
                        name_chirho: con_name_chirho.clone(),
                        span_chirho: *span_chirho,
                    },
                );
                for field_name_chirho in &field_names_chirho {
                    exports_chirho.values_chirho.insert(
                        field_name_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho: field_name_chirho.clone(),
                            span_chirho: *span_chirho,
                        },
                    );
                }

                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: type_name_chirho,
                        constructors_chirho: vec![con_name_chirho],
                        methods_chirho: field_names_chirho,
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
                    .map(|m_chirho| canonical_value_name_chirho(m_chirho.name_chirho.text_chirho()))
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
                let fn_name_chirho = canonical_value_name_chirho(name_chirho.text_chirho());
                exports_chirho.values_chirho.insert(
                    fn_name_chirho.clone(),
                    IfaceValueChirho {
                        name_chirho: fn_name_chirho,
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::PatBindChirho {
                pat_chirho,
                span_chirho,
                ..
            } => {
                for bound_name_chirho in pat_bound_names_for_iface_chirho(pat_chirho) {
                    let canonical_name_chirho = canonical_value_name_chirho(&bound_name_chirho);
                    exports_chirho.values_chirho.insert(
                        canonical_name_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho: canonical_name_chirho,
                            span_chirho: *span_chirho,
                        },
                    );
                }
            }
            DeclChirho::ForeignDeclChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                let foreign_value_name_chirho =
                    canonical_value_name_chirho(name_chirho.text_chirho());
                exports_chirho.values_chirho.insert(
                    foreign_value_name_chirho.clone(),
                    IfaceValueChirho {
                        name_chirho: foreign_value_name_chirho,
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::TypeSigChirho { .. }
            | DeclChirho::InstanceDeclChirho { .. }
            | DeclChirho::FixityDeclChirho { .. }
            | DeclChirho::DefaultDeclChirho { .. } => {}
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                let family_name_chirho = name_chirho.text_chirho().to_string();
                exports_chirho.types_chirho.insert(
                    family_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: family_name_chirho,
                        constructors_chirho: vec![],
                        methods_chirho: vec![],
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::TypeFamilyInstanceDeclChirho { .. } => {
                // Type family instances don't introduce new names
            }
            DeclChirho::SpliceDeclChirho { .. } => {
                // TH splice declarations don't directly export names;
                // they must be evaluated to generate concrete declarations first.
            }
            DeclChirho::StandaloneDerivingDeclChirho { .. } => {
                // Standalone deriving is handled by the deriving pass, not exports.
            }
            DeclChirho::PatSynDeclChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                let syn_name_chirho = name_chirho.text_chirho().to_string();
                exports_chirho.values_chirho.insert(
                    syn_name_chirho.clone(),
                    IfaceValueChirho {
                        name_chirho: syn_name_chirho,
                        span_chirho: *span_chirho,
                    },
                );
            }
        }
    }

    exports_chirho
}

fn pat_bound_names_for_iface_chirho(
    pat_chirho: &haskelujah_ast_chirho::pat_chirho::PatChirho,
) -> Vec<String> {
    use haskelujah_ast_chirho::pat_chirho::PatChirho;

    let mut names_chirho = Vec::new();
    match pat_chirho {
        PatChirho::VarChirho(name_chirho) => {
            names_chirho.push(name_chirho.text_chirho().to_string());
        }
        PatChirho::LitChirho(_) | PatChirho::WildcardChirho(_) | PatChirho::NegChirho { .. } => {}
        PatChirho::TupleChirho {
            elements_chirho, ..
        }
        | PatChirho::ListChirho {
            elements_chirho, ..
        } => {
            for inner_pat_chirho in elements_chirho {
                names_chirho.extend(pat_bound_names_for_iface_chirho(inner_pat_chirho));
            }
        }
        PatChirho::ConChirho { args_chirho, .. } => {
            for inner_pat_chirho in args_chirho {
                names_chirho.extend(pat_bound_names_for_iface_chirho(inner_pat_chirho));
            }
        }
        PatChirho::RecordChirho { fields_chirho, .. } => {
            for field_chirho in fields_chirho {
                names_chirho.extend(pat_bound_names_for_iface_chirho(
                    &field_chirho.pattern_chirho,
                ));
            }
        }
        PatChirho::InfixConChirho {
            left_chirho,
            right_chirho,
            ..
        } => {
            names_chirho.extend(pat_bound_names_for_iface_chirho(left_chirho));
            names_chirho.extend(pat_bound_names_for_iface_chirho(right_chirho));
        }
        PatChirho::AsChirho {
            name_chirho,
            pattern_chirho,
            ..
        } => {
            names_chirho.push(name_chirho.text_chirho().to_string());
            names_chirho.extend(pat_bound_names_for_iface_chirho(pattern_chirho));
        }
        PatChirho::BangChirho { inner_chirho, .. }
        | PatChirho::LazyChirho { inner_chirho, .. }
        | PatChirho::ParenChirho { inner_chirho, .. } => {
            names_chirho.extend(pat_bound_names_for_iface_chirho(inner_chirho));
        }
        PatChirho::ViewChirho {
            pat_chirho: inner_pat_chirho,
            ..
        }
        | PatChirho::TypeAnnotChirho {
            pat_chirho: inner_pat_chirho,
            ..
        } => {
            names_chirho.extend(pat_bound_names_for_iface_chirho(inner_pat_chirho));
        }
    }
    names_chirho
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
                let value_text_chirho = canonical_value_name_chirho(name_chirho.text_chirho());
                if let Some(val_chirho) = all_chirho.values_chirho.get(&value_text_chirho) {
                    result_chirho
                        .values_chirho
                        .insert(value_text_chirho.clone(), val_chirho.clone());
                } else if let Some(imported_value_chirho) = imported_value_in_scope_chirho(
                    module_chirho,
                    imported_ifaces_chirho,
                    &value_text_chirho,
                ) {
                    result_chirho
                        .values_chirho
                        .insert(value_text_chirho.clone(), imported_value_chirho);
                }
                let text_chirho = name_chirho.text_chirho();
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
                } else if let Some((imported_type_chirho, _accompanying_values_chirho)) =
                    imported_type_in_scope_chirho(
                        module_chirho,
                        imported_ifaces_chirho,
                        text_chirho,
                        &ExportMembersChirho::NoneChirho,
                    )
                {
                    result_chirho
                        .types_chirho
                        .insert(text_chirho.to_string(), imported_type_chirho);
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
                                .map(|n_chirho| canonical_value_name_chirho(n_chirho.text_chirho()))
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
                } else if let Some((imported_type_chirho, accompanying_values_chirho)) =
                    imported_type_in_scope_chirho(
                        module_chirho,
                        imported_ifaces_chirho,
                        text_chirho,
                        members_chirho,
                    )
                {
                    for value_chirho in accompanying_values_chirho {
                        result_chirho
                            .values_chirho
                            .insert(value_chirho.name_chirho.clone(), value_chirho);
                    }
                    result_chirho
                        .types_chirho
                        .insert(text_chirho.to_string(), imported_type_chirho);
                }
            }
            ExportSpecChirho::ModuleChirho(re_export_name_chirho) => {
                let target_mod_chirho = re_export_name_chirho.full_name_chirho();
                let matching_imports_chirho: Vec<&ImportDeclChirho> = module_chirho
                    .imports_chirho
                    .iter()
                    .filter(|imp_chirho| {
                        imp_chirho.module_chirho.full_name_chirho() == target_mod_chirho
                            || imp_chirho
                                .alias_chirho
                                .as_ref()
                                .is_some_and(|alias_chirho| {
                                    alias_chirho.full_name_chirho() == target_mod_chirho
                                })
                    })
                    .collect();
                // `module M` in the export list of module M itself means
                // "export all local definitions" — this is the self-re-export pattern.
                let is_self_chirho =
                    module_chirho.name_chirho.full_name_chirho() == target_mod_chirho;

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
                } else if !matching_imports_chirho.is_empty() {
                    for import_chirho in matching_imports_chirho {
                        if let Some(iface_chirho) =
                            imported_ifaces_chirho.iter().find(|iface_chirho| {
                                iface_chirho.name_chirho
                                    == import_chirho.module_chirho.full_name_chirho()
                            })
                        {
                            merge_imported_exports_chirho(
                                &mut result_chirho,
                                &iface_chirho.exports_chirho,
                                &import_chirho.spec_chirho,
                            );
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
        | ConDeclChirho::RecordChirho { name_chirho, .. }
        | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho.text_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::decl_chirho::ForeignDirectionChirho;
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;

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
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
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

        let iface_chirho = build_iface_chirho(&module_chirho);
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

        let iface_chirho = build_iface_chirho(&module_chirho);
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
                }],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
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
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: mk_name_chirho("Blue"),
                        fields_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
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
    fn class_methods_exported_chirho() {
        let module_chirho = mk_module_chirho(
            "Lib",
            None,
            vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: mk_name_chirho("Show"),
                type_vars_chirho: vec![],
                methods_chirho: vec![haskelujah_ast_chirho::decl_chirho::ClassMethodChirho {
                    name_chirho: mk_name_chirho("show"),
                    ty_chirho: haskelujah_ast_chirho::ty_chirho::TypeChirho::VarChirho(
                        mk_name_chirho("a"),
                    ),
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
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
                            names_chirho: vec![mk_name_chirho("opLetterChirho")],
                            ty_chirho: haskelujah_ast_chirho::ty_chirho::TypeChirho::ConChirho(
                                mk_name_chirho("String"),
                            ),
                            strictness_chirho:
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
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
                }],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
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
                constructor_chirho: ConDeclChirho::OrdinaryChirho {
                    name_chirho: mk_name_chirho("Wrap"),
                    fields_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
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
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &[inner_iface_chirho]);

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
                e_chirho
            },
        };

        let module_chirho = ModuleChirho {
            name_chirho: mk_name_chirho("Reexporter"),
            exports_chirho: Some(vec![ExportSpecChirho::ModuleChirho(mk_name_chirho(
                "Alias",
            ))]),
            imports_chirho: vec![ImportDeclChirho {
                module_chirho: mk_name_chirho("Inner"),
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
        };

        let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &[inner_iface_chirho]);
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
            name_chirho: "LeftMod".to_string(),
            exports_chirho: {
                let mut e_chirho = IfaceExportsChirho::default();
                e_chirho.values_chirho.insert(
                    "leftFn".to_string(),
                    IfaceValueChirho {
                        name_chirho: "leftFn".to_string(),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                );
                e_chirho.values_chirho.insert(
                    "hiddenLeft".to_string(),
                    IfaceValueChirho {
                        name_chirho: "hiddenLeft".to_string(),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                );
                e_chirho
            },
        };
        let right_iface_chirho = ModuleIfaceChirho {
            name_chirho: "RightMod".to_string(),
            exports_chirho: {
                let mut e_chirho = IfaceExportsChirho::default();
                e_chirho.values_chirho.insert(
                    "rightFn".to_string(),
                    IfaceValueChirho {
                        name_chirho: "rightFn".to_string(),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
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
                    module_chirho: mk_name_chirho("LeftMod"),
                    qualified_chirho: false,
                    alias_chirho: Some(mk_name_chirho("Alias")),
                    spec_chirho: Some(ImportSpecChirho {
                        hiding_chirho: true,
                        items_chirho: vec![ImportItemChirho::VarChirho(mk_name_chirho(
                            "hiddenLeft",
                        ))],
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                ImportDeclChirho {
                    module_chirho: mk_name_chirho("RightMod"),
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
        };

        let iface_chirho = build_iface_with_imports_chirho(
            &module_chirho,
            &[left_iface_chirho, right_iface_chirho],
        );
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
            name_chirho: "InnerChoice".to_string(),
            exports_chirho: {
                let mut exports_chirho = IfaceExportsChirho::default();
                exports_chirho.values_chirho.insert(
                    "choice".to_string(),
                    IfaceValueChirho {
                        name_chirho: "choice".to_string(),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                );
                exports_chirho
            },
        };

        let module_chirho = ModuleChirho {
            name_chirho: mk_name_chirho("OuterChoice"),
            exports_chirho: Some(vec![ExportSpecChirho::VarChirho(mk_name_chirho("choice"))]),
            imports_chirho: vec![ImportDeclChirho {
                module_chirho: mk_name_chirho("InnerChoice"),
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
        };

        let iface_chirho =
            build_iface_with_imports_chirho(&module_chirho, &[imported_iface_chirho]);
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
            name_chirho: "InnerParsec".to_string(),
            exports_chirho: {
                let mut exports_chirho = IfaceExportsChirho::default();
                exports_chirho.types_chirho.insert(
                    "Parsec".to_string(),
                    IfaceTypeChirho {
                        name_chirho: "Parsec".to_string(),
                        constructors_chirho: vec![],
                        methods_chirho: vec![],
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
                module_chirho: mk_name_chirho("InnerParsec"),
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
        };

        let iface_chirho =
            build_iface_with_imports_chirho(&module_chirho, &[imported_iface_chirho]);
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

    #[test]
    fn builtin_dedup_data_map_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        let data_map_chirho: Vec<_> = ifaces_chirho
            .iter()
            .filter(|i_chirho| i_chirho.name_chirho == "Data.Map")
            .collect();
        assert_eq!(
            data_map_chirho.len(),
            1,
            "Data.Map should be deduplicated to a single entry"
        );
        let dm_chirho = &data_map_chirho[0];
        // Should have standard Haskell API names from the second definition
        assert!(
            dm_chirho
                .exports_chirho
                .values_chirho
                .contains_key("singleton"),
            "Data.Map should export 'singleton' after dedup merge"
        );
        assert!(
            dm_chirho.exports_chirho.values_chirho.contains_key("empty"),
            "Data.Map should export 'empty' after dedup merge"
        );
        // Should also have internal names from the first definition
        assert!(
            dm_chirho
                .exports_chirho
                .values_chirho
                .contains_key("mapInsert"),
            "Data.Map should still export 'mapInsert' from first definition"
        );
    }

    #[test]
    fn builtin_dedup_foreign_storable_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        let fs_chirho: Vec<_> = ifaces_chirho
            .iter()
            .filter(|i_chirho| i_chirho.name_chirho == "Foreign.Storable")
            .collect();
        assert_eq!(
            fs_chirho.len(),
            1,
            "Foreign.Storable should be deduplicated to a single entry"
        );
        let s_chirho = &fs_chirho[0];
        assert!(
            s_chirho.exports_chirho.values_chirho.contains_key("sizeOf"),
            "Foreign.Storable should export 'sizeOf'"
        );
        assert!(
            s_chirho.exports_chirho.values_chirho.contains_key("peek"),
            "Foreign.Storable should export 'peek'"
        );
    }

    #[test]
    fn builtin_control_monad_class_methods_exported_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        let control_monad_chirho = ifaces_chirho
            .iter()
            .find(|iface_chirho| iface_chirho.name_chirho == "Control.Monad")
            .expect("Control.Monad builtin iface should exist");
        let monad_chirho = control_monad_chirho
            .exports_chirho
            .types_chirho
            .get("Monad")
            .expect("Control.Monad should export Monad");
        assert!(monad_chirho.methods_chirho.contains(&"return".to_string()));
        assert!(monad_chirho.methods_chirho.contains(&">>=".to_string()));
        assert!(monad_chirho.methods_chirho.contains(&">>".to_string()));
        assert!(
            control_monad_chirho
                .exports_chirho
                .values_chirho
                .contains_key("return")
        );
        assert!(
            control_monad_chirho
                .exports_chirho
                .values_chirho
                .contains_key(">>=")
        );
    }

    #[test]
    fn builtin_data_foldable_class_methods_exported_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        let foldable_mod_chirho = ifaces_chirho
            .iter()
            .find(|iface_chirho| iface_chirho.name_chirho == "Data.Foldable")
            .expect("Data.Foldable builtin iface should exist");
        let foldable_chirho = foldable_mod_chirho
            .exports_chirho
            .types_chirho
            .get("Foldable")
            .expect("Data.Foldable should export Foldable");
        for method_chirho in [
            "foldr", "foldl", "foldMap", "length", "null", "elem", "sum", "product", "maximum",
            "minimum", "toList",
        ] {
            assert!(
                foldable_chirho
                    .methods_chirho
                    .contains(&method_chirho.to_string()),
                "Foldable should expose {method_chirho} via (..)"
            );
            assert!(
                foldable_mod_chirho
                    .exports_chirho
                    .values_chirho
                    .contains_key(method_chirho),
                "Data.Foldable should export value {method_chirho}"
            );
        }
    }

    #[test]
    fn builtin_prelude_exports_alternative_helpers_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        let prelude_chirho = ifaces_chirho
            .iter()
            .find(|iface_chirho| iface_chirho.name_chirho == "Prelude")
            .expect("Prelude builtin iface should exist");
        assert!(
            prelude_chirho
                .exports_chirho
                .types_chirho
                .contains_key("Alternative"),
            "Prelude should export Alternative"
        );
        assert!(
            prelude_chirho
                .exports_chirho
                .types_chirho
                .contains_key("Const"),
            "Prelude should export Const"
        );
        for name_chirho in [
            "empty", "<|>", "some", "many", "optional", "liftA2", "bimap", "Const", "getConst",
            "!!",
        ] {
            assert!(
                prelude_chirho
                    .exports_chirho
                    .values_chirho
                    .contains_key(name_chirho),
                "Prelude should export {name_chirho}"
            );
        }
    }

    #[test]
    fn builtin_bytestring_internal_and_system_io_unsafe_exports_effect_helpers_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        let bytestring_internal_chirho = ifaces_chirho
            .iter()
            .find(|iface_chirho| iface_chirho.name_chirho == "Data.ByteString.Internal")
            .expect("Data.ByteString.Internal builtin iface should exist");
        assert!(
            bytestring_internal_chirho
                .exports_chirho
                .values_chirho
                .contains_key("accursedUnutterablePerformIO"),
            "Data.ByteString.Internal should export accursedUnutterablePerformIO"
        );

        let system_io_unsafe_chirho = ifaces_chirho
            .iter()
            .find(|iface_chirho| iface_chirho.name_chirho == "System.IO.Unsafe")
            .expect("System.IO.Unsafe builtin iface should exist");
        for name_chirho in ["unsafePerformIO", "inlinePerformIO"] {
            assert!(
                system_io_unsafe_chirho
                    .exports_chirho
                    .values_chirho
                    .contains_key(name_chirho),
                "System.IO.Unsafe should export {name_chirho}"
            );
        }
    }

    #[test]
    fn builtin_foreign_ptr_exports_unsafe_with_foreign_ptr_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        let ghc_foreign_ptr_chirho = ifaces_chirho
            .iter()
            .find(|iface_chirho| iface_chirho.name_chirho == "GHC.ForeignPtr")
            .expect("GHC.ForeignPtr builtin iface should exist");
        assert!(
            ghc_foreign_ptr_chirho
                .exports_chirho
                .values_chirho
                .contains_key("unsafeWithForeignPtr"),
            "GHC.ForeignPtr should export unsafeWithForeignPtr"
        );
    }

    #[test]
    fn builtin_ghc_exts_exports_aligned_byte_array_primop_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        let ghc_exts_chirho = ifaces_chirho
            .iter()
            .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Exts")
            .expect("GHC.Exts builtin iface should exist");
        assert!(
            ghc_exts_chirho
                .exports_chirho
                .values_chirho
                .contains_key("newAlignedPinnedByteArray#"),
            "GHC.Exts should export newAlignedPinnedByteArray#"
        );
    }

    #[test]
    fn builtin_ghc_base_exports_mkweak_primop_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        let ghc_base_chirho = ifaces_chirho
            .iter()
            .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Base")
            .expect("GHC.Base builtin iface should exist");
        assert!(
            ghc_base_chirho
                .exports_chirho
                .values_chirho
                .contains_key("mkWeak#"),
            "GHC.Base should export mkWeak#"
        );
    }

    #[test]
    fn builtin_with_index_instances_modules_exist_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        for module_name_chirho in &[
            "Data.Functor.WithIndex.Instances",
            "Data.Foldable.WithIndex.Instances",
            "Data.Traversable.WithIndex.Instances",
        ] {
            assert!(
                ifaces_chirho
                    .iter()
                    .any(|iface_chirho| iface_chirho.name_chirho == *module_name_chirho),
                "{module_name_chirho} builtin iface should exist"
            );
        }
    }

    #[test]
    fn builtin_semigroup_traversable_exports_traverse1_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        let semigroup_traversable_chirho = ifaces_chirho
            .iter()
            .find(|iface_chirho| iface_chirho.name_chirho == "Data.Semigroup.Traversable")
            .expect("Data.Semigroup.Traversable builtin iface should exist");
        assert!(
            semigroup_traversable_chirho
                .exports_chirho
                .values_chirho
                .contains_key("traverse1"),
            "Data.Semigroup.Traversable should export traverse1"
        );
        assert!(
            semigroup_traversable_chirho
                .exports_chirho
                .types_chirho
                .contains_key("Traversable1"),
            "Data.Semigroup.Traversable should export Traversable1"
        );
    }

    #[test]
    fn builtin_splitmix32_iface_exists_chirho() {
        let ifaces_chirho = builtin_module_ifaces_chirho();
        let splitmix32_iface_chirho = ifaces_chirho
            .iter()
            .find(|iface_chirho| iface_chirho.name_chirho == "System.Random.SplitMix32")
            .expect("System.Random.SplitMix32 builtin iface should exist");
        assert!(
            splitmix32_iface_chirho
                .exports_chirho
                .values_chirho
                .contains_key("nextWord32"),
            "System.Random.SplitMix32 should export nextWord32"
        );
        assert!(
            splitmix32_iface_chirho
                .exports_chirho
                .types_chirho
                .contains_key("SMGen"),
            "System.Random.SplitMix32 should export SMGen"
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
        };

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert!(
            iface_chirho
                .exports_chirho
                .values_chirho
                .contains_key("sinChirho"),
            "foreign import names should be exported through module interfaces"
        );
    }
}
