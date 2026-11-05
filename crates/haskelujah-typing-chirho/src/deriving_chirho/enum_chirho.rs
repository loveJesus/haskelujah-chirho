// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor metadata for every local data type that derives Enum (row 669).
//!
//! The derived instance writes `toEnum`, `fromEnum`, `succ` and `pred`. GHC's derived instance
//! also writes a bounded `enumFrom` and `enumFromThen`, which stop at the last (or first)
//! constructor; the dictionary pass builds those two in Core from this metadata, by row ids and
//! a primop only, so no generated reference can be captured by a program-owned namesake.
//! `enumFromTo` and `enumFromThenTo` take the class defaults, as GHC's do.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::BuiltinClassScopeChirho;

use super::stock_identity_chirho::StockIdentityChirho;
use super::{all_nullary_chirho, con_name_chirho, declaration_heads_chirho};

/// Every local `data` type whose deriving clause names Enum and whose instance derives, with
/// no builtin class proof: every class keeps its written name.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub fn derived_enum_metadata_chirho(module_chirho: &ModuleChirho) -> HashMap<String, Vec<String>> {
    derived_enum_metadata_with_builtins_chirho(module_chirho, &BuiltinClassScopeChirho::default())
}

/// Every local `data` type whose deriving clause asks for the stock Enum class, judged with
/// the builtin classes naming proved for the module (row 694), and whose instance derives, by
/// type name: its constructor names in declaration order. These are exactly the shapes the
/// derived instance accepts, with no visible parameter and one or more constructors, all
/// nullary, so every entry has a first and a last constructor.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub fn derived_enum_metadata_with_builtins_chirho(
    module_chirho: &ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
) -> HashMap<String, Vec<String>> {
    let identity_chirho =
        StockIdentityChirho::of_module_chirho(module_chirho, builtin_classes_chirho);
    let mut metadata_chirho = HashMap::new();
    for decl_chirho in &module_chirho.decls_chirho {
        let DeclChirho::DataDeclChirho {
            name_chirho,
            type_vars_chirho,
            constructors_chirho,
            deriving_chirho,
            kind_sig_chirho,
            span_chirho,
            ..
        } = decl_chirho
        else {
            continue;
        };
        // Only the stock instance enumerates constructors (row 694).
        if !deriving_chirho
            .iter()
            .any(|derived_chirho| identity_chirho.names_stock_class_chirho(derived_chirho, "Enum"))
        {
            continue;
        }
        let visible_chirho = declaration_heads_chirho::visible_instance_binders_chirho(
            type_vars_chirho,
            kind_sig_chirho.as_ref(),
            *span_chirho,
        );
        if !visible_chirho.is_empty()
            || constructors_chirho.is_empty()
            || !all_nullary_chirho(constructors_chirho)
        {
            continue;
        }
        metadata_chirho.insert(
            name_chirho.text_chirho().to_string(),
            constructors_chirho
                .iter()
                .map(|constructor_chirho| con_name_chirho(constructor_chirho).to_string())
                .collect(),
        );
    }
    metadata_chirho
}

#[cfg(test)]
#[path = "enum_tests_chirho.rs"]
mod tests_chirho;
