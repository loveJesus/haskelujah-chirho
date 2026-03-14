// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Cabal file parser
//!
//! Parses `.cabal` files into a structured `PackageDescChirho`. Handles:
//! - Top-level fields (name, version, license, author, etc.)
//! - `library` stanza (exposed-modules, other-modules, hs-source-dirs, build-depends, default-language, ghc-options, default-extensions)
//! - `executable <name>` stanzas (main-is, other-modules, hs-source-dirs, build-depends)
//! - `test-suite <name>` stanzas (type, main-is, other-modules, build-depends)
//!
//! ## Format
//!
//! The `.cabal` format is line-oriented with indentation-based continuation.
//! A field is `key: value`, and continuation lines are indented further than
//! the field key. Stanza headers (`library`, `executable foo`, `test-suite bar`)
//! start at column 0 or with indentation.
//!
//! Comments start with `--`.

use crate::version_chirho::{
    parse_version_chirho, parse_version_constraint_chirho, VersionChirho,
    VersionConstraintChirho,
};

// ---------------------------------------------------------------------------
// Package description types
// ---------------------------------------------------------------------------

/// A parsed Cabal package description.
#[derive(Debug, Clone, PartialEq)]
pub struct PackageDescChirho {
    pub name_chirho: String,
    pub version_chirho: Option<VersionChirho>,
    pub cabal_version_chirho: Option<String>,
    pub license_chirho: Option<String>,
    pub author_chirho: Option<String>,
    pub maintainer_chirho: Option<String>,
    pub synopsis_chirho: Option<String>,
    pub description_chirho: Option<String>,
    pub category_chirho: Option<String>,
    pub homepage_chirho: Option<String>,
    pub bug_reports_chirho: Option<String>,
    pub build_type_chirho: Option<String>,
    pub library_chirho: Option<LibraryChirho>,
    pub executables_chirho: Vec<ExecutableChirho>,
    pub test_suites_chirho: Vec<TestSuiteChirho>,
}

/// Shared build info fields.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BuildInfoChirho {
    pub build_depends_chirho: Vec<DependencyChirho>,
    pub hs_source_dirs_chirho: Vec<String>,
    pub default_language_chirho: Option<String>,
    pub ghc_options_chirho: Vec<String>,
    pub default_extensions_chirho: Vec<String>,
    pub other_extensions_chirho: Vec<String>,
}

/// A dependency: package name + optional version constraint.
#[derive(Debug, Clone, PartialEq)]
pub struct DependencyChirho {
    pub package_chirho: String,
    pub constraint_chirho: VersionConstraintChirho,
}

/// The library stanza.
#[derive(Debug, Clone, PartialEq)]
pub struct LibraryChirho {
    pub exposed_modules_chirho: Vec<String>,
    pub other_modules_chirho: Vec<String>,
    pub build_info_chirho: BuildInfoChirho,
}

/// An executable stanza.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutableChirho {
    pub name_chirho: String,
    pub main_is_chirho: Option<String>,
    pub other_modules_chirho: Vec<String>,
    pub build_info_chirho: BuildInfoChirho,
}

/// A test-suite stanza.
#[derive(Debug, Clone, PartialEq)]
pub struct TestSuiteChirho {
    pub name_chirho: String,
    pub type_chirho: Option<String>,
    pub main_is_chirho: Option<String>,
    pub other_modules_chirho: Vec<String>,
    pub build_info_chirho: BuildInfoChirho,
}

// ---------------------------------------------------------------------------
// Parser internals
// ---------------------------------------------------------------------------

/// A logical field: key + fully joined value (continuations merged).
struct FieldChirho {
    key_chirho: String,
    value_chirho: String,
}

/// Which stanza we're currently inside.
#[derive(Debug, Clone)]
enum StanzaChirho {
    TopLevelChirho,
    LibraryChirho,
    ExecutableChirho(String),
    TestSuiteChirho(String),
}

/// Parse a `.cabal` file string into a `PackageDescChirho`.
pub fn parse_cabal_chirho(input_chirho: &str) -> PackageDescChirho {
    let fields_chirho = lex_fields_chirho(input_chirho);
    let stanzas_chirho = group_stanzas_chirho(&fields_chirho);

    let mut pkg_chirho = PackageDescChirho {
        name_chirho: String::new(),
        version_chirho: None,
        cabal_version_chirho: None,
        license_chirho: None,
        author_chirho: None,
        maintainer_chirho: None,
        synopsis_chirho: None,
        description_chirho: None,
        category_chirho: None,
        homepage_chirho: None,
        bug_reports_chirho: None,
        build_type_chirho: None,
        library_chirho: None,
        executables_chirho: Vec::new(),
        test_suites_chirho: Vec::new(),
    };

    for (stanza_chirho, stanza_fields_chirho) in &stanzas_chirho {
        match stanza_chirho {
            StanzaChirho::TopLevelChirho => {
                apply_top_level_chirho(&mut pkg_chirho, stanza_fields_chirho);
            }
            StanzaChirho::LibraryChirho => {
                let lib_chirho = parse_library_chirho(stanza_fields_chirho);
                pkg_chirho.library_chirho = Some(lib_chirho);
            }
            StanzaChirho::ExecutableChirho(name_chirho) => {
                let exe_chirho =
                    parse_executable_chirho(name_chirho, stanza_fields_chirho);
                pkg_chirho.executables_chirho.push(exe_chirho);
            }
            StanzaChirho::TestSuiteChirho(name_chirho) => {
                let ts_chirho =
                    parse_test_suite_chirho(name_chirho, stanza_fields_chirho);
                pkg_chirho.test_suites_chirho.push(ts_chirho);
            }
        }
    }

    pkg_chirho
}

/// Lex the input into logical fields (key-value pairs with continuations merged).
fn lex_fields_chirho(input_chirho: &str) -> Vec<FieldChirho> {
    let mut fields_chirho: Vec<FieldChirho> = Vec::new();
    let mut current_key_chirho: Option<String> = None;
    let mut current_value_chirho = String::new();
    let mut field_indent_chirho: usize = 0;

    for raw_line_chirho in input_chirho.lines() {
        // Strip comments
        let line_chirho = strip_comment_chirho(raw_line_chirho);

        // Completely blank line → flush current field
        if line_chirho.trim().is_empty() {
            if let Some(key_chirho) = current_key_chirho.take() {
                fields_chirho.push(FieldChirho {
                    key_chirho,
                    value_chirho: current_value_chirho.trim().to_string(),
                });
                current_value_chirho.clear();
            }
            continue;
        }

        let indent_chirho = line_chirho.len() - line_chirho.trim_start().len();
        let trimmed_chirho = line_chirho.trim();

        // Check if this is a new field (contains `:` and starts at <= field indent)
        if let Some(colon_idx_chirho) = trimmed_chirho.find(':') {
            let potential_key_chirho = &trimmed_chirho[..colon_idx_chirho];
            // A key must be a single word (no spaces except in stanza headers)
            let is_stanza_header_chirho = matches!(
                potential_key_chirho.to_lowercase().as_str(),
                "library" | "common"
            ) || potential_key_chirho.to_lowercase().starts_with("executable")
                || potential_key_chirho.to_lowercase().starts_with("test-suite")
                || potential_key_chirho.to_lowercase().starts_with("benchmark")
                || potential_key_chirho.to_lowercase().starts_with("source-repository");

            let looks_like_field_chirho = !potential_key_chirho.is_empty()
                && !potential_key_chirho.contains(' ')
                || is_stanza_header_chirho;

            if looks_like_field_chirho
                && (current_key_chirho.is_none() || indent_chirho <= field_indent_chirho)
            {
                // Flush previous field
                if let Some(key_chirho) = current_key_chirho.take() {
                    fields_chirho.push(FieldChirho {
                        key_chirho,
                        value_chirho: current_value_chirho.trim().to_string(),
                    });
                    current_value_chirho.clear();
                }

                current_key_chirho =
                    Some(potential_key_chirho.to_lowercase().to_string());
                let val_part_chirho = trimmed_chirho[colon_idx_chirho + 1..].trim();
                current_value_chirho = val_part_chirho.to_string();
                field_indent_chirho = indent_chirho;
                continue;
            }
        }

        // Check for stanza header without colon (e.g. bare `library`)
        let lower_chirho = trimmed_chirho.to_lowercase();
        if lower_chirho == "library"
            || lower_chirho.starts_with("executable ")
            || lower_chirho.starts_with("test-suite ")
            || lower_chirho.starts_with("benchmark ")
            || lower_chirho.starts_with("source-repository ")
            || lower_chirho.starts_with("common ")
        {
            // Flush previous field
            if let Some(key_chirho) = current_key_chirho.take() {
                fields_chirho.push(FieldChirho {
                    key_chirho,
                    value_chirho: current_value_chirho.trim().to_string(),
                });
                current_value_chirho.clear();
            }

            // Create a pseudo-field for the stanza header
            let (header_chirho, name_part_chirho) =
                if let Some(idx_chirho) = lower_chirho.find(' ') {
                    (
                        lower_chirho[..idx_chirho].to_string(),
                        trimmed_chirho[idx_chirho + 1..].trim().to_string(),
                    )
                } else {
                    (lower_chirho.clone(), String::new())
                };

            fields_chirho.push(FieldChirho {
                key_chirho: format!("__stanza__{}", header_chirho),
                value_chirho: name_part_chirho,
            });
            field_indent_chirho = indent_chirho;
            continue;
        }

        // Continuation line
        if current_key_chirho.is_some() {
            if !current_value_chirho.is_empty() {
                current_value_chirho.push(' ');
            }
            current_value_chirho.push_str(trimmed_chirho);
        }
    }

    // Flush last field
    if let Some(key_chirho) = current_key_chirho.take() {
        fields_chirho.push(FieldChirho {
            key_chirho,
            value_chirho: current_value_chirho.trim().to_string(),
        });
    }

    fields_chirho
}

/// Strip `--` comments from a line.
fn strip_comment_chirho(line_chirho: &str) -> &str {
    // Simple: find first `--` not inside a string
    if let Some(idx_chirho) = line_chirho.find("--") {
        &line_chirho[..idx_chirho]
    } else {
        line_chirho
    }
}

/// Group fields into stanzas.
fn group_stanzas_chirho(
    fields_chirho: &[FieldChirho],
) -> Vec<(StanzaChirho, Vec<&FieldChirho>)> {
    let mut stanzas_chirho: Vec<(StanzaChirho, Vec<&FieldChirho>)> = Vec::new();
    let mut current_stanza_chirho = StanzaChirho::TopLevelChirho;
    let mut current_fields_chirho: Vec<&FieldChirho> = Vec::new();

    for field_chirho in fields_chirho {
        if let Some(stanza_type_chirho) =
            field_chirho.key_chirho.strip_prefix("__stanza__")
        {
            // Flush current stanza
            if !current_fields_chirho.is_empty()
                || matches!(current_stanza_chirho, StanzaChirho::TopLevelChirho)
            {
                stanzas_chirho
                    .push((current_stanza_chirho.clone(), current_fields_chirho));
                current_fields_chirho = Vec::new();
            }

            current_stanza_chirho = match stanza_type_chirho {
                "library" => StanzaChirho::LibraryChirho,
                "executable" => {
                    StanzaChirho::ExecutableChirho(field_chirho.value_chirho.clone())
                }
                "test-suite" => {
                    StanzaChirho::TestSuiteChirho(field_chirho.value_chirho.clone())
                }
                _ => StanzaChirho::TopLevelChirho,
            };
        } else {
            current_fields_chirho.push(field_chirho);
        }
    }

    // Flush final stanza
    stanzas_chirho.push((current_stanza_chirho, current_fields_chirho));

    stanzas_chirho
}

/// Apply top-level fields to the package description.
fn apply_top_level_chirho(
    pkg_chirho: &mut PackageDescChirho,
    fields_chirho: &[&FieldChirho],
) {
    for field_chirho in fields_chirho {
        match field_chirho.key_chirho.as_str() {
            "name" => pkg_chirho.name_chirho = field_chirho.value_chirho.clone(),
            "version" => {
                pkg_chirho.version_chirho =
                    parse_version_chirho(&field_chirho.value_chirho);
            }
            "cabal-version" => {
                pkg_chirho.cabal_version_chirho =
                    Some(field_chirho.value_chirho.clone());
            }
            "license" => {
                pkg_chirho.license_chirho = Some(field_chirho.value_chirho.clone());
            }
            "author" => {
                pkg_chirho.author_chirho = Some(field_chirho.value_chirho.clone());
            }
            "maintainer" => {
                pkg_chirho.maintainer_chirho = Some(field_chirho.value_chirho.clone());
            }
            "synopsis" => {
                pkg_chirho.synopsis_chirho = Some(field_chirho.value_chirho.clone());
            }
            "description" => {
                pkg_chirho.description_chirho =
                    Some(field_chirho.value_chirho.clone());
            }
            "category" => {
                pkg_chirho.category_chirho = Some(field_chirho.value_chirho.clone());
            }
            "homepage" => {
                pkg_chirho.homepage_chirho = Some(field_chirho.value_chirho.clone());
            }
            "bug-reports" => {
                pkg_chirho.bug_reports_chirho =
                    Some(field_chirho.value_chirho.clone());
            }
            "build-type" => {
                pkg_chirho.build_type_chirho =
                    Some(field_chirho.value_chirho.clone());
            }
            _ => {} // Ignore unknown top-level fields
        }
    }
}

/// Parse a list of dependencies from a `build-depends` value.
fn parse_deps_chirho(value_chirho: &str) -> Vec<DependencyChirho> {
    let mut deps_chirho = Vec::new();

    for dep_str_chirho in value_chirho.split(',') {
        let trimmed_chirho = dep_str_chirho.trim();
        if trimmed_chirho.is_empty() {
            continue;
        }

        // Split on first whitespace or operator
        let (name_chirho, constraint_str_chirho) =
            split_dep_name_chirho(trimmed_chirho);

        if name_chirho.is_empty() {
            continue;
        }

        let constraint_chirho = if constraint_str_chirho.is_empty() {
            VersionConstraintChirho::AnyChirho
        } else {
            parse_version_constraint_chirho(constraint_str_chirho)
                .unwrap_or(VersionConstraintChirho::AnyChirho)
        };

        deps_chirho.push(DependencyChirho {
            package_chirho: name_chirho.to_string(),
            constraint_chirho,
        });
    }

    deps_chirho
}

/// Split a dependency string into name and constraint parts.
fn split_dep_name_chirho(input_chirho: &str) -> (&str, &str) {
    let bytes_chirho = input_chirho.as_bytes();
    for (i_chirho, &b_chirho) in bytes_chirho.iter().enumerate() {
        if b_chirho == b' '
            || b_chirho == b'>'
            || b_chirho == b'<'
            || b_chirho == b'='
            || b_chirho == b'^'
        {
            return (
                input_chirho[..i_chirho].trim(),
                input_chirho[i_chirho..].trim(),
            );
        }
    }
    (input_chirho.trim(), "")
}

/// Parse comma-separated module list.
fn parse_modules_chirho(value_chirho: &str) -> Vec<String> {
    value_chirho
        .split([',', ' '])
        .map(|s_chirho| s_chirho.trim().to_string())
        .filter(|s_chirho| !s_chirho.is_empty())
        .collect()
}

/// Parse common build-info fields from a stanza's fields.
fn parse_build_info_chirho(fields_chirho: &[&FieldChirho]) -> BuildInfoChirho {
    let mut info_chirho = BuildInfoChirho::default();

    for field_chirho in fields_chirho {
        match field_chirho.key_chirho.as_str() {
            "build-depends" => {
                info_chirho.build_depends_chirho =
                    parse_deps_chirho(&field_chirho.value_chirho);
            }
            "hs-source-dirs" => {
                info_chirho.hs_source_dirs_chirho =
                    parse_modules_chirho(&field_chirho.value_chirho);
            }
            "default-language" => {
                info_chirho.default_language_chirho =
                    Some(field_chirho.value_chirho.clone());
            }
            "ghc-options" => {
                info_chirho.ghc_options_chirho = field_chirho
                    .value_chirho
                    .split_whitespace()
                    .map(|s_chirho| s_chirho.to_string())
                    .collect();
            }
            "default-extensions" => {
                info_chirho.default_extensions_chirho =
                    parse_modules_chirho(&field_chirho.value_chirho);
            }
            "other-extensions" => {
                info_chirho.other_extensions_chirho =
                    parse_modules_chirho(&field_chirho.value_chirho);
            }
            _ => {}
        }
    }

    info_chirho
}

/// Parse a library stanza.
fn parse_library_chirho(fields_chirho: &[&FieldChirho]) -> LibraryChirho {
    let mut exposed_chirho = Vec::new();
    let mut other_chirho = Vec::new();

    for field_chirho in fields_chirho {
        match field_chirho.key_chirho.as_str() {
            "exposed-modules" => {
                exposed_chirho = parse_modules_chirho(&field_chirho.value_chirho);
            }
            "other-modules" => {
                other_chirho = parse_modules_chirho(&field_chirho.value_chirho);
            }
            _ => {}
        }
    }

    LibraryChirho {
        exposed_modules_chirho: exposed_chirho,
        other_modules_chirho: other_chirho,
        build_info_chirho: parse_build_info_chirho(fields_chirho),
    }
}

/// Parse an executable stanza.
fn parse_executable_chirho(
    name_chirho: &str,
    fields_chirho: &[&FieldChirho],
) -> ExecutableChirho {
    let mut main_is_chirho = None;
    let mut other_chirho = Vec::new();

    for field_chirho in fields_chirho {
        match field_chirho.key_chirho.as_str() {
            "main-is" => {
                main_is_chirho = Some(field_chirho.value_chirho.clone());
            }
            "other-modules" => {
                other_chirho = parse_modules_chirho(&field_chirho.value_chirho);
            }
            _ => {}
        }
    }

    ExecutableChirho {
        name_chirho: name_chirho.to_string(),
        main_is_chirho,
        other_modules_chirho: other_chirho,
        build_info_chirho: parse_build_info_chirho(fields_chirho),
    }
}

/// Parse a test-suite stanza.
fn parse_test_suite_chirho(
    name_chirho: &str,
    fields_chirho: &[&FieldChirho],
) -> TestSuiteChirho {
    let mut type_chirho = None;
    let mut main_is_chirho = None;
    let mut other_chirho = Vec::new();

    for field_chirho in fields_chirho {
        match field_chirho.key_chirho.as_str() {
            "type" => {
                type_chirho = Some(field_chirho.value_chirho.clone());
            }
            "main-is" => {
                main_is_chirho = Some(field_chirho.value_chirho.clone());
            }
            "other-modules" => {
                other_chirho = parse_modules_chirho(&field_chirho.value_chirho);
            }
            _ => {}
        }
    }

    TestSuiteChirho {
        name_chirho: name_chirho.to_string(),
        type_chirho,
        main_is_chirho,
        other_modules_chirho: other_chirho,
        build_info_chirho: parse_build_info_chirho(fields_chirho),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    const SAMPLE_CABAL_CHIRHO: &str = r#"
cabal-version:      3.0
name:               hello-world
version:            0.1.0.0
license:            MIT
author:             Test Author
synopsis:           A simple example
build-type:         Simple

library
  exposed-modules:  Hello, Hello.World
  other-modules:    Hello.Internal
  hs-source-dirs:   src
  build-depends:    base >=4.14 && <5
  default-language: Haskell2010

executable hello
  main-is:          Main.hs
  other-modules:    CLI
  hs-source-dirs:   app
  build-depends:    base >=4.14 && <5,
                    hello-world
  default-language: Haskell2010

test-suite hello-test
  type:             exitcode-stdio-1.0
  main-is:          Spec.hs
  other-modules:    Test.Hello
  hs-source-dirs:   test
  build-depends:    base >=4.14 && <5,
                    hello-world,
                    hspec ^>=2.11
  default-language: Haskell2010
"#;

    #[test]
    fn parse_top_level_fields_chirho() {
        let pkg_chirho = parse_cabal_chirho(SAMPLE_CABAL_CHIRHO);
        assert_eq!(pkg_chirho.name_chirho, "hello-world");
        assert_eq!(
            pkg_chirho.version_chirho,
            Some(VersionChirho::new_chirho(vec![0, 1, 0, 0]))
        );
        assert_eq!(pkg_chirho.license_chirho, Some("MIT".to_string()));
        assert_eq!(pkg_chirho.author_chirho, Some("Test Author".to_string()));
        assert_eq!(pkg_chirho.build_type_chirho, Some("Simple".to_string()));
    }

    #[test]
    fn parse_library_stanza_chirho() {
        let pkg_chirho = parse_cabal_chirho(SAMPLE_CABAL_CHIRHO);
        let lib_chirho = pkg_chirho.library_chirho.as_ref().unwrap();

        assert_eq!(lib_chirho.exposed_modules_chirho, vec!["Hello", "Hello.World"]);
        assert_eq!(lib_chirho.other_modules_chirho, vec!["Hello.Internal"]);
        assert_eq!(lib_chirho.build_info_chirho.hs_source_dirs_chirho, vec!["src"]);
        assert_eq!(
            lib_chirho.build_info_chirho.default_language_chirho,
            Some("Haskell2010".to_string())
        );

        // Check dependency parsing
        assert_eq!(lib_chirho.build_info_chirho.build_depends_chirho.len(), 1);
        assert_eq!(
            lib_chirho.build_info_chirho.build_depends_chirho[0].package_chirho,
            "base"
        );
    }

    #[test]
    fn parse_executable_stanza_chirho() {
        let pkg_chirho = parse_cabal_chirho(SAMPLE_CABAL_CHIRHO);
        assert_eq!(pkg_chirho.executables_chirho.len(), 1);
        let exe_chirho = &pkg_chirho.executables_chirho[0];
        assert_eq!(exe_chirho.name_chirho, "hello");
        assert_eq!(exe_chirho.main_is_chirho, Some("Main.hs".to_string()));
        assert_eq!(exe_chirho.other_modules_chirho, vec!["CLI"]);
        assert_eq!(
            exe_chirho.build_info_chirho.hs_source_dirs_chirho,
            vec!["app"]
        );
        assert_eq!(exe_chirho.build_info_chirho.build_depends_chirho.len(), 2);
    }

    #[test]
    fn parse_test_suite_stanza_chirho() {
        let pkg_chirho = parse_cabal_chirho(SAMPLE_CABAL_CHIRHO);
        assert_eq!(pkg_chirho.test_suites_chirho.len(), 1);
        let ts_chirho = &pkg_chirho.test_suites_chirho[0];
        assert_eq!(ts_chirho.name_chirho, "hello-test");
        assert_eq!(
            ts_chirho.type_chirho,
            Some("exitcode-stdio-1.0".to_string())
        );
        assert_eq!(ts_chirho.main_is_chirho, Some("Spec.hs".to_string()));
        assert_eq!(ts_chirho.other_modules_chirho, vec!["Test.Hello"]);

        // hspec ^>=2.11 should parse
        let hspec_dep_chirho = ts_chirho
            .build_info_chirho
            .build_depends_chirho
            .iter()
            .find(|d_chirho| d_chirho.package_chirho == "hspec")
            .unwrap();
        assert!(matches!(
            hspec_dep_chirho.constraint_chirho,
            VersionConstraintChirho::CaretChirho(_)
        ));
    }

    #[test]
    fn parse_deps_multiple_chirho() {
        let deps_chirho = parse_deps_chirho(
            "base >=4.14 && <5, containers >=0.6, text ^>=2.0",
        );
        assert_eq!(deps_chirho.len(), 3);
        assert_eq!(deps_chirho[0].package_chirho, "base");
        assert_eq!(deps_chirho[1].package_chirho, "containers");
        assert_eq!(deps_chirho[2].package_chirho, "text");
    }

    #[test]
    fn parse_deps_no_constraint_chirho() {
        let deps_chirho = parse_deps_chirho("hello-world");
        assert_eq!(deps_chirho.len(), 1);
        assert_eq!(deps_chirho[0].package_chirho, "hello-world");
        assert_eq!(
            deps_chirho[0].constraint_chirho,
            VersionConstraintChirho::AnyChirho
        );
    }

    #[test]
    fn parse_comments_stripped_chirho() {
        let input_chirho = r#"
name:    foo -- package name
version: 1.0
-- this is a comment line
"#;
        let pkg_chirho = parse_cabal_chirho(input_chirho);
        assert_eq!(pkg_chirho.name_chirho, "foo");
        assert_eq!(
            pkg_chirho.version_chirho,
            Some(VersionChirho::new_chirho(vec![1, 0]))
        );
    }

    #[test]
    fn parse_continuation_lines_chirho() {
        let input_chirho = r#"
name:    multi
version: 2.0
description:
  This is a multi-line
  description of the package.
"#;
        let pkg_chirho = parse_cabal_chirho(input_chirho);
        assert_eq!(pkg_chirho.name_chirho, "multi");
        assert!(pkg_chirho
            .description_chirho
            .as_ref()
            .unwrap()
            .contains("multi-line"));
        assert!(pkg_chirho
            .description_chirho
            .as_ref()
            .unwrap()
            .contains("description of the package."));
    }

    #[test]
    fn parse_empty_cabal_chirho() {
        let pkg_chirho = parse_cabal_chirho("");
        assert_eq!(pkg_chirho.name_chirho, "");
        assert!(pkg_chirho.library_chirho.is_none());
        assert!(pkg_chirho.executables_chirho.is_empty());
    }

    #[test]
    fn dependency_constraint_satisfaction_chirho() {
        let deps_chirho =
            parse_deps_chirho("base >=4.14 && <5");
        let dep_chirho = &deps_chirho[0];

        let v_ok_chirho =
            parse_version_chirho("4.17.0.0").unwrap();
        assert!(dep_chirho.constraint_chirho.satisfied_by_chirho(&v_ok_chirho));

        let v_bad_chirho = parse_version_chirho("5.0").unwrap();
        assert!(!dep_chirho.constraint_chirho.satisfied_by_chirho(&v_bad_chirho));

        let v_old_chirho = parse_version_chirho("4.13").unwrap();
        assert!(!dep_chirho.constraint_chirho.satisfied_by_chirho(&v_old_chirho));
    }

    #[test]
    fn ghc_options_parsed_chirho() {
        let input_chirho = r#"
name: opts
version: 1.0

library
  exposed-modules: Opts
  ghc-options: -Wall -Wcompat -O2
  default-language: Haskell2010
"#;
        let pkg_chirho = parse_cabal_chirho(input_chirho);
        let lib_chirho = pkg_chirho.library_chirho.as_ref().unwrap();
        assert_eq!(
            lib_chirho.build_info_chirho.ghc_options_chirho,
            vec!["-Wall", "-Wcompat", "-O2"]
        );
    }
}
