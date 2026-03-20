// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

mod repl_chirho;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use haskelujah_backend_cranelift_chirho::{
    TargetConfigChirho, compile_core_to_object_executable_chirho,
};
use haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho;
use haskelujah_backend_wasm_chirho::compile_core_to_wasm_executable_chirho;
use haskelujah_driver_chirho::{
    compile_source_chirho, eval_source_with_machine_chirho, render_diagnostics_chirho,
    render_summary_chirho,
};
use haskelujah_runtime_chirho::ExecutionModeChirho;
use haskelujah_span_chirho::SourceMapChirho;

// Keep CLI linking on the same unoptimized path the LLVM round-trip tests verify.
const CLANG_OPT_LEVEL_CHIRHO: &str = "-O0";

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
    emit_llvm_chirho: bool,
    emit_cranelift_chirho: bool,
}

fn main_chirho() -> ExitCode {
    let raw_args_chirho: Vec<String> = env::args().collect();
    let program_name_chirho = raw_args_chirho
        .first()
        .map(String::as_str)
        .unwrap_or("haskelujah-cli-chirho");

    // Separate flags from positional args.
    let mut flags_chirho = FlagsChirho {
        dump_core_chirho: false,
        dump_stg_chirho: false,
        dump_llvm_chirho: false,
        output_path_chirho: None,
        emit_wasm_chirho: false,
        emit_llvm_chirho: false,
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
            "--llvm" => flags_chirho.emit_llvm_chirho = true,
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

    // Handle --version and --help as commands
    if command_chirho == "--version" || command_chirho == "-V" {
        eprintln!("haskelujah-chirho 0.1.0 (GHC compat: 861/938, 91.8%)");
        return ExitCode::SUCCESS;
    }
    if command_chirho == "--help" || command_chirho == "-h" || command_chirho == "help" {
        print_usage_chirho(program_name_chirho);
        return ExitCode::SUCCESS;
    }

    if flags_chirho.emit_llvm_chirho && flags_chirho.emit_cranelift_chirho {
        eprintln!("cannot combine --llvm and --cranelift");
        return ExitCode::from(2);
    }

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
                    haskelujah_syntax_chirho::SourceFileChirho::from_path_with_map_chirho(
                        &mut sm_chirho,
                        &path_chirho,
                    );
                match source_file_result_chirho {
                    Ok(source_file_chirho) => {
                        match haskelujah_driver_chirho::check_source_file_chirho(
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
        "build" => build_command_chirho(program_name_chirho, path_chirho, &flags_chirho),
        "build-run" => {
            // Build then run the first executable
            let result_chirho =
                build_command_chirho(program_name_chirho, path_chirho.clone(), &flags_chirho);
            if result_chirho != ExitCode::SUCCESS {
                return result_chirho;
            }
            // Find and run the executable
            let dir_chirho = path_chirho.as_deref().unwrap_or(".");
            let build_dir_chirho = Path::new(dir_chirho).join("dist-chirho").join("build");
            if let Ok(entries_chirho) = fs::read_dir(&build_dir_chirho) {
                for entry_chirho in entries_chirho.flatten() {
                    let p_chirho = entry_chirho.path();
                    if p_chirho.is_file() && !p_chirho.extension().is_some_and(|e| e == "ll") {
                        eprintln!("");
                        let status_chirho = Command::new(&p_chirho).status();
                        return match status_chirho {
                            Ok(s_chirho) if s_chirho.success() => ExitCode::SUCCESS,
                            Ok(s_chirho) => ExitCode::from(s_chirho.code().unwrap_or(1) as u8),
                            Err(e_chirho) => {
                                eprintln!("error running {}: {}", p_chirho.display(), e_chirho);
                                ExitCode::from(1)
                            }
                        };
                    }
                }
            }
            eprintln!("No executable found in {}", build_dir_chirho.display());
            ExitCode::from(1)
        }
        "install" => install_command_chirho(program_name_chirho, &positional_chirho),
        "repl" => repl_chirho::repl_command_chirho(),
        "init" => init_command_chirho(path_chirho),
        "clean" => clean_command_chirho(path_chirho),
        _ => {
            eprintln!("unknown command `{command_chirho}`");
            print_usage_chirho(program_name_chirho);
            ExitCode::from(2)
        }
    }
}

/// `haskelujah run <file.hs>` — compile via LLVM and execute the resulting
/// native binary. Falls back to the STG interpreter if LLVM compilation
/// or linking fails.
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
                        haskelujah_core_chirho::pretty_module_chirho(&result_chirho.core_chirho)
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

    // Try LLVM compile-and-run first for programs with main :: IO ()
    {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let search_dir_chirho = Path::new(&path_chirho).parent().unwrap_or(Path::new("."));
        if let Ok(result_chirho) = haskelujah_driver_chirho::compile_source_with_search_path_chirho(
            &source_text_chirho,
            &mut sm_chirho,
            file_name_chirho,
            search_dir_chirho,
        ) {
            let tmp_dir_chirho = std::env::temp_dir().join("haskelujah-run-chirho");
            let _ = fs::create_dir_all(&tmp_dir_chirho);
            let exe_path_chirho = tmp_dir_chirho.join("a.out");
            // Suppress clang stderr for the try-LLVM path
            let link_result_chirho = {
                let ll_path_chirho = exe_path_chirho.with_extension("ll");
                let _ = fs::write(&ll_path_chirho, &result_chirho.llvm_ir_chirho);
                let link_result_chirho =
                    link_llvm_file_chirho(&ll_path_chirho, &exe_path_chirho, "-O2", true);
                let _ = fs::remove_file(&ll_path_chirho);
                link_result_chirho.map_err(|_| ())
            };
            if let Ok(()) = link_result_chirho {
                let status_chirho = Command::new(&exe_path_chirho).status();
                let _ = fs::remove_file(&exe_path_chirho);
                match status_chirho {
                    Ok(s_chirho) => {
                        return if s_chirho.success() {
                            ExitCode::SUCCESS
                        } else {
                            ExitCode::from(s_chirho.code().unwrap_or(1) as u8)
                        };
                    }
                    Err(_) => {} // fall through to STG interpreter
                }
            }
        }
    }

    // Fallback: STG interpreter
    let mut source_map_chirho = SourceMapChirho::new_chirho();

    match eval_source_with_machine_chirho(
        &source_text_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        None,
    ) {
        Ok((_value_chirho, machine_chirho)) => {
            if flags_chirho.dump_stg_chirho {
                eprintln!(
                    "=== STG Code Table ({} entries) ===",
                    machine_chirho.code_table_chirho.len()
                );
                for (idx_chirho, code_chirho) in machine_chirho.code_table_chirho.iter().enumerate()
                {
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

/// `haskelujah compile <file.hs>` — run the full compilation pipeline and report
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

    // Use search path so sibling modules can be found
    let search_dir_chirho = Path::new(&path_chirho).parent().unwrap_or(Path::new("."));
    match haskelujah_driver_chirho::compile_source_with_search_path_chirho(
        &source_text_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        search_dir_chirho,
    ) {
        Ok(result_chirho) => {
            if let Some(ref output_path_chirho) = flags_chirho.output_path_chirho {
                if flags_chirho.emit_cranelift_chirho {
                    match link_cranelift_executable_from_core_chirho(
                        &result_chirho.core_chirho,
                        Path::new(output_path_chirho),
                    ) {
                        Ok(()) => {
                            println!(
                                "compiled (cranelift): {path_chirho} → {output_path_chirho}"
                            );
                        }
                        Err(error_chirho) => {
                            eprintln!("{error_chirho}");
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
                    println!(
                        "compiled: {path_chirho} → {output_path_chirho} ({} bytes wasm)",
                        wasm_bytes_chirho.len()
                    );
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
                    match link_llvm_file_chirho(
                        Path::new(&ll_path_chirho),
                        Path::new(output_path_chirho),
                        CLANG_OPT_LEVEL_CHIRHO,
                        false,
                    ) {
                        Ok(()) => {
                            println!("compiled: {path_chirho} → {output_path_chirho}");
                            // Clean up the intermediate .ll file
                            let _ = fs::remove_file(&ll_path_chirho);
                        }
                        Err(error_chirho) => {
                            eprintln!("{error_chirho}; LLVM IR saved to {ll_path_chirho}");
                            return ExitCode::from(1);
                        }
                    }
                }
            } else {
                println!(
                    "compiled: {} (module: {})",
                    path_chirho,
                    result_chirho.module_chirho.name_chirho.text_chirho(),
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
                        haskelujah_core_chirho::pretty_module_chirho(&result_chirho.core_chirho)
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

/// Print a status message with optional green coloring.
fn status_chirho(label_chirho: &str, msg_chirho: &str) {
    if atty_is_terminal_chirho() {
        eprintln!("\x1b[1;32m{:>12}\x1b[0m {}", label_chirho, msg_chirho);
    } else {
        eprintln!("{:>12} {}", label_chirho, msg_chirho);
    }
}

/// Print an error message with optional red coloring.
#[allow(dead_code)]
fn error_msg_chirho(msg_chirho: &str) {
    if atty_is_terminal_chirho() {
        eprintln!("\x1b[1;31merror\x1b[0m: {}", msg_chirho);
    } else {
        eprintln!("error: {}", msg_chirho);
    }
}

/// `haskelujah build [<dir>]` — compile a multi-module Haskell project from a directory.
///
/// If a `.cabal` file is found, uses Cabal-based compilation (parses `.cabal`,
/// resolves dependencies, discovers modules from `hs-source-dirs`).
/// Otherwise, discovers `.hs` files recursively and compiles in dependency order.
fn build_command_chirho(
    _program_name_chirho: &str,
    path_arg_chirho: Option<String>,
    flags_chirho: &FlagsChirho,
) -> ExitCode {
    let project_dir_chirho = path_arg_chirho.as_deref().unwrap_or(".");
    let project_path_chirho = std::path::Path::new(project_dir_chirho);

    if !project_path_chirho.is_dir() {
        eprintln!("error: `{}` is not a directory", project_dir_chirho);
        return ExitCode::from(1);
    }
    if flags_chirho.emit_wasm_chirho {
        eprintln!("error: `build` does not support `--wasm` yet");
        return ExitCode::from(2);
    }

    let build_start_chirho = std::time::Instant::now();

    // Check for .cabal file
    let cabal_file_chirho = find_cabal_file_chirho(project_path_chirho);

    if let Some(cabal_path_chirho) = cabal_file_chirho {
        eprintln!("Found Cabal file: {}", cabal_path_chirho.display());
        let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();
        let build_dir_chirho = project_path_chirho.join("dist-chirho").join("build");
        if let Err(error_chirho) = fs::create_dir_all(&build_dir_chirho) {
            eprintln!(
                "error creating build directory {}: {}",
                build_dir_chirho.display(),
                error_chirho,
            );
            return ExitCode::from(1);
        }

        match haskelujah_driver_chirho::build_cabal_project_chirho(
            &cabal_path_chirho,
            &index_chirho,
        ) {
            Ok(build_result_chirho) => {
                if build_result_chirho.executables_chirho.is_empty() {
                    match haskelujah_driver_chirho::compile_cabal_project_chirho(
                        &cabal_path_chirho,
                        &index_chirho,
                    ) {
                        Ok(result_chirho) => {
                            eprintln!(
                                "Compiled {} modules from package '{}'",
                                result_chirho.module_results_chirho.len(),
                                result_chirho.package_chirho.name_chirho,
                            );
                            for warning_chirho in &result_chirho.warnings_chirho {
                                eprintln!("warning: {}", warning_chirho);
                            }
                            status_chirho(
                                "Finished",
                                &format!(
                                    "build (no executables) in {:.2}s",
                                    build_start_chirho.elapsed().as_secs_f64()
                                ),
                            );
                            ExitCode::SUCCESS
                        }
                        Err(error_chirho) => {
                            eprintln!("error: {}", error_chirho);
                            ExitCode::from(1)
                        }
                    }
                } else {
                    for executable_chirho in &build_result_chirho.executables_chirho {
                        let output_path_chirho =
                            build_dir_chirho.join(&executable_chirho.name_chirho);
                        let backend_used_chirho = if flags_chirho.emit_llvm_chirho {
                            if let Err(error_chirho) = link_llvm_executable_chirho(
                                &executable_chirho.llvm_ir_chirho,
                                &output_path_chirho,
                            ) {
                                eprintln!(
                                    "error building executable {}: {}",
                                    executable_chirho.name_chirho, error_chirho
                                );
                                return ExitCode::from(1);
                            }
                            "llvm"
                        } else {
                            match link_cranelift_executable_from_core_chirho(
                                &executable_chirho.core_chirho,
                                &output_path_chirho,
                            ) {
                                Ok(()) => "cranelift",
                                Err(cranelift_error_chirho) => {
                                    eprintln!(
                                        "warning: Cranelift build failed for {}: {}; falling back to LLVM",
                                        executable_chirho.name_chirho, cranelift_error_chirho
                                    );
                                    if let Err(error_chirho) = link_llvm_executable_chirho(
                                        &executable_chirho.llvm_ir_chirho,
                                        &output_path_chirho,
                                    ) {
                                        eprintln!(
                                            "error building executable {}: {}",
                                            executable_chirho.name_chirho, error_chirho
                                        );
                                        return ExitCode::from(1);
                                    }
                                    "llvm-fallback"
                                }
                            }
                        };
                        eprintln!(
                            "Built executable {} → {} ({backend_used_chirho})",
                            executable_chirho.name_chirho,
                            output_path_chirho.display(),
                        );
                        for warning_chirho in &executable_chirho.warnings_chirho {
                            eprintln!("warning: {}", warning_chirho);
                        }
                    }
                    status_chirho(
                        "Finished",
                        &format!(
                            "build in {:.2}s",
                            build_start_chirho.elapsed().as_secs_f64()
                        ),
                    );
                    ExitCode::SUCCESS
                }
            }
            Err(error_chirho) => {
                eprintln!("error: {}", error_chirho);
                ExitCode::from(1)
            }
        }
    } else {
        eprintln!("Building project in {}...", project_path_chirho.display());
        let mut sm_chirho = SourceMapChirho::new_chirho();
        match haskelujah_driver_chirho::compile_project_dir_chirho(
            project_path_chirho,
            &mut sm_chirho,
        ) {
            Ok(result_chirho) => {
                eprintln!(
                    "Compiled {} modules in order: {}",
                    result_chirho.compilation_order_chirho.len(),
                    result_chirho.compilation_order_chirho.join(" → "),
                );
                for warning_chirho in &result_chirho.warnings_chirho {
                    eprintln!("warning: {}", warning_chirho);
                }
                status_chirho(
                    "Finished",
                    &format!(
                        "build in {:.2}s",
                        build_start_chirho.elapsed().as_secs_f64()
                    ),
                );
                ExitCode::SUCCESS
            }
            Err(error_chirho) => {
                eprintln!("error: {}", error_chirho);
                ExitCode::from(1)
            }
        }
    }
}

fn link_llvm_executable_chirho(
    llvm_ir_chirho: &str,
    output_path_chirho: &Path,
) -> Result<(), String> {
    let ll_path_chirho = output_path_chirho.with_extension("ll");
    fs::write(&ll_path_chirho, llvm_ir_chirho)
        .map_err(|e_chirho| format!("cannot write {}: {}", ll_path_chirho.display(), e_chirho))?;

    let link_result_chirho = link_llvm_file_chirho(
        &ll_path_chirho,
        output_path_chirho,
        CLANG_OPT_LEVEL_CHIRHO,
        false,
    );
    if link_result_chirho.is_ok() {
        let _ = fs::remove_file(&ll_path_chirho);
    }
    link_result_chirho
}

fn link_llvm_file_chirho(
    llvm_path_chirho: &Path,
    output_path_chirho: &Path,
    opt_level_chirho: &str,
    suppress_stderr_chirho: bool,
) -> Result<(), String> {
    let rts_lib_dir_chirho = ensure_rts_staticlib_chirho()?;
    let output_path_str_chirho = output_path_chirho
        .to_str()
        .ok_or_else(|| format!("non-utf8 output path: {}", output_path_chirho.display()))?;
    let llvm_path_str_chirho = llvm_path_chirho
        .to_str()
        .ok_or_else(|| format!("non-utf8 llvm path: {}", llvm_path_chirho.display()))?;

    let mut clang_command_chirho = Command::new("clang");
    clang_command_chirho.args([
        opt_level_chirho,
        "-o",
        output_path_str_chirho,
        llvm_path_str_chirho,
    ]);
    append_rts_link_args_chirho(&mut clang_command_chirho, &rts_lib_dir_chirho);
    if suppress_stderr_chirho {
        clang_command_chirho.stderr(Stdio::null());
    }

    let clang_status_chirho = clang_command_chirho
        .status()
        .map_err(|e_chirho| format!("could not run clang: {}", e_chirho))?;

    if clang_status_chirho.success() {
        Ok(())
    } else {
        Err(format!(
            "clang failed with exit code {}; LLVM IR saved to {}",
            clang_status_chirho.code().unwrap_or(-1),
            llvm_path_chirho.display(),
        ))
    }
}

fn link_cranelift_executable_from_core_chirho(
    core_chirho: &haskelujah_core_chirho::CoreModuleChirho,
    output_path_chirho: &Path,
) -> Result<(), String> {
    let config_chirho = TargetConfigChirho::default();
    let obj_chirho = compile_core_to_object_executable_chirho(core_chirho, &config_chirho)
        .map_err(|error_chirho| format!("cranelift compilation failed: {error_chirho}"))?;
    link_cranelift_object_file_chirho(&obj_chirho.object_bytes_chirho, output_path_chirho)
}

fn link_cranelift_object_file_chirho(
    object_bytes_chirho: &[u8],
    output_path_chirho: &Path,
) -> Result<(), String> {
    let obj_path_chirho = output_path_chirho.with_extension("o");
    fs::write(&obj_path_chirho, object_bytes_chirho)
        .map_err(|error_chirho| format!("cannot write {}: {}", obj_path_chirho.display(), error_chirho))?;
    let rts_lib_dir_chirho = ensure_rts_staticlib_chirho()?;
    let output_path_str_chirho = output_path_chirho
        .to_str()
        .ok_or_else(|| format!("non-utf8 output path: {}", output_path_chirho.display()))?;
    let obj_path_str_chirho = obj_path_chirho
        .to_str()
        .ok_or_else(|| format!("non-utf8 object path: {}", obj_path_chirho.display()))?;

    let mut linker_command_chirho = Command::new("cc");
    linker_command_chirho.args([
        "-o",
        output_path_str_chirho,
        obj_path_str_chirho,
        "-Wl,-no_fixup_chains",
        // 32MB stack for deeper recursion (non-TCO list operations)
        "-Wl,-stack_size,0x2000000",
    ]);
    append_rts_link_args_chirho(&mut linker_command_chirho, &rts_lib_dir_chirho);
    let linker_status_chirho = linker_command_chirho.status().map_err(|error_chirho| {
        format!(
            "could not run linker: {error_chirho}; object saved to {}",
            obj_path_chirho.display()
        )
    })?;
    if linker_status_chirho.success() {
        let _ = fs::remove_file(&obj_path_chirho);
        Ok(())
    } else {
        Err(format!(
            "linker failed with exit code {}; object saved to {}",
            linker_status_chirho.code().unwrap_or(-1),
            obj_path_chirho.display()
        ))
    }
}

fn append_rts_link_args_chirho(clang_command_chirho: &mut Command, rts_lib_dir_chirho: &Path) {
    clang_command_chirho
        .arg("-L")
        .arg(rts_lib_dir_chirho)
        .arg("-lhaskelujah_rts_chirho");
}

fn ensure_rts_staticlib_chirho() -> Result<PathBuf, String> {
    let workspace_root_chirho = workspace_root_chirho();
    let cargo_status_chirho = Command::new("cargo")
        .current_dir(&workspace_root_chirho)
        .args(["build", "-p", "haskelujah-rts-chirho", "--quiet"])
        .status()
        .map_err(|e_chirho| format!("could not build haskelujah-rts-chirho: {}", e_chirho))?;
    if !cargo_status_chirho.success() {
        return Err(format!(
            "cargo build -p haskelujah-rts-chirho failed with exit code {}",
            cargo_status_chirho.code().unwrap_or(-1)
        ));
    }
    Ok(workspace_root_chirho.join("target").join("debug"))
}

fn workspace_root_chirho() -> PathBuf {
    let crate_dir_chirho = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_dir_chirho
        .parent()
        .and_then(Path::parent)
        .expect("cli crate should live under workspace/crates")
        .to_path_buf()
}

/// `haskelujah init [name]` — create a new Haskell project with .cabal scaffold.
/// `haskelujah clean [dir]` — remove build artifacts (dist-chirho/).
fn clean_command_chirho(path_arg_chirho: Option<String>) -> ExitCode {
    let project_dir_chirho = path_arg_chirho.as_deref().unwrap_or(".");
    let dist_dir_chirho = Path::new(project_dir_chirho).join("dist-chirho");
    if dist_dir_chirho.exists() {
        match fs::remove_dir_all(&dist_dir_chirho) {
            Ok(()) => {
                eprintln!("Cleaned {}", dist_dir_chirho.display());
                ExitCode::SUCCESS
            }
            Err(e_chirho) => {
                eprintln!("error cleaning {}: {}", dist_dir_chirho.display(), e_chirho);
                ExitCode::from(1)
            }
        }
    } else {
        eprintln!("Nothing to clean.");
        ExitCode::SUCCESS
    }
}

fn init_command_chirho(name_arg_chirho: Option<String>) -> ExitCode {
    let project_path_chirho = name_arg_chirho.unwrap_or_else(|| "my-project".to_string());
    let project_dir_chirho = Path::new(&project_path_chirho);
    // Use just the directory name (not the full path) as the cabal project name
    let project_name_chirho = project_dir_chirho
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my-project")
        .to_string();

    if project_dir_chirho.exists() {
        eprintln!(
            "error: directory `{}` already exists",
            project_dir_chirho.display()
        );
        return ExitCode::from(1);
    }

    if let Err(e_chirho) = fs::create_dir_all(project_dir_chirho) {
        eprintln!("error creating directory: {}", e_chirho);
        return ExitCode::from(1);
    }

    let cabal_content_chirho = format!(
        "cabal-version: 2.4\nname: {name}\nversion: 0.1.0.0\n\nexecutable {name}\n  main-is: Main.hs\n  build-depends: base\n  default-language: Haskell2010\n",
        name = project_name_chirho
    );

    let main_content_chirho = "-- For God so loved the world that he gave his only begotten Son, that whoever\n\
         -- believes in him should not perish but have eternal life. — John 3:16\n\
         \n\
         module Main where\n\
         \n\
         main :: IO ()\n\
         main = putStrLn \"Hello from Haskelujah Chirho!\"\n";

    let cabal_path_chirho = project_dir_chirho.join(format!("{}.cabal", project_name_chirho));
    let main_path_chirho = project_dir_chirho.join("Main.hs");

    if let Err(e_chirho) = fs::write(&cabal_path_chirho, cabal_content_chirho) {
        eprintln!(
            "error writing {}: {}",
            cabal_path_chirho.display(),
            e_chirho
        );
        return ExitCode::from(1);
    }
    if let Err(e_chirho) = fs::write(&main_path_chirho, main_content_chirho) {
        eprintln!("error writing {}: {}", main_path_chirho.display(), e_chirho);
        return ExitCode::from(1);
    }

    eprintln!("Created project `{}`", project_name_chirho);
    eprintln!("  {}", cabal_path_chirho.display());
    eprintln!("  {}", main_path_chirho.display());
    eprintln!("");
    eprintln!("To build:  haskelujah build {}", project_name_chirho);
    eprintln!("To run:    haskelujah run {}/Main.hs", project_name_chirho);
    ExitCode::SUCCESS
}

/// Find a `.cabal` file in the given directory (first match).
fn find_cabal_file_chirho(dir_chirho: &std::path::Path) -> Option<std::path::PathBuf> {
    let entries_chirho = std::fs::read_dir(dir_chirho).ok()?;
    for entry_chirho in entries_chirho.flatten() {
        let path_chirho = entry_chirho.path();
        if path_chirho
            .extension()
            .map_or(false, |ext_chirho| ext_chirho == "cabal")
        {
            return Some(path_chirho);
        }
    }
    None
}

/// `haskelujah install <package-name> <version>` — fetch a package from Hackage,
/// compile it, and register it in the local package database.
fn install_command_chirho(program_name_chirho: &str, positional_chirho: &[&str]) -> ExitCode {
    // Expect: install <name> <version>
    let pkg_name_chirho = match positional_chirho.get(1) {
        Some(name_chirho) => *name_chirho,
        None => {
            eprintln!("missing package name for `install`");
            eprintln!(
                "usage: {} install <package-name> <version>",
                program_name_chirho
            );
            return ExitCode::from(2);
        }
    };

    let version_str_chirho = match positional_chirho.get(2) {
        Some(v_chirho) => *v_chirho,
        None => {
            eprintln!("missing version for `install`");
            eprintln!(
                "usage: {} install <package-name> <version>",
                program_name_chirho
            );
            return ExitCode::from(2);
        }
    };

    let version_chirho = match haskelujah_package_chirho::parse_version_chirho(version_str_chirho) {
        Some(v_chirho) => v_chirho,
        None => {
            eprintln!("invalid version: `{version_str_chirho}`");
            return ExitCode::from(2);
        }
    };

    eprintln!("Installing {pkg_name_chirho}-{version_chirho}...");

    // Use a subdirectory of the current directory as the install root.
    let install_dir_chirho = std::path::PathBuf::from(".haskelujah-packages-chirho");
    if let Err(e_chirho) = fs::create_dir_all(&install_dir_chirho) {
        eprintln!("cannot create install directory: {e_chirho}");
        return ExitCode::from(1);
    }

    // Load or create the package database.
    let db_path_chirho = install_dir_chirho.join("pkgdb-chirho.txt");
    let mut db_chirho = if db_path_chirho.exists() {
        match fs::read_to_string(&db_path_chirho) {
            Ok(content_chirho) => {
                haskelujah_package_chirho::InstalledPkgDbChirho::from_string_chirho(&content_chirho)
            }
            Err(_) => haskelujah_package_chirho::InstalledPkgDbChirho::new_chirho(),
        }
    } else {
        haskelujah_package_chirho::InstalledPkgDbChirho::new_chirho()
    };

    // Empty index for now — real Hackage index would be populated separately.
    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();

    match haskelujah_driver_chirho::install_package_chirho(
        pkg_name_chirho,
        &version_chirho,
        &install_dir_chirho,
        &index_chirho,
        &mut db_chirho,
    ) {
        Ok(result_chirho) => {
            // Save updated package database.
            if let Err(e_chirho) = fs::write(&db_path_chirho, db_chirho.to_string_chirho()) {
                eprintln!("warning: could not save package database: {e_chirho}");
            }

            eprintln!(
                "Installed {}-{} ({} modules, {} dependencies)",
                result_chirho.installed_pkg_chirho.name_chirho,
                result_chirho.installed_pkg_chirho.version_chirho,
                result_chirho.modules_compiled_chirho,
                result_chirho.installed_pkg_chirho.depends_chirho.len(),
            );
            if !result_chirho
                .installed_pkg_chirho
                .exposed_modules_chirho
                .is_empty()
            {
                eprintln!("  exposed modules:");
                for mod_chirho in &result_chirho.installed_pkg_chirho.exposed_modules_chirho {
                    eprintln!("    {}", mod_chirho.module_name_chirho);
                }
            }
            eprintln!("Package registered in {}", db_path_chirho.display());
            ExitCode::SUCCESS
        }
        Err(error_chirho) => {
            eprintln!("error: {error_chirho}");
            ExitCode::from(1)
        }
    }
}

fn print_usage_chirho(program_name_chirho: &str) {
    eprintln!("Haskelujah Chirho — A Haskell compiler in Rust");
    eprintln!("GHC compatibility: 858/938 (91.5%)");
    eprintln!();
    eprintln!("usage: {program_name_chirho} <command> [args] [options]");
    eprintln!();
    eprintln!("commands:");
    eprintln!("  init     [name]                 create a new project with .cabal scaffold");
    eprintln!("  build    [dir]                  compile a Cabal project to native executable");
    eprintln!("  run      <file.hs>              compile via LLVM and execute (STG fallback)");
    eprintln!("  check    <file.hs>              type-check without code generation");
    eprintln!("  compile  <file.hs> -o <exe>     compile to native executable via LLVM");
    eprintln!("  clean    [dir]                  remove build artifacts (dist-chirho/)");
    eprintln!("  install  <package> <version>    fetch from Hackage, compile, register");
    eprintln!("  repl                            interactive REPL");
    eprintln!();
    eprintln!("flags:");
    eprintln!("  -o, --output <path>  output native executable or .wasm file");
    eprintln!("  --wasm               emit WebAssembly instead of native");
    eprintln!("  --cranelift          emit native code via Cranelift for `compile`");
    eprintln!("  --llvm               force LLVM instead of default Cranelift `build`");
    eprintln!("  --dump-core          print Core IR to stderr");
    eprintln!("  --dump-stg           print STG code table to stderr");
    eprintln!("  --dump-llvm          print LLVM IR to stderr");
    eprintln!();
    eprintln!("example:");
    eprintln!("  {program_name_chirho} init my-project");
    eprintln!("  {program_name_chirho} build my-project");
    eprintln!("  ./my-project/dist-chirho/build/my-project");
}
