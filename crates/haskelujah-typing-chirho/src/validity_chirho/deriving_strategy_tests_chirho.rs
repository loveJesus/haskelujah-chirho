// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rules 7 to 15 on hand-built AST: each refusal at GHC's caret, and the strategies, editions
//! and extensions that must stay accepted (row 694; root's fresh GHC 9.14.1 contracts and the
//! boundary probes measured at the cf6f7976 gate). The extension lists are as the parser leaves
//! them, written flags after any edition.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DerivedClassChirho, DerivingStrategyChirho, StrictnessChirho,
};
use haskelujah_ast_chirho::name_chirho::{BuiltinClassChirho, CompilerClassChirho, RawNameChirho};
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
        prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
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
    errors_with_chirho(
        decls_chirho,
        extensions_chirho,
        &BuiltinClassScopeChirho::default(),
    )
}

fn errors_with_chirho(
    decls_chirho: Vec<DeclChirho>,
    extensions_chirho: &[&str],
    builtin_classes_chirho: &BuiltinClassScopeChirho,
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
    check_deriving_strategies_chirho(&module_chirho, builtin_classes_chirho, &mut errors_chirho);
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
        let accepted_chirho = errors_with_chirho(
            vec![data_chirho(vec![class_with_chirho("Show", stock_chirho())])],
            extensions_chirho,
            &prelude_scope_chirho(&[CompilerClassChirho::ShowChirho]),
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

/// `class <name> <parameters>` with no methods.
fn class_decl_chirho(class_chirho: &str, parameters_chirho: &[&str]) -> DeclChirho {
    DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: name_chirho(class_chirho),
        type_vars_chirho: parameters_chirho
            .iter()
            .map(|parameter_chirho| {
                haskelujah_ast_chirho::decl_chirho::TyVarChirho::plain_chirho(name_chirho(
                    parameter_chirho,
                ))
            })
            .collect(),
        methods_chirho: vec![],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `class_chirho` applied to `Int`, the class at offset 40.
fn applied_to_int_chirho(
    class_chirho: &str,
    strategy_chirho: Option<DerivingStrategyChirho>,
) -> DerivedClassChirho {
    DerivedClassChirho::applied_chirho(
        name_at_chirho(class_chirho, span_at_chirho(40)),
        vec![TypeChirho::ConChirho(name_chirho("Int"))],
        strategy_chirho,
    )
}

#[test]
fn a_class_application_must_leave_one_parameter_chirho() {
    // W2 (GHC-73993): a one-parameter class applied to an argument; W1, its twin, accepts.
    let found_chirho = errors_chirho(
        vec![
            class_decl_chirho("MarkChirho", &["valueChirho"]),
            data_chirho(vec![applied_to_int_chirho("MarkChirho", anyclass_chirho())]),
        ],
        &["DerivingStrategies", "DeriveAnyClass"],
    );
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [("GHC-73993", span_at_chirho(40))]
    );
    assert!(
        found_chirho[0]
            .message_chirho
            .contains("`MarkChirho Int` is not a unary constraint")
    );
    assert!(
        errors_chirho(
            vec![
                class_decl_chirho("PairChirho", &["keyChirho", "valueChirho"]),
                data_chirho(vec![applied_to_int_chirho("PairChirho", anyclass_chirho())]),
            ],
            &["DerivingStrategies", "DeriveAnyClass"],
        )
        .is_empty()
    );
    // A stock class takes no argument, with or without a written strategy.
    for strategy_chirho in [stock_chirho(), None] {
        let found_chirho = errors_with_chirho(
            vec![data_chirho(vec![applied_to_int_chirho(
                "Eq",
                strategy_chirho,
            )])],
            &["DerivingStrategies"],
            &prelude_scope_chirho(&[CompilerClassChirho::EqChirho]),
        );
        assert_eq!(
            codes_at_chirho(&found_chirho),
            [("GHC-73993", span_at_chirho(40))]
        );
    }
}

#[test]
fn the_stock_strategy_needs_the_builtin_class_itself_chirho() {
    // S1 (GHC-00158): `Eq` names the module's own class.
    let local_eq_chirho = || class_decl_chirho("Eq", &["valueChirho"]);
    let found_chirho = errors_chirho(
        vec![
            local_eq_chirho(),
            data_chirho(vec![class_with_chirho("Eq", stock_chirho())]),
        ],
        &["DerivingStrategies"],
    );
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [("GHC-00158", span_at_chirho(40))]
    );
    assert!(
        found_chirho[0]
            .message_chirho
            .contains("with the stock strategy")
    );
    // S2: `P.Eq` is the Prelude's class, proved by naming, beside the same local class.
    let builtin_chirho = BuiltinClassScopeChirho::new_chirho(HashMap::from([(
        "P.Eq".to_string(),
        BuiltinClassChirho {
            identity_chirho: CompilerClassChirho::EqChirho,
            module_chirho: "GHC.Classes",
            name_chirho: "Eq",
        },
    )]));
    let prelude_eq_chirho = DerivedClassChirho::new_chirho(
        NameChirho::RawChirho(RawNameChirho::qualified_chirho(
            "P",
            "Eq",
            span_at_chirho(40),
        )),
        stock_chirho(),
    );
    assert!(
        errors_with_chirho(
            vec![local_eq_chirho(), data_chirho(vec![prelude_eq_chirho])],
            &["DerivingStrategies"],
            &builtin_chirho,
        )
        .is_empty()
    );
}

#[test]
fn a_newtype_class_with_arguments_is_an_unsupported_gap_chirho() {
    // W3: GHC accepts it; the GND dictionary keys one head type, so Haskelujah refuses it by name.
    let scale_chirho = || class_decl_chirho("ScaleChirho", &["factorChirho", "valueChirho"]);
    for strategy_chirho in [newtype_strategy_chirho(), None] {
        let found_chirho = errors_chirho(
            vec![
                scale_chirho(),
                newtype_chirho(vec![applied_to_int_chirho("ScaleChirho", strategy_chirho)]),
            ],
            &["DerivingStrategies", "GeneralizedNewtypeDeriving"],
        );
        assert_eq!(
            codes_at_chirho(&found_chirho),
            [(UNSUPPORTED_SHAPE_CHIRHO, span_at_chirho(40))]
        );
    }
    // DeriveAnyClass takes the unwritten class instead, over the full head.
    assert!(
        errors_chirho(
            vec![
                scale_chirho(),
                newtype_chirho(vec![applied_to_int_chirho("ScaleChirho", None)]),
            ],
            &["DeriveAnyClass", "GeneralizedNewtypeDeriving"],
        )
        .is_empty()
    );
}

#[test]
fn a_standalone_head_derives_for_its_last_type_chirho() {
    let standalone_chirho = |strategy_chirho: Option<DerivingStrategyChirho>,
                             class_chirho: &str| {
        DeclChirho::StandaloneDerivingDeclChirho {
            strategy_chirho,
            context_chirho: vec![],
            class_chirho: name_at_chirho(class_chirho, span_at_chirho(50)),
            types_chirho: vec![
                TypeChirho::ConChirho(name_chirho("Int")),
                TypeChirho::ConChirho(name_chirho("BadChirho")),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    };
    let found_chirho = errors_chirho(
        vec![
            class_decl_chirho("PairChirho", &["keyChirho", "valueChirho"]),
            data_chirho(vec![]),
            standalone_chirho(anyclass_chirho(), "PairChirho"),
            standalone_chirho(None, "Eq"),
            standalone_chirho(newtype_strategy_chirho(), "PairChirho"),
        ],
        &["DerivingStrategies", "DeriveAnyClass"],
    );
    // `PairChirho Int BadChirho` derives; `Eq Int BadChirho` gives Eq two types; the newtype
    // strategy on the data type `BadChirho` is GHC-10333.
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [
            (UNMEASURED_GHC_CODE_CHIRHO, span_at_chirho(50)),
            ("GHC-10333", span_at_chirho(50)),
        ]
    );
}

/// A scope proving each class's bare spelling, as the implicit Prelude import does.
fn prelude_scope_chirho(classes_chirho: &[CompilerClassChirho]) -> BuiltinClassScopeChirho {
    BuiltinClassScopeChirho::new_chirho(
        classes_chirho
            .iter()
            .map(|&class_chirho| {
                (
                    class_chirho.legacy_name_chirho().to_string(),
                    BuiltinClassChirho {
                        identity_chirho: class_chirho,
                        module_chirho: "Prelude",
                        name_chirho: class_chirho.legacy_name_chirho(),
                    },
                )
            })
            .collect(),
    )
}

#[test]
fn an_explicit_stock_needs_the_builtin_proof_chirho() {
    // #31786: a spelling K01 cannot judge is a named gap, never a presumed stock class.
    let qualified_eq_chirho = DerivedClassChirho::new_chirho(
        NameChirho::RawChirho(RawNameChirho::qualified_chirho(
            "M",
            "Eq",
            span_at_chirho(40),
        )),
        stock_chirho(),
    );
    let found_chirho = errors_chirho(
        vec![data_chirho(vec![
            class_with_chirho("Read", stock_chirho()),
            qualified_eq_chirho,
        ])],
        &["DerivingStrategies"],
    );
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [
            (UNSUPPORTED_SHAPE_CHIRHO, span_at_chirho(40)),
            (UNSUPPORTED_SHAPE_CHIRHO, span_at_chirho(40)),
        ]
    );
    assert!(found_chirho[1].message_chirho.contains("`M.Eq`"));
    // A proven builtin class GHC does not derive with the stock strategy is GHC's refusal.
    let found_chirho = errors_with_chirho(
        vec![data_chirho(vec![class_with_chirho("Num", stock_chirho())])],
        &["DerivingStrategies"],
        &prelude_scope_chirho(&[CompilerClassChirho::NumChirho]),
    );
    assert_eq!(
        codes_at_chirho(&found_chirho),
        [("GHC-00158", span_at_chirho(40))]
    );
    // An unwritten strategy keeps the legacy route: no refusal for Read.
    assert!(
        errors_chirho(
            vec![data_chirho(vec![class_with_chirho("Read", None)])],
            &["DerivingStrategies"],
        )
        .is_empty()
    );
}
