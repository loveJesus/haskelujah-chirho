// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The actual Prelude's complete export names certify scope only, never types or instances.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_parser_chirho::lexer_chirho::{LexerChirho, RawTokenKindChirho};
use std::sync::{Arc, OnceLock};
const SOURCE_CHIRHO: &str = include_str!("vendor-chirho/base-chirho/Prelude.hs");
static EXPORTS_CHIRHO: OnceLock<Result<Arc<PreludeExportsChirho>, String>> = OnceLock::new();

pub(in crate::next_chirho) fn exports_chirho() -> Result<Arc<PreludeExportsChirho>, NextErrorChirho>
{
    EXPORTS_CHIRHO
        .get_or_init(|| {
            read_exports_chirho()
                .map(Arc::new)
                .map_err(|error_chirho| error_chirho.message_chirho)
        })
        .clone()
        .map_err(|message_chirho| {
            NextErrorChirho::new_chirho(NextErrorKindChirho::UnsupportedChirho, message_chirho)
        })
}
fn invalid_chirho() -> NextErrorChirho {
    NextErrorChirho::new_chirho(
        NextErrorKindChirho::UnsupportedChirho,
        "vendored Prelude export account is incomplete or malformed",
    )
}
struct ExportReaderChirho<'source_chirho> {
    tokens_chirho: Vec<(RawTokenKindChirho, &'source_chirho str)>,
    cursor_chirho: usize,
}
impl ExportReaderChirho<'_> {
    fn take_chirho(&mut self, kind_chirho: RawTokenKindChirho) -> bool {
        if self
            .tokens_chirho
            .get(self.cursor_chirho)
            .is_some_and(|(actual_chirho, _)| *actual_chirho == kind_chirho)
        {
            self.cursor_chirho += 1;
            true
        } else {
            false
        }
    }
    fn name_chirho(&mut self) -> Result<String, NextErrorChirho> {
        use RawTokenKindChirho::*;
        let parenthesized_chirho = self.take_chirho(LeftParenChirho);
        let &(kind_chirho, text_chirho) = self
            .tokens_chirho
            .get(self.cursor_chirho)
            .ok_or_else(invalid_chirho)?;
        if !(matches!(kind_chirho, VarIdChirho | ConIdChirho | QualifiedIdChirho)
            || parenthesized_chirho
                && matches!(kind_chirho, VarSymChirho | ConSymChirho | TildeChirho))
        {
            return Err(invalid_chirho());
        }
        self.cursor_chirho += 1;
        if parenthesized_chirho && !self.take_chirho(RightParenChirho) {
            return Err(invalid_chirho());
        }
        // A qualified export is brought into the importing scope under its occurrence.
        let mut occurrence_chirho = text_chirho;
        while let Some((qualifier_chirho, remaining_chirho)) = occurrence_chirho.split_once('.') {
            if !qualifier_chirho.starts_with(char::is_uppercase)
                || !qualifier_chirho.chars().all(|character_chirho| {
                    character_chirho.is_alphanumeric() || matches!(character_chirho, '_' | '\'')
                })
            {
                break;
            }
            occurrence_chirho = remaining_chirho;
        }
        Ok(occurrence_chirho.to_string())
    }
}
fn read_exports_chirho() -> Result<PreludeExportsChirho, NextErrorChirho> {
    use RawTokenKindChirho::*;
    verify_source_chirho(
        SOURCE_CHIRHO,
        "02f4309be195717f29c307f5041784c62a08297a68169bf4760cfe95abd1fa3c",
        "libraries/base/src/Prelude.hs",
        "9.14.1",
    )?;
    let tokens_chirho = LexerChirho::new_chirho(
        SOURCE_CHIRHO,
        haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
    )
    .lex_all_chirho()
    .into_iter()
    .filter(|token_chirho| !token_chirho.kind_chirho.is_trivia_chirho())
    .map(|token_chirho| {
        Ok((
            token_chirho.kind_chirho,
            token_chirho
                .span_chirho
                .text_chirho(SOURCE_CHIRHO)
                .ok_or_else(invalid_chirho)?,
        ))
    })
    .collect::<Result<Vec<_>, NextErrorChirho>>()?;
    let mut reader_chirho = ExportReaderChirho {
        tokens_chirho,
        cursor_chirho: 0,
    };
    if !reader_chirho.take_chirho(ModuleChirho)
        || reader_chirho.name_chirho()? != "Prelude"
        || !reader_chirho.take_chirho(LeftParenChirho)
    {
        return Err(invalid_chirho());
    }
    let mut result_chirho = BTreeSet::new();
    loop {
        let explicit_type_chirho = reader_chirho.take_chirho(TypeChirho);
        let name_chirho = reader_chirho.name_chirho()?;
        let namespace_chirho =
            if explicit_type_chirho || name_chirho.starts_with(char::is_uppercase) {
                NamespaceChirho::TypeChirho
            } else {
                NamespaceChirho::ValueChirho
            };
        result_chirho.insert((name_chirho, namespace_chirho));
        if reader_chirho.take_chirho(LeftParenChirho) {
            loop {
                // No wildcards or module re-exports are allowed to supply absence authority.
                let name_chirho = reader_chirho.name_chirho()?;
                result_chirho.insert((name_chirho, NamespaceChirho::ValueChirho));
                if reader_chirho.take_chirho(RightParenChirho) {
                    break;
                }
                if !reader_chirho.take_chirho(CommaChirho) {
                    return Err(invalid_chirho());
                }
            }
        }
        if reader_chirho.take_chirho(RightParenChirho) {
            break;
        }
        if !reader_chirho.take_chirho(CommaChirho) {
            return Err(invalid_chirho());
        }
    }
    if !reader_chirho.take_chirho(WhereChirho) {
        return Err(invalid_chirho());
    }
    Ok(result_chirho)
}
