// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: Types, signatures and contexts.
//! Moved from `lower_chirho.rs`'s tests in row 641.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};

use super::test_support_chirho::parse_and_lower_chirho;

#[test]
fn lower_type_sig_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\nfoo :: Int -> String\n");
    assert!(module_chirho.decls_chirho.iter().any(|d_chirho| {
        matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "foo")
    }));
}

#[test]
fn lower_qualified_type_sig_chirho() {
    // Type signature with Eq a => context should produce QualChirho
    let module_chirho = parse_and_lower_chirho("module M where\nfoo :: Eq a => a -> a -> Bool\n");
    let ty_sig_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "foo")
    });
    assert!(ty_sig_chirho.is_some(), "should have a TypeSig for foo");
    if let Some(DeclChirho::TypeSigChirho { ty_chirho, .. }) = ty_sig_chirho {
        assert!(
            matches!(ty_chirho, TypeChirho::QualChirho { context_chirho, .. } if !context_chirho.is_empty()),
            "type should be QualChirho with non-empty context, got: {:?}",
            ty_chirho
        );
    }
}

#[test]
fn lower_qualified_type_sig_tuple_context_chirho() {
    // (Eq a, Show a) => should produce QualChirho with 2 constraints
    let module_chirho =
        parse_and_lower_chirho("module M where\nfoo :: (Eq a, Show a) => a -> String\n");
    let ty_sig_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "foo")
    });
    assert!(ty_sig_chirho.is_some(), "should have a TypeSig for foo");
    if let Some(DeclChirho::TypeSigChirho { ty_chirho, .. }) = ty_sig_chirho {
        match ty_chirho {
            TypeChirho::QualChirho { context_chirho, .. } => {
                assert_eq!(
                    context_chirho.len(),
                    2,
                    "should have 2 constraints (Eq and Show), got: {:?}",
                    context_chirho
                );
            }
            other_chirho => panic!("expected QualChirho, got: {:?}", other_chirho),
        }
    }
}

#[test]
fn lower_stacked_qualified_type_contexts_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nfoo :: HasCallStack => Show a => a -> String\nfoo xChirho = show xChirho\n",
    );
    let ty_sig_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "foo")
    });
    assert!(ty_sig_chirho.is_some(), "should have a TypeSig for foo");
    if let Some(DeclChirho::TypeSigChirho { ty_chirho, .. }) = ty_sig_chirho {
        match ty_chirho {
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                assert_eq!(
                    context_chirho.len(),
                    2,
                    "stacked contexts should merge into one qualified type: {:?}",
                    context_chirho
                );
                assert!(
                    matches!(body_chirho.as_ref(), TypeChirho::FunChirho { .. }),
                    "qualified body should remain the function type, got {:?}",
                    body_chirho
                );
            }
            other_chirho => panic!("expected QualChirho, got: {:?}", other_chirho),
        }
    }
}

#[test]
fn lower_infix_type_operator_keeps_left_application_spine_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE TypeOperators #-}\nmodule M where\ndata a :- b = Sub\ninstF :: forall p f a . ForallF p f :- p (f a)\n",
    );
    let ty_sig_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
        matches!(decl_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "instF")
    });
    assert!(ty_sig_chirho.is_some(), "should have a TypeSig for instF");
    if let Some(DeclChirho::TypeSigChirho { ty_chirho, .. }) = ty_sig_chirho {
        let body_chirho = match ty_chirho {
            TypeChirho::ForallChirho { body_chirho, .. } => body_chirho.as_ref(),
            other_chirho => panic!("expected forall type, got {other_chirho:?}"),
        };
        let lhs_chirho = match body_chirho {
            TypeChirho::AppChirho { fun_chirho, .. } => match fun_chirho.as_ref() {
                TypeChirho::AppChirho { arg_chirho, .. } => arg_chirho.as_ref(),
                other_chirho => panic!("expected operator application, got {other_chirho:?}"),
            },
            other_chirho => panic!("expected infix type application, got {other_chirho:?}"),
        };
        match lhs_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                assert!(
                    matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "f"),
                    "left operand should keep final f argument, got {lhs_chirho:?}"
                );
                match fun_chirho.as_ref() {
                    TypeChirho::AppChirho {
                        fun_chirho,
                        arg_chirho,
                        ..
                    } => {
                        assert!(
                            matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "p"),
                            "left operand should keep p argument, got {lhs_chirho:?}"
                        );
                        assert!(
                            matches!(fun_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "ForallF"),
                            "left operand should keep ForallF head, got {lhs_chirho:?}"
                        );
                    }
                    other_chirho => {
                        panic!("expected ForallF p application, got {other_chirho:?}")
                    }
                }
            }
            other_chirho => panic!("expected ForallF p f application, got {other_chirho:?}"),
        }
    }
}

#[test]
fn lower_grouped_type_sig_decl_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nwithBarChirho, withEmptyChirho :: [String] -> [String]\nwithBarChirho xsChirho = xsChirho\nwithEmptyChirho xsChirho = xsChirho\n",
    );
    let type_sig_names_chirho: Vec<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeSigChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        type_sig_names_chirho,
        vec!["withBarChirho".to_string(), "withEmptyChirho".to_string()]
    );
}

#[test]
fn lower_type_alias_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\ntype Name = String\n");
    assert!(module_chirho.decls_chirho.iter().any(|d_chirho| {
        matches!(d_chirho, DeclChirho::TypeAliasDeclChirho { name_chirho, .. } if name_chirho.text_chirho() == "Name")
    }));
}

#[test]
fn lower_type_alias_rhs_stops_before_next_signature_chirho() {
    fn constraint_contains_name_chirho(
        constraint_chirho: &ConstraintChirho,
        wanted_chirho: &str,
    ) -> bool {
        match constraint_chirho {
            ConstraintChirho::ClassChirho {
                class_chirho,
                args_chirho,
                ..
            } => {
                class_chirho.text_chirho() == wanted_chirho
                    || args_chirho
                        .iter()
                        .any(|arg_chirho| type_contains_name_chirho(arg_chirho, wanted_chirho))
            }
            ConstraintChirho::QuantifiedChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                context_chirho.iter().any(|constraint_chirho| {
                    constraint_contains_name_chirho(constraint_chirho, wanted_chirho)
                }) || constraint_contains_name_chirho(body_chirho, wanted_chirho)
            }
        }
    }

    fn type_contains_name_chirho(ty_chirho: &TypeChirho, wanted_chirho: &str) -> bool {
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho)
            | TypeChirho::ConChirho(name_chirho)
            | TypeChirho::PromotedConChirho { name_chirho, .. } => {
                name_chirho.text_chirho() == wanted_chirho
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                type_contains_name_chirho(fun_chirho, wanted_chirho)
                    || type_contains_name_chirho(arg_chirho, wanted_chirho)
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                type_contains_name_chirho(arg_chirho, wanted_chirho)
                    || type_contains_name_chirho(result_chirho, wanted_chirho)
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => elements_chirho
                .iter()
                .any(|element_chirho| type_contains_name_chirho(element_chirho, wanted_chirho)),
            TypeChirho::ListChirho { element_chirho, .. }
            | TypeChirho::ParenChirho {
                inner_chirho: element_chirho,
                ..
            } => type_contains_name_chirho(element_chirho, wanted_chirho),
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                context_chirho.iter().any(|constraint_chirho| {
                    constraint_contains_name_chirho(constraint_chirho, wanted_chirho)
                }) || type_contains_name_chirho(body_chirho, wanted_chirho)
            }
            TypeChirho::ForallChirho { body_chirho, .. }
            | TypeChirho::RequiredForallChirho { body_chirho, .. } => {
                type_contains_name_chirho(body_chirho, wanted_chirho)
            }
            TypeChirho::WildcardChirho { .. } | TypeChirho::LitChirho { .. } => false,
        }
    }

    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE ConstraintKinds #-}\n{-# LANGUAGE PolyKinds #-}\n{-# LANGUAGE TypeOperators #-}\nmodule M where\nclass Forall p\nclass ForallT p t\nclass p (t a b) => R p t a b\nclass Forall (R p t a) => Q p t a\ninstT :: forall k1 k2 k3 k4 (p :: k4 -> Constraint) (t :: (k1 -> k2) -> k3 -> k4) (f :: k1 -> k2) (a :: k3). ForallT p t :- p (t f a)\ninstT = Sub $\n  case inst :: Forall (Q p t) :- Q p t f of { Sub Dict ->\n  case inst :: Forall (R p t f) :- R p t f a of\n    Sub Dict -> Dict }\ntype Forall1 p = Forall p\ninst1 :: forall (p :: (* -> *) -> Constraint) (f :: * -> *). Forall p :- p f\ninst1 = inst\n",
    );
    let alias_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "Forall1"
            )
        })
        .expect("expected Forall1 type alias");
    match alias_chirho {
        DeclChirho::TypeAliasDeclChirho {
            rhs_chirho,
            type_vars_chirho,
            ..
        } => {
            assert_eq!(type_vars_chirho.len(), 1);
            assert!(
                matches!(rhs_chirho, TypeChirho::AppChirho { fun_chirho, arg_chirho, .. }
                    if matches!(fun_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Forall")
                        && matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "p")),
                "Forall1 RHS should stop at `Forall p`, got {:?}",
                rhs_chirho
            );
        }
        other_chirho => panic!("expected type alias declaration, got {:?}", other_chirho),
    }
    let inst1_sig_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::TypeSigChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "inst1"
            )
        })
        .expect("expected inst1 type signature");
    if let DeclChirho::TypeSigChirho { ty_chirho, .. } = inst1_sig_chirho {
        assert!(
            !type_contains_name_chirho(ty_chirho, "inst1"),
            "inst1 signature type should not swallow following binding, got {:?}",
            ty_chirho
        );
    }
    assert!(
        module_chirho
            .decls_chirho
            .iter()
            .any(|decl_chirho| matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "inst1"
            )),
        "expected inst1 function binding after its type signature"
    );
}

#[test]
fn lower_type_operator_chain_stays_inside_dict_argument_chirho() {
    fn strip_forall_parens_chirho(ty_chirho: &TypeChirho) -> &TypeChirho {
        match ty_chirho {
            TypeChirho::ForallChirho { body_chirho, .. }
            | TypeChirho::ParenChirho {
                inner_chirho: body_chirho,
                ..
            } => strip_forall_parens_chirho(body_chirho),
            other_chirho => other_chirho,
        }
    }

    fn collect_app_spine_chirho<'a>(
        ty_chirho: &'a TypeChirho,
        args_chirho: &mut Vec<&'a TypeChirho>,
    ) -> &'a TypeChirho {
        match ty_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                args_chirho.push(arg_chirho);
                collect_app_spine_chirho(fun_chirho, args_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                collect_app_spine_chirho(inner_chirho, args_chirho)
            }
            other_chirho => other_chirho,
        }
    }

    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE DataKinds #-}\n{-# LANGUAGE TypeOperators #-}\nmodule M where\ndata Dict c = Dict\nplusDistributesOverTimes :: forall n m o. Dict ((n * (m + o)) ~ (n * m + n * o))\nplusDistributesOverTimes = unsafeAxiom\n",
    );
    let sig_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::TypeSigChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "plusDistributesOverTimes"
            )
        })
        .expect("expected plusDistributesOverTimes type signature");
    let DeclChirho::TypeSigChirho { ty_chirho, .. } = sig_chirho else {
        panic!("expected type signature");
    };
    let body_chirho = strip_forall_parens_chirho(ty_chirho);
    let mut args_chirho = Vec::new();
    let head_chirho = collect_app_spine_chirho(body_chirho, &mut args_chirho);
    args_chirho.reverse();
    assert!(
        matches!(head_chirho, TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Dict"),
        "type-operator chain should stay under Dict, got {:?}",
        body_chirho
    );
    assert_eq!(
        args_chirho.len(),
        1,
        "Dict proof should have exactly one constraint argument, got {:?}",
        body_chirho
    );
}

#[test]
fn lower_foreign_type_is_function_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nforeign import ccall sqrtChirho :: Double -> Double\n",
    );
    match &module_chirho.decls_chirho[0] {
        DeclChirho::ForeignDeclChirho { ty_chirho, .. } => {
            assert!(
                matches!(ty_chirho, TypeChirho::FunChirho { .. }),
                "expected function type, got {:?}",
                ty_chirho
            );
        }
        other_chirho => panic!("expected ForeignDeclChirho, got {:?}", other_chirho),
    }
}

#[test]
fn lower_foreign_type_preserves_type_application_between_arrows_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nforeign import ccall clockGetTimeChirho :: ClockId -> Ptr TimeSpec -> IO CInt\n",
    );
    match &module_chirho.decls_chirho[0] {
        DeclChirho::ForeignDeclChirho { ty_chirho, .. } => match ty_chirho {
            TypeChirho::FunChirho { result_chirho, .. } => match result_chirho.as_ref() {
                TypeChirho::FunChirho { arg_chirho, .. } => {
                    assert!(
                        matches!(
                            arg_chirho.as_ref(),
                            TypeChirho::AppChirho { fun_chirho, arg_chirho, .. }
                                if matches!(fun_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Ptr")
                                    && matches!(arg_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "TimeSpec")
                        ),
                        "expected second foreign argument to stay as Ptr TimeSpec, got {:?}",
                        arg_chirho
                    );
                }
                other_chirho => panic!(
                    "expected foreign type result to remain a function, got {:?}",
                    other_chirho
                ),
            },
            other_chirho => panic!(
                "expected foreign type to be a function, got {:?}",
                other_chirho
            ),
        },
        other_chirho => panic!("expected ForeignDeclChirho, got {:?}", other_chirho),
    }
}

#[test]
fn lower_foreign_type_preserves_unit_return_application_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nforeign import ccall putStrChirho :: String -> IO ()\n",
    );
    match &module_chirho.decls_chirho[0] {
        DeclChirho::ForeignDeclChirho { ty_chirho, .. } => match ty_chirho {
            TypeChirho::FunChirho { result_chirho, .. } => {
                assert!(
                    matches!(
                        result_chirho.as_ref(),
                        TypeChirho::AppChirho { fun_chirho, arg_chirho, .. }
                            if matches!(fun_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "IO")
                                && matches!(arg_chirho.as_ref(), TypeChirho::TupleChirho { elements_chirho, .. } if elements_chirho.is_empty())
                    ),
                    "expected foreign return type to stay as IO (), got {:?}",
                    result_chirho
                );
            }
            other_chirho => panic!(
                "expected foreign type to be a function, got {:?}",
                other_chirho
            ),
        },
        other_chirho => panic!("expected ForeignDeclChirho, got {:?}", other_chirho),
    }
}

#[test]
fn lower_parenthesized_partial_tuple_type_application_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\ntype OverChirho fChirho sChirho tChirho aChirho bChirho = (aChirho -> fChirho bChirho) -> sChirho -> fChirho tChirho\nfooChirho :: OverChirho ((,) rChirho) sChirho tChirho aChirho bChirho -> Int\nfooChirho = undefined\n",
    );
    let sig_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } if name_chirho.text_chirho() == "fooChirho" => Some(ty_chirho),
            _ => None,
        })
        .expect("expected fooChirho type signature");
    match sig_chirho {
        TypeChirho::FunChirho { arg_chirho, .. } => {
            match arg_chirho.as_ref() {
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    match fun_chirho.as_ref() {
                        TypeChirho::AppChirho {
                            fun_chirho,
                            arg_chirho,
                            ..
                        } => {
                            match fun_chirho.as_ref() {
                                TypeChirho::AppChirho {
                                    fun_chirho,
                                    arg_chirho,
                                    ..
                                } => {
                                    match fun_chirho.as_ref() {
                                        TypeChirho::AppChirho {
                                            fun_chirho,
                                            arg_chirho,
                                            ..
                                        } => {
                                            match fun_chirho.as_ref() {
                                                TypeChirho::AppChirho {
                                                    fun_chirho,
                                                    arg_chirho,
                                                    ..
                                                } => {
                                                    match fun_chirho.as_ref() {
                                                        TypeChirho::ConChirho(name_chirho) => {
                                                            assert_eq!(
                                                                name_chirho.text_chirho(),
                                                                "OverChirho"
                                                            );
                                                        }
                                                        other_chirho => panic!(
                                                            "expected OverChirho head, got {other_chirho:?}"
                                                        ),
                                                    }
                                                    match arg_chirho.as_ref() {
                                                        TypeChirho::ParenChirho {
                                                            inner_chirho,
                                                            ..
                                                        } => {
                                                            match inner_chirho.as_ref() {
                                                                TypeChirho::AppChirho {
                                                                    fun_chirho,
                                                                    arg_chirho,
                                                                    ..
                                                                } => {
                                                                    match fun_chirho.as_ref() {
                                                                TypeChirho::ConChirho(name_chirho) => {
                                                                    assert_eq!(name_chirho.text_chirho(), "(,)");
                                                                }
                                                                TypeChirho::ParenChirho { inner_chirho, .. } => match inner_chirho.as_ref() {
                                                                    TypeChirho::ConChirho(name_chirho) => {
                                                                        assert_eq!(name_chirho.text_chirho(), "(,)");
                                                                    }
                                                                    other_chirho => panic!(
                                                                        "expected tuple constructor head, got {other_chirho:?}"
                                                                    ),
                                                                },
                                                                other_chirho => panic!(
                                                                    "expected tuple constructor head, got {other_chirho:?}"
                                                                ),
                                                            }
                                                                    match arg_chirho.as_ref() {
                                                                        TypeChirho::VarChirho(
                                                                            name_chirho,
                                                                        ) => {
                                                                            assert_eq!(
                                                                                name_chirho
                                                                                    .text_chirho(),
                                                                                "rChirho"
                                                                            );
                                                                        }
                                                                        other_chirho => panic!(
                                                                            "expected tuple constructor argument, got {other_chirho:?}"
                                                                        ),
                                                                    }
                                                                }
                                                                other_chirho => panic!(
                                                                    "expected partial tuple application, got {other_chirho:?}"
                                                                ),
                                                            }
                                                        }
                                                        other_chirho => panic!(
                                                            "expected parenthesized partial tuple application, got {other_chirho:?}"
                                                        ),
                                                    }
                                                }
                                                other_chirho => panic!(
                                                    "expected OverChirho application spine, got {other_chirho:?}"
                                                ),
                                            }
                                            assert!(matches!(
                                                arg_chirho.as_ref(),
                                                TypeChirho::VarChirho(_)
                                            ));
                                        }
                                        other_chirho => panic!(
                                            "expected OverChirho application spine, got {other_chirho:?}"
                                        ),
                                    }
                                    assert!(matches!(
                                        arg_chirho.as_ref(),
                                        TypeChirho::VarChirho(_)
                                    ));
                                }
                                other_chirho => panic!(
                                    "expected OverChirho application spine, got {other_chirho:?}"
                                ),
                            }
                            assert!(matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(_)));
                        }
                        other_chirho => {
                            panic!("expected OverChirho application spine, got {other_chirho:?}")
                        }
                    }
                    assert!(matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(_)));
                }
                other_chirho => {
                    panic!("expected OverChirho application argument, got {other_chirho:?}")
                }
            }
        }
        other_chirho => panic!("expected function type signature, got {other_chirho:?}"),
    }
}

#[test]
fn lower_t17594f_preserves_rank_n_function_signature_chirho() {
    let source_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../ghc-tests-chirho/typecheck-chirho/should_compile/T17594f.hs"
    ));
    let module_chirho = parse_and_lower_chirho(source_chirho);
    let signature_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } if name_chirho.text_chirho() == "doubleOrInt" => Some(ty_chirho),
            _ => None,
        })
        .unwrap_or_else(|| {
            panic!(
                "expected doubleOrInt type signature, got {:?}",
                module_chirho.decls_chirho
            )
        });

    assert!(
        matches!(
            signature_chirho,
            TypeChirho::FunChirho { result_chirho, .. }
                if matches!(result_chirho.as_ref(), TypeChirho::ForallChirho { .. })
        ),
        "expected Bool -> forall r. ... signature, got {signature_chirho:?}"
    );

    let wrap_signature_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } if name_chirho.text_chirho() == "wrap" => Some(ty_chirho),
            _ => None,
        })
        .expect("expected wrap type signature");
    assert!(
        matches!(
            wrap_signature_chirho,
            TypeChirho::RequiredForallChirho { vars_chirho, .. }
                if vars_chirho.len() == 1
                    && vars_chirho[0].name_chirho.text_chirho() == "c"
        ),
        "expected required `forall c ->`, got {wrap_signature_chirho:?}"
    );

    fn contains_type_lambda_chirho(expr_chirho: &ExprChirho) -> bool {
        match expr_chirho {
            ExprChirho::TypeLamChirho { .. } => true,
            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => contains_type_lambda_chirho(fun_chirho) || contains_type_lambda_chirho(arg_chirho),
            ExprChirho::TypeAppChirho { expr_chirho, .. }
            | ExprChirho::ParenChirho {
                inner_chirho: expr_chirho,
                ..
            } => contains_type_lambda_chirho(expr_chirho),
            _ => false,
        }
    }

    let preserves_type_lambda_chirho =
        module_chirho
            .decls_chirho
            .iter()
            .any(|decl_chirho| match decl_chirho {
                DeclChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    ..
                } if name_chirho.text_chirho() == "doubleOrInt" => {
                    matches_chirho.iter().any(|match_chirho| {
                        matches!(
                            &match_chirho.rhs_chirho,
                            RhsChirho::UnguardedChirho(expr_chirho)
                                if contains_type_lambda_chirho(expr_chirho)
                        )
                    })
                }
                _ => false,
            });
    assert!(
        preserves_type_lambda_chirho,
        "expected `\\ @a -> f` to lower as a type lambda"
    );
}

#[test]
fn lower_qualified_symbol_type_operator_keeps_operator_head_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE DataKinds #-}\n{-# LANGUAGE TypeOperators #-}\nmodule M where\nimport GHC.TypeLits as L\nimport Data.Proxy\nf :: Proxy (2 L.* 3) -> Proxy 6\nf = id\n",
    );
    let sig_ty_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } if name_chirho.text_chirho() == "f" => Some(ty_chirho),
            _ => None,
        })
        .expect("expected f type signature");

    fn collect_type_spine_chirho<'a>(
        ty_chirho: &'a TypeChirho,
        args_chirho: &mut Vec<&'a TypeChirho>,
    ) -> &'a TypeChirho {
        match ty_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                args_chirho.push(arg_chirho);
                collect_type_spine_chirho(fun_chirho, args_chirho)
            }
            _ => ty_chirho,
        }
    }

    fn contains_l_star_head_chirho(ty_chirho: &TypeChirho) -> bool {
        let mut args_chirho = Vec::new();
        let head_chirho = collect_type_spine_chirho(ty_chirho, &mut args_chirho);
        if matches!(
            head_chirho,
            TypeChirho::ConChirho(name_chirho)
                if name_chirho.full_name_chirho() == "L.*" && args_chirho.len() == 2
        ) {
            return true;
        }
        match ty_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => contains_l_star_head_chirho(fun_chirho) || contains_l_star_head_chirho(arg_chirho),
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                contains_l_star_head_chirho(arg_chirho)
                    || contains_l_star_head_chirho(result_chirho)
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => elements_chirho.iter().any(contains_l_star_head_chirho),
            TypeChirho::ListChirho { element_chirho, .. }
            | TypeChirho::ParenChirho {
                inner_chirho: element_chirho,
                ..
            } => contains_l_star_head_chirho(element_chirho),
            TypeChirho::QualChirho { body_chirho, .. }
            | TypeChirho::ForallChirho { body_chirho, .. }
            | TypeChirho::RequiredForallChirho { body_chirho, .. } => {
                contains_l_star_head_chirho(body_chirho)
            }
            TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => elements_chirho.iter().any(contains_l_star_head_chirho),
            TypeChirho::ConChirho(_)
            | TypeChirho::VarChirho(_)
            | TypeChirho::PromotedConChirho { .. }
            | TypeChirho::WildcardChirho { .. }
            | TypeChirho::LitChirho { .. } => false,
        }
    }

    assert!(
        contains_l_star_head_chirho(sig_ty_chirho),
        "qualified type operator `L.*` should be the infix type head, got {:?}",
        sig_ty_chirho
    );
}

#[test]
fn lower_th_typed_splice_chirho() {
    // $$name lowers to ExprChirho::TypedSpliceChirho
    let module_chirho = parse_and_lower_chirho("module M where\nx = $$bar\n");
    let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "x")
    });
    assert!(decl_chirho.is_some(), "should have FunBind for x");
    if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
        let rhs_chirho = &matches_chirho[0].rhs_chirho;
        if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
            assert!(
                matches!(expr_chirho, ExprChirho::TypedSpliceChirho { .. }),
                "RHS should be TypedSpliceChirho for $$bar, got {:?}",
                expr_chirho
            );
        } else {
            panic!("expected unguarded RHS");
        }
    }
}

#[test]
fn lower_th_quote_type_chirho() {
    // [t| type |] lowers to ExprChirho::QuoteTypeChirho
    let module_chirho = parse_and_lower_chirho("module M where\nx = [t| Int |]\n");
    let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "x")
    });
    assert!(decl_chirho.is_some(), "should have FunBind for x");
    if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
        let rhs_chirho = &matches_chirho[0].rhs_chirho;
        if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
            assert!(
                matches!(expr_chirho, ExprChirho::QuoteTypeChirho { .. }),
                "RHS should be QuoteTypeChirho for [t| Int |], got {:?}",
                expr_chirho
            );
        } else {
            panic!("expected unguarded RHS");
        }
    }
}
