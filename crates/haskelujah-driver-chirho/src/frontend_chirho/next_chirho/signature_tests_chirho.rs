// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written source dictionaries and skolem scope, compared with GHC's exact-source verdicts.

use super::{NextErrorKindChirho, assert_character_chirho, compile_source_chirho};

#[test]
fn a_declared_qualified_function_dispatches_at_each_use_type_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/SignedDispatchChirho.hs"
        ),
        'q',
    );
}

#[test]
fn explicit_forall_binders_remain_rigid_in_the_body_and_fresh_at_uses_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/ExplicitForallChirho.hs"
        ),
        'e',
    );
}

#[test]
fn declared_dictionary_order_distinguishes_classes_and_variables_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/TwoClassesChirho.hs"
        ),
        'o',
    );
}

#[test]
fn a_nested_signature_reusing_a_variable_name_has_its_own_given_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/NestedSignatureChirho.hs"
        ),
        'q',
    );
}

#[test]
fn class_givens_leave_capture_type_unification_touchable_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/OuterUnificationChirho.hs"
        ),
        'u',
    );
}

#[test]
fn signed_recursive_occurrences_apply_their_declared_evidence_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/SignedRecursionChirho.hs"
        ),
        'r',
    );
}

#[test]
fn an_inferred_outer_constraint_solves_a_wanted_inside_a_written_local_signature_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/CapturedSignatureChirho.hs"
        ),
        'c',
    );
}

#[test]
fn an_unqualified_polymorphic_signature_is_checked_and_instantiated_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/DeclaredIdentityChirho.hs"
        ),
        'i',
    );
}

#[test]
fn missing_givens_rigid_specialization_ambiguity_and_escape_refuse_before_core_chirho() {
    for source_chirho in [
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/negative-chirho/TooGeneralChirho.hs"
        ),
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/negative-chirho/RigidSpecializationChirho.hs"
        ),
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/negative-chirho/AmbiguousSignatureChirho.hs"
        ),
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/negative-chirho/EscapingSignatureChirho.hs"
        ),
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/negative-chirho/WrongGivenChirho.hs"
        ),
    ] {
        let error_chirho = compile_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::TypeChirho,
            "{source_chirho}: {error_chirho}"
        );
    }
}

#[test]
fn a_class_given_with_one_available_instance_cannot_specialize_a_signature_skolem_chirho() {
    assert_eq!(
        compile_source_chirho(include_str!(
            "fixtures_chirho/signature_evidence_chirho/negative-chirho/GivenSpecializationChirho.hs"
        ))
        .unwrap_err()
        .kind_chirho,
        NextErrorKindChirho::TypeChirho
    );
}

#[test]
fn richer_valid_signature_forms_remain_explicitly_unsupported_chirho() {
    for source_chirho in [
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/unsupported-chirho/RankArgumentChirho.hs"
        ),
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/unsupported-chirho/CompoundContextChirho.hs"
        ),
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/unsupported-chirho/ScopedSignatureChirho.hs"
        ),
    ] {
        assert_eq!(
            compile_source_chirho(source_chirho)
                .unwrap_err()
                .kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{source_chirho}"
        );
    }
}

#[test]
fn an_implicit_inner_signature_captures_the_enclosing_explicit_forall_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/ScopedReuseGoodChirho.hs"
        ),
        's',
    );
    assert_eq!(
        compile_source_chirho(include_str!(
            "fixtures_chirho/signature_evidence_chirho/negative-chirho/ScopedReuseBadChirho.hs"
        ))
        .unwrap_err()
        .kind_chirho,
        NextErrorKindChirho::TypeChirho
    );
}

#[test]
fn an_rhs_annotation_uses_the_signature_declarations_scoped_variable_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/ScopedAnnotationChirho.hs"
        ),
        'a',
    );
}

#[test]
fn a_nested_explicit_forall_shadows_the_outer_variable_with_a_fresh_identity_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/ExplicitShadowChirho.hs"
        ),
        'q',
    );
}

#[test]
fn a_fixed_outer_variable_in_a_written_local_context_is_not_ambiguous_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/signature_evidence_chirho/positive-chirho/CapturedContextChirho.hs"
        ),
        't',
    );
}
