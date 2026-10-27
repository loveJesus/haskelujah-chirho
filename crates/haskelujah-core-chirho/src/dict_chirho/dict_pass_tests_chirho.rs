// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The dictionary-passing transform's unit tests, moved verbatim from `dict_chirho/mod.rs`.

use super::*;
use crate::expr_chirho::{CoreLitChirho, InlineAnnotationChirho};
use haskelujah_typing_chirho::class_chirho::{ClassDeclChirho, ClassEnvChirho, InstDeclChirho};
use haskelujah_typing_chirho::ty_chirho::{SchemePredChirho, TyVarChirho};

fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
    BinderChirho {
        id_chirho: CoreIdChirho(id_chirho),
        name_chirho: name_chirho.to_string(),
        ty_chirho: TyChirho::int_chirho(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn num_var_scheme_chirho() -> SchemeChirho {
    let var_chirho = TyChirho::VarChirho(TyVarChirho(0));
    SchemeChirho {
        vars_chirho: vec![TyVarChirho(0)],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Num".to_string(),
            ty_chirho: var_chirho.clone(),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho([var_chirho.clone(), var_chirho.clone()], var_chirho),
    }
}

#[test]
fn unconstrained_binding_unchanged_chirho() {
    let binding_chirho = CoreBindingChirho {
        binder_chirho: dummy_binder_chirho("f", 0),
        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };
    let scheme_chirho = SchemeChirho::mono_chirho(TyChirho::int_chirho());

    let mut ctx_chirho = DictPassCtxChirho::new_chirho(10, HashMap::new(), HashMap::new());
    let result_chirho = ctx_chirho.add_dict_params_chirho(&binding_chirho, &scheme_chirho);

    // No predicates, so binding is unchanged
    assert_eq!(
        result_chirho.rhs_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
    );
}

#[test]
fn single_predicate_adds_dict_lambda_chirho() {
    let binding_chirho = CoreBindingChirho {
        binder_chirho: dummy_binder_chirho("add", 0),
        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Num".to_string(),
            ty_chirho: TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
        ),
    };

    let mut ctx_chirho = DictPassCtxChirho::new_chirho(10, HashMap::new(), HashMap::new());
    let result_chirho = ctx_chirho.add_dict_params_chirho(&binding_chirho, &scheme_chirho);

    // Should be wrapped in a lambda: \$dNum -> 0
    assert!(matches!(
        result_chirho.rhs_chirho,
        CoreExprChirho::LamChirho { .. }
    ));

    if let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = &result_chirho.rhs_chirho
    {
        assert_eq!(binder_chirho.name_chirho, "$dNum");
        assert_eq!(
            binder_chirho.ty_chirho,
            TyChirho::ConChirho("$Dict_Num".to_string())
        );
        assert_eq!(
            **body_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
        );
    }

    // Binder type should be: $Dict_Num -> (t0 -> t0)
    assert!(matches!(
        result_chirho.binder_chirho.ty_chirho,
        TyChirho::FunChirho(_, _, _)
    ));
}

#[test]
fn multiple_predicates_add_nested_dict_lambdas_chirho() {
    let binding_chirho = CoreBindingChirho {
        binder_chirho: dummy_binder_chirho("cmp", 0),
        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)],
        preds_chirho: vec![
            SchemePredChirho {
                class_name_chirho: "Eq".to_string(),
                ty_chirho: TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
                extra_tys_chirho: vec![],
            },
            SchemePredChirho {
                class_name_chirho: "Ord".to_string(),
                ty_chirho: TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
                extra_tys_chirho: vec![],
            },
        ],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
            TyChirho::bool_chirho(),
        ),
    };

    let mut ctx_chirho = DictPassCtxChirho::new_chirho(10, HashMap::new(), HashMap::new());
    let result_chirho = ctx_chirho.add_dict_params_chirho(&binding_chirho, &scheme_chirho);

    // Should be: \$dEq -> \$dOrd -> 0
    if let CoreExprChirho::LamChirho {
        binder_chirho: outer_chirho,
        body_chirho,
    } = &result_chirho.rhs_chirho
    {
        assert_eq!(outer_chirho.name_chirho, "$dEq");
        if let CoreExprChirho::LamChirho {
            binder_chirho: inner_chirho,
            body_chirho: innermost_chirho,
        } = body_chirho.as_ref()
        {
            assert_eq!(inner_chirho.name_chirho, "$dOrd");
            assert_eq!(
                *innermost_chirho.as_ref(),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
            );
        } else {
            panic!("expected nested lambda");
        }
    } else {
        panic!("expected outer lambda");
    }
}

#[test]
fn build_layouts_from_class_env_chirho() {
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.seed_standard_chirho();

    let mut ctx_chirho = DictPassCtxChirho::new_chirho(0, HashMap::new(), HashMap::new());
    ctx_chirho.build_layouts_chirho(&class_env_chirho);

    // Eq: no supers, and GHC's two methods in sorted slots, `/=` then `==` (row 606).
    // Every Eq dictionary and selector follows this order.
    let eq_layout_chirho = ctx_chirho.layouts_chirho.get("Eq").unwrap();
    assert_eq!(eq_layout_chirho.super_slots_chirho.len(), 0);
    assert_eq!(
        eq_layout_chirho.method_slots_chirho,
        vec![("/=".to_string(), 0), ("==".to_string(), 1)]
    );
    assert_eq!(eq_layout_chirho.field_count_chirho, 2);

    // Ord: 1 super (Eq), 1 method (compare)
    let ord_layout_chirho = ctx_chirho.layouts_chirho.get("Ord").unwrap();
    assert_eq!(ord_layout_chirho.super_slots_chirho.len(), 1);
    assert_eq!(ord_layout_chirho.super_slots_chirho[0].0, "Eq");
    assert_eq!(ord_layout_chirho.method_slots_chirho.len(), 1);
    assert_eq!(ord_layout_chirho.method_slots_chirho[0].0, "compare");
    assert_eq!(ord_layout_chirho.field_count_chirho, 2);

    // Num: no supers (GHC dropped Eq and Show from Num in 7.4; the old
    // expectation here encoded the Haskell 98 report, not GHC), 7 methods
    // (+, *, -, negate, fromInteger, abs, signum)
    let num_layout_chirho = ctx_chirho.layouts_chirho.get("Num").unwrap();
    assert_eq!(num_layout_chirho.super_slots_chirho.len(), 0);
    assert_eq!(num_layout_chirho.method_slots_chirho.len(), 7);
    assert_eq!(num_layout_chirho.field_count_chirho, 7);

    // Integral: GHC's two supers (Real, Enum)
    let integral_layout_chirho = ctx_chirho.layouts_chirho.get("Integral").unwrap();
    assert_eq!(
        integral_layout_chirho
            .super_slots_chirho
            .iter()
            .map(|(name_chirho, _)| name_chirho.as_str())
            .collect::<Vec<_>>(),
        vec!["Real", "Enum"]
    );
}

#[test]
fn generate_selectors_creates_bindings_chirho() {
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.seed_standard_chirho();

    let mut ctx_chirho = DictPassCtxChirho::new_chirho(0, HashMap::new(), HashMap::new());
    ctx_chirho.build_layouts_chirho(&class_env_chirho);
    ctx_chirho.generate_selectors_chirho();

    // Should have selectors for each method across all classes
    assert!(!ctx_chirho.generated_bindings_chirho.is_empty());

    // Check that the == and /= selectors exist, both Eq's (row 606)
    assert!(ctx_chirho.method_selectors_chirho.contains_key("=="));
    assert_eq!(
        ctx_chirho
            .method_selectors_chirho
            .get("/=")
            .map(|(class_name_chirho, _)| class_name_chirho.as_str()),
        Some("Eq")
    );

    // Check that the selector is a lambda wrapping a case
    let (class_name_chirho, _sel_id_chirho) = ctx_chirho.method_selectors_chirho.get("==").unwrap();
    assert_eq!(class_name_chirho, "Eq");

    // Find the selector binding
    let sel_binding_chirho = ctx_chirho
        .generated_bindings_chirho
        .iter()
        .find(|b_chirho| b_chirho.binder_chirho.name_chirho == "$sel_Eq_==")
        .expect("selector binding should exist");

    assert!(matches!(
        sel_binding_chirho.rhs_chirho,
        CoreExprChirho::LamChirho { .. }
    ));
}

#[test]
fn max_id_finder_chirho() {
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("f", 5),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 10),
                body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(42))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
    };
    assert_eq!(find_max_id_chirho(&module_chirho), 42);
}

#[test]
fn instance_dicts_generated_for_ground_instances_chirho() {
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.seed_standard_chirho();

    let mut ctx_chirho = DictPassCtxChirho::new_chirho(0, HashMap::new(), HashMap::new());
    ctx_chirho.build_layouts_chirho(&class_env_chirho);
    ctx_chirho.generate_selectors_chirho();
    ctx_chirho.generate_instance_dicts_chirho(&class_env_chirho);

    // Should have generated instance dicts for ground instances
    // (Eq Int, Eq Char, Eq Bool, Show Int, Show Char, Show Bool,
    //  Ord Int, Ord Char, Num Int)
    assert!(!ctx_chirho.instance_dicts_chirho.is_empty());

    // Check Eq Int dict exists
    assert!(
        ctx_chirho
            .instance_dicts_chirho
            .contains_key(&("Eq".to_string(), "Int".to_string()))
    );

    // Check Num Int dict exists
    assert!(
        ctx_chirho
            .instance_dicts_chirho
            .contains_key(&("Num".to_string(), "Int".to_string()))
    );

    // Find the $fEqInt binding
    let eq_int_binding_chirho = ctx_chirho
        .generated_bindings_chirho
        .iter()
        .find(|b_chirho| b_chirho.binder_chirho.name_chirho == "$fEqInt")
        .expect("$fEqInt binding should exist");

    // Should be a constructor application (ConAppChirho)
    assert!(matches!(
        eq_int_binding_chirho.rhs_chirho,
        CoreExprChirho::ConAppChirho { .. }
    ));
}

#[test]
fn missing_method_ignores_dangling_name_map_entry_chirho() {
    let mut methods_chirho = HashMap::new();
    methods_chirho.insert(
        "needed".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::int_chirho(),
        )),
    );

    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.add_class_chirho(ClassDeclChirho {
        name_chirho: "Needs".to_string(),
        supers_chirho: vec![],
        var_chirho: haskelujah_typing_chirho::ty_chirho::TyVarChirho(0),
        methods_chirho,
        extra_vars_chirho: vec![],
        fundeps_chirho: vec![],
        defaults_chirho: HashMap::new(),
    });
    class_env_chirho.add_instance_chirho(InstDeclChirho {
        class_name_chirho: "Needs".to_string(),
        head_ty_chirho: TyChirho::int_chirho(),
        extra_head_tys_chirho: vec![],
        context_chirho: vec![],
    });

    let mut names_chirho = HashMap::new();
    names_chirho.insert(CoreIdChirho(7), "$prim_Needs_needed_Int".to_string());
    let mut ctx_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
    ctx_chirho.build_layouts_chirho(&class_env_chirho);
    ctx_chirho.generate_instance_dicts_chirho(&class_env_chirho);

    let dict_binding_chirho = ctx_chirho
        .generated_bindings_chirho
        .iter()
        .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == "$fNeedsInt")
        .expect("Needs Int dictionary should be generated");
    let method_id_chirho = match &dict_binding_chirho.rhs_chirho {
        CoreExprChirho::ConAppChirho { args_chirho, .. } => match args_chirho.first() {
            Some(CoreExprChirho::VarChirho(id_chirho)) => *id_chirho,
            other_chirho => panic!("expected method var slot, got {other_chirho:?}"),
        },
        other_chirho => panic!("expected dictionary constructor, got {other_chirho:?}"),
    };

    assert_ne!(
        method_id_chirho,
        CoreIdChirho(7),
        "dangling name-map entry must not be used as a method body"
    );
    let missing_binding_chirho = ctx_chirho
        .generated_bindings_chirho
        .iter()
        .find(|binding_chirho| binding_chirho.binder_chirho.id_chirho == method_id_chirho)
        .expect("missing-method body should be generated");
    assert_eq!(
        missing_binding_chirho.binder_chirho.name_chirho,
        "$prim_Needs_needed_Int"
    );
    assert!(
        matches!(
            missing_binding_chirho.rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ),
        "missing method should be a callable loud-error body"
    );
}

#[test]
fn method_ref_rewritten_to_selector_app_chirho() {
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.seed_standard_chirho();

    // Set up names map: id 5 is "+"
    let mut names_chirho = HashMap::new();
    names_chirho.insert(CoreIdChirho(5), "+".to_string());

    let mut ctx_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
    ctx_chirho.build_layouts_chirho(&class_env_chirho);
    ctx_chirho.generate_selectors_chirho();

    let binding_chirho = CoreBindingChirho {
        binder_chirho: dummy_binder_chirho("add", 8),
        rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(5)),
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };
    let result_chirho =
        ctx_chirho.add_dict_params_chirho(&binding_chirho, &num_var_scheme_chirho());

    // Should be: ($sel_Num_+ $dNum) where $dNum is the local evidence binder.
    if let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = &result_chirho.rhs_chirho
    {
        if let CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } = body_chirho.as_ref()
        {
            // The function should be the selector
            assert!(matches!(**fun_chirho, CoreExprChirho::VarChirho(_)));
            // The arg should be the locally-proven Num dictionary
            assert_eq!(
                **arg_chirho,
                CoreExprChirho::VarChirho(binder_chirho.id_chirho)
            );
        } else {
            panic!("expected method ref to be rewritten to selector application");
        }
    } else {
        panic!("expected Num dictionary lambda");
    }
}

#[test]
fn method_ref_uses_transitive_superclass_dict_chirho() {
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.seed_standard_chirho();

    let mut names_chirho = HashMap::new();
    names_chirho.insert(CoreIdChirho(5), "==".to_string());

    let mut ctx_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
    ctx_chirho.build_layouts_chirho(&class_env_chirho);
    ctx_chirho.generate_selectors_chirho();

    let binding_chirho = CoreBindingChirho {
        binder_chirho: dummy_binder_chirho("eq_from_integral", 8),
        rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(5)),
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };
    let integral_var_chirho = TyChirho::VarChirho(TyVarChirho(0));
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![TyVarChirho(0)],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Integral".to_string(),
            ty_chirho: integral_var_chirho.clone(),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_chirho(integral_var_chirho, TyChirho::bool_chirho()),
    };

    let result_chirho = ctx_chirho.add_dict_params_chirho(&binding_chirho, &scheme_chirho);

    // GHC's chain: Integral => Real => Ord => Eq. Num is not on it.
    let rhs_debug_chirho = format!("{:#?}", result_chirho.rhs_chirho);
    for dict_name_chirho in ["$dReal", "$dOrd", "$dEq"] {
        assert!(
            rhs_debug_chirho.contains(&format!("name_chirho: \"{dict_name_chirho}\"")),
            "missing {dict_name_chirho}: {rhs_debug_chirho}"
        );
    }
}

#[test]
fn non_method_var_unchanged_in_rewrite_chirho() {
    let mut names_chirho = HashMap::new();
    names_chirho.insert(CoreIdChirho(5), "x".to_string());

    let ctx_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());

    let dict_vars_chirho = HashMap::new();
    let expr_chirho = CoreExprChirho::VarChirho(CoreIdChirho(5));
    let result_chirho = ctx_chirho.rewrite_method_refs_chirho(&expr_chirho, &dict_vars_chirho);

    // "x" is not an overloaded method, should be unchanged
    assert_eq!(result_chirho, CoreExprChirho::VarChirho(CoreIdChirho(5)));
}

#[test]
fn rewrite_inside_app_chirho() {
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.seed_standard_chirho();

    let mut names_chirho = HashMap::new();
    names_chirho.insert(CoreIdChirho(5), "+".to_string());
    names_chirho.insert(CoreIdChirho(6), "x".to_string());
    names_chirho.insert(CoreIdChirho(7), "y".to_string());

    let mut ctx_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
    ctx_chirho.build_layouts_chirho(&class_env_chirho);
    ctx_chirho.generate_selectors_chirho();

    // Expression: (+) x y → App(App(+, x), y)
    let binding_chirho = CoreBindingChirho {
        binder_chirho: dummy_binder_chirho("add", 8),
        rhs_chirho: CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(5))),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(6))),
            }),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(7))),
        },
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };
    let result_chirho =
        ctx_chirho.add_dict_params_chirho(&binding_chirho, &num_var_scheme_chirho());

    // The "+" reference should be rewritten, "x" and "y" should not
    if let CoreExprChirho::LamChirho { body_chirho, .. } = &result_chirho.rhs_chirho {
        if let CoreExprChirho::AppChirho { fun_chirho, .. } = body_chirho.as_ref() {
            if let CoreExprChirho::AppChirho {
                fun_chirho: inner_fun_chirho,
                arg_chirho: inner_arg_chirho,
            } = fun_chirho.as_ref()
            {
                // inner_fun should be ($sel_Num_+ $dNum), which is an App
                assert!(matches!(
                    **inner_fun_chirho,
                    CoreExprChirho::AppChirho { .. }
                ));
                // inner_arg should be unchanged x
                assert_eq!(
                    **inner_arg_chirho,
                    CoreExprChirho::VarChirho(CoreIdChirho(6))
                );
            } else {
                panic!("expected nested App");
            }
        } else {
            panic!("expected rewritten method application");
        }
    } else {
        panic!("expected Num dictionary lambda");
    }
}

#[test]
fn transform_module_adds_selectors_and_wraps_chirho() {
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.seed_standard_chirho();

    let mut type_env_chirho = TyEnvChirho::new_chirho();
    // Register "add" as Num a => a -> a -> a
    type_env_chirho.bind_chirho(
        "add".to_string(),
        SchemeChirho {
            vars_chirho: vec![haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Num".to_string(),
                ty_chirho: TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                [
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
                ],
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
            ),
        },
    );

    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("add", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
    };

    let result_chirho = dict_pass_module_chirho(
        &module_chirho,
        HashMap::new(),
        &type_env_chirho,
        &class_env_chirho,
    );

    // Should have generated selector bindings + the transformed "add" binding
    assert!(result_chirho.module_chirho.bindings_chirho.len() > 1);

    // Find "add" binding — it should be wrapped in a dict lambda
    let add_binding_chirho = result_chirho
        .module_chirho
        .bindings_chirho
        .iter()
        .find(|b_chirho| b_chirho.binder_chirho.name_chirho == "add")
        .expect("add binding should exist");

    assert!(matches!(
        add_binding_chirho.rhs_chirho,
        CoreExprChirho::LamChirho { .. }
    ));

    // Layouts should be populated
    assert!(result_chirho.layouts_chirho.contains_key("Eq"));
    assert!(result_chirho.layouts_chirho.contains_key("Num"));
}

#[test]
fn a_concrete_top_level_value_supplies_its_type_key_chirho() {
    let mut type_env_chirho = TyEnvChirho::new_chirho();
    type_env_chirho.bind_chirho(
        "xChirho".to_string(),
        SchemeChirho::mono_chirho(TyChirho::ConChirho("Double".to_string())),
    );
    type_env_chirho.bind_chirho(
        "main".to_string(),
        SchemeChirho::mono_chirho(TyChirho::ConChirho("Double".to_string())),
    );

    let x_id_chirho = CoreIdChirho(1);
    let main_id_chirho = CoreIdChirho(0);
    let mut names_chirho = HashMap::new();
    names_chirho.insert(x_id_chirho, "xChirho".to_string());
    names_chirho.insert(main_id_chirho, "main".to_string());
    let module_chirho = CoreModuleChirho {
        name_chirho: "TopLevelDoubleChirho".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("xChirho", x_id_chirho.0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(1.5)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", main_id_chirho.0),
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "negate#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_id_chirho)],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: names_chirho.clone(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: Vec::new(),
    };

    let result_chirho = dict_pass_module_chirho(
        &module_chirho,
        names_chirho,
        &type_env_chirho,
        &ClassEnvChirho::new_chirho(),
    );
    let main_chirho = result_chirho
        .module_chirho
        .bindings_chirho
        .iter()
        .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == "main")
        .expect("main binding should exist");
    assert!(
        matches!(
            &main_chirho.rhs_chirho,
            CoreExprChirho::PrimOpChirho { name_chirho, args_chirho }
                if name_chirho == "negateFloat#"
                    && args_chirho == &[CoreExprChirho::VarChirho(x_id_chirho)]
        ),
        "the concrete top-level Double must retarget negate#: {:?}",
        main_chirho.rhs_chirho
    );
}

/// Evidence-threading P1 helpers: a module whose `main` applies a method
/// OCCURRENCE id (50) of `==` to two Int literals; canonical shared id 60;
/// a body-backed `$prim_Eq_==_Int` row exists as id 100.
fn occurrence_test_module_chirho() -> (
    CoreModuleChirho,
    HashMap<CoreIdChirho, String>,
    HashMap<CoreIdChirho, (String, CoreIdChirho)>,
) {
    let mut names_chirho = HashMap::new();
    names_chirho.insert(CoreIdChirho(0), "main".to_string());
    names_chirho.insert(CoreIdChirho(50), "==".to_string());
    names_chirho.insert(CoreIdChirho(60), "==".to_string());
    names_chirho.insert(CoreIdChirho(100), "$prim_Eq_==_Int".to_string());
    let main_rhs_chirho = CoreExprChirho::AppChirho {
        fun_chirho: Box::new(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(50))),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))),
        }),
        arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2))),
    };
    let module_chirho = CoreModuleChirho {
        name_chirho: "OccTest".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("$prim_Eq_==_Int", 100),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(7)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 0),
                rhs_chirho: main_rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: names_chirho.clone(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: Vec::new(),
    };
    let mut canon_chirho = HashMap::new();
    canon_chirho.insert(CoreIdChirho(50), ("==".to_string(), CoreIdChirho(60)));
    (module_chirho, names_chirho, canon_chirho)
}

fn app_spine_head_id_chirho(expr_chirho: &CoreExprChirho) -> Option<CoreIdChirho> {
    let mut current_chirho = expr_chirho;
    while let CoreExprChirho::AppChirho { fun_chirho, .. } = current_chirho {
        current_chirho = fun_chirho.as_ref();
    }
    match current_chirho {
        CoreExprChirho::VarChirho(id_chirho) => Some(*id_chirho),
        _ => None,
    }
}

#[test]
fn evidence_occurrence_dispatches_to_prim_row_chirho() {
    let (module_chirho, names_chirho, canon_chirho) = occurrence_test_module_chirho();
    let mut evidence_chirho = HashMap::new();
    evidence_chirho.insert(CoreIdChirho(50), ("Eq".to_string(), "Int".to_string()));
    let result_chirho = dict_pass_module_full_with_method_occurrences_chirho(
        &module_chirho,
        names_chirho,
        &TyEnvChirho::new_chirho(),
        &ClassEnvChirho::new_chirho(),
        HashMap::new(),
        HashMap::new(),
        HashSet::new(),
        canon_chirho,
        evidence_chirho,
    );
    let main_chirho = result_chirho
        .module_chirho
        .bindings_chirho
        .iter()
        .find(|b_chirho| b_chirho.binder_chirho.name_chirho == "main")
        .expect("main binding should exist");
    let head_id_chirho = app_spine_head_id_chirho(&main_chirho.rhs_chirho)
        .expect("main body should stay an application spine");
    assert_eq!(
        result_chirho
            .names_chirho
            .get(&head_id_chirho)
            .map(String::as_str),
        Some("$prim_Eq_==_Int"),
        "evidence must dispatch the occurrence head to the $prim row"
    );
}

#[test]
fn occurrence_without_evidence_restores_canonical_chirho() {
    let (module_chirho, names_chirho, canon_chirho) = occurrence_test_module_chirho();
    let result_chirho = dict_pass_module_full_with_method_occurrences_chirho(
        &module_chirho,
        names_chirho,
        &TyEnvChirho::new_chirho(),
        &ClassEnvChirho::new_chirho(),
        HashMap::new(),
        HashMap::new(),
        HashSet::new(),
        canon_chirho,
        HashMap::new(),
    );
    let main_chirho = result_chirho
        .module_chirho
        .bindings_chirho
        .iter()
        .find(|b_chirho| b_chirho.binder_chirho.name_chirho == "main")
        .expect("main binding should exist");
    let head_id_chirho = app_spine_head_id_chirho(&main_chirho.rhs_chirho)
        .expect("main body should stay an application spine");
    assert_eq!(
        head_id_chirho,
        CoreIdChirho(60),
        "without evidence the occurrence must be restored to the canonical id (guaranteed elimination)"
    );
}

// ── Row 580: the generated Core of `pi` and `atan2` at Double ──

/// The dictionary pass over a module with these bindings and names, with the standard
/// classes and instances.
fn standard_pass_chirho(
    bindings_chirho: Vec<CoreBindingChirho>,
    names_chirho: &[(u32, &str)],
) -> DictPassResultChirho {
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.seed_standard_chirho();
    let names_chirho: HashMap<CoreIdChirho, String> = names_chirho
        .iter()
        .map(|(id_chirho, name_chirho)| (CoreIdChirho(*id_chirho), name_chirho.to_string()))
        .collect();
    let module_chirho = CoreModuleChirho {
        name_chirho: "FloatingMethodsChirho".to_string(),
        bindings_chirho,
        names_chirho: names_chirho.clone(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
    };
    dict_pass_module_chirho(
        &module_chirho,
        names_chirho,
        &TyEnvChirho::new_chirho(),
        &class_env_chirho,
    )
}

/// Every binding of the passed module named `name_chirho`.
fn named_chirho<'a>(
    result_chirho: &'a DictPassResultChirho,
    name_chirho: &str,
) -> Vec<&'a CoreBindingChirho> {
    result_chirho
        .module_chirho
        .bindings_chirho
        .iter()
        .filter(|binding_chirho| binding_chirho.binder_chirho.name_chirho == name_chirho)
        .collect()
}

/// Whether `expr_chirho` or any expression inside it passes `test_chirho`.
fn any_part_chirho(
    expr_chirho: &CoreExprChirho,
    test_chirho: &dyn Fn(&CoreExprChirho) -> bool,
) -> bool {
    let mut found_chirho = test_chirho(expr_chirho);
    crate::transform_chirho::children_chirho(expr_chirho, &mut |child_chirho| {
        found_chirho = found_chirho || any_part_chirho(child_chirho, test_chirho);
    });
    found_chirho
}

/// `pi` is one Double value: its dictionary row holds π's bits, its Prelude name points
/// at that row, the Floating Double dictionary refers to it, and nothing applies it as
/// the old `\u -> pi# u` was applied. No generated binding names `pi#`.
#[test]
fn pi_is_one_double_value_chirho() {
    let result_chirho = standard_pass_chirho(vec![], &[]);
    let [row_chirho] = named_chirho(&result_chirho, "$prim_Floating_pi_Double")[..] else {
        panic!("one pi row");
    };
    let pi_id_chirho = row_chirho.binder_chirho.id_chirho;
    assert_eq!(
        row_chirho.binder_chirho.ty_chirho,
        TyChirho::double_chirho()
    );
    assert!(matches!(
        row_chirho.rhs_chirho,
        CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(value_chirho))
            if value_chirho.to_bits() == std::f64::consts::PI.to_bits()
    ));
    let [alias_chirho] = named_chirho(&result_chirho, "pi")[..] else {
        panic!("one pi alias");
    };
    assert_eq!(
        alias_chirho.binder_chirho.ty_chirho,
        TyChirho::double_chirho()
    );
    assert!(matches!(
        alias_chirho.rhs_chirho,
        CoreExprChirho::VarChirho(id_chirho) if id_chirho == pi_id_chirho
    ));
    let refers_chirho = |expr_chirho: &CoreExprChirho| matches!(expr_chirho, CoreExprChirho::VarChirho(id_chirho) if *id_chirho == pi_id_chirho);
    assert!(
        named_chirho(&result_chirho, "$fFloatingDouble")
            .iter()
            .any(|dictionary_chirho| any_part_chirho(
                &dictionary_chirho.rhs_chirho,
                &refers_chirho
            ))
    );
    for binding_chirho in &result_chirho.module_chirho.bindings_chirho {
        let applies_pi_chirho = any_part_chirho(
            &binding_chirho.rhs_chirho,
            &|expr_chirho| matches!(expr_chirho, CoreExprChirho::AppChirho { fun_chirho, .. } if refers_chirho(fun_chirho)),
        );
        let names_pi_hash_chirho = any_part_chirho(
            &binding_chirho.rhs_chirho,
            &|expr_chirho| matches!(expr_chirho, CoreExprChirho::PrimOpChirho { name_chirho, .. } if name_chirho == "pi#"),
        );
        assert!(
            !applies_pi_chirho && !names_pi_hash_chirho,
            "{}",
            binding_chirho.binder_chirho.name_chirho
        );
    }
}

/// `atan2` at Double is `\y x -> atan2# y x`, the RTS's one transcription of GHC's
/// RealFloat default, and nothing else; its Prelude name points at that row.
#[test]
fn atan2_is_the_one_routine_chirho() {
    let result_chirho = standard_pass_chirho(vec![], &[]);
    let [row_chirho] = named_chirho(&result_chirho, "$prim_Floating_atan2_Double")[..] else {
        panic!("one atan2 row");
    };
    let double_chirho = TyChirho::double_chirho();
    assert_eq!(
        row_chirho.binder_chirho.ty_chirho,
        TyChirho::fun_n_chirho(
            [double_chirho.clone(), double_chirho.clone()],
            double_chirho
        )
    );
    let CoreExprChirho::LamChirho {
        binder_chirho: y_chirho,
        body_chirho,
    } = &row_chirho.rhs_chirho
    else {
        panic!("atan2 takes y");
    };
    let CoreExprChirho::LamChirho {
        binder_chirho: x_chirho,
        body_chirho,
    } = &**body_chirho
    else {
        panic!("atan2 takes x");
    };
    assert_eq!(
        **body_chirho,
        CoreExprChirho::PrimOpChirho {
            name_chirho: "atan2#".to_string(),
            args_chirho: vec![
                CoreExprChirho::VarChirho(y_chirho.id_chirho),
                CoreExprChirho::VarChirho(x_chirho.id_chirho),
            ],
        }
    );
    let [alias_chirho] = named_chirho(&result_chirho, "atan2")[..] else {
        panic!("one atan2 alias");
    };
    assert!(matches!(
        alias_chirho.rhs_chirho,
        CoreExprChirho::VarChirho(id_chirho) if id_chirho == row_chirho.binder_chirho.id_chirho
    ));
}

/// A module whose `localChirho` binds its own `name_chirho` (id 9) in a `let`. With
/// `free_chirho`, its `useChirho` also uses the Prelude's `name_chirho` free (id 5).
fn local_named_module_chirho(
    name_chirho: &str,
    (ty_chirho, lit_chirho): (TyChirho, CoreLitChirho),
    free_chirho: bool,
) -> (Vec<CoreBindingChirho>, Vec<(u32, &str)>) {
    let local_chirho = BinderChirho {
        id_chirho: CoreIdChirho(9),
        name_chirho: name_chirho.to_string(),
        ty_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let use_chirho = if free_chirho {
        CoreExprChirho::VarChirho(CoreIdChirho(5))
    } else {
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
    };
    let bindings_chirho = vec![
        CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("useChirho", 1),
            rhs_chirho: use_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        },
        CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("localChirho", 2),
            rhs_chirho: CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(local_chirho, CoreExprChirho::LitChirho(lit_chirho))],
                body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(9))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        },
    ];
    let mut names_chirho = vec![(1, "useChirho"), (2, "localChirho"), (9, name_chirho)];
    if free_chirho {
        names_chirho.push((5, name_chirho));
    }
    (bindings_chirho, names_chirho)
}

/// The ids of the bindings named `name_chirho` after each of 16 passes over the module.
/// Each pass hashes its name table afresh.
fn named_ids_per_pass_chirho(
    bindings_chirho: &[CoreBindingChirho],
    names_chirho: &[(u32, &str)],
    name_chirho: &str,
) -> Vec<Vec<CoreIdChirho>> {
    (0..16)
        .map(|_| {
            let result_chirho = standard_pass_chirho(bindings_chirho.to_vec(), names_chirho);
            named_chirho(&result_chirho, name_chirho)
                .iter()
                .map(|binding_chirho| binding_chirho.binder_chirho.id_chirho)
                .collect()
        })
        .collect()
}

/// A local `pi` never takes the Prelude alias. The module uses the Prelude's `pi` free (id
/// 5) and binds its own `pi` in a `let` (id 9). The alias binds id 5 in every pass, where
/// resolving by name alone took id 9 in about half of them.
#[test]
fn a_local_pi_never_takes_the_prelude_alias_chirho() {
    let (bindings_chirho, names_chirho) = local_named_module_chirho(
        "pi",
        (TyChirho::double_chirho(), CoreLitChirho::FloatChirho(3.0)),
        true,
    );
    for ids_chirho in named_ids_per_pass_chirho(&bindings_chirho, &names_chirho, "pi") {
        assert_eq!(ids_chirho, vec![CoreIdChirho(5)]);
    }
}

/// Row 594: a generated Prelude body takes its id as the alias does. Beside a local
/// `length` (id 9), the generated `length` binds the id the module uses free (id 5) in
/// every pass. With no free use, the generated `reverse` never takes the local
/// `reverse`'s id.
#[test]
fn a_generated_prelude_binding_never_takes_a_local_binders_id_chirho() {
    let int_local_chirho = (TyChirho::int_chirho(), CoreLitChirho::IntChirho(99));
    let (bindings_chirho, names_chirho) =
        local_named_module_chirho("length", int_local_chirho.clone(), true);
    for ids_chirho in named_ids_per_pass_chirho(&bindings_chirho, &names_chirho, "length") {
        assert_eq!(ids_chirho, vec![CoreIdChirho(5)]);
    }
    let (bindings_chirho, names_chirho) =
        local_named_module_chirho("reverse", int_local_chirho, false);
    for ids_chirho in named_ids_per_pass_chirho(&bindings_chirho, &names_chirho, "reverse") {
        assert!(
            matches!(ids_chirho[..], [id_chirho] if id_chirho != CoreIdChirho(9)),
            "{ids_chirho:?}"
        );
    }
}

/// Row 594: the program's own top-level `length` (id 3) beside a free use of the Prelude's
/// (id 5), as `import Prelude hiding (length)` with a qualified `P.length` gives. The
/// generated `length` binds the free id in every pass, never the program's binder. Row
/// 597 spells it `$prelude_length`, apart from the program's `length`, which keeps its
/// name.
#[test]
fn a_generated_prelude_binding_takes_the_free_id_beside_a_top_level_namesake_chirho() {
    let bindings_chirho = vec![
        CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("useChirho", 1),
            rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(5)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        },
        CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("length", 3),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        },
    ];
    let names_chirho = [(1, "useChirho"), (3, "length"), (5, "length")];
    for _ in 0..16 {
        let result_chirho = standard_pass_chirho(bindings_chirho.clone(), &names_chirho);
        assert_eq!(
            ids_named_chirho(&result_chirho, "length"),
            vec![CoreIdChirho(3)]
        );
        assert_eq!(
            ids_named_chirho(&result_chirho, "$prelude_length"),
            vec![CoreIdChirho(5)]
        );
    }
}

/// The ids of the bindings named `name_chirho` in one pass's module.
fn ids_named_chirho(result_chirho: &DictPassResultChirho, name_chirho: &str) -> Vec<CoreIdChirho> {
    named_chirho(result_chirho, name_chirho)
        .iter()
        .map(|binding_chirho| binding_chirho.binder_chirho.id_chirho)
        .collect()
}

/// Row 597: a program's own top-level `insert` (id 3) is a function it wrote, as a binary
/// search tree's is. The generated Int `sort` calls the compiler's `insert`, a binding of
/// its own spelled `$prelude_insert`. No generated binding reaches id 3, and the program's
/// binding keeps its id and its name. Before, both were one binding: the program's body
/// won, and `sort` failed on its patterns.
#[test]
fn generated_code_never_reaches_a_program_written_namesake_chirho() {
    let bindings_chirho = vec![CoreBindingChirho {
        binder_chirho: dummy_binder_chirho("insert", 3),
        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99)),
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    }];
    let result_chirho = standard_pass_chirho(bindings_chirho, &[(3, "insert")]);
    assert_eq!(
        ids_named_chirho(&result_chirho, "insert"),
        vec![CoreIdChirho(3)]
    );
    let [owned_chirho] = ids_named_chirho(&result_chirho, "$prelude_insert")[..] else {
        panic!("one compiler-owned insert");
    };
    assert_ne!(owned_chirho, CoreIdChirho(3));
    let reaches_program_chirho = |expr_chirho: &CoreExprChirho| matches!(expr_chirho, CoreExprChirho::VarChirho(id_chirho) if *id_chirho == CoreIdChirho(3));
    assert!(
        result_chirho
            .module_chirho
            .bindings_chirho
            .iter()
            .filter(|binding_chirho| binding_chirho.binder_chirho.id_chirho != CoreIdChirho(3))
            .all(|binding_chirho| !any_part_chirho(
                &binding_chirho.rhs_chirho,
                &reaches_program_chirho
            ))
    );
}

/// Row 597: the compiler's own spellings, which no program can write, stay resolvable for
/// generated code; a source operator that starts with `$` does not count as one.
#[test]
fn only_the_compilers_own_spellings_are_internal_chirho() {
    for internal_chirho in [
        "$prim_Show_show_Bool",
        "$fShowInt",
        "$w",
        "$sel_Num_+",
        "$_x",
    ] {
        assert!(
            is_compiler_internal_name_chirho(internal_chirho),
            "{internal_chirho}"
        );
    }
    for source_chirho in ["$", "$!", "foldl", "insert", "++", ""] {
        assert!(
            !is_compiler_internal_name_chirho(source_chirho),
            "{source_chirho}"
        );
    }
}
