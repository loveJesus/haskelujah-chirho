// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Canonical type-export names and the small wired-in type-member inventory.
//!
//! Module interfaces are the authority used by source name resolution. Keeping
//! operator canonicalization and wired-in data-constructor membership here
//! prevents the resolver from guessing that an unknown source spelling is
//! available merely because a later compiler phase happens to recognize it.

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::module_chirho::{ExportMembersChirho, ModuleChirho};
use haskelujah_ast_chirho::name_chirho::{
    CompilerRepresentationChirho, TypeConstructorDeclarationChirho, TypeConstructorRoleChirho,
};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::{
    IfaceExportsChirho, IfaceTypeChirho, IfaceValueChirho, ModuleIfaceChirho,
};

pub(crate) const RUNTIME_REP_CONSTRUCTORS_CHIRHO: &[&str] =
    CompilerRepresentationChirho::RuntimeRepChirho.constructors_chirho();
pub(crate) const LEVITY_CONSTRUCTORS_CHIRHO: &[&str] =
    CompilerRepresentationChirho::LevityChirho.constructors_chirho();

const MULTIPLICITY_CONSTRUCTORS_CHIRHO: &[&str] = &["One", "Many"];
const VEC_COUNT_CONSTRUCTORS_CHIRHO: &[&str] =
    CompilerRepresentationChirho::VecCountChirho.constructors_chirho();
const VEC_ELEM_CONSTRUCTORS_CHIRHO: &[&str] =
    CompilerRepresentationChirho::VecElemChirho.constructors_chirho();
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

/// Export members retain explicit namespaces; import admission keeps its existing
/// rules until its own carrier is reviewed. A variable-symbol associated type
/// requires the written `type` marker; constructor-symbol policy is unchanged.
pub(crate) fn selected_associated_type_export_names_chirho(
    exports_chirho: &IfaceExportsChirho,
    parent_name_chirho: &str,
    members_chirho: &ExportMembersChirho,
) -> Vec<String> {
    let selected_chirho =
        selected_associated_type_names_chirho(exports_chirho, parent_name_chirho, members_chirho);
    let ExportMembersChirho::SomeChirho(members_chirho) = members_chirho else {
        return selected_chirho;
    };
    selected_chirho
        .into_iter()
        .filter(|name_chirho| {
            name_chirho.starts_with(':')
                || name_chirho.chars().any(|character_chirho| {
                    character_chirho.is_alphanumeric() || character_chirho == '_'
                })
                || members_chirho.iter().any(|member_chirho| {
                    member_chirho.is_explicit_type_member_chirho()
                        && canonical_type_name_chirho(member_chirho.text_chirho()) == *name_chirho
                })
        })
        .collect()
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

        // GHC 9.14.1, measured through a qualified reference: these modules
        // export the heterogeneous equality `~~`; the Prelude does not
        // (`P.~~` is GHC-76037). Declaration contexts such as boring's
        // `instance a Eq.~~ b => Boring (a Eq.:~~: b)` reach the scope check
        // since contexts are lowered with the type grammar.
        // Workflow: language-features-chirho/flat-type-syntax-chirho.
        if matches!(
            module_chirho.name_chirho.as_str(),
            "Data.Type.Equality" | "GHC.Exts" | "GHC.Types"
        ) {
            ensure_type_export_chirho(module_chirho, "~~", &[]);
        }

        match module_chirho.name_chirho.as_str() {
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
            declaration_chirho: None,
            name_chirho: canonical_name_chirho,
            constructors_chirho: Vec::new(),
            methods_chirho: Vec::new(),
            is_class_chirho: false,
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
                availability_chirho:
                    crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                name_chirho: canonical_constructor_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                defining_module_chirho: None,
                constructor_fields_chirho: None,
                compiler_value_chirho: None,
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
    if target_chirho.declaration_chirho != source_chirho.declaration_chirho {
        target_chirho.declaration_chirho = None;
    }
    target_chirho.is_class_chirho |= source_chirho.is_class_chirho;
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

/// A written ordinary defining header supplies positive algebraic authority.
/// Direct ASTs and recovery placeholders without that producer mark stay unknown.
/// Non-algebraic declaration variants can safely retain their explicit category.
/// Workflow: compiler-pipeline-chirho/type-scope-resolution-chirho#ordinary-type-definition-proof-chirho.
pub(crate) fn source_type_declaration_chirho(
    module_chirho: &ModuleChirho,
    declaration_chirho: &DeclChirho,
) -> Option<TypeConstructorDeclarationChirho> {
    let (name_chirho, role_chirho) = match declaration_chirho {
        DeclChirho::DataDeclChirho { name_chirho, .. } => {
            if name_chirho.written_type_declaration_role_chirho()
                != Some(TypeConstructorRoleChirho::DataChirho)
            {
                return None;
            }
            (name_chirho, TypeConstructorRoleChirho::DataChirho)
        }
        DeclChirho::NewtypeDeclChirho { name_chirho, .. } => {
            if name_chirho.written_type_declaration_role_chirho()
                != Some(TypeConstructorRoleChirho::NewtypeChirho)
            {
                return None;
            }
            (name_chirho, TypeConstructorRoleChirho::NewtypeChirho)
        }
        DeclChirho::ClassDeclChirho { name_chirho, .. } => {
            (name_chirho, TypeConstructorRoleChirho::ClassChirho)
        }
        DeclChirho::TypeAliasDeclChirho { name_chirho, .. } => {
            (name_chirho, TypeConstructorRoleChirho::TypeSynonymChirho)
        }
        DeclChirho::TypeFamilyDeclChirho {
            name_chirho,
            data_family_chirho,
            ..
        } => (
            name_chirho,
            if *data_family_chirho {
                TypeConstructorRoleChirho::DataFamilyChirho
            } else {
                TypeConstructorRoleChirho::TypeFamilyChirho
            },
        ),
        _ => return None,
    };
    if name_chirho.text_chirho().is_empty() {
        return None;
    }
    Some(TypeConstructorDeclarationChirho::source_chirho(
        &module_chirho.name_chirho.full_name_chirho(),
        name_chirho.text_chirho(),
        role_chirho,
    ))
}

/// Collect every locally declared value/type name before applying an export list.
///
/// Associated type and data families are exported as type names in their own
/// right while retaining their parent-class relationship for `Class(..)`.
/// Workflow: `spec-chirho/workflows-chirho/compiler-pipeline-chirho/
/// type-scope-resolution-chirho.md`.
pub(crate) fn collect_all_definitions_chirho(
    module_chirho: &ModuleChirho,
    type_constructor_scope_chirho: &haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho,
    projection_budget_chirho: &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho,
) -> Result<IfaceExportsChirho, haskelujah_ast_chirho::decl_chirho::ConstructorSignatureErrorChirho>
{
    let mut exports_chirho = IfaceExportsChirho::default();
    // Every value collected here is this module's own definition (row 642).
    // workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
    let defining_module_chirho = Some(module_chirho.name_chirho.full_name_chirho());

    for decl_chirho in &module_chirho.decls_chirho {
        let declaration_chirho = source_type_declaration_chirho(module_chirho, decl_chirho);
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
                            availability_chirho:
                                crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                            name_chirho: value_name_chirho.clone(),
                            span_chirho: *span_chirho,
                            defining_module_chirho: defining_module_chirho.clone(),
                            constructor_fields_chirho: None,
                            compiler_value_chirho: None,
                        },
                    );
                }
                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        declaration_chirho: declaration_chirho.clone(),
                        name_chirho: type_name_chirho,
                        constructors_chirho: constructor_names_chirho,
                        methods_chirho: field_names_chirho,
                        is_class_chirho: false,
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
                            availability_chirho:
                                crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                            name_chirho: value_name_chirho.clone(),
                            span_chirho: *span_chirho,
                            defining_module_chirho: defining_module_chirho.clone(),
                            constructor_fields_chirho: None,
                            compiler_value_chirho: None,
                        },
                    );
                }
                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        declaration_chirho: declaration_chirho.clone(),
                        name_chirho: type_name_chirho,
                        constructors_chirho: vec![constructor_name_chirho],
                        methods_chirho: field_names_chirho,
                        is_class_chirho: false,
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
                    declaration_chirho.clone(),
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
                            availability_chirho:
                                crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                            name_chirho: method_name_chirho.clone(),
                            span_chirho: *span_chirho,
                            defining_module_chirho: defining_module_chirho.clone(),
                            constructor_fields_chirho: None,
                            compiler_value_chirho: None,
                        },
                    );
                }

                let associated_names_chirho: Vec<String> = associated_tfs_chirho
                    .iter()
                    .map(|family_chirho| {
                        canonical_type_name_chirho(family_chirho.name_chirho.text_chirho())
                    })
                    .collect();
                for associated_chirho in associated_tfs_chirho {
                    let role_chirho = if associated_chirho.data_family_chirho {
                        TypeConstructorRoleChirho::DataFamilyChirho
                    } else {
                        TypeConstructorRoleChirho::TypeFamilyChirho
                    };
                    insert_type_export_chirho(
                        &mut exports_chirho,
                        associated_chirho.name_chirho.text_chirho(),
                        associated_chirho.span_chirho,
                        Some(TypeConstructorDeclarationChirho::source_chirho(
                            &module_chirho.name_chirho.full_name_chirho(),
                            associated_chirho.name_chirho.text_chirho(),
                            role_chirho,
                        )),
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
                        declaration_chirho: declaration_chirho.clone(),
                        name_chirho: class_name_chirho,
                        constructors_chirho: Vec::new(),
                        methods_chirho: method_names_chirho,
                        is_class_chirho: true,
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
                        availability_chirho:
                            crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                        name_chirho: value_name_chirho,
                        span_chirho: *span_chirho,
                        defining_module_chirho: defining_module_chirho.clone(),
                        constructor_fields_chirho: None,
                        compiler_value_chirho: None,
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
                            availability_chirho:
                                crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                            name_chirho: value_name_chirho,
                            span_chirho: *span_chirho,
                            defining_module_chirho: defining_module_chirho.clone(),
                            constructor_fields_chirho: None,
                            compiler_value_chirho: None,
                        },
                    );
                }
            }
            DeclChirho::TypeSigChirho { .. }
            | DeclChirho::PatSynSigDeclChirho { .. }
            | DeclChirho::InstanceDeclChirho { .. }
            | DeclChirho::FixityDeclChirho { .. }
            | DeclChirho::DefaultDeclChirho { .. }
            | DeclChirho::TypeFamilyInstanceDeclChirho { .. }
            | DeclChirho::SpliceDeclChirho { .. }
            | DeclChirho::RoleAnnotationDeclChirho { .. }
            | DeclChirho::StandaloneDerivingDeclChirho { .. } => {}
        }
    }

    for decl_chirho in &module_chirho.decls_chirho {
        let constructors_chirho = match decl_chirho {
            DeclChirho::DataDeclChirho {
                constructors_chirho,
                ..
            } => constructors_chirho.as_slice(),
            DeclChirho::NewtypeDeclChirho {
                constructor_chirho, ..
            } => std::slice::from_ref(constructor_chirho),
            _ => continue,
        };
        for con_chirho in constructors_chirho {
            crate::iface_chirho::publish_source_constructor_fields_chirho(
                &mut exports_chirho,
                con_chirho,
                source_type_declaration_chirho(module_chirho, decl_chirho),
            );
            if let ConDeclChirho::GadtChirho {
                signature_chirho, ..
            } = con_chirho
            {
                let availability_chirho = signature_chirho.selector_availability_chirho(
                    projection_budget_chirho,
                    &|name_chirho| {
                        type_constructor_scope_chirho
                            .declaration_chirho(&name_chirho.full_name_chirho())
                            .is_some_and(|declaration_chirho| {
                                declaration_chirho.is_nominally_generative_chirho()
                            })
                    },
                )?;
                for (field_chirho, available_chirho) in signature_chirho
                    .record_fields_chirho()
                    .unwrap_or_default()
                    .iter()
                    .zip(availability_chirho)
                {
                    if !available_chirho {
                        for name_chirho in &field_chirho.names_chirho {
                            if let Some(value_chirho) = exports_chirho
                                .values_chirho
                                .get_mut(name_chirho.text_chirho())
                            {
                                value_chirho.availability_chirho = crate::iface_chirho::IfaceValueAvailabilityChirho::FieldLabelOnlyChirho;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(exports_chirho)
}

/// The types and classes this module declares whose exported member lists are
/// complete, so that an importer's `T(..)` may close on them.
///
/// Every constructor, record field and class method a declaration names is in
/// its entry, with two gaps this leaves open: a GADT constructor's record
/// fields are not lowered, and a pattern synonym bundled into an export
/// (`T(.., P)`) is lost when `..` absorbs it. So a data type with a GADT
/// constructor is not marked, and neither is any type of a module that
/// declares or enables pattern synonyms. A re-exported type is never marked
/// here: its members come from its own module's interface.
/// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
pub(crate) fn locally_complete_type_names_chirho(
    module_chirho: &ModuleChirho,
    exports_chirho: &IfaceExportsChirho,
) -> HashSet<String> {
    let may_bundle_patterns_chirho = module_chirho
        .extensions_chirho
        .iter()
        .any(|extension_chirho| extension_chirho == "PatternSynonyms")
        || module_chirho
            .decls_chirho
            .iter()
            .any(|decl_chirho| matches!(decl_chirho, DeclChirho::PatSynDeclChirho { .. }));
    if may_bundle_patterns_chirho {
        return HashSet::new();
    }
    module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                ..
            } if !constructors_chirho.iter().any(is_gadt_constructor_chirho) => Some(name_chirho),
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                constructor_chirho,
                ..
            } if !is_gadt_constructor_chirho(constructor_chirho) => Some(name_chirho),
            DeclChirho::ClassDeclChirho { name_chirho, .. } => Some(name_chirho),
            _ => None,
        })
        .map(|name_chirho| canonical_type_name_chirho(name_chirho.text_chirho()))
        .filter(|type_name_chirho| exports_chirho.types_chirho.contains_key(type_name_chirho))
        .collect()
}

fn is_gadt_constructor_chirho(constructor_chirho: &ConDeclChirho) -> bool {
    matches!(constructor_chirho, ConDeclChirho::GadtChirho { .. })
}

/// The completeness marks of two interfaces of one module, merged per type.
///
/// A type both sides export stays marked only when both mark it: the
/// unmarked side may export a member no entry lists, such as a bundled
/// pattern synonym. A type only one side exports keeps that side's mark, so
/// disjoint marked types still union. Read both sides before their types
/// merge, since a side's own types say what it exports.
/// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
pub(crate) fn merged_complete_member_types_chirho(
    existing_chirho: &IfaceExportsChirho,
    incoming_chirho: &IfaceExportsChirho,
) -> HashSet<String> {
    let exported_unmarked_chirho = |exports_chirho: &IfaceExportsChirho, type_name_chirho: &str| {
        exports_chirho.types_chirho.contains_key(type_name_chirho)
            && !exports_chirho
                .complete_member_types_chirho
                .contains(type_name_chirho)
    };
    existing_chirho
        .complete_member_types_chirho
        .union(&incoming_chirho.complete_member_types_chirho)
        .filter(|type_name_chirho| {
            !exported_unmarked_chirho(existing_chirho, type_name_chirho)
                && !exported_unmarked_chirho(incoming_chirho, type_name_chirho)
        })
        .cloned()
        .collect()
}

fn insert_type_export_chirho(
    exports_chirho: &mut IfaceExportsChirho,
    name_chirho: &str,
    span_chirho: SpanChirho,
    declaration_chirho: Option<TypeConstructorDeclarationChirho>,
) {
    let type_name_chirho = canonical_type_name_chirho(name_chirho);
    exports_chirho.types_chirho.insert(
        type_name_chirho.clone(),
        IfaceTypeChirho {
            declaration_chirho,
            name_chirho: type_name_chirho,
            constructors_chirho: Vec::new(),
            methods_chirho: Vec::new(),
            is_class_chirho: false,
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
    decl_chirho
        .record_fields_chirho()
        .unwrap_or_default()
        .iter()
        .flat_map(|field_chirho| {
            field_chirho
                .names_chirho
                .iter()
                .map(|name_chirho| name_chirho.text_chirho().to_string())
        })
        .collect()
}

fn pat_bound_names_chirho(pat_chirho: &PatChirho) -> Vec<String> {
    let mut names_chirho = Vec::new();
    match pat_chirho {
        PatChirho::VarChirho(name_chirho) => {
            names_chirho.push(name_chirho.text_chirho().to_string());
        }
        PatChirho::SpliceChirho { .. } => {} // expansion has not supplied binders
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
