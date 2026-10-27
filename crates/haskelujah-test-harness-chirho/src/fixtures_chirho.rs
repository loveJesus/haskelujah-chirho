// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Hand-built Core for backend tests (row 628). A compiled program's provenance comes from its
//! producers: the driver records the entry and the builtin references, and the dictionary pass
//! its generated bindings. A test that builds Core by hand records the same facts here before it
//! hands the module to a backend.
//! workflow: compiler-pipeline-chirho/native-symbol-identity-chirho

use haskelujah_core_chirho::{CoreModuleChirho, module_free_reference_ids_chirho};

/// `module_chirho` with the provenance its producers would record, unless it records some
/// already. The binding spelled `main` is the entry. Every other binding is the compiler's own,
/// so a builtin spelling keeps its handling, as these fixtures were written to assume. Every free
/// reference is builtin. A test of a program's own namesake records its provenance itself.
pub fn recorded_fixture_chirho(module_chirho: &CoreModuleChirho) -> CoreModuleChirho {
    let mut recorded_chirho = module_chirho.clone();
    if recorded_chirho.provenance_chirho != Default::default() {
        return recorded_chirho;
    }
    let entry_chirho = module_chirho
        .bindings_chirho
        .iter()
        .map(|binding_chirho| &binding_chirho.binder_chirho)
        .find(|binder_chirho| binder_chirho.name_chirho == "main")
        .map(|binder_chirho| binder_chirho.id_chirho);
    let provenance_chirho = &mut recorded_chirho.provenance_chirho;
    provenance_chirho.entry_chirho = entry_chirho;
    provenance_chirho.compiler_bindings_chirho = module_chirho
        .bindings_chirho
        .iter()
        .map(|binding_chirho| binding_chirho.binder_chirho.id_chirho)
        .filter(|id_chirho| Some(*id_chirho) != entry_chirho)
        .collect();
    provenance_chirho.builtin_references_chirho = module_free_reference_ids_chirho(module_chirho);
    recorded_chirho
}
