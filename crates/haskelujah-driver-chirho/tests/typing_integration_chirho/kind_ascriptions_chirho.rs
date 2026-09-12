// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type-pattern ascriptions must bind actual matching indices, not invented RHS names.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::{SourceMapChirho, assert_execution_chirho};
use haskelujah_driver::typecheck_source_chirho;

const ASCRIBED_KEY_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/AscribedKeyChirho.hs"
));
const ASCRIBED_SPINE_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/AscribedSpineChirho.hs"
));

#[test]
fn annotated_heads_keep_the_same_solved_indices_as_bare_heads_chirho() {
    // The same source was accepted and executed by GHC9.14.1.
    assert_execution_chirho(ASCRIBED_SPINE_CHIRHO, "ascribed spines\n");
}

#[test]
fn an_ascription_owns_its_visible_quantifiers_chirho() {
    let polymorphic_chirho = format!(
        "{ASCRIBED_SPINE_CHIRHO}\n\
explicitChirho :: EqualChirho ((ProxyChirho :: forall keyChirho. keyChirho -> Type) @Bool 'True) (ProxyChirho 'True)\n\
explicitChirho = ReflChirho\n"
    );
    typecheck_source_chirho(
        &polymorphic_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ExplicitAscriptionChirho.hs",
    )
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));

    let hidden_chirho = format!(
        "{ASCRIBED_SPINE_CHIRHO}\n\
type HiddenChirho = (ProxyChirho :: Bool -> Type) @Bool 'True\n"
    );
    let error_chirho = typecheck_source_chirho(
        &hidden_chirho,
        &mut SourceMapChirho::new_chirho(),
        "HiddenAscriptionChirho.hs",
    )
    .map(|_result_chirho| ())
    .expect_err("a monomorphic ascription does not re-export the provider's quantifiers");
    assert!(
        error_chirho.to_string().contains("@ application"),
        "{error_chirho}"
    );
}

#[test]
fn polymorphic_ascriptions_cannot_generalize_a_fixed_kind_chirho() {
    for expression_chirho in [
        "Maybe :: forall keyChirho. keyChirho -> Type",
        "ProxyChirho @Bool :: forall keyChirho. keyChirho -> Type",
    ] {
        let source_chirho =
            format!("{ASCRIBED_SPINE_CHIRHO}\ntype BadChirho = ({expression_chirho})\n");
        let error_chirho = typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "FalsePolymorphicAscriptionChirho.hs",
        )
        .map(|_result_chirho| ())
        .expect_err("a universally quantified classifier is checked, not specialized");
        assert!(
            error_chirho.to_string().contains("kind mismatch"),
            "{error_chirho}"
        );
    }
}

#[test]
fn pattern_ascription_keys_select_distinct_family_results_chirho() {
    // GHC9.14.1 executes this exact source to the independently specified output.
    // Each Refl requires a separate reduction, not just an accepted declaration.
    assert_execution_chirho(ASCRIBED_KEY_CHIRHO, "distinct keys\n");
    for (source_chirho, name_chirho) in [
        (
            ASCRIBED_KEY_CHIRHO.replace("))) Int", "))) Bool"),
            "ContradictoryAscribedKeyChirho.hs",
        ),
        (
            ASCRIBED_KEY_CHIRHO.replace("= keyChirho\n", "= unknownKeyChirho\n"),
            "UnboundAscribedKeyChirho.hs",
        ),
    ] {
        assert_ne!(
            source_chirho, ASCRIBED_KEY_CHIRHO,
            "mutation must reach the source"
        );
        typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            name_chirho,
        )
        .map(|_result_chirho| ())
        .expect_err("annotation binding cannot validate a contradiction or a fresh RHS name");
    }
}

#[test]
fn type_ascription_checks_the_written_classifier_chirho() {
    let source_chirho = "{-# LANGUAGE KindSignatures #-}\n\
module WrongAscriptionChirho where\n\
import Data.Kind (Type)\n\
type AliasChirho = (Int :: Type -> Type)\n";
    let error_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongAscriptionChirho.hs",
    )
    .map(|_result_chirho| ())
    .expect_err("Int does not have the written Type -> Type kind");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn type_ascription_names_are_checked_after_the_double_colon_chirho() {
    let source_chirho = "{-# LANGUAGE KindSignatures #-}\n\
module MissingAscriptionChirho where\n\
type AliasChirho = (Int :: MissingClassifierChirho)\n";
    let error_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "MissingAscriptionChirho.hs",
    )
    .map(|_result_chirho| ())
    .expect_err("the kind child must reach naming");
    assert!(
        error_chirho.to_string().contains("MissingClassifierChirho"),
        "{error_chirho}"
    );
}

fn assert_nested_ascription_chirho(body_chirho: &str) {
    let source_chirho = format!(
        "{{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, FlexibleInstances, ExplicitForAll #-}}\n\
module NestedChirho where\nimport Data.Kind (Type)\n{body_chirho}"
    );
    typecheck_source_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "NestedChirho.hs",
    )
    .unwrap_or_else(|error_chirho| panic!("{source_chirho}\n{error_chirho}"));
    let wrong_chirho = source_chirho.replace(":: Type", ":: Bool");
    assert_ne!(source_chirho, wrong_chirho);
    let error_chirho = typecheck_source_chirho(
        &wrong_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongNestedChirho.hs",
    )
    .map(|_result_chirho| ())
    .expect_err("the nested kind is part of the written contract");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn bare_alias_result_ascriptions_are_checked_chirho() {
    assert_nested_ascription_chirho(
        "type BareChirho = Int :: Type\nsentinelChirho :: Int\nsentinelChirho = 1\n",
    );
}

#[test]
fn instance_head_ascriptions_are_checked_chirho() {
    assert_nested_ascription_chirho("class CChirho aChirho\ninstance CChirho (Int :: Type)\n");
}

#[test]
fn data_binder_nested_ascriptions_are_checked_chirho() {
    assert_nested_ascription_chirho("data DChirho (aChirho :: (kChirho :: Type)) = DChirho\n");
}

#[test]
fn forall_binder_nested_ascriptions_are_checked_chirho() {
    assert_nested_ascription_chirho(
        "fChirho :: forall kChirho (aChirho :: (kChirho :: Type)). ()\nfChirho = ()\n",
    );
}
