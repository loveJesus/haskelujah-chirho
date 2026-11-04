// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor maps for the backend, moved from the driver root (row 691, L7): each data
//! constructor's parent type for dictionary passing, each newtype's constructor and underlying type
//! key, and the first argument type of a GADT constructor signature.

use super::*;

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

/// Build a mapping from newtype type name to (constructor name, underlying type key).
pub(crate) fn build_newtype_info_chirho(
    module_chirho: &ModuleChirho,
    promotion_scope_chirho: Option<&PromotionScopeChirho>,
) -> std::collections::HashMap<String, (String, String)> {
    let mut map_chirho = std::collections::HashMap::new();
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
                            haskelujah_core_chirho::desugar_chirho::DesugarCtxChirho::type_key_from_ast_chirho(ty_chirho, promotion_scope_chirho)
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
                            haskelujah_core_chirho::desugar_chirho::DesugarCtxChirho::type_key_from_ast_chirho(&fd_chirho.ty_chirho, promotion_scope_chirho)
                        })
                        .unwrap_or_else(|| "()".to_string());
                    (cn_chirho.text_chirho().to_string(), underlying_chirho)
                }
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                    name_chirho: cn_chirho,
                    signature_chirho,
                    ..
                } => {
                    let underlying_chirho = signature_chirho.positional_fields_chirho().next()
                        .map(|field_chirho| haskelujah_core_chirho::desugar_chirho::DesugarCtxChirho::type_key_from_ast_chirho(field_chirho.ty_chirho, promotion_scope_chirho))
                        .unwrap_or_else(|| "()".to_string());
                    (cn_chirho.text_chirho().to_string(), underlying_chirho)
                }
            };
            map_chirho.insert(type_name_chirho, con_name_chirho);
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
            .or_insert((con_name_chirho.to_string(), underlying_chirho.to_string()));
    }
    map_chirho
}
