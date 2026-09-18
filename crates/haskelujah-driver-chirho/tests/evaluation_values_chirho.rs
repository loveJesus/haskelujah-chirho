// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use haskelujah_runtime_chirho::ValueChirho;
use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn mixed_unicode_operators_execute_the_complete_name_chirho() {
    // Unchanged sources and exact outputs independently measured with GHC9.14.1.
    let cases_chirho: &[(&str, &str, &str)] = &include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/imported-families-chirho/source-boot-chirho/declarations-chirho/classes-chirho/default-annotations-chirho/scope-chirho/corpus-chirho/unicode-operators-chirho/fixtures_chirho.rs"
    ));
    let mut failures_chirho = Vec::new();
    for (name_chirho, source_chirho, expected_chirho) in cases_chirho {
        match haskelujah_driver::eval_source_with_machine_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "Main.hs",
            None,
        ) {
            Ok((_, machine_chirho)) if machine_chirho.io_output_chirho == *expected_chirho => {}
            Ok((_, machine_chirho)) => failures_chirho.push(format!(
                "{name_chirho}: expected {expected_chirho:?}, got {:?}",
                machine_chirho.io_output_chirho
            )),
            Err(error_chirho) => failures_chirho.push(format!("{name_chirho}: {error_chirho}")),
        }
    }
    assert!(failures_chirho.is_empty(), "{}", failures_chirho.join("\n"));
}

#[test]
fn numeric_application_returns_its_value_chirho() {
    let source_chirho = "module Main where\nf_chirho x_chirho = x_chirho + 1\nmain = f_chirho 0\n";
    let (value_chirho, _machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "NumericApplicationChirho.hs",
        None,
    )
    .expect("numeric application evaluates");
    assert_eq!(value_chirho, ValueChirho::IntChirho(1));
}

#[test]
fn read_consumes_lazy_character_lists_chirho() {
    let source_chirho = r#"module Main where
main = do
  print (read (map id "42") :: Int)
  print (read (tail "x-2.25") :: Double)
  print (read (map id "True") :: Bool)
  print (read (map id "False") :: Bool)
"#;
    let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ReadCharactersChirho.hs",
        None,
    )
    .expect("read accepts the character-list representation of String");
    assert_eq!(machine_chirho.io_output_chirho, "42\n-2.25\nTrue\nFalse\n");
}

#[test]
fn read_propagates_a_demanded_character_or_tail_error_chirho() {
    for expression_chirho in [
        "[error \"demanded-read-chirho\"]",
        "'4' : error \"demanded-read-chirho\"",
    ] {
        let source_chirho =
            format!("module Main where\nmain = print (read ({expression_chirho}) :: Int)\n");
        let error_chirho = haskelujah_driver::eval_source_with_machine_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "ReadDemandChirho.hs",
            None,
        )
        .err()
        .expect("read must not accept a truncated prefix after a forcing failure");
        assert!(
            error_chirho.to_string().contains("demanded-read-chirho"),
            "{error_chirho}"
        );
    }
}
