// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rules 7 to 12 on hand-built AST: each refusal at GHC's caret, and the strategies, editions
//! and extensions that must stay accepted (row 694; root's fresh GHC 9.14.1 contracts). The
//! extension lists are as the parser leaves them, written flags after any edition.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DerivedClassChirho, DerivingStrategyChirho, StrictnessChirho,
};
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

/// The strategy keyword is written at offset 30 and the class at offset 40.
fn class_with_chirho(
    class_chirho: &str,
    strategy_chirho: Option<DerivingStrategyChirho>,
) -> DerivedClassChirho {
    DerivedClassChirho::new_chirho(
        name_at_chirho(class_chirho, span_at_chirho(40)),
        strategy_chirho,
    )
}

fn stock_chirho() -> Option<DerivingStrategyChirho> {
    Some(DerivingStrategyChirho::stock_at_chirho(span_at_chirho(30)))
}

fn newtype_strategy_chirho() -> Option<DerivingStrategyChirho> {
    Some(DerivingStrategyChirho::newtype_at_chirho(span_at_chirho(
        30,
    )))
}

fn anyclass_chirho() -> Option<DerivingStrategyChirho> {
    Some(DerivingStrategyChirho::anyclass_at_chirho(span_at_chirho(
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
            .map(|extension_chirho| extension_chirho.to_string())
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

/// Each error's GHC code and span.
fn codes_at_chirho(errors_chirho: &[DeclarationErrorChirho]) -> Vec<(&'static str, SpanChirho)> {
    errors_chirho
        .iter()
        .map(|error_chirho| (error_chirho.ghc_code_chirho, error_chirho.span_chirho))
        .collect()
}

#[test]
fn a_written_strategy_needs_deriving_strategies_chirho() {
    // A1: refused at the keyword. A2 and DerivingVia (which implies it): accepted.
    let found_chirho = errors_chirho(
        vec![data_chirho(vec![class_with_chirho("Show", stock_chirho())])],
        &[],
    );
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [("GHC-87139", span_at_chirho(30))]
    );
    assert_eq!(
        found_chirho[0].message_chirho,
        "Illegal deriving strategy: stock"
    );
    for extensions_chirho in [&["DerivingStrategies"][..], &["DerivingVia"][..]] {
        let accepted_chirho = errors_chirho(
            vec![data_chirho(vec![class_with_chirho("Show", stock_chirho())])],
            extensions_chirho,
        );
        assert!(
            accepted_chirho.is_empty(),
            "{extensions_chirho:?}: {accepted_chirho:?}"
        );
    }
}

#[test]
fn the_newtype_strategy_on_data_is_refused_at_the_class_chirho() {
    let found_chirho = errors_chirho(
        vec![data_chirho(vec![class_with_chirho(
            "Show",
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
fn the_newtype_strategy_follows_the_last_edition_or_written_flag_chirho() {
    let newtype_show_chirho = || {
        vec![newtype_chirho(vec![class_with_chirho(
            "Show",
            newtype_strategy_chirho(),
        )])]
    };
    // O1 refuses: Haskell2010 is the last edition and no flag is written.
    for extensions_chirho in [
        &["Haskell2010", "DerivingStrategies"][..],
        &[
            "DerivingStrategies",
            "GeneralizedNewtypeDeriving",
            "NoGeneralizedNewtypeDeriving",
        ][..],
    ] {
        let found_chirho = errors_chirho(newtype_show_chirho(), extensions_chirho);
        assert_eq!(
            codes_at_chirho(&found_chirho),
            [("GHC-26557", span_at_chirho(40))],
            "{extensions_chirho:?}"
        );
    }
    // The default edition, O2 and either written spelling enable it.
    for extensions_chirho in [
        &["DerivingStrategies"][..],
        &[
            "Haskell2010",
            "GeneralizedNewtypeDeriving",
            "DerivingStrategies",
        ][..],
        &[
            "Haskell2010",
            "GeneralisedNewtypeDeriving",
            "DerivingStrategies",
        ][..],
    ] {
        let found_chirho = errors_chirho(newtype_show_chirho(), extensions_chirho);
        assert!(
            found_chirho.is_empty(),
            "{extensions_chirho:?}: {found_chirho:?}"
        );
    }
}

#[test]
fn the_anyclass_strategy_needs_derive_any_class_chirho() {
    // B3 refuses at the class; B4 accepts.
    let anyclass_describe_chirho = || {
        vec![data_chirho(vec![class_with_chirho(
            "DescribeChirho",
            anyclass_chirho(),
        )])]
    };
    let found_chirho = errors_chirho(anyclass_describe_chirho(), &["DerivingStrategies"]);
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [("GHC-38178", span_at_chirho(40))]
    );
    assert!(
        errors_chirho(
            anyclass_describe_chirho(),
            &["DerivingStrategies", "DeriveAnyClass"]
        )
        .is_empty()
    );
}

#[test]
fn an_unwritten_strategy_derives_a_non_stock_class_only_with_an_extension_chirho() {
    // B1: a data type needs DeriveAnyClass (GHC-00158); B2 accepts with it.
    let describe_data_chirho =
        || vec![data_chirho(vec![class_with_chirho("DescribeChirho", None)])];
    assert_eq!(
        codes_at_chirho(&errors_chirho(describe_data_chirho(), &[])),
        [("GHC-00158", span_at_chirho(40))]
    );
    assert!(errors_chirho(describe_data_chirho(), &["DeriveAnyClass"]).is_empty());
    // C1 and C4: a newtype needs GND or DeriveAnyClass (GHC-82023); C2 and C5 accept.
    let num_newtype_chirho = || vec![newtype_chirho(vec![class_with_chirho("Num", None)])];
    for extensions_chirho in [&["Haskell2010"][..], &["NoGeneralizedNewtypeDeriving"][..]] {
        assert_eq!(
            codes_at_chirho(&errors_chirho(num_newtype_chirho(), extensions_chirho)),
            [("GHC-82023", span_at_chirho(40))],
            "{extensions_chirho:?}"
        );
    }
    for extensions_chirho in [
        &[][..],
        &["Haskell2010", "GeneralizedNewtypeDeriving"][..],
        &["Haskell2010", "DeriveAnyClass"][..],
    ] {
        assert!(
            errors_chirho(num_newtype_chirho(), extensions_chirho).is_empty(),
            "{extensions_chirho:?}"
        );
    }
    // A stock derivable class needs neither.
    assert!(
        errors_chirho(
            vec![data_chirho(vec![class_with_chirho("Show", None)])],
            &["Haskell2010"]
        )
        .is_empty()
    );
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
        &["Haskell2010", "DerivingStrategies"],
    );
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [
            ("GHC-10333", span_at_chirho(50)),
            ("GHC-26557", span_at_chirho(50))
        ]
    );
}
