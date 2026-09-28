// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn family_chirho(argument_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("Not".to_owned())),
        Box::new(argument_chirho),
    )
}

#[test]
fn pattern_family_given_expires_with_its_branch_chirho() {
    for rigid_chirho in [false, true] {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let argument_chirho = if rigid_chirho {
            TyChirho::ForallVarChirho("aChirho%0".to_owned())
        } else {
            ctx_chirho.fresh_var_chirho()
        };
        let family_chirho = family_chirho(argument_chirho.clone());
        let true_chirho = TyChirho::ConChirho("True".to_owned());
        let previous_chirho = (
            TyChirho::int_chirho(),
            TyChirho::bool_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        );
        ctx_chirho
            .deferred_equalities_chirho
            .push(previous_chirho.clone());
        ctx_chirho.env_chirho.push_scope_chirho();
        ctx_chirho
            .unify_pattern_result_chirho(
                true,
                &true_chirho,
                &family_chirho,
                SpanChirho::DUMMY_CHIRHO,
            )
            .unwrap();
        assert_eq!(ctx_chirho.normalize_ty_chirho(&family_chirho), true_chirho);
        assert_eq!(
            ctx_chirho.normalize_ty_chirho(&argument_chirho),
            argument_chirho
        );
        assert_eq!(ctx_chirho.deferred_equalities_chirho, vec![previous_chirho]);
        ctx_chirho.env_chirho.pop_scope_chirho();
        assert_eq!(
            ctx_chirho.normalize_ty_chirho(&family_chirho),
            family_chirho
        );
    }
}

#[test]
fn family_result_given_never_equates_its_arguments_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let left_chirho = ctx_chirho.fresh_var_chirho();
    let right_chirho = ctx_chirho.fresh_var_chirho();
    let subst_chirho = ctx_chirho
        .unify_pattern_result_chirho(
            true,
            &family_chirho(left_chirho.clone()),
            &family_chirho(right_chirho.clone()),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
    assert_eq!(subst_chirho.apply_ty_chirho(&left_chirho), left_chirho);
    assert_eq!(subst_chirho.apply_ty_chirho(&right_chirho), right_chirho);
    assert_ne!(
        ctx_chirho.normalize_ty_chirho(&left_chirho),
        ctx_chirho.normalize_ty_chirho(&right_chirho)
    );
}
