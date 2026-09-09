// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Action values may be shared; executions may not be memoized. Oracles are
//! checked independently with GHC, then held fixed across all three engines.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

fn assert_engines_chirho(source_chirho: &str, input_chirho: &str, expected_chirho: &str) {
    let input_lines_chirho: Vec<_> = input_chirho.lines().collect();
    let (_, machine_chirho) = haskelujah_driver::eval_source_with_input_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ActionsChirho.hs",
        None,
        &input_lines_chirho,
    )
    .unwrap_or_else(|error_chirho| panic!("STG: {error_chirho}"));
    assert_eq!(machine_chirho.io_output_chirho, expected_chirho, "STG");
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        assert_eq!(
            native_round_trip_chirho(source_chirho, backend_chirho, input_chirho)
                .unwrap_or_else(|error_chirho| panic!("{backend_chirho:?}: {error_chirho}")),
            (0, expected_chirho.to_string()),
            "{backend_chirho:?}",
        );
    }
}

#[test]
fn demanding_an_action_does_not_execute_it_chirho() {
    assert_engines_chirho(
        include_str!("io_actions_chirho/whnf_chirho.hs"),
        "",
        "whnf-chirho\nthen-chirho\nbind-chirho\n",
    );
}

#[test]
fn actions_shared_through_a_list_execute_each_time_chirho() {
    assert_engines_chirho(
        include_str!("io_actions_chirho/list_chirho.hs"),
        "",
        "effect-chirho\neffect-chirho\neffect-chirho\n",
    );
}

#[test]
fn a_shared_input_action_consumes_fresh_input_chirho() {
    assert_engines_chirho(
        include_str!("io_actions_chirho/input_chirho.hs"),
        "first-chirho\nsecond-chirho\n",
        "before-chirho\nfirst-chirho/second-chirho\n",
    );
}

// These execution contracts replace five backend-local tests built from
// zero-returning name stubs or assertions about incidental LLVM IR spelling.

#[test]
fn get_line_reads_and_prints_input_chirho() {
    assert_engines_chirho(
        "module Main where\nmain = getLine >>= putStrLn\n",
        "input-chirho\n",
        "input-chirho\n",
    );
}

#[test]
fn numeric_print_uses_its_show_instance_chirho() {
    assert_engines_chirho("module Main where\nmain = print (42 :: Int)\n", "", "42\n");
}

#[test]
fn io_bind_executes_its_continuation_chirho() {
    assert_engines_chirho(
        "module Main where\nmain = return (41 :: Int) >>= \\valueChirho -> print (valueChirho + 1)\n",
        "",
        "42\n",
    );
}

#[test]
fn integer_list_print_contains_every_element_chirho() {
    assert_engines_chirho(
        "module Main where\nmain = print ([1,2,3] :: [Int])\n",
        "",
        "[1,2,3]\n",
    );
}

#[test]
fn integer_list_print_demands_computed_elements_chirho() {
    assert_engines_chirho(
        "module Main where\nmain = print ([1+0,4 `div` 2,6 `div` 2] :: [Int])\n",
        "",
        "[1,2,3]\n",
    );
}
