// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use rhasky_driver_chirho::{
    check_source_path_chirho, compile_source_chirho, eval_source_with_machine_chirho,
    render_summary_chirho,
};
use rhasky_runtime_chirho::ExecutionModeChirho;
use rhasky_span_chirho::SourceMapChirho;

fn main() -> ExitCode {
    main_chirho()
}

fn main_chirho() -> ExitCode {
    let mut arguments_chirho = env::args();
    let program_name_chirho = arguments_chirho
        .next()
        .unwrap_or_else(|| "rhasky-cli-chirho".to_owned());
    let command_chirho = arguments_chirho.next();
    let path_chirho = arguments_chirho.next();

    let Some(command_chirho) = command_chirho else {
        print_usage_chirho(&program_name_chirho);
        return ExitCode::from(2);
    };

    match command_chirho.as_str() {
        "check" | "plan" | "script" => {
            let Some(path_chirho) = path_chirho else {
                eprintln!("missing path for `{command_chirho}`");
                print_usage_chirho(&program_name_chirho);
                return ExitCode::from(2);
            };

            let execution_mode_chirho = if command_chirho == "script" {
                ExecutionModeChirho::ScriptChirho
            } else {
                ExecutionModeChirho::BatchChirho
            };

            match check_source_path_chirho(&path_chirho, execution_mode_chirho) {
                Ok(check_summary_chirho) => {
                    println!("{}", render_summary_chirho(&check_summary_chirho));
                    ExitCode::SUCCESS
                }
                Err(diagnostic_bundle_chirho) => {
                    eprintln!("{diagnostic_bundle_chirho}");
                    ExitCode::from(1)
                }
            }
        }
        "run" => run_command_chirho(&program_name_chirho, path_chirho),
        "compile" => compile_command_chirho(&program_name_chirho, path_chirho),
        "repl" => {
            println!(
                "repl mode is not implemented yet, but the runtime plan reserves {:?} for it",
                ExecutionModeChirho::ReplChirho
            );
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("unknown command `{command_chirho}`");
            print_usage_chirho(&program_name_chirho);
            ExitCode::from(2)
        }
    }
}

/// `rhasky run <file.hs>` — evaluate a Haskell program through the STG machine
/// and print any IO output it produces.
fn run_command_chirho(
    program_name_chirho: &str,
    path_arg_chirho: Option<String>,
) -> ExitCode {
    let Some(path_chirho) = path_arg_chirho else {
        eprintln!("missing path for `run`");
        print_usage_chirho(program_name_chirho);
        return ExitCode::from(2);
    };

    let source_text_chirho = match fs::read_to_string(&path_chirho) {
        Ok(text_chirho) => text_chirho,
        Err(error_chirho) => {
            eprintln!("error reading `{path_chirho}`: {error_chirho}");
            return ExitCode::from(1);
        }
    };

    let file_name_chirho = Path::new(&path_chirho)
        .file_name()
        .and_then(|os_str_chirho| os_str_chirho.to_str())
        .unwrap_or(&path_chirho);

    let mut source_map_chirho = SourceMapChirho::new_chirho();

    match eval_source_with_machine_chirho(
        &source_text_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        None,
    ) {
        Ok((_value_chirho, machine_chirho)) => {
            if !machine_chirho.io_output_chirho.is_empty() {
                print!("{}", machine_chirho.io_output_chirho);
            }
            ExitCode::SUCCESS
        }
        Err(error_msg_chirho) => {
            eprintln!("runtime error: {error_msg_chirho}");
            ExitCode::from(1)
        }
    }
}

/// `rhasky compile <file.hs>` — run the full compilation pipeline and report
/// success or any diagnostics.
fn compile_command_chirho(
    program_name_chirho: &str,
    path_arg_chirho: Option<String>,
) -> ExitCode {
    let Some(path_chirho) = path_arg_chirho else {
        eprintln!("missing path for `compile`");
        print_usage_chirho(program_name_chirho);
        return ExitCode::from(2);
    };

    let source_text_chirho = match fs::read_to_string(&path_chirho) {
        Ok(text_chirho) => text_chirho,
        Err(error_chirho) => {
            eprintln!("error reading `{path_chirho}`: {error_chirho}");
            return ExitCode::from(1);
        }
    };

    let file_name_chirho = Path::new(&path_chirho)
        .file_name()
        .and_then(|os_str_chirho| os_str_chirho.to_str())
        .unwrap_or(&path_chirho);

    let mut source_map_chirho = SourceMapChirho::new_chirho();

    match compile_source_chirho(&source_text_chirho, &mut source_map_chirho, file_name_chirho) {
        Ok(result_chirho) => {
            println!(
                "compiled: {} (module: {})",
                path_chirho,
                result_chirho
                    .module_chirho
                    .name_chirho
                    .text_chirho(),
            );
            println!("  core bindings: {}", result_chirho.core_chirho.bindings_chirho.len());
            println!("  llvm ir bytes: {}", result_chirho.llvm_ir_chirho.len());
            println!("  wasm bytes:    {}", result_chirho.wasm_bytes_chirho.len());
            ExitCode::SUCCESS
        }
        Err(diagnostic_bundle_chirho) => {
            eprintln!("compilation failed:");
            eprintln!("{diagnostic_bundle_chirho}");
            ExitCode::from(1)
        }
    }
}

fn print_usage_chirho(program_name_chirho: &str) {
    eprintln!("usage: {program_name_chirho} <check|plan|script|run|compile|repl> [path]");
}
