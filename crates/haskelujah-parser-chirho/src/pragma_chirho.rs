// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! LANGUAGE pragma extraction shared by pre-layout token classification and CST lowering.
//!
//! `mdo` and `rec` are contextual keywords under `RecursiveDo`, but layout runs before CST
//! lowering. The raw lexer therefore emits both words as ordinary variable identifiers. This
//! module reads only actual pragma tokens, computes the extension state, and reclassifies those
//! two words before layout. Strings and ordinary comments cannot accidentally enable an extension.
//!
//! The module header's `OPTIONS_GHC` option words are read here from the same real pragma
//! tokens, for the driver's error-deferral policy.
//!
//! workflow: recursive-do-chirho, compiler-pipeline-chirho/error-deferral-chirho

use crate::lexer_chirho::{LexerChirho, RawTokenChirho, RawTokenKindChirho};
use haskelujah_span_chirho::FileIdChirho;

mod catalogue_chirho;
use catalogue_chirho::{edition_flags_chirho, expand_written_extension_chirho};

/// One resolved module extension payload, shared by pre-layout classification and lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveExtensionsChirho {
    flags_chirho: Vec<String>,
}

impl EffectiveExtensionsChirho {
    /// The default GHC2021 edition, the last selected edition, then explicit overrides.
    pub fn from_entries_chirho(
        entries_chirho: impl IntoIterator<Item = PragmaExtensionChirho>,
    ) -> Self {
        Self {
            flags_chirho: module_extensions_chirho(entries_chirho),
        }
    }

    pub fn from_source_chirho(source_chirho: &str, file_id_chirho: FileIdChirho) -> Self {
        Self::from_entries_chirho(
            header_pragmas_chirho(source_chirho, file_id_chirho).extensions_chirho,
        )
    }

    pub fn flags_chirho(&self) -> &[String] {
        &self.flags_chirho
    }

    pub fn enabled_chirho(&self, extension_chirho: &str) -> bool {
        extension_enabled_chirho(&self.flags_chirho, extension_chirho)
    }
}

impl Default for EffectiveExtensionsChirho {
    fn default() -> Self {
        Self::from_entries_chirho([])
    }
}

/// One extension a pragma names: written by the programmer, or implied by a language edition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PragmaExtensionChirho {
    /// The flag as consumers read it (`GADTs`, `NoGADTs`).
    pub name_chirho: String,
    /// The language edition that implied it; `None` for a written flag.
    pub edition_chirho: Option<&'static str>,
}

/// Parse the inner text between `{-#` and `#-}` into extension flags.
///
/// LANGUAGE names preserve source order because `NoFoo` must be able to override an earlier
/// `Foo`. OPTIONS and OPTIONS_GHC contribute their `-X...` flags through the same path.
pub fn pragma_extensions_from_inner_chirho(inner_chirho: &str) -> Vec<String> {
    pragma_extension_entries_from_inner_chirho(inner_chirho)
        .into_iter()
        .map(|entry_chirho| entry_chirho.name_chirho)
        .collect()
}

/// The extensions one pragma names, in source order, each marked written or edition-implied.
pub fn pragma_extension_entries_from_inner_chirho(
    inner_chirho: &str,
) -> Vec<PragmaExtensionChirho> {
    let inner_chirho = inner_chirho.trim();
    let Some((keyword_chirho, rest_chirho)) = inner_chirho.split_once(char::is_whitespace) else {
        return Vec::new();
    };
    if keyword_chirho.eq_ignore_ascii_case("LANGUAGE") {
        return rest_chirho
            .split(',')
            .flat_map(|name_chirho| extension_entries_chirho(name_chirho.trim()))
            .collect();
    }
    if is_options_keyword_chirho(keyword_chirho) {
        return extension_entries_from_options_chirho(
            &ghc_option_words_chirho(rest_chirho).unwrap_or_default(),
        );
    }
    Vec::new()
}

fn extension_entries_chirho(name_chirho: &str) -> Vec<PragmaExtensionChirho> {
    if let Some((edition_chirho, flags_chirho)) = edition_flags_chirho(name_chirho) {
        flags_chirho
            .iter()
            .map(|name_chirho| PragmaExtensionChirho {
                name_chirho: (*name_chirho).to_string(),
                edition_chirho: Some(edition_chirho),
            })
            .collect()
    } else if name_chirho.is_empty() {
        Vec::new()
    } else {
        vec![PragmaExtensionChirho {
            name_chirho: name_chirho.to_string(),
            edition_chirho: None,
        }]
    }
}

/// Caller and OPTIONS_GHC words use the same edition and extension vocabulary.
pub fn extension_entries_from_options_chirho(
    options_chirho: &[String],
) -> Vec<PragmaExtensionChirho> {
    options_chirho
        .iter()
        .filter_map(|option_chirho| {
            option_chirho
                .strip_prefix("-X")
                .or_else(|| (option_chirho == "-cpp").then_some("CPP"))
        })
        .flat_map(extension_entries_chirho)
        .collect()
}

/// Replace the edition baseline, retaining explicit On/Off flags in their written order.
/// Edition defaults are not closed under the implications of explicitly enabling a flag.
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub fn module_extensions_chirho(
    entries_chirho: impl IntoIterator<Item = PragmaExtensionChirho>,
) -> Vec<String> {
    let mut edition_chirho = "GHC2021";
    let mut written_chirho = Vec::new();
    for entry_chirho in entries_chirho {
        if let Some(selected_chirho) = entry_chirho.edition_chirho {
            edition_chirho = selected_chirho;
        } else {
            expand_written_extension_chirho(&entry_chirho.name_chirho, &mut written_chirho);
        }
    }
    let (_, baseline_chirho) = edition_flags_chirho(edition_chirho)
        .expect("extension entries carry a known language edition");
    let mut flags_chirho: Vec<_> = baseline_chirho
        .iter()
        .map(|flag_chirho| (*flag_chirho).to_string())
        .collect();
    flags_chirho.extend(written_chirho);
    // Publish effective states, so older presence-based consumers cannot observe an
    // overridden positive flag. Preserve the first slot's order without quadratic scans.
    let mut positions_chirho = std::collections::HashMap::new();
    let mut effective_chirho = Vec::new();
    for flag_chirho in flags_chirho {
        let key_chirho = flag_chirho
            .strip_prefix("No")
            .filter(|name_chirho| name_chirho.starts_with(char::is_uppercase))
            .unwrap_or(&flag_chirho)
            .to_string();
        if let Some(index_chirho) = positions_chirho.get(&key_chirho) {
            effective_chirho[*index_chirho] = flag_chirho;
        } else {
            positions_chirho.insert(key_chirho, effective_chirho.len());
            effective_chirho.push(flag_chirho);
        }
    }
    effective_chirho
}

/// `OPTIONS_GHC`, or its legacy spelling `OPTIONS`; pragma keywords ignore case.
fn is_options_keyword_chirho(keyword_chirho: &str) -> bool {
    keyword_chirho.eq_ignore_ascii_case("OPTIONS_GHC")
        || keyword_chirho.eq_ignore_ascii_case("OPTIONS")
}

/// The option words of one `OPTIONS_GHC` pragma, given the text between `{-#` and `#-}`, in
/// source order. Every other pragma has none.
pub fn pragma_ghc_options_from_inner_chirho(inner_chirho: &str) -> Vec<String> {
    match inner_chirho.trim().split_once(char::is_whitespace) {
        Some((keyword_chirho, rest_chirho)) if is_options_keyword_chirho(keyword_chirho) => {
            ghc_option_words_chirho(rest_chirho).unwrap_or_default()
        }
        _ => Vec::new(),
    }
}

/// An `OPTIONS_GHC` body read as GHC's `toArgs` reads it (row 695).
/// - Whitespace separates words.
/// - A word that starts with `"` is a Haskell string literal, taken without its quotes.
/// - A `"` inside a word opens a quoted part, which keeps its quotes and may hold spaces.
/// - Either quoted form must end the word.
/// - Single quotes are ordinary characters, so `'-fdefer-typed-holes'` names no switch.
///
/// A body GHC cannot read (an unterminated or misplaced quote) gives `None`, because GHC
/// rejects such a pragma. The `["..."]` list form GHC also reads is not modeled.
fn ghc_option_words_chirho(body_chirho: &str) -> Option<Vec<String>> {
    let mut words_chirho = Vec::new();
    let mut rest_chirho = body_chirho.trim_start();
    while !rest_chirho.is_empty() {
        let word_chirho = if let Some(after_open_chirho) = rest_chirho.strip_prefix('"') {
            let (text_chirho, after_close_chirho) = split_quoted_chirho(after_open_chirho)?;
            rest_chirho = after_close_chirho;
            text_chirho
        } else {
            let end_chirho = rest_chirho
                .find(|char_chirho: char| char_chirho.is_whitespace() || char_chirho == '"')
                .unwrap_or(rest_chirho.len());
            let (plain_chirho, after_plain_chirho) = rest_chirho.split_at(end_chirho);
            if let Some(after_open_chirho) = after_plain_chirho.strip_prefix('"') {
                let (_, after_close_chirho) = split_quoted_chirho(after_open_chirho)?;
                let quoted_len_chirho = after_plain_chirho.len() - after_close_chirho.len();
                rest_chirho = after_close_chirho;
                format!("{plain_chirho}{}", &after_plain_chirho[..quoted_len_chirho])
            } else {
                rest_chirho = after_plain_chirho;
                plain_chirho.to_string()
            }
        };
        if rest_chirho.starts_with(|char_chirho: char| !char_chirho.is_whitespace()) {
            return None;
        }
        words_chirho.push(word_chirho);
        rest_chirho = rest_chirho.trim_start();
    }
    Some(words_chirho)
}

/// A Haskell string literal after its opening `"`: the text, and what follows the closing
/// `"`. Only `\"` and `\\` are decoded. Any other escape is kept as written, so it never
/// spells a deferral switch. `None` when the literal never closes.
fn split_quoted_chirho(after_open_chirho: &str) -> Option<(String, &str)> {
    let mut text_chirho = String::new();
    let mut chars_chirho = after_open_chirho.char_indices();
    while let Some((index_chirho, char_chirho)) = chars_chirho.next() {
        match char_chirho {
            '"' => return Some((text_chirho, &after_open_chirho[index_chirho + 1..])),
            '\\' => match chars_chirho.next() {
                Some((_, escaped_chirho @ ('"' | '\\'))) => text_chirho.push(escaped_chirho),
                Some((_, other_chirho)) => {
                    text_chirho.push('\\');
                    text_chirho.push(other_chirho);
                }
                None => return None,
            },
            other_chirho => text_chirho.push(other_chirho),
        }
    }
    None
}

/// The option words of the module header's `OPTIONS_GHC` pragmas, in source order.
///
/// GHC reads them only from the file header: the pragmas before the first token that is
/// neither a pragma nor a comment. Only real pragma tokens count, so a flag spelled in a
/// comment, in a string literal or in a pragma after the header is not an option. Lexing
/// stops where the header ends.
/// workflow: compiler-pipeline-chirho/error-deferral-chirho
pub fn header_ghc_options_chirho(source_chirho: &str, file_id_chirho: FileIdChirho) -> Vec<String> {
    header_pragmas_chirho(source_chirho, file_id_chirho).ghc_options_chirho
}

/// Real header tokens, retaining extension provenance until all source views are combined.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeaderPragmasChirho {
    pub extensions_chirho: Vec<PragmaExtensionChirho>,
    pub ghc_options_chirho: Vec<String>,
}

pub fn header_pragmas_chirho(
    source_chirho: &str,
    file_id_chirho: FileIdChirho,
) -> HeaderPragmasChirho {
    let mut lexer_chirho = LexerChirho::new_chirho(source_chirho, file_id_chirho);
    let mut header_chirho = HeaderPragmasChirho::default();
    loop {
        let token_chirho = lexer_chirho.next_token_chirho();
        match token_chirho.kind_chirho {
            RawTokenKindChirho::PragmaChirho => {
                let start_chirho = token_chirho.span_chirho.start_chirho().as_usize_chirho();
                let end_chirho = token_chirho.span_chirho.end_chirho().as_usize_chirho();
                if let Some(inner_chirho) = source_chirho
                    .get(start_chirho..end_chirho)
                    .and_then(|text_chirho| text_chirho.strip_prefix("{-#"))
                    .and_then(|text_chirho| text_chirho.strip_suffix("#-}"))
                {
                    header_chirho
                        .extensions_chirho
                        .extend(pragma_extension_entries_from_inner_chirho(inner_chirho));
                    header_chirho
                        .ghc_options_chirho
                        .extend(pragma_ghc_options_from_inner_chirho(inner_chirho));
                }
            }
            kind_chirho if kind_chirho.is_trivia_chirho() => {}
            _ => return header_chirho,
        }
    }
}

/// Parse a complete pragma token such as `{-# LANGUAGE RecursiveDo #-}`.
pub fn pragma_extension_entries_from_text_chirho(text_chirho: &str) -> Vec<PragmaExtensionChirho> {
    let inner_chirho = text_chirho
        .strip_prefix("{-#")
        .and_then(|text_chirho| text_chirho.strip_suffix("#-}"))
        .unwrap_or("");
    pragma_extension_entries_from_inner_chirho(inner_chirho)
}

/// Collect extension flags from actual raw pragma tokens.
pub fn collect_raw_pragma_extensions_chirho(
    source_chirho: &str,
    tokens_chirho: &[RawTokenChirho],
) -> Vec<String> {
    module_extensions_chirho(
        tokens_chirho
            .iter()
            .take_while(|token_chirho| token_chirho.kind_chirho.is_trivia_chirho())
            .filter(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::PragmaChirho)
            .flat_map(|token_chirho| {
                let start_chirho = token_chirho.span_chirho.start_chirho().as_usize_chirho();
                let end_chirho = token_chirho.span_chirho.end_chirho().as_usize_chirho();
                source_chirho
                    .get(start_chirho..end_chirho)
                    .map(pragma_extension_entries_from_text_chirho)
                    .unwrap_or_default()
            }),
    )
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
    let extensions_chirho = EffectiveExtensionsChirho {
        flags_chirho: collect_raw_pragma_extensions_chirho(source_chirho, tokens_chirho),
    };
    classify_contextual_keywords_with_extensions_chirho(
        source_chirho,
        tokens_chirho,
        &extensions_chirho,
    );
    extensions_chirho.flags_chirho
}

pub fn classify_contextual_keywords_with_extensions_chirho(
    source_chirho: &str,
    tokens_chirho: &mut [RawTokenChirho],
    extensions_chirho: &EffectiveExtensionsChirho,
) {
    let extensions_chirho = extensions_chirho.flags_chirho();
    let recursive_do_enabled_chirho = extension_enabled_chirho(extensions_chirho, "RecursiveDo");
    let qualified_do_enabled_chirho = extension_enabled_chirho(extensions_chirho, "QualifiedDo");
    if !recursive_do_enabled_chirho && !qualified_do_enabled_chirho {
        return;
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
}

#[cfg(test)]
mod effective_tests_chirho;
#[cfg(test)]
mod tests_chirho {
    use super::*;

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

    /// A module's last-wins state of `extension_chirho` after these LANGUAGE pragmas.
    fn module_enables_chirho(pragmas_chirho: &[&str], extension_chirho: &str) -> bool {
        let extensions_chirho = module_extensions_chirho(
            pragmas_chirho
                .iter()
                .flat_map(|inner_chirho| pragma_extension_entries_from_inner_chirho(inner_chirho)),
        );
        extension_enabled_chirho(&extensions_chirho, extension_chirho)
    }

    #[test]
    fn a_written_flag_beats_an_edition_in_either_order_chirho() {
        let gnd_chirho = "GeneralizedNewtypeDeriving";
        // Root's O3 and O4 (GHC 9.14.1): the written flag decides, before or after GHC2021.
        assert!(!module_enables_chirho(
            &["LANGUAGE NoGeneralizedNewtypeDeriving, GHC2021"],
            gnd_chirho
        ));
        assert!(!module_enables_chirho(
            &["LANGUAGE NoGeneralizedNewtypeDeriving", "LANGUAGE GHC2021"],
            gnd_chirho
        ));
        assert!(module_enables_chirho(
            &["LANGUAGE NoGeneralizedNewtypeDeriving, GeneralizedNewtypeDeriving"],
            gnd_chirho
        ));
        // Without a written flag, the edition implies it.
        assert!(module_enables_chirho(&["LANGUAGE GHC2021"], gnd_chirho));
        assert!(module_enables_chirho(
            &["LANGUAGE Haskell2010, GHC2021"],
            gnd_chirho
        ));
    }

    #[test]
    fn ghc2024_implies_deriving_strategies_but_ghc2021_does_not_chirho() {
        assert!(module_enables_chirho(
            &["LANGUAGE GHC2024"],
            "DerivingStrategies"
        ));
        assert!(!module_enables_chirho(
            &["LANGUAGE GHC2021"],
            "DerivingStrategies"
        ));
        assert!(!module_enables_chirho(
            &["LANGUAGE NoDerivingStrategies, GHC2024"],
            "DerivingStrategies"
        ));
    }

    #[test]
    fn the_last_language_edition_wins_chirho() {
        // Root's RX3 (GHC 9.14.1): GHC2024 then GHC2021 is GHC2021, and a written flag still
        // holds over it.
        let last_chirho = ["LANGUAGE GHC2024", "LANGUAGE GHC2021, DerivingStrategies"];
        assert!(!module_enables_chirho(&last_chirho, "RoleAnnotations"));
        assert!(module_enables_chirho(&last_chirho, "DerivingStrategies"));
        assert!(module_enables_chirho(
            &["LANGUAGE GHC2021", "LANGUAGE GHC2024"],
            "RoleAnnotations"
        ));
        // A Haskell2010 edition after GHC2021 leaves none of GHC2021's flags.
        let legacy_chirho = ["LANGUAGE GHC2021", "LANGUAGE Haskell2010"];
        assert!(!module_enables_chirho(
            &legacy_chirho,
            "ScopedTypeVariables"
        ));
        assert!(module_enables_chirho(&legacy_chirho, "Haskell2010"));
    }

    #[test]
    fn ghc2024_implies_role_annotations_but_ghc2021_does_not_chirho() {
        // Root's RX1 and RX2 (GHC 9.14.1).
        assert!(module_enables_chirho(
            &["LANGUAGE GHC2024"],
            "RoleAnnotations"
        ));
        assert!(!module_enables_chirho(
            &["LANGUAGE GHC2021"],
            "RoleAnnotations"
        ));
        assert!(!module_enables_chirho(
            &["LANGUAGE DerivingStrategies"],
            "RoleAnnotations"
        ));
        assert!(!module_enables_chirho(
            &["LANGUAGE NoRoleAnnotations, GHC2024"],
            "RoleAnnotations"
        ));
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
