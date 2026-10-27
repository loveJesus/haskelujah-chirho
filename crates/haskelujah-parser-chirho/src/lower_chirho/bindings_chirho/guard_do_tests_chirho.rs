// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: Guards, pattern guards and `do`, `mdo` and `rec` blocks.
//! Moved from `lower_chirho.rs`'s tests in row 641.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, LocalBindChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use super::test_support_chirho::parse_and_lower_chirho;

#[test]
fn lower_infix_fun_bind_with_signature_and_do_rhs_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nimport Control.Monad\nnewtype ExceptTChirho eChirho mChirho aChirho = ExceptTChirho { runExceptTChirho :: mChirho (Either eChirho aChirho) }\ncatchEChirho :: Monad mChirho => ExceptTChirho eChirho mChirho aChirho -> (eChirho -> ExceptTChirho ePrimeChirho mChirho aChirho) -> ExceptTChirho ePrimeChirho mChirho aChirho\nmChirho `catchEChirho` hChirho = ExceptTChirho $ do\n  aChirho <- runExceptTChirho mChirho\n  case aChirho of\n    Left lChirho -> runExceptTChirho (hChirho lChirho)\n    Right rChirho -> return (Right rChirho)\n",
    );
    let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
        matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "catchEChirho")
    });
    assert!(decl_chirho.is_some(), "should lower catchE infix funbind");
    if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
        assert_eq!(
            matches_chirho[0].pats_chirho.len(),
            2,
            "signed infix catchE funbind should keep both lhs operands as patterns"
        );
        assert!(
            matches!(
                &matches_chirho[0].rhs_chirho,
                RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                    arg_chirho,
                    ..
                }) if matches!(&**arg_chirho, ExprChirho::DoChirho { .. })
            ),
            "signed infix catchE funbind should preserve the constructor/do RHS"
        );
    }
}

#[test]
fn lower_guarded_fun_bind_keeps_where_clause_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nsplitFileName_Chirho fpChirho\n  | isWindowsChirho = dirSlashChirho\n  | otherwise = fileChirho\n  where\n    dirSlashChirho = fpChirho\n    fileChirho = fpChirho\n",
    );
    let split_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "splitFileName_Chirho"
            )
        })
        .expect("expected splitFileName_Chirho binding");
    let DeclChirho::FunBindChirho { matches_chirho, .. } = split_decl_chirho else {
        panic!("expected function binding");
    };
    assert_eq!(matches_chirho.len(), 1);
    assert_eq!(
        matches_chirho[0].where_binds_chirho.len(),
        2,
        "guarded equations should retain trailing where bindings"
    );
}

#[test]
fn lower_guarded_instance_method_preserves_params_and_where_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module Main where\nclass AddRChirho aChirho where\n  addRChirho :: aChirho -> aChirho -> aChirho\ninstance AddRChirho Int where\n  addRChirho aChirho bChirho\n    | otherwise = aChirho + rChirho\n    where\n      rChirho = bChirho\n",
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
    assert_eq!(matches_chirho.len(), 1);
    assert_eq!(
        matches_chirho[0].pats_chirho.len(),
        2,
        "guarded instance method should keep both LHS parameters"
    );
    assert_eq!(
        matches_chirho[0].where_binds_chirho.len(),
        1,
        "guarded instance method should retain trailing where bindings"
    );
}

#[test]
fn lower_pattern_guard_keeps_its_qualifiers_chirho() {
    // A pattern guard stays a qualifier of its arm, with its infix
    // constructor pattern intact, so a failing arm can fall through.
    let module_chirho = parse_and_lower_chirho(
        "module M where\nfChirho xsChirho\n  | msgChirho : _ <- xsChirho, null msgChirho = 1\n  | otherwise = 0\n",
    );
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "fChirho")
        })
        .expect("expected fChirho binding");

    let arms_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::GuardedChirho(arms_chirho) => arms_chirho,
            other_chirho => panic!("expected guarded arms, got {:?}", other_chirho),
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };
    assert_eq!(arms_chirho.len(), 2, "{arms_chirho:?}");
    match arms_chirho[0].quals_chirho.as_slice() {
        [
            StmtChirho::BindChirho {
                pat_chirho:
                    PatChirho::InfixConChirho {
                        left_chirho,
                        op_chirho,
                        right_chirho,
                        ..
                    },
                ..
            },
            StmtChirho::ExprChirho { .. },
        ] => {
            assert!(
                matches!(left_chirho.as_ref(), PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "msgChirho")
            );
            assert_eq!(op_chirho.text_chirho(), ":");
            assert!(matches!(
                right_chirho.as_ref(),
                PatChirho::WildcardChirho(_)
            ));
        }
        other_chirho => panic!(
            "expected an infix pattern guard and a boolean guard, got {:?}",
            other_chirho
        ),
    }
}

#[test]
fn lower_pattern_guard_generator_preserves_source_application_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nfChirho fvssChirho\n  | True\n  , (asPrimeChirho, fvssPrimeChirho) <- insertChirho 1 [] fvssChirho\n  = (asPrimeChirho, fvssPrimeChirho)\n  | otherwise = ([], [])\n",
    );
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "fChirho")
        })
        .expect("expected fChirho binding");
    let scrutinee_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::GuardedChirho(arms_chirho) => match arms_chirho[0].quals_chirho.as_slice() {
                [
                    StmtChirho::ExprChirho { .. },
                    StmtChirho::BindChirho { expr_chirho, .. },
                ] => expr_chirho,
                other_chirho => panic!(
                    "expected a boolean guard then a pattern guard, got {:?}",
                    other_chirho
                ),
            },
            other_chirho => panic!("expected guarded arms, got {:?}", other_chirho),
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };
    match scrutinee_chirho {
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            assert!(
                matches!(arg_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "fvssChirho"),
                "generator scrutinee should preserve final argument, got {:?}",
                arg_chirho
            );
            match fun_chirho.as_ref() {
                ExprChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    assert!(
                        matches!(arg_chirho.as_ref(), ExprChirho::ListChirho { elements_chirho, .. } if elements_chirho.is_empty()),
                        "generator scrutinee should preserve empty-list argument, got {:?}",
                        arg_chirho
                    );
                    assert!(
                        matches!(fun_chirho.as_ref(), ExprChirho::AppChirho { .. }),
                        "generator scrutinee should remain a full application chain, got {:?}",
                        fun_chirho
                    );
                }
                other_chirho => panic!(
                    "generator scrutinee should remain nested application, got {:?}",
                    other_chirho
                ),
            }
        }
        other_chirho => panic!(
            "pattern guard generator should lower to an application scrutinee, got {:?}",
            other_chirho
        ),
    }
}

#[test]
fn lower_guard_expr_preserves_rhs_application_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nfChirho nChirho tChirho\n  | nChirho `elem` freeVariablesChirho tChirho = 1\n  | otherwise = 0\nfreeVariablesChirho tChirho = tChirho\n",
    );
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "fChirho")
        })
        .expect("expected fChirho binding");
    let guard_expr_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::GuardedChirho(guards_chirho) => {
                match guards_chirho[0].quals_chirho.as_slice() {
                    [StmtChirho::ExprChirho { expr_chirho, .. }] => expr_chirho,
                    other_chirho => {
                        panic!("expected one boolean qualifier, got {:?}", other_chirho)
                    }
                }
            }
            other_chirho => panic!("expected guarded rhs, got {:?}", other_chirho),
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };
    match guard_expr_chirho {
        ExprChirho::InfixChirho {
            op_chirho,
            right_chirho,
            ..
        } => {
            assert_eq!(op_chirho.text_chirho(), "elem");
            assert!(
                matches!(right_chirho.as_ref(), ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                    if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "freeVariablesChirho")
                    && matches!(arg_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "tChirho")),
                "guard rhs should preserve `freeVariablesChirho tChirho`, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!("expected infix guard expression, got {:?}", other_chirho),
    }
}

#[test]
fn lower_where_guarded_fun_bind_preserves_lhs_params_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nsplitFileName_Chirho fpChirho = dirSlashChirho where\n  dropExcessTrailingPathSeparatorsChirho xChirho\n    | hasTrailingPathSeparator xChirho\n    , let xPrimeChirho = dropWhileEnd isPathSeparator xChirho\n    , otherwise = if | null xPrimeChirho -> singleton (last xChirho)\n                     | otherwise -> addTrailingPathSeparator xPrimeChirho\n    | otherwise = xChirho\n  dirSlashChirho = dropExcessTrailingPathSeparatorsChirho fpChirho\n",
    );
    let split_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "splitFileName_Chirho"
            )
        })
        .expect("expected splitFileName_Chirho binding");
    let DeclChirho::FunBindChirho { matches_chirho, .. } = split_decl_chirho else {
        panic!("expected function binding");
    };
    let guarded_local_chirho = matches_chirho[0]
        .where_binds_chirho
        .iter()
        .find(|bind_chirho| {
            matches!(
                bind_chirho,
                LocalBindChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho()
                        == "dropExcessTrailingPathSeparatorsChirho"
            )
        })
        .expect("guarded local binding should lower as FunBindChirho");
    let LocalBindChirho::FunBindChirho {
        matches_chirho: local_matches_chirho,
        ..
    } = guarded_local_chirho
    else {
        panic!("expected guarded local function binding");
    };
    assert_eq!(local_matches_chirho.len(), 1);
    assert_eq!(local_matches_chirho[0].pats_chirho.len(), 1);
    assert!(
        matches!(
            &local_matches_chirho[0].pats_chirho[0],
            PatChirho::VarChirho(name_chirho)
                if name_chirho.text_chirho() == "xChirho"
        ),
        "guarded local function should retain lhs parameter, got {:?}",
        local_matches_chirho[0].pats_chirho
    );
    assert!(
        !matches_chirho[0]
            .where_binds_chirho
            .iter()
            .any(|bind_chirho| {
                matches!(
                    bind_chirho,
                    LocalBindChirho::PatBindChirho { pat_chirho, .. }
                        if matches!(
                            pat_chirho,
                            PatChirho::VarChirho(name_chirho)
                                if name_chirho.text_chirho()
                                    == "dropExcessTrailingPathSeparatorsChirho"
                        )
                )
            }),
        "guarded local function should not degrade to a pattern binding"
    );
}

#[test]
fn lower_guarded_outer_where_local_fun_bind_preserves_lhs_params_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nfChirho pChirho\n  | condChirho = helperChirho pChirho\n  | otherwise = helperChirho pChirho\n  where\n    condChirho = True\n    helperChirho xChirho\n      | True\n      , let xPrimeChirho = xChirho\n      , otherwise = if | True -> xChirho\n                       | otherwise -> xPrimeChirho\n      | otherwise = xChirho\n",
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
        .expect("expected fChirho binding");
    let DeclChirho::FunBindChirho { matches_chirho, .. } = fun_decl_chirho else {
        panic!("expected function binding");
    };
    assert_eq!(matches_chirho.len(), 1);
    let helper_bind_chirho = matches_chirho[0]
        .where_binds_chirho
        .iter()
        .find(|bind_chirho| {
            matches!(
                bind_chirho,
                LocalBindChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "helperChirho"
            )
        })
        .expect("expected helperChirho local function binding");
    let LocalBindChirho::FunBindChirho {
        matches_chirho: helper_matches_chirho,
        ..
    } = helper_bind_chirho
    else {
        panic!("expected helperChirho fun bind");
    };
    assert_eq!(helper_matches_chirho.len(), 1);
    assert!(
        matches!(
            &helper_matches_chirho[0].pats_chirho[0],
            PatChirho::VarChirho(name_chirho)
                if name_chirho.text_chirho() == "xChirho"
        ),
        "helperChirho should retain xChirho as a match parameter, got {:?}",
        helper_matches_chirho[0].pats_chirho
    );
}

#[test]
fn lower_pattern_guard_constructor_binder_chirho() {
    let source_chirho = "module M where\n\
data TupleSortChirho = TupleTChirho Int | OtherTChirho\n\
fChirho tupSortChirho\n  | TupleTChirho lenChirho <- tupSortChirho = lenChirho\n  | otherwise = 0\n";
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

    let quals_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
            RhsChirho::GuardedChirho(arms_chirho) => arms_chirho[0].quals_chirho.as_slice(),
            other_chirho => panic!("expected guarded arms, got {:?}", other_chirho),
        },
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };

    match quals_chirho {
        [
            StmtChirho::BindChirho {
                pat_chirho:
                    PatChirho::ConChirho {
                        con_chirho,
                        args_chirho,
                        ..
                    },
                ..
            },
        ] => {
            assert_eq!(con_chirho.text_chirho(), "TupleTChirho");
            assert_eq!(args_chirho.len(), 1);
            assert!(
                matches!(&args_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "lenChirho")
            );
        }
        other_chirho => panic!(
            "expected one constructor pattern guard, got {:?}",
            other_chirho
        ),
    }
}

#[cfg(test)]
#[test]
fn lower_recursive_do_group_to_mfix_knot_chirho() {
    let source_chirho = concat!(
        "{-# LANGUAGE RecursiveDo #-}\n",
        "module M where\n",
        "main = do\n",
        "  rec\n",
        "    xsChirho <- pure (1 : ysChirho)\n",
        "    ysChirho <- pure (2 : xsChirho)\n",
        "  print (take 4 xsChirho)\n",
    );
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let green_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho)
            .parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let main_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "main"
            )
        })
        .expect("expected main binding");
    let DeclChirho::FunBindChirho { matches_chirho, .. } = main_chirho else {
        panic!("expected main function binding");
    };
    let RhsChirho::UnguardedChirho(ExprChirho::DoChirho { stmts_chirho, .. }) =
        &matches_chirho[0].rhs_chirho
    else {
        panic!("expected a lowered do expression");
    };

    assert_eq!(stmts_chirho.len(), 2);
    assert!(matches!(
        &stmts_chirho[0],
        StmtChirho::BindChirho {
            pat_chirho: PatChirho::TupleChirho { elements_chirho, .. },
            expr_chirho: ExprChirho::AppChirho { fun_chirho, arg_chirho, .. },
            ..
        } if elements_chirho.len() == 2
            && matches!(
                fun_chirho.as_ref(),
                ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "mfix"
            )
            && matches!(
                arg_chirho.as_ref(),
                ExprChirho::LamChirho { pats_chirho, .. }
                    if matches!(
                        pats_chirho.as_slice(),
                        [PatChirho::LazyChirho { .. }]
                    )
            )
    ));
}

#[cfg(test)]
#[test]
fn lower_qualified_do_preserves_module_qualifier_chirho() {
    let source_chirho = concat!(
        "{-# LANGUAGE QualifiedDo #-}\n",
        "module M where\n",
        "main = FlowChirho.do\n",
        "  valueChirho <- actionChirho\n",
        "  finishChirho valueChirho\n",
        "nextChirho :: Int\n",
        "nextChirho = 1\n",
    );
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let green_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho)
            .parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let main_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "main"
            )
        })
        .expect("expected main binding");
    let DeclChirho::FunBindChirho { matches_chirho, .. } = main_chirho else {
        panic!("expected main function binding");
    };
    let RhsChirho::UnguardedChirho(ExprChirho::DoChirho {
        qualifier_chirho,
        stmts_chirho,
        ..
    }) = &matches_chirho[0].rhs_chirho
    else {
        panic!("expected a lowered qualified do expression");
    };

    assert_eq!(qualifier_chirho.as_deref(), Some("FlowChirho"));
    assert_eq!(stmts_chirho.len(), 2);
    assert!(matches!(stmts_chirho[0], StmtChirho::BindChirho { .. }));
    assert!(module_chirho.decls_chirho.iter().any(|decl_chirho| {
        matches!(
            decl_chirho,
            DeclChirho::TypeSigChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "nextChirho"
        )
    }));
    assert!(module_chirho.decls_chirho.iter().any(|decl_chirho| {
        matches!(
            decl_chirho,
            DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "nextChirho"
        )
    }));
}

#[cfg(test)]
#[test]
fn lower_mdo_wraps_statements_before_final_expression_chirho() {
    let source_chirho = concat!(
        "{-# LANGUAGE RecursiveDo #-}\n",
        "module M where\n",
        "main = mdo\n",
        "  xChirho <- pure yChirho\n",
        "  yChirho <- pure 1\n",
        "  pure xChirho\n",
    );
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let green_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho)
            .parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let main_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "main"
            )
        })
        .expect("expected main binding");
    let DeclChirho::FunBindChirho { matches_chirho, .. } = main_chirho else {
        panic!("expected main function binding");
    };
    let RhsChirho::UnguardedChirho(ExprChirho::DoChirho { stmts_chirho, .. }) =
        &matches_chirho[0].rhs_chirho
    else {
        panic!("expected a lowered mdo expression");
    };

    assert_eq!(stmts_chirho.len(), 2);
    assert!(matches!(
        &stmts_chirho[0],
        StmtChirho::BindChirho {
            pat_chirho: PatChirho::TupleChirho { elements_chirho, .. },
            expr_chirho: ExprChirho::AppChirho { fun_chirho, .. },
            ..
        } if elements_chirho.len() == 2
            && matches!(
                fun_chirho.as_ref(),
                ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "mfix"
            )
    ));
    assert!(matches!(
        stmts_chirho[1],
        StmtChirho::ExprChirho { expr_chirho: _, .. }
    ));
}

#[cfg(test)]
#[test]
fn lower_mdo_and_rec_as_plain_bindings_without_recursive_do_chirho() {
    let source_chirho = "module M where\nmdo = 7\nrec = 8\n";
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let green_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho)
            .parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    let names_chirho: Vec<&str> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => Some(name_chirho.text_chirho()),
            _ => None,
        })
        .collect();

    assert!(names_chirho.contains(&"mdo"));
    assert!(names_chirho.contains(&"rec"));
}
