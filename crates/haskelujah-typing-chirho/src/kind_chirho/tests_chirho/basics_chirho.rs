// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

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
            vec![runtime_chirho::builtin_term_chirho("Nat").unwrap(); 2],
            runtime_chirho::builtin_term_chirho("Nat").unwrap()
        )),
        "qualified TypeNats operator lookup should reuse the bare builtin family kind"
    );
    assert_eq!(
        env_chirho.lookup_chirho("AppendSymbol"),
        Some(&KindChirho::arrow_n_chirho(
            vec![runtime_chirho::builtin_term_chirho("Symbol").unwrap(); 2],
            runtime_chirho::builtin_term_chirho("Symbol").unwrap()
        ))
    );
    for (family_chirho, argument_chirho, result_chirho) in [
        ("CharToNat", "Char", "Nat"),
        ("GHC.TypeLits.NatToChar", "Nat", "Char"),
    ] {
        assert_eq!(
            env_chirho.lookup_chirho(family_chirho),
            Some(&KindChirho::arrow_chirho(
                runtime_chirho::builtin_term_chirho(argument_chirho).unwrap(),
                runtime_chirho::builtin_term_chirho(result_chirho).unwrap()
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
                        if tail_arg_chirho == result_chirho
                            && matches!(result_chirho.as_ref(), KindChirho::AppChirho(head_chirho, _)
                                if matches!(head_chirho.as_ref(), KindChirho::ConChirho(name_chirho) if name_chirho == "[]"))
                )
        ),
        "promoted list cons preserves the list kind in its tail and result"
    );
}

// -- Module-level kind inference tests --

#[test]
fn data_return_kind_constraint_is_rejected_chirho() {
    // GHC-55233: `data Foo :: Constraint` — unconditionally rejected, no
    // extension licenses it.
    let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("Foo"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![],
        deriving_chirho: vec![],
        kind_sig_chirho: Some(DeclKindSigChirho::ResultChirho(TypeChirho::ConChirho(
            mk_name_chirho("Constraint"),
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    assert!(
        infer_module_kinds_chirho(&module_chirho)
            .diagnostics_chirho
            .has_errors_chirho(),
        "a data declaration returning Constraint must be rejected"
    );
}

#[test]
fn data_binder_kind_constraint_is_accepted_chirho() {
    // `data Foo (_ :: Constraint)` — GHC ACCEPTS this: the Constraint is the
    // BINDER's kind. Before the data-head binder fix this lowered identically to
    // the case above, which is why the check could not be written.
    let mut binder_chirho: TyVarChirho = mk_name_chirho("_").into();
    binder_chirho.kind_annotation_chirho = Some(AstKindChirho::ConstraintChirho);
    let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("Foo"),
        type_vars_chirho: vec![binder_chirho],
        constructors_chirho: vec![],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    assert!(
        !infer_module_kinds_chirho(&module_chirho)
            .diagnostics_chirho
            .has_errors_chirho(),
        "a BINDER of kind Constraint is legal and must stay accepted"
    );
}

#[test]
fn data_return_kind_constraint_stands_down_when_shadowed_chirho() {
    // A module declaring its own `Constraint` shadows the wired-in one.
    let module_chirho = mk_module_chirho(vec![
        DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Constraint"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Foo"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: Some(DeclKindSigChirho::ResultChirho(TypeChirho::ConChirho(
                mk_name_chirho("Constraint"),
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ]);
    assert!(
        !infer_module_kinds_chirho(&module_chirho)
            .diagnostics_chirho
            .has_errors_chirho(),
        "a locally declared Constraint shadows the wired-in kind"
    );
}
