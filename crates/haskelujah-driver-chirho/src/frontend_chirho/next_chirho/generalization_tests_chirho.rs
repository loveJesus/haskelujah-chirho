// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Real source controls: independent instantiation, environment capture and monomorphic recursion.

use super::{NextErrorKindChirho, check_source_chirho, compile_source_chirho, eval_binding_chirho};
use haskelujah_runtime_chirho::ValueChirho;
use haskelujah_tc_chirho::{TyConChirho, TyNodeChirho};

#[test]
fn dependency_groups_generalize_top_level_uses_independently_chirho() {
    let source_chirho = include_str!("fixtures_chirho/TopPolymorphismChirho.hs");
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 10_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('q'));
}

#[test]
fn local_let_bindings_are_polymorphic_without_generalizing_the_environment_chirho() {
    let source_chirho = include_str!("fixtures_chirho/LocalPolymorphismChirho.hs");
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 10_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('l'));
    let bad_chirho = include_str!("fixtures_chirho/OuterCaptureChirho.hs");
    assert_eq!(
        check_source_chirho(bad_chirho).unwrap_err().kind_chirho,
        NextErrorKindChirho::TypeChirho
    );
}

#[test]
fn a_recursive_scc_is_monomorphic_inside_and_polymorphic_at_its_uses_chirho() {
    let source_chirho = include_str!("fixtures_chirho/MutualPolymorphismChirho.hs");
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 10_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('m'));
    let bad_chirho = include_str!("fixtures_chirho/PolymorphicRecursionChirho.hs");
    assert_eq!(
        check_source_chirho(bad_chirho).unwrap_err().kind_chirho,
        NextErrorKindChirho::TypeChirho
    );
}

#[test]
fn complete_signatures_break_the_inference_dependency_cycle_chirho() {
    let source_chirho = include_str!("fixtures_chirho/SignedCycleChirho.hs");
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 10_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('s'));
}

#[test]
fn unused_instantiation_variables_are_sealed_only_after_generalization_chirho() {
    let source_chirho = include_str!("fixtures_chirho/UnusedInstantiationChirho.hs");
    let checked_chirho = check_source_chirho(source_chirho).unwrap();
    let arena_chirho = &checked_chirho.solver_chirho.arena_chirho;
    assert!(
        checked_chirho
            .wrappers_chirho
            .values()
            .flatten()
            .filter_map(|step_chirho| match step_chirho {
                haskelujah_tc_chirho::WrapStepChirho::TypeChirho(ty_chirho) => Some(ty_chirho),
                haskelujah_tc_chirho::WrapStepChirho::EvidenceChirho(_) => None,
            })
            .any(|ty_chirho| {
                matches!(
                    arena_chirho
                        .node_chirho(arena_chirho.resolve_chirho(*ty_chirho).unwrap())
                        .unwrap(),
                    TyNodeChirho::ConChirho(TyConChirho::AnyChirho)
                )
            })
    );
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 10_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('u'));
}

#[test]
fn original_identity_and_s_k_k_programs_typecheck_without_impredicative_instantiation_chirho() {
    for source_chirho in [
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/tc022.hs"),
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/tc023.hs"),
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/tc024.hs"),
    ] {
        compile_source_chirho(source_chirho).unwrap();
    }
}
