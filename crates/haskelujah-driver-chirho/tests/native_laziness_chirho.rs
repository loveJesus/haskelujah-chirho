// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One source and an independent oracle through all three execution engines.
//! The demanded-field control must fail with its own message, not any failure.

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

const PARCEL_SOURCE_CHIRHO: &str = r#"module Main where
data ParcelChirho = ParcelChirho Int Int
{-# NOINLINE buildChirho #-}
buildChirho :: Int -> Int -> Int -> Int -> Int -> Int -> Int -> Int -> Int -> ParcelChirho
buildChirho aChirho bChirho cChirho dChirho eChirho fChirho gChirho hChirho iChirho =
  ParcelChirho (aChirho + bChirho + cChirho + dChirho + eChirho + fChirho + gChirho + hChirho + iChirho)
    (error "demanded-field-chirho")
firstChirho :: ParcelChirho -> Int
firstChirho (ParcelChirho valueChirho _) = valueChirho
secondChirho :: ParcelChirho -> Int
secondChirho (ParcelChirho _ valueChirho) = valueChirho
"#;

#[test]
fn lazy_field_captures_are_not_limited_by_call_trampoline_arity_chirho() {
    let source_chirho = format!(
        "{PARCEL_SOURCE_CHIRHO}\nmain = print (firstChirho (buildChirho 1 2 3 4 5 6 7 8 9))\n"
    );
    let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "LazyChirho.hs",
        None,
    )
    .expect("unused field stays unevaluated");
    assert_eq!(machine_chirho.io_output_chirho, "45\n");
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        assert_eq!(
            native_round_trip_chirho(&source_chirho, backend_chirho, "")
                .unwrap_or_else(|error_chirho| panic!("{backend_chirho:?}: {error_chirho}")),
            (0, "45\n".to_string()),
            "{backend_chirho:?}"
        );
    }
}

#[test]
fn demanding_the_other_field_reports_its_failure_chirho() {
    let source_chirho = format!(
        "{PARCEL_SOURCE_CHIRHO}\nmain = print (secondChirho (buildChirho 1 2 3 4 5 6 7 8 9))\n"
    );
    assert_runtime_error_chirho(&source_chirho, "demanded-field-chirho");
}

fn assert_runtime_error_chirho(source_chirho: &str, diagnostic_chirho: &str) {
    let error_chirho = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "DemandChirho.hs",
        None,
    )
    .expect_err("demanded error field must fail");
    assert!(
        error_chirho.starts_with("runtime error: ") && error_chirho.contains(diagnostic_chirho),
        "must fail during evaluation: {error_chirho}"
    );
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        let error_chirho = native_round_trip_chirho(source_chirho, backend_chirho, "")
            .expect_err("native demanded error field must fail");
        assert!(
            (error_chirho.starts_with("native program terminated by signal: ")
                || error_chirho.starts_with("native program emitted stderr "))
                && error_chirho.contains(diagnostic_chirho),
            "{backend_chirho:?}: {error_chirho}"
        );
    }
}

#[test]
fn strict_constructor_fields_are_demanded_before_matching_chirho() {
    assert_runtime_error_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
data StrictChirho = StrictChirho !Int
{-# NOINLINE makeStrictChirho #-}
makeStrictChirho :: Int -> StrictChirho
makeStrictChirho valueChirho = StrictChirho (if valueChirho == 1 then error "strict-field-chirho" else valueChirho)
markerChirho :: StrictChirho -> Int
markerChirho (StrictChirho _) = 7
main = print (markerChirho (makeStrictChirho 1))
"#,
        "strict-field-chirho",
    );
}

#[test]
fn reusing_an_io_action_performs_both_effects_chirho() {
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
main = do
  let actionChirho = putStrLn "again-chirho"
  actionChirho
  actionChirho
"#;
    let expected_chirho = "again-chirho\nagain-chirho\n";
    let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "RepeatedIoChirho.hs",
        None,
    )
    .expect("IO action can be reused");
    let mut outputs_chirho = vec![("STG".to_string(), machine_chirho.io_output_chirho)];
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        let (status_chirho, output_chirho) =
            native_round_trip_chirho(source_chirho, backend_chirho, "")
                .unwrap_or_else(|error_chirho| panic!("{backend_chirho:?}: {error_chirho}"));
        assert_eq!(status_chirho, 0, "{backend_chirho:?}");
        outputs_chirho.push((format!("{backend_chirho:?}"), output_chirho));
    }
    assert!(
        outputs_chirho
            .iter()
            .all(|(_, output_chirho)| output_chirho == expected_chirho),
        "expected {expected_chirho:?} on every engine; measured {outputs_chirho:?}"
    );
}

#[test]
fn call_demand_proofs_preserve_ignored_conditional_partial_and_recursive_arguments_chirho() {
    let source_chirho = r#"module Main where
{-# NOINLINE ignoreChirho #-}
ignoreChirho :: Int -> Int
ignoreChirho _ = 42
{-# NOINLINE chooseChirho #-}
chooseChirho :: Bool -> Int -> Int
chooseChirho flagChirho valueChirho = if flagChirho then 0 else valueChirho
{-# NOINLINE keepChirho #-}
keepChirho :: Int -> Int -> Int
keepChirho _ valueChirho = valueChirho
loopChirho :: Int -> Int -> Int
loopChirho countChirho unusedChirho = if countChirho == 0 then 0 else loopChirho (countChirho - 1) unusedChirho
main = do
  print (ignoreChirho (error "ignored-chirho"))
  print (chooseChirho True (error "unchosen-chirho"))
  let partialChirho = keepChirho (error "partial-chirho")
  print (partialChirho 7)
  print (loopChirho 100 (error "recursive-unused-chirho"))
"#;
    let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "CallDemandChirho.hs",
        None,
    )
    .unwrap_or_else(|error_chirho| panic!("STG: {error_chirho}"));
    assert_eq!(machine_chirho.io_output_chirho, "42\n0\n7\n0\n");
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        assert_eq!(
            native_round_trip_chirho(source_chirho, backend_chirho, "")
                .unwrap_or_else(|error_chirho| panic!("{backend_chirho:?}: {error_chirho}")),
            (0, "42\n0\n7\n0\n".to_string())
        );
    }
}
