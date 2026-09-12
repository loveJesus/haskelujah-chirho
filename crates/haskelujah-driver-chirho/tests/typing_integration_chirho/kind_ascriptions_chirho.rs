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
fn function_constructor_instance_dispatch_executes_chirho() {
    let source_chirho = r#"{-# LANGUAGE KindSignatures, FlexibleInstances #-}
module Main where
import Data.Kind (Type)
class CatChirho (kChirho :: Type -> Type -> Type) where
  composeChirho :: kChirho aChirho bChirho -> kChirho xChirho aChirho -> kChirho xChirho bChirho
instance CatChirho (->) where
  composeChirho = (.)
main = print (composeChirho (+2) (*2) (20 :: Int))
"#;
    assert_execution_chirho(source_chirho, "42\n");
}

#[test]
fn partial_function_constructor_ascriptions_keep_their_domain_chirho() {
    let source_chirho = "{-# LANGUAGE KindSignatures, FlexibleInstances #-}\nmodule ArrowChirho where\nimport Data.Kind (Type)\nclass MarkerChirho aChirho\ninstance MarkerChirho (((->) Int :: Type -> Type) Bool)\n";
    typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ArrowChirho.hs",
    )
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
    let wrong_chirho = format!(
        "{{-# LANGUAGE DataKinds #-}}\n{}",
        source_chirho.replace(":: Type -> Type", ":: Bool -> Type")
    );
    let error_chirho = typecheck_source_chirho(
        &wrong_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongArrowChirho.hs",
    )
    .map(|_result_chirho| ())
    .expect_err("the arrow constructor cannot acquire a Bool domain kind");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}

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

fn binder_syntax_source_chirho(body_chirho: &str) -> String {
    format!(
        "{{-# LANGUAGE DataKinds, KindSignatures, PolyKinds, RankNTypes #-}}\nmodule BinderChirho where\nimport Data.Kind (Type)\nimport GHC.Exts (TYPE, RuntimeRep(..), Levity(..))\ndata ProxyChirho (aChirho :: kChirho) = ProxyChirho\n{body_chirho}"
    )
}

#[test]
fn runtime_classifier_exports_obey_explicit_import_visibility_chirho() {
    // Both accepted and missing-import controls were independently checked by GHC9.14.1.
    for (import_chirho, classifier_chirho, visible_chirho) in [
        (
            "import GHC.Types (RuntimeRep(..), TYPE)",
            "TYPE 'IntRep",
            true,
        ),
        (
            "import qualified GHC.Types as RepresentationChirho",
            "RepresentationChirho.TYPE 'RepresentationChirho.IntRep",
            true,
        ),
        ("import GHC.Types (RuntimeRep(..))", "TYPE 'IntRep", false),
        ("import GHC.Types hiding (TYPE)", "TYPE 'IntRep", false),
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE DataKinds, KindSignatures, ExplicitForAll #-}}\nmodule RuntimeExportChirho where\n{import_chirho}\nfChirho :: forall (aChirho :: {classifier_chirho}). aChirho -> aChirho\nfChirho xChirho = xChirho\n"
        );
        let result_chirho = typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "RuntimeExportChirho.hs",
        );
        if visible_chirho {
            result_chirho.unwrap_or_else(|error_chirho| panic!("{source_chirho}\n{error_chirho}"));
        } else {
            let error_chirho = result_chirho
                .map(|_result_chirho| ())
                .expect_err("a wired-in classifier is not globally imported");
            assert!(
                error_chirho
                    .to_string()
                    .contains("type not in scope: `TYPE`"),
                "{error_chirho}"
            );
        }
    }
}

#[test]
fn binder_classifiers_keep_promoted_compound_and_literal_syntax_chirho() {
    // Each source is independently accepted by GHC9.14.1; frozen250b22fb
    // dropped its annotated binder and reported E0101 on the bound variable.
    for body_chirho in [
        "fChirho :: forall (vChirho :: Levity) (aChirho :: TYPE ('BoxedRep vChirho)). (Int -> aChirho) -> aChirho\nfChirho gChirho = gChirho 42\n",
        "fChirho :: forall (pChirho :: (Bool, Bool)). ProxyChirho pChirho -> ProxyChirho pChirho\nfChirho xChirho = xChirho\n",
        "fChirho :: forall (rChirho :: RuntimeRep) (aChirho :: TYPE ('TupleRep '[rChirho])). ProxyChirho aChirho -> ProxyChirho aChirho\nfChirho xChirho = xChirho\n",
        "fChirho :: forall (aChirho :: ProxyChirho 0). ProxyChirho aChirho -> ProxyChirho aChirho\nfChirho xChirho = xChirho\n",
    ] {
        let source_chirho = binder_syntax_source_chirho(body_chirho);
        typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "BinderChirho.hs",
        )
        .unwrap_or_else(|error_chirho| panic!("{source_chirho}\n{error_chirho}"));
    }
}

#[test]
fn promoted_binder_classifiers_reject_wrong_representation_arguments_chirho() {
    for body_chirho in [
        "fChirho :: forall (aChirho :: TYPE ('BoxedRep 'True)). ProxyChirho aChirho -> ProxyChirho aChirho\nfChirho xChirho = xChirho\n",
        "fChirho :: forall (aChirho :: TYPE ('TupleRep '[ 'True ])). ProxyChirho aChirho -> ProxyChirho aChirho\nfChirho xChirho = xChirho\n",
    ] {
        let source_chirho = binder_syntax_source_chirho(body_chirho);
        let error_chirho = typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "WrongBinderChirho.hs",
        )
        .map(|_result_chirho| ())
        .expect_err("GHC rejects a Bool supplied as Levity or RuntimeRep");
        assert!(
            error_chirho.to_string().contains("kind mismatch"),
            "{error_chirho}"
        );
        assert!(
            !error_chirho
                .to_string()
                .contains("type variable not in scope"),
            "{error_chirho}"
        );
    }
}

#[test]
fn constructor_contexts_do_not_become_runtime_fields_chirho() {
    // GHC9.14.1 produces 42/7 for this exact source. This proves field layout,
    // not storage/use of implicit evidence, which these AST constructors lack.
    let source_chirho = "{-# LANGUAGE ImplicitParams, ExistentialQuantification #-}\nmodule Main where\ndata RecordChirho = (?flagChirho :: Bool) => RecordChirho { fieldChirho :: Int }\ndata PlainChirho = (?flagChirho :: Bool) => PlainChirho Int\nreadChirho (PlainChirho nChirho) = nChirho\nmain = let ?flagChirho = True in do\n  print (fieldChirho (RecordChirho 42))\n  print (readChirho (PlainChirho 7))\n";
    assert_execution_chirho(source_chirho, "42\n7\n");
    let wrong_chirho = source_chirho.replace("PlainChirho 7", "PlainChirho True");
    let error_chirho = typecheck_source_chirho(
        &wrong_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongConstructorContextChirho.hs",
    )
    .map(|_result_chirho| ())
    .expect_err("the supplied runtime field still requires Int");
    assert!(
        error_chirho.to_string().contains("type mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn implicit_parameter_contexts_do_not_share_type_variable_classifiers_chirho() {
    // The signature alone is valid; GHC9.14.1 rejects the corpus-style coerce
    // body, which is a separate role/evidence question and is not asserted here.
    let source_chirho = "{-# LANGUAGE ImplicitParams, RankNTypes #-}\nmodule PayloadSignatureChirho where\nnewtype PayloadChirho = PayloadChirho Int\nfChirho :: ((?flagChirho :: PayloadChirho) => Int) -> ((?flagChirho :: Int) => Int)\nfChirho _ = 1\n";
    typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "PayloadSignatureChirho.hs",
    )
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn implicit_parameter_payload_names_and_lifted_kinds_are_checked_chirho() {
    for (payload_chirho, diagnostic_chirho) in [
        ("NoSuchPayloadChirho", "NoSuchPayloadChirho"),
        ("Maybe", "kind mismatch"),
        ("Int#", "kind mismatch"),
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE ImplicitParams, MagicHash #-}}\nmodule PayloadChirho where\nimport GHC.Exts (Int#)\nfChirho :: (?flagChirho :: {payload_chirho}) => Int\nfChirho = 1\n"
        );
        let error_chirho = typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "InvalidPayloadChirho.hs",
        )
        .map(|_result_chirho| ())
        .expect_err("implicit parameter payloads must be named lifted types");
        assert!(
            error_chirho.to_string().contains(diagnostic_chirho),
            "{error_chirho}"
        );
    }
}

fn flat_producer_source_chirho(body_chirho: &str) -> String {
    format!(
        "{{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, MultiParamTypeClasses, FlexibleContexts, FlexibleInstances, TypeFamilies, TypeOperators, UndecidableSuperClasses #-}}\nmodule FlatProducerChirho where\nimport Data.Kind (Type)\nimport GHC.TypeLits\n{body_chirho}"
    )
}

#[test]
fn instance_arguments_preserve_promotion_like_signatures_chirho() {
    let source_chirho = flat_producer_source_chirho(
        "data StackChirho (layersChirho :: [Type]) (tChirho :: Type -> Type) = StackChirho\nclass CChirho aChirho\nokChirho :: StackChirho '[] Maybe\nokChirho = StackChirho\ninstance CChirho (StackChirho '[] Maybe)\n",
    );
    typecheck_source_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "PromotedInstanceChirho.hs",
    )
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
    let invalid_chirho = flat_producer_source_chirho(
        "data StackChirho (layersChirho :: [Type]) (tChirho :: Type -> Type) = StackChirho\nclass CChirho aChirho\ninstance CChirho (StackChirho [] Maybe)\n",
    );
    let error_chirho = typecheck_source_chirho(
        &invalid_chirho,
        &mut SourceMapChirho::new_chirho(),
        "OrdinaryListInstanceChirho.hs",
    )
    .map(|_result_chirho| ())
    .expect_err("ordinary [] is not a promoted empty list");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn superclass_ascriptions_keep_literal_arguments_chirho() {
    for body_chirho in [
        "class ((CmpSymbol symbolChirho symbolChirho :: Ordering) ~ orderingChirho) => VariableChirho (symbolChirho :: Symbol) orderingChirho\n",
        "class ((CmpSymbol \"a\" \"a\" :: Ordering) ~ orderingChirho) => LiteralChirho orderingChirho\nclass ((CmpNat 1 1 :: Ordering) ~ orderingChirho) => NaturalChirho orderingChirho\n",
    ] {
        let source_chirho = flat_producer_source_chirho(body_chirho);
        typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "LiteralContextChirho.hs",
        )
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
    }
    let source_chirho = flat_producer_source_chirho(
        "class ((CmpSymbol \"a\" \"a\" :: Bool) ~ orderingChirho) => LiteralChirho orderingChirho\n",
    );
    let error_chirho = typecheck_source_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongLiteralContextChirho.hs",
    )
    .map(|_result_chirho| ())
    .expect_err("the saturated result is Ordering, not Bool");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}
