// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Superclass dictionaries execute with the same declaration-ordered layout as their methods.

use super::{
    NextErrorKindChirho, assert_character_chirho, check_source_chirho, compile_source_chirho,
};

#[test]
fn superclass_and_method_fields_follow_each_class_declaration_order_chirho() {
    assert_character_chirho(
        include_str!("fixtures_chirho/superclasses_chirho/positive-chirho/FieldOrderChirho.hs"),
        'o',
    );
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/superclasses_chirho/positive-chirho/ReversedFieldOrderChirho.hs"
        ),
        'o',
    );
}

#[test]
fn transitive_and_diamond_superclass_paths_select_the_right_dictionaries_chirho() {
    assert_character_chirho(
        include_str!("fixtures_chirho/superclasses_chirho/positive-chirho/TransitiveChirho.hs"),
        't',
    );
    assert_character_chirho(
        include_str!("fixtures_chirho/superclasses_chirho/positive-chirho/DiamondChirho.hs"),
        'd',
    );
}

#[test]
fn a_nested_use_gets_a_derived_dictionary_from_its_ancestor_owner_chirho() {
    assert_character_chirho(
        include_str!("fixtures_chirho/superclasses_chirho/positive-chirho/OuterGivenChirho.hs"),
        'g',
    );
}

#[test]
fn an_instance_superclass_closes_from_its_contexts_derived_given_chirho() {
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/superclasses_chirho/positive-chirho/ContextSuperclassChirho.hs"
        ),
        's',
    );
}

#[test]
fn inferred_contexts_drop_transitive_superclasses_but_written_parameters_stay_exact_chirho() {
    let source_chirho =
        include_str!("fixtures_chirho/superclasses_chirho/positive-chirho/MinimalContextChirho.hs");
    let checked_chirho = check_source_chirho(source_chirho).unwrap();
    let name_chirho = checked_chirho
        .module_chirho
        .names_chirho
        .iter()
        .position(|name_chirho| name_chirho.occurrence_chirho == "inspectChirho")
        .unwrap();
    let scheme_chirho =
        &checked_chirho.schemes_chirho[&haskelujah_hir_chirho::NameIdChirho(name_chirho as u32)];
    assert_eq!(scheme_chirho.context_chirho.len(), 1);
    let (class_chirho, _) = checked_chirho
        .solver_chirho
        .predicate_view_chirho(scheme_chirho.context_chirho[0])
        .unwrap();
    assert_eq!(
        checked_chirho.module_chirho.names_chirho[class_chirho.0 as usize].occurrence_chirho,
        "ThirdChirho"
    );
    assert_character_chirho(source_chirho, 'm');
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/superclasses_chirho/positive-chirho/ExplicitContextsChirho.hs"
        ),
        'e',
    );
}

#[test]
fn missing_superclasses_wrong_direction_and_class_cycles_never_reach_core_chirho() {
    for (source_chirho, expected_chirho) in [
        (
            include_str!("fixtures_chirho/superclasses_chirho/negative-chirho/DirectionChirho.hs"),
            "no instance",
        ),
        (
            include_str!(
                "fixtures_chirho/superclasses_chirho/negative-chirho/AbsentSuperclassChirho.hs"
            ),
            "no instance",
        ),
        (
            include_str!(
                "fixtures_chirho/superclasses_chirho/negative-chirho/MissingContextChirho.hs"
            ),
            "no instance",
        ),
        (
            include_str!("fixtures_chirho/superclasses_chirho/negative-chirho/CycleChirho.hs"),
            "cycle",
        ),
        (
            include_str!("fixtures_chirho/superclasses_chirho/negative-chirho/SelfCycleChirho.hs"),
            "cycle",
        ),
    ] {
        let error_chirho = compile_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::TypeChirho,
            "{error_chirho}"
        );
        assert!(
            error_chirho.message_chirho.contains(expected_chirho),
            "{error_chirho}"
        );
    }
}

#[test]
fn valid_compound_superclasses_stay_unsupported_chirho() {
    assert_eq!(
        compile_source_chirho(include_str!(
            "fixtures_chirho/superclasses_chirho/unsupported-chirho/CompoundSuperclassChirho.hs"
        ))
        .unwrap_err()
        .kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
}

#[test]
fn superclass_dictionaries_follow_each_type_argument_and_each_method_scope_chirho() {
    assert_character_chirho(
        include_str!("fixtures_chirho/superclasses_chirho/positive-chirho/TwoArgumentsChirho.hs"),
        'p',
    );
    assert_character_chirho(
        include_str!(
            "fixtures_chirho/superclasses_chirho/positive-chirho/MethodSuperclassChirho.hs"
        ),
        'h',
    );
    let checked_chirho = check_source_chirho(include_str!(
        "fixtures_chirho/superclasses_chirho/positive-chirho/TwoArgumentsChirho.hs"
    ))
    .unwrap();
    let name_chirho = checked_chirho
        .module_chirho
        .names_chirho
        .iter()
        .position(|name_chirho| name_chirho.occurrence_chirho == "inspectChirho")
        .unwrap();
    let scheme_chirho =
        &checked_chirho.schemes_chirho[&haskelujah_hir_chirho::NameIdChirho(name_chirho as u32)];
    assert_eq!(scheme_chirho.context_chirho.len(), 2);
    let (_, first_chirho) = checked_chirho
        .solver_chirho
        .predicate_view_chirho(scheme_chirho.context_chirho[0])
        .unwrap();
    let (_, second_chirho) = checked_chirho
        .solver_chirho
        .predicate_view_chirho(scheme_chirho.context_chirho[1])
        .unwrap();
    assert_ne!(first_chirho, second_chirho);
}
