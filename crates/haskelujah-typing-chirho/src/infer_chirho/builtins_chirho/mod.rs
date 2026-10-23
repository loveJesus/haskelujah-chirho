// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The builtin type environment, seeded by sharded parts in its original order: constructors,
//! then core, effects, classes and surface. Module declarations later remove and rebind any of
//! these names (`collect_module_shadowed_names_chirho`).

mod classes_chirho;
mod constructors_chirho;
mod core_chirho;
mod effects_chirho;
mod surface_chirho;
mod transformer_types_chirho;

use crate::env_chirho::TyEnvChirho;

/// Seed the type environment with Haskell Prelude types.
// workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
pub(super) fn seed_builtins_chirho(env_chirho: &mut TyEnvChirho) {
    constructors_chirho::seed_constructors_chirho(env_chirho);
    core_chirho::seed_core_chirho(env_chirho);
    effects_chirho::seed_effects_chirho(env_chirho);
    classes_chirho::seed_classes_chirho(env_chirho);
    surface_chirho::seed_surface_chirho(env_chirho);
}
