// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source roots use checked reachable companions, shared by checking and compiling.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::modules_chirho::{
    FrontendSeedArtifactsChirho, collect_frontend_artifacts_from_module_sources_chirho,
};
use crate::*;

fn checked_source_dependencies_chirho(
    source_chirho: &str,
    file_name_chirho: &str,
    search_dir_chirho: &Path,
    source_map_chirho: &mut SourceMapChirho,
) -> Result<FrontendSeedArtifactsChirho, DiagnosticBundleChirho> {
    let mut produce_chirho = || -> Result<FrontendSeedArtifactsChirho, String> {
        let sources_chirho = crate::module_search_chirho::reachable_module_sources_chirho(
            source_chirho,
            file_name_chirho,
            search_dir_chirho,
            source_map_chirho,
        )?;
        let seed_chirho = FrontendSeedArtifactsChirho::for_source_chirho(source_chirho);
        if sources_chirho.is_empty() {
            return Ok(seed_chirho);
        }
        collect_frontend_artifacts_from_module_sources_chirho(
            sources_chirho,
            source_map_chirho,
            seed_chirho.ifaces_chirho,
            seed_chirho.imported_types_chirho,
            seed_chirho.imported_type_synonyms_chirho,
            seed_chirho.imported_type_families_chirho,
            seed_chirho.type_contracts_chirho,
            true,
        )
    };
    produce_chirho().map_err(|error_chirho| {
        // Dependency offsets belong to their own files. Preserve that origin
        // in the message; never render them as offsets in the consumer's file.
        DiagnosticChirho::error_no_span_chirho(error_chirho)
            .with_code_chirho(haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(100))
            .into()
    })
}

fn frontend_with_search_path_chirho(
    source_chirho: &str,
    file_id_chirho: haskelujah_span_chirho::FileIdChirho,
    file_name_chirho: &str,
    search_dir_chirho: &Path,
    source_map_chirho: &mut SourceMapChirho,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    let seed_chirho = checked_source_dependencies_chirho(
        source_chirho,
        file_name_chirho,
        search_dir_chirho,
        source_map_chirho,
    )?;
    let imported_types_chirho = filter_seeded_imported_types_for_source_chirho(
        source_chirho,
        &seed_chirho.imported_types_chirho,
    );
    let imported_synonyms_chirho = filter_seeded_type_synonyms_for_source_chirho(
        source_chirho,
        &seed_chirho.imported_type_synonyms_chirho,
    );
    run_frontend_with_inputs_chirho(
        source_chirho,
        file_id_chirho,
        FrontendInputsChirho::new_chirho(
            &seed_chirho.ifaces_chirho,
            &imported_types_chirho,
            &imported_synonyms_chirho,
            &seed_chirho.imported_type_families_chirho,
        )
        .with_type_contracts_chirho(&seed_chirho.type_contracts_chirho),
    )
}

pub(crate) fn check_source_file_with_search_path_chirho(
    source_file_chirho: SourceFileChirho,
    execution_mode_chirho: ExecutionModeChirho,
    search_dir_chirho: &Path,
) -> Result<CheckSummaryChirho, DiagnosticBundleChirho> {
    let source_path_chirho = source_file_chirho.path_chirho().to_path_buf();
    let source_chirho = preprocess_cpp_chirho(source_file_chirho.contents_chirho());
    let frontend_chirho = frontend_with_search_path_chirho(
        &source_chirho,
        source_file_chirho.file_id_chirho(),
        &source_path_chirho.to_string_lossy(),
        search_dir_chirho,
        &mut SourceMapChirho::new_chirho(),
    )?;
    let module_name_chirho = frontend_chirho.module_chirho.name_chirho.full_name_chirho();
    Ok(CheckSummaryChirho {
        source_path_chirho,
        runtime_plan_chirho: RuntimePlanChirho::for_module_chirho(
            execution_mode_chirho,
            module_name_chirho.clone(),
        ),
        backend_plan_chirho: BackendPlanChirho {
            llvm_preview_chirho: compile_to_llvm_ir_stub_chirho(&module_name_chirho),
            wasm_stub_size_chirho: compile_to_wasm_stub_chirho(&module_name_chirho).len(),
        },
        module_name_chirho,
        warnings_chirho: frontend_chirho.warnings_chirho,
    })
}

/// Compile the consumer using checked companions from its active source root.
/// Dependency checking stops before Core and backend generation.
pub fn compile_source_with_search_path_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    search_dir_chirho: &Path,
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
    let source_chirho = preprocess_cpp_chirho(source_chirho);
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        source_map_chirho,
        file_name_chirho,
        &source_chirho,
    );
    let frontend_chirho = frontend_with_search_path_chirho(
        &source_chirho,
        source_file_chirho.file_id_chirho(),
        file_name_chirho,
        search_dir_chirho,
        source_map_chirho,
    )?;
    compile_backend_chirho(
        frontend_chirho.module_chirho,
        frontend_chirho.infer_result_chirho,
        std::collections::HashSet::new(),
    )
}
