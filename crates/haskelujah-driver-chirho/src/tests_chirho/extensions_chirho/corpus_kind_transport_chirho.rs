// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Exact immutable corpus movers, measured independently by root with GHC9.14.1.

fn check_chirho(axis_chirho: &str, name_chirho: &str) -> Result<crate::CheckSummaryChirho, String> {
    let path_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../ghc-tests-chirho/typecheck-chirho")
        .join(axis_chirho)
        .join(name_chirho);
    crate::check_source_path_chirho(path_chirho, crate::ExecutionModeChirho::BatchChirho).map_err(
        |diagnostics_chirho| {
            diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        },
    )
}

#[test]
fn corpus_t10562_chirho() {
    check_chirho("should_compile", "T10562.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t10635_chirho() {
    check_chirho("should_compile", "T10635.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t10776_chirho() {
    check_chirho("should_compile", "T10776.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t10009_chirho() {
    check_chirho("should_compile", "T10009.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t10195_chirho() {
    check_chirho("should_compile", "T10195.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t11811_chirho() {
    check_chirho("should_compile", "T11811.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_polytypedecomp_chirho() {
    let error_chirho = check_chirho("should_compile", "PolytypeDecomp.hs")
        .err()
        .expect("root's exact GHC-rejected source must reject");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn corpus_t14048a_chirho() {
    let error_chirho = check_chirho("should_fail", "T14048a.hs")
        .err()
        .expect("root's exact GHC-rejected source must reject");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn corpus_t16059c_chirho() {
    let error_chirho = check_chirho("should_fail", "T16059c.hs")
        .err()
        .expect("root's exact GHC-rejected source must reject");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn corpus_t22560_fail_b_chirho() {
    let error_chirho = check_chirho("should_fail", "T22560_fail_b.hs")
        .err()
        .expect("root's exact GHC-rejected source must reject");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn corpus_t4524_finishes_without_resource_exhaustion_chirho() {
    check_chirho("should_compile", "T4524.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t16512a_rejects_with_a_coded_diagnostic_chirho() {
    let error_chirho = check_chirho("should_fail", "T16512a.hs")
        .err()
        .expect("the retained GHC-rejected source must reject");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn corpus_t23543_keeps_universal_kind_annotations_chirho() {
    check_chirho("should_compile", "T23543.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_tc269_keeps_alpha_equivalent_kinds_and_universe_producers_chirho() {
    check_chirho("should_compile", "tc269.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_visflag2_keeps_required_quantifiers_distinct_chirho() {
    let error_chirho = check_chirho("should_fail", "VisFlag2.hs")
        .err()
        .expect("fresh GHC visibility negative rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn corpus_visflag3_keeps_associated_quantifier_visibility_chirho() {
    let error_chirho = check_chirho("should_fail", "VisFlag3.hs")
        .err()
        .expect("fresh GHC associated visibility negative rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn corpus_t8856_checks_annotation_with_its_owned_scope_chirho() {
    check_chirho("should_compile", "T8856.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t10031_checks_context_beneath_annotation_forall_chirho() {
    check_chirho("should_compile", "T10031.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}
