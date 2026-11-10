// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use haskelujah_span_chirho::SpanChirho;

use crate::*;

fn wanted_chirho(
    solver_chirho: &mut SolverChirho,
    left_chirho: TyChirho,
    right_chirho: TyChirho,
    scope_chirho: ScopeIdChirho,
) {
    solver_chirho.want_chirho(EqualityChirho {
        left_chirho,
        right_chirho,
        scope_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    });
}

#[test]
fn late_mismatch_rolls_back_the_equation_and_failure_cannot_be_forgotten_chirho() {
    let mut solver_chirho = SolverChirho::new_chirho(100);
    let kind_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::TypeChirho)
        .unwrap();
    let meta_chirho = solver_chirho
        .arena_chirho
        .fresh_meta_chirho(0, kind_chirho)
        .unwrap();
    let bool_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::BoolChirho)
        .unwrap();
    let char_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::CharChirho)
        .unwrap();
    let left_chirho = solver_chirho
        .arena_chirho
        .intern_chirho(TyNodeChirho::TupleChirho(vec![meta_chirho, bool_chirho]))
        .unwrap();
    let right_chirho = solver_chirho
        .arena_chirho
        .intern_chirho(TyNodeChirho::TupleChirho(vec![char_chirho, char_chirho]))
        .unwrap();
    let scope_chirho = solver_chirho.root_chirho();
    wanted_chirho(&mut solver_chirho, left_chirho, right_chirho, scope_chirho);
    assert_eq!(
        solver_chirho.solve_chirho().unwrap_err().kind_chirho,
        TcErrorKindChirho::TypeMismatchChirho
    );
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(meta_chirho)
            .unwrap(),
        meta_chirho
    );
    assert!(solver_chirho.solve_chirho().is_err());
}

#[test]
fn implication_given_equalities_keep_outer_metas_untouchable_chirho() {
    let mut solver_chirho = SolverChirho::new_chirho(100);
    let kind_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::TypeChirho)
        .unwrap();
    let outer_chirho = solver_chirho
        .arena_chirho
        .fresh_meta_chirho(0, kind_chirho)
        .unwrap();
    let bool_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::BoolChirho)
        .unwrap();
    let scope_chirho = solver_chirho
        .implication_chirho(solver_chirho.root_chirho(), true)
        .unwrap();
    wanted_chirho(&mut solver_chirho, outer_chirho, bool_chirho, scope_chirho);
    assert_eq!(
        solver_chirho.solve_chirho().unwrap_err().kind_chirho,
        TcErrorKindChirho::UntouchableChirho
    );
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(outer_chirho)
            .unwrap(),
        outer_chirho
    );
}

#[test]
fn inner_metas_are_touchable_but_younger_skolems_cannot_escape_chirho() {
    let mut solver_chirho = SolverChirho::new_chirho(100);
    let kind_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::TypeChirho)
        .unwrap();
    let inner_chirho = solver_chirho
        .arena_chirho
        .fresh_meta_chirho(1, kind_chirho)
        .unwrap();
    let bool_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::BoolChirho)
        .unwrap();
    let scope_chirho = solver_chirho
        .implication_chirho(solver_chirho.root_chirho(), true)
        .unwrap();
    wanted_chirho(&mut solver_chirho, inner_chirho, bool_chirho, scope_chirho);
    solver_chirho.solve_chirho().unwrap();
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(inner_chirho)
            .unwrap(),
        bool_chirho
    );

    let outer_chirho = solver_chirho
        .arena_chirho
        .fresh_meta_chirho(0, kind_chirho)
        .unwrap();
    let (_, skolem_chirho) = solver_chirho
        .arena_chirho
        .fresh_rigid_chirho(1, kind_chirho)
        .unwrap();
    let root_chirho = solver_chirho.root_chirho();
    wanted_chirho(&mut solver_chirho, outer_chirho, skolem_chirho, root_chirho);
    assert_eq!(
        solver_chirho.solve_chirho().unwrap_err().kind_chirho,
        TcErrorKindChirho::SkolemEscapeChirho
    );
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(outer_chirho)
            .unwrap(),
        outer_chirho
    );
}

#[test]
fn ordinary_metas_remain_monotypes_and_occurs_check_is_required_chirho() {
    let mut solver_chirho = SolverChirho::new_chirho(100);
    let kind_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::TypeChirho)
        .unwrap();
    let meta_chirho = solver_chirho
        .arena_chirho
        .fresh_meta_chirho(0, kind_chirho)
        .unwrap();
    let (binder_chirho, rigid_chirho) = solver_chirho
        .arena_chirho
        .fresh_rigid_chirho(0, kind_chirho)
        .unwrap();
    let forall_chirho = solver_chirho
        .arena_chirho
        .intern_chirho(TyNodeChirho::ForallChirho {
            binder_chirho,
            body_chirho: rigid_chirho,
        })
        .unwrap();
    let scope_chirho = solver_chirho.root_chirho();
    wanted_chirho(&mut solver_chirho, meta_chirho, forall_chirho, scope_chirho);
    assert_eq!(
        solver_chirho.solve_chirho().unwrap_err().kind_chirho,
        TcErrorKindChirho::TypeMismatchChirho
    );

    let mut solver_chirho = SolverChirho::new_chirho(100);
    let kind_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::TypeChirho)
        .unwrap();
    let meta_chirho = solver_chirho
        .arena_chirho
        .fresh_meta_chirho(0, kind_chirho)
        .unwrap();
    let function_chirho = solver_chirho
        .arena_chirho
        .function_chirho(meta_chirho, meta_chirho)
        .unwrap();
    let scope_chirho = solver_chirho.root_chirho();
    wanted_chirho(
        &mut solver_chirho,
        meta_chirho,
        function_chirho,
        scope_chirho,
    );
    assert_eq!(
        solver_chirho.solve_chirho().unwrap_err().kind_chirho,
        TcErrorKindChirho::InfiniteTypeChirho
    );
}

#[test]
fn arena_and_scope_ids_never_transfer_another_modules_authority_chirho() {
    let mut foreign_chirho = SolverChirho::new_chirho(100);
    let foreign_bool_chirho = foreign_chirho
        .arena_chirho
        .con_chirho(TyConChirho::BoolChirho)
        .unwrap();
    let mut local_chirho = SolverChirho::new_chirho(100);
    let scope_chirho = local_chirho.root_chirho();
    wanted_chirho(
        &mut local_chirho,
        foreign_bool_chirho,
        foreign_bool_chirho,
        scope_chirho,
    );
    assert_eq!(
        local_chirho.solve_chirho().unwrap_err().kind_chirho,
        TcErrorKindChirho::InvalidReferenceChirho
    );

    let mut local_chirho = SolverChirho::new_chirho(100);
    let bool_chirho = local_chirho
        .arena_chirho
        .con_chirho(TyConChirho::BoolChirho)
        .unwrap();
    wanted_chirho(
        &mut local_chirho,
        bool_chirho,
        bool_chirho,
        foreign_chirho.root_chirho(),
    );
    assert_eq!(
        local_chirho.solve_chirho().unwrap_err().kind_chirho,
        TcErrorKindChirho::InvalidReferenceChirho
    );
}

#[test]
fn repeated_nodes_are_shared_but_fresh_growth_has_a_module_ceiling_chirho() {
    let mut arena_chirho = TypeArenaChirho::new_chirho(2);
    let kind_chirho = arena_chirho.con_chirho(TyConChirho::TypeChirho).unwrap();
    for _repeat_chirho in 0..100 {
        assert_eq!(
            arena_chirho.con_chirho(TyConChirho::TypeChirho).unwrap(),
            kind_chirho
        );
    }
    assert_eq!(arena_chirho.allocation_count_chirho(), 1);
    assert_eq!(
        arena_chirho
            .fresh_meta_chirho(0, kind_chirho)
            .unwrap_err()
            .kind_chirho,
        TcErrorKindChirho::ResourceChirho
    );
}

#[test]
fn constraint_and_implication_growth_share_the_type_allocation_ceiling_chirho() {
    let mut solver_chirho = SolverChirho::new_chirho(4);
    let bool_chirho = solver_chirho
        .arena_chirho
        .con_chirho(TyConChirho::BoolChirho)
        .unwrap();
    let root_chirho = solver_chirho.root_chirho();
    solver_chirho
        .implication_chirho(root_chirho, false)
        .unwrap();
    wanted_chirho(&mut solver_chirho, bool_chirho, bool_chirho, root_chirho);
    wanted_chirho(&mut solver_chirho, bool_chirho, bool_chirho, root_chirho);
    assert_eq!(
        solver_chirho.solve_chirho().unwrap_err().kind_chirho,
        TcErrorKindChirho::ResourceChirho
    );
    assert_eq!(solver_chirho.arena_chirho.allocation_count_chirho(), 4);
    assert!(
        solver_chirho
            .implication_chirho(root_chirho, false)
            .is_err()
    );
}
