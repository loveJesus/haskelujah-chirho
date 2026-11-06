// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: Patterns, unboxed tuples and MagicHash.
//! Moved from `lower_chirho.rs`'s tests in row 641.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use super::test_support_chirho::{parse_and_lower_chirho, type_shape_chirho};

#[test]
fn lower_equality_constraint_type_sig_chirho() {
    // a ~ Int => should produce QualChirho with equality constraint
    let module_chirho = parse_and_lower_chirho("module M where\nfoo :: a ~ Int => a -> Int\n");
    let ty_sig_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "foo")
    });
    assert!(ty_sig_chirho.is_some(), "should have a TypeSig for foo");
    if let Some(DeclChirho::TypeSigChirho { ty_chirho, .. }) = ty_sig_chirho {
        match ty_chirho {
            TypeChirho::QualChirho { context_chirho, .. } => {
                assert_eq!(context_chirho.len(), 1, "should have 1 constraint");
                // The constraint should be the ~ operator applied to two types
                let c_chirho = &context_chirho[0];
                assert!(
                    matches!(
                        c_chirho,
                        ConstraintChirho::ClassChirho {
                            class_chirho,
                            args_chirho,
                            ..
                        } if class_chirho.text_chirho() == "~" && args_chirho.len() == 2
                    ),
                    "constraint should be simple (~) with 2 args, got {:?}",
                    c_chirho
                );
            }
            other_chirho => panic!("expected QualChirho, got: {:?}", other_chirho),
        }
    }
}

#[test]
fn lower_quantified_constraint_type_sig_keeps_quantified_constraint_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE QuantifiedConstraints #-}\nmodule M where\nfoo :: (forall a. Show a) => Int\n",
    );
    let ty_sig_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "foo")
    });
    assert!(ty_sig_chirho.is_some(), "should have a TypeSig for foo");
    if let Some(DeclChirho::TypeSigChirho { ty_chirho, .. }) = ty_sig_chirho {
        match ty_chirho {
            TypeChirho::QualChirho { context_chirho, .. } => {
                assert_eq!(context_chirho.len(), 1, "should have 1 constraint");
                match &context_chirho[0] {
                    ConstraintChirho::QuantifiedChirho {
                        vars_chirho,
                        body_chirho,
                        ..
                    } => {
                        assert_eq!(vars_chirho.len(), 1);
                        assert!(matches!(
                            body_chirho.as_ref(),
                            ConstraintChirho::ClassChirho {
                                class_chirho,
                                args_chirho,
                                ..
                            } if class_chirho.text_chirho() == "Show" && args_chirho.len() == 1
                        ));
                    }
                    other_chirho => {
                        panic!("expected quantified constraint, got {:?}", other_chirho)
                    }
                }
            }
            other_chirho => panic!("expected QualChirho, got: {:?}", other_chirho),
        }
    }
}

#[test]
fn lower_list_comp_generator_as_pattern_preserves_alias_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\ndata ChunkChirho = TextChirho Int | GapChirho\nkeepTextChirho xsChirho = [cChirho | cChirho@(TextChirho _) <- xsChirho]\n",
    );
    let fun_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "keepTextChirho"
            )
        })
        .expect("expected keepTextChirho fun bind");

    let DeclChirho::FunBindChirho { matches_chirho, .. } = fun_decl_chirho else {
        panic!("expected function binding");
    };
    let RhsChirho::UnguardedChirho(rhs_expr_chirho) = &matches_chirho[0].rhs_chirho else {
        panic!("expected unguarded rhs");
    };
    let ExprChirho::ListCompChirho { quals_chirho, .. } = rhs_expr_chirho else {
        panic!("expected list comprehension rhs, got {:?}", rhs_expr_chirho);
    };
    let haskelujah_ast_chirho::expr_chirho::ComprehensionQualifierChirho::StatementChirho(
        statement_chirho,
    ) = &quals_chirho[0]
    else {
        panic!("expected ordinary qualifier")
    };
    let StmtChirho::BindChirho { pat_chirho, .. } = statement_chirho.as_ref() else {
        panic!(
            "expected list comprehension generator, got {:?}",
            quals_chirho[0]
        );
    };
    let PatChirho::AsChirho {
        name_chirho,
        pattern_chirho,
        ..
    } = pat_chirho
    else {
        panic!("expected as-pattern generator, got {:?}", pat_chirho);
    };
    assert_eq!(name_chirho.text_chirho(), "cChirho");
    assert!(
        matches!(
            pattern_chirho.as_ref(),
            PatChirho::ParenChirho { .. } | PatChirho::ConChirho { .. }
        ),
        "expected aliased constructor pattern, got {:?}",
        pattern_chirho
    );
}

#[test]
fn lower_qualified_infix_constructor_pattern_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module MChirho where\nheadChirho xsChirho = case xsChirho of\n  aChirho SeqChirho.:< bChirho -> aChirho\n",
    );
    let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
        matches!(
            decl_chirho,
            DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "headChirho"
        )
    });
    let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho else {
        panic!("expected headChirho function binding");
    };
    let RhsChirho::UnguardedChirho(ExprChirho::CaseChirho { alts_chirho, .. }) =
        &matches_chirho[0].rhs_chirho
    else {
        panic!("expected case expression rhs");
    };
    match &alts_chirho[0].pat_chirho {
        PatChirho::InfixConChirho {
            left_chirho,
            op_chirho,
            right_chirho,
            ..
        } => {
            assert_eq!(op_chirho.full_name_chirho(), "SeqChirho.:<");
            assert!(
                matches!(
                    left_chirho.as_ref(),
                    PatChirho::VarChirho(name_chirho)
                        if name_chirho.text_chirho() == "aChirho"
                ),
                "expected left qualified infix pattern binder, got {:?}",
                left_chirho
            );
            assert!(
                matches!(
                    right_chirho.as_ref(),
                    PatChirho::VarChirho(name_chirho)
                        if name_chirho.text_chirho() == "bChirho"
                ),
                "expected right qualified infix pattern binder, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!(
            "expected qualified infix constructor pattern, got {:?}",
            other_chirho
        ),
    }
}

#[test]
fn lower_parenthesized_operator_parameter_pattern_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\n\
             data DigitChirho aChirho = OneChirho aChirho | TwoChirho aChirho aChirho\n\
             foldDigitChirho :: (bChirho -> bChirho -> bChirho) -> (aChirho -> bChirho) -> DigitChirho aChirho -> bChirho\n\
             foldDigitChirho (<+>) fChirho (TwoChirho aChirho bChirho) = fChirho aChirho <+> fChirho bChirho\n",
    );

    let fun_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "foldDigitChirho"
            )
        })
        .unwrap_or_else(|| panic!("expected foldDigitChirho funbind"));

    let DeclChirho::FunBindChirho { matches_chirho, .. } = fun_decl_chirho else {
        panic!("expected foldDigitChirho funbind");
    };

    match &matches_chirho[0].pats_chirho[0] {
        PatChirho::ParenChirho { inner_chirho, .. } => {
            assert!(
                matches!(inner_chirho.as_ref(), PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "<+>"),
                "expected parenthesized operator binder, got {:?}",
                inner_chirho
            );
        }
        other_chirho => panic!(
            "expected parenthesized operator pattern, got {:?}",
            other_chirho
        ),
    }
}

#[test]
fn lower_state_constructor_composition_argument_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nnewtype StateTChirho sChirho mChirho aChirho = StateTChirho { runStateTChirho :: sChirho -> mChirho (aChirho, sChirho) }\nstateChirho fChirho = StateTChirho (return . fChirho)\n",
    );
    match &module_chirho.decls_chirho[1] {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            }) => {
                assert!(matches!(
                    fun_chirho.as_ref(),
                    ExprChirho::ConChirho(name_chirho)
                        if name_chirho.text_chirho() == "StateTChirho"
                ));
                assert!(
                    matches!(
                        arg_chirho.as_ref(),
                        ExprChirho::InfixChirho { op_chirho, .. }
                            if op_chirho.text_chirho() == "."
                    ) || matches!(
                        arg_chirho.as_ref(),
                        ExprChirho::ParenChirho { inner_chirho, .. }
                            if matches!(
                                inner_chirho.as_ref(),
                                ExprChirho::InfixChirho { op_chirho, .. }
                                    if op_chirho.text_chirho() == "."
                            )
                    ),
                    "expected StateTChirho argument to preserve composition, got {:?}",
                    arg_chirho
                );
            }
            other_chirho => {
                panic!("expected constructor application, got {:?}", other_chirho)
            }
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    }
}

#[test]
fn lower_kind_sig_constraint_chirho() {
    // data Dict (c :: Constraint) = MkDict — Constraint kind annotation
    let module_chirho =
        parse_and_lower_chirho("module M where\ndata Dict (c :: Constraint) = MkDict\n");
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            type_vars_chirho, ..
        } => {
            assert_eq!(type_vars_chirho.len(), 1);
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "c");
            assert_eq!(
                type_vars_chirho[0]
                    .kind_annotation_chirho
                    .as_deref()
                    .map(type_shape_chirho),
                Some("Constraint".to_string())
            );
        }
        other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_kind_sig_star_to_constraint_chirho() {
    // data Proxy (c :: * -> Constraint) = MkProxy — arrow kind with Constraint
    let module_chirho =
        parse_and_lower_chirho("module M where\ndata Proxy (c :: * -> Constraint) = MkProxy\n");
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            type_vars_chirho, ..
        } => {
            assert_eq!(type_vars_chirho.len(), 1);
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "c");
            assert_eq!(
                type_vars_chirho[0]
                    .kind_annotation_chirho
                    .as_deref()
                    .map(type_shape_chirho),
                Some("(* -> Constraint)".to_string())
            );
        }
        other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_promoted_list_cons_type_operator_keeps_tail_chirho() {
    fn app_head_and_args_chirho<'a>(
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
                app_head_and_args_chirho(fun_chirho, args_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                app_head_and_args_chirho(inner_chirho, args_chirho)
            }
            other_chirho => other_chirho,
        }
    }

    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE DataKinds #-}\n\
{-# LANGUAGE TypeOperators #-}\n\
module M where\n\
type Consed l ls = l ': ls\n",
    );
    let alias_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "Consed"
            )
        })
        .expect("expected Consed type alias");
    let DeclChirho::TypeAliasDeclChirho { rhs_chirho, .. } = alias_chirho else {
        panic!("expected Consed type alias");
    };

    let mut cons_args_chirho = Vec::new();
    let cons_head_chirho = app_head_and_args_chirho(rhs_chirho, &mut cons_args_chirho);
    cons_args_chirho.reverse();
    assert!(
        matches!(cons_head_chirho, TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == ":" || name_chirho.text_chirho() == "':"),
        "expected promoted cons head, got {:?}",
        rhs_chirho
    );
    assert_eq!(
        cons_args_chirho.len(),
        2,
        "promoted cons should keep both head and tail args, got {:?}",
        rhs_chirho
    );
    assert!(
        matches!(cons_args_chirho[0], TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "l"),
        "promoted cons head arg should be l, got {:?}",
        cons_args_chirho[0]
    );
    assert!(
        matches!(cons_args_chirho[1], TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "ls"),
        "promoted cons tail arg should be ls, got {:?}",
        cons_args_chirho[1]
    );
}

#[test]
fn lower_th_quote_pat_chirho() {
    // [p| pat |] lowers to ExprChirho::QuotePatChirho
    let module_chirho = parse_and_lower_chirho("module M where\nx = [p| y |]\n");
    let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "x")
    });
    assert!(decl_chirho.is_some(), "should have FunBind for x");
    if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
        let rhs_chirho = &matches_chirho[0].rhs_chirho;
        if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
            assert!(
                matches!(expr_chirho, ExprChirho::QuotePatChirho { .. }),
                "RHS should be QuotePatChirho for [p| y |], got {:?}",
                expr_chirho
            );
        } else {
            panic!("expected unguarded RHS");
        }
    }
}

#[test]
fn lower_unboxed_tuple_case_pattern_binds_names_chirho() {
    let source_chirho = "module M where\nfChirho xChirho yChirho = case timesWord2# xChirho yChirho of\n  (# hiChirho, loChirho #) -> xor# hiChirho loChirho\n";
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let parser_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "fChirho")
        })
        .expect("expected fChirho binding");

    let rhs_expr_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
            other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };

    match rhs_expr_chirho {
        ExprChirho::CaseChirho { alts_chirho, .. } => match &alts_chirho[0].pat_chirho {
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => {
                assert_eq!(elements_chirho.len(), 2);
                assert!(
                    matches!(&elements_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "hiChirho")
                );
                assert!(
                    matches!(&elements_chirho[1], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "loChirho")
                );
            }
            other_chirho => panic!(
                "expected tuple pattern from unboxed tuple syntax, got {:?}",
                other_chirho
            ),
        },
        other_chirho => panic!("expected case expression, got {:?}", other_chirho),
    }
}

#[test]
fn lower_unboxed_tuple_case_result_expr_binds_names_chirho() {
    let source_chirho = "module M where\nfChirho xChirho yChirho = case timesWord2# xChirho yChirho of\n  (# hiChirho, loChirho #) -> (# hiChirho, loChirho #)\n";
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let parser_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "fChirho")
        })
        .expect("expected fChirho binding");

    let rhs_expr_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
            other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };

    match rhs_expr_chirho {
        ExprChirho::CaseChirho { alts_chirho, .. } => match &alts_chirho[0].rhs_chirho {
            RhsChirho::UnguardedChirho(ExprChirho::TupleChirho {
                elements_chirho, ..
            }) => {
                assert_eq!(elements_chirho.len(), 2);
                assert!(
                    matches!(&elements_chirho[0], ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "hiChirho")
                );
                assert!(
                    matches!(&elements_chirho[1], ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "loChirho")
                );
            }
            other_chirho => panic!(
                "expected tuple expression from unboxed tuple syntax, got {:?}",
                other_chirho
            ),
        },
        other_chirho => panic!("expected case expression, got {:?}", other_chirho),
    }
}

#[test]
fn lower_unboxed_tuple_case_hash_primed_binders_survive_in_pattern_and_expr_chirho() {
    let source_chirho = "module M where\nfChirho xChirho = case foo# xChirho of\n  (# s'#, arr'# #) -> (# s'#, ByteArray arr'# #)\n";
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let parser_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "fChirho")
        })
        .expect("expected fChirho binding");

    let rhs_expr_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
            other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };

    match rhs_expr_chirho {
        ExprChirho::CaseChirho { alts_chirho, .. } => {
            match &alts_chirho[0].pat_chirho {
                PatChirho::TupleChirho {
                    elements_chirho, ..
                } => {
                    assert_eq!(elements_chirho.len(), 2);
                    assert!(
                        matches!(&elements_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "s'#")
                    );
                    assert!(
                        matches!(&elements_chirho[1], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr'#")
                    );
                }
                other_chirho => panic!(
                    "expected tuple pattern from unboxed tuple syntax, got {:?}",
                    other_chirho
                ),
            }

            match &alts_chirho[0].rhs_chirho {
                RhsChirho::UnguardedChirho(ExprChirho::TupleChirho {
                    elements_chirho, ..
                }) => {
                    assert_eq!(elements_chirho.len(), 2);
                    assert!(
                        matches!(&elements_chirho[0], ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "s'#")
                    );
                    assert!(matches!(
                        &elements_chirho[1],
                        ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                            if matches!(fun_chirho.as_ref(), ExprChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "ByteArray")
                                && matches!(arg_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr'#")
                    ));
                }
                other_chirho => panic!(
                    "expected tuple expression from unboxed tuple syntax, got {:?}",
                    other_chirho
                ),
            }
        }
        other_chirho => panic!("expected case expression, got {:?}", other_chirho),
    }
}

#[test]
fn lower_magic_hash_constructor_pattern_and_nested_unboxed_tuple_case_chirho() {
    let source_chirho = "module M where\nfChirho (MutableByteArray arr#) = primitive (\\s# -> case unsafeFreezeByteArray# arr# s# of\n  (# s'#, arr'# #) -> (# s'#, ByteArray arr'# #))\n";
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let parser_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "fChirho")
        })
        .expect("expected fChirho binding");

    let match_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => &matches_chirho[0],
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };

    let outer_pat_chirho = match &match_chirho.pats_chirho[0] {
        PatChirho::ParenChirho { inner_chirho, .. } => inner_chirho.as_ref(),
        other_chirho => other_chirho,
    };
    match outer_pat_chirho {
        PatChirho::ConChirho {
            con_chirho,
            args_chirho,
            ..
        } if con_chirho.text_chirho() == "MutableByteArray"
            && args_chirho.len() == 1
            && matches!(&args_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr#") =>
            {}
        other_chirho => panic!(
            "expected MutableByteArray constructor pattern with arr# binder, got {:?}",
            other_chirho
        ),
    }

    let rhs_expr_chirho = match &match_chirho.rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
        other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
    };

    match rhs_expr_chirho {
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "primitive")
            && matches!(match arg_chirho.as_ref() {
                ExprChirho::ParenChirho { inner_chirho, .. } => inner_chirho.as_ref(),
                other_chirho => other_chirho,
            }, ExprChirho::LamChirho { pats_chirho, body_chirho, .. }
                if pats_chirho.len() == 1
                    && matches!(&pats_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "s#")
                    && matches!(body_chirho.as_ref(), ExprChirho::CaseChirho { alts_chirho, .. }
                        if matches!(&alts_chirho[0].pat_chirho, PatChirho::TupleChirho { elements_chirho, .. }
                            if elements_chirho.len() == 2
                                && matches!(&elements_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "s'#")
                                && matches!(&elements_chirho[1], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr'#"))
                            && matches!(&alts_chirho[0].rhs_chirho, RhsChirho::UnguardedChirho(ExprChirho::TupleChirho { elements_chirho, .. })
                                if elements_chirho.len() == 2
                                    && matches!(&elements_chirho[0], ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "s'#")
                                    && matches!(&elements_chirho[1], ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                                        if matches!(fun_chirho.as_ref(), ExprChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "ByteArray")
                                            && matches!(arg_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr'#"))))
            ) => {}
        other_chirho => panic!(
            "expected primitive lambda with nested hash binder case, got {:?}",
            other_chirho
        ),
    }
}

#[test]
fn lower_magic_hash_constructor_pattern_and_unsafe_coerce_rhs_keeps_hash_var_chirho() {
    let source_chirho = "module M where\nfChirho (ByteArray arr#) = primitive (\\s# -> (# s#, MutableByteArray (unsafeCoerce# arr#) #))\n";
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let parser_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "fChirho")
        })
        .expect("expected fChirho binding");

    let match_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => &matches_chirho[0],
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };

    let outer_pat_chirho = match &match_chirho.pats_chirho[0] {
        PatChirho::ParenChirho { inner_chirho, .. } => inner_chirho.as_ref(),
        other_chirho => other_chirho,
    };
    match outer_pat_chirho {
        PatChirho::ConChirho {
            con_chirho,
            args_chirho,
            ..
        } if con_chirho.text_chirho() == "ByteArray"
            && args_chirho.len() == 1
            && matches!(&args_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr#") =>
            {}
        other_chirho => panic!(
            "expected ByteArray constructor pattern with arr# binder, got {:?}",
            other_chirho
        ),
    }

    let rhs_expr_chirho = match &match_chirho.rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
        other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
    };
    let (primitive_fun_chirho, primitive_arg_chirho) = match rhs_expr_chirho {
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => (fun_chirho.as_ref(), arg_chirho.as_ref()),
        other_chirho => panic!("expected primitive application, got {:?}", other_chirho),
    };
    assert!(
        matches!(primitive_fun_chirho, ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "primitive"),
        "expected primitive callee, got {:?}",
        primitive_fun_chirho
    );

    let lam_chirho = match primitive_arg_chirho {
        ExprChirho::ParenChirho { inner_chirho, .. } => inner_chirho.as_ref(),
        other_chirho => other_chirho,
    };
    let body_chirho = match lam_chirho {
        ExprChirho::LamChirho {
            pats_chirho,
            body_chirho,
            ..
        } if pats_chirho.len() == 1
            && matches!(&pats_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "s#") =>
        {
            body_chirho.as_ref()
        }
        other_chirho => panic!("expected primitive lambda over s#, got {:?}", other_chirho),
    };

    assert!(
        matches!(body_chirho, ExprChirho::TupleChirho { elements_chirho, .. }
            if elements_chirho.len() == 2
                && matches!(&elements_chirho[0], ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "s#")
                && matches!(&elements_chirho[1], ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                    if matches!(fun_chirho.as_ref(), ExprChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "MutableByteArray")
                        && matches!(arg_chirho.as_ref(), ExprChirho::ParenChirho { inner_chirho, .. }
                            if matches!(inner_chirho.as_ref(), ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                                if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "unsafeCoerce#")
                                    && matches!(arg_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr#"))))),
        "expected unsafeCoerce# arr# to keep the hash binder in the tuple rhs, got {:?}",
        body_chirho
    );
}

#[test]
fn lower_preprocessed_primitive_bytearray_unsafe_thaw_keeps_hash_var_chirho() {
    let source_chirho = crate::test_fixtures_chirho::BYTEARRAY_PREPROCESSED_SOURCE_CHIRHO;
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let parser_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "unsafeThawByteArray")
        })
        .expect("expected unsafeThawByteArray binding");

    let match_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => &matches_chirho[0],
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };
    let outer_pat_chirho = match &match_chirho.pats_chirho[0] {
        PatChirho::ParenChirho { inner_chirho, .. } => inner_chirho.as_ref(),
        other_chirho => other_chirho,
    };
    assert!(
        matches!(outer_pat_chirho, PatChirho::ConChirho { con_chirho, args_chirho, .. }
            if con_chirho.text_chirho() == "ByteArray"
                && args_chirho.len() == 1
                && matches!(&args_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr#")),
        "expected unsafeThawByteArray binder to keep arr#, got {:?}",
        outer_pat_chirho
    );

    let rhs_expr_chirho = match &match_chirho.rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
        other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
    };
    assert!(
        matches!(rhs_expr_chirho, ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
            if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "primitive")
                && matches!(arg_chirho.as_ref(), ExprChirho::ParenChirho { inner_chirho, .. }
                    if matches!(inner_chirho.as_ref(), ExprChirho::LamChirho { pats_chirho, body_chirho, .. }
                        if pats_chirho.len() == 1
                            && matches!(&pats_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "s#")
                            && matches!(body_chirho.as_ref(), ExprChirho::TupleChirho { elements_chirho, .. }
                                if elements_chirho.len() == 2
                                    && matches!(&elements_chirho[0], ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "s#")
                                    && matches!(&elements_chirho[1], ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                                        if matches!(fun_chirho.as_ref(), ExprChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "MutableByteArray")
                                            && matches!(arg_chirho.as_ref(), ExprChirho::ParenChirho { inner_chirho, .. }
                                                if matches!(inner_chirho.as_ref(), ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                                                    if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "unsafeCoerce#")
                                                        && matches!(arg_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr#")))))))),
        "expected lowered unsafeThawByteArray rhs to keep arr# inside unsafeCoerce#, got {:?}",
        rhs_expr_chirho
    );
}

#[test]
fn lower_unboxed_tuple_case_pattern_tight_spacing_binds_names_chirho() {
    let source_chirho = "module M where\nfChirho xChirho yChirho = case timesWord2# xChirho yChirho of\n  (#hiChirho, loChirho#) -> xor# hiChirho loChirho\n";
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let parser_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "fChirho")
        })
        .expect("expected fChirho binding");

    let rhs_expr_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
            other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };

    match rhs_expr_chirho {
        ExprChirho::CaseChirho { alts_chirho, .. } => match &alts_chirho[0].pat_chirho {
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => {
                assert_eq!(elements_chirho.len(), 2);
                assert!(
                    matches!(&elements_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "hiChirho")
                );
                assert!(
                    matches!(&elements_chirho[1], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "loChirho")
                );
            }
            other_chirho => panic!(
                "expected tuple pattern from tight unboxed tuple syntax, got {:?}",
                other_chirho
            ),
        },
        other_chirho => panic!("expected case expression, got {:?}", other_chirho),
    }
}

#[test]
fn lower_nonempty_cons_groups_like_ghc_chirho() {
    let source_chirho = "module M where\nimport Data.List.NonEmpty (NonEmpty(..))\nfChirho xChirho yChirho = xChirho :| yChirho : []\n";
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let parser_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "fChirho")
        })
        .expect("expected fChirho binding");

    let rhs_expr_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
            other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };

    match rhs_expr_chirho {
        ExprChirho::InfixChirho {
            left_chirho,
            op_chirho,
            right_chirho,
            ..
        } => {
            assert_eq!(op_chirho.text_chirho(), ":|");
            assert!(
                matches!(
                    &**left_chirho,
                    ExprChirho::VarChirho(name_chirho)
                        if name_chirho.text_chirho() == "xChirho"
                ),
                "expected xChirho as the head element, got {:?}",
                left_chirho
            );
            assert!(
                matches!(
                    &**right_chirho,
                    ExprChirho::InfixChirho { op_chirho, .. }
                        if op_chirho.text_chirho() == ":"
                ),
                "expected the tail list to stay grouped on the right of :|, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!("expected infix expression tree, got {:?}", other_chirho),
    }
}
