// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Equation patterns are semantic arguments, including promoted empty groups.
use super::*;

#[test]
fn standalone_and_result_family_contracts_keep_independent_spans_chirho() {
    for declaration_chirho in ["type family", "data family"] {
        let source_chirho = format!(
            "module FamilyChirho where\ntype FamilyChirho :: forall kindChirho. kindChirho -> Type\n{declaration_chirho} FamilyChirho (valueChirho :: localChirho) :: Type\ncanaryChirho :: MissingTypeChirho\ncanaryChirho = undefined\n"
        );
        let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
        let module_chirho = lower_module_chirho(
            &crate::cst_parser_chirho::parse_to_cst_chirho(&source_chirho, file_chirho),
            file_chirho,
        );
        let DeclChirho::TypeFamilyDeclChirho {
            type_vars_chirho,
            result_chirho,
            equations_chirho,
            ..
        } = &module_chirho.decls_chirho[0]
        else {
            panic!("{module_chirho:?}");
        };
        let signature_chirho = result_chirho.kind_sig_chirho.as_ref().unwrap();
        for (kind_chirho, written_chirho) in [
            (
                signature_chirho.standalone_chirho().unwrap(),
                "forall kindChirho. kindChirho -> Type",
            ),
            (signature_chirho.result_chirho().unwrap(), "Type"),
        ] {
            let span_chirho = kind_chirho.span_chirho();
            assert_eq!(
                &source_chirho[span_chirho.start_chirho().as_usize_chirho()
                    ..span_chirho.end_chirho().as_usize_chirho()],
                written_chirho
            );
        }
        assert_eq!(type_vars_chirho.len(), 1);
        assert!(equations_chirho.is_empty());
        assert!(
            matches!(&module_chirho.decls_chirho[1], DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "canaryChirho")
        );
    }
}

#[test]
fn family_head_binders_preserve_invisible_scope_chirho() {
    for invisible_chirho in ["@kChirho", "@(kChirho :: Type)"] {
        let source_chirho = format!(
            "{{-# LANGUAGE TypeFamilies, TypeAbstractions #-}}\nmodule FamilyChirho where\ntype family IdentityChirho {invisible_chirho} (aChirho :: kChirho) :: kChirho where\n  IdentityChirho aChirho = aChirho\ncanaryChirho :: MissingTypeChirho\ncanaryChirho = undefined\n"
        );
        let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
        let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(&source_chirho, file_chirho);
        let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
        let DeclChirho::TypeFamilyDeclChirho {
            type_vars_chirho,
            equations_chirho,
            ..
        } = &module_chirho.decls_chirho[0]
        else {
            panic!("expected the family declaration");
        };
        assert_eq!(type_vars_chirho.len(), 2);
        assert_eq!(type_vars_chirho[0].text_chirho(), "kChirho");
        assert!(!type_vars_chirho[0].is_visible_chirho());
        assert_eq!(type_vars_chirho[1].text_chirho(), "aChirho");
        assert!(type_vars_chirho[1].is_visible_chirho());
        assert_eq!(equations_chirho[0].lhs_types_chirho.len(), 1);
        assert!(module_chirho.decls_chirho.iter().any(|declaration_chirho| matches!(declaration_chirho, DeclChirho::TypeSigChirho { ty_chirho: TypeChirho::ConChirho(name_chirho), .. } if name_chirho.text_chirho() == "MissingTypeChirho")));
    }
}

#[test]
fn family_result_binder_kind_and_dependency_survive_lowering_chirho() {
    let source_chirho = "{-# LANGUAGE TypeFamilyDependencies #-}\nmodule FamilyChirho where\ntype family IdentityChirho (aChirho :: Type) = (resultChirho :: Type) | resultChirho -> aChirho where\n  IdentityChirho aChirho = aChirho\ncanaryChirho :: MissingTypeChirho\ncanaryChirho = undefined\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let DeclChirho::TypeFamilyDeclChirho {
        type_vars_chirho,
        result_chirho,
        closed_chirho,
        equations_chirho,
        ..
    } = &module_chirho.decls_chirho[0]
    else {
        panic!("family declaration missing");
    };
    assert_eq!(type_vars_chirho.len(), 1);
    assert_eq!(type_vars_chirho[0].text_chirho(), "aChirho");
    assert_eq!(
        result_chirho.binder_chirho.as_ref().unwrap().text_chirho(),
        "resultChirho"
    );
    assert!(
        matches!(result_chirho.kind_sig_chirho.as_ref().and_then(DeclKindSigChirho::result_chirho), Some(TypeChirho::ConChirho(name_chirho)) if name_chirho.text_chirho() == "Type")
    );
    let dependency_chirho = result_chirho.injectivity_chirho.as_ref().unwrap();
    assert_eq!(
        dependency_chirho.result_chirho.text_chirho(),
        "resultChirho"
    );
    assert_eq!(
        dependency_chirho
            .parameters_chirho
            .iter()
            .map(NameChirho::text_chirho)
            .collect::<Vec<_>>(),
        ["aChirho"]
    );
    assert!(*closed_chirho);
    assert_eq!(equations_chirho.len(), 1);
    assert!(module_chirho.decls_chirho.iter().any(|declaration_chirho| matches!(declaration_chirho, DeclChirho::TypeSigChirho { ty_chirho: TypeChirho::ConChirho(name_chirho), .. } if name_chirho.text_chirho() == "MissingTypeChirho")));
}

#[test]
fn an_empty_closed_family_is_not_an_open_family_chirho() {
    let source_chirho = "{-# LANGUAGE TypeFamilies #-}\nmodule FamilyChirho where\ntype family OpenChirho aChirho\ntype family ClosedChirho aChirho where {}\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    for (declaration_chirho, expected_closed_chirho) in
        module_chirho.decls_chirho.iter().zip([false, true])
    {
        let DeclChirho::TypeFamilyDeclChirho {
            closed_chirho,
            equations_chirho,
            ..
        } = declaration_chirho
        else {
            panic!("family declaration missing");
        };
        assert_eq!(*closed_chirho, expected_closed_chirho);
        assert!(equations_chirho.is_empty());
    }
    assert_eq!(module_chirho.decls_chirho.len(), 2);
}

#[test]
fn literal_family_patterns_keep_values_and_equation_boundaries_chirho() {
    let source_chirho = "{-# LANGUAGE DataKinds, TypeFamilies #-}\nmodule FamilyChirho where\ntype family LiteralChirho aChirho bChirho where { LiteralChirho 0 \"zero\" = Int; LiteralChirho 1 \"one\" = Bool }\ncanaryChirho :: MissingTypeChirho\ncanaryChirho = undefined\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let DeclChirho::TypeFamilyDeclChirho {
        equations_chirho, ..
    } = &module_chirho.decls_chirho[0]
    else {
        panic!("expected a family");
    };
    assert_eq!(equations_chirho.len(), 2);
    for (equation_chirho, expected_chirho) in equations_chirho
        .iter()
        .zip([["0", "\"zero\""], ["1", "\"one\""]])
    {
        assert_eq!(equation_chirho.lhs_types_chirho.len(), 2);
        for (pattern_chirho, expected_chirho) in
            equation_chirho.lhs_types_chirho.iter().zip(expected_chirho)
        {
            assert!(
                matches!(pattern_chirho, TypeChirho::LitChirho { value_chirho, .. } if value_chirho == expected_chirho)
            );
            let span_chirho = pattern_chirho.span_chirho();
            assert_eq!(
                &source_chirho[span_chirho.start_chirho().as_usize_chirho()
                    ..span_chirho.end_chirho().as_usize_chirho()],
                expected_chirho
            );
        }
    }
    assert!(
        matches!(&module_chirho.decls_chirho[1], DeclChirho::TypeSigChirho { ty_chirho: TypeChirho::ConChirho(name_chirho), .. } if name_chirho.text_chirho() == "MissingTypeChirho")
    );
}

#[test]
fn malformed_family_pattern_recovery_reaches_the_next_declaration_chirho() {
    let source_chirho = "{-# LANGUAGE TypeFamilies #-}\nmodule FamilyChirho where\ntype family BadChirho aChirho where { ::; BadChirho Int = Int }\ncanaryChirho :: MissingTypeChirho\ncanaryChirho = undefined\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    assert!(module_chirho.decls_chirho.iter().any(|declaration_chirho| matches!(declaration_chirho, DeclChirho::TypeSigChirho { ty_chirho: TypeChirho::ConChirho(name_chirho), .. } if name_chirho.text_chirho() == "MissingTypeChirho")));
}

#[test]
fn family_cons_patterns_retain_promotion_chirho() {
    let source_chirho = "{-# LANGUAGE DataKinds, TypeFamilies, TypeOperators #-}\nmodule FamilyChirho where\ntype family HeadChirho xsChirho where\n  HeadChirho (itemChirho ': restChirho) = itemChirho\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let DeclChirho::TypeFamilyDeclChirho {
        equations_chirho, ..
    } = &module_chirho.decls_chirho[0]
    else {
        panic!("expected a family");
    };
    let TypeChirho::ParenChirho { inner_chirho, .. } = &equations_chirho[0].lhs_types_chirho[0]
    else {
        panic!("expected a grouped cons pattern");
    };
    let TypeChirho::AppChirho { fun_chirho, .. } = inner_chirho.as_ref() else {
        panic!("expected cons applied to a tail");
    };
    let TypeChirho::AppChirho { fun_chirho, .. } = fun_chirho.as_ref() else {
        panic!("expected cons applied to a head");
    };
    assert!(
        matches!(fun_chirho.as_ref(), TypeChirho::PromotedConChirho { name_chirho, .. } if name_chirho.text_chirho() == ":")
    );
}

#[test]
fn closed_family_keeps_the_empty_promoted_list_pattern_chirho() {
    let source_chirho = "{-# LANGUAGE DataKinds, TypeFamilies #-}\nmodule FamilyChirho where\nimport Data.Kind (Type)\ntype family PickChirho (xsChirho :: [Type]) (rChirho :: Type) where\n  PickChirho '[] rChirho = rChirho\ncanaryChirho :: MissingTypeChirho\ncanaryChirho = undefined\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let DeclChirho::TypeFamilyDeclChirho {
        equations_chirho, ..
    } = &module_chirho.decls_chirho[0]
    else {
        panic!("expected a family");
    };
    assert_eq!(equations_chirho.len(), 1);
    assert_eq!(equations_chirho[0].lhs_types_chirho.len(), 2);
    assert!(
        matches!(&equations_chirho[0].lhs_types_chirho[0], TypeChirho::PromotedListChirho { elements_chirho, .. } if elements_chirho.is_empty())
    );
    assert!(
        matches!(&equations_chirho[0].lhs_types_chirho[1], TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "rChirho")
    );
    let pattern_span_chirho = equations_chirho[0].lhs_types_chirho[0].span_chirho();
    assert_eq!(
        &source_chirho[pattern_span_chirho.start_chirho().as_usize_chirho()
            ..pattern_span_chirho.end_chirho().as_usize_chirho()],
        "'[]"
    );
    assert!(
        matches!(&module_chirho.decls_chirho[1], DeclChirho::TypeSigChirho { ty_chirho: TypeChirho::ConChirho(name_chirho), .. } if name_chirho.text_chirho() == "MissingTypeChirho")
    );
}

#[test]
fn open_family_keeps_nested_promoted_list_and_grouped_patterns_chirho() {
    let source_chirho = "{-# LANGUAGE DataKinds, TypeFamilies #-}\nmodule FamilyChirho where\ntype family PickChirho xsChirho rChirho\ntype instance PickChirho '[ '[Int], '[Bool]] (Maybe aChirho) = aChirho\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let DeclChirho::TypeFamilyInstanceDeclChirho {
        lhs_types_chirho, ..
    } = &module_chirho.decls_chirho[1]
    else {
        panic!("expected a family instance");
    };
    assert_eq!(lhs_types_chirho.len(), 2);
    let TypeChirho::PromotedListChirho {
        elements_chirho, ..
    } = &lhs_types_chirho[0]
    else {
        panic!("the outer promoted list pattern must survive");
    };
    assert_eq!(elements_chirho.len(), 2);
    for (element_chirho, expected_chirho) in elements_chirho.iter().zip(["Int", "Bool"]) {
        let TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } = element_chirho
        else {
            panic!("nested brackets belong to the inner promoted list");
        };
        assert!(
            matches!(elements_chirho.as_slice(), [TypeChirho::ConChirho(name_chirho)] if name_chirho.text_chirho() == expected_chirho)
        );
    }
    assert!(
        matches!(&lhs_types_chirho[1], TypeChirho::ParenChirho { inner_chirho, .. } if matches!(inner_chirho.as_ref(), TypeChirho::AppChirho { .. }))
    );
}
