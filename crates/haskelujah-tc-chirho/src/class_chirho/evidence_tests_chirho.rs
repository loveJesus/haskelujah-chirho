// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Evidence authority, implication touchability and aggregate growth boundaries.

use super::*;

fn solver_with_class_chirho(ceiling_chirho: usize) -> (SolverChirho, TyChirho) {
    let mut solver_chirho = SolverChirho::new_chirho(ceiling_chirho);
    let class_chirho = NameIdChirho(1);
    solver_chirho
        .arena_chirho
        .declare_class_chirho(class_chirho)
        .unwrap();
    let kind_chirho = solver_chirho
        .arena_chirho
        .con_chirho(crate::TyConChirho::TypeChirho)
        .unwrap();
    let (_, parameter_chirho) = solver_chirho
        .arena_chirho
        .fresh_rigid_chirho(0, kind_chirho)
        .unwrap();
    solver_chirho.classes_chirho.insert(
        class_chirho,
        ClassInfoChirho {
            name_chirho: class_chirho,
            parameter_chirho,
            superclasses_chirho: Vec::new(),
            methods_chirho: Vec::new(),
        },
    );
    let argument_chirho = solver_chirho
        .arena_chirho
        .con_chirho(crate::TyConChirho::BoolChirho)
        .unwrap();
    let predicate_chirho = solver_chirho
        .predicate_chirho(class_chirho, argument_chirho)
        .unwrap();
    (solver_chirho, predicate_chirho)
}

#[test]
fn a_child_given_cannot_escape_and_an_ancestor_solution_stays_at_the_wanted_owner_chirho() {
    let (mut solver_chirho, predicate_chirho) = solver_with_class_chirho(200);
    let root_chirho = solver_chirho.root_chirho();
    let child_chirho = solver_chirho
        .implication_chirho(root_chirho, false)
        .unwrap();
    let child_given_chirho = solver_chirho
        .given_dictionary_chirho(child_chirho, predicate_chirho)
        .unwrap();
    let outside_chirho = solver_chirho
        .want_dictionary_chirho(root_chirho, predicate_chirho, SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    solver_chirho.solve_dictionaries_chirho().unwrap();
    assert!(
        solver_chirho
            .evidence_term_chirho(outside_chirho)
            .unwrap()
            .is_none()
    );
    assert_eq!(
        solver_chirho
            .bind_evidence_chirho(
                outside_chirho,
                EvTermChirho::VariableChirho(child_given_chirho)
            )
            .unwrap_err()
            .kind_chirho,
        TcErrorKindChirho::InvalidReferenceChirho
    );
    let inner_chirho = solver_chirho
        .implication_chirho(child_chirho, false)
        .unwrap();
    let inside_chirho = solver_chirho
        .want_dictionary_chirho(inner_chirho, predicate_chirho, SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    solver_chirho.solve_dictionaries_chirho().unwrap();
    assert_eq!(
        solver_chirho.evidence_term_chirho(inside_chirho).unwrap(),
        Some(&EvTermChirho::VariableChirho(child_given_chirho))
    );
    assert_eq!(
        solver_chirho.scope_bindings_chirho(inner_chirho).unwrap(),
        &[inside_chirho]
    );
    assert!(
        solver_chirho
            .scope_bindings_chirho(child_chirho)
            .unwrap()
            .is_empty()
    );
    let (other_chirho, _) = solver_with_class_chirho(200);
    assert_eq!(
        other_chirho
            .evidence_predicate_chirho(inside_chirho)
            .unwrap_err()
            .kind_chirho,
        TcErrorKindChirho::InvalidReferenceChirho
    );
}

#[test]
fn class_only_givens_never_make_outer_metas_untouchable_chirho() {
    let (mut solver_chirho, predicate_chirho) = solver_with_class_chirho(200);
    let root_chirho = solver_chirho.root_chirho();
    let scope_chirho = solver_chirho
        .implication_chirho(root_chirho, false)
        .unwrap();
    solver_chirho
        .given_dictionary_chirho(scope_chirho, predicate_chirho)
        .unwrap();
    let type_chirho = solver_chirho
        .arena_chirho
        .con_chirho(crate::TyConChirho::TypeChirho)
        .unwrap();
    let variable_chirho = solver_chirho
        .arena_chirho
        .fresh_meta_chirho(0, type_chirho)
        .unwrap();
    let bool_chirho = solver_chirho
        .arena_chirho
        .con_chirho(crate::TyConChirho::BoolChirho)
        .unwrap();
    solver_chirho.want_chirho(EqualityChirho {
        left_chirho: variable_chirho,
        right_chirho: bool_chirho,
        scope_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    });
    solver_chirho.solve_chirho().unwrap();
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(variable_chirho)
            .unwrap(),
        bool_chirho
    );
}

#[test]
fn dictionary_wanteds_and_solutions_share_the_module_allocation_ceiling_chirho() {
    let (mut solver_chirho, predicate_chirho) = solver_with_class_chirho(100);
    let root_chirho = solver_chirho.root_chirho();
    let mut count_chirho = 0;
    loop {
        match solver_chirho.want_dictionary_chirho(
            root_chirho,
            predicate_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ) {
            Ok(_) => count_chirho += 1,
            Err(error_chirho) => {
                assert_eq!(error_chirho.kind_chirho, TcErrorKindChirho::ResourceChirho);
                break;
            }
        }
        assert!(
            count_chirho < 100,
            "dictionary growth bypassed the module ceiling"
        );
    }
    assert!(count_chirho > 0);
    assert_eq!(solver_chirho.arena_chirho.allocation_count_chirho(), 100);
    assert_eq!(
        solver_chirho
            .solve_dictionaries_chirho()
            .unwrap_err()
            .kind_chirho,
        TcErrorKindChirho::ResourceChirho
    );
    assert!(solver_chirho.solve_dictionaries_chirho().is_err());
}

#[test]
fn a_signature_owned_skolem_blocks_floating_but_an_outer_capture_still_floats_chirho() {
    let (mut solver_chirho, _) = solver_with_class_chirho(300);
    let outer_chirho = solver_chirho
        .implication_chirho(solver_chirho.root_chirho(), false)
        .unwrap();
    let signature_chirho = solver_chirho
        .implication_kind_chirho(outer_chirho, false, ScopeKindChirho::SignatureChirho)
        .unwrap();
    let type_chirho = solver_chirho
        .arena_chirho
        .con_chirho(crate::TyConChirho::TypeChirho)
        .unwrap();
    let skolem_chirho = solver_chirho
        .signature_variable_chirho(signature_chirho, NameIdChirho(2), type_chirho)
        .unwrap();
    let predicate_chirho = solver_chirho
        .predicate_chirho(NameIdChirho(1), skolem_chirho)
        .unwrap();
    solver_chirho
        .want_dictionary_chirho(signature_chirho, predicate_chirho, SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    assert!(
        solver_chirho
            .dictionary_candidates_chirho(outer_chirho)
            .unwrap()
            .is_empty()
    );
    let variable_chirho = solver_chirho
        .arena_chirho
        .fresh_meta_chirho(
            solver_chirho.level_chirho(outer_chirho).unwrap(),
            type_chirho,
        )
        .unwrap();
    let captured_chirho = solver_chirho
        .predicate_chirho(NameIdChirho(1), variable_chirho)
        .unwrap();
    solver_chirho
        .want_dictionary_chirho(signature_chirho, captured_chirho, SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    assert_eq!(
        solver_chirho
            .dictionary_candidates_chirho(outer_chirho)
            .unwrap(),
        vec![captured_chirho]
    );
}

fn register_list_instance_chirho(solver_chirho: &mut SolverChirho) -> NameIdChirho {
    let type_chirho = solver_chirho
        .arena_chirho
        .con_chirho(crate::TyConChirho::TypeChirho)
        .unwrap();
    let scope_chirho = solver_chirho
        .implication_kind_chirho(
            solver_chirho.root_chirho(),
            false,
            ScopeKindChirho::InstanceChirho,
        )
        .unwrap();
    let binder_chirho = solver_chirho
        .signature_variable_chirho(scope_chirho, NameIdChirho(10), type_chirho)
        .unwrap();
    let head_chirho = solver_chirho
        .arena_chirho
        .list_chirho(binder_chirho)
        .unwrap();
    let predicate_chirho = solver_chirho
        .predicate_chirho(NameIdChirho(1), binder_chirho)
        .unwrap();
    let instance_chirho = NameIdChirho(11);
    solver_chirho
        .register_instance_head_chirho(
            NameIdChirho(1),
            head_chirho,
            &[binder_chirho],
            instance_chirho,
        )
        .unwrap();
    solver_chirho.instances_chirho.insert(
        instance_chirho,
        InstanceInfoChirho {
            name_chirho: instance_chirho,
            class_chirho: NameIdChirho(1),
            binders_chirho: vec![binder_chirho],
            context_chirho: vec![predicate_chirho],
            head_chirho,
            scope_chirho,
            methods_chirho: Vec::new(),
        },
    );
    instance_chirho
}

#[test]
fn instantiated_instance_evidence_checks_context_identity_and_scope_before_publication_chirho() {
    let (mut solver_chirho, bool_predicate_chirho) = solver_with_class_chirho(20_000);
    let instance_chirho = register_list_instance_chirho(&mut solver_chirho);
    let root_chirho = solver_chirho.root_chirho();
    let bool_chirho = solver_chirho
        .arena_chirho
        .con_chirho(crate::TyConChirho::BoolChirho)
        .unwrap();
    let char_chirho = solver_chirho
        .arena_chirho
        .con_chirho(crate::TyConChirho::CharChirho)
        .unwrap();
    let char_predicate_chirho = solver_chirho
        .predicate_chirho(NameIdChirho(1), char_chirho)
        .unwrap();
    let list_chirho = solver_chirho.arena_chirho.list_chirho(bool_chirho).unwrap();
    let list_predicate_chirho = solver_chirho
        .predicate_chirho(NameIdChirho(1), list_chirho)
        .unwrap();
    let wanted_chirho = solver_chirho
        .want_dictionary_chirho(root_chirho, list_predicate_chirho, SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    let wrong_chirho = solver_chirho
        .given_dictionary_chirho(root_chirho, char_predicate_chirho)
        .unwrap();
    let child_chirho = solver_chirho
        .implication_chirho(root_chirho, false)
        .unwrap();
    let escaping_chirho = solver_chirho
        .given_dictionary_chirho(child_chirho, bool_predicate_chirho)
        .unwrap();
    for argument_chirho in [wrong_chirho, escaping_chirho] {
        let error_chirho = solver_chirho
            .bind_evidence_chirho(
                wanted_chirho,
                EvTermChirho::InstanceChirho {
                    instance_chirho,
                    type_arguments_chirho: vec![bool_chirho],
                    dictionary_arguments_chirho: vec![argument_chirho],
                },
            )
            .unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            TcErrorKindChirho::InvalidReferenceChirho
        );
        assert!(
            solver_chirho
                .evidence_term_chirho(wanted_chirho)
                .unwrap()
                .is_none()
        );
        assert!(
            solver_chirho
                .scope_bindings_chirho(root_chirho)
                .unwrap()
                .is_empty()
        );
    }
    let valid_chirho = solver_chirho
        .given_dictionary_chirho(root_chirho, bool_predicate_chirho)
        .unwrap();
    solver_chirho
        .bind_evidence_chirho(
            wanted_chirho,
            EvTermChirho::InstanceChirho {
                instance_chirho,
                type_arguments_chirho: vec![bool_chirho],
                dictionary_arguments_chirho: vec![valid_chirho],
            },
        )
        .unwrap();
    assert_eq!(
        solver_chirho.scope_bindings_chirho(root_chirho).unwrap(),
        &[wanted_chirho]
    );
}

#[test]
fn instance_subgoals_share_the_module_ceiling_and_never_specialize_a_flexible_head_chirho() {
    let (mut solver_chirho, bool_predicate_chirho) = solver_with_class_chirho(600);
    register_list_instance_chirho(&mut solver_chirho);
    let root_chirho = solver_chirho.root_chirho();
    solver_chirho
        .given_dictionary_chirho(root_chirho, bool_predicate_chirho)
        .unwrap();
    let type_chirho = solver_chirho
        .arena_chirho
        .con_chirho(crate::TyConChirho::TypeChirho)
        .unwrap();
    let flexible_chirho = solver_chirho
        .arena_chirho
        .fresh_meta_chirho(0, type_chirho)
        .unwrap();
    let flexible_predicate_chirho = solver_chirho
        .predicate_chirho(NameIdChirho(1), flexible_chirho)
        .unwrap();
    let flexible_wanted_chirho = solver_chirho
        .want_dictionary_chirho(
            root_chirho,
            flexible_predicate_chirho,
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
    solver_chirho.solve_dictionaries_chirho().unwrap();
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(flexible_chirho)
            .unwrap(),
        flexible_chirho
    );
    assert!(
        solver_chirho
            .evidence_term_chirho(flexible_wanted_chirho)
            .unwrap()
            .is_none()
    );
    let mut nested_chirho = solver_chirho
        .arena_chirho
        .con_chirho(crate::TyConChirho::BoolChirho)
        .unwrap();
    for _depth_chirho in 0..20 {
        nested_chirho = solver_chirho
            .arena_chirho
            .list_chirho(nested_chirho)
            .unwrap();
    }
    let predicate_chirho = solver_chirho
        .predicate_chirho(NameIdChirho(1), nested_chirho)
        .unwrap();
    solver_chirho
        .want_dictionary_chirho(root_chirho, predicate_chirho, SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    assert_eq!(
        solver_chirho
            .solve_dictionaries_chirho()
            .unwrap_err()
            .kind_chirho,
        TcErrorKindChirho::ResourceChirho
    );
    assert_eq!(
        solver_chirho
            .solve_dictionaries_chirho()
            .unwrap_err()
            .kind_chirho,
        TcErrorKindChirho::ResourceChirho
    );
}
