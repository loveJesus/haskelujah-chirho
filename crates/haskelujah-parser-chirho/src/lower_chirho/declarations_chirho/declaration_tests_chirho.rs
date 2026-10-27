// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: Declarations, pragmas, foreign declarations and Template Haskell.
//! Moved from `lower_chirho.rs`'s tests in row 641.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use super::test_support_chirho::parse_and_lower_chirho;

#[test]
fn lower_infix_expr_with_lambda_rhs_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nonEChirho action1Chirho action2Chirho = action1Chirho `catchEChirho` \\eChirho -> action2Chirho >> throwEChirho eChirho\n",
    );
    let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
        matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "onEChirho")
    });
    assert!(decl_chirho.is_some(), "should lower onE infix expression");
    if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
        assert!(
            matches!(
                &matches_chirho[0].rhs_chirho,
                RhsChirho::UnguardedChirho(ExprChirho::InfixChirho {
                    right_chirho,
                    ..
                }) if matches!(
                    &**right_chirho,
                    ExprChirho::LamChirho {
                        body_chirho,
                        ..
                    } if matches!(
                        &**body_chirho,
                        ExprChirho::InfixChirho {
                            op_chirho,
                            right_chirho,
                            ..
                        } if op_chirho.text_chirho() == ">>"
                            && matches!(
                                &**right_chirho,
                                ExprChirho::AppChirho {
                                    fun_chirho,
                                    arg_chirho,
                                    ..
                                } if matches!(
                                    (&**fun_chirho, &**arg_chirho),
                                    (
                                        ExprChirho::VarChirho(fun_name_chirho),
                                        ExprChirho::VarChirho(arg_name_chirho),
                                    ) if fun_name_chirho.text_chirho() == "throwEChirho"
                                        && arg_name_chirho.text_chirho() == "eChirho"
                                )
                            )
                    )
                )
            ),
            "infix catchE expression should preserve the full lambda body in the lowered AST"
        );
    }
}

#[test]
fn lower_default_decl_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\ndefault (Int, Double)\n");
    let default_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| matches!(d_chirho, DeclChirho::DefaultDeclChirho { .. }));
    assert!(default_chirho.is_some(), "expected a DefaultDeclChirho");

    match default_chirho.unwrap() {
        DeclChirho::DefaultDeclChirho { types_chirho, .. } => {
            assert_eq!(types_chirho.len(), 2);
            match &types_chirho[0] {
                TypeChirho::ConChirho(name_chirho) => {
                    assert_eq!(name_chirho.text_chirho(), "Int");
                }
                _ => panic!("expected ConChirho"),
            }
            match &types_chirho[1] {
                TypeChirho::ConChirho(name_chirho) => {
                    assert_eq!(name_chirho.text_chirho(), "Double");
                }
                _ => panic!("expected ConChirho"),
            }
        }
        _ => unreachable!(),
    }
}

#[test]
fn lower_specialize_pragma_chirho() {
    let src_chirho = "module M where\n{-# SPECIALIZE double :: Int -> Int #-}\ndouble x = x + x\n";
    let module_chirho = parse_and_lower_chirho(src_chirho);
    assert_eq!(module_chirho.specialize_pragmas_chirho.len(), 1);
    let specs_chirho = module_chirho
        .specialize_pragmas_chirho
        .get("double")
        .expect("missing double");
    assert_eq!(specs_chirho.len(), 1);
    assert_eq!(specs_chirho[0], "Int -> Int");
}

#[test]
fn lower_specialise_british_spelling_chirho() {
    let src_chirho = "module M where\n{-# SPECIALISE sort :: [Int] -> [Int] #-}\nsort xs = xs\n";
    let module_chirho = parse_and_lower_chirho(src_chirho);
    assert_eq!(module_chirho.specialize_pragmas_chirho.len(), 1);
    let specs_chirho = module_chirho
        .specialize_pragmas_chirho
        .get("sort")
        .expect("missing sort");
    assert_eq!(specs_chirho.len(), 1);
    assert_eq!(specs_chirho[0], "[Int] -> [Int]");
}

#[test]
fn lower_multiple_specialize_pragmas_chirho() {
    let src_chirho = "module M where\n{-# SPECIALIZE f :: Int -> Int #-}\n{-# SPECIALIZE f :: Double -> Double #-}\nf x = x\n";
    let module_chirho = parse_and_lower_chirho(src_chirho);
    let specs_chirho = module_chirho
        .specialize_pragmas_chirho
        .get("f")
        .expect("missing f");
    assert_eq!(specs_chirho.len(), 2);
    assert_eq!(specs_chirho[0], "Int -> Int");
    assert_eq!(specs_chirho[1], "Double -> Double");
}

#[test]
fn lower_no_specialize_pragma_chirho() {
    let src_chirho = "module M where\nf x = x\n";
    let module_chirho = parse_and_lower_chirho(src_chirho);
    assert!(module_chirho.specialize_pragmas_chirho.is_empty());
}

// -------------------------------------------------------------------
// Template Haskell lowering tests
// -------------------------------------------------------------------

#[test]
fn lower_th_splice_name_chirho() {
    // $name lowers to ExprChirho::SpliceChirho
    let module_chirho = parse_and_lower_chirho("module M where\nx = $foo\n");
    let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "x")
    });
    assert!(decl_chirho.is_some(), "should have FunBind for x");
    if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
        let rhs_chirho = &matches_chirho[0].rhs_chirho;
        if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
            assert!(
                matches!(expr_chirho, ExprChirho::SpliceChirho { .. }),
                "RHS should be SpliceChirho, got {:?}",
                expr_chirho
            );
        } else {
            panic!("expected unguarded RHS");
        }
    }
}

#[test]
fn lower_th_splice_parens_chirho() {
    // $(expr) lowers to ExprChirho::SpliceChirho wrapping the inner expression
    let module_chirho = parse_and_lower_chirho("module M where\nx = $(makeLenses foo)\n");
    let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "x")
    });
    assert!(decl_chirho.is_some(), "should have FunBind for x");
    if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
        let rhs_chirho = &matches_chirho[0].rhs_chirho;
        if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
            assert!(
                matches!(expr_chirho, ExprChirho::SpliceChirho { .. }),
                "RHS should be SpliceChirho for $(makeLenses foo), got {:?}",
                expr_chirho
            );
        } else {
            panic!("expected unguarded RHS");
        }
    }
}

#[test]
fn lower_th_splice_decl_chirho() {
    // Top-level $(expr) lowers to DeclChirho::SpliceDeclChirho
    let module_chirho = parse_and_lower_chirho("module M where\n$(makeLenses foo)\n");
    assert!(
        module_chirho
            .decls_chirho
            .iter()
            .any(|d_chirho| { matches!(d_chirho, DeclChirho::SpliceDeclChirho { .. }) }),
        "should have SpliceDeclChirho for top-level $(makeLenses foo): {:?}",
        module_chirho.decls_chirho
    );
}

#[test]
fn lower_th_quote_expr_chirho() {
    // [| expr |] lowers to ExprChirho::QuoteExprChirho
    let module_chirho = parse_and_lower_chirho("module M where\nx = [| foo |]\n");
    let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "x")
    });
    assert!(decl_chirho.is_some(), "should have FunBind for x");
    if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
        let rhs_chirho = &matches_chirho[0].rhs_chirho;
        if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
            assert!(
                matches!(expr_chirho, ExprChirho::QuoteExprChirho { .. }),
                "RHS should be QuoteExprChirho for [| foo |], got {:?}",
                expr_chirho
            );
        } else {
            panic!("expected unguarded RHS");
        }
    }
}

#[test]
fn lower_th_name_quote_value_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\nx = 'bimapConst\n");
    let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "x")
    });
    if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
        if let RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        }) = &matches_chirho[0].rhs_chirho
        {
            assert!(
                matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "mkName")
            );
            assert!(
                matches!(arg_chirho.as_ref(), ExprChirho::LitChirho(LitChirho::StringChirho(text_chirho, _, _)) if text_chirho == "bimapConst")
            );
        } else {
            panic!("quoted TH value name should lower to mkName application");
        }
    } else {
        panic!("expected funbind for x");
    }
}

#[test]
fn lower_th_name_quote_operator_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\nx = '(.)\n");
    let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "x")
    });
    if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
        if let RhsChirho::UnguardedChirho(ExprChirho::AppChirho { arg_chirho, .. }) =
            &matches_chirho[0].rhs_chirho
        {
            assert!(
                matches!(arg_chirho.as_ref(), ExprChirho::LitChirho(LitChirho::StringChirho(text_chirho, _, _)) if text_chirho == ".")
            );
        } else {
            panic!("quoted TH operator name should lower to mkName application");
        }
    } else {
        panic!("expected funbind for x");
    }
}

#[test]
fn lower_th_name_quote_tilde_operator_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\nx = ''(~)\n");
    let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
        matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "x")
    });
    if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
        if let RhsChirho::UnguardedChirho(ExprChirho::AppChirho { arg_chirho, .. }) =
            &matches_chirho[0].rhs_chirho
        {
            assert!(
                matches!(arg_chirho.as_ref(), ExprChirho::LitChirho(LitChirho::StringChirho(text_chirho, _, _)) if text_chirho == "~")
            );
        } else {
            panic!("quoted TH tilde operator should lower to mkName application");
        }
    } else {
        panic!("expected funbind for x");
    }
}

#[test]
fn lower_th_splice_decl_make_lenses_chirho() {
    // $(makeLenses ''Foo) at top level after a data declaration
    let module_chirho =
        parse_and_lower_chirho("module M where\ndata Foo = Foo\n$(makeLenses foo)\n");
    assert!(
        module_chirho
            .decls_chirho
            .iter()
            .any(|d_chirho| { matches!(d_chirho, DeclChirho::DataDeclChirho { .. }) }),
        "should have DataDecl"
    );
    assert!(
        module_chirho
            .decls_chirho
            .iter()
            .any(|d_chirho| { matches!(d_chirho, DeclChirho::SpliceDeclChirho { .. }) }),
        "should have SpliceDeclChirho"
    );
}

#[test]
fn lower_module_fixity_decl_for_backticked_operator_chirho() {
    let source_chirho = "module M where\ninfixl 3 `ApChirho`\nvalueChirho = uChirho `ApChirho` (flipChirho <$> fChirho) <*> vChirho\n";
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
                    if name_chirho.text_chirho() == "valueChirho")
        })
        .expect("expected valueChirho binding");

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
            assert_eq!(op_chirho.text_chirho(), "ApChirho");
            assert!(
                matches!(
                    &**left_chirho,
                    ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "uChirho"
                ),
                "expected left operand to remain uChirho, got {:?}",
                left_chirho
            );
            assert!(
                matches!(
                    &**right_chirho,
                    ExprChirho::InfixChirho { op_chirho, .. }
                        if op_chirho.text_chirho() == "<*>"
                ),
                "expected <*> to stay grouped on the right of ApChirho, got {:?}",
                right_chirho
            );
        }
        other_chirho => panic!("expected infix expression tree, got {:?}", other_chirho),
    }
}

#[test]
fn lower_show_paren_dollar_keeps_composition_rhs_with_char_literal_chirho() {
    let source_chirho = "module M where\nfChirho spChirho nameChirho dChirho xChirho = showParen (dChirho > 10) $\n  showString nameChirho . showChar ' ' . spChirho 11 xChirho\n";
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
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            assert!(
                matches!(
                    fun_chirho.as_ref(),
                    ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                        if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "showParen")
                            && matches!(arg_chirho.as_ref(), ExprChirho::ParenChirho { .. })
                ),
                "expected the dollar lhs to stay `showParen (dChirho > 10)`, got {:?}",
                fun_chirho
            );
            let ExprChirho::InfixChirho {
                left_chirho: outer_left_chirho,
                op_chirho: outer_op_chirho,
                right_chirho: outer_right_chirho,
                ..
            } = arg_chirho.as_ref()
            else {
                panic!(
                    "expected the dollar rhs to stay an infix composition tree, got {:?}",
                    arg_chirho
                );
            };
            assert_eq!(outer_op_chirho.text_chirho(), ".");
            assert!(
                matches!(
                    outer_left_chirho.as_ref(),
                    ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                        if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "showString")
                            && matches!(arg_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "nameChirho")
                ),
                "expected the lhs of the dollar rhs to stay `showString nameChirho`, got {:?}",
                outer_left_chirho
            );
            let ExprChirho::InfixChirho {
                left_chirho: inner_left_chirho,
                op_chirho: inner_op_chirho,
                right_chirho: inner_right_chirho,
                ..
            } = outer_right_chirho.as_ref()
            else {
                panic!(
                    "expected the rhs tail to stay a nested composition, got {:?}",
                    outer_right_chirho
                );
            };
            assert_eq!(inner_op_chirho.text_chirho(), ".");
            assert!(
                matches!(
                    inner_left_chirho.as_ref(),
                    ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                        if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "showChar")
                            && matches!(arg_chirho.as_ref(), ExprChirho::LitChirho(LitChirho::CharChirho(' ', _, _)))
                ),
                "expected the middle composition operand to stay `showChar ' '`, got {:?}",
                inner_left_chirho
            );
            assert!(
                matches!(
                    inner_right_chirho.as_ref(),
                    ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                        if matches!(fun_chirho.as_ref(), ExprChirho::AppChirho { fun_chirho, arg_chirho, .. }
                            if matches!(fun_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "spChirho")
                                && matches!(arg_chirho.as_ref(), ExprChirho::LitChirho(LitChirho::IntChirho(11, _, _))))
                            && matches!(arg_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "xChirho")
                ),
                "expected the tail operand to stay `spChirho 11 xChirho`, got {:?}",
                inner_right_chirho
            );
        }
        other_chirho => panic!(
            "expected `$` lowering to become application of the composition tree, got {:?}",
            other_chirho
        ),
    }
}

#[test]
fn lower_helpers_after_instance_and_haddock_comments_stay_top_level_chirho() {
    let source_chirho = "module M where\n\
class Read1Chirho fChirho where\n\
  liftReadsPrecChirho :: Int\n\
\n\
instance Read1Chirho Maybe where\n\
  liftReadsPrecChirho = readsDataChirho\n\
\n\
-- Building blocks\n\
\n\
-- | readsData docs\n\
readsDataChirho :: Int\n\
readsDataChirho = 1\n\
\n\
-- | readsUnaryWith docs\n\
readsUnaryWithChirho :: Int\n\
readsUnaryWithChirho = 2\n\
\n\
-- | showsUnaryWith docs\n\
showsUnaryWithChirho :: Int\n\
showsUnaryWithChirho = 3\n";
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let parser_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);

    let top_level_names_chirho: Vec<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeSigChirho { name_chirho, .. }
            | DeclChirho::FunBindChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect();

    assert!(
        top_level_names_chirho
            .iter()
            .any(|name_chirho| name_chirho == "readsDataChirho"),
        "readsData helper should stay top-level: {:?}",
        top_level_names_chirho
    );
    assert!(
        top_level_names_chirho
            .iter()
            .any(|name_chirho| name_chirho == "readsUnaryWithChirho"),
        "readsUnaryWith helper should stay top-level: {:?}",
        top_level_names_chirho
    );
    assert!(
        top_level_names_chirho
            .iter()
            .any(|name_chirho| name_chirho == "showsUnaryWithChirho"),
        "showsUnaryWith helper should stay top-level: {:?}",
        top_level_names_chirho
    );
}
