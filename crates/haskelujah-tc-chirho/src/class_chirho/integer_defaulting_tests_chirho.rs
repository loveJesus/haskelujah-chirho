// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn solver_chirho(ceiling_chirho: usize) -> (SolverChirho, TyChirho, TyChirho) {
    let mut solver_chirho = SolverChirho::new_chirho(ceiling_chirho);
    for class_chirho in [NameIdChirho(1), NameIdChirho(3)] {
        solver_chirho
            .arena_chirho
            .declare_class_chirho(class_chirho)
            .unwrap();
    }
    let type_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::TypeChirho)
        .unwrap();
    for class_chirho in [NameIdChirho(1), NameIdChirho(3)] {
        let (_, parameter_chirho) = solver_chirho
            .arena_chirho
            .fresh_rigid_chirho(0, type_chirho)
            .unwrap();
        solver_chirho.classes_chirho.insert(
            class_chirho,
            ClassInfoChirho {
                name_chirho: class_chirho,
                parameter_chirho,
                superclasses_chirho: Vec::new(),
                superclass_selectors_chirho: Vec::new(),
                methods_chirho: Vec::new(),
            },
        );
    }
    let variable_chirho = solver_chirho
        .arena_chirho
        .fresh_meta_chirho(1, type_chirho)
        .unwrap();
    let integer_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::IntegerChirho)
        .unwrap();
    let root_chirho = solver_chirho.root_chirho();
    solver_chirho
        .register_instance_head_chirho(NameIdChirho(1), integer_chirho, &[], NameIdChirho(2))
        .unwrap();
    solver_chirho.instances_chirho.insert(
        NameIdChirho(2),
        InstanceInfoChirho {
            name_chirho: NameIdChirho(2),
            class_chirho: NameIdChirho(1),
            binders_chirho: Vec::new(),
            context_chirho: Vec::new(),
            head_chirho: integer_chirho,
            scope_chirho: root_chirho,
            superclasses_chirho: Vec::new(),
            methods_chirho: Vec::new(),
        },
    );
    (solver_chirho, variable_chirho, integer_chirho)
}

fn wanted_chirho(
    solver_chirho: &mut SolverChirho,
    class_chirho: NameIdChirho,
    argument_chirho: TyChirho,
    scope_chirho: ScopeIdChirho,
) -> EvVarChirho {
    let predicate_chirho = solver_chirho
        .predicate_chirho(class_chirho, argument_chirho)
        .unwrap();
    solver_chirho
        .want_dictionary_chirho(scope_chirho, predicate_chirho, SpanChirho::DUMMY_CHIRHO)
        .unwrap()
}

#[test]
fn integer_defaulting_preserves_the_wanted_scope_and_uses_checked_instance_evidence_chirho() {
    let (mut solver_chirho, variable_chirho, integer_chirho) = solver_chirho(1000);
    let root_chirho = solver_chirho.root_chirho();
    let child_chirho = solver_chirho
        .implication_chirho(root_chirho, false)
        .unwrap();
    let evidence_chirho = wanted_chirho(
        &mut solver_chirho,
        NameIdChirho(1),
        variable_chirho,
        child_chirho,
    );
    solver_chirho
        .default_integer_chirho(NameIdChirho(1))
        .unwrap();
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(variable_chirho)
            .unwrap(),
        integer_chirho
    );
    assert!(matches!(
        solver_chirho.evidence_term_chirho(evidence_chirho).unwrap(),
        Some(EvTermChirho::InstanceChirho {
            instance_chirho: NameIdChirho(2),
            ..
        })
    ));
    assert_eq!(
        solver_chirho.scope_bindings_chirho(child_chirho).unwrap(),
        &[evidence_chirho]
    );
    assert!(
        solver_chirho
            .scope_bindings_chirho(root_chirho)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn another_class_or_a_non_unary_occurrence_blocks_the_same_default_variable_chirho() {
    for non_unary_chirho in [false, true] {
        let (mut solver_chirho, variable_chirho, _) = solver_chirho(1000);
        let root_chirho = solver_chirho.root_chirho();
        let scope_chirho = solver_chirho
            .implication_chirho(root_chirho, false)
            .unwrap();
        wanted_chirho(
            &mut solver_chirho,
            NameIdChirho(1),
            variable_chirho,
            scope_chirho,
        );
        let argument_chirho = if non_unary_chirho {
            solver_chirho
                .arena_chirho
                .list_chirho(variable_chirho)
                .unwrap()
        } else {
            variable_chirho
        };
        wanted_chirho(
            &mut solver_chirho,
            NameIdChirho(3),
            argument_chirho,
            scope_chirho,
        );
        solver_chirho
            .default_integer_chirho(NameIdChirho(1))
            .unwrap();
        assert_eq!(
            solver_chirho
                .arena_chirho
                .resolve_chirho(variable_chirho)
                .unwrap(),
            variable_chirho
        );
        assert_eq!(
            solver_chirho
                .unresolved_dictionaries_chirho()
                .unwrap()
                .len(),
            2
        );
    }
}

#[test]
fn generalized_variables_and_signature_skolems_cannot_be_defaulted_chirho() {
    let (mut solver_chirho, variable_chirho, _) = solver_chirho(1000);
    solver_chirho
        .arena_chirho
        .quantify_group_chirho(&[variable_chirho], 0)
        .unwrap();
    let type_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::TypeChirho)
        .unwrap();
    let (_, rigid_chirho) = solver_chirho
        .arena_chirho
        .fresh_rigid_chirho(1, type_chirho)
        .unwrap();
    let root_chirho = solver_chirho.root_chirho();
    for argument_chirho in [variable_chirho, rigid_chirho] {
        wanted_chirho(
            &mut solver_chirho,
            NameIdChirho(1),
            argument_chirho,
            root_chirho,
        );
    }
    solver_chirho
        .default_integer_chirho(NameIdChirho(1))
        .unwrap();
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(variable_chirho)
            .unwrap(),
        variable_chirho
    );
    assert_eq!(
        solver_chirho
            .unresolved_dictionaries_chirho()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn a_missing_candidate_or_unproved_instance_context_never_fills_a_meta_chirho() {
    for missing_chirho in [true, false] {
        let (mut solver_chirho, variable_chirho, integer_chirho) = solver_chirho(1000);
        let root_chirho = solver_chirho.root_chirho();
        let scope_chirho = solver_chirho
            .implication_chirho(root_chirho, false)
            .unwrap();
        wanted_chirho(
            &mut solver_chirho,
            NameIdChirho(1),
            variable_chirho,
            scope_chirho,
        );
        if missing_chirho {
            solver_chirho.constructor_instances_chirho.clear();
        } else {
            let context_chirho = solver_chirho
                .predicate_chirho(NameIdChirho(3), integer_chirho)
                .unwrap();
            solver_chirho
                .instances_chirho
                .get_mut(&NameIdChirho(2))
                .unwrap()
                .context_chirho
                .push(context_chirho);
        }
        solver_chirho
            .default_integer_chirho(NameIdChirho(1))
            .unwrap();
        assert_eq!(
            solver_chirho
                .arena_chirho
                .resolve_chirho(variable_chirho)
                .unwrap(),
            variable_chirho
        );
        assert_eq!(
            solver_chirho
                .unresolved_dictionaries_chirho()
                .unwrap()
                .len(),
            1
        );
    }
}

#[test]
fn exhausted_defaulting_is_a_resource_error_before_any_assignment_chirho() {
    let (mut solver_chirho, variable_chirho, _) = solver_chirho(100);
    let root_chirho = solver_chirho.root_chirho();
    wanted_chirho(
        &mut solver_chirho,
        NameIdChirho(1),
        variable_chirho,
        root_chirho,
    );
    while solver_chirho
        .arena_chirho
        .reserve_auxiliary_chirho()
        .is_ok()
    {}
    assert_eq!(
        solver_chirho
            .default_integer_chirho(NameIdChirho(1))
            .unwrap_err()
            .kind_chirho,
        TcErrorKindChirho::ResourceChirho
    );
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(variable_chirho)
            .unwrap(),
        variable_chirho
    );
}
