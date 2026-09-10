// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Equation patterns are semantic arguments, including promoted empty groups.
use super::*;

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
