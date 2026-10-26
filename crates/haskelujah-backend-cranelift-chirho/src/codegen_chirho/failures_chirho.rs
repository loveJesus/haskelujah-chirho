// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The module-level half of Cranelift's failure routes: the names unbound
//! references report, embedded beside the string literals, and the refusal of
//! primitives this backend cannot lower.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

use std::collections::{BTreeSet, HashMap, HashSet};

use cranelift_module::DataId;
use haskelujah_core_chirho::expr_chirho::{CoreIdChirho, CoreModuleChirho};

use super::{FuncDeclMapChirho, collect_free_runtime_var_ids_with_known_ids_chirho};

/// What the pre-scan embeds for every function body: the string literals by
/// content, and the name each unbound free id reports, whose bytes are among
/// those strings.
pub(super) struct EmbeddedDataChirho {
    pub(super) strings_chirho: HashMap<String, DataId>,
    pub(super) unbound_names_chirho: HashMap<CoreIdChirho, String>,
}

/// Each free id of a binding that no top-level function binds, in a stable
/// order, with the name a demand of it reports: its source name, or
/// `CoreId N` when it has none.
pub(super) fn unbound_reference_names_chirho(
    module_chirho: &CoreModuleChirho,
    func_decl_map_chirho: &FuncDeclMapChirho,
) -> Vec<(CoreIdChirho, String)> {
    let known_ids_chirho: HashSet<_> = func_decl_map_chirho.keys().copied().collect();
    let mut seen_chirho = HashSet::new();
    let mut names_chirho = Vec::new();
    for binding_chirho in &module_chirho.bindings_chirho {
        for id_chirho in collect_free_runtime_var_ids_with_known_ids_chirho(
            &binding_chirho.rhs_chirho,
            &known_ids_chirho,
        ) {
            if seen_chirho.insert(id_chirho) {
                let name_chirho = module_chirho
                    .names_chirho
                    .get(&id_chirho)
                    .cloned()
                    .unwrap_or_else(|| format!("CoreId {}", id_chirho.0));
                names_chirho.push((id_chirho, name_chirho));
            }
        }
    }
    names_chirho
}

/// An executable is refused when a body it compiles uses a primitive this
/// backend cannot lower, naming each as `name/arity`, as LLVM does.
pub(super) fn refuse_unsupported_primitives_chirho(
    unsupported_chirho: &BTreeSet<String>,
) -> Result<(), String> {
    if unsupported_chirho.is_empty() {
        return Ok(());
    }
    let names_chirho: Vec<&str> = unsupported_chirho.iter().map(String::as_str).collect();
    Err(format!(
        "Cranelift does not support primitive(s): {}",
        names_chirho.join(", ")
    ))
}
