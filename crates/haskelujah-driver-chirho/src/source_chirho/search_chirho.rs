// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source roots use checked reachable companions, shared by checking and compiling.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use crate::*;

fn frontend_with_search_path_chirho(
    source_chirho: &str,
    file_id_chirho: haskelujah_span_chirho::FileIdChirho,
    file_name_chirho: &str,
    search_dir_chirho: &Path,
    source_map_chirho: &mut SourceMapChirho,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    super::graph_chirho::check_source_graph_chirho(
        source_chirho,
        file_id_chirho,
        file_name_chirho,
        search_dir_chirho,
        source_map_chirho,
    )
    .map_err(|error_chirho| match error_chirho {
        super::graph_chirho::SourceGraphErrorChirho::RootChirho(diagnostics_chirho) => {
            diagnostics_chirho
        }
        super::graph_chirho::SourceGraphErrorChirho::DependencyChirho(message_chirho) => {
            DiagnosticChirho::error_no_span_chirho(message_chirho)
                .with_code_chirho(haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(100))
                .into()
        }
    })
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
