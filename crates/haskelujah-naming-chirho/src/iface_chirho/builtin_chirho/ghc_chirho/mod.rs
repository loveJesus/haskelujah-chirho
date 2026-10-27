// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces for GHC's own `GHC.*` modules, one file per family (row 648).

mod exts_chirho;
mod generics_chirho;
mod io_chirho;
mod numeric_chirho;
mod prim_chirho;
#[cfg(test)]
mod tests_chirho;

use crate::iface_chirho::ModuleIfaceChirho;

/// Push every family's interfaces for GHC's own `GHC.*` modules, family by family.
pub(super) fn push_ghc_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    exts_chirho::push_exts_chirho(modules_chirho);
    prim_chirho::push_prim_chirho(modules_chirho);
    generics_chirho::push_generics_chirho(modules_chirho);
    numeric_chirho::push_numeric_chirho(modules_chirho);
    io_chirho::push_io_chirho(modules_chirho);
}
