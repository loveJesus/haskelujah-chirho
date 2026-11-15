// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Never let the shared decoder turn an unrepresentable Haskell Char into source text.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::{NextErrorChirho, NextErrorKindChirho};
pub(super) fn validate_chirho(
    text_chirho: &str,
    string_chirho: bool,
) -> Result<(), NextErrorChirho> {
    let body_chirho = text_chirho
        .get(1..text_chirho.len().saturating_sub(1))
        .ok_or_else(error_chirho)?;
    let mut characters_chirho = body_chirho.chars();
    while let Some(character_chirho) = characters_chirho.next() {
        if character_chirho != '\\' {
            continue;
        }
        match characters_chirho.as_str().chars().next() {
            Some('&') if string_chirho => {
                characters_chirho.next();
            }
            Some(space_chirho) if string_chirho && space_chirho.is_whitespace() => {
                let mut closed_chirho = false;
                for gap_chirho in characters_chirho.by_ref() {
                    if gap_chirho == '\\' {
                        closed_chirho = true;
                        break;
                    }
                    if !gap_chirho.is_whitespace() {
                        return Err(error_chirho());
                    }
                }
                if !closed_chirho {
                    return Err(error_chirho());
                }
            }
            _ => {
                haskelujah_ast_chirho::lit_chirho::decode_escape_chirho(&mut characters_chirho)
                    .ok_or_else(error_chirho)?;
            }
        }
    }
    Ok(())
}
fn error_chirho() -> NextErrorChirho {
    NextErrorChirho::new_chirho(
        NextErrorKindChirho::UnsupportedChirho,
        "literal contains a Char outside the checked scalar decoder profile",
    )
}
