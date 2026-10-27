// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Native round trips with strict stage failures and bounded generated execution.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::Duration;

use haskelujah_core_chirho::CoreModuleChirho;
use haskelujah_driver_chirho::rts_snapshot_chirho::{RtsSnapshotChirho, rts_snapshot_chirho};
use haskelujah_driver_chirho::{build_cabal_project_chirho, compile_source_chirho};
use haskelujah_span_chirho::SourceMapChirho;

use crate::process_chirho::{run_bounded_chirho, run_bounded_with_memory_chirho};

#[derive(Clone, Copy, Debug)]
pub enum NativeBackendChirho {
    LlvmChirho,
    CraneliftChirho,
}

/// Generated input for the same reserved-workspace and bounded-child boundary
/// used by source round trips. Backend unit tests must not invent their own paths.
pub enum NativeArtifactChirho {
    LlvmIrChirho(String),
    CraneliftObjectChirho(Vec<u8>),
}

/// This link's private snapshot of the RTS archive (row 568). The freshness build runs
/// once per process, inside the snapshot's lock.
/// workflow: testing-chirho/native-fail-loud-chirho
fn runtime_snapshot_chirho() -> Result<RtsSnapshotChirho, String> {
    static BUILT_CHIRHO: OnceLock<Result<(), String>> = OnceLock::new();
    let root_chirho = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace crate");
    let target_chirho = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| root_chirho.join("target"));
    let target_chirho = if target_chirho.is_absolute() {
        target_chirho
    } else {
        root_chirho.join(target_chirho)
    };
    rts_snapshot_chirho(&target_chirho.join("debug"), || {
        BUILT_CHIRHO
            .get_or_init(|| {
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
                Ok(())
            })
            .clone()
    })
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
    let artifact_chirho = native_artifact_chirho(&compiled_chirho.core_chirho, backend_chirho)?;
    native_artifact_round_trip_chirho(artifact_chirho, input_chirho, deadline_chirho)
}

/// A multi-module program (row 628): `modules_chirho` are (module name, source) pairs, built as
/// one Cabal executable whose `main-is` is `Main.hs`, as `haskelujah build-run` builds it, then
/// linked and run like a single source. The chosen backend runs alone, with no fallback. The
/// caller names the run's deadline, as scale tests do.
/// workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
pub fn native_project_round_trip_chirho(
    modules_chirho: &[(&str, &str)],
    backend_chirho: NativeBackendChirho,
    input_chirho: &str,
    deadline_chirho: Duration,
) -> Result<(i32, String), String> {
    let directory_chirho = tempfile::tempdir().map_err(|error_chirho| error_chirho.to_string())?;
    for (module_chirho, source_chirho) in modules_chirho {
        std::fs::write(
            directory_chirho.path().join(format!("{module_chirho}.hs")),
            source_chirho,
        )
        .map_err(|error_chirho| error_chirho.to_string())?;
    }
    let others_chirho: Vec<&str> = modules_chirho
        .iter()
        .map(|(module_chirho, _)| *module_chirho)
        .filter(|module_chirho| *module_chirho != "Main")
        .collect();
    let cabal_path_chirho = directory_chirho.path().join("probe-chirho.cabal");
    std::fs::write(
        &cabal_path_chirho,
        format!(
            "cabal-version: 3.0\nname: probe-chirho\nversion: 0.1.0\nbuild-type: Simple\n\n\
             executable probe-chirho\n  main-is: Main.hs\n  other-modules: {}\n  \
             build-depends: base\n  hs-source-dirs: .\n  default-language: Haskell2010\n",
            others_chirho.join(", ")
        ),
    )
    .map_err(|error_chirho| error_chirho.to_string())?;
    let built_chirho = build_cabal_project_chirho(
        &cabal_path_chirho,
        &haskelujah_package_chirho::PackageIndexChirho::new_chirho(),
    )?;
    let executable_chirho = built_chirho
        .executables_chirho
        .first()
        .ok_or("the package built no executable")?;
    let artifact_chirho = native_artifact_chirho(&executable_chirho.core_chirho, backend_chirho)?;
    native_artifact_round_trip_chirho(artifact_chirho, input_chirho, deadline_chirho)
}

/// The backend's executable artifact for compiled Core.
fn native_artifact_chirho(
    core_chirho: &CoreModuleChirho,
    backend_chirho: NativeBackendChirho,
) -> Result<NativeArtifactChirho, String> {
    Ok(match backend_chirho {
        NativeBackendChirho::LlvmChirho => NativeArtifactChirho::LlvmIrChirho(
            haskelujah_backend_llvm_chirho::try_compile_core_to_llvm_executable_chirho(
                core_chirho,
            )?,
        ),
        NativeBackendChirho::CraneliftChirho => NativeArtifactChirho::CraneliftObjectChirho(
            haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
                core_chirho,
                &Default::default(),
            )?
            .object_bytes_chirho,
        ),
    })
}

/// Link and run an already-generated module with private, atomically reserved
/// storage. See testing-chirho/execution-oracles-chirho.md for the stage contract.
pub fn native_artifact_round_trip_chirho(
    artifact_chirho: NativeArtifactChirho,
    input_chirho: &str,
    deadline_chirho: Duration,
) -> Result<(i32, String), String> {
    let directory_chirho = tempfile::tempdir().map_err(|error_chirho| error_chirho.to_string())?;
    let executable_chirho = directory_chirho.path().join("main-chirho");
    let mut linker_chirho = Command::new(match &artifact_chirho {
        NativeArtifactChirho::LlvmIrChirho(_) => "clang",
        NativeArtifactChirho::CraneliftObjectChirho(_) => "cc",
    });
    linker_chirho.arg("-o").arg(&executable_chirho);
    match artifact_chirho {
        NativeArtifactChirho::LlvmIrChirho(ir_chirho) => {
            linker_chirho.args([
                "-target",
                &haskelujah_backend_llvm_chirho::native_target_chirho(),
            ]);
            let source_path_chirho = directory_chirho.path().join("main-chirho.ll");
            std::fs::write(&source_path_chirho, ir_chirho)
                .map_err(|error_chirho| error_chirho.to_string())?;
            linker_chirho.arg("-O0").arg(source_path_chirho);
        }
        NativeArtifactChirho::CraneliftObjectChirho(object_bytes_chirho) => {
            let object_path_chirho = directory_chirho.path().join("main-chirho.o");
            std::fs::write(&object_path_chirho, object_bytes_chirho)
                .map_err(|error_chirho| error_chirho.to_string())?;
            linker_chirho.arg(object_path_chirho);
            if cfg!(target_os = "macos") {
                linker_chirho.arg("-Wl,-no_fixup_chains");
            }
        }
    }
    // The snapshot stays alive until this function returns, after the link.
    let runtime_chirho = runtime_snapshot_chirho()?;
    linker_chirho
        .arg("-L")
        .arg(runtime_chirho.link_dir_chirho())
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
