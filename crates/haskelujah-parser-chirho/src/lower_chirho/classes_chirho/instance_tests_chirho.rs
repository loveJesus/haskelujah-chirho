// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: Instances and their heads, methods and associated types.
//! Moved from `lower_chirho.rs`'s tests in row 641.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, LocalBindChirho, RhsChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};

use super::test_support_chirho::{
    instance_head_shape_chirho, parse_and_lower_chirho, type_shape_chirho,
};
use crate::test_fixtures_chirho::CONTRAVARIANT_REP_SOURCE_CHIRHO;

#[test]
fn lower_infix_instance_method_with_as_pattern_lhs_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\ndata RGBAChirho aChirho = RGBAChirho aChirho aChirho\nclass FooChirho fChirho where\n  barChirho :: fChirho aChirho -> fChirho aChirho -> fChirho aChirho\ninstance FooChirho RGBAChirho where\n  xChirho@(RGBAChirho _ _) `barChirho` yChirho = xChirho\n",
    );
    let inst_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("expected instance declaration");
    match inst_decl_chirho {
        DeclChirho::InstanceDeclChirho { methods_chirho, .. } => {
            assert_eq!(methods_chirho.len(), 1);
            match &methods_chirho[0] {
                LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    ..
                } => {
                    assert_eq!(name_chirho.text_chirho(), "barChirho");
                    assert_eq!(matches_chirho.len(), 1);
                    assert_eq!(matches_chirho[0].pats_chirho.len(), 2);
                    match &matches_chirho[0].pats_chirho[0] {
                        PatChirho::AsChirho {
                            name_chirho,
                            pattern_chirho,
                            ..
                        } => {
                            assert_eq!(name_chirho.text_chirho(), "xChirho");
                            assert!(
                                matches!(
                                    pattern_chirho.as_ref(),
                                    PatChirho::ParenChirho { inner_chirho, .. }
                                        if matches!(
                                            inner_chirho.as_ref(),
                                            PatChirho::ConChirho { con_chirho, args_chirho, .. }
                                                if con_chirho.text_chirho() == "RGBAChirho"
                                                    && args_chirho.len() == 2
                                        )
                                ),
                                "expected as-pattern over parenthesized constructor pattern, got {:?}",
                                pattern_chirho
                            );
                        }
                        other_chirho => panic!(
                            "expected lhs to lower as an as-pattern, got {:?}",
                            other_chirho
                        ),
                    }
                }
                other_chirho => {
                    panic!("expected instance method funbind, got {:?}", other_chirho)
                }
            }
        }
        other_chirho => panic!("expected instance declaration, got {:?}", other_chirho),
    }
}

#[test]
fn lower_symbolic_minus_instance_method_with_as_pattern_lhs_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\ndata GFChirho = GFChirho Int\nclass MinusLikeChirho aChirho where\n  (-) :: aChirho -> aChirho -> aChirho\ninstance MinusLikeChirho GFChirho where\n  xChirho@(GFChirho aChirho) - GFChirho bChirho = xChirho\n",
    );
    let inst_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("expected instance declaration");
    match inst_decl_chirho {
        DeclChirho::InstanceDeclChirho { methods_chirho, .. } => {
            assert_eq!(methods_chirho.len(), 1);
            match &methods_chirho[0] {
                LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    ..
                } => {
                    assert_eq!(name_chirho.text_chirho(), "-");
                    assert_eq!(matches_chirho.len(), 1);
                    assert_eq!(matches_chirho[0].pats_chirho.len(), 2);
                    match &matches_chirho[0].pats_chirho[0] {
                        PatChirho::AsChirho {
                            name_chirho,
                            pattern_chirho,
                            ..
                        } => {
                            assert_eq!(name_chirho.text_chirho(), "xChirho");
                            assert!(
                                matches!(
                                    pattern_chirho.as_ref(),
                                    PatChirho::ParenChirho { inner_chirho, .. }
                                        if matches!(
                                            inner_chirho.as_ref(),
                                            PatChirho::ConChirho { con_chirho, args_chirho, .. }
                                                if con_chirho.text_chirho() == "GFChirho"
                                                    && args_chirho.len() == 1
                                        )
                                ),
                                "expected as-pattern over parenthesized constructor pattern, got {:?}",
                                pattern_chirho
                            );
                        }
                        other_chirho => panic!(
                            "expected lhs to lower as an as-pattern, got {:?}",
                            other_chirho
                        ),
                    }
                }
                other_chirho => {
                    panic!("expected instance method funbind, got {:?}", other_chirho)
                }
            }
        }
        other_chirho => panic!("expected instance declaration, got {:?}", other_chirho),
    }
}

#[test]
fn lower_instance_method_with_as_and_lazy_parameter_pattern_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module Main where\nclass ExtendChirho wChirho where\n  extendedChirho :: (wChirho aChirho -> bChirho) -> wChirho aChirho -> wChirho bChirho\ndata NonEmptyChirho aChirho = aChirho :| [aChirho]\ninstance ExtendChirho NonEmptyChirho where\n  extendedChirho fChirho wChirho@(~(_ :| aasChirho)) = fChirho wChirho :| aasChirho\n",
    );
    let instance_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("expected instance declaration");
    let DeclChirho::InstanceDeclChirho { methods_chirho, .. } = instance_decl_chirho else {
        panic!("expected instance declaration");
    };
    assert_eq!(methods_chirho.len(), 1);
    let LocalBindChirho::FunBindChirho {
        name_chirho,
        matches_chirho,
        ..
    } = &methods_chirho[0]
    else {
        panic!("expected instance method fun bind");
    };
    assert_eq!(name_chirho.text_chirho(), "extendedChirho");
    assert_eq!(matches_chirho.len(), 1);
    assert_eq!(matches_chirho[0].pats_chirho.len(), 2);
    assert!(matches!(
        &matches_chirho[0].pats_chirho[0],
        PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "fChirho"
    ));
    assert!(
        matches!(
            &matches_chirho[0].pats_chirho[1],
            PatChirho::AsChirho { .. }
        ),
        "expected second instance method argument to stay as an as-pattern, got {:?}",
        matches_chirho[0].pats_chirho[1]
    );
}

#[test]
fn lower_instance_method_with_as_and_record_constructor_pattern_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module Main where\ndata Par1Chirho aChirho = Par1Chirho {}\nclass ExtendChirho wChirho where\n  extendedChirho :: (wChirho aChirho -> bChirho) -> wChirho aChirho -> wChirho bChirho\ninstance ExtendChirho Par1Chirho where\n  extendedChirho fChirho wChirho@Par1Chirho{} = Par1Chirho {}\n",
    );
    let instance_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("expected instance declaration");
    let DeclChirho::InstanceDeclChirho { methods_chirho, .. } = instance_decl_chirho else {
        panic!("expected instance declaration");
    };
    let LocalBindChirho::FunBindChirho { matches_chirho, .. } = &methods_chirho[0] else {
        panic!("expected instance method fun bind");
    };
    assert_eq!(matches_chirho[0].pats_chirho.len(), 2);
    match &matches_chirho[0].pats_chirho[1] {
        PatChirho::AsChirho { pattern_chirho, .. } => {
            assert!(
                matches!(pattern_chirho.as_ref(), PatChirho::RecordChirho { .. }),
                "expected record constructor pattern inside as-pattern, got {:?}",
                pattern_chirho
            );
        }
        other_chirho => panic!("expected as-pattern parameter, got {:?}", other_chirho),
    }
}

// -----------------------------------------------------------------------
// Instance declarations
// -----------------------------------------------------------------------

#[test]
fn lower_instance_simple_chirho() {
    let module_chirho =
        parse_and_lower_chirho("module M where\ninstance Eq Bool where\n  eq x y = x\n");
    let inst_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| matches!(d_chirho, DeclChirho::InstanceDeclChirho { .. }));
    assert!(inst_chirho.is_some(), "expected an InstanceDeclChirho");

    match inst_chirho.unwrap() {
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            context_chirho,
            types_chirho,
            methods_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "Eq");
            assert!(context_chirho.is_empty());
            // "Bool" is the instance type
            assert!(!types_chirho.is_empty());
            // At least one method
            assert!(
                !methods_chirho.is_empty(),
                "expected at least one method binding"
            );
        }
        _ => unreachable!(),
    }
}

#[test]
fn lower_instance_with_context_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\ninstance Eq a => Eq (Maybe a) where\n  eq x y = x\n",
    );
    let inst_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| matches!(d_chirho, DeclChirho::InstanceDeclChirho { .. }));
    assert!(inst_chirho.is_some(), "expected an InstanceDeclChirho");

    match inst_chirho.unwrap() {
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            context_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "Eq");
            assert_eq!(context_chirho.len(), 1);
            assert!(
                matches!(
                    &context_chirho[0],
                    ConstraintChirho::ClassChirho {
                        class_chirho,
                        args_chirho,
                        ..
                    } if class_chirho.text_chirho() == "Eq" && args_chirho.len() == 1
                ),
                "expected simple Eq constraint, got {:?}",
                context_chirho[0]
            );
        }
        _ => unreachable!(),
    }
}

#[test]
fn lower_instance_preserves_applied_head_type_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\ninstance MonadZip m => MonadZip (ExceptTChirho eChirho m) where\n  mzipWith fChirho xChirho yChirho = xChirho\n",
    );
    let inst_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("expected an InstanceDeclChirho");

    match inst_chirho {
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            context_chirho,
            types_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "MonadZip");
            assert_eq!(context_chirho.len(), 1);
            assert_eq!(types_chirho.len(), 1);
            match &types_chirho[0] {
                TypeChirho::ParenChirho { inner_chirho, .. } => match inner_chirho.as_ref() {
                    TypeChirho::AppChirho {
                        fun_chirho,
                        arg_chirho,
                        ..
                    } => {
                        assert!(matches!(fun_chirho.as_ref(), TypeChirho::AppChirho { .. }));
                        match arg_chirho.as_ref() {
                            TypeChirho::VarChirho(name_chirho) => {
                                assert_eq!(name_chirho.text_chirho(), "m");
                            }
                            other_chirho => {
                                panic!(
                                    "expected final applied arg to be `m`, got {:?}",
                                    other_chirho
                                )
                            }
                        }
                    }
                    other_chirho => panic!(
                        "expected applied type inside parens for instance head, got {:?}",
                        other_chirho
                    ),
                },
                other_chirho => panic!(
                    "expected parenthesized applied type for instance head, got {:?}",
                    other_chirho
                ),
            }
        }
        _ => unreachable!(),
    }
}

#[test]
fn lower_instance_head_keeps_kind_annotated_hetero_eq_prefix_application_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nimport Data.Kind (Type)\nclass GReadChirho fChirho where\n  greadsPrecChirho :: Int -> String -> [(fChirho aChirho, String)]\ninstance k1 ~ k2 => GReadChirho ((:~~:) (a :: k1) :: k2 -> Type) where\n  greadsPrecChirho _Chirho sChirho = []\n",
    );
    let inst_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("expected an InstanceDeclChirho");

    match inst_chirho {
        DeclChirho::InstanceDeclChirho { types_chirho, .. } => {
            assert_eq!(types_chirho.len(), 1);
            match &types_chirho[0] {
                TypeChirho::ParenChirho { inner_chirho, .. } => match inner_chirho.as_ref() {
                    TypeChirho::AppChirho {
                        fun_chirho,
                        arg_chirho,
                        ..
                    } => {
                        assert!(
                            matches!(
                                fun_chirho.as_ref(),
                                TypeChirho::ParenChirho { .. } | TypeChirho::ConChirho(_)
                            ),
                            "expected preserved :~~: prefix head, got {:?}",
                            fun_chirho
                        );
                        match arg_chirho.as_ref() {
                            TypeChirho::ParenChirho { inner_chirho, .. } => {
                                match inner_chirho.as_ref() {
                                    TypeChirho::VarChirho(name_chirho) => {
                                        assert_eq!(name_chirho.text_chirho(), "a");
                                    }
                                    other_chirho => {
                                        panic!(
                                            "expected annotated argument to lower to `a`, got {:?}",
                                            other_chirho
                                        )
                                    }
                                }
                            }
                            other_chirho => {
                                panic!(
                                    "expected preserved parenthesized argument, got {:?}",
                                    other_chirho
                                )
                            }
                        }
                    }
                    other_chirho => panic!(
                        "expected applied hetero equality head, got {:?}",
                        other_chirho
                    ),
                },
                other_chirho => panic!(
                    "expected parenthesized hetero equality instance head, got {:?}",
                    other_chirho
                ),
            }
        }
        _ => unreachable!(),
    }
}

#[test]
fn lower_zero_arity_instance_method_tuple_constructor_rhs_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nclass BiapplicativeChirho pChirho where\n  bipureChirho :: aChirho -> bChirho -> pChirho aChirho bChirho\ninstance BiapplicativeChirho (,) where\n  bipureChirho = (,)\n",
    );
    let inst_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("expected an InstanceDeclChirho");

    let methods_chirho = match inst_chirho {
        DeclChirho::InstanceDeclChirho { methods_chirho, .. } => methods_chirho,
        _ => unreachable!(),
    };
    let method_chirho = methods_chirho
        .iter()
        .find(|bind_chirho| match bind_chirho {
            LocalBindChirho::FunBindChirho { name_chirho, .. } => {
                name_chirho.text_chirho() == "bipureChirho"
            }
            _ => false,
        })
        .expect("expected bipureChirho instance method");

    let matches_chirho = match method_chirho {
        LocalBindChirho::FunBindChirho { matches_chirho, .. } => matches_chirho,
        _ => unreachable!(),
    };
    assert_eq!(matches_chirho.len(), 1);
    assert_eq!(matches_chirho[0].pats_chirho.len(), 0);
    match &matches_chirho[0].rhs_chirho {
        RhsChirho::UnguardedChirho(ExprChirho::LamChirho { pats_chirho, .. }) => {
            assert_eq!(pats_chirho.len(), 2, "(,) should lower to a 2-arg lambda");
        }
        other_chirho => {
            panic!("expected tuple constructor rhs to lower to lambda, got {other_chirho:?}")
        }
    }
}

#[test]
fn lower_instance_tuple_type_constructor_head_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nclass BiapplicativeChirho pChirho where\n  bipureChirho :: aChirho -> bChirho -> pChirho aChirho bChirho\ninstance BiapplicativeChirho (,) where\n  bipureChirho = (,)\ninstance Monoid xChirho => BiapplicativeChirho ((,,) xChirho) where\n  bipureChirho = (,,) mempty\n",
    );
    let insts_chirho: Vec<_> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::InstanceDeclChirho { types_chirho, .. } => Some(types_chirho),
            _ => None,
        })
        .collect();

    assert_eq!(insts_chirho.len(), 2);
    match &insts_chirho[0][0] {
        TypeChirho::ParenChirho { inner_chirho, .. } => match inner_chirho.as_ref() {
            TypeChirho::ConChirho(name_chirho) => {
                assert_eq!(name_chirho.text_chirho(), "(,)");
            }
            other_chirho => panic!("expected tuple type constructor, got {other_chirho:?}"),
        },
        other_chirho => {
            panic!("expected parenthesized tuple constructor, got {other_chirho:?}")
        }
    }
    match &insts_chirho[1][0] {
        TypeChirho::ParenChirho { inner_chirho, .. } => match inner_chirho.as_ref() {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                match fun_chirho.as_ref() {
                    TypeChirho::ParenChirho { inner_chirho, .. } => match inner_chirho.as_ref() {
                        TypeChirho::ConChirho(name_chirho) => {
                            assert_eq!(name_chirho.text_chirho(), "(,,)");
                        }
                        other_chirho => {
                            panic!("expected (,,) constructor, got {other_chirho:?}")
                        }
                    },
                    other_chirho => panic!(
                        "expected parenthesized tuple constructor application, got {other_chirho:?}"
                    ),
                }
                match arg_chirho.as_ref() {
                    TypeChirho::VarChirho(name_chirho) => {
                        assert_eq!(name_chirho.text_chirho(), "xChirho");
                    }
                    other_chirho => panic!("expected xChirho arg, got {other_chirho:?}"),
                }
            }
            other_chirho => {
                panic!("expected applied tuple constructor type, got {other_chirho:?}")
            }
        },
        other_chirho => {
            panic!("expected parenthesized applied tuple constructor, got {other_chirho:?}")
        }
    }
}

#[test]
fn lower_instance_list_type_constructor_head_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nclass NFData1Chirho fChirho where\n  liftRnfChirho :: (aChirho -> ()) -> fChirho aChirho -> ()\ninstance NFData1Chirho [] where\n  liftRnfChirho fChirho = foldr (\\xChirho rChirho -> fChirho xChirho `seq` rChirho) ()\n",
    );
    let inst_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("expected an InstanceDeclChirho");
    let types_chirho = match inst_chirho {
        DeclChirho::InstanceDeclChirho { types_chirho, .. } => types_chirho,
        _ => unreachable!(),
    };
    assert_eq!(types_chirho.len(), 1);
    match &types_chirho[0] {
        TypeChirho::ConChirho(name_chirho) => {
            assert_eq!(name_chirho.text_chirho(), "[]");
        }
        other_chirho => panic!("expected list type constructor head, got {other_chirho:?}"),
    }
}

#[test]
fn lower_instance_no_methods_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\ninstance Show Int\n");
    let inst_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| matches!(d_chirho, DeclChirho::InstanceDeclChirho { .. }));
    assert!(inst_chirho.is_some(), "expected an InstanceDeclChirho");

    match inst_chirho.unwrap() {
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            methods_chirho,
            ..
        } => {
            assert_eq!(class_chirho.text_chirho(), "Show");
            assert!(methods_chirho.is_empty());
        }
        _ => unreachable!(),
    }
}

#[test]
fn lower_type_family_instance_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\ntype instance F Int = Bool\n");
    assert!(
        module_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeFamilyInstanceDeclChirho { family_name_chirho, .. }
            if family_name_chirho.text_chirho() == "F")
        }),
        "should have type family instance for F"
    );
}

#[test]
fn lower_type_family_instance_with_infix_lhs_type_chirho() {
    let module_chirho =
        parse_and_lower_chirho("module M where\ntype instance Swizzle (f ::+: g) = Bool\n");
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| matches!(d_chirho, DeclChirho::TypeFamilyInstanceDeclChirho { .. }))
        .expect("should have TypeFamilyInstanceDecl");
    match decl_chirho {
        DeclChirho::TypeFamilyInstanceDeclChirho {
            family_name_chirho,
            lhs_types_chirho,
            ..
        } => {
            assert_eq!(family_name_chirho.text_chirho(), "Swizzle");
            assert_eq!(lhs_types_chirho.len(), 1);
            assert!(
                format!("{:?}", lhs_types_chirho[0]).contains("::+:"),
                "lhs should preserve infix operator type, got {:?}",
                lhs_types_chirho[0]
            );
        }
        other_chirho => panic!("expected TypeFamilyInstanceDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_instance_assoc_type_family_with_concrete_lhs_uses_instance_head_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nclass C f where\n  type Rep f\ninstance C Proxy where\n  type Rep Proxy = ()\n",
    );
    let inst_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("should lower instance decl");
    match inst_decl_chirho {
        DeclChirho::InstanceDeclChirho {
            assoc_tf_instances_chirho,
            ..
        } => {
            assert_eq!(assoc_tf_instances_chirho.len(), 1);
            assert_eq!(
                assoc_tf_instances_chirho[0]
                    .family_name_chirho
                    .text_chirho(),
                "Rep"
            );
            assert_eq!(assoc_tf_instances_chirho[0].lhs_types_chirho.len(), 1);
            match &assoc_tf_instances_chirho[0].lhs_types_chirho[0] {
                TypeChirho::ConChirho(name_chirho) => {
                    assert_eq!(name_chirho.text_chirho(), "Proxy");
                }
                other_chirho => panic!("expected concrete Proxy lhs, got {:?}", other_chirho),
            }
        }
        other_chirho => panic!("expected instance decl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_single_parameter_instance_head_keeps_parenthesized_type_application_chirho() {
    let module_chirho =
        parse_and_lower_chirho("module M where\nclass C f where\ninstance C (Product f g) where\n");
    let inst_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("should lower instance decl");
    match inst_decl_chirho {
        DeclChirho::InstanceDeclChirho { types_chirho, .. } => {
            assert_eq!(
                types_chirho.len(),
                1,
                "single-parameter class instance head should stay a single applied type"
            );
            assert!(
                matches!(
                    &types_chirho[0],
                    TypeChirho::ParenChirho { inner_chirho, .. }
                        if matches!(inner_chirho.as_ref(), TypeChirho::AppChirho { .. })
                ),
                "instance head should preserve the parenthesized type application, got {:?}",
                types_chirho[0]
            );
        }
        other_chirho => panic!("expected instance decl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_instance_assoc_family_rhs_preserves_applied_tycons_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nclass C f where\n  type Rep f\ninstance C (Product f g) where\n  type Rep (Product f g) = (Rep f, Rep g)\n",
    );
    let inst_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("should lower instance decl");
    match inst_decl_chirho {
        DeclChirho::InstanceDeclChirho {
            assoc_tf_instances_chirho,
            ..
        } => {
            assert_eq!(assoc_tf_instances_chirho.len(), 1);
            match &assoc_tf_instances_chirho[0].rhs_chirho {
                TypeChirho::TupleChirho {
                    elements_chirho, ..
                } => {
                    assert_eq!(elements_chirho.len(), 2);
                    assert!(
                        matches!(elements_chirho[0], TypeChirho::AppChirho { .. }),
                        "first rhs tuple element should preserve `Rep f` application, got {:?}",
                        elements_chirho[0]
                    );
                    assert!(
                        matches!(elements_chirho[1], TypeChirho::AppChirho { .. }),
                        "second rhs tuple element should preserve `Rep g` application, got {:?}",
                        elements_chirho[1]
                    );
                }
                other_chirho => panic!("expected tuple rhs, got {:?}", other_chirho),
            }
        }
        other_chirho => panic!("expected instance decl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_real_contravariant_representable_instances_preserve_assoc_type_heads_chirho() {
    let module_chirho = parse_and_lower_chirho(CONTRAVARIANT_REP_SOURCE_CHIRHO);
    let instance_decls_chirho: Vec<_> = module_chirho
        .decls_chirho
        .iter()
        .filter(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .collect();

    let proxy_instance_chirho = instance_decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::InstanceDeclChirho { types_chirho, .. }
                    if instance_head_shape_chirho(types_chirho) == "Proxy"
            )
        })
        .expect("should lower Proxy instance");
    let product_instance_chirho = instance_decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::InstanceDeclChirho { types_chirho, .. }
                    if instance_head_shape_chirho(types_chirho) == "((Product f) g)"
            )
        })
        .expect("should lower Product instance");

    match proxy_instance_chirho {
        DeclChirho::InstanceDeclChirho {
            assoc_tf_instances_chirho,
            ..
        } => {
            assert_eq!(assoc_tf_instances_chirho.len(), 1);
            assert_eq!(
                type_shape_chirho(&assoc_tf_instances_chirho[0].lhs_types_chirho[0]),
                "Proxy",
                "Proxy instance should keep the concrete associated type head"
            );
            assert_eq!(
                type_shape_chirho(&assoc_tf_instances_chirho[0].rhs_chirho),
                "()",
                "Proxy instance should lower `type Rep Proxy = ()` as unit"
            );
        }
        other_chirho => panic!("expected Proxy instance decl, got {:?}", other_chirho),
    }

    match product_instance_chirho {
        DeclChirho::InstanceDeclChirho {
            assoc_tf_instances_chirho,
            ..
        } => {
            assert_eq!(assoc_tf_instances_chirho.len(), 1);
            assert_eq!(
                type_shape_chirho(&assoc_tf_instances_chirho[0].lhs_types_chirho[0]),
                "((Product f) g)",
                "Product instance should keep the concrete associated type head"
            );
            assert_eq!(
                type_shape_chirho(&assoc_tf_instances_chirho[0].rhs_chirho),
                "((Rep f), (Rep g))",
                "Product instance should lower `type Rep (Product f g) = (Rep f, Rep g)`"
            );
        }
        other_chirho => panic!("expected Product instance decl, got {:?}", other_chirho),
    }
}

#[test]
fn an_instance_fixity_still_ends_the_instance_walk_chirho() {
    // GHC 9.14.1 rejects a fixity declaration in an instance body (root's oracle,
    // row 685), so only class bodies admit one; the instance walk stops there.
    let module_chirho = parse_and_lower_chirho(
        "module M where
instance Num T where
  leftChirho + rightChirho = leftChirho
  infixl 6 +
  fromInteger _ = T
",
    );
    let DeclChirho::InstanceDeclChirho { methods_chirho, .. } = &module_chirho.decls_chirho[0]
    else {
        panic!("expected an instance declaration");
    };
    let names_chirho: Vec<&str> = methods_chirho
        .iter()
        .filter_map(|method_chirho| match method_chirho {
            LocalBindChirho::FunBindChirho { name_chirho, .. } => Some(name_chirho.text_chirho()),
            _ => None,
        })
        .collect();
    assert_eq!(names_chirho, ["+"]);
}

#[test]
fn a_unit_instance_head_is_concrete_beside_a_context_instance_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module Main where\nclass SizeChirho aChirho where\n  sizeChirho :: aChirho -> Int\ninstance SizeChirho () where\n  sizeChirho _ = 0\ninstance SizeChirho aChirho => SizeChirho (Maybe aChirho) where\n  sizeChirho _ = 1\n",
    );
    let heads_chirho: Vec<_> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::InstanceDeclChirho { types_chirho, .. } => Some(types_chirho),
            _ => None,
        })
        .collect();
    assert_eq!(heads_chirho.len(), 2);
    assert_eq!(type_shape_chirho(&heads_chirho[0][0]), "()");
    assert_eq!(type_shape_chirho(&heads_chirho[1][0]), "(Maybe aChirho)");
}
