// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: Expressions, operators and precedence.
//! Moved from `lower_chirho.rs`'s tests in row 641.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use super::test_support_chirho::{collect_char_literals_from_expr_chirho, parse_and_lower_chirho};

#[test]
fn lower_backticked_left_section_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\nrwhnfChirho = (`seq` ())\n");
    let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
        matches!(
            decl_chirho,
            DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "rwhnfChirho"
        )
    });
    assert!(
        decl_chirho.is_some(),
        "should lower backticked left section"
    );
    if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
        match &matches_chirho[0].rhs_chirho {
            RhsChirho::UnguardedChirho(ExprChirho::LeftSectionChirho {
                op_chirho,
                arg_chirho,
                ..
            }) => {
                assert_eq!(op_chirho.text_chirho(), "seq");
                assert!(
                    matches!(
                        arg_chirho.as_ref(),
                        ExprChirho::TupleChirho { elements_chirho, .. }
                            if elements_chirho.is_empty()
                    ),
                    "expected unit tuple argument, got {:?}",
                    arg_chirho
                );
            }
            other_chirho => panic!("expected backticked left section, got {:?}", other_chirho),
        }
    }
}

#[test]
fn lower_qualified_right_section_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module MChirho where\nbuildChirho kChirho = (kChirho SeqChirho.<|)\n",
    );
    let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
        matches!(
            decl_chirho,
            DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "buildChirho"
        )
    });
    assert!(
        decl_chirho.is_some(),
        "should lower qualified right section"
    );
    if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
        match &matches_chirho[0].rhs_chirho {
            RhsChirho::UnguardedChirho(ExprChirho::RightSectionChirho {
                arg_chirho,
                op_chirho,
                ..
            }) => {
                assert_eq!(op_chirho.full_name_chirho(), "SeqChirho.<|");
                assert!(
                    matches!(
                        arg_chirho.as_ref(),
                        ExprChirho::VarChirho(name_chirho)
                            if name_chirho.text_chirho() == "kChirho"
                    ),
                    "expected kChirho section argument, got {:?}",
                    arg_chirho
                );
            }
            other_chirho => panic!("expected qualified right section, got {:?}", other_chirho),
        }
    }
}

#[test]
fn lower_parallel_list_comp_preserves_qualifier_groups_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE MonadComprehensions, ParallelListComp #-}\n\
             module M where\n\
             fooChirho xsChirho ysChirho =\n\
               [ (fChirho yChirho True, fChirho xChirho 'c')\n\
               | let fChirho _ zChirho = zChirho, xChirho <- xsChirho\n\
               | yChirho <- ysChirho ]\n",
    );
    let fun_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "fooChirho"
            )
        })
        .expect("expected fooChirho fun bind");
    let DeclChirho::FunBindChirho { matches_chirho, .. } = fun_decl_chirho else {
        panic!("expected function binding");
    };
    let RhsChirho::UnguardedChirho(rhs_expr_chirho) = &matches_chirho[0].rhs_chirho else {
        panic!("expected unguarded rhs");
    };
    let ExprChirho::ListCompChirho {
        quals_chirho,
        parallel_quals_chirho,
        ..
    } = rhs_expr_chirho
    else {
        panic!("expected parallel list comprehension");
    };

    assert!(
        quals_chirho.is_empty(),
        "expected parallel groups, got sequential={quals_chirho:?}, parallel={parallel_quals_chirho:?}"
    );
    assert_eq!(parallel_quals_chirho.len(), 2);
    assert!(
        matches!(
            parallel_quals_chirho[0].as_slice(),
            [StmtChirho::LetChirho { .. }, StmtChirho::BindChirho { .. }]
        ),
        "unexpected first qualifier group: {:?}",
        parallel_quals_chirho[0]
    );
    assert!(
        matches!(
            parallel_quals_chirho[1].as_slice(),
            [StmtChirho::BindChirho { .. }]
        ),
        "unexpected second qualifier group: {:?}",
        parallel_quals_chirho[1]
    );
}

#[test]
fn lower_qualified_infix_operator_expr_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nimport qualified Data.Map as Map\nfChirho mChirho kChirho = mChirho Map.! kChirho\n",
    );
    let fun_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "fChirho"
            )
        })
        .unwrap_or_else(|| panic!("expected fChirho funbind"));

    let DeclChirho::FunBindChirho { matches_chirho, .. } = fun_decl_chirho else {
        panic!("expected function binding");
    };
    let RhsChirho::UnguardedChirho(rhs_expr_chirho) = &matches_chirho[0].rhs_chirho else {
        panic!("expected unguarded rhs");
    };

    match rhs_expr_chirho {
        ExprChirho::InfixChirho {
            left_chirho,
            op_chirho,
            right_chirho,
            ..
        } => {
            assert_eq!(op_chirho.full_name_chirho(), "Map.!");
            assert!(
                matches!(&**left_chirho, ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "mChirho"),
                "expected left operand mChirho, got {:?}",
                left_chirho
            );
            assert!(
                matches!(&**right_chirho, ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "kChirho"),
                "expected right operand kChirho, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!("expected infix expression, got {:?}", other_chirho),
    }
}

#[test]
fn lower_tabulate_projection_argument_keeps_dot_operator_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nclass Representable f where\n  type Rep f\n  tabulate :: (a -> Rep f) -> f a\ninstance (Representable f, Representable g) => Representable (Product f g) where\n  type Rep (Product f g) = (Rep f, Rep g)\n  tabulate h = Pair (tabulate (fst . h)) (tabulate (snd . h))\n",
    );
    let inst_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
        .expect("should lower instance decl");
    match inst_decl_chirho {
        DeclChirho::InstanceDeclChirho { methods_chirho, .. } => {
            let tabulate_bind_chirho = methods_chirho
                .iter()
                .find(|bind_chirho| {
                    matches!(
                        bind_chirho,
                        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                            name_chirho,
                            ..
                        } if name_chirho.text_chirho() == "tabulate"
                    )
                })
                .expect("should lower tabulate method");
            let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                matches_chirho,
                ..
            } = tabulate_bind_chirho
            else {
                unreachable!();
            };
            let rhs_debug_chirho = format!("{:?}", matches_chirho[0].rhs_chirho);
            assert!(
                rhs_debug_chirho.contains("\".\""),
                "tabulate rhs should preserve composition operator, got {:?}",
                matches_chirho[0].rhs_chirho
            );
        }
        other_chirho => panic!("expected instance decl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_nested_dollar_keeps_its_right_association_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nx = WriterT $ callCC $ \\c -> runWriterT (f (\\a -> WriterT (c (a, mempty))))\n",
    );
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. } if name_chirho.text_chirho() == "x"))
        .expect("expected x binding");
    let rhs_expr_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
            other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };

    // `$` stays an operator through lowering (row 619): the desugarer makes the Prelude's `$`
    // an application, and a program's own `$` keeps its own binding.
    match rhs_expr_chirho {
        ExprChirho::InfixChirho {
            left_chirho,
            op_chirho,
            right_chirho,
            ..
        } => {
            assert_eq!(op_chirho.text_chirho(), "$");
            assert!(
                matches!(&**left_chirho, ExprChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "WriterT"),
                "the outer $ should take WriterT on its left, got {:?}",
                left_chirho
            );
            assert!(
                matches!(&**right_chirho, ExprChirho::InfixChirho { op_chirho, .. } if op_chirho.text_chirho() == "$"),
                "the nested $ chain should stay right-associated, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!(
            "expected an infix $ tree for nested $, got {:?}",
            other_chirho
        ),
    }
}

#[test]
fn lower_bits_and_comparison_precedence_chirho() {
    let source_chirho = "module M where\nfChirho nChirho hChirho = nChirho .&. hChirho /= 0\n";
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
            assert_eq!(op_chirho.text_chirho(), "/=");
            assert!(
                matches!(
                    &**left_chirho,
                    ExprChirho::InfixChirho { op_chirho, .. }
                        if op_chirho.text_chirho() == ".&."
                ),
                "expected .&. to bind tighter than /=, got {:?}",
                left_chirho
            );
            assert!(
                matches!(
                    &**right_chirho,
                    ExprChirho::LitChirho(LitChirho::IntChirho(0, _, _))
                ),
                "expected rhs zero literal, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!("expected infix expression tree, got {:?}", other_chirho),
    }
}

#[test]
fn lower_comonad_apply_and_functor_precedence_chirho() {
    let source_chirho =
        "module M where\nfChirho f1Chirho aChirho bChirho = f1Chirho <$> aChirho <@> bChirho\n";
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
            assert_eq!(op_chirho.text_chirho(), "<@>");
            assert!(
                matches!(
                    &**left_chirho,
                    ExprChirho::InfixChirho { op_chirho, .. }
                        if op_chirho.text_chirho() == "<$>"
                ),
                "expected <$> to associate left with <@>, got {:?}",
                left_chirho
            );
            assert!(
                matches!(
                    &**right_chirho,
                    ExprChirho::VarChirho(name_chirho)
                        if name_chirho.text_chirho() == "bChirho"
                ),
                "expected rhs variable bChirho, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!("expected infix expression tree, got {:?}", other_chirho),
    }
}

#[test]
fn lower_semigroupoids_apply_and_functor_precedence_chirho() {
    let source_chirho =
        "module M where\nfChirho f1Chirho aChirho bChirho = f1Chirho <$> aChirho <.> bChirho\n";
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
            assert_eq!(op_chirho.text_chirho(), "<.>");
            assert!(
                matches!(
                    &**left_chirho,
                    ExprChirho::InfixChirho { op_chirho, .. }
                        if op_chirho.text_chirho() == "<$>"
                ),
                "expected <$> to associate left with <.>, got {:?}",
                left_chirho
            );
            assert!(
                matches!(
                    &**right_chirho,
                    ExprChirho::VarChirho(name_chirho)
                        if name_chirho.text_chirho() == "bChirho"
                ),
                "expected rhs variable bChirho, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!("expected infix expression tree, got {:?}", other_chirho),
    }
}

#[test]
fn lower_strict_dollar_binds_looser_than_addition_chirho() {
    let source_chirho = "module M where\nfChirho predicateChirho prefixChirho bPrimeChirho dChirho = predicateChirho $! prefixChirho + bPrimeChirho + dChirho\n";
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
            assert_eq!(op_chirho.text_chirho(), "$!");
            assert!(
                matches!(&**left_chirho, ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "predicateChirho"),
                "expected predicate as left operand, got {:?}",
                left_chirho
            );
            assert!(
                matches!(
                    &**right_chirho,
                    ExprChirho::InfixChirho { op_chirho, .. }
                        if op_chirho.text_chirho() == "+"
                ),
                "expected right operand to remain an addition tree, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!("expected infix expression tree, got {:?}", other_chirho),
    }
}

#[test]
fn lower_bind_binds_looser_than_composition_chirho() {
    let source_chirho = "module M where\nfChirho xsChirho alternativesChirho apPrimeChirho ysChirho = xsChirho >>= alternativesChirho . (`apPrimeChirho` ysChirho)\n";
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
            assert_eq!(op_chirho.text_chirho(), ">>=");
            assert!(
                matches!(&**left_chirho, ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "xsChirho"),
                "expected xsChirho as left operand, got {:?}",
                left_chirho
            );
            assert!(
                matches!(
                    &**right_chirho,
                    ExprChirho::InfixChirho { op_chirho, .. }
                        if op_chirho.text_chirho() == "."
                ),
                "expected composition on the rhs of >>=, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!("expected infix expression tree, got {:?}", other_chirho),
    }
}

#[test]
fn lower_show_composition_keeps_application_groups_around_char_literal_chirho() {
    let source_chirho = "module M where\nfChirho spChirho nameChirho dChirho xChirho = showString nameChirho . showChar ' ' . spChirho 11 xChirho\n";
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
            assert_eq!(op_chirho.text_chirho(), ".");
            assert!(
                matches!(
                    left_chirho.as_ref(),
                    ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                        if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "showString")
                            && matches!(arg_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "nameChirho")
                ),
                "expected the leftmost composition operand to stay `showString nameChirho`, got {:?}",
                left_chirho
            );
            assert!(
                matches!(
                    right_chirho.as_ref(),
                    ExprChirho::InfixChirho { left_chirho, op_chirho, right_chirho, .. }
                        if op_chirho.text_chirho() == "."
                            && matches!(left_chirho.as_ref(), ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                                if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "showChar")
                                    && matches!(arg_chirho.as_ref(), ExprChirho::LitChirho(LitChirho::CharChirho(' ', _, _))))
                            && matches!(right_chirho.as_ref(), ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                                if matches!(fun_chirho.as_ref(), ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                                    if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "spChirho")
                                        && matches!(arg_chirho.as_ref(), ExprChirho::LitChirho(LitChirho::IntChirho(11, _, _))))
                                    && matches!(arg_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "xChirho"))
                ),
                "expected the right-associated composition tail to preserve `showChar ' ' . spChirho 11 xChirho`, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!("expected infix composition tree, got {:?}", other_chirho),
    }
}

#[test]
fn lower_tuple_showchar_composition_chain_keeps_punctuation_char_literals_chirho() {
    let source_chirho = "module M where\ntupleShowsMiniChirho sp1Chirho sp2Chirho xChirho yChirho = showChar '(' . sp1Chirho 0 xChirho . showChar ',' . sp2Chirho 0 yChirho . showChar ')'\n";
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
                    if name_chirho.text_chirho() == "tupleShowsMiniChirho")
        })
        .expect("expected tupleShowsMiniChirho binding");

    let rhs_expr_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
            other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };
    let mut found_punctuation_chars_chirho = Vec::new();
    collect_char_literals_from_expr_chirho(rhs_expr_chirho, &mut found_punctuation_chars_chirho);
    assert_eq!(
        found_punctuation_chars_chirho,
        vec!['(', ',', ')'],
        "tuple-style showChar composition should lower punctuation chars as Char literals, got {:?}",
        rhs_expr_chirho
    );
}
