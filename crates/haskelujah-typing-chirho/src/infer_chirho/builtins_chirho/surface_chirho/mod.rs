// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, parts 12-14 (original lines 18235-21009), called in order.

mod part_12_chirho;
mod part_13_chirho;
mod part_14_chirho;

use crate::env_chirho::TyEnvChirho;

pub(super) fn seed_surface_chirho(env_chirho: &mut TyEnvChirho) {
    part_12_chirho::seed_part_12_chirho(env_chirho);
    part_13_chirho::seed_part_13_chirho(env_chirho);
    part_14_chirho::seed_part_14_chirho(env_chirho);
}
