// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rules 7 and 8 on hand-built AST: each refusal at its class, and the strategies,
//! editions and spellings that must stay accepted (row 694).

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DerivedClassChirho, StrictnessChirho};
use haskelujah_ast_chirho::name_chirho::RawNameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho, SpanChirho};

use super::*;

fn span_at_chirho(start_chirho: u32) -> SpanChirho {
    SpanChirho::new_chirho(
        FileIdChirho::SYNTHETIC_CHIRHO,
        ByteOffsetChirho::new_chirho(start_chirho),
        ByteOffsetChirho::new_chirho(start_chirho + 1),
    )
}

fn name_at_chirho(text_chirho: &str, span_chirho: SpanChirho) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(text_chirho, span_chirho))
}

fn name_chirho(text_chirho: &str) -> NameChirho {
    name_at_chirho(text_chirho, SpanChirho::DUMMY_CHIRHO)
}

/// `Show` written at offset 40, with `strategy_chirho`.
fn show_with_chirho(strategy_chirho: Option<DerivingStrategyChirho>) -> DerivedClassChirho {
    DerivedClassChirho::new_chirho(name_at_chirho("Show", span_at_chirho(40)), strategy_chirho)
}

fn newtype_strategy_chirho() -> Option<DerivingStrategyChirho> {
    Some(DerivingStrategyChirho::newtype_at_chirho(span_at_chirho(
        30,
    )))
}

fn int_constructor_chirho(name_text_chirho: &str) -> ConDeclChirho {
    ConDeclChirho::OrdinaryChirho {
        name_chirho: name_chirho(name_text_chirho),
        fields_chirho: vec![(
            StrictnessChirho::LazyChirho,
            TypeChirho::ConChirho(name_chirho("Int")),
        )],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn data_chirho(deriving_chirho: Vec<DerivedClassChirho>) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("BadChirho"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![int_constructor_chirho("BadChirho")],
        deriving_chirho,
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn newtype_chirho(deriving_chirho: Vec<DerivedClassChirho>) -> DeclChirho {
    DeclChirho::NewtypeDeclChirho {
        name_chirho: name_chirho("AgeChirho"),
        type_vars_chirho: vec![],
        constructor_chirho: int_constructor_chirho("AgeChirho"),
        deriving_chirho,
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `deriving newtype instance Show <target>`, with `Show` at offset 50.
fn standalone_newtype_show_chirho(target_chirho: &str) -> DeclChirho {
    DeclChirho::StandaloneDerivingDeclChirho {
        strategy_chirho: newtype_strategy_chirho(),
        context_chirho: vec![],
        class_chirho: name_at_chirho("Show", span_at_chirho(50)),
        types_chirho: vec![TypeChirho::ConChirho(name_chirho(target_chirho))],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn errors_chirho(
    decls_chirho: Vec<DeclChirho>,
    extensions_chirho: &[&str],
) -> Vec<DeclarationErrorChirho> {
    let module_chirho = ModuleChirho {
        name_chirho: name_chirho("Main"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: extensions_chirho
            .iter()
            .map(|e_chirho| e_chirho.to_string())
            .collect(),
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };
    let mut errors_chirho = Vec::new();
    check_deriving_strategies_chirho(&module_chirho, &mut errors_chirho);
    errors_chirho
}

/// Each error's GHC code and span start.
fn codes_at_chirho(errors_chirho: &[DeclarationErrorChirho]) -> Vec<(&'static str, SpanChirho)> {
    errors_chirho
        .iter()
        .map(|error_chirho| (error_chirho.ghc_code_chirho, error_chirho.span_chirho))
        .collect()
}

#[test]
fn the_newtype_strategy_on_data_is_refused_at_the_class_chirho() {
    let found_chirho = errors_chirho(
        vec![data_chirho(vec![show_with_chirho(
            newtype_strategy_chirho(),
        )])],
        &["DerivingStrategies", "GeneralizedNewtypeDeriving"],
    );
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [("GHC-10333", span_at_chirho(40))]
    );
    assert!(found_chirho[0].message_chirho.contains("`Show BadChirho`"));
}

#[test]
fn other_strategies_on_data_are_accepted_chirho() {
    let span_chirho = span_at_chirho(30);
    let found_chirho = errors_chirho(
        vec![data_chirho(vec![
            show_with_chirho(None),
            show_with_chirho(Some(DerivingStrategyChirho::stock_at_chirho(span_chirho))),
            show_with_chirho(Some(DerivingStrategyChirho::anyclass_at_chirho(
                span_chirho,
            ))),
        ])],
        &["Haskell2010"],
    );
    assert!(found_chirho.is_empty(), "{found_chirho:?}");
}

#[test]
fn haskell2010_without_newtype_deriving_refuses_the_strategy_chirho() {
    let found_chirho = errors_chirho(
        vec![newtype_chirho(vec![show_with_chirho(
            newtype_strategy_chirho(),
        )])],
        &["Haskell2010", "DerivingStrategies"],
    );
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [("GHC-26557", span_at_chirho(40))]
    );
}

#[test]
fn the_default_edition_and_either_spelling_enable_newtype_deriving_chirho() {
    let newtype_show_chirho = || {
        vec![newtype_chirho(vec![show_with_chirho(
            newtype_strategy_chirho(),
        )])]
    };
    for extensions_chirho in [
        &[][..],
        &["DerivingStrategies"][..],
        &["Haskell2010", "GeneralizedNewtypeDeriving"][..],
        &["GeneralisedNewtypeDeriving", "Haskell2010"][..],
    ] {
        let found_chirho = errors_chirho(newtype_show_chirho(), extensions_chirho);
        assert!(
            found_chirho.is_empty(),
            "{extensions_chirho:?}: {found_chirho:?}"
        );
    }
    // The last explicit flag wins.
    let found_chirho = errors_chirho(
        newtype_show_chirho(),
        &["GeneralizedNewtypeDeriving", "NoGeneralizedNewtypeDeriving"],
    );
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [("GHC-26557", span_at_chirho(40))]
    );
}

#[test]
fn an_unwritten_strategy_is_not_judged_here_chirho() {
    let found_chirho = errors_chirho(
        vec![newtype_chirho(vec![show_with_chirho(None)])],
        &["Haskell2010"],
    );
    assert!(found_chirho.is_empty(), "{found_chirho:?}");
}

#[test]
fn standalone_newtype_deriving_is_judged_against_its_local_target_chirho() {
    let found_chirho = errors_chirho(
        vec![
            data_chirho(vec![]),
            newtype_chirho(vec![]),
            standalone_newtype_show_chirho("BadChirho"),
            standalone_newtype_show_chirho("AgeChirho"),
            // An imported target is not judged here.
            standalone_newtype_show_chirho("ImportedChirho"),
        ],
        &["Haskell2010"],
    );
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [
            ("GHC-10333", span_at_chirho(50)),
            ("GHC-26557", span_at_chirho(50))
        ]
    );
}
