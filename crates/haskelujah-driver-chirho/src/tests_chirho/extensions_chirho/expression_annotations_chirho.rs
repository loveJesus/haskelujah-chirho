// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Universal annotation controls independently measured with GHC9.14.1.

use super::ordinary_elaboration_chirho::frontend_chirho;

#[test]
fn annotation_universal_annotation_positive_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/UniversalAnnotationPositiveChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_captured_annotation_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/CapturedAnnotationChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC universal negative rejects");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn annotation_specialized_annotation_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/SpecializedAnnotationChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC universal negative rejects");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn annotation_scoped_annotation_shadow_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/ScopedAnnotationShadowChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_annotation_given_generalized_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/AnnotationGivenGeneralizedChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_annotation_given_negative_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/AnnotationGivenNegativeChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC universal negative rejects");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn annotation_annotation_visible_type_application_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/AnnotationVisibleTypeApplicationChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}
