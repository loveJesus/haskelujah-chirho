// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor maps for the backend, moved from the driver root (row 691, L7): each data
//! constructor's parent type for dictionary passing, each newtype's constructor and underlying type
//! key, and the first argument type of a GADT constructor signature.

use super::*;

use haskelujah_core_chirho::dict_chirho::NewtypeInfoChirho;

/// Extract the first argument type from a GADT type signature.
/// Peels through forall and qualified types, then returns the first `arg` from
/// `arg -> result`.  Returns `None` for non-function types (nullary GADT ctors).
fn extract_first_fun_arg_chirho(
    ty_chirho: &haskelujah_ast_chirho::ty_chirho::TypeChirho,
) -> Option<haskelujah_ast_chirho::ty_chirho::TypeChirho> {
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;
    match ty_chirho {
        TypeChirho::FunChirho { arg_chirho, .. } => Some((**arg_chirho).clone()),
        TypeChirho::ForallChirho { body_chirho, .. } => extract_first_fun_arg_chirho(body_chirho),
        TypeChirho::QualChirho { body_chirho, .. } => extract_first_fun_arg_chirho(body_chirho),
        TypeChirho::ParenChirho { inner_chirho, .. } => extract_first_fun_arg_chirho(inner_chirho),
        _ => None,
    }
}

/// Build a mapping from data constructor names to their parent type name.
/// Used by the dictionary-passing transform to select the correct instance
/// dictionary when a class method is applied to a constructor value.
pub(crate) fn build_con_type_map_chirho(
    module_chirho: &ModuleChirho,
) -> std::collections::HashMap<String, String> {
    let mut map_chirho = std::collections::HashMap::new();
    for decl_chirho in &module_chirho.decls_chirho {
        if let haskelujah_ast_chirho::decl_chirho::DeclChirho::DataDeclChirho {
            name_chirho,
            constructors_chirho,
            ..
        } = decl_chirho
        {
            let type_name_chirho = name_chirho.text_chirho().to_string();
            for con_chirho in constructors_chirho {
                let con_name_chirho = match con_chirho {
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                        name_chirho: cn_chirho,
                        ..
                    } => cn_chirho.text_chirho().to_string(),
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                        name_chirho: cn_chirho,
                        ..
                    } => cn_chirho.text_chirho().to_string(),
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                        name_chirho: cn_chirho,
                        ..
                    } => cn_chirho.text_chirho().to_string(),
                };
                map_chirho.insert(con_name_chirho, type_name_chirho.clone());
            }
        }
    }
    // Also include newtype constructors
    for decl_chirho in &module_chirho.decls_chirho {
        if let haskelujah_ast_chirho::decl_chirho::DeclChirho::NewtypeDeclChirho {
            name_chirho,
            constructor_chirho,
            ..
        } = decl_chirho
        {
            let type_name_chirho = name_chirho.text_chirho().to_string();
            let con_name_chirho = match constructor_chirho {
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                    name_chirho: cn_chirho,
                    ..
                } => cn_chirho.text_chirho().to_string(),
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    name_chirho: cn_chirho,
                    ..
                } => cn_chirho.text_chirho().to_string(),
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                    name_chirho: cn_chirho,
                    ..
                } => cn_chirho.text_chirho().to_string(),
            };
            map_chirho.insert(con_name_chirho, type_name_chirho);
        }
    }
    // Also include built-in constructors
    map_chirho.insert("True".to_string(), "Bool".to_string());
    map_chirho.insert("False".to_string(), "Bool".to_string());
    map_chirho
}

/// Build each newtype's record by type name: its constructor, its underlying type key, and
/// the classes its own declarations derive with the newtype strategy (row 694).
/// workflow: language-features-chirho/deriving-strategies-chirho
pub(crate) fn build_newtype_info_chirho(
    module_chirho: &ModuleChirho,
) -> std::collections::HashMap<String, NewtypeInfoChirho> {
    let mut map_chirho = std::collections::HashMap::new();
    let mut derived_classes_chirho =
        haskelujah_typing_chirho::deriving_chirho::newtype_strategy_classes_chirho(module_chirho);
    for decl_chirho in &module_chirho.decls_chirho {
        if let haskelujah_ast_chirho::decl_chirho::DeclChirho::NewtypeDeclChirho {
            name_chirho,
            constructor_chirho,
            ..
        } = decl_chirho
        {
            let type_name_chirho = name_chirho.text_chirho().to_string();
            let con_name_chirho = match constructor_chirho {
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                    name_chirho: cn_chirho,
                    fields_chirho,
                    ..
                } => {
                    let underlying_chirho = fields_chirho
                        .first()
                        .map(|(_s_chirho, ty_chirho)| {
                            haskelujah_core_chirho::desugar_chirho::DesugarCtxChirho::type_key_from_ast_chirho(ty_chirho)
                        })
                        .unwrap_or_else(|| "()".to_string());
                    (cn_chirho.text_chirho().to_string(), underlying_chirho)
                }
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    name_chirho: cn_chirho,
                    fields_chirho,
                    ..
                } => {
                    let underlying_chirho = fields_chirho
                        .first()
                        .map(|fd_chirho| {
                            haskelujah_core_chirho::desugar_chirho::DesugarCtxChirho::type_key_from_ast_chirho(&fd_chirho.ty_chirho)
                        })
                        .unwrap_or_else(|| "()".to_string());
                    (cn_chirho.text_chirho().to_string(), underlying_chirho)
                }
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                    name_chirho: cn_chirho,
                    ty_chirho,
                    ..
                } => {
                    // Extract the first argument type from the GADT type signature.
                    // E.g. `forall a. Ctx => Int -> T a` → underlying is `Int`.
                    let underlying_chirho = extract_first_fun_arg_chirho(ty_chirho)
                        .map(|arg_ty_chirho| {
                            haskelujah_core_chirho::desugar_chirho::DesugarCtxChirho::type_key_from_ast_chirho(&arg_ty_chirho)
                        })
                        .unwrap_or_else(|| "()".to_string());
                    (cn_chirho.text_chirho().to_string(), underlying_chirho)
                }
            };
            let (constructor_chirho, underlying_key_chirho) = con_name_chirho;
            let newtype_derived_classes_chirho = derived_classes_chirho
                .remove(&type_name_chirho)
                .unwrap_or_default();
            map_chirho.insert(
                type_name_chirho,
                NewtypeInfoChirho {
                    constructor_chirho,
                    underlying_key_chirho,
                    newtype_derived_classes_chirho,
                },
            );
        }
    }
    for (type_name_chirho, con_name_chirho, underlying_chirho) in [
        ("Sum", "Sum", "Int"),
        ("Product", "Product", "Int"),
        ("All", "All", "Bool"),
        ("Any", "Any", "Bool"),
    ] {
        map_chirho
            .entry(type_name_chirho.to_string())
            .or_insert_with(|| NewtypeInfoChirho {
                constructor_chirho: con_name_chirho.to_string(),
                underlying_key_chirho: underlying_chirho.to_string(),
                newtype_derived_classes_chirho: Vec::new(),
            });
    }
    map_chirho
}
