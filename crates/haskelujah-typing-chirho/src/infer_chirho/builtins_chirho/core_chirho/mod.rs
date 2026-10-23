// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, parts 01-04 (original lines 7869-11592), called in order.

mod part_01_chirho;
mod part_02_chirho;
mod part_03_chirho;
mod part_04_chirho;

use crate::env_chirho::TyEnvChirho;

pub(super) fn seed_core_chirho(env_chirho: &mut TyEnvChirho) {
    part_01_chirho::seed_part_01_chirho(env_chirho);
    part_02_chirho::seed_part_02_chirho(env_chirho);
    part_03_chirho::seed_part_03_chirho(env_chirho);
    part_04_chirho::seed_part_04_chirho(env_chirho);
}
