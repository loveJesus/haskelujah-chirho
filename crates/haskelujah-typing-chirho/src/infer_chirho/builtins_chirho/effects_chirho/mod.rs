// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, parts 05-08 (original lines 11594-15382), called in order.

mod part_05_chirho;
mod part_06_chirho;
mod part_07_chirho;
mod part_08_chirho;
#[cfg(test)]
mod part_08_tests_chirho;

use crate::env_chirho::TyEnvChirho;

pub(super) fn seed_effects_chirho(env_chirho: &mut TyEnvChirho) {
    part_05_chirho::seed_part_05_chirho(env_chirho);
    part_06_chirho::seed_part_06_chirho(env_chirho);
    part_07_chirho::seed_part_07_chirho(env_chirho);
    part_08_chirho::seed_part_08_chirho(env_chirho);
}
