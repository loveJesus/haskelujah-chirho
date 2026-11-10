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

#[test]
fn pattern_variables_can_name_concrete_repeated_and_shared_existential_types_chirho() {
    for (name_chirho, source_chirho) in [
        (
            "ConcretePatternChirho.hs",
            include_str!("pattern_signature_scope_chirho/concrete-pattern-chirho.hs"),
        ),
        (
            "DistinctAliasesChirho.hs",
            include_str!("pattern_signature_scope_chirho/distinct-aliases-chirho.hs"),
        ),
        (
            "RepeatedVariableChirho.hs",
            include_str!("pattern_signature_scope_chirho/repeated-variable-chirho.hs"),
        ),
    ] {
        check_original_chirho(name_chirho, source_chirho)
            .unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho:?}"));
    }
}

#[test]
fn an_enclosing_forall_remains_rigid_at_a_pattern_signature_chirho() {
    let error_chirho = check_original_chirho(
        "EnclosingRigidChirho.hs",
        include_str!("pattern_signature_scope_chirho/enclosing-rigid-chirho.hs"),
    )
    .err()
    .expect("the pattern signature cannot rebind its enclosing rigid variable");
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| diagnostic_chirho.message_chirho.contains("type mismatch")),
        "{error_chirho:?}"
    );
}

#[test]
fn a_nested_forall_stays_inside_the_pattern_type_chirho() {
    check_original_chirho(
        "NestedForallChirho.hs",
        include_str!("pattern_signature_scope_chirho/nested-forall-chirho.hs"),
    )
    .expect("a nested forall is checked locally without acquiring a body alias");
}

#[test]
fn a_pattern_binding_cannot_introduce_a_scoped_type_variable_chirho() {
    let error_chirho = check_original_chirho(
        "PatternBindingChirho.hs",
        include_str!("pattern_signature_scope_chirho/pattern-binding-chirho.hs"),
    )
    .err()
    .expect("GHC rejects a new scoped variable in a pattern binding signature");
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| {
                diagnostic_chirho
                    .message_chirho
                    .contains("pattern binding signature")
                    && diagnostic_chirho
                        .message_chirho
                        .contains("scoped type variable")
            }),
        "{error_chirho:?}"
    );
}

#[test]
fn an_instance_head_variable_is_not_rebound_by_a_method_pattern_chirho() {
    let error_chirho = check_original_chirho(
        "InstancePatternRigidChirho.hs",
        include_str!("pattern_signature_scope_chirho/instance-pattern-rigid-chirho.hs"),
    )
    .err()
    .expect("the method pattern cannot equate its existential with the instance's rigid variable");
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| diagnostic_chirho.message_chirho.contains("type mismatch")),
        "{error_chirho:?}"
    );
}
