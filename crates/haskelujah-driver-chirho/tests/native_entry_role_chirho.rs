// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Real emitted launchers must distinguish an IO payload from process status.
//! Expectations are source-only controls until the sole compiler scheduler runs them.
//! Workflow: testing-chirho/native-entry-role-chirho.

use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_program_round_trip_chirho, native_round_trip_chirho,
};

const BACKENDS_CHIRHO: [NativeBackendChirho; 2] = [
    NativeBackendChirho::LlvmChirho,
    NativeBackendChirho::CraneliftChirho,
];

fn assert_program_chirho(source_chirho: &str, expected_output_chirho: &str) {
    for backend_chirho in BACKENDS_CHIRHO {
        let outcome_chirho = native_program_round_trip_chirho(source_chirho, backend_chirho, "")
            .unwrap_or_else(|error_chirho| panic!("{backend_chirho:?}: {error_chirho}"));
        assert_eq!(
            outcome_chirho,
            (0, expected_output_chirho.to_string()),
            "{backend_chirho:?}"
        );
    }
}

#[test]
fn an_io_int_payload_is_neither_printed_nor_process_status_chirho() {
    assert_program_chirho("module Main where\nmain :: IO Int\nmain = return 7\n", "");
}

#[test]
fn action_effects_run_before_its_payload_is_discarded_chirho() {
    assert_program_chirho(
        "module Main where\nmain :: IO Int\nmain = print (1 :: Int) >> return 5\n",
        "1\n",
    );
}

#[test]
fn discarding_an_io_payload_does_not_demand_it_chirho() {
    assert_program_chirho(
        "module Main where\nmain :: IO Int\nmain = return (error \"unused-payload-chirho\")\n",
        "",
    );
}

#[test]
fn pure_entry_keeps_each_backends_existing_value_observation_chirho() {
    let source_chirho = "module Main where\nmain :: Int\nmain = 7\n";
    for backend_chirho in BACKENDS_CHIRHO {
        let expected_stdout_chirho = match backend_chirho {
            NativeBackendChirho::LlvmChirho => "7\n",
            NativeBackendChirho::CraneliftChirho => "",
        };
        assert_eq!(
            native_round_trip_chirho(source_chirho, backend_chirho, "")
                .unwrap_or_else(|error_chirho| panic!("{backend_chirho:?}: {error_chirho}")),
            (7, expected_stdout_chirho.to_string()),
            "{backend_chirho:?}"
        );
    }
}

#[test]
fn an_action_failure_preserves_its_prior_effect_and_named_error_chirho() {
    let source_chirho = "module Main where\nmain :: IO Int\nmain = putStrLn \"before-program-error-chirho\" >> error \"program-entry-error-chirho\"\n";
    for backend_chirho in BACKENDS_CHIRHO {
        let error_chirho = native_program_round_trip_chirho(source_chirho, backend_chirho, "")
            .expect_err("the action must fail before its result can be discarded");
        assert!(
            error_chirho.contains("native program emitted stderr (exit 1)")
                && error_chirho.contains("program-entry-error-chirho")
                && error_chirho.contains("stdout:\nbefore-program-error-chirho\n"),
            "{backend_chirho:?}: {error_chirho}"
        );
    }
}

/// This control remains required even if a generic `return 7` gains empty stdout.
/// Entry dictionary application is a separate, still-open consumer seam (row 692).
#[test]
fn a_generic_program_entry_must_execute_its_effect_chirho() {
    assert_program_chirho(
        "{-# LANGUAGE NoMonomorphismRestriction #-}\nmodule Main where\nmain = print 7 >> return 7\n",
        "7\n",
    );
}
