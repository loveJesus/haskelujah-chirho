// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-driver-chirho
//!
//! Build orchestration for the Rhasky compiler. Coordinates parsing, checking,
//! and backend lowering across execution modes.

use std::path::{Path, PathBuf};

use rhasky_backend_llvm_chirho::compile_to_llvm_ir_stub_chirho;
use rhasky_backend_wasm_chirho::compile_to_wasm_stub_chirho;
use rhasky_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho};
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
