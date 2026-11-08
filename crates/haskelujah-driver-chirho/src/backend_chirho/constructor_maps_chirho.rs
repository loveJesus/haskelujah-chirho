// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor maps for the backend, moved from the driver root (row 691, L7): each data
//! constructor's parent type for dictionary passing, each newtype's constructor and underlying type
//! key, and the first argument type of a GADT constructor signature.

use super::*;

use haskelujah_core_chirho::dict_chirho::NewtypeInfoChirho;
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

/// Build each newtype's record: its constructor, its underlying type key, and the classes its
/// own declarations derive with the newtype strategy, routed with the builtin classes naming
/// proved for the module (row 694).
///
/// The record is keyed by the declaration's canonical type key, the one the checker spells the
/// newtype's instance heads with, so the dictionary pass finds it. The declaration is taken as
/// the elaborator takes it: none for an opaque name, else the name's own stamp, else the
/// selected type-constructor scope by the written name. A name with no declaration (legacy or
/// raw Template Haskell) keeps its bare text; no identity is minted.
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub(crate) fn build_newtype_info_chirho(
    module_chirho: &ModuleChirho,
    builtin_classes_chirho: &haskelujah_ast_chirho::name_chirho::BuiltinClassScopeChirho,
    coercion_proofs_chirho: &[haskelujah_typing_chirho::GndCoercionProofChirho],
    default_method_rows_chirho: &std::collections::HashSet<String>,
    promotion_scope_chirho: Option<&PromotionScopeChirho>,
    type_constructor_scope_chirho: Option<
        &haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho,
    >,
) -> std::collections::HashMap<String, NewtypeInfoChirho> {
    let mut map_chirho = std::collections::HashMap::new();
    let mut derived_classes_chirho =
        haskelujah_typing_chirho::deriving_chirho::newtype_strategy_classes_chirho(
            module_chirho,
            builtin_classes_chirho,
        );
    for decl_chirho in &module_chirho.decls_chirho {
        if let haskelujah_ast_chirho::decl_chirho::DeclChirho::NewtypeDeclChirho {
            name_chirho,
            constructor_chirho,
            ..
        } = decl_chirho
        {
            // The bare declared text keys only the deriving-request and coercion-proof joins
            // below; the record itself is keyed by the declaration's canonical type key.
            let type_name_chirho = name_chirho.text_chirho().to_string();
            let declaration_chirho = if name_chirho.type_constructor_opaque_chirho() {
                None
            } else {
                name_chirho.type_constructor_chirho().or_else(|| {
                    type_constructor_scope_chirho.and_then(|scope_chirho| {
                        scope_chirho.declaration_chirho(&name_chirho.full_name_chirho())
                    })
                })
            };
            let type_key_chirho = declaration_chirho.map_or_else(
                || type_name_chirho.clone(),
                haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho::type_key_chirho,
            );
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
            let (constructor_chirho, underlying_key_chirho) = con_name_chirho;
            let newtype_derived_classes_chirho = derived_classes_chirho
                .remove(&type_name_chirho)
                .unwrap_or_default();
            // Typing's proofs for this newtype, and the rows of its instances the desugarer
            // filled from class defaults (row 694).
            let newtype_proofs_chirho = coercion_proofs_chirho
                .iter()
                .filter(|proof_chirho| proof_chirho.newtype_chirho() == type_name_chirho)
                .cloned()
                .collect();
            let row_suffix_chirho = format!("_{type_key_chirho}");
            let newtype_default_rows_chirho = default_method_rows_chirho
                .iter()
                .filter(|row_chirho| row_chirho.ends_with(&row_suffix_chirho))
                .cloned()
                .collect();
            map_chirho.insert(
                type_key_chirho,
                NewtypeInfoChirho {
                    constructor_chirho,
                    underlying_key_chirho,
                    newtype_derived_classes_chirho,
                    coercion_proofs_chirho: newtype_proofs_chirho,
                    default_method_rows_chirho: newtype_default_rows_chirho,
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
                ..NewtypeInfoChirho::default()
            });
    }
    map_chirho
}
