// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-driver-chirho
//!
//! Build orchestration for the Rhasky compiler. Coordinates parsing, checking,
//! and backend lowering across execution modes.

use std::path::{Path, PathBuf};

use rhasky_ast_chirho::ModuleChirho;
use rhasky_backend_llvm_chirho::compile_to_llvm_ir_stub_chirho;
use rhasky_backend_wasm_chirho::compile_to_wasm_stub_chirho;
use rhasky_core_chirho::{desugar_module_chirho, CoreModuleChirho};
use rhasky_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho};
use rhasky_naming_chirho::resolve_chirho::resolve_module_chirho;
use rhasky_typing_chirho::infer_chirho::infer_module_chirho;
use rhasky_parser_chirho::cst_parser_chirho::ParserChirho;
use rhasky_parser_chirho::lower_chirho::lower_module_chirho;
use rhasky_parser_chirho::parse_source_file_chirho;
use rhasky_runtime_chirho::{ExecutionModeChirho, RuntimePlanChirho};
use rhasky_span_chirho::SourceMapChirho;
use rhasky_syntax_chirho::SourceFileChirho;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendPlanChirho {
    pub llvm_preview_chirho: String,
    pub wasm_stub_size_chirho: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummaryChirho {
    pub source_path_chirho: PathBuf,
    pub module_name_chirho: String,
    pub runtime_plan_chirho: RuntimePlanChirho,
    pub backend_plan_chirho: BackendPlanChirho,
}

pub fn check_source_path_chirho(
    path_chirho: impl AsRef<Path>,
    execution_mode_chirho: ExecutionModeChirho,
) -> Result<CheckSummaryChirho, DiagnosticBundleChirho> {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho =
        SourceFileChirho::from_path_with_map_chirho(&mut source_map_chirho, &path_chirho)
            .map_err(|error_chirho| {
                DiagnosticChirho::error_no_span_chirho(format!(
                    "unable to read `{}`: {error_chirho}",
                    path_chirho.as_ref().display()
                ))
            })?;

    check_source_file_chirho(source_file_chirho, execution_mode_chirho)
}

pub fn check_source_file_chirho(
    source_file_chirho: SourceFileChirho,
    execution_mode_chirho: ExecutionModeChirho,
) -> Result<CheckSummaryChirho, DiagnosticBundleChirho> {
    let parsed_module_chirho = parse_source_file_chirho(source_file_chirho.clone())?;
    let module_name_chirho = parsed_module_chirho
        .module_header_chirho
        .module_name_chirho
        .clone();
    let runtime_plan_chirho =
        RuntimePlanChirho::for_module_chirho(execution_mode_chirho, module_name_chirho.clone());
    let llvm_preview_chirho = compile_to_llvm_ir_stub_chirho(&module_name_chirho);
    let wasm_stub_size_chirho = compile_to_wasm_stub_chirho(&module_name_chirho).len();

    Ok(CheckSummaryChirho {
        source_path_chirho: source_file_chirho.path_chirho().to_path_buf(),
        module_name_chirho,
        runtime_plan_chirho,
        backend_plan_chirho: BackendPlanChirho {
            llvm_preview_chirho,
            wasm_stub_size_chirho,
        },
    })
}

/// Result of the full compilation pipeline.
#[derive(Debug, Clone)]
pub struct CompileResultChirho {
    pub module_chirho: ModuleChirho,
    pub core_chirho: CoreModuleChirho,
}

/// Run the full compiler pipeline: lex → layout → CST parse → AST lower →
/// name resolve → type infer → desugar to Core.
/// Returns the AST module and its Core IR translation.
pub fn compile_source_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
    let source_file_chirho =
        SourceFileChirho::from_source_map_chirho(source_map_chirho, file_name_chirho, source_chirho);
    let file_id_chirho = source_file_chirho.file_id_chirho();

    // Phase 1: CST parse (lex + layout + recursive-descent)
    let parser_chirho = ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();

    // Phase 2: CST → AST lowering
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);

    // Phase 3: Name resolution
    let resolve_result_chirho = resolve_module_chirho(&module_chirho);
    if resolve_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(resolve_result_chirho.diagnostics_chirho);
    }

    // Phase 4: Type inference
    let infer_result_chirho = infer_module_chirho(&module_chirho);
    if infer_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(infer_result_chirho.diagnostics_chirho);
    }

    // Phase 5: Desugar AST → Core IR
    let core_chirho = desugar_module_chirho(&module_chirho);

    Ok(CompileResultChirho {
        module_chirho,
        core_chirho,
    })
}

pub fn render_summary_chirho(check_summary_chirho: &CheckSummaryChirho) -> String {
    format!(
        "source: {}\nmodule: {}\nmode: {:?}\nincremental_session: {}\nllvm_preview: {}\nwasm_stub_size: {}",
        check_summary_chirho.source_path_chirho.display(),
        check_summary_chirho.module_name_chirho,
        check_summary_chirho.runtime_plan_chirho.execution_mode_chirho,
        check_summary_chirho.runtime_plan_chirho.incremental_session_chirho,
        check_summary_chirho
            .backend_plan_chirho
            .llvm_preview_chirho
            .trim_end(),
        check_summary_chirho.backend_plan_chirho.wasm_stub_size_chirho,
    )
}

#[cfg(test)]
mod tests_chirho {
    use super::{check_source_file_chirho, render_summary_chirho};
    use rhasky_runtime_chirho::ExecutionModeChirho;
    use rhasky_span_chirho::SourceMapChirho;
    use rhasky_syntax_chirho::SourceFileChirho;

    #[test]
    fn builds_a_check_summary_for_batch_mode_chirho() {
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            &mut source_map_chirho,
            "BatchSampleChirho.hs",
            "module BatchSampleChirho where\nvalueChirho = 1\n",
        );

        let check_summary_chirho =
            check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho)
                .expect("driver should accept a valid source file");

        assert_eq!(check_summary_chirho.module_name_chirho, "BatchSampleChirho");
        assert!(!check_summary_chirho.runtime_plan_chirho.incremental_session_chirho);
        assert!(
            render_summary_chirho(&check_summary_chirho).contains("llvm_preview: ; rhasky llvm stub")
        );
    }

    #[test]
    fn full_pipeline_parses_and_resolves_chirho() {
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(
            "module Test where\ndata Color = Red | Green\nf x = x\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        )
        .expect("full pipeline should succeed");

        assert_eq!(result_chirho.module_chirho.name_chirho.text_chirho(), "Test");
        assert!(result_chirho.module_chirho.decls_chirho.len() >= 2);
        // Core module was produced by desugaring
        assert_eq!(result_chirho.core_chirho.name_chirho, "Test");
        assert!(!result_chirho.core_chirho.bindings_chirho.is_empty());
    }

    #[test]
    fn script_mode_uses_incremental_runtime_plan_chirho() {
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            &mut source_map_chirho,
            "ScriptSampleChirho.hs",
            "mainChirho = print 42\n",
        );

        let check_summary_chirho =
            check_source_file_chirho(source_file_chirho, ExecutionModeChirho::ScriptChirho)
                .expect("script mode should accept module-less files");

        assert!(check_summary_chirho.runtime_plan_chirho.incremental_session_chirho);
        assert_eq!(check_summary_chirho.module_name_chirho, "Main");
    }
}
