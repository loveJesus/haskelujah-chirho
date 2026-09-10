// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::{DataKindSigChirho, DeclChirho, FileIdChirho, TypeChirho, lower_module_chirho};
use crate::cst_parser_chirho::parse_to_cst_chirho;

#[test]
fn nominal_kind_heads_keep_their_qualification_and_source_span_chirho() {
    use super::AstKindChirho;
    for keyword_chirho in ["data", "newtype"] {
        let source_chirho = format!(
            "module MChirho where\n{keyword_chirho} BoxChirho (aChirho :: LibraryChirho.ApplyChirho kChirho) = BoxChirho Int\n"
        );
        let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
        let module_chirho = lower_module_chirho(
            &parse_to_cst_chirho(&source_chirho, file_chirho),
            file_chirho,
        );
        let binders_chirho = match &module_chirho.decls_chirho[0] {
            DeclChirho::DataDeclChirho {
                type_vars_chirho, ..
            }
            | DeclChirho::NewtypeDeclChirho {
                type_vars_chirho, ..
            } => type_vars_chirho,
            other_chirho => panic!("{other_chirho:?}"),
        };
        assert_eq!(
            binders_chirho.len(),
            1,
            "kind identifiers are not head binders"
        );
        let Some(AstKindChirho::AppChirho(head_chirho, argument_chirho)) =
            &binders_chirho[0].kind_annotation_chirho
        else {
            panic!("the complete application must survive")
        };
        let AstKindChirho::ConChirho(name_chirho) = head_chirho.as_ref() else {
            panic!("a nominal kind is not an implicitly quantified variable")
        };
        assert_eq!(name_chirho.full_name_chirho(), "LibraryChirho.ApplyChirho");
        let span_chirho = name_chirho.span_chirho();
        assert_eq!(
            &source_chirho[span_chirho.start_chirho().as_usize_chirho()
                ..span_chirho.end_chirho().as_usize_chirho()],
            "LibraryChirho.ApplyChirho"
        );
        assert_eq!(
            argument_chirho.as_ref(),
            &AstKindChirho::VarChirho("kChirho".into())
        );
    }
}

#[test]
fn invisible_head_binders_retain_visibility_scope_and_group_boundaries_chirho() {
    for keyword_chirho in ["data", "newtype"] {
        for binder_chirho in [
            "@jChirho",
            "@(jChirho :: Type)",
            "@(jChirho :: [Type])",
            "@_",
            "@(_ :: Type)",
        ] {
            let source_chirho = format!(
                "{{-# LANGUAGE TypeAbstractions, PolyKinds #-}}\nmodule BindersChirho where\n{keyword_chirho} BoxChirho {binder_chirho} aChirho = MkBoxChirho aChirho\nafterChirho :: MissingChirho\nafterChirho = ()\n"
            );
            let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
            let module_chirho = lower_module_chirho(
                &parse_to_cst_chirho(&source_chirho, file_chirho),
                file_chirho,
            );
            let (binders_chirho, kind_chirho) = module_chirho
                .decls_chirho
                .iter()
                .find_map(|decl_chirho| match decl_chirho {
                    DeclChirho::DataDeclChirho {
                        type_vars_chirho,
                        kind_sig_chirho,
                        ..
                    }
                    | DeclChirho::NewtypeDeclChirho {
                        type_vars_chirho,
                        kind_sig_chirho,
                        ..
                    } => Some((type_vars_chirho, kind_sig_chirho)),
                    _ => None,
                })
                .expect("data/newtype declaration");
            assert_eq!(binders_chirho.len(), 2, "{source_chirho}");
            assert!(!binders_chirho[0].is_visible_chirho(), "{source_chirho}");
            assert!(binders_chirho[1].is_visible_chirho(), "{source_chirho}");
            assert!(
                kind_chirho.is_none(),
                "binder kind cannot become a declaration kind"
            );
            assert!(module_chirho.decls_chirho.iter().any(|decl_chirho| matches!(decl_chirho,
                DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "afterChirho")));
        }
    }
}

fn kind_signature_chirho(
    keyword_chirho: &str,
    complete_chirho: bool,
    inline_chirho: bool,
) -> DataKindSigChirho {
    let standalone_chirho = if complete_chirho {
        "type BoxChirho :: Type -> Type\n"
    } else {
        ""
    };
    let result_chirho = if inline_chirho { " :: Type" } else { "" };
    let source_chirho = format!(
        "{{-# LANGUAGE GADTs, KindSignatures, StandaloneKindSignatures #-}}\nmodule KindsChirho where\n{standalone_chirho}{keyword_chirho} BoxChirho aChirho{result_chirho} where\n  MkBoxChirho :: aChirho -> BoxChirho aChirho\nafterChirho :: MissingChirho\nafterChirho = ()\n"
    );
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(&source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    assert!(module_chirho.decls_chirho.iter().any(|decl_chirho| matches!(decl_chirho,
        DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "afterChirho")),
        "following signature must survive");
    let signature_chirho = module_chirho
        .decls_chirho
        .into_iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                kind_sig_chirho,
                ..
            }
            | DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                kind_sig_chirho,
                ..
            } if name_chirho.text_chirho() == "BoxChirho" => {
                assert_eq!(
                    type_vars_chirho.len(),
                    1,
                    "annotation names must not become head binders"
                );
                kind_sig_chirho
            }
            _ => None,
        })
        .expect("declaration kind contract must survive");
    for (kind_chirho, expected_chirho) in [
        (signature_chirho.standalone_chirho(), "Type -> Type"),
        (signature_chirho.result_chirho(), "Type"),
    ] {
        if let Some(kind_chirho) = kind_chirho {
            let span_chirho = kind_chirho.span_chirho();
            assert_eq!(
                &source_chirho[span_chirho.start_chirho().as_usize_chirho()
                    ..span_chirho.end_chirho().as_usize_chirho()],
                expected_chirho,
                "kind diagnostic span must name only the written annotation"
            );
        }
    }
    signature_chirho
}

#[test]
fn inline_kind_is_a_result_not_a_complete_signature_chirho() {
    for keyword_chirho in ["data", "newtype"] {
        let signature_chirho = kind_signature_chirho(keyword_chirho, false, true);
        assert!(signature_chirho.standalone_chirho().is_none());
        assert!(
            matches!(signature_chirho.result_chirho(), Some(TypeChirho::ConChirho(name_chirho)) if name_chirho.text_chirho() == "Type")
        );
    }
}

#[test]
fn standalone_kind_has_no_fabricated_result_annotation_chirho() {
    for keyword_chirho in ["data", "newtype"] {
        let signature_chirho = kind_signature_chirho(keyword_chirho, true, false);
        assert!(signature_chirho.result_chirho().is_none());
        assert!(matches!(
            signature_chirho.standalone_chirho(),
            Some(TypeChirho::FunChirho { .. })
        ));
    }
}

#[test]
fn combined_kind_contracts_both_survive_with_exact_spans_chirho() {
    for keyword_chirho in ["data", "newtype"] {
        let signature_chirho = kind_signature_chirho(keyword_chirho, true, true);
        let complete_chirho = signature_chirho
            .standalone_chirho()
            .expect("complete kind must not be dropped");
        let result_chirho = signature_chirho
            .result_chirho()
            .expect("written tail must not be dropped");
        assert!(matches!(complete_chirho, TypeChirho::FunChirho { .. }));
        assert!(
            matches!(result_chirho, TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Type")
        );
        assert_ne!(complete_chirho.span_chirho(), result_chirho.span_chirho());
    }
}

#[test]
fn composite_inline_kind_span_covers_only_the_written_tail_chirho() {
    for keyword_chirho in ["data", "newtype"] {
        let source_chirho = format!(
            "{{-# LANGUAGE GADTs, KindSignatures #-}}\nmodule InlineChirho where\n{keyword_chirho} BoxChirho aChirho :: Type -> Type where\n  MkBoxChirho :: aChirho -> BoxChirho aChirho bChirho\n"
        );
        let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
        let cst_chirho = parse_to_cst_chirho(&source_chirho, file_chirho);
        let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
        let tail_chirho = module_chirho
            .decls_chirho
            .iter()
            .find_map(|decl_chirho| match decl_chirho {
                DeclChirho::DataDeclChirho {
                    kind_sig_chirho, ..
                }
                | DeclChirho::NewtypeDeclChirho {
                    kind_sig_chirho, ..
                } => kind_sig_chirho
                    .as_ref()
                    .and_then(DataKindSigChirho::result_chirho),
                _ => None,
            })
            .expect("the written result kind must survive");
        assert!(matches!(tail_chirho, TypeChirho::FunChirho { .. }));
        let span_chirho = tail_chirho.span_chirho();
        assert_eq!(
            &source_chirho[span_chirho.start_chirho().as_usize_chirho()
                ..span_chirho.end_chirho().as_usize_chirho()],
            "Type -> Type"
        );
    }
}
