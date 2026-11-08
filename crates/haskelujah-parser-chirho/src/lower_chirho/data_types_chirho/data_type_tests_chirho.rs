// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: Data types, constructors, records and deriving.
//! Moved from `lower_chirho.rs`'s tests in row 641.

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DataKindSigChirho, DeclChirho, StrictnessChirho,
};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

use super::test_support_chirho::{
    collect_placeholder_expr_paths_in_decl_chirho, find_fun_decl_chirho, parse_and_lower_chirho,
    type_shape_chirho,
};

#[test]
fn lower_data_decl_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\ndata Color = Red | Green | Blue\n");
    assert_eq!(module_chirho.decls_chirho.len(), 1);
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            name_chirho,
            constructors_chirho,
            ..
        } => {
            assert_eq!(name_chirho.text_chirho(), "Color");
            assert_eq!(constructors_chirho.len(), 3);
        }
        other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_parsec_prim_state_record_decl_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module Text.Parsec.Prim where\n\
unknownErrorChirho :: StateChirho sChirho uChirho -> ParseErrorChirho\n\
unknownErrorChirho stateChirho = newErrorUnknownChirho (statePosChirho stateChirho)\n\
\n\
data StateChirho sChirho uChirho = StateChirho {\n\
      stateInputChirho :: sChirho,\n\
      statePosChirho   :: !SourcePosChirho,\n\
      stateUserChirho  :: !uChirho\n\
    }\n",
    );
    assert_eq!(module_chirho.decls_chirho.len(), 3);
    match &module_chirho.decls_chirho[2] {
        DeclChirho::DataDeclChirho {
            constructors_chirho,
            ..
        } => {
            assert_eq!(constructors_chirho.len(), 1);
            match &constructors_chirho[0] {
                ConDeclChirho::RecordChirho { fields_chirho, .. } => {
                    assert_eq!(fields_chirho.len(), 3);
                    let lowered_field_names_chirho: Vec<Vec<String>> = fields_chirho
                        .iter()
                        .map(|fd_chirho| {
                            fd_chirho
                                .names_chirho
                                .iter()
                                .map(|name_chirho| name_chirho.text_chirho().to_string())
                                .collect()
                        })
                        .collect();
                    assert_eq!(
                        lowered_field_names_chirho,
                        vec![
                            vec!["stateInputChirho".to_string()],
                            vec!["statePosChirho".to_string()],
                            vec!["stateUserChirho".to_string()]
                        ]
                    );
                }
                other_chirho => panic!("expected RecordChirho, got {:?}", other_chirho),
            }
        }
        other_chirho => panic!("expected DataDeclChirho, got {:?}", other_chirho),
    }
}

#[test]
fn lower_infix_data_constructor_with_strict_fields_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module Utils.Containers.Internal.StrictPair where\n\
data StrictPair a b = !a :*: !b\n",
    );
    assert_eq!(module_chirho.decls_chirho.len(), 1);
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            name_chirho,
            constructors_chirho,
            ..
        } => {
            assert_eq!(name_chirho.text_chirho(), "StrictPair");
            assert_eq!(constructors_chirho.len(), 1);
            match &constructors_chirho[0] {
                ConDeclChirho::OrdinaryChirho {
                    name_chirho,
                    fields_chirho,
                    ..
                } => {
                    assert_eq!(name_chirho.text_chirho(), ":*:");
                    assert_eq!(fields_chirho.len(), 2);
                    assert_eq!(fields_chirho[0].0, StrictnessChirho::StrictChirho);
                    assert_eq!(fields_chirho[1].0, StrictnessChirho::StrictChirho);
                }
                other_chirho => panic!(
                    "expected ordinary infix constructor, got {:?}",
                    other_chirho
                ),
            }
        }
        other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_infix_data_constructor_with_right_type_application_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module ViewLMiniChirho where\n\
data SeqChirho aChirho = SeqChirho\n\
data ViewLChirho aChirho = EmptyLChirho | aChirho :< SeqChirho aChirho\n",
    );
    match &module_chirho.decls_chirho[1] {
        DeclChirho::DataDeclChirho {
            constructors_chirho,
            ..
        } => match &constructors_chirho[1] {
            ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
                assert_eq!(fields_chirho.len(), 2, "{:?}", fields_chirho);
                assert!(
                    matches!(
                        &fields_chirho[1].1,
                        TypeChirho::AppChirho { fun_chirho, arg_chirho, .. }
                            if matches!(
                                fun_chirho.as_ref(),
                                TypeChirho::ConChirho(name_chirho)
                                    if name_chirho.text_chirho() == "SeqChirho"
                            ) && matches!(
                                arg_chirho.as_ref(),
                                TypeChirho::VarChirho(name_chirho)
                                    if name_chirho.text_chirho() == "aChirho"
                            )
                    ),
                    "expected right field SeqChirho aChirho, got {:?}",
                    fields_chirho[1].1
                );
            }
            other_chirho => panic!("expected ordinary constructor, got {:?}", other_chirho),
        },
        other_chirho => panic!(
            "expected second declaration to be data, got {:?}",
            other_chirho
        ),
    }
}

#[test]
fn lower_infix_data_constructor_with_left_type_application_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module ViewRMiniChirho where\n\
data SeqChirho aChirho = SeqChirho\n\
data ViewRChirho aChirho = EmptyRChirho | SeqChirho aChirho :> aChirho\n",
    );
    match &module_chirho.decls_chirho[1] {
        DeclChirho::DataDeclChirho {
            constructors_chirho,
            ..
        } => match &constructors_chirho[1] {
            ConDeclChirho::OrdinaryChirho {
                name_chirho,
                fields_chirho,
                ..
            } => {
                assert_eq!(name_chirho.text_chirho(), ":>");
                assert_eq!(fields_chirho.len(), 2, "{:?}", fields_chirho);
                assert!(
                    matches!(
                        &fields_chirho[0].1,
                        TypeChirho::AppChirho { fun_chirho, arg_chirho, .. }
                            if matches!(
                                fun_chirho.as_ref(),
                                TypeChirho::ConChirho(name_chirho)
                                    if name_chirho.text_chirho() == "SeqChirho"
                            ) && matches!(
                                arg_chirho.as_ref(),
                                TypeChirho::VarChirho(name_chirho)
                                    if name_chirho.text_chirho() == "aChirho"
                            )
                    ),
                    "expected left field SeqChirho aChirho, got {:?}",
                    fields_chirho[0].1
                );
                assert!(
                    matches!(
                        &fields_chirho[1].1,
                        TypeChirho::VarChirho(name_chirho)
                            if name_chirho.text_chirho() == "aChirho"
                    ),
                    "expected right field aChirho, got {:?}",
                    fields_chirho[1].1
                );
            }
            other_chirho => panic!("expected ordinary constructor, got {:?}", other_chirho),
        },
        other_chirho => panic!(
            "expected second declaration to be data, got {:?}",
            other_chirho
        ),
    }
}

// ── KindSignatures parsing tests ────────────────────────────────────

#[test]
fn lower_kind_sig_data_star_chirho() {
    // data Proxy (a :: *) = MkProxy
    let module_chirho = parse_and_lower_chirho("module M where\ndata Proxy (a :: *) = MkProxy\n");
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            name_chirho,
            type_vars_chirho,
            constructors_chirho,
            ..
        } => {
            assert_eq!(name_chirho.text_chirho(), "Proxy");
            assert_eq!(type_vars_chirho.len(), 1);
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "a");
            assert_eq!(
                type_vars_chirho[0]
                    .kind_annotation_chirho
                    .as_deref()
                    .map(type_shape_chirho),
                Some("*".to_string())
            );
            assert_eq!(constructors_chirho.len(), 1);
        }
        other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_kind_sig_newtype_chirho() {
    // newtype Id (a :: *) = MkId a
    let module_chirho = parse_and_lower_chirho("module M where\nnewtype Id (a :: *) = MkId a\n");
    match &module_chirho.decls_chirho[0] {
        DeclChirho::NewtypeDeclChirho {
            type_vars_chirho, ..
        } => {
            assert_eq!(type_vars_chirho.len(), 1);
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "a");
            assert_eq!(
                type_vars_chirho[0]
                    .kind_annotation_chirho
                    .as_deref()
                    .map(type_shape_chirho),
                Some("*".to_string())
            );
        }
        other_chirho => panic!("expected NewtypeDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_kind_sig_newtype_preserves_type_application_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nnewtype CodeChirho mChirho (aChirho :: TYPE rChirho) = CodeChirho (mChirho aChirho)\n",
    );
    match &module_chirho.decls_chirho[0] {
        DeclChirho::NewtypeDeclChirho {
            type_vars_chirho, ..
        } => {
            assert_eq!(type_vars_chirho.len(), 2);
            assert_eq!(type_vars_chirho[1].name_chirho.text_chirho(), "aChirho");
            assert_eq!(
                type_vars_chirho[1]
                    .kind_annotation_chirho
                    .as_deref()
                    .map(type_shape_chirho),
                Some("(TYPE rChirho)".to_string())
            );
        }
        other_chirho => panic!("expected NewtypeDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_data_decl_with_operator_type_name_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\ndata (::+:) f g a = InLChirho (f a) | InRChirho (g a)\n",
    );
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho { name_chirho, .. } => {
            assert_eq!(name_chirho.text_chirho(), "::+:");
        }
        other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_newtype_decl_with_operator_type_name_chirho() {
    let module_chirho =
        parse_and_lower_chirho("module M where\nnewtype (::*:) f g a = ProdChirho (f a, g a)\n");
    match &module_chirho.decls_chirho[0] {
        DeclChirho::NewtypeDeclChirho { name_chirho, .. } => {
            assert_eq!(name_chirho.text_chirho(), "::*:");
        }
        other_chirho => panic!("expected NewtypeDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_newtype_record_tuple_type_application_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nnewtype WriterTChirho wChirho mChirho aChirho = WriterTChirho { runWriterTChirho :: mChirho (aChirho, wChirho) }\n",
    );
    match &module_chirho.decls_chirho[0] {
        DeclChirho::NewtypeDeclChirho {
            constructor_chirho: ConDeclChirho::RecordChirho { fields_chirho, .. },
            ..
        } => {
            assert_eq!(fields_chirho.len(), 1);
            match &fields_chirho[0].ty_chirho {
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    assert!(
                        matches!(
                            fun_chirho.as_ref(),
                            TypeChirho::VarChirho(name_chirho)
                                if name_chirho.text_chirho() == "mChirho"
                        ),
                        "expected mChirho type constructor application head, got {:?}",
                        fun_chirho
                    );
                    match arg_chirho.as_ref() {
                        TypeChirho::TupleChirho {
                            elements_chirho, ..
                        } => {
                            assert_eq!(elements_chirho.len(), 2);
                            assert!(
                                matches!(
                                    &elements_chirho[0],
                                    TypeChirho::VarChirho(name_chirho)
                                        if name_chirho.text_chirho() == "aChirho"
                                ),
                                "expected first tuple element aChirho, got {:?}",
                                elements_chirho[0]
                            );
                            assert!(
                                matches!(
                                    &elements_chirho[1],
                                    TypeChirho::VarChirho(name_chirho)
                                        if name_chirho.text_chirho() == "wChirho"
                                ),
                                "expected second tuple element wChirho, got {:?}",
                                elements_chirho[1]
                            );
                        }
                        other_chirho => {
                            panic!("expected tuple type arg, got {:?}", other_chirho)
                        }
                    }
                }
                other_chirho => panic!("expected type application, got {:?}", other_chirho),
            }
        }
        other_chirho => panic!("expected record newtype, got {:?}", other_chirho),
    }
}

#[test]
fn lower_newtype_record_function_returning_tuple_application_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nnewtype StateTChirho sChirho mChirho aChirho = StateTChirho { runStateTChirho :: sChirho -> mChirho (aChirho, sChirho) }\n",
    );
    match &module_chirho.decls_chirho[0] {
        DeclChirho::NewtypeDeclChirho {
            constructor_chirho: ConDeclChirho::RecordChirho { fields_chirho, .. },
            ..
        } => {
            assert_eq!(fields_chirho.len(), 1);
            match &fields_chirho[0].ty_chirho {
                TypeChirho::FunChirho {
                    arg_chirho,
                    result_chirho,
                    ..
                } => {
                    assert!(
                        matches!(
                            arg_chirho.as_ref(),
                            TypeChirho::VarChirho(name_chirho)
                                if name_chirho.text_chirho() == "sChirho"
                        ),
                        "expected function arg type sChirho, got {:?}",
                        arg_chirho
                    );
                    match result_chirho.as_ref() {
                        TypeChirho::AppChirho {
                            fun_chirho,
                            arg_chirho,
                            ..
                        } => {
                            assert!(
                                matches!(
                                    fun_chirho.as_ref(),
                                    TypeChirho::VarChirho(name_chirho)
                                        if name_chirho.text_chirho() == "mChirho"
                                ),
                                "expected result head mChirho, got {:?}",
                                fun_chirho
                            );
                            match arg_chirho.as_ref() {
                                TypeChirho::TupleChirho {
                                    elements_chirho, ..
                                } => {
                                    assert_eq!(elements_chirho.len(), 2);
                                    assert!(
                                        matches!(
                                            &elements_chirho[0],
                                            TypeChirho::VarChirho(name_chirho)
                                                if name_chirho.text_chirho() == "aChirho"
                                        ),
                                        "expected first tuple element aChirho, got {:?}",
                                        elements_chirho[0]
                                    );
                                    assert!(
                                        matches!(
                                            &elements_chirho[1],
                                            TypeChirho::VarChirho(name_chirho)
                                                if name_chirho.text_chirho() == "sChirho"
                                        ),
                                        "expected second tuple element sChirho, got {:?}",
                                        elements_chirho[1]
                                    );
                                }
                                other_chirho => {
                                    panic!("expected tuple type arg, got {:?}", other_chirho)
                                }
                            }
                        }
                        other_chirho => {
                            panic!("expected result application, got {:?}", other_chirho)
                        }
                    }
                }
                other_chirho => panic!("expected function type, got {:?}", other_chirho),
            }
        }
        other_chirho => panic!("expected record newtype, got {:?}", other_chirho),
    }
}

#[test]
fn lower_data_binder_kind_never_becomes_the_declaration_kind_chirho() {
    // `[Symbol]` belongs to the binder. Its full syntax must survive there;
    // recording it as the declaration's return kind fabricated `A`'s kind.
    let module_chirho = parse_and_lower_chirho("module M where\ndata A (b :: [Symbol]) = A\n");
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            type_vars_chirho,
            kind_sig_chirho,
            ..
        } => {
            assert_eq!(type_vars_chirho.len(), 1);
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "b");
            assert_eq!(
                type_vars_chirho[0]
                    .kind_annotation_chirho
                    .as_deref()
                    .map(type_shape_chirho),
                Some("[Symbol]".to_string())
            );
            let Some(TypeChirho::ListChirho { element_chirho, .. }) =
                type_vars_chirho[0].kind_annotation_chirho.as_deref()
            else {
                panic!("expected the binder's retained list kind");
            };
            assert!(
                matches!(element_chirho.as_ref(), TypeChirho::ConChirho(name_chirho)
                if name_chirho.text_chirho() == "Symbol")
            );
            assert!(
                kind_sig_chirho.is_none(),
                "a binder's kind must not become the declaration's kind: {kind_sig_chirho:?}"
            );
        }
        other_chirho => panic!("expected DataDecl, got {other_chirho:?}"),
    }
}

#[test]
fn lower_data_binder_unreadable_kind_does_not_inflate_arity_chirho() {
    // Walking into the group bound the kind's OWN variables as head binders:
    // `[[a]]` added `a`, and `forall k. k -> Type` added `k`, so the
    // declaration silently gained an argument.
    for (src_chirho, binder_chirho) in [
        ("module M where\ndata SOP (xss :: [[a]])\n", "xss"),
        (
            "module M where\ndata S (p :: forall k. k -> Type) = S\n",
            "p",
        ),
    ] {
        let module_chirho = parse_and_lower_chirho(src_chirho);
        match &module_chirho.decls_chirho[0] {
            DeclChirho::DataDeclChirho {
                type_vars_chirho,
                kind_sig_chirho,
                ..
            } => {
                assert_eq!(
                    type_vars_chirho
                        .iter()
                        .map(|v_chirho| v_chirho.name_chirho.text_chirho().to_string())
                        .collect::<Vec<_>>(),
                    vec![binder_chirho.to_string()],
                    "only the binder itself is a head variable: {src_chirho:?}"
                );
                assert!(kind_sig_chirho.is_none(), "src: {src_chirho:?}");
            }
            other_chirho => panic!("expected DataDecl, got {other_chirho:?}"),
        }
    }
}

#[test]
fn gadt_constructor_double_colon_is_not_the_declaration_kind_chirho() {
    // `data T where` followed by `MkT :: { f :: Int, x :: Char } -> T`: the
    // constructor's `::` belongs to the constructor. Reading it as the
    // declaration's kind signature folded the brace fields into an application
    // (`Int Char`, hence a bogus E0300), and the field NAMES — `f` and `x` are
    // VarIds — were collected as head type parameters, giving `T` arity 2.
    // One field, or fields whose types are variables, hid it: the fold stayed
    // well-kinded and only the arity was silently wrong.
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE GADTs #-}\nmodule M where\ndata T where\n  MkT :: { f :: Int, x :: Char } -> T\n",
    );
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            type_vars_chirho,
            kind_sig_chirho,
            ..
        } => {
            assert!(
                type_vars_chirho.is_empty(),
                "record field names are not head binders: {type_vars_chirho:?}"
            );
            assert!(
                kind_sig_chirho.is_none(),
                "a constructor's `::` is not the declaration's kind: {kind_sig_chirho:?}"
            );
        }
        other_chirho => panic!("expected DataDecl, got {other_chirho:?}"),
    }
}

#[test]
fn lower_standalone_kind_sig_gadt_chirho() {
    // data V :: N -> Type where { VZ :: V Z }
    let module_chirho =
        parse_and_lower_chirho("module M where\ndata V :: N -> Type where\n  VZ :: V Z\n");
    let data_decl_chirho = &module_chirho.decls_chirho[0];
    match data_decl_chirho {
        DeclChirho::DataDeclChirho {
            name_chirho,
            kind_sig_chirho,
            constructors_chirho,
            ..
        } => {
            assert_eq!(name_chirho.text_chirho(), "V");
            assert!(
                kind_sig_chirho.is_some(),
                "standalone kind sig should be Some, got None. Decl: {:?}",
                data_decl_chirho
            );
            assert!(
                !constructors_chirho.is_empty(),
                "should have GADT constructors"
            );
        }
        other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_standalone_kind_sig_attaches_to_following_data_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE StandaloneKindSignatures #-}\n\
module M where\n\
type MyList :: * -> *\n\
data MyList a = Nil | Cons a (MyList a)\n",
    );
    assert!(
        module_chirho
            .decls_chirho
            .iter()
            .all(|decl_chirho| !matches!(
                decl_chirho,
                DeclChirho::TypeSigChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "type"
                        || name_chirho.text_chirho() == "MyList"
            )),
        "standalone kind sig should not lower as ordinary value signatures: {:?}",
        module_chirho.decls_chirho
    );
    let data_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::DataDeclChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "MyList"
            )
        })
        .expect("expected MyList data declaration");
    let DeclChirho::DataDeclChirho {
        kind_sig_chirho, ..
    } = data_decl_chirho
    else {
        panic!("expected DataDecl, got {:?}", data_decl_chirho);
    };
    let kind_sig_chirho = kind_sig_chirho
        .as_ref()
        .and_then(DataKindSigChirho::standalone_chirho)
        .expect("standalone kind sig should attach to data declaration");
    assert_eq!(type_shape_chirho(kind_sig_chirho), "(* -> *)");
}

#[test]
fn lower_flat_record_field_visible_type_application_retains_kind_arg_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE TypeApplications #-}\n\
module M where\n\
data BoxChirho fChirho aChirho = BoxChirho { unBoxChirho :: AppChirho @fChirho aChirho }\n",
    );
    let data_decl_chirho = &module_chirho.decls_chirho[0];
    let DeclChirho::DataDeclChirho {
        constructors_chirho,
        ..
    } = data_decl_chirho
    else {
        panic!("expected DataDecl, got {:?}", data_decl_chirho);
    };
    let ConDeclChirho::RecordChirho { fields_chirho, .. } = &constructors_chirho[0] else {
        panic!(
            "expected GADT constructor, got {:?}",
            constructors_chirho[0]
        );
    };

    assert_eq!(
        type_shape_chirho(&fields_chirho[0].ty_chirho),
        "((AppChirho @fChirho) aChirho)",
        "flat visible @fChirho must remain separate from the ordinary argument: {:?}",
        fields_chirho[0].ty_chirho
    );
    let TypeChirho::AppChirho {
        fun_chirho,
        arg_chirho,
        ..
    } = &fields_chirho[0].ty_chirho
    else {
        panic!("expected one ordinary field type argument");
    };
    assert!(
        matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho)
        if name_chirho.text_chirho() == "aChirho")
    );
    let TypeChirho::KindApplicationChirho {
        fun_chirho,
        arg_chirho,
        written_chirho,
        ..
    } = fun_chirho.as_ref()
    else {
        panic!("expected the written hidden kind prefix");
    };
    assert!(*written_chirho);
    assert!(
        matches!(fun_chirho.as_ref(), TypeChirho::ConChirho(name_chirho)
        if name_chirho.text_chirho() == "AppChirho")
    );
    assert!(
        matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho)
        if name_chirho.text_chirho() == "fChirho")
    );
}

#[test]
fn lower_gadt_infix_type_operator_signature_keeps_operator_apps_chirho() {
    fn count_named_type_apps_chirho(ty_chirho: &TypeChirho, needle_chirho: &str) -> usize {
        let current_count_chirho = match ty_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho: _,
                ..
            } => match fun_chirho.as_ref() {
                TypeChirho::AppChirho {
                    fun_chirho: nested_fun_chirho,
                    ..
                } if matches!(
                    nested_fun_chirho.as_ref(),
                    TypeChirho::ConChirho(name_chirho)
                        if name_chirho.text_chirho() == needle_chirho
                ) =>
                {
                    1
                }
                _ => 0,
            },
            _ => 0,
        };

        current_count_chirho
            + match ty_chirho {
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    count_named_type_apps_chirho(fun_chirho, needle_chirho)
                        + count_named_type_apps_chirho(arg_chirho, needle_chirho)
                }
                TypeChirho::FunChirho {
                    arg_chirho,
                    result_chirho,
                    ..
                } => {
                    count_named_type_apps_chirho(arg_chirho, needle_chirho)
                        + count_named_type_apps_chirho(result_chirho, needle_chirho)
                }
                TypeChirho::TupleChirho {
                    elements_chirho, ..
                } => elements_chirho
                    .iter()
                    .map(|elem_chirho| count_named_type_apps_chirho(elem_chirho, needle_chirho))
                    .sum(),
                TypeChirho::ListChirho { element_chirho, .. }
                | TypeChirho::ParenChirho {
                    inner_chirho: element_chirho,
                    ..
                } => count_named_type_apps_chirho(element_chirho, needle_chirho),
                TypeChirho::QualChirho { body_chirho, .. }
                | TypeChirho::ForallChirho { body_chirho, .. } => {
                    count_named_type_apps_chirho(body_chirho, needle_chirho)
                }
                TypeChirho::PromotedListChirho {
                    elements_chirho, ..
                } => elements_chirho
                    .iter()
                    .map(|elem_chirho| count_named_type_apps_chirho(elem_chirho, needle_chirho))
                    .sum(),
                _ => 0,
            }
    }

    let module_chirho = parse_and_lower_chirho(
        r#"{-# LANGUAGE GADTs, TypeOperators #-}
module M where
data aChirho :-> cChirho where
  PairChirho :: (aChirho :-> (bChirho :-> cChirho)) -> ((aChirho, bChirho) :-> cChirho)
  UnitChirho :: cChirho -> (() :-> cChirho)
"#,
    );

    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            name_chirho,
            type_vars_chirho,
            constructors_chirho,
            ..
        } => {
            assert_eq!(name_chirho.text_chirho(), ":->");
            assert_eq!(
                type_vars_chirho
                    .iter()
                    .map(|tv_chirho| tv_chirho.text_chirho().to_string())
                    .collect::<Vec<_>>(),
                vec!["aChirho".to_string(), "cChirho".to_string()]
            );
            assert!(
                !constructors_chirho.is_empty(),
                "expected GADT constructors for infix operator data decl, got {:?}",
                module_chirho.decls_chirho
            );
            match &constructors_chirho[0] {
                ConDeclChirho::GadtChirho {
                    signature_chirho, ..
                } => {
                    let ty_chirho = &signature_chirho.try_as_type_chirho(&mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho()).expect("small parser fixture fits projection budget");
                    assert!(
                        matches!(ty_chirho, TypeChirho::FunChirho { .. }),
                        "PairChirho signature should stay a function type, got {:?}",
                        ty_chirho
                    );
                    assert_eq!(
                        count_named_type_apps_chirho(ty_chirho, ":->"),
                        3,
                        "PairChirho signature should preserve three :-> applications, got {:?}",
                        ty_chirho
                    );
                }
                other_chirho => panic!(
                    "expected GADT constructor for PairChirho, got {:?}",
                    other_chirho
                ),
            }
        }
        other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_required_forall_record_field_preserves_quantifier_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE RequiredTypeArguments #-}\n\
module M where\n\
data HolderChirho = HolderChirho { fieldChirho :: forall typeChirho -> Proxy typeChirho }\n",
    );
    let field_type_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho {
                constructors_chirho,
                ..
            } => constructors_chirho.iter().find_map(|constructor_chirho| {
                let ConDeclChirho::RecordChirho { fields_chirho, .. } = constructor_chirho else {
                    return None;
                };
                fields_chirho
                    .first()
                    .map(|field_chirho| &field_chirho.ty_chirho)
            }),
            _ => None,
        })
        .expect("expected HolderChirho record field");

    assert!(
        matches!(
            field_type_chirho,
            TypeChirho::RequiredForallChirho { vars_chirho, .. }
                if vars_chirho.len() == 1
                    && vars_chirho[0].name_chirho.text_chirho() == "typeChirho"
        ),
        "expected record field required forall, got {field_type_chirho:?}"
    );
}

#[test]
fn lower_real_th_datatype_remaining_sites_have_no_placeholder_exprs_chirho() {
    let module_chirho =
        parse_and_lower_chirho(crate::test_fixtures_chirho::TH_DATATYPE_PREPROCESSED_SOURCE_CHIRHO);
    let mut placeholder_paths_chirho = Vec::new();
    for fun_name_chirho in ["mkExtraFunArgForalls", "freeVariablesWellScoped", "unify'"] {
        let decl_chirho = find_fun_decl_chirho(&module_chirho, fun_name_chirho);
        collect_placeholder_expr_paths_in_decl_chirho(
            decl_chirho,
            fun_name_chirho,
            &mut placeholder_paths_chirho,
        );
    }
    assert!(
        placeholder_paths_chirho.is_empty(),
        "real Datatype.hs should not lower placeholder expr vars in remaining failure sites: {:?}",
        placeholder_paths_chirho
    );
}

#[test]
fn lower_strict_data_fields_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\ndata Pair = MkPair !Int !Int\n");
    assert_eq!(module_chirho.decls_chirho.len(), 1);
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            constructors_chirho,
            ..
        } => {
            assert_eq!(constructors_chirho.len(), 1);
            match &constructors_chirho[0] {
                ConDeclChirho::OrdinaryChirho {
                    name_chirho,
                    fields_chirho,
                    ..
                } => {
                    assert_eq!(name_chirho.text_chirho(), "MkPair");
                    assert_eq!(fields_chirho.len(), 2);
                    assert_eq!(fields_chirho[0].0, StrictnessChirho::StrictChirho);
                    assert_eq!(fields_chirho[1].0, StrictnessChirho::StrictChirho);
                }
                _ => panic!("expected OrdinaryChirho constructor"),
            }
        }
        _ => panic!("expected DataDeclChirho"),
    }
}

#[test]
fn lower_mixed_strict_lazy_fields_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\ndata Mixed = MkMixed !Int Int\n");
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            constructors_chirho,
            ..
        } => match &constructors_chirho[0] {
            ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
                assert_eq!(fields_chirho.len(), 2);
                assert_eq!(fields_chirho[0].0, StrictnessChirho::StrictChirho);
                assert_eq!(fields_chirho[1].0, StrictnessChirho::LazyChirho);
            }
            _ => panic!("expected OrdinaryChirho"),
        },
        _ => panic!("expected DataDeclChirho"),
    }
}

#[test]
fn written_data_and_newtype_headers_carry_their_category_proof_chirho() {
    use haskelujah_ast_chirho::name_chirho::TypeConstructorRoleChirho;
    let module_chirho = parse_and_lower_chirho(
        "module DefChirho where\ndata DataChirho aChirho = DataChirho aChirho\nnewtype NewtypeChirho aChirho = NewtypeChirho aChirho\n",
    );
    let roles_chirho = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho { name_chirho, .. }
            | DeclChirho::NewtypeDeclChirho { name_chirho, .. } => Some((
                name_chirho.text_chirho(),
                name_chirho.written_type_declaration_role_chirho(),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        roles_chirho,
        vec![
            ("DataChirho", Some(TypeConstructorRoleChirho::DataChirho)),
            (
                "NewtypeChirho",
                Some(TypeConstructorRoleChirho::NewtypeChirho)
            )
        ]
    );
}

#[test]
fn skipped_data_instances_never_acquire_an_ordinary_data_role_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule DefChirho where\ndata family FChirho aChirho\ndata instance FChirho Int = MkFChirho Int\n",
    );
    for declaration_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::DataDeclChirho { name_chirho, .. } = declaration_chirho {
            assert_eq!(name_chirho.written_type_declaration_role_chirho(), None);
        }
    }
}

#[test]
fn unrepresented_newtype_context_heads_do_not_certify_the_context_class_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE DatatypeContexts #-}\nmodule DefChirho where\nnewtype Eq aChirho => NChirho aChirho = NChirho aChirho\n",
    );
    for declaration_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::NewtypeDeclChirho { name_chirho, .. } = declaration_chirho {
            assert_eq!(name_chirho.written_type_declaration_role_chirho(), None);
        }
    }
}

#[test]
fn lower_explicit_lazy_fields_keep_their_annotation_chirho() {
    // A leading `~` marks a positional, infix, record or newtype field explicitly
    // lazy and leaves the type without it; `!` and the default keep theirs. Before
    // the CST read `~`, a positional constructor stopped there (L01: no fields).
    let module_chirho = parse_and_lower_chirho(
        "module M where\n\
data PairChirho = PairChirho ~Int !Int Int\n\
data OpChirho = ~Int :&&: Int\n\
data BoxChirho = BoxChirho { lazyChirho :: ~(Maybe Int), plainChirho :: Int }\n\
newtype WrapChirho = WrapChirho ~Int\n",
    );
    let fields_chirho: Vec<Vec<(StrictnessChirho, String)>> = module_chirho
        .decls_chirho
        .iter()
        .map(|decl_chirho| {
            let constructor_chirho = match decl_chirho {
                DeclChirho::DataDeclChirho {
                    constructors_chirho,
                    ..
                } => &constructors_chirho[0],
                DeclChirho::NewtypeDeclChirho {
                    constructor_chirho, ..
                } => constructor_chirho,
                other_chirho => {
                    panic!("expected a data or newtype declaration, got {other_chirho:?}")
                }
            };
            match constructor_chirho {
                ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => fields_chirho
                    .iter()
                    .map(|(strictness_chirho, ty_chirho)| {
                        (*strictness_chirho, type_shape_chirho(ty_chirho))
                    })
                    .collect(),
                ConDeclChirho::RecordChirho { fields_chirho, .. } => fields_chirho
                    .iter()
                    .map(|field_chirho| {
                        (
                            field_chirho.strictness_chirho,
                            type_shape_chirho(&field_chirho.ty_chirho),
                        )
                    })
                    .collect(),
                other_chirho => panic!("expected a Haskell 98 constructor, got {other_chirho:?}"),
            }
        })
        .collect();
    let lazy_mark_chirho = StrictnessChirho::ExplicitLazyChirho;
    let default_chirho = StrictnessChirho::LazyChirho;
    let expected_chirho: Vec<Vec<(StrictnessChirho, String)>> = [
        vec![
            (lazy_mark_chirho, "Int"),
            (StrictnessChirho::StrictChirho, "Int"),
            (default_chirho, "Int"),
        ],
        vec![(lazy_mark_chirho, "Int"), (default_chirho, "Int")],
        vec![(lazy_mark_chirho, "(Maybe Int)"), (default_chirho, "Int")],
        vec![(lazy_mark_chirho, "Int")],
    ]
    .into_iter()
    .map(|row_chirho| {
        row_chirho
            .into_iter()
            .map(|(strictness_chirho, shape_chirho)| (strictness_chirho, shape_chirho.to_string()))
            .collect()
    })
    .collect();
    assert_eq!(fields_chirho, expected_chirho);
    // Only `!` and UNPACK make the constructor evaluate the field.
    assert_eq!(
        [
            default_chirho,
            StrictnessChirho::StrictChirho,
            StrictnessChirho::UnpackChirho,
            lazy_mark_chirho,
        ]
        .map(StrictnessChirho::is_strict_chirho),
        [false, true, true, false]
    );
}
