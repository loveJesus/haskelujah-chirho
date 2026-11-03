// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Kind algebra: display, arrows, free variables, substitution, unification, the occurs
//! check, the builtin environment, AST-kind conversion, arity and the variable cache.

use super::*;

// -- Kind representation tests --

#[test]
fn star_display_chirho() {
    assert_eq!(KindChirho::StarChirho.to_string(), "*");
}

#[test]
fn arrow_display_chirho() {
    let k_chirho = KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
    assert_eq!(k_chirho.to_string(), "* -> *");
}

#[test]
fn required_forall_kind_keeps_its_visible_parameter_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
    let type_kind_chirho = TypeChirho::ConChirho(mk_name_chirho("Type"));
    let required_forall_chirho = TypeChirho::RequiredForallChirho {
        vars_chirho: vec![TyVarChirho::annotated_chirho(
            mk_name_chirho("kindChirho"),
            AstKindChirho::StarChirho,
        )],
        body_chirho: Box::new(mk_fun_chirho(type_kind_chirho.clone(), type_kind_chirho)),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    assert_eq!(
        ctx_chirho.type_to_kind_chirho(&required_forall_chirho),
        KindChirho::arrow_n_chirho(
            [KindChirho::StarChirho, KindChirho::StarChirho],
            KindChirho::StarChirho,
        )
    );
}

#[test]
fn arrow_display_nested_chirho() {
    // (* -> *) -> *
    let k_chirho = KindChirho::arrow_chirho(
        KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
        KindChirho::StarChirho,
    );
    assert_eq!(k_chirho.to_string(), "(* -> *) -> *");
}

#[test]
fn arrow_n_builds_curried_chirho() {
    let k_chirho = KindChirho::arrow_n_chirho(
        vec![KindChirho::StarChirho, KindChirho::StarChirho],
        KindChirho::StarChirho,
    );
    assert_eq!(k_chirho.to_string(), "* -> * -> *");
}

#[test]
fn free_vars_chirho() {
    let v_chirho = KindVarChirho(0);
    let k_chirho =
        KindChirho::arrow_chirho(KindChirho::VarChirho(v_chirho), KindChirho::StarChirho);
    assert_eq!(k_chirho.free_vars_chirho(), vec![v_chirho]);
}

// -- Kind substitution tests --

#[test]
fn subst_applies_chirho() {
    let v_chirho = KindVarChirho(0);
    let subst_chirho = KindSubstChirho::singleton_chirho(v_chirho, KindChirho::StarChirho);
    let result_chirho = subst_chirho.apply_chirho(&KindChirho::VarChirho(v_chirho));
    assert_eq!(result_chirho, KindChirho::StarChirho);
}

#[test]
fn subst_compose_chirho() {
    let v0_chirho = KindVarChirho(0);
    let v1_chirho = KindVarChirho(1);
    let s1_chirho = KindSubstChirho::singleton_chirho(v0_chirho, KindChirho::VarChirho(v1_chirho));
    let s2_chirho = KindSubstChirho::singleton_chirho(v1_chirho, KindChirho::StarChirho);
    let composed_chirho = s2_chirho.compose_chirho(&s1_chirho);
    // v0 should map to * (through v1)
    assert_eq!(
        composed_chirho.apply_chirho(&KindChirho::VarChirho(v0_chirho)),
        KindChirho::StarChirho
    );
}

// -- Kind unification tests --

#[test]
fn unify_star_star_chirho() {
    let result_chirho = unify_kind_chirho(
        &KindChirho::StarChirho,
        &KindChirho::StarChirho,
        "test",
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(result_chirho.is_ok());
    assert!(result_chirho.unwrap().is_empty_chirho());
}

#[test]
fn unify_var_with_star_chirho() {
    let v_chirho = KindVarChirho(0);
    let result_chirho = unify_kind_chirho(
        &KindChirho::VarChirho(v_chirho),
        &KindChirho::StarChirho,
        "test",
        SpanChirho::DUMMY_CHIRHO,
    )
    .unwrap();
    assert_eq!(
        result_chirho.apply_chirho(&KindChirho::VarChirho(v_chirho)),
        KindChirho::StarChirho
    );
}

#[test]
fn unify_arrow_kinds_chirho() {
    let v_chirho = KindVarChirho(0);
    let k1_chirho =
        KindChirho::arrow_chirho(KindChirho::VarChirho(v_chirho), KindChirho::StarChirho);
    let k2_chirho = KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
    let result_chirho =
        unify_kind_chirho(&k1_chirho, &k2_chirho, "test", SpanChirho::DUMMY_CHIRHO).unwrap();
    assert_eq!(
        result_chirho.apply_chirho(&KindChirho::VarChirho(v_chirho)),
        KindChirho::StarChirho
    );
}

#[test]
fn unify_mismatch_chirho() {
    // Star and Constraint now unify (ConstraintKinds behavior).
    let result_chirho = unify_kind_chirho(
        &KindChirho::StarChirho,
        &KindChirho::ConstraintChirho,
        "test",
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(result_chirho.is_ok());

    // Arrow vs Star is a genuine mismatch.
    let result2_chirho = unify_kind_chirho(
        &KindChirho::StarChirho,
        &KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
        "test",
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(matches!(
        result2_chirho,
        Err(KindErrorChirho::MismatchChirho { .. })
    ));
}

#[test]
fn occurs_check_chirho() {
    let v_chirho = KindVarChirho(0);
    let result_chirho = unify_kind_chirho(
        &KindChirho::VarChirho(v_chirho),
        &KindChirho::arrow_chirho(KindChirho::VarChirho(v_chirho), KindChirho::StarChirho),
        "test",
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(matches!(
        result_chirho,
        Err(KindErrorChirho::OccursCheckChirho { .. })
    ));
}

// -- Kind environment tests --

#[test]
fn builtins_have_correct_kinds_chirho() {
    let env_chirho = KindEnvChirho::with_builtins_chirho();
    assert_eq!(
        env_chirho.lookup_chirho("Int"),
        Some(&KindChirho::StarChirho)
    );
    assert_eq!(
        env_chirho.lookup_chirho("Int64"),
        Some(&KindChirho::StarChirho)
    );
    assert_eq!(
        env_chirho.lookup_chirho("Word32"),
        Some(&KindChirho::StarChirho)
    );
    assert_eq!(
        env_chirho.lookup_chirho("Word64"),
        Some(&KindChirho::StarChirho)
    );
    assert_eq!(
        env_chirho.lookup_chirho("Maybe"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::StarChirho
        ))
    );
    assert_eq!(
        env_chirho.lookup_chirho("Either"),
        Some(&KindChirho::arrow_n_chirho(
            vec![KindChirho::StarChirho, KindChirho::StarChirho],
            KindChirho::StarChirho
        ))
    );
    assert_eq!(
        env_chirho.lookup_chirho("GHC.TypeNats.*"),
        Some(&KindChirho::arrow_n_chirho(
            vec![KindChirho::StarChirho, KindChirho::StarChirho],
            KindChirho::StarChirho
        )),
        "qualified TypeNats operator lookup should reuse the bare builtin family kind"
    );
    assert_eq!(
        env_chirho.lookup_chirho("AppendSymbol"),
        Some(&KindChirho::arrow_n_chirho(
            vec![KindChirho::StarChirho, KindChirho::StarChirho],
            KindChirho::StarChirho
        ))
    );
    for family_chirho in ["CharToNat", "GHC.TypeLits.NatToChar"] {
        assert_eq!(
            env_chirho.lookup_chirho(family_chirho),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::StarChirho
            )),
            "{family_chirho} should have a unary TypeLits family kind"
        );
    }
    assert!(
        matches!(
            env_chirho.lookup_chirho(":"),
            Some(KindChirho::ArrowChirho(_, tail_chirho))
                if matches!(
                    tail_chirho.as_ref(),
                    KindChirho::ArrowChirho(tail_arg_chirho, result_chirho)
                        if matches!(tail_arg_chirho.as_ref(), KindChirho::StarChirho)
                            && matches!(result_chirho.as_ref(), KindChirho::StarChirho)
                )
        ),
        "promoted list cons should accept an element and flattened tail"
    );
}

#[test]
fn default_unconstrained_vars_chirho() {
    let k_chirho = KindChirho::VarChirho(KindVarChirho(99));
    assert_eq!(default_kind_vars_chirho(&k_chirho), KindChirho::StarChirho);
}

#[test]
fn constraint_kind_display_chirho() {
    assert_eq!(KindChirho::ConstraintChirho.to_string(), "Constraint");
}

#[test]
fn ast_kind_constraint_converts_chirho() {
    let ast_chirho = AstKindChirho::ConstraintChirho;
    assert_eq!(
        ast_kind_to_kind_chirho(&ast_chirho),
        KindChirho::ConstraintChirho
    );
}

#[test]
fn ast_kind_constraint_arrow_converts_chirho() {
    let ast_chirho = AstKindChirho::ArrowChirho(
        Box::new(AstKindChirho::StarChirho),
        Box::new(AstKindChirho::ConstraintChirho),
    );
    let expected_chirho =
        KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::ConstraintChirho);
    assert_eq!(ast_kind_to_kind_chirho(&ast_chirho), expected_chirho);
}

#[test]
fn ast_kind_var_defaults_to_star_chirho() {
    // PolyKinds: standalone ast_kind_to_kind_chirho defaults kind vars to *
    let ast_chirho = AstKindChirho::VarChirho("k".to_string());
    assert_eq!(ast_kind_to_kind_chirho(&ast_chirho), KindChirho::StarChirho);
}

#[test]
fn kind_arity_is_final_only_when_tail_is_not_a_variable_chirho() {
    // `k -> *` has arity 1 even though the ARGUMENT kind is a variable —
    // only a variable TAIL leaves the arity open.
    let open_tail_chirho = KindChirho::arrow_chirho(
        KindChirho::StarChirho,
        KindChirho::VarChirho(KindVarChirho(0)),
    );
    assert_eq!(
        KindInferCtxChirho::kind_arity_chirho(&open_tail_chirho),
        (1, false)
    );
    let var_arg_chirho = KindChirho::arrow_chirho(
        KindChirho::VarChirho(KindVarChirho(0)),
        KindChirho::ConstraintChirho,
    );
    assert_eq!(
        KindInferCtxChirho::kind_arity_chirho(&var_arg_chirho),
        (1, true)
    );
}

#[test]
fn kind_var_cache_reuses_same_var_chirho() {
    // PolyKinds: context-aware conversion maps same name to same kind var
    let env_chirho = KindEnvChirho::new_chirho();
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(env_chirho);
    let k1_chirho =
        ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("k".to_string()));
    let k2_chirho =
        ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("k".to_string()));
    assert_eq!(k1_chirho, k2_chirho);
    // Different name gets different var
    let j_chirho =
        ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("j".to_string()));
    assert_ne!(k1_chirho, j_chirho);
}
