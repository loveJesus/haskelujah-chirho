// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, parts 01-04 (original lines 7869-11592), called in order.

mod part_01_chirho;
mod part_02_chirho;
mod part_03_chirho;
mod part_04_chirho;

use crate::env_chirho::TyEnvChirho;

/// These compiler value declarations explicitly quantify a lifted Type
/// parameter. Other (including higher-kinded) builtin schemes do not use this
/// producer. Keep their ID, source and specificity payloads unchanged.
/// Workflow: type-identity-chirho/committed-meta-kinds-chirho.md.
fn declared_type_parameter_chirho(
    variable_chirho: crate::ty_chirho::TyVarChirho,
) -> crate::ty_chirho::QuantifiedVariablesChirho {
    crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(vec![
        crate::ty_chirho::QuantifiedVariableChirho {
            variable_chirho,
            kind_chirho: Some(std::sync::Arc::new(
                crate::kind_chirho::elaboration_chirho::ElaborationStateChirho::type_universe_chirho(),
            )),
            source_chirho: None,
            specificity_chirho: None,
        },
    ])
}

pub(super) fn seed_core_chirho(env_chirho: &mut TyEnvChirho) {
    part_01_chirho::seed_part_01_chirho(env_chirho);
    part_02_chirho::seed_part_02_chirho(env_chirho);
    part_03_chirho::seed_part_03_chirho(env_chirho);
    part_04_chirho::seed_part_04_chirho(env_chirho);
}
