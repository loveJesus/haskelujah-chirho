// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn runtime_universe_chirho(constraint_chirho: bool, representation_chirho: TyChirho) -> TyChirho {
    let head_chirho = if constraint_chirho {
        haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho::RuntimeConstraintChirho
    } else {
        haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho::RuntimeTypeChirho
    };
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho(head_chirho.type_key_chirho().into())),
        Box::new(representation_chirho),
    )
}

#[test]
fn representation_equations_reach_both_existing_unifier_policies_chirho() {
    for constraint_chirho in [false, true] {
        let universe_chirho = TyChirho::ConChirho(
            if constraint_chirho {
                "%universe:constraint"
            } else {
                "%universe:type"
            }
            .into(),
        );
        let variable_chirho = TyVarChirho(74);
        let application_chirho =
            runtime_universe_chirho(constraint_chirho, TyChirho::VarChirho(variable_chirho));
        for substitution_chirho in [
            unify_chirho(
                &universe_chirho,
                &application_chirho,
                SpanChirho::DUMMY_CHIRHO,
            )
            .unwrap(),
            unify_kind_terms_chirho(
                &application_chirho,
                &universe_chirho,
                SpanChirho::DUMMY_CHIRHO,
            )
            .unwrap(),
        ] {
            assert_eq!(
                substitution_chirho.lookup_chirho(&variable_chirho),
                Some(crate::ty_chirho::representation_chirho::boxed_representation_chirho(true))
            );
        }
        let rigid_chirho = runtime_universe_chirho(
            constraint_chirho,
            TyChirho::ConChirho("$rigidRepChirho".into()),
        );
        assert!(
            unify_kind_terms_chirho(&universe_chirho, &rigid_chirho, SpanChirho::DUMMY_CHIRHO)
                .is_err()
        );
    }
}

#[test]
fn representation_equation_never_identifies_opaque_or_unlifted_heads_chirho() {
    let universe_chirho = TyChirho::ConChirho("%universe:type".into());
    let lifted_chirho =
        crate::ty_chirho::representation_chirho::boxed_representation_chirho(true).clone();
    for term_chirho in [
        runtime_universe_chirho(
            false,
            crate::ty_chirho::representation_chirho::boxed_representation_chirho(false).clone(),
        ),
        runtime_universe_chirho(true, lifted_chirho.clone()),
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("TYPE".into())),
            Box::new(lifted_chirho),
        ),
    ] {
        assert!(unify_chirho(&universe_chirho, &term_chirho, SpanChirho::DUMMY_CHIRHO).is_err());
        assert!(
            unify_kind_terms_chirho(&universe_chirho, &term_chirho, SpanChirho::DUMMY_CHIRHO)
                .is_err()
        );
    }
}

#[test]
fn unify_identical_con_chirho() {
    let s_chirho = unify_chirho(
        &TyChirho::int_chirho(),
        &TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    )
    .unwrap();
    assert!(s_chirho.is_empty_chirho());
}

#[test]
fn unify_var_with_con_chirho() {
    let v_chirho = TyVarChirho(0);
    let s_chirho = unify_chirho(
        &TyChirho::VarChirho(v_chirho),
        &TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    )
    .unwrap();
    assert_eq!(
        s_chirho.apply_ty_chirho(&TyChirho::VarChirho(v_chirho)),
        TyChirho::int_chirho()
    );
}

#[test]
fn unify_two_vars_chirho() {
    let a_chirho = TyVarChirho(0);
    let b_chirho = TyVarChirho(1);
    let s_chirho = unify_chirho(
        &TyChirho::VarChirho(a_chirho),
        &TyChirho::VarChirho(b_chirho),
        SpanChirho::DUMMY_CHIRHO,
    )
    .unwrap();
    // After applying, both should map to the same type
    let ta_chirho = s_chirho.apply_ty_chirho(&TyChirho::VarChirho(a_chirho));
    let tb_chirho = s_chirho.apply_ty_chirho(&TyChirho::VarChirho(b_chirho));
    assert_eq!(ta_chirho, tb_chirho);
}

#[test]
fn unify_fun_types_chirho() {
    let a_chirho = TyVarChirho(0);
    let ty1_chirho = TyChirho::fun_chirho(TyChirho::VarChirho(a_chirho), TyChirho::int_chirho());
    let ty2_chirho = TyChirho::fun_chirho(TyChirho::bool_chirho(), TyChirho::int_chirho());

    let s_chirho = unify_chirho(&ty1_chirho, &ty2_chirho, SpanChirho::DUMMY_CHIRHO).unwrap();
    assert_eq!(
        s_chirho.apply_ty_chirho(&TyChirho::VarChirho(a_chirho)),
        TyChirho::bool_chirho()
    );
}

#[test]
fn unify_mismatch_chirho() {
    let result_chirho = unify_chirho(
        &TyChirho::int_chirho(),
        &TyChirho::bool_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(matches!(
        result_chirho,
        Err(UnifyErrorChirho::MismatchChirho { .. })
    ));
}

#[test]
fn unify_same_qualified_suffix_tycons_chirho() {
    let left_ty_chirho = TyChirho::ConChirho("P.Result".to_string());
    let right_ty_chirho = TyChirho::ConChirho("Test.QuickCheck.Property.Result".to_string());
    let subst_chirho = unify_chirho(&left_ty_chirho, &right_ty_chirho, SpanChirho::DUMMY_CHIRHO)
        .expect("qualified aliases of the same imported type should unify");
    assert!(subst_chirho.is_empty_chirho());
}

#[test]
fn unify_bare_and_qualified_tycons_do_not_alias_chirho() {
    let left_ty_chirho = TyChirho::ConChirho("Result".to_string());
    let right_ty_chirho = TyChirho::ConChirho("Test.QuickCheck.Property.Result".to_string());
    let result_chirho = unify_chirho(&left_ty_chirho, &right_ty_chirho, SpanChirho::DUMMY_CHIRHO);
    assert!(
        matches!(result_chirho, Err(UnifyErrorChirho::MismatchChirho { .. })),
        "local bare tycons must stay distinct from imported qualified tycons"
    );
}

fn source_constructor_ty_chirho(defining_module_chirho: &str) -> TyChirho {
    use haskelujah_ast_chirho::name_chirho::{
        TypeConstructorDeclarationChirho, TypeConstructorRoleChirho,
    };

    let declaration_chirho = TypeConstructorDeclarationChirho::source_chirho(
        defining_module_chirho,
        "TokenChirho",
        TypeConstructorRoleChirho::DataChirho,
    );
    TyChirho::ConChirho(declaration_chirho.type_key_chirho())
}

#[test]
fn source_constructor_keys_with_dotted_definers_remain_nominal_chirho() {
    let left_chirho = source_constructor_ty_chirho("AlphaChirho.SharedChirho");
    let right_chirho = source_constructor_ty_chirho("OtherChirho.SharedChirho");
    assert_ne!(left_chirho, right_chirho);

    for (expected_chirho, actual_chirho) in [
        (left_chirho.clone(), right_chirho.clone()),
        (right_chirho, left_chirho),
    ] {
        for (expected_chirho, actual_chirho) in [
            (expected_chirho.clone(), actual_chirho.clone()),
            (
                TyChirho::ListChirho(Box::new(expected_chirho.clone())),
                TyChirho::ListChirho(Box::new(actual_chirho.clone())),
            ),
            (
                TyChirho::fun_chirho(TyChirho::int_chirho(), expected_chirho),
                TyChirho::fun_chirho(TyChirho::int_chirho(), actual_chirho),
            ),
        ] {
            assert!(
                matches!(
                    unify_chirho(&expected_chirho, &actual_chirho, SpanChirho::DUMMY_CHIRHO),
                    Err(UnifyErrorChirho::MismatchChirho { .. })
                ),
                "distinct source producers must remain nominal: {expected_chirho:?}, {actual_chirho:?}"
            );
        }
    }
}

#[test]
fn source_constructor_keys_keep_the_same_producer_and_meta_substitution_chirho() {
    let source_chirho = source_constructor_ty_chirho("AlphaChirho.SharedChirho");
    let same_chirho = source_constructor_ty_chirho("AlphaChirho.SharedChirho");
    let substitution_chirho = unify_chirho(&source_chirho, &same_chirho, SpanChirho::DUMMY_CHIRHO)
        .expect("the same source producer retains its identity");
    assert!(substitution_chirho.is_empty_chirho());

    let variable_chirho = TyChirho::VarChirho(TyVarChirho(5_000));
    for (left_chirho, right_chirho) in [
        (&variable_chirho, &source_chirho),
        (&source_chirho, &variable_chirho),
    ] {
        let substitution_chirho = unify_chirho(left_chirho, right_chirho, SpanChirho::DUMMY_CHIRHO)
            .expect("a metavariable can stand for the source constructor");
        assert_eq!(
            substitution_chirho.apply_ty_chirho(&variable_chirho),
            source_chirho
        );
    }
}

#[test]
fn occurs_check_prevents_infinite_type_chirho() {
    let a_chirho = TyVarChirho(0);
    // t0 ~ [t0] would require t0 = [[[...infinite...]]]
    let result_chirho = unify_chirho(
        &TyChirho::VarChirho(a_chirho),
        &TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(matches!(
        result_chirho,
        Err(UnifyErrorChirho::OccursCheckChirho { .. })
    ));
}

#[test]
fn unify_tuples_same_arity_chirho() {
    let a_chirho = TyVarChirho(0);
    let t1_chirho =
        TyChirho::TupleChirho(vec![TyChirho::VarChirho(a_chirho), TyChirho::int_chirho()]);
    let t2_chirho = TyChirho::TupleChirho(vec![TyChirho::bool_chirho(), TyChirho::int_chirho()]);

    let s_chirho = unify_chirho(&t1_chirho, &t2_chirho, SpanChirho::DUMMY_CHIRHO).unwrap();
    assert_eq!(
        s_chirho.apply_ty_chirho(&TyChirho::VarChirho(a_chirho)),
        TyChirho::bool_chirho()
    );
}

#[test]
fn unify_tuples_different_arity_fails_chirho() {
    let t1_chirho = TyChirho::TupleChirho(vec![TyChirho::int_chirho()]);
    let t2_chirho = TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::bool_chirho()]);

    let result_chirho = unify_chirho(&t1_chirho, &t2_chirho, SpanChirho::DUMMY_CHIRHO);
    assert!(matches!(
        result_chirho,
        Err(UnifyErrorChirho::TupleArityChirho { .. })
    ));
}

#[test]
fn unify_list_types_chirho() {
    let a_chirho = TyVarChirho(0);
    let s_chirho = unify_chirho(
        &TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
        &TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
        SpanChirho::DUMMY_CHIRHO,
    )
    .unwrap();
    assert_eq!(
        s_chirho.apply_ty_chirho(&TyChirho::VarChirho(a_chirho)),
        TyChirho::int_chirho()
    );
}

#[test]
fn unify_forall_same_structure_chirho() {
    // forall t0. t0 -> t0 ~ forall t1. t1 -> t1 (alpha-equivalent)
    let fa_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(0)].into(),
        body_chirho: Box::new(TyChirho::fun_chirho(
            TyChirho::VarChirho(TyVarChirho(0)),
            TyChirho::VarChirho(TyVarChirho(0)),
        )),
    };
    let fb_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(1)].into(),
        body_chirho: Box::new(TyChirho::fun_chirho(
            TyChirho::VarChirho(TyVarChirho(1)),
            TyChirho::VarChirho(TyVarChirho(1)),
        )),
    };
    let s_chirho = unify_chirho(&fa_chirho, &fb_chirho, SpanChirho::DUMMY_CHIRHO).unwrap();
    assert!(s_chirho.is_empty_chirho());
}

#[test]
fn unify_forall_with_concrete_fun_chirho() {
    // forall t0. t0 -> t0 ~ Int -> Int (strip forall, unify body)
    let forall_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(0)].into(),
        body_chirho: Box::new(TyChirho::fun_chirho(
            TyChirho::VarChirho(TyVarChirho(0)),
            TyChirho::VarChirho(TyVarChirho(0)),
        )),
    };
    let concrete_chirho = TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho());
    let s_chirho =
        unify_chirho(&forall_chirho, &concrete_chirho, SpanChirho::DUMMY_CHIRHO).unwrap();
    assert_eq!(
        s_chirho.apply_ty_chirho(&TyChirho::VarChirho(TyVarChirho(0))),
        TyChirho::int_chirho()
    );
}

#[test]
fn ordinary_meta_cannot_receive_a_polytype_chirho() {
    // Ordinary metas remain monotypes; the application prepass owns the
    // separate capability for a permitted impredicative instantiation.
    let forall_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(0)].into(),
        body_chirho: Box::new(TyChirho::fun_chirho(
            TyChirho::VarChirho(TyVarChirho(0)),
            TyChirho::VarChirho(TyVarChirho(0)),
        )),
    };
    let var_chirho = TyChirho::VarChirho(TyVarChirho(5));
    for (left_chirho, right_chirho) in
        [(&var_chirho, &forall_chirho), (&forall_chirho, &var_chirho)]
    {
        assert!(matches!(
            unify_chirho(left_chirho, right_chirho, SpanChirho::DUMMY_CHIRHO),
            Err(UnifyErrorChirho::MismatchChirho { .. })
        ));
    }
    assert!(
        unify_chirho(
            &var_chirho,
            &TyChirho::ListChirho(Box::new(forall_chirho)),
            SpanChirho::DUMMY_CHIRHO
        )
        .is_err()
    );
}

#[test]
fn subsume_forall_with_forall_chirho() {
    // forall t0. t0 -> t0 subsumes forall t1. t1 -> t1
    let fa_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(0)].into(),
        body_chirho: Box::new(TyChirho::fun_chirho(
            TyChirho::VarChirho(TyVarChirho(0)),
            TyChirho::VarChirho(TyVarChirho(0)),
        )),
    };
    let fb_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(1)].into(),
        body_chirho: Box::new(TyChirho::fun_chirho(
            TyChirho::VarChirho(TyVarChirho(1)),
            TyChirho::VarChirho(TyVarChirho(1)),
        )),
    };
    let mut next_var_chirho = 10u32;
    let result_chirho = subsume_chirho(
        &fa_chirho,
        &fb_chirho,
        &mut next_var_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(result_chirho.is_ok());
}

#[test]
fn checked_kind_prefix_cannot_specialize_an_earlier_universal_chirho() {
    use crate::ty_chirho::{QuantifiedVariableChirho, QuantifiedVariablesChirho};
    let group_chirho = |base_chirho: u32, dependent_chirho: bool| {
        QuantifiedVariablesChirho::from_records_chirho(vec![
            QuantifiedVariableChirho {
                variable_chirho: TyVarChirho(base_chirho),
                kind_chirho: Some(std::sync::Arc::new(TyChirho::ConChirho("Type".into()))),
                source_chirho: None,
                specificity_chirho: None,
            },
            QuantifiedVariableChirho {
                variable_chirho: TyVarChirho(base_chirho + 1),
                kind_chirho: Some(std::sync::Arc::new(if dependent_chirho {
                    TyChirho::VarChirho(TyVarChirho(base_chirho))
                } else {
                    TyChirho::ConChirho("Type".into())
                })),
                source_chirho: None,
                specificity_chirho: None,
            },
        ])
    };
    assert!(
        align_forall_prefix_chirho(
            &group_chirho(0, true),
            &group_chirho(10, true),
            SpanChirho::DUMMY_CHIRHO
        )
        .is_ok()
    );
    assert!(
        align_forall_prefix_chirho(
            &group_chirho(0, true),
            &group_chirho(10, false),
            SpanChirho::DUMMY_CHIRHO
        )
        .is_err()
    );
    assert!(
        unify_kind_terms_chirho(
            &TyChirho::int_chirho(),
            &TyChirho::ConChirho("Double".into()),
            SpanChirho::DUMMY_CHIRHO
        )
        .is_err()
    );
}

#[test]
fn numeric_nominality_rejects_int_double_in_both_directions_chirho() {
    for (left_chirho, right_chirho) in [
        (TyChirho::int_chirho(), TyChirho::double_chirho()),
        (TyChirho::double_chirho(), TyChirho::int_chirho()),
    ] {
        assert!(matches!(
            unify_chirho(&left_chirho, &right_chirho, SpanChirho::DUMMY_CHIRHO),
            Err(UnifyErrorChirho::MismatchChirho { .. })
        ));
    }
}

#[test]
fn numeric_nominality_rejects_nested_numeric_mismatches_chirho() {
    let int_chirho = TyChirho::int_chirho();
    let double_chirho = TyChirho::double_chirho();
    let cases_chirho = [
        (
            TyChirho::fun_chirho(int_chirho.clone(), TyChirho::bool_chirho()),
            TyChirho::fun_chirho(double_chirho.clone(), TyChirho::bool_chirho()),
        ),
        (
            TyChirho::fun_chirho(TyChirho::bool_chirho(), int_chirho.clone()),
            TyChirho::fun_chirho(TyChirho::bool_chirho(), double_chirho.clone()),
        ),
        (
            TyChirho::ListChirho(Box::new(int_chirho.clone())),
            TyChirho::ListChirho(Box::new(double_chirho.clone())),
        ),
        (
            TyChirho::TupleChirho(vec![TyChirho::bool_chirho(), int_chirho.clone()]),
            TyChirho::TupleChirho(vec![TyChirho::bool_chirho(), double_chirho.clone()]),
        ),
        (
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(int_chirho),
            ),
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(double_chirho),
            ),
        ),
    ];
    for (left_chirho, right_chirho) in cases_chirho {
        for (actual_chirho, expected_chirho) in
            [(&left_chirho, &right_chirho), (&right_chirho, &left_chirho)]
        {
            assert!(matches!(
                unify_chirho(actual_chirho, expected_chirho, SpanChirho::DUMMY_CHIRHO),
                Err(UnifyErrorChirho::MismatchChirho { .. })
            ));
        }
    }
}

#[test]
fn numeric_nominality_preserves_same_types_and_meta_substitution_chirho() {
    for ty_chirho in [TyChirho::int_chirho(), TyChirho::double_chirho()] {
        assert!(
            unify_chirho(&ty_chirho, &ty_chirho, SpanChirho::DUMMY_CHIRHO)
                .unwrap()
                .is_empty_chirho()
        );
        let variable_chirho = TyChirho::VarChirho(TyVarChirho(27));
        for (left_chirho, right_chirho) in [
            (&variable_chirho, &ty_chirho),
            (&ty_chirho, &variable_chirho),
        ] {
            let subst_chirho =
                unify_chirho(left_chirho, right_chirho, SpanChirho::DUMMY_CHIRHO).unwrap();
            assert_eq!(subst_chirho.apply_ty_chirho(&variable_chirho), ty_chirho);
        }
    }
}

#[test]
fn reflexive_monotypes_spend_work_without_materializing_type_output_chirho() {
    let terms_chirho = vec![
        TyChirho::ConChirho("%universe:type".into()),
        TyChirho::VarChirho(TyVarChirho(7001)),
        TyChirho::ForallVarChirho("rigidChirho".into()),
        TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
        TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::bool_chirho()]),
        TyChirho::TupleChirho(Vec::new()),
        TyChirho::FunChirho(
            Box::new(TyChirho::int_chirho()),
            Box::new(TyChirho::bool_chirho()),
            crate::ty_chirho::MultChirho::OneChirho,
        ),
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".into())),
            Box::new(TyChirho::VarChirho(TyVarChirho(7001))),
        ),
    ];
    for left_chirho in terms_chirho {
        let right_chirho = left_chirho.clone();
        let mut work_chirho = 4096usize;
        let mut output_chirho = 0usize;
        let result_chirho = unify_monomorphic_bounded_chirho(
            &left_chirho,
            &right_chirho,
            &|_| false,
            &mut |spent_work_chirho, spent_output_chirho| {
                work_chirho = work_chirho.checked_sub(spent_work_chirho)?;
                output_chirho = output_chirho.checked_sub(spent_output_chirho)?;
                Some(())
            },
        )
        .unwrap()
        .unwrap();
        assert!(result_chirho.iter_chirho().next().is_none());
        assert!(work_chirho < 4096);
        assert_eq!(output_chirho, 0);
    }
    assert!(
        unify_monomorphic_bounded_chirho(
            &TyChirho::int_chirho(),
            &TyChirho::int_chirho(),
            &|_| false,
            &mut |_, _| None,
        )
        .is_err()
    );
    let mut deep_chirho = TyChirho::int_chirho();
    for _depth_chirho in 0..130 {
        deep_chirho = TyChirho::ListChirho(Box::new(deep_chirho));
    }
    assert!(
        unify_monomorphic_bounded_chirho(&deep_chirho, &deep_chirho, &|_| false, &mut |_, _| Some(
            ()
        ),)
        .is_err()
    );
}

#[test]
fn reflexive_polytypes_and_singleton_tuples_never_become_monotypes_chirho() {
    let polytype_chirho = TyChirho::ForallChirho {
        vars_chirho: crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(Vec::new()),
        body_chirho: Box::new(TyChirho::int_chirho()),
    };
    let required_chirho = TyChirho::RequiredForallChirho {
        vars_chirho: crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(Vec::new()),
        body_chirho: Box::new(TyChirho::int_chirho()),
    };
    let qualified_chirho = TyChirho::QualChirho {
        context_chirho: Vec::new(),
        body_chirho: Box::new(TyChirho::int_chirho()),
    };
    for term_chirho in [
        polytype_chirho.clone(),
        required_chirho,
        qualified_chirho,
        TyChirho::TupleChirho(vec![TyChirho::int_chirho()]),
        TyChirho::ListChirho(Box::new(polytype_chirho)),
    ] {
        assert!(
            unify_monomorphic_bounded_chirho(&term_chirho, &term_chirho, &|_| true, &mut |_, _| {
                Some(())
            },)
            .unwrap()
            .is_none()
        );
    }
}

#[test]
fn non_reflexive_monotypes_keep_nominal_multiplicity_and_binding_rules_chirho() {
    let variable_chirho = TyVarChirho(7002);
    assert!(
        unify_monomorphic_bounded_chirho(
            &TyChirho::VarChirho(variable_chirho),
            &TyChirho::int_chirho(),
            &|_| false,
            &mut |_, _| Some(()),
        )
        .unwrap()
        .is_none()
    );
    let solved_chirho = unify_monomorphic_bounded_chirho(
        &TyChirho::VarChirho(variable_chirho),
        &TyChirho::int_chirho(),
        &|_| true,
        &mut |_, _| Some(()),
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        solved_chirho.lookup_chirho(&variable_chirho),
        Some(&TyChirho::int_chirho())
    );
    assert!(
        unify_monomorphic_bounded_chirho(
            &TyChirho::ConChirho("%constructor:source:4:Left:9:SameChirho".into()),
            &TyChirho::ConChirho("%constructor:source:5:Right:9:SameChirho".into()),
            &|_| true,
            &mut |_, _| Some(()),
        )
        .unwrap()
        .is_none()
    );
    let list_chirho = TyChirho::ListChirho(Box::new(TyChirho::int_chirho()));
    let application_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("[]".into())),
        Box::new(TyChirho::int_chirho()),
    );
    assert!(
        unify_monomorphic_bounded_chirho(
            &list_chirho,
            &application_chirho,
            &|_| false,
            &mut |_, _| Some(()),
        )
        .unwrap()
        .unwrap()
        .iter_chirho()
        .next()
        .is_none()
    );
    let linear_chirho = TyChirho::FunChirho(
        Box::new(TyChirho::int_chirho()),
        Box::new(TyChirho::bool_chirho()),
        crate::ty_chirho::MultChirho::OneChirho,
    );
    assert!(
        unify_monomorphic_bounded_chirho(
            &linear_chirho,
            &TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::bool_chirho()),
            &|_| true,
            &mut |_, _| Some(()),
        )
        .unwrap()
        .is_none()
    );
}
