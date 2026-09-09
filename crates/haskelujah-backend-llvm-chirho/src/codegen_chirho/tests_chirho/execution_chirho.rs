// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Unit-generated IR uses the source harness's artifact ownership and deadlines.

use std::time::Duration;

use haskelujah_test_harness_chirho::native_chirho::{
    NativeArtifactChirho, native_artifact_round_trip_chirho,
};

use super::*;

pub(super) fn run_executable_module_result_chirho(
    module_chirho: &CoreModuleChirho,
) -> (i32, String, String) {
    let ir_chirho = compile_core_to_llvm_executable_chirho(module_chirho);
    let (code_chirho, stdout_chirho) = native_artifact_round_trip_chirho(
        NativeArtifactChirho::LlvmIrChirho(ir_chirho),
        "",
        Duration::from_secs(15),
    )
    .expect("LLVM generation must link and finish without diagnostics");
    (code_chirho, stdout_chirho, String::new())
}

pub(super) fn run_executable_module_chirho(module_chirho: &CoreModuleChirho) -> String {
    let (code_chirho, stdout_chirho, _) = run_executable_module_result_chirho(module_chirho);
    assert_eq!(code_chirho, 0, "executable failed: {stdout_chirho}");
    stdout_chirho
}

#[test]
fn concurrent_native_programs_execute_their_own_artifact_chirho() {
    std::thread::scope(|scope_chirho| {
        for value_chirho in [0, 42, 7, 99] {
            scope_chirho.spawn(move || {
                for _round_chirho in 0..8 {
                    let module_chirho = CoreModuleChirho {
                        name_chirho: "IsolatedChirho".into(),
                        bindings_chirho: vec![CoreBindingChirho {
                            binder_chirho: dummy_binder_chirho("main", 0),
                            rhs_chirho: int_lit_chirho(value_chirho),
                            is_rec_chirho: false,
                            inline_chirho: InlineAnnotationChirho::NoneChirho,
                        }],
                        names_chirho: Default::default(),
                        specialize_pragmas_chirho: Default::default(),
                        foreign_exports_chirho: vec![],
                    };
                    let (code_chirho, stdout_chirho, _) =
                        run_executable_module_result_chirho(&module_chirho);
                    assert_eq!(i64::from(code_chirho), value_chirho);
                    assert_eq!(stdout_chirho, format!("{value_chirho}\n"));
                }
            });
        }
    });
}
