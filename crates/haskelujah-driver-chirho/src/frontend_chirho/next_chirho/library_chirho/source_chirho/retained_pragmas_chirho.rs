// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A pinned group may retain its own inert NOINLINE; no user pragma acquires authority.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_parser_chirho::lexer_chirho::{LexerChirho, RawTokenKindChirho};
use haskelujah_span_chirho::FileIdChirho;

pub(super) fn within_group_chirho(
    selection_chirho: &str,
    offset_chirho: usize,
) -> Result<Vec<(usize, usize)>, NextErrorChirho> {
    let mut spans_chirho = Vec::new();
    let tokens_chirho =
        LexerChirho::new_chirho(selection_chirho, FileIdChirho::SYNTHETIC_CHIRHO).lex_all_chirho();
    for token_chirho in tokens_chirho
        .iter()
        .filter(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::PragmaChirho)
    {
        let text_chirho = token_chirho
            .span_chirho
            .text_chirho(selection_chirho)
            .unwrap_or("");
        let inner_chirho = text_chirho
            .strip_prefix("{-#")
            .and_then(|text_chirho| text_chirho.strip_suffix("#-}"))
            .unwrap_or("")
            .trim();
        let Some(rest_chirho) = inner_chirho
            .strip_prefix("NOINLINE")
            .and_then(|text_chirho| text_chirho.strip_prefix(char::is_whitespace))
        else {
            continue;
        };
        let rest_chirho = rest_chirho.trim();
        let target_chirho = if let Some(phase_chirho) = rest_chirho.strip_prefix('[') {
            let (digits_chirho, rest_chirho) = phase_chirho
                .split_once(']')
                .ok_or_else(|| error_chirho("retained NOINLINE has no phase close"))?;
            if digits_chirho.is_empty()
                || !digits_chirho
                    .bytes()
                    .all(|byte_chirho| byte_chirho.is_ascii_digit())
            {
                return Err(error_chirho(
                    "retained NOINLINE needs a plain decimal phase",
                ));
            }
            rest_chirho.trim()
        } else {
            rest_chirho
        };
        let name_chirho = target_chirho
            .strip_prefix('(')
            .and_then(|text_chirho| text_chirho.strip_suffix(')'))
            .ok_or_else(|| error_chirho("retained NOINLINE needs one parenthesized operator"))?;
        if name_chirho.is_empty()
            || !name_chirho
                .chars()
                .all(|character_chirho| "!#$%&*+./<=>?@\\^|-~:".contains(character_chirho))
        {
            return Err(error_chirho(
                "retained NOINLINE has no single operator target",
            ));
        }
        let signature_chirho = selection_chirho.lines().any(|line_chirho| {
            line_chirho
                .strip_prefix(target_chirho)
                .is_some_and(|tail_chirho| tail_chirho.trim_start().starts_with("::"))
        });
        let equation_chirho = selection_chirho.lines().any(|line_chirho| {
            line_chirho
                .strip_prefix(target_chirho)
                .is_some_and(|tail_chirho| {
                    !tail_chirho.trim_start().starts_with("::") && tail_chirho.contains('=')
                })
        });
        let start_chirho = token_chirho.span_chirho.start_chirho().as_usize_chirho();
        let binders_chirho: std::collections::BTreeSet<_> =
            boundaries_chirho::declarations_chirho(selection_chirho)?
                .into_iter()
                .flatten()
                .collect();
        if binders_chirho != std::collections::BTreeSet::from([name_chirho.to_string()])
            || !signature_chirho
            || !equation_chirho
            || (start_chirho > 0 && selection_chirho.as_bytes()[start_chirho - 1] != b'\n')
        {
            return Err(error_chirho(
                "retained NOINLINE is not enclosed with its top-level signature and equation group",
            ));
        }
        spans_chirho.push((
            offset_chirho + start_chirho,
            offset_chirho + token_chirho.span_chirho.end_chirho().as_usize_chirho(),
        ));
    }
    Ok(spans_chirho)
}

fn error_chirho(message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(NextErrorKindChirho::UnsupportedChirho, message_chirho)
}
