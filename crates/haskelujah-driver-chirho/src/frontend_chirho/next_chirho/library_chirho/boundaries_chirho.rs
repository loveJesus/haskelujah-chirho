// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declared source spans cannot truncate indentation or continue the same equation group.
use super::*;
use haskelujah_parser_chirho::lexer_chirho::{LexerChirho, RawTokenKindChirho};
use haskelujah_span_chirho::FileIdChirho;

pub(super) fn validate_chirho(
    source_chirho: &str,
    start_chirho: usize,
    end_chirho: usize,
) -> Result<(), NextErrorChirho> {
    let selection_chirho = source_chirho
        .get(start_chirho..end_chirho)
        .ok_or_else(|| invalid_chirho("invalid source manifest byte span"))?;
    if (start_chirho > 0 && source_chirho.as_bytes().get(start_chirho - 1) != Some(&b'\n'))
        || source_chirho
            .as_bytes()
            .get(start_chirho)
            .is_none_or(u8::is_ascii_whitespace)
        || source_chirho.as_bytes().get(end_chirho.wrapping_sub(1)) != Some(&b'\n')
    {
        return Err(invalid_chirho(
            "source manifest span truncates a declaration",
        ));
    }
    let tail_chirho = &source_chirho[end_chirho..];
    let next_chirho = LexerChirho::new_chirho(tail_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
        .lex_all_chirho()
        .into_iter()
        .find(|token_chirho| {
            !token_chirho.kind_chirho.is_trivia_chirho()
                && token_chirho.kind_chirho != RawTokenKindChirho::EofChirho
        });
    if let Some(token_chirho) = &next_chirho {
        let offset_chirho = token_chirho.span_chirho.start_chirho().as_usize_chirho();
        let line_start_chirho = tail_chirho[..offset_chirho]
            .rfind('\n')
            .map_or(0, |index_chirho| index_chirho + 1);
        if line_start_chirho != offset_chirho {
            return Err(invalid_chirho(
                "source manifest span truncates a declaration",
            ));
        }
    }
    let selected_chirho = declarations_chirho(selection_chirho)?;
    let before_chirho = declarations_chirho(&source_chirho[..start_chirho])?;
    let after_chirho = declarations_chirho(tail_chirho)?;
    let overlaps_chirho =
        |left_chirho: Option<&std::collections::BTreeSet<String>>,
         right_chirho: Option<&std::collections::BTreeSet<String>>| {
            left_chirho
                .zip(right_chirho)
                .is_some_and(|(left_chirho, right_chirho)| !left_chirho.is_disjoint(right_chirho))
        };
    if overlaps_chirho(before_chirho.last(), selected_chirho.first())
        || overlaps_chirho(selected_chirho.last(), after_chirho.first())
    {
        return Err(invalid_chirho("source manifest splits an equation group"));
    }
    Ok(())
}
fn invalid_chirho(message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(NextErrorKindChirho::UnsupportedChirho, message_chirho)
}

fn declarations_chirho(
    source_chirho: &str,
) -> Result<Vec<std::collections::BTreeSet<String>>, NextErrorChirho> {
    use RawTokenKindChirho::*;
    let tokens_chirho = LexerChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
        .lex_all_chirho()
        .into_iter()
        .filter(|token_chirho| !token_chirho.kind_chirho.is_trivia_chirho())
        .collect::<Vec<_>>();
    let mut result_chirho = Vec::new();
    for (index_chirho, token_chirho) in tokens_chirho.iter().enumerate() {
        let offset_chirho = token_chirho.span_chirho.start_chirho().as_usize_chirho();
        if (offset_chirho > 0 && source_chirho.as_bytes().get(offset_chirho - 1) != Some(&b'\n'))
            || token_chirho.kind_chirho == EofChirho
        {
            continue;
        }
        let mut binders_chirho = std::collections::BTreeSet::new();
        match token_chirho.kind_chirho {
            VarIdChirho => {
                let mut cursor_chirho = index_chirho;
                loop {
                    let name_chirho = tokens_chirho
                        .get(cursor_chirho)
                        .filter(|token_chirho| token_chirho.kind_chirho == VarIdChirho)
                        .ok_or_else(|| {
                            invalid_chirho("source manifest needs a supported binder set")
                        })?;
                    binders_chirho.insert(
                        name_chirho
                            .span_chirho
                            .text_chirho(source_chirho)
                            .unwrap_or("")
                            .to_string(),
                    );
                    cursor_chirho += 1;
                    if tokens_chirho
                        .get(cursor_chirho)
                        .is_some_and(|token_chirho| token_chirho.kind_chirho == CommaChirho)
                    {
                        cursor_chirho += 1;
                    } else {
                        break;
                    }
                }
                let next_kind_chirho = tokens_chirho
                    .get(cursor_chirho)
                    .map(|token_chirho| token_chirho.kind_chirho);
                if (binders_chirho.len() > 1 && next_kind_chirho != Some(ColonColonChirho))
                    || matches!(
                        next_kind_chirho,
                        Some(VarSymChirho | ConSymChirho | BacktickChirho)
                    )
                {
                    return Err(invalid_chirho(
                        "source manifest needs a supported non-infix binder set",
                    ));
                }
            }
            ClassChirho => {
                let name_chirho = tokens_chirho
                    .get(index_chirho + 1)
                    .filter(|token_chirho| token_chirho.kind_chirho == ConIdChirho)
                    .ok_or_else(|| {
                        invalid_chirho("source manifest class has no declaration name")
                    })?;
                binders_chirho.insert(
                    name_chirho
                        .span_chirho
                        .text_chirho(source_chirho)
                        .unwrap_or("")
                        .to_string(),
                );
            }
            _ => {}
        }
        result_chirho.push(binders_chirho);
    }
    Ok(result_chirho)
}
