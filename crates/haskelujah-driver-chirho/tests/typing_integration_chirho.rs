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
fn higher_kinded_forall_shadowing_runs_on_every_engine_chirho() {
    // This identical source independently ran under GHC 9.14.1.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
retainChirho :: fChirho Int -> (forall fChirho. fChirho -> fChirho) -> fChirho Int
retainChirho valueChirho _ = valueChirho
retainLaterChirho :: (forall fChirho. fChirho -> fChirho) -> fChirho Int -> fChirho Int
retainLaterChirho _ valueChirho = valueChirho
main :: IO ()
main = do
  print (sum (retainChirho [42] id))
  print (sum (retainLaterChirho id [7]))
"#;
    assert_execution_chirho(source_chirho, "42\n7\n");
}

#[test]
fn required_higher_kinded_forall_shadowing_runs_on_every_engine_chirho() {
    // This identical source independently ran under GHC 9.14.1.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, RequiredTypeArguments #-}
module Main where
retainChirho :: fChirho Int -> (forall fChirho -> fChirho -> fChirho) -> fChirho Int
retainChirho valueChirho _ = valueChirho
retainLaterChirho :: (forall fChirho -> fChirho -> fChirho) -> fChirho Int -> fChirho Int
retainLaterChirho _ valueChirho = valueChirho
requiredIdChirho :: forall aChirho -> aChirho -> aChirho
requiredIdChirho typeChirho valueChirho = valueChirho
main :: IO ()
main = do
  print (sum (retainChirho [42] requiredIdChirho))
  print (sum (retainLaterChirho requiredIdChirho [7]))
"#;
    assert_execution_chirho(source_chirho, "42\n7\n");
}

#[test]
fn kind_annotated_forall_shadowing_runs_on_every_engine_chirho() {
    // This identical source independently ran under GHC 9.14.1.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, PolyKinds, ScopedTypeVariables, TypeApplications #-}
module Main where
import Data.Kind (Type)
data ProxyChirho (aChirho :: kChirho) = ProxyChirho
retainChirho :: forall kChirho (fChirho :: kChirho -> Type) (aChirho :: kChirho). fChirho aChirho -> (forall kChirho (aChirho :: kChirho). ProxyChirho aChirho -> ProxyChirho aChirho) -> fChirho aChirho
retainChirho valueChirho _ = valueChirho
proxyIdChirho :: forall kChirho (aChirho :: kChirho). ProxyChirho aChirho -> ProxyChirho aChirho
proxyIdChirho valueChirho = valueChirho
main :: IO ()
main = do
  print (sum (retainChirho @Type @[] @Int [42] proxyIdChirho))
  print (maybe False id (retainChirho @Type @Maybe @Bool (Just True) proxyIdChirho))
"#;
    assert_execution_chirho(source_chirho, "42\nTrue\n");
}

#[test]
fn quantified_constraint_kind_scope_runs_on_every_engine_chirho() {
    // This identical source independently ran under GHC 9.14.1.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE QuantifiedConstraints, FlexibleInstances, UndecidableInstances, MonoLocalBinds #-}
module Main where
class WitnessChirho aChirho
instance WitnessChirho aChirho
retainChirho :: (forall fChirho. WitnessChirho (fChirho Int)) => fChirho -> fChirho
retainChirho valueChirho = valueChirho
main :: IO ()
main = do
  print (retainChirho (42 :: Int))
  print (retainChirho True)
"#;
    assert_execution_chirho(source_chirho, "42\nTrue\n");
}

#[test]
fn a_shadow_does_not_hide_a_genuine_kind_mismatch_chirho() {
    // GHC 9.14.1 reports GHC-83865: a Type variable cannot also take an argument.
    let source_chirho = r#"{-# LANGUAGE RankNTypes #-}
module BadKindScopeChirho where
badChirho :: (forall fChirho. fChirho -> fChirho Int) -> Bool
badChirho _ = True
"#;
    let errors_chirho = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "BadKindScopeChirho.hs",
    )
    .map(|_| ())
    .expect_err("the same bound variable cannot have both Type and Type -> Type kinds");
    let text_chirho = errors_chirho.to_string();
    assert!(text_chirho.contains("kind mismatch"), "{text_chirho}");
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

#[test]
fn nested_forall_shadowing_preserves_outer_polymorphism_on_every_engine_chirho() {
    // Independently run, unchanged, under GHC 9.14.1. The outer variable is
    // encountered before a shadow, after one, first inside another binder,
    // and in the enclosing ScopedTypeVariables environment respectively.
    let source_chirho = r#"{-# LANGUAGE RankNTypes, ScopedTypeVariables, TypeApplications #-}
module Main where
preserveChirho :: aChirho -> (forall aChirho. aChirho -> aChirho) -> aChirho
preserveChirho valueChirho _ = valueChirho
preserveLaterChirho :: (forall aChirho. aChirho -> aChirho) -> aChirho -> aChirho
preserveLaterChirho _ valueChirho = valueChirho
retainFreeChirho :: (forall boundChirho. freeChirho -> boundChirho -> freeChirho) -> freeChirho -> freeChirho
retainFreeChirho functionChirho valueChirho = functionChirho valueChirho ()
outerChirho :: forall aChirho. aChirho -> aChirho
outerChirho valueChirho = innerChirho id valueChirho
  where
    innerChirho :: (forall aChirho. aChirho -> aChirho) -> aChirho -> aChirho
    innerChirho _ resultChirho = resultChirho
main :: IO ()
main = do
  print (preserveChirho (42 :: Int) id)
  print (preserveChirho True id)
  print (preserveLaterChirho id (7 :: Int))
  print (preserveLaterChirho id False)
  print (retainFreeChirho @Int const 11)
  print (retainFreeChirho @Bool const True)
  print (outerChirho @Int 13)
  print (outerChirho @Bool False)
"#;
    assert_execution_chirho(source_chirho, "42\nTrue\n7\nFalse\n11\nTrue\n13\nFalse\n");
}

#[test]
fn required_forall_shadowing_preserves_outer_polymorphism_on_every_engine_chirho() {
    // The same source independently produced this output under GHC 9.14.1.
    let source_chirho = r#"{-# LANGUAGE RankNTypes, RequiredTypeArguments #-}
module Main where
preserveChirho :: aChirho -> (forall aChirho -> aChirho -> aChirho) -> aChirho
preserveChirho valueChirho _ = valueChirho
preserveLaterChirho :: (forall aChirho -> aChirho -> aChirho) -> aChirho -> aChirho
preserveLaterChirho _ valueChirho = valueChirho
requiredIdChirho :: forall aChirho -> aChirho -> aChirho
requiredIdChirho typeChirho valueChirho = valueChirho
main :: IO ()
main = do
  print (preserveChirho (42 :: Int) requiredIdChirho)
  print (preserveChirho True requiredIdChirho)
  print (preserveLaterChirho requiredIdChirho (7 :: Int))
  print (preserveLaterChirho requiredIdChirho False)
"#;
    assert_execution_chirho(source_chirho, "42\nTrue\n7\nFalse\n");
}

#[test]
fn signature_scope_boundaries_preserve_type_application_on_every_engine_chirho() {
    // GHC 9.14.1 oracle: parentheses quantify once, explicit local binders
    // shadow enclosing ones, and consecutive groups retain distinct identities.
    let source_chirho = r#"{-# LANGUAGE RankNTypes, ScopedTypeVariables, TypeApplications, AllowAmbiguousTypes #-}
module Main where
parenthesizedChirho :: (forall aChirho. aChirho -> aChirho)
parenthesizedChirho valueChirho = valueChirho
outerShadowChirho :: forall aChirho. aChirho -> (Bool, aChirho)
outerShadowChirho valueChirho = (innerChirho True, valueChirho)
  where
    innerChirho :: forall aChirho. aChirho -> aChirho
    innerChirho resultChirho = resultChirho
chainChirho :: forall aChirho. forall aChirho. aChirho -> aChirho
chainChirho valueChirho = valueChirho
leadingChirho :: forall aChirho. Eq aChirho => forall aChirho. Eq aChirho => aChirho -> aChirho
leadingChirho valueChirho = valueChirho
main :: IO ()
main = do
  print (parenthesizedChirho @Int 42)
  print (parenthesizedChirho @Bool True)
  print (fst (outerShadowChirho @Int 7))
  print (snd (outerShadowChirho @Int 7))
  print (chainChirho @Int @Bool False)
  print (leadingChirho @Int @Bool True)
"#;
    assert_execution_chirho(source_chirho, "42\nTrue\nTrue\n7\nFalse\nTrue\n");
}

#[test]
fn result_forall_predicates_keep_their_shadowed_scope_chirho() {
    let source_chirho = r#"{-# LANGUAGE RankNTypes, TypeApplications #-}
module Main where
equalLaterChirho :: Int -> forall aChirho. Eq aChirho => aChirho -> aChirho -> Bool
equalLaterChirho _ leftChirho rightChirho = leftChirho == rightChirho
shadowLaterChirho :: forall aChirho. aChirho -> forall aChirho. Eq aChirho => aChirho -> aChirho -> Bool
shadowLaterChirho _ leftChirho rightChirho = leftChirho == rightChirho
main :: IO ()
main = do
  print (equalLaterChirho 0 @Int 3 3)
  print (equalLaterChirho 0 @Bool True False)
  print (shadowLaterChirho @Int 0 @Bool True True)
  print (shadowLaterChirho @Bool True @Int 3 7)
"#;
    assert_execution_chirho(source_chirho, "True\nFalse\nTrue\nFalse\n");
}

#[test]
fn only_the_syntactically_outermost_forall_scopes_over_the_definition_chirho() {
    // GHC 9.14.1 rejects each local signature below: parentheses and a second
    // forall group do not bring those signature variables into the definition.
    for source_chirho in [
        r#"{-# LANGUAGE RankNTypes, ScopedTypeVariables #-}
module ParenScopeChirho where
parenthesizedChirho :: (forall aChirho. aChirho -> aChirho)
parenthesizedChirho valueChirho = localChirho
  where
    localChirho :: aChirho
    localChirho = valueChirho
"#,
        r#"{-# LANGUAGE RankNTypes, ScopedTypeVariables #-}
module LaterScopeChirho where
laterChirho :: forall aChirho. forall bChirho. aChirho -> bChirho -> bChirho
laterChirho _ valueChirho = localChirho
  where
    localChirho :: bChirho
    localChirho = valueChirho
"#,
    ] {
        let errors_chirho = haskelujah_driver::typecheck_source_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "SignatureScopeChirho.hs",
        )
        .map(|_| ())
        .expect_err("a non-scoping signature binder cannot capture the local type variable");
        let text_chirho = errors_chirho.to_string();
        assert!(
            text_chirho.contains("type mismatch") && text_chirho.contains("rigid"),
            "{text_chirho}"
        );
    }
}

#[test]
fn nested_forall_result_cannot_escape_as_an_arbitrary_outer_type_chirho() {
    // GHC 9.14.1 rejects this with GHC-25897: outer aChirho is rigid, not Bool.
    let source_chirho = r#"{-# LANGUAGE RankNTypes #-}
module ForallLeakChirho where
leakChirho :: aChirho -> (forall aChirho. aChirho -> aChirho) -> aChirho
leakChirho _ functionChirho = functionChirho True
"#;
    let errors_chirho = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ForallLeakChirho.hs",
    )
    .map(|_| ())
    .expect_err("an arbitrary outer a cannot be implemented with Bool");
    let text_chirho = errors_chirho.to_string();
    assert!(
        text_chirho.contains("type mismatch")
            && text_chirho.contains("Bool")
            && text_chirho.contains("rigid"),
        "{text_chirho}"
    );
}

#[test]
fn prefix_list_constructor_in_a_record_field_executes_chirho() {
    // GHC 9.14.1 independently prints kept; [] must not acquire a fake element.
    let source_chirho = r#"module Main where
data HolderChirho = HolderChirho { fieldChirho :: [] Char }
main :: IO ()
main = putStrLn (fieldChirho (HolderChirho "kept"))
"#;
    assert_execution_chirho(source_chirho, "kept\n");
}

#[test]
fn nested_list_function_in_a_record_field_executes_chirho() {
    // GHC 9.14.1 independently prints 42; the inner ] cannot close the outer list.
    let source_chirho = r#"module Main where
data HolderChirho = HolderChirho { fieldChirho :: [[Int] -> Int] }
main :: IO ()
main = case fieldChirho (HolderChirho [\_ -> 42]) of
  firstChirho : _ -> print (firstChirho [1, 2])
  [] -> print (0 :: Int)
"#;
    assert_execution_chirho(source_chirho, "42\n");
}

#[test]
fn gadt_record_list_types_and_following_signature_execute_chirho() {
    let source_chirho = r#"{-# LANGUAGE GADTs #-}
module Main where
data HolderChirho where
  HolderChirho :: { textChirho :: [] Char, functionsChirho :: [[Int] -> Int] } -> HolderChirho
main :: IO ()
main = do
  let holderChirho = HolderChirho "kept" [\_ -> 42]
  putStrLn (textChirho holderChirho)
  case functionsChirho holderChirho of
    firstChirho : _ -> print (firstChirho [1, 2])
    [] -> print (0 :: Int)
"#;
    assert_execution_chirho(source_chirho, "kept\n42\n");
}

#[test]
fn bare_list_constructor_is_not_a_saturated_record_field_type_chirho() {
    for declaration_chirho in [
        "data HolderChirho = HolderChirho { fieldChirho :: [] }",
        "data HolderChirho where\n  HolderChirho :: { fieldChirho :: [] } -> HolderChirho",
        "data HolderChirho = HolderChirho { fieldChirho :: [] Int Bool }",
        "data HolderChirho where\n  HolderChirho :: { fieldChirho :: [] Int Bool } -> HolderChirho",
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE GADTs #-}}\nmodule UnsaturatedListChirho where\n{declaration_chirho}\n"
        );
        let error_chirho = haskelujah_driver::typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "UnsaturatedListChirho.hs",
        )
        .map(|_| ())
        .expect_err("GHC-83865: [] needs exactly one argument before it is a field type");
        let message_chirho = error_chirho.to_string();
        assert!(message_chirho.contains("kind mismatch"), "{message_chirho}");
    }
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
