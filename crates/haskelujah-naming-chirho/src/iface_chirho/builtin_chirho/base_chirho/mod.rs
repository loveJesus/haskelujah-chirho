// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces for the base library outside `GHC.*`, one file per family (row 648).

mod classes_chirho;
mod concurrency_chirho;
mod control_chirho;
mod numbers_chirho;
mod prelude_chirho;
mod reflection_chirho;

use crate::iface_chirho::ModuleIfaceChirho;

/// Push every family's interfaces for the base library outside `GHC.*`, family by family.
pub(super) fn push_base_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    prelude_chirho::push_prelude_chirho(modules_chirho);
    classes_chirho::push_classes_chirho(modules_chirho);
    reflection_chirho::push_reflection_chirho(modules_chirho);
    numbers_chirho::push_numbers_chirho(modules_chirho);
    control_chirho::push_control_chirho(modules_chirho);
    concurrency_chirho::push_concurrency_chirho(modules_chirho);
}
