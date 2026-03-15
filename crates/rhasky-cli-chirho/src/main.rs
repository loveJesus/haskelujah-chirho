// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

use rhasky_backend_llvm_chirho::compile_core_to_llvm_executable_chirho;
use rhasky_driver_chirho::{
    check_source_path_chirho, compile_source_chirho, eval_source_with_machine_chirho,
    render_summary_chirho,
};
use rhasky_runtime_chirho::ExecutionModeChirho;
use rhasky_span_chirho::SourceMapChirho;

fn main() -> ExitCode {
    main_chirho()
}

/// Parsed CLI flags.
struct FlagsChirho {
    dump_core_chirho: bool,
    dump_stg_chirho: bool,
    dump_llvm_chirho: bool,
    output_path_chirho: Option<String>,
}

fn main_chirho() -> ExitCode {
    let raw_args_chirho: Vec<String> = env::args().collect();
    let program_name_chirho = raw_args_chirho
        .first()
        .map(String::as_str)
        .unwrap_or("rhasky-cli-chirho");

    // Separate flags from positional args.
    let mut flags_chirho = FlagsChirho {
        dump_core_chirho: false,
        dump_stg_chirho: false,
        dump_llvm_chirho: false,
        output_path_chirho: None,
    };
    let mut positional_chirho: Vec<&str> = Vec::new();
    let mut skip_next_chirho = false;
    for (idx_chirho, arg_chirho) in raw_args_chirho.iter().skip(1).enumerate() {
        if skip_next_chirho {
            skip_next_chirho = false;
            continue;
        }
        match arg_chirho.as_str() {
            "--dump-core" => flags_chirho.dump_core_chirho = true,
            "--dump-stg" => flags_chirho.dump_stg_chirho = true,
            "--dump-llvm" => flags_chirho.dump_llvm_chirho = true,
            "-o" | "--output" => {
                if let Some(next_chirho) = raw_args_chirho.get(idx_chirho + 2) {
                    flags_chirho.output_path_chirho = Some(next_chirho.clone());
                    skip_next_chirho = true;
                } else {
                    eprintln!("-o requires an output path argument");
                    return ExitCode::from(2);
                }
            }
            _ => positional_chirho.push(arg_chirho),
        }
    }

    let command_chirho = positional_chirho.first().copied();
    let path_chirho = positional_chirho.get(1).copied().map(String::from);

    let Some(command_chirho) = command_chirho else {
        print_usage_chirho(program_name_chirho);
        return ExitCode::from(2);
    };

    match command_chirho {
        "check" | "plan" | "script" => {
            let Some(path_chirho) = path_chirho else {
                eprintln!("missing path for `{command_chirho}`");
                print_usage_chirho(program_name_chirho);
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
        "run" => run_command_chirho(program_name_chirho, path_chirho, &flags_chirho),
        "compile" => compile_command_chirho(program_name_chirho, path_chirho, &flags_chirho),
        "repl" => {
            println!(
                "repl mode is not implemented yet, but the runtime plan reserves {:?} for it",
                ExecutionModeChirho::ReplChirho
            );
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("unknown command `{command_chirho}`");
            print_usage_chirho(program_name_chirho);
            ExitCode::from(2)
        }
    }
}

/// `rhasky run <file.hs>` — evaluate a Haskell program through the STG machine
/// and print any IO output it produces.
fn run_command_chirho(
    program_name_chirho: &str,
    path_arg_chirho: Option<String>,
    flags_chirho: &FlagsChirho,
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

    // If any dump flags are set, compile first to get Core/LLVM IR.
    if flags_chirho.dump_core_chirho || flags_chirho.dump_llvm_chirho {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        match compile_source_chirho(&source_text_chirho, &mut sm_chirho, file_name_chirho) {
            Ok(result_chirho) => {
                if flags_chirho.dump_core_chirho {
                    eprintln!("=== Core IR ===");
                    eprintln!(
                        "{}",
                        rhasky_core_chirho::pretty_module_chirho(&result_chirho.core_chirho)
                    );
                }
                if flags_chirho.dump_llvm_chirho {
                    eprintln!("=== LLVM IR ===");
                    eprintln!("{}", result_chirho.llvm_ir_chirho);
                }
            }
            Err(e_chirho) => {
                eprintln!("compilation error (dump phase): {e_chirho}");
            }
        }
    }

    let mut source_map_chirho = SourceMapChirho::new_chirho();

    match eval_source_with_machine_chirho(
        &source_text_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        None,
    ) {
        Ok((_value_chirho, machine_chirho)) => {
            if flags_chirho.dump_stg_chirho {
                eprintln!("=== STG Code Table ({} entries) ===", machine_chirho.code_table_chirho.len());
                for (idx_chirho, code_chirho) in machine_chirho.code_table_chirho.iter().enumerate() {
                    eprintln!("  [{idx_chirho}] {code_chirho:?}");
                }
            }
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
    flags_chirho: &FlagsChirho,
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
            if let Some(ref output_path_chirho) = flags_chirho.output_path_chirho {
                // Generate executable LLVM IR with C main() entry point
                let exec_ir_chirho =
                    compile_core_to_llvm_executable_chirho(&result_chirho.core_chirho);

                let ll_path_chirho = format!("{output_path_chirho}.ll");
                if let Err(e_chirho) = fs::write(&ll_path_chirho, &exec_ir_chirho) {
                    eprintln!("error writing LLVM IR to `{ll_path_chirho}`: {e_chirho}");
                    return ExitCode::from(1);
                }

                if flags_chirho.dump_llvm_chirho {
                    eprintln!("=== LLVM IR ===");
                    eprintln!("{exec_ir_chirho}");
                }

                // Invoke clang to compile .ll → native executable
                let clang_status_chirho = Command::new("clang")
                    .args(["-O2", "-o", output_path_chirho, &ll_path_chirho])
                    .status();

                match clang_status_chirho {
                    Ok(status_chirho) if status_chirho.success() => {
                        println!("compiled: {path_chirho} → {output_path_chirho}");
                        // Clean up the intermediate .ll file
                        let _ = fs::remove_file(&ll_path_chirho);
                    }
                    Ok(status_chirho) => {
                        eprintln!(
                            "clang failed with exit code {}; LLVM IR saved to {ll_path_chirho}",
                            status_chirho.code().unwrap_or(-1)
                        );
                        return ExitCode::from(1);
                    }
                    Err(e_chirho) => {
                        eprintln!(
                            "could not run clang: {e_chirho}; LLVM IR saved to {ll_path_chirho}"
                        );
                        return ExitCode::from(1);
                    }
                }
            } else {
                println!(
                    "compiled: {} (module: {})",
                    path_chirho,
                    result_chirho
                        .module_chirho
                        .name_chirho
                        .text_chirho(),
                );
                println!(
                    "  core bindings: {}",
                    result_chirho.core_chirho.bindings_chirho.len()
                );
                println!("  llvm ir bytes: {}", result_chirho.llvm_ir_chirho.len());
                println!("  wasm bytes:    {}", result_chirho.wasm_bytes_chirho.len());

                if flags_chirho.dump_core_chirho {
                    println!("\n=== Core IR ===");
                    println!(
                        "{}",
                        rhasky_core_chirho::pretty_module_chirho(&result_chirho.core_chirho)
                    );
                }
                if flags_chirho.dump_llvm_chirho {
                    println!("\n=== LLVM IR ===");
                    println!("{}", result_chirho.llvm_ir_chirho);
                }
            }
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
    eprintln!("usage: {program_name_chirho} <check|run|compile|repl> <path> [options]");
    eprintln!();
    eprintln!("commands:");
    eprintln!("  check    type-check a .hs file without code generation");
    eprintln!("  run      evaluate a .hs file via the STG interpreter");
    eprintln!("  compile  compile a .hs file (with -o: produce native executable via clang)");
    eprintln!("  repl     interactive session (not yet implemented)");
    eprintln!();
    eprintln!("flags:");
    eprintln!("  -o, --output <path>  output native executable (compile only; requires clang)");
    eprintln!("  --dump-core          print Core IR to stderr");
    eprintln!("  --dump-stg           print STG code table to stderr (run only)");
    eprintln!("  --dump-llvm          print LLVM IR to stderr");
}
