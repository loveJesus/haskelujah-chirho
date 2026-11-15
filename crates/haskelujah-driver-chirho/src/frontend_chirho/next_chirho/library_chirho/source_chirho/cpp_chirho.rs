// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Select unconditional declarations and facade imports without choosing a CPP target.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use std::collections::BTreeSet;

#[derive(Clone, Copy)]
pub(in crate::next_chirho) struct SourceCppChirho {
    pub header_span_chirho: (usize, usize),
    /// GHC 9.14.1 Show's reviewed MachDeps/ghcautoconf include premise is
    /// explicit. New IO selections contain no include and supply false.
    pub include_premise_chirho: bool,
}
struct LineChirho {
    start_chirho: usize,
    end_chirho: usize,
    unconditional_chirho: bool,
}
struct AnalysisChirho<'source_chirho> {
    source_chirho: &'source_chirho str,
    lines_chirho: Vec<LineChirho>,
    macros_chirho: BTreeSet<&'source_chirho str>,
}
fn error_chirho() -> NextErrorChirho {
    NextErrorChirho::new_chirho(
        NextErrorKindChirho::UnsupportedChirho,
        "source CPP can affect a selected header or declaration",
    )
}
fn identifier_chirho(text_chirho: &str) -> Option<&str> {
    let length_chirho = text_chirho
        .bytes()
        .take_while(|byte_chirho| byte_chirho.is_ascii_alphanumeric() || *byte_chirho == b'_')
        .count();
    let name_chirho = &text_chirho[..length_chirho];
    name_chirho
        .starts_with(|character_chirho: char| {
            character_chirho.is_ascii_alphabetic() || character_chirho == '_'
        })
        .then_some(name_chirho)
}
impl<'source_chirho> AnalysisChirho<'source_chirho> {
    fn new_chirho(
        source_chirho: &'source_chirho str,
        include_premise_chirho: bool,
    ) -> Result<Self, NextErrorChirho> {
        let mut result_chirho = Self {
            source_chirho,
            lines_chirho: Vec::new(),
            macros_chirho: BTreeSet::new(),
        };
        let mut branches_chirho = Vec::new();
        let mut offset_chirho = 0;
        for line_chirho in source_chirho.split_inclusive('\n') {
            let trimmed_chirho = line_chirho.trim_start();
            // The lossless source's multiline LANGUAGE pragma has a #-} tail;
            // that is a Haskell delimiter, not a preprocessing directive.
            let directive_chirho = trimmed_chirho
                .strip_prefix('#')
                .filter(|text_chirho| !text_chirho.starts_with("-}"));
            let unconditional_chirho = directive_chirho.is_none() && branches_chirho.is_empty();
            if let Some(directive_chirho) = directive_chirho {
                if directive_chirho.trim_end().ends_with('\\') {
                    return Err(error_chirho());
                }
                let directive_chirho = directive_chirho.trim();
                let (word_chirho, rest_chirho) = directive_chirho
                    .split_once(char::is_whitespace)
                    .unwrap_or((directive_chirho, ""));
                let rest_chirho = rest_chirho.trim();
                match word_chirho {
                    "if" | "ifdef" | "ifndef" if !rest_chirho.is_empty() => {
                        branches_chirho.push(false)
                    }
                    "elif" if !rest_chirho.is_empty() && branches_chirho.last() == Some(&false) => {
                    }
                    "else" if branches_chirho.last() == Some(&false) => {
                        *branches_chirho.last_mut().ok_or_else(error_chirho)? = true;
                    }
                    "endif" if !branches_chirho.is_empty() => {
                        branches_chirho.pop();
                    }
                    "define" | "undef" => {
                        result_chirho
                            .macros_chirho
                            .insert(identifier_chirho(rest_chirho).ok_or_else(error_chirho)?);
                    }
                    "include"
                        if include_premise_chirho
                            && rest_chirho.starts_with('"')
                            && rest_chirho.ends_with('"') => {}
                    "error" if !branches_chirho.is_empty() => {}
                    _ => return Err(error_chirho()),
                }
            }
            result_chirho.lines_chirho.push(LineChirho {
                start_chirho: offset_chirho,
                end_chirho: offset_chirho + line_chirho.len(),
                unconditional_chirho,
            });
            offset_chirho += line_chirho.len();
        }
        if !branches_chirho.is_empty() {
            return Err(error_chirho());
        }
        Ok(result_chirho)
    }
    fn region_chirho(&self, span_chirho: (usize, usize)) -> Result<(), NextErrorChirho> {
        let (start_chirho, end_chirho) = span_chirho;
        let text_chirho = self
            .source_chirho
            .get(start_chirho..end_chirho)
            .filter(|_| start_chirho < end_chirho)
            .ok_or_else(error_chirho)?;
        let first_chirho = self
            .lines_chirho
            .partition_point(|line_chirho| line_chirho.end_chirho <= start_chirho);
        if self.lines_chirho[first_chirho..]
            .iter()
            .take_while(|line_chirho| line_chirho.start_chirho < end_chirho)
            .any(|line_chirho| !line_chirho.unconditional_chirho)
        {
            return Err(error_chirho());
        }
        // CPP identifiers are ASCII words, including the prefix before a Haskell
        // prime. A conservative scan also excludes words in comments/strings.
        for word_chirho in text_chirho.split(|character_chirho: char| {
            !(character_chirho.is_ascii_alphanumeric() || character_chirho == '_')
        }) {
            if self.macros_chirho.contains(word_chirho) {
                return Err(error_chirho());
            }
        }
        Ok(())
    }
    fn facade_chirho(&self) -> String {
        let mut bytes_chirho = self.source_chirho.as_bytes().to_vec();
        for line_chirho in &self.lines_chirho {
            if !line_chirho.unconditional_chirho {
                for byte_chirho in
                    &mut bytes_chirho[line_chirho.start_chirho..line_chirho.end_chirho]
                {
                    if !matches!(*byte_chirho, b'\n' | b'\r') {
                        *byte_chirho = b' ';
                    }
                }
            }
        }
        // Only entire lines are blanked. Retained UTF-8 and all offsets survive.
        String::from_utf8(bytes_chirho).expect("line masking preserves UTF-8")
    }
}
pub(super) fn validate_chirho(
    manifest_chirho: &SourceManifestChirho,
) -> Result<(), NextErrorChirho> {
    let Some(profile_chirho) = manifest_chirho.cpp_chirho else {
        return Ok(());
    };
    let analysis_chirho = AnalysisChirho::new_chirho(
        manifest_chirho.bytes_chirho,
        profile_chirho.include_premise_chirho,
    )?;
    analysis_chirho.region_chirho(profile_chirho.header_span_chirho)?;
    for span_chirho in manifest_chirho.spans_chirho {
        analysis_chirho.region_chirho(*span_chirho)?;
    }
    Ok(())
}
pub(super) fn facade_chirho(
    manifest_chirho: &SourceManifestChirho,
) -> Result<String, NextErrorChirho> {
    let Some(profile_chirho) = manifest_chirho.cpp_chirho else {
        return Ok(manifest_chirho.bytes_chirho.to_string());
    };
    validate_chirho(manifest_chirho)?;
    let analysis_chirho = AnalysisChirho::new_chirho(
        manifest_chirho.bytes_chirho,
        profile_chirho.include_premise_chirho,
    )?;
    let source_chirho = analysis_chirho.facade_chirho();
    super::facade_chirho::read_with_macros_chirho(
        &source_chirho,
        manifest_chirho.module_chirho,
        &analysis_chirho.macros_chirho,
    )?;
    Ok(source_chirho)
}

#[cfg(test)]
#[path = "cpp_tests_chirho.rs"]
mod tests_chirho;
