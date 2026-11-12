// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-class dictionary behavior against exact-source GHC references.

use super::{
    NextErrorKindChirho, assert_character_chirho, check_source_chirho, compile_source_chirho,
    eval_binding_chirho,
};
use haskelujah_runtime_chirho::ValueChirho;

#[test]
fn inferred_functions_abstract_and_apply_different_instance_dictionaries_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/class_evidence_chirho/positive-chirho/PolymorphicDispatchChirho.hs"
        ),
        'q',
    );
}

#[test]
fn captured_wanteds_stay_in_the_emitting_scope_and_receive_outer_evidence_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/class_evidence_chirho/positive-chirho/CapturedConstraintChirho.hs"
        ),
        'c',
    );
}

#[test]
fn recursive_references_reapply_the_groups_type_and_dictionary_parameters_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/class_evidence_chirho/positive-chirho/RecursiveEvidenceChirho.hs"
        ),
        'q',
    );
}

#[test]
fn monomorphism_restriction_waits_for_later_uses_to_fix_a_shared_alias_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/class_evidence_chirho/positive-chirho/MonomorphicUseChirho.hs"
        ),
        'b',
    );
}

#[test]
fn a_single_instance_never_specializes_a_flexible_inferred_argument_chirho() {
    let source_chirho = include_str!(
        "fixtures_chirho/class_evidence_chirho/positive-chirho/UniqueInstanceGeneralizationChirho.hs"
    );
    let checked_chirho = check_source_chirho(source_chirho).unwrap();
    let name_chirho = checked_chirho
        .module_chirho
        .names_chirho
        .iter()
        .position(|name_chirho| name_chirho.occurrence_chirho == "applyChirho")
        .unwrap();
    let scheme_chirho =
        &checked_chirho.schemes_chirho[&haskelujah_hir_chirho::NameIdChirho(name_chirho as u32)];
    assert_eq!(scheme_chirho.quantified_chirho.len(), 1);
    assert_eq!(scheme_chirho.context_chirho.len(), 1);
    let (_, argument_chirho) = checked_chirho
        .solver_chirho
        .predicate_view_chirho(scheme_chirho.context_chirho[0])
        .unwrap();
    assert_eq!(
        checked_chirho
            .solver_chirho
            .arena_chirho
            .resolve_chirho(argument_chirho)
            .unwrap(),
        scheme_chirho.quantified_chirho[0]
    );
    assert_character_chirho(source_chirho, 'b');
}

#[test]
fn instance_method_references_keep_using_the_generic_class_selector_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/class_evidence_chirho/positive-chirho/InstanceMethodScopeChirho.hs"
        ),
        'i',
    );
}

#[test]
fn two_variables_under_one_class_receive_distinct_evidence_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/class_evidence_chirho/positive-chirho/sharp-chirho/TwoVariablesChirho.hs"
        ),
        'q',
    );
}

#[test]
fn lexical_method_shadowing_does_not_generate_a_class_wanted_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/class_evidence_chirho/positive-chirho/sharp-chirho/ShadowedMethodChirho.hs"
        ),
        'z',
    );
}

#[test]
fn expected_result_types_choose_evidence_without_solver_meta_binding_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/class_evidence_chirho/positive-chirho/sharp-chirho/ResultDirectedChirho.hs"
        ),
        'd',
    );
}

#[test]
fn invalid_missing_ambiguous_or_wrong_class_evidence_never_reaches_core_chirho() {
    for (source_chirho, expected_chirho) in [
        (
            include_str!(
                "fixtures_chirho/class_evidence_chirho/negative-chirho/AmbiguousEvidenceChirho.hs"
            ),
            "ambiguous",
        ),
        (
            include_str!(
                "fixtures_chirho/class_evidence_chirho/negative-chirho/DuplicateInstanceChirho.hs"
            ),
            "duplicate",
        ),
        (
            include_str!(
                "fixtures_chirho/class_evidence_chirho/negative-chirho/MethodMismatchChirho.hs"
            ),
            "",
        ),
        (
            include_str!(
                "fixtures_chirho/class_evidence_chirho/negative-chirho/MissingInstanceChirho.hs"
            ),
            "no instance",
        ),
        (
            include_str!(
                "fixtures_chirho/class_evidence_chirho/negative-chirho/RestrictedAliasChirho.hs"
            ),
            "",
        ),
        (
            include_str!(
                "fixtures_chirho/class_evidence_chirho/negative-chirho/WrongClassChirho.hs"
            ),
            "no instance",
        ),
    ] {
        let error_chirho = compile_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::TypeChirho,
            "{source_chirho}: {error_chirho}"
        );
        assert!(
            error_chirho.message_chirho.contains(expected_chirho),
            "{error_chirho}"
        );
    }
}

#[test]
fn dictionary_fields_follow_class_order_and_methods_can_use_later_top_values_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/class_evidence_chirho/positive-chirho/sharp-chirho/MethodOrderChirho.hs"
        ),
        'o',
    );
}

#[test]
fn a_lambda_rhs_does_not_turn_a_restricted_binding_into_a_function_binding_chirho() {
    assert_eq!(
        compile_source_chirho(include_str!(
            "fixtures_chirho/class_evidence_chirho/negative-chirho/RestrictedLambdaChirho.hs"
        ))
        .unwrap_err()
        .kind_chirho,
        NextErrorKindChirho::TypeChirho
    );
}

#[test]
fn valid_flexible_and_applied_forms_stay_unsupported_until_their_solver_exists_chirho() {
    for source_chirho in [
        include_str!(
            "fixtures_chirho/class_evidence_chirho/unsupported-chirho/AppliedInstanceChirho.hs"
        ),
        include_str!(
            "fixtures_chirho/class_evidence_chirho/unsupported-chirho/FlexibleInstanceChirho.hs"
        ),
        include_str!(
            "fixtures_chirho/class_evidence_chirho/unsupported-chirho/HigherClassChirho.hs"
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
fn complex_application_heads_keep_each_invocations_environment_chirho() {
    let source_chirho = include_str!(
        "fixtures_chirho/class_evidence_chirho/positive-chirho/sharp-chirho/ComplexHeadChirho.hs"
    );
    let original_chirho = "(if flagChirho then firstChirho else secondChirho)";
    let mut outcomes_chirho = Vec::new();
    for head_chirho in [
        original_chirho,
        "(case flagChirho of { True -> firstChirho; False -> secondChirho })",
        "(let chosenChirho = if flagChirho then firstChirho else secondChirho in chosenChirho)",
        "((\\chooseChirho -> if chooseChirho then firstChirho else secondChirho) flagChirho)",
    ] {
        let variant_chirho = source_chirho.replace(original_chirho, head_chirho);
        let (value_chirho, _) =
            eval_binding_chirho(&variant_chirho, "resultChirho", 50_000).unwrap();
        outcomes_chirho.push(value_chirho);
    }
    assert_eq!(outcomes_chirho, vec![ValueChirho::CharChirho('s'); 4]);
}

#[test]
fn class_producers_retain_kind_and_value_namespace_errors_chirho() {
    for source_chirho in [
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/tcfail146.hs"),
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/tcfail147.hs"),
    ] {
        let error_chirho = compile_source_chirho(source_chirho).unwrap_err();
        assert_eq!(error_chirho.kind_chirho, NextErrorKindChirho::TypeChirho);
        assert!(error_chirho.message_chirho.contains("kind Type"));
    }
    assert_eq!(
        compile_source_chirho(include_str!(
            "fixtures_chirho/class_evidence_chirho/negative-chirho/KindedInstanceChirho.hs"
        ))
        .unwrap_err()
        .kind_chirho,
        NextErrorKindChirho::TypeChirho
    );
    assert_eq!(
        compile_source_chirho(include_str!(
            "fixtures_chirho/class_evidence_chirho/negative-chirho/DuplicateMethodChirho.hs"
        ))
        .unwrap_err()
        .kind_chirho,
        NextErrorKindChirho::InvalidChirho
    );
    // The lossless body must decline before the old AST producer drops an undeclared default.
    assert_eq!(
        compile_source_chirho(include_str!(
            "../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/tcfail077.hs"
        ))
        .unwrap_err()
        .kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
}
