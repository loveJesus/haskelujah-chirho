// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Original-source scope controls, referenced against GHC 9.14.1 without all.T flags.
//! workflow: type-identity-chirho/committed-meta-kinds-chirho

fn check_original_chirho(
    name_chirho: &str,
    source_chirho: &str,
) -> Result<crate::FrontendResultChirho, haskelujah_diagnostics_chirho::DiagnosticBundleChirho> {
    crate::typecheck_source_chirho(
        source_chirho,
        &mut haskelujah_span_chirho::SourceMapChirho::new_chirho(),
        name_chirho,
    )
}

#[test]
fn a_rank_n_pattern_signature_keeps_its_body_type_variable_chirho() {
    check_original_chirho(
        "tc198.hs",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../ghc-tests-chirho/typecheck-chirho/should_compile/tc198.hs"
        )),
    )
    .expect("GHC accepts the pattern signature and the body annotation together");
}

#[test]
fn an_existential_pattern_signature_keeps_its_body_type_variable_chirho() {
    check_original_chirho(
        "T11506.hs",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../ghc-tests-chirho/typecheck-chirho/should_compile/T11506.hs"
        )),
    )
    .expect("GHC accepts the scoped existential variable without escaping it");
}

#[test]
fn an_existential_pattern_variable_still_cannot_escape_chirho() {
    let error_chirho = check_original_chirho(
        "ExPatFail.hs",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../ghc-tests-chirho/typecheck-chirho/should_compile/ExPatFail.hs"
        )),
    )
    .err()
    .expect("GHC rejects the existential result escaping its pattern scope");
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| {
                diagnostic_chirho.message_chirho.contains("existential")
                    && diagnostic_chirho.message_chirho.contains("escape")
            })
    );
}
