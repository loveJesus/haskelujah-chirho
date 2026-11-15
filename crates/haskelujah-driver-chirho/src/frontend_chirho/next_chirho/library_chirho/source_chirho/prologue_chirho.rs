// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A pinned, declaration-free CPP prologue cannot change checked header or body names.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_parser_chirho::lexer_chirho::{LexerChirho, RawTokenKindChirho};
use haskelujah_span_chirho::FileIdChirho;
use std::collections::BTreeSet;

#[derive(Clone, Copy)]
pub(in crate::next_chirho) struct FacadePrologueChirho {
    // GHC 9.14.1 profile assumption, reviewed in DM4860: MachDeps.h and the
    // generated ghcautoconf.h define no identifier used by this header or these
    // selected groups. This is an explicit include assumption, not general CPP.
    pub span_chirho: (usize, usize),
    pub header_end_chirho: usize,
}

pub(super) fn validate_chirho(
    manifest_chirho: &SourceManifestChirho,
) -> Result<&str, NextErrorChirho> {
    let Some(prologue_chirho) = &manifest_chirho.facade_prologue_chirho else {
        return Ok(manifest_chirho.bytes_chirho);
    };
    let source_chirho = manifest_chirho.bytes_chirho;
    let (start_chirho, end_chirho) = prologue_chirho.span_chirho;
    if start_chirho >= end_chirho
        || end_chirho > prologue_chirho.header_end_chirho
        || source_chirho.as_bytes().get(start_chirho.wrapping_sub(1)) != Some(&b'\n')
        || source_chirho.as_bytes().get(end_chirho.wrapping_sub(1)) != Some(&b'\n')
    {
        return Err(error_chirho());
    }
    let region_chirho = source_chirho
        .get(start_chirho..end_chirho)
        .ok_or_else(error_chirho)?;
    let header_chirho = source_chirho
        .get(end_chirho..prologue_chirho.header_end_chirho)
        .ok_or_else(error_chirho)?;
    // No selected declaration may live under an unseen later conditional or
    // macro definition. This whole-file rule also guards future source re-pins.
    let mut offset_chirho = 0;
    for line_chirho in source_chirho.split_inclusive('\n') {
        if line_chirho.trim_start().starts_with('#')
            && !(start_chirho..end_chirho).contains(&offset_chirho)
        {
            return Err(error_chirho());
        }
        offset_chirho += line_chirho.len();
    }
    if LexerChirho::new_chirho(
        &source_chirho[..start_chirho],
        FileIdChirho::SYNTHETIC_CHIRHO,
    )
    .lex_all_chirho()
    .iter()
    .any(|token_chirho| {
        !token_chirho.kind_chirho.is_trivia_chirho()
            && token_chirho.kind_chirho != RawTokenKindChirho::EofChirho
    }) {
        return Err(error_chirho());
    }
    let mut defined_chirho = BTreeSet::new();
    let mut open_chirho = false;
    let mut seen_if_chirho = false;
    let mut seen_else_chirho = false;
    for line_chirho in region_chirho.lines().map(str::trim) {
        if line_chirho.is_empty() || line_chirho.starts_with("--") {
            continue;
        }
        let directive_chirho = line_chirho
            .strip_prefix('#')
            .ok_or_else(error_chirho)?
            .trim_start();
        let (word_chirho, rest_chirho) = directive_chirho
            .split_once(char::is_whitespace)
            .unwrap_or((directive_chirho, ""));
        let rest_chirho = rest_chirho.trim();
        match word_chirho {
            "include"
                if !seen_if_chirho
                    && rest_chirho.starts_with('"')
                    && rest_chirho.ends_with('"') => {}
            "if" if !seen_if_chirho && !rest_chirho.is_empty() => {
                open_chirho = true;
                seen_if_chirho = true;
            }
            "elif" if open_chirho && !seen_else_chirho && !rest_chirho.is_empty() => {}
            "else" if open_chirho && !seen_else_chirho && rest_chirho.is_empty() => {
                seen_else_chirho = true;
            }
            "endif" if open_chirho && rest_chirho.is_empty() => {
                open_chirho = false;
            }
            "error" if open_chirho && !rest_chirho.is_empty() => {}
            "define" if open_chirho => {
                let (name_chirho, value_chirho) = rest_chirho
                    .split_once(char::is_whitespace)
                    .ok_or_else(error_chirho)?;
                if !name_chirho.starts_with(|character_chirho: char| {
                    character_chirho.is_ascii_alphabetic() || character_chirho == '_'
                }) || !name_chirho.chars().all(|character_chirho| {
                    character_chirho.is_ascii_alphanumeric() || character_chirho == '_'
                }) || value_chirho.trim().is_empty()
                    || !value_chirho
                        .trim()
                        .bytes()
                        .all(|byte_chirho| byte_chirho.is_ascii_digit())
                {
                    return Err(error_chirho());
                }
                defined_chirho.insert(name_chirho);
            }
            _ => return Err(error_chirho()),
        }
    }
    if open_chirho || !seen_if_chirho || defined_chirho.is_empty() {
        return Err(error_chirho());
    }
    for region_chirho in
        std::iter::once(header_chirho).chain(manifest_chirho.spans_chirho.iter().map(
            |&(a_chirho, b_chirho)| source_chirho.get(a_chirho..b_chirho).unwrap_or("#invalid"),
        ))
    {
        if region_chirho
            .lines()
            .any(|line_chirho| line_chirho.trim_start().starts_with('#'))
        {
            return Err(error_chirho());
        }
        for token_chirho in
            LexerChirho::new_chirho(region_chirho, FileIdChirho::SYNTHETIC_CHIRHO).lex_all_chirho()
        {
            if matches!(
                token_chirho.kind_chirho,
                RawTokenKindChirho::VarIdChirho
                    | RawTokenKindChirho::ConIdChirho
                    | RawTokenKindChirho::QualifiedIdChirho
            ) && token_chirho
                .span_chirho
                .text_chirho(region_chirho)
                .is_some_and(|text_chirho| {
                    text_chirho
                        .split('.')
                        .any(|part_chirho| defined_chirho.contains(part_chirho))
                })
            {
                return Err(error_chirho());
            }
        }
    }
    Ok(&source_chirho[end_chirho..])
}

fn error_chirho() -> NextErrorChirho {
    NextErrorChirho::new_chirho(
        NextErrorKindChirho::UnsupportedChirho,
        "source CPP prologue can affect a selected header or declaration",
    )
}

#[cfg(test)]
#[path = "prologue_tests_chirho.rs"]
mod tests_chirho;
