// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use crate::*;
use haskelujah_span_chirho::SpanChirho;

fn equate_chirho(solver_chirho: &mut SolverChirho, left_chirho: TyChirho, right_chirho: TyChirho) {
    solver_chirho.want_chirho(EqualityChirho {
        left_chirho,
        right_chirho,
        scope_chirho: solver_chirho.root_chirho(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    });
}

#[test]
fn quantification_keeps_environment_variables_shared_and_opens_quantified_variables_freshly_chirho()
{
    let mut solver_chirho = SolverChirho::new_chirho(200);
    let arena_chirho = &mut solver_chirho.arena_chirho;
    let kind_chirho = arena_chirho.con_chirho(TyConChirho::TypeChirho).unwrap();
    let outer_chirho = arena_chirho.fresh_meta_chirho(0, kind_chirho).unwrap();
    let local_chirho = arena_chirho.fresh_meta_chirho(1, kind_chirho).unwrap();
    let body_chirho = arena_chirho
        .function_chirho(outer_chirho, local_chirho)
        .unwrap();
    let quantified_chirho = arena_chirho
        .quantify_group_chirho(&[body_chirho], 0)
        .unwrap();
    assert_eq!(quantified_chirho, vec![local_chirho]);
    let scheme_chirho = SchemeChirho {
        body_chirho,
        quantified_chirho,
    };
    let (_, first_chirho) = arena_chirho.instantiate_chirho(&scheme_chirho, 0).unwrap();
    let (_, second_chirho) = arena_chirho.instantiate_chirho(&scheme_chirho, 0).unwrap();
    assert_ne!(first_chirho, second_chirho);
    let bool_chirho = arena_chirho.con_chirho(TyConChirho::BoolChirho).unwrap();
    let char_chirho = arena_chirho.con_chirho(TyConChirho::CharChirho).unwrap();
    equate_chirho(&mut solver_chirho, first_chirho[0], bool_chirho);
    equate_chirho(&mut solver_chirho, second_chirho[0], char_chirho);
    equate_chirho(&mut solver_chirho, outer_chirho, bool_chirho);
    solver_chirho.solve_chirho().unwrap();
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(first_chirho[0])
            .unwrap(),
        bool_chirho
    );
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(second_chirho[0])
            .unwrap(),
        char_chirho
    );
    assert_eq!(
        solver_chirho
            .arena_chirho
            .resolve_chirho(local_chirho)
            .unwrap(),
        local_chirho
    );
}

#[test]
fn a_published_quantified_variable_cannot_be_assigned_or_smuggled_into_a_new_meta_chirho() {
    for reverse_chirho in [false, true] {
        let mut solver_chirho = SolverChirho::new_chirho(100);
        let kind_chirho = solver_chirho
            .arena_chirho
            .con_chirho(TyConChirho::TypeChirho)
            .unwrap();
        let generic_chirho = solver_chirho
            .arena_chirho
            .fresh_meta_chirho(1, kind_chirho)
            .unwrap();
        solver_chirho
            .arena_chirho
            .quantify_group_chirho(&[generic_chirho], 0)
            .unwrap();
        let fresh_chirho = solver_chirho
            .arena_chirho
            .fresh_meta_chirho(0, kind_chirho)
            .unwrap();
        let bool_chirho = solver_chirho
            .arena_chirho
            .con_chirho(TyConChirho::BoolChirho)
            .unwrap();
        if reverse_chirho {
            equate_chirho(&mut solver_chirho, fresh_chirho, generic_chirho);
        } else {
            equate_chirho(&mut solver_chirho, generic_chirho, bool_chirho);
        }
        assert_eq!(
            solver_chirho.solve_chirho().unwrap_err().kind_chirho,
            TcErrorKindChirho::TypeMismatchChirho
        );
        assert_eq!(
            solver_chirho
                .arena_chirho
                .resolve_chirho(generic_chirho)
                .unwrap(),
            generic_chirho
        );
        assert_eq!(
            solver_chirho
                .arena_chirho
                .resolve_chirho(fresh_chirho)
                .unwrap(),
            fresh_chirho
        );
    }
}
