// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: Module headers, exports and imports.
//! Moved from `lower_chirho.rs`'s tests in row 641.

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, ForeignDirectionChirho};
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ExportSpecChirho, ImportItemChirho,
};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

use super::test_support_chirho::parse_and_lower_chirho;

#[test]
fn lower_module_name_chirho() {
    let module_chirho = parse_and_lower_chirho("module Foo where\n");
    assert_eq!(module_chirho.name_chirho.text_chirho(), "Foo");
}

#[test]
fn lower_hierarchical_module_name_chirho() {
    let module_chirho = parse_and_lower_chirho("module Data.Reflection where\n");
    assert_eq!(module_chirho.name_chirho.text_chirho(), "Data.Reflection");
    assert_eq!(
        module_chirho.name_chirho.full_name_chirho(),
        "Data.Reflection"
    );
}

#[test]
fn lower_script_defaults_to_main_chirho() {
    let module_chirho = parse_and_lower_chirho("x = 1\n");
    assert_eq!(module_chirho.name_chirho.text_chirho(), "Main");
}

#[test]
fn lower_import_decl_preserves_hierarchical_module_name_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nimport qualified Data.Reflection as ReflectionChirho\nxChirho = ()\n",
    );
    assert_eq!(module_chirho.imports_chirho.len(), 1);
    let import_chirho = &module_chirho.imports_chirho[0];
    assert_eq!(import_chirho.module_chirho.text_chirho(), "Data.Reflection");
    assert_eq!(
        import_chirho.module_chirho.full_name_chirho(),
        "Data.Reflection"
    );
    assert_eq!(
        import_chirho
            .alias_chirho
            .as_ref()
            .expect("qualified import alias should lower")
            .text_chirho(),
        "ReflectionChirho"
    );
}

#[test]
fn lower_parsec_prim_trimmed_module_shape_chirho() {
    let module_chirho = parse_and_lower_chirho(
        r#"{-# LANGUAGE RankNTypes #-}
module Text.Parsec.Prim
    ( unknownErrorChirho
    , runParsecTChirho
    , mkPTChirho
    , StateChirho(..)
    ) where

unknownErrorChirho :: StateChirho sChirho uChirho -> ParseErrorChirho
unknownErrorChirho stateChirho = newErrorUnknownChirho (statePosChirho stateChirho)

{-# INLINABLE runParsecTChirho #-}
runParsecTChirho pChirho sChirho = unParserChirho pChirho sChirho cokChirho cerrChirho eokChirho eerrChirho
    where cokChirho aChirho s'Chirho errChirho = return (OkChirho aChirho s'Chirho errChirho)
          cerrChirho errChirho = return (ErrorChirho errChirho)
          eokChirho aChirho s'Chirho errChirho = return (OkChirho aChirho s'Chirho errChirho)
          eerrChirho errChirho = return (ErrorChirho errChirho)

{-# INLINABLE mkPTChirho #-}
mkPTChirho kChirho = ParsecTChirho $ \sChirho cokChirho cerrChirho eokChirho eerrChirho -> do
           consChirho <- kChirho sChirho
           case consChirho of
             ConsumedChirho mrepChirho -> do
                       repChirho <- mrepChirho
                       case repChirho of
                         OkChirho xChirho s'Chirho errChirho -> cokChirho xChirho s'Chirho errChirho
                         ErrorChirho errChirho -> cerrChirho errChirho
             EmptyChirho mrepChirho -> do
                       repChirho <- mrepChirho
                       case repChirho of
                         OkChirho xChirho s'Chirho errChirho -> eokChirho xChirho s'Chirho errChirho
                         ErrorChirho errChirho -> eerrChirho errChirho

data StateChirho sChirho uChirho = StateChirho {
      stateInputChirho :: sChirho,
      statePosChirho   :: !SourcePosChirho,
      stateUserChirho  :: !uChirho
    }
"#,
    );
    assert_eq!(
        module_chirho.name_chirho.full_name_chirho(),
        "Text.Parsec.Prim"
    );
    assert!(
        module_chirho
            .decls_chirho
            .iter()
            .any(|decl_chirho| matches!(
                decl_chirho,
                DeclChirho::DataDeclChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "StateChirho"
            )),
        "expected StateChirho data declaration, got {:?}",
        module_chirho
            .decls_chirho
            .iter()
            .map(|decl_chirho| match decl_chirho {
                DeclChirho::TypeSigChirho { name_chirho, .. } =>
                    format!("typesig:{}", name_chirho.text_chirho()),
                DeclChirho::FunBindChirho { name_chirho, .. } =>
                    format!("fun:{}", name_chirho.text_chirho()),
                DeclChirho::DataDeclChirho { name_chirho, .. } =>
                    format!("data:{}", name_chirho.text_chirho()),
                DeclChirho::NewtypeDeclChirho { name_chirho, .. } =>
                    format!("newtype:{}", name_chirho.text_chirho()),
                DeclChirho::PatBindChirho { .. } => "patbind".to_string(),
                other_chirho => format!("{:?}", other_chirho),
            })
            .collect::<Vec<_>>()
    );
}

#[test]
fn lower_imported_infix_type_operator_keeps_left_application_spine_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE TypeOperators #-}\nmodule M where\nimport Data.Constraint\ninstF :: forall p f a . ForallF p f :- p (f a)\n",
    );
    let ty_sig_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
        matches!(decl_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "instF")
    });
    assert!(ty_sig_chirho.is_some(), "should have a TypeSig for instF");
    if let Some(DeclChirho::TypeSigChirho { ty_chirho, .. }) = ty_sig_chirho {
        let body_chirho = match ty_chirho {
            TypeChirho::ForallChirho { body_chirho, .. } => body_chirho.as_ref(),
            other_chirho => panic!("expected forall type, got {other_chirho:?}"),
        };
        let lhs_chirho = match body_chirho {
            TypeChirho::AppChirho { fun_chirho, .. } => match fun_chirho.as_ref() {
                TypeChirho::AppChirho { arg_chirho, .. } => arg_chirho.as_ref(),
                other_chirho => panic!("expected operator application, got {other_chirho:?}"),
            },
            other_chirho => panic!("expected infix type application, got {other_chirho:?}"),
        };
        assert!(
            matches!(lhs_chirho, TypeChirho::AppChirho { fun_chirho, arg_chirho, .. }
                if matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "f")
                    && matches!(fun_chirho.as_ref(), TypeChirho::AppChirho { fun_chirho, arg_chirho, .. }
                        if matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "p")
                            && matches!(fun_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "ForallF"))),
            "imported infix operator should keep ForallF p f as lhs, got {lhs_chirho:?}"
        );
    }
}

#[test]
fn lower_import_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nimport Data.List\nimport qualified Data.Map as Map\n",
    );
    assert_eq!(module_chirho.imports_chirho.len(), 2);
    assert_eq!(
        module_chirho.imports_chirho[0].module_chirho.text_chirho(),
        "Data.List"
    );
    assert!(module_chirho.imports_chirho[1].qualified_chirho);
    assert_eq!(
        module_chirho.imports_chirho[1]
            .alias_chirho
            .as_ref()
            .unwrap()
            .text_chirho(),
        "Map"
    );
}

#[test]
fn lower_import_spec_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nimport Data.List (sort, nub)\nimport Data.Map hiding (map)\n",
    );
    assert_eq!(module_chirho.imports_chirho.len(), 2);

    // First import: explicit list
    let spec0_chirho = module_chirho.imports_chirho[0].spec_chirho.as_ref();
    assert!(spec0_chirho.is_some(), "expected import spec for Data.List");
    let spec0_chirho = spec0_chirho.unwrap();
    assert!(!spec0_chirho.hiding_chirho);
    assert_eq!(spec0_chirho.items_chirho.len(), 2);
    assert!(
        matches!(&spec0_chirho.items_chirho[0], ImportItemChirho::VarChirho(n_chirho) if n_chirho.text_chirho() == "sort")
    );
    assert!(
        matches!(&spec0_chirho.items_chirho[1], ImportItemChirho::VarChirho(n_chirho) if n_chirho.text_chirho() == "nub")
    );

    // Second import: hiding
    let spec1_chirho = module_chirho.imports_chirho[1].spec_chirho.as_ref();
    assert!(spec1_chirho.is_some(), "expected import spec for Data.Map");
    let spec1_chirho = spec1_chirho.unwrap();
    assert!(spec1_chirho.hiding_chirho);
    assert_eq!(spec1_chirho.items_chirho.len(), 1);
    assert!(
        matches!(&spec1_chirho.items_chirho[0], ImportItemChirho::VarChirho(n_chirho) if n_chirho.text_chirho() == "map")
    );
}

#[test]
fn lower_import_tycon_spec_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\nimport Data.Map (Map(..))\n");
    assert_eq!(module_chirho.imports_chirho.len(), 1);
    let spec_chirho = module_chirho.imports_chirho[0]
        .spec_chirho
        .as_ref()
        .unwrap();
    assert!(!spec_chirho.hiding_chirho);
    assert_eq!(spec_chirho.items_chirho.len(), 1);
    assert!(matches!(
        &spec_chirho.items_chirho[0],
        ImportItemChirho::TyConChirho { name_chirho, members_chirho: ExportMembersChirho::AllChirho }
            if name_chirho.text_chirho() == "Map"
    ));
}

#[test]
fn lower_import_operator_tycon_spec_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\nimport GHC.Generics ((:*:)(..))\n");
    assert_eq!(module_chirho.imports_chirho.len(), 1);
    let spec_chirho = module_chirho.imports_chirho[0]
        .spec_chirho
        .as_ref()
        .unwrap();
    assert!(!spec_chirho.hiding_chirho);
    assert_eq!(spec_chirho.items_chirho.len(), 1);
    assert!(matches!(
        &spec_chirho.items_chirho[0],
        ImportItemChirho::TyConChirho { name_chirho, members_chirho: ExportMembersChirho::AllChirho }
            if name_chirho.text_chirho() == ":*:"
    ));
}

#[test]
fn lower_import_operator_tycon_with_dot_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\nimport GHC.Generics ((:.:)(..))\n");
    assert_eq!(module_chirho.imports_chirho.len(), 1);
    let spec_chirho = module_chirho.imports_chirho[0]
        .spec_chirho
        .as_ref()
        .unwrap();
    assert_eq!(spec_chirho.items_chirho.len(), 1);
    assert!(matches!(
        &spec_chirho.items_chirho[0],
        ImportItemChirho::TyConChirho { name_chirho, members_chirho: ExportMembersChirho::AllChirho }
            if name_chirho.text_chirho() == ":.:"
    ));
}

#[test]
fn lower_foreign_import_basic_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nforeign import ccall unsafe \"sin\" sinChirho :: Double -> Double\n",
    );
    let foreign_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| matches!(d_chirho, DeclChirho::ForeignDeclChirho { .. }));
    assert!(foreign_chirho.is_some(), "expected ForeignDeclChirho");
    match foreign_chirho.unwrap() {
        DeclChirho::ForeignDeclChirho {
            direction_chirho,
            name_chirho,
            calling_conv_chirho,
            safety_chirho,
            foreign_name_chirho,
            ..
        } => {
            assert_eq!(*direction_chirho, ForeignDirectionChirho::ImportChirho);
            assert_eq!(name_chirho.text_chirho(), "sinChirho");
            assert_eq!(calling_conv_chirho, "ccall");
            assert_eq!(safety_chirho.as_deref(), Some("unsafe"));
            assert_eq!(foreign_name_chirho.as_deref(), Some("sin"));
        }
        _ => unreachable!(),
    }
}

#[test]
fn lower_foreign_import_no_safety_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nforeign import ccall putStrChirho :: String -> IO ()\n",
    );
    let foreign_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| matches!(d_chirho, DeclChirho::ForeignDeclChirho { .. }));
    assert!(foreign_chirho.is_some());
    match foreign_chirho.unwrap() {
        DeclChirho::ForeignDeclChirho {
            direction_chirho,
            name_chirho,
            calling_conv_chirho,
            safety_chirho,
            foreign_name_chirho,
            ..
        } => {
            assert_eq!(*direction_chirho, ForeignDirectionChirho::ImportChirho);
            assert_eq!(name_chirho.text_chirho(), "putStrChirho");
            assert_eq!(calling_conv_chirho, "ccall");
            assert_eq!(*safety_chirho, None);
            assert_eq!(*foreign_name_chirho, None);
        }
        _ => unreachable!(),
    }
}

#[test]
fn lower_foreign_export_chirho() {
    let module_chirho =
        parse_and_lower_chirho("module M where\nforeign export ccall mainChirho :: IO ()\n");
    let foreign_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| matches!(d_chirho, DeclChirho::ForeignDeclChirho { .. }));
    assert!(foreign_chirho.is_some());
    match foreign_chirho.unwrap() {
        DeclChirho::ForeignDeclChirho {
            direction_chirho,
            name_chirho,
            ..
        } => {
            assert_eq!(*direction_chirho, ForeignDirectionChirho::ExportChirho);
            assert_eq!(name_chirho.text_chirho(), "mainChirho");
        }
        _ => unreachable!(),
    }
}

#[test]
fn lower_spans_are_nonzero_chirho() {
    let module_chirho = parse_and_lower_chirho("module Foo where\ndata Bar = Baz\n");
    let span_chirho = module_chirho.span_chirho;
    assert!(span_chirho.end_chirho().as_usize_chirho() > 0);
}

// -----------------------------------------------------------------------
// Export list lowering tests
// -----------------------------------------------------------------------

#[test]
fn lower_export_list_var_chirho() {
    // module M (foo) where — exports a single variable
    let module_chirho = parse_and_lower_chirho(
        "module M (foo) where
foo = 1
",
    );
    let exports_chirho = module_chirho
        .exports_chirho
        .as_ref()
        .expect("expected an export list");
    assert_eq!(exports_chirho.len(), 1, "expected 1 export");
    match &exports_chirho[0] {
        ExportSpecChirho::VarChirho(name_chirho) => {
            assert_eq!(name_chirho.text_chirho(), "foo");
        }
        other_chirho => panic!("expected VarChirho, got {:?}", other_chirho),
    }
}

#[test]
fn lower_export_list_var_with_apostrophe_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M (liftCallCC') where
liftCallCC' = id
",
    );
    let exports_chirho = module_chirho
        .exports_chirho
        .as_ref()
        .expect("expected an export list");
    assert_eq!(exports_chirho.len(), 1, "expected 1 export");
    match &exports_chirho[0] {
        ExportSpecChirho::VarChirho(name_chirho) => {
            assert_eq!(name_chirho.text_chirho(), "liftCallCC'");
        }
        other_chirho => panic!("expected VarChirho, got {:?}", other_chirho),
    }
}

#[test]
fn lower_export_list_qualified_var_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M (M.lookup) where
lookup = 1
",
    );
    let exports_chirho = module_chirho
        .exports_chirho
        .as_ref()
        .expect("expected an export list");
    assert_eq!(exports_chirho.len(), 1, "expected 1 export");
    match &exports_chirho[0] {
        ExportSpecChirho::VarChirho(name_chirho) => {
            assert_eq!(name_chirho.text_chirho(), "lookup");
            assert_eq!(name_chirho.full_name_chirho(), "M.lookup");
        }
        other_chirho => panic!("expected VarChirho, got {:?}", other_chirho),
    }
}

#[test]
fn lower_explicit_type_operator_export_keeps_its_namespace_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE ExplicitNamespaces, TypeOperators #-}\nmodule MChirho (type (QChirho..), (QChirho..)) where\nimport qualified OriginChirho as QChirho\n",
    );
    let exports_chirho = module_chirho
        .exports_chirho
        .as_ref()
        .expect("explicit exports");
    assert_eq!(exports_chirho.len(), 2);
    match &exports_chirho[0] {
        ExportSpecChirho::TyConChirho {
            name_chirho,
            members_chirho,
        } => {
            assert_eq!(name_chirho.full_name_chirho(), "QChirho..");
            assert_eq!(*members_chirho, ExportMembersChirho::NoneChirho);
        }
        other_chirho => panic!("type marker must survive lowering: {other_chirho:?}"),
    }
    match &exports_chirho[1] {
        ExportSpecChirho::VarChirho(name_chirho) => {
            assert_eq!(name_chirho.full_name_chirho(), "QChirho..");
        }
        other_chirho => panic!("ordinary variable-symbol export stays a value: {other_chirho:?}"),
    }
}

#[test]
fn lower_symbolic_member_namespaces_survive_nested_parentheses_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE ExplicitNamespaces, TypeOperators #-}\nmodule MChirho (CChirho(type (#), (#), type (+), plainChirho)) where\n",
    );
    let exports_chirho = module_chirho
        .exports_chirho
        .as_ref()
        .expect("explicit exports");
    let ExportSpecChirho::TyConChirho {
        members_chirho: ExportMembersChirho::SomeChirho(members_chirho),
        ..
    } = &exports_chirho[0]
    else {
        panic!("subordinate member list must survive lowering");
    };
    assert_eq!(
        members_chirho
            .iter()
            .map(|name_chirho| (
                name_chirho.text_chirho(),
                name_chirho.is_explicit_type_member_chirho()
            ))
            .collect::<Vec<_>>(),
        vec![
            ("#", true),
            ("#", false),
            ("+", true),
            ("plainChirho", false),
        ]
    );
}

#[test]
fn lower_export_list_tycon_all_chirho() {
    // module M (Color(..)) where — exports a type with all members
    let module_chirho = parse_and_lower_chirho(
        "module M (Color(..)) where
data Color = Red | Green | Blue
",
    );
    let exports_chirho = module_chirho
        .exports_chirho
        .as_ref()
        .expect("expected an export list");
    assert_eq!(exports_chirho.len(), 1, "expected 1 export");
    match &exports_chirho[0] {
        ExportSpecChirho::TyConChirho {
            name_chirho,
            members_chirho,
        } => {
            assert_eq!(name_chirho.text_chirho(), "Color");
            assert_eq!(*members_chirho, ExportMembersChirho::AllChirho);
        }
        other_chirho => panic!("expected TyConChirho, got {:?}", other_chirho),
    }
}

#[test]
fn lower_export_list_operator_tycon_all_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M ((:-)(..)) where
newtype a :- b = Sub Int
",
    );
    let exports_chirho = module_chirho
        .exports_chirho
        .as_ref()
        .expect("expected an export list");
    assert_eq!(exports_chirho.len(), 1, "expected 1 export");
    match &exports_chirho[0] {
        ExportSpecChirho::TyConChirho {
            name_chirho,
            members_chirho,
        } => {
            assert_eq!(name_chirho.text_chirho(), ":-");
            assert_eq!(*members_chirho, ExportMembersChirho::AllChirho);
        }
        other_chirho => panic!("expected operator TyConChirho, got {:?}", other_chirho),
    }
}

#[test]
fn lower_export_list_tycon_specific_chirho() {
    // module M (Color(Red, Green)) where — exports a type with specific members
    let module_chirho = parse_and_lower_chirho(
        "module M (Color(Red, Green)) where
data Color = Red | Green | Blue
",
    );
    let exports_chirho = module_chirho
        .exports_chirho
        .as_ref()
        .expect("expected an export list");
    assert_eq!(exports_chirho.len(), 1, "expected 1 export");
    match &exports_chirho[0] {
        ExportSpecChirho::TyConChirho {
            name_chirho,
            members_chirho: ExportMembersChirho::SomeChirho(names_chirho),
        } => {
            assert_eq!(name_chirho.text_chirho(), "Color");
            assert_eq!(names_chirho.len(), 2);
            assert_eq!(names_chirho[0].text_chirho(), "Red");
            assert_eq!(names_chirho[1].text_chirho(), "Green");
        }
        other_chirho => panic!(
            "expected TyConChirho with SomeChirho, got {:?}",
            other_chirho
        ),
    }
}

#[test]
fn lower_export_list_mixed_chirho() {
    // module M (foo, Bar(..)) where — exports a var and a type
    let module_chirho = parse_and_lower_chirho(
        "module M (foo, Bar(..)) where
foo = 1
data Bar = Bar
",
    );
    let exports_chirho = module_chirho
        .exports_chirho
        .as_ref()
        .expect("expected an export list");
    assert_eq!(exports_chirho.len(), 2, "expected 2 exports");
    match &exports_chirho[0] {
        ExportSpecChirho::VarChirho(name_chirho) => {
            assert_eq!(name_chirho.text_chirho(), "foo");
        }
        other_chirho => panic!("expected VarChirho, got {:?}", other_chirho),
    }
    match &exports_chirho[1] {
        ExportSpecChirho::TyConChirho {
            name_chirho,
            members_chirho,
        } => {
            assert_eq!(name_chirho.text_chirho(), "Bar");
            assert_eq!(*members_chirho, ExportMembersChirho::AllChirho);
        }
        other_chirho => panic!("expected TyConChirho, got {:?}", other_chirho),
    }
}

#[test]
fn lower_no_export_list_chirho() {
    // module M where — no export list means None
    let module_chirho = parse_and_lower_chirho(
        "module M where
foo = 1
",
    );
    assert!(
        module_chirho.exports_chirho.is_none(),
        "expected None export list when no parens"
    );
}
