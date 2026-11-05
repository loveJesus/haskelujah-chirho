// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which class a GND request's written spelling names (row 694, #31891, Claude's H1 and H2).
//!
//! Inference keys every class by its bare text, and a local class replaces any seeded or
//! imported entry of that text, so `deriving newtype (F.WrapChirho)` beside a local
//! `WrapChirho` used to judge the local class's methods. On the module-sources path the class
//! environment also carries the previous module's whole environment, builtins included, so even
//! a builtin's bare key may hold an earlier module's homonym, and no absence of a local homonym
//! proves anything there. The entry a proof judges must be the one the spelling names, proved
//! from the module and the compiler alone:
//!
//! - the module's own class: the spelling is unqualified, or qualified with the module's own
//!   name, and the module declares that class;
//! - a builtin class: naming (K01) proves the spelling names one, the module declares nothing
//!   of that bare name, and the entry's schema (its parameters and method schemes, everything
//!   the proof reads) is the compiler's own seed (`ClassEnvChirho::seed_standard_chirho`).
//!
//! Any other spelling is a named Unsupported gap, never GHC's verdict: an imported user class,
//! a foreign class beside a local homonym, a builtin the module shadows, a carried homonym.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use haskelujah_ast_chirho::name_chirho::NameChirho;

use crate::class_chirho::ClassEnvChirho;
use crate::deriving_chirho::stock_identity_chirho::StockIdentityChirho;

/// Why `class_chirho` may not take the class-environment entry under its bare text; `None`
/// when the spelling names that entry. `seeded_chirho` is the compiler's own class seed.
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub(super) fn class_authority_gap_chirho(
    class_chirho: &NameChirho,
    identity_chirho: &StockIdentityChirho<'_>,
    class_env_chirho: &ClassEnvChirho,
    seeded_chirho: &ClassEnvChirho,
) -> Option<String> {
    if identity_chirho.names_local_chirho(class_chirho) {
        return None;
    }
    let text_chirho = class_chirho.text_chirho();
    let written_chirho = class_chirho.full_name_chirho();
    if !identity_chirho.proves_builtin_chirho(class_chirho) {
        return Some(format!(
            "Haskelujah cannot prove which class `{written_chirho}` names: it is neither this module's own class nor a builtin class"
        ));
    }
    if identity_chirho.declares_type_name_chirho(text_chirho) {
        return Some(format!(
            "`{written_chirho}` names a builtin class, but this module's own `{text_chirho}` holds that name in the checker"
        ));
    }
    let seeded_schema_chirho = match (
        class_env_chirho.classes_chirho.get(text_chirho),
        seeded_chirho.classes_chirho.get(text_chirho),
    ) {
        (Some(entry_chirho), Some(seed_chirho)) => {
            entry_chirho.var_chirho == seed_chirho.var_chirho
                && entry_chirho.extra_vars_chirho == seed_chirho.extra_vars_chirho
                && entry_chirho.methods_chirho == seed_chirho.methods_chirho
        }
        _ => false,
    };
    (!seeded_schema_chirho).then(|| {
        format!(
            "the checker's `{text_chirho}` is not the builtin class `{written_chirho}` names: another module's class may hold that name"
        )
    })
}

#[cfg(test)]
#[path = "class_authority_tests_chirho.rs"]
mod tests_chirho;
