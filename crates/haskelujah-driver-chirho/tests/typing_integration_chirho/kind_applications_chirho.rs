// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source kind-argument contracts, independently checked with GHC 9.14.1.
use super::{assert_compile_success_chirho, assert_execution_chirho};
use haskelujah_driver::typecheck_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

const FAMILY_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/FamilyKindApplicationsChirho.hs"
));
const ORDER_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/KindBinderOrderChirho.hs"
));
const INFERRED_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/InferredKindChirho.hs"
));
const AUTO_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/AutoKindChirho.hs"
));
const PHANTOM_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/PhantomKindChirho.hs"
));
const CONSTRUCTOR_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/ConstructorKindChirho.hs"
));
const ANNOTATED_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/AnnotatedKindChirho.hs"
));
const RECURSIVE_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/RecursiveKindChirho.hs"
));
const LOCAL_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/LocalKindChirho.hs"
));
const HIGHER_BINDER_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/HigherKindBinderChirho.hs"
));
const REFINED_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/RefinedKindChirho.hs"
));
const SYNONYM_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/SynonymKindChirho.hs"
));
const CLASSIFIER_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/KindClassifierChirho.hs"
));
const DEFAULTED_SYNONYM_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, EmptyDataDecls #-}
module DefaultedSynonymChirho where
data BoxChirho valueChirho
data InnerChirho valueChirho
type AliasChirho valueChirho = BoxChirho (InnerChirho valueChirho)
preserveChirho :: BoxChirho (InnerChirho Int) -> AliasChirho Int
preserveChirho valueChirho = valueChirho
"#;

fn rejection_chirho(source_chirho: &str) -> String {
    typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "InvalidKindApplicationChirho.hs",
    )
    .err()
    .unwrap_or_else(|| {
        panic!("this independently rejected kind-argument contract must fail:\n{source_chirho}")
    })
    .to_string()
}

#[test]
fn synonym_contracts_close_anonymous_kind_identities_chirho() {
    let source_chirho = r#"{-# LANGUAGE RankNTypes, PolyKinds #-}
module AnonymousSynonymChirho where
import Data.Kind (Type)
data TokenChirho (valueChirho :: kindChirho) = TokenChirho
data StoreChirho (polyChirho :: forall kindChirho. kindChirho -> Type) = StoreChirho (AliasChirho TokenChirho)
type AliasChirho = (StoreChirho :: (forall kindChirho. kindChirho -> Type) -> Type)
"#;
    assert_compile_success_chirho("AnonymousSynonymChirho.hs", source_chirho);
    let renamed_chirho = source_chirho.replace(
        "forall kindChirho. kindChirho -> Type",
        "forall innerChirho. innerChirho -> Type",
    );
    assert_compile_success_chirho("AnonymousSynonymChirho.hs", &renamed_chirho);
}

#[test]
fn qualified_kind_authority_survives_a_local_builtin_spelling_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, KindSignatures #-}
module QualifiedKindChirho where
import qualified Data.Kind as KChirho
data Type = LocalTypeChirho
data BoxChirho (valueChirho :: KChirho.Type) = BoxChirho
type GoodChirho = BoxChirho Int
"#;
    assert_compile_success_chirho("QualifiedKindChirho.hs", source_chirho);
    let wrong_chirho = source_chirho.replace("BoxChirho Int", "BoxChirho 'LocalTypeChirho");
    assert!(rejection_chirho(&wrong_chirho).contains("kind mismatch"));
}

#[test]
fn nested_synonym_foralls_are_fresh_and_still_rigid_chirho() {
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
type ResultChirho valueChirho = forall argumentChirho. argumentChirho -> valueChirho
answerChirho :: ResultChirho (ResultChirho Int)
answerChirho firstChirho secondChirho = 42
main = print (answerChirho 'c' True)
"#;
    assert_execution_chirho(source_chirho, "42\n");
    let wrong_chirho = source_chirho.replace(
        "secondChirho = 42",
        "secondChirho = if secondChirho then 42 else 0",
    );
    assert!(rejection_chirho(&wrong_chirho).contains("type mismatch"));
    let lambda_chirho = source_chirho.replace(
        "answerChirho firstChirho secondChirho = 42",
        "answerChirho = \\firstChirho secondChirho -> 42",
    );
    assert_execution_chirho(&lambda_chirho, "42\n");
    let wrong_lambda_chirho = lambda_chirho.replace(
        "secondChirho -> 42",
        "secondChirho -> if secondChirho then 42 else 0",
    );
    assert!(rejection_chirho(&wrong_lambda_chirho).contains("type mismatch"));
}

#[test]
fn inferred_hidden_arguments_solve_their_dependent_classifiers_chirho() {
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds #-}
module Main where
data ProxyChirho (valueChirho :: kindChirho) = ProxyChirho
data TagChirho (indexChirho :: ProxyChirho valueChirho) = TagChirho Int
type PairChirho (leftChirho :: ProxyChirho Either) (rightChirho :: ProxyChirho Maybe) = (TagChirho leftChirho, TagChirho rightChirho)
readChirho :: PairChirho 'ProxyChirho 'ProxyChirho -> Int
readChirho (TagChirho leftChirho, TagChirho rightChirho) = leftChirho + rightChirho
main = print (readChirho (TagChirho 40, TagChirho 2))
"#;
    assert_execution_chirho(source_chirho, "42\n");
}

#[test]
fn promoted_reflexivity_retains_the_kind_of_its_value_chirho() {
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, TypeOperators #-}
module PromotedReflexivityChirho where
import Data.Type.Equality
data ProofChirho (proofChirho :: True :~: True)
keepChirho :: ProofChirho Refl -> ProofChirho Refl
keepChirho valueChirho = valueChirho
"#;
    assert_compile_success_chirho("PromotedReflexivityChirho.hs", source_chirho);
    let wrong_chirho = source_chirho.replace("True :~: True", "True :~: False");
    assert!(rejection_chirho(&wrong_chirho).contains("kind mismatch"));
}

#[test]
fn visible_kind_argument_selects_the_family_classifier_chirho() {
    assert_compile_success_chirho("FamilyKindApplicationsChirho.hs", FAMILY_CHIRHO);
    let wrong_chirho = FAMILY_CHIRHO.replace("BoxChirho @Type Maybe", "BoxChirho @Type Int");
    assert!(rejection_chirho(&wrong_chirho).contains("kind mismatch"));
}

#[test]
fn visible_kind_arguments_follow_source_binder_order_chirho() {
    assert_compile_success_chirho("KindBinderOrderChirho.hs", ORDER_CHIRHO);
    let wrong_chirho = ORDER_CHIRHO.replace("@Type @Bool", "@Bool @Type");
    assert!(rejection_chirho(&wrong_chirho).contains("kind mismatch"));
}

#[test]
fn inferred_kind_binders_are_skipped_not_visibly_consumed_chirho() {
    assert_compile_success_chirho("InferredKindChirho.hs", INFERRED_CHIRHO);
    assert!(rejection_chirho(AUTO_CHIRHO).contains("kind argument"));
}

#[test]
fn visible_kind_arguments_are_name_checked_chirho() {
    let wrong_chirho = FAMILY_CHIRHO.replace("@Type Maybe", "@MissingKindChirho Maybe");
    assert!(rejection_chirho(&wrong_chirho).contains("MissingKindChirho"));
}

#[test]
fn phantom_kind_arguments_remain_distinct_during_type_equality_chirho() {
    let positive_chirho = PHANTOM_CHIRHO.replace("PhantomChirho @Type", "PhantomChirho @Bool");
    assert_compile_success_chirho("SamePhantomKindChirho.hs", &positive_chirho);
    assert!(rejection_chirho(PHANTOM_CHIRHO).contains("type mismatch"));
}

#[test]
fn explicit_kind_signatures_interoperate_with_implicit_constructor_arguments_chirho() {
    assert_execution_chirho(CONSTRUCTOR_CHIRHO, "42\n");
}

#[test]
fn wildcard_kind_arguments_are_inferred_from_the_ordinary_argument_chirho() {
    assert_execution_chirho(&CONSTRUCTOR_CHIRHO.replace("@Bool", "@_"), "42\n");
}

#[test]
fn recursive_nominal_fields_keep_the_group_kind_arguments_chirho() {
    assert_execution_chirho(RECURSIVE_CHIRHO, "42\n7\n");
}

#[test]
fn forall_annotated_kind_binders_instantiate_at_each_use_chirho() {
    assert_compile_success_chirho("HigherKindBinderChirho.hs", HIGHER_BINDER_CHIRHO);
    let monomorphic_chirho =
        HIGHER_BINDER_CHIRHO.replace("forall kindChirho. kindChirho -> Type", "Type -> Type");
    assert!(rejection_chirho(&monomorphic_chirho).contains("kind mismatch"));
}

#[test]
fn constructor_refinement_reaches_the_indexed_nominal_head_chirho() {
    assert_execution_chirho(REFINED_CHIRHO, "42\n");
    let no_equality_chirho = REFINED_CHIRHO.replace(
        "ReflChirho :: EqualChirho valueChirho valueChirho",
        "ReflChirho :: EqualChirho leftChirho rightChirho",
    );
    assert!(rejection_chirho(&no_equality_chirho).contains("type mismatch"));
}

#[test]
fn synonym_bodies_keep_solved_kind_arguments_and_parameter_sharing_chirho() {
    assert_execution_chirho(SYNONYM_CHIRHO, "42\n7\n11\n");
    let wrong_chirho = SYNONYM_CHIRHO.replace(
        "PhantomAliasChirho Bool -> PhantomChirho @Bool",
        "PhantomAliasChirho Bool -> PhantomChirho @Type",
    );
    assert!(rejection_chirho(&wrong_chirho).contains("type mismatch"));
}

#[test]
fn a_kind_annotation_requires_a_type_not_a_promoted_value_chirho() {
    assert_compile_success_chirho("KindClassifierChirho.hs", CLASSIFIER_CHIRHO);
    let wrong_chirho = CLASSIFIER_CHIRHO.replace("kindChirho :: Type", "kindChirho :: Bool");
    assert!(rejection_chirho(&wrong_chirho).contains("kind annotation"));
}

#[test]
fn monokinded_alias_bodies_share_the_defaulted_head_contract_chirho() {
    assert_compile_success_chirho("DefaultedSynonymChirho.hs", DEFAULTED_SYNONYM_CHIRHO);
    let wrong_chirho = DEFAULTED_SYNONYM_CHIRHO.replace("InnerChirho Int", "InnerChirho Maybe");
    assert!(rejection_chirho(&wrong_chirho).contains("kind mismatch"));
}

#[test]
fn expression_signatures_instantiate_nominal_kind_slots_without_erasing_them_chirho() {
    assert_execution_chirho(LOCAL_CHIRHO, "42\n");
    let wrong_chirho = LOCAL_CHIRHO.replace(
        "TokenChirho :: TokenChirho @Bool",
        "TokenChirho :: TokenChirho @Type",
    );
    assert!(rejection_chirho(&wrong_chirho).contains("type mismatch"));
}

#[test]
fn visible_kind_arguments_inside_binder_annotations_are_checked_chirho() {
    assert_compile_success_chirho("AnnotatedKindChirho.hs", ANNOTATED_CHIRHO);
    assert!(
        rejection_chirho(&ANNOTATED_CHIRHO.replace("@Type", "@MissingKindChirho"))
            .contains("MissingKindChirho")
    );
    assert!(
        rejection_chirho(&ANNOTATED_CHIRHO.replace("@Type", "@Bool")).contains("kind mismatch")
    );
}
