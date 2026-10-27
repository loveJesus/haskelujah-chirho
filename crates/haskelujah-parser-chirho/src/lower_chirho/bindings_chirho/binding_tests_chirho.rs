// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: Function and pattern bindings and `where` blocks.
//! Moved from `lower_chirho.rs`'s tests in row 641.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::LocalBindChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

use super::test_support_chirho::parse_and_lower_chirho;

#[test]
fn lower_fun_bind_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\nf x = x\n");
    assert!(module_chirho.decls_chirho.iter().any(|d_chirho| {
        matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. } if name_chirho.text_chirho() == "f")
    }));
}

#[test]
fn lower_where_block_stays_local_in_parsect_shape_chirho() {
    let module_chirho = parse_and_lower_chirho(
        r#"module M where
runParsecTChirho pChirho sChirho = unParserChirho pChirho sChirho cokChirho cerrChirho eokChirho eerrChirho
    where cokChirho aChirho s'Chirho errChirho = return (OkChirho aChirho s'Chirho errChirho)
          cerrChirho errChirho = return (ErrorChirho errChirho)
          eokChirho aChirho s'Chirho errChirho = return (OkChirho aChirho s'Chirho errChirho)
          eerrChirho errChirho = return (ErrorChirho errChirho)

data TailChirho = TailChirho
"#,
    );
    assert_eq!(
        module_chirho.decls_chirho.len(),
        2,
        "{:?}",
        module_chirho.decls_chirho
    );
    match &module_chirho.decls_chirho[0] {
        DeclChirho::FunBindChirho { matches_chirho, .. } => {
            assert_eq!(matches_chirho.len(), 1);
            let local_bind_names_chirho: Vec<String> = matches_chirho[0]
                .where_binds_chirho
                .iter()
                .filter_map(|bind_chirho| match bind_chirho {
                    LocalBindChirho::FunBindChirho { name_chirho, .. } => {
                        Some(name_chirho.text_chirho().to_string())
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(
                local_bind_names_chirho,
                vec![
                    "cokChirho".to_string(),
                    "cerrChirho".to_string(),
                    "eokChirho".to_string(),
                    "eerrChirho".to_string()
                ]
            );
        }
        other_chirho => panic!("expected FunBindChirho, got {:?}", other_chirho),
    }
}

#[test]
fn lower_infix_fun_bind_with_backticked_name_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\ncatchEChirho :: aChirho -> bChirho -> cChirho\nmChirho `catchEChirho` hChirho = hChirho\n",
    );
    let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
        matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "catchEChirho")
    });
    assert!(decl_chirho.is_some(), "should lower infix funbind name");
    if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
        assert_eq!(
            matches_chirho[0].pats_chirho.len(),
            2,
            "infix funbind should lower both lhs operands as patterns"
        );
    }
}

#[test]
fn lower_infix_fun_bind_with_parenthesized_constructor_pats_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nnewtype ChanChirho pChirho aChirho = ChanChirho aChirho\naddChirho :: Num aChirho => ChanChirho pChirho aChirho -> ChanChirho pChirho aChirho -> ChanChirho pChirho aChirho\n(ChanChirho aChirho) `addChirho` (ChanChirho bChirho) = ChanChirho (aChirho + bChirho)\n",
    );
    let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
        matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "addChirho")
    });
    assert!(
        decl_chirho.is_some(),
        "should lower constructor-pattern infix funbind name"
    );
    if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
        assert_eq!(
            matches_chirho[0].pats_chirho.len(),
            2,
            "constructor-pattern infix funbind should keep both lhs operands as patterns"
        );
    }
}

#[test]
fn lower_symbolic_infix_fun_bind_with_var_operands_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\ninfixr 0 ~:\n(~:) :: [Char] -> Int -> ([Char], Int)\nlabelChirho ~: valueChirho = (labelChirho, valueChirho)\n",
    );
    let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
        matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
            if name_chirho.text_chirho() == "~:")
    });
    assert!(
        decl_chirho.is_some(),
        "should lower symbolic infix funbind name"
    );
    if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
        assert_eq!(matches_chirho.len(), 1);
        assert_eq!(
            matches_chirho[0].pats_chirho.len(),
            2,
            "symbolic infix funbind should keep both lhs operands as patterns"
        );
    }
}

#[test]
fn lower_where_local_sig_preserves_qualified_tycon_spine_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nimport qualified Control.Monad.Trans.State.Lazy as LazyS\nf a = g a where\n  q :: (m (a, s) -> m (a, s)) -> LazyS.StateT s m a -> LazyS.StateT s m a\n  q u (LazyS.StateT b) = LazyS.StateT (u . b)\n  g x = x\n",
    );
    let fun_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "f"
            )
        })
        .expect("expected top-level f binding");
    let where_binds_chirho = match fun_decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => &matches_chirho[0].where_binds_chirho,
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };
    let q_sig_chirho = where_binds_chirho
        .iter()
        .find_map(|bind_chirho| match bind_chirho {
            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } if name_chirho.text_chirho() == "q" => Some(ty_chirho),
            _ => None,
        })
        .expect("expected local type signature for q");

    let arrow_arg_chirho = match q_sig_chirho {
        TypeChirho::FunChirho { result_chirho, .. } => match result_chirho.as_ref() {
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                assert!(
                    matches!(result_chirho.as_ref(), TypeChirho::AppChirho { .. }),
                    "final result should stay a fully applied qualified tycon, got {:?}",
                    result_chirho
                );
                arg_chirho.as_ref()
            }
            other_chirho => panic!(
                "expected second arrow for q signature, got {:?}",
                other_chirho
            ),
        },
        other_chirho => panic!(
            "expected function type for q signature, got {:?}",
            other_chirho
        ),
    };

    match arrow_arg_chirho {
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            assert!(
                matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "a"),
                "qualified tycon should keep final type argument `a`, got {:?}",
                arg_chirho
            );
            match fun_chirho.as_ref() {
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    assert!(
                        matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "m"),
                        "qualified tycon should keep middle type argument `m`, got {:?}",
                        arg_chirho
                    );
                    match fun_chirho.as_ref() {
                        TypeChirho::AppChirho {
                            fun_chirho,
                            arg_chirho,
                            ..
                        } => {
                            assert!(
                                matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "s"),
                                "qualified tycon should keep first type argument `s`, got {:?}",
                                arg_chirho
                            );
                            assert!(
                                matches!(fun_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.full_name_chirho() == "LazyS.StateT"),
                                "qualified constructor head should stay LazyS.StateT, got {:?}",
                                fun_chirho
                            );
                        }
                        other_chirho => panic!(
                            "expected left-nested type application for LazyS.StateT s m a, got {:?}",
                            other_chirho
                        ),
                    }
                }
                other_chirho => panic!(
                    "expected second application layer for LazyS.StateT s m a, got {:?}",
                    other_chirho
                ),
            }
        }
        other_chirho => panic!(
            "expected qualified type constructor application in q signature, got {:?}",
            other_chirho
        ),
    }
}

#[test]
fn lower_nested_as_pattern_vars_remain_visible_to_where_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module Main where\ndata BitQueueB = BQB Int Int\nnewtype BitQueue = BQ BitQueueB\nunconsQ (BQ bq@(BQB _ lo)) = Just (hd, BQ tl)\n  where\n    hd = lo == 0\n    tl = bq\n",
    );
    let fun_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "unconsQ"
            )
        })
        .expect("expected unconsQ fun bind");

    let DeclChirho::FunBindChirho { matches_chirho, .. } = fun_decl_chirho else {
        panic!("expected function binding");
    };
    assert_eq!(matches_chirho.len(), 1);
    assert_eq!(matches_chirho[0].where_binds_chirho.len(), 2);

    let PatChirho::ParenChirho { inner_chirho, .. } = &matches_chirho[0].pats_chirho[0] else {
        panic!(
            "expected parenthesized outer pattern, got {:?}",
            matches_chirho[0].pats_chirho[0]
        );
    };
    let PatChirho::ConChirho { args_chirho, .. } = inner_chirho.as_ref() else {
        panic!("expected outer constructor pattern, got {:?}", inner_chirho);
    };
    let PatChirho::AsChirho {
        name_chirho,
        pattern_chirho,
        ..
    } = &args_chirho[0]
    else {
        panic!("expected nested as pattern");
    };
    assert_eq!(name_chirho.text_chirho(), "bq");

    let inner_pattern_chirho = match pattern_chirho.as_ref() {
        PatChirho::ParenChirho { inner_chirho, .. } => inner_chirho.as_ref(),
        other_chirho => other_chirho,
    };
    let PatChirho::ConChirho {
        args_chirho: inner_args_chirho,
        ..
    } = inner_pattern_chirho
    else {
        panic!(
            "expected inner constructor pattern, got {:?}",
            inner_pattern_chirho
        );
    };
    assert_eq!(inner_args_chirho.len(), 2);
    assert!(matches!(
        &inner_args_chirho[0],
        PatChirho::WildcardChirho(_)
    ));
    assert!(matches!(
        &inner_args_chirho[1],
        PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "lo"
    ));
}

#[test]
fn lower_prefix_fun_bind_with_as_and_lazy_parameter_pattern_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module Main where\ndata NonEmptyChirho aChirho = aChirho :| [aChirho]\nextendedChirho fChirho wChirho@(~(_ :| aasChirho)) = fChirho wChirho :| aasChirho\n",
    );
    let fun_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "extendedChirho"
            )
        })
        .expect("expected extendedChirho fun bind");

    let DeclChirho::FunBindChirho { matches_chirho, .. } = fun_decl_chirho else {
        panic!("expected function binding");
    };
    assert_eq!(matches_chirho.len(), 1);
    assert_eq!(matches_chirho[0].pats_chirho.len(), 2);
    assert!(matches!(
        &matches_chirho[0].pats_chirho[0],
        PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "fChirho"
    ));
    match &matches_chirho[0].pats_chirho[1] {
        PatChirho::AsChirho {
            name_chirho,
            pattern_chirho,
            ..
        } => {
            assert_eq!(name_chirho.text_chirho(), "wChirho");
            let inner_pattern_chirho = match pattern_chirho.as_ref() {
                PatChirho::ParenChirho { inner_chirho, .. } => inner_chirho.as_ref(),
                other_chirho => other_chirho,
            };
            assert!(
                matches!(inner_pattern_chirho, PatChirho::LazyChirho { .. }),
                "expected lazy pattern inside as-pattern, got {:?}",
                inner_pattern_chirho
            );
        }
        other_chirho => panic!("expected as-pattern parameter, got {:?}", other_chirho),
    }
}

#[test]
fn lower_where_pattern_bind_with_infix_constructor_and_primed_names_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module Main where\ndata ComplexChirho aChirho = aChirho :+ aChirho\nclass BindChirho mChirho where\n  bindChirho :: mChirho aChirho -> (aChirho -> mChirho bChirho) -> mChirho bChirho\ninstance BindChirho ComplexChirho where\n  bindChirho (aChirho :+ bChirho) fChirho = aPrimeChirho :+ bPrimeChirho where\n    aPrimeChirho :+ _ = fChirho aChirho\n    _ :+ bPrimeChirho = fChirho bChirho\n",
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
    assert!(
        matches_chirho[0].where_binds_chirho.iter().any(|bind_chirho| {
            matches!(
                bind_chirho,
                LocalBindChirho::PatBindChirho { pat_chirho, .. }
                    if matches!(
                        pat_chirho,
                        PatChirho::InfixConChirho { left_chirho, op_chirho, .. }
                            if matches!(left_chirho.as_ref(), PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "aPrimeChirho")
                                && op_chirho.text_chirho() == ":+"
                    )
            )
        }),
        "expected aPrimeChirho where binding to stay a pattern binding with infix constructor, got {:?}",
        matches_chirho[0].where_binds_chirho
    );
    assert!(
        matches_chirho[0].where_binds_chirho.iter().any(|bind_chirho| {
            matches!(
                bind_chirho,
                LocalBindChirho::PatBindChirho { pat_chirho, .. }
                    if matches!(
                        pat_chirho,
                        PatChirho::InfixConChirho { right_chirho, op_chirho, .. }
                            if matches!(right_chirho.as_ref(), PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "bPrimeChirho")
                                && op_chirho.text_chirho() == ":+"
                    )
            )
        }),
        "expected bPrimeChirho where binding to stay a pattern binding with infix constructor, got {:?}",
        matches_chirho[0].where_binds_chirho
    );
}

#[test]
fn lower_containers_intset_retains_helper_funbinds_chirho() {
    let source_chirho = crate::test_fixtures_chirho::INTSET_RAW_SOURCE_CHIRHO;
    let module_chirho = parse_and_lower_chirho(source_chirho);
    let funbind_names_chirho: Vec<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect();
    for required_name_chirho in [
        "withBar",
        "withEmpty",
        "bin",
        "tip",
        "prefixOf",
        "lowestBitSet",
        "highestBitSet",
        "takeWhileAntitoneBits",
    ] {
        assert!(
            funbind_names_chirho
                .iter()
                .any(|name_chirho| name_chirho == required_name_chirho),
            "expected helper `{}` in lowered IntSet module; sample={:?}",
            required_name_chirho,
            funbind_names_chirho
                .iter()
                .rev()
                .take(20)
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn lower_containers_intset_preprocessed_retains_helper_funbinds_chirho() {
    let source_chirho = crate::test_fixtures_chirho::INTSET_PREPROCESSED_SOURCE_CHIRHO;
    let module_chirho = parse_and_lower_chirho(source_chirho);
    let funbind_names_chirho: Vec<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect();
    let decl_labels_chirho: Vec<String> = module_chirho
        .decls_chirho
        .iter()
        .map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                format!("fun:{}", name_chirho.text_chirho())
            }
            DeclChirho::PatBindChirho { .. } => "pat".to_string(),
            DeclChirho::TypeSigChirho { name_chirho, .. } => {
                format!("sig:{}", name_chirho.text_chirho())
            }
            DeclChirho::DataDeclChirho { name_chirho, .. } => {
                format!("data:{}", name_chirho.text_chirho())
            }
            DeclChirho::ClassDeclChirho { name_chirho, .. } => {
                format!("class:{}", name_chirho.text_chirho())
            }
            DeclChirho::InstanceDeclChirho { .. } => "instance".to_string(),
            other_chirho => format!("{:?}", other_chirho),
        })
        .collect();
    for required_name_chirho in [
        "withBar",
        "withEmpty",
        "bin",
        "tip",
        "prefixOf",
        "lowestBitSet",
        "highestBitSet",
        "takeWhileAntitoneBits",
    ] {
        assert!(
            funbind_names_chirho
                .iter()
                .any(|name_chirho| name_chirho == required_name_chirho),
            "expected helper `{}` in lowered preprocessed IntSet module; sample={:?}; decl_tail={:?}",
            required_name_chirho,
            funbind_names_chirho
                .iter()
                .rev()
                .take(20)
                .collect::<Vec<_>>(),
            decl_labels_chirho.iter().rev().take(40).collect::<Vec<_>>()
        );
    }
}

#[test]
fn lower_top_level_funbind_with_multiple_bang_patterns_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nsymDiffTip !t1 !kx1 !bm1 = go\n  where\n    go = t1\n",
    );
    let fun_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "symDiffTip"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "expected symDiffTip funbind; decls={:?}",
                module_chirho
                    .decls_chirho
                    .iter()
                    .map(|decl_chirho| match decl_chirho {
                        DeclChirho::FunBindChirho { name_chirho, .. } => {
                            format!("fun:{}", name_chirho.text_chirho())
                        }
                        DeclChirho::PatBindChirho { .. } => "pat".to_string(),
                        DeclChirho::TypeSigChirho { name_chirho, .. } => {
                            format!("sig:{}", name_chirho.text_chirho())
                        }
                        other_chirho => format!("{:?}", other_chirho),
                    })
                    .collect::<Vec<_>>()
            )
        });
    let DeclChirho::FunBindChirho {
        matches_chirho,
        name_chirho,
        ..
    } = fun_decl_chirho
    else {
        unreachable!("expected symDiffTip funbind");
    };
    assert_eq!(name_chirho.text_chirho(), "symDiffTip");
    assert_eq!(matches_chirho.len(), 1);
    assert_eq!(matches_chirho[0].pats_chirho.len(), 3);
}

#[test]
fn lower_parenthesized_operator_function_binding_name_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\n\
             (#.) _ xChirho = xChirho\n",
    );

    let fun_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::FunBindChirho { .. }))
        .unwrap_or_else(|| panic!("expected operator funbind"));

    let DeclChirho::FunBindChirho {
        name_chirho,
        matches_chirho,
        ..
    } = fun_decl_chirho
    else {
        panic!("expected operator funbind");
    };

    assert_eq!(name_chirho.text_chirho(), "#.");
    assert_eq!(matches_chirho.len(), 1);
    assert_eq!(matches_chirho[0].pats_chirho.len(), 2);
}

#[test]
fn lower_single_monadzip_instance_method_does_not_leak_top_level_funbind_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nimport Control.Applicative\nimport Control.Monad.Zip (MonadZip(mzipWith))\nnewtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }\ninstance MonadZip m => MonadZip (ExceptTChirho e m) where\n  mzipWith fChirho (ExceptTChirho aChirho) (ExceptTChirho bChirho) = ExceptTChirho $ mzipWith (liftA2 fChirho) aChirho bChirho\n",
    );
    let leaked_funbinds_chirho: Vec<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect();
    assert!(
        leaked_funbinds_chirho.is_empty(),
        "single MonadZip instance source should not leak top-level funbinds: {:?}",
        leaked_funbinds_chirho
    );
}

#[test]
fn instance_where_followed_by_top_level_helper_stays_top_level_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where
class C f where
  m :: Int
instance C Maybe where
  m = readsData

-- helper after the instance
readsData :: Int
readsData = 1
",
    );

    let helper_sig_count_chirho = module_chirho
        .decls_chirho
        .iter()
        .filter(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "readsData")
        })
        .count();
    let helper_fun_count_chirho = module_chirho
        .decls_chirho
        .iter()
        .filter(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. } if name_chirho.text_chirho() == "readsData")
        })
        .count();
    let helper_inside_instance_chirho = module_chirho.decls_chirho.iter().any(|decl_chirho| {
        matches!(
            decl_chirho,
            DeclChirho::InstanceDeclChirho { methods_chirho, .. }
                if methods_chirho.iter().any(|bind_chirho| matches!(
                    bind_chirho,
                    LocalBindChirho::FunBindChirho { name_chirho, .. }
                        if name_chirho.text_chirho() == "readsData"
                ))
        )
    });

    assert_eq!(
        helper_sig_count_chirho, 1,
        "helper signature should stay top-level"
    );
    assert_eq!(
        helper_fun_count_chirho, 1,
        "helper binding should stay top-level"
    );
    assert!(
        !helper_inside_instance_chirho,
        "helper binding should not be absorbed into the instance where-block"
    );
}

#[test]
fn lower_type_abstraction_application_stays_inside_equation_chirho() {
    for (case_name_chirho, rhs_chirho) in [
        ("name_chirho", "wrapChirho"),
        ("required_arg_chirho", "wrapChirho [Num, Show]"),
        ("visible_arg_chirho", "wrapChirho [Num, Show] @Double"),
        ("term_arg_chirho", "wrapChirho [Num, Show] @Double 4.15"),
        (
            "type_lambda_chirho",
            "wrapChirho [Num, Show] @Double 4.15 \\ @valueChirho -> functionChirho",
        ),
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE TypeAbstractions #-}}\n\
module M where\n\
chooseChirho :: Bool -> forall resultChirho. (forall valueChirho. valueChirho -> resultChirho) -> resultChirho\n\
chooseChirho True functionChirho = {rhs_chirho}\n\
chooseChirho False functionChirho = functionChirho @Int 14\n"
        );
        let module_chirho = parse_and_lower_chirho(&source_chirho);

        assert!(
            module_chirho.decls_chirho.iter().any(|decl_chirho| {
                matches!(
                    decl_chirho,
                    DeclChirho::TypeSigChirho { name_chirho, .. }
                        if name_chirho.text_chirho() == "chooseChirho"
                )
            }),
            "{case_name_chirho} crossed the declaration boundary: {:?}",
            module_chirho.decls_chirho
        );
    }
}

#[test]
fn lower_where_strict_tuple_pattern_binding_stays_tuple_pattern_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module MChirho where\nfChirho sChirho = r0Chirho where\n  (!q0Chirho, !r0Chirho) = sChirho `divMod` 10\n  (!q1Chirho, !r1Chirho) = q0Chirho `divMod` 10\n  !r2Chirho = q1Chirho `mod` 10\n",
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
    let strict_tuple_bind_chirho = matches_chirho[0]
        .where_binds_chirho
        .iter()
        .find(|bind_chirho| {
            matches!(
                bind_chirho,
                LocalBindChirho::PatBindChirho {
                    pat_chirho:
                        PatChirho::TupleChirho {
                            elements_chirho,
                            ..
                        },
                    ..
                } if elements_chirho.len() == 2
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "expected strict tuple pattern binding in where clause, got {:?}",
                matches_chirho[0].where_binds_chirho
            )
        });
    let LocalBindChirho::PatBindChirho { pat_chirho, .. } = strict_tuple_bind_chirho else {
        panic!("expected pattern binding");
    };
    let PatChirho::TupleChirho {
        elements_chirho, ..
    } = pat_chirho
    else {
        panic!("expected tuple pattern, got {:?}", pat_chirho);
    };
    assert!(
        matches!(
            &elements_chirho[0],
            PatChirho::BangChirho { inner_chirho, .. }
                if matches!(
                    inner_chirho.as_ref(),
                    PatChirho::VarChirho(name_chirho)
                        if name_chirho.text_chirho() == "q0Chirho"
                )
        ),
        "first tuple element should be strict q0Chirho, got {:?}",
        elements_chirho
    );
    assert!(
        matches!(
            &elements_chirho[1],
            PatChirho::BangChirho { inner_chirho, .. }
                if matches!(
                    inner_chirho.as_ref(),
                    PatChirho::VarChirho(name_chirho)
                        if name_chirho.text_chirho() == "r0Chirho"
                )
        ),
        "second tuple element should be strict r0Chirho, got {:?}",
        elements_chirho
    );
}
