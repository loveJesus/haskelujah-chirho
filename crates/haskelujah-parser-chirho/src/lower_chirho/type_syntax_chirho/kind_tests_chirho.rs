// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: Kinds, promoted types and kind-annotated binders.
//! Moved from `lower_chirho.rs`'s tests in row 641.

use haskelujah_ast_chirho::decl_chirho::{DataKindSigChirho, DeclChirho};
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

use super::test_support_chirho::{parse_and_lower_chirho, type_shape_chirho};

#[test]
fn lower_kind_sig_arrow_kind_chirho() {
    // data HKD (f :: * -> *) = MkHKD
    let module_chirho = parse_and_lower_chirho("module M where\ndata HKD (f :: * -> *) = MkHKD\n");
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            type_vars_chirho, ..
        } => {
            assert_eq!(type_vars_chirho.len(), 1);
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "f");
            assert_eq!(
                type_vars_chirho[0]
                    .kind_annotation_chirho
                    .as_deref()
                    .map(type_shape_chirho),
                Some("(* -> *)".to_string())
            );
        }
        other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_kind_sig_mixed_chirho() {
    // data Pair (a :: *) b = MkPair a b — mixed annotated and plain
    let module_chirho =
        parse_and_lower_chirho("module M where\ndata Pair (a :: *) b = MkPair a b\n");
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            type_vars_chirho, ..
        } => {
            assert_eq!(type_vars_chirho.len(), 2);
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "a");
            assert!(type_vars_chirho[0].kind_annotation_chirho.is_some());
            assert_eq!(type_vars_chirho[1].name_chirho.text_chirho(), "b");
            assert!(type_vars_chirho[1].kind_annotation_chirho.is_none());
        }
        other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_kind_sig_no_annotation_unchanged_chirho() {
    // data Maybe a = Nothing | Just a — no kind annotation, should be None
    let module_chirho = parse_and_lower_chirho("module M where\ndata Maybe a = Nothing | Just a\n");
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            type_vars_chirho, ..
        } => {
            assert_eq!(type_vars_chirho.len(), 1);
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "a");
            assert!(type_vars_chirho[0].kind_annotation_chirho.is_none());
        }
        other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_wildcard_head_binder_is_bound_not_a_declaration_kind_chirho() {
    // `data Foo (_ :: Constraint)` is ACCEPTED by GHC (the Constraint is the
    // binder's kind); `data Foo :: Constraint` is REJECTED (GHC-55233). They must
    // not lower to the same shape — that identity is what made the reject-axis
    // check unwritable, see spec-chirho/bug-data-binder-kind-misassigned-chirho.md.
    let binder_chirho = parse_and_lower_chirho("module M where\ndata Foo (_ :: Constraint)\n");
    match &binder_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            type_vars_chirho,
            kind_sig_chirho,
            ..
        } => {
            assert_eq!(type_vars_chirho.len(), 1, "`_` is a binder like any other");
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "_");
            assert_eq!(
                type_vars_chirho[0]
                    .kind_annotation_chirho
                    .as_deref()
                    .map(type_shape_chirho),
                Some("Constraint".to_string())
            );
            assert!(kind_sig_chirho.is_none());
        }
        other_chirho => panic!("expected DataDecl, got {other_chirho:?}"),
    }

    let decl_kind_chirho = parse_and_lower_chirho("module M where\ndata Foo :: Constraint\n");
    match &decl_kind_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            type_vars_chirho,
            kind_sig_chirho,
            ..
        } => {
            assert!(type_vars_chirho.is_empty());
            assert!(
                kind_sig_chirho.is_some(),
                "a real standalone kind signature still lands in kind_sig_chirho"
            );
        }
        other_chirho => panic!("expected DataDecl, got {other_chirho:?}"),
    }
}

#[test]
fn lower_standalone_forall_kind_sig_keeps_binder_scope_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE PolyKinds #-}\n\
{-# LANGUAGE StandaloneKindSignatures #-}\n\
module M where\n\
import Data.Kind\n\
data AppChirho :: forall (fChirho :: Type -> Type). Type -> Type where\n",
    );
    let data_decl_chirho = &module_chirho.decls_chirho[0];
    let DeclChirho::DataDeclChirho {
        kind_sig_chirho, ..
    } = data_decl_chirho
    else {
        panic!("expected DataDecl, got {:?}", data_decl_chirho);
    };
    let kind_sig_chirho = kind_sig_chirho
        .as_ref()
        .and_then(DataKindSigChirho::result_chirho)
        .expect("inline forall result kind signature should lower");
    assert_eq!(
        type_shape_chirho(kind_sig_chirho),
        "forall fChirho. (Type -> Type)",
        "the forall scopes over the whole result kind, and its binder is not a kind argument: {:?}",
        kind_sig_chirho
    );
}

#[test]
fn lower_visible_type_application_in_type_spine_retains_kind_arg_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE TypeApplications #-}\n\
module M where\n\
type VisibleChirho fChirho aChirho = AppChirho @fChirho aChirho\n",
    );
    let alias_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "VisibleChirho"
            )
        })
        .expect("expected VisibleChirho type alias");
    let DeclChirho::TypeAliasDeclChirho { rhs_chirho, .. } = alias_chirho else {
        panic!("expected VisibleChirho type alias");
    };

    assert_eq!(
        type_shape_chirho(rhs_chirho),
        "((AppChirho @fChirho) aChirho)",
        "visible @fChirho must remain a separate kind application: {:?}",
        rhs_chirho
    );
}

#[test]
fn lower_kind_annotated_type_lambda_preserves_binder_name_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE KindSignatures #-}\n\
{-# LANGUAGE TypeAbstractions #-}\n\
module M where\n\
valueChirho = \\ @(typeChirho :: Type) -> undefined\n",
    );
    let binder_name_chirho =
        module_chirho
            .decls_chirho
            .iter()
            .find_map(|decl_chirho| match decl_chirho {
                DeclChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    ..
                } if name_chirho.text_chirho() == "valueChirho" => {
                    matches_chirho
                        .first()
                        .and_then(|match_chirho| match &match_chirho.rhs_chirho {
                            RhsChirho::UnguardedChirho(ExprChirho::TypeLamChirho {
                                binder_chirho,
                                ..
                            }) => Some(binder_chirho.text_chirho().to_string()),
                            _ => None,
                        })
                }
                _ => None,
            });

    assert_eq!(binder_name_chirho.as_deref(), Some("typeChirho"));
}

#[test]
fn lower_type_alias_rhs_kind_annotation_keeps_full_function_rhs_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE TypeAbstractions #-}\n\
{-# LANGUAGE TypeFamilies #-}\n\
{-# LANGUAGE CUSKs #-}\n\
module M where\n\
import Data.Kind\n\
import Data.Proxy\n\
type S @(k :: Type) (a :: k) = Proxy a -> Proxy k :: Type\n",
    );
    let alias_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "S"
            )
        })
        .expect("expected S type alias");
    let DeclChirho::TypeAliasDeclChirho { rhs_chirho, .. } = alias_chirho else {
        panic!("expected type alias declaration");
    };

    assert_eq!(
        type_shape_chirho(rhs_chirho),
        "(((Proxy a) -> (Proxy k)) :: Type)",
        "RHS kind annotation should not drop the `k` argument: {:?}",
        rhs_chirho
    );
}

#[test]
fn lower_parenthesized_type_keeps_prefix_before_top_level_kind_annotation_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nimport Data.Kind (Type)\ntype HeteroEqPrefixChirho (a :: k1) = ((:~~:) (a :: k1) :: k2 -> Type)\n",
    );
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::TypeAliasDeclChirho { .. }))
        .expect("should have TypeAliasDecl");
    let DeclChirho::TypeAliasDeclChirho { rhs_chirho, .. } = decl_chirho else {
        panic!("expected type alias");
    };
    assert_eq!(
        type_shape_chirho(rhs_chirho),
        "((:~~: (a :: k1)) :: (k2 -> Type))",
        "both nested annotations retain their complete prefix and kind"
    );
}

#[test]
fn declaration_binder_kinds_keep_applied_tuple_and_list_types_chirho() {
    for (kind_chirho, shape_chirho) in [
        ("Maybe kChirho", "(Maybe kChirho)"),
        ("(Type, Type)", "(Type, Type)"),
        ("[kChirho]", "[kChirho]"),
        ("'Just kChirho", "('Just kChirho)"),
        ("'(Type, Type)", "(('(,) Type) Type)"),
        ("'[Type, Type]", "'[Type, Type]"),
        (
            "forall kChirho. kChirho -> Type",
            "forall kChirho. (kChirho -> Type)",
        ),
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE PolyKinds, DataKinds, RankNTypes #-}}\nmodule BinderKindsChirho where\ndata FChirho (aChirho :: {kind_chirho}) bChirho = FChirho\n"
        );
        let module_chirho = parse_and_lower_chirho(&source_chirho);
        let DeclChirho::DataDeclChirho {
            type_vars_chirho, ..
        } = &module_chirho.decls_chirho[0]
        else {
            panic!("expected data declaration");
        };
        assert_eq!(
            type_vars_chirho.len(),
            2,
            "the kind must not become declaration binders"
        );
        assert_eq!(
            type_vars_chirho[0]
                .kind_annotation_chirho
                .as_deref()
                .map(type_shape_chirho)
                .as_deref(),
            Some(shape_chirho),
            "kind {kind_chirho}"
        );
        assert_eq!(type_vars_chirho[1].name_chirho.text_chirho(), "bChirho");
    }
}

#[test]
fn forall_binder_kinds_keep_nominal_application_and_scope_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE PolyKinds, RankNTypes #-}\nmodule ForallKindChirho where\nvalueChirho :: forall (aChirho :: Maybe kChirho). Proxy aChirho -> Proxy aChirho\n",
    );
    let DeclChirho::TypeSigChirho { ty_chirho, .. } = &module_chirho.decls_chirho[0] else {
        panic!("expected signature");
    };
    let TypeChirho::ForallChirho { vars_chirho, .. } = ty_chirho else {
        panic!("expected forall");
    };
    assert_eq!(vars_chirho.len(), 1);
    assert_eq!(
        vars_chirho[0]
            .kind_annotation_chirho
            .as_deref()
            .map(type_shape_chirho)
            .as_deref(),
        Some("(Maybe kChirho)")
    );
}

#[test]
fn retained_kind_nodes_keep_their_written_source_ranges_chirho() {
    let source_chirho = "{-# LANGUAGE PolyKinds, TypeApplications #-}\nmodule KindRangesChirho where\ntype FChirho (aChirho :: Maybe kChirho) = Proxy @(Maybe kChirho) aChirho\n";
    let module_chirho = parse_and_lower_chirho(source_chirho);
    let DeclChirho::TypeAliasDeclChirho {
        type_vars_chirho,
        rhs_chirho,
        ..
    } = &module_chirho.decls_chirho[0]
    else {
        panic!("expected alias");
    };
    let binder_kind_chirho = type_vars_chirho[0]
        .kind_annotation_chirho
        .as_deref()
        .expect("retained binder kind");
    assert_eq!(
        binder_kind_chirho
            .span_chirho()
            .text_chirho(source_chirho)
            .map(str::trim),
        Some("Maybe kChirho")
    );
    let TypeChirho::AppChirho { fun_chirho, .. } = rhs_chirho else {
        panic!("ordinary argument remains outside hidden prefix");
    };
    let TypeChirho::KindApplicationChirho {
        arg_chirho,
        written_chirho,
        ..
    } = fun_chirho.as_ref()
    else {
        panic!("retained explicit kind application");
    };
    assert!(*written_chirho);
    assert_eq!(
        arg_chirho
            .span_chirho()
            .text_chirho(source_chirho)
            .map(str::trim),
        Some("(Maybe kChirho)")
    );
}

#[test]
fn a_kind_annotation_preserves_the_explicit_kind_prefix_to_its_left_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE TypeApplications #-}\nmodule AnnotatedPrefixChirho where\ntype XChirho = (FChirho @Type Int :: Maybe kChirho)\n",
    );
    let DeclChirho::TypeAliasDeclChirho { rhs_chirho, .. } = &module_chirho.decls_chirho[0] else {
        panic!("expected alias");
    };
    assert_eq!(
        type_shape_chirho(rhs_chirho),
        "(((FChirho @Type) Int) :: (Maybe kChirho))"
    );
}
