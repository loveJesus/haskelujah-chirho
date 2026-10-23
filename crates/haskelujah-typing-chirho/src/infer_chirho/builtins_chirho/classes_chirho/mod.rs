// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, parts 09-11 (original lines 15384-18233), called in order.

mod part_09_chirho;
mod part_10_chirho;
mod part_11_chirho;

use crate::env_chirho::TyEnvChirho;

pub(super) fn seed_classes_chirho(env_chirho: &mut TyEnvChirho) {
    part_09_chirho::seed_part_09_chirho(env_chirho);
    part_10_chirho::seed_part_10_chirho(env_chirho);
    part_11_chirho::seed_part_11_chirho(env_chirho);
}
