// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor-context dictionary functions, checked and executed against GHC references.

use super::{NextErrorKindChirho, assert_character_chirho, compile_source_chirho};

#[test]
fn nested_constructor_instances_build_the_matching_dictionary_at_each_type_chirho() {
    assert_character_chirho(
        include_str!("fixtures_chirho/context_instances_chirho/positive-chirho/NestedBoxChirho.hs"),
        'q',
    );
}

#[test]
fn a_recursive_instance_method_uses_its_context_without_a_self_given_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/context_instances_chirho/positive-chirho/RecursiveListChirho.hs"
        ),
        'b',
    );
}

#[test]
fn instance_binder_order_and_dictionary_context_order_are_independent_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/context_instances_chirho/positive-chirho/ReversedContextChirho.hs"
        ),
        'o',
    );
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/context_instances_chirho/positive-chirho/DistinctClassesChirho.hs"
        ),
        'd',
    );
}

#[test]
fn instance_contexts_remain_lexical_through_local_generalization_and_signatures_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/context_instances_chirho/positive-chirho/CapturedMethodChirho.hs"
        ),
        'c',
    );
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/context_instances_chirho/positive-chirho/ScopedInstanceChirho.hs"
        ),
        's',
    );
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/context_instances_chirho/positive-chirho/SignedCompoundUseChirho.hs"
        ),
        'g',
    );
}

#[test]
fn context_free_instances_do_not_demand_dictionaries_for_unused_parameters_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/context_instances_chirho/positive-chirho/UnconstrainedParameterChirho.hs"
        ),
        'u',
    );
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/context_instances_chirho/positive-chirho/PhantomFunctionChirho.hs"
        ),
        'f',
    );
}

#[test]
fn tuple_head_instances_preserve_both_argument_dictionaries_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/context_instances_chirho/positive-chirho/TupleDictionaryChirho.hs"
        ),
        't',
    );
}

#[test]
fn missing_contexts_leaf_instances_and_wrong_class_givens_never_reach_core_chirho() {
    for (source_chirho, message_chirho) in [
        (
            include_str!(
                "fixtures_chirho/context_instances_chirho/negative-chirho/MissingContextChirho.hs"
            ),
            "no instance",
        ),
        (
            include_str!(
                "fixtures_chirho/context_instances_chirho/negative-chirho/MissingLeafChirho.hs"
            ),
            "no instance",
        ),
        (
            include_str!(
                "fixtures_chirho/context_instances_chirho/negative-chirho/WrongContextChirho.hs"
            ),
            "no instance",
        ),
        (
            include_str!(
                "fixtures_chirho/context_instances_chirho/negative-chirho/UnusedMissingContextChirho.hs"
            ),
            "no instance",
        ),
        (
            include_str!(
                "fixtures_chirho/context_instances_chirho/negative-chirho/DuplicateHeadChirho.hs"
            ),
            "duplicate",
        ),
        (
            include_str!(
                "fixtures_chirho/context_instances_chirho/negative-chirho/RigidMethodChirho.hs"
            ),
            "",
        ),
    ] {
        let error_chirho = compile_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::TypeChirho,
            "{error_chirho}"
        );
        assert!(
            error_chirho.message_chirho.contains(message_chirho),
            "{error_chirho}"
        );
    }
}

#[test]
fn repeated_fixed_or_compound_instance_patterns_remain_unsupported_chirho() {
    for source_chirho in [
        include_str!(
            "fixtures_chirho/context_instances_chirho/unsupported-chirho/RepeatedVariableChirho.hs"
        ),
        include_str!(
            "fixtures_chirho/context_instances_chirho/unsupported-chirho/FixedArgumentChirho.hs"
        ),
        include_str!(
            "fixtures_chirho/context_instances_chirho/unsupported-chirho/CompoundInstanceContextChirho.hs"
        ),
    ] {
        let error_chirho = compile_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{error_chirho}"
        );
    }
}

#[test]
fn haskell2010_does_not_gain_scoped_instance_type_variables_chirho() {
    let source_chirho = include_str!(
        "fixtures_chirho/context_instances_chirho/positive-chirho/ScopedInstanceChirho.hs"
    )
    .replace(
        "module ScopedInstanceChirho",
        "{-# LANGUAGE Haskell2010 #-}\nmodule ScopedInstanceChirho",
    );
    let error_chirho = compile_source_chirho(&source_chirho).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
}

#[test]
fn inferred_compound_constraints_generalize_the_strict_subgoal_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/context_instances_chirho/positive-chirho/InferredCompoundChirho.hs"
        ),
        'i',
    );
}
