// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Unit-generated objects share the bounded native artifact execution boundary.

use std::time::Duration;

use haskelujah_test_harness_chirho::native_chirho::{
    NativeArtifactChirho, native_artifact_round_trip_chirho,
};

use super::*;

fn run_object_chirho(object_chirho: Vec<u8>) -> (i32, String) {
    native_artifact_round_trip_chirho(
        NativeArtifactChirho::CraneliftObjectChirho(object_chirho),
        "",
        Duration::from_secs(15),
    )
    .expect("Cranelift generation must link and finish without diagnostics")
}

pub(super) fn compile_and_run_exit_code_chirho(module_chirho: &CoreModuleChirho) -> i32 {
    run_object_chirho(compile_ok_chirho(module_chirho)).0
}

pub(super) fn compile_executable_and_run_exit_code_chirho(module_chirho: &CoreModuleChirho) -> i32 {
    let object_chirho =
        compile_core_to_object_executable_chirho(module_chirho, &TargetConfigChirho::default())
            .expect("executable object generation");
    run_object_chirho(object_chirho.object_bytes_chirho).0
}

pub(super) fn compile_and_run_stdout_chirho(module_chirho: &CoreModuleChirho) -> String {
    let (code_chirho, stdout_chirho) = run_object_chirho(compile_ok_chirho(module_chirho));
    assert_eq!(code_chirho, 0, "executable failed: {stdout_chirho}");
    stdout_chirho
}

#[test]
fn concurrent_native_programs_execute_their_own_artifact_chirho() {
    std::thread::scope(|scope_chirho| {
        for value_chirho in [0, 42, 7, 99] {
            scope_chirho.spawn(move || {
                for _round_chirho in 0..8 {
                    let module_chirho = single_binding_module_chirho(
                        "main",
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(value_chirho)),
                    );
                    assert_eq!(
                        i64::from(compile_executable_and_run_exit_code_chirho(&module_chirho)),
                        value_chirho,
                    );
                }
            });
        }
    });
}
