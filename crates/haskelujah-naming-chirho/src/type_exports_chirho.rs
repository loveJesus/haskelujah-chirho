// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Canonical type-export names and the small wired-in type-member inventory.
//!
//! Module interfaces are the authority used by source name resolution. Keeping
//! operator canonicalization and wired-in data-constructor membership here
//! prevents the resolver from guessing that an unknown source spelling is
//! available merely because a later compiler phase happens to recognize it.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::module_chirho::{ExportMembersChirho, ModuleChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::{
    IfaceExportsChirho, IfaceTypeChirho, IfaceValueChirho, ModuleIfaceChirho,
};

const RUNTIME_REP_CONSTRUCTORS_CHIRHO: &[&str] = &[
    "BoxedRep",
    "IntRep",
    "Int8Rep",
    "Int16Rep",
    "Int32Rep",
    "Int64Rep",
    "WordRep",
    "Word8Rep",
    "Word16Rep",
    "Word32Rep",
    "Word64Rep",
    "AddrRep",
    "FloatRep",
    "DoubleRep",
    "TupleRep",
    "SumRep",
    "VecRep",
];

const LEVITY_CONSTRUCTORS_CHIRHO: &[&str] = &["Lifted", "Unlifted"];
const MULTIPLICITY_CONSTRUCTORS_CHIRHO: &[&str] = &["One", "Many"];
const VEC_COUNT_CONSTRUCTORS_CHIRHO: &[&str] = &["Vec2", "Vec4", "Vec8", "Vec16", "Vec32", "Vec64"];
const VEC_ELEM_CONSTRUCTORS_CHIRHO: &[&str] = &[
    "Int8ElemRep",
    "Int16ElemRep",
    "Int32ElemRep",
    "Int64ElemRep",
    "Word8ElemRep",
    "Word16ElemRep",
    "Word32ElemRep",
    "Word64ElemRep",
    "FloatElemRep",
    "DoubleElemRep",
];
const ERROR_MESSAGE_CONSTRUCTORS_CHIRHO: &[&str] = &["Text", "ShowType", ":<>:", ":$$:"];

/// Normalize a value/operator spelling used by source imports and interfaces.
pub(crate) fn canonical_value_name_chirho(name_chirho: &str) -> String {
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

/// Normalize a type/operator spelling, including explicit `type` import/export syntax.
pub(crate) fn canonical_type_name_chirho(name_chirho: &str) -> String {
    canonical_value_name_chirho(name_chirho.strip_prefix("type ").unwrap_or(name_chirho))
}

/// Select the associated types named by a `Class`, `Class(..)`, or
/// `Class(Member, ...)` import/export item.
pub(crate) fn selected_associated_type_names_chirho(
    exports_chirho: &IfaceExportsChirho,
    parent_name_chirho: &str,
    members_chirho: &ExportMembersChirho,
) -> Vec<String> {
    let canonical_parent_chirho = canonical_type_name_chirho(parent_name_chirho);
    let Some(associated_names_chirho) = exports_chirho
        .associated_types_chirho
        .get(&canonical_parent_chirho)
    else {
        return Vec::new();
    };

    match members_chirho {
        ExportMembersChirho::AllChirho => associated_names_chirho.clone(),
        ExportMembersChirho::SomeChirho(selected_names_chirho) => {
            let selected_names_chirho: Vec<String> = selected_names_chirho
                .iter()
                .map(|name_chirho| canonical_type_name_chirho(name_chirho.text_chirho()))
                .collect();
            associated_names_chirho
                .iter()
                .filter(|name_chirho| selected_names_chirho.contains(name_chirho))
                .cloned()
                .collect()
        }
        ExportMembersChirho::NoneChirho => Vec::new(),
    }
}

/// Copy selected associated type exports and retain their parent relation.
pub(crate) fn merge_associated_type_exports_chirho(
    result_chirho: &mut IfaceExportsChirho,
    source_chirho: &IfaceExportsChirho,
    parent_name_chirho: &str,
    associated_names_chirho: &[String],
) {
    if associated_names_chirho.is_empty() {
        return;
    }
    let canonical_parent_chirho = canonical_type_name_chirho(parent_name_chirho);
    let result_names_chirho = result_chirho
        .associated_types_chirho
        .entry(canonical_parent_chirho)
        .or_default();
    for associated_name_chirho in associated_names_chirho {
        let canonical_name_chirho = canonical_type_name_chirho(associated_name_chirho);
        if let Some(type_chirho) = source_chirho.types_chirho.get(&canonical_name_chirho) {
            result_chirho
                .types_chirho
                .insert(canonical_name_chirho.clone(), type_chirho.clone());
        }
        if !result_names_chirho.contains(&canonical_name_chirho) {
            result_names_chirho.push(canonical_name_chirho);
        }
    }
}

/// Copy every associated type relation when all exports are imported/re-exported.
pub(crate) fn merge_all_associated_type_exports_chirho(
    result_chirho: &mut IfaceExportsChirho,
    source_chirho: &IfaceExportsChirho,
) {
    for (parent_name_chirho, associated_names_chirho) in &source_chirho.associated_types_chirho {
        merge_associated_type_exports_chirho(
            result_chirho,
            source_chirho,
            parent_name_chirho,
            associated_names_chirho,
        );
    }
}

/// Remove selected associated type exports and their parent relation.
pub(crate) fn remove_associated_type_exports_chirho(
    result_chirho: &mut IfaceExportsChirho,
    parent_name_chirho: &str,
    associated_names_chirho: &[String],
) {
    let canonical_parent_chirho = canonical_type_name_chirho(parent_name_chirho);
    for associated_name_chirho in associated_names_chirho {
        result_chirho
            .types_chirho
            .remove(&canonical_type_name_chirho(associated_name_chirho));
    }
    let remove_parent_chirho = result_chirho
        .associated_types_chirho
        .get_mut(&canonical_parent_chirho)
        .is_some_and(|result_names_chirho| {
            result_names_chirho.retain(|result_name_chirho| {
                !associated_names_chirho
                    .iter()
                    .any(|associated_name_chirho| {
                        canonical_type_name_chirho(associated_name_chirho) == *result_name_chirho
                    })
            });
            result_names_chirho.is_empty()
        });
    if remove_parent_chirho {
        result_chirho
            .associated_types_chirho
            .remove(&canonical_parent_chirho);
    }
}

/// Canonicalize built-in interface keys and complete their declared type members.
///
/// Workflow: `spec-chirho/workflows-chirho/compiler-pipeline-chirho/
/// type-scope-resolution-chirho.md`.
pub(crate) fn normalize_builtin_type_exports_chirho(modules_chirho: &mut [ModuleIfaceChirho]) {
    for module_chirho in modules_chirho {
        canonicalize_type_map_chirho(module_chirho);

        match module_chirho.name_chirho.as_str() {
            "GHC.Types" => {
                ensure_type_export_chirho(module_chirho, "UnliftedType", &[]);
                ensure_type_export_chirho(
                    module_chirho,
                    "Multiplicity",
                    MULTIPLICITY_CONSTRUCTORS_CHIRHO,
                );
            }
            "GHC.Exts" => {
                for type_name_chirho in &[
                    "Any",
                    "ByteArray#",
                    "UnliftedType",
                    "Levity",
                    "Multiplicity",
                    "VecCount",
                    "VecElem",
                    "WithDict",
                    "Word8#",
                ] {
                    ensure_type_export_chirho(module_chirho, type_name_chirho, &[]);
                }
                ensure_type_export_chirho(
                    module_chirho,
                    "RuntimeRep",
                    RUNTIME_REP_CONSTRUCTORS_CHIRHO,
                );
                ensure_type_export_chirho(module_chirho, "Levity", LEVITY_CONSTRUCTORS_CHIRHO);
                ensure_type_export_chirho(
                    module_chirho,
                    "Multiplicity",
                    MULTIPLICITY_CONSTRUCTORS_CHIRHO,
                );
                ensure_type_export_chirho(module_chirho, "VecCount", VEC_COUNT_CONSTRUCTORS_CHIRHO);
                ensure_type_export_chirho(module_chirho, "VecElem", VEC_ELEM_CONSTRUCTORS_CHIRHO);
                ensure_associated_type_export_chirho(module_chirho, "IsList", "Item");
            }
            "GHC.Prim" => {
                ensure_type_export_chirho(module_chirho, "ThreadId#", &[]);
            }
            "GHC.TypeLits" | "GHC.TypeNats" => {
                for type_name_chirho in &[
                    "Natural",
                    "+",
                    "-",
                    "*",
                    "^",
                    "<=",
                    "<=?",
                    "CmpNat",
                    "Div",
                    "Mod",
                    "Log2",
                    "CharToNat",
                    "NatToChar",
                    "UnconsSymbol",
                ] {
                    ensure_type_export_chirho(module_chirho, type_name_chirho, &[]);
                }
                ensure_type_export_chirho(
                    module_chirho,
                    "ErrorMessage",
                    ERROR_MESSAGE_CONSTRUCTORS_CHIRHO,
                );
            }
            "Data.Type.Equality" => {
                ensure_type_export_chirho(module_chirho, ":~:", &["Refl"]);
                ensure_type_export_chirho(module_chirho, ":~~:", &["HRefl"]);
                ensure_type_export_chirho(module_chirho, "==", &[]);
            }
            "Data.Typeable" => {
                ensure_type_export_chirho(module_chirho, ":~:", &["Refl"]);
            }
            "Data.Proxy" => {
                ensure_type_export_chirho(module_chirho, "Proxy", &["Proxy"]);
            }
            "Data.Data" => {
                ensure_type_export_chirho(module_chirho, "Proxy", &["Proxy"]);
                ensure_type_export_chirho(module_chirho, ":~:", &["Refl"]);
            }
            "Language.Haskell.TH" => {
                for type_name_chirho in &["Q", "Pat"] {
                    ensure_type_export_chirho(module_chirho, type_name_chirho, &[]);
                }
            }
            "Language.Haskell.TH.Syntax" => {
                ensure_type_export_chirho(module_chirho, "Q", &[]);
                ensure_type_export_chirho(module_chirho, "Lift", &[]);
            }
            "GHC.Base" => {
                for type_name_chirho in
                    &["ByteArray#", "Int#", "Word#", "Char#", "Float#", "Double#"]
                {
                    ensure_type_export_chirho(module_chirho, type_name_chirho, &[]);
                }
            }
            "GHC.Generics" => {
                ensure_type_export_chirho(module_chirho, "Generically", &["Generically"]);
                ensure_type_export_chirho(module_chirho, "NoSelector", &[]);
            }
            "Control.Monad.Trans.Reader" => {
                ensure_type_export_chirho(module_chirho, "Reader", &[]);
            }
            "GHC.IO.Handle.Types" | "GHC.IO.Handle.Internals" => {
                ensure_type_export_chirho(module_chirho, "Handle__", &[]);
            }
            "System.Posix.Types" => {
                ensure_type_export_chirho(module_chirho, "CClockId", &[]);
            }
            "Text.ParserCombinators.Parsec" => {
                ensure_type_export_chirho(module_chirho, "GenParser", &[]);
            }
            _ => {}
        }
    }
}

fn canonicalize_type_map_chirho(module_chirho: &mut ModuleIfaceChirho) {
    let previous_types_chirho = std::mem::take(&mut module_chirho.exports_chirho.types_chirho);
    let mut canonical_types_chirho = HashMap::with_capacity(previous_types_chirho.len());

    for (key_chirho, mut type_chirho) in previous_types_chirho {
        let canonical_key_chirho = canonical_type_name_chirho(&key_chirho);
        type_chirho.name_chirho = canonical_type_name_chirho(&type_chirho.name_chirho);
        match canonical_types_chirho.get_mut(&canonical_key_chirho) {
            Some(existing_chirho) => merge_type_members_chirho(existing_chirho, type_chirho),
            None => {
                canonical_types_chirho.insert(canonical_key_chirho, type_chirho);
            }
        }
    }

    module_chirho.exports_chirho.types_chirho = canonical_types_chirho;

    let previous_members_chirho =
        std::mem::take(&mut module_chirho.exports_chirho.associated_types_chirho);
    for (parent_chirho, members_chirho) in previous_members_chirho {
        let canonical_parent_chirho = canonical_type_name_chirho(&parent_chirho);
        let canonical_members_chirho = module_chirho
            .exports_chirho
            .associated_types_chirho
            .entry(canonical_parent_chirho)
            .or_default();
        for member_chirho in members_chirho {
            let canonical_member_chirho = canonical_type_name_chirho(&member_chirho);
            if !canonical_members_chirho.contains(&canonical_member_chirho) {
                canonical_members_chirho.push(canonical_member_chirho);
            }
        }
    }
}

fn ensure_type_export_chirho(
    module_chirho: &mut ModuleIfaceChirho,
    name_chirho: &str,
    constructors_chirho: &[&str],
) {
    let canonical_name_chirho = canonical_type_name_chirho(name_chirho);
    let type_chirho = module_chirho
        .exports_chirho
        .types_chirho
        .entry(canonical_name_chirho.clone())
        .or_insert_with(|| IfaceTypeChirho {
            name_chirho: canonical_name_chirho,
            constructors_chirho: Vec::new(),
            methods_chirho: Vec::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        });

    for constructor_chirho in constructors_chirho {
        let canonical_constructor_chirho = canonical_value_name_chirho(constructor_chirho);
        if !type_chirho
            .constructors_chirho
            .contains(&canonical_constructor_chirho)
        {
            type_chirho
                .constructors_chirho
                .push(canonical_constructor_chirho.clone());
        }
        module_chirho
            .exports_chirho
            .values_chirho
            .entry(canonical_constructor_chirho.clone())
            .or_insert_with(|| IfaceValueChirho {
                name_chirho: canonical_constructor_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            });
    }
}

fn ensure_associated_type_export_chirho(
    module_chirho: &mut ModuleIfaceChirho,
    parent_name_chirho: &str,
    associated_name_chirho: &str,
) {
    ensure_type_export_chirho(module_chirho, parent_name_chirho, &[]);
    ensure_type_export_chirho(module_chirho, associated_name_chirho, &[]);

    let canonical_parent_chirho = canonical_type_name_chirho(parent_name_chirho);
    let canonical_associated_chirho = canonical_type_name_chirho(associated_name_chirho);
    let associated_names_chirho = module_chirho
        .exports_chirho
        .associated_types_chirho
        .entry(canonical_parent_chirho)
        .or_default();
    if !associated_names_chirho.contains(&canonical_associated_chirho) {
        associated_names_chirho.push(canonical_associated_chirho);
    }
}

fn merge_type_members_chirho(target_chirho: &mut IfaceTypeChirho, source_chirho: IfaceTypeChirho) {
    for constructor_chirho in source_chirho.constructors_chirho {
        if !target_chirho
            .constructors_chirho
            .contains(&constructor_chirho)
        {
            target_chirho.constructors_chirho.push(constructor_chirho);
        }
    }
    for method_chirho in source_chirho.methods_chirho {
        if !target_chirho.methods_chirho.contains(&method_chirho) {
            target_chirho.methods_chirho.push(method_chirho);
        }
    }
}

/// Collect every locally declared value/type name before applying an export list.
///
/// Associated type and data families are exported as type names in their own
/// right while retaining their parent-class relationship for `Class(..)`.
/// Workflow: `spec-chirho/workflows-chirho/compiler-pipeline-chirho/
/// type-scope-resolution-chirho.md`.
pub(crate) fn collect_all_definitions_chirho(module_chirho: &ModuleChirho) -> IfaceExportsChirho {
    let mut exports_chirho = IfaceExportsChirho::default();

    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                span_chirho,
                ..
            } => {
                let type_name_chirho = canonical_type_name_chirho(name_chirho.text_chirho());
                let constructor_names_chirho: Vec<String> = constructors_chirho
                    .iter()
                    .map(|constructor_chirho| {
                        canonical_value_name_chirho(con_decl_name_chirho(constructor_chirho))
                    })
                    .collect();
                let field_names_chirho: Vec<String> = constructors_chirho
                    .iter()
                    .flat_map(con_decl_field_names_chirho)
                    .map(|field_chirho| canonical_value_name_chirho(&field_chirho))
                    .collect();

                for value_name_chirho in constructor_names_chirho
                    .iter()
                    .chain(field_names_chirho.iter())
                {
                    exports_chirho.values_chirho.insert(
                        value_name_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho: value_name_chirho.clone(),
                            span_chirho: *span_chirho,
                        },
                    );
                }
                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: type_name_chirho,
                        constructors_chirho: constructor_names_chirho,
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
                let type_name_chirho = canonical_type_name_chirho(name_chirho.text_chirho());
                let constructor_name_chirho =
                    canonical_value_name_chirho(con_decl_name_chirho(constructor_chirho));
                let field_names_chirho: Vec<String> =
                    con_decl_field_names_chirho(constructor_chirho)
                        .into_iter()
                        .map(|field_chirho| canonical_value_name_chirho(&field_chirho))
                        .collect();

                for value_name_chirho in
                    std::iter::once(&constructor_name_chirho).chain(field_names_chirho.iter())
                {
                    exports_chirho.values_chirho.insert(
                        value_name_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho: value_name_chirho.clone(),
                            span_chirho: *span_chirho,
                        },
                    );
                }
                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: type_name_chirho,
                        constructors_chirho: vec![constructor_name_chirho],
                        methods_chirho: field_names_chirho,
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                span_chirho,
                ..
            }
            | DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                insert_type_export_chirho(
                    &mut exports_chirho,
                    name_chirho.text_chirho(),
                    *span_chirho,
                );
            }
            DeclChirho::ClassDeclChirho {
                name_chirho,
                methods_chirho,
                associated_tfs_chirho,
                span_chirho,
                ..
            } => {
                let class_name_chirho = canonical_type_name_chirho(name_chirho.text_chirho());
                let method_names_chirho: Vec<String> = methods_chirho
                    .iter()
                    .map(|method_chirho| {
                        canonical_value_name_chirho(method_chirho.name_chirho.text_chirho())
                    })
                    .collect();
                for method_name_chirho in &method_names_chirho {
                    exports_chirho.values_chirho.insert(
                        method_name_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho: method_name_chirho.clone(),
                            span_chirho: *span_chirho,
                        },
                    );
                }

                let associated_names_chirho: Vec<String> = associated_tfs_chirho
                    .iter()
                    .map(|family_chirho| {
                        canonical_type_name_chirho(family_chirho.name_chirho.text_chirho())
                    })
                    .collect();
                for associated_name_chirho in &associated_names_chirho {
                    insert_type_export_chirho(
                        &mut exports_chirho,
                        associated_name_chirho,
                        *span_chirho,
                    );
                }
                if !associated_names_chirho.is_empty() {
                    exports_chirho
                        .associated_types_chirho
                        .insert(class_name_chirho.clone(), associated_names_chirho);
                }
                exports_chirho.types_chirho.insert(
                    class_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: class_name_chirho,
                        constructors_chirho: Vec::new(),
                        methods_chirho: method_names_chirho,
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::FunBindChirho {
                name_chirho,
                span_chirho,
                ..
            }
            | DeclChirho::ForeignDeclChirho {
                name_chirho,
                span_chirho,
                ..
            }
            | DeclChirho::PatSynDeclChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                let value_name_chirho = canonical_value_name_chirho(name_chirho.text_chirho());
                exports_chirho.values_chirho.insert(
                    value_name_chirho.clone(),
                    IfaceValueChirho {
                        name_chirho: value_name_chirho,
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::PatBindChirho {
                pat_chirho,
                span_chirho,
                ..
            } => {
                for bound_name_chirho in pat_bound_names_chirho(pat_chirho) {
                    let value_name_chirho = canonical_value_name_chirho(&bound_name_chirho);
                    exports_chirho.values_chirho.insert(
                        value_name_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho: value_name_chirho,
                            span_chirho: *span_chirho,
                        },
                    );
                }
            }
            DeclChirho::TypeSigChirho { .. }
            | DeclChirho::InstanceDeclChirho { .. }
            | DeclChirho::FixityDeclChirho { .. }
            | DeclChirho::DefaultDeclChirho { .. }
            | DeclChirho::TypeFamilyInstanceDeclChirho { .. }
            | DeclChirho::SpliceDeclChirho { .. }
            | DeclChirho::StandaloneDerivingDeclChirho { .. } => {}
        }
    }

    exports_chirho
}

fn insert_type_export_chirho(
    exports_chirho: &mut IfaceExportsChirho,
    name_chirho: &str,
    span_chirho: SpanChirho,
) {
    let type_name_chirho = canonical_type_name_chirho(name_chirho);
    exports_chirho.types_chirho.insert(
        type_name_chirho.clone(),
        IfaceTypeChirho {
            name_chirho: type_name_chirho,
            constructors_chirho: Vec::new(),
            methods_chirho: Vec::new(),
            span_chirho,
        },
    );
}

fn con_decl_name_chirho(decl_chirho: &ConDeclChirho) -> &str {
    match decl_chirho {
        ConDeclChirho::OrdinaryChirho { name_chirho, .. }
        | ConDeclChirho::RecordChirho { name_chirho, .. }
        | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho.text_chirho(),
    }
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
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn pat_bound_names_chirho(pat_chirho: &PatChirho) -> Vec<String> {
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
                names_chirho.extend(pat_bound_names_chirho(inner_pat_chirho));
            }
        }
        PatChirho::ConChirho { args_chirho, .. } => {
            for inner_pat_chirho in args_chirho {
                names_chirho.extend(pat_bound_names_chirho(inner_pat_chirho));
            }
        }
        PatChirho::RecordChirho { fields_chirho, .. } => {
            for field_chirho in fields_chirho {
                names_chirho.extend(pat_bound_names_chirho(&field_chirho.pattern_chirho));
            }
        }
        PatChirho::InfixConChirho {
            left_chirho,
            right_chirho,
            ..
        } => {
            names_chirho.extend(pat_bound_names_chirho(left_chirho));
            names_chirho.extend(pat_bound_names_chirho(right_chirho));
        }
        PatChirho::AsChirho {
            name_chirho,
            pattern_chirho,
            ..
        } => {
            names_chirho.push(name_chirho.text_chirho().to_string());
            names_chirho.extend(pat_bound_names_chirho(pattern_chirho));
        }
        PatChirho::BangChirho { inner_chirho, .. }
        | PatChirho::LazyChirho { inner_chirho, .. }
        | PatChirho::ParenChirho { inner_chirho, .. }
        | PatChirho::ViewChirho {
            pat_chirho: inner_chirho,
            ..
        }
        | PatChirho::TypeAnnotChirho {
            pat_chirho: inner_chirho,
            ..
        } => {
            names_chirho.extend(pat_bound_names_chirho(inner_chirho));
        }
    }
    names_chirho
}

#[cfg(test)]
#[path = "type_exports_tests_chirho.rs"]
mod tests_chirho;
