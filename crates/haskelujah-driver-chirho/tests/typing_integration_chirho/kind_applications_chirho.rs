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
