// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Concurrent preprocessing must consume its own input and retain include roots.

use std::io;
use std::sync::Barrier;

use crate::{
    preprocess_cpp_chirho, preprocess_cpp_source_with_options_chirho,
    preprocess_hsc_source_with_options_chirho,
};

fn assert_parallel_outputs_chirho(
    process_chirho: impl Fn(usize, i32) -> io::Result<String> + Sync,
) {
    let barrier_chirho = Barrier::new(4);
    let failures_chirho = std::thread::scope(|scope_chirho| {
        let handles_chirho: Vec<_> = [0, 42, 7, 99].into_iter().enumerate().map(
            |(worker_chirho, value_chirho)| {
                let barrier_chirho = &barrier_chirho;
                let process_chirho = &process_chirho;
                scope_chirho.spawn(move || {
                    let mut failures_chirho = Vec::new();
                    for round_chirho in 0..32 {
                        barrier_chirho.wait();
                        let expected_chirho = format!("main = print {value_chirho}");
                        match process_chirho(worker_chirho, value_chirho) {
                            Ok(output_chirho) if output_chirho.lines().any(
                                |line_chirho| line_chirho.trim() == expected_chirho
                            ) => {}
                            result_chirho => failures_chirho.push(format!(
                                "worker {worker_chirho}, round {round_chirho}: expected {expected_chirho:?}, got {result_chirho:?}"
                            )),
                        }
                    }
                    failures_chirho
                })
            },
        ).collect();
        handles_chirho
            .into_iter()
            .flat_map(|handle_chirho| handle_chirho.join().expect("CPP worker must finish"))
            .collect::<Vec<_>>()
    });
    assert!(failures_chirho.is_empty(), "{}", failures_chirho.join("\n"));
}

#[test]
fn concurrent_in_memory_cpp_preserves_each_input_chirho() {
    assert_parallel_outputs_chirho(|_, value_chirho| {
        Ok(preprocess_cpp_chirho(&format!(
            "{{-# LANGUAGE CPP #-}}\n#define VALUE_CHIRHO {value_chirho}\nmodule Main where\nmain = print VALUE_CHIRHO\n"
        )))
    });
}

#[test]
fn concurrent_sanitized_cpp_retains_local_and_support_headers_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    std::fs::write(
        directory_chirho.path().join("LocalChirho.h"),
        "#define LOCAL_CHIRHO 17\n",
    )
    .unwrap();
    assert_parallel_outputs_chirho(|worker_chirho, value_chirho| {
        let source_chirho = format!(
            "{{-# LANGUAGE CPP\n#-}}\n#include \"LocalChirho.h\"\n#include \"MachDeps.h\"\n#include \"HsBaseConfig.h\"\n#if LOCAL_CHIRHO != 17 || WORD_SIZE_IN_BITS != 64\n#error wrong_include_chirho\n#endif\n#ifndef HASKELUJAH_SYNTHETIC_HSBASECONFIG_H_CHIRHO\n#error missing_support_chirho\n#endif\n#define VALUE_CHIRHO {value_chirho}\nmodule Main where\nmain = print VALUE_CHIRHO\n"
        );
        let path_chirho = directory_chirho
            .path()
            .join(format!("Input{worker_chirho}Chirho.hs"));
        std::fs::write(&path_chirho, &source_chirho)?;
        preprocess_cpp_source_with_options_chirho(&path_chirho, &source_chirho, &[])
    });
    // Only the four original inputs and their local header remain.
    assert_eq!(
        std::fs::read_dir(directory_chirho.path()).unwrap().count(),
        5
    );
}

#[test]
fn concurrent_hsc_preprocessing_preserves_each_input_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    assert_parallel_outputs_chirho(|worker_chirho, value_chirho| {
        let source_chirho = format!(
            "#define VALUE_CHIRHO {value_chirho}\nmodule Main where\nsizeChirho = #{{size int}}\nmain = print VALUE_CHIRHO\n"
        );
        preprocess_hsc_source_with_options_chirho(
            &directory_chirho
                .path()
                .join(format!("Input{worker_chirho}Chirho.hsc")),
            &source_chirho,
            &[],
        )
    });
    assert_eq!(
        std::fs::read_dir(directory_chirho.path()).unwrap().count(),
        0
    );
}

#[test]
fn failed_cpp_retires_private_input_without_removing_original_chirho() {
    let directory_chirho = tempfile::tempdir().unwrap();
    let path_chirho = directory_chirho.path().join("InputChirho.hs");
    let source_chirho =
        "{-# LANGUAGE CPP\n#-}\n#error expected_failure_chirho\nmodule Main where\n";
    std::fs::write(&path_chirho, source_chirho).unwrap();
    let error_chirho = preprocess_cpp_source_with_options_chirho(&path_chirho, source_chirho, &[])
        .expect_err("the requested CPP error must propagate");
    assert!(error_chirho.to_string().contains("expected_failure_chirho"));
    assert_eq!(
        std::fs::read_to_string(&path_chirho).unwrap(),
        source_chirho
    );
    assert_eq!(
        std::fs::read_dir(directory_chirho.path()).unwrap().count(),
        1
    );
}
