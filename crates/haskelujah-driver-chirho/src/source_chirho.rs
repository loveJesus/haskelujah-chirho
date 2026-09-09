// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-text frontend entry shared by typecheck measurements and full compilation.
//! This path deliberately has no filesystem search and no backend generation.

pub(crate) mod cpp_chirho;
#[cfg(test)]
mod tests_chirho;

use super::{
    DiagnosticBundleChirho, FrontendResultChirho, ImportedTypeSynonymsChirho, SourceFileChirho,
    SourceMapChirho, merge_stdlib_frontend_artifacts_chirho, preprocess_cpp_chirho,
    run_frontend_chirho, run_frontend_with_type_synonyms_and_type_families_chirho,
    seed_builtin_type_families_chirho, source_imports_stdlib_chirho,
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

    let mut builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let mut imported_types_chirho = std::collections::HashMap::new();
    let mut imported_type_synonyms_chirho = ImportedTypeSynonymsChirho::new();
    let mut imported_type_families_chirho = seed_builtin_type_families_chirho();
    if source_imports_stdlib_chirho(effective_source_chirho) {
        merge_stdlib_frontend_artifacts_chirho(
            &mut builtin_ifaces_chirho,
            &mut imported_types_chirho,
            &mut imported_type_synonyms_chirho,
            &mut imported_type_families_chirho,
        );
    }

    if imported_types_chirho.is_empty()
        && imported_type_synonyms_chirho.is_empty()
        && imported_type_families_chirho.is_empty()
    {
        run_frontend_chirho(
            effective_source_chirho,
            file_id_chirho,
            &builtin_ifaces_chirho,
            &imported_types_chirho,
        )
    } else {
        run_frontend_with_type_synonyms_and_type_families_chirho(
            effective_source_chirho,
            file_id_chirho,
            &builtin_ifaces_chirho,
            &imported_types_chirho,
            &imported_type_synonyms_chirho,
            &imported_type_families_chirho,
        )
    }
}
