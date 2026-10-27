// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces for system, foreign, network and tooling modules, one file per family (row 648).

mod foreign_chirho;
mod formats_chirho;
mod posix_chirho;
mod system_chirho;
mod time_network_chirho;
mod tooling_chirho;

use crate::iface_chirho::ModuleIfaceChirho;

/// Push every family's interfaces for system, foreign, network and tooling modules, family by family.
pub(super) fn push_platform_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    system_chirho::push_system_chirho(modules_chirho);
    posix_chirho::push_posix_chirho(modules_chirho);
    foreign_chirho::push_foreign_chirho(modules_chirho);
    time_network_chirho::push_time_network_chirho(modules_chirho);
    formats_chirho::push_formats_chirho(modules_chirho);
    tooling_chirho::push_tooling_chirho(modules_chirho);
}
