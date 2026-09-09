// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Execute captured and indirect functions; LLVM temporary names are not an ABI.
//! The standalone Core adapter prints a pure integer result and returns it as the exit code.

use super::*;

#[test]
fn compile_indirect_call_for_local_function_value_chirho() {
    let module_chirho = CoreModuleChirho {
        name_chirho: "IndirectCallTest".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("main", 10),
            rhs_chirho: CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(
                    dummy_binder_chirho("f", 1),
                    CoreExprChirho::LamChirho {
                        binder_chirho: dummy_binder_chirho("x", 0),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                    },
                )],
                body_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
                    arg_chirho: Box::new(int_lit_chirho(42)),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
    };

    assert_eq!(
        run_executable_module_result_chirho(&module_chirho),
        (42, "42\n".to_string(), String::new())
    );
}

#[test]
fn compile_captured_local_lambda_passes_env_chirho() {
    let x_binder_chirho = dummy_binder_chirho("x", 1);
    let f_binder_chirho = dummy_binder_chirho("f", 2);
    let y_binder_chirho = dummy_binder_chirho("y", 3);
    let module_chirho = CoreModuleChirho {
        name_chirho: "CapturedLetTest".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![
                    (x_binder_chirho.clone(), int_lit_chirho(41)),
                    (
                        f_binder_chirho.clone(),
                        CoreExprChirho::LamChirho {
                            binder_chirho: y_binder_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                                name_chirho: "+#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(x_binder_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(y_binder_chirho.id_chirho),
                                ],
                            }),
                        },
                    ),
                ],
                body_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_binder_chirho.id_chirho)),
                    arg_chirho: Box::new(int_lit_chirho(1)),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
    };

    assert_eq!(
        run_executable_module_result_chirho(&module_chirho),
        (42, "42\n".to_string(), String::new())
    );
}

#[test]
fn compile_case_binder_capture_materializes_closure_chirho() {
    let xs_binder_chirho = dummy_binder_chirho("xs", 1);
    let case_binder_chirho = dummy_binder_chirho("wild", 2);
    let y_binder_chirho = dummy_binder_chirho("y", 3);
    let ys_binder_chirho = dummy_binder_chirho("ys", 4);
    let f_binder_chirho = dummy_binder_chirho("f", 5);
    let z_binder_chirho = dummy_binder_chirho("z", 6);
    let mut module_chirho = CoreModuleChirho {
        name_chirho: "CaseCaptureTest".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("captureChirho", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: xs_binder_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                        xs_binder_chirho.id_chirho,
                    )),
                    bind_chirho: case_binder_chirho,
                    result_ty_chirho: TyChirho::int_chirho(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho(":".to_string()),
                            binders_chirho: vec![y_binder_chirho.clone(), ys_binder_chirho],
                            rhs_chirho: CoreExprChirho::LetChirho {
                                rec_chirho: false,
                                binds_chirho: vec![(
                                    f_binder_chirho.clone(),
                                    CoreExprChirho::LamChirho {
                                        binder_chirho: z_binder_chirho.clone(),
                                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                                            name_chirho: "+#".to_string(),
                                            args_chirho: vec![
                                                CoreExprChirho::VarChirho(
                                                    y_binder_chirho.id_chirho,
                                                ),
                                                CoreExprChirho::VarChirho(
                                                    z_binder_chirho.id_chirho,
                                                ),
                                            ],
                                        }),
                                    },
                                )],
                                body_chirho: Box::new(CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                        f_binder_chirho.id_chirho,
                                    )),
                                    arg_chirho: Box::new(int_lit_chirho(1)),
                                }),
                            },
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: int_lit_chirho(0),
                        },
                    ],
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
    };

    module_chirho.bindings_chirho.push(CoreBindingChirho {
        binder_chirho: dummy_binder_chirho("main", 100),
        rhs_chirho: CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            arg_chirho: Box::new(CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    int_lit_chirho(41),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "[]".to_string(),
                        args_chirho: vec![],
                    },
                ],
            }),
        },
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    });
    assert_eq!(
        run_executable_module_result_chirho(&module_chirho),
        (42, "42\n".to_string(), String::new())
    );
}

#[test]
fn compile_transitive_local_lambda_capture_materializes_outer_value_chirho() {
    let x_binder_chirho = dummy_binder_chirho("x", 1);
    let f_binder_chirho = dummy_binder_chirho("f", 2);
    let g_binder_chirho = dummy_binder_chirho("g", 3);
    let y_binder_chirho = dummy_binder_chirho("y", 5);
    let z_binder_chirho = dummy_binder_chirho("z", 7);
    let module_chirho = CoreModuleChirho {
        name_chirho: "TransitiveCaptureTest".to_string(),
        bindings_chirho: vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![
                    (x_binder_chirho.clone(), int_lit_chirho(41)),
                    (
                        f_binder_chirho.clone(),
                        CoreExprChirho::LamChirho {
                            binder_chirho: y_binder_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                                name_chirho: "+#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(x_binder_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(y_binder_chirho.id_chirho),
                                ],
                            }),
                        },
                    ),
                    (
                        g_binder_chirho.clone(),
                        CoreExprChirho::LamChirho {
                            binder_chirho: z_binder_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                    f_binder_chirho.id_chirho,
                                )),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                    z_binder_chirho.id_chirho,
                                )),
                            }),
                        },
                    ),
                ],
                body_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(g_binder_chirho.id_chirho)),
                    arg_chirho: Box::new(int_lit_chirho(1)),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
    };

    assert_eq!(
        run_executable_module_result_chirho(&module_chirho),
        (42, "42\n".to_string(), String::new())
    );
}
