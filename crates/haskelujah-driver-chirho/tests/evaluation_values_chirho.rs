// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use haskelujah_runtime_chirho::ValueChirho;
use haskelujah_span_chirho::SourceMapChirho;

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
