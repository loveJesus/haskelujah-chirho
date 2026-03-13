// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life.

use rhasky_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho};
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
    let mut saw_non_comment_code_chirho = false;

    for (line_index_chirho, raw_line_chirho) in source_file_chirho.contents_chirho().lines().enumerate()
    {
        let trimmed_line_chirho = raw_line_chirho.trim();

        if trimmed_line_chirho.is_empty() {
            continue;
        }

        if trimmed_line_chirho.starts_with("--") {
            continue;
        }

        if let Some(module_header_chirho) =
            parse_module_header_line_chirho(trimmed_line_chirho, line_index_chirho + 1)?
        {
            return Ok(ParsedModuleChirho {
                module_header_chirho,
                source_file_chirho,
            });
        }

        saw_non_comment_code_chirho = true;
        break;
    }

    let line_number_chirho = if saw_non_comment_code_chirho { 1 } else { 0 };

    Ok(ParsedModuleChirho {
        module_header_chirho: ModuleHeaderChirho {
            module_name_chirho: DEFAULT_MODULE_NAME_CHIRHO.to_owned(),
            line_number_chirho,
        },
        source_file_chirho,
    })
}

fn parse_module_header_line_chirho(
    trimmed_line_chirho: &str,
    line_number_chirho: usize,
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
        Some(module_name_chirho) => Ok(Some(ModuleHeaderChirho {
            module_name_chirho: module_name_chirho.to_owned(),
            line_number_chirho,
        })),
        None => Err(DiagnosticChirho::error_chirho(
            "malformed module header; expected `module <Name> where`",
            Some(line_number_chirho),
        )
        .into()),
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::{DEFAULT_MODULE_NAME_CHIRHO, parse_source_file_chirho};
    use rhasky_syntax_chirho::SourceFileChirho;

    #[test]
    fn parses_explicit_module_header_chirho() {
        let source_file_chirho = SourceFileChirho::new_chirho(
            "SampleChirho.hs",
            "module SampleChirho where\nvalueChirho = 1\n",
        );

        let parsed_module_chirho = parse_source_file_chirho(source_file_chirho)
            .expect("parser should accept a valid module header");

        assert_eq!(
            parsed_module_chirho.module_header_chirho.module_name_chirho,
            "SampleChirho"
        );
        assert_eq!(parsed_module_chirho.module_header_chirho.line_number_chirho, 1);
    }

    #[test]
    fn defaults_to_main_when_no_module_header_exists_chirho() {
        let source_file_chirho =
            SourceFileChirho::new_chirho("MainChirho.hs", "mainChirho = putStrLn \"hi\"\n");

        let parsed_module_chirho = parse_source_file_chirho(source_file_chirho)
            .expect("parser should accept scripts without a module header");

        assert_eq!(
            parsed_module_chirho.module_header_chirho.module_name_chirho,
            DEFAULT_MODULE_NAME_CHIRHO
        );
    }
}

