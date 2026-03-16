// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Diagnostic renderer
//!
//! Renders [`DiagnosticChirho`] values as human-readable, Rust/Elm-style
//! error messages with source code snippets, underline annotations, and
//! optional ANSI color output.

use haskelujah_span_chirho::{SourceMapChirho, SpanChirho};

use crate::{DiagnosticBundleChirho, DiagnosticChirho, SeverityChirho};

/// Configuration for the diagnostic renderer.
#[derive(Debug, Clone)]
pub struct RenderConfigChirho {
    /// Whether to emit ANSI color escape codes.
    pub color_chirho: bool,
    /// Number of context lines to show around the primary span.
    pub context_lines_chirho: u32,
}

impl Default for RenderConfigChirho {
    fn default() -> Self {
        Self {
            color_chirho: true,
            context_lines_chirho: 1,
        }
    }
}

impl RenderConfigChirho {
    /// Config with no colors (for testing or piped output).
    pub fn plain_chirho() -> Self {
        Self {
            color_chirho: false,
            context_lines_chirho: 1,
        }
    }
}

/// ANSI escape helpers.
struct AnsiChirho {
    color_chirho: bool,
}

impl AnsiChirho {
    fn new_chirho(color_chirho: bool) -> Self {
        Self { color_chirho }
    }

    fn bold_chirho(&self, text_chirho: &str) -> String {
        if self.color_chirho {
            format!("\x1b[1m{text_chirho}\x1b[0m")
        } else {
            text_chirho.to_string()
        }
    }

    fn severity_color_chirho(&self, severity_chirho: SeverityChirho, text_chirho: &str) -> String {
        if !self.color_chirho {
            return text_chirho.to_string();
        }
        let code_chirho = match severity_chirho {
            SeverityChirho::ErrorChirho => "\x1b[1;31m",   // bold red
            SeverityChirho::WarningChirho => "\x1b[1;33m",  // bold yellow
            SeverityChirho::InfoChirho => "\x1b[1;36m",     // bold cyan
            SeverityChirho::HintChirho => "\x1b[1;32m",     // bold green
        };
        format!("{code_chirho}{text_chirho}\x1b[0m")
    }

    fn blue_chirho(&self, text_chirho: &str) -> String {
        if self.color_chirho {
            format!("\x1b[1;34m{text_chirho}\x1b[0m")
        } else {
            text_chirho.to_string()
        }
    }

    fn underline_char_chirho(&self, severity_chirho: SeverityChirho) -> char {
        match severity_chirho {
            SeverityChirho::ErrorChirho => '^',
            SeverityChirho::WarningChirho => '^',
            SeverityChirho::InfoChirho => '-',
            SeverityChirho::HintChirho => '-',
        }
    }
}

/// Render a single diagnostic to a string.
pub fn render_diagnostic_chirho(
    diag_chirho: &DiagnosticChirho,
    source_map_chirho: &SourceMapChirho,
    config_chirho: &RenderConfigChirho,
) -> String {
    let ansi_chirho = AnsiChirho::new_chirho(config_chirho.color_chirho);
    let mut output_chirho = String::new();

    // Header line: severity[code]: message
    let severity_str_chirho = format!("{}", diag_chirho.severity_chirho);
    let header_chirho = if let Some(ref code_chirho) = diag_chirho.code_chirho {
        format!("{severity_str_chirho}[{code_chirho}]")
    } else {
        severity_str_chirho
    };
    let colored_header_chirho =
        ansi_chirho.severity_color_chirho(diag_chirho.severity_chirho, &header_chirho);
    let bold_msg_chirho = ansi_chirho.bold_chirho(&diag_chirho.message_chirho);
    output_chirho.push_str(&format!("{colored_header_chirho}: {bold_msg_chirho}\n"));

    // Location arrow: --> file:line:col
    if let Some(primary_span_chirho) = diag_chirho.primary_span_chirho() {
        if primary_span_chirho != SpanChirho::DUMMY_CHIRHO {
            if let Some((file_name_chirho, line_col_chirho)) =
                source_map_chirho.resolve_span_start_chirho(primary_span_chirho)
            {
                let arrow_chirho = ansi_chirho.blue_chirho("-->");
                output_chirho.push_str(&format!(
                    " {arrow_chirho} {file_name_chirho}:{}:{}\n",
                    line_col_chirho.line_chirho, line_col_chirho.col_chirho
                ));

                // Source snippet with underline
                render_snippet_chirho(
                    &mut output_chirho,
                    primary_span_chirho,
                    diag_chirho.severity_chirho,
                    source_map_chirho,
                    &ansi_chirho,
                    config_chirho,
                    diag_chirho
                        .labels_chirho
                        .iter()
                        .find(|l_chirho| l_chirho.is_primary_chirho)
                        .and_then(|l_chirho| l_chirho.message_chirho.as_deref()),
                );
            }
        }
    }

    // Secondary labels
    for label_chirho in &diag_chirho.labels_chirho {
        if label_chirho.is_primary_chirho {
            continue;
        }
        if label_chirho.span_chirho == SpanChirho::DUMMY_CHIRHO {
            continue;
        }
        if let Some((file_name_chirho, line_col_chirho)) =
            source_map_chirho.resolve_span_start_chirho(label_chirho.span_chirho)
        {
            let arrow_chirho = ansi_chirho.blue_chirho("-->");
            output_chirho.push_str(&format!(
                " {arrow_chirho} {file_name_chirho}:{}:{}\n",
                line_col_chirho.line_chirho, line_col_chirho.col_chirho
            ));
            render_snippet_chirho(
                &mut output_chirho,
                label_chirho.span_chirho,
                SeverityChirho::InfoChirho,
                source_map_chirho,
                &ansi_chirho,
                config_chirho,
                label_chirho.message_chirho.as_deref(),
            );
        }
    }

    // Notes
    for note_chirho in &diag_chirho.notes_chirho {
        let note_prefix_chirho = ansi_chirho.blue_chirho("  = ");
        let note_label_chirho = ansi_chirho.bold_chirho("note:");
        output_chirho.push_str(&format!(
            "{note_prefix_chirho}{note_label_chirho} {note_chirho}\n"
        ));
    }

    // Suggestions
    for suggestion_chirho in &diag_chirho.suggestions_chirho {
        let help_prefix_chirho = ansi_chirho.blue_chirho("  = ");
        let help_label_chirho =
            ansi_chirho.severity_color_chirho(SeverityChirho::HintChirho, "help:");
        output_chirho.push_str(&format!(
            "{help_prefix_chirho}{help_label_chirho} {}\n",
            suggestion_chirho.description_chirho
        ));
    }

    output_chirho
}

/// Render a source code snippet with underline annotation.
fn render_snippet_chirho(
    output_chirho: &mut String,
    span_chirho: SpanChirho,
    severity_chirho: SeverityChirho,
    source_map_chirho: &SourceMapChirho,
    ansi_chirho: &AnsiChirho,
    config_chirho: &RenderConfigChirho,
    label_message_chirho: Option<&str>,
) {
    let file_id_chirho = span_chirho.file_id_chirho();
    let source_chirho = match source_map_chirho.file_source_chirho(file_id_chirho) {
        Some(s_chirho) => s_chirho,
        None => return,
    };
    let line_index_chirho = match source_map_chirho.line_index_chirho(file_id_chirho) {
        Some(li_chirho) => li_chirho,
        None => return,
    };

    let start_lc_chirho = line_index_chirho.line_col_chirho(span_chirho.start_chirho());
    let end_lc_chirho = line_index_chirho.line_col_chirho(span_chirho.end_chirho());

    let first_line_chirho = start_lc_chirho
        .line_chirho
        .saturating_sub(config_chirho.context_lines_chirho)
        .max(1);
    let last_line_chirho = (end_lc_chirho.line_chirho + config_chirho.context_lines_chirho)
        .min(line_index_chirho.line_count_chirho() as u32);

    let gutter_width_chirho = format!("{last_line_chirho}").len();

    // Empty gutter line
    let gutter_pad_chirho = " ".repeat(gutter_width_chirho);
    output_chirho.push_str(&format!(
        " {}\n",
        ansi_chirho.blue_chirho(&format!("{gutter_pad_chirho} |"))
    ));

    let lines_chirho: Vec<&str> = source_chirho.lines().collect();

    for line_num_chirho in first_line_chirho..=last_line_chirho {
        let line_idx_chirho = (line_num_chirho - 1) as usize;
        let line_text_chirho = lines_chirho
            .get(line_idx_chirho)
            .copied()
            .unwrap_or("");

        // Source line: " NN | text"
        let num_str_chirho = format!("{line_num_chirho:>gutter_width_chirho$}");
        output_chirho.push_str(&format!(
            " {} {}\n",
            ansi_chirho.blue_chirho(&format!("{num_str_chirho} |")),
            line_text_chirho
        ));

        // Underline line (only for lines within the span)
        if line_num_chirho >= start_lc_chirho.line_chirho
            && line_num_chirho <= end_lc_chirho.line_chirho
        {
            let underline_start_chirho = if line_num_chirho == start_lc_chirho.line_chirho {
                (start_lc_chirho.col_chirho - 1) as usize
            } else {
                0
            };
            let underline_end_chirho = if line_num_chirho == end_lc_chirho.line_chirho {
                (end_lc_chirho.col_chirho - 1) as usize
            } else {
                line_text_chirho.len()
            };
            let underline_len_chirho = underline_end_chirho
                .saturating_sub(underline_start_chirho)
                .max(1);

            let spaces_chirho = " ".repeat(underline_start_chirho);
            let carets_chirho = ansi_chirho
                .underline_char_chirho(severity_chirho)
                .to_string()
                .repeat(underline_len_chirho);

            let underline_text_chirho = if let Some(msg_chirho) = label_message_chirho {
                if line_num_chirho == end_lc_chirho.line_chirho {
                    format!("{spaces_chirho}{carets_chirho} {msg_chirho}")
                } else {
                    format!("{spaces_chirho}{carets_chirho}")
                }
            } else {
                format!("{spaces_chirho}{carets_chirho}")
            };

            let colored_underline_chirho =
                ansi_chirho.severity_color_chirho(severity_chirho, &underline_text_chirho);
            output_chirho.push_str(&format!(
                " {} {colored_underline_chirho}\n",
                ansi_chirho.blue_chirho(&format!("{gutter_pad_chirho} |")),
            ));
        }
    }
}

/// Render an entire diagnostic bundle.
pub fn render_bundle_chirho(
    bundle_chirho: &DiagnosticBundleChirho,
    source_map_chirho: &SourceMapChirho,
    config_chirho: &RenderConfigChirho,
) -> String {
    let mut output_chirho = String::new();
    for diag_chirho in bundle_chirho.diagnostics_chirho() {
        output_chirho.push_str(&render_diagnostic_chirho(
            diag_chirho,
            source_map_chirho,
            config_chirho,
        ));
        output_chirho.push('\n');
    }

    // Summary line
    let error_count_chirho = bundle_chirho.error_count_chirho();
    let warning_count_chirho = bundle_chirho.warning_count_chirho();
    if error_count_chirho > 0 || warning_count_chirho > 0 {
        let ansi_chirho = AnsiChirho::new_chirho(config_chirho.color_chirho);
        let mut parts_chirho = Vec::new();
        if error_count_chirho > 0 {
            let s_chirho = if error_count_chirho == 1 { "" } else { "s" };
            parts_chirho.push(ansi_chirho.severity_color_chirho(
                SeverityChirho::ErrorChirho,
                &format!("{error_count_chirho} error{s_chirho}"),
            ));
        }
        if warning_count_chirho > 0 {
            let s_chirho = if warning_count_chirho == 1 { "" } else { "s" };
            parts_chirho.push(ansi_chirho.severity_color_chirho(
                SeverityChirho::WarningChirho,
                &format!("{warning_count_chirho} warning{s_chirho}"),
            ));
        }
        output_chirho.push_str(&format!(
            "{} generated\n",
            parts_chirho.join(" and ")
        ));
    }

    output_chirho
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::{ErrorCodeChirho, LabelChirho};
    use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho};

    fn setup_chirho() -> (SourceMapChirho, FileIdChirho) {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let source_chirho = "module Main where\n\nmain = putStrLn \"hello\"\n\nfoo = bar + baz\n";
        let fid_chirho = sm_chirho.add_file_chirho("Main.hs", source_chirho);
        (sm_chirho, fid_chirho)
    }

    #[test]
    fn render_error_with_span_chirho() {
        let (sm_chirho, fid_chirho) = setup_chirho();
        let config_chirho = RenderConfigChirho::plain_chirho();

        let span_chirho = SpanChirho::new_chirho(
            fid_chirho,
            ByteOffsetChirho::new_chirho(50), // "bar" in "foo = bar + baz"
            ByteOffsetChirho::new_chirho(53),
        );
        let diag_chirho = DiagnosticChirho::error_with_code_chirho(
            ErrorCodeChirho::error_chirho(100),
            "variable not in scope: `bar`",
            span_chirho,
        );

        let rendered_chirho =
            render_diagnostic_chirho(&diag_chirho, &sm_chirho, &config_chirho);

        assert!(rendered_chirho.contains("error[E0100]"));
        assert!(rendered_chirho.contains("variable not in scope: `bar`"));
        assert!(rendered_chirho.contains("--> Main.hs:5:7"));
        assert!(rendered_chirho.contains("^^^"));
        assert!(rendered_chirho.contains("foo = bar + baz"));
    }

    #[test]
    fn render_warning_with_note_chirho() {
        let (sm_chirho, fid_chirho) = setup_chirho();
        let config_chirho = RenderConfigChirho::plain_chirho();

        let span_chirho = SpanChirho::new_chirho(
            fid_chirho,
            ByteOffsetChirho::new_chirho(19), // "main" on line 3
            ByteOffsetChirho::new_chirho(23),
        );
        let diag_chirho = DiagnosticChirho::warning_chirho("unused binding: `main`", span_chirho)
            .with_code_chirho(ErrorCodeChirho::warning_chirho(401))
            .with_note_chirho("prefix with an underscore to suppress this warning: `_main`");

        let rendered_chirho =
            render_diagnostic_chirho(&diag_chirho, &sm_chirho, &config_chirho);

        assert!(rendered_chirho.contains("warning[W0401]"));
        assert!(rendered_chirho.contains("unused binding: `main`"));
        assert!(rendered_chirho.contains("note:"));
        assert!(rendered_chirho.contains("_main"));
    }

    #[test]
    fn render_error_with_secondary_label_chirho() {
        let (sm_chirho, fid_chirho) = setup_chirho();
        let config_chirho = RenderConfigChirho::plain_chirho();

        let primary_span_chirho = SpanChirho::new_chirho(
            fid_chirho,
            ByteOffsetChirho::new_chirho(50),
            ByteOffsetChirho::new_chirho(53),
        );
        let secondary_span_chirho = SpanChirho::new_chirho(
            fid_chirho,
            ByteOffsetChirho::new_chirho(19),
            ByteOffsetChirho::new_chirho(23),
        );
        let diag_chirho = DiagnosticChirho::error_chirho("type mismatch", primary_span_chirho)
            .with_label_chirho(LabelChirho::secondary_chirho(
                secondary_span_chirho,
                "expected type from here",
            ));

        let rendered_chirho =
            render_diagnostic_chirho(&diag_chirho, &sm_chirho, &config_chirho);

        // Two --> arrows (primary + secondary)
        assert_eq!(
            rendered_chirho.matches("-->").count(),
            2,
            "should have 2 location arrows"
        );
    }

    #[test]
    fn render_bundle_summary_chirho() {
        let (sm_chirho, fid_chirho) = setup_chirho();
        let config_chirho = RenderConfigChirho::plain_chirho();

        let span_chirho = SpanChirho::new_chirho(
            fid_chirho,
            ByteOffsetChirho::new_chirho(50),
            ByteOffsetChirho::new_chirho(53),
        );
        let mut bundle_chirho = DiagnosticBundleChirho::empty_chirho();
        bundle_chirho.push_chirho(DiagnosticChirho::error_chirho("e1", span_chirho));
        bundle_chirho.push_chirho(DiagnosticChirho::error_chirho("e2", span_chirho));
        bundle_chirho.push_chirho(DiagnosticChirho::warning_chirho("w1", span_chirho));

        let rendered_chirho = render_bundle_chirho(&bundle_chirho, &sm_chirho, &config_chirho);

        assert!(rendered_chirho.contains("2 errors"));
        assert!(rendered_chirho.contains("1 warning"));
        assert!(rendered_chirho.contains("generated"));
    }

    #[test]
    fn render_dummy_span_no_snippet_chirho() {
        let sm_chirho = SourceMapChirho::new_chirho();
        let config_chirho = RenderConfigChirho::plain_chirho();

        let diag_chirho =
            DiagnosticChirho::error_no_span_chirho("cannot read file: missing.hs");

        let rendered_chirho =
            render_diagnostic_chirho(&diag_chirho, &sm_chirho, &config_chirho);

        assert!(rendered_chirho.contains("cannot read file"));
        // No --> arrow for spanless diagnostic
        assert!(!rendered_chirho.contains("-->"));
    }

    #[test]
    fn render_primary_label_message_chirho() {
        let (sm_chirho, fid_chirho) = setup_chirho();
        let config_chirho = RenderConfigChirho::plain_chirho();

        let span_chirho = SpanChirho::new_chirho(
            fid_chirho,
            ByteOffsetChirho::new_chirho(50),
            ByteOffsetChirho::new_chirho(53),
        );
        let diag_chirho = DiagnosticChirho {
            severity_chirho: SeverityChirho::ErrorChirho,
            code_chirho: None,
            message_chirho: "undefined variable".to_string(),
            labels_chirho: vec![LabelChirho::primary_chirho(span_chirho, "not found in this scope")],
            notes_chirho: vec![],
            suggestions_chirho: vec![],
        };

        let rendered_chirho =
            render_diagnostic_chirho(&diag_chirho, &sm_chirho, &config_chirho);

        assert!(rendered_chirho.contains("not found in this scope"));
    }
}
