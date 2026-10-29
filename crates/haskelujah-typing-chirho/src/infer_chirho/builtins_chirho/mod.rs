// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The builtin type environment, seeded by sharded parts in its original order: constructors,
//! then core, effects, classes and surface. Module declarations later remove and rebind any of
//! these names (`collect_module_shadowed_names_chirho`).

mod classes_chirho;
mod constructors_chirho;
mod core_chirho;
mod effects_chirho;
mod qualified_alias_agreement_chirho;
mod surface_chirho;
mod transformer_types_chirho;

use std::collections::HashMap;

use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub use qualified_alias_agreement_chirho::QUALIFIED_ALIAS_AGREEMENT_CHIRHO;

/// The standard Enum enumeration scheme, shared by builtin seeding and range syntax.
/// Range syntax selects this method independently of the lexical value environment.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub(super) fn enumeration_scheme_chirho(name_chirho: &str) -> Option<SchemeChirho> {
    let (var_chirho, arity_chirho) = match name_chirho {
        "enumFrom" => (TyVarChirho(3266), 1),
        "enumFromThen" => (TyVarChirho(3267), 2),
        "enumFromTo" => (TyVarChirho(3268), 2),
        "enumFromThenTo" => (TyVarChirho(3269), 3),
        _ => return None,
    };
    let elem_chirho = TyChirho::VarChirho(var_chirho);
    Some(SchemeChirho {
        vars_chirho: vec![var_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Enum".to_string(),
            ty_chirho: elem_chirho.clone(),
            extra_tys_chirho: Vec::new(),
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![elem_chirho.clone(); arity_chirho],
            TyChirho::ListChirho(Box::new(elem_chirho)),
        ),
    })
}

/// The scheme a qualified import of builtin module `module_chirho` gives `name_chirho` when no
/// `module.name` key exists (row 608): the bare builtin scheme, exactly where GHC 9.14.1 types the
/// export as that scheme. `None` keeps the accept-anything placeholder. The generated table is the
/// whole decision.
/// workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
pub fn qualified_builtin_alias_scheme_chirho(
    module_chirho: &str,
    name_chirho: &str,
    builtins_chirho: &HashMap<String, SchemeChirho>,
) -> Option<SchemeChirho> {
    QUALIFIED_ALIAS_AGREEMENT_CHIRHO
        .binary_search_by(|(entry_module_chirho, entry_name_chirho, _)| {
            (*entry_module_chirho, *entry_name_chirho).cmp(&(module_chirho, name_chirho))
        })
        .ok()
        .and_then(|_| builtins_chirho.get(name_chirho).cloned())
}

/// Seed the type environment with Haskell Prelude types.
// workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
pub(super) fn seed_builtins_chirho(env_chirho: &mut TyEnvChirho) {
    constructors_chirho::seed_constructors_chirho(env_chirho);
    core_chirho::seed_core_chirho(env_chirho);
    effects_chirho::seed_effects_chirho(env_chirho);
    classes_chirho::seed_classes_chirho(env_chirho);
    surface_chirho::seed_surface_chirho(env_chirho);
}
