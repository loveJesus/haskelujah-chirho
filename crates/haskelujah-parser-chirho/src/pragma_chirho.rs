// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! LANGUAGE pragma extraction shared by pre-layout token classification and CST lowering.
//!
//! `mdo` and `rec` are contextual keywords under `RecursiveDo`, but layout runs before CST
//! lowering. The raw lexer therefore emits both words as ordinary variable identifiers. This
//! module reads only actual pragma tokens, computes the extension state, and reclassifies those
//! two words before layout. Strings and ordinary comments cannot accidentally enable an extension.
//!
//! workflow: recursive-do-chirho

use crate::lexer_chirho::{RawTokenChirho, RawTokenKindChirho};

/// Extensions enabled by GHC2021.
///
/// Haskelujah currently treats GHC2024 as this baseline too; a distinct GHC2024 expansion belongs
/// in the broader extension-versioning work rather than the RecursiveDo slice.
const GHC2021_EXTENSIONS_CHIRHO: &[&str] = &[
    "BangPatterns",
    "BinaryLiterals",
    "ConstrainedClassMethods",
    "ConstraintKinds",
    "DeriveDataTypeable",
    "DeriveFoldable",
    "DeriveFunctor",
    "DeriveGeneric",
    "DeriveLift",
    "DeriveTraversable",
    "DoAndIfThenElse",
    "EmptyCase",
    "EmptyDataDecls",
    "EmptyDataDeriving",
    "ExistentialQuantification",
    "ExplicitForAll",
    "FlexibleContexts",
    "FlexibleInstances",
    "ForeignFunctionInterface",
    "GADTSyntax",
    "GeneralizedNewtypeDeriving",
    "HexFloatLiterals",
    "ImportQualifiedPost",
    "InstanceSigs",
    "KindSignatures",
    "MultiParamTypeClasses",
    "NamedFieldPuns",
    "NamedWildCards",
    "NumericUnderscores",
    "PolyKinds",
    "PostfixOperators",
    "RankNTypes",
    "ScopedTypeVariables",
    "StandaloneDeriving",
    "StandaloneKindSignatures",
    "TupleSections",
    "TypeApplications",
    "TypeOperators",
    "TypeSynonymInstances",
];

/// Parse the inner text between `{-#` and `#-}` into extension flags.
///
/// LANGUAGE names preserve source order because `NoFoo` must be able to override an earlier
/// `Foo`. OPTIONS and OPTIONS_GHC contribute their `-X...` flags through the same path.
pub fn pragma_extensions_from_inner_chirho(inner_chirho: &str) -> Vec<String> {
    let inner_chirho = inner_chirho.trim();
    let Some((keyword_chirho, rest_chirho)) = inner_chirho.split_once(char::is_whitespace) else {
        return Vec::new();
    };

    if keyword_chirho.eq_ignore_ascii_case("LANGUAGE") {
        return rest_chirho
            .split(',')
            .flat_map(|extension_chirho| {
                let extension_chirho = extension_chirho.trim();
                if matches!(extension_chirho, "GHC2021" | "GHC2024") {
                    GHC2021_EXTENSIONS_CHIRHO
                        .iter()
                        .map(|extension_chirho| (*extension_chirho).to_string())
                        .collect::<Vec<_>>()
                } else if extension_chirho.is_empty() {
                    Vec::new()
                } else {
                    vec![extension_chirho.to_string()]
                }
            })
            .collect();
    }

    if keyword_chirho.eq_ignore_ascii_case("OPTIONS_GHC")
        || keyword_chirho.eq_ignore_ascii_case("OPTIONS")
    {
        return rest_chirho
            .split_whitespace()
            .filter_map(|word_chirho| word_chirho.strip_prefix("-X"))
            .filter(|extension_chirho| !extension_chirho.is_empty())
            .map(str::to_string)
            .collect();
    }

    Vec::new()
}

/// Parse a complete pragma token such as `{-# LANGUAGE RecursiveDo #-}`.
pub fn pragma_extensions_from_text_chirho(text_chirho: &str) -> Vec<String> {
    let inner_chirho = text_chirho
        .strip_prefix("{-#")
        .and_then(|text_chirho| text_chirho.strip_suffix("#-}"))
        .unwrap_or("");
    pragma_extensions_from_inner_chirho(inner_chirho)
}

/// Collect extension flags from actual raw pragma tokens.
pub fn collect_raw_pragma_extensions_chirho(
    source_chirho: &str,
    tokens_chirho: &[RawTokenChirho],
) -> Vec<String> {
    tokens_chirho
        .iter()
        .filter(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::PragmaChirho)
        .flat_map(|token_chirho| {
            let start_chirho = token_chirho.span_chirho.start_chirho().as_usize_chirho();
            let end_chirho = token_chirho.span_chirho.end_chirho().as_usize_chirho();
            source_chirho
                .get(start_chirho..end_chirho)
                .map(pragma_extensions_from_text_chirho)
                .unwrap_or_default()
        })
        .collect()
}

/// Evaluate an extension flag sequence with GHC-style `NoFoo` override order.
pub fn extension_enabled_chirho(extensions_chirho: &[String], extension_name_chirho: &str) -> bool {
    let disabled_name_chirho = format!("No{extension_name_chirho}");
    extensions_chirho
        .iter()
        .fold(false, |enabled_chirho, flag_chirho| {
            if flag_chirho == extension_name_chirho {
                true
            } else if flag_chirho == &disabled_name_chirho {
                false
            } else {
                enabled_chirho
            }
        })
}

/// Reclassify extension-controlled contextual words after raw lexing and before layout.
///
/// The raw lexer intentionally leaves `mdo`/`rec` as identifiers and `Module.do` as a qualified
/// identifier. This pass is the only place that turns them into layout-sensitive keywords.
/// workflow: recursive-do-chirho, language-features-chirho/qualified-do-chirho
pub fn classify_contextual_keywords_chirho(
    source_chirho: &str,
    tokens_chirho: &mut [RawTokenChirho],
) -> Vec<String> {
    let extensions_chirho = collect_raw_pragma_extensions_chirho(source_chirho, tokens_chirho);
    let recursive_do_enabled_chirho = extension_enabled_chirho(&extensions_chirho, "RecursiveDo");
    let qualified_do_enabled_chirho = extension_enabled_chirho(&extensions_chirho, "QualifiedDo");
    if !recursive_do_enabled_chirho && !qualified_do_enabled_chirho {
        return extensions_chirho;
    }

    for token_chirho in tokens_chirho {
        let start_chirho = token_chirho.span_chirho.start_chirho().as_usize_chirho();
        let end_chirho = token_chirho.span_chirho.end_chirho().as_usize_chirho();
        let Some(text_chirho) = source_chirho.get(start_chirho..end_chirho) else {
            continue;
        };

        if recursive_do_enabled_chirho
            && token_chirho.kind_chirho == RawTokenKindChirho::VarIdChirho
        {
            match text_chirho {
                "mdo" => token_chirho.kind_chirho = RawTokenKindChirho::DoChirho,
                "rec" => token_chirho.kind_chirho = RawTokenKindChirho::RecChirho,
                _ => {}
            }
        } else if qualified_do_enabled_chirho
            && token_chirho.kind_chirho == RawTokenKindChirho::QualifiedIdChirho
            && text_chirho
                .rsplit_once('.')
                .is_some_and(|(qualifier_chirho, local_chirho)| {
                    !qualifier_chirho.is_empty() && local_chirho == "do"
                })
        {
            token_chirho.kind_chirho = RawTokenKindChirho::DoChirho;
        }
    }

    extensions_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::lexer_chirho::LexerChirho;
    use haskelujah_span_chirho::FileIdChirho;

    fn classified_kinds_chirho(source_chirho: &str) -> Vec<RawTokenKindChirho> {
        let mut lexer_chirho =
            LexerChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
        let mut tokens_chirho = lexer_chirho.lex_all_chirho();
        classify_contextual_keywords_chirho(source_chirho, &mut tokens_chirho);
        tokens_chirho
            .into_iter()
            .filter(|token_chirho| !token_chirho.kind_chirho.is_trivia_chirho())
            .map(|token_chirho| token_chirho.kind_chirho)
            .collect()
    }

    #[test]
    fn parses_language_and_options_extensions_chirho() {
        assert_eq!(
            pragma_extensions_from_inner_chirho("Language GADTs, RecursiveDo"),
            vec!["GADTs".to_string(), "RecursiveDo".to_string()]
        );
        assert_eq!(
            pragma_extensions_from_inner_chirho("OPTIONS_GHC -Wall -XRecursiveDo"),
            vec!["RecursiveDo".to_string()]
        );
        assert!(pragma_extensions_from_inner_chirho("LANGUAGEISH RecursiveDo").is_empty());
    }

    #[test]
    fn recursive_do_words_are_contextual_chirho() {
        let plain_chirho = classified_kinds_chirho("module M where\nmdo = rec\n");
        assert_eq!(
            plain_chirho
                .iter()
                .filter(|kind_chirho| **kind_chirho == RawTokenKindChirho::VarIdChirho)
                .count(),
            2
        );

        let enabled_chirho = classified_kinds_chirho(
            "{-# LANGUAGE RecursiveDo #-}\nmodule M where\nmain = mdo\n  rec\n    x <- pure x\n  pure x\n",
        );
        assert!(enabled_chirho.contains(&RawTokenKindChirho::DoChirho));
        assert!(enabled_chirho.contains(&RawTokenKindChirho::RecChirho));
    }

    #[test]
    fn no_recursive_do_overrides_recursive_do_chirho() {
        let kinds_chirho = classified_kinds_chirho(
            "{-# LANGUAGE RecursiveDo, NoRecursiveDo #-}\nmodule M where\nmdo = rec\n",
        );
        assert!(!kinds_chirho.contains(&RawTokenKindChirho::RecChirho));
        assert_eq!(
            kinds_chirho
                .iter()
                .filter(|kind_chirho| **kind_chirho == RawTokenKindChirho::VarIdChirho)
                .count(),
            2
        );
    }

    #[test]
    fn qualified_do_word_is_contextual_chirho() {
        let plain_chirho = classified_kinds_chirho("module M where\nmain = FlowChirho.do\n");
        assert!(plain_chirho.contains(&RawTokenKindChirho::QualifiedIdChirho));
        assert!(!plain_chirho.contains(&RawTokenKindChirho::DoChirho));

        let enabled_chirho = classified_kinds_chirho(
            "{-# LANGUAGE QualifiedDo #-}\nmodule M where\nmain = FlowChirho.do\n  actionChirho\n",
        );
        assert!(enabled_chirho.contains(&RawTokenKindChirho::DoChirho));
    }

    #[test]
    fn pragma_text_inside_string_does_not_enable_extension_chirho() {
        let kinds_chirho = classified_kinds_chirho(
            "module M where\ntext = \"{-# LANGUAGE RecursiveDo #-}\"\nmdo = rec\n",
        );
        assert!(!kinds_chirho.contains(&RawTokenKindChirho::RecChirho));
        assert_eq!(
            kinds_chirho
                .iter()
                .filter(|kind_chirho| **kind_chirho == RawTokenKindChirho::VarIdChirho)
                .count(),
            3
        );
    }
}
