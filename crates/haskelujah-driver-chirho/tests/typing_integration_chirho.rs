// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

use haskelujah_driver::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

// Independently executed under GHC 9.14.1: 42 / 7 / 11 / True.
// The infix and context orders differ from normalized body traversal; the
// explicit forall and prefix form are controls that must retain their order.
const SOURCE_ORDER_CHIRHO: &str = r#"{-# LANGUAGE TypeOperators, TypeApplications, ExplicitForAll #-}
module Main where
implicitChirho :: aChirho `opChirho` bChirho -> aChirho `opChirho` bChirho
implicitChirho valueChirho = valueChirho
explicitChirho :: forall opChirho aChirho bChirho. aChirho `opChirho` bChirho -> aChirho `opChirho` bChirho
explicitChirho valueChirho = valueChirho
prefixChirho :: opChirho aChirho bChirho -> opChirho aChirho bChirho
prefixChirho valueChirho = valueChirho
contextChirho :: (Eq bChirho, Eq aChirho) => aChirho -> bChirho -> Bool
contextChirho leftChirho rightChirho = leftChirho == leftChirho && rightChirho == rightChirho
main :: IO ()
main = do
  print (fst (implicitChirho @Int @(,) @Bool (42, True)))
  print (snd (explicitChirho @(,) @Bool @Int (False, 7)))
  print (fst (prefixChirho @(,) @Int @Bool (11, True)))
  print (contextChirho @Bool @Int 9 False)
"#;

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

#[test]
fn signature_type_arguments_follow_source_order_on_every_engine_chirho() {
    assert_execution_chirho(SOURCE_ORDER_CHIRHO, "42\n7\n11\nTrue\n");
}

fn assert_execution_chirho(source_chirho: &str, expected_chirho: &str) {
    use haskelujah_test_harness_chirho::native_chirho::{
        NativeBackendChirho, native_round_trip_chirho,
    };

    let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "TypeOrderChirho.hs",
        None,
    )
    .expect("the GHC-accepted signature applications must compile and run");
    assert_eq!(machine_chirho.io_output_chirho, expected_chirho);
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        let (status_chirho, output_chirho) =
            native_round_trip_chirho(source_chirho, backend_chirho, "")
                .expect("bounded native compile, link and execution must succeed");
        assert_eq!(status_chirho, 0, "{backend_chirho:?}");
        assert_eq!(output_chirho, expected_chirho, "{backend_chirho:?}");
    }
}

#[test]
fn type_operator_fixity_agrees_with_constructor_values_on_every_engine_chirho() {
    // The unchanged source was run under GHC 9.14.1: 3 / True / 'c'.
    let source_chirho = r#"{-# LANGUAGE TypeOperators #-}
module Main where
infixl 4 :*:
infixl 3 :+:
data aChirho :*: bChirho = ProductChirho aChirho bChirho
data aChirho :+: bChirho = SumChirho aChirho bChirho
valueChirho :: Int :*: Bool :+: Char
valueChirho = SumChirho (ProductChirho 3 True) 'c'
main :: IO ()
main = case valueChirho of
  SumChirho (ProductChirho numberChirho flagChirho) letterChirho -> do
    print numberChirho
    print flagChirho
    print letterChirho
"#;
    assert_execution_chirho(source_chirho, "3\nTrue\n'c'\n");
}

#[test]
fn class_parameters_and_scoped_and_rank_n_binders_keep_their_type_arguments_chirho() {
    // This is a typechecking contract. The analogous custom class-method call
    // has a separate pre-existing STG dispatch limitation, so it is not being
    // presented as an execution test. GHC 9.14.1 accepts and runs this source.
    let source_chirho = r#"{-# LANGUAGE TypeApplications, ExplicitForAll, ScopedTypeVariables, RankNTypes #-}
module Main where
class KeepChirho aChirho where
  keepChirho :: bChirho -> aChirho -> bChirho
instance KeepChirho Int where
  keepChirho valueChirho _ = valueChirho
outerChirho :: forall aChirho. aChirho -> aChirho
outerChirho valueChirho = innerChirho @Bool True
  where
    innerChirho :: bChirho -> aChirho
    innerChirho _ = valueChirho
polyChirho :: (forall qChirho. qChirho -> qChirho) -> aChirho -> aChirho
polyChirho functionChirho valueChirho = functionChirho valueChirho
main :: IO ()
main = do
  print (keepChirho @Int @Bool True 3)
  print (outerChirho @Int 7)
  print (polyChirho @Char id 'x')
"#;
    haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ScopeOrderChirho.hs",
    )
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn explicit_type_arguments_do_not_accept_reversed_operand_types_chirho() {
    let source_chirho = SOURCE_ORDER_CHIRHO.replace(
        "implicitChirho @Int @(,) @Bool (42, True)",
        "implicitChirho @Int @(,) @Bool (True, 42)",
    );
    let errors_chirho = haskelujah_driver::typecheck_source_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "TypeOrderMismatchChirho.hs",
    )
    .map(|_| ())
    .expect_err("the explicitly Int first component cannot be Bool");
    let text_chirho = format!("{errors_chirho}");
    assert!(
        text_chirho.contains("type mismatch")
            && text_chirho.contains("Int")
            && text_chirho.contains("Bool"),
        "{text_chirho}"
    );
}
