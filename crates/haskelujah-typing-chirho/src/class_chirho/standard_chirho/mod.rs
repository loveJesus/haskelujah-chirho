// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The standard class/instance seed, in its original registration order.

use super::ClassEnvChirho;

mod additional_classes_instances_chirho;
mod arithmetic_classes_chirho;
mod effect_instances_chirho;
mod higher_kinded_classes_chirho;
mod monad_defaults_chirho;
pub use monad_defaults_chirho::monad_then_default_arms_chirho;
mod remaining_classes_chirho;
mod scalar_container_instances_chirho;

pub(super) fn seed_standard_chirho(environment_chirho: &mut ClassEnvChirho) {
    environment_chirho.seed_arithmetic_classes_chirho();
    environment_chirho.seed_higher_kinded_classes_chirho();
    environment_chirho.seed_remaining_classes_chirho();
    environment_chirho.seed_scalar_container_instances_chirho();
    environment_chirho.seed_effect_instances_chirho();
    environment_chirho.seed_additional_classes_instances_chirho();
}
