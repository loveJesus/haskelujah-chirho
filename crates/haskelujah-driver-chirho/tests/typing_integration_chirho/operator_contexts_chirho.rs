// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Flat and structured equality operands share Haskell operator fixity.
//! Workflow: language-features-chirho/flat-type-syntax-chirho.
use super::{assert_compile_success_chirho, assert_execution_chirho, assert_kind_error_chirho};

// Exact positive/negative sources independently checked by GHC 9.14.1 in
// ascriptions-chirho/operators-chirho/reference-chirho.jsonl, including parentheses.
const PREFIX_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, KindSignatures, TypeFamilies, TypeOperators, MultiParamTypeClasses, FlexibleContexts, FlexibleInstances, UndecidableSuperClasses #-}
module OperatorChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy(..))
"#;

const SIGNATURE_CHIRHO: &str = "probeChirho :: (xsChirho ~ Int ': restChirho) => Proxy xsChirho -> Proxy restChirho -> Int\nprobeChirho _ _ = 42\n";

#[test]
fn flat_superclass_equation_uses_the_entire_family_application_chirho() {
    let family_chirho = "type family (xsChirho :: [Type]) :++ (xChirho :: Type) :: [Type] where\n  xsChirho :++ xChirho = xChirho ': xsChirho\n";
    for context_chirho in [
        "xsChirho :++ xChirho ~ ysChirho",
        "(xsChirho :++ xChirho) ~ ysChirho",
    ] {
        assert_compile_success_chirho(
            "FlatEqualityChirho.hs",
            &format!(
                "{PREFIX_CHIRHO}{family_chirho}class {context_chirho} => PushChirho xsChirho xChirho ysChirho\n"
            ),
        );
    }
}

#[test]
fn promoted_cons_is_the_equality_operand_not_the_constraint_head_chirho() {
    assert_compile_success_chirho(
        "SignatureEqualityChirho.hs",
        &format!("{PREFIX_CHIRHO}{SIGNATURE_CHIRHO}"),
    );
    assert_compile_success_chirho(
        "SignatureEqualityParenthesizedChirho.hs",
        &format!(
            "{PREFIX_CHIRHO}{}",
            SIGNATURE_CHIRHO.replace("Int ': restChirho", "(Int ': restChirho)")
        ),
    );
    // This was accepted while the malformed context acquired a synthetic `?`
    // head. The recovered equality must inspect the invalid promoted-list tail.
    assert_kind_error_chirho(&format!(
        "{PREFIX_CHIRHO}probeChirho :: (xsChirho ~ Int ': 'True) => Proxy xsChirho -> Int\nprobeChirho _ = 42\n"
    ));
}

#[test]
fn equality_grouping_executes_the_same_source_on_every_engine_chirho() {
    let source_chirho = format!(
        "{PREFIX_CHIRHO}{SIGNATURE_CHIRHO}main = print (probeChirho (Proxy :: Proxy '[Int, Bool]) (Proxy :: Proxy '[Bool]))\n"
    ).replace("module OperatorChirho where", "module Main where");
    assert_execution_chirho(&source_chirho, "42\n");
    let with_unused_chirho = source_chirho.replace(
        "probeChirho ::",
        "unusedChirho :: Proxy '[]\nunusedChirho = Proxy\nprobeChirho ::",
    );
    assert_execution_chirho(&with_unused_chirho, "42\n");
}

#[test]
fn expression_promoted_cons_does_not_require_a_signature_occurrence_chirho() {
    // GHC produces 42 with and without the unused cons signature. Before the
    // catalog repair only the latter signature made the expression typecheck.
    for unused_chirho in [
        "",
        "unusedChirho :: Proxy (Int ': '[])\nunusedChirho = Proxy\n",
    ] {
        let source_chirho = format!(
            "{PREFIX_CHIRHO}{unused_chirho}keepChirho :: Proxy '[Int] -> Proxy '[Int]\nkeepChirho valueChirho = valueChirho\nmain = print (const (42 :: Int) (keepChirho (Proxy :: Proxy (Int ': '[]))))\n"
        ).replace("module OperatorChirho where", "module Main where");
        assert_execution_chirho(&source_chirho, "42\n");
    }
}
