// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

mod repl_chirho;

use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

use rhasky_backend_cranelift_chirho::{
    compile_core_to_object_executable_chirho, TargetConfigChirho,
};
use rhasky_backend_llvm_chirho::compile_core_to_llvm_executable_chirho;
use rhasky_backend_wasm_chirho::compile_core_to_wasm_executable_chirho;
use rhasky_driver_chirho::{
    compile_source_chirho, eval_source_with_machine_chirho,
    render_diagnostics_chirho, render_summary_chirho,
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
    emit_wasm_chirho: bool,
    emit_cranelift_chirho: bool,
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
        emit_wasm_chirho: false,
        emit_cranelift_chirho: false,
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
            "--wasm" => flags_chirho.emit_wasm_chirho = true,
            "--cranelift" => flags_chirho.emit_cranelift_chirho = true,
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

            {
                let mut sm_chirho = SourceMapChirho::new_chirho();
                let source_file_result_chirho =
                    rhasky_syntax_chirho::SourceFileChirho::from_path_with_map_chirho(
                        &mut sm_chirho,
                        &path_chirho,
                    );
                match source_file_result_chirho {
                    Ok(source_file_chirho) => {
                        match rhasky_driver_chirho::check_source_file_chirho(
                            source_file_chirho,
                            execution_mode_chirho,
                        ) {
                            Ok(check_summary_chirho) => {
                                println!("{}", render_summary_chirho(&check_summary_chirho));
                                ExitCode::SUCCESS
                            }
                            Err(diagnostic_bundle_chirho) => {
                                let use_color_chirho = atty_is_terminal_chirho();
                                eprint!(
                                    "{}",
                                    render_diagnostics_chirho(
                                        &diagnostic_bundle_chirho,
                                        &sm_chirho,
                                        use_color_chirho,
                                    )
                                );
                                ExitCode::from(1)
                            }
                        }
                    }
                    Err(error_chirho) => {
                        eprintln!("error reading `{path_chirho}`: {error_chirho}");
                        ExitCode::from(1)
                    }
                }
            }
        }
        "run" => run_command_chirho(program_name_chirho, path_chirho, &flags_chirho),
        "compile" => compile_command_chirho(program_name_chirho, path_chirho, &flags_chirho),
        "build" => build_command_chirho(program_name_chirho, path_chirho),
        "repl" => repl_chirho::repl_command_chirho(),
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
                if flags_chirho.emit_cranelift_chirho {
                    // Generate native object file via Cranelift
                    let config_chirho = TargetConfigChirho::default();
                    match compile_core_to_object_executable_chirho(
                        &result_chirho.core_chirho,
                        &config_chirho,
                    ) {
                        Ok(obj_chirho) => {
                            let obj_path_chirho = format!("{output_path_chirho}.o");
                            if let Err(e_chirho) = fs::write(&obj_path_chirho, &obj_chirho.object_bytes_chirho) {
                                eprintln!("error writing object to `{obj_path_chirho}`: {e_chirho}");
                                return ExitCode::from(1);
                            }
                            // Link with system linker
                            let linker_status_chirho = Command::new("cc")
                                .args(["-o", output_path_chirho, &obj_path_chirho])
                                .status();
                            match linker_status_chirho {
                                Ok(status_chirho) if status_chirho.success() => {
                                    println!("compiled (cranelift): {path_chirho} → {output_path_chirho}");
                                    let _ = fs::remove_file(&obj_path_chirho);
                                }
                                Ok(status_chirho) => {
                                    eprintln!("linker failed with exit code {}; object saved to {obj_path_chirho}",
                                        status_chirho.code().unwrap_or(-1));
                                    return ExitCode::from(1);
                                }
                                Err(e_chirho) => {
                                    eprintln!("could not run linker: {e_chirho}; object saved to {obj_path_chirho}");
                                    return ExitCode::from(1);
                                }
                            }
                        }
                        Err(e_chirho) => {
                            eprintln!("cranelift compilation failed: {e_chirho}");
                            return ExitCode::from(1);
                        }
                    }
                } else if flags_chirho.emit_wasm_chirho {
                    // Generate WASM binary with dict elision
                    let wasm_bytes_chirho =
                        compile_core_to_wasm_executable_chirho(&result_chirho.core_chirho);
                    if let Err(e_chirho) = fs::write(output_path_chirho, &wasm_bytes_chirho) {
                        eprintln!("error writing WASM to `{output_path_chirho}`: {e_chirho}");
                        return ExitCode::from(1);
                    }
                    println!("compiled: {path_chirho} → {output_path_chirho} ({} bytes wasm)", wasm_bytes_chirho.len());
                } else {
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
            let use_color_chirho = atty_is_terminal_chirho();
            eprint!(
                "{}",
                render_diagnostics_chirho(
                    &diagnostic_bundle_chirho,
                    &source_map_chirho,
                    use_color_chirho,
                )
            );
            ExitCode::from(1)
        }
    }
}

/// Check if stderr is a terminal (for color output decision).
fn atty_is_terminal_chirho() -> bool {
    use std::io::IsTerminal;
    std::io::stderr().is_terminal()
}

/// `rhasky build [<dir>]` — compile a multi-module Haskell project from a directory.
/// Discovers `.hs` files, resolves inter-module dependencies, and compiles
/// in topological order.
fn build_command_chirho(
    _program_name_chirho: &str,
    path_arg_chirho: Option<String>,
) -> ExitCode {
    let project_dir_chirho = path_arg_chirho
        .as_deref()
        .unwrap_or(".");
    let project_path_chirho = std::path::Path::new(project_dir_chirho);

    if !project_path_chirho.is_dir() {
        eprintln!("error: `{}` is not a directory", project_dir_chirho);
        return ExitCode::from(1);
    }

    eprintln!("Building project in {}...", project_path_chirho.display());

    let mut sm_chirho = SourceMapChirho::new_chirho();
    match rhasky_driver_chirho::compile_project_dir_chirho(project_path_chirho, &mut sm_chirho) {
        Ok(result_chirho) => {
            eprintln!(
                "Compiled {} modules in order: {}",
                result_chirho.compilation_order_chirho.len(),
                result_chirho.compilation_order_chirho.join(" → "),
            );
            for warning_chirho in &result_chirho.warnings_chirho {
                eprintln!("warning: {}", warning_chirho);
            }
            eprintln!("Build successful.");
            ExitCode::SUCCESS
        }
        Err(error_chirho) => {
            eprintln!("error: {}", error_chirho);
            ExitCode::from(1)
        }
    }
}

fn print_usage_chirho(program_name_chirho: &str) {
    eprintln!("usage: {program_name_chirho} <check|run|compile|build|repl> <path> [options]");
    eprintln!();
    eprintln!("commands:");
    eprintln!("  check    type-check a .hs file without code generation");
    eprintln!("  run      evaluate a .hs file via the STG interpreter");
    eprintln!("  compile  compile a .hs file (with -o: produce native executable or .wasm)");
    eprintln!("  build    compile a multi-module project from a directory");
    eprintln!("  repl     interactive REPL with expression evaluation");
    eprintln!();
    eprintln!("flags:");
    eprintln!("  -o, --output <path>  output native executable or .wasm file (compile only)");
    eprintln!("  --wasm               emit WebAssembly instead of native (with -o)");
    eprintln!("  --cranelift          emit native code via Cranelift (with -o)");
    eprintln!("  --dump-core          print Core IR to stderr");
    eprintln!("  --dump-stg           print STG code table to stderr (run only)");
    eprintln!("  --dump-llvm          print LLVM IR to stderr");
}
