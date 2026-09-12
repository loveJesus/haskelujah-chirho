// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source frontend entry points and checked module companions.
//! The source-text entry below deliberately performs no filesystem search.

mod boot_chirho;
pub(crate) mod cpp_chirho;
mod graph_chirho;
pub(crate) mod modules_chirho;
#[cfg(test)]
pub(crate) mod package_fixtures_chirho;
pub(crate) mod search_chirho;
#[cfg(test)]
mod tests_chirho;

use super::{
    DiagnosticBundleChirho, FrontendInputsChirho, FrontendResultChirho, SourceFileChirho,
    SourceMapChirho, preprocess_cpp_chirho, run_frontend_with_inputs_chirho,
};

/// Preprocess, parse, resolve and typecheck source text, stopping before Core and
/// code generation. Uses the same builtin/stdlib interfaces as full compilation.
/// See spec-chirho/workflows-chirho/testing-chirho/execution-oracles-chirho.md.
pub fn typecheck_source_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    // Run CPP preprocessing if {-# LANGUAGE CPP #-} is present
    let preprocessed_chirho = preprocess_cpp_chirho(source_chirho);
    let effective_source_chirho = &preprocessed_chirho;

    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        source_map_chirho,
        file_name_chirho,
        effective_source_chirho,
    );
    let file_id_chirho = source_file_chirho.file_id_chirho();

    let seed_chirho =
        modules_chirho::FrontendSeedArtifactsChirho::for_source_chirho(effective_source_chirho);

    run_frontend_with_inputs_chirho(
        effective_source_chirho,
        file_id_chirho,
        FrontendInputsChirho::new_chirho(
            &seed_chirho.ifaces_chirho,
            &seed_chirho.imported_types_chirho,
            &seed_chirho.imported_type_synonyms_chirho,
            &seed_chirho.imported_type_families_chirho,
        )
        .with_type_contracts_chirho(&seed_chirho.type_contracts_chirho),
    )
}
