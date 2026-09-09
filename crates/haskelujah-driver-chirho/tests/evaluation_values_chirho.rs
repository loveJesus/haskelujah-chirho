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
