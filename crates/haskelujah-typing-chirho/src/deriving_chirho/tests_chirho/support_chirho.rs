// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

/// Classes from a deriving clause that writes no strategy.
pub(super) fn unspecified_chirho(classes_chirho: &[&str]) -> Vec<DerivedClassChirho> {
    classes_chirho
        .iter()
        .map(|class_chirho| DerivedClassChirho::unspecified_chirho(var_name_chirho(class_chirho)))
        .collect()
}

pub(super) fn make_enum_module_chirho() -> ModuleChirho {
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
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Green"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("Blue"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
            ],
            deriving_chirho: unspecified_chirho(&["Eq", "Ord", "Show"]),
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
