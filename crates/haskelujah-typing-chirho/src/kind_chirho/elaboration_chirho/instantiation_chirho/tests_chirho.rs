// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn published_nested_forall_cannot_capture_a_fresh_top_image_chirho() {
    for required_chirho in [false, true] {
        let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
        state_chirho.next_variable_chirho = 20;
        let top_chirho = state_chirho
            .fresh_chirho(None, SpanChirho::DUMMY_CHIRHO)
            .unwrap();
        let mut parameters_chirho = SubstChirho::empty_chirho();
        parameters_chirho.insert_chirho(TyVarChirho(0), TyChirho::VarChirho(top_chirho));
        let body_chirho = Box::new(TyChirho::fun_chirho(
            TyChirho::VarChirho(TyVarChirho(0)),
            TyChirho::VarChirho(TyVarChirho(20)),
        ));
        let term_chirho = if required_chirho {
            TyChirho::RequiredForallChirho {
                vars_chirho: vec![TyVarChirho(20)].into(),
                body_chirho,
            }
        } else {
            TyChirho::ForallChirho {
                vars_chirho: vec![TyVarChirho(20)].into(),
                body_chirho,
            }
        };
        let result_chirho = state_chirho
            .instantiate_term_chirho(
                &term_chirho,
                &parameters_chirho,
                false,
                SpanChirho::DUMMY_CHIRHO,
            )
            .unwrap();
        assert_eq!(result_chirho.free_vars_chirho(), vec![top_chirho]);
        let expected_body_chirho = Box::new(TyChirho::fun_chirho(
            TyChirho::VarChirho(top_chirho),
            TyChirho::VarChirho(TyVarChirho(21)),
        ));
        let expected_chirho = if required_chirho {
            TyChirho::RequiredForallChirho {
                vars_chirho: vec![TyVarChirho(21)].into(),
                body_chirho: expected_body_chirho,
            }
        } else {
            TyChirho::ForallChirho {
                vars_chirho: vec![TyVarChirho(21)].into(),
                body_chirho: expected_body_chirho,
            }
        };
        assert_eq!(result_chirho, expected_chirho);
    }
}

#[test]
fn published_free_slots_never_read_the_consumers_substitution_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    state_chirho.substitution_chirho.insert_chirho(
        TyVarChirho(99),
        ElaborationStateChirho::type_universe_chirho(),
    );
    assert!(
        state_chirho
            .instantiate_term_chirho(
                &TyChirho::VarChirho(TyVarChirho(99)),
                &SubstChirho::empty_chirho(),
                false,
                SpanChirho::DUMMY_CHIRHO
            )
            .is_err()
    );
}
#[test]
fn universal_kind_scope_prevents_direct_and_indirect_escape_chirho() {
    for indirect_chirho in [false, true] {
        let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
        let outer_chirho = state_chirho
            .fresh_chirho(None, SpanChirho::DUMMY_CHIRHO)
            .unwrap();
        let universal_chirho = state_chirho
            .fresh_chirho(
                Some(ElaborationStateChirho::type_universe_chirho()),
                SpanChirho::DUMMY_CHIRHO,
            )
            .unwrap();
        let cutoff_chirho = state_chirho.next_variable_chirho;
        let mark_chirho = state_chirho
            .open_kind_guard_chirho(&[universal_chirho], cutoff_chirho, SpanChirho::DUMMY_CHIRHO)
            .unwrap();
        let target_chirho = if indirect_chirho {
            let fresh_chirho = state_chirho
                .fresh_chirho(None, SpanChirho::DUMMY_CHIRHO)
                .unwrap();
            state_chirho
                .unify_chirho(
                    &TyChirho::VarChirho(outer_chirho),
                    &TyChirho::VarChirho(fresh_chirho),
                    SpanChirho::DUMMY_CHIRHO,
                )
                .unwrap();
            fresh_chirho
        } else {
            outer_chirho
        };
        let error_chirho = state_chirho
            .unify_chirho(
                &TyChirho::VarChirho(target_chirho),
                &TyChirho::VarChirho(universal_chirho),
                SpanChirho::DUMMY_CHIRHO,
            )
            .unwrap_err();
        assert!(
            error_chirho
                .message_chirho
                .contains("universal binder escaped")
        );
        state_chirho.close_kind_guard_chirho(mark_chirho);
        state_chirho
            .unify_chirho(
                &TyChirho::VarChirho(target_chirho),
                &ElaborationStateChirho::type_universe_chirho(),
                SpanChirho::DUMMY_CHIRHO,
            )
            .expect("closing the failed scope restores ordinary solving");
    }
}
