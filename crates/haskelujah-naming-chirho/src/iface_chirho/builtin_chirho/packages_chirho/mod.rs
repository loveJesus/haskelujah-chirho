// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces for the packages the stdlib and Hackage builds lean on, one file per family (row 648).

mod arrays_chirho;
mod bytestring_chirho;
mod collections_chirho;
mod functors_chirho;
mod maps_chirho;
mod mtl_chirho;
mod text_chirho;
mod transformers_chirho;

use crate::iface_chirho::ModuleIfaceChirho;

/// Push every family's interfaces for the packages the stdlib and Hackage builds lean on, family by family.
pub(super) fn push_packages_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    maps_chirho::push_maps_chirho(modules_chirho);
    collections_chirho::push_collections_chirho(modules_chirho);
    arrays_chirho::push_arrays_chirho(modules_chirho);
    bytestring_chirho::push_bytestring_chirho(modules_chirho);
    text_chirho::push_text_chirho(modules_chirho);
    transformers_chirho::push_transformers_chirho(modules_chirho);
    mtl_chirho::push_mtl_chirho(modules_chirho);
    functors_chirho::push_functors_chirho(modules_chirho);
}
