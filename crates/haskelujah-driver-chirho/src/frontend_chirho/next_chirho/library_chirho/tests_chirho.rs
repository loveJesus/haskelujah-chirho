// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::super::{check_source_chirho, eval_binding_chirho};
use super::*;

#[test]
fn actual_tuple_slice_executes_and_missing_swap_declines_chirho() {
    let source_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule TupleUseChirho where\nimport GHC.Internal.Data.Tuple (fst, snd)\nresultChirho = fst ('t','f')\n";
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 50_000).unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('t')
    );
    let error_chirho = check_source_chirho(
        &source_chirho
            .replace("(fst, snd)", "(swap)")
            .replace("fst (", "swap ("),
    )
    .unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
}
#[test]
fn manifest_refuses_changed_bytes_and_bad_spans_chirho() {
    let changed_chirho = SourceManifestChirho {
        bytes_chirho: "fst x = x",
        ..TUPLE_CHIRHO
    };
    assert!(
        changed_chirho
            .source_chirho()
            .unwrap_err()
            .message_chirho
            .contains("hash mismatch")
    );
    let changed_chirho = SourceManifestChirho {
        spans_chirho: &[(926, usize::MAX)],
        ..TUPLE_CHIRHO
    };
    assert!(
        changed_chirho
            .source_chirho()
            .unwrap_err()
            .message_chirho
            .contains("byte span")
    );
}
#[test]
fn slice_does_not_turn_known_mismatch_into_absence_chirho() {
    let error_chirho = check_source_chirho(
        "{-# LANGUAGE NoImplicitPrelude #-}\nmodule TupleBadChirho where\nimport GHC.Internal.Data.Tuple (fst, snd)\nresultChirho = fst 'x'\n",
    )
    .unwrap_err();
    assert_eq!(error_chirho.kind_chirho, NextErrorKindChirho::TypeChirho);
}

#[test]
fn actual_num_default_executes_at_home_type_chirho() {
    let source_chirho = include_str!("fixtures-chirho/NumDefaultChirho.hs");
    let checked_chirho = check_source_chirho(source_chirho).unwrap();
    assert!(
        checked_chirho
            .module_chirho
            .sliced_units_chirho
            .contains(&UnitIdentityChirho::CompilerChirho)
    );
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 100_000).unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('b')
    );
}
#[test]
fn ordinary_literal_never_uses_lexical_from_integer_chirho() {
    let error_chirho =
        check_source_chirho(include_str!("fixtures-chirho/LexicalNumericChirho.hs")).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    assert!(
        error_chirho
            .message_chirho
            .contains("incomplete source unit"),
        "{error_chirho}"
    );
}
#[test]
fn spelling_grouped_operator_chain_is_unsupported_chirho() {
    let error_chirho =
        check_source_chirho(include_str!("fixtures-chirho/FixityChainChirho.hs")).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    assert!(
        error_chirho.message_chirho.contains("fixity regrouping"),
        "{error_chirho}"
    );
}

#[test]
fn known_operations_ignore_home_numeric_spellings_chirho() {
    let source_chirho = include_str!("fixtures-chirho/NumDefaultChirho.hs").replace(
        "resultChirho = pickChirho (5 - 7)",
        "fromInteger _ = 'x'\nnegate _ = BChirho\nresultChirho = pickChirho (-5)",
    );
    let (value_chirho, _) = eval_binding_chirho(&source_chirho, "resultChirho", 100_000).unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('a')
    );
}
#[test]
fn same_spelled_home_module_does_not_own_known_num_chirho() {
    let source_chirho = include_str!("fixtures-chirho/LexicalNumericChirho.hs")
        .replace("module LexicalNumericChirho", "module GHC.Internal.Num");
    let error_chirho = check_source_chirho(&source_chirho).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    assert!(
        error_chirho
            .message_chirho
            .contains("incomplete source unit"),
        "{error_chirho}"
    );
}
#[test]
fn defaulting_and_unseen_library_instances_decline_chirho() {
    for source_chirho in [
        "module DefaultingChirho where\nresultChirho = 5\n",
        "module LibraryInstanceChirho where\nimport GHC.Internal.Num (Num(..))\nresultChirho :: Char\nresultChirho = fromInteger 5\n",
    ] {
        let error_chirho = check_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{error_chirho}"
        );
        assert!(
            error_chirho
                .message_chirho
                .contains("incomplete source unit")
                || error_chirho
                    .message_chirho
                    .contains("checked defining interface"),
            "{error_chirho}"
        );
    }
}
#[test]
fn a_visible_duplicate_at_partial_owners_is_a_presence_refusal_chirho() {
    let source_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule PartialIntegerChirho where\nimport GHC.Internal.Num (Num(..))\nimport GHC.Internal.Bignum.Integer (Integer)\ninstance Num Integer where\n fromInteger xChirho = xChirho\n xChirho + _ = xChirho\n _ * yChirho = yChirho\n abs xChirho = xChirho\n signum xChirho = xChirho\n negate xChirho = xChirho\nresultChirho = 'i'\n";
    let error_chirho = check_source_chirho(source_chirho).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::TypeChirho,
        "{error_chirho}"
    );
    assert!(
        error_chirho.message_chirho.contains("duplicate"),
        "{error_chirho}"
    );
}

#[test]
fn infix_and_exported_fixity_follow_original_name_chirho() {
    let source_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule OperatorChirho where\ninfixr 4 +\nxChirho + _ = xChirho\nresultChirho = 't' + 'f'\n";
    let checked_chirho = check_source_chirho(source_chirho).unwrap();
    let interface_chirho =
        haskelujah_tc_chirho::project_interface_chirho(&checked_chirho, &mut 10_000).unwrap();
    let name_chirho = OriginalNameChirho {
        unit_chirho: UnitIdentityChirho::StandaloneHomeChirho,
        module_chirho: "OperatorChirho".into(),
        namespace_chirho: NamespaceChirho::ValueChirho,
        occurrence_chirho: "+".into(),
    };
    assert_eq!(
        interface_chirho.fixities_chirho[&name_chirho],
        InterfaceFixityChirho {
            associativity_chirho: AssociativityChirho::RightChirho,
            precedence_chirho: 4
        }
    );
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 50_000).unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('t')
    );
}
#[test]
fn class_pragmas_keep_later_defaults_and_validate_their_owner_chirho() {
    let source_chirho = "module PragmasChirho where\nclass EchoChirho aChirho where\n {-# MINIMAL echoChirho #-}\n echoChirho :: aChirho -> Char\n {-# INLINE echoChirho #-}\n echoChirho _ = 'p'\n laterChirho :: aChirho -> Char\n laterChirho xChirho = echoChirho xChirho\ninstance EchoChirho Bool\nresultChirho = laterChirho True\n";
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 50_000).unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('p')
    );
    for invalid_chirho in [
        source_chirho.replace("MINIMAL echoChirho", "MINIMAL missingChirho"),
        source_chirho.replace("INLINE echoChirho", "INLINE missingChirho"),
        source_chirho.replace(" echoChirho _ = 'p'\n", ""),
    ] {
        assert_eq!(
            check_source_chirho(&invalid_chirho)
                .unwrap_err()
                .kind_chirho,
            NextErrorKindChirho::InvalidChirho
        );
    }
}

#[test]
fn nested_and_mid_type_pragmas_do_not_gain_class_ownership_chirho() {
    for source_chirho in [
        "module NestedPragmaChirho where\nclass CChirho aChirho where\n mChirho :: aChirho -> Char\n mChirho xChirho = let { nChirho _ = 'x'; {-# INLINE nChirho #-} } in nChirho xChirho\n",
        "module MidTypePragmaChirho where\nclass CChirho aChirho where\n mChirho :: aChirho -> {-# INLINE mChirho #-} Char\n mChirho _ = 'x'\n",
    ] {
        let error_chirho = check_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{error_chirho}"
        );
    }
}

#[test]
fn imported_default_negate_and_signed_operators_execute_chirho() {
    for (source_chirho, expected_chirho) in [
        (
            include_str!("fixtures-chirho/positive-chirho/NumNegateDefaultChirho.hs"),
            'a',
        ),
        (
            include_str!("fixtures-chirho/positive-chirho/OperatorSignatureChirho.hs"),
            's',
        ),
        (
            include_str!("fixtures-chirho/positive-chirho/TupleSndChirho.hs"),
            'f',
        ),
    ] {
        let (value_chirho, _) =
            eval_binding_chirho(source_chirho, "resultChirho", 100_000).unwrap();
        assert_eq!(
            value_chirho,
            haskelujah_runtime_chirho::ValueChirho::CharChirho(expected_chirho)
        );
    }
}

#[test]
fn open_slice_scope_and_absent_explicit_items_are_unsupported_chirho() {
    let source_chirho = include_str!("fixtures-chirho/NumDefaultChirho.hs");
    for declined_chirho in [
        source_chirho.replace("(Num(..))", ""),
        source_chirho.replace("(Num(..))", "(Num(..), subtract)"),
    ] {
        assert_eq!(
            check_source_chirho(&declined_chirho)
                .unwrap_err()
                .kind_chirho,
            NextErrorKindChirho::UnsupportedChirho
        );
    }
}
#[test]
fn top_level_import_clashes_are_use_site_errors_and_nested_names_shadow_chirho() {
    let library_chirho = "module AChirho where\nfChirho _ = 'i'\n";
    let unused_chirho =
        "module BChirho where\nimport AChirho\nfChirho _ = 'h'\nresultChirho = 'u'\n";
    let used_chirho = unused_chirho.replace("resultChirho = 'u'", "resultChirho = fChirho 'x'");
    assert_eq!(
        super::super::check_sources_chirho(&[library_chirho, &used_chirho], "BChirho")
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::InvalidChirho
    );
    super::super::check_sources_chirho(&[library_chirho, unused_chirho], "BChirho").unwrap();
    let nested_chirho =
        "module BChirho where\nimport AChirho\nresultChirho = let fChirho _ = 'n' in fChirho 'x'\n";
    let (value_chirho, _) = super::super::eval_sources_binding_chirho(
        &[library_chirho, nested_chirho],
        "BChirho",
        "resultChirho",
        50_000,
    )
    .unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('n')
    );
}
#[test]
fn home_prelude_is_not_replaced_by_wired_scope_chirho() {
    let error_chirho = super::super::check_sources_chirho(
        &[
            "module Prelude where\nxChirho = 'p'\n",
            "module UserChirho where\nresultChirho = True\n",
        ],
        "UserChirho",
    )
    .unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    assert!(error_chirho.message_chirho.contains("home Prelude"));
}
#[test]
fn manifest_cannot_cut_off_num_defaults_chirho() {
    let changed_chirho = SourceManifestChirho {
        // A full line boundary still cannot end before this class's equations.
        spans_chirho: &[(1066, 1093), (2395, 3231)],
        ..NUM_CHIRHO
    };
    assert!(
        changed_chirho
            .source_chirho()
            .unwrap_err()
            .message_chirho
            .contains("truncates a declaration")
    );
}

#[test]
fn hidden_imported_methods_cannot_be_implemented_by_spelling_chirho() {
    let library_chirho = "module HiddenClassChirho where\nclass CChirho aChirho where\n mChirho :: aChirho -> Char\n mChirho _ = 'd'\n";
    let user_chirho = "module HiddenUseChirho where\nimport HiddenClassChirho (CChirho)\ndata TChirho = TChirho\ninstance CChirho TChirho where\n mChirho _ = 'x'\nresultChirho = 'r'\n";
    let error_chirho =
        super::super::check_sources_chirho(&[library_chirho, user_chirho], "HiddenUseChirho")
            .unwrap_err();
    assert_eq!(error_chirho.kind_chirho, NextErrorKindChirho::InvalidChirho);
    assert!(error_chirho.message_chirho.contains("visible method"));
}
#[test]
fn trailing_and_misindented_pragmas_are_not_class_members_chirho() {
    for source_chirho in [
        "module TrailingPragmaChirho where\nclass CChirho aChirho where\n mChirho :: aChirho -> Char\n mChirho _ = 'x'\n{-# INLINE mChirho #-}\n",
        "module NestedMinimalChirho where\nclass CChirho aChirho where\n mChirho :: aChirho -> Char\n mChirho xChirho = hChirho xChirho where\n  {-# MINIMAL mChirho #-}\n  hChirho _ = 'x'\n",
        "module MisindentedPragmaChirho where\nclass CChirho aChirho where\n    {-# MINIMAL mChirho #-}\n mChirho :: aChirho -> Char\n mChirho _ = 'x'\n",
    ] {
        let error_chirho = check_source_chirho(source_chirho).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{error_chirho}"
        );
    }
}

#[test]
fn class_fixity_is_not_silently_discarded_chirho() {
    let error_chirho = check_source_chirho("module ClassFixityChirho where\nclass CChirho aChirho where\n infixr 4 <+>\n (<+>) :: aChirho -> aChirho -> aChirho\n").unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
}

#[test]
fn prelude_names_certify_clashes_but_never_supply_a_type_chirho() {
    let exports_chirho = prelude_exports_chirho().unwrap();
    for (name_chirho, namespace_chirho) in [
        ("not", NamespaceChirho::ValueChirho),
        ("+", NamespaceChirho::ValueChirho),
        ("map", NamespaceChirho::ValueChirho),
        ("++", NamespaceChirho::ValueChirho),
        ("True", NamespaceChirho::ValueChirho),
        ("~", NamespaceChirho::TypeChirho),
        ("Num", NamespaceChirho::TypeChirho),
    ] {
        assert!(
            exports_chirho.contains(&(name_chirho.into(), namespace_chirho)),
            "{name_chirho}"
        );
    }
    let source_chirho = "module PreludeClashChirho where\nnot :: Bool -> Bool\nnot True = False\nnot False = True\nresultChirho = not True\n";
    let error_chirho = check_source_chirho(source_chirho).unwrap_err();
    assert_eq!(error_chirho.kind_chirho, NextErrorKindChirho::InvalidChirho);
    let library_chirho = "module AChirho where\nfChirho _ = 'a'\n";
    let user_chirho = "module BChirho where\nimport AChirho\nresultChirho = not True\n";
    assert_eq!(
        super::super::check_sources_chirho(&[library_chirho, user_chirho], "BChirho")
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    let nested_chirho =
        "module NestedNotChirho where\nresultChirho = let not xChirho = xChirho in not True\n";
    check_source_chirho(nested_chirho).unwrap();
}

#[test]
fn manifest_cannot_take_only_a_functions_signature_chirho() {
    let changed_chirho = SourceManifestChirho {
        spans_chirho: &[(926, 964)],
        ..TUPLE_CHIRHO
    };
    assert!(
        changed_chirho
            .source_chirho()
            .unwrap_err()
            .message_chirho
            .contains("equation group")
    );
}
#[test]
fn pragma_member_tails_and_explicit_delimiters_are_checked_chirho() {
    let source_chirho = "module TrailingDefaultInlineChirho where\nclass CChirho aChirho where\n mChirho :: aChirho -> Char\n mChirho _ = 'p'\n {-# INLINE mChirho #-}\ninstance CChirho Bool\nresultChirho = mChirho True\n";
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 50_000).unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('p')
    );
    let error_chirho = check_source_chirho("module MissingDelimiterChirho where\nclass CChirho aChirho where { mChirho :: aChirho -> Char {-# INLINE mChirho #-}; mChirho _ = 'x' }\n").unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
}

#[test]
fn all_eight_accepted_oracle_programs_execute_byte_identical_sources_chirho() {
    for (source_chirho, expected_chirho) in [
        (include_str!("fixtures-chirho/NumDefaultChirho.hs"), 'b'),
        (
            include_str!("fixtures-chirho/positive-chirho/TupleUseChirho.hs"),
            't',
        ),
        (
            include_str!("fixtures-chirho/positive-chirho/TupleSndChirho.hs"),
            'f',
        ),
        (
            include_str!("fixtures-chirho/positive-chirho/OperatorFixityChirho.hs"),
            't',
        ),
        (
            include_str!("fixtures-chirho/positive-chirho/OperatorSignatureChirho.hs"),
            's',
        ),
        (
            include_str!("fixtures-chirho/positive-chirho/ClassPragmasChirho.hs"),
            'p',
        ),
        (
            include_str!("fixtures-chirho/positive-chirho/NumShadowOpsChirho.hs"),
            'a',
        ),
        (
            include_str!("fixtures-chirho/positive-chirho/NumNegateDefaultChirho.hs"),
            'a',
        ),
    ] {
        let (value_chirho, _) =
            eval_binding_chirho(source_chirho, "resultChirho", 100_000).unwrap();
        assert_eq!(
            value_chirho,
            haskelujah_runtime_chirho::ValueChirho::CharChirho(expected_chirho)
        );
    }
}

#[test]
fn self_qualification_and_wired_constructor_forms_never_gain_absence_authority_chirho() {
    let library_chirho = "module AChirho where\naChirho :: Bool\naChirho = True\n";
    for user_chirho in [
        "module BChirho where\nimport AChirho\nnot :: Bool -> Bool\nnot bChirho = bChirho\ncChirho :: Bool\ncChirho = BChirho.not aChirho\n",
        "module BChirho where\nimport AChirho\nxsChirho :: [Bool]\nxsChirho = aChirho : []\n",
    ] {
        assert_eq!(
            super::super::check_sources_chirho(&[library_chirho, user_chirho], "BChirho")
                .unwrap_err()
                .kind_chirho,
            NextErrorKindChirho::UnsupportedChirho
        );
    }
}
#[test]
fn manifest_cannot_begin_at_a_later_equation_chirho() {
    let changed_chirho = SourceManifestChirho {
        spans_chirho: &[(964, 993)],
        ..TUPLE_CHIRHO
    };
    assert!(
        changed_chirho
            .source_chirho()
            .unwrap_err()
            .message_chirho
            .contains("equation group")
    );
}
