// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

use haskelujah_driver::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

fn assert_compile_success_chirho(file_name_chirho: &str, source_chirho: &str) {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(source_chirho, &mut source_map_chirho, file_name_chirho);
    assert!(
        result_chirho.is_ok(),
        "expected compile_source_chirho to succeed for {file_name_chirho}, got {:?}",
        result_chirho.err()
    );
}

#[test]
fn identity_function_infers_chirho() {
    assert_compile_success_chirho(
        "IdentityFunctionChirho.hs",
        "module IdentityFunctionChirho where\nfChirho xChirho = xChirho\n",
    );
}

#[test]
fn data_type_with_constructors_infers_chirho() {
    assert_compile_success_chirho(
        "DataTypeConstructorsChirho.hs",
        "module DataTypeConstructorsChirho where\ndata ColorChirho = RedChirho | GreenChirho | BlueChirho\n",
    );
}

#[test]
fn if_expression_infers_chirho() {
    assert_compile_success_chirho(
        "IfExpressionChirho.hs",
        "module IfExpressionChirho where\ngChirho xChirho = if True then xChirho else xChirho\n",
    );
}

#[test]
fn list_literal_infers_chirho() {
    assert_compile_success_chirho(
        "ListLiteralChirho.hs",
        "module ListLiteralChirho where\nxsChirho = [1, 2, 3]\n",
    );
}

#[test]
fn tuple_literal_infers_chirho() {
    assert_compile_success_chirho(
        "TupleLiteralChirho.hs",
        "module TupleLiteralChirho where\npChirho = (1, True)\n",
    );
}

#[test]
fn let_expression_infers_chirho() {
    assert_compile_success_chirho(
        "LetExpressionChirho.hs",
        "module LetExpressionChirho where\nhChirho = let { yChirho = 42 } in 42\n",
    );
}

#[test]
fn lambda_expression_infers_chirho() {
    assert_compile_success_chirho(
        "LambdaExpressionChirho.hs",
        "module LambdaExpressionChirho where\nkChirho = \\xChirho -> xChirho\n",
    );
}

#[test]
fn negative_type_error_reports_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module NegativeTypeErrorChirho where\nbadChirho = if 42 then 1 else 2\n",
        &mut source_map_chirho,
        "NegativeTypeErrorChirho.hs",
    );
    assert!(
        result_chirho.is_err(),
        "expected compile_source_chirho to report a type error"
    );
}
