// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type families: registration, reduction and the built-in type-level literal families.
//! Moved from `infer_chirho.rs` in row 661.

use super::test_support_chirho::{
    bind_family_fixture_alias_chirho, dummy_name_chirho, family_fixture_context_chirho,
    type_family_app1_chirho, type_family_app2_chirho, type_lit_chirho,
};
use super::*;
use crate::ty_chirho::MultChirho;
use haskelujah_ast_chirho::decl_chirho::{AssocTfInstanceChirho, DeclChirho};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

/// Test: AST type to internal type conversion roundtrip.
#[test]
fn ast_type_conversion_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    let mut var_map_chirho = HashMap::new();

    // Convert `a -> Int`
    let ast_ty_chirho = TypeChirho::FunChirho {
        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
        mult_chirho: None,
        result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let ty_chirho = ctx_chirho.ast_type_to_ty_chirho(&ast_ty_chirho, &mut var_map_chirho);

    // Should be FunChirho(VarChirho(_), ConChirho("Int"))
    if let TyChirho::FunChirho(ref arg_chirho, ref ret_chirho, _) = ty_chirho {
        assert!(matches!(**arg_chirho, TyChirho::VarChirho(_)));
        assert_eq!(**ret_chirho, TyChirho::int_chirho());
    } else {
        panic!("expected function type, got: {ty_chirho}");
    }

    // Same `a` should reuse the same variable
    let a2_chirho = ctx_chirho.ast_type_to_ty_chirho(
        &TypeChirho::VarChirho(dummy_name_chirho("a")),
        &mut var_map_chirho,
    );
    if let (TyChirho::FunChirho(arg_chirho, _, _), TyChirho::VarChirho(v2_chirho)) =
        (&ty_chirho, &a2_chirho)
        && let TyChirho::VarChirho(v1_chirho) = &**arg_chirho
    {
        assert_eq!(*v1_chirho, *v2_chirho, "same `a` should yield same var");
    }
}

#[test]
fn ast_type_conversion_preserves_qualified_tycon_spine_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    let mut var_map_chirho = HashMap::new();
    let qualified_state_t_chirho = NameChirho::RawChirho(RawNameChirho::qualified_chirho(
        "LazyS",
        "StateT",
        SpanChirho::DUMMY_CHIRHO,
    ));
    let ast_ty_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::ConChirho(qualified_state_t_chirho)),
                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("s"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("m"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let ty_chirho = ctx_chirho.ast_type_to_ty_chirho(&ast_ty_chirho, &mut var_map_chirho);
    assert_eq!(
        ty_chirho,
        TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("LazyS.StateT".to_string())),
                    Box::new(TyChirho::VarChirho(
                        *var_map_chirho.get("s").expect("s var missing"),
                    )),
                )),
                Box::new(TyChirho::VarChirho(
                    *var_map_chirho.get("m").expect("m var missing"),
                )),
            )),
            Box::new(TyChirho::VarChirho(
                *var_map_chirho.get("a").expect("a var missing"),
            )),
        ),
        "qualified type constructors in local signatures must preserve their full application spine",
    );
}

#[test]
fn type_family_registration_and_reduction_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    // Register a closed type family: F Int = Bool, F Char = Int
    ctx_chirho.register_type_family_chirho(
        "F".to_string(),
        1,
        vec![
            (vec![TyChirho::int_chirho()], TyChirho::bool_chirho()),
            (vec![TyChirho::char_chirho()], TyChirho::int_chirho()),
        ],
    );
    // Reduce F Int → Bool
    let result_chirho = ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::int_chirho()]);
    assert_eq!(
        result_chirho,
        Some(TyChirho::bool_chirho()),
        "F Int should reduce to Bool"
    );
    // Reduce F Char → Int
    let result2_chirho = ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::char_chirho()]);
    assert_eq!(
        result2_chirho,
        Some(TyChirho::int_chirho()),
        "F Char should reduce to Int"
    );
    // F String should not reduce (stuck)
    let result3_chirho = ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::string_chirho()]);
    assert_eq!(
        result3_chirho, None,
        "F String should be stuck (no matching equation)"
    );
}

#[test]
fn type_family_instance_registration_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    // Register open type family F
    ctx_chirho.register_type_family_chirho("F".to_string(), 1, vec![]);
    // Add instance: F Int = Bool
    ctx_chirho.register_type_family_instance_chirho(
        "F".to_string(),
        vec![TyChirho::int_chirho()],
        TyChirho::bool_chirho(),
    );
    let result_chirho = ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::int_chirho()]);
    assert_eq!(
        result_chirho,
        Some(TyChirho::bool_chirho()),
        "open F Int should reduce to Bool"
    );
}

#[test]
fn class_assoc_type_family_default_applies_per_instance_chirho() {
    let class_decl_ast_chirho = DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: dummy_name_chirho("Representable"),
        type_vars_chirho: vec![dummy_name_chirho("f").into()],
        methods_chirho: vec![],
        associated_tfs_chirho: vec![haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
            name_chirho: dummy_name_chirho("Rep"),
            type_vars_chirho: vec![dummy_name_chirho("f")],
            data_family_chirho: false,
            default_rhs_chirho: Some(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("GRep"))),
                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            default_params_chirho: vec![],
            default_binders_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    // `instance Representable Proxy` leaves `Rep` undefined, so the
    // class default applies to it — and only to it.
    let instance_decl_ast_chirho = DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: dummy_name_chirho("Representable"),
        types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Proxy"))],
        methods_chirho: vec![],
        assoc_tf_instances_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.process_class_decl_chirho(&class_decl_ast_chirho);
    ctx_chirho.process_instance_decl_chirho(&instance_decl_ast_chirho);

    let reduced_chirho =
        ctx_chirho.reduce_type_family_chirho("Rep", &[TyChirho::ConChirho("Proxy".to_string())]);
    assert_eq!(
        reduced_chirho,
        Some(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("GRep".to_string())),
            Box::new(TyChirho::ConChirho("Proxy".to_string())),
        )),
        "an instance without its own equation takes the class default"
    );

    // No instance for `Maybe`: the default is not a general equation, so
    // `Rep Maybe` stays stuck (GHC agrees).
    let stuck_chirho =
        ctx_chirho.reduce_type_family_chirho("Rep", &[TyChirho::ConChirho("Maybe".to_string())]);
    assert_eq!(
        stuck_chirho, None,
        "a class default must not reduce for a type without an instance"
    );
}

#[test]
fn instance_assoc_type_family_override_beats_class_default_chirho() {
    let class_decl_ast_chirho = DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: dummy_name_chirho("Representable"),
        type_vars_chirho: vec![dummy_name_chirho("f").into()],
        methods_chirho: vec![],
        associated_tfs_chirho: vec![haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
            name_chirho: dummy_name_chirho("Rep"),
            type_vars_chirho: vec![dummy_name_chirho("f")],
            data_family_chirho: false,
            default_rhs_chirho: Some(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("GRep"))),
                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            default_params_chirho: vec![],
            default_binders_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let instance_decl_ast_chirho = DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: dummy_name_chirho("Representable"),
        types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Proxy"))],
        methods_chirho: vec![],
        assoc_tf_instances_chirho: vec![AssocTfInstanceChirho {
            family_name_chirho: dummy_name_chirho("Rep"),
            lhs_types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Proxy"))],
            rhs_chirho: TypeChirho::TupleChirho {
                elements_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.process_class_decl_chirho(&class_decl_ast_chirho);
    ctx_chirho.process_instance_decl_chirho(&instance_decl_ast_chirho);

    let reduced_chirho =
        ctx_chirho.reduce_type_family_chirho("Rep", &[TyChirho::ConChirho("Proxy".to_string())]);

    assert_eq!(
        reduced_chirho,
        Some(TyChirho::TupleChirho(vec![])),
        "instance associated family equation should override the class default"
    );
}

#[test]
fn type_family_oversaturated_application_reduces_and_reapplies_args_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.register_type_family_chirho("F".to_owned(), 1, vec![]);
    ctx_chirho.register_type_family_instance_chirho(
        "F".to_string(),
        vec![TyChirho::int_chirho()],
        TyChirho::ConChirho("Maybe".to_string()),
    );
    let app_ty_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("F".to_string())),
            Box::new(TyChirho::int_chirho()),
        )),
        Box::new(TyChirho::char_chirho()),
    );
    let reduced_chirho = ctx_chirho.reduce_type_families_in_ty_chirho(&app_ty_chirho);
    assert_eq!(
        reduced_chirho,
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(TyChirho::char_chirho()),
        ),
        "oversaturated family application should reduce the family head and reapply leftover args"
    );
}

#[test]
fn typenats_builtin_arithmetic_and_comparison_families_reduce_chirho() {
    let ctx_chirho = family_fixture_context_chirho();
    let x_chirho = TyChirho::VarChirho(TyVarChirho(7002));

    for (family_chirho, left_chirho, right_chirho, expected_chirho) in [
        (
            "+",
            type_lit_chirho("2"),
            type_lit_chirho("3"),
            type_lit_chirho("5"),
        ),
        (
            "+",
            type_lit_chirho("0"),
            x_chirho.clone(),
            x_chirho.clone(),
        ),
        (
            "*",
            type_lit_chirho("2"),
            type_lit_chirho("3"),
            type_lit_chirho("6"),
        ),
        (
            "*",
            x_chirho.clone(),
            type_lit_chirho("1"),
            x_chirho.clone(),
        ),
        (
            "^",
            type_lit_chirho("2"),
            type_lit_chirho("3"),
            type_lit_chirho("8"),
        ),
        (
            "^",
            x_chirho.clone(),
            type_lit_chirho("0"),
            type_lit_chirho("1"),
        ),
        (
            "-",
            type_lit_chirho("3"),
            type_lit_chirho("2"),
            type_lit_chirho("1"),
        ),
        (
            "-",
            x_chirho.clone(),
            type_lit_chirho("0"),
            x_chirho.clone(),
        ),
        (
            "<=?",
            type_lit_chirho("1"),
            type_lit_chirho("2"),
            TyChirho::ConChirho("True".to_string()),
        ),
        (
            "<=?",
            type_lit_chirho("2"),
            type_lit_chirho("1"),
            TyChirho::ConChirho("False".to_string()),
        ),
        (
            "<=?",
            x_chirho.clone(),
            x_chirho.clone(),
            TyChirho::ConChirho("True".to_string()),
        ),
        (
            "GHC.TypeNats.Div",
            type_lit_chirho("10"),
            type_lit_chirho("3"),
            type_lit_chirho("3"),
        ),
        (
            "Mod",
            type_lit_chirho("10"),
            type_lit_chirho("3"),
            type_lit_chirho("1"),
        ),
    ] {
        assert_eq!(
            ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app2_chirho(
                family_chirho,
                left_chirho,
                right_chirho,
            )),
            expected_chirho,
            "{family_chirho} should reduce through the TypeNats builtin reducer"
        );
    }

    assert_eq!(
        ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app1_chirho(
            "Log2",
            type_lit_chirho("10"),
        )),
        type_lit_chirho("3"),
        "Log2 10 should reduce to 3"
    );
}

#[test]
fn typelits_character_conversion_families_reduce_chirho() {
    let ctx_chirho = family_fixture_context_chirho();

    for (character_chirho, codepoint_chirho, canonical_character_chirho) in [
        ("'a'", "97", "'a'"),
        ("'\\1'", "1", "'\\x1'"),
        ("'λ'", "955", "'λ'"),
        ("'\\NUL'", "0", "'\\0'"),
    ] {
        assert_eq!(
            ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app1_chirho(
                "GHC.TypeLits.CharToNat",
                type_lit_chirho(character_chirho),
            )),
            type_lit_chirho(codepoint_chirho),
            "CharToNat should reduce concrete character literals"
        );
        assert_eq!(
            ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app1_chirho(
                "NatToChar",
                type_lit_chirho(codepoint_chirho),
            )),
            type_lit_chirho(canonical_character_chirho),
            "NatToChar should reduce valid Unicode scalar values"
        );
    }

    let variable_chirho = TyChirho::VarChirho(TyVarChirho(7003));
    let stuck_variable_chirho = type_family_app1_chirho("CharToNat", variable_chirho.clone());
    assert_eq!(
        ctx_chirho.reduce_type_families_in_ty_chirho(&stuck_variable_chirho),
        stuck_variable_chirho,
        "variable character applications should stay stuck"
    );

    let surrogate_chirho = type_family_app1_chirho("NatToChar", type_lit_chirho("55296"));
    assert_eq!(
        ctx_chirho.reduce_type_families_in_ty_chirho(&surrogate_chirho),
        surrogate_chirho,
        "surrogate codepoints should stay stuck"
    );
}

#[test]
fn source_family_and_intrinsic_have_separate_selected_identities_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.register_type_family_chirho(
        "AppendSymbol".to_owned(),
        2,
        vec![(
            vec![type_lit_chirho("\"foo\""), type_lit_chirho("\"bar\"")],
            type_lit_chirho("\"override\""),
        )],
    );
    assert_eq!(
        ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app2_chirho(
            "AppendSymbol",
            type_lit_chirho("\"foo\""),
            type_lit_chirho("\"bar\"")
        )),
        type_lit_chirho("\"override\"")
    );
    let intrinsic_chirho =
        TypeFamilyIdChirho::IntrinsicChirho(IntrinsicTypeFamilyChirho::AppendSymbolChirho)
            .key_chirho();
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho(
            &intrinsic_chirho,
            &[type_lit_chirho("\"foo\""), type_lit_chirho("\"bar\"")]
        ),
        Some(type_lit_chirho("\"foobar\""))
    );
}

#[test]
fn conflicting_open_product_rows_are_unverified_and_stuck_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.register_type_family_chirho("Rep".to_owned(), 1, vec![]);
    let pattern_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("Product".to_owned())),
        Box::new(TyChirho::ForallVarChirho("f".to_owned())),
    );
    ctx_chirho.register_type_family_instance_chirho(
        "Rep".to_owned(),
        vec![pattern_chirho.clone()],
        TyChirho::int_chirho(),
    );
    ctx_chirho.register_type_family_instance_chirho(
        "Rep".to_owned(),
        vec![pattern_chirho],
        TyChirho::bool_chirho(),
    );
    assert!(ctx_chirho.diagnostics_chirho.has_errors_chirho());
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho(
            "Rep",
            &[TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Product".to_owned())),
                Box::new(TyChirho::int_chirho())
            )]
        ),
        None
    );
}

#[test]
fn qualified_export_alias_selects_the_same_family_definition_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.register_type_family_chirho(
        "Rep".to_owned(),
        1,
        vec![(vec![TyChirho::int_chirho()], TyChirho::bool_chirho())],
    );
    bind_family_fixture_alias_chirho(&mut ctx_chirho, "Rep", "Forwarder.Rep");
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho("Forwarder.Rep", &[TyChirho::int_chirho()]),
        Some(TyChirho::bool_chirho())
    );
    assert_eq!(
        ctx_chirho.family_name_key_chirho("Rep"),
        ctx_chirho.family_name_key_chirho("Forwarder.Rep")
    );
}

#[test]
fn empty_selected_family_never_borrows_a_namesakes_equations_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.register_type_family_chirho(
        "MutableGen".to_owned(),
        1,
        vec![(vec![TyChirho::int_chirho()], TyChirho::bool_chirho())],
    );
    let id_chirho = TypeFamilyIdChirho::SourceChirho {
        module_chirho: "Other".to_owned(),
        name_chirho: "MutableGen".to_owned(),
    };
    ctx_chirho
        .type_families_chirho
        .insert_chirho(
            TypeFamilyDefinitionChirho::new_chirho(
                id_chirho.clone(),
                1,
                TypeFamilyReductionChirho::ClosedChirho,
                vec![],
            )
            .unwrap(),
        )
        .unwrap();
    ctx_chirho
        .type_families_chirho
        .export_alias_chirho("Other.MutableGen".to_owned(), &id_chirho)
        .unwrap();
    let mut scope_chirho = ctx_chirho.family_scope_chirho.as_deref().cloned().unwrap();
    scope_chirho.import_export_chirho(
        "Other.MutableGen".to_owned(),
        "Other.MutableGen",
        &ctx_chirho.type_families_chirho,
    );
    ctx_chirho.family_scope_chirho = Some(Arc::new(scope_chirho));
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho("MutableGen", &[TyChirho::int_chirho()]),
        Some(TyChirho::bool_chirho())
    );
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho("Other.MutableGen", &[TyChirho::int_chirho()]),
        None
    );
}

#[test]
fn function_unification_retries_after_result_fixes_type_family_args_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.register_type_family_chirho("MutableGen".to_owned(), 2, vec![]);
    ctx_chirho.register_type_family_instance_chirho(
        "MutableGen".to_string(),
        vec![
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STGen".to_string())),
                Box::new(TyChirho::ForallVarChirho("g".to_string())),
            ),
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::ForallVarChirho("s".to_string())),
            ),
        ],
        TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STGenM".to_string())),
                Box::new(TyChirho::ForallVarChirho("g".to_string())),
            )),
            Box::new(TyChirho::ForallVarChirho("s".to_string())),
        ),
    );

    let monad_ty_chirho = ctx_chirho.fresh_var_chirho();
    let expected_ty_chirho = TyChirho::FunChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("MutableGen".to_string())),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("STGen".to_string())),
                    Box::new(TyChirho::ConChirho("StdGen".to_string())),
                )),
            )),
            Box::new(monad_ty_chirho.clone()),
        )),
        Box::new(TyChirho::AppChirho(
            Box::new(monad_ty_chirho),
            Box::new(TyChirho::ConChirho("Int".to_string())),
        )),
        MultChirho::ManyChirho,
    );
    let actual_ty_chirho = TyChirho::FunChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STGenM".to_string())),
                Box::new(TyChirho::ConChirho("StdGen".to_string())),
            )),
            Box::new(TyChirho::ConChirho("S0".to_string())),
        )),
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::ConChirho("S0".to_string())),
            )),
            Box::new(TyChirho::ConChirho("Int".to_string())),
        )),
        MultChirho::ManyChirho,
    );

    let subst_chirho = ctx_chirho
        .unify_normalized_chirho(
            &expected_ty_chirho,
            &actual_ty_chirho,
            SpanChirho::DUMMY_CHIRHO,
        )
        .expect("result-side monad refinement should unlock MutableGen reduction");
    let expected_normalized_chirho =
        ctx_chirho.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(&expected_ty_chirho));
    let actual_normalized_chirho =
        ctx_chirho.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(&actual_ty_chirho));

    assert_eq!(
        expected_normalized_chirho, actual_normalized_chirho,
        "function unification should retry after result-type substitutions make associated family arguments reducible"
    );
}

#[test]
fn type_family_nonexistent_chirho() {
    let ctx_chirho = family_fixture_context_chirho();
    let result_chirho =
        ctx_chirho.reduce_type_family_chirho("NoSuchFamily", &[TyChirho::int_chirho()]);
    assert_eq!(
        result_chirho, None,
        "unregistered family should return None"
    );
}

#[test]
fn normalization_reserves_enough_work_for_a_recorded_fallback_blocker_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.register_type_family_chirho(
        "Frob".to_owned(),
        1,
        vec![
            (vec![TyChirho::int_chirho()], TyChirho::int_chirho()),
            (
                vec![TyChirho::ForallVarChirho("a".to_owned())],
                TyChirho::char_chirho(),
            ),
        ],
    );
    assert_eq!(
        ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app1_chirho(
            "Frob",
            TyChirho::bool_chirho()
        )),
        TyChirho::char_chirho()
    );
    let stuck_chirho = type_family_app1_chirho("Frob", TyChirho::VarChirho(TyVarChirho(42)));
    assert_eq!(
        ctx_chirho.reduce_type_families_in_ty_chirho(&stuck_chirho),
        stuck_chirho
    );
}

#[test]
fn recursive_family_expansion_shares_one_outer_budget_and_stays_stuck_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    let key_chirho = TypeFamilyIdChirho::SourceChirho {
        module_chirho: "Main".to_owned(),
        name_chirho: "Grow".to_owned(),
    }
    .key_chirho();
    let nested_chirho =
        type_family_app1_chirho(&key_chirho, TyChirho::ForallVarChirho("a".to_owned()));
    ctx_chirho.register_type_family_chirho(
        "Grow".to_owned(),
        1,
        vec![(
            vec![TyChirho::ForallVarChirho("a".to_owned())],
            TyChirho::TupleChirho(vec![nested_chirho.clone(), nested_chirho]),
        )],
    );
    assert!(
        matches!(ctx_chirho.reduce_type_family_chirho("Grow", &[TyChirho::int_chirho()]), Some(TyChirho::TupleChirho(elements_chirho)) if elements_chirho.len() == 2)
    );
    let input_chirho = type_family_app1_chirho("Grow", TyChirho::int_chirho());
    assert_eq!(
        ctx_chirho.reduce_type_families_in_ty_chirho(&input_chirho),
        input_chirho
    );
}

#[test]
fn written_associated_rows_and_defaults_keep_nested_family_identity_chirho() {
    for written_chirho in [false, true] {
        let mut ctx_chirho = family_fixture_context_chirho();
        ctx_chirho.register_type_family_chirho(
            "NestedChirho".to_owned(),
            1,
            vec![(vec![TyChirho::int_chirho()], TyChirho::bool_chirho())],
        );
        let rhs_chirho = |argument_chirho| TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("NestedChirho"))),
            arg_chirho: Box::new(argument_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let class_chirho = DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: dummy_name_chirho("HasRepChirho"),
            type_vars_chirho: vec![dummy_name_chirho("aChirho").into()],
            methods_chirho: vec![],
            associated_tfs_chirho: vec![
                haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
                    name_chirho: dummy_name_chirho("RepChirho"),
                    type_vars_chirho: vec![dummy_name_chirho("aChirho")],
                    data_family_chirho: false,
                    default_rhs_chirho: (!written_chirho)
                        .then(|| rhs_chirho(TypeChirho::VarChirho(dummy_name_chirho("aChirho")))),
                    default_params_chirho: vec![],
                    default_binders_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let instance_chirho = DeclChirho::InstanceDeclChirho {
            context_chirho: vec![],
            class_chirho: dummy_name_chirho("HasRepChirho"),
            types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Int"))],
            methods_chirho: vec![],
            assoc_tf_instances_chirho: if written_chirho {
                vec![AssocTfInstanceChirho {
                    family_name_chirho: dummy_name_chirho("RepChirho"),
                    lhs_types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Int"))],
                    rhs_chirho: rhs_chirho(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }]
            } else {
                vec![]
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        ctx_chirho.process_class_decl_chirho(&class_chirho);
        ctx_chirho.process_instance_decl_chirho(&instance_chirho);
        let key_chirho = ctx_chirho.family_name_key_chirho("RepChirho").unwrap();
        let nested_key_chirho = ctx_chirho.family_name_key_chirho("NestedChirho").unwrap();
        let row_chirho = &ctx_chirho
            .type_families_chirho
            .definition_chirho(&key_chirho)
            .unwrap()
            .branches_chirho()[0];
        assert_eq!(
            row_chirho.rhs_chirho,
            type_family_app1_chirho(&nested_key_chirho, TyChirho::int_chirho())
        );
        assert_eq!(
            ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app1_chirho(
                "RepChirho",
                TyChirho::int_chirho()
            )),
            TyChirho::bool_chirho()
        );
    }
}

fn rhs_shape_module_chirho(rhs_chirho: TypeChirho) -> ModuleChirho {
    let equation_chirho =
        |lhs_chirho, rhs_chirho| haskelujah_ast_chirho::decl_chirho::TypeFamilyEquationChirho {
            head_chirho: Some(dummy_name_chirho("HeldRhsChirho")),
            explicit_binders_chirho: None,
            unwritten_binders_chirho: vec![],
            lhs_types_chirho: vec![lhs_chirho],
            rhs_chirho,
            malformed_chirho: false,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
    ModuleChirho {
        name_chirho: dummy_name_chirho("RhsShapeChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::TypeFamilyDeclChirho {
            name_chirho: dummy_name_chirho("HeldRhsChirho"),
            type_vars_chirho: vec![dummy_name_chirho("aChirho").into()],
            result_kind_chirho: None,
            data_family_chirho: false,
            standalone_kind_chirho: None,
            closed_chirho: true,
            equations_chirho: vec![
                equation_chirho(TypeChirho::ConChirho(dummy_name_chirho("Int")), rhs_chirho),
                equation_chirho(
                    TypeChirho::ConChirho(dummy_name_chirho("Bool")),
                    TypeChirho::ConChirho(dummy_name_chirho("Char")),
                ),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

#[test]
fn rhs_shapes_the_internal_type_erases_hold_the_closed_boundary_chirho() {
    let variable_chirho = TypeChirho::VarChirho(dummy_name_chirho("bChirho"));
    for rhs_chirho in [
        TypeChirho::ForallChirho {
            vars_chirho: vec![dummy_name_chirho("bChirho").into()],
            body_chirho: Box::new(variable_chirho.clone()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        TypeChirho::RequiredForallChirho {
            vars_chirho: vec![dummy_name_chirho("bChirho").into()],
            body_chirho: Box::new(variable_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        TypeChirho::FunChirho {
            arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
            mult_chirho: Some(
                haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::MultVarChirho(
                    dummy_name_chirho("pChirho"),
                ),
            ),
            result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Bool"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ] {
        let module_chirho = rhs_shape_module_chirho(rhs_chirho);
        let mut ctx_chirho = family_fixture_context_chirho();
        ctx_chirho.prepare_family_scope_chirho(&module_chirho, None);
        ctx_chirho.register_module_families_chirho(&module_chirho);
        let key_chirho = ctx_chirho.family_name_key_chirho("HeldRhsChirho").unwrap();
        assert!(
            ctx_chirho
                .type_families_chirho
                .definition_chirho(&key_chirho)
                .unwrap()
                .branches_chirho()[0]
                .held_chirho
        );
        assert_eq!(
            ctx_chirho.reduce_type_family_chirho("HeldRhsChirho", &[TyChirho::bool_chirho()]),
            None
        );
    }
}

#[test]
fn retained_linear_rhs_does_not_hold_an_ordinary_fallback_chirho() {
    let module_chirho = rhs_shape_module_chirho(TypeChirho::FunChirho {
        arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
        mult_chirho: Some(haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho),
        result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Bool"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    });
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.prepare_family_scope_chirho(&module_chirho, None);
    ctx_chirho.register_module_families_chirho(&module_chirho);
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho("HeldRhsChirho", &[TyChirho::int_chirho()]),
        Some(TyChirho::linear_fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::bool_chirho()
        ))
    );
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho("HeldRhsChirho", &[TyChirho::bool_chirho()]),
        Some(TyChirho::char_chirho())
    );
}
