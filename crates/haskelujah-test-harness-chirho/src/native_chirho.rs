// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Native round trips with strict stage failures and bounded generated execution.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::Duration;

use haskelujah_driver_chirho::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

use crate::process_chirho::{run_bounded_chirho, run_bounded_with_memory_chirho};

#[derive(Clone, Copy, Debug)]
pub enum NativeBackendChirho {
    LlvmChirho,
    CraneliftChirho,
}

fn runtime_library_chirho() -> Result<PathBuf, String> {
    static LIBRARY_CHIRHO: OnceLock<Result<PathBuf, String>> = OnceLock::new();
    LIBRARY_CHIRHO
        .get_or_init(|| {
            let root_chirho = Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .and_then(Path::parent)
                .expect("workspace crate");
            let output_chirho = run_bounded_chirho(
                Command::new("cargo").current_dir(root_chirho).args([
                    "build",
                    "-p",
                    "haskelujah-rts",
                    "--quiet",
                ]),
                &[],
                Duration::from_secs(120),
            )?;
            if !output_chirho.status.success() {
                return Err(format!(
                    "RTS build failed: {}",
                    String::from_utf8_lossy(&output_chirho.stderr)
                ));
            }
            if !output_chirho.stderr.is_empty() {
                return Err(format!(
                    "RTS build diagnostics: {}",
                    String::from_utf8_lossy(&output_chirho.stderr)
                ));
            }
            let target_chirho = std::env::var_os("CARGO_TARGET_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| root_chirho.join("target"));
            let target_chirho = if target_chirho.is_absolute() {
                target_chirho
            } else {
                root_chirho.join(target_chirho)
            };
            Ok(target_chirho.join("debug"))
        })
        .clone()
}

pub fn native_round_trip_chirho(
    source_chirho: &str,
    backend_chirho: NativeBackendChirho,
    input_chirho: &str,
) -> Result<(i32, String), String> {
    native_round_trip_with_deadline_chirho(
        source_chirho,
        backend_chirho,
        input_chirho,
        Duration::from_secs(15),
    )
}

/// A workload-specific execution deadline, without weakening stage failures,
/// output limits, memory bounds, or cleanup. Scale tests name their budget.
pub fn native_round_trip_with_deadline_chirho(
    source_chirho: &str,
    backend_chirho: NativeBackendChirho,
    input_chirho: &str,
    deadline_chirho: Duration,
) -> Result<(i32, String), String> {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let compiled_chirho =
        compile_source_chirho(source_chirho, &mut source_map_chirho, "Main.hs")
            .map_err(|diagnostics_chirho| format!("frontend failed: {diagnostics_chirho:?}"))?;
    let directory_chirho = tempfile::tempdir().map_err(|error_chirho| error_chirho.to_string())?;
    let executable_chirho = directory_chirho.path().join("main-chirho");
    let mut linker_chirho = Command::new(match backend_chirho {
        NativeBackendChirho::LlvmChirho => "clang",
        NativeBackendChirho::CraneliftChirho => "cc",
    });
    linker_chirho.arg("-o").arg(&executable_chirho);
    match backend_chirho {
        NativeBackendChirho::LlvmChirho => {
            linker_chirho.args([
                "-target",
                &haskelujah_backend_llvm_chirho::native_target_chirho(),
            ]);
            let ir_chirho =
                haskelujah_backend_llvm_chirho::try_compile_core_to_llvm_executable_chirho(
                    &compiled_chirho.core_chirho,
                )?;
            let source_path_chirho = directory_chirho.path().join("main-chirho.ll");
            std::fs::write(&source_path_chirho, ir_chirho)
                .map_err(|error_chirho| error_chirho.to_string())?;
            linker_chirho.arg("-O0").arg(source_path_chirho);
        }
        NativeBackendChirho::CraneliftChirho => {
            let object_chirho =
                haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
                    &compiled_chirho.core_chirho,
                    &Default::default(),
                )?;
            let object_path_chirho = directory_chirho.path().join("main-chirho.o");
            std::fs::write(&object_path_chirho, object_chirho.object_bytes_chirho)
                .map_err(|error_chirho| error_chirho.to_string())?;
            linker_chirho.arg(object_path_chirho);
            if cfg!(target_os = "macos") {
                linker_chirho.arg("-Wl,-no_fixup_chains");
            }
        }
    }
    linker_chirho
        .arg("-L")
        .arg(runtime_library_chirho()?)
        .arg("-lhaskelujah_rts");
    let linked_chirho = run_bounded_chirho(&mut linker_chirho, &[], Duration::from_secs(60))?;
    if !linked_chirho.status.success() {
        return Err(format!(
            "native linking failed: {}",
            String::from_utf8_lossy(&linked_chirho.stderr)
        ));
    }
    if !linked_chirho.stderr.is_empty() {
        return Err(format!(
            "native link diagnostics: {}",
            String::from_utf8_lossy(&linked_chirho.stderr)
        ));
    }
    let mut command_chirho = Command::new(&executable_chirho);
    let output_chirho = run_bounded_with_memory_chirho(
        &mut command_chirho,
        input_chirho.as_bytes(),
        deadline_chirho,
        Some(2 * 1024 * 1024 * 1024),
    )?;
    let code_chirho = output_chirho.status.code().ok_or_else(|| {
        format!(
            "native program terminated by signal: {}\nstdout:\n{}\nstderr:\n{}",
            output_chirho.status,
            String::from_utf8_lossy(&output_chirho.stdout),
            String::from_utf8_lossy(&output_chirho.stderr)
        )
    })?;
    let stdout_chirho =
        String::from_utf8(output_chirho.stdout).map_err(|error_chirho| error_chirho.to_string())?;
    if !output_chirho.stderr.is_empty() {
        return Err(format!(
            "native program emitted stderr (exit {code_chirho}): {}\nstdout:\n{stdout_chirho}",
            String::from_utf8_lossy(&output_chirho.stderr)
        ));
    }
    Ok((code_chirho, stdout_chirho))
}
