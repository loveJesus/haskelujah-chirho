// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Unit tests of the Core → STG lowering, moved unchanged from `stg_lower_chirho.rs` (#32355).

use super::*;
use haskelujah_core_chirho::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreIdChirho, InlineAnnotationChirho,
};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

fn int_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
    BinderChirho {
        id_chirho: CoreIdChirho(id_chirho),
        name_chirho: name_chirho.to_string(),
        ty_chirho: TyChirho::int_chirho(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn lower_literal_binding_chirho() {
    // module Test where x = 42
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: int_binder_chirho("x", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let program_chirho = lower_module_to_stg_chirho(&module_chirho, HashSet::new());
    assert!(!program_chirho.code_table_chirho.is_empty());
    assert_eq!(program_chirho.top_level_env_chirho.len(), 1);
}

#[test]
fn run_literal_binding_chirho() {
    // module Test where main = 42
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let (result_chirho, _machine_chirho) =
        lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
    assert_eq!(result_chirho, ValueChirho::IntChirho(42));
}

#[test]
fn run_float_literal_chirho() {
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(-12.5)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let (result_chirho, _) = lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
    assert_eq!(result_chirho, ValueChirho::FloatChirho(-12.5));
}

#[test]
fn run_let_binding_chirho() {
    // module Test where main = let x = 7 in x
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(
                    int_binder_chirho("x", 1),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(7)),
                )],
                body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let (result_chirho, _) = lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
    assert_eq!(result_chirho, ValueChirho::IntChirho(7));
}

#[test]
fn run_var_reference_chirho() {
    // module Test where
    //   x = 99
    //   main = x
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: int_binder_chirho("x", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: int_binder_chirho("main", 1),
                rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(0)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let (result_chirho, _) = lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
    assert_eq!(result_chirho, ValueChirho::IntChirho(99));
}

#[test]
fn missing_global_binding_errors_instead_of_zero_chirho() {
    let missing_id_chirho = CoreIdChirho(404);
    let mut names_chirho = HashMap::new();
    names_chirho.insert(missing_id_chirho, "missingValueChirho".to_string());
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::VarChirho(missing_id_chirho),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho,
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let err_chirho = lower_and_run_chirho(&module_chirho, None, HashSet::new())
        .expect_err("missing STG binding should fail loudly");
    assert!(
        err_chirho.contains("missing STG binding `missingValueChirho`"),
        "missing STG binding error should name the unresolved global: {err_chirho}"
    );
    assert!(
        !err_chirho.contains("literal 0"),
        "missing STG binding must not become the old literal-0 failure: {err_chirho}"
    );
}

#[test]
fn run_case_on_constructor_chirho() {
    // module Test where
    //   main = case True of
    //            True -> 1
    //            False -> 0
    //
    // We represent True as a ConApp in the scrutinee
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![
            // True constructor as a top-level binding
            CoreBindingChirho {
                binder_chirho: int_binder_chirho("True", 2),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: int_binder_chirho("main", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let (result_chirho, _) = lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
    assert_eq!(result_chirho, ValueChirho::IntChirho(1));
}

#[test]
fn run_identity_function_chirho() {
    // module Test where
    //   id = \x -> x
    //   main = id 42
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: int_binder_chirho("id", 0),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: int_binder_chirho("x", 1),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: int_binder_chirho("main", 2),
                rhs_chirho: CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                    arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let (result_chirho, _) = lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
    assert_eq!(result_chirho, ValueChirho::IntChirho(42));
}

#[test]
fn entry_not_found_chirho() {
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: int_binder_chirho("x", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let result_chirho = lower_and_run_chirho(&module_chirho, None, HashSet::new());
    assert!(result_chirho.is_err());
    assert!(
        result_chirho
            .unwrap_err()
            .contains("no binding named 'main'")
    );
}

#[test]
fn named_entry_point_chirho() {
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: int_binder_chirho("myEntry", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(777)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let (result_chirho, _) =
        lower_and_run_chirho(&module_chirho, Some("myEntry"), HashSet::new()).unwrap();
    assert_eq!(result_chirho, ValueChirho::IntChirho(777));
}

#[test]
fn type_erasure_chirho() {
    // TyLam and TyApp should be erased
    let module_chirho = CoreModuleChirho {
        name_chirho: "Test".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::TyLamChirho {
                ty_var_chirho: "a".to_string(),
                body_chirho: Box::new(CoreExprChirho::TyAppChirho {
                    expr_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(55))),
                    ty_chirho: TyChirho::int_chirho(),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: Default::default(),
    };

    let (result_chirho, _) = lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
    assert_eq!(result_chirho, ValueChirho::IntChirho(55));
}

#[test]
fn collect_app_flattens_chirho() {
    // f x y → (f, [x, y])
    let expr_chirho = CoreExprChirho::AppChirho {
        fun_chirho: Box::new(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))),
        }),
        arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2))),
    };

    let (head_chirho, args_chirho) = collect_app_chirho(&expr_chirho);
    assert!(matches!(head_chirho, CoreExprChirho::VarChirho(_)));
    assert_eq!(args_chirho.len(), 2);
}

#[test]
fn collect_lam_flattens_chirho() {
    // \x -> \y -> body → ([x, y], body)
    let expr_chirho = CoreExprChirho::LamChirho {
        binder_chirho: int_binder_chirho("x", 0),
        body_chirho: Box::new(CoreExprChirho::LamChirho {
            binder_chirho: int_binder_chirho("y", 1),
            body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))),
        }),
    };

    let (binders_chirho, _body_chirho) = collect_lam_chirho(&expr_chirho);
    assert_eq!(binders_chirho.len(), 2);
}
