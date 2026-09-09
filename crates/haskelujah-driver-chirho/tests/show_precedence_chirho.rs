// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

#[test]
fn constructor_show_respects_precedence_and_preserves_compound_evidence_chirho() {
    for (expression_chirho, expected_chirho) in [
        ("Just (Just 42)", "Just (Just 42)"),
        ("Just (Nothing :: Maybe Int)", "Just Nothing"),
        ("Just (-42 :: Int)", "Just (-42)"),
        ("Just (-2.5 :: Double)", "Just (-2.5)"),
        ("Just (True, Just 3)", "Just (True,Just 3)"),
        (
            "(Left (Just 2) :: Either (Maybe Int) Bool)",
            "Left (Just 2)",
        ),
        (
            "(Right (Just False) :: Either Int (Maybe Bool))",
            "Right (Just False)",
        ),
        ("Just \"a b\"", "Just \"a b\""),
    ] {
        let source_chirho =
            format!("module Main where\nmain = putStrLn (show ({expression_chirho}))\n");
        let (_, machine_chirho) = eval_source_with_machine_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "ShowPrecedenceChirho.hs",
            None,
        )
        .unwrap();
        assert_eq!(
            machine_chirho.io_output_chirho,
            format!("{expected_chirho}\n"),
            "{expression_chirho}"
        );
    }
}

#[test]
fn constructor_show_is_portable_to_both_native_backends_chirho() {
    let source_chirho = "module Main where\nmain = putStrLn (show (Just (Just (42 :: Int))))\n";
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        let (status_chirho, output_chirho) =
            native_round_trip_chirho(source_chirho, backend_chirho, "").unwrap();
        assert_eq!(status_chirho, 0, "{backend_chirho:?}");
        assert_eq!(output_chirho, "Just (Just 42)\n", "{backend_chirho:?}");
    }
}
