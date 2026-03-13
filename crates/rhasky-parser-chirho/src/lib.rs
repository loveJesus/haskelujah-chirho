// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-parser-chirho
//!
//! Parser for Haskell source files. Currently handles module header extraction;
//! will be expanded to a full recursive-descent parser with layout rule support.

use rhasky_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use rhasky_span_chirho::{ByteOffsetChirho, SpanChirho};
use rhasky_syntax_chirho::{ModuleHeaderChirho, SourceFileChirho};

pub const DEFAULT_MODULE_NAME_CHIRHO: &str = "Main";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedModuleChirho {
    pub module_header_chirho: ModuleHeaderChirho,
    pub source_file_chirho: SourceFileChirho,
}

pub fn parse_source_file_chirho(
    source_file_chirho: SourceFileChirho,
) -> Result<ParsedModuleChirho, DiagnosticBundleChirho> {
    let file_id_chirho = source_file_chirho.file_id_chirho();
    let mut saw_non_comment_code_chirho = false;
    let mut byte_offset_chirho: usize = 0;

    for raw_line_chirho in source_file_chirho.contents_chirho().lines() {
        let line_start_chirho = byte_offset_chirho;
        let line_len_chirho = raw_line_chirho.len();
        // Advance past this line + newline
        byte_offset_chirho += line_len_chirho + 1; // +1 for \n

        let trimmed_line_chirho = raw_line_chirho.trim();

        if trimmed_line_chirho.is_empty() {
            continue;
        }

        if trimmed_line_chirho.starts_with("--") {
            continue;
        }

        if let Some(module_header_chirho) =
            parse_module_header_line_chirho(trimmed_line_chirho, file_id_chirho, line_start_chirho)?
        {
            return Ok(ParsedModuleChirho {
                module_header_chirho,
                source_file_chirho,
            });
        }

        saw_non_comment_code_chirho = true;
        break;
    }

    let span_chirho = if saw_non_comment_code_chirho {
        SpanChirho::new_chirho(
            file_id_chirho,
            ByteOffsetChirho::new_chirho(0),
            ByteOffsetChirho::new_chirho(0),
        )
    } else {
        SpanChirho::DUMMY_CHIRHO
    };

    Ok(ParsedModuleChirho {
        module_header_chirho: ModuleHeaderChirho {
            module_name_chirho: DEFAULT_MODULE_NAME_CHIRHO.to_owned(),
            span_chirho,
        },
        source_file_chirho,
    })
}

fn parse_module_header_line_chirho(
    trimmed_line_chirho: &str,
    file_id_chirho: rhasky_span_chirho::FileIdChirho,
    line_start_offset_chirho: usize,
) -> Result<Option<ModuleHeaderChirho>, DiagnosticBundleChirho> {
    if !trimmed_line_chirho.starts_with("module ") {
        return Ok(None);
    }

    let remainder_chirho = trimmed_line_chirho
        .strip_prefix("module ")
        .expect("module prefix was checked");
    let remainder_chirho = remainder_chirho.trim();
    let module_name_chirho = remainder_chirho
        .strip_suffix(" where")
        .or_else(|| remainder_chirho.strip_suffix("where"))
        .map(str::trim)
        .filter(|module_name_chirho| !module_name_chirho.is_empty());

    match module_name_chirho {
        Some(module_name_chirho) => {
            let span_chirho = SpanChirho::new_chirho(
                file_id_chirho,
                ByteOffsetChirho::from_usize_chirho(line_start_offset_chirho),
                ByteOffsetChirho::from_usize_chirho(
                    line_start_offset_chirho + trimmed_line_chirho.len(),
                ),
            );
            Ok(Some(ModuleHeaderChirho {
                module_name_chirho: module_name_chirho.to_owned(),
                span_chirho,
            }))
        }
        None => {
            let span_chirho = SpanChirho::new_chirho(
                file_id_chirho,
                ByteOffsetChirho::from_usize_chirho(line_start_offset_chirho),
                ByteOffsetChirho::from_usize_chirho(
                    line_start_offset_chirho + trimmed_line_chirho.len(),
                ),
            );
            Err(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(1),
                "malformed module header; expected `module <Name> where`",
                span_chirho,
            )
            .into())
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::{DEFAULT_MODULE_NAME_CHIRHO, parse_source_file_chirho};
    use rhasky_span_chirho::SourceMapChirho;
    use rhasky_syntax_chirho::SourceFileChirho;

    #[test]
    fn parses_explicit_module_header_chirho() {
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            &mut source_map_chirho,
            "SampleChirho.hs",
            "module SampleChirho where\nvalueChirho = 1\n",
        );

        let parsed_module_chirho = parse_source_file_chirho(source_file_chirho)
            .expect("parser should accept a valid module header");

        assert_eq!(
            parsed_module_chirho.module_header_chirho.module_name_chirho,
            "SampleChirho"
        );
    }

    #[test]
    fn defaults_to_main_when_no_module_header_exists_chirho() {
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            &mut source_map_chirho,
            "MainChirho.hs",
            "mainChirho = putStrLn \"hi\"\n",
        );

        let parsed_module_chirho = parse_source_file_chirho(source_file_chirho)
            .expect("parser should accept scripts without a module header");

        assert_eq!(
            parsed_module_chirho.module_header_chirho.module_name_chirho,
            DEFAULT_MODULE_NAME_CHIRHO
        );
    }
}
